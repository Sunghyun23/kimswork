use loop_club::{
    audio::RecordTarget,
    deck::{Player, Settings},
    engine::{Command, Engine},
    model::*,
    ui::{Snapshot, Ui},
};
fn core() -> Engine {
    let mut e = Engine::new(8000, 240., 16000);
    e.preroll = 0;
    let t = &mut e.tracks[0];
    t.audio_target = RecordTarget::Turntable;
    t.deck.reverse_cut = false;
    t.source_frames = 8000;
    t.audio_frames = 8000;
    t.source_ticks = 4 * PPQ;
    t.length_ticks = 4 * PPQ;
    t.chops.divide(8000);
    for (i, x) in t.audio[..8000].iter_mut().enumerate() {
        *x = Frame::mono((i as f32 * 0.091).sin() * 0.2);
    }
    e
}
fn frames(e: &mut Engine, n: usize) -> f32 {
    (0..n)
        .map(|_| e.process(Frame::ZERO).peak())
        .fold(0., f32::max)
}
#[test]
fn fader_direction_and_stationary_stylus_are_finite_and_silent() {
    let e = core();
    let pcm = &e.tracks[0].audio[..8000];
    let cfg = Settings {
        reverse_cut: false,
        ..Settings::default()
    };
    let mut d = Player::default();
    d.grab(true, Some(0.5), cfg, 8000);
    d.move_to(0.9, cfg, 240., 8000);
    let mut peak = 0f32;
    for _ in 0..200 {
        peak = peak.max(d.sample(pcm, cfg, 8000).peak());
    }
    assert!(peak > 0.01 && d.rate >= 0.);
    d.move_to(0.1, cfg, 240., 8000);
    for _ in 0..100 {
        assert!(d.sample(pcm, cfg, 8000).finite());
    }
    assert!(d.rate < 0.);
    for _ in 0..8000 {
        d.sample(pcm, cfg, 8000);
    }
    assert!(d.sample(pcm, cfg, 8000).peak() < 0.00001);
}
#[test]
fn slip_returns_to_moving_playhead_and_off_keeps_scratched_position() {
    for slip in [true, false] {
        let cfg = Settings {
            slip,
            ..Settings::default()
        };
        let mut d = Player::default();
        d.start(100.);
        d.grab(true, Some(0.5), cfg, 8000);
        d.move_to(0.7, cfg, 240., 8000);
        d.advance(400, cfg, 8000);
        let head = d.state.head;
        let underlying = d.state.slip;
        d.grab(false, None, cfg, 8000);
        assert!((d.state.head - if slip { underlying } else { head }).abs() < 1e-9);
        assert!(!d.state.grabbed);
    }
}
#[test]
fn analytical_seek_matches_continuous_playback_and_large_ranges_keep_direction() {
    let e = core();
    let pcm = &e.tracks[0].audio[..8000];
    let cfg = Settings {
        range: 2,
        ..Settings::default()
    };
    let mut a = Player::default();
    a.grab(true, Some(0.), cfg, 8000);
    a.move_to(1., cfg, 40., 8000);
    let mut b = a;
    for _ in 0..2000 {
        a.sample(pcm, cfg, 8000);
    }
    b.advance(2000, cfg, 8000);
    assert!((a.state.head - b.state.head).abs() < 1.);
    assert!(a.state.head > 0. && b.state.head > 0.);
    b.advance(u32::MAX as u64, cfg, 8000);
    assert!(b.sample(pcm, cfg, 8000).finite());
}
#[test]
fn recording_replays_controls_keeps_pcm_and_supports_undo_redo_and_project_recall() {
    let mut e = core();
    let original = e.tracks[0].audio[..8000].to_vec();
    e.command(Command::LoopRecord(0)).unwrap();
    frames(&mut e, 100);
    e.command(Command::DeckGrab(0, true, Some(0.5))).unwrap();
    e.command(Command::DeckFader(0, 0.9)).unwrap();
    assert!(frames(&mut e, 500) > 0.01);
    e.command(Command::DeckFader(0, 0.1)).unwrap();
    frames(&mut e, 500);
    e.command(Command::DeckGrab(0, false, None)).unwrap();
    e.command(Command::LoopRecord(0)).unwrap();
    frames(&mut e, 7000);
    assert!(e.recorder.is_none());
    assert!(e.tracks[0].audio_sequence);
    assert!(e.tracks[0].events.len() >= 4);
    assert!(
        e.tracks[0]
            .events
            .iter()
            .all(|x| x.deck.is_some() && x.timing.is_none())
    );
    assert_eq!(e.tracks[0].audio[..8000], original);
    e.command(Command::Stop).unwrap();
    assert!(e.is_idle());
    let recorded = e.tracks[0].events.clone();
    e.command(Command::Undo).unwrap();
    assert!(!e.tracks[0].audio_sequence && e.tracks[0].events.is_empty());
    assert_eq!(e.tracks[0].audio[..8000], original);
    e.command(Command::Redo).unwrap();
    assert_eq!(e.tracks[0].events, recorded);
    let file = std::env::temp_dir().join(format!("loop-deck-{}", std::process::id()));
    let _ = std::fs::remove_file(&file);
    loop_club::project::save(&e, &file).unwrap();
    let mut loaded = loop_club::project::load(&file, 8000, 16000).unwrap();
    std::fs::remove_file(&file).unwrap();
    assert_eq!(loaded.tracks[0].events, recorded);
    loaded.command(Command::PlayAll).unwrap();
    assert!(frames(&mut loaded, 9000) > 0.01);
    assert_eq!(loaded.tracks[0].audio[..8000], original);
}
#[test]
fn pad_grab_and_fader_never_change_volume_and_release_survives_navigation() {
    let mut e = core();
    let mut u = Ui::default();
    u.page = Page::Play;
    let level = e.tracks[0].mix.level;
    u.fader(0.5, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    u.pad(0, true, 90, false, false, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    assert!(e.decks[0].state.grabbed);
    u.fader(0.9, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    assert_eq!(e.tracks[0].mix.level, level);
    assert!(frames(&mut e, 500) > 0.01);
    u.navigate(Page::Mix, &Snapshot::capture(&e));
    u.pad(0, false, 0, false, false, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    assert!(!e.decks[0].state.grabbed);
    e.command(Command::Stop).unwrap();
    assert!(e.is_idle());
}
#[test]
fn scratch_automation_bypasses_global_swing_and_mixed_sampler_overdub_preserves_source() {
    let mut e = core();
    e.tracks[0].record_mode = RecordMode::Overdub;
    e.tracks[0].audio_sequence = true;
    e.tracks[0].events.push(Event {
        deck: None,
        timing: None,
        tick: 0,
        note: 48,
        velocity: 90,
        on: true,
        slice: false,
    });
    assert!(e.command(Command::LoopRecord(0)).is_err());
    assert_eq!(e.tracks[0].source_frames, 8000);
    assert!(e.recorder.is_none());
    let control = loop_club::deck::Control {
        head: 10.,
        slip: 100.,
        ..Default::default()
    };
    let event = Event {
        deck: Some(control),
        timing: None,
        tick: PPQ / 2,
        note: 0,
        velocity: 0,
        on: false,
        slice: false,
    };
    let b = loop_club::swing::Basis {
        amount: 75.,
        sixteenth: false,
    };
    assert_eq!(b.event(event, 4 * PPQ), event);
}
#[test]
fn scratch_overdub_preserves_untouched_gestures_and_noop_take() {
    let mut e = core();
    e.command(Command::LoopRecord(0)).unwrap();
    frames(&mut e, 100);
    e.command(Command::DeckGrab(0, true, Some(0.5))).unwrap();
    e.command(Command::DeckFader(0, 0.8)).unwrap();
    frames(&mut e, 300);
    e.command(Command::DeckGrab(0, false, None)).unwrap();
    frames(&mut e, 500);
    e.command(Command::DeckGrab(0, true, Some(0.5))).unwrap();
    e.command(Command::DeckFader(0, 0.2)).unwrap();
    frames(&mut e, 300);
    e.command(Command::DeckGrab(0, false, None)).unwrap();
    e.command(Command::LoopRecord(0)).unwrap();
    while e.recorder.is_some() {
        e.process(Frame::ZERO);
    }
    e.command(Command::Stop).unwrap();
    let original = e.tracks[0].events.clone();
    e.tracks[0].record_mode = RecordMode::Overdub;
    e.command(Command::LoopRecord(0)).unwrap();
    let start = e.recorder.unwrap().start;
    while e.clock.tick() <= start {
        e.process(Frame::ZERO);
    }
    frames(&mut e, 600);
    let punch_in = e.clock.tick() - start;
    e.command(Command::DeckGrab(0, true, Some(0.5))).unwrap();
    e.command(Command::DeckFader(0, 1.)).unwrap();
    frames(&mut e, 500);
    let punch_out = e.clock.tick() - start;
    e.command(Command::DeckGrab(0, false, None)).unwrap();
    e.command(Command::LoopRecord(0)).unwrap();
    while e.recorder.is_some() {
        e.process(Frame::ZERO);
    }
    e.command(Command::Stop).unwrap();
    let edited = e.tracks[0].events.clone();
    assert_ne!(edited, original);
    for event in original {
        if event.tick < punch_in || event.tick > punch_out {
            assert!(edited.contains(&event), "untouched {:?}", event);
        }
    }
    assert!(edited.iter().all(|e| !e.on));
    e.command(Command::LoopRecord(0)).unwrap();
    let start = e.recorder.unwrap().start;
    while e.clock.tick() <= start {
        e.process(Frame::ZERO);
    }
    frames(&mut e, 300);
    e.command(Command::LoopRecord(0)).unwrap();
    while e.recorder.is_some() {
        e.process(Frame::ZERO);
    }
    e.command(Command::Stop).unwrap();
    assert_eq!(e.tracks[0].events, edited);
}

fn deck_clip(e: &mut Engine) {
    let t = &mut e.tracks[0];
    t.audio_sequence = true;
    t.events.push(Event {
        deck: Some(Default::default()),
        timing: None,
        tick: 0,
        note: 0,
        velocity: 0,
        on: false,
        slice: false,
    });
}
#[test]
fn deck_to_sampler_overdub_is_refused_in_both_directions_without_changing_clip() {
    for target in [RecordTarget::Chromatic, RecordTarget::Chop] {
        let mut e = core();
        deck_clip(&mut e);
        let original = e.tracks[0].events.clone();
        e.command(Command::AudioTarget(0, target)).unwrap();
        e.tracks[0].record_mode = RecordMode::Overdub;
        assert_eq!(e.command(Command::LoopRecord(0)), Err(Error::ResampleFirst));
        assert!(e.recorder.is_none());
        assert_eq!(e.tracks[0].events, original);
    }
}
#[test]
fn replacing_a_deck_clip_with_notes_silences_the_old_deck_and_undo_restores_it() {
    let mut e = core();
    deck_clip(&mut e);
    e.command(Command::AudioTarget(0, RecordTarget::Chromatic))
        .unwrap();
    e.command(Command::PlayAll).unwrap();
    assert!(frames(&mut e, 800) > 0.01);
    assert!(e.decks[0].active);
    e.command(Command::LoopRecord(0)).unwrap();
    let start = e.recorder.unwrap().start;
    while e.clock.tick() < start {
        e.process(Frame::ZERO);
    }
    frames(&mut e, 300);
    assert!(!e.decks[0].active, "old scratch must stop at Replace start");
    assert_eq!(frames(&mut e, 800), 0.);
    e.command(Command::LoopRecord(0)).unwrap();
    while e.recorder.is_some() {
        e.process(Frame::ZERO);
    }
    assert!(e.tracks[0].audio_sequence && e.tracks[0].events.is_empty());
    assert!(!e.decks[0].active);
    assert_eq!(frames(&mut e, 1000), 0.);
    e.command(Command::Undo).unwrap();
    assert!(frames(&mut e, 1000) > 0.01);
    e.command(Command::Redo).unwrap();
    assert_eq!(frames(&mut e, 1000), 0.);
}
#[test]
fn grid_follows_recording_and_saved_clip_identity_after_switching_targets() {
    let mut e = core();
    deck_clip(&mut e);
    e.command(Command::AudioTarget(0, RecordTarget::Chromatic))
        .unwrap();
    assert!(!Snapshot::capture(&e).note_view(0));
    e.command(Command::LoopRecord(0)).unwrap();
    assert!(
        Snapshot::capture(&e).note_view(0),
        "show incoming MIDI while replacing an old Deck clip"
    );
    e.command(Command::Stop).unwrap();
    e.tracks[0].events.clear();
    e.tracks[0].events.push(Event {
        deck: None,
        timing: None,
        tick: 0,
        note: 48,
        velocity: 90,
        on: true,
        slice: false,
    });
    e.tracks[0].audio_sequence = true;
    e.command(Command::AudioTarget(0, RecordTarget::Turntable))
        .unwrap();
    assert!(
        Snapshot::capture(&e).note_view(0),
        "a saved note clip stays MIDI even when next input is Turntable"
    );
}
#[test]
fn canceling_an_automated_deck_take_restores_silence_but_manual_audition_can_continue() {
    let mut e = core();
    deck_clip(&mut e);
    e.tracks[0].record_mode = RecordMode::Overdub;
    e.command(Command::LoopRecord(0)).unwrap();
    assert!(frames(&mut e, 1000) > 0.01);
    e.command(Command::CancelRecord).unwrap();
    assert!(!e.tracks[0].playing);
    assert_eq!(frames(&mut e, 800), 0.);
    e.command(Command::DeckGrab(0, true, Some(0.5))).unwrap();
    e.command(Command::DeckGrab(0, false, None)).unwrap();
    assert!(
        frames(&mut e, 1000) > 0.01,
        "an explicit manual audition remains available until STOP"
    );
}
#[test]
fn turntable_respects_pan_track_filter_and_master_level() {
    let mut e = core();
    for (i, f) in e.tracks[0].audio[..8000].iter_mut().enumerate() {
        *f = Frame::mono((i as f32 * 0.9).sin() * 0.2);
    }
    e.command(Command::DeckGrab(0, true, Some(0.5))).unwrap();
    e.command(Command::DeckGrab(0, false, None)).unwrap();
    e.command(Command::Pan(0, -1.)).unwrap();
    frames(&mut e, 4000);
    let (mut l, mut r) = (0f32, 0f32);
    for _ in 0..4000 {
        let f = e.process(Frame::ZERO);
        l = l.max(f.l.abs());
        r = r.max(f.r.abs());
    }
    assert!(l > 0.01 && r < 0.0001);
    e.command(Command::Pan(0, 1.)).unwrap();
    frames(&mut e, 4000);
    let (mut l, mut r) = (0f32, 0f32);
    for _ in 0..4000 {
        let f = e.process(Frame::ZERO);
        l = l.max(f.l.abs());
        r = r.max(f.r.abs());
    }
    assert!(r > 0.01 && l < 0.0001);
    let plain = frames(&mut e, 2000);
    e.command(Command::SetFx {
        track: 0,
        slot: 0,
        fx: Fx {
            effect: Effect::Filter,
            a: 0.,
            b: 0.,
            mix: 1.,
            ..Default::default()
        },
    })
    .unwrap();
    frames(&mut e, 2000);
    let filtered = frames(&mut e, 2000);
    assert!(filtered < plain * 0.8, "plain {plain}, filtered {filtered}");
    e.command(Command::Master(0.)).unwrap();
    frames(&mut e, 6000);
    assert!(frames(&mut e, 1000) < 0.00001);
    assert!(e.decks[0].active);
}
#[test]
fn empty_source_and_invalid_controls_fail_without_creating_a_player_or_recording() {
    let mut e = Engine::new(8000, 240., 16000);
    e.command(Command::AudioTarget(0, RecordTarget::Turntable))
        .unwrap();
    assert_eq!(
        e.command(Command::DeckGrab(0, true, Some(0.5))),
        Err(Error::NoContent)
    );
    assert_eq!(e.command(Command::LoopRecord(0)), Err(Error::NoContent));
    assert!(!e.decks[0].active && e.recorder.is_none());
    assert!(e.command(Command::DeckFader(0, f32::NAN)).is_err());
    assert!(e.command(Command::DeckValue(0, 0, f32::INFINITY)).is_err());
}
#[test]
fn scratch_grab_capacity_reports_error_and_preserves_previous_clip() {
    let mut e = core();
    let original = e.tracks[0].audio[..8000].to_vec();
    e.command(Command::LoopRecord(0)).unwrap();
    frames(&mut e, 1);
    for _ in 0..128 {
        e.command(Command::DeckGrab(0, true, Some(0.5))).unwrap();
        frames(&mut e, 2);
        e.command(Command::DeckGrab(0, false, None)).unwrap();
        frames(&mut e, 2);
    }
    assert_eq!(
        e.command(Command::DeckGrab(0, true, Some(0.5))),
        Err(Error::Capacity)
    );
    assert_eq!(e.error, Some(Error::Capacity));
    assert!(e.recorder.is_none());
    assert_eq!(e.tracks[0].audio[..8000], original);
}
#[test]
fn sound_source_button_during_recording_preserves_target_and_chord_mode() {
    let mut e = core();
    e.command(Command::AudioTarget(0, RecordTarget::Chromatic))
        .unwrap();
    e.command(Command::ChordMode(0, true)).unwrap();
    e.command(Command::LoopRecord(0)).unwrap();
    let mut u = Ui::default();
    u.page = Page::Play;
    u.chord_modes[0] = true;
    u.soft(0, false, &Snapshot::capture(&e), |c| {
        let _ = e.command(c);
    });
    assert_eq!(e.tracks[0].audio_target, RecordTarget::Chromatic);
    assert!(e.tracks[0].chord_mode);
    assert!(!u.notice.is_empty() || e.error.is_some());
}
#[test]
fn replacing_midway_through_a_long_note_clip_releases_only_its_playback_owner() {
    for kind in [Kind::Synth, Kind::Audio] {
        let mut e = core();
        let t = &mut e.tracks[0];
        t.kind = kind;
        t.audio_target = RecordTarget::Chromatic;
        t.audio_sequence = kind == Kind::Audio;
        t.synth.release = 0.01;
        t.length_ticks = 16 * PPQ;
        if kind != Kind::Audio {
            t.audio_frames = 0;
            t.source_frames = 0;
        }
        t.events.extend([
            Event {
                deck: None,
                timing: None,
                tick: 0,
                note: 48,
                velocity: 90,
                on: true,
                slice: false,
            },
            Event {
                deck: None,
                timing: None,
                tick: 15 * PPQ,
                note: 48,
                velocity: 0,
                on: false,
                slice: false,
            },
        ]);
        e.command(Command::PlayAll).unwrap();
        assert!(frames(&mut e, 1000) > 0.01);
        e.command(Command::LoopRecord(0)).unwrap();
        let start = e.recorder.unwrap().start;
        while e.clock.tick() < start {
            e.process(Frame::ZERO);
        }
        frames(&mut e, 400);
        assert!(
            frames(&mut e, 300) < 0.00001,
            "{kind:?}: old held playback must not leak into the replacement take"
        );
        e.command(Command::Note {
            track: 0,
            note: 60,
            velocity: 90,
            on: true,
            source: 100,
        })
        .unwrap();
        assert!(frames(&mut e, 300) > 0.01);
    }
}

#[test]
fn down_cut_mutes_return_rearms_and_release_restores_forward_playback() {
    let e = core();
    let pcm = &e.tracks[0].audio[..8000];
    let cfg = Settings {
        reverse_cut: true,
        ..Settings::default()
    };
    let mut d = Player::default();
    d.grab(true, Some(0.8), cfg, 8000);
    d.move_to(0.1, cfg, 240., 8000);
    let mut peak = 0f32;
    for _ in 0..400 {
        peak = peak.max(d.sample(pcm, cfg, 8000).peak());
    }
    assert!(peak > 0.01 && d.rate < 0.);
    d.move_to(0.9, cfg, 240., 8000);
    for _ in 0..100 {
        d.sample(pcm, cfg, 8000);
    }
    assert!(d.state.returning && d.rate > 0.);
    for _ in 0..100 {
        assert_eq!(d.sample(pcm, cfg, 8000), Frame::ZERO);
    }
    let recorded = d.state;
    let mut replay = Player::default();
    replay.restore(recorded, 0, cfg, 8000);
    for _ in 0..100 {
        assert_eq!(replay.sample(pcm, cfg, 8000), Frame::ZERO);
    }
    d.move_to(0.2, cfg, 240., 8000);
    peak = 0.;
    for _ in 0..300 {
        peak = peak.max(d.sample(pcm, cfg, 8000).peak());
    }
    assert!(peak > 0.01 && !d.state.returning);
    d.grab(false, None, cfg, 8000);
    peak = 0.;
    for _ in 0..200 {
        peak = peak.max(d.sample(pcm, cfg, 8000).peak());
    }
    assert!(peak > 0.01 && d.rate > 0. && !d.state.grabbed);
}
#[test]
fn legacy_and_new_tracks_default_to_bidirectional_with_optional_down_cut() {
    let legacy: Settings = serde_json::from_str(r#"{"speed":1.0,"range":1,"slip":true}"#).unwrap();
    assert!(!legacy.reverse_cut);
    assert!(!Settings::default().reverse_cut);
    let old:loop_club::deck::Control=serde_json::from_str(r#"{"head":0,"slip":0,"target":0,"anchor":0,"fader":0.5,"anchor_fader":0.5,"grabbed":true}"#).unwrap();
    assert!(!old.returning);
    let mut e = core();
    e.command(Command::DeckValue(0, 3, 1.)).unwrap();
    let mut u = Ui::default();
    u.navigate(Page::Play, &Snapshot::capture(&e));
    u.soft(1, true, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    assert!(!e.tracks[0].deck.reverse_cut);
}

#[test]
fn project10_preserves_cut_option_and_muted_return_controls() {
    let mut e = core();
    e.tracks[0].deck.reverse_cut = true;
    e.tracks[0].audio_sequence = true;
    e.tracks[0].events.push(Event {
        deck: Some(loop_club::deck::Control {
            grabbed: true,
            returning: true,
            fader: 0.9,
            anchor_fader: 0.1,
            target: 1000.,
            head: 100.,
            ..Default::default()
        }),
        timing: None,
        tick: 0,
        note: 0,
        velocity: 0,
        on: false,
        slice: false,
    });
    let file = std::env::temp_dir().join(format!("loop-down-cut-{}", std::process::id()));
    let _ = std::fs::remove_file(&file);
    loop_club::project::save(&e, &file).unwrap();
    let loaded = loop_club::project::load(&file, 8000, 16000).unwrap();
    std::fs::remove_file(&file).unwrap();
    assert!(loaded.tracks[0].deck.reverse_cut);
    assert!(loaded.tracks[0].events[0].deck.unwrap().returning);
    assert_eq!(loaded.tracks[0].audio[..8000], e.tracks[0].audio[..8000]);
}

#[test]
fn negative_subsample_loop_boundary_never_indexes_the_pcm_endpoint() {
    let pcm = vec![Frame::mono(0.2); 11025];
    let cfg = Settings::default();
    for head in [
        -f64::MIN_POSITIVE,
        -f64::EPSILON,
        -1e-14,
        -11025. - 1e-12,
        11025.,
        0.,
    ] {
        let mut d = Player::default();
        d.start(head);
        d.grab(true, Some(0.5), cfg, 44100);
        for _ in 0..1000 {
            assert!(d.sample(&pcm, cfg, 44100).finite());
        }
    }
}
