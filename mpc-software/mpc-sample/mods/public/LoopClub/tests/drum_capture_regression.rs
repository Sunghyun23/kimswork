use loop_club::{
    browser_device::FileWorker,
    engine::{Command, Engine},
    model::*,
    runtime::Runtime,
    ui::{Action, DRUM_PREVIEW, Snapshot, Ui},
    view::{self, Painter},
};
fn core() -> Engine {
    let mut e = Engine::new(8000, 240., 40000);
    e.preroll = 0;
    e.tracks[0].kind = Kind::Drum;
    e
}
fn ev(note: u8, tick: u64, on: bool) -> Event {
    Event {
        deck: None,
        timing: None,
        slice: false,
        note,
        tick,
        on,
        velocity: if on { 100 } else { 0 },
    }
}
fn until(e: &mut Engine, frame: u64) {
    while e.clock.frame < frame {
        e.process(Frame::ZERO);
    }
}
fn hit(e: &mut Engine, note: u8) {
    for on in [true, false] {
        e.command(Command::Note {
            track: 0,
            note,
            velocity: if on { 100 } else { 0 },
            on,
            source: 100,
        })
        .unwrap();
    }
}
#[test]
fn replace_does_not_play_any_previous_lane_but_overdub_does() {
    for mode in [RecordMode::Replace, RecordMode::Overdub] {
        let mut e = core();
        let t = &mut e.tracks[0];
        t.length_ticks = 4 * PPQ;
        t.record_mode = mode;
        t.playing = true;
        t.events.extend([
            ev(0, 0, true),
            ev(0, PPQ / 8, false),
            ev(2, PPQ / 2, true),
            ev(2, PPQ, false),
        ]);
        e.command(Command::LoopRecord(0)).unwrap();
        let mut peak = 0f32;
        for _ in 0..2000 {
            let x = e.process(Frame::ZERO);
            peak = peak.max(x.l.abs()).max(x.r.abs());
        }
        if mode == RecordMode::Replace {
            assert_eq!(peak, 0.);
            assert!(e.visible_events(0).is_empty());
            assert_eq!(e.drum_activity(0), 0);
        } else {
            assert!(peak > 0.001);
            assert_eq!(e.visible_events(0).len(), 4);
        }
    }
}
#[test]
fn replace_cancels_old_long_one_shots_without_cutting_live_pad_voices() {
    let mut e = core();
    e.tracks[0].length_ticks = 4 * PPQ;
    e.tracks[0].playing = true;
    e.tracks[0].events.push(ev(13, 0, true));
    e.command(Command::PlayAll).unwrap();
    until(&mut e, 100);
    assert!(e.active_voices() > 0);
    e.command(Command::LoopRecord(0)).unwrap();
    let start = e.recorder.unwrap().start;
    let after_start = e.clock.frame_at(start) + 1000;
    until(&mut e, after_start);
    assert_eq!(e.active_voices(), 0);
    hit(&mut e, 0);
    assert!(e.active_voices() > 0);
    assert!(e.visible_events(0).iter().any(|n| n.note == 0 && n.on));
}
#[test]
fn later_recording_bars_and_hits_after_first64_events_are_visible() {
    let mut e = core();
    e.command(Command::LoopRecord(0)).unwrap();
    e.process(Frame::ZERO);
    for n in 0..40 {
        until(&mut e, 20 + n * 30);
        hit(&mut e, 0);
    }
    until(&mut e, 16020);
    hit(&mut e, 3);
    let s = Snapshot::capture(&e);
    assert!(s.words[22] > 64);
    assert_eq!(s.words[DRUM_PREVIEW], 8 * PPQ);
    assert_ne!(s.words[DRUM_PREVIEW + 2 + 3], 0);
    assert!((0..64).all(|i| s.words[353 + 2 * i] & 255 != 3));
    struct Marks(usize);
    impl Painter for Marks {
        fn rect(&mut self, _: i32, _: i32, _: i32, _: i32, c: u32) {
            if c == 0xffdd775d {
                self.0 += 1;
            }
        }
        fn text(&mut self, _: i32, _: i32, _: i32, _: i32, _: f32, _: u32, _: &str) {}
    }
    for page in [Page::Loop, Page::Grid, Page::Play, Page::DrumSound] {
        let mut u = Ui::default();
        u.navigate(page, &s);
        let mut p = Marks(0);
        view::render(&u, &s, &mut p);
        assert!(p.0 > 0, "recent-hit marker missing on {page:?}");
    }
}
#[test]
fn brief_drum_activity_survives_note_off_and_expires_while_transport_is_stopped() {
    let mut e = core();
    hit(&mut e, 2);
    assert_ne!(e.drum_activity(0) & 4, 0);
    for _ in 0..1400 {
        e.process(Frame::ZERO);
    }
    assert_eq!(e.drum_activity(0) & 4, 0);
    assert!(!e.running);
}
#[test]
fn explicit_generation_is_refused_during_recording_and_shift_record_never_generates() {
    for page in [Page::Loop, Page::Play, Page::DrumSound] {
        let e = core();
        let mut u = Ui::default();
        u.navigate(page, &Snapshot::capture(&e));
        let mut c = Vec::new();
        u.pad(0, true, 100, true, false, &Snapshot::capture(&e), |x| {
            c.push(x)
        });
        assert!(matches!(c.as_slice(), [Command::LoopRecord(0)]));
    }
    let mut e = core();
    e.command(Command::LoopRecord(0)).unwrap();
    let s = Snapshot::capture(&e);
    let mut u = Ui::default();
    u.navigate(Page::Play, &s);
    let mut c = Vec::new();
    u.soft(1, false, &s, |x| c.push(x));
    assert!(c.is_empty());
    assert_eq!(u.notice, "FINISH REC BEFORE GENERATE");
}
#[test]
fn original_default_scratch_loads_on_worker_and_does_not_overwrite_existing_audio() {
    let r = Runtime::default();
    assert!(
        r.initialize(Box::new(Engine::new(8000, 100., 32000)))
            .is_ok()
    );
    r.active.store(true, std::sync::atomic::Ordering::Release);
    let mut w = FileWorker::with_root(None, 8000, 32000);
    w.load_builtin(&r, 0).unwrap();
    assert!(w.pending());
    let mut out = [0; 128];
    r.output(&mut out, 2, 8000);
    assert_eq!(w.tick(&r), Some(true));
    assert_eq!(w.take_builtin_loaded(), Some(0));
    r.output(&mut out, 2, 8000);
    r.with_engine(|slot| {
        let e = slot.as_ref().unwrap();
        let t = &e.tracks[0];
        assert_eq!(t.name, "Classic Ah Yeah");
        assert_eq!(t.audio_target, loop_club::audio::RecordTarget::Turntable);
        assert_eq!(t.source_frames, 7600);
        let peak = t.audio[..t.source_frames]
            .iter()
            .map(|x| x.l.abs())
            .fold(0f32, f32::max);
        assert!(peak > 0.05 && peak < 0.99);
        assert!(t.audio.iter().all(|x| x.l.is_finite() && x.r.is_finite()));
    })
    .unwrap();
    assert!(w.load_builtin(&r, 0).is_err());
    let e = Engine::new(8000, 100., 32000);
    let mut u = Ui::default();
    u.navigate(Page::Source, &Snapshot::capture(&e));
    assert_eq!(
        u.pad(7, true, 100, false, false, &Snapshot::capture(&e), |_| {}),
        Action::DefaultScratch
    );
}

