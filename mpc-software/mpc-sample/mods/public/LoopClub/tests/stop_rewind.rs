use loop_club::{
    engine::{Command, Engine},
    model::*,
    ui::Snapshot,
};
fn run(e: &mut Engine, n: usize) {
    for _ in 0..n {
        e.process(Frame::ZERO);
    }
}
fn clip(e: &mut Engine, i: usize, kind: Kind) {
    e.tracks[i].kind = kind;
    e.tracks[i].length_ticks = 4 * PPQ;
    if kind == Kind::Audio {
        e.tracks[i].audio_frames = 800;
        e.tracks[i].source_frames = 800;
        e.tracks[i].audio[..800].fill(Frame::mono(0.1));
    } else {
        e.tracks[i].events.push(Event {
            tick: 0,
            note: 60,
            velocity: 90,
            on: true,
            slice: false,
            deck: None,
            timing: None,
        });
        e.tracks[i].events.push(Event {
            tick: PPQ,
            note: 60,
            velocity: 0,
            on: false,
            slice: false,
            deck: None,
            timing: None,
        });
    }
}
#[test]
fn playback_stop_rewinds_all_sources_and_play_restarts_at_zero() {
    let mut e = Engine::new(8000, 100., 8000);
    for (i, kind) in [Kind::Synth, Kind::Drum, Kind::Audio]
        .into_iter()
        .enumerate()
    {
        clip(&mut e, i, kind);
        e.tracks[i].origin = i as u64 * PPQ;
    }
    e.command(Command::PlayAll).unwrap();
    run(&mut e, 19000);
    let stopped = e.clock.frame;
    e.command(Command::Stop).unwrap();
    assert!(stopped > 0);
    assert_eq!(e.clock.frame, 0);
    assert!(!e.running);
    run(&mut e, 100);
    assert_eq!(e.clock.frame, 0);
    e.command(Command::Stop).unwrap();
    assert_eq!(e.clock.frame, 0);
    for t in &e.tracks {
        assert_eq!(t.origin, 0);
        assert_eq!(t.cursor, 0);
        assert_eq!(t.phase(e.clock.tick()), 0);
        assert!(!t.playing);
    }
    let s = Snapshot::capture(&e);
    assert_eq!(s.words[8], 0);
    e.command(Command::PlayAll).unwrap();
    run(&mut e, 1);
    assert_eq!(e.clock.frame, 1);
    assert!(e.active_voices() > 0);
}
#[test]
fn rewind_does_not_change_audio_midi_sound_or_recorded_length() {
    let mut e = Engine::new(8000, 100., 8000);
    clip(&mut e, 0, Kind::Synth);
    clip(&mut e, 1, Kind::Audio);
    e.tracks[0].synth.cutoff = 1234.;
    let events = e.tracks[0].events.clone();
    let audio = e.tracks[1].audio.clone();
    e.command(Command::PlayAll).unwrap();
    run(&mut e, 2300);
    e.command(Command::Stop).unwrap();
    e.command(Command::Stop).unwrap();
    assert_eq!(e.tracks[0].events, events);
    assert_eq!(e.tracks[1].audio, audio);
    assert_eq!(e.tracks[0].synth.cutoff, 1234.);
    assert_eq!(e.tracks[0].length_ticks, 4 * PPQ);
    assert_eq!(e.tracks[1].source_frames, 800);
    e.command(Command::Stop).unwrap();
    assert_eq!(e.clock.frame, 0);
}
#[test]
fn stop_during_empty_replace_keeps_previous_take_then_rewinds() {
    let mut e = Engine::new(8000, 100., 32000);
    clip(&mut e, 0, Kind::Synth);
    let old = e.tracks[0].events.clone();
    e.command(Command::PlayAll).unwrap();
    run(&mut e, 11000);
    e.preroll = 0;
    e.command(Command::Record(0)).unwrap();
    run(&mut e, 10000);
    assert!(e.recorder.is_some());
    let position = e.clock.frame;
    e.command(Command::Stop).unwrap();
    assert_eq!(e.clock.frame, position);
    assert!(e.recorder.is_none());
    assert_eq!(e.tracks[0].events, old);
    e.command(Command::Stop).unwrap();
    assert_eq!(e.clock.frame, 0);
    assert_eq!(e.tracks[0].events, old);
}

