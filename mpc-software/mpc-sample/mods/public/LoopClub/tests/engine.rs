use loop_club::{
    engine::{Command, Engine},
    model::*,
};
fn engine() -> Engine {
    let mut e = Engine::new(8000, 240., 48000);
    e.preroll = 0;
    e
}
fn render(e: &mut Engine, n: usize, x: Frame) {
    for _ in 0..n {
        assert!(e.process(x).finite());
    }
}
#[test]
fn count_in_and_record_boundary() {
    let mut e = engine();
    e.preroll = 1;
    e.command(Command::Record(0)).unwrap();
    assert_eq!(e.status(0), Status::CountIn);
    render(&mut e, 8000, Frame::mono(0.2));
    assert_eq!(e.status(0), Status::Record);
    assert_eq!(e.tracks[0].audio_frames, 0);
    render(&mut e, 8000, Frame::mono(0.25));
    assert_eq!(e.tracks[0].audio_frames, 8000);
    e.command(Command::Pad(0, false)).unwrap();
    render(&mut e, 8001, Frame::mono(0.3));
    assert_eq!(e.tracks[0].length_ticks, 8 * PPQ);
    assert_eq!(e.tracks[0].audio_frames, 16000);
    assert_eq!(e.status(0), Status::Record);
}
#[test]
fn cancel_count_in_keeps_other_track_playing() {
    let mut e = engine();
    e.tracks[1].length_ticks = 4 * PPQ;
    e.tracks[1].audio_frames = 8000;
    e.tracks[1].playing = true;
    e.running = true;
    e.preroll = 1;
    e.command(Command::Record(0)).unwrap();
    e.command(Command::Pad(0, false)).unwrap();
    assert!(e.recorder.is_none());
    assert!(e.tracks[1].playing);
}
#[test]
fn three_bar_loop_is_not_power_of_two() {
    let mut e = engine();
    e.command(Command::Record(0)).unwrap();
    render(&mut e, 17000, Frame::mono(0.2));
    e.command(Command::Pad(0, false)).unwrap();
    render(&mut e, 7001, Frame::mono(0.3));
    assert_eq!(e.tracks[0].length_ticks, 12 * PPQ);
    assert_eq!(e.tracks[0].audio_frames, 24000);
}
#[test]
fn midi_recording_preserves_notes_and_replays() {
    let mut e = engine();
    e.command(Command::SetKind(0, Kind::Synth)).unwrap();
    e.command(Command::Record(0)).unwrap();
    render(&mut e, 100, Frame::ZERO);
    e.command(Command::Note {
        track: 0,
        note: 60,
        velocity: 80,
        on: true,
        source: 2,
    })
    .unwrap();
    render(&mut e, 500, Frame::ZERO);
    e.command(Command::Note {
        track: 0,
        note: 60,
        velocity: 0,
        on: false,
        source: 2,
    })
    .unwrap();
    e.command(Command::Pad(0, false)).unwrap();
    e.command(Command::Record(0)).unwrap(); // Stop after the first completed take.
    render(&mut e, 7401, Frame::ZERO);
    assert_eq!(e.tracks[0].events.len(), 2);
    assert_eq!(e.tracks[0].events[0].note, 60);
    let mut peak = 0_f32;
    for _ in 0..1000 {
        peak = peak.max(e.process(Frame::ZERO).peak())
    }
    assert!(peak > 0.001);
}
#[test]
fn held_midi_note_is_closed_at_loop_end() {
    let mut e = engine();
    e.command(Command::SetKind(0, Kind::Synth)).unwrap();
    e.command(Command::Record(0)).unwrap();
    e.command(Command::Note {
        track: 0,
        note: 60,
        velocity: 100,
        on: true,
        source: 2,
    })
    .unwrap();
    e.command(Command::Pad(0, false)).unwrap();
    render(&mut e, 8001, Frame::ZERO);
    assert_eq!(e.tracks[0].events.len(), 2);
    assert!(!e.tracks[0].events[1].on);
    assert_eq!(e.tracks[0].events[1].tick, 4 * PPQ - 1);
}
#[test]
fn pickup_does_not_jump_or_touch_other_channel() {
    let mut p = Pickup::default();
    p.select(0.7);
    assert!(p.move_to(0.2, 0.7).is_none());
    assert_eq!(p.move_to(0.8, 0.7), Some(0.8));
    p.select(0.1);
    assert!(p.move_to(0.5, 0.1).is_none());
}
#[test]
fn kind_change_cannot_destroy_recorded_material() {
    let mut e = engine();
    e.tracks[0].length_ticks = 4 * PPQ;
    assert_eq!(
        e.command(Command::SetKind(0, Kind::Synth)),
        Err(Error::NotEmpty)
    );
}
#[test]
fn tempo_change_rejected_instead_of_desynchronizing_audio() {
    let mut e = engine();
    e.tracks[0].length_ticks = 4 * PPQ;
    assert_eq!(e.command(Command::Bpm(88.)), Err(Error::TempoFrozen));
    assert_eq!(e.clock.bpm, 240.);
}
#[test]
fn midi_sources_have_independent_note_ownership() {
    let mut e = engine();
    e.command(Command::SetKind(0, Kind::Synth)).unwrap();
    for source in [1, 2] {
        e.command(Command::Note {
            track: 0,
            note: 60,
            velocity: 80,
            on: true,
            source,
        })
        .unwrap()
    }
    e.command(Command::Note {
        track: 0,
        note: 60,
        velocity: 0,
        on: false,
        source: 2,
    })
    .unwrap();
    render(&mut e, 12000, Frame::ZERO);
    assert_eq!(e.active_voices(), 1);
}
#[test]
fn recording_capacity_closes_at_last_complete_bar() {
    let mut e = engine();
    e.command(Command::Record(0)).unwrap();
    render(&mut e, 48001, Frame::mono(0.2));
    assert_eq!(e.tracks[0].audio_frames, 48000);
    assert_eq!(e.tracks[0].length_ticks, 24 * PPQ);
    assert!(e.recorder.unwrap().repeating);
}
#[test]
fn output_peak_guard_bounds_loud_inputs() {
    let mut e = engine();
    for i in 0..16 {
        e.tracks[i].audio[..8000].fill(Frame::mono(1.));
        e.tracks[i].audio_frames = 8000;
        e.tracks[i].length_ticks = 4 * PPQ;
        e.tracks[i].playing = true;
    }
    e.running = true;
    for _ in 0..9000 {
        assert!(e.process(Frame::ZERO).peak() <= 0.98);
    }
}
#[test]
fn invalid_values_and_indices_are_rejected() {
    let mut e = engine();
    assert_eq!(e.command(Command::Level(16, 0.1)), Err(Error::InvalidTrack));
    assert_eq!(
        e.command(Command::Level(0, f32::NAN)),
        Err(Error::InvalidValue)
    );
    assert_eq!(e.command(Command::Preroll(3)), Err(Error::InvalidValue));
}
#[test]
fn project_round_trip_and_failed_load_do_not_mutate_source() {
    let mut e = engine();
    e.tracks[0].audio_frames = 200;
    e.tracks[0].audio[..200].fill(Frame::mono(0.234));
    e.tracks[0].length_ticks = 4 * PPQ;
    e.tracks[1].kind = Kind::Synth;
    e.tracks[1].length_ticks = 8 * PPQ;
    e.tracks[1].events.push(Event {
        deck: None,
        timing: None,
        slice: false,
        tick: 42,
        note: 65,
        velocity: 78,
        on: true,
    });
    e.tracks[0].drums[1].mix.room = 0.42;
    e.performance[0].effect = Effect::Crush;
    let p = std::env::temp_dir().join(format!("loopclub-test-{}.lcproj", std::process::id()));
    loop_club::project::save(&e, &p).unwrap();
    let restored = loop_club::project::load(&p, 8000, 48000).unwrap();
    assert_eq!(restored.tracks[0].audio[100], e.tracks[0].audio[100]);
    assert_eq!(restored.tracks[1].events, e.tracks[1].events);
    assert!(!restored.running);
    assert_eq!(restored.performance[0].effect, Effect::Crush);
    assert_eq!(restored.tracks[0].drums[1].mix.room, 0.42);
    assert!(loop_club::project::load(&p, 44100, 48000).is_err());
    std::fs::write(&p, b"broken").unwrap();
    assert!(loop_club::project::load(&p, 8000, 48000).is_err());
    std::fs::remove_file(p).unwrap();
}
#[test]
fn save_during_play_is_rejected_without_overwriting() {
    let mut e = engine();
    e.running = true;
    let p = std::env::temp_dir().join("loopclub-must-not-write.lcproj");
    assert!(loop_club::project::save(&e, &p).is_err());
}
#[test]
fn bounded_queue_fails_full_and_retains_fifo() {
    let mut q = loop_club::queue::Queue::<u32, 4>::default();
    for i in 0..4 {
        assert!(q.push(i))
    }
    assert!(!q.push(9));
    for i in 0..4 {
        assert_eq!(q.pop(), Some(i))
    }
    assert_eq!(q.pop(), None);
    assert!(q.push(12));
    assert_eq!(q.pop(), Some(12));
}
#[test]
fn drum_closed_hat_chokes_open_hat_only_in_its_kit() {
    let mut e = engine();
    e.tracks[0].kind = Kind::Drum;
    e.tracks[1].kind = Kind::Drum;
    for track in [0, 1] {
        e.command(Command::Note {
            track,
            note: 3,
            velocity: 100,
            on: true,
            source: 2,
        })
        .unwrap()
    }
    assert_eq!(e.active_voices(), 2);
    e.command(Command::Note {
        track: 0,
        note: 2,
        velocity: 100,
        on: true,
        source: 2,
    })
    .unwrap();
    assert_eq!(e.active_voices(), 2);
}
#[test]
fn midi_record_has_no_embedded_audio_frames() {
    let mut e = engine();
    e.command(Command::SetKind(0, Kind::Drum)).unwrap();
    e.command(Command::Record(0)).unwrap();
    e.command(Command::Pad(0, false)).unwrap();
    render(&mut e, 8001, Frame::ZERO);
    assert_eq!(e.tracks[0].audio_frames, 0);
}
#[test]
fn stopped_transport_finishes_input_latency_tail() {
    let mut e = engine();
    e.latency_in = 16;
    e.command(Command::Record(0)).unwrap();
    e.command(Command::Pad(0, false)).unwrap();
    render(&mut e, 8001, Frame::mono(0.21));
    assert!(e.recorder.is_some());
    e.command(Command::Stop).unwrap();
    render(&mut e, 20, Frame::mono(0.42));
    assert!(e.recorder.is_none());
    assert_eq!(e.tracks[0].audio[7999], Frame::mono(0.42));
    assert!(!e.running);
}
#[test]
fn resample_records_current_mix_without_feedback() {
    let mut e = engine();
    let t = &mut e.tracks[1];
    t.audio[..8000].fill(Frame::mono(0.3));
    t.length_ticks = 4 * PPQ;
    t.audio_frames = 8000;
    t.playing = true;
    e.running = true;
    e.command(Command::Resample(0)).unwrap();
    let mut expected = Vec::new();
    for _ in 0..500 {
        expected.push(e.process(Frame::ZERO));
    }
    e.command(Command::Pad(0, false)).unwrap();
    render(&mut e, 7501, Frame::ZERO);
    assert_eq!(&e.tracks[0].audio[..500], expected.as_slice());
    assert!(e.recorder.is_none());
}

