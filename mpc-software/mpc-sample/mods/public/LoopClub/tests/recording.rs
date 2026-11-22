use loop_club::{
    engine::{Command, Engine},
    model::*,
    project,
    ui::{Snapshot, Ui},
};
fn core(kind: Kind) -> Engine {
    let mut e = Engine::new(8000, 240., 16000);
    e.preroll = 0;
    e.tracks[0].kind = kind;
    e
}
fn until(e: &mut Engine, frame: u64, x: f32) {
    while e.clock.frame < frame {
        e.process(Frame::mono(x));
    }
}
fn hit(e: &mut Engine, note: u8) {
    for on in [true, false] {
        e.command(Command::Note {
            track: 0,
            note,
            velocity: if on { 80 } else { 0 },
            on,
            source: 2,
        })
        .unwrap();
    }
}
fn begin(e: &mut Engine) {
    e.command(Command::Record(0)).unwrap();
    until(e, 100, 0.1);
    e.command(Command::Record(0)).unwrap(); // Establish one bar; recording continues.
}
#[test]
fn replace_default_repeats_and_rec_stops_only_capture() {
    let mut e = core(Kind::Audio);
    begin(&mut e);
    until(&mut e, 8001, 0.1);
    assert_eq!(e.status(0), Status::Record);
    assert!(e.recorder.unwrap().repeating);
    assert_eq!(e.tracks[0].audio[50], Frame::mono(0.1));
    e.command(Command::Record(0)).unwrap();
    until(&mut e, 16001, 0.3);
    assert!(e.recorder.is_none());
    assert!(e.running && e.tracks[0].playing);
    assert_eq!(e.tracks[0].audio[500], Frame::mono(0.3));
    assert_eq!(e.tracks[0].length_ticks, 4 * PPQ);
}
#[test]
fn audio_overdub_adds_previous_loop_mode_switch_is_boundary_latched() {
    let mut e = core(Kind::Audio);
    begin(&mut e);
    until(&mut e, 8100, 0.1);
    e.command(Command::RecordMode(0, RecordMode::Overdub))
        .unwrap();
    assert_eq!(e.recorder.unwrap().mode, RecordMode::Replace);
    until(&mut e, 16001, 0.2);
    assert_eq!(e.tracks[0].audio[500], Frame::mono(0.2));
    assert_eq!(e.recorder.unwrap().mode, RecordMode::Overdub);
    e.command(Command::Record(0)).unwrap();
    until(&mut e, 24001, 0.3);
    assert!((e.tracks[0].audio[500].l - 0.5).abs() < 1e-6);
}
#[test]
fn synth_replace_discards_previous_notes_overdub_keeps_them() {
    for mode in [RecordMode::Replace, RecordMode::Overdub] {
        let mut e = core(Kind::Synth);
        e.command(Command::RecordMode(0, mode)).unwrap();
        begin(&mut e);
        hit(&mut e, 60);
        until(&mut e, 8001, 0.);
        hit(&mut e, 64);
        e.command(Command::Record(0)).unwrap();
        until(&mut e, 16001, 0.);
        let notes: Vec<_> = e.tracks[0]
            .events
            .iter()
            .filter(|e| e.on)
            .map(|e| e.note)
            .collect();
        assert_eq!(
            notes,
            if mode == RecordMode::Replace {
                vec![64]
            } else {
                vec![64, 60]
            }
        );
    }
}
#[test]
fn drum_replace_touches_only_played_lane_and_does_not_accumulate() {
    let mut e = core(Kind::Drum);
    begin(&mut e);
    hit(&mut e, 0);
    until(&mut e, 200, 0.);
    hit(&mut e, 1);
    until(&mut e, 8001, 0.);
    let kick: Vec<_> = e.tracks[0]
        .events
        .iter()
        .filter(|e| e.note == 0)
        .copied()
        .collect();
    until(&mut e, 8500, 0.);
    hit(&mut e, 1);
    until(&mut e, 8700, 0.);
    hit(&mut e, 1);
    until(&mut e, 16001, 0.);
    assert_eq!(
        e.tracks[0]
            .events
            .iter()
            .filter(|e| e.note == 0)
            .copied()
            .collect::<Vec<_>>(),
        kick
    );
    assert_eq!(
        e.tracks[0]
            .events
            .iter()
            .filter(|e| e.note == 1 && e.on)
            .count(),
        2
    );
    hit(&mut e, 1);
    e.command(Command::Record(0)).unwrap();
    until(&mut e, 24001, 0.);
    assert_eq!(
        e.tracks[0]
            .events
            .iter()
            .filter(|e| e.note == 1 && e.on)
            .count(),
        1
    );
    assert_eq!(
        e.tracks[0]
            .events
            .iter()
            .filter(|e| e.note == 0)
            .copied()
            .collect::<Vec<_>>(),
        kick
    );
}
#[test]
fn drum_overdub_accumulates_same_lane_hits() {
    let mut e = core(Kind::Drum);
    e.command(Command::RecordMode(0, RecordMode::Overdub))
        .unwrap();
    begin(&mut e);
    hit(&mut e, 1);
    until(&mut e, 8001, 0.);
    hit(&mut e, 1);
    e.command(Command::Record(0)).unwrap();
    until(&mut e, 16001, 0.);
    assert_eq!(
        e.tracks[0]
            .events
            .iter()
            .filter(|e| e.note == 1 && e.on)
            .count(),
        2
    );
}
#[test]
fn cancelled_replace_preserves_completed_take_and_other_track() {
    let mut e = core(Kind::Audio);
    e.tracks[1].length_ticks = 4 * PPQ;
    e.tracks[1].audio_frames = 8000;
    e.tracks[1].audio[..8000].fill(Frame::mono(0.15));
    e.tracks[1].playing = true;
    begin(&mut e);
    until(&mut e, 8500, 0.1);
    e.command(Command::CancelRecord).unwrap();
    assert_eq!(e.tracks[0].audio[500], Frame::mono(0.1));
    assert!(e.tracks[0].playing && e.tracks[1].playing);
    assert_eq!(e.tracks[1].audio[500], Frame::mono(0.15));
}
#[test]
fn first_frame_at_midi_wrap_is_not_lost_and_held_notes_are_closed() {
    let mut e = core(Kind::Synth);
    begin(&mut e);
    until(&mut e, 8000, 0.);
    e.command(Command::Note {
        track: 0,
        note: 60,
        velocity: 90,
        on: true,
        source: 2,
    })
    .unwrap();
    until(&mut e, 16001, 0.);
    assert!(
        e.tracks[0]
            .events
            .iter()
            .any(|n| n.on && n.tick == 0 && n.note == 60)
    );
    assert!(
        e.tracks[0]
            .events
            .iter()
            .any(|n| !n.on && n.tick == 4 * PPQ - 1)
    );
    e.command(Command::Record(0)).unwrap();
    until(&mut e, 24001, 0.);
    assert_eq!(e.tracks[0].events.len(), 2);
}
#[test]
fn existing_track_can_reenter_record_without_destroying_other_loops() {
    let mut e = core(Kind::Audio);
    begin(&mut e);
    e.command(Command::Record(0)).unwrap();
    until(&mut e, 8001, 0.1);
    e.command(Command::Record(0)).unwrap();
    assert_eq!(e.recorder.unwrap().start, 8 * PPQ);
    e.command(Command::CancelRecord).unwrap();
    assert!(e.tracks[0].playing && !e.tracks[0].empty());
}
#[test]
fn button_is_per_track_and_mode_is_saved() {
    let mut e = core(Kind::Synth);
    let mut ui = Ui::default();
    for page in [Page::Loop, Page::Play] {
        ui.page = page;
        ui.soft(2, false, &Snapshot::capture(&e), |c| {
            e.command(c).unwrap();
        });
    }
    assert_eq!(e.tracks[0].record_mode, RecordMode::Replace);
    e.command(Command::RecordMode(0, RecordMode::Overdub))
        .unwrap();
    assert_eq!(e.tracks[1].record_mode, RecordMode::Replace);
    let path = std::env::temp_dir().join(format!("lc-record-mode-{}.lcproj", std::process::id()));
    project::save(&e, &path).unwrap();
    let loaded = project::load(&path, 8000, 16000).unwrap();
    assert_eq!(loaded.tracks[0].record_mode, RecordMode::Overdub);
    std::fs::remove_file(path).unwrap();
}

