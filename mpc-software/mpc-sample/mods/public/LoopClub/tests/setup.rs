use loop_club::{
    engine::{Command, Engine},
    model::*,
    ui::{Action, Snapshot, Ui},
};
#[test]
fn setup_switches_only_monitor_and_preroll_with_visible_back() {
    let mut e = Engine::new(8000, 240., 16000);
    let mut u = Ui::default();
    u.navigate(Page::Setup, &Snapshot::capture(&e));
    u.setup_timing = true;
    let kind = e.tracks[0].kind;
    for key in [0, 0] {
        u.soft(key, false, &Snapshot::capture(&e), |c| {
            e.command(c).unwrap();
        });
    }
    assert!(!e.metronome);
    u.soft(1, false, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    assert_eq!(e.preroll, 0);
    u.soft(1, false, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    assert_eq!(e.preroll, 1);
    assert_eq!(
        u.pad(0, true, 80, false, false, &Snapshot::capture(&e), |c| {
            e.command(c).unwrap();
        }),
        Action::None
    );
    assert!(e.metronome);
    u.pad(1, true, 80, false, false, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    assert_eq!(e.preroll, 0);
    u.pad(4, true, 80, false, false, &Snapshot::capture(&e), |_| {
        panic!("Unused setup pad must be inert")
    });
    assert_eq!(e.tracks[0].kind, kind);
    assert!(e.recorder.is_none());
    u.soft(2, false, &Snapshot::capture(&e), |_| {
        panic!("Back sends no command")
    });
    assert_eq!(u.page, Page::Loop);
}
#[test]
fn setup_restores_last_preroll_length_and_keeps_project_input_access() {
    let mut e = Engine::new(8000, 240., 16000);
    e.preroll = 2;
    let mut u = Ui::default();
    u.navigate(Page::Setup, &Snapshot::capture(&e));
    u.setup_timing = true;
    for _ in 0..2 {
        u.soft(1, false, &Snapshot::capture(&e), |c| {
            e.command(c).unwrap();
        });
    }
    assert_eq!(e.preroll, 2);
    u.knob(1, 0., &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    assert_eq!(e.preroll, 0);
    u.soft(1, false, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    assert_eq!(e.preroll, 2);
    u.setup_timing = false;
    u.soft(0, false, &Snapshot::capture(&e), |_| {});
    assert!(u.setup_timing);
    u.setup_timing = false;
    u.soft(2, false, &Snapshot::capture(&e), |_| {});
    assert!(u.input_view);
}
#[test]
fn waveform_and_audio_use_identical_wrap_at_both_fader_endpoints() {
    use loop_club::deck::{Player, Settings, position_fraction, wrapped_position};
    let mut e = Engine::new(44100, 100., 44100);
    e.tracks[0].source_frames = 11025;
    e.tracks[0].audio_frames = 11025;
    let cfg = Settings::default();
    let mut d = Player::default();
    d.start(2000.);
    d.grab(true, Some(0.5), cfg, 44100);
    for raw in [0, 127] {
        let value = loop_club::device::normalize_cc(raw, 0., 127.);
        d.move_to(value, cfg, 100., 44100);
        let expected = 2000. + f64::from(value - 0.5) * cfg.beats() * 60. / 100. * 44100.;
        assert!((d.state.target - expected).abs() < 1e-8);
        d.advance(200000, cfg, 44100);
        assert!((d.state.head - expected).abs() < 1e-6);
        e.decks[0] = d;
        let s = Snapshot::capture(&e);
        assert_eq!(s.deck_head(), position_fraction(expected, 11025));
        assert_eq!(d.state.fader, value);
        assert_eq!(
            wrapped_position(expected, 11025),
            expected.rem_euclid(11025.)
        );
    }
    for head in [
        -f64::MIN_POSITIVE,
        -f64::EPSILON,
        -1e-14,
        0.,
        11025.,
        22050.,
    ] {
        e.decks[0].state.head = head;
        let s = Snapshot::capture(&e);
        assert_eq!(s.deck_head(), 0.);
        assert_eq!(wrapped_position(head, 11025), 0.);
    }
}
#[test]
fn regrab_after_repositioning_at_min_or_max_has_no_initial_jump() {
    use loop_club::deck::{Player, Settings};
    let cfg = Settings::default();
    let mut d = Player::default();
    d.start(1234.);
    for fader in [0., 1., 0.5] {
        let before = d.state.head;
        d.grab(true, Some(fader), cfg, 44100);
        d.move_to(fader, cfg, 100., 44100);
        assert_eq!(d.state.head, before);
        assert_eq!(d.state.target, before);
        d.grab(false, None, cfg, 44100);
    }
}

#[test]
fn turntable_entry_uses_full_wave_but_chop_zoom_and_unselected_tracks_are_untouched() {
    use loop_club::audio::RecordTarget;
    let mut e = Engine::new(8000, 100., 16000);
    e.tracks[0].source_frames = 11025;
    e.tracks[0].audio_frames = 11025;
    e.tracks[0].chops.divide(11025);
    e.tracks[0].audio_target = RecordTarget::Turntable;
    e.command(Command::WaveView {
        track: 0,
        start: 1000,
        end: 3000,
    })
    .unwrap();
    let s = Snapshot::capture(&e);
    let mut u = Ui::default();
    u.navigate(Page::Chop, &s);
    assert!(loop_club::device::turntable_wave_reset(&u, &s).is_none());
    u.navigate(Page::Play, &s);
    e.command(loop_club::device::turntable_wave_reset(&u, &s).unwrap())
        .unwrap();
    let full = Snapshot::capture(&e);
    assert_eq!(full.words[768], 0);
    assert_eq!(full.words[769], 11025);
    assert!(loop_club::device::turntable_wave_reset(&u, &full).is_none());
    assert_eq!(e.tracks[0].chops.count, 16);
    e.tracks[1].audio_target = RecordTarget::Turntable;
    let old = Snapshot::capture(&e);
    u.selected = 1;
    assert!(loop_club::device::turntable_wave_reset(&u, &old).is_none());
}

#[test]
fn turntable_wave_strip_has_aligned_endpoints_and_only_one_playhead() {
    #[derive(Default)]
    struct Paint(Vec<(i32, i32, i32, i32)>);
    impl loop_club::view::Painter for Paint {
        fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, _: u32) {
            self.0.push((x, y, w, h));
        }
        fn text(&mut self, _: i32, _: i32, _: i32, _: i32, _: f32, _: u32, _: &str) {}
    }
    let mut e = Engine::new(8000, 100., 16000);
    let t = &mut e.tracks[0];
    t.source_frames = 11025;
    t.audio_frames = 11025;
    t.audio_target = loop_club::audio::RecordTarget::Turntable;
    t.length_ticks = 4 * PPQ;
    t.playing = true;
    e.running = true;
    e.clock.frame = e.clock.frame_at(2 * PPQ);
    let mut ui = Ui::default();
    ui.page = Page::Play;
    for (head, x) in [(0., 5), (11024., 314), (-f64::EPSILON, 5)] {
        e.decks[0].state.head = head;
        let mut paint = Paint::default();
        loop_club::view::render(&ui, &Snapshot::capture(&e), &mut paint);
        let cursors: Vec<_> = paint
            .0
            .iter()
            .filter(|(_, y, w, h)| *y == 170 && *w == 1 && *h >= 19)
            .collect();
        assert_eq!(cursors.len(), 1);
        assert_eq!(cursors[0].0, x);
        let peaks: Vec<_> = paint
            .0
            .iter()
            .filter(|(_, y, _, h)| *y == 180 && *h == 1)
            .collect();
        assert_eq!(peaks.iter().map(|r| r.0).min(), Some(5));
        assert_eq!(peaks.iter().map(|r| r.0 + r.2).max(), Some(315));
    }
}
