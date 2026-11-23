//! Bounded tap-tempo estimator; timestamps are captured before the UI queue.
#[derive(Default)]
pub struct TapTempo {
    last: Option<u64>,
    intervals: [u64; 4],
    len: usize,
    next: usize,
}
impl TapTempo {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
    pub fn tap(&mut self, milliseconds: u64) -> Option<f64> {
        let Some(previous) = self.last else {
            self.last = Some(milliseconds);
            return None;
        };
        let Some(interval) = milliseconds.checked_sub(previous) else {
            self.reset();
            self.last = Some(milliseconds);
            return None;
        };
        if interval < 120 {
            return None;
        } // contact bounce is not a new tap
        self.last = Some(milliseconds);
        if !(200..=1800).contains(&interval) {
            self.len = 0;
            self.next = 0;
            return None;
        }
        self.intervals[self.next] = interval;
        self.next = (self.next + 1) % self.intervals.len();
        self.len = (self.len + 1).min(self.intervals.len());
        let sum: u64 = self.intervals[..self.len].iter().sum();
        Some(
            (60000. * self.len as f64 / sum as f64)
                .round()
                .clamp(40., 240.),
        )
    }
}