#[test]
fn default_project_is100bpm_and_stopped_midi_tempo_change_preserves_events() {
    let mut e = Engine::new_project(8000, 8000);
    assert_eq!(e.clock.bpm, 100.);
    assert!(e.tracks.iter().all(|t| t.empty()));
    e.tracks[0].kind = Kind::Synth;
    e.tracks[0].length_ticks = 4 * PPQ;
    e.tracks[0].events.push(Event {
        deck: None,
        timing: None,
        slice: false,
        tick: 0,
        note: 60,
        velocity: 90,
        on: true,
    });
    e.tracks[1].kind = Kind::Drum;
    e.tracks[1].length_ticks = 4 * PPQ;
    let before = e.tracks[0].events.clone();
    e.command(Command::Bpm(240.)).unwrap();
    e.command(Command::Bpm(100.)).unwrap();
    assert_eq!(e.clock.bpm, 100.);
    assert_eq!(e.clock.frame_at(4 * PPQ), 19200);
    assert_eq!(e.tracks[0].events.len(), before.len());
    assert_eq!(e.tracks[0].events[0].tick, before[0].tick);
    assert_eq!(e.tracks[0].events[0].note, 60);
    assert_eq!(e.tracks[0].length_ticks, 4 * PPQ);
    e.command(Command::PlayAll).unwrap();
    assert_eq!(e.command(Command::Bpm(110.)), Err(Error::TempoFrozen));
    e.command(Command::Stop).unwrap();
    e.tracks[2].length_ticks = 4 * PPQ;
    assert_eq!(e.command(Command::Bpm(110.)), Err(Error::TempoFrozen));
}

