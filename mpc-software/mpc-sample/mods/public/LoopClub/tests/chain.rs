use loop_club::{
    chain::{Chain, Change, Step},
    engine::{Command, Engine},
    model::*,
    ui::{Snapshot, Ui},
};
fn engine() -> Engine {
    let mut e = Engine::new(8000, 240., 16000);
    e.preroll = 0;
    for i in 0..3 {
        e.tracks[i].kind = Kind::Synth;
        e.tracks[i].length_ticks = 4 * PPQ;
    }
    e
}
#[test]
fn chain_switches_only_members_and_completes_whole_loops() {
    let mut e = engine();
    e.tracks[2].playing = true;
    e.command(Command::ChainAppend(0)).unwrap();
    e.command(Command::ChainAppend(1)).unwrap();
    e.command(Command::ChainRepeats(0, 2)).unwrap();
    e.command(Command::ChainRepeat(false)).unwrap();
    e.command(Command::ChainToggle).unwrap();
    e.process(Frame::ZERO);
    assert!(e.tracks[0].playing && !e.tracks[1].playing && e.tracks[2].playing);
    for _ in 0..8000 {
        e.process(Frame::ZERO);
    }
    assert!(e.tracks[0].playing && e.tracks[2].playing);
    assert_eq!(e.chain.remaining, 1);
    for _ in 0..8000 {
        e.process(Frame::ZERO);
    }
    assert!(!e.tracks[0].playing && e.tracks[1].playing && e.tracks[2].playing);
    for _ in 0..8000 {
        e.process(Frame::ZERO);
    }
    assert!(!e.chain.running && !e.tracks[1].playing && e.tracks[2].playing);
}
#[test]
fn edits_wait_for_boundary_and_duplicate_steps_work() {
    let mut c = Chain::default();
    c.config.len = 3;
    c.config.steps[0] = Step {
        track: 0,
        repeats: 2,
    };
    c.config.steps[1] = Step {
        track: 1,
        repeats: 1,
    };
    c.config.steps[2] = Step {
        track: 1,
        repeats: 1,
    };
    c.start(0);
    assert!(matches!(
        c.tick(0, |_| 100),
        Some(Change::Activate { track: 0, .. })
    ));
    c.config.steps[0].repeats = 1;
    c.remove(0);
    assert_eq!(c.current, Some(0));
    assert!(c.tick(99, |_| 100).is_none());
    assert!(c.tick(100, |_| 100).is_none());
    assert_eq!(c.remaining, 1);
    assert!(matches!(
        c.tick(200, |_| 100),
        Some(Change::Activate { track: 1, .. })
    ));
    assert!(matches!(
        c.tick(300, |_| 100),
        Some(Change::Activate { track: 1, .. })
    ));
}
#[test]
fn boundaries_validate_capacity_empty_and_recording_conflicts() {
    let mut e = engine();
    assert!(e.command(Command::ChainAppend(5)).is_err());
    assert!(e.command(Command::ChainToggle).is_err());
    for _ in 0..16 {
        e.command(Command::ChainAppend(0)).unwrap();
    }
    assert!(e.command(Command::ChainAppend(0)).is_err());
    assert!(e.command(Command::ChainRepeats(0, 0)).is_err());
    e.command(Command::ChainToggle).unwrap();
    assert_eq!(e.command(Command::LoopRecord(0)), Err(Error::Busy));
    e.command(Command::ChainClear).unwrap();
    assert_eq!(e.chain.config.len, 0);
}
#[test]
fn mute_preserves_playhead_and_chain_controls_do_not_record() {
    let mut e = engine();
    e.running = true;
    e.tracks[0].playing = true;
    e.tracks[0].origin = 123;
    let mut u = Ui::default();
    u.soft(1, true, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    assert_eq!(u.loop_mode, 1);
    u.pad(0, true, 90, false, false, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    assert!(e.tracks[0].mix.mute && e.tracks[0].playing);
    assert_eq!(e.tracks[0].origin, 123);
    assert!(e.recorder.is_none());
    u.soft(2, true, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    assert_eq!(u.loop_mode, 2);
    u.pad(0, true, 90, false, false, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    assert_eq!(e.chain.config.len, 1);
    assert!(e.recorder.is_none());
    u.knob(0, 1., &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    assert_eq!(e.chain.config.steps[0].repeats, 16);
    assert_eq!(Snapshot::capture(&e).chain_config(), e.chain.config);
}
#[test]
fn project_restores_chain_configuration_but_never_autoplays() {
    let mut e = engine();
    e.command(Command::ChainAppend(0)).unwrap();
    e.command(Command::ChainAppend(0)).unwrap();
    e.command(Command::ChainToggle).unwrap();
    let path = std::env::temp_dir().join(format!("lc-chain-{}", std::process::id()));
    let _ = std::fs::remove_file(&path);
    e.command(Command::Stop).unwrap();
    loop_club::project::save(&e, &path).unwrap();
    let loaded = loop_club::project::load(&path, 8000, 16000).unwrap();
    assert_eq!(loaded.chain.config, e.chain.config);
    assert!(!loaded.chain.running);
    std::fs::remove_file(path).unwrap();
}

#[test]
fn mode_shortcuts_keep_sound_and_record_mode_accessible() {
    let mut e = engine();
    let mut u = Ui::default();
    for mode in 0..3 {
        u.page = Page::Loop;
        u.soft(mode, true, &Snapshot::capture(&e), |c| {
            e.command(c).unwrap();
        });
        assert_eq!(u.loop_mode, mode as u8);
        u.soft(2, false, &Snapshot::capture(&e), |c| {
            e.command(c).unwrap();
        });
        assert!(e.recorder.is_none());
        u.soft(1, false, &Snapshot::capture(&e), |c| {
            e.command(c).unwrap();
        });
        assert_eq!(u.page, Page::Play);
    }
    u.page = Page::Loop;
    e.command(Command::LoopRecord(0)).unwrap();
    u.loop_perform = true;
    u.soft(1, true, &Snapshot::capture(&e), |_| {
        panic!("must not dispatch")
    });
    assert_eq!(u.loop_mode, 2);
}
