use loop_club::{
    engine::{Command, Engine},
    model::*,
    motion::*,
    project,
    ui::{Snapshot, Ui},
};
#[test]
fn sidechain_attack_release_targets_and_sync() {
    let mut d = Duck::default();
    let c = Sidechain {
        depth: 1.,
        threshold: 0.05,
        ..Sidechain::default()
    };
    assert_eq!(d.sample(0., c, 8000., 88.), 1.);
    for _ in 0..1000 {
        d.sample(0.3, c, 8000., 88.);
    }
    assert!(d.reduction() > 0.9);
    let early = d.reduction();
    for _ in 0..16000 {
        d.sample(0., c, 8000., 88.);
    }
    assert!(d.reduction() < early * 0.1);
    assert!(!c.targets(0));
    assert!(c.targets(15));
    let selected = Sidechain {
        mode: 1,
        mask: 4,
        ..c
    };
    assert!(selected.targets(2));
    assert!(!selected.targets(1));
    let master = Sidechain { mode: 2, ..c };
    assert!(!(0..16).any(|i| master.targets(i)));
    let sync = Sidechain {
        sync: true,
        release: 0.5,
        ..c
    };
    assert!((sync.release_seconds(120.) - 0.5).abs() < 1e-6);
}
fn tone() -> Engine {
    let mut e = Engine::new(8000, 88., 8000);
    e.tracks[1].kind = Kind::Synth;
    e.tracks[1].synth.lfo_pan = 1.;
    e.tracks[1].synth.lfo_rate = 2.;
    e.tracks[1].synth.cutoff = 8000.;
    e.command(Command::Note {
        track: 1,
        note: 60,
        velocity: 100,
        on: true,
        source: 1,
    })
    .unwrap();
    e
}
#[test]
fn auto_pan_moves_both_directions_and_static_pan_is_shared() {
    let mut e = tone();
    let (mut left, mut right) = (0usize, 0usize);
    for _ in 0..12000 {
        let s = e.process(Frame::ZERO);
        if s.l.abs() > s.r.abs() * 1.3 {
            left += 1
        }
        if s.r.abs() > s.l.abs() * 1.3 {
            right += 1
        }
    }
    assert!(left > 1000 && right > 1000, "{left} {right}");
    let mut ui = Ui::default();
    ui.selected = 1;
    ui.synth_page = 7;
    ui.navigate(Page::Play, &Snapshot::capture(&e));
    let s = Snapshot::capture(&e);
    ui.knob(0, 0., &s, |c| e.command(c).unwrap());
    ui.knob(1, 0., &s, |c| e.command(c).unwrap());
    assert_eq!(e.tracks[1].mix.pan, -1.);
    assert_eq!(e.tracks[1].synth.lfo_pan, 0.);
    for _ in 0..1000 {
        assert_eq!(e.process(Frame::ZERO).r, 0.);
    }
}
#[test]
fn source_mute_prevents_trigger_and_other_tracks_duck() {
    fn fixture(mute: bool) -> Engine {
        let mut e = tone();
        e.tracks[1].synth.lfo_pan = 0.;
        e.tracks[0].kind = Kind::Drum;
        e.tracks[0].kit.clips[0] = vec![Frame::mono(0.5); 2000];
        e.tracks[0].mix.mute = mute;
        e.sidechain = Sidechain {
            depth: 1.,
            threshold: 0.02,
            ..Sidechain::default()
        };
        e.command(Command::Note {
            track: 0,
            note: 0,
            velocity: 127,
            on: true,
            source: 1,
        })
        .unwrap();
        e
    }
    let mut audible = fixture(false);
    let mut muted = fixture(true);
    for _ in 0..500 {
        audible.process(Frame::ZERO);
        muted.process(Frame::ZERO);
    }
    assert!(audible.duck.reduction() > 0.8);
    assert_eq!(muted.duck.reduction(), 0.);
    // Source drum selection excludes unrelated drum voices.
    let mut e = fixture(false);
    e.sidechain.sound = 1;
    for _ in 0..500 {
        e.process(Frame::ZERO);
    }
    assert_eq!(e.duck.reduction(), 0.);
}
#[test]
fn texture_simultaneous_bypass_and_finite_extremes() {
    let mut state = TextureState::new(8000);
    let off = Texture::default();
    assert_eq!(
        state.process(Frame::mono(0.25), off, 8000., 88.),
        Frame::mono(0.25)
    );
    let on = Texture {
        enabled: [true; 5],
        params: [[1.; 3]; 5],
        ..Texture::default()
    };
    let mut changed = false;
    for i in 0..24000 {
        let input = Frame::mono(if i % 200 < 100 { 0.3 } else { -0.3 });
        let out = state.process(input, on, 8000., 88.);
        assert!(out.finite());
        changed |= (out.l - input.l).abs() > 0.01;
    }
    assert!(changed);
}
#[test]
fn motion_roundtrip_and_missing_legacy_defaults() {
    let mut e = tone();
    e.command(Command::Stop).unwrap();
    for _ in 0..8000 {
        e.process(Frame::ZERO);
    }
    e.sidechain = Sidechain {
        source: 3,
        sound: 16,
        mode: 1,
        mask: 12,
        depth: 0.7,
        sync: true,
        ..Sidechain::default()
    };
    e.texture.enabled = [true; 5];
    let p = std::env::temp_dir().join(format!("lc-motion-{}.lcproj", std::process::id()));
    project::save(&e, &p).unwrap();
    let load = project::load(&p, 8000, 8000).unwrap();
    assert_eq!(load.sidechain.source, 3);
    assert_eq!(load.texture.enabled, [true; 5]);
    assert_eq!(load.tracks[1].synth.lfo_pan, 1.);
    let bytes = std::fs::read(&p).unwrap();
    let n = u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize;
    let mut v: serde_json::Value = serde_json::from_slice(&bytes[8..8 + n]).unwrap();
    v.as_object_mut().unwrap().remove("sidechain");
    v.as_object_mut().unwrap().remove("texture");
    for t in v["tracks"].as_array_mut().unwrap() {
        t["synth"].as_object_mut().unwrap().remove("lfo_pan");
    }
    let json = serde_json::to_vec(&v).unwrap();
    let mut b = b"LC01".to_vec();
    b.extend((json.len() as u32).to_le_bytes());
    b.extend(json);
    b.extend(&bytes[8 + n..]);
    std::fs::write(&p, b).unwrap();
    let old = project::load(&p, 8000, 8000).unwrap();
    assert_eq!(old.sidechain.depth, 0.);
    assert_eq!(old.tracks[1].synth.lfo_pan, 0.);
    assert_eq!(old.texture.enabled, [false; 5]);
    std::fs::remove_file(p).unwrap();
}
#[test]
fn texture_and_sidechain_controls_do_not_overwrite_track_fx() {
    let mut e = Engine::new(8000, 88., 8000);
    let mut ui = Ui::default();
    ui.navigate(Page::Texture, &Snapshot::capture(&e));
    for pad in 0..5 {
        let s = Snapshot::capture(&e);
        ui.pad(pad, true, 100, false, false, &s, |c| e.command(c).unwrap());
    }
    assert_eq!(e.texture.enabled, [true; 5]);
    assert_eq!(e.tracks[0].fx[0].effect, Effect::Off);
    ui.navigate(Page::Sidechain, &Snapshot::capture(&e));
    let s = Snapshot::capture(&e);
    ui.knob(0, 0.8, &s, |c| e.command(c).unwrap());
    assert_eq!(e.sidechain.depth, 0.8);
    let s = Snapshot::capture(&e);
    ui.knob(2, 1., &s, |c| e.command(c).unwrap());
    assert_eq!(e.sidechain.threshold, 0.01);
    let invalid = Sidechain {
        source: 255,
        ..Sidechain::default()
    };
    assert!(e.command(Command::Sidechain(invalid)).is_err());
}

