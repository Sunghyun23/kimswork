use loop_club::{
    engine::{Command, Engine},
    model::*,
    ui::{Snapshot, Ui},
};
fn core() -> Engine {
    let mut e = Engine::new(8000, 240., 24000);
    e.preroll = 0;
    e
}
#[test]
fn part_generation_preserves_other_drums_and_mix() {
    let mut e = core();
    e.tracks[0].kind = Kind::Drum;
    e.tracks[0].mix.level = 0.32;
    e.tracks[0].fx[0].effect = Effect::Drive;
    e.command(Command::GenerateDrumPart { track: 0, sound: 0 })
        .unwrap();
    let kick: Vec<_> = e.tracks[0]
        .events
        .iter()
        .filter(|n| n.note == 0)
        .copied()
        .collect();
    e.command(Command::GenerateDrumPart { track: 0, sound: 1 })
        .unwrap();
    assert_eq!(
        kick,
        e.tracks[0]
            .events
            .iter()
            .filter(|n| n.note == 0)
            .copied()
            .collect::<Vec<_>>()
    );
    assert_eq!(e.tracks[0].mix.level, 0.32);
    assert_eq!(e.tracks[0].fx[0].effect, Effect::Drive);
    assert_eq!(e.tracks[0].length_ticks, 4 * PPQ);
    assert!(
        e.tracks[0]
            .events
            .iter()
            .filter(|n| n.on)
            .all(|n| n.note <= 1)
    );
}
#[test]
fn recorded_part_is_protected_while_new_parts_can_be_added() {
    let mut e = core();
    let t = &mut e.tracks[0];
    t.kind = Kind::Drum;
    t.length_ticks = 4 * PPQ;
    t.events.extend([
        Event {
            tick: PPQ,
            note: 1,
            velocity: 87,
            on: true,
        },
        Event {
            tick: PPQ + 10,
            note: 1,
            velocity: 0,
            on: false,
        },
    ]);
    let original = t.events.clone();
    assert_eq!(
        e.command(Command::GenerateDrumPart { track: 0, sound: 1 }),
        Err(Error::NotEmpty)
    );
    assert_eq!(e.tracks[0].events, original);
    e.command(Command::GenerateDrumPart { track: 0, sound: 0 })
        .unwrap();
    assert_eq!(
        e.tracks[0]
            .events
            .iter()
            .filter(|n| n.note == 1)
            .copied()
            .collect::<Vec<_>>(),
        original
    );
}
#[test]
fn every_sound_can_generate_a_nonempty_lane() {
    let mut e = core();
    e.tracks[0].kind = Kind::Drum;
    for sound in 0..16 {
        e.command(Command::GenerateDrumPart { track: 0, sound })
            .unwrap();
        assert!(
            e.tracks[0]
                .events
                .iter()
                .any(|n| n.note == sound as u8 && n.on)
        );
    }
}
#[test]
fn generator_does_not_restart_phase_or_other_track() {
    let mut e = core();
    e.tracks[0].kind = Kind::Drum;
    e.command(Command::GenerateDrumPart { track: 0, sound: 0 })
        .unwrap();
    for _ in 0..4300 {
        e.process(Frame::ZERO);
    }
    let origin = e.tracks[0].origin;
    let frame = e.clock.frame;
    e.tracks[1].length_ticks = 4 * PPQ;
    e.tracks[1].playing = true;
    e.command(Command::GenerateDrumPart { track: 0, sound: 0 })
        .unwrap();
    assert_eq!(e.clock.frame, frame);
    assert_eq!(e.tracks[0].origin, origin);
    assert!(e.tracks[1].playing);
}
#[test]
fn drum_shortcut_is_in_drum_play_not_main() {
    let mut e = core();
    e.tracks[0].kind = Kind::Drum;
    let mut ui = Ui::default();
    ui.pad(0, true, 80, true, false, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    assert!(e.tracks[0].empty());
    ui.page = Page::Play;
    ui.pad(1, true, 80, true, false, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    assert!(!e.tracks[0].empty());
    assert!(e.tracks[0].events.iter().all(|n| n.note == 1));
}
#[test]
fn generated_lane_undo_restores_only_last_change() {
    let mut e = core();
    e.tracks[0].kind = Kind::Drum;
    e.command(Command::GenerateDrumPart { track: 0, sound: 0 })
        .unwrap();
    let before = e.tracks[0].events.clone();
    e.command(Command::GenerateDrumPart { track: 0, sound: 1 })
        .unwrap();
    e.command(Command::UndoDrumPart { track: 0, sound: 1 })
        .unwrap();
    assert_eq!(e.tracks[0].events, before);
}
#[test]
fn chord_release_uses_notes_before_key_change_and_records_real_midi() {
    let mut e = core();
    e.tracks[0].kind = Kind::Synth;
    e.command(Command::Record(0)).unwrap();
    e.command(Command::Chord {
        track: 0,
        pad: 0,
        velocity: 73,
        on: true,
    })
    .unwrap();
    for _ in 0..500 {
        e.process(Frame::ZERO);
    }
    e.command(Command::ChordValue {
        track: 0,
        index: 0,
        value: 3.,
    })
    .unwrap();
    e.command(Command::Chord {
        track: 0,
        pad: 0,
        velocity: 0,
        on: false,
    })
    .unwrap();
    e.command(Command::Record(0)).unwrap();
    e.command(Command::Record(0)).unwrap();
    for _ in 0..7501 {
        e.process(Frame::ZERO);
    }
    let on: Vec<_> = e.tracks[0]
        .events
        .iter()
        .filter(|n| n.on)
        .map(|n| (n.note, n.velocity))
        .collect();
    let off: Vec<_> = e.tracks[0]
        .events
        .iter()
        .filter(|n| !n.on)
        .map(|n| n.note)
        .collect();
    assert_eq!(on, vec![(48, 73), (52, 73), (55, 73), (59, 73)]);
    assert_eq!(off, vec![48, 52, 55, 59]);
    for _ in 0..14000 {
        e.process(Frame::ZERO);
    }
    assert_eq!(e.active_voices(), 0);
}
#[test]
fn chop_playback_leaves_source_unchanged_and_release_is_owned() {
    let mut e = core();
    let t = &mut e.tracks[0];
    t.length_ticks = 4 * PPQ;
    t.audio_frames = 8000;
    t.source_frames = 8000;
    t.audio[..8000].fill(Frame::mono(0.2));
    t.chops.divide(8000);
    t.chops.gate = true;
    let original = t.audio.clone();
    e.command(Command::Slice {
        track: 0,
        pad: 0,
        velocity: 100,
        on: true,
    })
    .unwrap();
    let peak = (0..100)
        .map(|_| e.process(Frame::ZERO).peak())
        .fold(0f32, f32::max);
    assert!(peak > 0.01);
    e.command(Command::Slice {
        track: 0,
        pad: 0,
        velocity: 0,
        on: false,
    })
    .unwrap();
    for _ in 0..100 {
        e.process(Frame::ZERO);
    }
    assert_eq!(e.slice_activity(0), 0);
    assert_eq!(e.tracks[0].audio, original);
}
#[test]
fn chop_and_generated_state_roundtrip() {
    let mut e = core();
    e.tracks[0].kind = Kind::Drum;
    e.command(Command::GenerateDrumPart { track: 0, sound: 1 })
        .unwrap();
    e.command(Command::Stop).unwrap();
    let t = &mut e.tracks[1];
    t.audio_frames = 8000;
    t.source_frames = 7000;
    t.length_ticks = 4 * PPQ;
    t.chops.divide(7000);
    t.chops.slices[3].start += 10;
    let path = std::env::temp_dir().join(format!("loopclub-modes-{}.lcproj", std::process::id()));
    loop_club::project::save(&e, &path).unwrap();
    let loaded = loop_club::project::load(&path, 8000, 24000).unwrap();
    assert_eq!(loaded.tracks[0].generated_sounds, 2);
    assert_eq!(loaded.tracks[1].source_frames, 7000);
    assert_eq!(
        loaded.tracks[1].chops.slices[3].start,
        e.tracks[1].chops.slices[3].start
    );
    std::fs::remove_file(path).unwrap();
}
#[test]
fn new_generated_part_repeats_inside_existing_three_bar_loop() {
    let mut e = core();
    let t = &mut e.tracks[0];
    t.kind = Kind::Drum;
    t.length_ticks = 12 * PPQ;
    t.events.extend([
        Event {
            tick: PPQ,
            note: 1,
            velocity: 90,
            on: true,
        },
        Event {
            tick: PPQ + 20,
            note: 1,
            velocity: 0,
            on: false,
        },
    ]);
    let old = t.events.clone();
    e.command(Command::GenerateDrumPart { track: 0, sound: 0 })
        .unwrap();
    assert_eq!(e.tracks[0].length_ticks, 12 * PPQ);
    assert_eq!(
        old,
        e.tracks[0]
            .events
            .iter()
            .filter(|e| e.note == 1)
            .copied()
            .collect::<Vec<_>>()
    );
    let first: Vec<_> = e.tracks[0]
        .events
        .iter()
        .filter(|e| e.note == 0 && e.tick < 4 * PPQ)
        .map(|e| e.tick)
        .collect();
    let second: Vec<_> = e.tracks[0]
        .events
        .iter()
        .filter(|e| e.note == 0 && e.tick >= 4 * PPQ && e.tick < 8 * PPQ)
        .map(|e| e.tick - 4 * PPQ)
        .collect();
    assert_eq!(first, second);
}
#[test]
fn chop_modes_and_manual_undo_preserve_audio() {
    use loop_club::chop::Mode;
    let mut e = core();
    let t = &mut e.tracks[0];
    t.audio_frames = 8000;
    t.source_frames = 8000;
    t.length_ticks = 4 * PPQ;
    for i in 0..8000 {
        let sample = if i % 2000 < 50 { 0.8 } else { 0. };
        t.audio[i] = Frame::mono(sample);
        t.features.feed(Frame::mono(sample), 8000);
    }
    t.chops.divide(8000);
    let original = t.audio.clone();
    for mode in [Mode::Equal, Mode::Transient, Mode::Bpm, Mode::Manual] {
        e.tracks[0].chops.mode = mode;
        e.tracks[0].chops.regions = 4;
        e.command(Command::ChopApply(0)).unwrap();
        assert!(e.tracks[0].chops.valid(8000));
        assert_eq!(e.tracks[0].audio, original);
    }
    assert_eq!(e.tracks[0].chops.count, 1);
    e.command(Command::SplitSlice {
        track: 0,
        pad: 0,
        at: 4000,
    })
    .unwrap();
    assert_eq!(e.tracks[0].chops.count, 2);
    e.command(Command::DeleteSlice { track: 0, pad: 0 })
        .unwrap();
    assert_eq!(e.tracks[0].chops.slices[0].end, 8000);
    e.command(Command::ChopUndo(0)).unwrap();
    assert_eq!(e.tracks[0].chops.count, 2);
    assert_eq!(e.tracks[0].audio, original);
}
#[test]
fn transient_chops_find_separate_attacks() {
    let mut features = loop_club::features::Features::default();
    for i in 0..8000 {
        features.feed(Frame::mono(if i % 2000 < 100 { 0.7 } else { 0. }), 8000);
    }
    let mut chops = loop_club::chop::Chops {
        mode: loop_club::chop::Mode::Transient,
        regions: 8,
        sensitivity: 1.,
        ..loop_club::chop::Chops::default()
    };
    assert!(chops.apply(8000, 8000, 240., &features));
    assert!(chops.count >= 4);
    assert!(chops.valid(8000));
}
#[test]
fn too_dense_bpm_grid_is_rejected_without_replacing_regions() {
    let mut chops = loop_club::chop::Chops::default();
    chops.divide(16000);
    chops.mode = loop_club::chop::Mode::Bpm;
    chops.grid = 10;
    let before = chops.slices;
    assert!(!chops.apply(16000, 8000, 240., &loop_club::features::Features::default()));
    assert_eq!(chops.slices[15].end, before[15].end);
}
#[test]
fn temporary_effects_are_not_saved_as_track_inserts() {
    let mut e = core();
    e.command(Command::FxSelect {
        target: 0,
        track: 0,
        slot: 0,
        effect: Effect::Filter,
        temporary: true,
    })
    .unwrap();
    assert_eq!(e.saved_track_fx(0)[0].effect, Effect::Off);
    e.release_temporary_fx();
    assert_eq!(e.tracks[0].fx[0].effect, Effect::Off);
}

#[test]
fn qa_save_load_directory_cannot_replace_normal_session() {
    let base = std::path::Path::new("/card/LoopClub/Projects");
    let normal = loop_club::project::session_directory(base, false);
    let qa = loop_club::project::session_directory(base, true);
    assert_eq!(normal, base);
    assert_eq!(qa, base.join("QA"));
    assert_ne!(normal.join("Session.lcproj"), qa.join("Session.lcproj"));
}