#[test]
fn recording_waveform_reads_current_take_before_commit() {
    let mut e = Engine::new_project(8000, 24000);
    e.command(Command::Preroll(0)).unwrap();
    e.command(Command::Record(0)).unwrap();
    render(&mut e, 1600, Frame::mono(0.4));
    assert_eq!(e.tracks[0].features.frames, 0);
    let (features, frames) = e.visible_audio(0);
    assert!(frames > 0);
    assert!(features.window(0, frames).max > 0.3);
    let s = loop_club::ui::Snapshot::capture(&e);
    assert!(s.words[770] > 0);
    assert!(
        s.words[512..768]
            .iter()
            .any(|w| f32::from_bits((w >> 32) as u32) > 0.3)
    );
    e.command(Command::CancelRecord).unwrap();
    assert_eq!(e.visible_audio(0).1, 0);
}

#[test]
fn restored_sixteen_track_looper_plays_all_backing_tracks() {
    let mut e = Engine::new_project(8000, 8000);
    let t = &mut e.tracks[8];
    t.audio[..8000].fill(Frame::mono(0.6));
    t.audio_frames = 8000;
    t.length_ticks = 4 * PPQ;
    t.mix.solo = true;
    e.command(Command::PlayAll).unwrap();
    assert!(e.tracks[8].playing);
    assert!((0..1000).any(|_| e.process(Frame::ZERO).peak() > 0.01));
    e.command(Command::Stop).unwrap();
    assert_eq!(e.tracks[8].audio_frames, 8000);
    assert!(e.tracks[8].mix.solo);
}

