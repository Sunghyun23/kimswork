use loop_club::{
    engine::{Command, Engine},
    model::*,
    runtime::Runtime,
    tempo::TapTempo,
};
use std::sync::atomic::Ordering;
fn pulses(e: &mut Engine, until: u64) -> Vec<u64> {
    let mut out = Vec::new();
    let mut last = None;
    while e.clock.frame < until {
        let frame = e.clock.frame;
        let sound = e.metronome_sample().peak();
        if sound > 0.00001 {
            if last.is_none_or(|previous| frame - previous > u64::from(e.clock.rate) / 10) {
                out.push(frame);
            }
            last = Some(frame);
        }
        e.process(Frame::ZERO);
    }
    out
}
#[test]
fn tap_two_and_multiple_presses_average_real_intervals() {
    let mut t = TapTempo::default();
    assert_eq!(t.tap(0), None);
    assert_eq!(t.tap(600), Some(100.));
    assert_eq!(t.tap(1202), Some(100.));
    assert_eq!(t.tap(1800), Some(100.));
    assert_eq!(t.tap(2400), Some(100.));
}
#[test]
fn tap_bounce_long_pause_reset_and_supported_limits() {
    let mut t = TapTempo::default();
    assert_eq!(t.tap(100), None);
    assert_eq!(t.tap(150), None);
    assert_eq!(t.tap(600), Some(120.));
    assert_eq!(t.tap(5000), None);
    assert_eq!(t.tap(6500), Some(40.));
    t.reset();
    assert_eq!(t.tap(7000), None);
    assert_eq!(t.tap(7250), Some(240.));
    assert_eq!(t.tap(7000), None);
    assert_eq!(t.tap(7500), Some(120.));
}
#[test]
fn one_bar_count_in_has_exactly_four_full_clicks_at_any_start_phase() {
    for bpm in [40., 88., 100., 240.] {
        for phase in [0., 0.13, 0.9, 2.75] {
            let mut e = Engine::new(8000, bpm, 48000);
            e.tracks[0].kind = Kind::Synth;
            e.clock.frame = e.clock.frame_at((PPQ as f64 * phase) as u64);
            e.command(Command::LoopRecord(0)).unwrap();
            let r = e.recorder.unwrap();
            let start = e.clock.frame_at(r.start);
            let origin = e.clock.frame_at(r.count_in_start.unwrap());
            let hits = pulses(&mut e, start);
            assert_eq!(hits.len(), 4, "bpm={bpm} phase={phase}");
            assert!(hits[0] >= origin && hits[0] <= origin + 3);
            for pair in hits.windows(2) {
                assert!((pair[1] - pair[0]).abs_diff((8000. * 60. / bpm).round() as u64) <= 2);
            }
            assert_eq!(e.metronome_sample(), Frame::ZERO);
        }
    }
}
#[test]
fn preroll_off_and_two_bars_respect_configuration() {
    for (n, count) in [(0, 0), (2, 8)] {
        let mut e = Engine::new(8000, 240., 24000);
        e.tracks[0].kind = Kind::Synth;
        e.command(Command::Preroll(n)).unwrap();
        e.command(Command::LoopRecord(0)).unwrap();
        let start = e.clock.frame_at(e.recorder.unwrap().start);
        assert_eq!(pulses(&mut e, start).len(), count);
    }
}
#[test]
fn metronome_toggle_is_monitor_only_and_stops_with_transport() {
    let mut e = Engine::new(8000, 240., 16000);
    e.command(Command::ToggleMetronome).unwrap();
    assert!(e.metronome);
    assert_eq!(e.metronome_sample(), Frame::ZERO);
    e.command(Command::PlayAll).unwrap();
    assert_eq!(pulses(&mut e, 16000).len(), 8);
    e.command(Command::ToggleMetronome).unwrap();
    assert!(pulses(&mut e, 20000).is_empty());
    assert!(!e.metronome);
    e.command(Command::Stop).unwrap();
    e.command(Command::ToggleMetronome).unwrap();
    assert_eq!(e.metronome_sample(), Frame::ZERO);
}
#[test]
fn click_is_not_printed_into_audio_input_recording() {
    let r = Box::new(Runtime::default());
    let mut e = Box::new(Engine::new(8000, 240., 24000));
    e.tracks[0].kind = Kind::Audio;
    r.initialize(e).ok().unwrap();
    r.active.store(true, Ordering::Release);
    r.enqueue(Command::ToggleMetronome);
    r.enqueue(Command::LoopRecord(0));
    let mut output = [0i32; 128];
    let mut audible = false;
    for block in 0..300 {
        if block == 160 {
            r.enqueue(Command::LoopRecord(0));
        }
        for _ in 0..64 {
            r.capture(Frame::ZERO, 8000);
        }
        output.fill(0);
        r.output(&mut output, 2, 8000);
        audible |= output.iter().any(|x| x.abs() > 1000);
    }
    assert!(audible);
    r.with_engine(|slot| {
        let e = slot.as_ref().unwrap();
        assert!(e.recorder.is_none());
        assert!(e.tracks[0].audio_frames > 0);
        assert!(
            e.tracks[0].audio[..e.tracks[0].audio_frames]
                .iter()
                .all(|x| *x == Frame::ZERO)
        );
    })
    .unwrap();
}
#[test]
fn metronome_option_recalls_and_tempo_guard_preserves_recording() {
    let mut e = Engine::new(8000, 100., 24000);
    e.command(Command::ToggleMetronome).unwrap();
    let path = std::env::temp_dir().join(format!("lc-metro-{}", std::process::id()));
    let _ = std::fs::remove_file(&path);
    loop_club::project::save(&e, &path).unwrap();
    let mut loaded = loop_club::project::load(&path, 8000, 24000).unwrap();
    std::fs::remove_file(path).unwrap();
    assert!(loaded.metronome);
    loaded.tracks[0].kind = Kind::Synth;
    loaded.command(Command::LoopRecord(0)).unwrap();
    assert_eq!(loaded.command(Command::Bpm(120.)), Err(Error::TempoFrozen));
    assert_eq!(loaded.clock.bpm, 100.);
}

#[test]
fn enabled_metronome_keeps_playing_while_waiting_for_count_in_bar() {
    let mut e = Engine::new(8000, 240., 16000);
    e.tracks[0].kind = Kind::Synth;
    e.clock.frame = 1000;
    e.command(Command::ToggleMetronome).unwrap();
    e.command(Command::PlayAll).unwrap();
    e.command(Command::LoopRecord(0)).unwrap();
    assert_eq!(e.recorder.unwrap().count_in_start, Some(4 * PPQ));
    e.clock.frame = 2010;
    assert!(e.metronome_sample().peak() > 0.001);
    e.command(Command::ToggleMetronome).unwrap();
    assert_eq!(e.metronome_sample(), Frame::ZERO);
}

#[test]
fn tap_boundary_jitter_clamps_without_resetting_the_sequence() {
    let mut tap = TapTempo::default();
    assert_eq!(tap.tap(0), None);
    assert_eq!(tap.tap(248), Some(240.));
    assert_eq!(tap.tap(500), Some(240.));
    tap.reset();
    tap.tap(1000);
    assert_eq!(tap.tap(2510), Some(40.));
    assert_eq!(tap.tap(4000), Some(40.));
}
