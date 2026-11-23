use loop_club::{
    engine::{Command, Engine},
    model::*,
    swing::Basis,
    ui::{Snapshot, Ui},
};
fn swing(n: f32) -> Basis {
    Basis {
        amount: n,
        sixteenth: false,
    }
}
fn event(tick: u64, on: bool) -> Event {
    Event {
        tick,
        on,
        note: 60,
        velocity: if on { 90 } else { 0 },
        slice: false,
        deck: None,
        timing: None,
    }
}
#[test]
fn eighth_and_sixteenth_swing_have_exact_boundaries_and_invert() {
    for sixteenth in [false, true] {
        for amount in [50., 56., 66., 75.] {
            let b = Basis { amount, sixteenth };
            let pair = if sixteenth { PPQ / 2 } else { PPQ };
            assert_eq!(b.warp(pair, false), pair);
            assert_eq!(
                b.warp(pair / 2, false),
                (pair as f64 * f64::from(amount) / 100.).round() as u64
            );
            for n in 0..2000 {
                let t = n * PPQ / 99;
                assert!(b.warp(b.warp(t, false), true).abs_diff(t) <= 1);
            }
        }
    }
}
#[test]
fn existing_swing_is_replaced_and_recording_does_not_get_swung_twice() {
    let t = PPQ / 2;
    let mut e = event(swing(56.).warp(t, false), true);
    e.timing = Some(swing(56.));
    let out = swing(70.).event(e, 4 * PPQ);
    assert!(out.tick.abs_diff(swing(70.).warp(t, false)) <= 1);
    assert_eq!(swing(56.).event(e, 4 * PPQ).tick, e.tick);
}
#[test]
fn all_midi_kinds_schedule_later_without_modifying_source_events() {
    for kind in [Kind::Synth, Kind::Drum, Kind::Audio] {
        let mut e = Engine::new(8000, 240., 16000);
        e.preroll = 0;
        let t = &mut e.tracks[0];
        t.kind = kind;
        t.length_ticks = 4 * PPQ;
        t.playing = true;
        if kind == Kind::Audio {
            t.audio_sequence = true;
            t.source_frames = 800;
            t.audio_frames = 800;
            t.source_ticks = 4 * PPQ;
            t.audio[..800].fill(Frame::mono(0.2));
        }
        let mut note = event(PPQ / 2, true);
        if kind == Kind::Drum {
            note.note = 0;
        }
        t.events.push(note);
        let mut off = note;
        off.tick = PPQ * 3 / 4;
        off.on = false;
        t.events.push(off);
        let original = t.events.clone();
        e.command(Command::Swing(Some(swing(75.)))).unwrap();
        e.running = true;
        for _ in 0..1450 {
            e.process(Frame::ZERO);
        }
        assert_eq!(
            e.active_voices(),
            0,
            "{kind:?} should not start before swing beat"
        );
        let peak = (0..250)
            .map(|_| e.process(Frame::ZERO).peak())
            .fold(0., f32::max);
        assert!(peak > 0.001, "{kind:?}");
        assert_eq!(e.tracks[0].events, original);
    }
}
#[test]
fn project_timing_and_ui_knobs_roundtrip_and_invalid_values_are_refused() {
    let mut e = Engine::new(8000, 240., 16000);
    let mut ui = Ui::default();
    ui.page = Page::Setup;
    ui.knob(2, 0.8, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    assert_eq!(e.swing, Some(swing(70.)));
    ui.shift_view = true;
    ui.knob(2, 1., &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    assert!(e.swing.unwrap().sixteenth);
    let dir = std::env::temp_dir().join(format!("loop-swing-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    loop_club::project::save(&e, &dir).unwrap();
    let restored = loop_club::project::load(&dir, 8000, 16000).unwrap();
    assert_eq!(restored.swing, e.swing);
    std::fs::remove_file(&dir).unwrap();
    assert!(e.command(Command::Swing(Some(swing(f32::NAN)))).is_err());
    assert!(e.command(Command::Swing(Some(swing(90.)))).is_err());
}
#[test]
fn raw_audio_remains_identical_with_global_swing() {
    let setup = || {
        let mut e = Engine::new(8000, 240., 16000);
        let t = &mut e.tracks[0];
        t.length_ticks = 4 * PPQ;
        t.audio_frames = 8000;
        t.source_frames = 8000;
        t.source_ticks = 4 * PPQ;
        t.playing = true;
        for (i, x) in t.audio[..8000].iter_mut().enumerate() {
            *x = Frame::mono((i as f32 * 0.07).sin() * 0.2);
        }
        e.running = true;
        e
    };
    let mut a = setup();
    let mut b = setup();
    b.command(Command::Swing(Some(swing(75.)))).unwrap();
    for _ in 0..12000 {
        assert_eq!(a.process(Frame::ZERO), b.process(Frame::ZERO));
    }
}

#[test]
fn swing_edit_restores_a_live_mono_owner_when_playback_had_priority() {
    let mut e = Engine::new(8000, 240., 16000);
    e.tracks[0].kind = Kind::Synth;
    e.tracks[0].play.voice = 1;
    e.tracks[0].synth.release = 0.01;
    for (note, source) in [(48, 100), (60, 1)] {
        e.command(Command::Note {
            track: 0,
            note,
            source,
            on: true,
            velocity: 90,
        })
        .unwrap();
    }
    e.command(Command::Swing(Some(swing(66.)))).unwrap();
    for _ in 0..2000 {
        e.process(Frame::ZERO);
    }
    assert_eq!(e.active_voices(), 1);
    e.command(Command::Note {
        track: 0,
        note: 48,
        source: 100,
        on: false,
        velocity: 0,
    })
    .unwrap();
    for _ in 0..2000 {
        e.process(Frame::ZERO);
    }
    assert_eq!(e.active_voices(), 0);
}
#[test]
fn only_lanes_with_baked_generator_swing_carry_a_timing_basis() {
    use loop_club::music::{PatternConfig, event_tick, generate_sound, swing_basis};
    let mut config = PatternConfig {
        rhythm: 3,
        swing: 70.,
        nudge_ms: 0.,
        snare_late_ms: 0.,
        ..PatternConfig::default()
    };
    for lane in [2, 12, 14] {
        let baked = generate_sound(config, 100., lane);
        config.swing = 50.;
        let straight = generate_sound(config, 100., lane);
        config.swing = 70.;
        assert_eq!(baked.len, straight.len);
        let basis = swing_basis(config, lane).expect("hat-derived lane has baked swing");
        for (a, b) in baked.hits[..baked.len]
            .iter()
            .zip(&straight.hits[..straight.len])
        {
            assert!(
                basis
                    .warp(event_tick(a.tick), true)
                    .abs_diff(event_tick(b.tick))
                    < PPQ / 960,
                "lane {lane}"
            );
        }
    }
    assert!(
        swing_basis(config, 3).is_none(),
        "dedicated open-hat generator is straight"
    );
}
#[test]
fn old_project_generator_timing_metadata_is_repaired_without_rewriting_events() {
    for version in [8, 9] {
        let mut e = Engine::new(8000, 240., 16000);
        e.tracks[0].kind = Kind::Drum;
        e.tracks[0].pattern.swing = 70.;
        for sound in [3, 12] {
            e.command(Command::GenerateDrumPart { track: 0, sound })
                .unwrap();
        }
        e.command(Command::Stop).unwrap();
        let file = std::env::temp_dir().join(format!("lc-timing-legacy-{}", std::process::id()));
        let _ = std::fs::remove_file(&file);
        loop_club::project::save(&e, &file).unwrap();
        let b = std::fs::read(&file).unwrap();
        let size = u32::from_le_bytes(b[4..8].try_into().unwrap()) as usize;
        let mut state: serde_json::Value = serde_json::from_slice(&b[8..8 + size]).unwrap();
        state.as_object_mut().unwrap().remove("timing_revision");
        state["version"] = version.into();
        state["tracks"][0]["pattern"]["rhythm"] = 6.into();
        for event in state["tracks"][0]["events"].as_array_mut().unwrap() {
            if event["note"] == 3 {
                event["timing"] = serde_json::json!({"amount":70.,"sixteenth":false});
            }
            if event["note"] == 12 {
                event.as_object_mut().unwrap().remove("timing");
            }
        }
        let meta = serde_json::to_vec(&state).unwrap();
        let mut out = b[..4].to_vec();
        out.extend((meta.len() as u32).to_le_bytes());
        out.extend(meta);
        out.extend(&b[8 + size..]);
        std::fs::write(&file, out).unwrap();
        let restored = loop_club::project::load(&file, 8000, 16000).unwrap();
        std::fs::remove_file(file).unwrap();
        assert!(restored.swing.is_none());
        for (a, b) in restored.tracks[0].events.iter().zip(&e.tracks[0].events) {
            assert_eq!(
                (a.tick, a.note, a.on, a.velocity),
                (b.tick, b.note, b.on, b.velocity)
            );
            if a.note == 3 {
                assert!(a.timing.is_none());
            }
            if a.note == 12 {
                assert_eq!(a.timing, Some(swing(70.)));
            }
        }
    }
}
