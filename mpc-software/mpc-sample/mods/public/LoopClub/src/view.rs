//! Fixed 320x240 layout, shared by firmware drawing and host previews.
use crate::{
    model::*,
    ui::{Snapshot, Ui, effect_name, fx_base},
};
pub trait Painter {
    fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32);
    #[allow(clippy::too_many_arguments)]
    fn text(&mut self, x: i32, y: i32, w: i32, h: i32, size: f32, color: u32, text: &str);
}
const BG: u32 = 0xff142620;
const PANEL: u32 = 0xff20392e;
const GOLD: u32 = 0xffd6b46d;
const INK: u32 = 0xffece4cf;
const DIM: u32 = 0xffc0cbbd;
const RED: u32 = 0xffdd775d;
fn label(p: &mut impl Painter, x: i32, y: i32, w: i32, color: u32, text: &str) {
    p.text(x, y, w, 17, 15., color, text)
}
fn cell(
    p: &mut impl Painter,
    pad: usize,
    title: &str,
    subtitle: &str,
    selected: bool,
    level: Option<f32>,
    accent: u32,
) {
    let x = 5 + (pad % 4) as i32 * 79;
    let y = 53 + (3 - pad / 4) as i32 * 32;
    p.rect(x, y, 74, 30, if selected { GOLD } else { PANEL });
    p.rect(x + 1, y + 1, 72, 28, if selected { GOLD } else { PANEL });
    label(p, x + 3, y, 67, if selected { BG } else { INK }, title);
    p.text(
        x + 3,
        y + 14,
        68,
        16,
        14.,
        if selected { BG } else { accent },
        subtitle,
    );
    if let Some(v) = level {
        p.rect(x + 3, y + 27, 68, 1, BG);
        p.rect(x + 3, y + 27, (v.clamp(0., 1.) * 68.) as i32, 1, accent)
    }
}

fn mixer_cell(
    p: &mut impl Painter,
    pad: usize,
    title: &str,
    selected: bool,
    level: f32,
    mute: bool,
    solo: bool,
) {
    let x = 5 + (pad % 4) as i32 * 79;
    let y = 80 + (3 - pad / 4) as i32 * 27;
    p.rect(x, y, 74, 24, if selected { GOLD } else { PANEL });
    let title: String = title
        .chars()
        .take(if mute || solo { 7 } else { 9 })
        .collect();
    p.text(
        x + 3,
        y + 1,
        68,
        17,
        16.,
        if selected { BG } else { INK },
        &format!(
            "{title}{}",
            if mute {
                " M"
            } else if solo {
                " S"
            } else {
                ""
            }
        ),
    );
    p.rect(x + 4, y + 17, 66, 4, BG);
    p.rect(
        x + 4,
        y + 17,
        (level.clamp(0., 1.) * 66.) as i32,
        4,
        if mute {
            DIM
        } else if selected {
            INK
        } else {
            GOLD
        },
    );
}