#[test]
fn stopped_grid_later_bar_selection_edits_the_true_event_beyond_first64() {
    let mut e = core();
    e.tracks[0].length_ticks = 12 * PPQ;
    for i in 0..40 {
        e.tracks[0].events.push(ev(0, i * PPQ / 32, true));
        e.tracks[0].events.push(ev(0, i * PPQ / 32 + 1, false));
    }
    e.tracks[0].events.extend([
        ev(3, 8 * PPQ + PPQ / 4, true),
        ev(3, 8 * PPQ + PPQ / 2, false),
    ]);
    e.command(Command::DrumPreviewBar(Some((0, 2)))).unwrap();
    let s = Snapshot::capture(&e);
    assert_eq!(s.words[DRUM_PREVIEW], 8 * PPQ);
    assert_eq!(s.grid_event_count(), 2);
    assert_eq!(s.grid_event_index(0), 80);
    assert_eq!(s.words[353] & 255, 3);
    assert_ne!(s.words[DRUM_PREVIEW + 5], 0);
    let mut u = Ui::default();
    u.navigate(Page::Grid, &s);
    u.grid_bar = 2;
    u.grid_event = 0;
    u.knob(2, 0.4, &s, |c| {
        e.command(c).unwrap();
    });
    assert_eq!(e.tracks[0].events[0].velocity, 100);
    assert_eq!(e.tracks[0].events[80].velocity, 51);
    e.command(Command::DrumPreviewBar(None)).unwrap();
    let s = Snapshot::capture(&e);
    assert_eq!(s.grid_event_index(0), 0);
}

#[test]
fn default_scratch_is_finite_and_bounded_at_every_supported_rate() {
    for rate in [8000, 44100, 96000] {
        let job = loop_club::scratch_source::prepare(0, rate, 100., rate as usize * 4).unwrap();
        assert_eq!(job.source_frames, (rate as f64 * 0.95).round() as usize);
        assert_eq!(job.audio[0], Frame::ZERO);
        assert_eq!(job.audio[job.source_frames - 1], Frame::ZERO);
        assert!(
            job.audio
                .iter()
                .all(|x| x.l.is_finite() && x.r.is_finite() && x.l.abs().max(x.r.abs()) <= 0.601)
        );
        let power = job.audio[..job.source_frames]
            .iter()
            .map(|x| x.l * x.l)
            .sum::<f32>()
            / job.source_frames as f32;
        assert!(power > 0.0001);
    }
    assert!(loop_club::scratch_source::prepare(0, 44100, 100., 100).is_err());
}
