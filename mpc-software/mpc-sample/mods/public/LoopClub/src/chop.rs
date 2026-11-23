//! Non-destructive regions into a track's original audio, never copied per slice.
use crate::model::Frame;
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Slice {
    pub start: u32,
    pub end: u32,
    pub level: f32,
}
impl Default for Slice {
    fn default() -> Self {
        Self {
            start: 0,
            end: 0,
            level: 1.,
        }
    }
}
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Equal,
    Transient,
    Bpm,
    Manual,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Chops {
    pub slices: [Slice; 16],
    pub gate: bool,
    pub count: u8,
    pub regions: u8,
    pub mode: Mode,
    pub sensitivity: f32,
    pub min_gap_ms: f32,
    pub grid: u8,
    pub pending: bool,
}
impl Default for Chops {
    fn default() -> Self {
        Self {
            slices: [Slice::default(); 16],
            gate: false,
            count: 16,
            regions: 16,
            mode: Mode::Equal,
            sensitivity: 0.5,
            min_gap_ms: 70.,
            grid: 0,
            pending: false,
        }
    }
}
pub const GRID_BEATS: [f64; 11] = [
    4.,
    2.,
    1.,
    0.5,
    0.25,
    3.,
    1.5,
    0.75,
    2. / 3.,
    1. / 3.,
    1. / 6.,
];
pub const GRID_NAMES: [&str; 11] = [
    "1 BAR", "1/2", "1/4", "1/8", "1/16", "1/2D", "1/4D", "1/8D", "1/4T", "1/8T", "1/16T",
];
impl Chops {
    pub fn divide(&mut self, frames: usize) {
        let n = frames.min(u32::MAX as usize) as u32;
        self.count = 16;
        self.regions = 16;
        self.mode = Mode::Equal;
        self.pending = false;
        for (i, s) in self.slices.iter_mut().enumerate() {
            let start = (u64::from(n) * i as u64 / 16) as u32;
            let end = (u64::from(n) * (i + 1) as u64 / 16) as u32;
            *s = Slice {
                start,
                end: if n == 0 { 0 } else { end.max(start + 1).min(n) },
                level: 1.,
            };
        }
    }
    pub fn valid(&self, frames: usize) -> bool {
        (1..=16).contains(&self.count)
            && (1..=16).contains(&self.regions)
            && self.grid < 11
            && self.sensitivity.is_finite()
            && (0.0..=1.0).contains(&self.sensitivity)
            && self.min_gap_ms.is_finite()
            && (10.0..=500.0).contains(&self.min_gap_ms)
            && self.slices.iter().enumerate().all(|(i, s)| {
                s.level.is_finite()
                    && (0.0..=1.0).contains(&s.level)
                    && if frames == 0 || i >= usize::from(self.count) {
                        s.start == 0 && s.end == 0
                    } else {
                        s.start < s.end && s.end as usize <= frames
                    }
            })
    }
    pub fn apply(
        &mut self,
        frames: usize,
        rate: u32,
        bpm: f64,
        features: &crate::features::Features,
    ) -> bool {
        if frames == 0 {
            return false;
        }
        let mut points = [0u32; 17];
        let mut count = 1usize;
        match self.mode {
            Mode::Equal => {
                count = usize::from(self.regions.clamp(1, 16));
                for (i, p) in points[..=count].iter_mut().enumerate() {
                    *p = (frames as u64 * i as u64 / count as u64) as u32;
                }
            }
            Mode::Manual => {
                points[0] = 0;
                points[1] = frames as u32;
            }
            Mode::Bpm => {
                let span = f64::from(rate) * 60. / bpm * GRID_BEATS[usize::from(self.grid.min(10))];
                count = (frames as f64 / span).ceil().max(1.) as usize;
                if count > 16 {
                    return false;
                }
                for (i, p) in points[..count].iter_mut().enumerate() {
                    *p = (i as f64 * span).round() as u32;
                }
                points[count] = frames as u32;
            }
            Mode::Transient => {
                let gap = (self.min_gap_ms * f64::from(rate) as f32 / 1000.).round() as u32;
                let threshold = 1.8 + (1. - self.sensitivity) * 6.;
                let mut chosen = [false; 128];
                while count < usize::from(self.regions.clamp(1, 16)) {
                    let next = features.onsets[..features.onset_count]
                        .iter()
                        .enumerate()
                        .filter(|(i, o)| {
                            !chosen[*i]
                                && o.frame > 0
                                && o.frame < frames as u32
                                && o.strength >= threshold
                                && points[..count].iter().all(|p| p.abs_diff(o.frame) >= gap)
                        })
                        .max_by(|(_, a), (_, b)| a.strength.total_cmp(&b.strength));
                    let Some((i, onset)) = next else { break };
                    points[count] = onset.frame;
                    chosen[i] = true;
                    count += 1;
                }
                points[..count].sort_unstable();
                points[count] = frames as u32;
            }
        }
        let mut next = [Slice::default(); 16];
        for i in 0..count {
            if points[i] >= points[i + 1] {
                return false;
            }
            next[i] = Slice {
                start: points[i],
                end: points[i + 1],
                level: 1.,
            };
        }
        self.slices = next;
        self.count = count as u8;
        self.pending = false;
        true
    }
    pub fn split(&mut self, index: usize, at: u32) -> bool {
        let count = usize::from(self.count);
        if index >= count
            || count >= 16
            || at <= self.slices[index].start
            || at >= self.slices[index].end
        {
            return false;
        }
        for i in ((index + 1)..count).rev() {
            self.slices[i + 1] = self.slices[i]
        }
        let old = self.slices[index];
        self.slices[index].end = at;
        self.slices[index + 1] = Slice { start: at, ..old };
        self.count += 1;
        self.mode = Mode::Manual;
        self.pending = false;
        true
    }
    pub fn delete(&mut self, index: usize) -> bool {
        let count = usize::from(self.count);
        if index >= count || count <= 1 {
            return false;
        }
        let left = if index + 1 < count { index } else { index - 1 };
        let a = self.slices[left];
        let b = self.slices[left + 1];
        self.slices[left] = Slice {
            start: a.start.min(b.start),
            end: a.end.max(b.end),
            level: a.level,
        };
        for i in (left + 1)..count - 1 {
            self.slices[i] = self.slices[i + 1]
        }
        self.slices[count - 1] = Slice::default();
        self.count -= 1;
        self.mode = Mode::Manual;
        self.pending = false;
        true
    }
}
#[derive(Clone, Copy, Default)]
pub struct SliceVoice {
    pub active: bool,
    pub track: usize,
    pub pad: usize,
    pub start: usize,
    pub end: usize,
    pub at: usize,
    pub velocity: f32,
    pub gate: bool,
    pub release: bool,
    pub env: f32,
    pub age: u64,
    pub sampler: Option<crate::audio::Voice>,
}
impl SliceVoice {
    pub fn sample(&mut self, audio: &[Frame], rate: u32) -> Frame {
        if !self.active {
            return Frame::ZERO;
        }
        if self.at >= self.end || self.at >= audio.len() {
            self.active = false;
            return Frame::ZERO;
        }
        if self.release {
            self.env = (self.env - 1. / (rate as f32 * 0.005)).max(0.);
            if self.env <= 0. {
                self.active = false;
                return Frame::ZERO;
            }
        }
        let fade = (rate as f32 * 0.001).max(1.);
        let edge = ((self.at - self.start) as f32 / fade)
            .min((self.end - self.at) as f32 / fade)
            .min(1.);
        let out = audio[self.at].gain(self.velocity * self.env * edge);
        self.at += 1;
        out
    }
}
