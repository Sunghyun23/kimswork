//! Single preallocated take buffer: completed loops survive cancellation/replace.
use super::*;
impl Engine {
    fn mixed_audio_overdub(t: &Track) -> bool {
        t.kind == Kind::Audio
            && t.audio_sequence
            && t.record_mode == RecordMode::Overdub
            && ((t.audio_target == crate::audio::RecordTarget::Turntable)
                != t.events.first().is_some_and(|e| e.deck.is_some()))
    }

    pub(super) fn check_record_capacity(&self, i: usize) -> Result<(), Error> {
        let t = &self.tracks[i];
        if Self::mixed_audio_overdub(t) {
            return Err(Error::ResampleFirst);
        }
        if t.kind == Kind::Audio && t.audio_target != crate::audio::RecordTarget::Input {
            return if t.empty() || t.audio_frames == 0 {
                Err(Error::NoContent)
            } else {
                Ok(())
            };
        }
        if t.kind == Kind::Audio
            && !t.empty()
            && t.record_mode == RecordMode::Overdub
            && (t.audio_sequence
                || t.audio_frames
                    .abs_diff(self.clock.frame_at(t.length_ticks) as usize)
                    > 1)
        {
            return Err(Error::ResampleFirst);
        }
        let bar_frames = 60. / self.clock.bpm * 4. * f64::from(self.clock.rate);
        let bars = (self.capture_audio.len().min(t.audio.len()) as f64 / bar_frames).floor() as u64;
        if bars == 0 || (!t.empty() && t.length_ticks > bars * 4 * PPQ) {
            return Err(Error::Capacity);
        }
        Ok(())
    }
    pub(super) fn start_loop_record(&mut self, i: usize, at: Option<u64>) -> Result<(), Error> {
        if self.chain_protects(i) {
            return Err(Error::Busy);
        }
        self.arm_at(i, false, at, true)?;
        if self.tracks[i].record_mode == RecordMode::Replace {
            let start = at.unwrap_or_else(|| {
                self.clock.bar_at_or_after(self.clock.tick()) + u64::from(self.preroll) * 4 * PPQ
            });
            if let Some(r) = &mut self.recorder {
                r.limit = start + r.limit.saturating_sub(r.start);
                r.start = start;
                r.end = None;
                r.repeating = false;
                r.count_in_start = (at.is_none() && self.preroll > 0)
                    .then(|| start.saturating_sub(u64::from(self.preroll) * 4 * PPQ));
            }
            self.prepare_take();
        }
        self.selected = i;
        Ok(())
    }
    pub(super) fn arm(&mut self, i: usize, resample: bool) -> Result<(), Error> {
        self.arm_at(i, resample, None, false)
    }
    fn arm_at(
        &mut self,
        i: usize,
        resample: bool,
        at: Option<u64>,
        selected_target: bool,
    ) -> Result<(), Error> {
        if self.recorder.is_some() {
            return Err(Error::Busy);
        }
        if resample && (self.tracks[i].kind != Kind::Audio || !self.tracks[i].empty()) {
            return Err(Error::WrongKind);
        }
        let midi = self.tracks[i].kind != Kind::Audio
            || (selected_target
                && self.tracks[i].audio_target != crate::audio::RecordTarget::Input);
        if midi && self.tracks[i].kind == Kind::Audio && self.tracks[i].audio_frames == 0 {
            return Err(Error::NoContent);
        }
        if Self::mixed_audio_overdub(&self.tracks[i]) {
            return Err(Error::ResampleFirst);
        }
        let existing = !self.tracks[i].empty();
        let now = self.clock.tick();
        let t = &self.tracks[i];
        let start = at.unwrap_or_else(|| {
            if existing && self.running && t.playing {
                let phase = t.phase(now);
                now + if phase == 0 {
                    0
                } else {
                    t.length_ticks - phase
                }
            } else {
                self.clock.bar_at_or_after(now) + u64::from(self.preroll) * 4 * PPQ
            }
        });
        if !midi
            && t.kind == Kind::Audio
            && !t.empty()
            && t.record_mode == RecordMode::Overdub
            && (t.audio_sequence
                || t.audio_frames
                    .abs_diff(self.clock.frame_at(t.length_ticks) as usize)
                    > 1)
        {
            return Err(Error::ResampleFirst);
        }
        let bar_frames = 60. / self.clock.bpm * 4. * f64::from(self.clock.rate);
        let bars = (self.capture_audio.len().min(t.audio.len()) as f64 / bar_frames).floor() as u64;
        if bars == 0 || !midi && existing && t.length_ticks > bars * 4 * PPQ {
            return Err(Error::Capacity);
        }
        let count_in_start =
            (at.is_none() && self.preroll > 0 && !(existing && self.running && t.playing))
                .then(|| start.saturating_sub(u64::from(self.preroll) * 4 * PPQ));
        let length = t.length_ticks;
        self.record_before = Some(ClipState::read(t));
        self.record_history_committed = false;
        let previous_origin = t.origin;
        let previous_playing = t.playing;
        if !existing {
            self.tracks[i].origin = start;
            self.tracks[i].events.clear();
            self.tracks[i].audio_frames = 0;
            self.tracks[i].generated = false;
            self.tracks[i].generated_sounds = 0;
        } else if at.is_some() || !self.running || !self.tracks[i].playing {
            self.tracks[i].origin = start;
            self.tracks[i].playing = true;
            self.tracks[i].cursor = 0;
            self.tracks[i].last_phase = u64::MAX;
        }
        self.next_record = None;
        self.running = true;
        self.capture_velocities = [0; 128];
        self.open_notes[i] = [false; 128];
        self.recorder = Some(Recorder {
            midi,
            preserve_on_cancel: existing,
            previous_origin,
            previous_playing,
            track: i,
            start,
            end: existing.then_some(start + length),
            limit: start
                + if midi {
                    (bars * 4 * PPQ).max(length)
                } else {
                    bars * 4 * PPQ
                },
            resample,
            finish_started: false,
            notes_started: false,
            repeating: existing,
            stop_after_cycle: resample,
            mode: self.tracks[i].record_mode,
            touched: 0,
            edited_pads: 0,
            count_in_start,
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
    pub(super) fn prepare_take(&mut self) {
        self.deck_spans.reset();
        let r = self.recorder.unwrap();
        let t = &self.tracks[r.track];
        self.capture_events.clear();
        if r.midi && r.repeating && (r.mode == RecordMode::Overdub || t.kind == Kind::Drum) {
            self.capture_events.extend_from_slice(&t.events);
        }
        if r.midi
            && r.repeating
            && r.mode == RecordMode::Overdub
            && t.kind == Kind::Audio
            && !t.audio_sequence
            && t.audio_target != crate::audio::RecordTarget::Turntable
        {
            self.capture_events.push(Event {
                deck: None,
                timing: None,
                slice: false,
                tick: 0,
                note: t.audio_settings.root,
                velocity: 127,
                on: true,
            });
            self.capture_events.push(Event {
                deck: None,
                timing: None,
                slice: false,
                tick: t.length_ticks - 1,
                note: t.audio_settings.root,
                velocity: 0,
                on: false,
            });
        }
        self.capture_prepared = 0;
        if !r.midi && r.repeating && r.mode == RecordMode::Overdub && t.kind == Kind::Audio {
            self.capture_features.begin_overdub(&t.features);
        } else {
            *self.capture_features = crate::features::Features::default();
        }
    }
    fn begin_capture_notes(&mut self, r: &mut Recorder) {
        if r.notes_started || self.clock.tick() < r.start {
            return;
        }
        if r.mode == RecordMode::Replace && self.tracks[r.track].kind != Kind::Drum {
            self.release_sequence_for_recording(r.track);
        }

        if r.midi
            && self.tracks[r.track].audio_target == crate::audio::RecordTarget::Turntable
            && (r.mode != RecordMode::Overdub
                || !self.tracks[r.track].audio_sequence
                || self.decks[r.track].live)
        {
            let i = r.track;
            if self.capture_events.len() >= MAX_EVENTS {
                self.event_overflows += 1;
                self.cancel_capture();
                self.error = Some(Error::Capacity);
                return;
            }
            if !self.decks[i].active {
                self.decks[i].start(0.);
            }
            self.capture_events.push(Event {
                deck: Some(self.decks[i].state.normalized(self.tracks[i].source_frames)),
                timing: None,
                slice: false,
                tick: 0,
                note: 0,
                velocity: 0,
                on: true,
            });
        }

        if r.midi
            && self.tracks[r.track].audio_target == crate::audio::RecordTarget::Turntable
            && self.decks[r.track].live
        {
            self.deck_spans.edge(0, true);
        }
        if r.midi && self.tracks[r.track].kind != Kind::Drum {
            for (note, &velocity) in self.capture_velocities.iter().enumerate() {
                if velocity > 0 && self.capture_events.len() < MAX_EVENTS - 128 {
                    self.capture_events.push(Event {
                        deck: None,
                        timing: None,
                        slice: self.tracks[r.track].kind == Kind::Audio
                            && self.tracks[r.track].audio_target
                                == crate::audio::RecordTarget::Chop,
                        tick: 0,
                        note: note as u8,
                        velocity,
                        on: true,
                    });
                }
            }
        }
        r.notes_started = true;
        self.recorder = Some(*r);
    }

    pub(super) fn cancel_capture(&mut self) {
        self.next_record = None;
        if let Some(r) = self.recorder.take() {
            if r.preserve_on_cancel {
                self.tracks[r.track].origin = r.previous_origin;
                self.tracks[r.track].playing = r.previous_playing;
            } else if !r.repeating {
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
            for v in &mut self.slice_voices {
                if v.track == r.track && v.sampler.is_some_and(|s| s.source != 1) {
                    v.release = true;
                }
            }
            self.open_notes[r.track] = [false; 128];
        }
    }
    pub(super) fn capture_note(&mut self, i: usize, note: u8, velocity: u8, on: bool) {
        let Some(mut r) = self.recorder.filter(|r| r.track == i) else {
            return;
        };
        if !r.midi {
            return;
        }
        let tick = self.clock.tick();
        let on = on && velocity > 0;
        if tick < r.start {
            if self.tracks[i].kind != Kind::Drum {
                self.capture_velocities[usize::from(note)] = if on { velocity } else { 0 };
                self.open_notes[i][usize::from(note)] = on;
            }
            return;
        }
        if tick >= r.end.unwrap_or(r.limit) {
            return;
        }
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
            deck: None,
            timing: None,
            slice: self.tracks[r.track].kind == Kind::Audio
                && self.tracks[r.track].audio_target == crate::audio::RecordTarget::Chop,
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
        let captured_length = end - r.start;
        let captured_frames = (self.clock.frame_at(end) - self.clock.frame_at(r.start)) as usize;
        let keep_length = r.repeating && r.mode == RecordMode::Overdub;
        let length = if keep_length {
            self.tracks[i].length_ticks
        } else {
            captured_length
        };
        let frames = if keep_length && self.tracks[i].kind == Kind::Audio {
            self.tracks[i].audio_frames
        } else {
            captured_frames
        };
        if !self.record_history_committed {
            // Save the pre-arm metadata; old PCM/events are in capture scratch after the swap.
            self.history.checkpoint(
                i,
                &self.tracks[i],
                if !r.midi {
                    EditKind::Audio
                } else {
                    EditKind::Midi
                },
            );
            self.history.state = self.record_before;
        }
        if !r.midi {
            self.clear_slice_voices(i);
        }
        if !(r.midi && self.tracks[i].audio_target == crate::audio::RecordTarget::Turntable) {
            self.decks[i] = crate::deck::Player::default();
        }
        let t = &mut self.tracks[i];
        if !r.repeating {
            t.origin = r.start;
        }
        if !r.midi {
            if keep_length {
                self.capture_features
                    .retain_tail_onsets(&t.features, captured_frames);
            }
            std::mem::swap(&mut t.audio, &mut self.capture_audio);
            std::mem::swap(&mut t.features, &mut self.capture_features);
            t.audio_frames = frames;
            t.source_frames = frames;
            t.source_ticks = length;
            t.audio_sequence = false;
            t.audio_player.reset();
            t.events.clear();
            t.chops.divide(frames);
            t.wave_view = (0, 0);
        } else {
            if t.audio_target == crate::audio::RecordTarget::Turntable {
                self.capture_events.retain(|e| {
                    e.deck.is_none() || e.on || !self.deck_spans.covered(e.tick, captured_length)
                });
            }
            for (note, &velocity) in self.capture_velocities.iter().enumerate() {
                if velocity > 0 && self.capture_events.len() < MAX_EVENTS {
                    self.capture_events.push(Event {
                        deck: None,
                        timing: None,
                        slice: t.kind == Kind::Audio
                            && t.audio_target == crate::audio::RecordTarget::Chop,
                        tick: captured_length - 1,
                        note: note as u8,
                        velocity: 0,
                        on: false,
                    });
                }
            }
            self.capture_events.retain(|event| event.tick < length);
            self.capture_events.sort_unstable_by_key(|e| (e.tick, e.on));
            for e in &mut self.capture_events {
                if e.deck.is_some() {
                    e.on = false;
                }
            }
            std::mem::swap(&mut t.events, &mut self.capture_events);
            if t.kind == Kind::Audio {
                t.audio_sequence = true;
            } else {
                t.audio_frames = 0;
            }
            // Generated/Undo ownership must not resurrect the previous recorded take.
            if t.kind == Kind::Drum {
                if !r.repeating && r.mode == RecordMode::Replace {
                    t.generated_sounds = 0;
                } else {
                    t.generated_sounds &= !r.touched;
                }
                t.generated = t.generated_sounds != 0;
            }
        }
        if !self.record_history_committed {
            if !r.midi {
                std::mem::swap(&mut self.capture_audio, &mut self.history.audio);
                std::mem::swap(&mut self.capture_features, &mut self.history.features);
            } else {
                std::mem::swap(&mut self.capture_events, &mut self.history.events);
            }
            self.record_history_committed = true;
        }
        t.length_ticks = length;
        t.playing = self.running;
        t.cursor = t
            .events
            .partition_point(|e| e.tick < t.phase(self.clock.tick()));
        t.last_phase = u64::MAX;
        t.revision += 1;
        if r.stop_after_cycle || !self.running {
            let next = self.next_record.take();
            self.recorder = None;
            self.capture_events.clear();
            self.capture_velocities = [0; 128];
            // Don't duplicate held live notes when completed playback takes over.
            for v in &mut self.voices {
                if v.track == i && v.source != 1 {
                    v.release();
                }
            }
            for v in &mut self.slice_voices {
                if v.track == i && v.sampler.is_some_and(|s| s.source != 1) {
                    v.release = true;
                }
            }
            if let Some(next) = next.filter(|_| self.running) {
                if let Err(error) = self.start_loop_record(next, Some(end)) {
                    self.error = Some(error);
                }
                self.loop_revision = self.loop_revision.wrapping_add(1);
            }
        } else {
            r.start = end;
            r.end = Some(end + length);
            r.repeating = true;
            r.finish_started = false;
            r.notes_started = false;
            r.mode = t.record_mode;
            r.touched = 0;
            r.edited_pads = 0;
            r.count_in_start = None;
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
        self.begin_capture_notes(&mut r);
        if self.recorder.is_none() {
            return;
        }
        if !r.midi
            && r.repeating
            && r.mode == RecordMode::Overdub
            && self.tracks[r.track].kind == Kind::Audio
        {
            let stop = (self.capture_prepared + 64).min(self.tracks[r.track].audio_frames);
            if stop > self.capture_prepared {
                self.capture_audio[self.capture_prepared..stop]
                    .copy_from_slice(&self.tracks[r.track].audio[self.capture_prepared..stop]);
                self.capture_prepared = stop;
            }
        }
        let end = r.end.unwrap_or(r.limit);
        if self.clock.tick() >= end && !r.finish_started {
            r.finish_started = true;
            self.recorder = Some(r);
        }
        let latency = if !r.midi && !r.resample {
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
        if !r.resample && !r.midi {
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
        if r.repeating && r.mode == RecordMode::Overdub {
            self.capture_features.feed_at(pos, x, self.clock.rate);
        } else {
            self.capture_features.feed(x, self.clock.rate);
        }
        if !r.repeating && !r.preserve_on_cancel {
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

impl Engine {
    pub(super) fn capture_deck(&mut self, i: usize) -> Result<(), Error> {
        let Some(r) = self.recorder.filter(|r| {
            r.track == i
                && r.midi
                && self.tracks[i].audio_target == crate::audio::RecordTarget::Turntable
        }) else {
            return Ok(());
        };
        let tick = self.clock.tick();
        if tick < r.start || tick >= r.end.unwrap_or(r.limit) {
            return Ok(());
        }
        let coalesce = self
            .capture_events
            .last()
            .is_some_and(|e| e.tick == tick - r.start && e.deck.is_some() && e.on);
        if (self.capture_events.len() >= MAX_EVENTS && !coalesce)
            || !self
                .deck_spans
                .edge(tick - r.start, self.decks[i].state.grabbed)
        {
            self.event_overflows += 1;
            self.cancel_capture();
            self.error = Some(Error::Capacity);
            return Err(Error::Capacity);
        }
        let event = Event {
            deck: Some(self.decks[i].state.normalized(self.tracks[i].source_frames)),
            timing: None,
            slice: false,
            tick: tick - r.start,
            note: 0,
            velocity: 0,
            on: true,
        };
        if let Some(last) = self.capture_events.last_mut()
            && last.tick == event.tick
            && last.deck.is_some()
            && last.on
        {
            *last = event;
        } else {
            self.capture_events.push(event);
        }
        Ok(())
    }
}
