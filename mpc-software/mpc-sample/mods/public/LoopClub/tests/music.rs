use loop_club::{model::*, music::*};
#[test]
fn groove_dice_accents_are_strong_weak_medium_weak() {
    assert_eq!(
        [
            hat_accent(0, false),
            hat_accent(240, false),
            hat_accent(480, false),
            hat_accent(720, false)
        ],
        [90, 45, 68, 40]
    );
    assert_eq!(hat_accent(120, false), 38);
}
#[test]
fn generated_patterns_keep_backbeats_and_avoid_kick_on_two_four() {
    for style in 1..=16 {
        for rhythm in 0..24 {
            let c = PatternConfig {
                rules: [style; 4],
                rhythm,
                swing: 50.,
                nudge_ms: 0.,
                snare_late_ms: 0.,
                seed: 42,
            };
            let p = generate(c, 88.);
            let hits = &p.hits[..p.len];
            assert!(hits.iter().any(|h| h.sound == 1 && h.tick == 960));
            assert!(hits.iter().any(|h| h.sound == 1 && h.tick == 2880));
            assert!(
                !hits
                    .iter()
                    .any(|h| h.sound == 0 && (h.tick == 960 || h.tick == 2880))
            );
            assert!(
                hits.iter()
                    .all(|h| h.tick < 3840 && h.velocity >= 1 && h.velocity <= 127 && h.sound < 16)
            );
            assert!(p.len < 128);
            assert!(hits.windows(2).all(|h| h[0].tick <= h[1].tick));
        }
    }
}
#[test]
fn same_seed_is_reproducible_and_new_seeds_vary_motifs() {
    let mut signatures = std::collections::HashSet::new();
    for seed in 1..40 {
        let config = PatternConfig {
            seed,
            swing: 50.,
            nudge_ms: 0.,
            snare_late_ms: 0.,
            ..PatternConfig::default()
        };
        let p = generate(config, 88.);
        let signature: Vec<_> = p.hits[..p.len]
            .iter()
            .filter(|h| h.sound == 0)
            .map(|h| h.tick)
            .collect();
        signatures.insert(signature);
        let q = generate(config, 88.);
        assert_eq!(p.len, q.len);
        assert!(
            p.hits[..p.len]
                .iter()
                .zip(q.hits)
                .all(|(a, b)| a.tick == b.tick && a.sound == b.sound && a.velocity == b.velocity)
        );
    }
    assert!(signatures.len() >= 6);
}
#[test]
fn chord_notes_stay_diatonic_and_have_correct_names() {
    let config = ChordConfig::default();
    assert_eq!(config.notes(0), ([48, 52, 55, 59, 0], 4));
    assert_eq!(config.label(0), "Cmaj7");
    assert_eq!(config.label(1), "Dm7");
    assert_eq!(config.label(6), "Bm7b5");
    let minor = ChordConfig { scale: 1, ..config };
    assert_eq!(minor.label(2), "Ebmaj7");
    for key in 0..12 {
        for scale in 0..3 {
            for color in 0..3 {
                let c = ChordConfig {
                    key,
                    scale,
                    color,
                    ..config
                };
                for pad in 0..16 {
                    let (notes, len) = c.notes(pad);
                    assert_eq!(len, usize::from(color) + 3);
                    assert!(notes[..len].windows(2).all(|n| n[0] < n[1]));
                    assert!(notes[..len].iter().all(|n| *n < 128));
                }
            }
        }
    }
}
#[test]
fn musical_ticks_convert_without_bar_drift() {
    assert_eq!(event_tick(3840), 4 * PPQ);
    assert_eq!(event_tick(960), PPQ);
}
#[test]
fn hat_regeneration_changes_musical_grid_not_just_velocity() {
    let mut cfg = PatternConfig::default();
    let mut rhythms = std::collections::HashSet::new();
    for _ in 0..50 {
        cfg = next_config(cfg, 2);
        rhythms.insert(cfg.rhythm);
    }
    assert!(rhythms.len() >= 6);
}
#[test]
fn generated_eighth_hats_keep_strong_weak_medium_weak_velocity() {
    for seed in 1..100 {
        let p = generate(
            PatternConfig {
                rules: [2, 2, 2, 1],
                rhythm: 3,
                swing: 50.,
                nudge_ms: 0.,
                snare_late_ms: 0.,
                seed,
            },
            88.,
        );
        let hats: Vec<_> = p.hits[..p.len].iter().filter(|h| h.sound == 2).collect();
        assert!(hats.len() >= 4);
        let v: Vec<_> = (0..4)
            .map(|i| hats.iter().find(|h| h.tick == i * 480).unwrap().velocity)
            .collect();
        assert!(v[0] > v[2] && v[2] > v[1] && v[2] > v[3]);
        assert!(v[0] < 120);
    }
}
#[test]
fn generated_ghosts_are_softer_than_backbeat_and_triplets_have_accents() {
    for rhythm in [15, 18, 21] {
        let p = generate(
            PatternConfig {
                rules: [2, 7, 2, 1],
                rhythm,
                swing: 50.,
                nudge_ms: 0.,
                snare_late_ms: 0.,
                seed: 42,
            },
            88.,
        );
        let notes = &p.hits[..p.len];
        let back = notes
            .iter()
            .filter(|h| h.sound == 1 && [960, 2880].contains(&h.tick))
            .map(|h| h.velocity)
            .min()
            .unwrap();
        assert!(
            notes
                .iter()
                .filter(|h| h.sound == 1 && ![960, 2880].contains(&h.tick))
                .all(|h| h.velocity + 25 < back)
        );
        let hats: Vec<_> = notes
            .iter()
            .filter(|h| h.sound == 2)
            .map(|h| h.velocity)
            .collect();
        assert!(hats.iter().max().unwrap() - hats.iter().min().unwrap() > 25);
        assert!(notes.iter().all(|h| h.velocity < 120));
    }
}

