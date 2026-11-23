//! Bounded varispeed deck: relative fader scratch, independent slip playhead.
use crate::model::Frame;
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub speed: f32,
    pub range: u8,
    pub slip: bool,
    #[serde(default)]
    pub reverse_cut: bool,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            speed: 1.,
            range: 1,
            slip: true,
            reverse_cut: false,
        }
    }
}
impl Settings {
    pub fn valid(self) -> bool {
        self.speed.is_finite() && (0.5..=2.).contains(&self.speed) && self.range < 3
    }
    pub fn beats(self) -> f64 {
        [0.5, 1., 2.][usize::from(self.range.min(2))]
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Control {
    pub head: f64,
    pub slip: f64,
    pub target: f64,
    pub anchor: f64,
    pub fader: f32,
    pub anchor_fader: f32,
    pub grabbed: bool,
    #[serde(default)]
    pub returning: bool,
}
impl Control {
    pub fn valid(self) -> bool {
        [self.head, self.slip, self.target, self.anchor]
            .iter()
            .all(|x| x.is_finite() && x.abs() < 1e12)
            && self.fader.is_finite()
            && (0.0..=1.0).contains(&self.fader)
            && self.anchor_fader.is_finite()
            && (0.0..=1.0).contains(&self.anchor_fader)
    }
    pub fn normalized(mut self, len: usize) -> Self {
        let n = len.max(1) as f64;
        let offset = (self.slip / n).floor() * n;
        self.head -= offset;
        self.slip -= offset;
        self.target -= offset;
        self.anchor -= offset;
        self
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct Player {
    pub state: Control,
    pub active: bool,
    pub live: bool,
    pub audition: bool,
    pub rate: f32,
    anchor_pending: bool,
    fade: u32,
    fade_head: f64,
    fade_rate: f64,
    dc_in: Frame,
    dc_out: Frame,
    attack: f32,
    cut_gain: f32,
}
impl Player {
    pub fn start(&mut self, head: f64) {
        *self = Self::default();
        self.active = true;
        self.cut_gain = 1.;
        self.state.head = head;
        self.state.slip = head;
        self.state.target = head;
        self.state.anchor = head;
    }
    pub fn grab(&mut self, on: bool, fader: Option<f32>, cfg: Settings, rate: u32) {
        if on {
            if !self.active {
                self.start(0.);
            }
            self.state.grabbed = true;
            self.state.returning = false;
            self.live = true;
            self.audition = true;
            self.state.anchor = self.state.head;
            self.state.target = self.state.head;
            self.anchor_pending = fader.is_none();
            self.state.anchor_fader = fader.unwrap_or(self.state.fader).clamp(0., 1.);
            self.state.fader = self.state.anchor_fader;
        } else {
            self.fade_head = self.state.head;
            self.fade_rate = f64::from(self.rate);
            self.fade = (rate / 250).max(1);
            if cfg.slip {
                self.state.head = self.state.slip;
            } else {
                self.state.slip = self.state.head;
            }
            self.state.target = self.state.head;
            self.state.grabbed = false;
            self.live = false;
        }
    }
    pub fn move_to(&mut self, value: f32, cfg: Settings, bpm: f64, rate: u32) -> bool {
        if !self.live || !self.state.grabbed {
            return false;
        }
        let next = value.clamp(0., 1.);
        if next != self.state.fader {
            self.state.returning = next > self.state.fader;
        }
        self.state.fader = next;
        if self.anchor_pending {
            self.state.anchor_fader = self.state.fader;
            self.anchor_pending = false;
        }
        self.state.target = self.state.anchor
            + f64::from(self.state.fader - self.state.anchor_fader) * cfg.beats() * 60. / bpm
                * f64::from(rate);
        true
    }
    pub fn restore(&mut self, control: Control, elapsed: u64, cfg: Settings, rate: u32) {
        let was_active = self.active;
        let prior = self.state;
        self.state = control;
        self.live = false;
        self.audition = false;
        self.anchor_pending = false;
        self.active = true;
        if !was_active {
            self.cut_gain = if cfg.reverse_cut && control.grabbed && control.returning {
                0.
            } else {
                1.
            };
        }
        if !control.grabbed && prior.grabbed && was_active {
            self.fade = (rate / 250).max(1);
            self.fade_head = prior.head;
            self.fade_rate = f64::from(self.rate);
        }
        self.advance(elapsed, cfg, rate);
    }
    // Analytical catch-up; no loop proportional to elapsed transport time.
    pub fn advance(&mut self, n: u64, cfg: Settings, rate: u32) {
        let n = n as f64;
        self.state.slip += n * f64::from(cfg.speed);
        if self.state.grabbed {
            let alpha = 1. / (f64::from(rate) * 0.002 + 1.);
            if n == 1. {
                self.state.head += ((self.state.target - self.state.head) * alpha).clamp(-4., 4.);
                return;
            }
            let diff = self.state.target - self.state.head;
            let sign = diff.signum();
            let distance = diff.abs();
            let threshold = 4. / alpha;
            let linear = ((distance - threshold).max(0.) / 4.).min(n);
            let rest = n - linear;
            let remaining = (distance - linear * 4.) * (1. - alpha).powf(rest);
            self.state.head = self.state.target - sign * remaining;
        } else {
            self.state.head += n * f64::from(cfg.speed);
        }
    }
    pub fn sample(&mut self, pcm: &[Frame], cfg: Settings, rate: u32) -> Frame {
        if !self.active || pcm.is_empty() {
            return Frame::ZERO;
        }
        let previous = self.state.head;
        self.advance(1, cfg, rate);
        self.rate = (self.state.head - previous) as f32;
        let mut out = read(pcm, self.state.head);
        if self.fade > 0 {
            let mix = self.fade as f32 / (rate / 250).max(1) as f32;
            out = out
                .gain(1. - mix)
                .combine(read(pcm, self.fade_head).gain(mix));
            self.fade_head += self.fade_rate;
            self.fade -= 1;
        }
        // DC blocker silences a stationary stylus without a hard gate at zero speed.
        let coefficient = 1. - 50. / rate as f32;
        let filtered = Frame {
            l: out.l - self.dc_in.l + coefficient * self.dc_out.l,
            r: out.r - self.dc_in.r + coefficient * self.dc_out.r,
        };
        self.dc_in = out;
        self.dc_out = filtered;
        self.attack = (self.attack + 1. / (rate as f32 * 0.003)).min(1.);
        // Keep long running unwrapped counters bounded; relative targets follow together.
        if self.state.slip.abs() > pcm.len().max(1) as f64 * 1024. {
            self.state = self.state.normalized(pcm.len());
        }
        let audible = !(cfg.reverse_cut && self.state.grabbed && self.state.returning);
        let step = 1. / (rate as f32 * 0.003).max(1.);
        self.cut_gain = if audible {
            (self.cut_gain + step).min(1.)
        } else {
            (self.cut_gain - step).max(0.)
        };
        filtered.gain(self.attack * self.cut_gain)
    }
}
fn read(pcm: &[Frame], head: f64) -> Frame {
    let n = pcm.len() as i64;
    let wrapped = head.rem_euclid(n as f64);
    // rem_euclid can round a tiny negative remainder up to exactly len.
    // Treat that endpoint as the loop start before indexing or interpolating.
    let pos = if wrapped >= n as f64 { 0. } else { wrapped };
    let i = pos.floor() as i64;
    let f = (pos - i as f64) as f32;
    let a = pcm[(i - 1).rem_euclid(n) as usize];
    let b = pcm[i as usize];
    let c = pcm[(i + 1).rem_euclid(n) as usize];
    let d = pcm[(i + 2).rem_euclid(n) as usize];
    let cubic = |a: f32, b: f32, c: f32, d: f32| {
        b + 0.5 * f * (c - a + f * (2. * a - 5. * b + 4. * c - d + f * (3. * (b - c) + d - a)))
    };
    Frame {
        l: cubic(a.l, b.l, c.l, d.l),
        r: cubic(a.r, b.r, c.r, d.r),
    }
}

/// Bounded punch-in regions: replace only the fader gestures actually performed.
pub struct Spans {
    ranges: [(u64, u64); 128],
    len: usize,
    start: Option<u64>,
}
impl Default for Spans {
    fn default() -> Self {
        Self {
            ranges: [(0, 0); 128],
            len: 0,
            start: None,
        }
    }
}
impl Spans {
    pub fn reset(&mut self) {
        self.len = 0;
        self.start = None;
    }
    pub fn edge(&mut self, tick: u64, grabbed: bool) -> bool {
        if grabbed {
            if self.start.is_none() {
                if self.len == 128 {
                    return false;
                }
                self.start = Some(tick);
            }
        } else if let Some(start) = self.start.take() {
            self.ranges[self.len] = (start, tick);
            self.len += 1;
        }
        true
    }
    pub fn covered(&self, tick: u64, end: u64) -> bool {
        self.ranges[..self.len]
            .iter()
            .any(|&(a, b)| tick >= a && tick <= b)
            || self.start.is_some_and(|a| tick >= a && tick < end)
    }
}