#[test]
fn audio_latency_keeps_last_take_until_complete_and_captures_every_wrap() {
    let mut e = core(Kind::Audio);
    e.latency_in = 16;
    begin(&mut e);
    until(&mut e, 8017, 0.1);
    assert_eq!(e.tracks[0].audio[7999], Frame::mono(0.1));
    until(&mut e, 16001, 0.2);
    assert!(e.recorder.unwrap().finish_started);
    e.command(Command::Record(0)).unwrap();
    until(&mut e, 16017, 0.3);
    assert!(e.recorder.is_none());
    assert_eq!(e.tracks[0].audio[7999], Frame::mono(0.3));
    assert_eq!(e.tracks[0].audio[500], Frame::mono(0.2));
}
#[test]
fn midi_capacity_failure_retains_last_complete_take() {
    let mut e = core(Kind::Drum);
    e.command(Command::RecordMode(0, RecordMode::Overdub))
        .unwrap();
    begin(&mut e);
    hit(&mut e, 0);
    until(&mut e, 8001, 0.);
    let before = e.tracks[0].events.clone();
    for _ in 0..2100 {
        hit(&mut e, 1);
    }
    assert!(e.recorder.is_none());
    assert_eq!(e.tracks[0].events, before);
    assert!(e.tracks[0].playing);
    assert!(e.event_overflows > 0);
}
