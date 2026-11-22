use loop_club::{
    engine::Engine,
    model::*,
    synth::{Envelope, SynthVoice},
    ui::{Snapshot, Ui},
    view::{self, Painter},
};
fn held() -> Synth {
    Synth {
        attack: 0.001,
        decay: 0.001,
        sustain: 1.,
        release: 0.01,
        detune: 0.,
        level2: 0.,
        wave: 0,
        cutoff: 18000.,
        ..Synth::default()
    }
}
fn render(s: Synth, hz: f32, n: usize) -> Vec<f32> {
    let mut v = SynthVoice::default();
    (0..n).map(|_| v.sample(48000., hz, s, true)).collect()
}
fn rms(x: &[f32]) -> f32 {
    (x.iter().map(|v| v * v).sum::<f32>() / x.len() as f32).sqrt()
}
#[test]
fn adsr_reaches_peak_sustain_and_releases_from_current_level() {
    let s = Synth {
        attack: 0.01,
        decay: 0.02,
        sustain: 0.3,
        release: 0.04,
        ..Synth::default()
    };
    let mut env = Envelope::default();
    let mut x = 0.;
    for _ in 0..80 {
        x = env.sample(8000., s, true);
    }
    assert!((x - 1.).abs() < 0.02);
    for _ in 0..170 {
        x = env.sample(8000., s, true);
    }
    assert!((x - 0.3).abs() < 0.01);
    let first = env.sample(8000., s, false);
    assert!((first - 0.3).abs() < 0.01);
    for _ in 0..330 {
        x = env.sample(8000., s, false);
    }
    assert_eq!(x, 0.);
    assert!(env.done());
}
#[test]
fn independent_oscillator_tune_is_one_octave_and_levels_can_silence() {
    let a = render(held(), 220., 48000);
    let b = render(
        Synth {
            tune1: 12.,
            ..held()
        },
        220.,
        48000,
    );
    let count = |x: &[f32]| {
        x[4800..]
            .windows(2)
            .filter(|w| w[0] <= 0. && w[1] > 0.)
            .count()
    };
    let ratio = count(&b) as f32 / count(&a) as f32;
    assert!((ratio - 2.).abs() < 0.02);
    let second = render(
        Synth {
            level1: 0.,
            level2: 0.5,
            wave2: 0,
            tune2: 12.,
            ..held()
        },
        220.,
        48000,
    );
    assert!((count(&second) as f32 / count(&a) as f32 - 2.).abs() < 0.02);
    assert!(
        render(
            Synth {
                level1: 0.,
                level2: 0.,
                ..held()
            },
            220.,
            4096
        )
        .iter()
        .all(|v| *v == 0.)
    );
}
#[test]
fn filter_types_and_envelope_change_audio_independently_of_amp() {
    let low = render(
        Synth {
            cutoff: 100.,
            ..held()
        },
        4000.,
        10000,
    );
    let high = render(
        Synth {
            cutoff: 8000.,
            ..held()
        },
        4000.,
        10000,
    );
    assert!(rms(&low[5000..]) < rms(&high[5000..]) * 0.01);
    let hp = render(
        Synth {
            cutoff: 4000.,
            filter: 2,
            ..held()
        },
        100.,
        10000,
    );
    let lp = render(
        Synth {
            cutoff: 4000.,
            filter: 0,
            ..held()
        },
        100.,
        10000,
    );
    assert!(rms(&hp[5000..]) < rms(&lp[5000..]) * 0.01);
    let dry = Synth {
        wave: 1,
        cutoff: 200.,
        ..held()
    };
    let a = render(dry, 220., 4096);
    let b = render(
        Synth {
            env_amount: 1.,
            filter_attack: 0.001,
            filter_decay: 0.1,
            filter_sustain: 0.,
            ..dry
        },
        220.,
        4096,
    );
    assert!(a.iter().zip(&b).map(|(a, b)| (a - b).abs()).sum::<f32>() > 10.);
    assert_eq!(
        dry.attack,
        Synth {
            env_amount: 1.,
            ..dry
        }
        .attack
    );
}
#[test]
fn all_wave_pairs_filter_extremes_and_rates_are_finite() {
    for rate in [8000., 44100., 48000., 96000.] {
        for a in 0..4 {
            for b in 0..4 {
                for filter in 0..3 {
                    let s = Synth {
                        wave: a,
                        wave2: b,
                        filter,
                        level1: 1.,
                        level2: 1.,
                        resonance: 1.,
                        cutoff: 18000.,
                        env_amount: 1.,
                        tune1: 24.,
                        tune2: -24.,
                        ..held()
                    };
                    let mut v = SynthVoice::default();
                    for i in 0..1500 {
                        let x = v.sample(rate, 2000., s, i < 1000);
                        assert!(x.is_finite() && x.abs() < 1000.);
                    }
                }
            }
        }
    }
}
struct Bounds;
impl Painter for Bounds {
    fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, _: u32) {
        assert!(x >= 0 && y >= 0 && x + w <= 320 && y + h <= 240);
    }
    fn text(&mut self, x: i32, y: i32, w: i32, h: i32, _: f32, _: u32, _: &str) {
        assert!(x >= 0 && y >= 0 && x + w <= 320 && y + h <= 240);
    }
}
#[test]
fn synth_pages_map_knobs_without_changing_chords_or_record_mode() {
    let mut e = Engine::new(8000, 240., 16000);
    e.tracks[0].kind = Kind::Synth;
    let mut ui = Ui::default();
    ui.page = Page::Play;
    ui.chord_modes[0] = true;
    for page in 1..=6 {
        for shift in [false, true] {
            ui.synth_page = page;
            ui.shift_view = shift;
            for index in 0..3 {
                ui.knob(index, 0.75, &Snapshot::capture(&e), |c| {
                    e.command(c).unwrap();
                });
            }
            assert!(e.tracks[0].synth.valid());
            view::render(&ui, &Snapshot::capture(&e), &mut Bounds);
        }
    }
    assert_eq!(e.tracks[0].chord.key, 0);
    assert_eq!(e.tracks[0].record_mode, RecordMode::Replace);
    assert_eq!(e.tracks[0].synth.tune1, 12.);
    assert_eq!(e.tracks[0].synth.tune2, 12.);
    assert_eq!(e.tracks[0].synth.fine1, 50.);
    assert_eq!(e.tracks[0].synth.fine2, 50.);
    let restored = Snapshot::capture(&e).synth();
    assert_eq!(
        serde_json::to_value(restored).unwrap(),
        serde_json::to_value(e.tracks[0].synth).unwrap()
    );
}
#[test]
fn legacy_synth_state_defaults_new_controls_and_new_state_roundtrips() {
    let old = r#"{"wave":2,"cutoff":1000.0,"attack":0.01,"release":0.3,"detune":0.003,"octave":0}"#;
    let s: Synth = serde_json::from_str(old).unwrap();
    assert!(s.valid());
    assert_eq!(s.filter, 0);
    let s = Synth {
        wave2: 3,
        tune1: -12.,
        tune2: 7.,
        fine2: 13.,
        env_amount: 0.8,
        filter_release: 1.2,
        ..s
    };
    let copy: Synth = serde_json::from_slice(&serde_json::to_vec(&s).unwrap()).unwrap();
    assert!(copy.valid());
    assert_eq!(copy.tune2, 7.);
    assert_eq!(copy.filter_release, 1.2);
}