fn status(n: u64) -> &'static str {
    match (n >> 8) & 255 {
        1 => "ARMED",
        2 => "COUNT IN",
        3 => "REC",
        4 => "CLOSING",
        5 => "PLAY",
        6 => "STOP",
        _ => "EMPTY",
    }
}
pub fn render(ui: &Ui, s: &Snapshot, p: &mut impl Painter) {
    p.rect(0, 0, 320, 240, BG);
    let heading = match ui.page {
        Page::Loop => "Loop Club".to_owned(),
        Page::Play => format!(
            "{} {:02}",
            match s.track_kind(ui.selected) {
                Kind::Synth => "SYNTH",
                Kind::Drum => "DRUMS",
                Kind::Audio => "AUDIO",
            },
            ui.selected + 1
        ),
        Page::Grid => "NOTE GRID".to_owned(),
        Page::Mix => "TRACK MIX".to_owned(),
        Page::KitMix => "DRUM MIX".to_owned(),
        Page::TrackFx => "TRACK FX".to_owned(),
        Page::MasterFx => {
            ["PERFORM FX", "BUS A FX", "BUS B FX"][ui.master_target.min(2) as usize].to_owned()
        }
        Page::Setup => "PROJECT".to_owned(),
        Page::Chop => "CHOP".to_owned(),
        Page::Source => "TRACK SOURCE".to_owned(),
        Page::Output => "OUTPUT".to_owned(),
        Page::DrumSound => "DRUM SOUND".to_owned(),
        Page::Sidechain => "SIDECHAIN".to_owned(),
        Page::Texture => "MASTER FX".to_owned(),
    };
    p.text(7, 2, 165, 23, 20., INK, &heading);
    p.text(
        182,
        5,
        133,
        19,
        15.,
        GOLD,
        &format!(
            "{:.0} BPM {}",
            s.bpm(),
            if s.words[4] & 1 != 0 { "RUN" } else { "STOP" }
        ),
    );
    let fx = [
        Page::Mix,
        Page::KitMix,
        Page::TrackFx,
        Page::MasterFx,
        Page::Texture,
    ]
    .contains(&ui.page);
    let mut tabs = if fx {
        ["MIX", "TRACK FX", "MASTER FX"]
    } else {
        ["LOOP", "PLAY", "GRID"]
    };
    if ui.page == Page::Loop {
        tabs = ["SOURCE", "SOUND", "REPLACE"];
    }
    if ui.page == Page::Output {
        tabs = ["BACK", "TRACK <", "TRACK >"];
    }
    if ui.page == Page::Sidechain {
        tabs = ["BACK", "SOURCE", "TARGET"];
    }
    if ui.page == Page::Source {
        tabs = ["BACK", "PROJECT", "LOAD"];
    }
    if ui.page == Page::DrumSound {
        tabs = ["BACK", "LAYER 1", "LAYER 2"];
    }
    if ui.page == Page::Chop {
        tabs[1] = ["EQUAL >", "TRANSIENT >", "BPM >", "MANUAL >"]
            [((s.words[506] >> 8) & 255).min(3) as usize];
        tabs[2] = if s.words[774] != 0 {
            "APPLY *"
        } else if ((s.words[506] >> 8) & 255) == 3 {
            "RESET"
        } else {
            "APPLY"
        };
    }
    if ui.page == Page::Play && s.track_kind(ui.selected) == Kind::Drum {
        tabs[1] = "DRUM";
    }
    if ui.page == Page::Play && s.track_kind(ui.selected) == Kind::Synth {
        tabs[1] = if ui.chord_modes[ui.selected] {
            "CHORD >"
        } else {
            "NOTES >"
        };
    }
    if matches!(ui.page, Page::Loop | Page::Play) {
        tabs[2] = if s.words[16 + ui.selected * 12] & (1 << 18) == 0 {
            "REPLACE"
        } else {
            "OVERDUB"
        };
    }
    for (i, title) in tabs.iter().enumerate() {
        let active = if ui.page == Page::Loop {
            i == 2 && s.words[16 + ui.selected * 12] & (1 << 18) != 0
        } else if ui.page == Page::Source {
            false
        } else if ui.page == Page::DrumSound {
            i == ui.layer + 1
        } else if fx {
            matches!(
                (i, ui.page),
                (0, Page::Mix | Page::KitMix)
                    | (1, Page::TrackFx)
                    | (2, Page::MasterFx | Page::Texture)
            )
        } else {
            matches!(
                (i, ui.page),
                (0, Page::Loop) | (1, Page::Play | Page::Chop) | (2, Page::Grid)
            ) || (i == 2 && ui.page == Page::Chop && s.words[774] != 0)
        };
        p.rect(
            5 + i as i32 * 105,
            27,
            100,
            22,
            if active { GOLD } else { PANEL },
        );
        p.text(
            9 + i as i32 * 105,
            29,
            93,
            19,
            16.,
            if active { BG } else { INK },
            title,
        );
    }
    match ui.page {
        Page::Sidechain => sidechain_panel(ui, s, p),
        Page::Texture => texture_panel(ui, s, p),
        Page::Output => output_panel(ui, s, p),
        Page::Chop => {
            p.rect(5, 53, 310, 48, PANEL);
            p.rect(5, 74, 310, 1, DIM);
            for i in 0..256 {
                let v = s.words[512 + i];
                let min = f32::from_bits(v as u32).clamp(-1., 1.);
                let max = f32::from_bits((v >> 32) as u32).clamp(-1., 1.);
                let x = 5 + i as i32 * 310 / 256;
                let right = 5 + (i + 1) as i32 * 310 / 256;
                let y = 74 - (max * 24.) as i32;
                let bottom = 74 - (min * 24.) as i32;
                p.rect(x, y, (right - x).max(1), (bottom - y).max(1), DIM);
            }
            let start = s.words[768];
            let end = s.words[769].max(start + 1);
            let x_at = |frame: u64| {
                5 + ((frame.saturating_sub(start)) as f64 / (end - start) as f64 * 310.) as i32
            };
            let count = ((s.words[506] >> 16) & 255).min(16) as usize;
            for pad in 0..count {
                let slice = s.slice(pad);
                for frame in [slice.start, slice.end] {
                    if u64::from(frame) >= start && u64::from(frame) <= end {
                        p.rect(
                            x_at(u64::from(frame)),
                            53,
                            1,
                            48,
                            if pad == ui.chop_slice { GOLD } else { BG },
                        )
                    }
                }
            }
            if u64::from(ui.chop_cursor) >= start && u64::from(ui.chop_cursor) < end {
                p.rect(x_at(u64::from(ui.chop_cursor)), 53, 1, 48, RED);
            }
            for pad in 0..16 {
                let x = 5 + (pad % 4) as i32 * 79;
                let y = 108 + (3 - pad / 4) as i32 * 18;
                let enabled = pad < count;
                let active = pad == ui.chop_slice || s.words[507] & (1 << pad) != 0;
                p.rect(x, y, 74, 16, if active { GOLD } else { PANEL });
                label(
                    p,
                    x + 3,
                    y + 1,
                    68,
                    if active {
                        BG
                    } else if enabled {
                        INK
                    } else {
                        DIM
                    },
                    &if enabled {
                        format!(
                            "{:02}  {:.0}ms",
                            pad + 1,
                            f64::from(s.slice(pad).end - s.slice(pad).start)
                                / s.words[1].max(1) as f64
                                * 1000.
                        )
                    } else {
                        format!("{:02}  --", pad + 1)
                    },
                );
            }
        }
        Page::Source => {
            label(
                p,
                8,
                56,
                305,
                GOLD,
                &format!("TRACK {:02} / CHOOSE A SOURCE", ui.selected + 1),
            );
            for (i, (name, detail)) in [
                ("AUDIO", "REC / WAV"),
                ("SYNTH", "NOTES / CHORD"),
                ("DRUM", "KIT / PADS"),
            ]
            .iter()
            .enumerate()
            {
                let x = 5 + i as i32 * 105;
                p.rect(x, 86, 100, 88, PANEL);
                p.text(x + 4, 91, 92, 20, 14., GOLD, &format!("PAD {}", i + 1));
                p.text(x + 4, 116, 92, 24, 19., INK, name);
                p.text(x + 4, 145, 92, 18, 12., DIM, detail);
            }
        }
        Page::DrumSound => {
            p.text(
                7,
                55,
                165,
                23,
                18.,
                INK,
                &format!("{:02} {}", ui.sound + 1, DRUM_NAMES[ui.sound]),
            );
            p.text(
                175,
                56,
                138,
                22,
                17.,
                GOLD,
                &format!("TOTAL {:.0}%", s.drum(ui.sound).level * 100.),
            );
            for layer in 0..2 {
                let y = 82 + layer as i32 * 49;
                let cfg = s.layer(ui.sound, layer);
                let active = layer == ui.layer;
                p.rect(5, y, 310, 44, if active { GOLD } else { PANEL });
                p.rect(7, y + 2, 306, 40, PANEL);
                p.text(
                    12,
                    y + 3,
                    180,
                    18,
                    14.,
                    if active { GOLD } else { DIM },
                    &format!("L{}  {}", layer + 1, if cfg.enabled { "ON" } else { "OFF" }),
                );
                p.text(
                    219,
                    y + 3,
                    89,
                    18,
                    14.,
                    DIM,
                    if cfg.reverse { "REV ON" } else { "REV OFF" },
                );
                p.text(
                    12,
                    y + 21,
                    293,
                    22,
                    18.,
                    INK,
                    &s.layer_name(ui.sound, layer),
                );
            }
        }
        Page::Loop => {
            let b = 16 + ui.selected * 12;
            let recording = s.words[9] == ui.selected as u64 + 1;
            let length = if recording && s.words[11] > s.words[10] {
                s.words[11] - s.words[10]
            } else {
                s.words[b + 4]
            };
            let kind = match s.track_kind(ui.selected) {
                Kind::Audio => "AUDIO",
                Kind::Synth => "SYNTH",
                Kind::Drum => "DRUM",
            };
            p.rect(5, 53, 310, 49, PANEL);
            p.text(
                10,
                54,
                200,
                21,
                17.,
                GOLD,
                &format!("{:02} {} / {}", ui.selected + 1, kind, status(s.words[b])),
            );
            p.text(
                218,
                54,
                90,
                21,
                17.,
                GOLD,
                &if length > 0 {
                    format!("{:.0} BAR", length as f64 / (4 * PPQ) as f64)
                } else {
                    "FREE".into()
                },
            );
            let action = if s.words[9] != 0 && !recording {
                format!("REC BUSY: TRACK {:02}", s.words[9])
            } else if recording && s.words[8] < s.words[10] {
                format!(
                    "COUNT-IN: {} BEATS",
                    (s.words[10] - s.words[8]).div_ceil(PPQ)
                )
            } else if recording && s.words[12] & 2 != 0 {
                "REC ENDS AT LOOP END".into()
            } else if recording && s.words[12] & 1 != 0 {
                "LOOP REC / REC: FINISH".into()
            } else if recording {
                "REC: SET LOOP LENGTH".into()
            } else {
                "REC: START / SHIFT+PAD: PLAY".into()
            };
            p.text(
                10,
                77,
                298,
                20,
                18.,
                if recording { RED } else { INK },
                if s.words[7] != 0 {
                    view_error(s.words[7], ui.page)
                } else if !ui.notice.is_empty() {
                    ui.notice
                } else {
                    &action
                },
            );
            let phase = if recording {
                s.words[8].saturating_sub(s.words[10])
            } else {
                s.words[b + 5]
            };
            let span = length.max(4 * PPQ);
            p.rect(10, 98, 298, 2, BG);
            p.rect(
                10,
                98,
                ((phase % span) as f64 / span as f64 * 298.) as i32,
                2,
                GOLD,
            );
            for pad in 0..16 {
                let bb = 16 + pad * 12;
                let x = 5 + (pad % 4) as i32 * 79;
                let y = 108 + (3 - pad / 4) as i32 * 22;
                let active = pad == ui.selected;
                let rec = matches!((s.words[bb] >> 8) & 255, 2..=4);
                p.rect(
                    x,
                    y,
                    74,
                    20,
                    if rec {
                        RED
                    } else if active {
                        GOLD
                    } else {
                        PANEL
                    },
                );
                p.text(
                    x + 3,
                    y + 1,
                    68,
                    19,
                    18.,
                    if active || rec { BG } else { INK },
                    &format!(
                        "{:02} {}",
                        pad + 1,
                        match (s.words[bb] >> 8) & 255 {
                            0 => "--",
                            2 => "WAIT",
                            3 | 4 => "REC",
                            5 => "PLAY",
                            6 => "STOP",
                            _ => "ARM",
                        }
                    ),
                );
            }
        }
        Page::Mix => {
            p.text(
                7,
                55,
                210,
                24,
                18.,
                INK,
                &format!("{:02} {}", ui.selected + 1, s.name(ui.selected)),
            );
            p.text(
                245,
                54,
                68,
                25,
                21.,
                GOLD,
                &format!("{:.0}%", s.mix(ui.selected).level * 100.),
            );
            for pad in 0..16 {
                let b = 16 + pad * 12;
                let m = s.mix(pad);
                let name = s.name(pad);
                let state = if s.words[b + 10] != 0 && ((s.words[b] >> 8) & 255) == 5 {
                    "DICE"
                } else if m.mute {
                    "MUTE"
                } else if m.solo {
                    "SOLO"
                } else {
                    status(s.words[b])
                };
                let subtitle = if ui.page == Page::Mix {
                    format!("{:3.0}% {}", m.level * 100., state)
                } else {
                    let bars = s.words[b + 4] as f64 / (PPQ * 4) as f64;
                    format!(
                        "{} {}",
                        state,
                        if bars > 0. {
                            format!("{bars:.0}b")
                        } else {
                            String::new()
                        }
                    )
                };
                let kind = match s.track_kind(pad) {
                    Kind::Audio => "AUDIO",
                    Kind::Synth => "SYNTH",
                    Kind::Drum => "DRUM",
                };
                let title = if name.starts_with("TRACK ") {
                    format!("{:02} {kind}", pad + 1)
                } else {
                    format!("{:02} {name}", pad + 1)
                };
                let _ = subtitle;
                mixer_cell(p, pad, &title, pad == ui.selected, m.level, m.mute, m.solo);
            }
        }
        Page::KitMix => {
            p.text(
                7,
                55,
                210,
                24,
                18.,
                INK,
                &format!("{:02} {}", ui.sound + 1, DRUM_NAMES[ui.sound]),
            );
            p.text(
                245,
                54,
                68,
                25,
                21.,
                GOLD,
                &format!("{:.0}%", s.drum(ui.sound).level * 100.),
            );
            for (pad, name) in DRUM_NAMES.iter().enumerate() {
                let m = s.drum(pad);
                mixer_cell(p, pad, name, pad == ui.sound, m.level, m.mute, m.solo);
            }
        }
        Page::Play if s.track_kind(ui.selected) == Kind::Synth && ui.synth_page > 0 => {
            synth_panel(ui, s, p);
        }
        Page::Play => {
            for pad in 0..16 {
                let drum = s.track_kind(ui.selected) == Kind::Drum;
                let title = if drum {
                    DRUM_NAMES[pad].to_owned()
                } else if ui.chord_modes[ui.selected] {
                    s.chord().label(pad)
                } else {
                    const N: [&str; 12] = [
                        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
                    ];
                    format!("{}{}", N[pad % 12], 3 + pad / 12)
                };
                cell(
                    p,
                    pad,
                    &title,
                    &if drum && s.words[787] & (1 << pad) != 0 {
                        format!("{:02} SAMPLE", pad + 1)
                    } else if drum && s.words[481] & (1 << pad) != 0 {
                        format!("{:02} AUTO", pad + 1)
                    } else {
                        format!("PAD {:02}", pad + 1)
                    },
                    ui.held[pad].is_some() || ui.held_chords[pad].is_some(),
                    None,
                    DIM,
                )
            }
        }
        Page::TrackFx | Page::MasterFx => {
            let mut names = [
                "FILTER",
                "ISOLATE",
                "REPEAT",
                "REVERSE",
                "TAPE STOP",
                "DUB ECHO",
                "CRUSH",
                "WOBBLE",
                "SLOT 1",
                "SLOT 2",
                "LATCH",
                "FX OFF",
                "DRY",
                "BUS A",
                "BUS B",
                "SETUP",
            ];
            let target = if ui.page == Page::MasterFx {
                ui.master_target + 1
            } else {
                0
            };
            if ui.shift_view {
                names[0] = "DRIVE";
                names[1] = "COMP";
                names[2] = "LOFI";
                names[3] = "FILTER";
            }
            names[10] = if ui.latch { "TOGGLE" } else { "HOLD" };
            if target >= 2 {
                names[9] = "ONE SLOT";
            }
            if target != 0 {
                names[12] = "MASTER";
            }
            let count = if target >= 2 { 1 } else { 2 };
            for (pad, name) in names.iter().enumerate() {
                cell(
                    p,
                    pad,
                    name,
                    if pad < 8 {
                        let effect = if ui.shift_view {
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
                            crate::ui::PERFORMANCE[pad]
                        };
                        if (0..count).any(|i| s.fx(fx_base(target, i)).effect == effect) {
                            "ON"
                        } else if ui.latch {
                            "OFF"
                        } else {
                            "HOLD"
                        }
                    } else if pad == 8 || pad == 9 {
                        effect_name(s.fx(fx_base(target, pad - 8)).effect)
                    } else {
                        ""
                    },
                    (pad == 8 + ui.fx_slot)
                        || (pad == 10 && ui.latch)
                        || (pad < 8 && {
                            let effect = if ui.shift_view {
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
                                crate::ui::PERFORMANCE[pad]
                            };
                            (0..count).any(|i| s.fx(fx_base(target, i)).effect == effect)
                        }),
                    None,
                    DIM,
                )
            }
        }
        Page::Setup => {
            let names = [
                "AUDIO",
                "SYNTH",
                "DRUM",
                "CLEAR",
                "PRE OFF",
                "PRE 1 BAR",
                "PRE 2 BAR",
                "SAVE",
                "LOAD",
                "RESAMPLE",
                "STOP",
                "PLAY ALL",
                "LOAD WAV",
                "RE-SLICE",
                "",
                "EXIT",
            ];
            for (pad, name) in names.iter().enumerate() {
                cell(
                    p,
                    pad,
                    name,
                    if pad == 3 { "ERASE + PAD" } else { "" },
                    false,
                    None,
                    DIM,
                )
            }
        }
        Page::Grid => {
            let x = 27;
            let y = 53;
            let width = 286;
            let height = 128;
            for i in 0..=16 {
                let xx = x + i * width / 16;
                p.rect(xx, y, 1, height, if i % 4 == 0 { DIM } else { PANEL });
            }
            for row in 0..=8 {
                p.rect(x, y + row * 16, width, 1, PANEL)
            }
            let start = ui.grid_bar * 4 * PPQ;
            let drum = s.track_kind(ui.selected) == Kind::Drum;
            let center = if drum {
                ui.sound as i32
            } else {
                (s.words[353 + ui.grid_event.min(63) * 2] & 255) as i32
            };
            for row in 0..8 {
                let n = (center + 3 - row).clamp(0, 127);
                label(p, 2, y + row * 16, 25, DIM, &n.to_string());
            }
            let count = s.words[16 + ui.selected * 12 + 6].min(64) as usize;
            for i in 0..count {
                let tick = s.words[352 + i * 2];
                let flags = s.words[353 + i * 2];
                if flags & (1 << 16) == 0 || tick < start || tick >= start + 4 * PPQ {
                    continue;
                }
                let note = (flags & 255) as i32;
                let row = center + 3 - note;
                if !(0..8).contains(&row) {
                    continue;
                }
                let off = ((i + 1)..count)
                    .find(|j| {
                        let f = s.words[353 + j * 2];
                        f & (1 << 16) == 0 && f & 255 == flags & 255
                    })
                    .map_or(tick + PPQ / 4, |j| s.words[352 + j * 2]);
                let xx = x + ((tick - start) as f64 / (4 * PPQ) as f64 * f64::from(width)) as i32;
                let len = ((off.saturating_sub(tick)) as f64 / (4 * PPQ) as f64 * f64::from(width))
                    as i32;
                p.rect(
                    xx,
                    y + row * 16 + 3,
                    len.max(2).min(x + width - xx),
                    10,
                    if i == ui.grid_event { GOLD } else { DIM },
                );
            }
            let b = 16 + ui.selected * 12;
            let playhead = if s.words[9] == ui.selected as u64 + 1 && s.words[8] >= s.words[10] {
                Some(s.words[8] - s.words[10])
            } else if (s.words[b] >> 8) & 255 == 5 && s.words[4] & 1 != 0 {
                Some(s.words[b + 5])
            } else {
                None
            };
            if let Some(tick) = playhead
                && tick >= start
                && tick < start + 4 * PPQ
            {
                let xx = x + ((tick - start) as f64 / (4 * PPQ) as f64 * f64::from(width)) as i32;
                p.rect(xx, y, 1, height, GOLD);
            }
        }
    }
    let title = match ui.page {
        Page::Loop => "LOOP",
        Page::Play => "PLAY",
        Page::Grid => "GRID",
        Page::Mix => "MIX",
        Page::KitMix => "KIT MIX",
        Page::TrackFx => "TRACK FX",
        Page::MasterFx => "MASTER FX",
        Page::Setup => "PROJECT",
        Page::Chop => "CHOP",
        Page::Source => "SOURCE",
        Page::Output => "OUTPUT",
        Page::DrumSound => "LAYERS",
        Page::Sidechain => "SIDECHAIN",
        Page::Texture => "TEXTURE",
    };
    let fx_target = if ui.page == Page::MasterFx {
        ui.master_target + 1
    } else {
        0
    };
    let current_fx = s.fx(fx_base(fx_target, ui.fx_slot));
    let controls = match ui.page {
        Page::Sidechain => "K1 DUCK     K2 RELEASE  K3 SENSE",
        Page::Texture => match ui.texture_slot {
            0 => "K1 GRAIN    K2 TONE     K3 MIX",
            1 => "K1 DRIVE    K2 TONE     K3 MIX",
            2 => "K1 SQUEEZE  K2 RELEASE  K3 MIX",
            3 => "K1 RATE     K2 DEPTH    K3 MIX",
            4 => "K1 DIV      K2 FEEDBACK K3 MIX",
            5 => "K1 DUCK     K2 RELEASE  K3 SENSE",
            _ => fx_controls(
                s.fx(fx_base(1, ui.texture_slot.saturating_sub(6).min(1)))
                    .effect,
            ),
        },
        Page::Output => "K1 OUTPUT    K2 --       K3 --",
        Page::Source => "K1 --       K2 --       K3 --",
        Page::DrumSound if ui.layer_detail => "K1 DECAY    K2 ON/OFF   K3 REV",
        Page::DrumSound => "K1 VOLUME   K2 TUNE     K3 SHIFT",
        Page::Chop if ui.shift_view && ((s.words[506] >> 8) & 255) != 3 => {
            match (s.words[506] >> 8) & 255 {
                0 => "K1 SLICES   K2 --       K3 --",
                1 => "K1 SENS     K2 GAP      K3 MAX",
                _ => "K1 GRID     K2 --       K3 --",
            }
        }
        Page::Chop => "K1 START    K2 END      K3 LEVEL",
        Page::Play if s.track_kind(ui.selected) == Kind::Synth && ui.synth_page > 0 => {
            match ui.synth_page {
                7 => "K1 PAN     K2 PAN DEPTH K3 RATE",
                6 if ui.shift_view => "K1 SHAPE   K2 PITCH    K3 VOLUME",
                6 => "K1 RATE    K2 PITCH    K3 FILTER",
                1 | 2 if ui.shift_view => "K1 WAVE    K2 FINE      K3 LEVEL",
                1 | 2 => "K1 WAVE    K2 TUNE      K3 LEVEL",
                3 if ui.shift_view => "K1 CUTOFF  K2 ENV AMT   K3 TYPE",
                3 => "K1 CUTOFF  K2 RESO      K3 TYPE",
                _ if ui.shift_view => "K1 ATTACK  K2 DECAY     K3 RELEASE",
                _ => "K1 ATTACK  K2 DECAY     K3 SUSTAIN",
            }
        }
        Page::Play if ui.chord_modes[ui.selected] && s.track_kind(ui.selected) == Kind::Synth => {
            if ui.shift_view {
                "K1 OCTAVE   K2 VOICING  K3 RELEASE"
            } else {
                "K1 KEY      K2 SCALE    K3 COLOR"
            }
        }

        Page::Grid => "K1 PITCH   K2 LENGTH   K3 VELOCITY",
        Page::Play if s.track_kind(ui.selected) == Kind::Drum => "K1 TUNE    K2 DECAY     K3 LEVEL",
        Page::Play if ui.shift_view => "K1 WAVE    K2 DETUNE    K3 OCTAVE",
        Page::Play => "K1 CUTOFF   K2 ATTACK   K3 RELEASE",
        Page::Setup => "K1 BPM    K2 COUNT-IN    ERASE+CLEAR",
        Page::TrackFx | Page::MasterFx => fx_controls(current_fx.effect),
        _ => "K1 PAN     K2 DELAY     K3 REVERB",
    };
    let cuts = [
        0,
        controls.find("K2").unwrap_or(controls.len()),
        controls
            .find("K3")
            .or_else(|| controls.find("ERASE"))
            .unwrap_or(controls.len()),
        controls.len(),
    ];
    for i in 0..3 {
        if cuts[i] <= cuts[i + 1] {
            let x = 5 + i as i32 * 105;
            p.rect(x, 200, 100, 39, PANEL);
            p.text(
                x + 4,
                201,
                92,
                17,
                14.,
                INK,
                controls[cuts[i]..cuts[i + 1]].trim(),
            );
        }
    }
    let values: [String; 3] = match ui.page {
        Page::Sidechain => {
            let c = s.sidechain();
            [
                format!("{:.0}%", c.depth * 100.),
                format!(
                    "{:.0}ms{}",
                    c.release_seconds(s.bpm() as f32) * 1000.,
                    if c.sync { "S" } else { "" }
                ),
                format!("{:.0}%", (1. - (c.threshold - 0.01) / 0.49) * 100.),
            ]
        }
        Page::Texture => {
            if ui.texture_slot < 5 {
                let v = s.texture().params[ui.texture_slot];
                let mut values = v.map(|v| format!("{:.0}%", v * 100.));
                if ui.texture_slot == 2 {
                    values[1] = format!("{:.0}ms", (0.04 + v[1] * 0.8) * 1000.);
                }
                if ui.texture_slot == 3 {
                    values[0] = format!("{:.2}Hz", 0.2 + v[0] * 4.);
                }
                if ui.texture_slot == 4 {
                    values[0] = ["1/16", "1/8", "1/8 D", "1/4"][(v[0] * 3.99) as usize].into();
                }
                values
            } else if ui.texture_slot == 5 {
                let c = s.sidechain();
                [
                    format!("{:.0}%", c.depth * 100.),
                    format!("{:.0}ms", c.release_seconds(s.bpm() as f32) * 1000.),
                    format!("{:.0}%", (1. - (c.threshold - 0.01) / 0.49) * 100.),
                ]
            } else {
                let fx = s.fx(fx_base(1, ui.texture_slot - 6));
                [
                    format!("{:.0}%", fx.a * 100.),
                    format!("{:.0}%", fx.b * 100.),
                    format!("{:.0}%", fx.mix * 100.),
                ]
            }
        }
        Page::Output => [
            s.output(ui.selected).label().into(),
            String::new(),
            String::new(),
        ],
        Page::Source => [String::new(), String::new(), String::new()],
        Page::DrumSound => {
            let cfg = s.layer(ui.sound, ui.layer);
            if ui.layer_detail {
                [
                    format!("{:.0} ms", cfg.decay * 1000.),
                    if cfg.enabled {
                        "ON".into()
                    } else {
                        "OFF".into()
                    },
                    if cfg.reverse {
                        "ON".into()
                    } else {
                        "OFF".into()
                    },
                ]
            } else {
                [
                    format!("{:.0}%", cfg.level * 100.),
                    format!("{:+.0} st", cfg.tune),
                    format!("{:+.0} ms", cfg.shift),
                ]
            }
        }
        Page::Chop if ui.shift_view && ((s.words[506] >> 8) & 255) != 3 => {
            match (s.words[506] >> 8) & 255 {
                0 => [s.words[772].to_string(), String::new(), String::new()],
                1 => [
                    format!("{:.0}%", f32::from_bits(s.words[771] as u32) * 100.),
                    format!("{:.0} ms", f32::from_bits((s.words[771] >> 32) as u32)),
                    s.words[772].to_string(),
                ],
                _ => [
                    crate::chop::GRID_NAMES[s.words[773].min(10) as usize].into(),
                    String::new(),
                    String::new(),
                ],
            }
        }
        Page::Chop => {
            let slice = s.slice(ui.chop_slice.min(15));
            let rate = s.words[1].max(1) as f64;
            [
                format!("{:.0} ms", f64::from(slice.start) / rate * 1000.),
                format!("{:.0} ms", f64::from(slice.end) / rate * 1000.),
                format!("{:.0}%", slice.level * 100.),
            ]
        }
        Page::Play if s.track_kind(ui.selected) == Kind::Synth && ui.synth_page > 0 => {
            let t = if ui.synth_page == 5 {
                s.synth().filter_adsr()
            } else {
                s.synth()
            };
            match ui.synth_page {
                7 => [
                    format!("{:+.0}", s.mix(ui.selected).pan * 100.),
                    format!("{:.0}%", t.lfo_pan * 100.),
                    format!("{:.2} Hz", t.lfo_rate),
                ],
                6 => [
                    if ui.shift_view {
                        crate::synth::LFO_SHAPES[usize::from(t.lfo_shape)].into()
                    } else {
                        format!("{:.2} Hz", t.lfo_rate)
                    },
                    format!("{:.0} ct", t.lfo_pitch),
                    if ui.shift_view {
                        format!("{:.0}%", t.lfo_volume * 100.)
                    } else {
                        format!("{:.2} oct", t.lfo_filter)
                    },
                ],
                1 | 2 => {
                    let second = ui.synth_page == 2;
                    [
                        WAVES[usize::from(if second { t.wave2 } else { t.wave })].into(),
                        if ui.shift_view {
                            format!("{:+.0} ct", if second { t.fine2 } else { t.fine1 })
                        } else {
                            format!("{:+.0} st", if second { t.tune2 } else { t.tune1 })
                        },
                        format!("{:.0}%", 100. * if second { t.level2 } else { t.level1 }),
                    ]
                }
                3 => [
                    format!("{:.0} Hz", t.cutoff),
                    if ui.shift_view {
                        format!("{:+.0}%", 100. * t.env_amount)
                    } else {
                        format!("{:.0}%", 100. * t.resonance)
                    },
                    FILTERS[usize::from(t.filter)].into(),
                ],
                _ => [
                    format!("{:.0} ms", 1000. * t.attack),
                    format!("{:.0} ms", 1000. * t.decay),
                    if ui.shift_view {
                        format!("{:.0} ms", 1000. * t.release)
                    } else {
                        format!("{:.0}%", 100. * t.sustain)
                    },
                ],
            }
        }
        Page::Play if ui.chord_modes[ui.selected] && s.track_kind(ui.selected) == Kind::Synth => {
            let c = s.chord();
            if ui.shift_view {
                [
                    format!("{:+} oct", c.octave),
                    if c.open {
                        "OPEN".into()
                    } else {
                        "CLOSE".into()
                    },
                    format!("{:.0} ms", s.synth().release * 1000.),
                ]
            } else {
                [
                    [
                        "C", "C#", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B",
                    ][usize::from(c.key.min(11))]
                    .into(),
                    ["MAJOR", "MINOR", "DORIAN"][usize::from(c.scale.min(2))].into(),
                    ["TRIAD", "7TH", "9TH"][usize::from(c.color.min(2))].into(),
                ]
            }
        }

        Page::TrackFx | Page::MasterFx => [
            format!("{:.0}%", current_fx.a * 100.),
            format!("{:.0}%", current_fx.b * 100.),
            format!("{:.0}%", current_fx.mix * 100.),
        ],
        Page::Play if s.track_kind(ui.selected) == Kind::Drum => {
            let (tune, decay) = s.drum_shape(ui.sound);
            [
                format!("{tune:+.1} st"),
                format!("{:.0} ms", decay * 1000.),
                format!("{:.0}%", s.drum(ui.sound).level * 100.),
            ]
        }
        Page::Play if ui.shift_view => {
            let t = s.synth();
            [
                ["SINE", "SAW", "SQUARE", "TRIANGLE"][usize::from(t.wave.min(3))].to_owned(),
                format!("{:.1} ct", t.detune * 1200.),
                format!("{:+} oct", t.octave),
            ]
        }
        Page::Play => {
            let t = s.synth();
            [
                format!("{:.0} Hz", t.cutoff),
                format!("{:.0} ms", t.attack * 1000.),
                format!("{:.0} ms", t.release * 1000.),
            ]
        }
        Page::Loop | Page::Mix | Page::KitMix => {
            let m = if ui.page == Page::KitMix {
                s.drum(ui.sound)
            } else {
                s.mix(ui.selected)
            };
            [
                format!("{:+.0}", m.pan * 100.),
                format!("{:.0}%", m.delay * 100.),
                format!("{:.0}%", m.room * 100.),
            ]
        }
        Page::Setup => [
            format!("{:.0}", s.bpm()),
            format!("{} bar", (s.words[4] >> 8) & 255),
            String::new(),
        ],
        _ => [String::new(), String::new(), String::new()],
    };
    for (i, value) in values.iter().enumerate() {
        p.text(9 + i as i32 * 105, 217, 92, 21, 19., GOLD, value);
    }
    let target = if ui.page == Page::MasterFx {
        "MASTER"
    } else if ui.page == Page::KitMix {
        "SOUND"
    } else {
        "TRACK"
    };
    let level = if ui.page == Page::KitMix {
        s.drum(ui.sound).level
    } else {
        s.level(if ui.page == Page::MasterFx {
            16
        } else {
            ui.selected
        })
    };
    let err = view_error(s.words[7], ui.page);
    let footer = if ui.page == Page::Texture && err.is_empty() {
        "PAD: ON/OFF / DATA: SELECT > SC".into()
    } else if ui.page == Page::Sidechain && err.is_empty() {
        format!(
            "SHIFT+B2: {} / DATA PUSH: FX",
            if s.sidechain().sync { "SYNC" } else { "FREE" }
        )
    } else if ui.page == Page::DrumSound && err.is_empty() {
        if ui.notice.is_empty() {
            "DATA: SAMPLE / +/-: DETAIL".into()
        } else {
            ui.notice.into()
        }
    } else if ui.page == Page::Source && err.is_empty() {
        "PAD 1:AUDIO 2:SYNTH 3:DRUM".into()
    } else if ui.page == Page::Chop && err.is_empty() {
        format!(
            "SL{:02} {} / {:.0}ms / Z{}",
            ui.chop_slice + 1,
            if s.words[506] & 1 != 0 {
                "GATE"
            } else {
                "SHOT"
            },
            f64::from(ui.chop_cursor) / s.words[1].max(1) as f64 * 1000.,
            1u32 << ui.chop_zoom
        )
    } else if s.words[9] != 0 && s.words[8] < s.words[10] {
        format!(
            "COUNT-IN: {} BEATS",
            (s.words[10] - s.words[8]).div_ceil(PPQ)
        )
    } else if !err.is_empty() {
        err.to_owned()
    } else if s.words[9] != 0 && matches!(ui.page, Page::Loop | Page::Play) {
        let track = (s.words[9] - 1) as usize;
        let active_mode = (s.words[12] >> 2) & 1;
        let pending_mode = (s.words[16 + track * 12] >> 18) & 1;
        if active_mode != pending_mode {
            "REC MODE: NEXT LOOP".to_owned()
        } else if s.words[12] & 2 != 0 {
            "REC ENDS AT LOOP END".to_owned()
        } else if s.words[12] & 1 != 0 {
            "LOOP REC / REC: FINISH".to_owned()
        } else if s.words[11] != 0 {
            "LENGTH SET / LOOP REC".to_owned()
        } else {
            "FIRST TAKE / REC: SET LENGTH".to_owned()
        }
    } else if ui.page == Page::Play
        && s.track_kind(ui.selected) == Kind::Synth
        && ui.synth_page == 0
    {
        "+/-: SYNTH EDIT / PADS: PLAY".to_owned()
    } else if !ui.notice.is_empty() {
        ui.notice.to_owned()
    } else if ui.page == Page::Output {
        "USB STEREO / MASTER FX ON BOTH".into()
    } else if ui.page == Page::Mix {
        format!(
            "DATA: OUTPUT / T{:02} {}",
            ui.selected + 1,
            s.output(ui.selected).label()
        )
    } else if ui.page == Page::MasterFx {
        format!(
            "{} / DATA: TEXTURE > SIDECHAIN",
            ["MASTER", "BUS A", "BUS B"][ui.master_target.min(2) as usize]
        )
    } else if ui.page == Page::Grid {
        format!("BAR {} / DATA: NOTE / +/-: BAR", ui.grid_bar + 1)
    } else if ui.page == Page::Play && s.track_kind(ui.selected) == Kind::Drum {
        let rhythm = s.words[509].min(23) as usize;
        format!(
            "{:02} {} / {} {}",
            ui.sound + 1,
            DRUM_NAMES[ui.sound],
            [
                "1/4", "1/8", "1/16", "1/8D", "1/16D", "1/4T", "1/8T", "1/16T"
            ][rhythm / 3],
            ["", "FILL", "ROLL"][rhythm % 3]
        )
    } else if ui.page == Page::Loop && s.words[16 + ui.selected * 12 + 4] == 0 {
        "SHIFT+DATA: CHOOSE INSTRUMENT".to_owned()
    } else {
        format!(
            "{title} {:02} / {target} {:.0}% {}",
            ui.selected + 1,
            level * 100.,
            if ui.pickup.picked { "" } else { "PICKUP" }
        )
    };
    if ui.page != Page::Loop {
        p.text(
            6,
            if matches!(ui.page, Page::Mix | Page::KitMix) {
                185
            } else {
                182
            },
            308,
            18,
            14.,
            if err.is_empty() { DIM } else { RED },
            &footer,
        );
    }
}

