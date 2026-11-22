use loop_club::{
    engine::{Command, Engine},
    model::*,
    runtime::Runtime,
    ui::Snapshot,
};
use std::sync::atomic::Ordering;
fn runtime() -> Box<Runtime> {
    let r = Box::new(Runtime::default());
    let mut e = Box::new(Engine::new(8000, 240., 16000));
    e.tracks[0].kind = Kind::Synth;
    e.tracks[1].kind = Kind::Synth;
    r.initialize(e).ok().unwrap();
    r.active.store(true, Ordering::Release);
    r.output(&mut [0; 2], 2, 8000);
    r
}
#[test]
fn output_mixes_without_erasing_native_audio() {
    let r = runtime();
    let mut buf = [100; 128];
    r.output(&mut buf, 2, 8000);
    assert_eq!(buf, [100; 128]);
    r.enqueue(Command::Note {
        track: 0,
        note: 60,
        velocity: 90,
        on: true,
        source: 2,
    });
    r.output(&mut buf, 2, 8000);
    assert!(buf.iter().any(|x| *x != 100));
}
#[test]
fn external_release_uses_original_track_after_selection() {
    let r = runtime();
    assert!(r.external_note(1, 60, 100, true));
    r.output(&mut [0; 128], 2, 8000);
    r.enqueue(Command::Select(1));
    r.output(&mut [0; 128], 2, 8000);
    assert!(r.external_note(1, 60, 0, false));
    for _ in 0..100 {
        r.output(&mut [0; 256], 2, 8000)
    }
    r.with_engine(|slot| assert_eq!(slot.as_ref().unwrap().active_voices(), 0));
}
#[test]
fn external_sources_are_independent() {
    let r = runtime();
    assert!(r.external_note(1, 60, 100, true));
    assert!(r.external_note(2, 60, 100, true));
    r.output(&mut [0; 128], 2, 8000);
    assert!(r.external_note(1, 60, 0, false));
    for _ in 0..100 {
        r.output(&mut [0; 256], 2, 8000)
    }
    r.with_engine(|slot| assert_eq!(slot.as_ref().unwrap().active_voices(), 1));
}
#[test]
fn project_guard_never_blocks_audio_or_changes_native_buffer() {
    let r = runtime();
    let mut buf = [123; 128];
    r.with_engine(|_| r.output(&mut buf, 2, 8000));
    assert_eq!(buf, [123; 128]);
}
#[test]
fn full_command_queue_fails_to_stop_without_stuck_notes() {
    let r = runtime();
    for _ in 0..300 {
        r.enqueue(Command::Note {
            track: 0,
            note: 60,
            velocity: 90,
            on: true,
            source: 2,
        })
    }
    r.output(&mut [0; 128], 2, 8000);
    assert!(r.rejected_commands.load(Ordering::Relaxed) > 0);
    r.with_engine(|s| assert_eq!(s.as_ref().unwrap().active_voices(), 0));
}
#[test]
fn audio_record_without_input_is_rejected() {
    let r = runtime();
    r.enqueue(Command::Record(2));
    r.output(&mut [0; 128], 2, 8000);
    r.with_engine(|s| {
        let e = s.as_ref().unwrap();
        assert!(e.recorder.is_none());
        assert_eq!(e.error, Some(Error::InputUnavailable));
    });
}
#[test]
fn input_loss_cancels_only_new_take() {
    let r = runtime();
    r.with_engine(|s| {
        let e = s.as_mut().unwrap();
        e.preroll = 0;
        e.running = true;
        e.tracks[1].length_ticks = 4 * PPQ;
        e.tracks[1].playing = true;
    });
    r.capture(Frame::mono(0.2), 8000);
    r.enqueue(Command::Record(2));
    r.output(&mut [0; 4], 2, 8000);
    r.with_engine(|s| {
        let e = s.as_ref().unwrap();
        assert!(e.recorder.is_none());
        assert!(e.tracks[1].playing);
        assert_eq!(e.tracks[2].audio_frames, 0);
    });
}
#[test]
fn snapshot_matches_published_track_state() {
    let r = runtime();
    r.enqueue(Command::Level(0, 0.23));
    r.output(&mut [0; 128], 2, 8000);
    let s = r.snapshot.read().unwrap();
    assert_eq!(s.mix(0).level, 0.23);
    assert_eq!(s.track_kind(0), Kind::Synth);
    assert!(s.words[1] > 0);
    r.with_engine(|e| {
        let actual = Snapshot::capture(e.as_ref().unwrap());
        assert_eq!(s.words[0], actual.words[0]);
    });
}
#[test]
fn concurrent_queue_preserves_every_producer_order() {
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    let q = Arc::new(loop_club::queue::Queue::<usize, 64>::default());
    let stop = Arc::new(AtomicBool::new(false));
    let mut handles = Vec::new();
    for producer in 0..4 {
        let q = q.clone();
        let stop = stop.clone();
        handles.push(std::thread::spawn(move || {
            for i in 0..500 {
                while !q.push(producer * 500 + i) {
                    if stop.load(Ordering::Relaxed) {
                        return;
                    }
                    std::thread::yield_now();
                }
            }
        }));
    }
    let mut next = [0usize; 4];
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    let mut count = 0;
    while count < 2000 && std::time::Instant::now() < deadline {
        let item = unsafe { q.pop_shared() };
        if let Some(v) = item {
            let producer = v / 500;
            assert_eq!(v % 500, next[producer]);
            next[producer] += 1;
            count += 1
        } else {
            std::thread::yield_now();
        }
    }
    stop.store(true, Ordering::Relaxed);
    for thread in handles {
        thread.join().unwrap()
    }
    assert_eq!(count, 2000);
}
#[test]
fn midi_uses_requested_track_before_next_audio_snapshot() {
    let r = runtime();
    r.with_engine(|e| e.as_mut().unwrap().preroll = 0);
    r.enqueue(Command::Select(1));
    r.enqueue(Command::Record(1));
    assert!(r.external_note(1, 62, 70, true));
    r.output(&mut [0; 128], 2, 8000);
    r.with_engine(|s| {
        let e = s.as_ref().unwrap();
        assert!(e.visible_events(1).iter().any(|n| n.note == 62 && n.on));
        assert!(e.visible_events(0).is_empty());
    });
}