#[test]
fn lfo_has_saw_directions_square_triangle_sine_and_random_hold() {
    use loop_club::synth::Lfo;
    let values = |shape| {
        let mut l = Lfo::default();
        (0..125)
            .map(|_| l.sample(100., 1., shape))
            .collect::<Vec<_>>()
    };
    let sine = values(0);
    let tri = values(1);
    let up = values(2);
    let down = values(3);
    let sq = values(4);
    let random = values(5);
    assert!((sine[25] - 1.).abs() < 1e-5 && (sine[75] + 1.).abs() < 1e-5);
    assert!((tri[25] - 1.).abs() < 1e-5 && (tri[75] + 1.).abs() < 1e-5);
    assert_eq!(up[0], -1.);
    assert!((up[75] - 0.5).abs() < 1e-5);
    for i in 0..125 {
        assert!((up[i] + down[i]).abs() < 1e-5);
    }
    assert_eq!(sq[0], 1.);
    assert_eq!(sq[75], -1.);
    assert!(random[..100].iter().all(|v| *v == random[0]));
    assert_ne!(random[110], random[0]);
    for shape in 0..6 {
        assert!(
            values(shape)
                .iter()
                .all(|v| v.is_finite() && (-1.0..=1.0).contains(v))
        );
    }
}
#[test]
fn zero_depth_preserves_sound_and_each_lfo_target_changes_only_its_routing() {
    let base = held();
    let dry = render(base, 220., 48000);
    for shape in 0..6 {
        assert_eq!(
            dry,
            render(
                Synth {
                    lfo_rate: 13.,
                    lfo_shape: shape,
                    ..base
                },
                220.,
                48000
            )
        );
    }
    let pitch = render(
        Synth {
            lfo_rate: 1.,
            lfo_pitch: 200.,
            ..base
        },
        220.,
        48000,
    );
    let count = |x: &[f32]| x.windows(2).filter(|w| w[0] <= 0. && w[1] > 0.).count();
    assert!(count(&pitch[6000..18000]) > count(&pitch[30000..42000]));
    let trem = render(
        Synth {
            lfo_rate: 1.,
            lfo_volume: 1.,
            ..base
        },
        220.,
        48000,
    );
    assert!(rms(&trem[9000..15000]) < rms(&trem[33000..39000]) * 0.1);
    let dark = Synth {
        wave: 1,
        cutoff: 400.,
        ..base
    };
    let a = render(dark, 220., 48000);
    let b = render(
        Synth {
            lfo_filter: 4.,
            ..dark
        },
        220.,
        48000,
    );
    assert!(a.iter().zip(&b).map(|(a, b)| (a - b).abs()).sum::<f32>() > 100.);
}
#[test]
fn all_lfo_targets_at_extremes_are_finite_and_pages_save_independent_depths() {
    for rate in [8000., 48000., 96000.] {
        for shape in 0..6 {
            let s = Synth {
                lfo_rate: 20.,
                lfo_shape: shape,
                lfo_pitch: 200.,
                lfo_filter: 4.,
                lfo_volume: 1.,
                resonance: 1.,
                cutoff: 18000.,
                env_amount: 1.,
                ..held()
            };
            let mut v = SynthVoice::default();
            for n in 0..4000 {
                assert!(v.sample(rate, 1800., s, n < 2000).is_finite());
            }
        }
    }
    let mut e = Engine::new(8000, 240., 16000);
    e.tracks[0].kind = Kind::Synth;
    let mut ui = Ui::default();
    ui.page = Page::Play;
    ui.synth_page = 6;
    for shift in [false, true] {
        ui.shift_view = shift;
        for index in 0..3 {
            ui.knob(index, 0.5, &Snapshot::capture(&e), |c| {
                e.command(c).unwrap();
            });
        }
    }
    let s = e.tracks[0].synth;
    assert_eq!(s.lfo_pitch, 100.);
    assert_eq!(s.lfo_filter, 2.);
    assert_eq!(s.lfo_volume, 0.5);
    assert_eq!(s.lfo_shape, 2);
    assert_eq!(
        serde_json::to_value(s).unwrap(),
        serde_json::to_value(Snapshot::capture(&e).synth()).unwrap()
    );
}

#[test]
fn phase_sine_keeps_wave_shape_within_one_part_per_million() {
    for i in 0..100000 {
        let phase = i as f32 / 100000.;
        let expected = (phase * std::f32::consts::TAU).sin();
        assert!((loop_club::synth::phase_sine(phase) - expected).abs() < 0.000001);
    }
}
