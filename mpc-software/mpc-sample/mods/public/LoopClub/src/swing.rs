//! Non-destructive MIDI timing. PCM and performance automation never pass here.
use crate::model::{Event, PPQ};
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Basis {
    pub amount: f32,
    pub sixteenth: bool,
}
impl Basis {
    pub fn valid(self) -> bool {
        self.amount.is_finite() && (50.0..=75.0).contains(&self.amount)
    }
    fn pair(self) -> f64 {
        if self.sixteenth {
            PPQ as f64 / 2.
        } else {
            PPQ as f64
        }
    }
    pub fn warp(self, tick: u64, inverse: bool) -> u64 {
        let pair = self.pair();
        let cycle = (tick as f64 / pair).floor() * pair;
        let phase = tick as f64 - cycle;
        let split = pair * f64::from(self.amount) / 100.;
        let (from, to) = if inverse {
            (split, pair * 0.5)
        } else {
            (pair * 0.5, split)
        };
        let result = if phase <= from {
            phase * to / from
        } else {
            to + (phase - from) * (pair - to) / (pair - from)
        };
        (cycle + result).round() as u64
    }
    pub fn event(self, mut event: Event, length: u64) -> Event {
        if event.deck.is_some() {
            return event;
        }
        let tick = event
            .timing
            .map_or(event.tick, |b| b.warp(event.tick, true));
        event.tick = self.warp(tick, false).min(length.saturating_sub(1));
        event
    }
}
