use loop_club::{
    engine::Engine,
    model::*,
    ui::{Snapshot, Ui},
    view::{self, Painter},
};
struct Bounds;
impl Painter for Bounds {
    fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, _: u32) {
        assert!(
            x >= 0 && y >= 0 && w >= 0 && h >= 0 && x + w <= 320 && y + h <= 240,
            "rectangle {x},{y} {w}x{h}"
        );
    }
    fn text(&mut self, x: i32, y: i32, w: i32, h: i32, size: f32, _: u32, _: &str) {
        assert!(x >= 0 && y >= 0 && w > 0 && h > 0 && x + w <= 320 && y + h <= 240);
        assert!(size > 0. && size <= 22.);
    }
}
#[test]
fn every_page_stays_inside_native_screen() {
    let mut e = Engine::new(8000, 88., 24000);
    e.tracks[0].kind = Kind::Drum;
    let s = Snapshot::capture(&e);
    let mut ui = Ui::default();
    for page in [
        Page::Loop,
        Page::Play,
        Page::Grid,
        Page::Mix,
        Page::KitMix,
        Page::TrackFx,
        Page::MasterFx,
        Page::Setup,
        Page::Source,
        Page::DrumSound,
        Page::Output,
        Page::Sidechain,
        Page::Texture,
        Page::FxSettings,
    ] {
        ui.navigate(page, &s);
        view::render(&ui, &s, &mut Bounds);
    }
}

#[test]
fn chop_and_browser_stay_inside_native_screen() {
    let mut e = Engine::new(8000, 88., 24000);
    e.tracks[0].audio_frames = 8000;
    e.tracks[0].source_frames = 8000;
    e.tracks[0].length_ticks = 4 * PPQ;
    e.tracks[0].chops.divide(8000);
    let s = Snapshot::capture(&e);
    let mut ui = Ui::default();
    ui.navigate(Page::Chop, &s);
    assert_eq!(ui.page, Page::Chop);
    view::render(&ui, &s, &mut Bounds);
    loop_club::browser_device::BrowserUi::default().render(&mut Bounds);
}

#[test]
fn actionable_notice_is_not_hidden_by_empty_track_hint() {
    struct Captions(Vec<String>);
    impl Painter for Captions {
        fn rect(&mut self, _: i32, _: i32, _: i32, _: i32, _: u32) {}
        fn text(&mut self, _: i32, _: i32, _: i32, _: i32, _: f32, _: u32, text: &str) {
            self.0.push(text.into());
        }
    }
    let e = Engine::new(8000, 88., 16000);
    let mut ui = Ui::default();
    ui.notice = "SESSION SAVED";
    let mut captions = Captions(Vec::new());
    view::render(&ui, &Snapshot::capture(&e), &mut captions);
    assert!(captions.0.iter().any(|s| s == "SESSION SAVED"));
    assert!(
        !captions
            .0
            .iter()
            .any(|s| s == "SHIFT+DATA: CHOOSE INSTRUMENT")
    );
}

#[test]
fn enlarged_synth_views_and_shift_cards_stay_inside_screen() {
    let mut e = Engine::new(8000, 88., 16000);
    e.tracks[0].kind = Kind::Synth;
    let s = Snapshot::capture(&e);
    let mut ui = Ui::default();
    ui.navigate(Page::Play, &s);
    for page in 0..=7 {
        ui.synth_page = page;
        for shift in [false, true] {
            ui.shift_view = shift;
            view::render(&ui, &s, &mut Bounds);
        }
    }
}