fn fx_controls(effect: Effect) -> &'static str {
    match effect {
        Effect::Filter => "K1 CUTOFF   K2 RESO     K3 MIX",
        Effect::Compressor => "K1 THRESH   K2 RATIO    K3 MIX",
        Effect::Repeat => "K1 DIV      K2 SMOOTH   K3 MIX",
        Effect::Reverse => "K1 LENGTH   K2 TONE     K3 MIX",
        Effect::TapeStop => "K1 TIME     K2 TONE     K3 MIX",
        Effect::DubEcho => "K1 DIV      K2 FEEDBACK K3 MIX",
        Effect::Wobble => "K1 RATE     K2 DEPTH    K3 MIX",
        Effect::LoFi | Effect::Crush => "K1 RATE     K2 BITS     K3 MIX",
        Effect::Isolator => "K1 LOW      K2 HIGH     K3 MIX",
        Effect::Drive => "K1 DRIVE    K2 TONE     K3 MIX",
        _ => "K1 AMOUNT   K2 TONE     K3 MIX",
    }
}

const WAVES: [&str; 4] = ["SINE", "SAW", "SQUARE", "TRIANGLE"];
const FILTERS: [&str; 3] = ["LOW-PASS", "BAND-PASS", "HIGH-PASS"];
fn synth_panel(ui: &Ui, s: &Snapshot, p: &mut impl Painter) {
    let page = ui.synth_page.min(7);
    let t = if page == 5 {
        s.synth().filter_adsr()
    } else {
        s.synth()
    };
    p.rect(5, 53, 310, 128, PANEL);
    label(
        p,
        12,
        54,
        295,
        GOLD,
        &format!(
            "-  {}  {}/8  +",
            [
                "PLAY",
                "OSC 1",
                "OSC 2",
                "FILTER",
                "AMP ADSR",
                "FILTER ADSR",
                "LFO",
                "PAN / LFO"
            ][usize::from(page)],
            page + 1
        ),
    );
    if page == 7 {
        p.text(
            16,
            80,
            280,
            24,
            19.,
            INK,
            &format!("PAN {:+.0}", s.mix(ui.selected).pan * 100.),
        );
        p.rect(18, 118, 284, 3, DIM);
        let x = 18 + ((hi_pan(s) + 1.) * 142.) as i32;
        p.rect(x.clamp(18, 296), 111, 6, 17, GOLD);
        p.text(
            16,
            142,
            280,
            22,
            18.,
            INK,
            &format!("LFO {:.0}%  /  {:.2} Hz", t.lfo_pan * 100., t.lfo_rate),
        );
    } else if page == 6 {
        for (i, (name, value)) in [
            (
                "SHAPE",
                crate::synth::LFO_SHAPES[usize::from(t.lfo_shape)].to_owned(),
            ),
            ("RATE", format!("{:.2} Hz", t.lfo_rate)),
            ("PITCH DEPTH", format!("{:.0} ct", t.lfo_pitch)),
            ("FILTER DEPTH", format!("{:.2} oct", t.lfo_filter)),
            ("VOLUME DEPTH", format!("{:.0}%", t.lfo_volume * 100.)),
        ]
        .iter()
        .enumerate()
        {
            let y = 74 + i as i32 * 21;
            p.text(16, y, 141, 21, 16., DIM, name);
            p.text(163, y, 141, 21, 19., INK, value);
        }
    } else if page >= 4 {
        let total = t.attack + t.decay + 0.5 + t.release;
        let xs = [
            15.,
            15. + 290. * t.attack / total,
            15. + 290. * (t.attack + t.decay) / total,
            15. + 290. * (t.attack + t.decay + 0.5) / total,
            305.,
        ];
        let ys = [
            124.,
            78.,
            124. - 46. * t.sustain,
            124. - 46. * t.sustain,
            124.,
        ];
        for n in 0..4 {
            let steps = (xs[n + 1] - xs[n])
                .abs()
                .max((ys[n + 1] - ys[n]).abs())
                .ceil()
                .max(1.) as i32;
            for step in 0..=steps {
                let frac = step as f32 / steps as f32;
                let x = xs[n] + (xs[n + 1] - xs[n]) * frac;
                let y = ys[n] + (ys[n + 1] - ys[n]) * frac;
                p.rect(x as i32, y as i32, 1, 2, GOLD);
            }
        }
        for (i, (name, value)) in [
            ("ATTACK", format!("{:.0}ms", t.attack * 1000.)),
            ("DECAY", format!("{:.0}ms", t.decay * 1000.)),
            ("SUSTAIN", format!("{:.0}%", t.sustain * 100.)),
            ("RELEASE", format!("{:.0}ms", t.release * 1000.)),
        ]
        .iter()
        .enumerate()
        {
            p.text(11 + i as i32 * 77, 137, 72, 18, 15., DIM, name);
            p.text(11 + i as i32 * 77, 156, 72, 23, 19., INK, value);
        }
    } else {
        let entries = if page == 3 {
            vec![
                ("TYPE", FILTERS[usize::from(t.filter)].into()),
                ("CUTOFF", format!("{:.0} Hz", t.cutoff)),
                ("RESONANCE", format!("{:.0}%", t.resonance * 100.)),
                ("ENV AMOUNT", format!("{:+.0}%", t.env_amount * 100.)),
            ]
        } else {
            let second = page == 2;
            vec![
                (
                    "WAVE",
                    WAVES[usize::from(if second { t.wave2 } else { t.wave })].into(),
                ),
                (
                    "TUNE",
                    format!("{:+.0} st", if second { t.tune2 } else { t.tune1 }),
                ),
                (
                    "FINE",
                    format!("{:+.0} ct", if second { t.fine2 } else { t.fine1 }),
                ),
                (
                    "LEVEL",
                    format!("{:.0}%", 100. * if second { t.level2 } else { t.level1 }),
                ),
            ]
        };
        for (i, (name, value)) in entries.iter().enumerate() {
            let y = 78 + i as i32 * 24;
            p.text(16, y, 141, 21, 16., DIM, name);
            p.text(163, y, 141, 21, 19., INK, value);
        }
    }
}

