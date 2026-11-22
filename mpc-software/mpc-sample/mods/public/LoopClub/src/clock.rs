use crate::model::PPQ;
#[derive(Clone, Copy, Debug)]
pub struct Clock {
    pub frame: u64,
    pub rate: u32,
    pub bpm: f64,
}
impl Clock {
    pub fn new(rate: u32, bpm: f64) -> Self {
        Self {
            frame: 0,
            rate: rate.max(1),
            bpm: bpm.clamp(40., 240.),
        }
    }
    pub fn tick(&self) -> u64 {
        self.tick_at(self.frame)
    }
    pub fn tick_at(&self, frame: u64) -> u64 {
        (frame as f64 * self.bpm / (60. * f64::from(self.rate)) * PPQ as f64).round() as u64
    }
    pub fn frame_at(&self, tick: u64) -> u64 {
        (tick as f64 / PPQ as f64 * 60. * f64::from(self.rate) / self.bpm).round() as u64
    }
    pub fn bar_at_or_after(&self, tick: u64) -> u64 {
        tick.div_ceil(4 * PPQ) * (4 * PPQ)
    }
    pub fn next_bar(&self) -> u64 {
        (self.tick() / (4 * PPQ) + 1) * 4 * PPQ
    }
}
