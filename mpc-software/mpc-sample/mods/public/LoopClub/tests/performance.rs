use loop_club::{
    audio::{RecordTarget, Settings, Voice as SampleVoice},
    engine::{Command, Engine},
    model::*,
    performance::Settings as Play,
    ui::{Snapshot, Ui},
};
fn core() -> Engine {
    let mut e = Engine::new(8000, 240., 32000);
    e.preroll = 0;
    e.tracks[0].kind = Kind::Synth;
    e.tracks[0].synth.release = 0.01;
    e
}
fn frames(e: &mut Engine, n: usize) -> f32 {
    (0..n)
        .map(|_| e.process(Frame::ZERO).peak())
        .fold(0., f32::max)
}
fn note(e: &mut Engine, n: u8, on: bool, source: u16) {
    e.command(Command::Note {
        track: 0,
        note: n,
        velocity: if on { 100 } else { 0 },
        on,
        source,
    })
    .unwrap();
}
fn chord(e: &mut Engine, on: bool) {
    e.command(Command::Chord {
        track: 0,
        pad: 0,
        velocity: if on { 100 } else { 0 },
        on,
    })
    .unwrap();
}
fn audio(e: &mut Engine) {
    let t = &mut e.tracks[0];
    t.kind = Kind::Audio;
    t.source_frames = 800;
    t.audio_frames = 800;
    t.source_ticks = 4 * PPQ;
    t.length_ticks = 4 * PPQ;
    t.audio_target = RecordTarget::Chromatic;
    t.chops.divide(800);
    for (i, f) in t.audio[..800].iter_mut().enumerate() {
        *f = Frame::mono((i as f32 * 0.2).sin() * 0.2);
    }
}
#[test]
fn mono_has_one_voice_and_falls_back_by_owner() {
    let mut e = core();
    e.tracks[0].play.voice = 1;
    note(&mut e, 48, true, 100);
    frames(&mut e, 100);
    note(&mut e, 60, true, 101);
    assert_eq!(e.active_voices(), 1);
    note(&mut e, 60, false, 101);
    assert!(frames(&mut e, 1000) > 0.01);
    note(&mut e, 48, false, 100);
    frames(&mut e, 200);
    assert_eq!(e.active_voices(), 0);
}
#[test]
fn unrelated_note_off_does_not_end_current_mono_note() {
    let mut e = core();
    e.tracks[0].play.voice = 2;
    note(&mut e, 48, true, 100);
    note(&mut e, 60, true, 101);
    note(&mut e, 48, false, 100);
    frames(&mut e, 500);
    assert_eq!(e.active_voices(), 1);
    note(&mut e, 60, false, 101);
    frames(&mut e, 200);
    assert_eq!(e.active_voices(), 0);
}
#[test]
fn mono_setting_does_not_collapse_chord_voices() {
    let mut e = core();
    e.tracks[0].play.voice = 2;
    chord(&mut e, true);
    assert_eq!(e.active_voices(), 4);
    chord(&mut e, false);
    frames(&mut e, 200);
    assert_eq!(e.active_voices(), 0);
}
#[test]
fn strum_runs_without_transport_and_release_cancels_unplayed_notes() {
    let mut e = core();
    e.tracks[0].chord.strum_ms = 20.;
    chord(&mut e, true);
    assert_eq!(e.active_voices(), 1);
    frames(&mut e, 160);
    assert_eq!(e.active_voices(), 1);
    frames(&mut e, 1);
    assert_eq!(e.active_voices(), 2);
    chord(&mut e, false);
    frames(&mut e, 1600);
    assert_eq!(e.active_voices(), 0);
}
#[test]
fn strum_records_expanded_note_onsets_in_real_time() {
    let mut e = core();
    e.tracks[0].chord.strum_ms = 20.;
    e.command(Command::LoopRecord(0)).unwrap();
    chord(&mut e, true);
    frames(&mut e, 600);
    chord(&mut e, false);
    e.command(Command::LoopRecord(0)).unwrap();
    for _ in 0..16000 {
        e.process(Frame::ZERO);
        if e.recorder.is_none() {
            break;
        }
    }
    let ons: Vec<_> = e.tracks[0].events.iter().filter(|n| n.on).collect();
    assert_eq!(ons.len(), 4);
    assert!(ons.windows(2).all(|p| p[1].tick > p[0].tick));
    assert_eq!(ons[1].tick - ons[0].tick, e.clock.tick_at(160));
}
#[test]
fn audio_chord_uses_source_pitch_and_preserves_pcm() {
    let mut e = core();
    audio(&mut e);
    e.command(Command::ChordMode(0, true)).unwrap();
    let pcm = e.tracks[0].audio[..800].to_vec();
    chord(&mut e, true);
    assert_ne!(e.slice_activity(0).count_ones(), 1);
    assert!(frames(&mut e, 100) > 0.01);
    chord(&mut e, false);
    frames(&mut e, 100);
    assert_eq!(e.slice_activity(0), 0);
    assert!(
        pcm.iter()
            .zip(&e.tracks[0].audio)
            .all(|(a, b)| a.l == b.l && a.r == b.r)
    );
}
#[test]
fn audio_chord_cannot_reinterpret_chop_notes() {
    let mut e = core();
    audio(&mut e);
    e.command(Command::AudioTarget(0, RecordTarget::Chop))
        .unwrap();
    assert!(e.command(Command::ChordMode(0, true)).is_err());
    assert!(
        e.command(Command::Chord {
            track: 0,
            pad: 0,
            velocity: 90,
            on: true
        })
        .is_err()
    );
}
#[test]
fn sampled_one_shot_ends_while_gate_repeats() {
    let pcm = [Frame::mono(0.2); 800];
    let settings = Settings::default();
    let mut shot = SampleVoice::new(settings.root, 2, settings);
    shot.one_shot = true;
    let mut gate = shot;
    gate.one_shot = false;
    for _ in 0..801 {
        shot.sample(&pcm, settings, 4 * PPQ, 240., 8000);
        gate.sample(&pcm, settings, 4 * PPQ, 240., 8000);
    }
    assert!(shot.finished);
    assert!(!gate.finished);
    assert_eq!(shot.sample(&pcm, settings, 4 * PPQ, 240., 8000).peak(), 0.);
}
#[test]
fn audio_one_shot_ignores_release_then_ends_at_sample_endpoint() {
    let mut e = core();
    audio(&mut e);
    e.tracks[0].play.trigger = 2;
    note(&mut e, 48, true, 100);
    note(&mut e, 48, false, 100);
    assert!(frames(&mut e, 100) > 0.01);
    frames(&mut e, 900);
    assert_eq!(e.slice_activity(0), 0);
}
#[test]
fn synth_one_shot_releases_automatically_but_gate_waits_for_release() {
    let mut e = core();
    e.tracks[0].play = Play {
        trigger: 2,
        shot_ms: 30.,
        ..Play::default()
    };
    note(&mut e, 60, true, 100);
    frames(&mut e, 400);
    assert_eq!(e.active_voices(), 0);
    e.tracks[0].play.trigger = 1;
    note(&mut e, 60, true, 100);
    frames(&mut e, 800);
    assert_eq!(e.active_voices(), 1);
    note(&mut e, 60, false, 100);
    frames(&mut e, 200);
    assert_eq!(e.active_voices(), 0);
}
#[test]
fn procedural_drum_gate_cuts_and_one_shot_ignores_note_off() {
    let mut e = core();
    e.tracks[0].kind = Kind::Drum;
    e.tracks[0].drums[0].layers[1].enabled = false;
    e.tracks[0].play.trigger = 1;
    note(&mut e, 0, true, 100);
    frames(&mut e, 50);
    note(&mut e, 0, false, 100);
    frames(&mut e, 100);
    assert_eq!(e.active_voices(), 0);
    e.tracks[0].play.trigger = 2;
    note(&mut e, 0, true, 100);
    note(&mut e, 0, false, 100);
    frames(&mut e, 100);
    assert_eq!(e.active_voices(), 1);
}
#[test]
fn one_shot_strum_finishes_phrase_after_pad_release_and_stop_cancels_it() {
    let mut e = core();
    e.tracks[0].chord.strum_ms = 20.;
    e.tracks[0].play.trigger = 2;
    chord(&mut e, true);
    chord(&mut e, false);
    frames(&mut e, 500);
    assert_eq!(e.active_voices(), 4);
    e.command(Command::Stop).unwrap();
    frames(&mut e, 1000);
    assert_eq!(e.active_voices(), 0);
}
#[test]
fn audio_sound_selector_explicitly_cycles_input_notes_chord_chop() {
    let mut e = core();
    audio(&mut e);
    e.tracks[0].audio_target = RecordTarget::Input;
    let mut u = Ui::default();
    u.navigate(Page::Play, &Snapshot::capture(&e));
    for (target, chord) in [
        (RecordTarget::Chromatic, false),
        (RecordTarget::Chromatic, true),
        (RecordTarget::Chop, false),
        (RecordTarget::Turntable, false),
        (RecordTarget::Input, false),
    ] {
        u.soft(0, false, &Snapshot::capture(&e), |c| {
            e.command(c).unwrap();
        });
        u.sync_chord_modes(&Snapshot::capture(&e));
        assert_eq!(e.tracks[0].audio_target, target);
        assert_eq!(e.tracks[0].chord_mode, chord);
    }
}
#[test]
fn play_page_opens_from_each_sound_editor_and_settings_round_trip() {
    let mut e = core();
    for kind in [Kind::Audio, Kind::Synth, Kind::Drum] {
        e.tracks[0].kind = kind;
        if kind == Kind::Audio {
            audio(&mut e);
        }
        let mut u = Ui::default();
        let s = Snapshot::capture(&e);
        u.navigate(
            if kind == Kind::Drum {
                Page::DrumSound
            } else {
                Page::Play
            },
            &s,
        );
        u.soft(0, true, &s, |_| {});
        assert_eq!(u.page, Page::Perform);
        u.perform_section = 2;
        u.knob(0, 1., &Snapshot::capture(&e), |c| {
            e.command(c).unwrap();
        });
        assert_eq!(Snapshot::capture(&e).play_settings().trigger, 2);
    }
}
#[test]
fn project8_preserves_play_settings_and_copy_keeps_them() {
    let mut e = core();
    e.tracks[0].play = Play {
        voice: 2,
        glide_ms: 220.,
        trigger: 2,
        shot_ms: 700.,
        octave: 0,
    };
    e.tracks[0].chord.strum_ms = 30.;
    e.tracks[0].chord.strum_direction = 2;
    e.tracks[0].length_ticks = 4 * PPQ;
    let path = std::env::temp_dir().join(format!("lc-play-{}.lcproj", std::process::id()));
    loop_club::project::save(&e, &path).unwrap();
    let mut loaded = loop_club::project::load(&path, 8000, 32000).unwrap();
    std::fs::remove_file(path).unwrap();
    assert_eq!(loaded.tracks[0].play.voice, 2);
    assert_eq!(loaded.tracks[0].play.glide_ms, 220.);
    assert_eq!(loaded.tracks[0].play.shot_ms, 700.);
    assert_eq!(loaded.tracks[0].chord.strum_ms, 30.);
    assert_eq!(loaded.tracks[0].chord.strum_direction, 2);
    loaded.transfer_track_stopped(0, 1, false).unwrap();
    assert_eq!(loaded.tracks[1].play.trigger, 2);
    assert_eq!(loaded.tracks[1].chord.strum_ms, 30.);
}
#[test]
fn mono_lane_does_not_steal_polyphonic_chord_owners() {
    let mut e = core();
    e.tracks[0].play.voice = 1;
    chord(&mut e, true);
    note(&mut e, 72, true, 100);
    assert_eq!(e.active_voices(), 5);
    note(&mut e, 76, true, 101);
    assert_eq!(e.active_voices(), 5);
    chord(&mut e, false);
    frames(&mut e, 200);
    assert_eq!(e.active_voices(), 1);
    note(&mut e, 76, false, 101);
    note(&mut e, 72, false, 100);
    frames(&mut e, 200);
    assert_eq!(e.active_voices(), 0);
}
#[test]
fn gate_cancels_delayed_drum_layers_but_native_one_shot_taps_do_not() {
    let mut e = core();
    e.tracks[0].kind = Kind::Drum;
    e.tracks[0].drums[0].layers[1].enabled = false;
    e.tracks[0].drums[0].layers[0].shift = 50.;
    e.tracks[0].play.trigger = 1;
    note(&mut e, 0, true, 100);
    assert_eq!(frames(&mut e, 10), 0.);
    note(&mut e, 0, false, 100);
    assert_eq!(frames(&mut e, 800), 0.);
    assert_eq!(e.active_voices(), 0);
    e.tracks[0].play.trigger = 0;
    note(&mut e, 0, true, 100);
    note(&mut e, 0, false, 100);
    assert!(frames(&mut e, 800) > 0.001);
    e.command(Command::Stop).unwrap();
    frames(&mut e, 16000);
    note(&mut e, 0, true, 100);
    e.command(Command::Stop).unwrap();
    assert_eq!(frames(&mut e, 800), 0.);
}
#[test]
fn loop_shift_k1_octave_records_actual_notes_and_keeps_held_note_release() {
    let mut e = core();
    let mut u = Ui::default();
    u.shift_view = true;
    u.knob(0, 0.75, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    assert_eq!(e.tracks[0].play.octave, 1);
    assert_eq!(e.tracks[0].mix.pan, 0.);
    e.command(Command::LoopRecord(0)).unwrap();
    u.shift_view = false;
    u.pad(0, true, 100, false, false, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    frames(&mut e, 300);
    u.shift_view = true;
    u.knob(0, 0.25, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    assert_eq!(e.tracks[0].play.octave, -1);
    u.pad(0, false, 0, false, false, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    e.command(Command::LoopRecord(0)).unwrap();
    while e.recorder.is_some() {
        e.process(Frame::ZERO);
    }
    let ons: Vec<_> = e.tracks[0].events.iter().filter(|n| n.on).collect();
    let offs: Vec<_> = e.tracks[0].events.iter().filter(|n| !n.on).collect();
    assert_eq!(ons[0].note, 60);
    assert!(offs.iter().any(|n| n.note == 60));
    assert!(!offs.iter().any(|n| n.note == 36));
}
#[test]
fn performance_octave_transposes_synth_and_audio_chords_without_changing_key() {
    for kind in [Kind::Synth, Kind::Audio] {
        let mut e = core();
        if kind == Kind::Audio {
            audio(&mut e);
        }
        e.tracks[0].play.octave = 1;
        e.command(Command::LoopRecord(0)).unwrap();
        chord(&mut e, true);
        frames(&mut e, 200);
        chord(&mut e, false);
        e.command(Command::LoopRecord(0)).unwrap();
        while e.recorder.is_some() {
            e.process(Frame::ZERO);
        }
        let notes: Vec<_> = e.tracks[0]
            .events
            .iter()
            .filter(|n| n.on)
            .map(|n| n.note)
            .collect();
        assert_eq!(notes, [60, 64, 67, 71]);
    }
}
#[test]
fn octave_shortcut_leaves_drum_and_chop_pad_identity_unchanged() {
    let mut e = core();
    let mut u = Ui::default();
    for kind in [Kind::Drum, Kind::Audio] {
        e.tracks[0].kind = kind;
        if kind == Kind::Audio {
            audio(&mut e);
            e.tracks[0].audio_target = RecordTarget::Chop;
        }
        u.shift_view = true;
        u.knob(0, 1., &Snapshot::capture(&e), |c| {
            e.command(c).unwrap();
        });
        assert_eq!(e.tracks[0].play.octave, 0);
    }
}
