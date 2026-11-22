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
