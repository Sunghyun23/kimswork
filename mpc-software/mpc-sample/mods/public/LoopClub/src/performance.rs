//! Bounded note ownership and persisted performance settings. No realtime allocation.
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// 0 Poly, 1 Mono retrigger, 2 Mono legato; chords retain polyphony.
    pub voice: u8,
    pub octave: i8,
    pub glide_ms: f32,
    /// 0 original/native program behavior, 1 Gate, 2 One Shot.
    pub trigger: u8,
    /// Synth automatic gate duration; samples use their actual endpoint.
    pub shot_ms: f32,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            voice: 0,
            octave: 0,
            glide_ms: 0.,
            trigger: 0,
            shot_ms: 500.,
        }
    }
}
impl Settings {
    pub fn valid(self) -> bool {
        (-2..=2).contains(&self.octave)
            && self.voice < 3
            && self.trigger < 3
            && self.glide_ms.is_finite()
            && (0.0..=2000.).contains(&self.glide_ms)
            && self.shot_ms.is_finite()
            && (10.0..=8000.).contains(&self.shot_ms)
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct Held {
    pub note: u8,
    pub source: u16,
    pub velocity: u8,
    pub age: u64,
}
#[derive(Clone, Copy)]
pub struct Mono {
    pub notes: [Held; 128],
}
impl Default for Mono {
    fn default() -> Self {
        Self {
            notes: [Held::default(); 128],
        }
    }
}
impl Mono {
    pub fn latest(&self) -> Option<Held> {
        self.notes
            .iter()
            .filter(|n| n.velocity > 0)
            .max_by_key(|n| n.age)
            .copied()
    }
    pub fn press(&mut self, note: u8, source: u16, velocity: u8, age: u64) {
        let at = self
            .notes
            .iter()
            .position(|n| n.velocity > 0 && n.note == note && n.source == source)
            .or_else(|| self.notes.iter().position(|n| n.velocity == 0))
            .unwrap_or_else(|| {
                self.notes
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, n)| n.age)
                    .unwrap()
                    .0
            });
        self.notes[at] = Held {
            note,
            source,
            velocity,
            age,
        };
    }
    pub fn release(&mut self, note: u8, source: u16) {
        for n in &mut self.notes {
            if n.note == note && n.source == source {
                n.velocity = 0;
            }
        }
    }
    pub fn release_source(&mut self, source: u16) {
        for n in &mut self.notes {
            if n.source == source {
                n.velocity = 0;
            }
        }
    }
}
