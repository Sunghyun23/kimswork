use loop_club::{
    engine::Engine,
    model::*,
    ui::{Snapshot, Ui},
    view::{self, Painter},
};
use serde_json::{Value, json};
#[derive(Default)]
struct Canvas {
    ops: Vec<Value>,
}
impl Painter for Canvas {
    fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, c: u32) {
        self.ops
            .push(json!({"rect":[x,y,w,h],"color":format!("#{:06x}",c&0xffffff)}));
    }
    fn text(&mut self, x: i32, y: i32, w: i32, h: i32, size: f32, c: u32, t: &str) {
        self.ops.push(
            json!({"text":t,"box":[x,y,w,h],"size":size,"color":format!("#{:06x}",c&0xffffff)}),
        );
    }
}
fn main() -> std::io::Result<()> {
    let output = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "build/previews".into());
    let dir = std::path::Path::new(&output);
    std::fs::create_dir_all(dir)?;
    let mut e = Engine::new(8000, 88., 24000);
    e.tracks[0].kind = Kind::Synth;
    e.tracks[0].name = "SOUL KEYS".into();
    e.tracks[0].length_ticks = 4 * PPQ;
    e.tracks[0].events.extend([
        Event {
            deck: None,
            timing: None,
            slice: false,
            tick: 0,
            note: 60,
            velocity: 80,
            on: true,
        },
        Event {
            deck: None,
            timing: None,
            slice: false,
            tick: PPQ,
            note: 60,
            velocity: 0,
            on: false,
        },
        Event {
            deck: None,
            timing: None,
            slice: false,
            tick: 2 * PPQ,
            note: 65,
            velocity: 66,
            on: true,
        },
        Event {
            deck: None,
            timing: None,
            slice: false,
            tick: 3 * PPQ,
            note: 65,
            velocity: 0,
            on: false,
        },
    ]);
    e.tracks[0].playing = true;
    e.running = true;
    for _ in 0..1000 {
        e.process(Frame::ZERO);
    }
    e.tracks[1].name = "VINYL LOOP".into();
    e.tracks[1].audio_frames = 20000;
    e.tracks[1].source_frames = 18000;
    e.tracks[1].length_ticks = 4 * PPQ;
    e.tracks[1].chops.divide(18000);
    for i in 0..18000 {
        let phase = (i % 4000) as f32 / 8000.;
        let sample = (phase * std::f32::consts::TAU * 440.).sin() * (-phase * 18.).exp() * 0.7;
        e.tracks[1].audio[i] = Frame::mono(sample);
        e.tracks[1].features.feed(Frame::mono(sample), 8000);
    }

    let mut ui = Ui::default();
    for (page, name) in [
        (Page::Loop, "loop"),
        (Page::Mix, "mix"),
        (Page::Play, "play"),
        (Page::Play, "chord"),
        (Page::Play, "drum"),
        (Page::Grid, "grid"),
        (Page::KitMix, "kit-mix"),
        (Page::TrackFx, "track-fx"),
        (Page::MasterFx, "master-fx"),
        (Page::Setup, "setup"),
        (Page::Setup, "setup-timing"),
        (Page::Setup, "setup-timing-on"),
        (Page::Chop, "chop"),
        (Page::Chop, "chop-transient"),
        (Page::Chop, "chop-bpm"),
        (Page::Chop, "chop-manual"),
    ] {
        if name == "chord" {
            ui.chord_modes[0] = true;
        }
        if name == "drum" || page == Page::KitMix {
            e.tracks[2].kind = Kind::Drum;
            e.tracks[2].name = "BREAK KIT".into();
            e.selected = 2;
            ui.selected = 2;
            for sound in 0..3 {
                e.command(loop_club::engine::Command::GenerateDrumPart { track: 2, sound })
                    .unwrap();
            }
        }
        if page == Page::Chop {
            e.selected = 1;
            ui.selected = 1;
            if name == "chop-transient" {
                e.tracks[1].chops.mode = loop_club::chop::Mode::Transient;
                e.tracks[1].chops.sensitivity = 0.8;
                e.command(loop_club::engine::Command::ChopApply(1)).unwrap();
            }
            if name == "chop-bpm" {
                e.tracks[1].chops.mode = loop_club::chop::Mode::Bpm;
                e.tracks[1].chops.grid = 3;
                e.command(loop_club::engine::Command::ChopApply(1)).unwrap();
            }
            if name == "chop-manual" {
                e.tracks[1].chops.mode = loop_club::chop::Mode::Manual;
                e.command(loop_club::engine::Command::ChopApply(1)).unwrap();
                e.command(loop_club::engine::Command::SplitSlice {
                    track: 1,
                    pad: 0,
                    at: 6000,
                })
                .unwrap();
                e.command(loop_club::engine::Command::SplitSlice {
                    track: 1,
                    pad: 1,
                    at: 12000,
                })
                .unwrap();
                ui.chop_cursor = 3000;
            }
        }
        let previous_metronome = e.metronome;
        if name == "setup-timing-on" {
            e.metronome = true;
        }
        let s = Snapshot::capture(&e);
        ui.navigate(page, &s);
        ui.setup_timing = name.starts_with("setup-timing");
        let mut canvas = Canvas::default();
        view::render(&ui, &s, &mut canvas);
        let data = json!({"kind":"host-render-not-device-screenshot","width":320,"height":240,"ops":canvas.ops});
        std::fs::write(
            dir.join(format!("{name}.json")),
            serde_json::to_vec_pretty(&data)?,
        )?;
        e.metronome = previous_metronome;
    }
    e.selected = 0;
    ui.selected = 0;
    ui.chord_modes[0] = false;
    for (page, name) in [
        (1, "synth-osc1"),
        (2, "synth-osc2"),
        (3, "synth-filter"),
        (4, "synth-amp-adsr"),
        (5, "synth-filter-adsr"),
        (6, "synth-lfo"),
    ] {
        ui.page = Page::Play;
        ui.synth_page = page;
        let s = Snapshot::capture(&e);
        let mut canvas = Canvas::default();
        view::render(&ui, &s, &mut canvas);
        let data = json!({"kind":"host-render-not-device-screenshot","width":320,"height":240,"ops":canvas.ops});
        std::fs::write(
            dir.join(format!("{name}.json")),
            serde_json::to_vec_pretty(&data)?,
        )?;
    }
    Ok(())
}
