use crate::{
    clock::Clock,
    dsp::{Delay, FxState, Room, Voice},
    model::*,
};
pub const MAX_EVENTS: usize = 4096;
// Measured Sample1.3.0.12 budget:32shared voices;64exceeded steady deadlines.
pub const VOICES: usize = 32;
pub struct Track {
    pub kit: Box<crate::bank::DrumKit>,
    pub record_mode: RecordMode,
    pub source_frames: usize,
    pub features: Box<crate::features::Features>,
    pub wave_view: (u32, u32),
    chop_undo: Option<crate::chop::Chops>,
    pub chops: crate::chop::Chops,
    pub generated: bool,
    pub generated_sounds: u16,
    last_generated_sound: Option<usize>,
    pub pattern: crate::music::PatternConfig,
    pub chord: crate::music::ChordConfig,
    previous_events: Vec<Event>,
    previous_clip: (u64, u64, bool, bool, u16),
    pattern_undo: bool,
    pub kind: Kind,
    pub name: String,
    pub mix: Mix,
    pub synth: Synth,
    pub pan_current: f32,
    duck_gain: f32,
    pan_lfo: crate::synth::Lfo,
    pub drums: [Drum; 16],
    pub fx: [Fx; 2],
    pub route: Route,
    pub output: OutputTarget,
    pub main_gain: f32,
    pub usb_gain: f32,
    pub audio: Vec<Frame>,
    pub events: Vec<Event>,
    pub length_ticks: u64,
    pub audio_frames: usize,
    pub playing: bool,
    pub origin: u64,
    pub meter: f32,
    pub gain: f32,
    pub sound_gain: [f32; 16],
    pub cursor: usize,
    pub last_phase: u64,
    pub revision: u64,
}
impl Track {
    fn new(capacity: usize, index: usize) -> Self {
        Self {
            kit: Box::new(crate::bank::DrumKit::default()),
            record_mode: RecordMode::Replace,
            source_frames: 0,
            features: Box::new(crate::features::Features::default()),
            wave_view: (0, 0),
            chop_undo: None,
            chops: crate::chop::Chops::default(),
            generated: false,
            generated_sounds: 0,
            last_generated_sound: None,
            pattern: crate::music::PatternConfig::default(),
            chord: crate::music::ChordConfig::default(),
            previous_events: Vec::with_capacity(MAX_EVENTS),
            previous_clip: (0, 0, false, false, 0),
            pattern_undo: false,
            kind: Kind::Audio,
            name: format!("TRACK {:02}", index + 1),
            mix: Mix::default(),
            synth: Synth::default(),
            pan_current: 0.,
            duck_gain: 1.,
            pan_lfo: crate::synth::Lfo::default(),
            drums: [Drum::default(); 16],
            fx: [Fx::default(); 2],
            route: Route::Dry,
            output: OutputTarget::Both,
            main_gain: 1.,
            usb_gain: 1.,
            audio: vec![Frame::ZERO; capacity],
            events: Vec::with_capacity(MAX_EVENTS),
            length_ticks: 0,
            audio_frames: 0,
            playing: false,
            origin: 0,
            meter: 0.,
            gain: 0.7,
            sound_gain: [0.7; 16],
            cursor: 0,
            last_phase: 0,
            revision: 0,
        }
    }
    pub fn empty(&self) -> bool {
        self.length_ticks == 0
    }
    pub fn phase(&self, tick: u64) -> u64 {
        if self.length_ticks == 0 {
            0
        } else {
            tick.saturating_sub(self.origin) % self.length_ticks
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Recorder {
    pub track: usize,
    pub start: u64,
    pub end: Option<u64>,
    pub limit: u64,
    pub resample: bool,
    pub finish_started: bool,
    pub repeating: bool,
    pub stop_after_cycle: bool,
    pub mode: RecordMode,
    pub touched: u16,
}
#[derive(Clone, Copy, Debug)]
pub enum Command {
    Sidechain(crate::motion::Sidechain),
    MasterBypass {
        slot: usize,
        bypass: bool,
    },
    TextureValue {
        slot: usize,
        index: u8,
        value: f32,
    },
    RecordMode(usize, RecordMode),
    Output(usize, OutputTarget),
    ChopMode(usize),
    ChopApply(usize),
    ChopUndo(usize),
    ChopOption {
        track: usize,
        index: u8,
        value: f32,
    },
    SplitSlice {
        track: usize,
        pad: usize,
        at: u32,
    },
    DeleteSlice {
        track: usize,
        pad: usize,
    },
    WaveView {
        track: usize,
        start: u32,
        end: u32,
    },
    Slice {
        track: usize,
        pad: usize,
        velocity: u8,
        on: bool,
    },
    SliceValue {
        track: usize,
        pad: usize,
        index: u8,
        value: f32,
    },
    SliceGate(usize),
    ResetSlices(usize),
    GenerateDrumPart {
        track: usize,
        sound: usize,
    },
    UndoDrumPart {
        track: usize,
        sound: usize,
    },
    UndoPattern(usize),
    Chord {
        track: usize,
        pad: usize,
        velocity: u8,
        on: bool,
    },
    ChordValue {
        track: usize,
        index: u8,
        value: f32,
    },
    SynthValue {
        track: usize,
        index: u8,
        value: f32,
    },
    DrumValue {
        track: usize,
        sound: usize,
        index: u8,
        value: f32,
    },
    DrumToggle {
        track: usize,
        sound: usize,
        solo: bool,
    },
    FxValue {
        target: u8,
        track: usize,
        slot: usize,
        index: u8,
        value: f32,
    },
    FxSelect {
        target: u8,
        track: usize,
        slot: usize,
        effect: Effect,
        temporary: bool,
    },

    Select(usize),
    SetKind(usize, Kind),
    Pad(usize, bool),
    Record(usize),
    Stop,
    CancelRecord,
    PlayAll,
    Clear(usize),
    Bpm(f64),
    Preroll(u8),
    Level(usize, f32),
    Pan(usize, f32),
    Send(usize, bool, f32),
    Mute(usize),
    Solo(usize),
    Note {
        track: usize,
        note: u8,
        velocity: u8,
        on: bool,
        source: u16,
    },
    SetFx {
        track: usize,
        slot: usize,
        fx: Fx,
    },
    Performance {
        slot: usize,
        effect: Effect,
        on: bool,
    },
    Master(f32),
    FxOff,
    Route(usize, Route),
    Resample(usize),
    DrumMix {
        track: usize,
        sound: usize,
        mix: Mix,
    },
    Synth(usize, Synth),
    Edit {
        track: usize,
        index: usize,
        note: u8,
        velocity: u8,
        length: u64,
    },
    PerformanceFx {
        slot: usize,
        fx: Fx,
    },
    BusFx {
        slot: usize,
        fx: Fx,
    },
}
#[derive(Clone, Copy, Default)]
struct HeldChord {
    track: usize,
    notes: [u8; 5],
    len: usize,
}
#[path = "recording.rs"]
mod recording;

pub struct Engine {
    capture_audio: Vec<Frame>,
    capture_features: Box<crate::features::Features>,
    capture_events: Vec<Event>,
    capture_velocities: [u8; 128],
    held_chords: [HeldChord; 16],
    slice_voices: [crate::chop::SliceVoice; 16],
    held_fx: u64,
    pub clock: Clock,
    pub tracks: [Track; 16],
    pub selected: usize,
    pub running: bool,
    pub preroll: u8,
    pub recorder: Option<Recorder>,
    pub latency_in: u32,
    pub master: f32,
    pub master_gain: f32,
    pub master_meter: Frame,
    pub usb_output: Frame,
    pub usb_meter: Frame,
    pub separate_usb: bool,
    usb_stage: crate::usb_bus::UsbStage,
    pub performance: [Fx; 2],
    pub bus_fx: [Fx; 2],
    pub master_fx: Fx,
    pub sidechain: crate::motion::Sidechain,
    pub duck: crate::motion::Duck,
    pub texture: crate::motion::Texture,
    texture_main: crate::motion::TextureState,
    texture_usb: crate::motion::TextureState,
    pub master_comp: f32,
    master_duck_gain: f32,
    pub error: Option<Error>,
    pub event_overflows: u64,
    pub recorded_frames: u64,
    voices: [Voice; VOICES],
    age: u64,
    track_fx: [[FxState; 2]; 16],
    perf_state: [FxState; 2],
    bus_state: [FxState; 2],
    master_state: FxState,
    delay: Delay,
    room: Room,
    previous_master: Frame,
    open_notes: [[bool; 128]; 16],
}
impl Engine {
    pub fn new(rate: u32, bpm: f64, frames_per_track: usize) -> Self {
        let rate = rate.clamp(8000, 96000);
        Self {
            capture_audio: vec![Frame::ZERO; frames_per_track],
            capture_features: Box::new(crate::features::Features::default()),
            capture_events: Vec::with_capacity(MAX_EVENTS),
            capture_velocities: [0; 128],
            held_chords: [HeldChord::default(); 16],
            slice_voices: [crate::chop::SliceVoice::default(); 16],
            held_fx: 0,
            clock: Clock::new(rate, bpm),
            tracks: std::array::from_fn(|i| Track::new(frames_per_track, i)),
            selected: 0,
            running: false,
            preroll: 1,
            recorder: None,
            latency_in: 0,
            master: 0.8,
            master_gain: 0.8,
            master_meter: Frame::ZERO,
            usb_output: Frame::ZERO,
            usb_meter: Frame::ZERO,
            separate_usb: false,
            usb_stage: crate::usb_bus::UsbStage::new(rate),
            performance: [Fx::default(); 2],
            bus_fx: [Fx::default(); 2],
            master_fx: Fx::default(),
            sidechain: crate::motion::Sidechain::default(),
            duck: crate::motion::Duck::default(),
            texture: crate::motion::Texture::default(),
            texture_main: crate::motion::TextureState::new(rate),
            texture_usb: crate::motion::TextureState::new(rate),
            master_comp: 0.,
            master_duck_gain: 1.,
            error: None,
            event_overflows: 0,
            recorded_frames: 0,
            voices: [Voice::default(); VOICES],
            age: 0,
            track_fx: std::array::from_fn(|_| std::array::from_fn(|_| FxState::new(rate))),
            perf_state: std::array::from_fn(|_| FxState::new(rate)),
            bus_state: std::array::from_fn(|_| FxState::new(rate)),
            master_state: FxState::new(rate),
            delay: Delay::new(rate),
            room: Room::new(rate),
            previous_master: Frame::ZERO,
            open_notes: [[false; 128]; 16],
        }
    }
    pub fn command(&mut self, c: Command) -> Result<(), Error> {
        let result = self.apply(c);
        self.error = result.err();
        result
    }
    fn valid_track(i: usize) -> Result<usize, Error> {
        if i < 16 {
            Ok(i)
        } else {
            Err(Error::InvalidTrack)
        }
    }
    fn fx_bit(target: u8, track: usize, slot: usize) -> u64 {
        1 << match target {
            0 => track * 2 + slot,
            1 => 32 + slot,
            _ => 34 + usize::from(target - 2),
        }
    }
    pub fn release_temporary_fx(&mut self) {
        for track in 0..16 {
            for slot in 0..2 {
                if self.held_fx & Self::fx_bit(0, track, slot) != 0 {
                    self.tracks[track].fx[slot].effect = Effect::Off
                }
            }
        }
        for slot in 0..2 {
            if self.held_fx & Self::fx_bit(1, 0, slot) != 0 {
                self.performance[slot].effect = Effect::Off
            }
            if self.held_fx & Self::fx_bit(slot as u8 + 2, 0, 0) != 0 {
                self.bus_fx[slot].effect = Effect::Off
            }
        }
        self.held_fx = 0;
    }
    pub fn saved_track_fx(&self, track: usize) -> [Fx; 2] {
        std::array::from_fn(|slot| {
            let fx = self.tracks[track].fx[slot];
            if self.held_fx & Self::fx_bit(0, track, slot) != 0 {
                Fx {
                    effect: Effect::Off,
                    ..fx
                }
            } else {
                fx
            }
        })
    }
    pub fn saved_bus_fx(&self) -> [Fx; 2] {
        std::array::from_fn(|slot| {
            let fx = self.bus_fx[slot];
            if self.held_fx & Self::fx_bit(slot as u8 + 2, 0, 0) != 0 {
                Fx {
                    effect: Effect::Off,
                    ..fx
                }
            } else {
                fx
            }
        })
    }
    fn effect_mut(&mut self, target: u8, track: usize, slot: usize) -> Result<&mut Fx, Error> {
        if slot >= 2 {
            return Err(Error::InvalidValue);
        }
        match target {
            0 => {
                Self::valid_track(track)?;
                Ok(&mut self.tracks[track].fx[slot])
            }
            1 => Ok(&mut self.performance[slot]),
            2 => Ok(&mut self.bus_fx[0]),
            3 => Ok(&mut self.bus_fx[1]),
            _ => Err(Error::InvalidValue),
        }
    }
    fn apply(&mut self, c: Command) -> Result<(), Error> {
        match c {
            Command::MasterBypass { slot, bypass } => {
                if slot >= 2 {
                    return Err(Error::InvalidValue);
                }
                self.texture.master_bypass[slot] = bypass;
            }
            Command::Sidechain(cfg) => {
                if !cfg.valid() {
                    return Err(Error::InvalidValue);
                }
                self.sidechain = cfg;
            }
            Command::TextureValue { slot, index, value } => {
                if slot >= 5 || index > 3 || !value.is_finite() || !(0.0..=1.0).contains(&value) {
                    return Err(Error::InvalidValue);
                }
                if index == 3 {
                    self.texture.enabled[slot] = value >= 0.5
                } else {
                    self.texture.params[slot][index as usize] = value;
                }
            }
            Command::WaveView { track, start, end } => {
                Self::valid_track(track)?;
                let t = &mut self.tracks[track];
                if start >= end || end as usize > t.source_frames.max(t.audio_frames) {
                    return Err(Error::InvalidValue);
                }
                t.wave_view = (start, end);
            }
            Command::ChopMode(track) => {
                Self::valid_track(track)?;
                let c = &mut self.tracks[track].chops;
                c.mode = match c.mode {
                    crate::chop::Mode::Equal => crate::chop::Mode::Transient,
                    crate::chop::Mode::Transient => crate::chop::Mode::Bpm,
                    crate::chop::Mode::Bpm => crate::chop::Mode::Manual,
                    crate::chop::Mode::Manual => crate::chop::Mode::Equal,
                };
                c.pending = true;
            }
            Command::ChopApply(track) => {
                Self::valid_track(track)?;
                let t = &mut self.tracks[track];
                if t.kind != Kind::Audio || t.source_frames == 0 {
                    return Err(Error::NoContent);
                }
                let mut c = t.chops;
                if !c.apply(
                    t.source_frames,
                    self.clock.rate,
                    self.clock.bpm,
                    &t.features,
                ) {
                    return Err(Error::Capacity);
                }
                t.chop_undo = Some(t.chops);
                t.chops = c;
            }
            Command::ChopUndo(track) => {
                Self::valid_track(track)?;
                let t = &mut self.tracks[track];
                let old = t.chop_undo.take().ok_or(Error::NoContent)?;
                if !old.valid(t.source_frames) {
                    return Err(Error::InvalidValue);
                }
                t.chops = old;
            }
            Command::ChopOption {
                track,
                index,
                value,
            } => {
                Self::valid_track(track)?;
                if !value.is_finite() || !(0.0..=1.0).contains(&value) {
                    return Err(Error::InvalidValue);
                }
                let c = &mut self.tracks[track].chops;
                match index {
                    0 => c.regions = (value * 15.99) as u8 + 1,
                    1 => c.sensitivity = value,
                    2 => c.min_gap_ms = 10. + value * 490.,
                    3 => c.grid = (value * 10.99) as u8,
                    _ => return Err(Error::InvalidValue),
                }
                c.pending = true;
            }
            Command::SplitSlice { track, pad, at } => {
                Self::valid_track(track)?;
                let t = &mut self.tracks[track];
                let mut c = t.chops;
                if !c.split(pad, at) {
                    return Err(Error::InvalidValue);
                }
                t.chop_undo = Some(t.chops);
                t.chops = c;
            }
            Command::DeleteSlice { track, pad } => {
                Self::valid_track(track)?;
                let t = &mut self.tracks[track];
                let mut c = t.chops;
                if !c.delete(pad) {
                    return Err(Error::InvalidValue);
                }
                t.chop_undo = Some(t.chops);
                t.chops = c;
            }

            Command::ResetSlices(track) => {
                Self::valid_track(track)?;
                let t = &mut self.tracks[track];
                if t.kind != Kind::Audio || t.audio_frames == 0 {
                    return Err(Error::NoContent);
                }
                t.source_frames = t.source_frames.min(t.audio_frames);
                if t.source_frames == 0 {
                    t.source_frames = t.audio_frames
                }
                t.chop_undo = Some(t.chops);
                t.chops.divide(t.source_frames);
            }
            Command::SliceGate(track) => {
                Self::valid_track(track)?;
                self.tracks[track].chops.gate = !self.tracks[track].chops.gate;
            }
            Command::SliceValue {
                track,
                pad,
                index,
                value,
            } => {
                Self::valid_track(track)?;
                if pad >= 16 || !value.is_finite() || !(0.0..=1.0).contains(&value) {
                    return Err(Error::InvalidValue);
                }
                let t = &mut self.tracks[track];
                if t.kind != Kind::Audio || t.source_frames == 0 {
                    return Err(Error::NoContent);
                }
                if pad >= usize::from(t.chops.count) {
                    return Err(Error::NoContent);
                }
                t.chop_undo = Some(t.chops);
                if index < 2 {
                    t.chops.mode = crate::chop::Mode::Manual;
                    t.chops.pending = false;
                }
                let slice = &mut t.chops.slices[pad];
                match index {
                    0 => {
                        slice.start = ((value * t.source_frames as f32) as u32)
                            .min(slice.end.saturating_sub(1))
                    }
                    1 => {
                        slice.end = ((value * t.source_frames as f32) as u32)
                            .max(slice.start + 1)
                            .min(t.source_frames as u32)
                    }
                    2 => slice.level = value,
                    _ => return Err(Error::InvalidValue),
                }
            }
            Command::Slice {
                track,
                pad,
                velocity,
                on,
            } => {
                Self::valid_track(track)?;
                if pad >= 16 || velocity > 127 {
                    return Err(Error::InvalidValue);
                }
                if !on || velocity == 0 {
                    for v in &mut self.slice_voices {
                        if v.track == track && v.pad == pad && v.gate {
                            v.release = true;
                        }
                    }
                } else {
                    if self.recorder.is_some_and(|r| r.track == track) {
                        return Err(Error::Busy);
                    }
                    let t = &mut self.tracks[track];
                    if t.kind != Kind::Audio || t.audio_frames == 0 {
                        return Err(Error::NoContent);
                    }
                    if t.source_frames == 0 {
                        t.source_frames = t.audio_frames;
                        t.chops.divide(t.source_frames);
                    }
                    if pad >= usize::from(t.chops.count) {
                        return Err(Error::NoContent);
                    }
                    let slice = t.chops.slices[pad];
                    if slice.end <= slice.start || slice.end as usize > t.audio_frames {
                        return Err(Error::InvalidValue);
                    }
                    let slot = self
                        .slice_voices
                        .iter()
                        .position(|v| !v.active)
                        .unwrap_or_else(|| {
                            self.slice_voices
                                .iter()
                                .enumerate()
                                .min_by_key(|(_, v)| v.age)
                                .unwrap()
                                .0
                        });
                    self.age += 1;
                    self.slice_voices[slot] = crate::chop::SliceVoice {
                        active: true,
                        track,
                        pad,
                        start: slice.start as usize,
                        end: slice.end as usize,
                        at: slice.start as usize,
                        velocity: f32::from(velocity) / 127. * slice.level,
                        gate: t.chops.gate,
                        release: false,
                        env: 1.,
                        age: self.age,
                    };
                }
            }

            Command::GenerateDrumPart { track, sound } => self.generate_drum_part(track, sound)?,
            Command::UndoDrumPart { track, sound } => {
                Self::valid_track(track)?;
                if self.tracks[track].last_generated_sound != Some(sound) {
                    return Err(Error::NoContent);
                }
                self.apply(Command::UndoPattern(track))?;
            }
            Command::UndoPattern(track) => {
                Self::valid_track(track)?;
                if self.recorder.is_some_and(|r| r.track == track) {
                    return Err(Error::Busy);
                }
                let t = &mut self.tracks[track];
                if !t.pattern_undo {
                    return Err(Error::NoContent);
                }
                std::mem::swap(&mut t.events, &mut t.previous_events);
                (
                    t.length_ticks,
                    t.origin,
                    t.playing,
                    t.generated,
                    t.generated_sounds,
                ) = t.previous_clip;
                t.pattern_undo = false;
                let phase = t.phase(self.clock.tick());
                t.cursor = t.events.partition_point(|e| e.tick < phase);
                t.last_phase = phase;
                t.revision += 1;
                self.release_track(track);
            }
            Command::ChordValue {
                track,
                index,
                value,
            } => {
                Self::valid_track(track)?;
                if !value.is_finite() {
                    return Err(Error::InvalidValue);
                }
                let mut cfg = self.tracks[track].chord;
                match index {
                    0 => cfg.key = value.round().clamp(0., 11.) as u8,
                    1 => cfg.scale = value.round().clamp(0., 2.) as u8,
                    2 => cfg.color = value.round().clamp(0., 2.) as u8,
                    3 => cfg.octave = value.round().clamp(-2., 2.) as i8,
                    4 => cfg.open = value >= 0.5,
                    _ => return Err(Error::InvalidValue),
                }
                self.tracks[track].chord = cfg;
            }
            Command::Chord {
                track,
                pad,
                velocity,
                on,
            } => {
                Self::valid_track(track)?;
                if pad >= 16 || velocity > 127 {
                    return Err(Error::InvalidValue);
                }
                let previous = self.held_chords[pad];
                self.held_chords[pad].len = 0;
                for note in &previous.notes[..previous.len] {
                    self.live_note(previous.track, *note, 0, false, 100 + pad as u16);
                }
                if on && velocity > 0 {
                    if self.tracks[track].kind != Kind::Synth {
                        return Err(Error::WrongKind);
                    }
                    let (notes, len) = self.tracks[track].chord.notes(pad);
                    self.held_chords[pad] = HeldChord { track, notes, len };
                    for note in &notes[..len] {
                        self.live_note(track, *note, velocity, true, 100 + pad as u16);
                    }
                }
            }

            Command::FxSelect {
                target,
                track,
                slot,
                effect,
                temporary,
            } => {
                self.effect_mut(target, track, slot)?.effect = effect;
                if target == 1 {
                    self.texture.master_bypass[slot] = false;
                }
                let bit = Self::fx_bit(target, track, slot);
                if temporary && effect != Effect::Off {
                    self.held_fx |= bit
                } else {
                    self.held_fx &= !bit
                }
            }
            Command::FxValue {
                target,
                track,
                slot,
                index,
                value,
            } => {
                if !value.is_finite() || !(0.0..=1.0).contains(&value) || index > 2 {
                    return Err(Error::InvalidValue);
                }
                let fx = self.effect_mut(target, track, slot)?;
                match index {
                    0 => fx.a = value,
                    1 => fx.b = value,
                    _ => fx.mix = value,
                }
            }
            Command::SynthValue {
                track,
                index,
                value,
            } => {
                Self::valid_track(track)?;
                if !value.is_finite() {
                    return Err(Error::InvalidValue);
                }
                let mut synth = self.tracks[track].synth;
                match index {
                    0 => synth.cutoff = value,
                    1 => synth.attack = value,
                    2 => synth.release = value,
                    3 => synth.wave = value.round().clamp(0., 3.) as u8,
                    4 => synth.detune = value,
                    5 => synth.octave = value.round().clamp(-2., 2.) as i8,
                    6 => synth.wave2 = value.round().clamp(0., 3.) as u8,
                    7 => synth.tune1 = value,
                    8 => synth.tune2 = value,
                    9 => synth.level1 = value,
                    10 => synth.level2 = value,
                    11 => synth.resonance = value,
                    12 => synth.filter = value.round().clamp(0., 2.) as u8,
                    13 => synth.decay = value,
                    14 => synth.sustain = value,
                    15 => synth.fine1 = value,
                    16 => synth.fine2 = value,
                    17 => synth.filter_attack = value,
                    18 => synth.filter_decay = value,
                    19 => synth.filter_sustain = value,
                    20 => synth.filter_release = value,
                    21 => synth.env_amount = value,
                    22 => synth.lfo_rate = value,
                    23 => synth.lfo_shape = value.round().clamp(0., 5.) as u8,
                    24 => synth.lfo_pitch = value,
                    25 => synth.lfo_filter = value,
                    26 => synth.lfo_volume = value,
                    27 => synth.lfo_pan = value,
                    _ => return Err(Error::InvalidValue),
                }
                if !synth.valid() {
                    return Err(Error::InvalidValue);
                }
                self.tracks[track].synth = synth;
            }
            Command::DrumValue {
                track,
                sound,
                index,
                value,
            } => {
                Self::valid_track(track)?;
                if sound >= 16 || !value.is_finite() {
                    return Err(Error::InvalidValue);
                }
                let mut drum = self.tracks[track].drums[sound];
                match index {
                    0 => drum.mix.level = value,
                    1 => drum.mix.pan = value,
                    2 => drum.mix.delay = value,
                    3 => drum.mix.room = value,
                    4 => drum.tune = value,
                    5 => drum.decay = value,
                    6..=17 => {
                        let layer = usize::from((index - 6) / 6);
                        let cfg = &mut drum.layers[layer];
                        match (index - 6) % 6 {
                            0 => cfg.level = value,
                            1 => cfg.tune = value,
                            2 => cfg.shift = value,
                            3 => cfg.decay = value,
                            4 => cfg.enabled = value >= 0.5,
                            _ => cfg.reverse = value >= 0.5,
                        }
                    }
                    _ => return Err(Error::InvalidValue),
                }
                if !drum.valid() {
                    return Err(Error::InvalidValue);
                }
                self.tracks[track].drums[sound] = drum
            }
            Command::DrumToggle { track, sound, solo } => {
                Self::valid_track(track)?;
                if sound >= 16 {
                    return Err(Error::InvalidValue);
                }
                let mix = &mut self.tracks[track].drums[sound].mix;
                if solo {
                    mix.solo = !mix.solo
                } else {
                    mix.mute = !mix.mute
                }
            }

            Command::RecordMode(i, mode) => {
                self.tracks[Self::valid_track(i)?].record_mode = mode;
            }
            Command::Select(i) => {
                self.selected = Self::valid_track(i)?;
            }
            Command::SetKind(i, k) => {
                let i = Self::valid_track(i)?;
                if !self.tracks[i].empty() || self.recorder.is_some_and(|r| r.track == i) {
                    return Err(Error::NotEmpty);
                }
                self.release_track(i);
                self.tracks[i].kind = k
            }
            Command::Record(i) | Command::Resample(i) => {
                let i = Self::valid_track(i)?;
                if self.recorder.is_some_and(|r| r.track == i) {
                    self.close_record()?;
                } else {
                    self.arm(i, matches!(c, Command::Resample(_)))?;
                }
            }
            Command::Pad(i, select) => {
                let i = Self::valid_track(i)?;
                self.selected = i;
                if !select {
                    if self.recorder.is_some_and(|r| r.track == i) {
                        self.close_record()?;
                    } else if self.tracks[i].empty() {
                        self.arm(i, false)?
                    } else {
                        self.tracks[i].playing = !self.tracks[i].playing;
                        if !self.tracks[i].playing {
                            self.release_track(i)
                        } else {
                            self.running = true;
                            self.tracks[i].cursor = 0;
                            self.tracks[i].last_phase = u64::MAX;
                        }
                    }
                }
            }
            Command::CancelRecord => self.cancel_capture(),
            Command::Stop => {
                if let Some(mut r) = self.recorder {
                    if r.finish_started {
                        r.stop_after_cycle = true;
                        self.recorder = Some(r);
                    } else {
                        self.cancel_capture();
                    }
                }
                self.running = false;
                for i in 0..16 {
                    self.tracks[i].playing = false;
                    self.release_track(i)
                }
            }
            Command::PlayAll => {
                self.running = true;
                for t in &mut self.tracks {
                    if !t.empty() {
                        t.playing = true;
                        t.cursor = 0;
                        t.last_phase = u64::MAX
                    }
                }
            }
            Command::Clear(i) => {
                let i = Self::valid_track(i)?;
                if self.recorder.is_some_and(|r| r.track == i) {
                    self.recorder = None
                }
                self.release_track(i);
                let t = &mut self.tracks[i];
                t.playing = false;
                t.generated = false;
                t.generated_sounds = 0;
                t.pattern_undo = false;
                t.length_ticks = 0;
                t.audio_frames = 0;
                t.source_frames = 0;
                t.chops = crate::chop::Chops::default();
                t.chop_undo = None;
                t.wave_view = (0, 0);
                *t.features = crate::features::Features::default();
                t.events.clear();
                t.revision += 1;
            }
            Command::Bpm(b) => {
                if !b.is_finite() || !(40.0..=240.0).contains(&b) {
                    return Err(Error::InvalidValue);
                }
                if self.recorder.is_some() || self.tracks.iter().any(|t| !t.empty()) {
                    return Err(Error::TempoFrozen);
                }
                self.clock.bpm = b
            }
            Command::Preroll(n) => {
                if n > 2 {
                    return Err(Error::InvalidValue);
                }
                self.preroll = n
            }
            Command::Level(i, v) | Command::Pan(i, v) => {
                let i = Self::valid_track(i)?;
                if !v.is_finite() {
                    return Err(Error::InvalidValue);
                }
                if matches!(c, Command::Level(..)) {
                    self.tracks[i].mix.level = v.clamp(0., 1.)
                } else {
                    self.tracks[i].mix.pan = v.clamp(-1., 1.)
                }
            }
            Command::Send(i, room, v) => {
                let i = Self::valid_track(i)?;
                if !v.is_finite() {
                    return Err(Error::InvalidValue);
                }
                if room {
                    self.tracks[i].mix.room = v.clamp(0., 1.)
                } else {
                    self.tracks[i].mix.delay = v.clamp(0., 1.)
                }
            }
            Command::Mute(i) => {
                let i = Self::valid_track(i)?;
                self.tracks[i].mix.mute = !self.tracks[i].mix.mute
            }
            Command::Solo(i) => {
                let i = Self::valid_track(i)?;
                self.tracks[i].mix.solo = !self.tracks[i].mix.solo
            }
            Command::Master(v) => {
                if !v.is_finite() {
                    return Err(Error::InvalidValue);
                }
                self.master = v.clamp(0., 1.)
            }
            Command::Output(i, target) => {
                Self::valid_track(i)?;
                let idle = self.is_idle();
                self.separate_usb |= target != OutputTarget::Both;
                let t = &mut self.tracks[i];
                t.output = target;
                if idle {
                    t.main_gain = if target == OutputTarget::Usb { 0. } else { 1. };
                    t.usb_gain = if target == OutputTarget::Main { 0. } else { 1. };
                }
            }
            Command::Route(i, r) => {
                let i = Self::valid_track(i)?;
                self.tracks[i].route = r
            }
            Command::Note {
                track,
                note,
                velocity,
                on,
                source,
            } => {
                let track = Self::valid_track(track)?;
                if note > 127 || velocity > 127 {
                    return Err(Error::InvalidValue);
                }
                if self.tracks[track].kind == Kind::Audio {
                    return Err(Error::WrongKind);
                }
                self.live_note(track, note, velocity, on, source);
            }
            Command::SetFx { track, slot, fx } => {
                let track = Self::valid_track(track)?;
                if slot >= 2 || !fx.valid() {
                    return Err(Error::InvalidValue);
                }
                self.tracks[track].fx[slot] = fx;
                self.held_fx &= !Self::fx_bit(0, track, slot);
            }
            Command::Performance { slot, effect, on } => {
                if slot >= 2 {
                    return Err(Error::InvalidValue);
                }
                self.performance[slot].effect = if on { effect } else { Effect::Off }
            }
            Command::DrumMix { track, sound, mix } => {
                let track = Self::valid_track(track)?;
                if sound >= 16 || !mix.valid() {
                    return Err(Error::InvalidValue);
                }
                self.tracks[track].drums[sound].mix = mix
            }
            Command::Synth(i, synth) => {
                let i = Self::valid_track(i)?;
                if !synth.valid() {
                    return Err(Error::InvalidValue);
                }
                self.tracks[i].synth = synth
            }
            Command::PerformanceFx { slot, fx } | Command::BusFx { slot, fx } => {
                if slot >= 2 || !fx.valid() {
                    return Err(Error::InvalidValue);
                }
                if matches!(c, Command::BusFx { .. }) {
                    self.bus_fx[slot] = fx;
                    self.held_fx &= !Self::fx_bit(slot as u8 + 2, 0, 0);
                } else {
                    self.performance[slot] = fx;
                    self.held_fx &= !Self::fx_bit(1, 0, slot);
                }
            }
            Command::Edit {
                track,
                index,
                note,
                velocity,
                length,
            } => {
                let track = Self::valid_track(track)?;
                let t = &mut self.tracks[track];
                if index >= t.events.len() || !t.events[index].on || note > 127 || velocity > 127 {
                    return Err(Error::InvalidValue);
                }
                let old = t.events[index];
                let end = t
                    .events
                    .iter()
                    .enumerate()
                    .skip(index + 1)
                    .find(|(_, e)| !e.on && e.note == old.note)
                    .map(|(i, _)| i);
                t.events[index].note = note;
                t.events[index].velocity = velocity;
                if let Some(off) = end {
                    t.events[off].note = note;
                    t.events[off].tick = old
                        .tick
                        .saturating_add(length.max(1))
                        .min(t.length_ticks.saturating_sub(1));
                }
                t.events.sort_unstable_by_key(|e| e.tick);
                let phase = t.phase(self.clock.tick());
                t.cursor = t.events.partition_point(|e| e.tick <= phase);
                t.last_phase = phase;
                t.revision += 1;
                self.release_track(track);
            }
            Command::FxOff => {
                for p in &mut self.performance {
                    p.effect = Effect::Off
                }
            }
        }
        Ok(())
    }
    fn generate_drum_part(&mut self, track: usize, sound: usize) -> Result<(), Error> {
        Self::valid_track(track)?;
        if sound >= 16 {
            return Err(Error::InvalidValue);
        }
        let t = &self.tracks[track];
        if t.kind != Kind::Drum {
            return Err(Error::WrongKind);
        }
        if self.recorder.is_some_and(|r| r.track == track) {
            return Err(Error::Busy);
        }
        if !t.empty() && !t.length_ticks.is_multiple_of(4 * PPQ) {
            return Err(Error::NotEmpty);
        }
        if t.generated_sounds & (1 << sound) == 0
            && t.events.iter().any(|e| usize::from(e.note) % 16 == sound)
        {
            return Err(Error::NotEmpty);
        }
        let cfg = crate::music::next_config(t.pattern, sound);
        let generated = crate::music::generate_sound(cfg, self.clock.bpm, sound as u8);
        if generated.len == 0 {
            return Err(Error::InvalidValue);
        }
        let retained = t
            .events
            .iter()
            .filter(|e| usize::from(e.note) % 16 != sound)
            .count();
        let bars = if t.empty() {
            1
        } else {
            (t.length_ticks / (4 * PPQ)) as usize
        };
        if retained + generated.len * 2 * bars > MAX_EVENTS {
            return Err(Error::Capacity);
        }
        let now = self.clock.tick();
        let was_running = self.running;
        let t = &mut self.tracks[track];
        t.previous_events.clear();
        t.previous_events.extend_from_slice(&t.events);
        t.previous_clip = (
            t.length_ticks,
            t.origin,
            t.playing,
            t.generated,
            t.generated_sounds,
        );
        t.pattern_undo = true;
        t.last_generated_sound = Some(sound);
        if t.empty() {
            t.origin = if was_running {
                now / (4 * PPQ) * (4 * PPQ)
            } else {
                now
            };
        }
        t.events.retain(|e| usize::from(e.note) % 16 != sound);
        t.pattern = cfg;
        t.length_ticks = bars as u64 * 4 * PPQ;
        t.audio_frames = 0;
        t.generated = true;
        t.generated_sounds |= 1 << sound;
        t.playing = true;
        for bar in 0..bars {
            for hit in &generated.hits[..generated.len] {
                let base = bar as u64 * 4 * PPQ;
                let tick = base + crate::music::event_tick(hit.tick);
                t.events.push(Event {
                    tick,
                    note: hit.sound,
                    velocity: hit.velocity,
                    on: true,
                });
                t.events.push(Event {
                    tick: (tick + crate::music::event_tick(hit.gate)).min(base + 4 * PPQ - 1),
                    note: hit.sound,
                    velocity: 0,
                    on: false,
                });
            }
        }
        t.events.sort_unstable_by_key(|e| (e.tick, e.on));
        let phase = t.phase(now);
        t.cursor = t.events.partition_point(|e| e.tick < phase);
        t.last_phase = phase;
        t.revision += 1;
        self.running = true;
        Ok(())
    }
    pub fn visible_events(&self, i: usize) -> &[Event] {
        if self.recorder.is_some_and(|r| r.track == i) && self.tracks[i].kind != Kind::Audio {
            &self.capture_events
        } else {
            &self.tracks[i].events
        }
    }
    pub fn status(&self, i: usize) -> Status {
        let t = &self.tracks[i];
        if let Some(r) = self.recorder.filter(|r| r.track == i) {
            if self.clock.tick() < r.start {
                return Status::CountIn;
            }
            if !r.finish_started {
                return Status::Record;
            }
            return Status::Finishing;
        }
        if t.empty() {
            Status::Empty
        } else if t.playing {
            Status::Play
        } else {
            Status::Stop
        }
    }
    pub fn kill_layer_voices(&mut self, track: usize, sound: usize, layer: usize) {
        for voice in &mut self.voices {
            if voice.track == track && voice.drum == Some(sound) && voice.layer == layer {
                voice.active = false;
            }
        }
    }
    pub fn kill_track_voices(&mut self, track: usize) {
        for v in &mut self.voices {
            if v.track == track {
                v.active = false;
            }
        }
    }
    fn release_track(&mut self, i: usize) {
        for voice in &mut self.slice_voices {
            if voice.track == i {
                voice.release = true;
            }
        }
        for held in &mut self.held_chords {
            if held.track == i {
                held.len = 0;
            }
        }
        for v in &mut self.voices {
            if v.track == i {
                v.release()
            }
        }
        self.open_notes[i] = [false; 128]
    }
    fn note(&mut self, track: usize, note: u8, velocity: u8, on: bool, source: u16) {
        if !on || velocity == 0 {
            for v in &mut self.voices {
                if v.active && v.track == track && v.note == note && v.source == source {
                    v.release()
                }
            }
            return;
        }
        let drum = if self.tracks[track].kind == Kind::Drum {
            Some(usize::from(note) % 16)
        } else {
            None
        };
        if drum == Some(2) {
            for v in &mut self.voices {
                if v.track == track && v.drum == Some(3) {
                    v.active = false;
                }
            }
        }
        let settings = drum.map(|d| self.tracks[track].drums[d]);
        let earliest = settings.map_or(0., |d| {
            d.layers
                .iter()
                .filter(|l| l.enabled)
                .map(|l| l.shift)
                .fold(0., f32::min)
        });
        for layer in 0..if drum.is_some() { 2 } else { 1 } {
            if settings.is_some_and(|d| !d.layers[layer].enabled) {
                continue;
            }
            let i = self
                .voices
                .iter()
                .position(|v| !v.active)
                .unwrap_or_else(|| {
                    self.voices
                        .iter()
                        .enumerate()
                        .min_by_key(|(_, v)| v.age())
                        .unwrap()
                        .0
                });
            self.age += 1;
            let tune = settings.map_or(0., |d| d.tune + d.layers[layer].tune);
            self.voices[i].start(
                track,
                note,
                velocity,
                source,
                drum,
                self.age,
                self.tracks[track].synth,
                tune,
            );
            self.voices[i].layer = layer;
            self.voices[i].delay_frames = settings.map_or(0, |d| {
                ((d.layers[layer].shift - earliest) * self.clock.rate as f32 / 1000.).round() as u32
            });
        }
    }

    fn live_note(&mut self, i: usize, note: u8, velocity: u8, on: bool, source: u16) {
        // MIDI commands may arrive on the exact frame of a wrap, before process().
        if self.tracks[i].kind != Kind::Audio && self.recorder.is_some_and(|r| r.track == i) {
            self.process_capture(Frame::ZERO);
        }
        self.note(i, note, velocity, on, source);
        self.capture_note(i, note, velocity, on);
    }

    pub fn process(&mut self, input: Frame) -> Frame {
        let mut profile = crate::qa::dsp_start(|| self.active_voices());
        let tick = self.clock.tick();
        let rate = self.clock.rate as f32;
        self.process_capture(input);
        if self.running {
            for i in 0..16 {
                if self.tracks[i].playing
                    && self.tracks[i].kind != Kind::Audio
                    && !self.tracks[i].empty()
                {
                    let phase = self.tracks[i].phase(tick);
                    if phase < self.tracks[i].last_phase {
                        for v in &mut self.voices {
                            if v.track == i && v.source == 1 {
                                v.release()
                            }
                        }
                        self.tracks[i].cursor = 0
                    }
                    while self.tracks[i].cursor < self.tracks[i].events.len() {
                        let e = self.tracks[i].events[self.tracks[i].cursor];
                        if e.tick > phase {
                            break;
                        }
                        self.tracks[i].cursor += 1;
                        let suppressed = self.recorder.is_some_and(|r| {
                            r.track == i
                                && tick >= r.start
                                && r.mode == RecordMode::Replace
                                && (self.tracks[i].kind == Kind::Synth
                                    || r.touched & (1 << (e.note % 16)) != 0)
                        });
                        if !suppressed {
                            self.note(i, e.note, e.velocity, e.on, 1)
                        }
                    }
                    self.tracks[i].last_phase = phase;
                }
            }
        }
        let mut signals = [Frame::ZERO; 16];
        for voice in &mut self.slice_voices {
            if voice.active {
                let t = &self.tracks[voice.track];
                if t.kind != Kind::Audio || t.audio_frames == 0 {
                    voice.active = false;
                    continue;
                }
                let sample = voice.sample(&t.audio[..t.audio_frames], self.clock.rate);
                signals[voice.track] = signals[voice.track].combine(sample);
            }
        }

        let mut drum_delay = [Frame::ZERO; 16];
        let mut drum_room = [Frame::ZERO; 16];
        for t in &mut self.tracks {
            if t.kind != Kind::Drum {
                continue;
            }
            for j in 0..16 {
                t.sound_gain[j] += (t.drums[j].mix.level - t.sound_gain[j]) * 0.002
            }
        }
        crate::qa::dsp_mark(&mut profile, 0);
        let mut trigger_sound = Frame::ZERO;
        for v in &mut self.voices {
            if v.active {
                let t = &self.tracks[v.track];
                let drum = v.drum.map_or(Drum::default(), |d| t.drums[d]);
                if v.delay_frames > 0 {
                    v.delay_frames -= 1;
                    continue;
                }
                let mut adjusted = drum;
                let mut layer_gain = 1.;
                if v.drum.is_some() {
                    let cfg = drum.layers[v.layer];
                    adjusted.tune += cfg.tune;
                    if cfg.decay > 0. {
                        adjusted.decay = cfg.decay;
                    }
                    layer_gain = if cfg.enabled { cfg.level } else { 0. };
                }
                let mut sample =
                    if let Some(d) = v.drum.filter(|d| !t.kit.clip(*d, v.layer).is_empty()) {
                        v.sample_pcm(t.kit.clip(d, v.layer), rate, adjusted)
                    } else {
                        Frame::mono(v.sample(rate, &t.synth, adjusted))
                    };
                sample = v.apply_layer_gain(sample, layer_gain);
                if let Some(d) = v.drum {
                    let anysolo = t.drums.iter().any(|s| s.mix.solo);
                    if drum.mix.mute || (anysolo && !drum.mix.solo) {
                        sample = Frame::ZERO
                    } else {
                        sample = drum.mix.apply(sample, t.sound_gain[d])
                    }
                    if v.track == usize::from(self.sidechain.source)
                        && d == usize::from(self.sidechain.sound)
                    {
                        trigger_sound = trigger_sound.combine(sample);
                    }
                    drum_delay[v.track] = drum_delay[v.track].combine(sample.gain(drum.mix.delay));
                    drum_room[v.track] = drum_room[v.track].combine(sample.gain(drum.mix.room));
                }
                signals[v.track] = signals[v.track].combine(sample);
            }
        }
        crate::qa::dsp_mark(&mut profile, 1);
        let solo = self.tracks.iter().any(|t| t.mix.solo);
        let mut sum = Frame::ZERO;
        let split = self.separate_usb;
        let mut usb_sum = Frame::ZERO;
        let mut usb_buses = [Frame::ZERO; 2];
        let mut usb_ds = Frame::ZERO;
        let mut usb_rs = Frame::ZERO;
        let mut buses = [Frame::ZERO; 2];
        let mut ds = Frame::ZERO;
        let mut rs = Frame::ZERO;
        for (i, signal) in signals.iter_mut().enumerate() {
            let t = &self.tracks[i];
            if self.running
                && t.playing
                && t.kind == Kind::Audio
                && t.audio_frames > 0
                && !t.empty()
            {
                let pos = t.phase(tick) as f64 / t.length_ticks as f64 * t.audio_frames as f64;
                let j = pos.floor() as usize % t.audio_frames;
                let k = (j + 1) % t.audio_frames;
                let f = pos.fract() as f32;
                let edge = (j.min(t.audio_frames - 1 - j) as f32 / (rate * 0.002)).min(1.);
                *signal = signal.combine(
                    t.audio[j]
                        .gain(1. - f)
                        .combine(t.audio[k].gain(f))
                        .gain(edge),
                );
            }
        }
        let source = usize::from(self.sidechain.source);
        let st = &self.tracks[source];
        let audible = !st.mix.mute && (!solo || st.mix.solo);
        let detector = if !audible {
            0.
        } else {
            let raw = if self.sidechain.sound < 16 && st.kind == Kind::Drum {
                trigger_sound
            } else {
                signals[source]
            };
            raw.peak() * st.gain
        };
        let duck_gain = self
            .duck
            .sample(detector, self.sidechain, rate, self.clock.bpm as f32);
        for (i, signal) in signals.iter().enumerate() {
            let t = &mut self.tracks[i];
            let target = if t.mix.mute || (solo && !t.mix.solo) {
                0.
            } else {
                t.mix.level
            };
            t.gain += (target - t.gain) * 0.002;
            let dg = if self.sidechain.targets(i) {
                duck_gain
            } else {
                1.
            };
            let mut s = *signal;
            for slot in 0..2 {
                s = self.track_fx[i][slot].process(s, t.fx[slot], rate, self.clock.bpm as f32)
            }
            t.duck_gain += (dg - t.duck_gain) * 0.004;
            let dg = t.duck_gain;
            s = s.gain(dg);
            let mut mix = t.mix;
            if t.kind == Kind::Synth && t.synth.lfo_pan > 0. {
                let pan = (mix.pan
                    + t.pan_lfo.sample(rate, t.synth.lfo_rate, t.synth.lfo_shape)
                        * t.synth.lfo_pan)
                    .clamp(-1., 1.);
                t.pan_current += (pan - t.pan_current) * 0.004;
                mix.pan = t.pan_current;
            } else {
                t.pan_current = mix.pan;
            }
            s = mix.apply(s, t.gain);
            t.meter = (t.meter * 0.999).max(s.peak());
            let delay = s
                .gain(t.mix.delay)
                .combine(mix.apply(drum_delay[i], t.gain * dg));
            let room = s
                .gain(t.mix.room)
                .combine(mix.apply(drum_room[i], t.gain * dg));
            let main_target = if t.output == OutputTarget::Usb {
                0.
            } else {
                1.
            };
            let usb_target = if t.output == OutputTarget::Main {
                0.
            } else {
                1.
            };
            t.main_gain += (main_target - t.main_gain) * 0.002;
            t.usb_gain += (usb_target - t.usb_gain) * 0.002;
            ds = ds.combine(delay.gain(t.main_gain));
            rs = rs.combine(room.gain(t.main_gain));
            let main = s.gain(t.main_gain);
            match t.route {
                Route::Dry => sum = sum.combine(main),
                Route::A => buses[0] = buses[0].combine(main),
                Route::B => buses[1] = buses[1].combine(main),
            }
            if split {
                usb_ds = usb_ds.combine(delay.gain(t.usb_gain));
                usb_rs = usb_rs.combine(room.gain(t.usb_gain));
                let usb = s.gain(t.usb_gain);
                match t.route {
                    Route::Dry => usb_sum = usb_sum.combine(usb),
                    Route::A => usb_buses[0] = usb_buses[0].combine(usb),
                    Route::B => usb_buses[1] = usb_buses[1].combine(usb),
                }
            }
        }
        crate::qa::dsp_mark(&mut profile, 2);
        for (i, bus) in buses.iter().enumerate() {
            sum = sum.combine(self.bus_state[i].process(
                *bus,
                self.bus_fx[i],
                rate,
                self.clock.bpm as f32,
            ));
        }
        sum = sum
            .combine(self.delay.process(ds, rate, self.clock.bpm as f32))
            .combine(self.room.process(rs));
        for i in 0..2 {
            let mut fx = self.performance[i];
            if self.texture.master_bypass[i] {
                fx.effect = Effect::Off;
            }
            sum = self.perf_state[i].process(sum, fx, rate, self.clock.bpm as f32)
        }
        sum = self
            .master_state
            .process(sum, self.master_fx, rate, self.clock.bpm as f32);
        sum = self
            .texture_main
            .process(sum, self.texture, rate, self.clock.bpm as f32);
        let master_target = if self.sidechain.mode == 2 {
            duck_gain
        } else {
            1.
        };
        self.master_duck_gain += (master_target - self.master_duck_gain) * 0.004;
        sum = sum.gain(self.master_duck_gain);
        self.master_gain += (self.master - self.master_gain) * 0.002;
        sum = sum.gain(self.master_gain);
        sum = Frame {
            l: sum.l.clamp(-0.98, 0.98),
            r: sum.r.clamp(-0.98, 0.98),
        };
        if !sum.finite() {
            sum = Frame::ZERO;
            self.error = Some(Error::InvalidValue)
        }
        self.usb_output = if split {
            self.usb_stage.process(
                usb_sum,
                usb_buses,
                usb_ds,
                usb_rs,
                rate,
                self.clock.bpm as f32,
                self.bus_fx,
                self.performance,
                self.master_fx,
                self.master_gain,
                &mut self.texture_usb,
                self.texture,
                self.master_duck_gain,
            )
        } else {
            sum
        };
        self.usb_meter = Frame {
            l: (self.usb_meter.l * 0.999).max(self.usb_output.l.abs()),
            r: (self.usb_meter.r * 0.999).max(self.usb_output.r.abs()),
        };
        crate::qa::dsp_mark(&mut profile, 3);
        self.capture_resample(sum);
        self.previous_master = sum;
        self.master_meter = Frame {
            l: (self.master_meter.l * 0.999).max(sum.l.abs()),
            r: (self.master_meter.r * 0.999).max(sum.r.abs()),
        };
        if self.running || self.recorder.is_some_and(|r| r.finish_started) {
            self.clock.frame += 1
        }
        sum
    }
    pub fn is_idle(&self) -> bool {
        !self.running
            && self.recorder.is_none()
            && self.active_voices() == 0
            && !self.slice_voices.iter().any(|v| v.active)
    }
    pub fn emergency_silence(&mut self) {
        for voice in &mut self.voices {
            voice.active = false;
        }
        for voice in &mut self.slice_voices {
            voice.active = false;
        }
        self.held_chords = [HeldChord::default(); 16];
        self.release_temporary_fx();
    }
    pub fn reset_chop_history(&mut self, track: usize) {
        self.tracks[track].chop_undo = None;
    }
    pub fn clear_slice_voices(&mut self, track: usize) {
        for voice in &mut self.slice_voices {
            if voice.track == track {
                voice.active = false;
            }
        }
    }
    pub fn slice_activity(&self, track: usize) -> u16 {
        self.slice_voices
            .iter()
            .filter(|v| v.active && v.track == track)
            .fold(0, |mask, v| mask | (1 << v.pad))
    }
    pub fn active_voices(&self) -> usize {
        self.voices.iter().filter(|v| v.active).count()
    }
}