#[test]
fn smooth_voicings_preserve_pitch_classes_and_reduce_large_jumps() {
    let config = ChordConfig {
        color: 2,
        ..Default::default()
    };
    let (first, n) = config.notes(0);
    let (distant, len) = config.notes(14);
    let moved = loop_club::music::smooth_voicing(distant, len, first, n);
    assert_eq!(len, n);
    let cost = |notes: [u8; 5]| {
        (0..n)
            .map(|i| (i32::from(notes[i]) - i32::from(first[i])).abs())
            .sum::<i32>()
    };
    assert!(cost(moved) < cost(distant));
    let mut a = distant[..len].iter().map(|n| n % 12).collect::<Vec<_>>();
    let mut b = moved[..len].iter().map(|n| n % 12).collect::<Vec<_>>();
    a.sort();
    b.sort();
    assert_eq!(a, b);
    assert!(moved[..len].windows(2).all(|w| w[0] <= w[1]));
    assert_eq!(
        loop_club::music::smooth_voicing(distant, len, [0; 5], 0),
        distant
    );
}

#[test]
fn sync_divisions_follow_bpm_and_old_synth_json_stays_free() {
    let mut synth: Synth = serde_json::from_str("{}").unwrap();
    assert!(!synth.lfo_sync);
    synth.lfo_sync = true;
    synth.lfo_division = 7;
    assert!((synth.effective_lfo_rate(120.) - 4.).abs() < 1e-5);
    synth.lfo_division = 8;
    assert!((synth.effective_lfo_rate(120.) - 8. / 3.).abs() < 1e-5);
    synth.lfo_division = 9;
    assert!((synth.effective_lfo_rate(120.) - 6.).abs() < 1e-5);
    assert!((synth.effective_lfo_rate(60.) - 3.).abs() < 1e-5);
    assert_eq!(
        serde_json::from_str::<Synth>(&serde_json::to_string(&synth).unwrap())
            .unwrap()
            .lfo_division,
        9
    );
}

#[test]
fn optional_top_note_mode_prioritizes_melody_motion_over_total_voice_motion() {
    let chord = [60, 64, 67, 0, 0];
    let previous = [43, 47, 72, 0, 0];
    let top = loop_club::music::top_note_voicing(chord, 3, previous, 3);
    let classic = loop_club::music::smooth_voicing(chord, 3, previous, 3);
    assert_eq!(top[2], 72);
    assert_ne!(classic[2], 72);
    let mut classes = top[..3].iter().map(|n| n % 12).collect::<Vec<_>>();
    classes.sort();
    assert_eq!(classes, vec![0, 4, 7]);
    assert!(top[..3].windows(2).all(|p| p[0] <= p[1]));
    assert_eq!(
        loop_club::music::top_note_voicing(chord, 3, [0; 5], 0),
        chord
    );
    let mut cfg = ChordConfig::default();
    assert_eq!(cfg.voice_lead_mode(), 1);
    cfg.top_note = true;
    assert_eq!(cfg.voice_lead_mode(), 2);
    let json = serde_json::to_string(&cfg).unwrap();
    let recalled: ChordConfig = serde_json::from_str(&json).unwrap();
    assert!(recalled.top_note);
    let old: ChordConfig = serde_json::from_str("{\"smooth\":true}").unwrap();
    assert!(!old.top_note);
    assert_eq!(old.voice_lead_mode(), 1);
}
