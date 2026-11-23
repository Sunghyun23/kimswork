use crate::model::*;
#[derive(Clone, Copy)]
pub struct Voice {
    pub active: bool,
    pub track: usize,
    pub note: u8,
    pub source: u16,
    pub drum: Option<usize>,
    pub layer: usize,
    pub xpm_zone: usize,
    pub pcm_gate: bool,
    pcm_release: bool,
    pcm_release_env: f32,
    pub delay_frames: u32,
    gate: bool,
    age: u64,
    phase: f32,
    synth_voice: crate::synth::SynthVoice,
    frequency: f32,
    target_frequency: f32,
    glide_ratio: f64,
    glide_frequency: f64,
    glide_remaining: u32,
    one_shot: bool,
    shot_frames: u32,
    forced_gate: bool,
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
            xpm_zone: 0,
            pcm_gate: false,
            pcm_release: false,
            pcm_release_env: 1.,
            delay_frames: 0,
            gate: false,
            age: 0,
            phase: 0.,
            synth_voice: crate::synth::SynthVoice::default(),
            frequency: 440.,
            target_frequency: 440.,
            glide_ratio: 1.,
            glide_frequency: 440.,
            glide_remaining: 0,
            one_shot: false,
            shot_frames: 0,
            forced_gate: false,
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
    pub fn configure_trigger(&mut self, cfg: crate::performance::Settings, rate: u32) {
        self.one_shot = cfg.trigger == 2;
        self.forced_gate = cfg.trigger == 1;
        self.shot_frames = if self.one_shot && self.drum.is_none() {
            (cfg.shot_ms * rate as f32 / 1000.).round() as u32
        } else {
            0
        };
    }
    pub fn note_off(&mut self) {
        if !self.one_shot && (self.drum.is_none() || self.pcm_gate || self.forced_gate) {
            self.release();
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn retarget(
        &mut self,
        note: u8,
        velocity: u8,
        source: u16,
        synth: Synth,
        glide: f32,
        rate: u32,
        legato: bool,
    ) {
        self.note = note;
        self.source = source;
        self.velocity = f32::from(velocity) / 127.;
        self.gate = true;
        self.target_frequency =
            440. * 2_f32.powf((f32::from(note) - 69. + f32::from(synth.octave) * 12.) / 12.);
        self.glide_remaining = (glide * rate as f32 / 1000.).round() as u32;
        if self.glide_remaining == 0 {
            self.frequency = self.target_frequency;
            self.glide_ratio = 1.;
        } else {
            self.glide_ratio = (f64::from(self.target_frequency) / f64::from(self.frequency))
                .powf(1. / f64::from(self.glide_remaining));
        }
        self.glide_frequency = f64::from(self.frequency);
        if !legato {
            self.synth_voice.retrigger();
        }
    }
    pub fn release(&mut self) {
        if self.delay_frames > 0 {
            self.active = false;
        }
        if self.pcm_gate {
            self.pcm_release = true;
        }
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
        if self.pcm_release {
            self.pcm_release_env = (self.pcm_release_env - 1. / (rate * 0.005)).max(0.);
            if self.pcm_release_env <= 0. {
                self.active = false;
                return Frame::ZERO;
            }
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
        let x = a
            .gain(1. - f)
            .combine(b.gain(f))
            .gain(self.velocity * fade * self.pcm_release_env);
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
        if self.shot_frames > 0 {
            self.shot_frames -= 1;
            if self.shot_frames == 0 {
                self.release();
            }
        }
        if self.glide_remaining > 0 {
            self.glide_remaining -= 1;
            self.frequency = if self.glide_remaining == 0 {
                self.target_frequency
            } else {
                self.glide_frequency *= self.glide_ratio;
                self.glide_frequency as f32
            };
        }
        if self.forced_gate && !self.gate && self.drum.is_some() {
            self.pcm_release_env = (self.pcm_release_env - 1. / (rate * 0.005)).max(0.);
            if self.pcm_release_env == 0. {
                self.active = false;
                return 0.;
            }
        }
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
            return sound * env * self.velocity * 0.5 * self.pcm_release_env;
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
    mod_lfo: crate::synth::Lfo,
    mod_rate_key: (u32, u32, bool, u8, u8),
    mod_hz: f32,
    mod_signal: f32,
    mod_depth: f32,
    mod_filter: Frame,
    mod_coefficient: f32,
    mod_counter: u8,
    tape: f32,
    wet: f32,
    last: Fx,
    read: f64,
    history_written: usize,
    history_signal_age: usize,
    repeat_ready: bool,
    repeat_requested: bool,
    repeat_remaining: usize,
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
            mod_lfo: Default::default(),
            mod_rate_key: (0, 0, false, 0, 0),
            mod_hz: 0.,
            mod_signal: 0.,
            mod_depth: 0.,
            mod_filter: Frame::ZERO,
            mod_coefficient: 0.,
            mod_counter: 0,
            tape: 1.,
            wet: 0.,
            last: Fx::default(),
            read: 0.,
            history_written: 0,
            history_signal_age: usize::MAX,
            repeat_ready: false,
            repeat_requested: false,
            repeat_remaining: 0,
        }
    }
    fn at(&self, delay: f64) -> Frame {
        let n = self.history.len();
        let p = wrap_history(self.write as f64 - delay, n as f64);
        let i = p.floor() as usize;
        let f = p.fract() as f32;
        self.history[i]
            .gain(1. - f)
            .combine(self.history[if i + 1 == n { 0 } else { i + 1 }].gain(f))
    }
    fn remember_signal(&mut self, input: Frame) {
        self.history_written = (self.history_written + 1).min(self.history.len());
        self.history_signal_age = if input.peak() > 1e-6 {
            0
        } else {
            self.history_signal_age.saturating_add(1)
        };
    }
    #[inline]
    pub fn process(&mut self, input: Frame, fx: Fx, rate: f32, bpm: f32) -> Frame {
        // Keep bypass small enough to inline into the16-track mixer. The full
        // effect function otherwise forces a large config copy/call32times/frame.
        if fx.effect == Effect::Off && self.wet < 0.00001 {
            self.repeat_requested = false;
            self.wet = 0.;
            self.history[self.write] = input;
            self.remember_signal(input);
            self.write += 1;
            if self.write == self.history.len() {
                self.write = 0;
            }
            self.tape = 1.;
            self.read = 0.;
            return input;
        }
        self.process_active(input, fx, rate, bpm)
    }
    fn process_active(&mut self, input: Frame, fx: Fx, rate: f32, bpm: f32) -> Frame {
        let repeat_entered = fx.effect == Effect::Repeat && !self.repeat_requested;
        self.repeat_requested = fx.effect == Effect::Repeat;
        let mut repeat_capture_complete = false;
        if fx.effect != Effect::Off && fx.effect != self.last.effect {
            self.read = 0.;
            self.tape = 1.;
        }
        if repeat_entered {
            self.read = 0.;
            let len = (rate * 60. / bpm * [0.125, 0.25, 0.5, 1.][(fx.a * 3.99) as usize])
                .max(1.)
                .min((self.history.len() - 1) as f32) as usize;
            self.repeat_ready = self.history_written >= len && self.history_signal_age < len;
            self.repeat_remaining = 0;
        }
        // A stopped/silent source must not become a permanently frozen zero loop.
        // Arm from the first audible sample and collect one complete repeat window.
        if fx.effect == Effect::Repeat && !self.repeat_ready {
            if self.repeat_remaining == 0 && input.peak() > 1e-6 {
                self.repeat_remaining =
                    (rate * 60. / bpm * [0.125, 0.25, 0.5, 1.][(fx.a * 3.99) as usize])
                        .max(1.)
                        .min((self.history.len() - 1) as f32) as usize;
            }
            if self.repeat_remaining > 0 {
                self.repeat_remaining -= 1;
                repeat_capture_complete = self.repeat_remaining == 0;
            }
        }
        let freeze = (fx.effect == Effect::Repeat && self.repeat_ready)
            || (fx.effect == Effect::Off
                && self.last.effect == Effect::Repeat
                && self.repeat_ready
                && self.wet > 0.00001);
        if !freeze {
            self.history[self.write] = input;
            self.remember_signal(input);
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
                Effect::Repeat if !self.repeat_ready => input,
                Effect::Repeat => {
                    let divisions = [0.125, 0.25, 0.5, 1.];
                    let beats = divisions[(p.a * 3.99) as usize];
                    let len = ((rate * 60. / bpm * beats).max(1.) as f64)
                        .min((self.history.len() - 1) as f64);
                    self.read = wrap_history(self.read + 1., len);
                    let edge = (self.read.min(len - self.read) / (1. + f64::from(p.b) * len * 0.05))
                        .clamp(0., 1.) as f32;
                    self.at(len - self.read).gain(edge)
                }
                Effect::Reverse => {
                    let len = f64::from(rate * (0.08 + p.a * 0.8));
                    self.read = wrap_history(self.read + 2., len);
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
                Effect::Wobble | Effect::Tremolo | Effect::AutoPan | Effect::AutoFilter => {
                    let key = (
                        p.a.to_bits(),
                        bpm.to_bits(),
                        p.modulation.sync,
                        p.modulation.division,
                        p.effect as u8,
                    );
                    if self.mod_rate_key != key {
                        self.mod_rate_key = key;
                        self.mod_hz = p.rate_hz(bpm);
                    }
                    let lfo = self.mod_lfo.sample(rate, self.mod_hz, p.modulation.shape);
                    self.mod_signal += (lfo - self.mod_signal) * 0.02;
                    self.mod_depth += (p.b - self.mod_depth) * 0.004;
                    let value = self.mod_signal;
                    let depth = self.mod_depth;
                    match p.effect {
                        Effect::Tremolo => input.gain(1. - depth * (value + 1.) * 0.5),
                        Effect::AutoPan => {
                            let pan = (value * depth).clamp(-1., 1.);
                            let mid = (input.l + input.r) * 0.5;
                            let side = (input.l - input.r) * 0.5 * (1. - pan.abs()).sqrt();
                            Frame {
                                l: mid * (1. - pan).sqrt() + side,
                                r: mid * (1. + pan).sqrt() - side,
                            }
                        }
                        Effect::AutoFilter => {
                            if self.mod_counter == 0 {
                                let cutoff =
                                    (800. * 2_f32.powf(value * depth * 4.)).clamp(40., rate * 0.18);
                                self.mod_coefficient =
                                    1. - (-std::f32::consts::TAU * cutoff / rate).exp();
                            }
                            self.mod_counter = (self.mod_counter + 1) % 16;
                            self.mod_filter = self.mod_filter.combine(
                                Frame {
                                    l: input.l - self.mod_filter.l,
                                    r: input.r - self.mod_filter.r,
                                }
                                .gain(self.mod_coefficient),
                            );
                            input.gain(1. - depth).combine(self.mod_filter.gain(depth))
                        }
                        _ => self.at(f64::from(rate * (0.008 + 0.006 * depth * value))),
                    }
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
        if repeat_capture_complete {
            self.repeat_ready = true;
            self.read = 0.;
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

// FX positions normally stay within one cycle. Keep the full modulo fallback
// for abrupt rate/window changes and parameter extremes.
#[inline]
fn wrap_history(position: f64, length: f64) -> f64 {
    let p = if position < 0. {
        position + length
    } else if position >= length {
        position - length
    } else {
        position
    };
    if p >= 0. && p < length {
        return p;
    }
    let p = position.rem_euclid(length);
    if p >= length { 0. } else { p }
}
#[cfg(test)]
mod history_wrap_tests {
    #[test]
    fn fast_wrap_matches_reference_for_boundaries_and_abrupt_window_changes() {
        for n in [1., 3., 8001., 88201.] {
            for p in [
                -1e9,
                -3. * n,
                -n,
                -0.001,
                -f64::MIN_POSITIVE,
                0.,
                n - 0.001,
                n,
                2. * n + 0.3,
                1e9,
            ] {
                let actual = super::wrap_history(p, n);
                let expected = p.rem_euclid(n);
                assert!(actual >= 0. && actual < n);
                assert!((actual - expected).abs() < 1e-10 || expected == n && actual == 0.);
            }
        }
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    #[test]
    fn glide_reaches_target_in_the_selected_time_without_a_pitch_jump() {
        let s = Synth::default();
        let mut v = Voice::default();
        v.start(0, 48, 100, 2, None, 1, s, 0.);
        for _ in 0..100 {
            v.sample(8000., &s, Drum::default());
        }
        let begin = v.frequency;
        v.retarget(60, 100, 2, s, 200., 8000, true);
        assert_eq!(v.frequency, begin);
        for _ in 0..800 {
            v.sample(8000., &s, Drum::default());
        }
        assert!(v.frequency > begin && v.frequency < v.target_frequency);
        for _ in 0..800 {
            v.sample(8000., &s, Drum::default());
        }
        assert_eq!(v.frequency, v.target_frequency);
        assert_eq!(v.glide_remaining, 0);
    }
    #[test]
    fn legato_keeps_envelope_while_mono_retriggers_it() {
        let s = Synth {
            attack: 1.,
            ..Synth::default()
        };
        let mut v = Voice::default();
        v.start(0, 48, 100, 2, None, 1, s, 0.);
        for _ in 0..4000 {
            v.sample(8000., &s, Drum::default());
        }
        let mut retrigger = v;
        v.retarget(48, 100, 2, s, 0., 8000, true);
        retrigger.retarget(48, 100, 2, s, 0., 8000, false);
        let mut a = 0.;
        let mut b = 0.;
        for _ in 0..80 {
            a += v.sample(8000., &s, Drum::default()).abs();
            b += retrigger.sample(8000., &s, Drum::default()).abs();
        }
        assert!(a > b * 10., "legato {a}, mono {b}");
    }
}
#[cfg(test)]
mod long_glide_tests {
    use super::*;
    #[test]
    fn two_second_semitone_glide_has_no_rounding_jump_at_the_endpoint() {
        let s = Synth::default();
        let mut v = Voice::default();
        v.start(0, 60, 100, 2, None, 1, s, 0.);
        v.retarget(61, 100, 2, s, 2000., 44100, true);
        for _ in 0..88199 {
            v.sample(44100., &s, Drum::default());
        }
        assert!((v.target_frequency - v.frequency).abs() < 0.001);
        v.sample(44100., &s, Drum::default());
        assert_eq!(v.frequency, v.target_frequency);
    }
}
