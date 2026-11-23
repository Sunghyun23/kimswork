use loop_club::{
    audio::{RecordTarget as Target, Settings},
    engine::{Command, Engine},
    model::*,
};
fn core() -> Engine {
    let mut e = Engine::new(8000, 240., 32000);
    e.preroll = 0;
    let t = &mut e.tracks[0];
    t.audio_frames = 8000;
    t.source_frames = 8000;
    t.source_ticks = 4 * PPQ;
    t.length_ticks = 4 * PPQ;
    t.chops.divide(8000);
    for (i, x) in t.audio[..8000].iter_mut().enumerate() {
        *x = Frame::mono((i as f32 * std::f32::consts::TAU * 200. / 8000.).sin() * 0.2);
    }
    e
}
fn notes(e: &mut Engine, n: u8) {
    e.tracks[0].audio_sequence = true;
    e.tracks[0].events = vec![
        Event {
            deck: None,
            timing: None,
            tick: 0,
            note: n,
            velocity: 90,
            on: true,
            slice: true,
        },
        Event {
            deck: None,
            timing: None,
            tick: PPQ,
            note: n,
            velocity: 0,
            on: false,
            slice: true,
        },
    ];
}
fn render(e: &mut Engine, n: usize) -> Vec<f32> {
    (0..n).map(|_| e.process(Frame::ZERO).l).collect()
}
fn save_load(e: &mut Engine, name: &str) {
    e.command(Command::Stop).unwrap();
    render(e, 1000);
    let p = std::env::temp_dir().join(format!("lc-deep-{}-{name}.lcproj", std::process::id()));
    loop_club::project::save(e, &p).unwrap();
    let out = loop_club::project::load(&p, 8000, 32000);
    std::fs::remove_file(p).unwrap();
    assert!(out.is_ok(), "{name}: {:?}", out.err());
}
#[test]
fn reducing_chop_count_keeps_recorded_slice_events_valid_and_undoable() {
    let mut e = core();
    notes(&mut e, 15);
    let old = e.tracks[0].events.clone();
    e.command(Command::ChopOption {
        track: 0,
        index: 0,
        value: 0.,
    })
    .unwrap();
    e.command(Command::ChopApply(0)).unwrap();
    assert!(
        e.tracks[0]
            .events
            .iter()
            .all(|x| !x.slice || x.note < e.tracks[0].chops.count)
    );
    save_load(&mut e, "reduced");
    e.command(Command::Undo).unwrap();
    assert_eq!(e.tracks[0].events, old);
    assert_eq!(e.tracks[0].chops.count, 16);
    e.command(Command::Redo).unwrap();
    assert_eq!(e.tracks[0].chops.count, 1);
    save_load(&mut e, "reduced-redo");
}
#[test]
fn deleting_and_splitting_chops_remaps_existing_performance_owners() {
    let mut e = core();
    notes(&mut e, 15);
    e.command(Command::DeleteSlice { track: 0, pad: 15 })
        .unwrap();
    assert_eq!(e.tracks[0].events[0].note, 14);
    save_load(&mut e, "delete");
    e.command(Command::Undo).unwrap();
    e.command(Command::DeleteSlice { track: 0, pad: 0 })
        .unwrap();
    assert_eq!(e.tracks[0].events[0].note, 14);
    e.command(Command::SplitSlice {
        track: 0,
        pad: 0,
        at: 200,
    })
    .unwrap();
    assert_eq!(e.tracks[0].events[0].note, 15);
    save_load(&mut e, "split");
}
#[test]
fn structural_chop_edits_are_refused_while_that_track_is_recording() {
    let mut e = core();
    e.command(Command::AudioTarget(0, Target::Chop)).unwrap();
    e.command(Command::LoopRecord(0)).unwrap();
    assert_eq!(
        e.command(Command::DeleteSlice { track: 0, pad: 0 }),
        Err(Error::Busy)
    );
    assert_eq!(e.command(Command::ChopApply(0)), Err(Error::Busy));
    assert_eq!(e.command(Command::ResetSlices(0)), Err(Error::Busy));
    assert!(e.recorder.is_some());
}
#[test]
fn original_loop_tune_works_without_warp() {
    let mut e = core();
    e.command(Command::AudioSettings(
        0,
        Settings {
            tune: 12.,
            ..Default::default()
        },
    ))
    .unwrap();
    e.command(Command::PlayAll).unwrap();
    render(&mut e, 1000);
    let out = render(&mut e, 8000);
    let hz = out.windows(2).filter(|x| x[0] <= 0. && x[1] > 0.).count();
    assert!((390..=410).contains(&hz), "expected400Hz got{hz}");
}
#[test]
fn unity_warp_preserves_loaded_wav_padding_and_source_content() {
    let mut a = core();
    a.tracks[0].source_frames = 2000;
    a.tracks[0].audio[2000..8000].fill(Frame::ZERO);
    let mut b = core();
    b.tracks[0].source_frames = 2000;
    b.tracks[0].audio[2000..8000].fill(Frame::ZERO);
    b.command(Command::AudioSettings(
        0,
        Settings {
            warp: true,
            ..Default::default()
        },
    ))
    .unwrap();
    a.command(Command::PlayAll).unwrap();
    b.command(Command::PlayAll).unwrap();
    let a = render(&mut a, 8000);
    let b = render(&mut b, 8000);
    let mse = a.iter().zip(b).map(|(a, b)| (a - b).powi(2)).sum::<f32>() / 8000.;
    assert!(mse < 1e-10, "unityWarp changed source/padding mse{mse}");
}
#[test]
fn sampler_can_record_from_source_padded_beyond_capture_capacity() {
    let mut e = Engine::new(8000, 100., 32000);
    e.preroll = 0;
    let t = &mut e.tracks[0];
    t.audio.resize(38400, Frame::ZERO);
    t.audio_frames = 38400;
    t.source_frames = 32000;
    t.source_ticks = 8 * PPQ;
    t.length_ticks = 8 * PPQ;
    t.chops.divide(32000);
    e.command(Command::AudioTarget(0, Target::Chromatic))
        .unwrap();
    assert!(e.command(Command::LoopRecord(0)).is_ok());
    assert!(e.recorder.unwrap().midi);
}
#[test]
fn invalid_empty_sampler_handoff_does_not_cancel_current_countin_or_schedule_a_bad_target() {
    let mut e = core();
    e.tracks[0].kind = Kind::Synth;
    e.tracks[0].audio_frames = 0;
    e.tracks[0].source_frames = 0;
    e.tracks[0].length_ticks = 0;
    e.preroll = 1;
    e.command(Command::AudioTarget(1, Target::Chop)).unwrap();
    e.command(Command::LoopRecord(0)).unwrap();
    let before = e.recorder.unwrap();
    assert_eq!(e.command(Command::LoopRecord(1)), Err(Error::NoContent));
    assert_eq!(e.recorder.unwrap().track, before.track);
    assert_eq!(e.next_record, None);
}

