use loop_club::{
    dsp::Voice,
    engine::{Command, Engine},
    model::*,
    project,
    ui::{Snapshot, Ui},
};
fn hit(e: &mut Engine, sound: u8) {
    e.command(Command::Note {
        track: 0,
        note: sound,
        velocity: 127,
        on: true,
        source: 7,
    })
    .unwrap();
}
fn file() -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "lc-layers-{}-{}.lcproj",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ))
}
#[test]
fn layers_are_independent_and_open_hat_chokes_both() {
    let mut e = Engine::new(8000, 240., 8000);
    e.tracks[0].kind = Kind::Drum;
    hit(&mut e, 3);
    assert_eq!(e.active_voices(), 1);
    e.kill_track_voices(0);
    e.tracks[0].drums[3].layers[1].enabled = true;
    hit(&mut e, 3);
    assert_eq!(e.active_voices(), 2);
    hit(&mut e, 2);
    assert_eq!(e.active_voices(), 1);
}
#[test]
fn layer_shift_delays_onset_without_allocating_a_second_timeline() {
    let mut e = Engine::new(8000, 240., 8000);
    e.tracks[0].kind = Kind::Drum;
    e.tracks[0].kit.secondary[0] = vec![Frame::mono(0.5); 100];
    e.tracks[0].drums[0].layers[0].enabled = false;
    let cfg = &mut e.tracks[0].drums[0].layers[1];
    cfg.enabled = true;
    cfg.level = 1.;
    cfg.shift = 10.;
    hit(&mut e, 0);
    for _ in 0..80 {
        assert_eq!(e.process(Frame::ZERO), Frame::ZERO);
    }
    assert!(e.process(Frame::ZERO).peak() > 0.01);
}
#[test]
fn sample_reverse_tune_and_layer_gain_have_audible_effect() {
    let mut v = Voice::default();
    let mut drum = Drum::default();
    drum.layers[1].reverse = true;
    v.start(0, 0, 127, 1, Some(0), 1, Synth::default(), 0.);
    v.layer = 1;
    let pcm = [Frame::mono(0.1), Frame::mono(0.2), Frame::mono(0.3)];
    assert_eq!(v.sample_pcm(&pcm, 8000., drum), Frame::mono(0.3));
    assert!((v.sample_pcm(&pcm, 8000., drum).l - 0.2).abs() < 1e-6);
    let mut forward = Voice::default();
    forward.start(0, 0, 127, 1, Some(0), 1, Synth::default(), 0.);
    let pitched = Drum {
        tune: 12.,
        ..Drum::default()
    };
    assert_eq!(forward.sample_pcm(&pcm, 8000., pitched), Frame::mono(0.1));
    assert_eq!(forward.sample_pcm(&pcm, 8000., pitched), Frame::mono(0.3));
    let mut e = Engine::new(8000, 240., 8000);
    e.tracks[0].kind = Kind::Drum;
    e.tracks[0].kit.clips[0] = vec![Frame::mono(0.5); 16];
    e.command(Command::DrumValue {
        track: 0,
        sound: 0,
        index: 6,
        value: 0.,
    })
    .unwrap();
    hit(&mut e, 0);
    assert_eq!(e.process(Frame::ZERO), Frame::ZERO);
    e.kill_track_voices(0);
    e.command(Command::DrumValue {
        track: 0,
        sound: 0,
        index: 6,
        value: 1.,
    })
    .unwrap();
    hit(&mut e, 0);
    assert!(e.process(Frame::ZERO).peak() > 0.01);
}
#[test]
fn both_layers_roundtrip_and_old_project_defaults_preserve_primary() {
    let mut e = Engine::new(8000, 240., 8000);
    e.tracks[0].kind = Kind::Drum;
    e.tracks[0].kit.clips[0] = vec![Frame::mono(0.1); 4];
    e.tracks[0].kit.secondary[0] = vec![Frame::mono(0.2); 5];
    e.tracks[0].kit.names[0] = ["Primary".into(), "Layer".into()];
    e.tracks[0].drums[0].layers[1].enabled = true;
    e.tracks[0].drums[0].layers[1].shift = -12.;
    let p = file();
    project::save(&e, &p).unwrap();
    let loaded = project::load(&p, 8000, 8000).unwrap();
    assert_eq!(
        loaded.tracks[0].kit.secondary[0],
        e.tracks[0].kit.secondary[0]
    );
    assert_eq!(loaded.tracks[0].kit.names[0][1], "Layer");
    assert_eq!(loaded.tracks[0].drums[0].layers[1].shift, -12.);
    std::fs::remove_file(&p).unwrap();
    let legacy = Engine::new(8000, 240., 8000);
    project::save(&legacy, &p).unwrap();
    let bytes = std::fs::read(&p).unwrap();
    let n = u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize;
    let mut meta: serde_json::Value = serde_json::from_slice(&bytes[8..8 + n]).unwrap();
    meta["version"] = 2.into();
    for track in meta["tracks"].as_array_mut().unwrap() {
        track.as_object_mut().unwrap().remove("secondary_frames");
        track.as_object_mut().unwrap().remove("layer_names");
        for drum in track["drums"].as_array_mut().unwrap() {
            drum.as_object_mut().unwrap().remove("layers");
        }
    }
    let json = serde_json::to_vec(&meta).unwrap();
    let mut out = b"LC01".to_vec();
    out.extend((json.len() as u32).to_le_bytes());
    out.extend(json);
    out.extend(&bytes[8 + n..]);
    std::fs::write(&p, out).unwrap();
    let old = project::load(&p, 8000, 8000).unwrap();
    assert!(old.tracks[0].drums[0].layers[0].enabled);
    assert!(!old.tracks[0].drums[0].layers[1].enabled);
    std::fs::remove_file(p).unwrap();
}
#[test]
fn layer_knobs_and_source_sound_navigation_leave_total_gain_intact() {
    let mut e = Engine::new(8000, 240., 8000);
    e.tracks[0].kind = Kind::Drum;
    let mut ui = Ui::default();
    ui.soft(0, false, &Snapshot::capture(&e), |_| {});
    assert_eq!(ui.page, Page::Source);
    ui.soft(0, false, &Snapshot::capture(&e), |_| {});
    ui.soft(1, false, &Snapshot::capture(&e), |_| {});
    assert_eq!(ui.page, Page::DrumSound);
    ui.soft(2, false, &Snapshot::capture(&e), |_| {});
    assert_eq!(ui.layer, 1);
    let total = e.tracks[0].drums[0].mix.level;
    for (index, value) in [(0, 0.25), (1, 0.75), (2, 0.6)] {
        ui.knob(index, value, &Snapshot::capture(&e), |c| {
            e.command(c).unwrap()
        });
    }
    let cfg = e.tracks[0].drums[0].layers[1];
    assert_eq!(cfg.level, 0.25);
    assert_eq!(cfg.tune, 12.);
    assert_eq!(cfg.shift, 10.);
    assert_eq!(e.tracks[0].drums[0].mix.level, total);
}
#[test]
fn recording_shorter_second_loop_does_not_resize_first() {
    let mut e = Engine::new(8000, 240., 40000);
    e.preroll = 0;
    e.command(Command::Record(0)).unwrap();
    while e.clock.frame < 24001 {
        e.process(Frame::mono(0.1));
    }
    e.command(Command::Record(0)).unwrap();
    while e.clock.frame < 32001 {
        e.process(Frame::mono(0.1));
    }
    e.command(Command::Record(0)).unwrap();
    while e.clock.frame < 64001 {
        e.process(Frame::mono(0.1));
    }
    assert_eq!(e.tracks[0].length_ticks, 16 * PPQ);
    assert!(e.recorder.is_none());
    e.command(Command::Record(1)).unwrap();
    let start = e.recorder.unwrap().start;
    while e.clock.tick() < start + PPQ / 2 {
        e.process(Frame::mono(0.2));
    }
    e.command(Command::Record(1)).unwrap();
    while !e.recorder.unwrap().repeating {
        e.process(Frame::mono(0.2));
    }
    e.command(Command::Record(1)).unwrap();
    while e.recorder.is_some() {
        e.process(Frame::mono(0.2));
    }
    assert_eq!(e.tracks[0].length_ticks, 16 * PPQ);
    assert_eq!(e.tracks[1].length_ticks, 4 * PPQ);
    assert!(e.tracks[0].playing && e.tracks[1].playing);
}

#[test]
fn sound_reentry_returns_to_primary_controls_without_resetting_layer_values() {
    let mut e = Engine::new(8000, 240., 8000);
    e.tracks[0].kind = Kind::Drum;
    let mut ui = Ui::default();
    let s = Snapshot::capture(&e);
    ui.navigate(Page::DrumSound, &s);
    ui.layer = 1;
    ui.layer_detail = true;
    ui.navigate(Page::Loop, &s);
    ui.soft(1, false, &s, |_| {});
    assert_eq!(ui.page, Page::DrumSound);
    assert!(!ui.layer_detail);
    assert_eq!(ui.layer, 1);
    ui.soft(0, false, &s, |_| {});
    assert_eq!(ui.page, Page::Loop);
}
