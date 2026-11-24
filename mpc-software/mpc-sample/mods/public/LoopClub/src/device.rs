//! SDK boundary. Firmware validation remains entirely in the unmodified SDK.
use crate::{
    engine::{Command, Engine},
    model::*,
    runtime::Runtime,
    ui::{Action, Snapshot, Ui},
    view::{self, Painter},
};
use mpc_sample_sdk::{
    AudioHooks, ButtonEvent, CcEvent, EncoderEvent, Flow, InputHooks, Mod, NoteEvent, PadAction,
    PadEvent, PadResponse, PaintEvent,
};
use std::{
    sync::{
        OnceLock,
        atomic::{AtomicU8, AtomicU32, Ordering},
    },
    time::{Duration, Instant},
};
// Scratch-only fast path: immutable UI route + lock-free audio commands.
// Never calls a native widget, Graphics, filesystem, or mutable Ui from a worker.
static SCRATCH_ROUTE: AtomicU8 = AtomicU8::new(255);
static SCRATCH_CUT_OWNER: AtomicU8 = AtomicU8::new(255);
static FADER_VALUE: AtomicU32 = AtomicU32::new(f32::NAN.to_bits());
static FADER_EPOCH: OnceLock<Instant> = OnceLock::new();
static FADER_TIMES: [std::sync::atomic::AtomicU64; 16] =
    [const { std::sync::atomic::AtomicU64::new(0) }; 16];
fn scratch_route(ui: &Ui, s: &Snapshot, browser: bool) -> u8 {
    if !browser
        && ui.active
        && (ui.page == Page::Play || ui.page == Page::Loop && ui.loop_mode == 0 && !ui.loop_edit)
        && s.track_kind(ui.selected) == Kind::Audio
        && s.audio_target(ui.selected) == crate::audio::RecordTarget::Turntable
    {
        ui.selected as u8
    } else {
        255
    }
}
fn timed_fader(track: usize, value: f32, received: Instant) {
    let now = FADER_EPOCH
        .get()
        .and_then(|epoch| received.checked_duration_since(*epoch))
        .map_or(1, |dt| {
            dt.as_micros().min(u128::from(u64::MAX - 1)) as u64 + 1
        });
    let prior = FADER_TIMES[track].swap(now, Ordering::AcqRel);
    let us = if prior == 0 {
        0
    } else {
        now.saturating_sub(prior).min(u64::from(u32::MAX)) as u32
    };
    FADER_VALUE.store(value.to_bits(), Ordering::Release);
    send(Command::DeckFaderTimed(track, value, us));
}
fn fast_fader(value: f32, received: Instant) -> bool {
    let route = SCRATCH_ROUTE.load(Ordering::Acquire);
    if route >= 16 || !runtime().is_some_and(|r| r.active.load(Ordering::Acquire)) {
        return false;
    }
    timed_fader(usize::from(route), value, received);
    true
}
fn fast_cut(on: bool, shift: bool) -> bool {
    if !on {
        let old = SCRATCH_CUT_OWNER.swap(255, Ordering::AcqRel);
        if old < 16 {
            send(Command::DeckCut(usize::from(old), false));
            return true;
        }
        return false;
    }
    if shift || !runtime().is_some_and(|r| r.active.load(Ordering::Acquire)) {
        return false;
    }
    let route = SCRATCH_ROUTE.load(Ordering::Acquire);
    if route >= 16 {
        return false;
    }
    let old = SCRATCH_CUT_OWNER.swap(route, Ordering::AcqRel);
    if old != route {
        if old < 16 {
            send(Command::DeckCut(usize::from(old), false));
        }
        send(Command::DeckCut(usize::from(route), true));
    }
    true
}
pub(crate) fn fast_qa_input(input: crate::qa::Input, received: Instant) -> bool {
    match input {
        crate::qa::Input::Knob {
            index: 20,
            value,
            shift: false,
        } => fast_fader((value * 127.).round() / 127., received),
        crate::qa::Input::Pad {
            index: 1,
            on,
            shift,
            ..
        } => fast_cut(on, shift),
        _ => false,
    }
}
static RUNTIME: OnceLock<Box<Runtime>> = OnceLock::new();
#[derive(Clone, Copy)]
struct HardwarePad {
    pad: usize,
    on: bool,
    velocity: u8,
    shift: bool,
    erase: bool,
}
static PAD_EVENTS: OnceLock<crate::queue::Queue<HardwarePad, 256>> = OnceLock::new();
#[derive(Clone, Copy)]
enum HardwareControl {
    Button {
        code: u32,
        value: u32,
        pressed: bool,
        shift: bool,
        erase: bool,
        timestamp: Instant,
    },
    Cc {
        control: u32,
        value: u32,
        low: f32,
        range: f32,
        shift: bool,
        timestamp: Instant,
    },
    Encoder {
        delta: i32,
        shift: bool,
    },
}
static CONTROL_EVENTS: OnceLock<crate::queue::Queue<HardwareControl, 512>> = OnceLock::new();
static ARM_PENDING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
fn queue_control(control: HardwareControl) {
    if !CONTROL_EVENTS.get().is_some_and(|q| q.push(control))
        && let Some(r) = runtime()
    {
        r.emergency_stop();
    }
}
fn hardware_button(event: ButtonEvent) -> Flow {
    crate::qa::physical_button(event.code, event.pressed, event.shift);
    let active = runtime().is_some_and(|r| r.active.load(Ordering::Acquire));
    if !active {
        if event.code == 6 && event.pressed {
            ARM_PENDING.store(true, Ordering::Release);
        }
        return Flow::Forward;
    }
    if mpc_sample_sdk::loader::with_mod(|module| module.on_button(event)).is_some() {
        return Flow::Consume;
    }
    queue_control(HardwareControl::Button {
        code: event.code,
        value: event.value,
        pressed: event.pressed,
        shift: event.shift,
        erase: event.erase,
        timestamp: Instant::now(),
    });
    Flow::Consume
}
static DATA_CC_PENDING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
fn claim_data_turn(pending: &std::sync::atomic::AtomicBool, delta: i32) -> bool {
    delta != 0 && pending.swap(false, Ordering::AcqRel)
}
thread_local! {static CONTROL_ARRIVAL:std::cell::Cell<Option<Instant>>=const{std::cell::Cell::new(None)};}
fn hardware_cc(event: CcEvent) -> Flow {
    let received = Instant::now();
    if event.control == 20
        && fast_fader(normalize_cc(event.value, event.low, event.range), received)
    {
        return Flow::Consume;
    }
    // DATA CC is the firmware's input to its higher-level encoder handler.
    // Forward it once; our independent encoder hook consumes the resulting turn.
    if event.control == u32::from(mpc_sample_sdk::event::cc::DATA) {
        if runtime().is_some_and(|r| r.active.load(Ordering::Acquire)) {
            DATA_CC_PENDING.store(true, Ordering::Release);
            crate::qa::data_cc();
        }
        return Flow::Forward;
    }
    if !runtime().is_some_and(|r| r.active.load(Ordering::Acquire)) {
        return Flow::Forward;
    }
    if mpc_sample_sdk::loader::with_mod(|module| {
        CONTROL_ARRIVAL.with(|slot| {
            let previous = slot.replace(Some(received));
            let result = module.on_cc(event);
            slot.set(previous);
            result
        })
    })
    .is_some()
    {
        return if event.control == 19 {
            Flow::Forward
        } else {
            Flow::Consume
        };
    }
    queue_control(HardwareControl::Cc {
        control: event.control,
        value: event.value,
        low: event.low,
        range: event.range,
        shift: mpc_sample_sdk::shift(),
        timestamp: received,
    });
    if event.control == 19 {
        Flow::Forward
    } else {
        Flow::Consume
    }
}
fn hardware_encoder(event: EncoderEvent) -> Flow {
    if !runtime().is_some_and(|r| r.active.load(Ordering::Acquire)) {
        return Flow::Forward;
    }
    // The same native setter is also called while restoring project widgets.
    // Only a turn dispatched from the physical DATA CC belongs to our UI.
    if !claim_data_turn(&DATA_CC_PENDING, event.delta) {
        crate::qa::native_encoder_ignored();
        return Flow::Forward;
    }
    crate::qa::encoder(event.delta, true);
    if mpc_sample_sdk::loader::with_mod(|module| module.on_encoder(event)).is_some() {
        return Flow::Consume;
    }
    queue_control(HardwareControl::Encoder {
        delta: normalize_encoder(event.delta),
        shift: mpc_sample_sdk::shift(),
    });
    Flow::Consume
}
static PAD_OWNED: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
fn pad_input(event: PadEvent) -> PadResponse {
    crate::qa::midi_input_trace(
        2,
        event.source as u32,
        event.pad as u8,
        u8::from(event.action == PadAction::Press),
        (event.velocity.clamp(0., 1.) * 127.).round() as u8,
    );
    let physical = mpc_sample_sdk::event::current_surface_event()
        .filter(|s| s.is_pad_event() && (0..16).contains(&s.code))
        .map_or(event.pad, |s| s.code);
    if !(0..16).contains(&physical) {
        return PadResponse {
            flow: Flow::Forward,
            token: 0,
        };
    }
    let Some(r) = runtime() else {
        return PadResponse {
            flow: Flow::Forward,
            token: 0,
        };
    };
    let pad = physical as usize;
    let bit = 1u32 << pad;
    let on = event.action == PadAction::Press;
    if on {
        if !r.active.load(Ordering::Acquire) {
            return PadResponse {
                flow: Flow::Forward,
                token: 0,
            };
        }
        PAD_OWNED.fetch_or(bit, Ordering::AcqRel);
    } else if PAD_OWNED.fetch_and(!bit, Ordering::AcqRel) & bit == 0 {
        return PadResponse {
            flow: Flow::Forward,
            token: 0,
        };
    }
    if pad == 1 && fast_cut(on, mpc_sample_sdk::shift()) {
        return PadResponse {
            flow: Flow::Consume,
            token: 0,
        };
    }
    SCRATCH_ROUTE.store(255, Ordering::Release);
    // Only the SDK's nonblocking exclusive Mod dispatch may consume UI events.
    // Busy/reentrant callbacks retain ownership and enqueue the exact edge below.
    if mpc_sample_sdk::loader::with_mod(|module| {
        module.on_pad(PadEvent {
            pad: physical,
            ..event
        })
    })
    .is_some()
    {
        return PadResponse {
            flow: Flow::Consume,
            token: 0,
        };
    }
    let queued = PAD_EVENTS.get().is_some_and(|q| {
        q.push(HardwarePad {
            pad,
            on,
            velocity: (event.velocity.clamp(0., 1.) * 127.).round() as u8,
            shift: mpc_sample_sdk::shift(),
            erase: mpc_sample_sdk::erase(),
        })
    });
    if !queued {
        r.emergency_stop();
    }
    PadResponse {
        flow: Flow::Consume,
        token: 0,
    }
}
static IO_REQUEST: AtomicU32 = AtomicU32::new(0);
static IO_STATUS: AtomicU8 = AtomicU8::new(0);
fn runtime() -> Option<&'static Runtime> {
    RUNTIME.get().map(Box::as_ref)
}
#[cfg(test)]
thread_local! {
    static AUDIT_COMMANDS: std::cell::RefCell<Option<Vec<Command>>> = const { std::cell::RefCell::new(None) };
}
fn send(c: Command) {
    #[cfg(test)]
    AUDIT_COMMANDS.with(|commands| {
        if let Some(commands) = commands.borrow_mut().as_mut() {
            commands.push(c);
        }
    });
    if let Some(r) = runtime() {
        r.enqueue(c)
    }
}
#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
pub(crate) fn sequencer_note(source: u32, note: u8, velocity: u8, on: bool) {
    if let Some(r) = runtime() {
        r.external_note(source, note, velocity, on);
    }
}
fn note(event: &mut NoteEvent<'_>) -> Flow {
    crate::qa::midi_input_trace(
        1,
        event.source as u32,
        event.note,
        u8::from(event.note_on) | (u8::from(event.note_off) << 1) | (u8::from(event.external) << 2),
        event.kind,
    );
    let Some(r) = runtime() else {
        return Flow::Forward;
    };
    if !event.external || event.note > 127 || !(event.note_on || event.note_off) {
        return Flow::Forward;
    }
    // Native firmware1.3.0.12 forwards only note-on to this upper hook.
    // ALSA supplies complete note ownership; do not also play its upper replay.
    if crate::midi_input::observed() && r.active.load(Ordering::Acquire) {
        return Flow::Consume;
    }
    if r.external_note(
        event.source as u32,
        event.note,
        (event.velocity.clamp(0., 1.) * 127.).round() as u8,
        event.note_on,
    ) {
        Flow::Consume
    } else {
        Flow::Forward
    }
}

