use loop_club::{
    engine::{Command, Engine},
    model::*,
    ui::Snapshot,
};
fn core(kind: Kind) -> Engine {
    let mut e = Engine::new(8000, 240., 16000);
    e.preroll = 0;
    e.tracks[0].kind = kind;
    e
}
fn until(e: &mut Engine, frame: u64, value: f32) {
    while e.clock.frame <= frame {
        e.process(Frame::mono(value));
    }
}
fn record(e: &mut Engine, value: f32) {
    e.command(Command::LoopRecord(0)).unwrap();
    let start = e.recorder.unwrap().start;
    let start_frame = e.clock.frame_at(start);
    until(e, start_frame + 100, value);
    e.command(Command::LoopRecord(0)).unwrap();
    let end = e.clock.frame_at(e.recorder.unwrap().end.unwrap());
    until(e, end, value);
    assert!(e.recorder.is_none());
}
#[test]
fn completed_audio_record_undo_redo_preserves_pcm_chops_and_other_tracks() {
    let mut e = core(Kind::Audio);
    e.tracks[1].mix.level = 0.31;
    record(&mut e, 0.2);
    let frames = e.tracks[0].audio_frames;
    assert_eq!(e.tracks[0].audio[500], Frame::mono(0.2));
    e.command(Command::Undo).unwrap();
    assert!(e.tracks[0].empty());
    assert_eq!(e.tracks[0].audio_frames, 0);
    assert_eq!(e.command(Command::Undo), Err(Error::NoUndo));
    e.command(Command::Redo).unwrap();
    assert_eq!(e.tracks[0].audio_frames, frames);
    assert_eq!(e.tracks[0].audio[500], Frame::mono(0.2));
    assert!(e.tracks[0].chops.valid(frames));
    assert_eq!(e.tracks[1].mix.level, 0.31);
    assert_eq!(e.command(Command::Redo), Err(Error::NoRedo));
}
#[test]
fn replace_and_overdub_restore_prior_take_and_new_take_repeatedly() {
    for mode in [RecordMode::Replace, RecordMode::Overdub] {
        let mut e = core(Kind::Audio);
        record(&mut e, 0.2);
        e.command(Command::RecordMode(0, mode)).unwrap();
        record(&mut e, 0.3);
        let expected = if mode == RecordMode::Replace {
            0.3
        } else {
            0.5
        };
        for _ in 0..3 {
            assert!((e.tracks[0].audio[500].l - expected).abs() < 1e-6);
            e.command(Command::Undo).unwrap();
            assert_eq!(e.tracks[0].audio[500], Frame::mono(0.2));
            e.command(Command::Redo).unwrap();
        }
    }
}
#[test]
fn canceled_capture_keeps_prior_redo_and_does_not_undo_another_edit() {
    let mut e = core(Kind::Audio);
    record(&mut e, 0.2);
    e.command(Command::Undo).unwrap();
    e.command(Command::LoopRecord(0)).unwrap();
    assert_eq!(e.command(Command::Redo), Err(Error::Busy));
    e.command(Command::Undo).unwrap();
    assert!(e.recorder.is_none());
    e.command(Command::Redo).unwrap();
    assert_eq!(e.tracks[0].audio[500], Frame::mono(0.2));
}
#[test]
fn continuous_recording_is_one_undo_gesture_even_after_multiple_commits() {
    let mut e = core(Kind::Audio);
    record(&mut e, 0.2);
    e.command(Command::RecordMode(0, RecordMode::Overdub))
        .unwrap();
    e.command(Command::Record(0)).unwrap();
    let end = e.recorder.unwrap().end.unwrap();
    let frame = e.clock.frame_at(end + 8 * PPQ);
    until(&mut e, frame + 50, 0.1);
    assert!(e.recorder.is_some());
    e.command(Command::Undo).unwrap();
    assert!(e.recorder.is_none());
    assert_eq!(e.tracks[0].audio[500], Frame::mono(0.2));
    e.command(Command::Redo).unwrap();
    assert!(e.tracks[0].audio[500].l > 0.35);
}
#[test]
fn generated_pattern_undo_redo_restores_seed_and_lane_notes() {
    let mut e = core(Kind::Drum);
    e.command(Command::GenerateDrumPart { track: 0, sound: 0 })
        .unwrap();
    let first = e.tracks[0].events.clone();
    let seed = e.tracks[0].pattern.seed;
    e.command(Command::GenerateDrumPart { track: 0, sound: 1 })
        .unwrap();
    let second = e.tracks[0].events.clone();
    e.command(Command::Undo).unwrap();
    assert_eq!(e.tracks[0].events, first);
    assert_eq!(e.tracks[0].pattern.seed, seed);
    e.command(Command::Redo).unwrap();
    assert_eq!(e.tracks[0].events, second);
}
#[test]
fn note_edit_and_clear_have_redo_and_do_not_rewind_mix() {
    let mut e = core(Kind::Drum);
    e.command(Command::GenerateDrumPart { track: 0, sound: 0 })
        .unwrap();
    let old = e.tracks[0].events.clone();
    let index = old.iter().position(|e| e.on).unwrap();
    e.command(Command::Edit {
        track: 0,
        index,
        note: 42,
        velocity: 33,
        length: 80,
    })
    .unwrap();
    let edited = e.tracks[0].events.clone();
    e.tracks[0].mix.level = 0.22;
    e.command(Command::Undo).unwrap();
    assert_eq!(e.tracks[0].events, old);
    e.command(Command::Redo).unwrap();
    assert_eq!(e.tracks[0].events, edited);
    assert_eq!(e.tracks[0].mix.level, 0.22);
    e.command(Command::Clear(0)).unwrap();
    assert!(e.tracks[0].empty());
    e.command(Command::Undo).unwrap();
    assert_eq!(e.tracks[0].events, edited);
    e.command(Command::Redo).unwrap();
    assert!(e.tracks[0].empty());
}
#[test]
fn chops_undo_redo_leave_pcm_and_transport_state_intact() {
    let mut e = core(Kind::Audio);
    record(&mut e, 0.2);
    let end = e.tracks[0].chops.slices[0].end;
    e.command(Command::SliceValue {
        track: 0,
        pad: 0,
        index: 1,
        value: 0.01,
    })
    .unwrap();
    let short = e.tracks[0].chops.slices[0].end;
    assert!(short < end);
    e.command(Command::Stop).unwrap();
    e.command(Command::Undo).unwrap();
    assert_eq!(e.tracks[0].chops.slices[0].end, end);
    assert!(!e.tracks[0].playing);
    e.command(Command::Redo).unwrap();
    assert_eq!(e.tracks[0].chops.slices[0].end, short);
    assert_eq!(e.tracks[0].audio[500], Frame::mono(0.2));
}
#[test]
fn audio_clear_undo_retains_pcm_and_tempo_change_invalidates_stale_redo() {
    let mut e = core(Kind::Audio);
    record(&mut e, 0.2);
    e.command(Command::Clear(0)).unwrap();
    e.command(Command::Undo).unwrap();
    assert_eq!(e.tracks[0].audio[500], Frame::mono(0.2));
    e.command(Command::Redo).unwrap();
    assert!(e.tracks[0].empty());
    e.command(Command::Stop).unwrap();
    e.command(Command::Bpm(100.)).unwrap();
    assert_eq!(e.command(Command::Undo), Err(Error::NoUndo));
}
#[test]
fn history_is_global_but_does_not_change_selection_and_kind_change_invalidates() {
    let mut e = core(Kind::Audio);
    record(&mut e, 0.2);
    e.command(Command::Select(1)).unwrap();
    e.command(Command::Undo).unwrap();
    assert_eq!(e.selected, 1);
    assert!(e.tracks[0].empty());
    assert_eq!(Snapshot::capture(&e).words[1046], 2 | (1 << 8));
    e.command(Command::SetKind(0, Kind::Synth)).unwrap();
    assert_eq!(e.command(Command::Redo), Err(Error::NoRedo));
}
#[test]
fn new_committed_edit_replaces_redo_but_invalid_edit_does_not() {
    let mut e = core(Kind::Drum);
    e.command(Command::GenerateDrumPart { track: 0, sound: 0 })
        .unwrap();
    e.command(Command::Undo).unwrap();
    assert_eq!(
        e.command(Command::GenerateDrumPart {
            track: 0,
            sound: 16
        }),
        Err(Error::InvalidValue)
    );
    e.command(Command::Redo).unwrap();
    e.command(Command::Undo).unwrap();
    e.command(Command::GenerateDrumPart { track: 0, sound: 1 })
        .unwrap();
    assert_eq!(e.command(Command::Redo), Err(Error::NoRedo));
}

#[test]
fn recorded_synth_and_drum_events_undo_redo_with_different_loop_lengths() {
    for kind in [Kind::Synth, Kind::Drum] {
        let mut e = core(kind);
        for (pass, note) in [60, 64].into_iter().enumerate() {
            e.command(Command::LoopRecord(0)).unwrap();
            let start = e.clock.frame_at(e.recorder.unwrap().start);
            until(&mut e, start + 100, 0.);
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
            if pass == 0 {
                until(&mut e, start + 8100, 0.);
            }
            e.command(Command::LoopRecord(0)).unwrap();
            while e.recorder.is_some() {
                e.process(Frame::ZERO);
            }
        }
        let after = e.tracks[0].events.clone();
        assert_eq!(e.tracks[0].length_ticks, 4 * PPQ);
        e.command(Command::Undo).unwrap();
        assert_eq!(e.tracks[0].length_ticks, 8 * PPQ);
        assert!(e.tracks[0].events.iter().any(|e| e.on && e.note == 60));
        e.command(Command::Redo).unwrap();
        assert_eq!(e.tracks[0].events, after);
        assert_eq!(e.tracks[0].length_ticks, 4 * PPQ);
    }
}
