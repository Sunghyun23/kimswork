//! Isolated QA-only sample-clock input replay, never enabled by normal builds.
//! Producer validates/allocates on HTTP worker; audio only consumes Copy points.
use crate::{
    engine::{Command, Engine},
    model::PPQ,
    queue::Queue,
};
use serde::Deserialize;
use std::sync::{
    OnceLock,
    atomic::{AtomicBool, AtomicU64, Ordering},
};
#[derive(Clone, Copy)]
struct Event {
    tick: u64,
    track: usize,
    position: Option<f32>,
    cut: Option<bool>,
    interval_us: u32,
    last: bool,
}
static EVENTS: OnceLock<Queue<Event, 4096>> = OnceLock::new();
static BUSY: AtomicBool = AtomicBool::new(false);
static READY: AtomicBool = AtomicBool::new(false);
static APPLIED: AtomicU64 = AtomicU64::new(0);
static MAX_LATE: AtomicU64 = AtomicU64::new(0);
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Plan {
    track: usize,
    start_tick: u64,
    events: Vec<Point>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Point {
    tick: u64,
    position: Option<f32>,
    cut: Option<bool>,
}
#[derive(Default)]
pub struct Playback {
    pending: Option<Event>,
    track: Option<usize>,
    cancel: bool,
}
impl Playback {
    fn step(&mut self, e: &mut Engine, q: &Queue<Event, 4096>) -> (u64, u64, bool) {
        let mut applied = 0;
        let mut late = 0;
        for _ in 0..64 {
            if self.pending.is_none() {
                // SAFETY: Runtime exclusive audio guard is the single playback consumer.
                self.pending = unsafe { q.pop_shared() };
            }
            let Some(p) = self.pending else {
                return (applied, late, true);
            };
            self.track = Some(p.track);
            if !e.running
                || e.selected != p.track
                || !e.decks[p.track].state.grabbed
                || e.recorder.is_some()
            {
                self.cancel = true;
            }
            if self.cancel {
                let _ = e.command(Command::DeckCut(p.track, false));
                self.pending = None;
                if p.last {
                    self.track = None;
                    self.cancel = false;
                    return (applied, late, true);
                }
                continue;
            }
            let tick = e.clock.tick();
            if tick < p.tick {
                return (applied, late, false);
            }
            late = late.max(tick - p.tick);
            if let Some(value) = p.position {
                let _ = e.command(Command::DeckFaderTimed(
                    p.track,
                    (value * 127.).round() / 127.,
                    p.interval_us,
                ));
            }
            if let Some(value) = p.cut {
                let _ = e.command(Command::DeckCut(p.track, value));
            }
            applied += 1;
            self.pending = None;
            if p.last {
                self.track = None;
                return (applied, late, true);
            }
        }
        (applied, late, false)
    }
}
pub fn frame(playback: &mut Playback, e: &mut Engine) {
    if option_env!("LOOP_CLUB_QA_SESSION").is_none() || !READY.load(Ordering::Acquire) {
        return;
    }
    if let Some(q) = EVENTS.get() {
        let (n, late, done) = playback.step(e, q);
        APPLIED.fetch_add(n, Ordering::Relaxed);
        MAX_LATE.fetch_max(late, Ordering::Relaxed);
        if done {
            READY.store(false, Ordering::Release);
            BUSY.store(false, Ordering::Release);
        }
    }
}
pub fn status() -> serde_json::Value {
    serde_json::json!({"ready":READY.load(Ordering::Acquire),"busy":BUSY.load(Ordering::Acquire),"applied":APPLIED.load(Ordering::Relaxed),"max_late_ticks":MAX_LATE.load(Ordering::Relaxed),"scope":"isolated QA sample-clock, quantized fader input; NOT physical hand performance"})
}
fn validate(p: &Plan, s: &crate::ui::Snapshot) -> Result<(), &'static str> {
    let now = s.words[8];
    let bpm = f64::from_bits(s.words[2]);
    if p.track >= 16
        || p.track != s.words[3] as usize
        || !bpm.is_finite()
        || !(40. ..=240.).contains(&bpm)
        || s.words[4] & 1 == 0
        || s.words[9] != 0
        || s.track_kind(p.track) != crate::model::Kind::Audio
        || s.audio_target(p.track) != crate::audio::RecordTarget::Turntable
    {
        return Err("wrong active scratch context");
    }
    if p.start_tick < now.saturating_add(PPQ)
        || p.start_tick > now.saturating_add(16 * PPQ)
        || !(2..=3000).contains(&p.events.len())
    {
        return Err("invalid future start or event count");
    }
    let mut previous = 0;
    for point in &p.events {
        if point.tick < previous
            || point.tick > 64 * PPQ
            || point
                .position
                .is_some_and(|v| !v.is_finite() || !(0. ..=1.).contains(&v))
            || point.position.is_none() && point.cut.is_none()
        {
            return Err("invalid point");
        }
        previous = point.tick;
    }
    if p.start_tick.checked_add(previous).is_none() {
        return Err("tick overflow");
    }
    if !p.events.last().is_some_and(|p| p.cut == Some(true)) {
        return Err("plan must end muted");
    }
    Ok(())
}
pub fn submit(p: Plan, s: &crate::ui::Snapshot) -> Result<serde_json::Value, &'static str> {
    if option_env!("LOOP_CLUB_QA_SESSION").is_none() {
        return Err("isolated QA build required");
    }
    validate(&p, s)?;
    if BUSY
        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
        .is_err()
    {
        return Err("scratch schedule busy");
    }
    let q = EVENTS.get_or_init(Queue::default);
    let mut prior = 0;
    let n = p.events.len();
    let bpm = f64::from_bits(s.words[2]);
    for (i, point) in p.events.into_iter().enumerate() {
        let us = if point.position.is_some() {
            let us = ((point.tick - prior) as f64 * 60_000_000. / (PPQ as f64 * bpm))
                .round()
                .min(u32::MAX as f64) as u32;
            prior = point.tick;
            us
        } else {
            0
        };
        let accepted = q.push(Event {
            tick: p.start_tick + point.tick,
            track: p.track,
            position: point.position,
            cut: point.cut,
            interval_us: us,
            last: i + 1 == n,
        });
        // Count is preflighted below4096; a single producer owns an empty queue.
        assert!(accepted);
    }
    APPLIED.store(0, Ordering::Relaxed);
    MAX_LATE.store(0, Ordering::Relaxed);
    READY.store(true, Ordering::Release);
    Ok(serde_json::json!({"start_tick":p.start_tick,"events":n,"queued":true}))
}
#[cfg(test)]
mod tests {
    use super::*;
    fn engine() -> Engine {
        let mut e = Engine::new(8000, 100., 8000);
        e.running = true;
        e.tracks[0].audio_target = crate::audio::RecordTarget::Turntable;
        e.tracks[0].source_frames = 8000;
        e.command(Command::DeckGrab(0, true, Some(0.2))).unwrap();
        e
    }
    #[test]
    fn schedule_runs_on_exact_audio_tick_and_stop_cancels_remaining_events() {
        let mut e = engine();
        let q = Queue::default();
        let mut p = Playback::default();
        q.push(Event {
            tick: PPQ / 4,
            track: 0,
            position: None,
            cut: Some(true),
            interval_us: 0,
            last: false,
        });
        q.push(Event {
            tick: PPQ / 2,
            track: 0,
            position: None,
            cut: Some(true),
            interval_us: 0,
            last: true,
        });
        while e.clock.tick() < PPQ / 4 {
            assert_eq!(p.step(&mut e, &q).0, 0);
            e.process(crate::model::Frame::ZERO);
        }
        assert_eq!(p.step(&mut e, &q).0, 1);
        assert!(e.deck_cuts[0]);
        e.running = false;
        assert!(p.step(&mut e, &q).2);
        assert!(!e.deck_cuts[0]);
    }
    #[test]
    fn schedule_rejects_wrong_context_unsorted_or_unmuted_end() {
        let mut e = engine();
        e.tracks[0].audio_target = crate::audio::RecordTarget::Turntable;
        let s = crate::ui::Snapshot::capture(&e);
        let mut p = Plan {
            track: 0,
            start_tick: 2 * PPQ,
            events: vec![
                Point {
                    tick: 0,
                    position: Some(0.2),
                    cut: None,
                },
                Point {
                    tick: PPQ,
                    position: None,
                    cut: Some(true),
                },
            ],
        };
        assert!(validate(&p, &s).is_ok());
        p.events[1].cut = Some(false);
        assert!(validate(&p, &s).is_err());
        p.events[1].cut = Some(true);
        p.events[0].tick = 2 * PPQ;
        assert!(validate(&p, &s).is_err());
        p.track = 16;
        assert!(validate(&p, &s).is_err());
    }
}