fn input_interleaved(samples: &[i32], channels: usize, rate: u32) {
    // The pinned Sample's measured USB input period is24, analog ADC is128.
    // SDK taps both raw readers: never append USB return to the analog timeline.
    if channels == 0
        || !crate::input_hw::accepts_period(
            crate::input_hw::selected(),
            mpc_sample_sdk::audio::input_period(),
        )
    {
        return;
    }
    if let Some(r) = runtime() {
        for frame in samples.chunks_exact(channels) {
            r.capture(
                Frame {
                    l: frame[0] as f32 / 2147483648.,
                    r: frame[channels.min(2) - 1] as f32 / 2147483648.,
                },
                rate,
            )
        }
    }
}
fn input_planar(channels: &[&[i32]], rate: u32) {
    if !crate::input_hw::accepts_period(
        crate::input_hw::selected(),
        mpc_sample_sdk::audio::input_period(),
    ) {
        return;
    }
    let Some(first) = channels.first() else {
        return;
    };
    let second = channels.get(1).unwrap_or(first);
    if let Some(r) = runtime() {
        for (l, rr) in first.iter().zip(second.iter()) {
            r.capture(
                Frame {
                    l: *l as f32 / 2147483648.,
                    r: *rr as f32 / 2147483648.,
                },
                rate,
            )
        }
    }
}
fn output(samples: &mut [i32], info: &mpc_sample_sdk::audio::OutputInfo) {
    let Some(r) = runtime() else { return };
    if !(8000..=96000).contains(&info.rate()) || info.channels() == 0 {
        return;
    }
    let old = r.pcm.load(Ordering::Acquire);
    // The connected Sample's measured I2S period is128; USB uses24.
    // Never let an early USB writer become the Engine clock source.
    if old == 0 && info.period_out() >= 64 {
        let _ = r
            .pcm
            .compare_exchange(0, info.pcm(), Ordering::AcqRel, Ordering::Relaxed);
    }
    crate::qa::output_probe(info, r.pcm.load(Ordering::Acquire) == info.pcm());
    if r.pcm.load(Ordering::Acquire) != info.pcm() {
        if r.active.load(Ordering::Acquire) && r.rate.load(Ordering::Acquire) == info.rate() {
            r.output_mirror.add(info.pcm(), samples, info.channels());
            crate::qa::output_peak(info.pcm(), samples);
        }
        return;
    }
    r.rate.store(info.rate(), Ordering::Release);
    let started = crate::qa::output_start();
    r.output(samples, info.channels(), info.rate());
    crate::qa::output_finish(started, info.block_frames(), info.rate());
    crate::qa::output_peak(info.pcm(), samples);
}
fn worker() {
    let Some(r) = runtime() else { return };
    while r.rate.load(Ordering::Acquire) == 0 {
        std::thread::sleep(Duration::from_millis(20));
    }
    let rate = r.rate.load(Ordering::Acquire);
    let capacity = rate as usize * 16;
    let dir = std::env::var_os("LOOP_CLUB_DATA_DIR")
        .map(std::path::PathBuf::from)
        .filter(|p| p.is_absolute())
        .map(|path| crate::project::session_directory(&path, crate::qa::enabled()));
    let mut initial = Engine::new_project(rate, capacity);
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u32;
    for (i, t) in initial.tracks.iter_mut().enumerate() {
        t.pattern.seed = seed ^ (i as u32 + 1).wrapping_mul(0x9e3779b9);
    }

    if let Some(dir) = &dir {
        let path = crate::project_manager::startup_path(dir);
        if path.is_file() {
            match crate::project::load(&path, rate, capacity) {
                Ok(loaded) => {
                    initial = loaded;
                    IO_STATUS.store(2, Ordering::Release);
                }
                Err(_) => IO_STATUS.store(4, Ordering::Release),
            }
        }
    }
    let _ = r.initialize(Box::new(initial));
    if let Some(dir) = &dir
        && std::fs::create_dir_all(dir).is_ok()
    {
        let report = serde_json::json!({"product":"Loop Club","version":"0.1.0","output_rate":rate,"sdk_hooks":"loader fingerprint check passed before on_start","scope":"startup evidence only; physical controls/audio/CPU unverified"});
        if let Ok(bytes) = serde_json::to_vec_pretty(&report) {
            let _ = std::fs::write(dir.join("startup.json"), bytes);
        }
    }
    let mut projects = crate::project_manager::Worker::new(dir.clone());
    let mut input = crate::input_hw::InputWorker::new(dir.as_deref());
    let mut files = crate::browser_device::FileWorker::new(rate, capacity);
    loop {
        std::thread::sleep(Duration::from_millis(20));
        crate::qa::tick();
        input.tick(r.active.load(Ordering::Acquire));
        if let Some(ok) = files.tick(r) {
            let builtin = files.take_builtin_loaded().is_some();
            IO_STATUS.store(
                if ok && builtin {
                    11
                } else if ok {
                    5
                } else {
                    4
                },
                Ordering::Release,
            );
        }
        // Finish previously queued sample/kit imports before replacing a project.
        if !files.pending() {
            projects.tick(r, rate, capacity);
        }
        let action = IO_REQUEST.swap(0, Ordering::AcqRel);
        if action == 0 {
            continue;
        }
        if action & 8 != 0 {
            if files.pending() {
                let _ = IO_REQUEST.compare_exchange(0, action, Ordering::AcqRel, Ordering::Relaxed);
            } else if files
                .load_builtin(r, ((action >> 4) & 15) as usize)
                .is_err()
            {
                IO_STATUS.store(4, Ordering::Release);
            }
            continue;
        }
        if action & 4 != 0 {
            let source = ((action >> 4) & 15) as usize;
            let target = ((action >> 8) & 15) as usize;
            let moving = action & (1 << 12) != 0;
            let result = r.with_engine(|slot| {
                let Some(e) = slot.as_mut() else { return 4 };
                let status = match e.transfer_track_stopped(source, target, moving) {
                    Ok(()) => {
                        if moving {
                            7
                        } else {
                            6
                        }
                    }
                    Err(Error::Busy) => 3,
                    Err(Error::NoContent) => 9,
                    Err(Error::Capacity) => 10,
                    Err(_) => 8,
                };
                r.snapshot.publish(&Snapshot::capture(e));
                status
            });
            if let Some(status) = result {
                IO_STATUS.store(status, Ordering::Release);
            } else {
                let _ = IO_REQUEST.compare_exchange(0, action, Ordering::AcqRel, Ordering::Relaxed);
            }
            continue;
        }
    }
}
#[derive(Default)]
pub struct LoopClub {
    browser: crate::browser_device::BrowserUi,
    projects: crate::project_manager::Manager,
    drum_preview_request: Option<(usize, Option<u64>)>,
    ui: Ui,
    snapshot: Snapshot,
    armed: bool,
    exit_requested: bool,
    button_timestamp: Option<Instant>,
    tap_epoch: Option<Instant>,
    tap: crate::tempo::TapTempo,
    draining_inputs: bool,
    control_timestamp: Option<Instant>,
}
impl LoopClub {
    /// Consume fallback edges under the SDK's exclusive Mod lock. Input callbacks
    /// and paint are serialized by that lock; never called by an audio worker.
    fn drain_pending_inputs(&mut self) {
        self.drain_pending_sources(CONTROL_EVENTS.get(), PAD_EVENTS.get());
    }
    fn drain_pending_sources(
        &mut self,
        controls: Option<&crate::queue::Queue<HardwareControl, 512>>,
        pads: Option<&crate::queue::Queue<HardwarePad, 256>>,
    ) {
        if self.draining_inputs {
            return;
        }
        self.draining_inputs = true;
        if let Some(events) = controls {
            for _ in 0..512 {
                // SAFETY: serialized owning UI callbacks are the only consumer.
                let Some(event) = (unsafe { events.pop_shared() }) else {
                    break;
                };
                match event {
                    HardwareControl::Button {
                        code,
                        value,
                        pressed,
                        shift,
                        erase,
                        timestamp,
                    } => {
                        self.button_timestamp = Some(timestamp);
                        self.on_button(ButtonEvent {
                            event: 0,
                            bytes: [0; 16],
                            code,
                            value,
                            pressed,
                            shift,
                            erase,
                            surface: 0,
                        });
                    }
                    HardwareControl::Cc {
                        control,
                        value,
                        low,
                        range,
                        shift,
                        timestamp,
                    } => {
                        self.control_timestamp = Some(timestamp);
                        self.control_cc(
                            CcEvent {
                                event: 0,
                                control,
                                value,
                                low,
                                range,
                            },
                            shift,
                        );
                    }
                    HardwareControl::Encoder { delta, shift } => {
                        self.refresh();
                        if self.projects.open {
                            self.projects.encoder(delta);
                        } else if self.browser.open {
                            self.browser.encoder(delta);
                        } else {
                            self.ui.shift_view = shift;
                            self.ui.encoder(delta, &self.snapshot, send);
                        }
                    }
                }
            }
        }
        if let Some(events) = pads {
            for _ in 0..256 {
                // SAFETY: exclusive SDK Mod callbacks are the only consumer.
                let Some(event) = (unsafe { events.pop_shared() }) else {
                    break;
                };
                if (self.browser.open || self.projects.open) && event.on {
                    continue;
                }
                let action = self.ui.pad(
                    event.pad,
                    event.on,
                    event.velocity,
                    event.shift,
                    event.erase,
                    &self.snapshot,
                    send,
                );
                self.action(action);
                if !self.ui.active {
                    break;
                }
            }
        }
        self.draining_inputs = false;
    }
    fn control_cc(&mut self, event: CcEvent, shift: bool) -> Flow {
        let received = self
            .control_timestamp
            .take()
            .or_else(|| CONTROL_ARRIVAL.with(std::cell::Cell::get))
            .unwrap_or_else(Instant::now);
        if event.control == u32::from(mpc_sample_sdk::event::cc::DATA) {
            return Flow::Forward;
        }
        if !self.ui.active {
            return Flow::Forward;
        }
        self.refresh();
        if event.control == 19 {
            // Our signal is added after the stock mix: keep native volume for
            // that mix, and control our injected signal independently of the
            // project master fader. `low` is not the current knob value.
            send(Command::HardwareVolume(normalize_cc(
                event.value,
                event.low,
                event.range,
            )));
            return Flow::Forward;
        }
        if self.browser.open || self.projects.open {
            return Flow::Consume;
        }
        self.ui.shift_view = shift;
        // SDK supplies the raw controller value and firmware normalization geometry.
        let value = normalize_cc(event.value, event.low, event.range);
        if self.ui.page == Page::Play
            && self.ui.synth_page == 0
            && self.snapshot.track_kind(self.ui.selected) == Kind::Synth
            && self.ui.chord_modes[self.ui.selected]
            && shift
            && (16..=18).contains(&event.control)
        {
            match event.control {
                16 => send(Command::ChordValue {
                    track: self.ui.selected,
                    index: 3,
                    value: (value * 4.99).floor() - 2.,
                }),
                17 => send(Command::ChordValue {
                    track: self.ui.selected,
                    index: 4,
                    value,
                }),
                _ => send(Command::SynthValue {
                    track: self.ui.selected,
                    index: 2,
                    value: 0.02 + value * 3.,
                }),
            }
            return Flow::Consume;
        }
        if self.ui.page == Page::Play
            && self.ui.synth_page == 0
            && self.snapshot.track_kind(self.ui.selected) == Kind::Synth
            && shift
            && (16..=18).contains(&event.control)
        {
            let (index, value) = match event.control {
                16 => (3, (value * 3.99).floor()),
                17 => (4, value * 0.02),
                _ => (5, (value * 4.99).floor() - 2.),
            };
            send(Command::SynthValue {
                track: self.ui.selected,
                index,
                value,
            });
            return Flow::Consume;
        }
        match event.control {
            16..=18 => self
                .ui
                .knob((event.control - 16) as usize, value, &self.snapshot, send),
            20 => {
                FADER_VALUE.store(value.to_bits(), Ordering::Release);
                self.ui.fader(value, &self.snapshot, |command| {
                    if let Command::DeckFader(i, v) = command {
                        timed_fader(i, v, received);
                    } else {
                        send(command);
                    }
                });
            }
            _ => {}
        }
        Flow::Consume
    }
    fn refresh(&mut self) {
        if self.projects.refresh() {
            let active = self.ui.active;
            self.ui = Ui::default();
            self.ui.active = active;
            SCRATCH_CUT_OWNER.store(255, Ordering::Release);
            SCRATCH_ROUTE.store(255, Ordering::Release);
            self.tap.reset();
        }
        let value = f32::from_bits(FADER_VALUE.load(Ordering::Acquire));
        if value.is_finite() {
            self.ui.last_fader = Some(value);
        }
        if let Some(s) = runtime().and_then(|r| r.snapshot.read()) {
            if s.words[0] == 0
                && self.snapshot.words[0] > 0
                && s.words[4] & 1 == 0
                && s.words[9] == 0
            {
                self.ui.grid_bar = 0;
                self.ui.grid_event = 0;
            }
            self.snapshot = s;
        }
        if self.projects.take_return_to_loop() {
            self.ui.loop_mode = 0;
            self.ui.loop_edit = false;
            self.ui.edit_source = None;
            self.ui.navigate(Page::Loop, &self.snapshot);
        }
        if runtime().is_some() {
            let bar = if self.ui.page == Page::Grid
                && self.snapshot.track_kind(self.ui.selected) == Kind::Drum
            {
                Some(self.ui.grid_bar)
            } else {
                None
            };
            let request = (self.ui.selected, bar);
            if self.drum_preview_request != Some(request) {
                if bar.is_some() {
                    self.ui.grid_event = 0;
                }
                send(Command::DrumPreviewBar(
                    bar.map(|bar| (self.ui.selected, bar)),
                ));
                self.drum_preview_request = Some(request);
            }
        }
        self.ui.sync_chord_modes(&self.snapshot);
        self.ui.sync_loop_role(&self.snapshot);
        if self.ui.page == Page::Chop {
            let count = ((self.snapshot.words[506] >> 16) & 255).clamp(1, 16) as usize;
            self.ui.chop_slice = self.ui.chop_slice.min(count - 1);
        }
        let status = IO_STATUS.swap(0, Ordering::AcqRel);
        if status == 5 || status == 11 {
            self.browser.open = false;
            let page = if status == 11 {
                Page::Play
            } else if self.browser.target.is_some() {
                Page::DrumSound
            } else if self.snapshot.track_kind(self.ui.selected) == Kind::Drum {
                Page::Play
            } else {
                Page::Loop
            };
            self.ui.navigate(page, &self.snapshot);
        }
        self.browser.refresh();
        if status != 0 {
            crate::qa::io_result(status);
            self.ui.notice = match status {
                1 => "SESSION SAVED",
                2 => "SESSION LOADED",
                3 => "STOP / RELEASE NOTES FIRST",
                6 => "COPIED / SHIFT -: UNDO",
                7 => "MOVED / SHIFT -: UNDO",
                8 => "DESTINATION HAS CONTENT",
                9 => "SOURCE HAS NO CONTENT",
                10 => "KIT MEMORY LIMIT",
                11 => "ORIGINAL AHH / PAD 1 GRAB",
                5 if self.browser.target.is_some() => "LAYER SAMPLE LOADED",
                5 if self.snapshot.track_kind(self.ui.selected) == Kind::Drum => "KIT LOADED",
                5 => "SAMPLE LOADED",
                _ => "PROJECT IO ERROR",
            };
        }
    }
    fn leave(&mut self) {
        SCRATCH_ROUTE.store(255, Ordering::Release);
        let owner = SCRATCH_CUT_OWNER.swap(255, Ordering::AcqRel);
        if owner < 16 {
            send(Command::DeckCut(usize::from(owner), false));
        }
        self.ui.release_deck_latch(send);
        if let Some(r) = runtime() {
            r.emergency_stop();
            r.active.store(false, Ordering::Release)
        }
        self.ui.active = false;
        self.armed = false;
        self.exit_requested = true;
        for pad in 0..16 {
            self.ui
                .pad(pad, false, 0, false, false, &self.snapshot, send);
        }
        // Keep ownership until each physical release has been consumed.
    }
    fn action(&mut self, action: Action) {
        match action {
            Action::Transfer {
                source,
                target,
                moving,
            } => {
                let request =
                    4 | ((source as u32) << 4) | ((target as u32) << 8) | (u32::from(moving) << 12);
                self.ui.notice = if IO_REQUEST
                    .compare_exchange(0, request, Ordering::AcqRel, Ordering::Relaxed)
                    .is_ok()
                {
                    "LOOP EDIT WORKING..."
                } else {
                    "PROJECT BUSY"
                };
            }
            Action::InputSource(source) => {
                if self.snapshot.words[9] != 0 || self.snapshot.words[4] & 1 != 0 {
                    self.ui.notice = "STOP BEFORE CHANGING INPUT";
                } else if !crate::input_hw::request(source) {
                    self.ui.notice = "INPUT SWITCH BUSY";
                } else {
                    self.ui.notice = "";
                }
            }
            Action::DefaultScratch => {
                if self.snapshot.words[9] != 0 {
                    self.ui.notice = "FINISH RECORDING FIRST";
                } else if IO_REQUEST
                    .compare_exchange(
                        0,
                        8 | ((self.ui.selected as u32) << 4),
                        Ordering::AcqRel,
                        Ordering::Relaxed,
                    )
                    .is_ok()
                {
                    self.ui.notice = "PREPARING ORIGINAL AHH...";
                } else {
                    self.ui.notice = "PROJECT BUSY";
                }
            }
            Action::BrowseXpm => {
                if self.snapshot.track_kind(self.ui.selected) != Kind::Drum {
                    self.ui.notice = "SELECT A DRUM TRACK FIRST";
                } else {
                    self.browser.start_xpm(self.ui.selected);
                }
            }
            Action::Browse => {
                if self.ui.page == Page::DrumSound {
                    self.browser
                        .start_layer(self.ui.selected, self.ui.sound, self.ui.layer);
                    return;
                }
                self.browser.target = None;
                if self.snapshot.track_kind(self.ui.selected) == Kind::Audio
                    && self.snapshot.words[16 + self.ui.selected * 12 + 4] == 0
                {
                    self.browser.start(self.ui.selected)
                } else if self.snapshot.track_kind(self.ui.selected) == Kind::Drum {
                    self.browser.start_kind(self.ui.selected, true);
                } else if self.snapshot.track_kind(self.ui.selected) == Kind::Audio {
                    self.ui.notice = "USE EMPTY AUDIO TRACK";
                } else {
                    self.ui.notice = "SELECT AUDIO OR DRUM TRACK";
                }
            }
            Action::Save | Action::Load | Action::NewProject | Action::Projects => {
                use crate::project_manager::Operation;
                if self.snapshot.words[9] != 0 {
                    self.ui.notice = "FINISH RECORDING FIRST";
                } else if IO_REQUEST.load(Ordering::Acquire) != 0 || crate::input_hw::pending() {
                    self.ui.notice = "PROJECT BUSY";
                } else {
                    self.projects.start(match action {
                        Action::Save => Operation::Save,
                        Action::Load => Operation::Open(0),
                        Action::NewProject => Operation::New,
                        _ => Operation::Scan,
                    });
                }
            }
            Action::Exit => self.leave(),
            Action::None => {}
        }
    }
}
struct NativePainter(mpc_sample_sdk::draw::Graphics);
impl Painter for NativePainter {
    fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32) {
        if w <= 0 || h <= 0 {
            return;
        } // SAFETY: wrapper is created only inside on_paint, never retained or sent.
        unsafe {
            self.0.colour(color);
            self.0.rect(x, y, w, h)
        }
    }
    fn text(&mut self, x: i32, y: i32, w: i32, h: i32, size: f32, color: u32, text: &str) {
        // SAFETY: same live UI-thread Graphics callback lifetime as rect.
        unsafe {
            self.0.colour(color);
            self.0.text(text, x, y, w, h, size)
        }
    }
}
impl Mod for LoopClub {
    fn on_start(&mut self) {
        let _ = FADER_EPOCH.set(Instant::now());
        let _ = RUNTIME.set(Box::new(Runtime::default()));
        self.armed = true;
        crate::qa::start();
        let _ = PAD_EVENTS.set(crate::queue::Queue::default());
        let _ = CONTROL_EVENTS.set(crate::queue::Queue::default());
        crate::browser_device::initialize();
        crate::project_manager::initialize();
        let _ = std::thread::Builder::new()
            .name("loopclub-project".into())
            .spawn(worker);
    }
    fn input(&self) -> InputHooks {
        InputHooks {
            button: Some(hardware_button),
            cc: Some(hardware_cc),
            encoder: Some(hardware_encoder),
            surface: Some(crate::qa::surface),
            note: Some(note),
            pad: Some(pad_input),
            ..InputHooks::default()
        }
    }
    fn audio(&self) -> AudioHooks {
        AudioHooks {
            output: Some(output),
            input_interleaved: Some(input_interleaved),
            input_planar: Some(input_planar),
        }
    }
    fn on_button(&mut self, event: ButtonEvent) -> Flow {
        SCRATCH_ROUTE.store(255, Ordering::Release);
        self.drain_pending_inputs();
        crate::qa::button();
        let timestamp = self.button_timestamp.take().unwrap_or_else(Instant::now);
        if !self.ui.active {
            if event.pressed && event.code == 6 {
                self.armed = true;
                self.exit_requested = false;
            }
            return Flow::Forward;
        }
        if !event.pressed {
            return Flow::Consume;
        }
        self.refresh();
        if event.code == 53 {
            // measured TAP TEMPO physical press/release, firmware1.3.0.12
            if event.shift {
                self.tap.reset();
                send(Command::ToggleMetronome);
                self.ui.notice = if self.snapshot.words[4] & (1 << 16) == 0 {
                    "METRONOME ON"
                } else {
                    "METRONOME OFF"
                };
            } else {
                let epoch = *self.tap_epoch.get_or_insert(timestamp);
                let elapsed = timestamp
                    .checked_duration_since(epoch)
                    .unwrap_or_default()
                    .as_millis() as u64;
                if let Some(bpm) = self.tap.tap(elapsed) {
                    send(Command::Bpm(bpm));
                    self.ui.notice = "TAP TEMPO";
                } else {
                    self.ui.notice = "TAP AGAIN TO SET BPM";
                }
            }
            return Flow::Consume;
        }
        if self.projects.open {
            if event.code == 81 {
                self.ui.release_deck_latch(send);
                send(Command::Stop);
            } else {
                self.projects.button(event.code as u8, event.shift);
                if !self.projects.open && matches!(event.code, 6 | 74 | 14) {
                    self.ui.navigate(Page::Loop, &self.snapshot);
                }
            }
            return Flow::Consume;
        }
        if self.browser.open {
            if event.code == 81 {
                send(Command::Stop)
            } else {
                self.browser.button_with_shift(event.code, event.shift)
            }
            return Flow::Consume;
        }
        if self.ui.page == Page::Setup
            && !self.ui.input_view
            && !event.shift
            && matches!(event.code, 54 | 55)
        {
            self.ui.setup_timing = !self.ui.setup_timing;
            self.ui.notice = "";
            return Flow::Consume;
        }
        if event.shift && matches!(event.code, 54 | 55) {
            send(if event.code == 55 {
                Command::Undo
            } else {
                Command::Redo
            });
            self.ui.notice = if event.code == 55 { "UNDO" } else { "REDO" };
            return Flow::Consume;
        }
        if self.ui.page == Page::Setup && self.ui.input_view {
            match event.code {
                12 => {
                    self.ui.back(&self.snapshot);
                    return Flow::Consume;
                }
                13 | 111 => {
                    self.action(Action::InputSource(self.ui.input_choice as u8));
                    return Flow::Consume;
                }
                14 => {
                    self.ui.navigate(Page::Loop, &self.snapshot);
                    return Flow::Consume;
                }
                _ => {}
            }
        }
        if self
            .ui
            .pad_notes_button(event.code, event.shift, &self.snapshot)
        {
            return Flow::Consume;
        }
        if self
            .ui
            .loop_edit_button(event.code, event.shift, &self.snapshot)
        {
            return Flow::Consume;
        }
        match event.code {
            13 if self.ui.page == Page::Setup && !self.ui.setup_timing && !self.ui.input_view => {
                self.action(Action::Projects)
            }
            6 if event.shift => {
                self.ui.navigate(Page::Setup, &self.snapshot);
                self.ui.input_view = true;
                self.ui.input_choice = usize::from(crate::input_hw::selected());
            }
            14 if self.ui.page == Page::Source => self.action(Action::Browse),
            13 if self.ui.page == Page::Loop
                && !event.shift
                && !self.ui.loop_edit
                && self.ui.pad_notes_edit.is_none()
                && self.ui.loop_mode != 2
                && self.snapshot.track_kind(self.ui.selected) == Kind::Audio
                && self.snapshot.words[9] == self.ui.selected as u64 + 1
                && self.snapshot.words[12] & (1 << 4) == 0 =>
            {
                self.ui.notice = "AUDIO REC / SHIFT+PAD: END";
            }
            13 if self.ui.page == Page::Loop
                && !event.shift
                && !self.ui.loop_edit
                && self.ui.pad_notes_edit.is_none()
                && self.ui.loop_mode != 2
                && self.snapshot.track_kind(self.ui.selected) == Kind::Audio
                && self.snapshot.words[16 + self.ui.selected * 12 + 11] == 0 =>
            {
                self.action(Action::Browse)
            }
            12..=14 => self.ui.soft(
                (event.code - 12) as usize,
                event.shift,
                &self.snapshot,
                send,
            ),
            6 | 74 => {
                self.ui.loop_edit = false;
                self.ui.edit_source = None;
                self.ui.loop_mode = 0;
                self.ui.navigate(Page::Loop, &self.snapshot);
            }
            7 if event.shift => self.action(Action::Projects),
            7 => self.ui.navigate(Page::Grid, &self.snapshot),
            40 => self.ui.navigate(Page::Play, &self.snapshot),
            36 if event.shift => self.action(Action::BrowseXpm),
            36 => self.action(Action::Browse),
            75 => {
                if event.shift {
                    send(Command::SliceGate(self.ui.selected))
                } else {
                    self.ui.navigate(Page::Chop, &self.snapshot)
                }
            }
            1 => self.ui.navigate(Page::Mix, &self.snapshot),
            0 => self.ui.navigate(Page::TrackFx, &self.snapshot),
            111 => {
                if self.ui.page == Page::Setup && !event.shift {
                    self.ui.navigate(Page::Loop, &self.snapshot);
                } else if self.ui.page == Page::Loop && self.ui.loop_edit && !event.shift {
                    self.ui.loop_edit = false;
                    self.ui.edit_source = None;
                    self.ui.notice = "";
                } else if matches!(
                    self.ui.page,
                    Page::TrackFx | Page::MasterFx | Page::Texture | Page::FxSettings
                ) && !event.shift
                {
                    self.ui.open_fx_settings(&self.snapshot);
                } else if self.ui.page == Page::Loop && self.ui.loop_mode == 2 && !event.shift {
                    send(Command::ChainToggle);
                } else if self.ui.page == Page::Sidechain && !event.shift {
                    self.ui.navigate(Page::MasterFx, &self.snapshot);
                } else if self.ui.page == Page::Output && !event.shift {
                    self.ui.back(&self.snapshot);
                } else if self.ui.page == Page::Mix && !event.shift {
                    self.ui.navigate(Page::Output, &self.snapshot);
                } else if self.ui.page == Page::DrumSound {
                    if event.shift {
                        self.ui.layer_detail = !self.ui.layer_detail;
                    } else {
                        self.action(Action::Browse);
                    }
                } else if self.ui.page == Page::Chop && !event.shift {
                    send(Command::SplitSlice {
                        track: self.ui.selected,
                        pad: self.ui.chop_slice,
                        at: self.ui.chop_cursor,
                    });
                } else if event.shift {
                    self.ui.navigate(Page::Setup, &self.snapshot);
                    self.ui.setup_timing = true;
                } else {
                    if self.snapshot.words[16 + self.ui.selected * 12 + 4] > 0 {
                        send(Command::Pad(self.ui.selected, false));
                    } else {
                        self.ui.notice = "PRESS REC TO RECORD";
                    }
                }
            }
            81 => {
                self.ui.release_deck_latch(send);
                send(Command::Stop);
            }
            82 => send(Command::PlayAll),
            73 | 80 => {
                if self.ui.loop_edit {
                    self.ui.notice = "EXIT EDIT BEFORE RECORDING";
                    return Flow::Consume;
                }
                if crate::input_hw::pending() {
                    self.ui.notice = "WAIT FOR INPUT SWITCH";
                    return Flow::Consume;
                }
                let track = if self.snapshot.words[9] > 0 {
                    (self.snapshot.words[9] - 1) as usize
                } else {
                    self.ui.selected
                };
                if self.snapshot.words[9] == 0 && self.ui.page == Page::Loop {
                    self.ui.loop_mode = 0;
                    self.ui.loop_perform = true;
                    self.ui.loop_record_seen = false;
                    self.ui.loop_play_track = track;
                    self.ui.loop_start_revision = self.snapshot.words[1047];
                }
                send(Command::LoopRecord(track));
            }
            54 => {
                if self.ui.page == Page::DrumSound {
                    self.ui.layer_detail = !self.ui.layer_detail;
                } else if self.ui.page == Page::Chop {
                    send(Command::SplitSlice {
                        track: self.ui.selected,
                        pad: self.ui.chop_slice,
                        at: self.ui.chop_cursor,
                    });
                } else if self.ui.page == Page::Play
                    && self.snapshot.track_kind(self.ui.selected) == Kind::Synth
                {
                    self.ui.synth_page =
                        (self.ui.synth_page + 1) % crate::ui::synth_page_count(&self.snapshot);
                } else if self.ui.page == Page::Grid {
                    self.ui.grid_bar = self.ui.grid_bar.saturating_add(1).min(4095)
                } else {
                    self.ui.encoder(1, &self.snapshot, send)
                }
            }
            55 => {
                if self.ui.page == Page::DrumSound {
                    self.ui.layer_detail = !self.ui.layer_detail;
                } else if self.ui.page == Page::Chop {
                    send(if event.shift {
                        Command::ChopUndo(self.ui.selected)
                    } else {
                        Command::DeleteSlice {
                            track: self.ui.selected,
                            pad: self.ui.chop_slice,
                        }
                    });
                } else if self.ui.page == Page::Play
                    && self.snapshot.track_kind(self.ui.selected) == Kind::Synth
                {
                    let pages = crate::ui::synth_page_count(&self.snapshot);
                    self.ui.synth_page = (self.ui.synth_page + pages - 1) % pages;
                } else if self.ui.page == Page::Grid {
                    self.ui.grid_bar = self.ui.grid_bar.saturating_sub(1)
                } else {
                    self.ui.encoder(-1, &self.snapshot, send)
                }
            }
            _ => {}
        }
        Flow::Consume
    }
    fn on_encoder(&mut self, event: EncoderEvent) -> Flow {
        SCRATCH_ROUTE.store(255, Ordering::Release);
        self.drain_pending_inputs();
        if !self.ui.active {
            return Flow::Forward;
        }
        self.refresh();
        let delta = normalize_encoder(event.delta);
        if self.projects.open {
            self.projects.encoder(delta);
            return Flow::Consume;
        }
        if self.browser.open {
            self.browser.encoder(delta);
            return Flow::Consume;
        }
        self.ui.shift_view = mpc_sample_sdk::shift();
        self.ui.encoder(delta, &self.snapshot, send);
        Flow::Consume
    }
    fn on_pad(&mut self, event: PadEvent) -> Flow {
        SCRATCH_ROUTE.store(255, Ordering::Release);
        self.drain_pending_inputs();
        self.refresh();
        if !(0..16).contains(&event.pad) {
            return Flow::Forward;
        }
        let on = event.action == PadAction::Press;
        if !self.ui.active {
            return Flow::Consume;
        }
        if !((self.browser.open || self.projects.open) && on) {
            let action = self.ui.pad(
                event.pad as usize,
                on,
                (event.velocity.clamp(0., 1.) * 127.).round() as u8,
                mpc_sample_sdk::shift(),
                mpc_sample_sdk::erase(),
                &self.snapshot,
                send,
            );
            self.action(action);
        }
        Flow::Consume
    }
    fn on_cc(&mut self, event: CcEvent) -> Flow {
        self.drain_pending_inputs();
        self.control_cc(event, mpc_sample_sdk::shift())
    }
    fn on_paint(&mut self, graphics: &mut mpc_sample_sdk::draw::Graphics, event: PaintEvent) {
        if !event.sample_page {
            return;
        }
        if ARM_PENDING.swap(false, Ordering::AcqRel) {
            self.armed = true;
            self.exit_requested = false;
        }
        if self.armed && !self.exit_requested {
            self.ui.active = true;
            self.armed = false;
            if let Some(r) = runtime() {
                r.active.store(true, Ordering::Release)
            }
        }
        if !self.ui.active {
            return;
        }
        self.refresh();
        self.ui.shift_view = mpc_sample_sdk::shift();
        self.drain_pending_inputs();
        for _ in 0..64 {
            let Some(input) = crate::qa::pop() else {
                break;
            };
            let received = input.received;
            match input.input {
                crate::qa::Input::TransportButton {
                    code,
                    on,
                    shift,
                    erase,
                } => {
                    // Deliberately runs while the Mod paint mutex is held: this
                    // exercises the same independent callback as hardware, without
                    // pretending that an injected event is a physical key press.
                    crate::qa::injected_control(|| {
                        hardware_button(ButtonEvent {
                            event: 0,
                            bytes: [0; 16],
                            code,
                            value: if on { 127 } else { 0 },
                            pressed: on,
                            shift,
                            erase,
                            surface: 0,
                        })
                    });
                }
                crate::qa::Input::Button { code, on, shift } => {
                    crate::qa::remote_button();
                    self.on_button(ButtonEvent {
                        event: 0,
                        bytes: [0; 16],
                        code,
                        value: if on { 127 } else { 0 },
                        pressed: on,
                        shift,
                        erase: false,
                        surface: 0,
                    });
                }
                crate::qa::Input::Knob {
                    index,
                    value,
                    shift,
                } => {
                    self.control_timestamp = Some(received);
                    self.control_cc(
                        CcEvent {
                            event: 0,
                            control: index,
                            value: (value * 127.).round() as u32,
                            low: value,
                            range: 1.,
                        },
                        shift,
                    );
                }
                crate::qa::Input::Encoder { delta } => {
                    crate::qa::encoder(delta, false);
                    self.on_encoder(EncoderEvent { surface: 0, delta });
                }
                crate::qa::Input::Pad {
                    index,
                    on,
                    velocity,
                    shift,
                    erase,
                } => {
                    if (self.browser.open || self.projects.open) && on {
                        continue;
                    }
                    let action =
                        self.ui
                            .pad(index, on, velocity, shift, erase, &self.snapshot, send);
                    self.action(action);
                }
            }
        }
        self.drain_pending_inputs();
        self.refresh();
        SCRATCH_ROUTE.store(
            scratch_route(
                &self.ui,
                &self.snapshot,
                self.browser.open || self.projects.open,
            ),
            Ordering::Release,
        );
        crate::qa::browser(self.browser.open, self.browser.row, self.browser.total());
        crate::qa::paint(&self.ui, &self.snapshot, self.browser.open);
        crate::project_manager::observe(&self.projects);
        if !self.ui.active {
            return;
        }
        if let Some(command) = turntable_wave_reset(&self.ui, &self.snapshot) {
            send(command);
        }
        if self.projects.open {
            self.projects.render(&mut NativePainter(*graphics))
        } else if self.browser.open {
            self.browser.render(&mut NativePainter(*graphics))
        } else {
            view::render(&self.ui, &self.snapshot, &mut NativePainter(*graphics));
        }
        // Inputs that arrived while native drawing owned the Mod must not wait
        // through another complete paint before their audio command is sent.
        self.drain_pending_inputs();
        // SAFETY: request the next asynchronous repaint only from this live paint
        // callback. No retained component pointer or background-thread repaint pump.
        unsafe { mpc_sample_sdk::draw::repaint_component(event.component) };
    }
}
// Constructor only on the intended target; host tests never install hooks.
#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
mpc_sample_sdk::export_mod!(LoopClub);

