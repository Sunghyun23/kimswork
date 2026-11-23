use loop_club::{
    engine::{Command, Engine, MAX_EVENTS},
    model::*,
    ui::{Action, Snapshot, Ui},
};
fn event(note: u8, tick: u64, on: bool, slice: bool) -> Event {
    Event {
        note,
        tick,
        on,
        slice,
        velocity: if on { 77 } else { 0 },
        deck: None,
        timing: None,
    }
}
fn core(kind: Kind) -> Engine {
    let mut e = Engine::new(8000, 240., 16000);
    e.preroll = 0;
    let t = &mut e.tracks[0];
    t.kind = kind;
    t.length_ticks = 4 * PPQ;
    t.events.extend([
        event(0, 0, true, kind == Kind::Audio),
        event(0, PPQ / 4, false, kind == Kind::Audio),
        event(1, PPQ, true, kind == Kind::Audio),
        event(1, PPQ + PPQ / 4, false, kind == Kind::Audio),
    ]);
    if kind == Kind::Audio {
        t.audio_frames = 800;
        t.source_frames = 800;
        t.source_ticks = 4 * PPQ;
        t.audio[..800].fill(Frame::mono(0.2));
        t.chops.divide(800);
        t.audio_sequence = true;
        t.audio_target = loop_club::audio::RecordTarget::Chop;
    }
    e
}
fn edit(e: &mut Engine, source: u8, target: Option<u8>) -> Result<(), Error> {
    e.command(Command::PadNotes {
        track: 0,
        source,
        target,
    })
}
#[test]
fn drum_erase_and_copy_preserve_other_lanes_sound_timing_and_undo() {
    let mut e = core(Kind::Drum);
    let original = e.tracks[0].events.clone();
    e.tracks[0].drums[0].tune = 3.;
    edit(&mut e, 0, Some(2)).unwrap();
    assert_eq!(e.tracks[0].events.iter().filter(|x| x.note == 2).count(), 2);
    assert_eq!(
        e.tracks[0]
            .events
            .iter()
            .find(|x| x.note == 2 && x.on)
            .unwrap()
            .velocity,
        77
    );
    e.command(Command::Undo).unwrap();
    assert_eq!(e.tracks[0].events, original);
    e.command(Command::Redo).unwrap();
    edit(&mut e, 0, None).unwrap();
    assert!(!e.tracks[0].events.iter().any(|x| x.note == 0));
    assert!(e.tracks[0].events.iter().any(|x| x.note == 1));
    assert_eq!(e.tracks[0].drums[0].tune, 3.);
    e.command(Command::Undo).unwrap();
    assert!(e.tracks[0].events.iter().any(|x| x.note == 0));
}
#[test]
fn chop_notes_do_not_delete_regions_pcm_or_chromatic_notes() {
    let mut e = core(Kind::Audio);
    e.tracks[0].events.extend([
        event(0, 2 * PPQ, true, false),
        event(0, 2 * PPQ + 1, false, false),
    ]);
    let before = e.tracks[0].chops;
    edit(&mut e, 0, Some(2)).unwrap();
    edit(&mut e, 0, None).unwrap();
    assert!(e.tracks[0].events.iter().any(|x| x.note == 0 && !x.slice));
    assert!(!e.tracks[0].events.iter().any(|x| x.note == 0 && x.slice));
    assert_eq!(e.tracks[0].audio[400], Frame::mono(0.2));
    assert_eq!(e.tracks[0].source_frames, 800);
    assert_eq!(e.tracks[0].chops.slices[0].end, before.slices[0].end);
    assert_eq!(e.tracks[0].chops.count, before.count);
}
#[test]
fn occupied_capacity_invalid_and_wrong_kind_are_atomic() {
    let mut e = core(Kind::Drum);
    let original = e.tracks[0].events.clone();
    assert_eq!(edit(&mut e, 0, Some(1)), Err(Error::NotEmpty));
    assert_eq!(e.tracks[0].events, original);
    assert_eq!(edit(&mut e, 0, Some(0)), Err(Error::InvalidValue));
    assert_eq!(edit(&mut e, 16, None), Err(Error::InvalidValue));
    while e.tracks[0].events.len() < MAX_EVENTS {
        e.tracks[0].events.push(event(1, 1, false, false));
    }
    assert_eq!(edit(&mut e, 0, Some(2)), Err(Error::Capacity));
    assert_eq!(e.tracks[0].events.len(), MAX_EVENTS);
    e.tracks[0].kind = Kind::Synth;
    assert_eq!(edit(&mut e, 0, None), Err(Error::WrongKind));
}
#[test]
fn active_capture_edit_is_cancelable_and_does_not_change_completed_clip() {
    for kind in [Kind::Drum, Kind::Audio] {
        let mut e = core(kind);
        e.tracks[0].record_mode = RecordMode::Overdub;
        let old = e.tracks[0].events.clone();
        e.command(Command::LoopRecord(0)).unwrap();
        e.process(Frame::ZERO);
        edit(&mut e, 0, None).unwrap();
        edit(&mut e, 1, Some(2)).unwrap();
        assert_eq!(e.tracks[0].events, old);
        assert!(!e.visible_events(0).iter().any(|x| x.note == 0));
        assert!(e.visible_events(0).iter().any(|x| x.note == 2));
        e.command(Command::Undo).unwrap();
        assert!(e.recorder.is_none());
        assert_eq!(e.tracks[0].events, old);
    }
}
#[test]
fn capture_commit_and_undo_preserve_original_and_reject_open_gate_copy() {
    let mut e = core(Kind::Audio);
    e.tracks[0].record_mode = RecordMode::Overdub;
    let old = e.tracks[0].events.clone();
    e.command(Command::LoopRecord(0)).unwrap();
    e.process(Frame::ZERO);
    e.command(Command::Note {
        track: 0,
        note: 0,
        velocity: 88,
        on: true,
        source: 100,
    })
    .unwrap();
    assert_eq!(edit(&mut e, 0, Some(2)), Err(Error::Busy));
    e.command(Command::Note {
        track: 0,
        note: 0,
        velocity: 0,
        on: false,
        source: 100,
    })
    .unwrap();
    edit(&mut e, 0, None).unwrap();
    edit(&mut e, 1, Some(2)).unwrap();
    e.command(Command::LoopRecord(0)).unwrap();
    while e.recorder.is_some() {
        e.process(Frame::ZERO);
    }
    assert!(!e.tracks[0].events.iter().any(|x| x.note == 0 && x.slice));
    assert!(e.tracks[0].events.iter().any(|x| x.note == 2));
    e.command(Command::Undo).unwrap();
    assert_eq!(e.tracks[0].events, old);
    e.command(Command::Redo).unwrap();
    assert!(e.tracks[0].events.iter().any(|x| x.note == 2));
}
#[test]
fn edit_toggle_done_and_source_dest_do_not_play_or_arm_record() {
    let e = core(Kind::Drum);
    let s = Snapshot::capture(&e);
    let mut u = Ui::default();
    u.navigate(Page::Play, &s);
    assert!(u.pad_notes_button(9, false, &s));
    let mut c = Vec::new();
    assert_eq!(
        u.pad(0, true, 90, false, true, &s, |x| c.push(x)),
        Action::None
    );
    assert!(matches!(
        c.last(),
        Some(Command::PadNotes {
            source: 0,
            target: None,
            ..
        })
    ));
    assert!(!c.iter().any(|x| matches!(
        x,
        Command::Note { .. } | Command::LoopRecord(_) | Command::GenerateDrumPart { .. }
    )));
    assert!(u.pad_notes_button(9, false, &s));
    assert!(u.pad_notes_edit.is_none());
    assert!(u.pad_notes_button(9, true, &s));
    u.pad(0, true, 90, false, false, &s, |x| c.push(x));
    u.pad(2, true, 90, false, false, &s, |x| c.push(x));
    assert!(matches!(
        c.last(),
        Some(Command::PadNotes {
            source: 0,
            target: Some(2),
            ..
        })
    ));
    u.soft(2, false, &s, |_| {
        panic!("DONE must not send audio commands")
    });
    assert!(u.pad_notes_edit.is_none());
}
#[test]
fn generation_does_not_start_transport_or_recording_but_updates_running_loop() {
    let mut e = core(Kind::Drum);
    e.command(Command::GenerateDrumPart { track: 0, sound: 0 })
        .unwrap();
    assert!(!e.running);
    assert!(!e.tracks[0].playing);
    assert!(e.recorder.is_none());
    e.command(Command::PlayAll).unwrap();
    e.command(Command::GenerateDrumPart { track: 0, sound: 1 })
        .unwrap();
    assert!(e.running && e.tracks[0].playing);
    assert!(e.recorder.is_none());
    let mut u = Ui::default();
    let s = Snapshot::capture(&e);
    let mut c = Vec::new();
    u.pad(0, true, 90, true, false, &s, |x| c.push(x));
    assert!(matches!(c.last(), Some(Command::LoopRecord(0))));
    assert!(
        !c.iter()
            .any(|x| matches!(x, Command::GenerateDrumPart { .. }))
    );
    u.navigate(Page::Play, &s);
    c.clear();
    u.pad(0, true, 90, true, false, &s, |x| c.push(x));
    assert!(matches!(c.last(), Some(Command::GenerateDrumPart { .. })));
    assert!(!c.iter().any(|x| matches!(x, Command::LoopRecord(_))));
}