#[test]
fn zero_master_is_visible_on_every_page_and_fx_shows_master_value() {
    struct Texts(Vec<String>);
    impl Painter for Texts {
        fn rect(&mut self, _: i32, _: i32, _: i32, _: i32, _: u32) {}
        fn text(&mut self, _: i32, _: i32, _: i32, _: i32, _: f32, _: u32, s: &str) {
            self.0.push(s.into());
        }
    }
    let mut e = Engine::new(8000, 88., 8000);
    e.tracks[0].kind = Kind::Drum;
    e.master = 0.;
    let s = Snapshot::capture(&e);
    let mut ui = Ui::default();
    for page in [
        Page::Loop,
        Page::Play,
        Page::Grid,
        Page::Mix,
        Page::KitMix,
        Page::TrackFx,
        Page::MasterFx,
        Page::Texture,
        Page::Sidechain,
        Page::Source,
        Page::Setup,
        Page::Output,
        Page::DrumSound,
    ] {
        ui.navigate(page, &s);
        let mut p = Texts(Vec::new());
        view::render(&ui, &s, &mut p);
        assert!(p.0.iter().any(|t| t == "MASTER MUTED"), "{page:?}");
    }
    e.master = 0.8;
    let s = Snapshot::capture(&e);
    for page in [Page::MasterFx, Page::Texture, Page::Sidechain] {
        ui.navigate(page, &s);
        let mut p = Texts(Vec::new());
        view::render(&ui, &s, &mut p);
        assert!(p.0.iter().any(|t| t.contains("M80%")));
    }
}

#[test]
fn recording_previews_and_audio_grid_stay_inside_screen() {
    for kind in [Kind::Audio, Kind::Synth, Kind::Drum] {
        let mut e = Engine::new_project(8000, 24000);
        e.tracks[0].kind = kind;
        e.command(loop_club::engine::Command::Record(0)).unwrap();
        for _ in 0..1600 {
            e.process(Frame::mono(0.3));
        }
        let s = Snapshot::capture(&e);
        let mut ui = Ui::default();
        for page in [Page::Loop, Page::Play, Page::DrumSound, Page::Grid] {
            ui.navigate(page, &s);
            view::render(&ui, &s, &mut Bounds);
        }
    }
}

#[test]
fn sixteen_loop_performance_role_and_sidechain_targets_stay_in_bounds() {
    let mut e = Engine::new_project(8000, 24000);
    e.tracks[0].kind = Kind::Synth;
    e.sidechain.mode = 1;
    e.command(loop_club::engine::Command::LoopRecord(0))
        .unwrap();
    let s = Snapshot::capture(&e);
    let mut ui = Ui::default();
    ui.loop_perform = true;
    ui.loop_play_track = 0;
    ui.chord_modes[0] = true;
    view::render(&ui, &s, &mut Bounds);
    ui.navigate(Page::Sidechain, &s);
    view::render(&ui, &s, &mut Bounds);
}

#[test]
fn fx_boards_keep_effect_pads_without_utility_actions_and_show_inline_controls() {
    struct Texts(Vec<String>);
    impl Painter for Texts {
        fn rect(&mut self, _: i32, _: i32, _: i32, _: i32, _: u32) {}
        fn text(&mut self, _: i32, _: i32, _: i32, _: i32, _: f32, _: u32, s: &str) {
            self.0.push(s.into());
        }
    }
    let e = Engine::new(8000, 100., 8000);
    let mut u = Ui::default();
    for page in [Page::TrackFx, Page::MasterFx, Page::Texture] {
        u.page = page;
        let mut p = Texts(Vec::new());
        view::render(&u, &Snapshot::capture(&e), &mut p);
        for banned in [
            "SLOT 1",
            "SLOT 2",
            "TOGGLE",
            "FX OFF",
            "PERF EDIT",
            "SC SET",
            "ALL OFF",
            "ACTION",
            "SETUP",
        ] {
            assert!(
                !p.0.iter().any(|t| t.contains(banned)),
                "{page:?}: {banned}"
            );
        }
    }
}

