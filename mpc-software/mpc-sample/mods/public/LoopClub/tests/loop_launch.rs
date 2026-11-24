use loop_club::{
    engine::{Command, Engine},
    model::*,
};
fn core() -> Engine {
    let mut e = Engine::new(8000, 240., 32000);
    e.preroll = 1;
    e.tracks[1].kind = Kind::Synth;
    e.tracks[2].kind = Kind::Synth;
    e
}
fn source(e: &mut Engine, kind: Kind) {
    let t = &mut e.tracks[0];
    t.kind = kind;
    t.length_ticks = 8 * PPQ;
    if kind == Kind::Audio {
        t.audio_frames = 16000;
        t.source_frames = 16000;
        t.audio[..16000].fill(Frame::mono(0.2));
    } else {
        t.events.push(Event {
            tick: 0,
            note: 48,
            velocity: 100,
            on: true,
            slice: false,
            deck: None,
            timing: None,
        });
        t.events.push(Event {
            tick: PPQ / 4,
            note: 48,
            velocity: 0,
            on: false,
            slice: false,
            deck: None,
            timing: None,
        });
    }
}
#[test]
fn additional_recording_waits_for_two_bar_loop_start_without_another_preroll() {
    let mut e = core();
    source(&mut e, Kind::Synth);
    e.running = true;
    e.tracks[0].playing = true;
    e.clock.frame = 10000;
    e.command(Command::LoopRecord(1)).unwrap();
    let r = e.recorder.unwrap();
    assert_eq!(r.start, 8 * PPQ);
    assert_eq!(r.count_in_start, None);
    assert!(!r.launch_on_start);
    assert_eq!(e.tracks[0].phase(r.start), 0);
    while e.clock.tick() < r.start {
        assert_eq!(e.metronome_sample(), Frame::ZERO);
        e.process(Frame::ZERO);
    }
    assert_eq!(e.tracks[0].phase(e.clock.tick()), 0);
    assert_eq!(e.recorder.unwrap().track, 1);
}
#[test]
fn stopped_countin_is_silent_and_all_sources_launch_from_first_beat_with_capture() {
    for kind in [Kind::Synth, Kind::Drum, Kind::Audio] {
        let mut e = core();
        source(&mut e, kind);
        e.command(Command::LoopRecord(1)).unwrap();
        let r = e.recorder.unwrap();
        assert_eq!(r.start, 4 * PPQ);
        assert!(r.launch_on_start);
        while e.clock.tick() < r.start {
            assert_eq!(e.process(Frame::ZERO), Frame::ZERO);
            assert!(!e.tracks[0].playing);
        }
        let mut peak = 0f32;
        for _ in 0..128 {
            peak = peak.max(e.process(Frame::ZERO).peak());
        }
        assert!(peak > 0.0001, "{kind:?}");
        assert_eq!(e.tracks[0].origin, r.start);
        assert_eq!(e.tracks[0].phase(r.start), 0);
        assert!(e.tracks[0].playing);
        assert!(!e.recorder.unwrap().launch_on_start);
    }
}
#[test]
fn cancel_and_retarget_before_countin_do_not_launch_early_or_repeat_the_count() {
    let mut e = core();
    source(&mut e, Kind::Synth);
    e.command(Command::LoopRecord(1)).unwrap();
    let r = e.recorder.unwrap();
    for _ in 0..1000 {
        e.process(Frame::ZERO);
    }
    e.command(Command::LoopRecord(2)).unwrap();
    let next = e.recorder.unwrap();
    assert_eq!(next.start, r.start);
    assert_eq!(next.count_in_start, r.count_in_start);
    assert!(next.launch_on_start);
    e.command(Command::LoopRecord(2)).unwrap();
    assert!(e.recorder.is_none());
    for _ in 0..9000 {
        assert_eq!(e.process(Frame::ZERO), Frame::ZERO);
    }
    assert!(!e.tracks[0].playing);
}
#[test]
fn replacing_or_overdubbing_shorter_clip_uses_longer_reference_cycle() {
    for mode in [RecordMode::Replace, RecordMode::Overdub] {
        let mut e = core();
        source(&mut e, Kind::Synth);
        e.tracks[0].playing = true;
        e.tracks[1].length_ticks = 4 * PPQ;
        e.tracks[1].playing = true;
        e.tracks[1].record_mode = mode;
        e.running = true;
        e.clock.frame = 4000;
        e.command(Command::LoopRecord(1)).unwrap();
        assert_eq!(e.recorder.unwrap().start, 8 * PPQ);
        assert_eq!(e.recorder.unwrap().count_in_start, None);
    }
}
#[test]
fn first_downbeat_note_is_captured_once_and_launches_other_loops() {
    let mut e = core();
    source(&mut e, Kind::Synth);
    e.preroll = 0;
    e.command(Command::LoopRecord(1)).unwrap();
    e.command(Command::Note {
        track: 1,
        note: 60,
        velocity: 80,
        on: true,
        source: 2,
    })
    .unwrap();
    e.process(Frame::ZERO);
    assert_eq!(e.visible_events(1).iter().filter(|n| n.on).count(), 1);
    assert!(e.tracks[0].playing);
}
