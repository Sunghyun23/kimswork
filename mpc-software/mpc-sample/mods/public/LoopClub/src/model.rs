pub const ACTIVE_TRACKS: usize = 16;
// Sixteen backing slots remain in project files to preserve older sessions.
use serde::{Deserialize, Serialize};
pub const TRACKS: usize = 16;
pub const DEFAULT_BPM: f64 = 100.;
pub const SOUNDS: usize = 16;
pub const PPQ: u64 = 1 << 24;
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Frame {
    pub l: f32,
    pub r: f32,
}
impl Frame {
    pub const ZERO: Self = Self { l: 0.0, r: 0.0 };
    pub fn mono(x: f32) -> Self {
        Self { l: x, r: x }
    }
    pub fn gain(self, x: f32) -> Self {
        Self {
            l: self.l * x,
            r: self.r * x,
        }
    }
    pub fn combine(self, b: Self) -> Self {
        Self {
            l: self.l + b.l,
            r: self.r + b.r,
        }
    }
    pub fn peak(self) -> f32 {
        self.l.abs().max(self.r.abs())
    }
    pub fn finite(self) -> bool {
        self.l.is_finite() && self.r.is_finite()
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Kind {
    #[default]
    Audio,
    Synth,
    Drum,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Page {
    #[default]
    Loop,
    Play,
    Grid,
    Mix,
    KitMix,
    TrackFx,
    MasterFx,
    Setup,
    Chop,
    Source,
    DrumSound,
    Output,
    Sidechain,
    Texture,
    FxSettings,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Status {
    #[default]
    Empty,
    Armed,
    CountIn,
    Record,
    Finishing,
    Play,
    Stop,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Route {
    #[default]
    Dry,
    A,
    B,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputTarget {
    Main,
    Usb,
    #[default]
    Both,
}
impl OutputTarget {
    pub fn label(self) -> &'static str {
        match self {
            Self::Main => "MAIN",
            Self::Usb => "USB",
            Self::Both => "MAIN+USB",
        }
    }
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Mix {
    pub level: f32,
    pub pan: f32,
    pub delay: f32,
    pub room: f32,
    pub mute: bool,
    pub solo: bool,
}
impl Default for Mix {
    fn default() -> Self {
        Self {
            level: 0.7,
            pan: 0.,
            delay: 0.,
            room: 0.,
            mute: false,
            solo: false,
        }
    }
}
impl Mix {
    pub fn valid(&self) -> bool {
        [self.level, self.pan, self.delay, self.room]
            .iter()
            .all(|x| x.is_finite())
            && (0.0..=1.0).contains(&self.level)
            && (-1.0..=1.0).contains(&self.pan)
            && (0.0..=1.0).contains(&self.delay)
            && (0.0..=1.0).contains(&self.room)
    }
    pub fn apply(&self, s: Frame, level: f32) -> Frame {
        if s.l == 0. && s.r == 0. {
            return Frame::ZERO;
        }
        let l = (1.0 - self.pan.max(0.0)).sqrt();
        let r = (1.0 + self.pan.min(0.0)).sqrt();
        Frame {
            l: s.l * l * level,
            r: s.r * r * level,
        }
    }
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Synth {
    pub lfo_sync: bool,
    pub lfo_division: u8,
    pub lfo_rate: f32,
    pub lfo_shape: u8,
    pub lfo_pitch: f32,
    pub lfo_filter: f32,
    pub lfo_volume: f32,
    pub lfo_pan: f32,
    pub filter_attack: f32,
    pub filter_decay: f32,
    pub filter_sustain: f32,
    pub filter_release: f32,
    pub env_amount: f32,
    pub wave2: u8,
    pub tune1: f32,
    pub tune2: f32,
    pub fine1: f32,
    pub fine2: f32,
    pub level1: f32,
    pub level2: f32,
    pub resonance: f32,
    pub filter: u8,
    pub decay: f32,
    pub sustain: f32,
    pub wave: u8,
    pub cutoff: f32,
    pub attack: f32,
    pub release: f32,
    pub detune: f32,
    pub octave: i8,
}
impl Default for Synth {
    fn default() -> Self {
        Self {
            lfo_sync: false,
            lfo_division: 7,
            lfo_rate: 1.,
            lfo_shape: 0,
            lfo_pitch: 0.,
            lfo_filter: 0.,
            lfo_volume: 0.,
            lfo_pan: 0.,
            filter_attack: 0.01,
            filter_decay: 0.3,
            filter_sustain: 0.3,
            filter_release: 0.25,
            env_amount: 0.,
            wave2: 1,
            tune1: 0.,
            tune2: 0.,
            fine1: 0.,
            fine2: 0.,
            level1: 0.5,
            level2: 0.5,
            resonance: 0.,
            filter: 0,
            decay: 0.3,
            sustain: 0.7,
            wave: 1,
            cutoff: 2400.,
            attack: 0.008,
            release: 0.25,
            detune: 0.003,
            octave: 0,
        }
    }
}
impl Synth {
    pub fn filter_adsr(self) -> Self {
        Self {
            attack: self.filter_attack,
            decay: self.filter_decay,
            sustain: self.filter_sustain,
            release: self.filter_release,
            ..self
        }
    }
    pub fn valid(&self) -> bool {
        usize::from(self.lfo_division) < SYNC_LABELS.len()
            && self.lfo_pan.is_finite()
            && (0.0..=1.0).contains(&self.lfo_pan)
            && self.lfo_shape < 6
            && self.lfo_rate.is_finite()
            && (0.05..=20.0).contains(&self.lfo_rate)
            && self.lfo_pitch.is_finite()
            && (0.0..=200.0).contains(&self.lfo_pitch)
            && self.lfo_filter.is_finite()
            && (0.0..=4.0).contains(&self.lfo_filter)
            && self.lfo_volume.is_finite()
            && (0.0..=1.0).contains(&self.lfo_volume)
            && self.filter_attack.is_finite()
            && (0.001..=2.0).contains(&self.filter_attack)
            && self.filter_decay.is_finite()
            && (0.001..=6.0).contains(&self.filter_decay)
            && self.filter_sustain.is_finite()
            && (0.0..=1.0).contains(&self.filter_sustain)
            && self.filter_release.is_finite()
            && (0.01..=6.0).contains(&self.filter_release)
            && self.env_amount.is_finite()
            && (-1.0..=1.0).contains(&self.env_amount)
            && self.wave2 < 4
            && self.filter < 3
            && [self.tune1, self.tune2]
                .iter()
                .all(|v| v.is_finite() && (-24.0..=24.0).contains(v))
            && [self.fine1, self.fine2]
                .iter()
                .all(|v| v.is_finite() && (-100.0..=100.0).contains(v))
            && [self.level1, self.level2, self.resonance, self.sustain]
                .iter()
                .all(|v| v.is_finite() && (0.0..=1.0).contains(v))
            && self.decay.is_finite()
            && (0.001..=6.0).contains(&self.decay)
            && self.wave < 4
            && self.octave >= -2
            && self.octave <= 2
            && self.cutoff.is_finite()
            && (40.0..=18000.0).contains(&self.cutoff)
            && self.attack.is_finite()
            && (0.001..=2.0).contains(&self.attack)
            && self.release.is_finite()
            && (0.01..=6.0).contains(&self.release)
            && self.detune.is_finite()
            && (0.0..=0.02).contains(&self.detune)
    }
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct DrumLayer {
    pub enabled: bool,
    pub level: f32,
    pub tune: f32,
    pub shift: f32,
    pub decay: f32,
    pub reverse: bool,
}
impl Default for DrumLayer {
    fn default() -> Self {
        Self {
            enabled: true,
            level: 1.,
            tune: 0.,
            shift: 0.,
            decay: 0.,
            reverse: false,
        }
    }
}
impl DrumLayer {
    pub fn valid(&self) -> bool {
        self.level.is_finite()
            && (0.0..=1.0).contains(&self.level)
            && self.tune.is_finite()
            && (-24.0..=24.0).contains(&self.tune)
            && self.shift.is_finite()
            && (-50.0..=50.0).contains(&self.shift)
            && self.decay.is_finite()
            && (self.decay == 0. || (0.01..=16.0).contains(&self.decay))
    }
}
fn default_layers() -> [DrumLayer; 2] {
    [
        DrumLayer::default(),
        DrumLayer {
            enabled: false,
            level: 0.5,
            ..DrumLayer::default()
        },
    ]
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Drum {
    pub mix: Mix,
    pub tune: f32,
    pub decay: f32,
    #[serde(default = "default_layers")]
    pub layers: [DrumLayer; 2],
}
impl Default for Drum {
    fn default() -> Self {
        Self {
            mix: Mix::default(),
            tune: 0.,
            decay: 0.25,
            layers: default_layers(),
        }
    }
}
impl Drum {
    pub fn valid(&self) -> bool {
        self.mix.valid()
            && self.layers.iter().all(DrumLayer::valid)
            && self.tune.is_finite()
            && (-24.0..=24.0).contains(&self.tune)
            && self.decay.is_finite()
            && (0.01..=16.0).contains(&self.decay)
    }
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct Event {
    #[serde(default)]
    pub slice: bool,
    pub tick: u64,
    pub note: u8,
    pub velocity: u8,
    pub on: bool,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Effect {
    #[default]
    Off,
    Drive,
    Filter,
    Compressor,
    LoFi,
    Isolator,
    Repeat,
    Reverse,
    TapeStop,
    DubEcho,
    Crush,
    Wobble,
    Tremolo,
    AutoPan,
    AutoFilter,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct FxMod {
    pub sync: bool,
    pub division: u8,
    pub shape: u8,
}
impl Default for FxMod {
    fn default() -> Self {
        Self {
            sync: false,
            division: 7,
            shape: 0,
        }
    }
}
impl Effect {
    pub fn modulated(self) -> bool {
        matches!(
            self,
            Self::Wobble | Self::Tremolo | Self::AutoPan | Self::AutoFilter
        )
    }
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Fx {
    #[serde(default)]
    pub modulation: FxMod,
    pub effect: Effect,
    pub a: f32,
    pub b: f32,
    pub mix: f32,
}
impl Default for Fx {
    fn default() -> Self {
        Self {
            modulation: FxMod::default(),
            effect: Effect::Off,
            a: 0.5,
            b: 0.5,
            mix: 0.5,
        }
    }
}
impl Fx {
    pub fn rate_hz(&self, bpm: f32) -> f32 {
        if self.modulation.sync {
            bpm / 60. / SYNC_BEATS[usize::from(self.modulation.division.min(15))]
        } else if self.effect == Effect::Wobble {
            0.2 + self.a * 4.
        } else {
            0.05 * 400_f32.powf(self.a)
        }
    }
    pub fn rate_label(&self) -> String {
        if self.modulation.sync {
            SYNC_LABELS[usize::from(self.modulation.division.min(15))].into()
        } else {
            format!("{:.2} Hz", self.rate_hz(100.))
        }
    }
    pub fn valid(&self) -> bool {
        self.modulation.division < 16
            && self.modulation.shape < 6
            && [self.a, self.b, self.mix]
                .iter()
                .all(|x| x.is_finite() && (0.0..=1.0).contains(x))
    }
}
pub const DRUM_NAMES: [&str; 16] = [
    "KICK", "SNARE", "CH HAT", "OH HAT", "CLAP", "RIM", "TAMB", "COWBL", "LO TOM", "HI TOM",
    "CONGA", "SHAKER", "RIDE", "CRASH", "BELL", "FX",
];
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidTrack,
    InvalidValue,
    Busy,
    NotEmpty,
    NoContent,
    Capacity,
    TempoFrozen,
    WrongKind,
    InputUnavailable,
    NoUndo,
    NoRedo,
    ResampleFirst,
}
#[derive(Clone, Copy, Debug)]
pub struct Pickup {
    pub position: f32,
    pub picked: bool,
    seen: bool,
}
impl Default for Pickup {
    fn default() -> Self {
        Self {
            position: 0.0,
            picked: false,
            seen: false,
        }
    }
}
impl Pickup {
    pub fn select(&mut self, target: f32) {
        self.picked = self.seen && (self.position - target).abs() < 1.0 / 127.0
    }
    pub fn move_to(&mut self, value: f32, target: f32) -> Option<f32> {
        let next = value.clamp(0.0, 1.0);
        let old = self.position;
        self.position = next;
        if (next - target).abs() < 1.0 / 127.0
            || (self.seen && (old - target) * (next - target) <= 0.0)
        {
            self.picked = true
        }
        self.seen = true;
        self.picked.then_some(next)
    }
}

/// Per-track recording preference. Active recording latches it at each loop boundary.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RecordMode {
    #[default]
    Replace,
    Overdub,
}

pub const SYNC_LABELS: [&str; 16] = [
    "1 BAR", "1/2", "1/2D", "1/2T", "1/4", "1/4D", "1/4T", "1/8", "1/8D", "1/8T", "1/16", "1/16D",
    "1/16T", "1/32", "1/32D", "1/32T",
];
pub const SYNC_BEATS: [f32; 16] = [
    4.,
    2.,
    3.,
    4. / 3.,
    1.,
    1.5,
    2. / 3.,
    0.5,
    0.75,
    1. / 3.,
    0.25,
    0.375,
    1. / 6.,
    0.125,
    0.1875,
    1. / 12.,
];
impl Synth {
    pub fn effective_lfo_rate(&self, bpm: f32) -> f32 {
        if self.lfo_sync {
            bpm / (60. * SYNC_BEATS[usize::from(self.lfo_division).min(15)])
        } else {
            self.lfo_rate
        }
    }
    pub fn lfo_rate_label(&self) -> String {
        if self.lfo_sync {
            SYNC_LABELS[usize::from(self.lfo_division).min(15)].into()
        } else {
            format!("{:.2} Hz", self.lfo_rate)
        }
    }
}