fn view_error(code: u64, page: Page) -> &'static str {
    match code {
        0 => "",
        1 => "INVALID TRACK",
        2 => "INVALID VALUE",
        3 => "RECORDER BUSY",
        4 => "TRACK NOT EMPTY",
        5 => "NO CONTENT",
        6 if page == Page::Chop => "MAX 16 SLICES - USE A LONGER GRID",
        6 => "CAPACITY LIMIT",
        7 => "CLEAR LOOPS TO CHANGE BPM",
        8 => "CHOOSE SYNTH OR DRUM",
        _ => "NO INPUT - CHECK SOURCE",
    }
}

fn output_panel(ui: &Ui, s: &Snapshot, p: &mut impl Painter) {
    p.text(
        7,
        55,
        305,
        22,
        18.,
        INK,
        &format!(
            "TRACK {:02} / {}",
            ui.selected + 1,
            s.output(ui.selected).label()
        ),
    );
    for (i, target) in [OutputTarget::Main, OutputTarget::Usb, OutputTarget::Both]
        .into_iter()
        .enumerate()
    {
        let x = 5 + i as i32 * 105;
        let active = s.output(ui.selected) == target;
        p.rect(x, 82, 100, 50, if active { GOLD } else { PANEL });
        p.text(
            x + 3,
            85,
            94,
            17,
            13.,
            if active { BG } else { DIM },
            &format!("PAD {}", i + 1),
        );
        p.text(
            x + 3,
            105,
            94,
            23,
            17.,
            if active { BG } else { INK },
            target.label(),
        );
    }
    for (i, (name, peak)) in [
        (
            "MAIN",
            f32::from_bits((s.words[5] >> 32) as u32).max(f32::from_bits(s.words[6] as u32)),
        ),
        (
            "USB",
            f32::from_bits(s.words[1024] as u32).max(f32::from_bits((s.words[1024] >> 32) as u32)),
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let x = 9 + i as i32 * 157;
        p.text(x, 140, 145, 20, 15., DIM, name);
        p.rect(x, 165, 145, 6, PANEL);
        p.rect(x, 165, (peak.clamp(0., 1.) * 145.) as i32, 6, GOLD);
    }
}

fn hi_pan(s: &Snapshot) -> f32 {
    f32::from_bits((s.words[1025] >> 32) as u32)
}
fn sidechain_panel(ui: &Ui, s: &Snapshot, p: &mut impl Painter) {
    let c = s.sidechain();
    let src = if c.sound == 16 || s.track_kind(usize::from(c.source)) != Kind::Drum {
        "WHOLE TRACK"
    } else {
        crate::model::DRUM_NAMES[usize::from(c.sound)]
    };
    p.text(
        8,
        54,
        303,
        23,
        18.,
        INK,
        &format!("FROM {:02} {}", c.source + 1, src),
    );
    p.text(
        8,
        80,
        303,
        22,
        17.,
        GOLD,
        [
            "TO ALL EXCEPT SOURCE",
            "TO SELECTED TRACKS",
            "TO FULL MASTER",
        ][usize::from(c.mode)],
    );
    if c.mode == 1 {
        for i in 0..16 {
            let x = 8 + (i % 8) as i32 * 38;
            let y = 108 + (i / 8) as i32 * 27;
            let on = c.mask & (1 << i) != 0 && i != usize::from(c.source);
            p.rect(x, y, 34, 23, if on { GOLD } else { PANEL });
            p.text(
                x + 2,
                y + 1,
                30,
                21,
                16.,
                if on { BG } else { DIM },
                &format!("{:02}", i + 1),
            );
        }
    } else {
        p.text(8, 108, 303, 22, 16., DIM, "DATA: source track / PAD: drum");
        p.text(8, 134, 303, 22, 16., DIM, "SHIFT+PAD: target / B2: whole");
    }
    p.rect(8, 166, 303, 10, PANEL);
    p.rect(
        8,
        166,
        (303. * s.duck_reduction().clamp(0., 1.)) as i32,
        10,
        GOLD,
    );
    let _ = ui;
}
fn texture_panel(ui: &Ui, s: &Snapshot, p: &mut impl Painter) {
    let t = s.texture();
    for pad in 0..16 {
        let title: String;
        let mut sub = String::new();
        let mut on = false;
        if pad < 5 {
            title = crate::motion::TEXTURES[pad].into();
            on = t.enabled[pad];
            sub = if on { "ON" } else { "OFF" }.into();
        } else if pad == 5 {
            title = "DUCK".into();
            on = s.sidechain().enabled && s.sidechain().depth > 0.;
            sub = if on { "ON" } else { "OFF" }.into();
        } else if pad == 6 || pad == 7 {
            let slot = pad - 6;
            let fx = s.fx(fx_base(1, slot));
            title = effect_name(if fx.effect == Effect::Off {
                if slot == 0 {
                    Effect::Filter
                } else {
                    Effect::Repeat
                }
            } else {
                fx.effect
            })
            .into();
            on = fx.effect != Effect::Off && !t.master_bypass[slot];
            sub = format!("FX{} {}", slot + 1, if on { "ON" } else { "OFF" });
        } else {
            title = match pad {
                8 => "PERF EDIT",
                9 => "SC SET",
                10 => "BUS A",
                11 => "BUS B",
                12 => "ALL OFF",
                _ => "--",
            }
            .into();
            if pad <= 12 {
                sub = "ACTION".into();
            }
        }
        if pad < 8 && ui.texture_slot == pad {
            sub.push_str(" <");
        }
        cell(
            p,
            pad,
            &title,
            &sub,
            on,
            None,
            if pad >= 8 { DIM } else { GOLD },
        );
    }
}