/// TURN renders full-source coordinates; a previous CHOP viewport is unrelated.
pub fn turntable_wave_reset(ui: &Ui, s: &Snapshot) -> Option<Command> {
    (ui.page == Page::Play
        && s.words[3] == ui.selected as u64
        && s.track_kind(ui.selected) == Kind::Audio
        && s.audio_target(ui.selected) == crate::audio::RecordTarget::Turntable
        && (s.words[768] != 0 || s.words[769] != s.words[770]))
        .then_some(Command::WaveView {
            track: ui.selected,
            start: 0,
            end: s.words[770] as u32,
        })
}

/// Normalize the current raw CC value; `low` is not the current control value.
pub fn normalize_cc(raw: u32, _low: f32, range: f32) -> f32 {
    let scale = if range.is_finite() && range > 1. {
        range
    } else {
        127.
    };
    (raw as f32 / scale).clamp(0., 1.)
}

/// Firmware deltas may be accelerated or scaled. One hardware event moves one
/// logical detent, avoiding modulo-list-length cancellation (e.g.128 %4 ==0).
pub fn normalize_encoder(delta: i32) -> i32 {
    delta.signum()
}

#[cfg(test)]
mod regression_tests {
    use super::*;
    #[test]
    fn scratch_fast_route_is_limited_to_visible_turntable_performance() {
        let mut e = Engine::new(8000, 100., 8000);
        e.tracks[0].audio_target = crate::audio::RecordTarget::Turntable;
        let s = Snapshot::capture(&e);
        let mut ui = Ui::default();
        ui.active = true;
        for page in [
            Page::Loop,
            Page::Play,
            Page::Grid,
            Page::Mix,
            Page::KitMix,
            Page::TrackFx,
            Page::MasterFx,
            Page::Setup,
            Page::Chop,
            Page::Source,
            Page::DrumSound,
            Page::Output,
            Page::Sidechain,
            Page::Texture,
            Page::FxSettings,
            Page::Perform,
        ] {
            ui.page = page;
            assert_eq!(
                scratch_route(&ui, &s, false),
                if matches!(page, Page::Loop | Page::Play) {
                    0
                } else {
                    255
                }
            );
            assert_eq!(scratch_route(&ui, &s, true), 255);
        }
        ui.page = Page::Loop;
        for mode in [1, 2] {
            ui.loop_mode = mode;
            assert_eq!(scratch_route(&ui, &s, false), 255);
        }
        ui.loop_mode = 0;
        ui.loop_edit = true;
        assert_eq!(scratch_route(&ui, &s, false), 255);
        ui.loop_edit = false;
        ui.active = false;
        assert_eq!(scratch_route(&ui, &s, false), 255);
        ui.active = true;
        e.tracks[0].kind = Kind::Synth;
        assert_eq!(scratch_route(&ui, &Snapshot::capture(&e), false), 255);
        e.tracks[0].kind = Kind::Audio;
        e.tracks[0].audio_target = crate::audio::RecordTarget::Chromatic;
        assert_eq!(scratch_route(&ui, &Snapshot::capture(&e), false), 255);
    }
    #[test]
    fn immediate_mod_input_handles_navigation_and_fx_edges_without_paint() {
        let mut app = LoopClub::default();
        app.ui.active = true;
        app.ui.page = Page::Loop;
        app.on_encoder(EncoderEvent {
            surface: 0,
            delta: 1,
        });
        assert_eq!(app.ui.selected, 1);
        app.ui.page = Page::TrackFx;
        let event = PadEvent {
            action: PadAction::Press,
            input: 0,
            source: 0,
            pad: 0,
            properties: 0,
            velocity: 0.75,
        };
        assert_eq!(app.on_pad(event), Flow::Consume);
        assert!(app.ui.fx_held[0].is_some());
        assert_eq!(
            app.on_pad(PadEvent {
                action: PadAction::Release,
                velocity: 0.,
                ..event
            }),
            Flow::Consume
        );
        assert!(app.ui.fx_held.iter().all(Option::is_none));
    }
    #[test]
    fn fallback_controls_keep_order_and_are_consumed_once_without_a_paint() {
        let controls = crate::queue::Queue::<HardwareControl, 512>::default();
        let mut app = LoopClub::default();
        app.ui.active = true;
        app.ui.page = Page::Grid;
        let now = Instant::now();
        assert!(controls.push(HardwareControl::Button {
            code: 6,
            value: 127,
            pressed: true,
            shift: false,
            erase: false,
            timestamp: now,
        }));
        assert!(controls.push(HardwareControl::Encoder {
            delta: 1,
            shift: false
        }));
        assert!(controls.push(HardwareControl::Encoder {
            delta: 1,
            shift: false
        }));
        app.drain_pending_sources(Some(&controls), None);
        assert_eq!(app.ui.page, Page::Loop);
        assert_eq!(app.ui.selected, 2);
        assert!(!app.draining_inputs);
        app.drain_pending_sources(Some(&controls), None);
        assert_eq!(app.ui.selected, 2);
    }
    #[test]
    fn fallback_fx_release_cannot_be_lost_or_replayed_by_immediate_dispatch() {
        let pads = crate::queue::Queue::<HardwarePad, 256>::default();
        let mut app = LoopClub::default();
        app.ui.active = true;
        app.ui.page = Page::TrackFx;
        assert!(pads.push(HardwarePad {
            pad: 0,
            on: true,
            velocity: 84,
            shift: false,
            erase: false
        }));
        app.drain_pending_sources(None, Some(&pads));
        assert!(app.ui.fx_held[0].is_some());
        assert!(pads.push(HardwarePad {
            pad: 0,
            on: false,
            velocity: 0,
            shift: false,
            erase: false
        }));
        app.drain_pending_sources(None, Some(&pads));
        assert!(app.ui.fx_held.iter().all(Option::is_none));
        assert!(!app.draining_inputs);
        app.drain_pending_sources(None, Some(&pads));
        assert!(app.ui.fx_held.iter().all(Option::is_none));
    }
    #[test]
    fn reentrant_drain_leaves_events_for_the_exclusive_outer_consumer() {
        let controls = crate::queue::Queue::<HardwareControl, 512>::default();
        let mut app = LoopClub::default();
        app.ui.active = true;
        assert!(controls.push(HardwareControl::Encoder {
            delta: 1,
            shift: false
        }));
        app.draining_inputs = true;
        app.drain_pending_sources(Some(&controls), None);
        assert_eq!(app.ui.selected, 0);
        app.draining_inputs = false;
        app.drain_pending_sources(Some(&controls), None);
        assert_eq!(app.ui.selected, 1);
    }
    #[test]
    fn audio_sampler_recording_allows_sound_entry_but_unfinished_pcm_does_not() {
        for midi in [false, true] {
            let mut app = LoopClub::default();
            app.ui.active = true;
            app.snapshot.words[9] = 1;
            app.snapshot.words[12] = if midi { 1 << 4 } else { 0 };
            app.snapshot.words[20] = 4 * PPQ;
            app.snapshot.words[27] = 8000;
            app.on_button(ButtonEvent {
                event: 0,
                bytes: [0; 16],
                code: 13,
                value: 127,
                pressed: true,
                shift: false,
                erase: false,
                surface: 0,
            });
            assert_eq!(app.ui.page, if midi { Page::Play } else { Page::Loop });
        }
    }
    #[test]
    fn browse_on_a_filled_audio_track_explains_the_empty_track_requirement() {
        let mut app = LoopClub::default();
        app.snapshot.words[20] = 4 * PPQ;
        app.action(Action::Browse);
        assert!(!app.browser.open);
        assert_eq!(app.ui.notice, "USE EMPTY AUDIO TRACK");
    }
    #[test]
    fn shifted_minus_plus_never_select_tracks_or_change_editor_pages() {
        for page in [
            Page::Loop,
            Page::Play,
            Page::Grid,
            Page::Chop,
            Page::DrumSound,
            Page::Texture,
        ] {
            let mut ui = LoopClub::default();
            ui.ui.active = true;
            ui.ui.page = page;
            ui.ui.selected = 3;
            ui.ui.synth_page = 2;
            for code in [55, 54] {
                ui.on_button(ButtonEvent {
                    event: 0,
                    bytes: [0; 16],
                    code,
                    value: 127,
                    pressed: true,
                    shift: true,
                    erase: false,
                    surface: 0,
                });
                assert_eq!(ui.ui.selected, 3);
                assert_eq!(ui.ui.page, page);
                assert_eq!(ui.ui.synth_page, 2);
                assert_eq!(ui.ui.notice, if code == 55 { "UNDO" } else { "REDO" });
            }
        }
    }
    #[test]
    fn native_widget_restore_is_not_a_physical_data_turn() {
        let pending = std::sync::atomic::AtomicBool::new(false);
        assert!(!claim_data_turn(&pending, 7));
        pending.store(true, Ordering::Release);
        assert!(!claim_data_turn(&pending, 0));
        assert!(claim_data_turn(&pending, 1));
        assert!(!claim_data_turn(&pending, 1));
        pending.store(true, Ordering::Release);
        assert!(claim_data_turn(&pending, -1));
    }
    #[test]
    fn rec_from_loop_keeps_screen_and_switches_pad_role_only() {
        let mut ui = LoopClub::default();
        ui.ui.active = true;
        ui.snapshot.words[16] = Kind::Synth as u64;
        ui.ui.page = Page::Loop;
        ui.ui.synth_page = 4;
        assert_eq!(
            ui.on_button(ButtonEvent {
                event: 0,
                bytes: [0; 16],
                code: 73,
                value: 127,
                pressed: true,
                shift: false,
                erase: false,
                surface: 0
            }),
            Flow::Consume
        );
        assert_eq!(ui.ui.page, Page::Loop);
        assert!(ui.ui.loop_perform);
        assert_eq!(ui.ui.synth_page, 4);
    }
    #[test]
    fn data_cc_reaches_native_encoder_dispatch_even_with_browser_open() {
        let mut ui = LoopClub::default();
        ui.ui.active = true;
        ui.browser.open = true;
        for value in [1, 63, 64, 65, 127, u32::MAX] {
            let event = CcEvent {
                event: 0,
                control: 100,
                value,
                low: 0.,
                range: 127.,
            };
            assert_eq!(hardware_cc(event), Flow::Forward);
            assert_eq!(ui.control_cc(event, false), Flow::Forward);
        }
    }
    #[test]
    fn encoder_deltas_have_one_detent_for_all_signed_scales() {
        for value in [1, 2, 4, 6, 16, 127, 128, 1024, 65536, i32::MAX] {
            assert_eq!(normalize_encoder(value), 1);
            assert_eq!(normalize_encoder(-value), -1);
        }
        assert_eq!(normalize_encoder(0), 0);
        assert_eq!(normalize_encoder(i32::MIN), -1);
    }
    #[test]
    fn physical_volume_preserves_project_master_and_controls_injected_audio() {
        let r = Box::new(Runtime::default());
        let mut e = Box::new(Engine::new(8000, 88., 8000));
        e.tracks[0].kind = Kind::Synth;
        assert!(r.initialize(e).is_ok());
        assert!(RUNTIME.set(r).is_ok());
        let r = RUNTIME.get().unwrap();
        r.active.store(true, Ordering::Release);
        r.enqueue(Command::Note {
            track: 0,
            note: 60,
            velocity: 100,
            on: true,
            source: 1,
        });
        let mut block = [0i32; 256];
        block.fill(0);
        r.output(&mut block, 2, 8000);
        let mut ui = LoopClub::default();
        ui.ui.active = true;
        for value in [20, 100, 127] {
            assert_eq!(
                ui.control_cc(
                    CcEvent {
                        event: 0,
                        control: 19,
                        value,
                        low: 0.,
                        range: 127.
                    },
                    false
                ),
                Flow::Forward
            );
            for _ in 0..10 {
                block.fill(0);
                r.output(&mut block, 2, 8000);
            }
            let s = r.snapshot.read().unwrap();
            assert_eq!(s.level(16), 0.8);
            assert!((f32::from_bits(s.words[1043] as u32) - value as f32 / 127.).abs() < 1e-6);
            assert!(block.iter().any(|v| *v != 0));
        }
        ui.ui.page = Page::Play;
        ui.control_cc(
            CcEvent {
                event: 0,
                control: 16,
                value: 64,
                low: 0.,
                range: 127.,
            },
            false,
        );
        block.fill(0);
        r.output(&mut block, 2, 8000);
        let s = r.snapshot.read().unwrap();
        assert!((s.synth().cutoff - (80. * 225_f32.powf(64. / 127.))).abs() < 0.01);
        ui.control_cc(
            CcEvent {
                event: 0,
                control: 19,
                value: 0,
                low: 0.,
                range: 127.,
            },
            false,
        );
        for _ in 0..100 {
            block.fill(0);
            r.output(&mut block, 2, 8000);
        }
        assert_eq!(r.snapshot.read().unwrap().level(16), 0.8);
        assert!(block.iter().all(|v| v.abs() < 1000));
    }
}

#[cfg(test)]
#[path = "device_input_tests.rs"]
mod input_audit_tests;
