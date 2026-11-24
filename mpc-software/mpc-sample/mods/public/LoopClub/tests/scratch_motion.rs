use loop_club::{
    audio::RecordTarget,
    deck::{Control, Motion, Player, Settings},
    engine::{Command, Engine},
    model::*,
    project,
    ui::{Snapshot, Ui},
};
fn pcm(rate: u32) -> Vec<Frame> {
    (0..rate * 3)
        .map(|i| Frame::mono((i as f32 * 0.1).sin() * 0.2))
        .collect()
}
fn core() -> Engine {
    let mut e = Engine::new(8000, 240., 16000);
    e.preroll = 0;
    let t = &mut e.tracks[0];
    t.audio_target = RecordTarget::Turntable;
    t.deck.slip = false;
    t.source_frames = 8000;
    t.audio_frames = 8000;
    t.source_ticks = 4 * PPQ;
    t.length_ticks = 4 * PPQ;
    t.chops.divide(8000);
    t.audio[..8000].copy_from_slice(&pcm(8000)[..8000]);
    e
}
fn frames(e: &mut Engine, n: usize) {
    for _ in 0..n {
        assert!(e.process(Frame::ZERO).finite());
    }
}
#[test]
fn steady_motion_has_no_repeated_stops_or_four_x_bursts_after_initial_observation() {
    for rate in [8000, 44100, 48000] {
        for hz in [10, 20, 50, 100, 200] {
            let wave = pcm(rate);
            let mut d = Player::default();
            let cfg = Settings {
                slip: false,
                ..Settings::default()
            };
            d.grab(true, Some(0.), cfg, rate);
            let interval = rate / hz;
            let mut speeds = Vec::new();
            for i in 0..rate * 3 / 10 {
                if i % interval == 0 {
                    d.move_to_timed(
                        0.45 * (i + interval).min(rate * 3 / 10) as f32 / (rate * 3 / 10) as f32,
                        cfg,
                        100.,
                        rate,
                        Some(1_000_000 / hz),
                    );
                }
                d.sample(&wave, cfg, rate);
                if i >= interval && i + interval <= rate * 3 / 10 {
                    speeds.push(d.rate);
                }
                assert!(d.rate.abs() <= 4.001);
            }
            let mean = speeds.iter().sum::<f32>() / speeds.len() as f32;
            let deviation = (speeds.iter().map(|v| (v - 0.9).powi(2)).sum::<f32>()
                / speeds.len() as f32)
                .sqrt();
            assert!((mean - 0.9).abs() < 0.015, "{rate}/{hz}: {mean}");
            assert!(deviation < 0.03, "{rate}/{hz}: {deviation}");
            assert!(speeds.iter().all(|v| *v > 0.8), "{rate}/{hz} stalled");
        }
    }
}
#[test]
fn seven_bit_position_quantization_stays_bounded_and_continuous() {
    let rate = 44100;
    let wave = pcm(rate);
    let cfg = Settings::default();
    for hz in [50, 100, 200] {
        let mut d = Player::default();
        d.grab(true, Some(0.), cfg, rate);
        let interval = rate / hz;
        let mut rates = Vec::new();
        for i in 0..rate / 2 {
            if i % interval == 0 {
                let value = (0.7 * (i + interval) as f32 / (rate / 2) as f32 * 127.).round() / 127.;
                d.move_to_timed(value, cfg, 100., rate, Some(1_000_000 / hz));
            }
            d.sample(&wave, cfg, rate);
            if i > interval * 2 {
                rates.push(d.rate);
            }
        }
        assert!(rates.iter().all(|v| v.is_finite() && v.abs() < 2.));
        assert!((rates.iter().sum::<f32>() / rates.len() as f32 - 0.84).abs() < 0.04);
    }
}
#[test]
fn reversal_settle_duplicate_and_full_range_jump_do_not_overshoot_or_run_away() {
    let rate = 44100;
    let wave = pcm(rate);
    let cfg = Settings {
        range: 2,
        ..Settings::default()
    };
    let mut d = Player::default();
    d.grab(true, Some(0.5), cfg, rate);
    d.move_to_timed(0.8, cfg, 100., rate, Some(100_000));
    for _ in 0..2205 {
        d.sample(&wave, cfg, rate);
    }
    let prior = d.state.head;
    d.move_to_timed(0.2, cfg, 100., rate, Some(100_000));
    let target = d.state.target;
    let mut last = prior;
    for _ in 0..10000 {
        d.sample(&wave, cfg, rate);
        assert!(d.state.head <= last + 1e-8 && d.state.head >= target - 1e-8);
        assert!(d.rate.abs() <= 4.001);
        last = d.state.head;
    }
    assert!((d.state.head - target).abs() < 1e-8);
    assert_eq!(d.rate, 0.);
    assert!(!d.move_to_timed(0.2, cfg, 100., rate, Some(100_000)));
    d.move_to_timed(1., cfg, 40., rate, Some(1));
    for _ in 0..44100 {
        d.sample(&wave, cfg, rate);
        assert!(d.rate.abs() <= 4.001);
    }
    assert!((d.state.head - d.state.target).abs() < 1e-8);
}
#[test]
fn same_audio_block_coalescing_preserves_the_total_input_interval() {
    let mut d = Player::default();
    let cfg = Settings::default();
    d.grab(true, Some(0.), cfg, 44100);
    d.move_to_timed(0.1, cfg, 100., 44100, Some(100_000));
    d.move_to_timed(0.2, cfg, 100., 44100, Some(100_000));
    let m = d.state.motion.unwrap();
    assert_eq!(m.duration, 8820);
    d.advance(8820, cfg, 44100);
    assert!((d.state.head - 5292.).abs() < 0.001);
}
#[test]
fn legacy_target_chase_remains_available_for_old_saved_controls() {
    let mut d = Player::default();
    let cfg = Settings::default();
    let c = Control {
        head: 0.,
        target: 1000.,
        grabbed: true,
        ..Control::default()
    };
    d.restore(c, 0, cfg, 44100);
    d.advance(1, cfg, 44100);
    assert_eq!(d.state.head, 4.);
    assert!(d.state.motion.is_none());
}
#[test]
fn motion_validation_rejects_malformed_and_unbounded_segments() {
    let good = Motion {
        start: 0.,
        end: 10.,
        duration: 100,
        elapsed: 0,
        slope: 1.,
    };
    let c = Control {
        motion: Some(good),
        ..Control::default()
    };
    assert!(c.valid());
    for m in [
        Motion {
            duration: 0,
            ..good
        },
        Motion {
            elapsed: 101,
            ..good
        },
        Motion {
            slope: f64::NAN,
            ..good
        },
        Motion {
            end: f64::INFINITY,
            ..good
        },
        Motion {
            end: 10000.,
            ..good
        },
    ] {
        assert!(
            !Control {
                motion: Some(m),
                ..Control::default()
            }
            .valid()
        );
    }
}
#[test]
fn cut_reaches_silence_in_three_ms_without_stopping_the_head_or_changing_level() {
    for rate in [8000, 44100, 48000] {
        let cfg = Settings::default();
        let wave = pcm(rate);
        let mut d = Player::default();
        d.start(0.);
        for _ in 0..rate / 20 {
            d.sample(&wave, cfg, rate);
        }
        let head = d.state.head;
        d.manual_cut = true;
        for _ in 0..(rate * 31 / 10000 + 2) {
            d.sample(&wave, cfg, rate);
        }
        for _ in 0..100 {
            assert!(d.sample(&wave, cfg, rate).peak() < 1e-7);
        }
        assert!(d.state.head > head);
        d.manual_cut = false;
        let mut peak = 0f32;
        for _ in 0..rate / 100 {
            peak = peak.max(d.sample(&wave, cfg, rate).peak());
        }
        assert!(peak > 0.05);
    }
    let mut e = core();
    let mix = e.tracks[0].mix;
    e.command(Command::DeckCut(0, true)).unwrap();
    assert!(!e.is_idle());
    e.command(Command::DeckGrab(0, true, Some(0.))).unwrap();
    e.command(Command::DeckFaderTimed(0, 0.5, 100_000)).unwrap();
    frames(&mut e, 500);
    assert_eq!(e.tracks[0].mix.level, mix.level);
    e.command(Command::DeckGrab(0, false, None)).unwrap();
    e.command(Command::DeckCut(0, false)).unwrap();
    assert!(!e.decks[0].state.muted);
    e.command(Command::Stop).unwrap();
    assert!(e.is_idle());
}
#[test]
fn two_hand_latch_cut_and_release_survive_navigation_without_affecting_loop_record_mapping() {
    let mut e = core();
    let mut ui = Ui::default();
    ui.page = Page::Play;
    let s = Snapshot::capture(&e);
    let mut cmds = Vec::new();
    ui.pad(0, true, 84, false, false, &s, |c| cmds.push(c));
    assert_eq!(ui.deck_latched, Some(0));
    assert_eq!(cmds.len(), 1);
    for c in cmds {
        e.command(c).unwrap();
    }
    let mut cmds = Vec::new();
    ui.pad(0, false, 0, false, false, &s, |c| cmds.push(c));
    assert!(cmds.is_empty());
    assert!(e.decks[0].live);
    ui.pad(1, true, 84, false, false, &s, |c| {
        e.command(c).unwrap();
    });
    assert!(e.deck_cuts[0]);
    ui.navigate(Page::Grid, &s);
    ui.pad(1, false, 0, false, false, &s, |c| {
        e.command(c).unwrap();
    });
    assert!(!e.deck_cuts[0]);
    ui.navigate(Page::Play, &s);
    ui.pad(0, true, 84, false, false, &s, |c| {
        e.command(c).unwrap();
    });
    assert!(ui.deck_latched.is_none());
    assert!(!e.decks[0].live);
    ui.navigate(Page::Loop, &s);
    let mut cmds = Vec::new();
    ui.pad(0, true, 84, true, false, &s, |c| cmds.push(c));
    assert!(matches!(cmds.as_slice(), [Command::LoopRecord(0)]));
    assert!(ui.deck_latched.is_none());
}
#[test]
fn motion_and_cut_recording_recall_and_undo_preserve_pcm_and_legacy_defaults() {
    let mut e = core();
    let pcm = e.tracks[0].audio[..8000].to_vec();
    e.command(Command::LoopRecord(0)).unwrap();
    frames(&mut e, 100);
    e.command(Command::DeckGrab(0, true, Some(0.))).unwrap();
    e.command(Command::DeckFaderTimed(0, 0.8, 100_000)).unwrap();
    frames(&mut e, 300);
    e.command(Command::DeckCut(0, true)).unwrap();
    frames(&mut e, 300);
    e.command(Command::DeckCut(0, false)).unwrap();
    e.command(Command::DeckGrab(0, false, None)).unwrap();
    e.command(Command::LoopRecord(0)).unwrap();
    frames(&mut e, 8000);
    e.command(Command::Stop).unwrap();
    assert!(
        e.tracks[0]
            .events
            .iter()
            .any(|e| e.deck.is_some_and(|d| d.muted))
    );
    assert!(
        e.tracks[0]
            .events
            .iter()
            .any(|e| e.deck.is_some_and(|d| d.motion.is_some()))
    );
    assert_eq!(e.tracks[0].audio[..8000], pcm);
    let file = std::env::temp_dir().join(format!("lc-motion-cut-{}.lcproj", std::process::id()));
    project::save(&e, &file).unwrap();
    let loaded = project::load(&file, 8000, 16000).unwrap();
    assert_eq!(loaded.tracks[0].events, e.tracks[0].events);
    assert!(!loaded.deck_cuts[0]);
    std::fs::remove_file(file).unwrap();
    e.command(Command::Undo).unwrap();
    assert!(e.tracks[0].events.is_empty());
    e.command(Command::Redo).unwrap();
    assert!(
        e.tracks[0]
            .events
            .iter()
            .any(|e| e.deck.is_some_and(|d| d.muted))
    );
    let old:Control=serde_json::from_str(r#"{"head":0.0,"slip":0.0,"target":1.0,"anchor":0.0,"fader":0.0,"anchor_fader":0.0,"grabbed":true}"#).unwrap();
    assert!(old.motion.is_none() && !old.muted);
}

#[test]
fn duplicate_packets_accumulate_time_without_restarting_or_accelerating_the_segment() {
    let mut d = Player::default();
    let cfg = Settings::default();
    d.grab(true, Some(0.), cfg, 44100);
    d.move_to_timed(0.1, cfg, 100., 44100, Some(100_000));
    d.advance(4410, cfg, 44100);
    assert!(!d.move_to_timed(0.1, cfg, 100., 44100, Some(100_000)));
    d.advance(4410, cfg, 44100);
    d.move_to_timed(0.2, cfg, 100., 44100, Some(100_000));
    assert_eq!(d.state.motion.unwrap().duration, 8820);
}
