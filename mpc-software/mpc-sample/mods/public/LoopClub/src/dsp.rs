use crate::model::*;
#[derive(Clone, Copy)]
pub struct Voice {
    pub active: bool,
    pub track: usize,
    pub note: u8,
    pub source: u16,
    pub drum: Option<usize>,
    pub layer: usize,
    pub delay_frames: u32,
    gate: bool,
    age: u64,
    phase: f32,
    synth_voice: crate::synth::SynthVoice,
    frequency: f32,
    velocity: f32,
    noise: u32,
    low: f32,
    age_seconds: f32,
    pcm_pos: f64,
    pcm_tune: f32,
    pcm_step: f64,
    pcm_reverse: Option<bool>,
    layer_gain: f32,
}
impl Default for Voice {
    fn default() -> Self {
        Self {
            active: false,
            track: 0,
            note: 0,
            source: 0,
            drum: None,
            layer: 0,
            delay_frames: 0,
            gate: false,
            age: 0,
            phase: 0.,
            synth_voice: crate::synth::SynthVoice::default(),
            frequency: 440.,
            velocity: 0.,
            noise: 0x12345678,
            low: 0.,
            age_seconds: 0.,
            pcm_pos: 0.,
            pcm_tune: f32::NAN,
            pcm_step: 1.,
            pcm_reverse: None,
            layer_gain: f32::NAN,
        }
    }
}
impl Voice {
    // Voice start captures note ownership and synthesis settings atomically.
    #[allow(clippy::too_many_arguments)]
    pub fn start(
        &mut self,
        track: usize,
        note: u8,
        velocity: u8,
        source: u16,
        drum: Option<usize>,
        age: u64,
        synth: Synth,
        tune: f32,
    ) {
        *self = Self {
            active: true,
            track,
            note,
            source,
            drum,
            gate: true,
            age,
            frequency: 440.
                * 2_f32.powf((f32::from(note) - 69. + f32::from(synth.octave) * 12. + tune) / 12.),
            velocity: f32::from(velocity) / 127.,
            noise: 0xdeadbeef ^ (u32::from(note) * 0x10203) ^ (age as u32),
            ..Self::default()
        }
    }
    pub fn release(&mut self) {
        self.gate = false
    }
    pub fn age(&self) -> u64 {
        self.age
    }
    pub fn apply_layer_gain(&mut self, x: Frame, target: f32) -> Frame {
        if !self.layer_gain.is_finite() {
            self.layer_gain = target;
        }
        self.layer_gain += 0.02 * (target - self.layer_gain);
        x.gain(self.layer_gain)
    }
    pub fn sample_pcm(&mut self, pcm: &[Frame], rate: f32, drum: Drum) -> Frame {
        let index = self.pcm_pos as usize;
        let duration = drum.decay;
        if !self.active || index >= pcm.len() || self.age_seconds >= duration {
            self.active = false;
            return Frame::ZERO;
        }
        let f = self.pcm_pos.fract() as f32;
        let reverse = *self
            .pcm_reverse
            .get_or_insert(drum.layers[self.layer].reverse);
        let at = |i: usize| {
            if reverse {
                pcm[pcm.len() - 1 - i]
            } else {
                pcm[i]
            }
        };
        let a = at(index);
        let b = at((index + 1).min(pcm.len() - 1));
        let fade = ((duration - self.age_seconds) / (duration * 0.05).max(0.002)).clamp(0., 1.);
        let x = a.gain(1. - f).combine(b.gain(f)).gain(self.velocity * fade);
        if self.pcm_tune != drum.tune {
            self.pcm_step = f64::from(2_f32.powf(drum.tune / 12.));
            self.pcm_tune = drum.tune;
        }
        self.pcm_pos += self.pcm_step;
        self.age_seconds += 1. / rate;
        x
    }
    pub fn sample(&mut self, rate: f32, synth: &Synth, drum: Drum) -> f32 {
        if !self.active {
            return 0.;
        }
        self.age_seconds += 1. / rate;
        if let Some(k) = self.drum {
            self.noise ^= self.noise << 13;
            self.noise ^= self.noise >> 17;
            self.noise ^= self.noise << 5;
            let noise = self.noise as i32 as f32 / i32::MAX as f32;
            let (base, decay) = match k {
                0 => (48., 0.45),
                1 => (180., 0.2),
                2 => (6700., 0.045),
                3 => (5800., 0.38),
                4 => (1400., 0.12),
                5 => (460., 0.045),
                6 => (7900., 0.14),
                7 => (620., 0.16),
                8 => (100., 0.4),
                9 => (155., 0.32),
                10 => (260., 0.22),
                11 => (9000., 0.07),
                12 => (4600., 0.65),
                13 => (8300., 1.2),
                14 => (900., 0.45),
                _ => (360., 0.35),
            };
            let decay = decay * (drum.decay / 0.25);
            let pitch = base * 2_f32.powf(drum.tune / 12.);
            let env = (-self.age_seconds * 6. / decay.max(0.01)).exp();
            let freq = if k == 0 {
                pitch * (1. + 3. * (-self.age_seconds * 70.).exp())
            } else {
                pitch
            };
            self.phase = (self.phase + freq.min(rate * 0.45) / rate).fract();
            let tone = (self.phase * std::f32::consts::TAU).sin();
            self.low += 0.18 * (noise - self.low);
            let high = noise - self.low;
            let sound = match k {
                0 => tone * 0.9 + high * (-self.age_seconds * 150.).exp() * 0.08,
                1 => tone * 0.22 + high * 0.68,
                2 | 3 | 6 | 11 | 12 | 13 => high * (0.75 + 0.25 * (self.phase * 37.).sin()),
                4 => high * (0.55 + 0.45 * (self.age_seconds * 650.).sin().max(0.)),
                5 | 7 | 14 => tone * 0.55 + (self.phase * std::f32::consts::TAU * 1.47).sin() * 0.3,
                _ => tone * 0.85 + high * 0.1,
            };
            if env < 0.00005 {
                self.active = false
            }
            return sound * env * self.velocity * 0.5;
        }
        let x = self
            .synth_voice
            .sample(rate, self.frequency, synth, self.gate);
        if self.synth_voice.done() {
            self.active = false;
        }
        x * self.velocity * 0.22
    }
}

