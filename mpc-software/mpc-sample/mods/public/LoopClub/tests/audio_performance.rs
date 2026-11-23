use loop_club::{
    audio::{RecordTarget as Target, Settings, Voice, Warp},
    engine::{Command, Engine},
    model::*,
    runtime::Runtime,
    ui::{Snapshot, Ui},
};
use std::sync::atomic::Ordering;
fn core() -> Engine {
    let mut e = Engine::new(8000, 240., 32000);
    e.preroll = 0;
    e
}
fn source(e: &mut Engine) {
    let t = &mut e.tracks[0];
    t.audio_frames = 8000;
    t.source_frames = 8000;
    t.source_ticks = 4 * PPQ;
    t.length_ticks = 4 * PPQ;
    t.chops.divide(8000);
    for (i, x) in t.audio[..8000].iter_mut().enumerate() {
        *x = Frame::mono((i as f32 * std::f32::consts::TAU * 200. / 8000.).sin() * 0.25);
    }
}
fn frames(e: &mut Engine, n: usize) -> f32 {
    let mut peak = 0f32;
    for _ in 0..n {
        peak = peak.max(e.process(Frame::ZERO).peak());
    }
    peak
}
fn until(e: &mut Engine, n: u64) {
    while e.clock.frame <= n {
        e.process(Frame::ZERO);
    }
}
fn note(e: &mut Engine, n: u8, on: bool) {
    e.command(Command::Note {
        track: 0,
        note: n,
        velocity: if on { 90 } else { 0 },
        on,
        source: 100,
    })
    .unwrap();
}
fn finish(e: &mut Engine) {
    e.command(Command::LoopRecord(0)).unwrap();
    for _ in 0..32001 {
        e.process(Frame::ZERO);
        if e.recorder.is_none() {
            return;
        }
    }
    panic!("record did not end");
}
#[test]
fn explicit_input_captures_pcm_and_never_automatically_changes_target() {
    let mut e = core();
    e.command(Command::LoopRecord(0)).unwrap();
    assert!(!e.recorder.unwrap().midi);
    for _ in 0..1000 {
        e.process(Frame::mono(0.12));
    }
    e.command(Command::LoopRecord(0)).unwrap();
    while e.recorder.is_some() {
        e.process(Frame::mono(0.12));
    }
    let t = &e.tracks[0];
    assert_eq!(t.audio_target, Target::Input);
    assert_eq!(t.kind, Kind::Audio);
    assert!(!t.audio_sequence);
    assert_eq!(t.source_ticks, 4 * PPQ);
    assert!(t.audio[..t.audio_frames].iter().any(|x| x.peak() > 0.1));
}
#[test]
fn held_chromatic_repeats_and_releases_without_transport() {
    let mut e = core();
    source(&mut e);
    e.command(Command::AudioTarget(0, Target::Chromatic))
        .unwrap();
    note(&mut e, 60, true);
    assert!(frames(&mut e, 24000) > 0.01);
    assert_eq!(e.slice_activity(0).count_ones(), 1);
    assert!(!e.running);
    note(&mut e, 60, false);
    frames(&mut e, 100);
    assert_eq!(e.slice_activity(0).count_ones(), 0);
    assert_eq!(frames(&mut e, 200), 0.);
}
#[test]
fn chromatic_record_keeps_pcm_chops_and_input_choice_while_recording_real_events() {
    let mut e = core();
    source(&mut e);
    let pcm = e.tracks[0].audio.clone();
    let chops = e.tracks[0].chops;
    e.command(Command::AudioTarget(0, Target::Chromatic))
        .unwrap();
    e.command(Command::LoopRecord(0)).unwrap();
    assert!(e.recorder.unwrap().midi);
    frames(&mut e, 100);
    note(&mut e, 55, true);
    frames(&mut e, 700);
    note(&mut e, 55, false);
    finish(&mut e);
    assert!(e.tracks[0].audio_sequence);
    assert_eq!(e.tracks[0].audio, pcm);
    assert_eq!(
        serde_json::to_value(e.tracks[0].chops).unwrap(),
        serde_json::to_value(chops).unwrap()
    );
    assert_eq!(e.tracks[0].source_ticks, 4 * PPQ);
    assert!(
        e.tracks[0]
            .events
            .iter()
            .any(|n| n.note == 55 && n.on && !n.slice)
    );
    assert!(frames(&mut e, 16000) > 0.01);
}
#[test]
fn chop_hold_repeats_only_selected_region_and_release_is_owned() {
    let mut e = core();
    source(&mut e);
    e.tracks[0].audio[..500].fill(Frame::mono(0.2));
    e.tracks[0].audio[500..].fill(Frame::mono(-0.2));
    e.command(Command::AudioTarget(0, Target::Chop)).unwrap();
    note(&mut e, 0, true);
    for _ in 0..2500 {
        assert!(e.process(Frame::ZERO).l >= 0.);
    }
    note(&mut e, 0, false);
    frames(&mut e, 100);
    assert_eq!(e.slice_activity(0).count_ones(), 0);
}
#[test]
fn chop_record_replays_as_chops_after_target_changes_and_can_overdub_chromatic() {
    let mut e = core();
    source(&mut e);
    e.command(Command::AudioTarget(0, Target::Chop)).unwrap();
    e.command(Command::LoopRecord(0)).unwrap();
    frames(&mut e, 100);
    note(&mut e, 3, true);
    frames(&mut e, 600);
    note(&mut e, 3, false);
    finish(&mut e);
    assert!(e.tracks[0].events.iter().all(|n| n.slice));
    let old = e.tracks[0].events.clone();
    e.command(Command::AudioTarget(0, Target::Chromatic))
        .unwrap();
    assert!(frames(&mut e, 16000) > 0.01);
    e.command(Command::RecordMode(0, RecordMode::Overdub))
        .unwrap();
    e.command(Command::LoopRecord(0)).unwrap();
    let start = e.clock.frame_at(e.recorder.unwrap().start);
    until(&mut e, start + 80);
    note(&mut e, 60, true);
    frames(&mut e, 500);
    note(&mut e, 60, false);
    finish(&mut e);
    assert!(old.iter().all(|n| e.tracks[0].events.contains(n)));
    assert!(e.tracks[0].events.iter().any(|n| !n.slice && n.note == 60));
}
#[test]
fn sequence_undo_redo_and_input_replace_undo_preserve_original_source() {
    let mut e = core();
    source(&mut e);
    let pcm = e.tracks[0].audio.clone();
    e.command(Command::AudioTarget(0, Target::Chromatic))
        .unwrap();
    e.command(Command::LoopRecord(0)).unwrap();
    note(&mut e, 52, true);
    frames(&mut e, 100);
    note(&mut e, 52, false);
    finish(&mut e);
    let notes = e.tracks[0].events.clone();
    e.command(Command::Undo).unwrap();
    assert!(!e.tracks[0].audio_sequence);
    assert!(e.tracks[0].events.is_empty());
    assert_eq!(e.tracks[0].audio, pcm);
    e.command(Command::Redo).unwrap();
    assert_eq!(e.tracks[0].events, notes);
    e.command(Command::AudioTarget(0, Target::Input)).unwrap();
    e.command(Command::LoopRecord(0)).unwrap();
    let start = e.clock.frame_at(e.recorder.unwrap().start);
    until(&mut e, start + 100);
    finish(&mut e);
    assert!(!e.tracks[0].audio_sequence);
    assert!(e.tracks[0].events.is_empty());
    e.command(Command::Undo).unwrap();
    assert!(e.tracks[0].audio_sequence);
    assert_eq!(e.tracks[0].audio, pcm);
    assert_eq!(e.tracks[0].events, notes);
}
#[test]
fn empty_sampler_refuses_capture_and_changing_target_during_capture_is_refused() {
    let mut e = core();
    e.command(Command::AudioTarget(0, Target::Chop)).unwrap();
    assert_eq!(e.command(Command::LoopRecord(0)), Err(Error::NoContent));
    assert!(!e.running);
    source(&mut e);
    e.command(Command::LoopRecord(0)).unwrap();
    assert_eq!(
        e.command(Command::AudioTarget(0, Target::Input)),
        Err(Error::Busy)
    );
}
#[test]
fn snapshot_and_ui_choose_three_targets_without_leaving_audio_sound() {
    let mut e = core();
    source(&mut e);
    let mut u = Ui::default();
    u.navigate(Page::Play, &Snapshot::capture(&e));
    assert_eq!(u.page, Page::Play);
    for expected in [Target::Chromatic, Target::Chop, Target::Input] {
        u.soft(0, false, &Snapshot::capture(&e), |c| {
            e.command(c).unwrap();
        });
        assert_eq!(e.tracks[0].audio_target, expected);
        assert_eq!(u.page, Page::Play);
    }
    u.knob(2, 1., &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    assert!(e.tracks[0].audio_settings.warp);
}
#[test]
fn runtime_sampler_capture_requires_no_audio_input_stream() {
    let r = Box::new(Runtime::default());
    let mut e = core();
    source(&mut e);
    e.command(Command::AudioTarget(0, Target::Chromatic))
        .unwrap();
    r.initialize(Box::new(e)).ok().unwrap();
    r.active.store(true, Ordering::Release);
    r.enqueue(Command::LoopRecord(0));
    r.output(&mut [0; 256], 2, 8000);
    r.with_engine(|e| assert!(e.as_ref().unwrap().recorder.unwrap().midi));
    r.external_note(256, 60, 80, true);
    r.output(&mut [0; 512], 2, 8000);
    r.external_note(256, 60, 0, false);
    r.output(&mut [0; 512], 2, 8000);
    r.with_engine(|e| {
        assert!(
            e.as_ref()
                .unwrap()
                .visible_events(0)
                .iter()
                .any(|n| n.note == 60 && n.on)
        )
    });
}
#[test]
fn warp_identity_preserves_pcm_and_octave_changes_pitch_independently_of_loop_duration() {
    let audio: Vec<_> = (0..8000)
        .map(|i| Frame::mono((i as f32 * std::f32::consts::TAU * 200. / 8000.).sin() * 0.2))
        .collect();
    let mut w = Warp::default();
    for (i, x) in audio.iter().enumerate() {
        assert_eq!(w.sample(&audio, i as f64, 1., 1., 8000), *x);
    }
    for (warp, note, hz) in [(false, 60, 400), (true, 60, 400), (true, 48, 200)] {
        let mut v = Voice::new(
            note,
            1,
            Settings {
                warp,
                ..Settings::default()
            },
        );
        let out: Vec<_> = (0..16000)
            .map(|_| {
                v.sample(
                    &audio,
                    Settings {
                        warp,
                        ..Settings::default()
                    },
                    4 * PPQ,
                    120.,
                    8000,
                )
                .l
            })
            .collect();
        let crossings = out[2000..14000]
            .windows(2)
            .filter(|x| x[0] <= 0. && x[1] > 0.)
            .count();
        assert!(
            (crossings as i32 - (hz * 3 / 2)).abs() < 25,
            "warp={warp} note={note} crossings={crossings}"
        );
        assert!(out.iter().all(|x| x.is_finite() && x.abs() <= 0.201));
    }
}
#[test]
fn project_v5_roundtrip_keeps_source_sequence_target_and_warp() {
    let mut e = core();
    source(&mut e);
    e.tracks[0].audio_settings.warp = true;
    e.command(Command::AudioTarget(0, Target::Chop)).unwrap();
    e.command(Command::LoopRecord(0)).unwrap();
    note(&mut e, 2, true);
    frames(&mut e, 150);
    note(&mut e, 2, false);
    finish(&mut e);
    e.command(Command::Stop).unwrap();
    frames(&mut e, 800);
    let path = std::env::temp_dir().join(format!("lc-sampler-{}.lcp", std::process::id()));
    loop_club::project::save(&e, &path).unwrap();
    let loaded = loop_club::project::load(&path, 8000, 32000).unwrap();
    assert_eq!(loaded.tracks[0].events, e.tracks[0].events);
    assert_eq!(loaded.tracks[0].audio[..8000], e.tracks[0].audio[..8000]);
    assert!(loaded.tracks[0].audio_sequence);
    assert!(loaded.tracks[0].audio_settings.warp);
    assert_eq!(loaded.tracks[0].audio_target, Target::Chop);
    std::fs::remove_file(path).unwrap();
}

#[test]
fn wav_import_can_be_used_for_chromatic_and_chop_recording_without_replacing_the_file() {
    let path = std::env::temp_dir().join(format!("lc-existing-source-{}.wav", std::process::id()));
    let mut bytes = Vec::new();
    bytes.extend(b"RIFF");
    bytes.extend(8036u32.to_le_bytes());
    bytes.extend(b"WAVEfmt ");
    bytes.extend(16u32.to_le_bytes());
    bytes.extend(1u16.to_le_bytes());
    bytes.extend(1u16.to_le_bytes());
    bytes.extend(8000u32.to_le_bytes());
    bytes.extend(16000u32.to_le_bytes());
    bytes.extend(2u16.to_le_bytes());
    bytes.extend(16u16.to_le_bytes());
    bytes.extend(b"data");
    bytes.extend(8000u32.to_le_bytes());
    for i in 0..4000 {
        bytes.extend(
            (((i as f64 * std::f64::consts::TAU * 220. / 8000.).sin() * 10000.) as i16)
                .to_le_bytes(),
        );
    }
    std::fs::write(&path, &bytes).unwrap();
    let mut e = core();
    let mut job = loop_club::sample::prepare(&path, 0, 8000, 240., 32000).unwrap();
    job.commit(&mut e);
    assert!(job.error.is_none());
    assert_eq!(e.tracks[0].source_frames, 4000);
    assert_eq!(e.tracks[0].audio_target, Target::Input);
    for target in [Target::Chromatic, Target::Chop] {
        e.command(Command::AudioTarget(0, target)).unwrap();
        e.command(Command::LoopRecord(0)).unwrap();
        let start = e.clock.frame_at(e.recorder.unwrap().start);
        until(&mut e, start + 100);
        let n = if target == Target::Chop { 2 } else { 55 };
        note(&mut e, n, true);
        frames(&mut e, 150);
        note(&mut e, n, false);
        finish(&mut e);
        assert_eq!(e.tracks[0].source_frames, 4000);
        assert!(e.tracks[0].events.iter().any(|e| e.on && e.note == n));
    }
    assert_eq!(std::fs::read(&path).unwrap(), bytes);
    std::fs::remove_file(path).unwrap();
}
#[test]
fn stopped_warp_audio_allows_bpm_change_without_rewriting_source_ticks() {
    let mut e = core();
    source(&mut e);
    assert_eq!(e.command(Command::Bpm(100.)), Err(Error::TempoFrozen));
    let settings = Settings {
        warp: true,
        ..Default::default()
    };
    e.command(Command::AudioSettings(0, settings)).unwrap();
    e.command(Command::Bpm(100.)).unwrap();
    assert_eq!(e.tracks[0].source_ticks, 4 * PPQ);
    assert_eq!(e.tracks[0].audio_frames, 8000);
}
#[test]
fn loop_mixer_controls_do_not_change_sampler_settings_and_source_picker_is_explicit() {
    let mut e = core();
    source(&mut e);
    let mut u = Ui::default();
    u.knob(0, 0.8, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    assert!((e.tracks[0].mix.pan - 0.6).abs() < 1e-5);
    assert_eq!(e.tracks[0].audio_settings.root, 48);
    u.navigate(Page::Source, &Snapshot::capture(&e));
    u.pad(6, true, 90, false, false, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    assert_eq!(e.tracks[0].audio_target, Target::Chop);
    assert_eq!(u.page, Page::Source);
}

#[test]
fn unfinished_first_input_take_cannot_audition_stale_track_pcm() {
    let mut e = core();
    e.tracks[0].audio.fill(Frame::mono(0.3));
    e.command(Command::LoopRecord(0)).unwrap();
    for _ in 0..200 {
        e.process(Frame::mono(0.1));
    }
    assert!(e.tracks[0].audio_frames > 0);
    assert_eq!(
        e.command(Command::Note {
            track: 0,
            note: 48,
            velocity: 90,
            on: true,
            source: 100
        }),
        Err(Error::NoContent)
    );
    assert_eq!(e.slice_activity(0), 0);
    e.command(Command::CancelRecord).unwrap();
    assert!(e.tracks[0].empty());
}
