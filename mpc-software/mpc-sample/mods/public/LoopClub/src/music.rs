//! Original Loop Club harmony plus an allocation-free port of Groove Dice rules.
//! Rhythm motifs/accents adapted from GrooveDice/OneBarLoop.inc and
//! VelocityPattern.h (kim, MIT; groove-dice/v1.11.0). No sound banks/MIDI copied.
use crate::model::*;
use serde::{Deserialize, Serialize};
pub const GRID: u32 = 960;
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct PatternConfig {
    pub rules: [u8; 4],
    pub rhythm: u8,
    pub swing: f32,
    pub nudge_ms: f32,
    pub snare_late_ms: f32,
    pub seed: u32,
}
impl Default for PatternConfig {
    fn default() -> Self {
        Self {
            rules: [2, 2, 2, 1],
            rhythm: 3,
            swing: 56.,
            nudge_ms: 3.,
            snare_late_ms: 8.,
            seed: 0x91b7d13,
        }
    }
}
impl PatternConfig {
    pub fn valid(&self) -> bool {
        self.rules.iter().all(|r| (1..=16).contains(r))
            && self.rhythm < 24
            && self.swing.is_finite()
            && (50.0..=75.0).contains(&self.swing)
            && self.nudge_ms.is_finite()
            && (0.0..=12.0).contains(&self.nudge_ms)
            && self.snare_late_ms.is_finite()
            && (0.0..=25.0).contains(&self.snare_late_ms)
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct Hit {
    pub tick: u32,
    pub sound: u8,
    pub velocity: u8,
    pub gate: u32,
}
pub struct Pattern {
    pub hits: [Hit; 128],
    pub len: usize,
}
impl Default for Pattern {
    fn default() -> Self {
        Self {
            hits: [Hit::default(); 128],
            len: 0,
        }
    }
}
impl Pattern {
    fn put(&mut self, tick: u32, sound: u8, velocity: u8, gate: u32) {
        if tick >= 4 * GRID
            || self.len >= 128
            || self.hits[..self.len]
                .iter()
                .any(|h| h.tick == tick && h.sound == sound)
        {
            return;
        }
        self.hits[self.len] = Hit {
            tick,
            sound,
            velocity: velocity.clamp(1, 127),
            gate,
        };
        self.len += 1;
    }
}
fn draw(seed: &mut u32) -> u32 {
    let mut x = *seed;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    *seed = if x == 0 { 0x91357 } else { x };
    *seed
}
pub fn hat_accent(tick: u32, strong: bool) -> u8 {
    let phase = tick % GRID;
    let v = if strong {
        [100, 35, 66, 30]
    } else {
        [90, 45, 68, 40]
    }[(phase / 240) as usize];
    if !phase.is_multiple_of(240) {
        v.min(38)
    } else {
        v
    }
}
fn kick_accent(t: u32) -> u8 {
    match t % GRID {
        0 => {
            if t < GRID {
                108
            } else {
                100
            }
        }
        480 => 84,
        _ => 64,
    }
}
/// Accents follow the chosen division, so eighth-note hats include the soft strokes too.
fn subdivision_accent(cell: u32, step: u32, strong: bool) -> u8 {
    let cycle: &[u8] = match (step, strong) {
        (160, true) => &[100, 32, 62, 30, 70, 34],
        (160, false) => &[90, 40, 62, 36, 68, 38],
        (320 | 640, true) => &[100, 34, 64],
        (320 | 640, false) => &[90, 42, 66],
        (_, true) => &[100, 35, 66, 30],
        _ => &[90, 45, 68, 40],
    };
    cycle[cell as usize % cycle.len()]
}

fn perc_accent(t: u32) -> u8 {
    [60, 42, 54, 40][((t % 960) / 240) as usize]
}
pub fn generate(config: PatternConfig, bpm: f64) -> Pattern {
    let mut p = Pattern::default();
    if !config.valid() || !bpm.is_finite() || !(40.0..=240.0).contains(&bpm) {
        return p;
    }
    let mut rng = config.seed;
    let motifs: [[i32; 6]; 17] = [
        [0, 6, 10, -1, -1, -1],
        [0, 7, 10, 14, -1, -1],
        [0, 3, 8, 11, -1, -1],
        [0, -1, -1, -1, -1, -1],
        [0, 6, 10, 14, -1, -1],
        [0, 3, 7, 10, 15, -1],
        [0, 2, 7, 10, 14, -1],
        [0, 8, -1, -1, -1, -1],
        [0, 7, 15, -1, -1, -1],
        [0, 10, 15, -1, -1, -1],
        [0, 3, 6, -1, -1, -1],
        [0, 6, 14, -1, -1, -1],
        [0, 6, 8, 14, -1, -1],
        [0, 3, 10, -1, -1, -1],
        [0, 2, 9, 14, 15, -1],
        [0, 6, 14, 15, -1, -1],
        [0, 3, 6, 10, 15, -1],
    ];
    let kick = config.rules[0];
    let base = [
        [0, 6, 10, -1, -1, -1],
        [0, 7, 10, 14, -1, -1],
        [0, 3, 8, 11, -1, -1],
        [0, 6, 9, 14, -1, -1],
        [0, 7, 10, -1, -1, -1],
        [0, 3, 6, 10, 15, -1],
        [0, 2, 7, 10, -1, -1],
        [0, 3, 10, 14, -1, -1],
    ];
    let motif = if kick <= 3 {
        base[(draw(&mut rng) % 8) as usize]
    } else {
        motifs[usize::from(kick - 1)]
    };
    for step in motif {
        if step >= 0 && step != 4 && step != 12 {
            let tick = step as u32 * 240;
            p.put(tick, 0, kick_accent(tick), 80)
        }
    }
    let snare = config.rules[1];
    let (a, b) = match snare {
        4 => (68, 68),
        8 => (78, 110),
        9 => (110, 78),
        10 => (68, 78),
        _ => (98, 104),
    };
    p.put(960, 1, a, 80);
    p.put(2880, 1, b, 80);
    let ghosts: &[(u32, u8)] = match snare {
        5 => &[(720, 33)],
        6 => &[(3120, 33)],
        7 => &[(720, 30), (2640, 35)],
        11 => &[(1200, 29), (3120, 35)],
        12 => &[(480, 25), (720, 37)],
        13 => &[(3120, 28), (3360, 38)],
        14 => &[(640, 32), (3200, 36)],
        15 => &[(720, 24), (840, 39)],
        16 => &[(3240, 26), (3480, 34), (3720, 44)],
        _ => &[],
    };
    for &(tick, v) in ghosts {
        p.put(tick, 1, v, 45)
    }
    if snare == 3 && draw(&mut rng) % 100 < 35 {
        p.put(720, 1, 30, 45)
    }
    let rhythm = config.rhythm;
    let stride = [960u32, 480, 240, 720, 360, 640, 320, 160][usize::from(rhythm / 3)];
    let ornament = rhythm % 3;
    let hat = config.rules[2];
    let step = if hat == 4 { stride * 2 } else { stride };
    let offset = if hat == 11 {
        480
    } else if hat == 5 {
        stride / 2
    } else {
        0
    };
    for tick in (offset..3840).step_by(step as usize) {
        if ornament == 2 && tick >= 3360 {
            continue;
        }
        let cell = (tick - offset) / step;
        if (hat == 8 && tick >= 1920)
            || (hat == 9 && tick < 1920)
            || (hat == 10 && !tick.is_multiple_of(960))
            || (hat == 11 && tick % 960 < 480)
            || (hat == 12 && cell % 3 == 2)
            || (hat == 13 && !cell.is_multiple_of(3))
            || (hat == 16 && tick / 960 == 1)
        {
            continue;
        }
        let velocity = if hat == 14 {
            (88 - tick as i32 / 60).clamp(27, 95) as u8
        } else if hat == 15 {
            (34 + tick / 60).clamp(25, 100) as u8
        } else {
            subdivision_accent(cell, step, hat == 6)
        };
        let sound = if hat == 7 && tick >= 3840 - step {
            3
        } else {
            2
        };
        p.put(tick, sound, velocity, if sound == 3 { 160 } else { 50 });
    }
    if ornament == 2 {
        for j in 0..4 {
            let tick = 3360 + j * 120;
            p.put(tick, 2, [52, 34, 65, 44][j as usize], 35)
        }
    } else if ornament == 1 {
        let tick = if 3600 % step == offset { 3720 } else { 3600 };
        p.put(tick, 2, 42, 45)
    }
    let perc = config.rules[3];
    let fixed: &[(u32, u8, u8)] = match perc {
        4 => &[(3360, 11, 49)],
        5 => &[(480, 11, 57), (1440, 8, 52), (2400, 11, 56), (3360, 4, 48)],
        6 => &[(1440, 11, 59), (3360, 8, 54)],
        7 => &[(720, 11, 56), (1680, 4, 49), (2640, 8, 58), (3600, 11, 44)],
        8 => &[(1440, 4, 54), (3360, 4, 62)],
        9 => &[(3120, 8, 61), (3600, 8, 48)],
        10 => &[
            (480, 11, 45),
            (1440, 11, 45),
            (2400, 11, 45),
            (3360, 11, 45),
        ],
        11 => &[(720, 11, 58), (2640, 8, 52), (3360, 4, 47)],
        12 => &[(480, 8, 55), (2400, 8, 63)],
        13 => &[(1920, 4, 49), (3360, 4, 61)],
        14 => &[(0, 11, 67), (1440, 11, 51), (2880, 11, 58)],
        15 => &[(720, 8, 60), (2640, 8, 48), (3360, 4, 56)],
        16 => &[(3360, 11, 52), (3600, 8, 63), (3720, 4, 46)],
        _ => &[],
    };
    for &(tick, sound, v) in fixed {
        p.put(tick, sound, v, 80)
    }
    if perc <= 3 {
        for j in 0..perc {
            if draw(&mut rng) % 100 < 45 + u32::from(perc) * 10 {
                let tick = [720, 1680, 2640, 3360, 3600][(draw(&mut rng) % 5) as usize];
                p.put(tick, [11, 8, 4][usize::from(j)], perc_accent(tick), 80)
            }
        }
    }
    for hit in &mut p.hits[..p.len] {
        let velocity_delta = (draw(&mut rng) % 9) as i16 - 4;
        hit.velocity = (i16::from(hit.velocity) + velocity_delta).clamp(1, 127) as u8;
        let jitter = (draw(&mut rng) % 2001) as f64 / 1000. - 1.;
        let mut ms = jitter * f64::from(config.nudge_ms);
        if hit.sound == 1 {
            ms += f64::from(config.snare_late_ms)
        }
        let mut tick = f64::from(hit.tick) + ms * bpm / 60000. * 960.;
        if (hit.sound == 2 || hit.sound == 3) && rhythm / 3 < 3 {
            let pair = if stride >= 480 { 960 } else { 480 };
            let phase = f64::from(hit.tick % pair) / f64::from(pair);
            let triangle = if phase <= 0.5 {
                phase * 2.
            } else {
                (1. - phase) * 2.
            };
            tick += f64::from(config.swing - 50.) / 100. * f64::from(pair) * triangle;
        }
        hit.tick = tick.round().clamp(0., 3839.) as u32;
    }
    p.hits[..p.len].sort_unstable_by_key(|h| (h.tick, h.sound));
    p
}
pub fn event_tick(tick: u32) -> u64 {
    u64::from(tick) * PPQ / 960
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ChordConfig {
    pub strum_ms: f32,
    pub strum_direction: u8,
    pub smooth: bool,
    pub key: u8,
    pub scale: u8,
    pub color: u8,
    pub octave: i8,
    pub open: bool,
}
impl Default for ChordConfig {
    fn default() -> Self {
        Self {
            strum_ms: 0.,
            strum_direction: 0,
            smooth: true,
            key: 0,
            scale: 0,
            color: 1,
            octave: 0,
            open: false,
        }
    }
}
impl ChordConfig {
    pub fn valid(&self) -> bool {
        self.strum_ms.is_finite()
            && (0.0..=80.).contains(&self.strum_ms)
            && self.strum_direction < 3
            && self.key < 12
            && self.scale < 3
            && self.color < 3
            && (-2..=2).contains(&self.octave)
    }
    pub fn notes(&self, pad: usize) -> ([u8; 5], usize) {
        let mut notes = [0; 5];
        if pad >= 16 || !self.valid() {
            return (notes, 0);
        }
        let scale = [
            [0, 2, 4, 5, 7, 9, 11],
            [0, 2, 3, 5, 7, 8, 10],
            [0, 2, 3, 5, 7, 9, 10],
        ][usize::from(self.scale)];
        let degree = pad % 7;
        let base = 48 + i32::from(self.key) + i32::from(self.octave) * 12 + (pad / 7) as i32 * 12;
        let count = usize::from(self.color) + 3;
        for (i, note) in notes[..count].iter_mut().enumerate() {
            let d = degree + i * 2;
            let mut n = base + scale[d % 7] + (d / 7) as i32 * 12;
            if self.open && i == 1 {
                n -= 12
            }
            *note = n.clamp(0, 127) as u8;
        }
        notes[..count].sort_unstable();
        (notes, count)
    }
    pub fn label(&self, pad: usize) -> String {
        let (notes, len) = self.notes(pad);
        if len == 0 {
            return "--".into();
        }
        let scale = [
            [0u8, 2, 4, 5, 7, 9, 11],
            [0, 2, 3, 5, 7, 8, 10],
            [0, 2, 3, 5, 7, 9, 10],
        ][usize::from(self.scale)];
        let root = (self.key + scale[pad % 7]) % 12;
        let has = |pc: u8| notes[..len].iter().any(|n| (*n + 12 - root) % 12 == pc);
        let suffix = if has(6) {
            if self.color == 0 { "dim" } else { "m7b5" }
        } else if has(3) {
            if self.color == 0 {
                "m"
            } else if self.color == 2 {
                if has(1) { "m7b9" } else { "m9" }
            } else {
                "m7"
            }
        } else if self.color == 0 {
            ""
        } else if has(11) {
            if self.color == 2 { "maj9" } else { "maj7" }
        } else if self.color == 2 {
            "9"
        } else {
            "7"
        };
        format!(
            "{}{}",
            [
                "C", "C#", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B"
            ][usize::from(root)],
            suffix
        )
    }
}

/// Generate one playable kit lane; other sounds are never returned or changed.
pub fn generate_sound(config: PatternConfig, bpm: f64, sound: u8) -> Pattern {
    let mut result = Pattern::default();
    if sound >= 16 {
        return result;
    }
    let source = generate(config, bpm);
    if source.len == 0 {
        return result;
    }
    if sound == 3 {
        result.put(3360, 3, 70 + (config.seed % 9) as u8, 220);
        if config.seed & 1 != 0 {
            result.put(1440, 3, 62, 180)
        }
    } else if sound == 13 {
        result.put(0, 13, 86 + (config.seed % 9) as u8, 600)
    } else {
        for hit in &source.hits[..source.len] {
            let take = match sound {
                0 => hit.sound == 0,
                1 | 4 | 5 => hit.sound == 1,
                2 | 12 | 14 => hit.sound == 2,
                _ => matches!(hit.sound, 4 | 8 | 11),
            };
            if take {
                let velocity = if matches!(sound, 4 | 5 | 12 | 14) {
                    (f32::from(hit.velocity) * 0.75).round() as u8
                } else {
                    hit.velocity
                };
                result.put(hit.tick, sound, velocity, hit.gate)
            }
        }
        if result.len == 0 {
            result.put(
                [720, 1680, 2640, 3360][(config.seed % 4) as usize],
                sound,
                55 + (config.seed % 14) as u8,
                90,
            )
        }
    }
    result.hits[..result.len].sort_unstable_by_key(|h| h.tick);
    result
}

/// Draw a coherent profile for the requested lane, rather than random hits.
pub fn next_config(mut config: PatternConfig, sound: usize) -> PatternConfig {
    let seed = draw(&mut config.seed);
    let group = match sound {
        0 => 0,
        1 | 4 | 5 => 1,
        2 | 3 | 12 | 14 => 2,
        _ => 3,
    };
    let choices: &[u8] = match group {
        0 => &[1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        1 => &[2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        2 => &[1, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        _ => &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
    };
    config.rules[group] = choices[seed as usize % choices.len()];
    if matches!(sound, 2 | 3) {
        let rhythms = [3, 3, 3, 4, 6, 6, 7, 8, 9, 12, 18, 19, 21];
        config.rhythm = rhythms[(seed.rotate_left(11) as usize) % rhythms.len()];
    } else if matches!(sound, 12 | 14) {
        config.rhythm = if seed & 1 == 0 { 0 } else { 3 };
        config.rules[2] = 2;
    }
    config
}

/// Search inversions and octave placement without allocating or changing pitch classes.
pub fn smooth_voicing(
    notes: [u8; 5],
    len: usize,
    previous: [u8; 5],
    previous_len: usize,
) -> [u8; 5] {
    if len == 0 || len > 5 || previous_len == 0 || previous_len > 5 {
        return notes;
    }
    let mut best = notes;
    let mut score = i32::MAX;
    for inversion in 0..len {
        for shift in -3..=3 {
            let mut candidate = [0u8; 5];
            let mut valid = true;
            for i in 0..len {
                let n = i32::from(notes[(i + inversion) % len])
                    + if i + inversion >= len { 12 } else { 0 }
                    + shift * 12;
                if !(0..=127).contains(&n) {
                    valid = false;
                    break;
                }
                candidate[i] = n as u8;
            }
            if !valid {
                continue;
            }
            candidate[..len].sort_unstable();
            let movement = (0..len)
                .map(|i| {
                    (i32::from(candidate[i]) - i32::from(previous[i * previous_len / len])).abs()
                })
                .sum::<i32>();
            let top = (i32::from(candidate[len - 1]) - i32::from(previous[previous_len - 1])).abs();
            let cost = movement * 4 + top * 3;
            if cost < score {
                score = cost;
                best = candidate;
            }
        }
    }
    best
}

/// Generator swing is baked only into straight hats. Store its basis once so
/// global Swing replaces it instead of adding another delay.
pub fn swing_basis(config: PatternConfig, sound: u8) -> Option<crate::swing::Basis> {
    if config.swing == 50. || !matches!(sound, 2 | 12 | 14) || config.rhythm / 3 >= 3 {
        return None;
    }
    Some(crate::swing::Basis {
        amount: config.swing,
        sixteenth: config.rhythm / 3 >= 2,
    })
}
