//! Single preallocated take buffer: completed loops survive cancellation/replace.
use super::*;
impl Engine {
    pub(super) fn arm(&mut self, i: usize, resample: bool) -> Result<(), Error> {
        if self.recorder.is_some() {
            return Err(Error::Busy);
        }
        if resample && (self.tracks[i].kind != Kind::Audio || !self.tracks[i].empty()) {
            return Err(Error::WrongKind);
        }
        let existing = !self.tracks[i].empty();
        let now = self.clock.tick();
        let t = &self.tracks[i];
        let start = if existing && self.running && t.playing {
            let phase = t.phase(now);
            now + if phase == 0 {
                0
            } else {
                t.length_ticks - phase
            }
        } else {
            self.clock.bar_at_or_after(now) + u64::from(self.preroll) * 4 * PPQ
        };
        let bar_frames = 60. / self.clock.bpm * 4. * f64::from(self.clock.rate);
        let bars = (self.capture_audio.len().min(t.audio.len()) as f64 / bar_frames).floor() as u64;
        if bars == 0 || existing && t.length_ticks > bars * 4 * PPQ {
            return Err(Error::Capacity);
        }
        let length = t.length_ticks;
        if !existing {
            self.tracks[i].origin = start;
            self.tracks[i].events.clear();
            self.tracks[i].audio_frames = 0;
            self.tracks[i].generated = false;
            self.tracks[i].generated_sounds = 0;
        } else if !self.running || !self.tracks[i].playing {
            self.tracks[i].origin = start;
            self.tracks[i].playing = true;
            self.tracks[i].cursor = 0;
            self.tracks[i].last_phase = u64::MAX;
        }
        self.running = true;
        self.capture_velocities = [0; 128];
        self.open_notes[i] = [false; 128];
        self.recorder = Some(Recorder {
            track: i,
            start,
            end: existing.then_some(start + length),
            limit: start + bars * 4 * PPQ,
            resample,
            finish_started: false,
            repeating: existing,
            stop_after_cycle: resample,
            mode: self.tracks[i].record_mode,
            touched: 0,
        });
        self.prepare_take();
        Ok(())
    }
    pub(super) fn close_record(&mut self) -> Result<(), Error> {
        let Some(mut r) = self.recorder else {
            return Err(Error::NoContent);
        };
        if self.clock.tick() < r.start {
            self.cancel_capture();
            return Ok(());
        }
        if r.end.is_none() {
            r.end = Some(self.clock.next_bar().min(r.limit).max(r.start + 4 * PPQ));
        } else {
            r.stop_after_cycle = true;
        }
        self.recorder = Some(r);
        Ok(())
    }
    fn prepare_take(&mut self) {
        let r = self.recorder.unwrap();
        let t = &self.tracks[r.track];
        self.capture_events.clear();
        if r.repeating && (r.mode == RecordMode::Overdub || t.kind == Kind::Drum) {
            self.capture_events.extend_from_slice(&t.events);
        }
        *self.capture_features = crate::features::Features::default();
        if t.kind == Kind::Synth {
            for (note, &velocity) in self.capture_velocities.iter().enumerate() {
                if velocity > 0 && self.capture_events.len() < MAX_EVENTS - 128 {
                    self.capture_events.push(Event {
                        tick: 0,
                        note: note as u8,
                        velocity,
                        on: true,
                    });
                }
            }
        }
    }
    pub(super) fn cancel_capture(&mut self) {
        if let Some(r) = self.recorder.take() {
            if !r.repeating {
                let t = &mut self.tracks[r.track];
                t.length_ticks = 0;
                t.audio_frames = 0;
                t.events.clear();
                t.playing = false;
            }
            self.capture_events.clear();
            self.capture_velocities = [0; 128];
            // Live owners are released, but playback of a completed take can continue.
            for v in &mut self.voices {
                if v.track == r.track && v.source != 1 {
                    v.release();
                }
            }
            self.open_notes[r.track] = [false; 128];
        }
    }
    pub(super) fn capture_note(&mut self, i: usize, note: u8, velocity: u8, on: bool) {
        let Some(mut r) = self.recorder.filter(|r| r.track == i) else {
            return;
        };
        let tick = self.clock.tick();
        if tick < r.start || tick >= r.end.unwrap_or(r.limit) {
            return;
        }
        let on = on && velocity > 0;
        if self.tracks[i].kind == Kind::Drum && on {
            let bit = 1 << (note % 16);
            if r.mode == RecordMode::Replace && r.touched & bit == 0 {
                self.capture_events.retain(|e| e.note % 16 != note % 16);
                for v in &mut self.voices {
                    if v.track == i && v.source == 1 && v.note % 16 == note % 16 {
                        v.release();
                    }
                }
            }
            r.touched |= bit;
            self.recorder = Some(r);
        }
        if self.capture_events.len() >= MAX_EVENTS - 128 {
            self.event_overflows += 1;
            self.cancel_capture();
            self.error = Some(Error::Capacity);
            return;
        }
        self.capture_events.push(Event {
            tick: tick - r.start,
            note,
            velocity,
            on,
        });
        self.capture_velocities[usize::from(note)] = if on { velocity } else { 0 };
        self.open_notes[i][usize::from(note)] = on;
    }
    fn commit_take(&mut self, mut r: Recorder, end: u64) {
        let i = r.track;
        let length = end - r.start;
        let frames = (self.clock.frame_at(end) - self.clock.frame_at(r.start)) as usize;
        let t = &mut self.tracks[i];
        if t.kind == Kind::Audio {
            std::mem::swap(&mut t.audio, &mut self.capture_audio);
            std::mem::swap(&mut t.features, &mut self.capture_features);
            t.audio_frames = frames;
            t.source_frames = frames;
            t.chops.divide(frames);
            t.chop_undo = None;
            t.wave_view = (0, 0);
        } else {
            for (note, &velocity) in self.capture_velocities.iter().enumerate() {
                if velocity > 0 && self.capture_events.len() < MAX_EVENTS {
                    self.capture_events.push(Event {
                        tick: length - 1,
                        note: note as u8,
                        velocity: 0,
                        on: false,
                    });
                }
            }
            self.capture_events.sort_unstable_by_key(|e| (e.tick, e.on));
            std::mem::swap(&mut t.events, &mut self.capture_events);
            t.audio_frames = 0;
            // Generated/Undo ownership must not resurrect the previous recorded take.
            if t.kind == Kind::Drum {
                t.generated_sounds &= !r.touched;
                t.generated = t.generated_sounds != 0;
            }
        }
        t.pattern_undo = false;
        t.length_ticks = length;
        t.playing = self.running;
        t.cursor = t
            .events
            .partition_point(|e| e.tick < t.phase(self.clock.tick()));
        t.last_phase = u64::MAX;
        t.revision += 1;
        if r.stop_after_cycle || !self.running {
            self.recorder = None;
            self.capture_events.clear();
            self.capture_velocities = [0; 128];
            // Don't duplicate held live notes when completed playback takes over.
            for v in &mut self.voices {
                if v.track == i && v.source != 1 {
                    v.release();
                }
            }
        } else {
            r.start = end;
            r.end = Some(end + length);
            r.repeating = true;
            r.finish_started = false;
            r.mode = t.record_mode;
            r.touched = 0;
            if t.kind == Kind::Drum {
                self.capture_velocities = [0; 128];
            }
            self.recorder = Some(r);
            self.prepare_take();
        }
    }
    pub(super) fn process_capture(&mut self, input: Frame) {
        let Some(mut r) = self.recorder else {
            return;
        };
        let end = r.end.unwrap_or(r.limit);
        if self.clock.tick() >= end && !r.finish_started {
            r.finish_started = true;
            self.recorder = Some(r);
        }
        let latency = if self.tracks[r.track].kind == Kind::Audio && !r.resample {
            self.latency_in
        } else {
            0
        };
        let Some(frame) = self.clock.frame.checked_sub(u64::from(latency)) else {
            return;
        };
        if self.clock.tick_at(frame) >= end {
            self.commit_take(r, end);
        }
        let Some(r) = self.recorder else {
            return;
        };
        if !r.resample && self.tracks[r.track].kind == Kind::Audio {
            self.capture_frame(r, frame, input);
        }
    }
    fn capture_frame(&mut self, r: Recorder, frame: u64, input: Frame) {
        let tick = self.clock.tick_at(frame);
        if tick < r.start || tick >= r.end.unwrap_or(r.limit) {
            return;
        }
        let pos = frame.saturating_sub(self.clock.frame_at(r.start)) as usize;
        if pos >= self.capture_audio.len() {
            return;
        }
        let old = if r.repeating && r.mode == RecordMode::Overdub {
            self.tracks[r.track]
                .audio
                .get(pos)
                .copied()
                .unwrap_or(Frame::ZERO)
        } else {
            Frame::ZERO
        };
        // Finite, bounded PCM even after many overdubs; no automatic feedback decay.
        let x = input.combine(old);
        let x = if x.finite() {
            Frame {
                l: x.l.clamp(-1., 1.),
                r: x.r.clamp(-1., 1.),
            }
        } else {
            Frame::ZERO
        };
        self.capture_audio[pos] = x;
        self.capture_features.feed(x, self.clock.rate);
        if !r.repeating {
            self.tracks[r.track].audio_frames = pos + 1;
        }
        self.recorded_frames += 1;
    }
    pub(super) fn capture_resample(&mut self, sum: Frame) {
        if let Some(r) = self.recorder.filter(|r| r.resample) {
            self.capture_frame(r, self.clock.frame, sum);
        }
    }
}