fn fresh(kind: Kind) -> Engine {
    let mut e = Engine::new(8000, 240., 24000);
    e.preroll = 0;
    e.tracks[0].kind = kind;
    e
}
fn input_until(e: &mut Engine, frame: u64, value: f32) {
    while e.clock.frame < frame {
        e.process(Frame::mono(value));
    }
}
fn note(e: &mut Engine, n: u8, on: bool) {
    e.command(Command::Note {
        track: 0,
        note: n,
        velocity: if on { 90 } else { 0 },
        on,
        source: 2,
    })
    .unwrap();
}
fn finish_stopped(e: &mut Engine) {
    for _ in 0..1200 {
        if e.recorder.is_none() {
            return;
        }
        e.process(Frame::ZERO);
    }
    assert!(e.recorder.is_none());
}
#[test]
fn resume_mid_bar_does_not_fire_all_past_notes_in_one_sample() {
    let mut e = fresh(Kind::Synth);
    e.tracks[0].length_ticks = 4 * PPQ;
    for j in 0..3 {
        e.tracks[0].events.push(Event {
            tick: (j + 1) * PPQ / 4,
            note: 60 + j as u8,
            velocity: 90,
            on: true,
            slice: false,
            deck: None,
            timing: None,
        });
        e.tracks[0].events.push(Event {
            tick: (j + 1) * PPQ / 4 + PPQ / 8,
            note: 60 + j as u8,
            velocity: 0,
            on: false,
            slice: false,
            deck: None,
            timing: None,
        });
    }
    let original = e.tracks[0].events.clone();
    e.command(Command::PlayAll).unwrap();
    run(&mut e, 5000);
    e.running = false;
    e.tracks[0].playing = false;
    run(&mut e, 12000);
    assert_eq!(e.active_voices(), 0);
    let frame = e.clock.frame;
    e.command(Command::PlayAll).unwrap();
    run(&mut e, 1);
    assert_eq!(
        e.active_voices(),
        0,
        "past note-ons must not bunch at resume"
    );
    assert_eq!(e.clock.frame, frame + 1);
    assert_eq!(e.tracks[0].events, original);
}
#[test]
fn stop_keeps_last_musical_bar_and_ignores_empty_next_bar_for_midi() {
    for kind in [Kind::Synth, Kind::Drum] {
        let mut e = fresh(kind);
        e.command(Command::LoopRecord(0)).unwrap();
        input_until(&mut e, 500, 0.);
        note(&mut e, 60, true);
        input_until(&mut e, 700, 0.);
        note(&mut e, 60, false);
        input_until(&mut e, 7500, 0.);
        note(&mut e, 62, true);
        input_until(&mut e, 7700, 0.);
        note(&mut e, 62, false);
        input_until(&mut e, 9000, 0.);
        e.command(Command::Stop).unwrap();
        assert!(e.recorder.is_none());
        assert_eq!(e.tracks[0].length_ticks, 4 * PPQ);
        let ticks = e.tracks[0]
            .events
            .iter()
            .filter(|n| n.on)
            .map(|n| n.tick)
            .collect::<Vec<_>>();
        assert_eq!(ticks, vec![PPQ / 4, 15 * PPQ / 4]);
        e.command(Command::Undo).unwrap();
        assert!(e.tracks[0].empty());
        e.command(Command::Redo).unwrap();
        assert_eq!(e.tracks[0].length_ticks, 4 * PPQ);
    }
}
#[test]
fn held_synth_crossing_bar_counts_but_held_drum_release_does_not() {
    for kind in [Kind::Synth, Kind::Drum] {
        let mut e = fresh(kind);
        e.command(Command::LoopRecord(0)).unwrap();
        input_until(&mut e, 7500, 0.);
        note(&mut e, 60, true);
        input_until(&mut e, 8500, 0.);
        e.command(Command::Stop).unwrap();
        assert_eq!(
            e.tracks[0].length_ticks,
            if kind == Kind::Drum { 4 * PPQ } else { 8 * PPQ }
        );
    }
}
#[test]
fn audio_stop_trims_silent_bar_and_below_threshold_input_without_changing_pcm_timing() {
    let mut e = fresh(Kind::Audio);
    e.command(Command::LoopRecord(0)).unwrap();
    input_until(&mut e, 1000, 0.);
    input_until(&mut e, 2000, 0.2);
    input_until(&mut e, 9500, 0.00001);
    let frame = e.clock.frame;
    e.command(Command::Stop).unwrap();
    finish_stopped(&mut e);
    assert!(e.clock.frame >= frame && e.running && e.tracks[0].playing);
    assert_eq!(e.tracks[0].length_ticks, 4 * PPQ);
    assert_eq!(e.tracks[0].audio_frames, 8000);
    assert_eq!(e.tracks[0].audio[1500], Frame::mono(0.2));
    assert_eq!(e.tracks[0].audio[500], Frame::ZERO);
}
#[test]
fn stopped_audio_padding_is_zero_and_second_stop_waits_for_finish_before_rewind() {
    let mut e = fresh(Kind::Audio);
    e.command(Command::LoopRecord(0)).unwrap();
    input_until(&mut e, 2000, 0.2);
    e.command(Command::Stop).unwrap();
    assert!(e.recorder.is_some());
    e.command(Command::Stop).unwrap();
    finish_stopped(&mut e);
    run(&mut e, 1);
    assert_eq!(e.clock.frame, 0);
    assert!(
        e.tracks[0].audio[2000..8000]
            .iter()
            .all(|x| *x == Frame::ZERO)
    );
    assert_eq!(e.tracks[0].length_ticks, 4 * PPQ);
}
#[test]
fn silence_and_unmatched_note_offs_do_not_create_a_blank_loop() {
    for kind in [Kind::Synth, Kind::Drum, Kind::Audio] {
        let mut e = fresh(kind);
        e.command(Command::LoopRecord(0)).unwrap();
        input_until(&mut e, 9000, 0.00001);
        if kind != Kind::Audio {
            note(&mut e, 60, false);
        }
        e.command(Command::Stop).unwrap();
        finish_stopped(&mut e);
        assert!(e.tracks[0].empty());
    }
}
#[test]
fn stopped_midbar_record_has_four_beats_of_countin_with_no_empty_wait() {
    for preroll in [0, 1, 2] {
        let mut e = fresh(Kind::Synth);
        e.clock.frame = 3000;
        e.preroll = preroll;
        e.command(Command::LoopRecord(0)).unwrap();
        let r = e.recorder.unwrap();
        assert_eq!(r.start - e.clock.tick(), u64::from(preroll) * 4 * PPQ);
        if preroll > 0 {
            assert_eq!(r.count_in_start, Some(e.clock.tick()));
        }
        assert_eq!(r.start % (4 * PPQ), 0);
    }
}

