//! Two independently mixed oscillators, resonant TPT SVF and a complete amp ADSR.
use crate::model::Synth;
#[derive(Clone, Copy, Default)]
pub struct Envelope {
    stage: u8,
    elapsed: f32,
    value: f32,
    release_start: f32,
    rate: f32,
    timestep: f32,
    sustain_smooth: f32,
}
impl Envelope {
    pub fn sample<S: std::borrow::Borrow<Synth>>(
        &mut self,
        rate: f32,
        settings: S,
        gate: bool,
    ) -> f32 {
        let s = settings.borrow();
        self.sample_values(rate, [s.attack, s.decay, s.sustain, s.release], gate)
    }
    fn sample_values(&mut self, rate: f32, values: [f32; 4], gate: bool) -> f32 {
        let [attack, decay, sustain, release] = values;
        if gate && self.stage == 2 && self.value == sustain {
            return self.value;
        }
        if self.rate != rate {
            self.rate = rate;
            self.timestep = 1. / rate;
            self.sustain_smooth = (1. / (0.005 * rate)).min(1.);
        }
        if !gate && self.stage < 3 {
            self.stage = 3;
            self.elapsed = 0.;
            self.release_start = self.value;
        }
        self.elapsed += self.timestep;
        match self.stage {
            0 => {
                self.value = (self.elapsed / attack).min(1.);
                if self.elapsed >= attack {
                    self.stage = 1;
                    self.elapsed = 0.;
                }
            }
            1 => {
                self.value = 1. - (1. - sustain) * (self.elapsed / decay).min(1.);
                if self.elapsed >= decay {
                    self.stage = 2;
                    self.elapsed = 0.;
                }
            }
            2 => self.value += (sustain - self.value) * self.sustain_smooth,
            3 => {
                self.value = self.release_start * (1. - self.elapsed / release).max(0.);
                if self.elapsed >= release {
                    self.stage = 4;
                    self.value = 0.;
                }
            }
            _ => self.value = 0.,
        }
        self.value
    }
    pub fn done(&self) -> bool {
        self.stage == 4
    }
}
/// Phase is already bounded to [0,1): range reduction needs no libm call.
/// An odd degree11 polynomial stays within1e-6FS of f32 sine over a cycle.
#[inline]
pub fn phase_sine(phase: f32) -> f32 {
    let turn = if phase < 0.25 {
        phase
    } else if phase < 0.75 {
        0.5 - phase
    } else {
        phase - 1.
    };
    let x = turn * std::f32::consts::TAU;
    let x2 = x * x;
    let mut p = -1.0f32 / 39916800.;
    p = p.mul_add(x2, 1. / 362880.);
    p = p.mul_add(x2, -1. / 5040.);
    p = p.mul_add(x2, 1. / 120.);
    p = p.mul_add(x2, -1. / 6.);
    x * p.mul_add(x2, 1.)
}
pub const LFO_SHAPES: [&str; 6] = [
    "SINE",
    "TRIANGLE",
    "SAW UP",
    "SAW DOWN",
    "SQUARE",
    "RANDOM HOLD",
];
#[derive(Clone, Copy)]
pub struct Lfo {
    phase: f64,
    noise: u32,
    held: f32,
    rate: f32,
    hz: f32,
    step: f64,
}
impl Default for Lfo {
    fn default() -> Self {
        Self {
            phase: 0.,
            noise: 0x8d34ac17,
            held: 0.,
            rate: 0.,
            hz: f32::NAN,
            step: 0.,
        }
    }
}
impl Lfo {
    pub fn sample(&mut self, rate: f32, hz: f32, shape: u8) -> f32 {
        let phase = self.phase as f32;
        let value = match shape {
            0 => phase_sine(phase),
            1 => 1. - 4. * ((phase + 0.25).fract() - 0.5).abs(),
            2 => phase * 2. - 1.,
            3 => 1. - phase * 2.,
            4 => {
                if phase < 0.5 {
                    1.
                } else {
                    -1.
                }
            }
            _ => self.held,
        };
        if self.rate != rate || self.hz != hz {
            self.rate = rate;
            self.hz = hz;
            self.step = f64::from(hz) / f64::from(rate);
        }
        self.phase += self.step;
        if self.phase >= 1. {
            self.phase = self.phase.fract();
            self.noise ^= self.noise << 13;
            self.noise ^= self.noise >> 17;
            self.noise ^= self.noise << 5;
            self.held = self.noise as i32 as f32 / i32::MAX as f32;
        }
        value
    }
}
#[derive(Clone, Copy)]
pub struct SynthVoice {
    lfo: Lfo,
    pitch_ratio: f32,
    target_pitch_ratio: f32,
    pitch_counter: u8,
    volume_mod: f32,
    envelope: Envelope,
    filter_envelope: Envelope,
    mod_counter: u8,
    mod_g: f32,
    phase: [f32; 2],
    last_pitch: [f32; 2],
    frequency: [f32; 2],
    tune_ratio: [f32; 2],
    base: f32,
    rate: f32,
    cutoff: f32,
    resonance: f32,
    target_g: f32,
    target_k: f32,
    g: f32,
    k: f32,
    smooth: f32,
    levels: [f32; 2],
    filter_mix: [f32; 3],
    ic1: f32,
    ic2: f32,
}
impl Default for SynthVoice {
    fn default() -> Self {
        Self {
            lfo: Lfo::default(),
            pitch_ratio: 1.,
            target_pitch_ratio: 1.,
            pitch_counter: 0,
            volume_mod: 1.,
            envelope: Envelope::default(),
            filter_envelope: Envelope::default(),
            mod_counter: 0,
            mod_g: 0.,
            phase: [0.; 2],
            last_pitch: [f32::NAN; 2],
            frequency: [0.; 2],
            tune_ratio: [1.; 2],
            base: 0.,
            rate: 0.,
            cutoff: 0.,
            resonance: -1.,
            target_g: 0.,
            target_k: 2.,
            g: 0.,
            k: 2.,
            smooth: 0.,
            levels: [0.; 2],
            filter_mix: [1., 0., 0.],
            ic1: 0.,
            ic2: 0.,
        }
    }
}
fn blep(t: f32, dt: f32) -> f32 {
    if t < dt {
        let x = t / dt;
        x + x - x * x - 1.
    } else if t > 1. - dt {
        let x = (t - 1.) / dt;
        x * x + x + x + 1.
    } else {
        0.
    }
}
fn wave(t: f32, dt: f32, kind: u8) -> f32 {
    match kind {
        0 => phase_sine(t),
        1 => 2. * t - 1. - blep(t, dt),
        2 => (if t < 0.5 { 1. } else { -1. }) + blep(t, dt) - blep((t + 0.5).fract(), dt),
        _ => 1. - 4. * (t - 0.5).abs(),
    }
}
impl SynthVoice {
    pub fn retrigger(&mut self) {
        self.envelope = Envelope::default();
        self.filter_envelope = Envelope::default();
        self.lfo = Lfo::default();
    }
    pub fn done(&self) -> bool {
        self.envelope.done()
    }
    pub fn sample<S: std::borrow::Borrow<Synth>>(
        &mut self,
        rate: f32,
        base: f32,
        settings: S,
        gate: bool,
    ) -> f32 {
        let s = settings.borrow();
        let fresh = self.rate == 0.;
        if self.rate != rate || self.cutoff != s.cutoff || self.resonance != s.resonance {
            self.target_g = (std::f32::consts::PI * s.cutoff.min(rate * 0.45) / rate).tan();
            self.target_k = 2. - 1.9 * s.resonance;
            self.smooth = 1. - (-1. / (0.005 * rate)).exp();
            self.rate = rate;
            self.cutoff = s.cutoff;
            self.resonance = s.resonance;
        }
        if fresh {
            self.g = self.target_g;
            self.k = self.target_k;
            self.levels = [s.level1, s.level2];
            self.filter_mix = [0.; 3];
            self.filter_mix[usize::from(s.filter)] = 1.;
        }
        let lfo = self.lfo.sample(rate, s.lfo_rate, s.lfo_shape);
        if self.pitch_counter == 0 || s.lfo_pitch == 0. {
            self.target_pitch_ratio = if s.lfo_pitch == 0. {
                1.
            } else {
                2_f32.powf(lfo * s.lfo_pitch / 1200.)
            };
            self.pitch_counter = 15;
        } else {
            self.pitch_counter -= 1;
        }
        self.pitch_ratio += (self.target_pitch_ratio - self.pitch_ratio) * self.smooth;
        let volume_target = 1. - s.lfo_volume * (lfo + 1.) * 0.5;
        self.volume_mod += (volume_target - self.volume_mod) * self.smooth;
        let filter_env = self.filter_envelope.sample_values(
            rate,
            [
                s.filter_attack,
                s.filter_decay,
                s.filter_sustain,
                s.filter_release,
            ],
            gate,
        );
        if self.mod_counter == 0 || (s.env_amount == 0. && s.lfo_filter == 0.) {
            self.mod_g = if s.env_amount == 0. && s.lfo_filter == 0. {
                self.target_g
            } else {
                let hz = (s.cutoff
                    * 2_f32.powf(s.env_amount * filter_env * 4. + lfo * s.lfo_filter))
                .clamp(20., rate * 0.45);
                (std::f32::consts::PI * hz / rate).tan()
            };
            self.mod_counter = 15;
        } else {
            self.mod_counter -= 1;
        }
        self.g += (self.mod_g - self.g) * self.smooth;
        self.k += (self.target_k - self.k) * self.smooth;
        let pitches = [s.tune1 + s.fine1 * 0.01, s.tune2 + s.fine2 * 0.01];
        for (i, pitch) in pitches.iter().enumerate() {
            if self.last_pitch[i] != *pitch {
                self.tune_ratio[i] = 2_f32.powf(*pitch / 12.);
                self.last_pitch[i] = *pitch;
            }
            self.frequency[i] = base * self.tune_ratio[i];
        }
        self.base = base;
        self.levels[0] += (s.level1 - self.levels[0]) * self.smooth;
        self.levels[1] += (s.level2 - self.levels[1]) * self.smooth;
        let dt = [
            (self.frequency[0] * self.pitch_ratio / rate).clamp(0.000001, 0.45),
            (self.frequency[1] * (1. + s.detune) * self.pitch_ratio / rate).clamp(0.000001, 0.45),
        ];
        for (i, step) in dt.iter().enumerate() {
            self.phase[i] += step;
            if self.phase[i] >= 1. {
                self.phase[i] -= 1.;
            }
        }
        let raw = (wave(self.phase[0], dt[0], s.wave) * self.levels[0]
            + wave(self.phase[1], dt[1], s.wave2) * self.levels[1])
            / (self.levels[0] + self.levels[1]).max(1.);
        let a1 = 1. / (1. + self.g * (self.g + self.k));
        let v1 = (self.ic1 + self.g * (raw - self.ic2)) * a1;
        let v2 = self.ic2 + self.g * v1;
        self.ic1 = 2. * v1 - self.ic1;
        self.ic2 = 2. * v2 - self.ic2;
        let outputs = [v2, v1, raw - self.k * v1 - v2];
        let mut filtered = 0.;
        for (i, &v) in outputs.iter().enumerate() {
            self.filter_mix[i] +=
                ((usize::from(s.filter) == i) as u8 as f32 - self.filter_mix[i]) * self.smooth;
            filtered += v * self.filter_mix[i];
        }
        filtered * self.envelope.sample(rate, s, gate) * self.volume_mod
    }
}
