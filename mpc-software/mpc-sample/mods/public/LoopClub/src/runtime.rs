//! Audio/UI handoff. Audio never waits for UI or performs filesystem work.
use crate::{
    engine::{Command, Engine},
    model::*,
    queue::Queue,
    ui::{Snapshot, WORDS},
};
use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, AtomicPtr, AtomicU32, AtomicU64, AtomicUsize, Ordering},
};

pub struct AtomicSnapshot {
    sequence: AtomicU64,
    words: [AtomicU64; WORDS],
}
impl Default for AtomicSnapshot {
    fn default() -> Self {
        Self {
            sequence: AtomicU64::new(0),
            words: std::array::from_fn(|_| AtomicU64::new(0)),
        }
    }
}
impl AtomicSnapshot {
    // Single writer: only while holding Runtime's exclusive engine guard.
    pub(crate) fn publish(&self, s: &Snapshot) {
        self.sequence.fetch_add(1, Ordering::SeqCst);
        for (dst, src) in self.words.iter().zip(s.words) {
            dst.store(src, Ordering::SeqCst)
        }
        self.sequence.fetch_add(1, Ordering::SeqCst);
    }
    pub fn read(&self) -> Option<Snapshot> {
        for _ in 0..3 {
            let begin = self.sequence.load(Ordering::SeqCst);
            if begin & 1 != 0 {
                continue;
            }
            let mut s = Snapshot::default();
            for (dst, src) in s.words.iter_mut().zip(&self.words) {
                *dst = src.load(Ordering::SeqCst)
            }
            if self.sequence.load(Ordering::SeqCst) == begin {
                return Some(s);
            }
        }
        None
    }
}
pub struct Runtime {
    pub output_mirror: crate::output_mirror::OutputMirror,
    kit_ready: AtomicPtr<crate::bank::KitJob>,
    kit_done: AtomicPtr<crate::bank::KitJob>,
    import_ready: AtomicPtr<crate::sample::ImportJob>,
    import_done: AtomicPtr<crate::sample::ImportJob>,
    external: [AtomicU64; 512],
    requested_track: AtomicUsize,
    was_active: AtomicBool,
    engine: UnsafeCell<Option<Box<Engine>>>,
    busy: AtomicBool,
    commands: Queue<Command, 256>,
    input: Queue<Frame, 16384>,
    pub snapshot: AtomicSnapshot,
    pub active: AtomicBool,
    pub rate: AtomicU32,
    pub pcm: AtomicUsize,
    pub lost_input: AtomicU64,
    pub missing_input: AtomicU64,
    input_seen: AtomicBool,
    panic: AtomicBool,
    pub rejected_commands: AtomicU64,
    input_rate: AtomicU32,
}
// SAFETY: Engine has exactly one owner behind `busy`; callbacks never wait.
// Queues and snapshots synchronize their own slots. No references escape with_engine.
unsafe impl Sync for Runtime {}
struct Guard<'a>(&'a AtomicBool);
impl Drop for Guard<'_> {
    fn drop(&mut self) {
        self.0.store(false, Ordering::Release)
    }
}
impl Default for Runtime {
    fn default() -> Self {
        Self {
            output_mirror: crate::output_mirror::OutputMirror::default(),
            kit_ready: AtomicPtr::new(std::ptr::null_mut()),
            kit_done: AtomicPtr::new(std::ptr::null_mut()),
            import_ready: AtomicPtr::new(std::ptr::null_mut()),
            import_done: AtomicPtr::new(std::ptr::null_mut()),
            external: std::array::from_fn(|_| AtomicU64::new(0)),
            requested_track: AtomicUsize::new(0),
            was_active: AtomicBool::new(false),
            engine: UnsafeCell::new(None),
            busy: AtomicBool::new(false),
            commands: Queue::default(),
            input: Queue::default(),
            snapshot: AtomicSnapshot::default(),
            active: AtomicBool::new(false),
            rate: AtomicU32::new(0),
            pcm: AtomicUsize::new(0),
            lost_input: AtomicU64::new(0),
            missing_input: AtomicU64::new(0),
            input_seen: AtomicBool::new(false),
            panic: AtomicBool::new(false),
            rejected_commands: AtomicU64::new(0),
            input_rate: AtomicU32::new(0),
        }
    }
}
impl Runtime {
    pub fn with_engine<R>(&self, f: impl FnOnce(&mut Option<Box<Engine>>) -> R) -> Option<R> {
        self.busy
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .ok()?;
        let _guard = Guard(&self.busy);
        // SAFETY: successful compare_exchange grants exclusive access through the closure.
        Some(f(unsafe { &mut *self.engine.get() }))
    }
    /// Allocate outside realtime, then transfer an already constructed engine.
    pub fn initialize(&self, e: Box<Engine>) -> Result<(), Box<Engine>> {
        let mut value = Some(e);
        let accepted = self
            .with_engine(|slot| {
                if slot.is_some() {
                    return false;
                }
                *slot = value.take();
                if let Some(e) = slot {
                    self.snapshot.publish(&Snapshot::capture(e))
                }
                true
            })
            .unwrap_or(false);
        if accepted {
            Ok(())
        } else {
            Err(value.unwrap())
        }
    }
    /// Keep note-off ownership when the selected track changes. Four independent
    /// external sources per pitch; source IDs are the SDK's 32-bit firmware IDs.
    pub fn external_note(&self, source: u32, note: u8, velocity: u8, on: bool) -> bool {
        if note > 127 {
            return false;
        }
        let key = u64::from(source) << 8;
        let start = usize::from(note) * 4;
        let active = self.active.load(Ordering::Acquire);
        for slot in start..start + 4 {
            let old = self.external[slot].load(Ordering::Acquire);
            if old != 0 && old & !255 == key {
                let track = (old & 255) as usize - 1;
                if !on || velocity == 0 {
                    if self.external[slot]
                        .compare_exchange(old, 0, Ordering::AcqRel, Ordering::Relaxed)
                        .is_ok()
                    {
                        self.enqueue(Command::Note {
                            track,
                            note,
                            velocity: 0,
                            on: false,
                            source: 1000 + slot as u16,
                        });
                    }
                    return true;
                }
                if !active {
                    return false;
                }
                let selected = self.requested_track.load(Ordering::Acquire).min(15);
                if self.external[slot]
                    .compare_exchange(
                        old,
                        key | (selected as u64 + 1),
                        Ordering::AcqRel,
                        Ordering::Relaxed,
                    )
                    .is_ok()
                {
                    self.enqueue(Command::Note {
                        track,
                        note,
                        velocity: 0,
                        on: false,
                        source: 1000 + slot as u16,
                    });
                    self.enqueue(Command::Note {
                        track: selected,
                        note,
                        velocity,
                        on: true,
                        source: 1000 + slot as u16,
                    });
                } else {
                    self.emergency_stop()
                }
                return true;
            }
        }
        if !active || !on || velocity == 0 {
            return false;
        }
        let track = self.requested_track.load(Ordering::Acquire).min(15);
        for slot in start..start + 4 {
            if self.external[slot]
                .compare_exchange(
                    0,
                    key | (track as u64 + 1),
                    Ordering::AcqRel,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                self.enqueue(Command::Note {
                    track,
                    note,
                    velocity,
                    on: true,
                    source: 1000 + slot as u16,
                });
                return true;
            }
        }
        self.emergency_stop();
        true
    }
    /// Submit ownership from the file worker. No deallocation occurs in audio.
    pub fn submit_kit(
        &self,
        job: Box<crate::bank::KitJob>,
    ) -> Result<(), Box<crate::bank::KitJob>> {
        let ptr = Box::into_raw(job);
        match self.kit_ready.compare_exchange(
            std::ptr::null_mut(),
            ptr,
            Ordering::Release,
            Ordering::Relaxed,
        ) {
            Ok(_) => Ok(()),
            // SAFETY: failed publication retains unique ownership.
            Err(_) => Err(unsafe { Box::from_raw(ptr) }),
        }
    }
    pub fn collect_kit(&self) -> Option<Box<crate::bank::KitJob>> {
        let ptr = self.kit_done.swap(std::ptr::null_mut(), Ordering::AcqRel);
        if ptr.is_null() {
            None
        } else {
            // SAFETY: the swap transfers the completed job's unique ownership.
            Some(unsafe { Box::from_raw(ptr) })
        }
    }
    pub fn submit_import(
        &self,
        job: Box<crate::sample::ImportJob>,
    ) -> Result<(), Box<crate::sample::ImportJob>> {
        let ptr = Box::into_raw(job);
        match self.import_ready.compare_exchange(
            std::ptr::null_mut(),
            ptr,
            Ordering::Release,
            Ordering::Relaxed,
        ) {
            Ok(_) => Ok(()),
            Err(_) => {
                // SAFETY: publication failed, so ownership never left this call.
                Err(unsafe { Box::from_raw(ptr) })
            }
        }
    }
    /// File worker only; collecting/dropping the returned buffers is not realtime.
    pub fn collect_import(&self) -> Option<Box<crate::sample::ImportJob>> {
        let ptr = self
            .import_done
            .swap(std::ptr::null_mut(), Ordering::AcqRel);
        if ptr.is_null() {
            None
        } else {
            // SAFETY: swap grants unique ownership of the audio-completed job.
            Some(unsafe { Box::from_raw(ptr) })
        }
    }
    pub fn enqueue(&self, c: Command) {
        if let Command::Select(track) | Command::Pad(track, _) = c
            && track < 16
        {
            self.requested_track.store(track, Ordering::Release);
        }
        if !self.commands.push(c) {
            self.rejected_commands.fetch_add(1, Ordering::Relaxed);
            self.panic.store(true, Ordering::Release);
        }
    }
    pub fn capture(&self, frame: Frame, rate: u32) {
        self.input_rate.store(rate, Ordering::Relaxed);
        self.input_seen.store(true, Ordering::Release);
        if !self.input.push(frame) {
            self.lost_input.fetch_add(1, Ordering::Relaxed);
        }
    }
    pub fn emergency_stop(&self) {
        self.panic.store(true, Ordering::Release)
    }
    /// Called for exactly one bound hardware output stream.
    pub fn output(&self, samples: &mut [i32], channels: usize, rate: u32) {
        if channels == 0 || !samples.len().is_multiple_of(channels) {
            return;
        }
        let _ = self.with_engine(|slot| {
            let Some(e) = slot.as_mut() else { return };
            if e.clock.rate != rate {
                self.emergency_stop();
                return;
            }
            if self.panic.swap(false, Ordering::AcqRel) {
                let _ = e.command(Command::Stop);
                e.emergency_silence();
                for _ in 0..256 {
                    // SAFETY: exclusive guard makes this the only consumer.
                    if unsafe { self.commands.pop_shared() }.is_none() {
                        break;
                    }
                }
            }
            let active = self.active.load(Ordering::Acquire);
            if active && !self.was_active.swap(active, Ordering::AcqRel) {
                for _ in 0..16384 {
                    // SAFETY: exclusively held busy guard owns the input consumer.
                    if unsafe { self.input.pop_shared() }.is_none() {
                        break;
                    }
                }
            } else if !active {
                self.was_active.store(false, Ordering::Release);
            }

            for _ in 0..256 {
                // SAFETY: busy excludes all other output/project consumers.
                let Some(c) = (unsafe { self.commands.pop_shared() }) else {
                    break;
                };
                let is_capture = matches!(c, Command::Record(_))
                    || matches!(c,Command::Pad(i,false) if i<16&&e.tracks[i].empty());
                let track = match c {
                    Command::Record(i) | Command::Pad(i, _) => i,
                    _ => 16,
                };
                if is_capture
                    && track < 16
                    && e.tracks[track].kind == Kind::Audio
                    && (!self.input_seen.load(Ordering::Acquire)
                        || self.input_rate.load(Ordering::Relaxed) != rate)
                {
                    let _ = e.command(Command::Select(track));
                    e.error = Some(Error::InputUnavailable);
                    continue;
                }
                let _ = e.command(c);
            }
            if self.kit_done.load(Ordering::Acquire).is_null() {
                let ptr = self.kit_ready.swap(std::ptr::null_mut(), Ordering::AcqRel);
                if !ptr.is_null() {
                    // SAFETY: swap transfers unique worker ownership under engine guard.
                    unsafe { (&mut *ptr).commit(e) };
                    self.kit_done.store(ptr, Ordering::Release);
                }
            }
            if self.import_done.load(Ordering::Acquire).is_null() {
                let ptr = self
                    .import_ready
                    .swap(std::ptr::null_mut(), Ordering::AcqRel);
                if !ptr.is_null() {
                    // SAFETY: successful swap transfers unique worker ownership here.
                    unsafe { (&mut *ptr).commit(e) };
                    self.import_done.store(ptr, Ordering::Release);
                }
            }
            for chunk in samples.chunks_exact_mut(channels) {
                // SAFETY: only output under busy consumes input; producers only reserve/write.
                let input = unsafe { self.input.pop_shared() };
                if input.is_none()
                    && e.recorder.is_some_and(|r| {
                        e.tracks[r.track].kind == Kind::Audio
                            && !r.resample
                            && e.clock.tick() >= r.start
                    })
                {
                    self.missing_input.fetch_add(1, Ordering::Relaxed);
                    let _ = e.command(Command::CancelRecord);
                    e.error = Some(Error::InputUnavailable)
                }
                if !active {
                    if e.recorder.is_some_and(|r| r.finish_started) {
                        e.process(input.unwrap_or(Frame::ZERO));
                    }
                    self.output_mirror.push(Frame::ZERO);
                    continue;
                }
                let mut s = e.process(input.unwrap_or(Frame::ZERO));
                let mut usb = e.usb_output;
                if let Some(r) = e.recorder
                    && e.clock.tick() < r.start
                {
                    let tick = e.clock.tick();
                    let phase = tick % PPQ;
                    let seconds = phase as f32 / PPQ as f32 * 60. / e.clock.bpm as f32;
                    if seconds < 0.025 {
                        let frequency = if (tick / PPQ).is_multiple_of(4) {
                            1320.
                        } else {
                            880.
                        };
                        let click = Frame::mono(
                            (seconds * frequency * std::f32::consts::TAU).sin()
                                * (1. - seconds / 0.025)
                                * 0.08
                                * e.master_gain,
                        );
                        s = s.combine(click);
                        if !e.separate_usb {
                            usb = usb.combine(click);
                        }
                    }
                }
                self.output_mirror.push(usb);
                for (ch, dst) in chunk.iter_mut().enumerate().take(2) {
                    let x = if channels == 1 {
                        (s.l + s.r) * 0.5
                    } else if ch == 0 {
                        s.l
                    } else {
                        s.r
                    };
                    let added = f64::from(*dst) + f64::from(x) * f64::from(i32::MAX);
                    *dst = added.clamp(f64::from(i32::MIN), f64::from(i32::MAX)) as i32;
                }
            }
            let mut s = Snapshot::capture(e);
            s.words[792] = self.lost_input.load(Ordering::Relaxed);
            s.words[793] = self.missing_input.load(Ordering::Relaxed);
            s.words[794] = self.rejected_commands.load(Ordering::Relaxed);
            self.snapshot.publish(&s);
        });
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        for slot in [&self.kit_ready, &self.kit_done] {
            let ptr = slot.swap(std::ptr::null_mut(), Ordering::AcqRel);
            // SAFETY: exclusive destruction has no live callbacks.
            if !ptr.is_null() {
                unsafe {
                    drop(Box::from_raw(ptr));
                }
            }
        }
        for slot in [&self.import_ready, &self.import_done] {
            let ptr = slot.swap(std::ptr::null_mut(), Ordering::AcqRel);
            if !ptr.is_null() {
                // SAFETY: exclusive Runtime destruction has no concurrent callbacks; this slot still owns its job.
                unsafe {
                    drop(Box::from_raw(ptr));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    #[test]
    fn atomic_snapshot_never_returns_torn_payload() {
        let shared = Arc::new(AtomicSnapshot::default());
        let writer = shared.clone();
        let thread = std::thread::spawn(move || {
            for n in 1..500 {
                writer.publish(&Snapshot { words: [n; WORDS] });
            }
        });
        for _ in 0..1000 {
            if let Some(s) = shared.read() {
                assert!(s.words.iter().all(|v| *v == s.words[0]));
            }
        }
        thread.join().unwrap();
    }
}