#[test]
fn stopped_partial_overdub_keeps_existing_two_bar_length_and_audio_tail() {
    let mut e = fresh(Kind::Audio);
    e.tracks[0].length_ticks = 8 * PPQ;
    e.tracks[0].audio_frames = 16000;
    e.tracks[0].source_frames = 16000;
    e.tracks[0].audio[..16000].fill(Frame::mono(0.1));
    e.tracks[0].features.feed(Frame::mono(0.1), 8000);
    e.command(Command::RecordMode(0, RecordMode::Overdub))
        .unwrap();
    e.command(Command::LoopRecord(0)).unwrap();
    input_until(&mut e, 2000, 0.2);
    e.command(Command::Stop).unwrap();
    finish_stopped(&mut e);
    assert_eq!(e.tracks[0].length_ticks, 8 * PPQ);
    assert!((e.tracks[0].audio[1000].l - 0.3).abs() < 1e-6);
    assert_eq!(e.tracks[0].audio[14000], Frame::mono(0.1));
}
#[test]
fn preroll_from_partial_stop_emits_four_click_windows_without_a_silent_lead_in() {
    let mut e = fresh(Kind::Synth);
    e.preroll = 1;
    e.clock.frame = 3000;
    e.command(Command::LoopRecord(0)).unwrap();
    let begin = e.clock.frame;
    let mut heard = [false; 4];
    for i in 0..8000 {
        if e.metronome_sample().peak() > 0.001 {
            heard[i / 2000] = true;
        }
        e.process(Frame::ZERO);
    }
    assert!(heard.into_iter().all(|v| v));
    assert_eq!(e.recorder.unwrap().start, e.clock.tick());
    assert_eq!(e.clock.frame - begin, 8000);
}

#[test]
fn running_alignment_wait_is_not_labeled_as_six_beat_preroll() {
    use loop_club::view::{self, Painter};
    struct Text(Vec<String>);
    impl Painter for Text {
        fn rect(&mut self, _: i32, _: i32, _: i32, _: i32, _: u32) {}
        fn text(&mut self, _: i32, _: i32, _: i32, _: i32, _: f32, _: u32, t: &str) {
            self.0.push(t.into());
        }
    }
    let mut e = fresh(Kind::Synth);
    e.preroll = 1;
    e.running = true;
    e.clock.frame = 3000;
    e.command(Command::LoopRecord(0)).unwrap();
    let start = e.recorder.unwrap().count_in_start.unwrap();
    let ui = loop_club::ui::Ui::default();
    let mut text = Text(Vec::new());
    view::render(&ui, &Snapshot::capture(&e), &mut text);
    assert!(text.0.iter().any(|t| t.contains("WAIT FOR BAR")));
    e.clock.frame = e.clock.frame_at(start);
    let mut text = Text(Vec::new());
    view::render(&ui, &Snapshot::capture(&e), &mut text);
    assert!(text.0.iter().any(|t| t.contains("PRE 4 BEATS")));
}

#[test]
fn recording_stop_commits_without_stopping_other_loops_then_next_stop_rewinds() {
    let mut e = fresh(Kind::Synth);
    clip(&mut e, 1, Kind::Drum);
    e.tracks[1].playing = true;
    e.running = true;
    e.command(Command::LoopRecord(0)).unwrap();
    input_until(&mut e, 1000, 0.);
    note(&mut e, 60, true);
    input_until(&mut e, 1400, 0.);
    note(&mut e, 60, false);
    input_until(&mut e, 9000, 0.);
    e.command(Command::Stop).unwrap();
    assert!(e.recorder.is_none());
    assert!(e.running);
    assert!(e.tracks[0].playing && e.tracks[1].playing);
    let frame = e.clock.frame;
    run(&mut e, 10);
    assert_eq!(e.clock.frame, frame + 10);
    e.command(Command::Stop).unwrap();
    assert!(!e.running);
    assert_eq!(e.clock.frame, 0);
    assert!(e.tracks.iter().all(|t| !t.playing));
    assert_eq!(e.tracks[0].length_ticks, 4 * PPQ);
}
