//! Chromatic sample playback and bounded waveform-aligned overlap/add Warp.
use crate::model::Frame;
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub root: u8,
    pub tune: f32,
    pub warp: bool,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordTarget {
    #[default]
    Input,
    Chromatic,
    Chop,
    Turntable,
}
impl RecordTarget {
    pub fn name(self) -> &'static str {
        match self {
            Self::Input => "INPUT",
            Self::Chromatic => "CHROMATIC",
            Self::Chop => "CHOP",
            Self::Turntable => "TURNTABLE",
        }
    }
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            root: 48,
            tune: 0.,
            warp: false,
        }
    }
}
impl Settings {
    pub fn valid(self) -> bool {
        self.root <= 127 && self.tune.is_finite() && (-24.0..=24.0).contains(&self.tune)
    }
    pub fn ratio(self, note: u8) -> f64 {
        2f64.powf((f64::from(note) - f64::from(self.root) + f64::from(self.tune)) / 12.)
    }
}
#[inline]
fn wrap(p: f64, n: f64) -> f64 {
    let v = if p < 0. {
        p + n
    } else if p >= n {
        p - n
    } else {
        p
    };
    if v >= 0. && v < n {
        v
    } else {
        let v = p.rem_euclid(n);
        if v >= n { 0. } else { v }
    }
}
#[inline]
fn at(audio: &[Frame], position: f64) -> Frame {
    if audio.is_empty() {
        return Frame::ZERO;
    }
    let p = wrap(position, audio.len() as f64);
    let i = p as usize;
    let j = if i + 1 == audio.len() { 0 } else { i + 1 };
    let f = (p - i as f64) as f32;
    audio[i].gain(1. - f).combine(audio[j].gain(f))
}
#[derive(Clone, Copy, Default)]
struct Grain {
    anchor: f64,
    age: u32,
}
#[derive(Clone, Copy, Default)]
pub struct Warp {
    grains: [Grain; 2],
    size: u32,
    frames: usize,
    rate: u32,
    ready: bool,
}
impl Warp {
    pub fn reset(&mut self) {
        self.ready = false;
    }
    fn score(
        audio: &[Frame],
        reference: &[Frame; 12],
        energy: f64,
        position: f64,
        pitch: f64,
    ) -> f64 {
        let mut dot = 0.;
        let mut e = 0.;
        for (i, r) in reference.iter().enumerate() {
            let x = at(audio, position + i as f64 * 4. * pitch);
            dot += f64::from(r.l) * f64::from(x.l) + f64::from(r.r) * f64::from(x.r);
            e += f64::from(x.l) * f64::from(x.l) + f64::from(x.r) * f64::from(x.r);
        }
        dot / (energy * e + 1e-24).sqrt()
    }
    fn align(
        audio: &[Frame],
        proposed: f64,
        reference_position: f64,
        pitch: f64,
        rate: u32,
    ) -> f64 {
        let mut reference = [Frame::ZERO; 12];
        let mut energy = 0.;
        for (i, r) in reference.iter_mut().enumerate() {
            *r = at(audio, reference_position + i as f64 * 4. * pitch);
            energy += f64::from(r.l) * f64::from(r.l) + f64::from(r.r) * f64::from(r.r);
        }
        if energy < 1e-10 {
            return proposed;
        }
        let radius = ((rate as usize * 6 / 1000).min(256).min(audio.len() / 4)) as i32;
        let mut best = 0;
        let mut score = Self::score(audio, &reference, energy, proposed, pitch);
        for offset in (-radius..=radius).step_by(4) {
            let value = Self::score(
                audio,
                &reference,
                energy,
                proposed + f64::from(offset),
                pitch,
            ) - f64::from(offset * offset) * 1e-10;
            if value > score {
                score = value;
                best = offset;
            }
        }
        let coarse = best;
        for offset in coarse - 3..=coarse + 3 {
            if offset.abs() > radius {
                continue;
            }
            let value = Self::score(
                audio,
                &reference,
                energy,
                proposed + f64::from(offset),
                pitch,
            ) - f64::from(offset * offset) * 1e-10;
            if value > score {
                score = value;
                best = offset;
            }
        }
        let p = proposed + f64::from(best);
        let left = Self::score(audio, &reference, energy, p - 1., pitch);
        let mid = Self::score(audio, &reference, energy, p, pitch);
        let right = Self::score(audio, &reference, energy, p + 1., pitch);
        let denominator = left - 2. * mid + right;
        let fraction = if denominator.abs() > 1e-12 {
            (0.5 * (left - right) / denominator).clamp(-0.5, 0.5)
        } else {
            0.
        };
        p + fraction
    }
    pub fn sample(
        &mut self,
        audio: &[Frame],
        position: f64,
        time_step: f64,
        pitch: f64,
        rate: u32,
    ) -> Frame {
        if audio.is_empty() {
            return Frame::ZERO;
        }
        if (pitch - time_step).abs() < 1e-10 {
            self.ready = false;
            return at(audio, position);
        }
        let size = ((rate as usize * 46 / 1000)
            .min(audio.len().max(64) / 2)
            .max(32)
            & !1) as u32;
        if !self.ready || self.frames != audio.len() || self.rate != rate || self.size != size {
            self.size = size;
            self.frames = audio.len();
            self.rate = rate;
            self.ready = true;
            self.grains[1] = Grain {
                anchor: position - f64::from(size / 2) * pitch,
                age: size / 2,
            };
            let proposed = position + f64::from(size / 2) * (time_step - pitch);
            self.grains[0] = Grain {
                anchor: Self::align(audio, proposed, position, pitch, rate),
                age: 0,
            };
        }
        let mut out = Frame::ZERO;
        for g in &self.grains {
            let phase = f64::from(g.age) / f64::from(size);
            let weight = (1. - (2. * phase - 1.).abs()) as f32;
            out = out.combine(at(audio, g.anchor + f64::from(g.age) * pitch).gain(weight));
        }
        for g in &mut self.grains {
            g.age += 1;
        }
        for i in 0..2 {
            if self.grains[i].age >= size {
                let other = self.grains[1 - i];
                let reference = other.anchor + f64::from(other.age) * pitch;
                let proposed = position + time_step + f64::from(size / 2) * (time_step - pitch);
                self.grains[i] = Grain {
                    anchor: Self::align(audio, proposed, reference, pitch, rate),
                    age: 0,
                };
            }
        }
        out
    }
}
pub fn time_step(frames: usize, source_ticks: u64, bpm: f64, rate: u32) -> f64 {
    if source_ticks == 0 || rate == 0 {
        1.
    } else {
        frames as f64 * bpm * crate::model::PPQ as f64
            / (source_ticks as f64 * 60. * f64::from(rate))
    }
}
#[derive(Clone, Copy)]
pub struct Voice {
    pub one_shot: bool,
    pub finished: bool,
    pub note: u8,
    pub source: u16,
    played_note: u8,
    pub slice: bool,
    position: f64,
    pitch: f64,
    target: f64,
    last_root: u8,
    last_tune: f32,
    warp: Warp,
}
impl Voice {
    pub fn new(note: u8, source: u16, settings: Settings) -> Self {
        let pitch = settings.ratio(note);
        Self {
            one_shot: false,
            finished: false,
            note,
            source,
            played_note: note,
            slice: false,
            position: 0.,
            pitch,
            target: pitch,
            last_root: settings.root,
            last_tune: settings.tune,
            warp: Warp::default(),
        }
    }
    pub fn set_played_note(&mut self, note: u8, settings: Settings) {
        self.played_note = note;
        self.target = settings.ratio(note);
        self.pitch = self.target;
    }
    pub fn sample(
        &mut self,
        audio: &[Frame],
        settings: Settings,
        ticks: u64,
        bpm: f64,
        rate: u32,
    ) -> Frame {
        if audio.is_empty() || self.finished {
            return Frame::ZERO;
        }
        if self.last_root != settings.root || self.last_tune != settings.tune {
            self.target = settings.ratio(if self.slice {
                settings.root
            } else {
                self.played_note
            });
            self.last_root = settings.root;
            self.last_tune = settings.tune;
        }
        self.pitch += (self.target - self.pitch) * 0.004;
        let step = if settings.warp {
            time_step(audio.len(), ticks, bpm, rate)
        } else {
            self.pitch
        };
        let sample = if settings.warp {
            self.warp
                .sample(audio, self.position, step, self.pitch, rate)
        } else {
            self.warp.reset();
            at(audio, self.position)
        };
        let fade = (f64::from(rate) * 0.002 * step).max(1.);
        let edge = (self.position / fade)
            .min((audio.len() as f64 - self.position) / fade)
            .clamp(0., 1.) as f32;
        let next = self.position + step;
        self.finished = self.one_shot && next >= audio.len() as f64;
        self.position = wrap(next, audio.len() as f64);
        sample.gain(edge)
    }
}