#[test]
fn master_subpages_fader_controls_master_without_touching_track() {
    let mut e = Engine::new(8000, 88., 8000);
    let mut ui = Ui::default();
    for page in [Page::Texture, Page::Sidechain] {
        let s = Snapshot::capture(&e);
        ui.navigate(page, &s);
        ui.fader(1., &s, |c| e.command(c).unwrap());
        ui.fader(0.4, &s, |c| e.command(c).unwrap());
        assert_eq!(e.master, 0.4);
        assert_eq!(e.tracks[0].mix.level, 0.7);
    }
}

#[test]
fn direct_pad_toggle_keeps_two_effects_and_refuses_silent_replacement() {
    let mut e = Engine::new(8000, 88., 8000);
    let mut ui = Ui::default();
    ui.navigate(Page::TrackFx, &Snapshot::capture(&e));
    fn tap(ui: &mut Ui, e: &mut Engine, pad: usize) {
        for on in [true, false] {
            let s = Snapshot::capture(e);
            ui.pad(pad, on, 90, true, false, &s, |c| e.command(c).unwrap());
        }
    }
    tap(&mut ui, &mut e, 0);
    tap(&mut ui, &mut e, 5);
    assert_eq!(e.tracks[0].fx[0].effect, Effect::Filter);
    assert_eq!(e.tracks[0].fx[1].effect, Effect::DubEcho);
    tap(&mut ui, &mut e, 7);
    assert_eq!(e.tracks[0].fx[1].effect, Effect::DubEcho);
    assert!(!ui.notice.is_empty());
    tap(&mut ui, &mut e, 0);
    assert_eq!(e.tracks[0].fx[0].effect, Effect::Off);
    assert_eq!(e.tracks[0].fx[1].effect, Effect::DubEcho);
    tap(&mut ui, &mut e, 7);
    assert_eq!(e.tracks[0].fx[0].effect, Effect::Wobble);
}
#[test]
fn master_pad_bypass_preserves_effect_and_knobs_and_recall() {
    let mut e = Engine::new(8000, 88., 8000);
    let mut ui = Ui::default();
    ui.navigate(Page::Texture, &Snapshot::capture(&e));
    fn tap(ui: &mut Ui, e: &mut Engine, pad: usize) {
        for on in [true, false] {
            let s = Snapshot::capture(e);
            ui.pad(pad, on, 90, true, false, &s, |c| e.command(c).unwrap());
        }
    }
    tap(&mut ui, &mut e, 6);
    e.performance[0].a = 0.17;
    tap(&mut ui, &mut e, 6);
    assert!(e.texture.master_bypass[0]);
    assert_eq!(e.performance[0].effect, Effect::Filter);
    assert_eq!(e.performance[0].a, 0.17);
    let p = std::env::temp_dir().join(format!("lc-bypass-{}.lcproj", std::process::id()));
    project::save(&e, &p).unwrap();
    let mut e = project::load(&p, 8000, 8000).unwrap();
    assert!(e.texture.master_bypass[0]);
    tap(&mut ui, &mut e, 6);
    assert!(!e.texture.master_bypass[0]);
    assert_eq!(e.performance[0].a, 0.17);
    std::fs::remove_file(p).unwrap();
}
#[test]
fn master_pad_duck_bypass_keeps_depth_and_source() {
    let mut e = Engine::new(8000, 88., 8000);
    e.sidechain.depth = 0.8;
    e.sidechain.source = 4;
    let mut ui = Ui::default();
    ui.navigate(Page::Texture, &Snapshot::capture(&e));
    for enabled in [false, true] {
        let s = Snapshot::capture(&e);
        ui.pad(5, true, 90, true, false, &s, |c| e.command(c).unwrap());
        assert_eq!(e.sidechain.enabled, enabled);
        assert_eq!(e.sidechain.depth, 0.8);
        assert_eq!(e.sidechain.source, 4);
    }
}

#[test]
fn custom_master_bypass_restores_audio_without_destroying_effect() {
    let mut e = Engine::new(8000, 240., 8000);
    e.tracks[0].audio.fill(Frame::mono(0.2));
    e.tracks[0].audio_frames = 8000;
    e.tracks[0].length_ticks = 4 * PPQ;
    e.performance[0] = Fx {
        effect: Effect::Isolator,
        a: 0.,
        b: 0.,
        mix: 1.,
        ..Fx::default()
    };
    e.command(Command::PlayAll).unwrap();
    for _ in 0..5000 {
        e.process(Frame::ZERO);
    }
    let quiet = e.process(Frame::ZERO).peak();
    e.command(Command::MasterBypass {
        slot: 0,
        bypass: true,
    })
    .unwrap();
    for _ in 0..5000 {
        e.process(Frame::ZERO);
    }
    let audible = e.process(Frame::ZERO).peak();
    assert!(quiet < 0.001 && audible > 0.05, "{quiet} {audible}");
    assert_eq!(e.performance[0].effect, Effect::Isolator);
}
