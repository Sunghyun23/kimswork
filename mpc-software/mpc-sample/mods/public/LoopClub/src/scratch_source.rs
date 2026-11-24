//! Original procedural scratch practice source. No sampled recording or speech asset.
//! Prepare only on the file worker; ImportJob swaps finished PCM at an audio boundary.
use crate::{model::*, sample::ImportJob};
struct Formant {
    a: f32,
    r2: f32,
    gain: f32,
    y1: f32,
    y2: f32,
}
impl Formant {
    fn new(rate: u32, frequency: f32, bandwidth: f32) -> Self {
        let r = (-std::f32::consts::PI * bandwidth / rate as f32).exp();
        Self {
            a: 2. * r * (std::f32::consts::TAU * frequency / rate as f32).cos(),
            r2: r * r,
            gain: 1. - r,
            y1: 0.,
            y2: 0.,
        }
    }
    fn sample(&mut self, x: f32) -> f32 {
        let y = self.gain * x + self.a * self.y1 - self.r2 * self.y2;
        self.y2 = self.y1;
        self.y1 = y;
        y
    }
}
fn envelope(t: f32, start: f32, end: f32) -> f32 {
    if t < start || t >= end {
        0.
    } else {
        ((t - start) / 0.012).min(1.) * ((end - t) / 0.035).min(1.)
    }
}
pub fn prepare(
    track: usize,
    rate: u32,
    bpm: f64,
    capacity: usize,
) -> Result<Box<ImportJob>, Error> {
    if track >= 16
        || !(8000..=96000).contains(&rate)
        || !bpm.is_finite()
        || !(40. ..=240.).contains(&bpm)
    {
        return Err(Error::InvalidValue);
    }
    let source_frames = (rate as f64 * 1.8).round() as usize;
    let bar = 60. / bpm * 4. * f64::from(rate);
    let bars = (source_frames as f64 / bar).ceil().max(1.) as u64;
    let frames = (bars as f64 * bar).round() as usize;
    if frames > capacity {
        return Err(Error::Capacity);
    }
    let mut audio = vec![Frame::ZERO; capacity];
    let mut filters = [
        Formant::new(rate, 740., 150.),
        Formant::new(rate, 1220., 190.),
        Formant::new(rate, 2550., 320.),
    ];
    let mut seed = 0x6c635243u32;
    let mut phase = 0f32;
    let mut peak = 0f32;
    for (i, frame) in audio.iter_mut().take(source_frames).enumerate() {
        let t = i as f32 / rate as f32;
        seed ^= seed << 13;
        seed ^= seed >> 17;
        seed ^= seed << 5;
        let noise = (seed as f64 / u32::MAX as f64 * 2. - 1.) as f32;
        let f0 = 112. - 10. * t + 1.2 * (t * 19.).sin();
        phase = (phase + f0 / rate as f32).fract();
        let a = phase * std::f32::consts::TAU;
        let buzz = a.sin() + 0.5 * (2. * a).sin() + 0.3 * (3. * a).sin() + 0.15 * (5. * a).sin();
        let excitation = buzz + noise * 0.5;
        let vowel = filters[0].sample(excitation)
            + 0.55 * filters[1].sample(excitation)
            + 0.3 * filters[2].sample(excitation);
        let ahh = envelope(t, 0., 0.96) * (vowel + noise * 0.018);
        let stab = envelope(t, 1.12, 1.77)
            * (vowel * 0.65
                + noise
                    * if !(1.23..=1.59).contains(&t) {
                        0.24
                    } else {
                        0.025
                    });
        let x = ahh + stab;
        peak = peak.max(x.abs());
        *frame = Frame::mono(x);
    }
    if peak > 0. {
        for frame in &mut audio[..source_frames] {
            *frame = frame.gain(0.6 / peak);
        }
    }
    let mut features = Box::new(crate::features::Features::default());
    for frame in &audio[..source_frames] {
        features.feed(*frame, rate);
    }
    Ok(Box::new(ImportJob {
        features,
        track,
        rate,
        bpm,
        audio,
        frames,
        source_frames,
        ticks: bars * 4 * PPQ,
        name: "Original AHH".into(),
        error: None,
    }))
}
