use loop_club::{
    engine::{Command, Engine},
    model::*,
    ui::{Snapshot, Ui},
};
fn core() -> Engine {
    let mut e = Engine::new(8000, 240., 40000);
    e.preroll = 0;
    for i in 0..3 {
        e.tracks[i].kind = Kind::Synth;
    }
    e
}
fn until(e: &mut Engine, frame: u64) {
    while e.clock.frame <= frame {
        e.process(Frame::mono(0.1));
    }
}
fn pad(u: &mut Ui, e: &mut Engine, i: usize, shift: bool, on: bool) {
    u.pad(
        i,
        on,
        if on { 85 } else { 0 },
        shift,
        false,
        &Snapshot::capture(e),
        |c| {
            e.command(c).unwrap();
        },
    );
}
#[test]
fn normal_loop_pads_play_without_recording_before_and_after_capture() {
    let mut e = core();
    let mut u = Ui::default();
    pad(&mut u, &mut e, 2, false, true);
    assert!(e.recorder.is_none());
    assert_eq!(e.active_voices(), 1);
    assert_eq!(u.selected, 0);
    pad(&mut u, &mut e, 2, false, false);
    pad(&mut u, &mut e, 1, true, true);
    assert_eq!(e.recorder.unwrap().track, 1);
    assert_eq!(u.page, Page::Loop);
    pad(&mut u, &mut e, 1, true, false);
    until(&mut e, 100);
    pad(&mut u, &mut e, 4, false, true);
    pad(&mut u, &mut e, 4, false, false);
    pad(&mut u, &mut e, 1, true, true);
    until(&mut e, 8000);
    assert!(e.recorder.is_none());
    pad(&mut u, &mut e, 7, false, true);
    assert!(e.recorder.is_none());
    assert_eq!(u.selected, 1);
    assert!(e.active_note_mask(1)[0] & (1 << 55) != 0);
}
#[test]
fn different_shift_pad_queues_without_switching_sound_until_bar_end() {
    let mut e = core();
    let mut u = Ui::default();
    pad(&mut u, &mut e, 0, true, true);
    until(&mut e, 2000);
    pad(&mut u, &mut e, 1, true, true);
    assert_eq!(e.next_record, Some(1));
    assert_eq!(e.selected, 0);
    assert_eq!(u.selected, 0);
    assert_eq!((Snapshot::capture(&e).words[12] >> 8) & 255, 2);
    pad(&mut u, &mut e, 4, false, true);
    assert!(e.active_note_mask(0)[0] & (1 << 52) != 0);
    until(&mut e, 8000);
    u.sync_loop_role(&Snapshot::capture(&e));
    assert_eq!(e.recorder.unwrap().track, 1);
    assert_eq!(e.recorder.unwrap().start, 4 * PPQ);
    assert_eq!(u.selected, 1);
    assert!(e.tracks[0].playing);
    assert_eq!(e.tracks[0].length_ticks, 4 * PPQ);
    pad(&mut u, &mut e, 4, false, false);
    pad(&mut u, &mut e, 5, false, true);
    assert!(e.active_note_mask(1)[0] & (1 << 53) != 0);
}
#[test]
fn transition_has_no_second_preroll_for_every_source_pair() {
    for from in [Kind::Audio, Kind::Synth, Kind::Drum] {
        for to in [Kind::Audio, Kind::Synth, Kind::Drum] {
            let mut e = core();
            e.preroll = 1;
            e.tracks[0].kind = from;
            e.tracks[1].kind = to;
            e.command(Command::LoopRecord(0)).unwrap();
            assert_eq!(e.recorder.unwrap().start, 4 * PPQ);
            until(&mut e, 9000);
            e.command(Command::LoopRecord(1)).unwrap();
            let end = e.recorder.unwrap().end.unwrap();
            let frame = e.clock.frame_at(end);
            until(&mut e, frame);
            let r = e.recorder.unwrap();
            assert_eq!(r.track, 1);
            assert_eq!(r.start, end);
            assert_eq!(e.preroll, 1);
            assert_eq!(e.selected, 1);
            assert_eq!(e.next_record, None);
        }
    }
}
#[test]
fn pending_target_is_replaced_and_same_track_requests_finish_only() {
    let mut e = core();
    e.command(Command::LoopRecord(0)).unwrap();
    until(&mut e, 100);
    e.command(Command::LoopRecord(1)).unwrap();
    e.command(Command::LoopRecord(2)).unwrap();
    assert_eq!(e.next_record, Some(2));
    e.command(Command::LoopRecord(0)).unwrap();
    assert_eq!(e.next_record, None);
    until(&mut e, 8000);
    assert!(e.recorder.is_none());
    assert_eq!(e.tracks[1].length_ticks, 0);
    assert_eq!(e.tracks[2].length_ticks, 0);
}
#[test]
fn stop_and_undo_cancel_pending_next_recording() {
    for command in [Command::Stop, Command::Undo] {
        let mut e = core();
        e.command(Command::LoopRecord(0)).unwrap();
        until(&mut e, 100);
        e.command(Command::LoopRecord(1)).unwrap();
        e.command(command).unwrap();
        assert_eq!(e.next_record, None);
        assert!(e.recorder.is_none());
        for _ in 0..8100 {
            e.process(Frame::ZERO);
        }
        assert!(e.recorder.is_none());
    }
}
#[test]
fn count_in_retarget_reuses_original_start_without_committing_empty_take() {
    let mut e = core();
    e.preroll = 2;
    e.command(Command::LoopRecord(0)).unwrap();
    let start = e.recorder.unwrap().start;
    until(&mut e, 100);
    e.command(Command::LoopRecord(1)).unwrap();
    assert_eq!(e.recorder.unwrap().track, 1);
    assert_eq!(e.recorder.unwrap().start, start);
    assert!(e.tracks[0].empty());
    assert_eq!(e.history_status(), 0);
}
#[test]
fn existing_overdub_can_handoff_at_next_bar_and_keeps_full_original_length() {
    let mut e = core();
    e.tracks[0].length_ticks = 16 * PPQ;
    e.tracks[0].record_mode = RecordMode::Overdub;
    e.command(Command::LoopRecord(0)).unwrap();
    until(&mut e, 100);
    e.command(Command::LoopRecord(1)).unwrap();
    assert_eq!(e.recorder.unwrap().end, Some(4 * PPQ));
    until(&mut e, 8000);
    assert_eq!(e.tracks[0].length_ticks, 16 * PPQ);
    assert_eq!(e.recorder.unwrap().track, 1);
}
#[test]
fn invalid_or_oversized_target_does_not_disturb_active_recording() {
    let mut e = core();
    e.command(Command::LoopRecord(0)).unwrap();
    until(&mut e, 100);
    assert_eq!(e.command(Command::LoopRecord(16)), Err(Error::InvalidTrack));
    e.tracks[1].length_ticks = 100 * PPQ;
    assert_eq!(e.command(Command::LoopRecord(1)), Err(Error::Capacity));
    assert_eq!(e.next_record, None);
    assert_eq!(e.recorder.unwrap().track, 0);
    assert_eq!(e.recorder.unwrap().end, None);
}