#[test]
fn legacy_hidden_track_audio_survives_project_roundtrip() {
    let mut e = engine();
    let t = &mut e.tracks[12];
    t.audio[17] = Frame::mono(0.3);
    t.audio_frames = 8000;
    t.length_ticks = 4 * PPQ;
    let path = std::env::temp_dir().join(format!("loopclub-legacy8-{}.lcproj", std::process::id()));
    loop_club::project::save(&e, &path).unwrap();
    let restored = loop_club::project::load(&path, 8000, 48000).unwrap();
    assert_eq!(restored.tracks[12].audio[17], Frame::mono(0.3));
    assert_eq!(restored.tracks[12].audio_frames, 8000);
    std::fs::remove_file(path).unwrap();
}

#[test]
fn loop_replace_resets_length_and_cancel_preserves_previous_clip() {
    let mut e = engine();
    e.tracks[0].kind = Kind::Synth;
    e.tracks[0].length_ticks = 16 * PPQ;
    let original = Event {
        deck: None,
        timing: None,
        slice: false,
        tick: 0,
        note: 60,
        velocity: 80,
        on: true,
    };
    e.tracks[0].events.push(original);
    e.preroll = 1;
    e.command(Command::LoopRecord(0)).unwrap();
    e.command(Command::LoopRecord(0)).unwrap();
    assert!(e.recorder.is_none());
    assert_eq!(e.tracks[0].length_ticks, 16 * PPQ);
    assert_eq!(e.tracks[0].events[0], original);
    assert!(!e.tracks[0].playing);
    e.preroll = 0;
    e.command(Command::LoopRecord(0)).unwrap();
    e.command(Command::Note {
        track: 0,
        note: 64,
        velocity: 90,
        on: true,
        source: 1000,
    })
    .unwrap();
    render(&mut e, 1000, Frame::ZERO);
    e.command(Command::LoopRecord(0)).unwrap();
    render(&mut e, 7001, Frame::ZERO);
    assert!(e.recorder.is_none());
    assert_eq!(e.tracks[0].length_ticks, 4 * PPQ);
    assert!(e.tracks[0].playing);
    assert!(e.tracks[0].events.iter().any(|n| n.note == 64));
    assert!(!e.tracks[0].events.iter().any(|n| n.note == 60));
}

#[test]
fn partial_audio_overdub_ends_this_bar_and_keeps_original_tail_and_length() {
    let mut e = engine();
    let t = &mut e.tracks[0];
    t.audio_frames = 32000;
    t.source_frames = 32000;
    t.length_ticks = 16 * PPQ;
    t.record_mode = RecordMode::Overdub;
    for (i, f) in t.audio[..32000].iter_mut().enumerate() {
        *f = Frame::mono(if i < 8000 { 0.1 } else { 0.25 });
        t.features.feed(*f, 8000);
    }
    e.command(Command::LoopRecord(0)).unwrap();
    render(&mut e, 1000, Frame::mono(0.2));
    e.command(Command::LoopRecord(0)).unwrap();
    render(&mut e, 7001, Frame::mono(0.2));
    assert!(e.recorder.is_none());
    assert_eq!(e.tracks[0].length_ticks, 16 * PPQ);
    assert_eq!(e.tracks[0].audio_frames, 32000);
    assert!((e.tracks[0].audio[100].l - 0.3).abs() < 1e-6);
    assert_eq!(e.tracks[0].audio[16000], Frame::mono(0.25));
    assert!(e.tracks[0].features.window(16000, 20000).max > 0.24);
}

#[test]
fn live_notes_and_audio_are_visible_before_recording() {
    let mut e = engine();
    render(&mut e, 1000, Frame::mono(0.2));
    let s = loop_club::ui::Snapshot::capture(&e);
    assert!(
        s.words[1056..1184]
            .iter()
            .any(|w| f32::from_bits((w >> 32) as u32) > 0.1)
    );
    e.tracks[0].kind = Kind::Synth;
    e.command(Command::Note {
        track: 0,
        note: 67,
        velocity: 90,
        on: true,
        source: 1000,
    })
    .unwrap();
    let s = loop_club::ui::Snapshot::capture(&e);
    assert_ne!(s.words[1045] & (1 << 3), 0);
    assert!(e.recorder.is_none());
}
