use crate::{
    clock::Clock,
    dsp::{Delay, FxState, Room, Voice},
    model::*,
};
pub const MAX_EVENTS: usize = 4096;
// Measured Sample1.3.0.12 budget:32shared voices;64exceeded steady deadlines.
pub const VOICES: usize = 32;
pub struct Track {
    pub deck: crate::deck::Settings,
    schedule: Vec<Event>,
    schedule_revision: u64,
    schedule_swing: Option<crate::swing::Basis>,
    pub kit: Box<crate::bank::DrumKit>,
    pub record_mode: RecordMode,
    pub source_frames: usize,
    pub source_ticks: u64,
    pub audio_sequence: bool,
    pub audio_settings: crate::audio::Settings,
    pub audio_target: crate::audio::RecordTarget,
    audio_player: crate::audio::LoopPlayer,
    pub features: Box<crate::features::Features>,
    pub wave_view: (u32, u32),
    last_chord: ([u8; 5], usize),
    pub chops: crate::chop::Chops,
    pub generated: bool,
    pub generated_sounds: u16,
    last_generated_sound: Option<usize>,
    pub pattern: crate::music::PatternConfig,
    pub chord: crate::music::ChordConfig,
    pub chord_mode: bool,
    pub play: crate::performance::Settings,
    strum_reverse: bool,
    pub xpm_step: [u32; 16],
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
            deck: crate::deck::Settings::default(),
            schedule: Vec::with_capacity(MAX_EVENTS),
            schedule_revision: u64::MAX,
            schedule_swing: None,
            kit: Box::new(crate::bank::DrumKit::default()),
            record_mode: RecordMode::Replace,
            source_frames: 0,
            source_ticks: 0,
            audio_sequence: false,
            audio_settings: crate::audio::Settings::default(),
            audio_target: crate::audio::RecordTarget::Input,
            audio_player: crate::audio::LoopPlayer::default(),
            features: Box::new(crate::features::Features::default()),
            wave_view: (0, 0),
            last_chord: ([0; 5], 0),
            chops: crate::chop::Chops::default(),
            generated: false,
            generated_sounds: 0,
            last_generated_sound: None,
            pattern: crate::music::PatternConfig::default(),
            chord: crate::music::ChordConfig::default(),
            chord_mode: false,
            play: crate::performance::Settings::default(),
            strum_reverse: false,
            xpm_step: [0; 16],
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
    // File worker only: keep full recording capacity and MAX_EVENTS after copying.
    fn duplicate_stopped(&self, capacity: usize, index: usize) -> Self {
        let mut out = Self::new(capacity, index);
        out.kit = self.kit.clone();
        out.features = self.features.clone();
        out.name = self.name.clone();
        let frames = self
            .source_frames
            .max(self.audio_frames)
            .min(self.audio.len());
        out.audio[..frames].copy_from_slice(&self.audio[..frames]);
        out.events.extend_from_slice(&self.events);
        macro_rules! copy {($($f:ident),*)=>{$(out.$f=self.$f;)*};}
        copy!(
            deck,
            record_mode,
            source_frames,
            source_ticks,
            origin,
            audio_sequence,
            audio_settings,
            audio_target,
            wave_view,
            chops,
            generated,
            generated_sounds,
            pattern,
            chord,
            chord_mode,
            play,
            kind,
            mix,
            synth,
            drums,
            fx,
            route,
            output,
            main_gain,
            usb_gain,
            length_ticks,
            audio_frames
        );
        out.gain = out.mix.level;
        out.pan_current = out.mix.pan;
        out.sound_gain = out.drums.map(|d| d.mix.level);
        out
    }
    pub(crate) fn reset_audio_player(&mut self) {
        self.audio_player.reset();
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
    pub midi: bool,
    pub preserve_on_cancel: bool,
    pub previous_origin: u64,
    pub previous_playing: bool,
    pub track: usize,
    pub start: u64,
    pub end: Option<u64>,
    pub limit: u64,
    pub resample: bool,
    pub finish_started: bool,
    pub notes_started: bool,
    pub repeating: bool,
    pub stop_after_cycle: bool,
    pub mode: RecordMode,
    pub touched: u16,
}
#[derive(Clone, Copy, Debug)]
pub enum Command {
    DeckGrab(usize, bool, Option<f32>),
    DeckFader(usize, f32),
    DeckValue(usize, u8, f32),
    Swing(Option<crate::swing::Basis>),
    PlayValue {
        track: usize,
        index: u8,
        value: f32,
    },
    AudioTarget(usize, crate::audio::RecordTarget),
    AudioSettings(usize, crate::audio::Settings),
    Undo,
    Redo,
    HardwareVolume(f32),
    Sidechain(crate::motion::Sidechain),
    SidechainHeld(bool),
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
    ChainAppend(usize),
    ChainRemove(usize),
    ChainRepeats(usize, u8),
    ChainRepeat(bool),
    ChainToggle,
    ChainClear,
    LoopRecord(usize),
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
    ChordMode(usize, bool),
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
    next: usize,
    velocity: u8,
    step: u32,
    countdown: u32,
}
#[path = "history.rs"]
mod history;
use history::{ClipState, EditKind, History};
#[path = "recording.rs"]
mod recording;

pub struct Engine {
    pub decks: Box<[crate::deck::Player; 16]>,
    deck_spans: Box<crate::deck::Spans>,
    pub swing: Option<crate::swing::Basis>,
    history: History,
    record_before: Option<ClipState>,
    record_history_committed: bool,
    pub chain: crate::chain::Chain,
    pub loop_revision: u64,
    input_preview: [crate::features::Peak; 128],
    input_preview_head: usize,
    input_preview_count: u32,
    capture_audio: Vec<Frame>,
    capture_prepared: usize,
    capture_features: Box<crate::features::Features>,
    capture_events: Vec<Event>,
    capture_velocities: [u8; 128],
    held_chords: [HeldChord; 16],
    pending_chords: u16,
    mono: Box<[crate::performance::Mono; 16]>,
    slice_voices: [crate::chop::SliceVoice; 16],
    held_fx: u64,
    pub clock: Clock,
    pub tracks: [Track; 16],
    pub selected: usize,
    pub running: bool,
    pub preroll: u8,
    pub recorder: Option<Recorder>,
    pub next_record: Option<usize>,
    pub latency_in: u32,
    pub master: f32,
    pub master_gain: f32,
    pub hardware_volume: f32,
    pub hardware_gain: f32,
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
    pub fn new_project(rate: u32, frames_per_track: usize) -> Self {
        Self::new(rate, DEFAULT_BPM, frames_per_track)
    }
    pub fn new(rate: u32, bpm: f64, frames_per_track: usize) -> Self {
        let rate = rate.clamp(8000, 96000);
        Self {
            decks: Box::new([crate::deck::Player::default(); 16]),
            deck_spans: Box::default(),
            swing: None,
            history: History::new(frames_per_track),
            record_before: None,
            record_history_committed: false,
            chain: crate::chain::Chain::default(),
            loop_revision: 0,
            input_preview: [crate::features::Peak::default(); 128],
            input_preview_head: 0,
            input_preview_count: 0,
            capture_audio: vec![Frame::ZERO; frames_per_track],
            capture_prepared: 0,
            capture_features: Box::new(crate::features::Features::default()),
            capture_events: Vec::with_capacity(MAX_EVENTS),
            capture_velocities: [0; 128],
            held_chords: [HeldChord::default(); 16],
            pending_chords: 0,
            mono: Box::new([crate::performance::Mono::default(); 16]),
            slice_voices: [crate::chop::SliceVoice::default(); 16],
            held_fx: 0,
            clock: Clock::new(rate, bpm),
            tracks: std::array::from_fn(|i| Track::new(frames_per_track, i)),
            selected: 0,
            running: false,
            preroll: 1,
            recorder: None,
            next_record: None,
            latency_in: 0,
            master: 0.8,
            master_gain: 0.8,
            hardware_volume: 1.,
            hardware_gain: 1.,
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
        if i < ACTIVE_TRACKS {
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
    pub fn saved_texture(&self) -> crate::motion::Texture {
        let mut texture = self.texture;
        for slot in 0..2 {
            if self.held_fx & Self::fx_bit(1, 0, slot) != 0 {
                texture.master_bypass[slot] = true;
            }
        }
        for slot in 0..5 {
            if self.held_fx & (1u64 << (36 + slot)) != 0 {
                texture.enabled[slot] = false;
            }
        }
        texture
    }
    pub fn saved_sidechain(&self) -> crate::motion::Sidechain {
        let mut c = self.sidechain;
        if self.held_fx & (1u64 << 41) != 0 {
            c.enabled = false;
        }
        c
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
        for slot in 0..5 {
            if self.held_fx & (1u64 << (36 + slot)) != 0 {
                self.texture.enabled[slot] = false;
            }
        }
        if self.held_fx & (1u64 << 41) != 0 {
            self.sidechain.enabled = false;
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
        // Clip-edit checkpoints must not replace an in-flight recording's history.
        if self.recorder.is_some()
            && matches!(
                c,
                Command::ChopApply(_)
                    | Command::SplitSlice { .. }
                    | Command::DeleteSlice { .. }
                    | Command::ResetSlices(_)
                    | Command::SliceValue { .. }
                    | Command::Edit { .. }
                    | Command::GenerateDrumPart { .. }
            )
        {
            return Err(Error::Busy);
        }
        match c {
            Command::PlayValue {
                track,
                index,
                value,
            } => {
                Self::valid_track(track)?;
                if !value.is_finite() {
                    return Err(Error::InvalidValue);
                }
                let mut cfg = self.tracks[track].play;
                match index {
                    0 => cfg.voice = value.round().clamp(0., 2.) as u8,
                    1 => cfg.glide_ms = value,
                    2 => cfg.trigger = value.round().clamp(0., 2.) as u8,
                    3 => cfg.shot_ms = value,
                    4 => cfg.octave = value.round().clamp(-2., 2.) as i8,
                    _ => return Err(Error::InvalidValue),
                }
                if !cfg.valid() {
                    return Err(Error::InvalidValue);
                }
                if (cfg.voice != self.tracks[track].play.voice
                    || cfg.trigger != self.tracks[track].play.trigger)
                    && self.recorder.is_some_and(|r| r.track == track)
                {
                    return Err(Error::Busy);
                }
                if cfg.voice != self.tracks[track].play.voice
                    || cfg.trigger != self.tracks[track].play.trigger
                {
                    self.release_track(track);
                    self.kill_track_voices(track);
                }
                if cfg.octave != self.tracks[track].play.octave {
                    self.tracks[track].last_chord = ([0; 5], 0);
                }
                self.tracks[track].play = cfg;
            }
            Command::DeckValue(i, index, value) => {
                Self::valid_track(i)?;
                if self.recorder.is_some_and(|r| r.track == i) {
                    return Err(Error::Busy);
                }
                if !value.is_finite() || self.tracks[i].kind != Kind::Audio {
                    return Err(Error::InvalidValue);
                }
                match index {
                    0 => self.tracks[i].deck.speed = value.clamp(0.5, 2.),
                    1 => self.tracks[i].deck.range = value.round().clamp(0., 2.) as u8,
                    2 => self.tracks[i].deck.slip = value >= 0.5,
                    _ => return Err(Error::InvalidValue),
                }
            }
            Command::DeckGrab(i, on, fader) => {
                Self::valid_track(i)?;
                if fader.is_some_and(|f| !f.is_finite() || !(0.0..=1.0).contains(&f)) {
                    return Err(Error::InvalidValue);
                }
                if on
                    && (self.tracks[i].kind != Kind::Audio
                        || self.tracks[i].audio_target != crate::audio::RecordTarget::Turntable
                        || self.tracks[i].source_frames == 0)
                {
                    return Err(Error::NoContent);
                }
                if self.recorder.is_some_and(|r| r.track == i && r.midi) {
                    let overflows = self.event_overflows;
                    self.process_capture(Frame::ZERO);
                    if self.event_overflows != overflows && self.recorder.is_none() {
                        return Err(Error::Capacity);
                    }
                }
                self.decks[i].grab(on, fader, self.tracks[i].deck, self.clock.rate);
                self.capture_deck(i)?;
            }
            Command::DeckFader(i, value) => {
                Self::valid_track(i)?;
                if !value.is_finite() || !(0.0..=1.0).contains(&value) {
                    return Err(Error::InvalidValue);
                }
                if self.recorder.is_some_and(|r| r.track == i && r.midi) {
                    let overflows = self.event_overflows;
                    self.process_capture(Frame::ZERO);
                    if self.event_overflows != overflows && self.recorder.is_none() {
                        return Err(Error::Capacity);
                    }
                }
                if self.decks[i].move_to(
                    value,
                    self.tracks[i].deck,
                    self.clock.bpm,
                    self.clock.rate,
                ) {
                    self.capture_deck(i)?;
                }
            }
            Command::AudioTarget(i, target) => {
                Self::valid_track(i)?;
                if self.tracks[i].kind != Kind::Audio {
                    return Err(Error::WrongKind);
                }
                if self.recorder.is_some_and(|r| r.track == i) {
                    return Err(Error::Busy);
                }
                self.release_track(i);
                self.tracks[i].audio_target = target;
                if target != crate::audio::RecordTarget::Chromatic {
                    self.tracks[i].chord_mode = false;
                }
            }
            Command::AudioSettings(i, settings) => {
                Self::valid_track(i)?;
                if !settings.valid() {
                    return Err(Error::InvalidValue);
                }
                self.tracks[i].audio_settings = settings;
            }
            Command::Undo => self.swap_history(false)?,
            Command::Redo => self.swap_history(true)?,
            Command::HardwareVolume(value) => {
                if !value.is_finite() || !(0.0..=1.0).contains(&value) {
                    return Err(Error::InvalidValue);
                }
                self.hardware_volume = value;
            }
            Command::MasterBypass { slot, bypass } => {
                if slot >= 2 {
                    return Err(Error::InvalidValue);
                }
                self.texture.master_bypass[slot] = bypass;
            }
            Command::SidechainHeld(held) => {
                if held {
                    self.held_fx |= 1u64 << 41;
                } else {
                    self.held_fx &= !(1u64 << 41);
                }
            }
            Command::Sidechain(cfg) => {
                if !cfg.valid() {
                    return Err(Error::InvalidValue);
                }
                self.sidechain = cfg;
            }
            Command::TextureValue { slot, index, value } => {
                if slot >= 5 || index > 7 || !value.is_finite() || !(0.0..=1.0).contains(&value) {
                    return Err(Error::InvalidValue);
                }
                if index == 7 {
                    let bit = 1u64 << (36 + slot);
                    if value >= 0.5 {
                        self.held_fx |= bit;
                    } else {
                        self.held_fx &= !bit;
                    }
                } else if index >= 4 {
                    let m = &mut self.texture.modulation[slot];
                    match index {
                        4 => m.sync = value >= 0.5,
                        5 => m.division = (value * 15.99) as u8,
                        _ => m.shape = (value * 5.99) as u8,
                    }
                } else if index == 3 {
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
                self.history.checkpoint(track, t, EditKind::ChopMidi);
                let old = t.chops;
                t.chops = c;
                Self::remap_slices(t, old);
                self.clear_slice_voices(track);
            }
            Command::ChopUndo(track) => {
                Self::valid_track(track)?;
                if self.history.target != Some(track)
                    || self.history.undone
                    || !matches!(self.history.kind, EditKind::Chop | EditKind::ChopMidi)
                {
                    return Err(Error::NoContent);
                }
                self.swap_history(false)?;
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
                self.history.checkpoint(track, t, EditKind::ChopMidi);
                let old = t.chops;
                t.chops = c;
                Self::remap_slices(t, old);
                self.clear_slice_voices(track);
            }
            Command::DeleteSlice { track, pad } => {
                Self::valid_track(track)?;
                let t = &mut self.tracks[track];
                let mut c = t.chops;
                if !c.delete(pad) {
                    return Err(Error::InvalidValue);
                }
                self.history.checkpoint(track, t, EditKind::ChopMidi);
                let old = t.chops;
                t.chops = c;
                Self::remap_slices(t, old);
                self.clear_slice_voices(track);
            }

            Command::ResetSlices(track) => {
                Self::valid_track(track)?;
                let t = &mut self.tracks[track];
                if t.kind != Kind::Audio || t.audio_frames == 0 {
                    return Err(Error::NoContent);
                }
                self.history.checkpoint(track, t, EditKind::ChopMidi);
                t.source_frames = t.source_frames.min(t.audio_frames);
                if t.source_frames == 0 {
                    t.source_frames = t.audio_frames
                }
                let old = t.chops;
                t.chops.divide(t.source_frames);
                Self::remap_slices(t, old);
                self.clear_slice_voices(track);
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
                if pad >= 16 || index > 2 || !value.is_finite() || !(0.0..=1.0).contains(&value) {
                    return Err(Error::InvalidValue);
                }
                let t = &mut self.tracks[track];
                if t.kind != Kind::Audio || t.source_frames == 0 {
                    return Err(Error::NoContent);
                }
                if pad >= usize::from(t.chops.count) {
                    return Err(Error::NoContent);
                }
                self.history.checkpoint(track, t, EditKind::Chop);
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
                        gate: match t.play.trigger {
                            1 => true,
                            2 => false,
                            _ => t.chops.gate,
                        },
                        release: false,
                        env: 1.,
                        age: self.age,
                        sampler: None,
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
                if self.history.target != Some(track)
                    || self.history.undone
                    || self.history.kind != EditKind::Midi
                {
                    return Err(Error::NoContent);
                }
                self.swap_history(false)?;
            }
            Command::ChordMode(track, enabled) => {
                Self::valid_track(track)?;
                if enabled
                    && (self.tracks[track].kind == Kind::Drum
                        || self.tracks[track].kind == Kind::Audio
                            && self.tracks[track].audio_target
                                != crate::audio::RecordTarget::Chromatic)
                {
                    return Err(Error::WrongKind);
                }
                if self.recorder.is_some_and(|r| r.track == track) {
                    return Err(Error::Busy);
                }
                if self.tracks[track].chord_mode != enabled {
                    self.release_track(track);
                }
                self.tracks[track].chord_mode = enabled;
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
                    5 => cfg.smooth = value >= 0.5,
                    6 => cfg.strum_ms = value,
                    7 => cfg.strum_direction = value.round().clamp(0., 2.) as u8,
                    _ => return Err(Error::InvalidValue),
                }
                if !cfg.valid() {
                    return Err(Error::InvalidValue);
                }
                self.tracks[track].chord = cfg;
                if index <= 5 {
                    self.tracks[track].last_chord = ([0; 5], 0);
                }
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
                let on = on && velocity > 0;
                if on
                    && (self.tracks[track].kind == Kind::Drum
                        || self.tracks[track].kind == Kind::Audio
                            && self.tracks[track].audio_target
                                != crate::audio::RecordTarget::Chromatic)
                {
                    return Err(Error::WrongKind);
                }
                let previous = self.held_chords[pad];
                // One Shot keeps its already-triggered phrase after physical release.
                if !on && self.tracks[previous.track].play.trigger == 2 {
                    return Ok(());
                }
                self.pending_chords &= !(1 << pad);
                self.held_chords[pad] = HeldChord::default();
                for note in &previous.notes[..previous.next] {
                    self.live_note(previous.track, *note, 0, false, 0x4000 + pad as u16);
                }
                if on && velocity > 0 {
                    let t = &mut self.tracks[track];
                    let (mut notes, len) = t.chord.notes(pad);
                    if len > 0 {
                        let mut shift = i16::from(t.play.octave) * 12;
                        while i16::from(notes[len - 1]) + shift > 127 {
                            shift -= 12;
                        }
                        while i16::from(notes[0]) + shift < 0 {
                            shift += 12;
                        }
                        for note in &mut notes[..len] {
                            *note = (i16::from(*note) + shift) as u8;
                        }
                    }
                    if t.chord.smooth {
                        notes = crate::music::smooth_voicing(
                            notes,
                            len,
                            t.last_chord.0,
                            t.last_chord.1,
                        );
                    }
                    t.last_chord = (notes, len);
                    if t.chord.strum_direction == 1
                        || t.chord.strum_direction == 2 && t.strum_reverse
                    {
                        notes[..len].reverse();
                    }
                    t.strum_reverse = !t.strum_reverse;
                    let step = (t.chord.strum_ms * self.clock.rate as f32 / 1000.).round() as u32;
                    let next = if step == 0 { len } else { len.min(1) };
                    self.held_chords[pad] = HeldChord {
                        track,
                        notes,
                        len,
                        next,
                        velocity,
                        step,
                        countdown: step,
                    };
                    for note in &notes[..next] {
                        self.live_note(track, *note, velocity, true, 0x4000 + pad as u16);
                    }
                    if next < len {
                        self.pending_chords |= 1 << pad;
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
                if !value.is_finite() || !(0.0..=1.0).contains(&value) || index > 5 {
                    return Err(Error::InvalidValue);
                }
                let fx = self.effect_mut(target, track, slot)?;
                match index {
                    0 => fx.a = value,
                    1 => fx.b = value,
                    2 => fx.mix = value,
                    3 => fx.modulation.sync = value >= 0.5,
                    4 => fx.modulation.division = (value * 15.99) as u8,
                    _ => fx.modulation.shape = (value * 5.99) as u8,
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
                    28 => synth.lfo_sync = value >= 0.5,
                    29 => synth.lfo_division = value.round().clamp(0., 15.) as u8,
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
                if self.tracks[i].kind != k {
                    self.history.invalidate(i);
                }
                self.release_track(i);
                self.tracks[i].kind = k
            }
            Command::ChainAppend(track) => {
                Self::valid_track(track)?;
                if self.tracks[track].empty() {
                    return Err(Error::NoContent);
                }
                let index = usize::from(self.chain.config.len);
                if index >= 16 {
                    return Err(Error::Capacity);
                }
                self.chain.config.steps[index] = crate::chain::Step {
                    track: track as u8,
                    repeats: 1,
                };
                self.chain.config.len += 1;
            }
            Command::ChainRemove(index) => {
                if !self.chain.remove(index) {
                    return Err(Error::NoContent);
                }
            }
            Command::ChainRepeats(index, repeats) => {
                if index >= usize::from(self.chain.config.len) || !(1..=16).contains(&repeats) {
                    return Err(Error::InvalidValue);
                }
                self.chain.config.steps[index].repeats = repeats;
            }
            Command::ChainRepeat(repeat) => self.chain.config.repeat = repeat,
            Command::ChainToggle => {
                if self.chain.running {
                    let change = self.chain.stop();
                    self.apply_chain_change(change);
                } else {
                    if self.chain.config.len == 0
                        || self.chain.config.steps[..usize::from(self.chain.config.len)]
                            .iter()
                            .any(|step| self.tracks[usize::from(step.track)].empty())
                    {
                        return Err(Error::NoContent);
                    }
                    if self
                        .recorder
                        .is_some_and(|r| self.chain.config.members() & (1 << r.track) != 0)
                        || self
                            .next_record
                            .is_some_and(|i| self.chain.config.members() & (1 << i) != 0)
                    {
                        return Err(Error::Busy);
                    }
                    self.chain
                        .start(self.clock.bar_at_or_after(self.clock.tick()));
                    self.running = true;
                }
            }
            Command::ChainClear => {
                let change = self.chain.stop();
                self.apply_chain_change(change);
                self.chain.config = crate::chain::Config::default();
            }
            Command::LoopRecord(i) => {
                let i = Self::valid_track(i)?;
                if self.chain_protects(i) {
                    return Err(Error::Busy);
                }
                self.loop_revision = self.loop_revision.wrapping_add(1);
                if let Some(mut r) = self.recorder {
                    if i != r.track {
                        self.check_record_capacity(i)?;
                        if self.clock.tick() < r.start {
                            let start = r.start;
                            self.cancel_capture();
                            self.start_loop_record(i, Some(start))?;
                        } else {
                            r.end = Some(self.clock.next_bar().min(r.limit).max(r.start + 4 * PPQ));
                            r.stop_after_cycle = true;
                            self.next_record = Some(i);
                            self.recorder = Some(r);
                        }
                    } else if self.clock.tick() < r.start {
                        self.cancel_capture();
                    } else {
                        self.next_record = None;
                        r.end = Some(self.clock.next_bar().min(r.limit).max(r.start + 4 * PPQ));
                        r.stop_after_cycle = true;
                        self.recorder = Some(r);
                    }
                } else {
                    self.start_loop_record(i, None)?;
                }
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
                    if self.chain_protects(i) {
                        return Err(Error::Busy);
                    }
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
                            for v in &mut self.slice_voices {
                                if v.track == i && v.sampler.is_some_and(|s| s.source == 1) {
                                    v.release = true;
                                }
                            }
                            self.tracks[i].cursor = 0;
                            self.tracks[i].last_phase = u64::MAX;
                        }
                    }
                }
            }
            Command::CancelRecord => self.cancel_capture(),
            Command::Stop => {
                let change = self.chain.stop();
                self.apply_chain_change(change);
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
                let managed = if self.chain.running {
                    self.chain.members | self.chain.config.members()
                } else {
                    0
                };
                for (i, t) in self.tracks.iter_mut().take(ACTIVE_TRACKS).enumerate() {
                    if !t.empty() && managed & (1 << i) == 0 {
                        t.playing = true;
                        t.cursor = 0;
                        t.last_phase = u64::MAX
                    }
                }
            }
            Command::Clear(i) => {
                let i = Self::valid_track(i)?;
                if self.chain_protects(i) {
                    return Err(Error::Busy);
                }
                if self.recorder.is_some() {
                    return Err(Error::Busy);
                }
                if self.tracks[i].empty() {
                    return Err(Error::NoContent);
                }
                self.checkpoint_clear(i);
                self.clear_slice_voices(i);
                self.release_track(i);
                let t = &mut self.tracks[i];
                t.playing = false;
                t.generated = false;
                t.generated_sounds = 0;
                t.length_ticks = 0;
                t.audio_frames = 0;
                t.source_frames = 0;
                t.source_ticks = 0;
                t.audio_sequence = false;
                t.chops = crate::chop::Chops::default();
                t.wave_view = (0, 0);
                *t.features = crate::features::Features::default();
                t.events.clear();
                t.revision += 1;
            }
            Command::Swing(value) => {
                if value.is_some_and(|v| !v.valid()) {
                    return Err(Error::InvalidValue);
                }
                if self.recorder.is_some() {
                    return Err(Error::Busy);
                }
                if self.swing != value {
                    self.swing = value;
                    // Only sequenced owners are released; live notes remain owned by their input.
                    for m in self.mono.iter_mut() {
                        m.release_source(1);
                    }
                    for v in &mut self.voices {
                        if v.source == 1 {
                            let cfg = self.tracks[v.track].play;
                            if cfg.voice > 0
                                && v.drum.is_none()
                                && let Some(n) = self.mono[v.track].latest()
                            {
                                v.retarget(
                                    n.note,
                                    n.velocity,
                                    n.source,
                                    self.tracks[v.track].synth,
                                    cfg.glide_ms,
                                    self.clock.rate,
                                    cfg.voice == 2,
                                );
                                v.configure_trigger(cfg, self.clock.rate);
                            } else {
                                v.release();
                            }
                        }
                    }
                    for v in &mut self.slice_voices {
                        if v.sampler.is_some_and(|s| s.source == 1) {
                            v.release = true;
                        }
                    }
                }
            }
            Command::Bpm(b) => {
                if !b.is_finite() || !(40.0..=240.0).contains(&b) {
                    return Err(Error::InvalidValue);
                }
                if self.running
                    || self.recorder.is_some()
                    || self.tracks.iter().take(ACTIVE_TRACKS).any(|t| {
                        t.kind == Kind::Audio
                            && !t.empty()
                            && !t.audio_sequence
                            && !t.audio_settings.warp
                    })
                {
                    return Err(Error::TempoFrozen);
                }
                if b != self.clock.bpm && self.history.kind == EditKind::Audio {
                    self.history.target = None;
                }
                let tick = self.clock.tick();
                self.clock.bpm = b;
                self.clock.frame = self.clock.frame_at(tick);
                for track in &mut self.tracks {
                    track.cursor = 0;
                    track.last_phase = u64::MAX;
                }
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
                if self.tracks[track].kind == Kind::Audio
                    && (self.tracks[track].audio_frames == 0 || self.tracks[track].empty())
                {
                    return Err(Error::NoContent);
                }
                let note = if self.tracks[track].kind == Kind::Audio
                    && self.tracks[track].audio_target == crate::audio::RecordTarget::Chop
                    && source >= 256
                {
                    if (48..64).contains(&note) {
                        note - 48
                    } else {
                        note
                    }
                } else {
                    note
                };
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
                if index >= t.events.len()
                    || t.events[index].deck.is_some()
                    || !t.events[index].on
                    || note > 127
                    || velocity > 127
                    || (t.events[index].slice && note >= t.chops.count)
                {
                    return Err(Error::InvalidValue);
                }
                if self.recorder.is_some() || self.chain_protects(track) {
                    return Err(Error::Busy);
                }
                let t = &mut self.tracks[track];
                self.history.checkpoint(track, t, EditKind::Midi);
                let old = t.events[index];
                let end = t
                    .events
                    .iter()
                    .enumerate()
                    .skip(index + 1)
                    .find(|(_, e)| !e.on && e.note == old.note && e.slice == old.slice)
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
        if self.chain_protects(track) {
            return Err(Error::Busy);
        }
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
        let mut cfg = crate::music::next_config(t.pattern, sound);
        if self.swing.is_some() {
            cfg.swing = 50.;
        }
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
        if self.recorder.is_some() {
            return Err(Error::Busy);
        }
        self.history.checkpoint(track, t, EditKind::Midi);
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
                    deck: None,
                    timing: crate::music::swing_basis(cfg, hit.sound),
                    slice: false,
                    tick,
                    note: hit.sound,
                    velocity: hit.velocity,
                    on: true,
                });
                t.events.push(Event {
                    deck: None,
                    timing: crate::music::swing_basis(cfg, hit.sound),
                    slice: false,
                    tick: {
                        let basis = crate::music::swing_basis(cfg, hit.sound);
                        let straight = basis.map_or(tick, |b| b.warp(tick, true));
                        let end = straight + crate::music::event_tick(hit.gate);
                        basis
                            .map_or(end, |b| b.warp(end, false))
                            .min(base + 4 * PPQ - 1)
                    },
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
    pub fn visible_audio(&self, i: usize) -> (&crate::features::Features, usize) {
        if self.tracks[i].kind == Kind::Audio
            && self
                .recorder
                .is_some_and(|r| r.track == i && !r.midi && self.clock.tick() >= r.start)
        {
            (&self.capture_features, self.capture_features.frames)
        } else {
            let t = &self.tracks[i];
            (
                &t.features,
                if t.source_frames > 0 {
                    t.source_frames
                } else {
                    t.audio_frames
                },
            )
        }
    }
    pub fn input_preview_peak(&self, index: usize) -> crate::features::Peak {
        self.input_preview[(self.input_preview_head + 1 + index) % 128]
    }
    pub fn active_note_mask(&self, track: usize) -> [u64; 2] {
        let mut notes = [0u64; 2];
        for voice in &self.voices {
            if voice.active && voice.track == track {
                notes[usize::from(voice.note) / 64] |= 1u64 << (voice.note % 64);
            }
        }
        for v in &self.slice_voices {
            if v.active
                && v.track == track
                && let Some(s) = v.sampler
            {
                notes[usize::from(s.note) / 64] |= 1u64 << (s.note % 64);
            }
        }
        notes
    }
    pub fn visible_events(&self, i: usize) -> &[Event] {
        if self.recorder.is_some_and(|r| r.track == i && r.midi) {
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
    fn release_sequence_for_recording(&mut self, i: usize) {
        self.mono[i].release_source(1);
        let cfg = self.tracks[i].play;
        for v in &mut self.voices {
            if v.track == i && v.source == 1 {
                if cfg.voice > 0
                    && v.drum.is_none()
                    && let Some(n) = self.mono[i].latest()
                {
                    v.retarget(
                        n.note,
                        n.velocity,
                        n.source,
                        self.tracks[i].synth,
                        cfg.glide_ms,
                        self.clock.rate,
                        cfg.voice == 2,
                    );
                    v.configure_trigger(cfg, self.clock.rate);
                } else {
                    v.release();
                }
            }
        }
        for v in &mut self.slice_voices {
            if v.track == i && v.sampler.is_some_and(|s| s.source == 1) {
                v.release = true;
            }
        }
    }
    fn release_track(&mut self, i: usize) {
        self.decks[i] = crate::deck::Player::default();
        self.mono[i] = crate::performance::Mono::default();
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
        self.pending_chords &= self
            .held_chords
            .iter()
            .enumerate()
            .filter(|(_, h)| h.len > 0)
            .fold(0, |bits, (p, _)| bits | (1 << p));
        self.open_notes[i] = [false; 128]
    }
    fn note(&mut self, track: usize, note: u8, velocity: u8, on: bool, source: u16) {
        if self.tracks[track].kind == Kind::Synth
            && self.tracks[track].play.voice > 0
            && !self.tracks[track].chord_mode
            && !(0x4000..0x4010).contains(&source)
        {
            self.mono_note(track, note, velocity, on, source);
        } else {
            self.poly_note(track, note, velocity, on, source);
        }
    }
    fn mono_note(&mut self, track: usize, note: u8, velocity: u8, on: bool, source: u16) {
        let on = on && velocity > 0;
        let previous = self.mono[track].latest();
        if on && velocity > 0 {
            self.age += 1;
            self.mono[track].press(note, source, velocity, self.age);
        } else {
            self.mono[track].release(note, source);
        }
        let next = self.mono[track].latest();
        if !on && previous.map(|n| (n.note, n.source)) != Some((note, source)) {
            return;
        }
        let slot = self.voices.iter().position(|v| {
            v.active
                && v.track == track
                && v.drum.is_none()
                && !(0x4000..0x4010).contains(&v.source)
        });
        if let Some(n) = next {
            let cfg = self.tracks[track].play;
            if let Some(slot) = slot {
                let overlap = previous.is_some();
                let glide = if cfg.voice == 1 || overlap {
                    cfg.glide_ms
                } else {
                    0.
                };
                self.voices[slot].retarget(
                    n.note,
                    n.velocity,
                    n.source,
                    self.tracks[track].synth,
                    glide,
                    self.clock.rate,
                    cfg.voice == 2 && overlap,
                );
                self.voices[slot].configure_trigger(cfg, self.clock.rate);
            } else {
                self.poly_note(track, n.note, n.velocity, true, n.source);
            }
        } else if let Some(slot) = slot {
            self.voices[slot].note_off();
        }
    }
    fn poly_note(&mut self, track: usize, note: u8, velocity: u8, on: bool, source: u16) {
        if self.tracks[track].kind == Kind::Audio {
            self.sample_note(
                track,
                note,
                velocity,
                on,
                source,
                self.tracks[track].audio_target == crate::audio::RecordTarget::Chop,
            );
            return;
        }

        if !on || velocity == 0 {
            for v in &mut self.voices {
                if v.active && v.track == track && v.note == note && v.source == source {
                    v.note_off()
                }
            }
            return;
        }
        let drum = if self.tracks[track].kind == Kind::Drum {
            Some(usize::from(note) % 16)
        } else {
            None
        };
        let native = drum.and_then(|d| self.tracks[track].kit.xpm.as_ref().map(|x| x.meta.pads[d]));
        if let Some(meta) = native {
            for v in &mut self.voices {
                if v.track == track
                    && let Some(other) = v.drum
                {
                    let group =
                        self.tracks[track].kit.xpm.as_ref().unwrap().meta.pads[other].mute_group;
                    if (meta.mono && Some(other) == drum)
                        || (meta.mute_group > 0 && meta.mute_group == group)
                        || meta.mute_targets.contains(&(other as u8 + 1))
                    {
                        v.active = false;
                    }
                }
            }
        } else if drum == Some(2) {
            for v in &mut self.voices {
                if v.track == track && v.drum == Some(3) {
                    v.active = false;
                }
            }
        }
        let mut triggers = [(0usize, 0usize); 5];
        let mut count = 0;
        if let Some(d) = drum {
            for layer in 0..2 {
                if !self.tracks[track].drums[d].layers[layer].enabled {
                    continue;
                }
                if layer == 0
                    && let Some(meta) = native.filter(|m| m.count > 0)
                {
                    let state = &mut self.tracks[track].xpm_step[d];
                    let selected = match meta.mode {
                        0 => {
                            let zone = *state as usize % usize::from(meta.count);
                            *state = state.wrapping_add(1);
                            1u8 << zone
                        }
                        2 => {
                            if *state == 0 {
                                *state = 0x9e3779b9u32 ^ (d as u32 * 0x10203);
                            }
                            *state ^= *state << 13;
                            *state ^= *state >> 17;
                            *state ^= *state << 5;
                            1u8 << (*state as usize % usize::from(meta.count))
                        }
                        _ => {
                            let mut mask = 0;
                            for z in 0..usize::from(meta.count) {
                                if (meta.ranges[z][0]..=meta.ranges[z][1]).contains(&velocity) {
                                    mask |= 1 << z;
                                }
                            }
                            mask
                        }
                    };
                    for zone in 0..usize::from(meta.count) {
                        if selected & (1 << zone) != 0 {
                            triggers[count] = (0, zone);
                            count += 1;
                        }
                    }
                } else {
                    triggers[count] = (layer, 0);
                    count += 1;
                }
            }
        } else {
            count = 1;
        }
        let settings = drum.map(|d| self.tracks[track].drums[d]);
        let earliest = settings.map_or(0., |d| {
            d.layers
                .iter()
                .filter(|l| l.enabled)
                .map(|l| l.shift)
                .fold(0., f32::min)
        });
        for &(layer, zone) in &triggers[..count] {
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
                native.map_or(velocity, |m| {
                    ((1. - m.sensitivity) * 127. + m.sensitivity * f32::from(velocity))
                        .round()
                        .clamp(1., 127.) as u8
                }),
                source,
                drum,
                self.age,
                self.tracks[track].synth,
                tune,
            );
            self.voices[i].layer = layer;
            self.voices[i].xpm_zone = zone;
            self.voices[i].pcm_gate = match self.tracks[track].play.trigger {
                1 => true,
                2 => false,
                _ => native.is_some_and(|m| m.gate),
            };
            self.voices[i].configure_trigger(self.tracks[track].play, self.clock.rate);
            self.voices[i].delay_frames = settings.map_or(0, |d| {
                ((d.layers[layer].shift - earliest) * self.clock.rate as f32 / 1000.).round() as u32
            });
        }
    }

    fn sample_note(
        &mut self,
        track: usize,
        note: u8,
        velocity: u8,
        on: bool,
        source: u16,
        chop: bool,
    ) {
        if !on || velocity == 0 {
            for v in &mut self.slice_voices {
                if v.active
                    && v.track == track
                    && v.sampler
                        .is_some_and(|s| s.note == note && s.source == source && s.slice == chop)
                    && v.gate
                {
                    v.release = true;
                }
            }
            return;
        }
        let t = &self.tracks[track];
        if t.audio_frames == 0 {
            return;
        }
        let frames = if t.source_frames > 0 {
            t.source_frames
        } else {
            t.audio_frames
        };
        let (start, end, level, played_note) = if chop {
            if note >= t.chops.count {
                return;
            }
            let slice = t.chops.slices[usize::from(note)];
            (
                slice.start as usize,
                slice.end as usize,
                slice.level,
                t.audio_settings.root,
            )
        } else {
            (0, frames, 1., note)
        };
        if start >= end || end > frames {
            return;
        }
        // Repeated note-on replaces only the same live/playback owner.
        for v in &mut self.slice_voices {
            if v.track == track
                && v.sampler
                    .is_some_and(|s| s.note == note && s.source == source && s.slice == chop)
            {
                v.active = false;
            }
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
        let mut voice = crate::audio::Voice::new(played_note, source, t.audio_settings);
        voice.note = note;
        voice.slice = chop;
        voice.one_shot = t.play.trigger == 2;
        voice.set_played_note(played_note, t.audio_settings);
        self.slice_voices[slot] = crate::chop::SliceVoice {
            active: true,
            track,
            pad: if chop {
                usize::from(note)
            } else {
                usize::from(note.saturating_sub(48)) % 16
            },
            start,
            end,
            at: start,
            velocity: f32::from(velocity) / 127. * level,
            gate: t.play.trigger != 2,
            release: false,
            env: 1.,
            age: self.age,
            sampler: Some(voice),
        };
    }
    fn live_note(&mut self, i: usize, note: u8, velocity: u8, on: bool, source: u16) {
        if self.tracks[i].kind == Kind::Audio
            && self.tracks[i].audio_target == crate::audio::RecordTarget::Turntable
        {
            return;
        }
        // MIDI commands may arrive on the exact frame of a wrap, before process().
        if self.recorder.is_some_and(|r| r.track == i && r.midi) {
            self.process_capture(Frame::ZERO);
        }
        self.note(i, note, velocity, on, source);
        self.capture_note(i, note, velocity, on);
    }

    fn remap_slices(t: &mut Track, old: crate::chop::Chops) {
        let count = usize::from(t.chops.count).min(16);
        let mut map = [0u8; 16];
        for (i, to) in map.iter_mut().enumerate() {
            let start = old.slices[i].start;
            *to = t.chops.slices[..count]
                .iter()
                .position(|s| start >= s.start && start < s.end)
                .unwrap_or_else(|| {
                    t.chops.slices[..count]
                        .iter()
                        .enumerate()
                        .min_by_key(|(_, s)| s.start.abs_diff(start))
                        .map_or(0, |(n, _)| n)
                }) as u8;
        }
        for event in &mut t.events {
            if event.slice {
                event.note = map[usize::from(event.note).min(15)];
            }
        }
        t.revision = t.revision.wrapping_add(1);
    }
    fn chain_protects(&self, track: usize) -> bool {
        self.chain.running && (self.chain.members | self.chain.config.members()) & (1 << track) != 0
    }
    fn apply_chain_change(&mut self, change: crate::chain::Change) {
        let (members, next) = match change {
            crate::chain::Change::Activate {
                track,
                members,
                origin,
            } => {
                self.tracks[track].origin = origin;
                (members, Some(track))
            }
            crate::chain::Change::Stop { members } => (members, None),
        };
        for track in 0..16 {
            if members & (1 << track) != 0 {
                self.tracks[track].playing = false;
                self.release_track(track);
            }
        }
        if let Some(track) = next {
            let t = &mut self.tracks[track];
            t.playing = true;
            t.cursor = 0;
            t.last_phase = u64::MAX;
        }
    }
    fn refresh_schedule(&mut self, i: usize, tick: u64) {
        let t = &mut self.tracks[i];
        if t.schedule_revision == t.revision && t.schedule_swing == self.swing {
            return;
        }
        let changed_swing = t.schedule_swing != self.swing;
        t.schedule.clear();
        t.schedule.extend(
            t.events
                .iter()
                .map(|&e| self.swing.map_or(e, |s| s.event(e, t.length_ticks))),
        );
        if !t
            .schedule
            .windows(2)
            .all(|w| (w[0].tick, w[0].on) <= (w[1].tick, w[1].on))
        {
            t.schedule.sort_unstable_by_key(|e| (e.tick, e.on));
        }
        t.schedule_revision = t.revision;
        t.schedule_swing = self.swing;
        // A timing edit must not replay notes already passed in this cycle.
        if changed_swing || t.last_phase != u64::MAX {
            let phase = t.phase(tick);
            t.cursor = t.schedule.partition_point(|e| e.tick < phase);
            t.last_phase = phase;
        }
        if t.events.first().is_some_and(|e| e.deck.is_some()) && !self.decks[i].active {
            t.cursor = 0;
        }
    }
    pub fn process(&mut self, input: Frame) -> Frame {
        let mut profile = crate::qa::dsp_start(|| self.active_voices());
        let tick = self.clock.tick();
        let rate = self.clock.rate as f32;
        let peak = &mut self.input_preview[self.input_preview_head];
        peak.min = peak.min.min(input.l.min(input.r));
        peak.max = peak.max.max(input.l.max(input.r));
        self.input_preview_count += 1;
        if self.input_preview_count >= self.clock.rate / 512 {
            self.input_preview_count = 0;
            self.input_preview_head = (self.input_preview_head + 1) % 128;
            self.input_preview[self.input_preview_head] = crate::features::Peak::default();
        }
        self.process_capture(input);
        if self.pending_chords != 0 {
            for pad in 0..16 {
                if self.pending_chords & (1 << pad) == 0 {
                    continue;
                }
                let h = &mut self.held_chords[pad];
                if h.countdown > 0 {
                    h.countdown -= 1;
                    continue;
                }
                if h.countdown == 0 {
                    let (track, note, velocity) = (h.track, h.notes[h.next], h.velocity);
                    h.next += 1;
                    h.countdown = h.step.saturating_sub(1);
                    if h.next >= h.len {
                        self.pending_chords &= !(1 << pad);
                    }
                    self.live_note(track, note, velocity, true, 0x4000 + pad as u16);
                }
            }
        }
        if self.running && self.chain.running {
            let tracks = &self.tracks;
            if let Some(change) = self.chain.tick(tick, |track| tracks[track].length_ticks) {
                self.apply_chain_change(change);
            }
        }
        if self.running {
            for i in 0..ACTIVE_TRACKS {
                if self.tracks[i].playing
                    && tick >= self.tracks[i].origin
                    && (self.tracks[i].kind != Kind::Audio || self.tracks[i].audio_sequence)
                    && !self.tracks[i].empty()
                {
                    self.refresh_schedule(i, tick);
                    let phase = self.tracks[i].phase(tick);
                    if phase < self.tracks[i].last_phase {
                        let was = self.mono[i].latest();
                        self.mono[i].release_source(1);
                        if was.is_some_and(|n| n.source == 1)
                            && let Some(n) = self.mono[i].latest()
                            && let Some(v) = self
                                .voices
                                .iter_mut()
                                .find(|v| v.active && v.track == i && v.drum.is_none())
                        {
                            let cfg = self.tracks[i].play;
                            v.retarget(
                                n.note,
                                n.velocity,
                                n.source,
                                self.tracks[i].synth,
                                cfg.glide_ms,
                                self.clock.rate,
                                cfg.voice == 2,
                            );
                            v.configure_trigger(cfg, self.clock.rate);
                        }
                        for v in &mut self.voices {
                            if v.track == i && v.source == 1 {
                                v.release()
                            }
                        }
                        for v in &mut self.slice_voices {
                            if v.track == i && v.sampler.is_some_and(|s| s.source == 1) {
                                v.release = true;
                            }
                        }
                        self.tracks[i].cursor = 0
                    }
                    while self.tracks[i].cursor < self.tracks[i].schedule.len() {
                        let e = self.tracks[i].schedule[self.tracks[i].cursor];
                        if e.tick > phase {
                            break;
                        }
                        self.tracks[i].cursor += 1;
                        let suppressed = self.recorder.is_some_and(|r| {
                            r.track == i
                                && tick >= r.start
                                && r.mode == RecordMode::Replace
                                && (self.tracks[i].kind != Kind::Drum
                                    || r.touched & (1 << (e.note % 16)) != 0)
                        });
                        if !suppressed {
                            if let Some(control) = e.deck {
                                if !self.decks[i].live {
                                    let elapsed = self
                                        .clock
                                        .frame_at(phase)
                                        .saturating_sub(self.clock.frame_at(e.tick));
                                    self.decks[i].restore(
                                        control,
                                        elapsed,
                                        self.tracks[i].deck,
                                        self.clock.rate,
                                    );
                                }
                            } else if self.tracks[i].kind == Kind::Audio {
                                self.sample_note(i, e.note, e.velocity, e.on, 1, e.slice);
                            } else {
                                self.note(i, e.note, e.velocity, e.on, 1)
                            }
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
                let sample = if let Some(sampler) = &mut voice.sampler {
                    if voice.release {
                        voice.env = (voice.env - 1. / (self.clock.rate as f32 * 0.005)).max(0.);
                        if voice.env <= 0. {
                            voice.active = false;
                        }
                    }
                    let settings = t.audio_settings;
                    let source_frames = t.source_frames.max(1);
                    let ticks = ((if t.source_ticks > 0 {
                        t.source_ticks
                    } else {
                        t.length_ticks
                    }) as f64
                        * (voice.end - voice.start) as f64
                        / source_frames as f64)
                        .round()
                        .max(1.) as u64;
                    let out = sampler
                        .sample(
                            &t.audio[voice.start..voice.end],
                            settings,
                            ticks,
                            self.clock.bpm,
                            self.clock.rate,
                        )
                        .gain(voice.velocity * voice.env);
                    if sampler.finished {
                        voice.active = false;
                    }
                    out
                } else {
                    voice.sample(&t.audio[..t.audio_frames], self.clock.rate)
                };
                signals[voice.track] = signals[voice.track].combine(sample);
            }
        }

        for (i, signal) in signals.iter_mut().enumerate() {
            let t = &self.tracks[i];
            let replacing = self.recorder.is_some_and(|r| {
                r.track == i
                    && tick >= r.start
                    && r.mode == RecordMode::Replace
                    && !(r.midi && t.audio_target == crate::audio::RecordTarget::Turntable)
            });
            if !self.decks[i].live
                && (replacing
                    || (!self.decks[i].audition
                        && (!self.running || !t.playing || tick < t.origin)))
            {
                self.decks[i] = crate::deck::Player::default();
            }
            let raw = !replacing
                && self.running
                && t.playing
                && tick >= t.origin
                && !t.audio_sequence
                && t.audio_target == crate::audio::RecordTarget::Turntable
                && !t.empty();
            if raw && !self.decks[i].active {
                self.decks[i]
                    .start(self.clock.frame_at(t.phase(tick)) as f64 * f64::from(t.deck.speed));
            }
            if t.kind == Kind::Audio && t.source_frames > 0 {
                *signal = signal.combine(self.decks[i].sample(
                    &t.audio[..t.source_frames],
                    t.deck,
                    self.clock.rate,
                ));
            }
        }
        let mut drum_delay = [Frame::ZERO; 16];
        let mut drum_room = [Frame::ZERO; 16];
        for t in self.tracks.iter_mut().take(ACTIVE_TRACKS) {
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
                let mut sample = if let Some(d) = v
                    .drum
                    .filter(|d| !t.kit.voice_clip(*d, v.layer, v.xpm_zone).is_empty())
                {
                    v.sample_pcm(t.kit.voice_clip(d, v.layer, v.xpm_zone), rate, adjusted)
                } else {
                    Frame::mono(if t.synth.lfo_sync {
                        let mut config = t.synth;
                        config.lfo_rate = config.effective_lfo_rate(self.clock.bpm as f32);
                        v.sample(rate, &config, adjusted)
                    } else {
                        v.sample(rate, &t.synth, adjusted)
                    })
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
        let solo = self.tracks.iter().take(ACTIVE_TRACKS).any(|t| t.mix.solo);
        let mut sum = Frame::ZERO;
        let split = self.separate_usb;
        let mut usb_sum = Frame::ZERO;
        let mut usb_buses = [Frame::ZERO; 2];
        let mut usb_ds = Frame::ZERO;
        let mut usb_rs = Frame::ZERO;
        let mut buses = [Frame::ZERO; 2];
        let mut ds = Frame::ZERO;
        let mut rs = Frame::ZERO;
        for (i, signal) in signals.iter_mut().take(ACTIVE_TRACKS).enumerate() {
            let t = &mut self.tracks[i];
            if self.running
                && t.playing
                && tick >= t.origin
                && t.kind == Kind::Audio
                && !t.audio_sequence
                && t.audio_target != crate::audio::RecordTarget::Turntable
                && !self.recorder.is_some_and(|r| {
                    r.track == i && r.midi && r.mode == RecordMode::Replace && tick >= r.start
                })
                && t.audio_frames > 0
                && !t.empty()
            {
                let phase = t.phase(tick) as f64 / t.length_ticks as f64 * t.audio_frames as f64;
                *signal = signal.combine(t.audio_player.sample(
                    &t.audio[..t.audio_frames],
                    t.audio_settings,
                    crate::audio::LoopTime {
                        frame: self.clock.frame,
                        origin: t.origin,
                        phase,
                        ticks: if t.source_ticks > 0 {
                            t.source_ticks
                        } else {
                            t.length_ticks
                        },
                        bpm: self.clock.bpm,
                        rate: self.clock.rate,
                    },
                ));
            }
        }
        let source = usize::from(self.sidechain.source);
        let st = &self.tracks[source];
        let audible = source < ACTIVE_TRACKS && !st.mix.mute && (!solo || st.mix.solo);
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
        for (i, signal) in signals.iter().take(ACTIVE_TRACKS).enumerate() {
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
                    + t.pan_lfo.sample(
                        rate,
                        t.synth.effective_lfo_rate(self.clock.bpm as f32),
                        t.synth.lfo_shape,
                    ) * t.synth.lfo_pan)
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
        self.hardware_gain += (self.hardware_volume - self.hardware_gain) * 0.002;
        sum = sum.gain(self.master_gain * self.hardware_gain);
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
                self.master_gain * self.hardware_gain,
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
    /// File-worker operation only, while holding exclusive runtime ownership.
    /// Refuse occupied targets and running audio; no background source can be lost.
    pub fn transfer_track_stopped(
        &mut self,
        source: usize,
        target: usize,
        moving: bool,
    ) -> Result<(), Error> {
        Self::valid_track(source)?;
        Self::valid_track(target)?;
        if !self.is_idle() || self.chain.running || self.held_fx != 0 {
            return Err(Error::Busy);
        }
        if source == target {
            return Err(Error::InvalidValue);
        }
        let from = &self.tracks[source];
        let to = &self.tracks[target];
        if from.empty() && from.source_frames == 0 && from.kit.bytes() == 0 {
            return Err(Error::NoContent);
        }
        if !to.empty()
            || to.source_frames > 0
            || to.audio_frames > 0
            || !to.events.is_empty()
            || to.kit.bytes() > 0
        {
            return Err(Error::InvalidValue);
        }
        if from.source_frames.max(from.audio_frames) > to.audio.len() {
            return Err(Error::Capacity);
        }
        if !moving
            && self.tracks.iter().map(|t| t.kit.bytes()).sum::<usize>() + from.kit.bytes()
                > crate::bank::SESSION_KIT_BYTES
        {
            return Err(Error::Capacity);
        }
        if moving {
            self.tracks.swap(source, target);
            self.track_fx.swap(source, target);
            self.remap_track_references(source, target);
            self.history.kind = EditKind::TrackMove;
        } else {
            let copied = from.duplicate_stopped(to.audio.len(), target);
            let old = std::mem::replace(&mut self.tracks[target], copied);
            self.track_fx[target] = std::array::from_fn(|_| FxState::new(self.clock.rate));
            self.history.track = Some(Box::new(old)); // Reclaims previous full-track history on worker.
            self.history.kind = EditKind::TrackCopy;
        }
        self.history.target = Some(target);
        self.history.peer = Some(source);
        self.history.undone = false;
        self.history.state = Some(ClipState::read(&self.tracks[target]));
        self.history.events.clear();
        for i in [source, target] {
            let t = &mut self.tracks[i];
            t.playing = false;
            t.reset_audio_player();
            t.revision = t.revision.wrapping_add(1);
        }
        self.loop_revision = self.loop_revision.wrapping_add(1);
        Ok(())
    }
    fn remap_track_references(&mut self, a: usize, b: usize) {
        let swap = |n: u8| {
            if n as usize == a {
                b as u8
            } else if n as usize == b {
                a as u8
            } else {
                n
            }
        };
        for step in &mut self.chain.config.steps[..self.chain.config.len as usize] {
            step.track = swap(step.track);
        }
        self.sidechain.source = swap(self.sidechain.source);
        let a_set = self.sidechain.mask & (1 << a) != 0;
        let b_set = self.sidechain.mask & (1 << b) != 0;
        self.sidechain.mask &= !((1 << a) | (1 << b));
        if a_set {
            self.sidechain.mask |= 1 << b;
        }
        if b_set {
            self.sidechain.mask |= 1 << a;
        }
    }
    pub fn is_idle(&self) -> bool {
        !self.running
            && !self.decks.iter().any(|d| d.active)
            && self.pending_chords == 0
            && self.recorder.is_none()
            && self.active_voices() == 0
            && !self.slice_voices.iter().any(|v| v.active)
    }
    pub fn emergency_silence(&mut self) {
        self.decks.fill(crate::deck::Player::default());
        for voice in &mut self.voices {
            voice.active = false;
        }
        for voice in &mut self.slice_voices {
            voice.active = false;
        }
        self.held_chords = [HeldChord::default(); 16];
        self.pending_chords = 0;
        self.mono.fill(crate::performance::Mono::default());
        self.release_temporary_fx();
    }
    pub fn reset_chop_history(&mut self, track: usize) {
        self.history.invalidate(track);
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
