//! Allocation-free sidechain detection and an opt-in five-stage master texture.
use crate::{
    dsp::FxState,
    model::{Effect, Frame, Fx},
};
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Sidechain {
    pub source: u8,
    pub enabled: bool,
    /// 0..15 selects a drum;16 selects the whole track.
    pub sound: u8,
    /// 0 = all other tracks;1 = custom track mask;2 = full master including source.
    pub mode: u8,
    pub mask: u16,
    pub depth: f32,
    pub release: f32,
    pub threshold: f32,
    pub sync: bool,
}
impl Default for Sidechain {
    fn default() -> Self {
        Self {
            source: 0,
            enabled: true,
            sound: 0,
            mode: 0,
            mask: 65535,
            depth: 0.,
            release: 0.4,
            threshold: 0.08,
            sync: false,
        }
    }
}
impl Sidechain {
    pub fn valid(self) -> bool {
        self.source < 16
            && self.sound <= 16
            && self.mode < 3
            && [self.depth, self.release, self.threshold]
                .iter()
                .all(|v| v.is_finite() && (0.0..=1.0).contains(v))
            && self.threshold >= 0.01
    }
    pub fn targets(self, i: usize) -> bool {
        i < 16
            && i != usize::from(self.source)
            && (self.mode == 0 || self.mode == 1 && self.mask & (1 << i) != 0)
    }
    pub fn release_seconds(self, bpm: f32) -> f32 {
        if self.sync {
            let beats = [0.125, 0.25, 0.5, 0.75, 1., 1.5, 2., 3., 4.];
            60. / bpm * beats[(self.release * 8.99) as usize]
        } else {
            0.04 * 50_f32.powf(self.release)
        }
    }
}
#[derive(Default)]
pub struct Duck {
    envelope: f32,
    gain: f32,
    initialized: bool,
    rate: f32,
    last_release: f32,
    last_control: f32,
    last_bpm: f32,
    last_sync: bool,
    release_coeff: f32,
    attack_coeff: f32,
}
impl Duck {
    pub fn sample(&mut self, input: f32, c: Sidechain, rate: f32, bpm: f32) -> f32 {
        if !self.initialized {
            self.gain = 1.;
            self.initialized = true;
        }
        let depth = if c.enabled { c.depth } else { 0. };
        if depth == 0. && self.gain >= 0.999999 {
            self.envelope = 0.;
            self.gain = 1.;
            return 1.;
        }
        if self.rate != rate
            || self.last_control != c.release
            || self.last_bpm != bpm
            || self.last_sync != c.sync
        {
            let release = c.release_seconds(bpm);
            self.last_control = c.release;
            self.last_bpm = bpm;
            self.last_sync = c.sync;
            self.rate = rate;
            self.last_release = release;
            self.release_coeff = 1. - (-1. / (rate * release)).exp();
            self.attack_coeff = 1. - (-1. / (rate * 0.002)).exp();
        }
        let level = input.abs();
        self.envelope += (level - self.envelope)
            * if level > self.envelope {
                self.attack_coeff
            } else {
                self.release_coeff
            };
        let amount = ((self.envelope / c.threshold.max(0.01) - 0.25) / 0.75).clamp(0., 1.);
        let target = 1. - depth * 0.95 * amount;
        self.gain += (target - self.gain)
            * if target < self.gain {
                self.attack_coeff
            } else {
                self.release_coeff
            };
        self.gain
    }
    pub fn reduction(&self) -> f32 {
        if self.initialized { 1. - self.gain } else { 0. }
    }
}
pub const TEXTURES: [&str; 5] = ["DUST", "DRIVE", "SQUEEZE", "WOBBLE", "DUB ECHO"];
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Texture {
    pub modulation: [crate::model::FxMod; 5],
    pub enabled: [bool; 5],
    pub master_bypass: [bool; 2],
    pub params: [[f32; 3]; 5],
}
impl Default for Texture {
    fn default() -> Self {
        Self {
            modulation: [crate::model::FxMod::default(); 5],
            enabled: [false; 5],
            master_bypass: [false; 2],
            params: [
                [0.2, 0.6, 0.3],
                [0.15, 0.6, 0.3],
                [0.35, 0.4, 0.5],
                [0.2, 0.08, 0.3],
                [0.5, 0.25, 0.2],
            ],
        }
    }
}
impl Texture {
    pub fn valid(self) -> bool {
        self.modulation
            .iter()
            .all(|m| m.division < 16 && m.shape < 6)
            && self
                .params
                .iter()
                .flatten()
                .all(|v| v.is_finite() && (0.0..=1.0).contains(v))
    }
}
pub struct TextureState {
    wet: [f32; 5],
    low: Frame,
    drive: Frame,
    held: Frame,
    hold: u32,
    envelope: f32,
    wobble: FxState,
    echo: FxState,
    release: f32,
    rate: f32,
    release_coeff: f32,
}
impl TextureState {
    pub fn new(rate: u32) -> Self {
        Self {
            wet: [0.; 5],
            low: Frame::ZERO,
            drive: Frame::ZERO,
            held: Frame::ZERO,
            hold: 0,
            envelope: 0.,
            wobble: FxState::new(rate),
            echo: FxState::new(rate),
            release: -1.,
            rate: 0.,
            release_coeff: 0.,
        }
    }
    pub fn process(&mut self, mut s: Frame, t: Texture, rate: f32, bpm: f32) -> Frame {
        for i in 0..5 {
            let [a, b, mix] = t.params[i];
            let target = if t.enabled[i] { mix } else { 0. };
            self.wet[i] += (target - self.wet[i]) * 0.002;
            if i >= 3 {
                // The underlying effect retains its own bypass fade and history.
                let fx = Fx {
                    effect: if t.enabled[i] {
                        if i == 3 {
                            Effect::Wobble
                        } else {
                            Effect::DubEcho
                        }
                    } else {
                        Effect::Off
                    },
                    a,
                    b,
                    mix,
                    modulation: t.modulation[i],
                };
                s = if i == 3 {
                    self.wobble.process(s, fx, rate, bpm)
                } else {
                    self.echo.process(s, fx, rate, bpm)
                };
                continue;
            }
            if self.wet[i] < 0.000001 {
                continue;
            }
            let out = match i {
                0 => {
                    self.hold += 1;
                    if self.hold > (a * 7.) as u32 {
                        self.hold = 0;
                        let steps = 2_f32.powf(12. - a * 5.);
                        self.held = Frame {
                            l: (s.l * steps).round() / steps,
                            r: (s.r * steps).round() / steps,
                        };
                    }
                    let cutoff = 1200. + b * b * 14800.;
                    let c = (std::f32::consts::TAU * cutoff / rate).min(0.9);
                    self.low = self.low.combine(
                        Frame {
                            l: self.held.l - self.low.l,
                            r: self.held.r - self.low.r,
                        }
                        .gain(c),
                    );
                    self.low
                }
                1 => {
                    let g = 1. + a * 8.;
                    let driven = Frame {
                        l: (s.l * g).tanh() / g.sqrt(),
                        r: (s.r * g).tanh() / g.sqrt(),
                    };
                    let c = 0.04 + b * 0.8;
                    self.drive = self.drive.combine(
                        Frame {
                            l: driven.l - self.drive.l,
                            r: driven.r - self.drive.r,
                        }
                        .gain(c),
                    );
                    self.drive
                }
                _ => {
                    if self.release != b || self.rate != rate {
                        self.release = b;
                        self.rate = rate;
                        self.release_coeff = 1. - (-1. / (rate * (0.04 + b * 0.8))).exp();
                    }
                    let peak = s.peak();
                    self.envelope += (peak - self.envelope)
                        * if peak > self.envelope {
                            0.02
                        } else {
                            self.release_coeff
                        };
                    let threshold = 0.3 * (1. - a) + 0.015;
                    let reduction = (threshold / self.envelope.max(threshold)).sqrt();
                    s.gain(reduction * (1. + a * 0.8))
                }
            };
            s = s.gain(1. - self.wet[i]).combine(out.gain(self.wet[i]));
        }
        s
    }
}
