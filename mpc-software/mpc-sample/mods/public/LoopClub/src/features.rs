//! Streaming peak envelope and onset candidates; fixed storage, no realtime allocation.
use crate::model::Frame;
#[derive(Clone, Copy, Default)]
pub struct Peak {
    pub min: f32,
    pub max: f32,
}
#[derive(Clone, Copy, Default)]
pub struct Onset {
    pub frame: u32,
    pub strength: f32,
}
#[derive(Clone)]
pub struct Features {
    pub peaks: [Peak; 1024],
    pub bin_size: usize,
    pub frames: usize,
    pub onsets: [Onset; 128],
    pub onset_count: usize,
    fast: f32,
    slow: f32,
    ready: bool,
    last: usize,
    rate: u32,
    fast_a: f32,
    slow_a: f32,
    active_onset: Option<usize>,
}
impl Default for Features {
    fn default() -> Self {
        Self {
            peaks: [Peak::default(); 1024],
            bin_size: 32,
            frames: 0,
            onsets: [Onset::default(); 128],
            onset_count: 0,
            fast: 0.,
            slow: 0.,
            ready: true,
            last: 0,
            rate: 0,
            fast_a: 0.,
            slow_a: 0.,
            active_onset: None,
        }
    }
}
impl Features {
    pub fn feed(&mut self, frame: Frame, rate: u32) {
        if self.rate != rate {
            self.rate = rate;
            self.fast_a = 1. - (-1. / (rate.max(1) as f32 * 0.001)).exp();
            self.slow_a = 1. - (-1. / (rate.max(1) as f32 * 0.03)).exp();
        }
        if self.frames / self.bin_size >= 1024 {
            for i in 0..512 {
                self.peaks[i] = Peak {
                    min: self.peaks[i * 2].min.min(self.peaks[i * 2 + 1].min),
                    max: self.peaks[i * 2].max.max(self.peaks[i * 2 + 1].max),
                }
            }
            self.bin_size *= 2;
        }
        let index = self.frames / self.bin_size;
        let min = frame.l.min(frame.r);
        let max = frame.l.max(frame.r);
        if self.frames.is_multiple_of(self.bin_size) {
            self.peaks[index] = Peak { min, max }
        } else {
            self.peaks[index].min = self.peaks[index].min.min(min);
            self.peaks[index].max = self.peaks[index].max.max(max);
        }
        self.fast += (frame.peak() - self.fast) * self.fast_a;
        self.slow += (self.fast - self.slow) * self.slow_a;
        let ratio = self.fast / (self.slow + 0.00005);
        if ratio < 1.3 {
            self.ready = true
        }
        if self.ready
            && ratio > 2.2
            && self.fast > 0.0001
            && self.frames.saturating_sub(self.last) > rate as usize / 80
        {
            let onset = Onset {
                frame: self.frames.saturating_sub(rate as usize / 2000) as u32,
                strength: ratio.min(32.),
            };
            self.active_onset = None;
            if self.onset_count < 128 {
                self.active_onset = Some(self.onset_count);
                self.onsets[self.onset_count] = onset;
                self.onset_count += 1;
            } else if let Some((index, old)) = self
                .onsets
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.strength.total_cmp(&b.strength))
                && onset.strength > old.strength
            {
                self.onsets[index] = onset;
                self.active_onset = Some(index);
            }
            self.last = self.frames;
            self.ready = false;
        }
        if !self.ready
            && self.frames.saturating_sub(self.last) < rate as usize / 50
            && let Some(i) = self.active_onset
        {
            self.onsets[i].strength = self.onsets[i].strength.max(ratio.min(32.));
        }
        self.frames += 1;
    }
    pub fn window(&self, start: usize, end: usize) -> Peak {
        if self.frames == 0 || start >= end || start >= self.frames {
            return Peak::default();
        }
        let first = (start / self.bin_size).min(1023);
        let last = end
            .div_ceil(self.bin_size)
            .min(self.frames.div_ceil(self.bin_size))
            .min(1024);
        let mut peak = self.peaks[first];
        for p in &self.peaks[first..last.max(first + 1)] {
            peak.min = peak.min.min(p.min);
            peak.max = peak.max.max(p.max)
        }
        peak
    }
}
