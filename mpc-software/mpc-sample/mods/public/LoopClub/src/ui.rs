//! Same layout is drawn by the native firmware painter and the host SVG renderer.
use crate::{
    engine::{Command, Engine},
    model::*,
};
pub const WORDS: usize = 1056;
#[derive(Clone, Copy)]
pub struct Snapshot {
    pub words: [u64; WORDS],
}
impl Default for Snapshot {
    fn default() -> Self {
        Self { words: [0; WORDS] }
    }
}
fn pair(a: f32, b: f32) -> u64 {
    u64::from(a.to_bits()) | (u64::from(b.to_bits()) << 32)
}
fn lo(v: u64) -> f32 {
    f32::from_bits(v as u32)
}
fn hi(v: u64) -> f32 {
    f32::from_bits((v >> 32) as u32)
}
fn kind(n: u64) -> Kind {
    match n & 255 {
        1 => Kind::Synth,
        2 => Kind::Drum,
        _ => Kind::Audio,
    }
}
fn effect(n: u64) -> Effect {
    match n {
        1 => Effect::Drive,
        2 => Effect::Filter,
        3 => Effect::Compressor,
        4 => Effect::LoFi,
        5 => Effect::Isolator,
        6 => Effect::Repeat,
        7 => Effect::Reverse,
        8 => Effect::TapeStop,
        9 => Effect::DubEcho,
        10 => Effect::Crush,
        11 => Effect::Wobble,
        _ => Effect::Off,
    }
}
impl Snapshot {
    pub fn sidechain(&self) -> crate::motion::Sidechain {
        let w = self.words[1026];
        crate::motion::Sidechain {
            source: (w & 255) as u8,
            sound: ((w >> 8) & 255) as u8,
            mode: ((w >> 16) & 255) as u8,
            mask: ((w >> 24) & 65535) as u16,
            sync: w & (1 << 40) != 0,
            enabled: w & (1 << 41) != 0,
            depth: lo(self.words[1027]),
            release: hi(self.words[1027]),
            threshold: lo(self.words[1028]),
        }
    }
    pub fn duck_reduction(&self) -> f32 {
        hi(self.words[1028])
    }
    pub fn texture(&self) -> crate::motion::Texture {
        let mut t = crate::motion::Texture {
            master_bypass: [self.words[1042] & 1 != 0, self.words[1042] & 2 != 0],
            ..crate::motion::Texture::default()
        };
        for i in 0..5 {
            t.params[i] = [
                lo(self.words[1032 + i * 2]),
                hi(self.words[1032 + i * 2]),
                lo(self.words[1033 + i * 2]),
            ];
            t.enabled[i] = hi(self.words[1033 + i * 2]) > 0.5;
        }
        t
    }