pub struct FxState {
    history: Vec<Frame>,
    write: usize,
    low: Frame,
    envelope: f32,
    filter: Frame,
    band: Frame,
    tone: Frame,
    hold: Frame,
    hold_phase: u32,
    phase: f32,
    tape: f32,
    wet: f32,
    last: Fx,
    read: f64,
}
impl FxState {
    pub fn new(rate: u32) -> Self {
        Self {
            history: vec![Frame::ZERO; rate as usize * 2 + 1],
            write: 0,
            low: Frame::ZERO,
            envelope: 0.,
            filter: Frame::ZERO,
            band: Frame::ZERO,
            tone: Frame::ZERO,
            hold: Frame::ZERO,
            hold_phase: 0,
            phase: 0.,
            tape: 1.,
            wet: 0.,
            last: Fx::default(),
            read: 0.,
        }
    }
    fn at(&self, delay: f64) -> Frame {
        let n = self.history.len();
        let p = (self.write as f64 - delay).rem_euclid(n as f64);
        let i = p.floor() as usize;
        let f = p.fract() as f32;
        self.history[i]
            .gain(1. - f)
            .combine(self.history[(i + 1) % n].gain(f))
    }
    pub fn process(&mut self, input: Frame, fx: Fx, rate: f32, bpm: f32) -> Frame {
        // Preserve the complete history even while bypassed, so a later Reverse
        // or Repeat trigger still sees the same audio without the full FX path.
        if fx.effect == Effect::Off && self.wet == 0. {
            self.history[self.write] = input;
            self.write += 1;
            if self.write == self.history.len() {
                self.write = 0;
            }
            self.tape = 1.;
            self.read = 0.;
            return input;
        }
        if fx.effect != Effect::Off && fx.effect != self.last.effect {
            self.read = 0.;
            self.tape = 1.;
        }
        let freeze = fx.effect == Effect::Repeat
            || (fx.effect == Effect::Off
                && self.last.effect == Effect::Repeat
                && self.wet > 0.00001);
        if !freeze {
            self.history[self.write] = input;
        }
        let target = if fx.effect == Effect::Off { 0. } else { fx.mix };
        if fx.effect != Effect::Off {
            self.last = fx
        }
        self.wet += (target - self.wet) * 0.004;
        let p = self.last;
        let mut out = input;
        if self.wet > 0.00001 {
            self.low.l += (input.l - self.low.l) * (0.002 + p.a * 0.4);
            self.low.r += (input.r - self.low.r) * (0.002 + p.a * 0.4);
            out = match p.effect {
                Effect::Off => input,
                Effect::Drive => {
                    let g = 1. + p.a * 15.;
                    Frame {
                        l: (input.l * g).tanh() / g.sqrt(),
                        r: (input.r * g).tanh() / g.sqrt(),
                    }
                }
                Effect::Filter => {
                    let f = 0.002 + p.a * 0.6;
                    let damping = 0.25 + (1. - p.b) * 1.7;
                    self.band = self.band.combine(
                        Frame {
                            l: input.l - self.filter.l - damping * self.band.l,
                            r: input.r - self.filter.r - damping * self.band.r,
                        }
                        .gain(f),
                    );
                    self.filter = self.filter.combine(self.band.gain(f));
                    self.filter
                }
                Effect::Compressor => {
                    self.envelope += (input.peak() - self.envelope)
                        * if input.peak() > self.envelope {
                            0.03
                        } else {
                            0.0005
                        };
                    let threshold = 0.05 + (1. - p.a) * 0.6;
                    input.gain(if self.envelope > threshold {
                        (threshold / self.envelope).powf(1. - 1. / (1.5 + p.b * 6.5))
                    } else {
                        1.
                    })
                }
                Effect::LoFi | Effect::Crush => {
                    self.hold_phase += 1;
                    let period = 1 + (p.a * 30.) as u32;
                    if self.hold_phase >= period {
                        self.hold_phase = 0;
                        let steps = 2_f32.powf(3. + (1. - p.b) * 11.);
                        self.hold = Frame {
                            l: (input.l * steps).round() / steps,
                            r: (input.r * steps).round() / steps,
                        }
                    }
                    self.hold
                }
                Effect::Isolator => {
                    let high = Frame {
                        l: input.l - self.low.l,
                        r: input.r - self.low.r,
                    };
                    self.low.gain(p.a * 2.).combine(high.gain(p.b * 2.))
                }
                Effect::Repeat => {
                    let divisions = [0.125, 0.25, 0.5, 1.];
                    let beats = divisions[(p.a * 3.99) as usize];
                    let len = ((rate * 60. / bpm * beats).max(1.) as f64)
                        .min((self.history.len() - 1) as f64);
                    self.read = (self.read + 1.).rem_euclid(len);
                    let edge = (self.read.min(len - self.read) / (1. + f64::from(p.b) * len * 0.05))
                        .clamp(0., 1.) as f32;
                    self.at(len - self.read).gain(edge)
                }
                Effect::Reverse => {
                    let len = f64::from(rate * (0.08 + p.a * 0.8));
                    self.read = (self.read + 2.).rem_euclid(len);
                    self.at(self.read)
                }
                Effect::TapeStop => {
                    self.tape = (self.tape - 1. / (rate * (0.15 + p.a * 2.))).max(0.);
                    self.read += 1. - f64::from(self.tape);
                    self.at(self.read.min(f64::from(rate) * 1.9))
                        .gain(self.tape.sqrt())
                }
                Effect::DubEcho => {
                    let beats = [0.25, 0.5, 0.75, 1.][(p.a * 3.99) as usize];
                    let delayed = self.at(f64::from(rate * 60. / bpm * beats));
                    self.history[self.write] = input.combine(delayed.gain(p.b * 0.7));
                    input.combine(delayed)
                }
                Effect::Wobble => {
                    self.phase = (self.phase + (0.2 + p.a * 4.) / rate).fract();
                    let delay =
                        rate * (0.008 + 0.006 * p.b * (self.phase * std::f32::consts::TAU).sin());
                    self.at(f64::from(delay))
                }
            };
            if matches!(p.effect, Effect::Drive | Effect::Reverse | Effect::TapeStop) {
                let coefficient = 0.02 + (1. - p.b) * 0.96;
                self.tone = self.tone.combine(
                    Frame {
                        l: out.l - self.tone.l,
                        r: out.r - self.tone.r,
                    }
                    .gain(coefficient),
                );
                out = self.tone;
            }
        } else {
            self.tape = 1.;
            self.read = 0.;
        }
        if !freeze {
            self.write += 1;
            if self.write == self.history.len() {
                self.write = 0;
            }
        }
        input.gain(1. - self.wet).combine(out.gain(self.wet))
    }
}
pub struct Delay {
    buffer: Vec<Frame>,
    write: usize,
}
impl Delay {
    pub fn new(rate: u32) -> Self {
        Self {
            buffer: vec![Frame::ZERO; rate as usize * 3],
            write: 0,
        }
    }
    pub fn process(&mut self, input: Frame, rate: f32, bpm: f32) -> Frame {
        let delay = (rate * 60. / bpm * 0.75) as usize;
        let p =
            (self.write + self.buffer.len() - delay.min(self.buffer.len() - 1)) % self.buffer.len();
        let out = self.buffer[p];
        self.buffer[self.write] = input.combine(out.gain(0.35));
        self.write = (self.write + 1) % self.buffer.len();
        out
    }
}
pub struct Room {
    lines: [Vec<Frame>; 4],
    positions: [usize; 4],
    damping: [Frame; 4],
}
impl Room {
    pub fn new(rate: u32) -> Self {
        Self {
            lines: [0.0297, 0.0371, 0.0411, 0.0437]
                .map(|t| vec![Frame::ZERO; (rate as f32 * t) as usize + 1]),
            positions: [0; 4],
            damping: [Frame::ZERO; 4],
        }
    }
    pub fn process(&mut self, input: Frame) -> Frame {
        let mut out = Frame::ZERO;
        for i in 0..4 {
            let p = self.positions[i];
            let sample = self.lines[i][p];
            self.damping[i] = self.damping[i].gain(0.65).combine(sample.gain(0.35));
            let feedback = Frame {
                l: self.damping[i].r,
                r: self.damping[i].l,
            };
            self.lines[i][p] = input.gain(0.3).combine(feedback.gain(0.76));
            self.positions[i] = (p + 1) % self.lines[i].len();
            out = out.combine(sample.gain(0.25));
        }
        out
    }
}