#[test]
fn emergency_stop_clears_held_notes_and_temporary_effects() {
    let r = runtime();
    r.enqueue(Command::FxSelect {
        target: 0,
        track: 0,
        slot: 0,
        effect: Effect::Filter,
        temporary: true,
    });
    r.enqueue(Command::Note {
        track: 0,
        note: 60,
        velocity: 90,
        on: true,
        source: 2,
    });
    r.output(&mut [0; 128], 2, 8000);
    r.emergency_stop();
    r.output(&mut [0; 128], 2, 8000);
    r.with_engine(|s| {
        let e = s.as_ref().unwrap();
        assert_eq!(e.active_voices(), 0);
        assert_eq!(e.tracks[0].fx[0].effect, Effect::Off);
    });
}

#[test]
fn runtime_diagnostics_do_not_overwrite_recording_mode_and_loop_flags() {
    let r = runtime();
    r.enqueue(Command::RecordMode(0, RecordMode::Overdub));
    r.enqueue(Command::Record(0));
    r.output(&mut [0; 128], 2, 8000);
    let expected = r
        .with_engine(|slot| Snapshot::capture(slot.as_ref().unwrap()).words[12])
        .unwrap();
    let actual = r.snapshot.read().unwrap();
    assert_eq!(actual.words[12], expected);
    assert_ne!(actual.words[12] & 4, 0);
    assert_eq!(actual.words[792], r.lost_input.load(Ordering::Relaxed));
}
