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
    if ui.page == Page::Play
        && s.track_kind(ui.selected) == Kind::Audio
        && s.audio_target(ui.selected) == crate::audio::RecordTarget::Turntable
    {
        turntable_panel(ui, s, p);
        return;
    }

    p.rect(0, 0, 320, 240, BG);
    let heading = match ui.page {
        Page::Perform => "PLAY SETTINGS".to_owned(),
        Page::Loop if ui.loop_edit => "Loop Club / EDIT".into(),
        Page::Loop => format!(
            "Loop Club / {}",
            ["LOOP", "MUTE", "CHAIN"][usize::from(ui.loop_mode.min(2))]
        ),
        Page::Play => format!(
            "{} {:02}",
            match s.track_kind(ui.selected) {
                Kind::Synth => "SYNTH PLAY",
                Kind::Drum => "DRUM PLAY",
                Kind::Audio => "SAMPLE SOUND",
            },
            ui.selected + 1
        ),
        Page::Grid => if !s.note_view(ui.selected) {
            "AUDIO WAVEFORM"
        } else {
            "NOTE GRID"
        }
        .to_owned(),
        Page::Mix => "TRACK MIX".to_owned(),
        Page::KitMix => "DRUM MIX".to_owned(),
        Page::FxSettings => "FX SETTINGS".to_owned(),
        Page::TrackFx => format!(
            "T{:02} / {}",
            ui.selected + 1,
            effect_name(crate::ui::FX_PADS[ui.fx_focus.min(crate::ui::FX_PADS.len() - 1)])
        ),
        Page::MasterFx => format!(
            "{} / {}",
            ["MASTER", "BUS A", "BUS B"][ui.master_target.min(2) as usize],
            effect_name(crate::ui::FX_PADS[ui.fx_focus.min(crate::ui::FX_PADS.len() - 1)])
        ),
        Page::Setup => if ui.input_view {
            "INPUT SOURCE"
        } else {
            "PROJECT"
        }
        .to_owned(),
        Page::Chop => "CHOP".to_owned(),
        Page::Source => "TRACK SOURCE".to_owned(),
        Page::Output => "OUTPUT".to_owned(),
        Page::DrumSound => "DRUM SOUND".to_owned(),
        Page::Sidechain => "SIDECHAIN".to_owned(),
        Page::Texture => format!(
            "MASTER / {}",
            if ui.texture_slot < 5 {
                crate::motion::TEXTURES[ui.texture_slot]
            } else if ui.texture_slot == 5 {
                "DUCK"
            } else {
                ui.quick_fx(s)
                    .map_or("FX", |(_, _, fx)| effect_name(fx.effect))
            }
        ),
    };
    p.text(7, 2, 165, 23, 20., INK, &heading);
    p.text(
        182,
        5,
        133,
        19,
        15.,
        if s.level(16) == 0. || f32::from_bits(s.words[1043] as u32) == 0. {
            RED
        } else {
            GOLD
        },
        &if s.level(16) == 0. {
            "MASTER MUTED".into()
        } else if f32::from_bits(s.words[1043] as u32) == 0. {
            "MAIN VOL MUTED".into()
        } else if matches!(ui.page, Page::MasterFx | Page::Texture | Page::Sidechain)
            || (ui.page == Page::FxSettings
                && ui.fx_edit.is_some_and(|e| e.texture || e.target != 0))
        {
            format!("{:.0} BPM M{:.0}%", s.bpm(), s.level(16) * 100.)
        } else {
            format!(
                "{:.0} BPM {}",
                s.bpm(),
                if s.words[4] & 1 != 0 { "RUN" } else { "STOP" }
            )
        },
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
    if ui.page == Page::Perform {
        tabs = [
            "BACK",
            match ui.perform_section {
                0 => "VOICE >",
                1 => "CHORD >",
                _ => "TRIGGER >",
            },
            if s.track_kind(ui.selected) == Kind::Drum {
                "--"
            } else if ui.shift_view {
                if s.chord().smooth {
                    "SMOOTH ON"
                } else {
                    "SMOOTH OFF"
                }
            } else if ui.chord_modes[ui.selected] {
                "CHORD >"
            } else {
                "NOTES >"
            },
        ];
    }
    if matches!(ui.page, Page::Play | Page::DrumSound) && ui.shift_view {
        tabs[0] = "PLAY SET";
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
            "CHROMATIC >"
        };
    }
    if matches!(ui.page, Page::Loop | Page::Play) {
        tabs[2] = if s.words[16 + ui.selected * 12] & (1 << 18) == 0 {
            "REPLACE"
        } else {
            "OVERDUB"
        };
    }
    if ui.page == Page::Play && s.track_kind(ui.selected) == Kind::Audio {
        tabs[0] = match s.audio_target(ui.selected) {
            crate::audio::RecordTarget::Input => "REC: INPUT >",
            crate::audio::RecordTarget::Chromatic => {
                if ui.chord_modes[ui.selected] {
                    "REC: CHORD >"
                } else {
                    "REC: CHROM >"
                }
            }
            crate::audio::RecordTarget::Chop => "REC: CHOP >",
            crate::audio::RecordTarget::Turntable => "TURN >",
        };
        tabs[1] = "CHOP EDIT";
    }
    if ui.page == Page::Loop && ui.shift_view {
        tabs = ["LOOP", "MUTE", "CHAIN"];
    }
    if ui.page == Page::Loop && ui.loop_edit {
        tabs = ["COPY", "DELETE", "MOVE"];
    }
    if ui.page == Page::Play && s.track_kind(ui.selected) == Kind::Synth && ui.shift_view {
        if ui.synth_page >= 6 {
            tabs[2] = if s.synth().lfo_sync {
                "SYNC ON"
            } else {
                "SYNC OFF"
            };
        }
        if ui.chord_modes[ui.selected] {
            tabs[1] = if s.chord().smooth {
                "SMOOTH ON"
            } else {
                "SMOOTH OFF"
            };
        }
    }
    if ui.page == Page::Setup && ui.input_view {
        tabs = ["BACK", "SELECT", "LOOP"];
    }
    if ui.page == Page::FxSettings {
        let fx = ui.edited_fx(s);
        tabs = [
            "BACK",
            if fx.effect.modulated() {
                "SHAPE >"
            } else {
                "--"
            },
            if fx.effect.modulated() {
                if fx.modulation.sync {
                    "SYNC ON"
                } else {
                    "SYNC OFF"
                }
            } else {
                "--"
            },
        ];
    }
    for (i, title) in tabs.iter().enumerate() {
        let active = if ui.page == Page::Loop && ui.loop_edit {
            i == ui.edit_operation as usize
        } else if ui.page == Page::Loop {
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
    if matches!(ui.page, Page::TrackFx | Page::MasterFx | Page::Texture) {
        fx_quick_panel(ui, s, p);
    }
    let recording_view = s.words[9] == ui.selected as u64 + 1
        && matches!(ui.page, Page::Play | Page::DrumSound | Page::Grid);
    match ui.page {
        Page::Perform => performance_panel(ui, s, p),
        Page::FxSettings => fx_settings_panel(ui, s, p),
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
        Page::Setup if ui.input_view => {
            for (i, name) in crate::input_hw::LABELS.iter().enumerate() {
                let x = 5 + i as i32 * 105;
                let active = crate::input_hw::selected() as usize == i;
                p.rect(x, 75, 100, 88, if active { GOLD } else { PANEL });
                p.text(
                    x + 4,
                    79,
                    92,
                    20,
                    14.,
                    if active { BG } else { DIM },
                    &format!("PAD {}", i + 1),
                );
                p.text(x + 4, 105, 92, 24, 19., if active { BG } else { INK }, name);
                p.text(
                    x + 4,
                    137,
                    92,
                    20,
                    14.,
                    if active { BG } else { GOLD },
                    if active {
                        "ACTIVE"
                    } else if ui.input_choice == i {
                        "SELECT >"
                    } else {
                        ""
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
                p.rect(x, 80, 100, 66, PANEL);
                p.text(x + 4, 81, 92, 18, 14., GOLD, &format!("PAD {}", i + 1));
                p.text(x + 4, 100, 92, 24, 19., INK, name);
                p.text(x + 4, 126, 92, 18, 12., DIM, detail);
            }
            if s.track_kind(ui.selected) == Kind::Audio {
                for (i, target) in [
                    crate::audio::RecordTarget::Input,
                    crate::audio::RecordTarget::Chromatic,
                    crate::audio::RecordTarget::Chop,
                ]
                .into_iter()
                .enumerate()
                {
                    let x = 5 + i as i32 * 105;
                    let selected = s.audio_target(ui.selected) == target;
                    p.rect(x, 152, 100, 44, if selected { GOLD } else { PANEL });
                    p.text(
                        x + 4,
                        153,
                        92,
                        18,
                        13.,
                        if selected { BG } else { DIM },
                        &format!("PAD {} / REC", i + 5),
                    );
                    p.text(
                        x + 4,
                        173,
                        92,
                        20,
                        15.,
                        if selected { BG } else { INK },
                        target.name(),
                    );
                }
            }
            if s.track_kind(ui.selected) == Kind::Drum {
                p.rect(5, 152, 310, 44, PANEL);
                p.text(12, 153, 296, 18, 14., GOLD, "PAD 5 / IMPORT XPM");
                p.text(12, 174, 296, 19, 15., INK, "CHOOSE FILE > BANK > 16 PADS");
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
        Page::Loop => loop_workspace(ui, s, p),
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
            for pad in 0..ACTIVE_TRACKS {
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
                track_mixer_cell(p, pad, &title, pad == ui.selected, m.level, m.mute, m.solo);
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
        Page::Play => loop_workspace(ui, s, p),
        Page::TrackFx | Page::MasterFx => {
            let target = if ui.page == Page::MasterFx {
                ui.master_target + 1
            } else {
                0
            };
            let count = if target >= 2 { 1 } else { 2 };
            for pad in 0..16 {
                if let Some(&effect) = crate::ui::FX_PADS.get(pad) {
                    let slot = (0..count).find(|slot| {
                        s.fx(fx_base(target, *slot)).effect == effect
                            && (target != 1 || !s.texture().master_bypass[*slot])
                    });
                    let state = slot.map_or(
                        if ui.fx_refused == Some(pad)
                            && !(0..count).any(|i| s.fx(fx_base(target, i)).effect == Effect::Off)
                        {
                            "FULL"
                        } else {
                            "OFF"
                        },
                        |slot| {
                            if ui.fx_is_held(target, slot) {
                                "HOLD"
                            } else {
                                "LATCH"
                            }
                        },
                    );
                    fx_cell(p, pad, effect_name(effect), state, slot.is_some());
                    if ui.fx_focus == pad {
                        fx_focus(p, pad);
                    }
                } else {
                    fx_cell(p, pad, "", "", false);
                }
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
            if !s.note_view(ui.selected) {
                audio_timeline(s, p);
            } else {
                note_timeline(ui, s, p);
            }
        }
    }
    if ui.page == Page::Source {
        let message = if s.words[7] != 0 {
            view_error(s.words[7], ui.page)
        } else if !ui.notice.is_empty() {
            ui.notice
        } else {
            "B3: LOAD / SAMPLE: LOOP"
        };
        p.text(
            6,
            207,
            308,
            22,
            14.,
            if s.words[7] != 0 { RED } else { DIM },
            message,
        );
        return;
    }
    if matches!(
        ui.page,
        Page::TrackFx | Page::MasterFx | Page::Texture | Page::FxSettings | Page::Perform
    ) {
        return;
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
        Page::FxSettings => "FX SETTINGS",
        Page::Perform => "PLAY SETTINGS",
    };
    let fx_target = if ui.page == Page::MasterFx {
        ui.master_target + 1
    } else {
        0
    };
    let current_fx = s.fx(fx_base(fx_target, ui.fx_slot));
    let controls = match ui.page {
        Page::Loop if ui.loop_mode == 2 => "K1 REPEATS  K2 REPEAT   K3 --",
        Page::Loop
            if ui.loop_mode == 0
                && ui.shift_view
                && crate::ui::melodic_pad_octave(s, ui.selected) =>
        {
            "K1 PAD OCT   K2 DELAY    K3 REVERB"
        }
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
                7 if ui.shift_view => "K1 PAN     K2 VOL DEPTH K3 RATE",
                7 => "K1 PAN     K2 PAN DEPTH K3 RATE",
                6 if ui.shift_view => "K1 SHAPE   K2 PITCH    K3 FILTER",
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

        Page::Grid if !s.note_view(ui.selected) => "AUDIO WAVEFORM / SOUND: CHOP",
        Page::Grid => "K1 PITCH   K2 LENGTH   K3 VELOCITY",
        Page::Play if s.track_kind(ui.selected) == Kind::Drum => "K1 TUNE    K2 DECAY     K3 LEVEL",
        Page::Play if s.track_kind(ui.selected) == Kind::Audio => {
            "K1 ROOT     K2 TUNE      K3 WARP"
        }
        Page::Play if ui.shift_view => "K1 WAVE    K2 DETUNE    K3 OCTAVE",
        Page::Play => "K1 CUTOFF   K2 ATTACK   K3 RELEASE",
        Page::Setup if ui.input_view => "INPUT SOURCE / DATA: SELECT",
        Page::Setup if ui.shift_view => "K1 BPM    K2 COUNT-IN    K3 GRID",
        Page::Setup => "K1 BPM    K2 COUNT-IN    K3 SWING",
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
        Page::Loop
            if ui.loop_mode == 0
                && ui.shift_view
                && crate::ui::melodic_pad_octave(s, ui.selected) =>
        {
            [
                format!("{:+}oct", s.play_settings().octave),
                format!("{:.0}%", s.mix(ui.selected).delay * 100.),
                format!("{:.0}%", s.mix(ui.selected).room * 100.),
            ]
        }
        Page::Loop if ui.loop_mode == 2 => {
            let c = s.chain_config();
            [
                if c.len > 0 {
                    format!(
                        "{}x",
                        c.steps[ui.chain_step.min(usize::from(c.len) - 1)].repeats
                    )
                } else {
                    "--".into()
                },
                if c.repeat { "ON".into() } else { "OFF".into() },
                String::new(),
            ]
        }
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
                    format!(
                        "{:.0}%",
                        if ui.shift_view {
                            t.lfo_volume
                        } else {
                            t.lfo_pan
                        } * 100.
                    ),
                    t.lfo_rate_label(),
                ],
                6 => [
                    if ui.shift_view {
                        crate::synth::LFO_SHAPES[usize::from(t.lfo_shape)].into()
                    } else {
                        t.lfo_rate_label()
                    },
                    format!("{:.0} ct", t.lfo_pitch),
                    format!("{:.2} oct", t.lfo_filter),
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
        Page::Play if s.track_kind(ui.selected) == Kind::Audio => {
            let a = s.audio_settings();
            [
                format!("MIDI {}", a.root),
                format!("{:+.0} st", a.tune),
                if a.warp { "ON".into() } else { "OFF".into() },
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
        Page::Setup if ui.input_view => [String::new(), String::new(), String::new()],
        Page::Setup => [
            format!("{:.0}", s.bpm()),
            format!("{} bar", (s.words[4] >> 8) & 255),
            if ui.shift_view {
                if s.swing().is_some_and(|v| v.sixteenth) {
                    "1/16"
                } else {
                    "1/8"
                }
                .into()
            } else {
                s.swing()
                    .map_or_else(|| "AS REC".into(), |v| format!("{:.0}%", v.amount))
            },
        ],
        _ => [String::new(), String::new(), String::new()],
    };
    for (i, value) in values.iter().enumerate() {
        p.text(9 + i as i32 * 105, 217, 92, 21, 19., GOLD, value);
    }
    let target = if matches!(ui.page, Page::MasterFx | Page::Texture | Page::Sidechain) {
        "MASTER"
    } else if ui.page == Page::KitMix {
        "SOUND"
    } else {
        "TRACK"
    };
    let level = if ui.page == Page::KitMix {
        s.drum(ui.sound).level
    } else {
        s.level(
            if matches!(ui.page, Page::MasterFx | Page::Texture | Page::Sidechain) {
                16
            } else {
                ui.selected
            },
        )
    };
    let err = view_error(s.words[7], ui.page);
    let footer = if ui.page == Page::Texture && err.is_empty() {
        "PAD: ON/OFF / DATA: SELECT > SC".into()
    } else if ui.page == Page::Sidechain && err.is_empty() {
        format!(
            "SHIFT+B2: {} / DATA PUSH: FX",
            if s.sidechain().sync { "SYNC" } else { "FREE" }
        )
    } else if ui.page == Page::DrumSound && !recording_view && err.is_empty() {
        if ui.notice.is_empty() {
            "SHIFT+PAD: RANDOM / DATA: SAMPLE".into()
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
    } else if s.words[9] != 0
        && matches!(
            ui.page,
            Page::Loop | Page::Play | Page::DrumSound | Page::Grid
        )
    {
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
        && ui.synth_page >= 6
    {
        format!(
            "SHIFT+B3: SYNC {}",
            if s.synth().lfo_sync { "ON" } else { "OFF" }
        )
    } else if ui.page == Page::Play
        && s.track_kind(ui.selected) == Kind::Synth
        && ui.chord_modes[ui.selected]
    {
        format!(
            "SHIFT+B2: SMOOTH {}",
            if s.chord().smooth { "ON" } else { "OFF" }
        )
    } else if ui.page == Page::Play
        && s.track_kind(ui.selected) == Kind::Synth
        && ui.synth_page == 0
    {
        "SHIFT+B1: PLAY SETTINGS / +/-: EDIT".to_owned()
    } else if !ui.notice.is_empty() {
        ui.notice.to_owned()
    } else if ui.page == Page::Setup && ui.input_view {
        if crate::input_hw::pending() {
            "SWITCHING INPUT...".into()
        } else {
            match crate::input_hw::result() {
                1 => "INPUT READY / DATA: SELECT",
                2 => "INPUT SWITCH FAILED",
                3 => "INPUT ACTIVE / SAVE FAILED",
                _ => "PAD 1 LINE / 2 MIC / 3 USB",
            }
            .into()
        }
    } else if ui.page == Page::Setup {
        "SHIFT K3: SWING GRID / MIDI ONLY".into()
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
    } else if ui.page == Page::Grid && !s.note_view(ui.selected) {
        format!("AUDIO {:02} / SAMPLE: BACK", ui.selected + 1)
    } else if ui.page == Page::Grid {
        format!("BAR {} / DATA: NOTE / +/-: BAR", ui.grid_bar + 1)
    } else if (ui.page == Page::Play || (ui.page == Page::Loop && ui.loop_mode == 0))
        && s.track_kind(ui.selected) == Kind::Drum
    {
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
    if ui.page != Page::Loop
        && !(ui.page == Page::Play
            && (s.track_kind(ui.selected) != Kind::Synth || ui.synth_page == 0))
    {
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
        Effect::Wobble | Effect::Tremolo | Effect::AutoPan | Effect::AutoFilter => {
            "K1 RATE     K2 DEPTH    K3 MIX"
        }
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
            "-  {}  {}/{}  +",
            [
                "PLAY",
                "OSC 1",
                "OSC 2",
                "FILTER",
                "AMP ADSR",
                "FILTER ADSR",
                "LFO",
                "LEGACY MOD"
            ][usize::from(page)],
            page + 1,
            crate::ui::synth_page_count(s)
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
            &format!("LFO {:.0}% / {}", t.lfo_pan * 100., t.lfo_rate_label()),
        );
        p.text(
            16,
            162,
            280,
            18,
            15.,
            DIM,
            &format!("LEGACY VOLUME {:.0}%", t.lfo_volume * 100.),
        );
    } else if page == 6 {
        for (i, (name, value)) in [
            (
                "SHAPE",
                crate::synth::LFO_SHAPES[usize::from(t.lfo_shape)].to_owned(),
            ),
            ("RATE", t.lfo_rate_label()),
            ("PITCH DEPTH", format!("{:.0} ct", t.lfo_pitch)),
            ("FILTER DEPTH", format!("{:.2} oct", t.lfo_filter)),
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
        10 => "NOTHING TO UNDO",
        11 => "NOTHING TO REDO",
        12 => "RESAMPLE BEFORE INPUT OVERDUB",
        6 if page == Page::Chop => "MAX 16 SLICES - USE A LONGER GRID",
        6 => "CAPACITY LIMIT",
        7 => "STOP / AUDIO LOOPS LOCK BPM",
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
        for i in 0..ACTIVE_TRACKS {
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
fn fx_quick_panel(ui: &Ui, s: &Snapshot, p: &mut impl Painter) {
    let (names, values) = if ui.page == Page::Texture && ui.texture_slot < 5 {
        let slot = ui.texture_slot;
        let v = s.texture().params[slot];
        let names = [
            ["GRAIN", "TONE", "MIX"],
            ["DRIVE", "TONE", "MIX"],
            ["SQUEEZE", "RELEASE", "MIX"],
            ["RATE", "DEPTH", "MIX"],
            ["DIVISION", "FEEDBACK", "MIX"],
        ][slot];
        let mut values = v.map(|x| format!("{:.0}%", x * 100.));
        if slot == 2 {
            values[1] = format!("{:.0}ms", (0.04 + v[1] * 0.8) * 1000.);
        }
        if slot == 3 {
            values[0] = if s.texture().modulation[3].sync {
                SYNC_LABELS[usize::from(s.texture().modulation[3].division.min(15))].into()
            } else {
                format!("{:.2}Hz", 0.2 + v[0] * 4.)
            };
        }
        if slot == 4 {
            values[0] = ["1/16", "1/8", "1/8D", "1/4"][(v[0] * 3.99) as usize].into();
        }
        (names, values)
    } else if ui.page == Page::Texture && ui.texture_slot == 5 {
        let c = s.sidechain();
        (
            ["DUCK", "RELEASE", "SENSE"],
            [
                format!("{:.0}%", c.depth * 100.),
                format!("{:.0}ms", c.release_seconds(s.bpm() as f32) * 1000.),
                format!("{:.0}%", (1. - (c.threshold - 0.01) / 0.49) * 100.),
            ],
        )
    } else if let Some((_, _, fx)) = ui.quick_fx(s) {
        let names = match fx.effect {
            Effect::Filter => ["CUTOFF", "RESO", "MIX"],
            Effect::Isolator => ["LOW", "HIGH", "MIX"],
            Effect::Repeat => ["DIVISION", "SMOOTH", "MIX"],
            Effect::Reverse | Effect::TapeStop => ["TIME", "TONE", "MIX"],
            Effect::DubEcho => ["DIVISION", "FEEDBACK", "MIX"],
            Effect::LoFi | Effect::Crush => ["RATE", "BITS", "MIX"],
            Effect::Drive => ["DRIVE", "TONE", "MIX"],
            Effect::Compressor => ["THRESHOLD", "RATIO", "MIX"],
            _ => ["RATE", "DEPTH", "MIX"],
        };
        let mut values = [
            format!("{:.0}%", fx.a * 100.),
            format!("{:.0}%", fx.b * 100.),
            format!("{:.0}%", fx.mix * 100.),
        ];
        if fx.effect.modulated() {
            values[0] = fx.rate_label();
        }
        if fx.effect == Effect::Repeat {
            values[0] = ["1/32", "1/16", "1/8", "1/4"][(fx.a * 3.99) as usize].into();
        }
        if fx.effect == Effect::DubEcho {
            values[0] = ["1/16", "1/8", "1/8D", "1/4"][(fx.a * 3.99) as usize].into();
        }
        if fx.effect == Effect::Reverse {
            values[0] = format!("{:.0}ms", (0.08 + fx.a * 0.8) * 1000.);
        }
        if fx.effect == Effect::TapeStop {
            values[0] = format!("{:.0}ms", (0.15 + fx.a * 2.) * 1000.);
        }
        (names, values)
    } else {
        (
            ["RELEASE FX", "NO SLOT", "--"],
            ["--".into(), "--".into(), "--".into()],
        )
    };
    for i in 0..3 {
        let x = 5 + i as i32 * 105;
        p.rect(x, 193, 100, 43, PANEL);
        p.text(
            x + 3,
            194,
            94,
            17,
            14.,
            DIM,
            &format!("K{} {}", i + 1, names[i]),
        );
        p.text(x + 3, 213, 94, 21, 18., GOLD, &values[i]);
    }
}
fn fx_cell(p: &mut impl Painter, pad: usize, title: &str, state: &str, on: bool) {
    let x = 5 + (pad % 4) as i32 * 79;
    let y = 55 + (3 - pad / 4) as i32 * 34;
    p.rect(x, y, 74, 32, if on { GOLD } else { PANEL });
    if title.is_empty() {
        return;
    }
    p.text(x + 3, y + 1, 68, 17, 15., if on { BG } else { INK }, title);
    p.text(x + 3, y + 17, 68, 14, 14., if on { BG } else { DIM }, state);
}
fn fx_focus(p: &mut impl Painter, pad: usize) {
    let x = 5 + (pad % 4) as i32 * 79;
    let y = 55 + (3 - pad / 4) as i32 * 34;
    p.rect(x + 3, y + 1, 68, 1, INK);
}
fn fx_settings_panel(ui: &Ui, s: &Snapshot, p: &mut impl Painter) {
    let fx = ui.edited_fx(s);
    let title = if fx.effect == Effect::AutoFilter {
        "AUTO FILTER"
    } else {
        effect_name(fx.effect)
    };
    p.text(8, 55, 205, 25, 21., GOLD, title);
    if let Some(edit) = ui.fx_edit {
        p.text(
            220,
            55,
            92,
            25,
            17.,
            INK,
            &if edit.texture || edit.target != 0 {
                "MASTER".into()
            } else {
                format!("TRACK {:02}", edit.track + 1)
            },
        );
    }
    let names = if let Some(edit) = ui.fx_edit {
        if edit.texture {
            match edit.slot {
                0 => ["GRAIN", "TONE", "MIX"],
                1 => ["DRIVE", "TONE", "MIX"],
                2 => ["AMOUNT", "RELEASE", "MIX"],
                3 => ["RATE", "DEPTH", "MIX"],
                _ => ["DIVISION", "FEEDBACK", "MIX"],
            }
        } else {
            match fx.effect {
                Effect::Filter => ["CUTOFF", "RESONANCE", "MIX"],
                Effect::Compressor => ["THRESHOLD", "RATIO", "MIX"],
                Effect::Repeat => ["DIVISION", "SMOOTH", "MIX"],
                Effect::Reverse | Effect::TapeStop => ["LENGTH", "TONE", "MIX"],
                Effect::DubEcho => ["DIVISION", "FEEDBACK", "MIX"],
                Effect::LoFi | Effect::Crush => ["RATE", "BITS", "MIX"],
                Effect::Isolator => ["LOW", "HIGH", "MIX"],
                Effect::Drive => ["DRIVE", "TONE", "MIX"],
                _ => ["RATE", "DEPTH", "MIX"],
            }
        }
    } else {
        ["CONTROL A", "CONTROL B", "MIX"]
    };
    let shape = crate::synth::LFO_SHAPES[usize::from(fx.modulation.shape.min(5))];
    let rows = if fx.effect.modulated() {
        [
            ("RATE", fx.rate_label()),
            ("DEPTH", format!("{:.0}%", fx.b * 100.)),
            ("SHAPE", shape.into()),
            ("MIX", format!("{:.0}%", fx.mix * 100.)),
        ]
    } else {
        [
            (names[0], format!("{:.0}%", fx.a * 100.)),
            (names[1], format!("{:.0}%", fx.b * 100.)),
            ("MIX", format!("{:.0}%", fx.mix * 100.)),
            ("", String::new()),
        ]
    };
    for (i, (name, value)) in rows.iter().enumerate() {
        let y = 86 + i as i32 * 24;
        p.text(10, y, 119, 22, 17., DIM, name);
        p.text(134, y, 177, 22, 19., INK, value);
    }
    let labels = names.map(str::to_owned);
    let values = if fx.effect.modulated() {
        [
            fx.rate_label(),
            format!("{:.0}%", fx.b * 100.),
            format!("{:.0}%", fx.mix * 100.),
        ]
    } else {
        [
            format!("{:.0}%", fx.a * 100.),
            format!("{:.0}%", fx.b * 100.),
            format!("{:.0}%", fx.mix * 100.),
        ]
    };
    for i in 0..3 {
        let x = 5 + i as i32 * 105;
        p.rect(x, 200, 100, 39, PANEL);
        p.text(
            x + 4,
            201,
            92,
            17,
            14.,
            INK,
            &format!("K{} {}", i + 1, labels[i]),
        );
        p.text(x + 4, 219, 92, 19, 17., GOLD, &values[i]);
    }
    p.text(8, 182, 303, 16, 14., DIM, "DATA: BACK / B2 SHAPE / B3 SYNC");
}
fn texture_panel(ui: &Ui, s: &Snapshot, p: &mut impl Painter) {
    let t = s.texture();
    for pad in 0..16 {
        let (name, on) = match pad {
            0..=4 => (crate::motion::TEXTURES[pad], t.enabled[pad]),
            5 => ("DUCK", s.sidechain().enabled && s.sidechain().depth > 0.),
            6 | 7 => {
                let slot = pad - 6;
                let fx = s.fx(fx_base(1, slot));
                if matches!(
                    fx.effect,
                    Effect::Tremolo | Effect::AutoPan | Effect::AutoFilter
                ) {
                    fx_cell(p, pad, "", "", false);
                    continue;
                }
                (
                    effect_name(if fx.effect == Effect::Off {
                        if slot == 0 {
                            Effect::Filter
                        } else {
                            Effect::Repeat
                        }
                    } else {
                        fx.effect
                    }),
                    fx.effect != Effect::Off && !t.master_bypass[slot],
                )
            }
            8..=10 => {
                let effect = [Effect::Tremolo, Effect::AutoPan, Effect::AutoFilter][pad - 8];
                let slot =
                    (0..2).find(|i| s.fx(fx_base(1, *i)).effect == effect && !t.master_bypass[*i]);
                let on = slot.is_some();
                let state = slot.map_or(
                    "OFF",
                    |i| if ui.fx_is_held(1, i) { "HOLD" } else { "LATCH" },
                );
                fx_cell(p, pad, effect_name(effect), state, on);
                if ui.texture_slot == pad {
                    fx_focus(p, pad);
                }
                continue;
            }
            _ => ("", false),
        };
        let state = if !on {
            "OFF"
        } else if ui.texture_held[pad] {
            "HOLD"
        } else {
            "LATCH"
        };
        fx_cell(p, pad, name, if name.is_empty() { "" } else { state }, on);
        if ui.texture_slot == pad && !name.is_empty() {
            fx_focus(p, pad);
        }
    }
}

fn note_timeline(ui: &Ui, s: &Snapshot, p: &mut impl Painter) {
    let x = 27;
    let y = 53;
    let width = 286;
    let height = 128;
    for i in 0..=16 {
        let xx = x + i * width / 16;
        p.rect(xx, y, 1, height, if i % 4 == 0 { DIM } else { PANEL });
    }
    let recording = s.words[9] == ui.selected as u64 + 1;
    let elapsed = s.words[8].saturating_sub(s.words[10]);
    let start = if recording {
        elapsed / (4 * PPQ) * 4 * PPQ
    } else if matches!(ui.page, Page::Loop | Page::Play) && s.words[4] & 1 != 0 {
        s.words[16 + ui.selected * 12 + 5] / (4 * PPQ) * (4 * PPQ)
    } else {
        ui.grid_bar * 4 * PPQ
    };
    let count = s.words[16 + ui.selected * 12 + 6].min(64) as usize;
    let notes = (0..count).filter_map(|i| {
        let tick = s.words[352 + i * 2];
        let flags = s.words[353 + i * 2];
        (flags & (1 << 16) != 0 && tick >= start && tick < start + 4 * PPQ)
            .then_some((flags & 255) as i32)
    });
    let fallback = if s.track_kind(ui.selected) == Kind::Drum {
        0
    } else {
        48
    };
    let mut low = notes.clone().min().unwrap_or(fallback);
    let mut high = notes.max().unwrap_or(low).max(low + 7).min(127);
    for note in 0..128 {
        if s.words[1044 + note / 64] & (1u64 << (note % 64)) != 0 {
            low = low.min(note as i32);
            high = high.max(note as i32);
        }
    }

    let rows = (high - low + 1).max(1);
    let lane = |row: i32| y + row * height / rows;
    for row in 0..=rows {
        p.rect(x, lane(row), width, 1, PANEL);
    }
    for row in (0..rows).step_by(((rows + 7) / 8) as usize) {
        if lane(row) + 17 <= y + height {
            label(p, 2, lane(row), 25, DIM, &(high - row).to_string());
        }
    }
    for i in 0..count {
        let tick = s.words[352 + i * 2];
        let flags = s.words[353 + i * 2];
        if flags & (1 << 16) == 0 || tick < start || tick >= start + 4 * PPQ {
            continue;
        }
        let note = (flags & 255) as i32;
        let row = high - note;
        if !(0..rows).contains(&row) {
            continue;
        }
        let off = ((i + 1)..count)
            .find(|j| {
                let f = s.words[353 + j * 2];
                f & (1 << 16) == 0 && f & 255 == flags & 255 && f & (1 << 24) == flags & (1 << 24)
            })
            .map_or(tick + PPQ / 4, |j| s.words[352 + j * 2]);
        let xx = x + ((tick - start) as f64 / (4 * PPQ) as f64 * f64::from(width)) as i32;
        let len = ((off.saturating_sub(tick)) as f64 / (4 * PPQ) as f64 * f64::from(width)) as i32;
        p.rect(
            xx,
            lane(row) + 1,
            len.max(2).min(x + width - xx),
            (lane(row + 1) - lane(row) - 1).max(1),
            if i == ui.grid_event { GOLD } else { DIM },
        );
    }
    for note in 0..128 {
        if s.words[1044 + note / 64] & (1u64 << (note % 64)) != 0 {
            let row = high - note as i32;
            if (0..rows).contains(&row) {
                p.rect(
                    x + width - 6,
                    lane(row) + 1,
                    5,
                    (lane(row + 1) - lane(row) - 1).max(1),
                    RED,
                );
            }
        }
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

fn audio_timeline(s: &Snapshot, p: &mut impl Painter) {
    p.rect(5, 53, 310, 128, PANEL);
    p.rect(5, 117, 310, 1, DIM);
    for i in 0..256 {
        let v = s.words[512 + i];
        let lo = f32::from_bits(v as u32).clamp(-1., 1.);
        let hi = f32::from_bits((v >> 32) as u32).clamp(-1., 1.);
        let x = 5 + i as i32 * 310 / 256;
        let right = 5 + (i + 1) as i32 * 310 / 256;
        let top = 117 - (hi * 62.) as i32;
        let bottom = 117 - (lo * 62.) as i32;
        p.rect(x, top, (right - x).max(1), (bottom - top).max(1), GOLD);
    }
    if s.words[770] == 0 {
        label(p, 12, 58, 290, INK, "REC TO CAPTURE AUDIO");
    }
    let b = 16 + s.words[3] as usize * 12;
    if s.words[b + 4] > 0 && s.words[4] & 1 != 0 && s.words[9] != s.words[3] + 1 {
        let pos = s.words[b + 5] as f64 / s.words[b + 4] as f64 * s.words[770] as f64;
        let start = s.words[768] as f64;
        let end = s.words[769] as f64;
        if end > start && pos >= start && pos < end {
            p.rect(
                5 + ((pos - start) / (end - start) * 310.) as i32,
                53,
                1,
                128,
                RED,
            );
        }
    }
}

/// Scale the full source timeline into a bounded overview, omitting tiny axis text.
struct TimelinePreview<'a, P: Painter> {
    p: &'a mut P,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}
impl<P: Painter> Painter for TimelinePreview<'_, P> {
    fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32) {
        let left = self.x + x * self.w / 320;
        let top = self.y + (y - 53).max(0) * self.h / 129;
        let right = (self.x + (x + w) * self.w / 320).min(self.x + self.w);
        let bottom = (self.y
            + ((y + h - 53).max(0) * self.h / 129).max((y - 53).max(0) * self.h / 129 + 1))
        .min(self.y + self.h);
        if right > left && bottom > top {
            self.p.rect(left, top, right - left, bottom - top, color);
        }
    }
    fn text(&mut self, _: i32, _: i32, _: i32, _: i32, _: f32, _: u32, _: &str) {}
}
fn loop_workspace(ui: &Ui, s: &Snapshot, p: &mut impl Painter) {
    let b = 16 + ui.selected * 12;
    let recording = s.words[9] == ui.selected as u64 + 1;
    let chord_label = if (ui.page == Page::Play || (ui.page == Page::Loop && ui.loop_mode == 0))
        && matches!(s.track_kind(ui.selected), Kind::Synth | Kind::Audio)
        && ui.chord_modes[ui.selected]
    {
        Some(s.chord().label(ui.sound))
    } else {
        None
    };
    let kind = if ui.page == Page::Loop && ui.loop_edit {
        match s.track_kind(ui.selected) {
            Kind::Audio => "AUDIO",
            Kind::Synth => "SYNTH",
            Kind::Drum => "DRUM",
        }
    } else if let Some(label) = &chord_label {
        label.as_str()
    } else if (ui.page == Page::Play || (ui.page == Page::Loop && ui.loop_mode == 0))
        && s.track_kind(ui.selected) == Kind::Drum
    {
        DRUM_NAMES[ui.sound]
    } else {
        match s.track_kind(ui.selected) {
            Kind::Audio => s.audio_target(ui.selected).name(),
            Kind::Synth => "SYNTH",
            Kind::Drum => "DRUM",
        }
    };
    p.text(
        7,
        51,
        217,
        20,
        17.,
        GOLD,
        &format!("{:02} {} / {}", ui.selected + 1, kind, status(s.words[b])),
    );
    let length = if recording && s.words[12] & 1 == 0 {
        s.words[11].saturating_sub(s.words[10])
    } else {
        s.words[b + 4]
    };
    p.text(
        232,
        51,
        81,
        20,
        17.,
        INK,
        &if length > 0 {
            format!("{:.0} BAR", length as f64 / (4 * PPQ) as f64)
        } else {
            "FREE".into()
        },
    );
    p.rect(5, 73, 310, 28, PANEL);
    let mut preview = TimelinePreview {
        p,
        x: 5,
        y: 73,
        w: 310,
        h: 28,
    };
    if s.track_kind(ui.selected) == Kind::Audio
        && (ui.page == Page::Play || !s.note_view(ui.selected))
    {
        if s.words[770] == 0 && !recording {
            input_timeline(s, &mut preview);
        } else {
            audio_timeline(s, &mut preview);
        }
    } else {
        note_timeline(ui, s, &mut preview);
    }
    let role = if ui.page == Page::Loop && ui.loop_mode == 0 {
        match s.track_kind(ui.selected) {
            Kind::Audio => match s.audio_target(ui.selected) {
                crate::audio::RecordTarget::Input => "REC: INPUT",
                crate::audio::RecordTarget::Chromatic => "REC: CHROMATIC",
                crate::audio::RecordTarget::Chop => "REC: CHOP",
                crate::audio::RecordTarget::Turntable => "PAD 1 GRAB / FADER",
            },
            Kind::Drum => "DRUM PADS",
            Kind::Synth => {
                if ui.chord_modes[ui.selected] {
                    "CHORD PADS"
                } else {
                    "CHROMATIC PADS"
                }
            }
        }
    } else if ui.page == Page::Loop {
        "PAD: PLAY / SHIFT+PAD: REC"
    } else if s.track_kind(ui.selected) == Kind::Audio {
        if s.audio_target(ui.selected) == crate::audio::RecordTarget::Chop {
            "PADS: CHOP / HOLD TO LOOP"
        } else {
            "PADS: CHROMATIC / HOLD TO LOOP"
        }
    } else {
        "16 PADS: PLAY"
    };
    let message = if s.words[7] != 0 {
        view_error(s.words[7], ui.page).to_owned()
    } else if !ui.notice.is_empty() {
        ui.notice.into()
    } else if recording && (s.words[12] >> 8) & 255 != 0 {
        format!(
            "{:02} REC > {:02} NEXT",
            ui.selected + 1,
            (s.words[12] >> 8) & 255
        )
    } else if recording && s.words[8] < s.words[10] {
        format!(
            "PRE {} BEATS / {}",
            (s.words[10] - s.words[8]).div_ceil(PPQ),
            role
        )
    } else if recording {
        format!(
            "{} / {}",
            if s.words[12] & 2 != 0 {
                "REC ENDING"
            } else {
                "SHIFT+PAD: END"
            },
            role
        )
    } else if ui.loop_edit {
        match ui.edit_source {
            Some(source) if ui.edit_operation == 1 => {
                format!("PAD {:02} AGAIN: DELETE", source + 1)
            }
            Some(source) => format!("{:02} > EMPTY PAD / DATA: EXIT", source + 1),
            None => "PAD: SOURCE / DATA: EXIT".into(),
        }
    } else if ui.loop_mode == 1 {
        "PAD: MUTE / SAMPLE: LOOP".into()
    } else if ui.loop_mode == 2 {
        let c = s.chain_config();
        if s.chain_running() {
            let index = ((s.words[1048] >> 16) & 255) as usize;
            let remaining = (s.words[1048] >> 24) & 255;
            let next = if index + 1 < usize::from(c.len) {
                Some(index + 1)
            } else if c.repeat && c.len > 0 {
                Some(0)
            } else {
                None
            };
            format!(
                "STEP {} / {}x / NEXT {}",
                index + 1,
                remaining,
                next.map_or("END".into(), |i| format!("{:02}", c.steps[i].track + 1))
            )
        } else if c.len > 0 {
            let i = ui.chain_step.min(usize::from(c.len) - 1);
            format!(
                "STEP {}/{}: LOOP {:02} / {}x",
                i + 1,
                c.len,
                c.steps[i].track + 1,
                c.steps[i].repeats
            )
        } else {
            "PAD ADD / DATA PUSH: START".into()
        }
    } else if ui.page == Page::Loop && ui.shift_view {
        if crate::ui::melodic_pad_octave(s, ui.selected) {
            format!("K1: OCT {:+} / SHIFT+ERASE: COPY", s.play_settings().octave)
        } else {
            "ERASE: DELETE / SHIFT+ERASE: COPY".into()
        }
    } else if ui.page == Page::Loop {
        if crate::ui::melodic_pad_octave(s, ui.selected) {
            format!(
                "SHIFT+PAD: START / END REC | OCT{:+}",
                s.play_settings().octave
            )
        } else {
            "SHIFT+PAD: START / END REC".into()
        }
    } else {
        role.into()
    };
    p.text(
        6,
        103,
        308,
        16,
        14.,
        if recording { RED } else { DIM },
        &message,
    );
    for i in 0..ACTIVE_TRACKS {
        let base = 16 + i * 12;
        let x = 5 + (i % 4) as i32 * 79;
        let y = 120 + (ACTIVE_TRACKS / 4 - 1 - i / 4) as i32 * 19;
        let rec = matches!((s.words[base] >> 8) & 255, 2..=4);
        let chain = s.chain_config();
        let selected = if ui.loop_mode == 2 && chain.len > 0 {
            i == usize::from(chain.steps[ui.chain_step.min(usize::from(chain.len) - 1)].track)
        } else {
            i == ui.selected
        };
        let bg = if ui.loop_edit && ui.edit_source == Some(i) {
            GOLD
        } else if rec {
            RED
        } else if selected {
            GOLD
        } else {
            PANEL
        };
        let ink = if rec || selected { BG } else { INK };
        p.rect(x, y, 74, 18, bg);
        let label = if rec {
            if s.words[base] >> 8 & 255 == 2 {
                "PRE"
            } else {
                "REC"
            }
        } else if s.mix(i).mute {
            "MUTE"
        } else if ui.loop_mode == 2
            && s.chain_running()
            && ((s.words[1048] >> 32) & 255) == i as u64 + 1
        {
            "CHAIN"
        } else if ui.loop_mode == 2 && chain.members() & (1 << i) != 0 {
            "IN"
        } else {
            match s.words[base] >> 8 & 255 {
                5 => "PLAY",
                6 => "STOP",
                _ => "--",
            }
        };
        p.text(
            x + 3,
            y,
            69,
            18,
            15.,
            ink,
            &format!("{:02} {}", i + 1, label),
        );
        let len = s.words[base + 4];
        if len > 0 {
            p.rect(
                x + 2,
                y + 17,
                (s.words[base + 5].min(len) as f64 / len as f64 * 70.) as i32,
                1,
                ink,
            );
        }
    }
}
fn track_mixer_cell(
    p: &mut impl Painter,
    pad: usize,
    title: &str,
    selected: bool,
    level: f32,
    mute: bool,
    solo: bool,
) {
    if ACTIVE_TRACKS == 16 {
        mixer_cell(p, pad, title, selected, level, mute, solo);
        return;
    }
    let x = 5 + (pad % 4) as i32 * 79;
    let y = 80 + (1 - pad / 4) as i32 * 54;
    p.rect(x, y, 74, 50, if selected { GOLD } else { PANEL });
    p.text(
        x + 3,
        y + 2,
        68,
        20,
        15.,
        if selected { BG } else { INK },
        &title.chars().take(9).collect::<String>(),
    );
    p.text(
        x + 3,
        y + 17,
        68,
        17,
        14.,
        if selected { BG } else { DIM },
        &format!(
            "{:.0}%{}",
            level * 100.,
            if mute {
                " M"
            } else if solo {
                " S"
            } else {
                ""
            }
        ),
    );
    p.rect(x + 3, y + 44, 68, 3, BG);
    p.rect(
        x + 3,
        y + 44,
        (level.clamp(0., 1.) * 68.) as i32,
        3,
        if selected { BG } else { GOLD },
    );
}

fn input_timeline(s: &Snapshot, p: &mut impl Painter) {
    p.rect(5, 53, 310, 128, PANEL);
    p.rect(5, 117, 310, 1, DIM);
    for i in 0..128 {
        let v = s.words[1056 + i];
        let lo = f32::from_bits(v as u32).clamp(-1., 1.);
        let hi = f32::from_bits((v >> 32) as u32).clamp(-1., 1.);
        let x = 5 + i as i32 * 310 / 128;
        let right = 5 + (i + 1) as i32 * 310 / 128;
        let y = 117 - (hi * 62.) as i32;
        let bottom = 117 - (lo * 62.) as i32;
        p.rect(x, y, (right - x).max(1), (bottom - y).max(1), GOLD);
    }
}

fn performance_panel(ui: &Ui, s: &Snapshot, p: &mut impl Painter) {
    let cfg = s.play_settings();
    let c = s.chord();
    let kind = s.track_kind(ui.selected);
    let (names, values) = match ui.perform_section {
        0 => (
            [
                "VOICE",
                "GLIDE",
                if ui.shift_view { "STRUM DIR" } else { "STRUM" },
            ],
            [
                ["POLY", "MONO", "LEGATO"][usize::from(cfg.voice.min(2))].into(),
                format!("{:.0}ms", cfg.glide_ms),
                if ui.shift_view {
                    ["UP", "DOWN", "ALTERNATE"][usize::from(c.strum_direction.min(2))].into()
                } else if c.strum_ms == 0. {
                    "OFF".into()
                } else {
                    format!("{:.0}ms", c.strum_ms)
                },
            ],
        ),
        1 => {
            if ui.shift_view {
                (
                    ["OCTAVE", "VOICING", "STRUM"],
                    [
                        format!("{:+}oct", c.octave),
                        if c.open {
                            "OPEN".into()
                        } else {
                            "CLOSE".into()
                        },
                        format!("{:.0}ms", c.strum_ms),
                    ],
                )
            } else {
                (
                    ["KEY", "SCALE", "COLOR"],
                    [
                        [
                            "C", "C#", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B",
                        ][usize::from(c.key.min(11))]
                        .into(),
                        ["MAJOR", "MINOR", "DORIAN"][usize::from(c.scale.min(2))].into(),
                        ["TRIAD", "7TH", "9TH"][usize::from(c.color.min(2))].into(),
                    ],
                )
            }
        }
        _ => (
            [
                "TRIGGER",
                if kind == Kind::Synth {
                    "SHOT GATE"
                } else {
                    "--"
                },
                "--",
            ],
            [
                if cfg.trigger == 0 && kind != Kind::Drum {
                    "GATE".into()
                } else {
                    ["KIT", "GATE", "ONE SHOT"][usize::from(cfg.trigger.min(2))].into()
                },
                if kind == Kind::Synth {
                    format!("{:.0}ms", cfg.shot_ms)
                } else {
                    "--".into()
                },
                "--".into(),
            ],
        ),
    };
    p.rect(5, 53, 310, 128, PANEL);
    p.text(
        14,
        59,
        292,
        23,
        20.,
        GOLD,
        match ui.perform_section {
            0 => "VOICE / STRUM",
            1 => "CHORD PERFORMANCE",
            _ => "GATE / ONE SHOT",
        },
    );
    let lines = match ui.perform_section {
        0 => [
            "MONO / LEGATO: MELODY NOTES",
            "CHORDS KEEP THEIR POLYPHONY",
            "SHIFT+K3: STRUM DIRECTION",
        ],
        1 => [
            "B3: NOTES / CHORD",
            "SHIFT+K1 OCT / K2 VOICING",
            "SHIFT+B3: SMOOTH VOICING",
        ],
        _ if kind == Kind::Synth => [
            "GATE: RELEASE ON NOTE-OFF",
            "ONE SHOT: AUTO RELEASE",
            "SHOT GATE: TIME BEFORE RELEASE",
        ],
        _ => [
            "GATE: HOLD / RELEASE TO STOP",
            "ONE SHOT: PLAY TO SAMPLE END",
            "KIT: ORIGINAL PAD SETTINGS",
        ],
    };
    for (i, line) in lines.iter().enumerate() {
        p.text(14, 91 + i as i32 * 26, 292, 23, 16., INK, line);
    }
    if s.words[7] != 0 {
        p.rect(8, 151, 304, 27, PANEL);
        p.text(14, 153, 292, 23, 15., RED, view_error(s.words[7], ui.page));
    }

    for i in 0..3 {
        let x = 5 + i as i32 * 105;
        p.rect(x, 193, 100, 43, PANEL);
        p.text(
            x + 3,
            194,
            94,
            17,
            14.,
            DIM,
            &format!("K{} {}", i + 1, names[i]),
        );
        p.text(x + 3, 213, 94, 21, 18., GOLD, &values[i]);
    }
}

fn disk(p: &mut impl Painter, cx: i32, cy: i32, r: i32, color: u32) {
    for y in -r..=r {
        let width = ((r * r - y * y) as f32).sqrt().floor() as i32;
        p.rect(cx - width, cy + y, width * 2 + 1, 1, color);
    }
}
fn line(p: &mut impl Painter, x: i32, y: i32, ex: i32, ey: i32, color: u32) {
    let n = (ex - x).abs().max((ey - y).abs()).max(1);
    for i in 0..=n {
        p.rect(x + (ex - x) * i / n, y + (ey - y) * i / n, 2, 2, color);
    }
}
fn turntable_panel(ui: &Ui, s: &Snapshot, p: &mut impl Painter) {
    p.rect(0, 0, 320, 240, BG);
    p.text(7, 2, 210, 23, 18., INK, "SOUND / TURNTABLE");
    p.text(222, 4, 93, 19, 15., GOLD, &format!("{:.0} BPM", s.bpm()));
    let mode = if s.words[16 + ui.selected * 12] & (1 << 18) != 0 {
        "OVERDUB"
    } else {
        "REPLACE"
    };
    for (i, title) in ["TURN >", "CHOP EDIT", mode].iter().enumerate() {
        p.rect(
            5 + i as i32 * 105,
            27,
            100,
            22,
            if i == 0 { GOLD } else { PANEL },
        );
        p.text(
            9 + i as i32 * 105,
            29,
            92,
            19,
            16.,
            if i == 0 { BG } else { INK },
            title,
        );
    }
    p.rect(5, 54, 310, 111, PANEL);
    disk(p, 76, 110, 55, DIM);
    disk(p, 76, 110, 54, 0xff111c18);
    for r in [48, 42, 35, 28] {
        disk(p, 76, 110, r, 0xff344a3c);
        disk(p, 76, 110, r - 1, 0xff111c18);
    }
    disk(p, 76, 110, 19, GOLD);
    p.text(60, 97, 34, 14, 12., BG, "LOOP");
    p.text(58, 115, 40, 13, 10., BG, "SIDE A");
    disk(p, 76, 110, 2, BG);
    let angle = s.deck_head() * std::f32::consts::TAU - std::f32::consts::FRAC_PI_2;
    line(
        p,
        76 + (angle.cos() * 24.) as i32,
        110 + (angle.sin() * 24.) as i32,
        76 + (angle.cos() * 48.) as i32,
        110 + (angle.sin() * 48.) as i32,
        GOLD,
    );
    disk(p, 136, 63, 6, DIM);
    line(p, 136, 63, 128, 106, INK);
    line(p, 128, 106, 110, 127, INK);
    p.rect(104, 125, 11, 7, GOLD);
    p.text(
        150,
        55,
        160,
        18,
        14.,
        GOLD,
        &format!("SAMPLE / {:02}", ui.selected + 1),
    );
    p.text(150, 76, 160, 20, 17., INK, &s.name(ui.selected));
    let grabbed = s.words[1191] & 2 != 0;
    let speed = s.deck_rate();
    let state = if s.words[1191] & 4 == 0 {
        "READY"
    } else if !grabbed {
        "PLAY"
    } else if speed.abs() < 0.02 {
        "GRAB"
    } else if speed < 0. {
        "REV"
    } else {
        "FWD"
    };
    p.text(
        150,
        99,
        160,
        24,
        20.,
        GOLD,
        &format!("{state} {:.2}x", speed.abs()),
    );
    let seconds = s.words[770] as f64 / s.words[1].max(1) as f64;
    p.text(
        150,
        124,
        160,
        18,
        14.,
        INK,
        &format!(
            "{:.2} / {:.2}s",
            f64::from(s.deck_head()) * seconds,
            seconds
        ),
    );
    p.rect(150, 144, 70, 17, if s.deck().slip { GOLD } else { BG });
    p.text(
        153,
        144,
        66,
        17,
        13.,
        if s.deck().slip { BG } else { DIM },
        if s.deck().slip { "SLIP ON" } else { "SLIP OFF" },
    );
    p.text(
        224,
        144,
        87,
        17,
        13.,
        if grabbed { GOLD } else { DIM },
        if grabbed { "GRABBED" } else { "PAD 1 GRAB" },
    );
    p.rect(5, 170, 310, 21, PANEL);
    let mut wave = TimelinePreview {
        p,
        x: 5,
        y: 170,
        w: 310,
        h: 21,
    };
    audio_timeline(s, &mut wave);
    p.rect(5 + (s.deck_head() * 309.) as i32, 170, 1, 21, GOLD);
    for (i, (name, value)) in [
        ("K1 SPEED", format!("{:.2}x", s.deck().speed)),
        (
            "K2 RANGE",
            format!(
                "{} BEAT",
                ["1/2", "1", "2"][usize::from(s.deck().range.min(2))]
            ),
        ),
        ("K3 LEVEL", format!("{:.0}%", s.level(ui.selected) * 100.)),
    ]
    .iter()
    .enumerate()
    {
        let x = 5 + i as i32 * 105;
        p.rect(x, 196, 100, 42, PANEL);
        p.text(x + 4, 197, 92, 17, 14., DIM, name);
        p.text(x + 4, 214, 92, 23, 20., GOLD, value);
    }
    let err = view_error(s.words[7], ui.page);
    if !err.is_empty() {
        p.rect(5, 170, 310, 21, BG);
        p.text(8, 171, 304, 20, 14., RED, err);
    } else if s.words[9] == ui.selected as u64 + 1 {
        p.rect(5, 170, 94, 21, RED);
        p.text(
            8,
            171,
            90,
            20,
            14.,
            BG,
            if s.words[8] < s.words[10] {
                "COUNT IN"
            } else {
                "REC FADER"
            },
        );
    }
}