#[derive(Clone, Copy)]
pub struct LoopTime {
    pub frame: u64,
    pub origin: u64,
    pub phase: f64,
    pub ticks: u64,
    pub bpm: f64,
    pub rate: u32,
}
#[derive(Default)]
pub struct LoopPlayer {
    warp: Warp,
    last: Option<u64>,
    origin: u64,
    frames: usize,
    position: f64,
    tune: f32,
    pitch: f64,
    target: f64,
    warped: bool,
}
impl LoopPlayer {
    pub fn reset(&mut self) {
        self.last = None;
        self.warp.reset();
    }
    pub fn sample(&mut self, audio: &[Frame], settings: Settings, time: LoopTime) -> Frame {
        if audio.is_empty() {
            return Frame::ZERO;
        }
        let reset = self
            .last
            .is_none_or(|f| f.checked_add(1) != Some(time.frame))
            || self.origin != time.origin
            || self.frames != audio.len();
        if reset || self.tune != settings.tune {
            self.target = settings.ratio(settings.root);
            self.tune = settings.tune;
        }
        if reset {
            self.pitch = self.target;
            self.position = wrap(
                if settings.warp {
                    time.phase
                } else {
                    time.phase * self.pitch
                },
                audio.len() as f64,
            );
            self.warp.reset();
        }
        if self.warped != settings.warp {
            self.warp.reset();
            if !settings.warp {
                self.position = wrap(time.phase, audio.len() as f64);
            }
        }
        self.warped = settings.warp;
        self.last = Some(time.frame);
        self.origin = time.origin;
        self.frames = audio.len();
        self.pitch += (self.target - self.pitch) * 0.004;
        let pos = self.position;
        let sample = if settings.warp {
            self.warp.sample(
                audio,
                pos,
                time_step(audio.len(), time.ticks, time.bpm, time.rate),
                self.pitch,
                time.rate,
            )
        } else {
            at(audio, pos)
        };
        let j = wrap(pos, audio.len() as f64) as usize;
        let edge = (j.min(audio.len() - 1 - j) as f32 / (time.rate as f32 * 0.002)).min(1.);
        self.position = wrap(
            self.position
                + if settings.warp {
                    time_step(audio.len(), time.ticks, time.bpm, time.rate)
                } else {
                    self.pitch
                },
            audio.len() as f64,
        );
        sample.gain(edge)
    }
}