    pub fn capture(e: &Engine) -> Self {
        let mut s = Self::default();
        s.words[0] = e.clock.frame;
        s.words[1] = u64::from(e.clock.rate);
        s.words[2] = e.clock.bpm.to_bits();
        s.words[3] = e.selected as u64;
        s.words[4] = u64::from(e.running) | (u64::from(e.preroll) << 8);
        s.words[5] = pair(e.master, e.master_meter.l);
        s.words[6] = pair(e.master_meter.r, 0.);
        s.words[1024] = pair(e.usb_meter.l, e.usb_meter.r);
        s.words[1025] = pair(
            e.tracks[e.selected].synth.lfo_pan,
            e.tracks[e.selected].pan_current,
        );
        let sc = e.sidechain;
        s.words[1026] = u64::from(sc.source)
            | (u64::from(sc.sound) << 8)
            | (u64::from(sc.mode) << 16)
            | (u64::from(sc.mask) << 24)
            | (u64::from(sc.sync) << 40)
            | (u64::from(sc.enabled) << 41);
        s.words[1027] = pair(sc.depth, sc.release);
        s.words[1028] = pair(sc.threshold, e.duck.reduction());
        s.words[1042] =
            u64::from(e.texture.master_bypass[0]) | (u64::from(e.texture.master_bypass[1]) << 1);
        for i in 0..5 {
            s.words[1032 + i * 2] = pair(e.texture.params[i][0], e.texture.params[i][1]);
            s.words[1033 + i * 2] = pair(e.texture.params[i][2], f32::from(e.texture.enabled[i]));
        }
        s.words[7] = e.error.map_or(0, |e| e as u64 + 1);
        s.words[8] = e.clock.tick();
        if let Some(r) = e.recorder {
            s.words[9] = r.track as u64 + 1;
            s.words[10] = r.start;
            s.words[11] = r.end.unwrap_or(0);
            s.words[12] = u64::from(r.repeating)
                | (u64::from(r.stop_after_cycle) << 1)
                | ((r.mode as u64) << 2)
        }
        for i in 0..16 {
            let t = &e.tracks[i];
            let b = 16 + i * 12;
            s.words[b] = t.kind as u64
                | ((e.status(i) as u64) << 8)
                | (u64::from(t.mix.mute) << 16)
                | (u64::from(t.mix.solo) << 17)
                | ((t.record_mode as u64) << 18);
            s.words[b + 1] = pair(t.mix.level, t.mix.pan);
            s.words[b + 2] = pair(t.mix.delay, t.mix.room);
            s.words[b + 3] = pair(t.meter, t.gain);
            s.words[b + 4] = t.length_ticks;
            s.words[b + 5] = t.phase(e.clock.tick());
            s.words[b + 6] = e.visible_events(i).len() as u64;
            s.words[b + 9] = t.route as u64 | ((t.output as u64) << 8);
            s.words[b + 10] = u64::from(t.generated);
            s.words[b + 11] = if t.source_frames > 0 {
                t.source_frames as u64
            } else {
                t.audio_frames as u64
            };
            let mut name = [0u8; 16];
            let n = t.name.len().min(16);
            name[..n].copy_from_slice(&t.name.as_bytes()[..n]);
            s.words[b + 7] = u64::from_le_bytes(name[..8].try_into().unwrap());
            s.words[b + 8] = u64::from_le_bytes(name[8..].try_into().unwrap());
        }
        let t = &e.tracks[e.selected];
        for sound in 0..16 {
            for layer in 0..2 {
                let cfg = t.drums[sound].layers[layer];
                let b = 800 + (sound * 2 + layer) * 7;
                s.words[b] = pair(cfg.level, cfg.tune);
                s.words[b + 1] = pair(
                    cfg.shift,
                    if cfg.decay > 0. {
                        cfg.decay
                    } else {
                        t.drums[sound].decay
                    },
                );
                s.words[b + 2] = u64::from(cfg.enabled)
                    | (u64::from(cfg.reverse) << 1)
                    | (u64::from(!t.kit.clip(sound, layer).is_empty()) << 2);
                let mut bytes = [0u8; 32];
                let name = t.kit.names[sound][layer].as_bytes();
                let mut n = name.len().min(32);
                while !t.kit.names[sound][layer].is_char_boundary(n) {
                    n -= 1;
                }
                bytes[..n].copy_from_slice(&name[..n]);
                for j in 0..4 {
                    s.words[b + 3 + j] =
                        u64::from_le_bytes(bytes[j * 8..j * 8 + 8].try_into().unwrap());
                }
            }
        }
        for i in 0..16 {
            let m = t.drums[i].mix;
            let b = 208 + i * 6;
            s.words[b] = pair(m.level, m.pan);
            s.words[b + 1] = pair(m.delay, m.room);
            s.words[b + 2] = u64::from(m.mute) | (u64::from(m.solo) << 1);
            s.words[b + 3] = pair(t.drums[i].tune, t.drums[i].decay);
        }
        for (i, fx) in
            t.fx.iter()
                .chain(e.performance.iter())
                .chain(e.bus_fx.iter())
                .enumerate()
        {
            let b = 320 + i * 4;
            s.words[b] = fx.effect as u64;
            s.words[b + 1] = pair(fx.a, fx.b);
            s.words[b + 2] = pair(fx.mix, 0.)
        }
        for (i, slice) in t.chops.slices.iter().enumerate() {
            s.words[482 + i] = u64::from(slice.start) | (u64::from(slice.end) << 32);
            let b = 498 + i / 2;
            let shift = (i % 2) * 32;
            s.words[b] |= u64::from(slice.level.to_bits()) << shift;
        }
        s.words[508] = u64::from(t.pattern.rules[0])
            | (u64::from(t.pattern.rules[1]) << 8)
            | (u64::from(t.pattern.rules[2]) << 16)
            | (u64::from(t.pattern.rules[3]) << 24);
        s.words[509] = u64::from(t.pattern.rhythm);
        s.words[510] = pair(t.pattern.swing, t.pattern.nudge_ms);
        s.words[511] = u64::from(t.pattern.seed);
        s.words[506] = u64::from(t.chops.gate)
            | ((t.chops.mode as u64) << 8)
            | (u64::from(t.chops.count) << 16);
        let frames = if t.source_frames == 0 {
            t.audio_frames
        } else {
            t.source_frames
        };
        let (start, end) = if t.wave_view.0 < t.wave_view.1 && t.wave_view.1 as usize <= frames {
            t.wave_view
        } else {
            (0, frames as u32)
        };
        for i in 0..256 {
            let a = u64::from(start) + u64::from(end - start) * i as u64 / 256;
            let b = u64::from(start) + u64::from(end - start) * (i + 1) as u64 / 256;
            let peak = t
                .features
                .window(a as usize, (b as usize).max(a as usize + 1));
            s.words[512 + i] = pair(peak.min, peak.max);
        }
        s.words[768] = u64::from(start);
        s.words[769] = u64::from(end);
        s.words[770] = frames as u64;
        s.words[771] = pair(t.chops.sensitivity, t.chops.min_gap_ms);
        s.words[772] = u64::from(t.chops.regions);
        s.words[773] = u64::from(t.chops.grid);
        s.words[774] = u64::from(t.chops.pending);
        s.words[507] = u64::from(e.slice_activity(e.selected));
        s.words[481] = u64::from(t.generated_sounds);
        s.words[480] = u64::from(t.chord.key)
            | (u64::from(t.chord.scale) << 8)
            | (u64::from(t.chord.color) << 16)
            | ((t.chord.octave as u8 as u64) << 24)
            | (u64::from(t.chord.open) << 32);
        s.words[348] = u64::from(t.synth.wave) | ((t.synth.octave as u8 as u64) << 8);
        s.words[349] = pair(t.synth.cutoff, t.synth.attack);
        s.words[350] = pair(t.synth.release, t.synth.detune);
        let kit_name = t.kit.name.as_bytes();
        let mut kit_bytes = [0; 32];
        let n = kit_name.len().min(32);
        kit_bytes[..n].copy_from_slice(&kit_name[..n]);
        for i in 0..4 {
            s.words[788 + i] = u64::from_le_bytes(kit_bytes[i * 8..i * 8 + 8].try_into().unwrap());
        }
        s.words[787] = t
            .kit
            .clips
            .iter()
            .enumerate()
            .fold(0u64, |bits, (i, c)| bits | (u64::from(!c.is_empty()) << i));
        s.words[775] = u64::from(t.synth.wave2) | (u64::from(t.synth.filter) << 8);
        s.words[776] = pair(t.synth.tune1, t.synth.tune2);
        s.words[777] = pair(t.synth.level1, t.synth.level2);
        s.words[778] = pair(t.synth.resonance, t.synth.decay);
        s.words[779] = pair(t.synth.sustain, t.synth.fine1);
        s.words[780] = pair(t.synth.fine2, 0.);
        s.words[781] = pair(t.synth.filter_attack, t.synth.filter_decay);
        s.words[782] = pair(t.synth.filter_sustain, t.synth.filter_release);
        s.words[783] = pair(t.synth.env_amount, 0.);
        s.words[784] = pair(t.synth.lfo_rate, t.synth.lfo_pitch);
        s.words[785] = pair(t.synth.lfo_filter, t.synth.lfo_volume);
        s.words[786] = u64::from(t.synth.lfo_shape);
        for (i, event) in e.visible_events(e.selected).iter().take(64).enumerate() {
            let b = 352 + i * 2;
            s.words[b] = event.tick;
            s.words[b + 1] = u64::from(event.note)
                | (u64::from(event.velocity) << 8)
                | (u64::from(event.on) << 16)
        }
        s
    }
    pub fn slice(&self, i: usize) -> crate::chop::Slice {
        let v = self.words[482 + i];
        crate::chop::Slice {
            start: v as u32,
            end: (v >> 32) as u32,
            level: f32::from_bits((self.words[498 + i / 2] >> ((i % 2) * 32)) as u32),
        }
    }
    pub fn chord(&self) -> crate::music::ChordConfig {
        let v = self.words[480];
        crate::music::ChordConfig {
            key: v as u8,
            scale: (v >> 8) as u8,
            color: (v >> 16) as u8,
            octave: (v >> 24) as u8 as i8,
            open: v & (1 << 32) != 0,
        }
    }
    pub fn bpm(&self) -> f64 {
        f64::from_bits(self.words[2])
    }
    pub fn mix(&self, i: usize) -> Mix {
        let b = 16 + i * 12;
        Mix {
            level: lo(self.words[b + 1]),
            pan: hi(self.words[b + 1]),
            delay: lo(self.words[b + 2]),
            room: hi(self.words[b + 2]),
            mute: self.words[b] & (1 << 16) != 0,
            solo: self.words[b] & (1 << 17) != 0,
        }
    }
    pub fn drum(&self, i: usize) -> Mix {
        let b = 208 + i * 6;
        Mix {
            level: lo(self.words[b]),
            pan: hi(self.words[b]),
            delay: lo(self.words[b + 1]),
            room: hi(self.words[b + 1]),
            mute: self.words[b + 2] & 1 != 0,
            solo: self.words[b + 2] & 2 != 0,
        }
    }
    pub fn drum_shape(&self, i: usize) -> (f32, f32) {
        (
            lo(self.words[208 + i * 6 + 3]),
            hi(self.words[208 + i * 6 + 3]),
        )
    }
    pub fn fx(&self, base: usize) -> Fx {
        Fx {
            effect: effect(self.words[base]),
            a: lo(self.words[base + 1]),
            b: hi(self.words[base + 1]),
            mix: lo(self.words[base + 2]),
        }
    }
    pub fn track_kind(&self, i: usize) -> Kind {
        kind(self.words[16 + i * 12])
    }
    pub fn synth(&self) -> Synth {
        Synth {
            lfo_rate: lo(self.words[784]),
            lfo_pitch: hi(self.words[784]),
            lfo_filter: lo(self.words[785]),
            lfo_volume: hi(self.words[785]),
            lfo_pan: lo(self.words[1025]),
            lfo_shape: self.words[786] as u8,
            filter_attack: lo(self.words[781]),
            filter_decay: hi(self.words[781]),
            filter_sustain: lo(self.words[782]),
            filter_release: hi(self.words[782]),
            env_amount: lo(self.words[783]),
            wave2: self.words[775] as u8,
            filter: (self.words[775] >> 8) as u8,
            tune1: lo(self.words[776]),
            tune2: hi(self.words[776]),
            level1: lo(self.words[777]),
            level2: hi(self.words[777]),
            resonance: lo(self.words[778]),
            decay: hi(self.words[778]),
            sustain: lo(self.words[779]),
            fine1: hi(self.words[779]),
            fine2: lo(self.words[780]),
            wave: self.words[348] as u8,
            cutoff: lo(self.words[349]),
            attack: hi(self.words[349]),
            release: lo(self.words[350]),
            detune: hi(self.words[350]),
            octave: (self.words[348] >> 8) as u8 as i8,
        }
    }
    pub fn output(&self, track: usize) -> OutputTarget {
        match (self.words[16 + track.min(15) * 12 + 9] >> 8) & 255 {
            0 => OutputTarget::Main,
            1 => OutputTarget::Usb,
            _ => OutputTarget::Both,
        }
    }
    pub fn layer(&self, sound: usize, layer: usize) -> DrumLayer {
        let b = 800 + (sound.min(15) * 2 + layer.min(1)) * 7;
        DrumLayer {
            level: lo(self.words[b]),
            tune: hi(self.words[b]),
            shift: lo(self.words[b + 1]),
            decay: hi(self.words[b + 1]),
            enabled: self.words[b + 2] & 1 != 0,
            reverse: self.words[b + 2] & 2 != 0,
        }
    }
    pub fn layer_name(&self, sound: usize, layer: usize) -> String {
        let b = 800 + (sound.min(15) * 2 + layer.min(1)) * 7;
        let mut bytes = [0u8; 32];
        for j in 0..4 {
            bytes[j * 8..j * 8 + 8].copy_from_slice(&self.words[b + 3 + j].to_le_bytes());
        }
        let name = String::from_utf8_lossy(&bytes)
            .trim_end_matches('\0')
            .to_owned();
        if name.is_empty() {
            format!("SYNTH {}", DRUM_NAMES[sound.min(15)])
        } else {
            name
        }
    }
    pub fn layer_pcm(&self, sound: usize, layer: usize) -> bool {
        self.words[800 + (sound.min(15) * 2 + layer.min(1)) * 7 + 2] & 4 != 0
    }
    pub fn level(&self, i: usize) -> f32 {
        if i == 16 {
            lo(self.words[5])
        } else {
            self.mix(i).level
        }
    }
    pub fn name(&self, i: usize) -> String {
        let b = 16 + i * 12;
        let mut bytes = [0; 16];
        bytes[..8].copy_from_slice(&self.words[b + 7].to_le_bytes());
        bytes[8..].copy_from_slice(&self.words[b + 8].to_le_bytes());
        String::from_utf8_lossy(&bytes)
            .trim_end_matches('\0')
            .to_owned()
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Browse,
    None,
    Save,
    Load,
    Exit,
}
pub struct Ui {
    pub synth_page: u8,
    pub texture_slot: usize,
    pub layer: usize,
    pub layer_detail: bool,
    pub chop_slice: usize,
    pub chop_cursor: u32,
    pub chop_zoom: u8,
    pub held_slices: [Option<usize>; 16],
    pub chord_modes: [bool; 16],
    pub held_chords: [Option<usize>; 16],
    pub page: Page,
    pub selected: usize,
    pub sound: usize,
    pub fx_slot: usize,
    pub latch: bool,
    pub pickup: Pickup,
    pub grid_event: usize,
    pub grid_bar: u64,
    pub notice: &'static str,
    pub active: bool,
    pub held: [Option<(usize, u8, u16)>; 16],
    pub fx_held: [Option<(usize, u8, usize)>; 16],
    pub master_target: u8,
    pub shift_view: bool,
    fx_pages: [(usize, bool); 2],
}
impl Default for Ui {
    fn default() -> Self {
        Self {
            synth_page: 0,
            texture_slot: 0,
            layer: 0,
            layer_detail: false,
            chop_slice: 0,
            chop_cursor: 0,
            chop_zoom: 0,
            held_slices: [None; 16],
            chord_modes: [false; 16],
            held_chords: [None; 16],
            page: Page::Loop,
            selected: 0,
            sound: 0,
            fx_slot: 0,
            latch: true,
            pickup: Pickup::default(),
            grid_event: 0,
            grid_bar: 0,
            notice: "",
            active: false,
            held: [None; 16],
            fx_held: [None; 16],
            master_target: 0,
            shift_view: false,
            fx_pages: [(0, true); 2],
        }
    }
}
pub const PERFORMANCE: [Effect; 8] = [
    Effect::Filter,
    Effect::Isolator,
    Effect::Repeat,
    Effect::Reverse,
    Effect::TapeStop,
    Effect::DubEcho,
    Effect::Crush,
    Effect::Wobble,
];
pub fn effect_name(e: Effect) -> &'static str {
    match e {
        Effect::Off => "OFF",
        Effect::Drive => "DRIVE",
        Effect::Filter => "FILTER",
        Effect::Compressor => "COMP",
        Effect::LoFi => "LOFI",
        Effect::Isolator => "ISOLATE",
        Effect::Repeat => "REPEAT",
        Effect::Reverse => "REVERSE",
        Effect::TapeStop => "TAPE",
        Effect::DubEcho => "DUB",
        Effect::Crush => "CRUSH",
        Effect::Wobble => "WOBBLE",
    }
}
impl Ui {
    pub fn navigate(&mut self, page: Page, s: &Snapshot) {
        let page = if page == Page::Play && s.track_kind(self.selected) == Kind::Audio {
            if s.words[16 + self.selected * 12 + 11] == 0 {
                self.notice = "RECORD OR LOAD A WAV FIRST";
                return;
            }
            Page::Chop
        } else {
            page
        };
        if page == Page::Chop
            && (s.track_kind(self.selected) != Kind::Audio
                || s.words[16 + self.selected * 12 + 11] == 0)
        {
            self.notice = "SELECT AN AUDIO LOOP";
            return;
        }
        if matches!(page, Page::KitMix | Page::DrumSound)
            && s.track_kind(self.selected) != Kind::Drum
        {
            self.notice = "SELECT A DRUM TRACK FIRST";
            return;
        }
        if self.page == Page::TrackFx {
            self.fx_pages[0] = (self.fx_slot, self.latch)
        } else if self.page == Page::MasterFx {
            self.fx_pages[1] = (self.fx_slot, self.latch)
        }
        if page == Page::TrackFx {
            (self.fx_slot, self.latch) = self.fx_pages[0]
        } else if page == Page::MasterFx {
            (self.fx_slot, self.latch) = self.fx_pages[1]
        }
        if page == Page::DrumSound && self.page != page {
            self.layer_detail = false;
        }
        self.page = page;
        self.notice = "";
        let level = if matches!(page, Page::KitMix | Page::DrumSound) {
            s.drum(self.sound).level
        } else {
            s.level(
                if matches!(page, Page::MasterFx | Page::Texture | Page::Sidechain) {
                    16
                } else {
                    self.selected
                },
            )
        };
        self.pickup.select(level)
    }
    pub fn soft(&mut self, key: usize, shift: bool, s: &Snapshot, mut send: impl FnMut(Command)) {
        if self.page == Page::Output {
            if key == 0 {
                self.navigate(Page::Mix, s);
            } else {
                self.selected = (self.selected + if key == 1 { 15 } else { 1 }) % 16;
                send(Command::Select(self.selected));
            }
            return;
        }
        if self.page == Page::Source {
            if key == 0 {
                self.navigate(Page::Loop, s);
            } else if key == 1 {
                self.navigate(Page::Setup, s);
            }
            return;
        }
        if self.page == Page::DrumSound {
            self.notice = "";
            if key == 0 {
                self.navigate(Page::Loop, s);
                return;
            }
            let layer = key - 1;
            if shift {
                let cfg = s.layer(self.sound, layer);
                send(Command::DrumValue {
                    track: self.selected,
                    sound: self.sound,
                    index: (6 + layer * 6 + 4) as u8,
                    value: if cfg.enabled { 0. } else { 1. },
                });
            } else {
                self.layer = layer;
            }
            return;
        }
        if self.page == Page::Loop && key < 2 {
            self.navigate(
                if key == 0 {
                    Page::Source
                } else if s.track_kind(self.selected) == Kind::Drum {
                    Page::DrumSound
                } else {
                    Page::Play
                },
                s,
            );
            return;
        }
        if self.page == Page::Chop {
            match key {
                0 => self.navigate(Page::Loop, s),
                1 => send(Command::ChopMode(self.selected)),
                2 if ((s.words[506] >> 8) & 255) == 3 && s.words[774] == 0 => {
                    send(Command::ResetSlices(self.selected))
                }
                2 => send(Command::ChopApply(self.selected)),
                _ => {}
            }
            return;
        }
        if matches!(self.page, Page::Loop | Page::Play) && key == 2 {
            let mode = if s.words[16 + self.selected * 12] & (1 << 18) == 0 {
                RecordMode::Overdub
            } else {
                RecordMode::Replace
            };
            send(Command::RecordMode(self.selected, mode));
            self.notice = "";
            return;
        }
        if self.page == Page::Play && key == 1 && s.track_kind(self.selected) == Kind::Synth {
            self.chord_modes[self.selected] = !self.chord_modes[self.selected];
            self.notice = "";
            return;
        }
        if self.page == Page::Sidechain {
            let mut c = s.sidechain();
            match key {
                0 => {
                    self.navigate(Page::Texture, s);
                    return;
                }
                1 => {
                    if shift {
                        c.sync = !c.sync;
                    } else {
                        c.sound = if c.sound == 16 { 0 } else { 16 };
                    }
                }
                _ => c.mode = (c.mode + 1) % 3,
            }
            send(Command::Sidechain(c));
            return;
        }
        if [
            Page::Mix,
            Page::TrackFx,
            Page::MasterFx,
            Page::KitMix,
            Page::Texture,
        ]
        .contains(&self.page)
        {
            if key == 0 && self.page == Page::Mix && s.track_kind(self.selected) == Kind::Drum
                || shift && key == 0
            {
                self.navigate(Page::KitMix, s);
                return;
            }
            self.navigate([Page::Mix, Page::TrackFx, Page::Texture][key.min(2)], s)
        } else {
            self.navigate([Page::Loop, Page::Play, Page::Grid][key.min(2)], s)
        }
        send(Command::Select(self.selected));
    }
    // Preserve the decoded event fields together at the controller boundary.
    #[allow(clippy::too_many_arguments)]
    pub fn pad(
        &mut self,
        pad: usize,
        on: bool,
        velocity: u8,
        shift: bool,
        erase: bool,
        s: &Snapshot,
        mut send: impl FnMut(Command),
    ) -> Action {
        if pad >= 16 {
            return Action::None;
        }
        if !on {
            if let Some(track) = self.held_slices[pad].take() {
                send(Command::Slice {
                    track,
                    pad,
                    velocity: 0,
                    on: false,
                });
            }
            if let Some(track) = self.held_chords[pad].take() {
                send(Command::Chord {
                    track,
                    pad,
                    velocity: 0,
                    on: false,
                });
            }
            if let Some((track, note, source)) = self.held[pad].take() {
                send(Command::Note {
                    track,
                    note,
                    velocity: 0,
                    on: false,
                    source,
                })
            }
            if let Some((slot, target, track)) = self.fx_held[pad].take() {
                let fx = Fx {
                    effect: Effect::Off,
                    ..Fx::default()
                };
                send(fx_command(target, track, slot, fx, false));
            }
            return Action::None;
        }
        match self.page {
            Page::Output => {
                if pad < 3 {
                    send(Command::Output(
                        self.selected,
                        [OutputTarget::Main, OutputTarget::Usb, OutputTarget::Both][pad],
                    ));
                }
            }
            Page::Chop => {
                if pad >= ((s.words[506] >> 16) & 255) as usize {
                    return Action::None;
                }
                self.chop_slice = pad;
                self.chop_cursor = s.slice(pad).start;
                self.wave_request(s, &mut send);
                self.held_slices[pad] = Some(self.selected);
                if !shift {
                    send(Command::Slice {
                        track: self.selected,
                        pad,
                        velocity,
                        on: true,
                    });
                }
            }
            Page::Source => {
                if pad < 3 {
                    send(Command::SetKind(
                        self.selected,
                        [Kind::Audio, Kind::Synth, Kind::Drum][pad],
                    ));
                    self.navigate(Page::Loop, s);
                } else if pad == 12 {
                    return Action::Browse;
                }
            }
            Page::DrumSound => {
                if self.sound != pad {
                    self.pickup.select(s.drum(pad).level);
                }
                self.sound = pad;
                self.held[pad] = Some((self.selected, pad as u8, 100 + pad as u16));
                send(Command::Note {
                    track: self.selected,
                    note: pad as u8,
                    velocity,
                    on: true,
                    source: 100 + pad as u16,
                });
            }
            Page::Loop => {
                // Selection never accidentally starts/stops a loop; DATA controls selected-loop playback.
                send(Command::Select(pad));
                if shift && s.words[16 + pad * 12 + 4] > 0 {
                    send(Command::Pad(pad, false));
                }
                self.selected = pad;
                self.pickup.select(s.level(pad));
            }
            Page::Mix => {
                self.selected = pad;
                send(Command::Select(pad));
                self.pickup.select(s.level(pad));
                if erase {
                    send(if shift {
                        Command::Solo(pad)
                    } else {
                        Command::Mute(pad)
                    })
                }
            }
            Page::KitMix => {
                self.sound = pad;
                self.pickup.select(s.drum(pad).level);
                if erase {
                    send(Command::DrumToggle {
                        track: self.selected,
                        sound: pad,
                        solo: shift,
                    })
                } else if !shift {
                    send(Command::Note {
                        track: self.selected,
                        note: pad as u8,
                        velocity,
                        on: true,
                        source: 100 + pad as u16,
                    });
                }
            }
            Page::Play => {
                if shift && s.track_kind(self.selected) == Kind::Drum {
                    self.sound = pad;
                    send(if erase {
                        Command::UndoDrumPart {
                            track: self.selected,
                            sound: pad,
                        }
                    } else {
                        Command::GenerateDrumPart {
                            track: self.selected,
                            sound: pad,
                        }
                    });
                } else if shift {
                    self.selected = pad;
                    send(Command::Select(pad));
                    self.pickup.select(s.level(pad));
                } else if s.track_kind(self.selected) == Kind::Synth
                    && self.chord_modes[self.selected]
                {
                    self.held_chords[pad] = Some(self.selected);
                    send(Command::Chord {
                        track: self.selected,
                        pad,
                        velocity,
                        on: true,
                    });
                } else {
                    let note = if s.track_kind(self.selected) == Kind::Drum {
                        pad as u8
                    } else {
                        48 + pad as u8
                    };
                    if s.track_kind(self.selected) == Kind::Drum {
                        self.sound = pad;
                    }
                    let source = 100 + pad as u16;
                    self.held[pad] = Some((self.selected, note, source));
                    send(Command::Note {
                        track: self.selected,
                        note,
                        velocity,
                        on: true,
                        source,
                    });
                }
            }
            Page::Sidechain => {
                let mut c = s.sidechain();
                if shift {
                    c.mode = 1;
                    c.mask ^= 1 << pad;
                } else {
                    c.sound = pad as u8;
                }
                send(Command::Sidechain(c));
            }
            Page::Texture => {
                if pad < 5 {
                    self.texture_slot = pad;
                    send(Command::TextureValue {
                        slot: pad,
                        index: 3,
                        value: if s.texture().enabled[pad] { 0. } else { 1. },
                    });
                } else if pad == 5 {
                    self.texture_slot = 5;
                    let mut c = s.sidechain();
                    if c.depth == 0. {
                        c.depth = 0.65;
                        c.enabled = true;
                    } else {
                        c.enabled = !c.enabled;
                    }
                    send(Command::Sidechain(c));
                } else if pad == 6 || pad == 7 {
                    self.texture_slot = pad;
                    let slot = pad - 6;
                    let fx = s.fx(fx_base(1, slot));
                    if fx.effect == Effect::Off {
                        send(Command::FxSelect {
                            target: 1,
                            track: self.selected,
                            slot,
                            effect: if slot == 0 {
                                Effect::Filter
                            } else {
                                Effect::Repeat
                            },
                            temporary: false,
                        });
                        send(Command::MasterBypass {
                            slot,
                            bypass: false,
                        });
                    } else {
                        send(Command::MasterBypass {
                            slot,
                            bypass: !s.texture().master_bypass[slot],
                        });
                    }
                } else {
                    match pad {
                        8 => {
                            self.master_target = 0;
                            self.navigate(Page::MasterFx, s);
                        }
                        9 => self.navigate(Page::Sidechain, s),
                        10 | 11 => {
                            self.master_target = (pad - 9) as u8;
                            self.navigate(Page::MasterFx, s);
                        }
                        12 => {
                            for slot in 0..5 {
                                send(Command::TextureValue {
                                    slot,
                                    index: 3,
                                    value: 0.,
                                });
                            }
                            for slot in 0..2 {
                                send(Command::MasterBypass { slot, bypass: true });
                            }
                            let mut c = s.sidechain();
                            c.enabled = false;
                            send(Command::Sidechain(c));
                        }
                        _ => {}
                    }
                }
            }
            Page::TrackFx | Page::MasterFx => {
                let master = self.page == Page::MasterFx;
                if pad < 8 {
                    let slot = self.fx_slot;
                    let target = if master { self.master_target + 1 } else { 0 };
                    for held in &mut self.fx_held {
                        if held.is_some_and(|(h, m, t)| {
                            (target >= 2 || h == slot)
                                && m == target
                                && (target != 0 || t == self.selected)
                        }) {
                            *held = None
                        }
                    }
                    let effect = if shift {
                        [
                            Effect::Drive,
                            Effect::Compressor,
                            Effect::LoFi,
                            Effect::Filter,
                            Effect::TapeStop,
                            Effect::DubEcho,
                            Effect::Crush,
                            Effect::Wobble,
                        ][pad]
                    } else {
                        PERFORMANCE[pad]
                    };
                    if self.latch {
                        let count = if target >= 2 { 1 } else { 2 };
                        let existing =
                            (0..count).find(|i| s.fx(fx_base(target, *i)).effect == effect);
                        let chosen = existing.or_else(|| {
                            (0..count).find(|i| s.fx(fx_base(target, *i)).effect == Effect::Off)
                        });
                        if let Some(slot) = chosen {
                            self.fx_slot = slot;
                            let mut fx = s.fx(fx_base(target, slot));
                            fx.effect = if existing.is_some() {
                                Effect::Off
                            } else {
                                effect
                            };
                            send(fx_command(target, self.selected, slot, fx, false));
                            if target == 1 {
                                send(Command::MasterBypass {
                                    slot,
                                    bypass: false,
                                });
                            }
                        } else {
                            self.notice = if count == 1 {
                                "BUS FULL: TURN FX OFF"
                            } else {
                                "2 FX MAX: TURN ONE OFF"
                            };
                        }
                        return Action::None;
                    }
                    let current = s.fx(fx_base(target, slot));
                    let enabled = !(self.latch && current.effect == effect);
                    let fx = Fx {
                        effect: if enabled { effect } else { Effect::Off },
                        ..current
                    };
                    send(fx_command(target, self.selected, slot, fx, !self.latch));
                    if !self.latch {
                        self.fx_held[pad] = Some((slot, target, self.selected))
                    }
                } else {
                    match pad {
                        8 => self.fx_slot = 0,
                        9 => {
                            if !master || self.master_target == 0 {
                                self.fx_slot = 1
                            }
                        }
                        10 => self.latch = !self.latch,
                        11 => {
                            for held in &mut self.fx_held {
                                if held.is_some_and(|(_, target, track)| {
                                    if master {
                                        target == self.master_target + 1
                                    } else {
                                        target == 0 && track == self.selected
                                    }
                                }) {
                                    *held = None
                                }
                            }
                            if master && self.master_target != 0 {
                                send(Command::BusFx {
                                    slot: usize::from(self.master_target - 1),
                                    fx: Fx::default(),
                                })
                            } else if master {
                                send(Command::FxOff)
                            } else {
                                for slot in 0..2 {
                                    send(Command::SetFx {
                                        track: self.selected,
                                        slot,
                                        fx: Fx::default(),
                                    })
                                }
                            }
                        }
                        12 => {
                            if master {
                                self.master_target = 0
                            } else {
                                send(Command::Route(self.selected, Route::Dry))
                            }
                        }
                        13 => {
                            if master {
                                self.master_target = 1;
                                self.fx_slot = 0;
                            } else {
                                send(Command::Route(self.selected, Route::A))
                            }
                        }
                        14 => {
                            if master {
                                self.master_target = 2;
                                self.fx_slot = 0;
                            } else {
                                send(Command::Route(self.selected, Route::B))
                            }
                        }
                        _ => self.navigate(Page::Setup, s),
                    }
                }
            }
            Page::Setup => match pad {
                0..=2 => send(Command::SetKind(
                    self.selected,
                    [Kind::Audio, Kind::Synth, Kind::Drum][pad],
                )),
                3 => {
                    if erase {
                        send(Command::Clear(self.selected))
                    } else {
                        self.notice = "HOLD ERASE TO CLEAR"
                    }
                }
                4..=6 => send(Command::Preroll((pad - 4) as u8)),
                7 => return Action::Save,
                8 => return Action::Load,
                9 => send(Command::Resample(self.selected)),
                10 => send(Command::Stop),
                11 => send(Command::PlayAll),
                12 => return Action::Browse,
                13 => send(Command::ResetSlices(self.selected)),
                15 => return Action::Exit,
                _ => {}
            },
            Page::Grid => {
                self.grid_event = pad;
                self.notice = "DATA: NOTE  K1:PITCH K2:LEN K3:VEL"
            }
        }
        Action::None
    }
    pub fn encoder(&mut self, delta: i32, s: &Snapshot, mut send: impl FnMut(Command)) {
        if self.page == Page::Sidechain {
            let mut c = s.sidechain();
            c.source = (i32::from(c.source) + delta).rem_euclid(16) as u8;
            send(Command::Sidechain(c));
            return;
        }
        if self.page == Page::Texture {
            self.texture_slot = (self.texture_slot as i32 + delta).rem_euclid(8) as usize;
            return;
        }
        if self.page == Page::Grid {
            let n = s.words[16 + self.selected * 12 + 6].min(64) as usize;
            let valid: Vec<usize> = (0..n)
                .filter(|i| s.words[353 + i * 2] & (1 << 16) != 0)
                .collect();
            if !valid.is_empty() {
                let old = valid
                    .iter()
                    .position(|x| *x == self.grid_event)
                    .unwrap_or(0);
                self.grid_event =
                    valid[(old as i32 + delta).rem_euclid(valid.len() as i32) as usize]
            }
        } else if self.page == Page::Chop {
            if self.shift_view {
                self.chop_zoom = (i32::from(self.chop_zoom) + delta).clamp(0, 8) as u8;
            } else {
                let step = ((s.words[769].saturating_sub(s.words[768])) / 310).max(1);
                self.chop_cursor = (i64::from(self.chop_cursor) + i64::from(delta) * step as i64)
                    .clamp(0, s.words[770].saturating_sub(1) as i64)
                    as u32;
            }
            self.wave_request(s, &mut send);
        } else if matches!(self.page, Page::KitMix | Page::DrumSound) {
            self.sound = (self.sound as i32 + delta).rem_euclid(16) as usize;
            self.pickup.select(s.drum(self.sound).level)
        } else {
            self.selected = (self.selected as i32 + delta).rem_euclid(16) as usize;
            send(Command::Select(self.selected));
            self.pickup.select(s.level(self.selected))
        }
    }
    fn wave_request(&self, s: &Snapshot, send: &mut impl FnMut(Command)) {
        let frames = s.words[770] as u32;
        if frames == 0 {
            return;
        }
        let span = (frames / (1u32 << self.chop_zoom)).max(16).min(frames);
        let start = self.chop_cursor.saturating_sub(span / 2).min(frames - span);
        send(Command::WaveView {
            track: self.selected,
            start,
            end: start + span,
        });
    }
    pub fn fader(&mut self, value: f32, s: &Snapshot, mut send: impl FnMut(Command)) {
        if matches!(self.page, Page::KitMix | Page::DrumSound) {
            let m = s.drum(self.sound);
            if let Some(v) = self.pickup.move_to(value, m.level) {
                send(Command::DrumValue {
                    track: self.selected,
                    sound: self.sound,
                    index: 0,
                    value: v,
                });
            }
        } else {
            let i = if matches!(self.page, Page::MasterFx | Page::Texture | Page::Sidechain) {
                16
            } else {
                self.selected
            };
            if let Some(v) = self.pickup.move_to(value, s.level(i)) {
                send(if i == 16 {
                    Command::Master(v)
                } else {
                    Command::Level(i, v)
                })
            }
        }
    }
    pub fn knob(&mut self, index: usize, value: f32, s: &Snapshot, mut send: impl FnMut(Command)) {
        let value = value.clamp(0., 1.);
        if index > 2 {
            return;
        }
        match self.page {
            Page::Sidechain => {
                let mut c = s.sidechain();
                match index {
                    0 => {
                        c.depth = value;
                        c.enabled = value > 0.;
                    }
                    1 => c.release = value,
                    _ => c.threshold = (0.5 - value * 0.49).max(0.01),
                }
                send(Command::Sidechain(c));
            }
            Page::Texture => {
                if self.texture_slot < 5 {
                    send(Command::TextureValue {
                        slot: self.texture_slot,
                        index: index as u8,
                        value,
                    });
                } else if self.texture_slot == 5 {
                    let mut c = s.sidechain();
                    match index {
                        0 => {
                            c.depth = value;
                            c.enabled = value > 0.;
                        }
                        1 => c.release = value,
                        _ => c.threshold = (0.5 - value * 0.49).max(0.01),
                    }
                    send(Command::Sidechain(c));
                } else {
                    send(Command::FxValue {
                        target: 1,
                        track: self.selected,
                        slot: self.texture_slot - 6,
                        index: index as u8,
                        value,
                    });
                }
            }
            Page::Output => {
                if index == 0 {
                    send(Command::Output(
                        self.selected,
                        [OutputTarget::Main, OutputTarget::Usb, OutputTarget::Both]
                            [(value * 2.99).floor() as usize],
                    ));
                }
            }
            Page::DrumSound => {
                self.notice = "";
                let index = if self.layer_detail {
                    match index {
                        0 => 3,
                        1 => 4,
                        _ => 5,
                    }
                } else {
                    index
                };
                if index == 5 && !s.layer_pcm(self.sound, self.layer) && value >= 0.5 {
                    self.notice = "LOAD A SAMPLE FOR REV";
                    return;
                }
                let value = match index {
                    0 => value,
                    1 => (value * 48.).round() - 24.,
                    2 => (value * 100.).round() - 50.,
                    3 => 0.01 + value * 15.99,
                    _ => {
                        if value >= 0.5 {
                            1.
                        } else {
                            0.
                        }
                    }
                };
                send(Command::DrumValue {
                    track: self.selected,
                    sound: self.sound,
                    index: (6 + self.layer * 6 + index) as u8,
                    value,
                });
            }
            Page::Source => {}
            Page::Chop => {
                let mode = (s.words[506] >> 8) & 255;
                if self.shift_view && mode != 3 {
                    let field = match (mode, index) {
                        (0, 0) => Some(0),
                        (1, 0) => Some(1),
                        (1, 1) => Some(2),
                        (1, 2) => Some(0),
                        (2, 0) => Some(3),
                        _ => None,
                    };
                    if let Some(index) = field {
                        send(Command::ChopOption {
                            track: self.selected,
                            index,
                            value,
                        });
                    }
                } else {
                    let frames = s.words[770].max(1) as f32;
                    let mapped = if index < 2 {
                        ((s.words[768] as f32
                            + value * s.words[769].saturating_sub(s.words[768]) as f32)
                            / frames)
                            .clamp(0., 1.)
                    } else {
                        value
                    };
                    send(Command::SliceValue {
                        track: self.selected,
                        pad: self.chop_slice,
                        index: index as u8,
                        value: mapped,
                    });
                }
            }
            Page::Mix | Page::Loop => send(match index {
                0 => Command::Pan(self.selected, value * 2. - 1.),
                1 => Command::Send(self.selected, false, value),
                _ => Command::Send(self.selected, true, value),
            }),
            Page::KitMix => send(Command::DrumValue {
                track: self.selected,
                sound: self.sound,
                index: index as u8 + 1,
                value: if index == 0 { value * 2. - 1. } else { value },
            }),
            Page::TrackFx | Page::MasterFx => {
                let master = self.page == Page::MasterFx;
                let target = if master { self.master_target + 1 } else { 0 };
                send(Command::FxValue {
                    target,
                    track: self.selected,
                    slot: self.fx_slot,
                    index: index as u8,
                    value,
                });
            }
            Page::Play => {
                if s.track_kind(self.selected) == Kind::Synth && self.synth_page == 7 {
                    match index {
                        0 => send(Command::Pan(self.selected, value * 2. - 1.)),
                        1 => send(Command::SynthValue {
                            track: self.selected,
                            index: 27,
                            value,
                        }),
                        _ => send(Command::SynthValue {
                            track: self.selected,
                            index: 22,
                            value: 0.05 * 400_f32.powf(value),
                        }),
                    }
                    return;
                }
                if s.track_kind(self.selected) == Kind::Synth && self.synth_page > 0 {
                    let (field, v) = match (self.synth_page, index, self.shift_view) {
                        (6, 0, true) => (23, (value * 5.99).floor()),
                        (6, 0, false) => (22, 0.05 * 400_f32.powf(value)),
                        (6, 1, _) => (24, value * 200.),
                        (6, _, true) => (26, value),
                        (6, _, false) => (25, value * 4.),
                        (1, 0, _) => (3, (value * 3.99).floor()),
                        (2, 0, _) => (6, (value * 3.99).floor()),
                        (1, 1, false) => (7, (value * 48.).round() - 24.),
                        (2, 1, false) => (8, (value * 48.).round() - 24.),
                        (1, 1, true) => (15, value * 200. - 100.),
                        (2, 1, true) => (16, value * 200. - 100.),
                        (1, _, _) => (9, value),
                        (2, _, _) => (10, value),
                        (3, 0, _) => (0, 40. * 450_f32.powf(value)),
                        (3, 1, false) => (11, value),
                        (3, 1, true) => (21, value * 2. - 1.),
                        (3, _, _) => (12, (value * 2.99).floor()),
                        (5, 0, _) => (17, 0.001 * 2000_f32.powf(value)),
                        (5, 1, _) => (18, 0.001 * 6000_f32.powf(value)),
                        (5, _, true) => (20, 0.01 * 600_f32.powf(value)),
                        (5, _, false) => (19, value),
                        (4, 0, _) => (1, 0.001 * 2000_f32.powf(value)),
                        (4, 1, _) => (13, 0.001 * 6000_f32.powf(value)),
                        (4, _, true) => (2, 0.01 * 600_f32.powf(value)),
                        _ => (14, value),
                    };
                    send(Command::SynthValue {
                        track: self.selected,
                        index: field,
                        value: v,
                    });
                    return;
                }
                if s.track_kind(self.selected) == Kind::Synth && self.chord_modes[self.selected] {
                    let value = match index {
                        0 => (value * 11.99).floor(),
                        _ => (value * 2.99).floor(),
                    };
                    send(Command::ChordValue {
                        track: self.selected,
                        index: index as u8,
                        value,
                    });
                    return;
                }

                if s.track_kind(self.selected) == Kind::Drum {
                    let (field, v) = match index {
                        0 => (4, value * 48. - 24.),
                        1 => (
                            5,
                            0.01 + value
                                * if s.words[787] & (1 << self.sound) != 0 {
                                    15.99
                                } else {
                                    3.99
                                },
                        ),
                        _ => (0, value),
                    };
                    send(Command::DrumValue {
                        track: self.selected,
                        sound: self.sound,
                        index: field,
                        value: v,
                    });
                } else {
                    let v = match index {
                        0 => 80. * (18000_f32 / 80.).powf(value),
                        1 => 0.001 + value * 0.5,
                        _ => 0.02 + value * 3.,
                    };
                    send(Command::SynthValue {
                        track: self.selected,
                        index: index as u8,
                        value: v,
                    });
                }
            }
            Page::Setup => match index {
                0 => send(Command::Bpm(40. + f64::from(value) * 200.)),
                1 => send(Command::Preroll((value * 2.99) as u8)),
                _ => {}
            },
            Page::Grid => {
                let b = 352 + self.grid_event.min(63) * 2;
                let tick = s.words[b];
                let flags = s.words[b + 1];
                if flags & (1 << 16) == 0 {
                    return;
                }
                let note = if index == 0 {
                    (value * 127.) as u8
                } else {
                    flags as u8
                };
                let velocity = if index == 2 {
                    (1. + value * 126.) as u8
                } else {
                    (flags >> 8) as u8
                };
                let length = if index == 1 {
                    (PPQ as f32 * (0.0625 + value * 3.9375)) as u64
                } else {
                    let n = s.words[16 + self.selected * 12 + 6].min(64) as usize;
                    ((self.grid_event + 1)..n)
                        .find(|i| {
                            let f = s.words[353 + i * 2];
                            f & (1 << 16) == 0 && f as u8 == flags as u8
                        })
                        .map_or(PPQ / 4, |i| s.words[352 + i * 2].saturating_sub(tick))
                };
                send(Command::Edit {
                    track: self.selected,
                    index: self.grid_event,
                    note,
                    velocity,
                    length,
                })
            }
        }
    }
}

pub fn fx_base(target: u8, slot: usize) -> usize {
    match target {
        0 => 320 + slot * 4,
        1 => 328 + slot * 4,
        2 => 336,
        _ => 340,
    }
}
fn fx_command(target: u8, track: usize, slot: usize, fx: Fx, temporary: bool) -> Command {
    Command::FxSelect {
        target,
        track,
        slot,
        effect: fx.effect,
        temporary,
    }
}
