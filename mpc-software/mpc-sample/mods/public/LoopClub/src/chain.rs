//! Fixed-size loop arrangement. Configuration edits take effect at the next step.
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Step {
    pub track: u8,
    pub repeats: u8,
}
impl Default for Step {
    fn default() -> Self {
        Self {
            track: 0,
            repeats: 1,
        }
    }
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Config {
    pub steps: [Step; 16],
    pub len: u8,
    pub repeat: bool,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            steps: [Step::default(); 16],
            len: 0,
            repeat: true,
        }
    }
}
impl Config {
    pub fn valid(&self) -> bool {
        self.len <= 16
            && self.steps[..usize::from(self.len)]
                .iter()
                .all(|s| s.track < 16 && (1..=16).contains(&s.repeats))
    }
    pub fn members(&self) -> u16 {
        self.steps[..usize::from(self.len.min(16))]
            .iter()
            .fold(0, |mask, s| mask | 1u16 << s.track.min(15))
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Change {
    Activate {
        track: usize,
        members: u16,
        origin: u64,
    },
    Stop {
        members: u16,
    },
}
#[derive(Default)]
pub struct Chain {
    pub config: Config,
    pub running: bool,
    pub index: usize,
    pub remaining: u8,
    pub boundary: u64,
    pub duration: u64,
    pub members: u16,
    pub current: Option<usize>,
    removed_current: bool,
}
impl Chain {
    pub fn start(&mut self, start: u64) {
        self.running = true;
        self.index = 0;
        self.remaining = 0;
        self.boundary = start;
        self.duration = 0;
        self.members = 0;
        self.current = None;
        self.removed_current = false;
    }
    pub fn stop(&mut self) -> Change {
        let members = self.members;
        self.running = false;
        self.remaining = 0;
        self.current = None;
        self.members = 0;
        Change::Stop { members }
    }
    pub fn remove(&mut self, index: usize) -> bool {
        let len = usize::from(self.config.len);
        if index >= len {
            return false;
        }
        if self.running {
            if index < self.index {
                self.index -= 1;
            } else if index == self.index {
                self.removed_current = true;
            }
        }
        self.config.steps.copy_within(index + 1..len, index);
        self.config.steps[len - 1] = Step::default();
        self.config.len -= 1;
        true
    }
    pub fn tick(&mut self, now: u64, length: impl Fn(usize) -> u64) -> Option<Change> {
        if !self.running || now < self.boundary {
            return None;
        }
        if self.current.is_some() && self.remaining > 1 {
            self.remaining -= 1;
            self.boundary = self.boundary.saturating_add(self.duration);
            return None;
        }
        let mut next = if self.current.is_none() {
            0
        } else if self.removed_current {
            self.index
        } else {
            self.index + 1
        };
        let len = usize::from(self.config.len);
        if len == 0 {
            return Some(self.stop());
        }
        if next >= len {
            if self.config.repeat {
                next = 0;
            } else {
                return Some(self.stop());
            }
        }
        let step = self.config.steps[next];
        let duration = length(usize::from(step.track));
        if duration == 0 {
            return Some(self.stop());
        }
        let origin = self.boundary;
        let members = self.members | self.config.members();
        self.members = self.config.members();
        self.index = next;
        self.current = Some(usize::from(step.track));
        self.remaining = step.repeats;
        self.duration = duration;
        self.boundary = origin.saturating_add(duration);
        self.removed_current = false;
        Some(Change::Activate {
            track: usize::from(step.track),
            members,
            origin,
        })
    }
}
