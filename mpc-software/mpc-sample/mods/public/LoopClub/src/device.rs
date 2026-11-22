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
        atomic::{AtomicU8, Ordering},
    },
    time::Duration,
};
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
    },
    Cc {
        control: u32,
        value: u32,
        low: f32,
        range: f32,
        shift: bool,
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
    queue_control(HardwareControl::Button {
        code: event.code,
        value: event.value,
        pressed: event.pressed,
        shift: event.shift,
        erase: event.erase,
    });
    Flow::Consume
}
fn hardware_cc(event: CcEvent) -> Flow {
    if !runtime().is_some_and(|r| r.active.load(Ordering::Acquire)) {
        return Flow::Forward;
    }
    queue_control(HardwareControl::Cc {
        control: event.control,
        value: event.value,
        low: event.low,
        range: event.range,
        shift: mpc_sample_sdk::shift(),
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
    queue_control(HardwareControl::Encoder {
        delta: event.delta,
        shift: mpc_sample_sdk::shift(),
    });
    Flow::Consume
}
static PAD_OWNED: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
fn pad_input(event: PadEvent) -> PadResponse {
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
static IO_REQUEST: AtomicU8 = AtomicU8::new(0);
static IO_STATUS: AtomicU8 = AtomicU8::new(0);
fn runtime() -> Option<&'static Runtime> {
    RUNTIME.get().map(Box::as_ref)
}
fn send(c: Command) {
    if let Some(r) = runtime() {
        r.enqueue(c)
    }
}
fn note(event: &mut NoteEvent<'_>) -> Flow {
    let Some(r) = runtime() else {
        return Flow::Forward;
    };
    if !event.external || event.note > 127 || !(event.note_on || event.note_off) {
        return Flow::Forward;
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
    if channels == 0 || mpc_sample_sdk::audio::input_period() == 24 {
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
    if mpc_sample_sdk::audio::input_period() == 24 {
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
    let mut initial = Engine::new(rate, 88., capacity);
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u32;
    for (i, t) in initial.tracks.iter_mut().enumerate() {
        t.pattern.seed = seed ^ (i as u32 + 1).wrapping_mul(0x9e3779b9);
    }

    if let Some(dir) = &dir {
        let path = dir.join("Session.lcproj");
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
    let mut files = crate::browser_device::FileWorker::new(rate, capacity);
    loop {
        std::thread::sleep(Duration::from_millis(20));
        crate::qa::tick();
        if let Some(ok) = files.tick(r) {
            IO_STATUS.store(if ok { 5 } else { 4 }, Ordering::Release);
        }
        let action = IO_REQUEST.swap(0, Ordering::AcqRel);
        if action == 0 {
            continue;
        }
        let Some(dir) = dir.as_ref() else {
            IO_STATUS.store(4, Ordering::Release);
            continue;
        };
        if !dir.is_absolute() {
            IO_STATUS.store(4, Ordering::Release);
            continue;
        }
        let path = dir.join("Session.lcproj");
        let done = r.with_engine(|slot| {
            let Some(e) = slot.as_mut() else { return 4 };
            if !e.is_idle() {
                return 3;
            }
            if action == 1 {
                if path.is_file() {
                    let backup = dir.join("Session.previous.lcproj");
                    let temp = dir.join("Session.previous.lcproj.new");
                    let preserved = (|| -> std::io::Result<()> {
                        std::fs::copy(&path, &temp)?;
                        std::fs::File::open(&temp)?.sync_all()?;
                        std::fs::rename(&temp, &backup)
                    })();
                    if preserved.is_err() {
                        return 4;
                    }
                }
                if crate::project::save(e, &path).is_ok() {
                    1
                } else {
                    4
                }
            } else {
                match crate::project::load(&path, rate, capacity) {
                    Ok(mut loaded) => {
                        loaded.selected = e.selected;
                        **e = loaded;
                        2
                    }
                    Err(_) => 4,
                }
            }
        });
        if let Some(status) = done {
            IO_STATUS.store(status, Ordering::Release)
        } else {
            let _ = IO_REQUEST.compare_exchange(0, action, Ordering::AcqRel, Ordering::Relaxed);
        }
    }
}
#[derive(Default)]
pub struct LoopClub {
    browser: crate::browser_device::BrowserUi,
    ui: Ui,
    snapshot: Snapshot,
    armed: bool,
    exit_requested: bool,
}
impl LoopClub {
    fn control_cc(&mut self, event: CcEvent, shift: bool) -> Flow {
        if !self.ui.active {
            return Flow::Forward;
        }
        self.refresh();
        if self.browser.open && event.control != 19 {
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
            && self.snapshot.track_kind(self.ui.selected) != Kind::Drum
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
            20 => self.ui.fader(value, &self.snapshot, send),
            19 => {
                send(Command::Master(value));
                return Flow::Forward;
            }
            _ => {}
        }
        Flow::Consume
    }
    fn refresh(&mut self) {
        if let Some(s) = runtime().and_then(|r| r.snapshot.read()) {
            self.snapshot = s;
        }
        if self.ui.page == Page::Chop {
            let count = ((self.snapshot.words[506] >> 16) & 255).clamp(1, 16) as usize;
            self.ui.chop_slice = self.ui.chop_slice.min(count - 1);
        }
        let status = IO_STATUS.swap(0, Ordering::AcqRel);
        if status == 5 {
            self.browser.open = false;
            let page = if self.browser.target.is_some() {
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
                5 if self.browser.target.is_some() => "LAYER SAMPLE LOADED",
                5 if self.snapshot.track_kind(self.ui.selected) == Kind::Drum => "KIT LOADED",
                5 => "SAMPLE LOADED",
                _ => "PROJECT IO ERROR",
            };
        }
    }
    fn leave(&mut self) {
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
                } else {
                    self.ui.notice = "SELECT AUDIO OR DRUM TRACK";
                }
            }
            Action::Save | Action::Load => {
                let n = if action == Action::Save { 1 } else { 2 };
                if IO_REQUEST
                    .compare_exchange(0, n, Ordering::AcqRel, Ordering::Relaxed)
                    .is_ok()
                {
                    self.ui.notice = "PROJECT WORKING..."
                } else {
                    self.ui.notice = "PROJECT BUSY"
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
        let _ = RUNTIME.set(Box::new(Runtime::default()));
        self.armed = true;
        crate::qa::start();
        let _ = PAD_EVENTS.set(crate::queue::Queue::default());
        let _ = CONTROL_EVENTS.set(crate::queue::Queue::default());
        crate::browser_device::initialize();
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
        crate::qa::button();
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
        if self.browser.open {
            if event.code == 81 {
                send(Command::Stop)
            } else {
                self.browser.button(event.code)
            }
            return Flow::Consume;
        }
        match event.code {
            14 if self.ui.page == Page::Source => self.action(Action::Browse),
            13 if self.ui.page == Page::Loop
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
            6 | 74 => self.ui.navigate(Page::Loop, &self.snapshot),
            7 => self.ui.navigate(Page::Grid, &self.snapshot),
            40 => self.ui.navigate(Page::Play, &self.snapshot),
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
                if matches!(
                    self.ui.page,
                    Page::MasterFx | Page::Texture | Page::Sidechain
                ) && !event.shift
                {
                    self.ui.navigate(
                        match self.ui.page {
                            Page::MasterFx => Page::Texture,
                            Page::Texture => Page::Sidechain,
                            _ => Page::MasterFx,
                        },
                        &self.snapshot,
                    );
                } else if self.ui.page == Page::Output && !event.shift {
                    self.ui.navigate(Page::Mix, &self.snapshot);
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
                    self.ui.navigate(Page::Setup, &self.snapshot)
                } else {
                    if self.snapshot.words[16 + self.ui.selected * 12 + 4] > 0 {
                        send(Command::Pad(self.ui.selected, false));
                    } else {
                        self.ui.notice = "PRESS REC TO RECORD";
                    }
                }
            }
            81 => send(Command::Stop),
            82 => send(Command::PlayAll),
            73 | 80 => send(Command::Record(self.ui.selected)),
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
                    self.ui.synth_page = (self.ui.synth_page + 1) % 8;
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
                    self.ui.synth_page = (self.ui.synth_page + 7) % 8;
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
        if !self.ui.active {
            return Flow::Forward;
        }
        self.refresh();
        if self.browser.open {
            self.browser.encoder(event.delta);
            return Flow::Consume;
        }
        self.ui.shift_view = mpc_sample_sdk::shift();
        self.ui.encoder(event.delta, &self.snapshot, send);
        Flow::Consume
    }
    fn on_cc(&mut self, event: CcEvent) -> Flow {
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
        if let Some(events) = CONTROL_EVENTS.get() {
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
                    } => {
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
                    } => {
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
                        if self.browser.open {
                            self.browser.encoder(delta);
                        } else {
                            self.ui.shift_view = shift;
                            self.ui.encoder(delta, &self.snapshot, send);
                        }
                    }
                }
            }
        }
        for _ in 0..64 {
            let Some(input) = crate::qa::pop() else {
                break;
            };
            match input {
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
                    self.on_encoder(EncoderEvent { surface: 0, delta });
                }
                crate::qa::Input::Pad {
                    index,
                    on,
                    velocity,
                    shift,
                    erase,
                } => {
                    if self.browser.open && on {
                        continue;
                    }
                    let action =
                        self.ui
                            .pad(index, on, velocity, shift, erase, &self.snapshot, send);
                    self.action(action);
                }
            }
        }
        crate::qa::paint(&self.ui, &self.snapshot, self.browser.open);
        if let Some(events) = PAD_EVENTS.get() {
            for _ in 0..256 {
                // SAFETY: only serialized on_paint callbacks consume this queue.
                let Some(event) = (unsafe { events.pop_shared() }) else {
                    break;
                };
                if self.browser.open && event.on {
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
        if !self.ui.active {
            return;
        }
        if self.browser.open {
            self.browser.render(&mut NativePainter(*graphics))
        } else {
            view::render(&self.ui, &self.snapshot, &mut NativePainter(*graphics));
        }
        // SAFETY: request the next asynchronous repaint only from this live paint
        // callback. No retained component pointer or background-thread repaint pump.
        unsafe { mpc_sample_sdk::draw::repaint_component(event.component) };
    }
}
// Constructor only on the intended target; host tests never install hooks.
#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
mpc_sample_sdk::export_mod!(LoopClub);

/// Follow the pinned SDK's documented CC geometry, not an assumed MIDI range.
pub fn normalize_cc(raw: u32, value: f32, range: f32) -> f32 {
    if range.is_finite() && range > 0. && value.is_finite() {
        (value / range).clamp(0., 1.)
    } else {
        raw.min(127) as f32 / 127.
    }
}
