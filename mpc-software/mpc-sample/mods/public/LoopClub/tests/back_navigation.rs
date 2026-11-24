use loop_club::{
    engine::Engine,
    model::*,
    ui::{Snapshot, Ui},
    view::{self, Painter},
};
fn core(kind: Kind) -> Engine {
    let mut e = Engine::new(8000, 100., 24000);
    e.tracks[0].kind = kind;
    if kind == Kind::Audio {
        e.tracks[0].source_frames = 8000;
        e.tracks[0].audio_frames = 8000;
        e.tracks[0].length_ticks = 4 * PPQ;
    }
    e
}
fn soft(u: &mut Ui, key: usize, s: &Snapshot) {
    u.soft(key, false, s, |_| panic!("Back emitted an audio edit"));
}
#[test]
fn source_setup_and_input_have_ordered_return_to_the_actual_parent() {
    let e = core(Kind::Synth);
    let s = Snapshot::capture(&e);
    let mut u = Ui::default();
    u.navigate(Page::Source, &s);
    u.soft(1, false, &s, |_| {});
    assert_eq!(u.page, Page::Setup);
    assert!(u.setup_timing);
    u.setup_timing = false;
    u.input_view = true;
    u.back(&s);
    assert!(!u.input_view && !u.setup_timing);
    soft(&mut u, 0, &s);
    assert!(u.setup_timing);
    soft(&mut u, 2, &s);
    assert_eq!(u.page, Page::Source);
    soft(&mut u, 0, &s);
    assert_eq!(u.page, Page::Loop);
}
#[test]
fn synth_deep_editor_back_returns_to_play_then_loop_without_changing_parameters() {
    let e = core(Kind::Synth);
    let s = Snapshot::capture(&e);
    let mut u = Ui::default();
    u.navigate(Page::Play, &s);
    u.synth_page = 5;
    let before = e.tracks[0].synth;
    soft(&mut u, 0, &s);
    assert_eq!(u.page, Page::Play);
    assert_eq!(u.synth_page, 0);
    soft(&mut u, 0, &s);
    assert_eq!(u.page, Page::Loop);
    assert_eq!(e.tracks[0].synth.attack, before.attack);
    u.synth_page = 6;
    u.navigate(Page::Play, &s);
    assert_eq!(u.synth_page, 0);
}
#[test]
fn grid_back_restores_editor_subpage_but_sound_entry_from_root_starts_at_play() {
    let e = core(Kind::Synth);
    let s = Snapshot::capture(&e);
    let mut u = Ui::default();
    u.navigate(Page::Play, &s);
    u.synth_page = 2;
    u.navigate(Page::Grid, &s);
    soft(&mut u, 0, &s);
    assert_eq!(u.page, Page::Play);
    assert_eq!(u.synth_page, 2);
    u.navigate(Page::Loop, &s);
    u.navigate(Page::Play, &s);
    assert_eq!(u.synth_page, 0);
}
#[test]
fn audio_sound_has_back_without_changing_the_record_mode_and_chop_returns_to_sound() {
    let e = core(Kind::Audio);
    let s = Snapshot::capture(&e);
    let mut u = Ui::default();
    u.navigate(Page::Play, &s);
    u.navigate(Page::Chop, &s);
    soft(&mut u, 0, &s);
    assert_eq!(u.page, Page::Play);
    soft(&mut u, 2, &s);
    assert_eq!(u.page, Page::Loop);
    assert_eq!(e.tracks[0].record_mode, RecordMode::Replace);
}
#[test]
fn drum_layer_detail_and_play_settings_return_one_level_at_a_time() {
    let e = core(Kind::Drum);
    let s = Snapshot::capture(&e);
    let mut u = Ui::default();
    u.navigate(Page::DrumSound, &s);
    u.layer = 1;
    u.layer_detail = true;
    u.soft(0, true, &s, |_| {});
    assert_eq!(u.page, Page::Perform);
    soft(&mut u, 0, &s);
    assert_eq!(u.page, Page::DrumSound);
    assert!(u.layer_detail);
    soft(&mut u, 0, &s);
    assert_eq!(u.page, Page::DrumSound);
    assert!(!u.layer_detail);
    assert_eq!(u.layer, 1);
    soft(&mut u, 0, &s);
    assert_eq!(u.page, Page::Loop);
}
#[test]
fn mixer_output_fx_and_advanced_settings_back_preserve_parent() {
    let e = core(Kind::Synth);
    let s = Snapshot::capture(&e);
    let mut u = Ui::default();
    u.navigate(Page::Mix, &s);
    u.navigate(Page::Output, &s);
    soft(&mut u, 0, &s);
    assert_eq!(u.page, Page::Mix);
    u.navigate(Page::TrackFx, &s);
    u.open_fx_settings(&s);
    assert_eq!(u.page, Page::FxSettings);
    soft(&mut u, 0, &s);
    assert_eq!(u.page, Page::TrackFx);
    soft(&mut u, 0, &s);
    assert_eq!(u.page, Page::Mix);
    soft(&mut u, 0, &s);
    assert_eq!(u.page, Page::Loop);
}
#[test]
fn sidechain_back_returns_to_texture_and_history_is_bounded() {
    let e = core(Kind::Synth);
    let s = Snapshot::capture(&e);
    let mut u = Ui::default();
    u.navigate(Page::Texture, &s);
    u.texture_slot = 5;
    u.open_fx_settings(&s);
    assert_eq!(u.page, Page::Sidechain);
    soft(&mut u, 0, &s);
    assert_eq!(u.page, Page::Texture);
    for _ in 0..50 {
        u.navigate(Page::Mix, &s);
        u.navigate(Page::TrackFx, &s);
    }
    for _ in 0..12 {
        u.back(&s);
    }
    assert_eq!(u.page, Page::Loop);
}
#[test]
fn loop_edit_has_back_and_shift_copy_preserves_move_access() {
    let e = core(Kind::Synth);
    let s = Snapshot::capture(&e);
    let mut u = Ui::default();
    u.loop_edit = true;
    u.soft(0, true, &s, |_| {});
    assert_eq!(u.edit_operation, 2);
    soft(&mut u, 2, &s);
    assert!(!u.loop_edit);
}
#[test]
fn every_non_root_view_has_a_visible_back_or_done_label() {
    struct Labels(Vec<String>);
    impl Painter for Labels {
        fn rect(&mut self, _: i32, _: i32, _: i32, _: i32, _: u32) {}
        fn text(&mut self, _: i32, y: i32, _: i32, _: i32, _: f32, _: u32, t: &str) {
            if (20..55).contains(&y) {
                self.0.push(t.into());
            }
        }
    }
    for kind in [Kind::Audio, Kind::Synth, Kind::Drum] {
        let e = core(kind);
        let s = Snapshot::capture(&e);
        for page in [
            Page::Play,
            Page::Grid,
            Page::Mix,
            Page::TrackFx,
            Page::MasterFx,
            Page::Texture,
            Page::Output,
            Page::Sidechain,
            Page::Source,
            Page::Setup,
            Page::Perform,
            Page::FxSettings,
        ] {
            let mut u = Ui::default();
            u.navigate(page, &s);
            let mut labels = Labels(Vec::new());
            view::render(&u, &s, &mut labels);
            assert!(
                labels.0.iter().any(|t| t == "BACK"),
                "{kind:?}/{page:?}: {:?}",
                labels.0
            );
        }
    }
}

#[test]
fn every_audio_sound_target_including_custom_turntable_draws_its_back_button() {
    struct Labels(Vec<String>);
    impl Painter for Labels {
        fn rect(&mut self, _: i32, _: i32, _: i32, _: i32, _: u32) {}
        fn text(&mut self, _: i32, y: i32, _: i32, _: i32, _: f32, _: u32, t: &str) {
            if (20..55).contains(&y) {
                self.0.push(t.into());
            }
        }
    }
    use loop_club::audio::RecordTarget;
    for target in [
        RecordTarget::Input,
        RecordTarget::Chromatic,
        RecordTarget::Chop,
        RecordTarget::Turntable,
    ] {
        let mut e = core(Kind::Audio);
        e.tracks[0].audio_target = target;
        let s = Snapshot::capture(&e);
        let mut u = Ui::default();
        u.navigate(Page::Play, &s);
        let mut labels = Labels(Vec::new());
        view::render(&u, &s, &mut labels);
        assert!(labels.0.iter().any(|text| text == "BACK"), "{target:?}");
        soft(&mut u, 2, &s);
        assert_eq!(u.page, Page::Loop);
        assert_eq!(e.tracks[0].record_mode, RecordMode::Replace);
    }
}