#[test]
fn input_overdub_never_silently_flattens_performance_or_discards_tempo_changed_pcm() {
    let mut e = core();
    notes(&mut e, 3);
    let before = e.tracks[0].events.clone();
    e.command(Command::RecordMode(0, RecordMode::Overdub))
        .unwrap();
    assert_eq!(e.command(Command::LoopRecord(0)), Err(Error::ResampleFirst));
    assert_eq!(e.tracks[0].events, before);
    assert!(e.recorder.is_none());
    e.tracks[0].audio_sequence = false;
    e.tracks[0].events.clear();
    e.command(Command::AudioSettings(
        0,
        Settings {
            warp: true,
            ..Default::default()
        },
    ))
    .unwrap();
    e.command(Command::Bpm(100.)).unwrap();
    assert_eq!(e.command(Command::LoopRecord(0)), Err(Error::ResampleFirst));
    assert_eq!(e.tracks[0].audio_frames, 8000);
}
#[test]
fn chromatic_countin_cancel_overdub_and_pending_target_guards_preserve_source() {
    let mut e = core();
    let old = e.tracks[0].audio.clone();
    e.preroll = 2;
    e.command(Command::AudioTarget(0, Target::Chromatic))
        .unwrap();
    e.command(Command::LoopRecord(0)).unwrap();
    e.command(Command::Note {
        track: 0,
        note: 60,
        velocity: 90,
        on: true,
        source: 100,
    })
    .unwrap();
    render(&mut e, 100);
    e.command(Command::LoopRecord(0)).unwrap();
    assert!(e.recorder.is_none());
    assert_eq!(e.tracks[0].audio, old);
    assert!(e.tracks[0].events.is_empty());
    assert!(!e.tracks[0].audio_sequence);
}
#[test]
fn negative_tune_runs_through_whole_source_over_two_bars_without_bar_retrigger() {
    let mut e = core();
    e.tracks[0].audio[..4000].fill(Frame::mono(0.2));
    e.tracks[0].audio[4000..8000].fill(Frame::mono(-0.2));
    e.command(Command::AudioSettings(
        0,
        Settings {
            tune: -12.,
            ..Default::default()
        },
    ))
    .unwrap();
    e.command(Command::PlayAll).unwrap();
    let out = render(&mut e, 16000);
    assert!(out[2000..7000].iter().all(|x| *x > 0.));
    assert!(out[9000..15000].iter().all(|x| *x < 0.));
}
#[test]
fn random_valid_audio_edits_always_save_load_with_valid_sample_owners() {
    let mut e = core();
    notes(&mut e, 15);
    let mut seed = 0x1649u32;
    for n in 0..120 {
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        match seed % 5 {
            0 => {
                let _ = e.command(Command::DeleteSlice {
                    track: 0,
                    pad: (seed as usize / 5) % usize::from(e.tracks[0].chops.count),
                });
            }
            1 => {
                let i = (seed as usize / 5) % usize::from(e.tracks[0].chops.count);
                let s = e.tracks[0].chops.slices[i];
                let _ = e.command(Command::SplitSlice {
                    track: 0,
                    pad: i,
                    at: s.start + (s.end - s.start) / 2,
                });
            }
            2 => {
                e.command(Command::ChopOption {
                    track: 0,
                    index: 0,
                    value: (seed >> 16) as f32 / 65535.,
                })
                .unwrap();
                let _ = e.command(Command::ChopApply(0));
            }
            3 => {
                let _ = e.command(Command::Undo);
            }
            _ => {
                let _ = e.command(Command::Redo);
            }
        }
        assert!(
            e.tracks[0]
                .events
                .iter()
                .all(|x| !x.slice || x.note < e.tracks[0].chops.count),
            "iteration{n}"
        );
        if n % 20 == 0 {
            save_load(&mut e, &format!("fuzz-{n}"));
        }
    }
}