#[test]
fn loop_record_hint_does_not_appear_on_the_instrument_performance_page() {
    struct Texts(Vec<String>);
    impl Painter for Texts {
        fn rect(&mut self, _: i32, _: i32, _: i32, _: i32, _: u32) {}
        fn text(&mut self, _: i32, _: i32, _: i32, _: i32, _: f32, _: u32, text: &str) {
            self.0.push(text.into());
        }
    }
    let mut e = Engine::new_project(8000, 24000);
    e.tracks[0].kind = Kind::Synth;
    let s = Snapshot::capture(&e);
    let mut u = Ui::default();
    let mut texts = Texts(Vec::new());
    view::render(&u, &s, &mut texts);
    assert!(
        texts
            .0
            .iter()
            .any(|t| t.contains("SHIFT+PAD: START / END REC"))
    );
    u.page = Page::Play;
    texts.0.clear();
    view::render(&u, &s, &mut texts);
    assert!(
        !texts
            .0
            .iter()
            .any(|t| t.contains("SHIFT+PAD: START / END REC"))
    );
    assert!(texts.0.iter().any(|t| t.contains("16 PADS: PLAY")));
    e.tracks[0].kind = Kind::Audio;
    u.page = Page::Loop;
    texts.0.clear();
    view::render(&u, &Snapshot::capture(&e), &mut texts);
    assert!(
        texts
            .0
            .iter()
            .any(|t| t.contains("SHIFT+PAD: START / END REC"))
    );
}

#[test]
fn audio_source_target_labels_have_no_generic_footer_overlaid_on_their_rows() {
    struct Texts(Vec<(i32, String)>);
    impl Painter for Texts {
        fn rect(&mut self, _: i32, _: i32, _: i32, _: i32, _: u32) {}
        fn text(&mut self, _: i32, y: i32, _: i32, _: i32, _: f32, _: u32, text: &str) {
            self.0.push((y, text.into()));
        }
    }
    let mut e = Engine::new_project(8000, 24000);
    e.tracks[0].kind = Kind::Audio;
    let mut u = Ui::default();
    u.navigate(Page::Source, &Snapshot::capture(&e));
    let mut p = Texts(Vec::new());
    view::render(&u, &Snapshot::capture(&e), &mut p);
    for target in ["INPUT", "CHROMATIC", "CHOP"] {
        assert!(p.0.iter().any(|(y, t)| *y == 173 && t == target));
    }
    assert!(
        !p.0.iter()
            .any(|(y, t)| *y >= 152 && *y < 200 && t.contains("PAD 1:AUDIO"))
    );
    assert!(p.0.iter().any(|(y, t)| *y >= 200 && t.contains("B3: LOAD")));
    u.notice = "USE EMPTY AUDIO TRACK";
    p.0.clear();
    view::render(&u, &Snapshot::capture(&e), &mut p);
    assert!(
        p.0.iter()
            .any(|(y, t)| *y >= 200 && t == "USE EMPTY AUDIO TRACK")
    );
}

#[test]
fn loop_edit_captions_name_the_whole_instrument_not_one_performance_pad() {
    let mut e = Engine::new(8000, 100., 16000);
    e.tracks[0].kind = Kind::Drum;
    let s = Snapshot::capture(&e);
    let mut ui = Ui::default();
    ui.loop_edit = true;
    ui.sound = 2;
    #[derive(Default)]
    struct Captions(Vec<String>);
    impl Painter for Captions {
        fn rect(&mut self, _: i32, _: i32, _: i32, _: i32, _: u32) {}
        fn text(&mut self, _: i32, _: i32, _: i32, _: i32, _: f32, _: u32, text: &str) {
            self.0.push(text.into());
        }
    }
    let mut texts = Captions::default();
    view::render(&ui, &s, &mut texts);
    assert!(texts.0.iter().any(|t| t.contains("01 DRUM /")));
    assert!(!texts.0.iter().any(|t| t.contains("CH HAT /")));
    for op in 0..3 {
        ui.edit_operation = op;
        view::render(&ui, &s, &mut Bounds);
    }
}
