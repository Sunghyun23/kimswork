//! Non-destructive pad-lane MIDI editing, including the current uncommitted take.
use super::*;
impl Engine {
    pub(super) fn edit_pad_notes(
        &mut self,
        track: usize,
        source: u8,
        target: Option<u8>,
    ) -> Result<(), Error> {
        Self::valid_track(track)?;
        if source >= 16 || target.is_some_and(|n| n >= 16 || n == source) {
            return Err(Error::InvalidValue);
        }
        let drum = self.tracks[track].kind == Kind::Drum;
        if !drum && self.tracks[track].kind != Kind::Audio {
            return Err(Error::WrongKind);
        }
        if !drum
            && (source >= self.tracks[track].chops.count
                || target.is_some_and(|n| n >= self.tracks[track].chops.count))
        {
            return Err(Error::InvalidValue);
        }
        if self.chain_protects(track) {
            return Err(Error::Busy);
        }
        let recording = self.recorder.is_some();
        if let Some(r) = self.recorder {
            if r.track != track
                || !r.midi
                || self.clock.tick() < r.start
                || (!drum && self.tracks[track].audio_target != crate::audio::RecordTarget::Chop)
            {
                return Err(Error::Busy);
            }
            // Copying an unfinished gate would strand a new note-on on the destination.
            if target.is_some()
                && self.open_notes[track]
                    .iter()
                    .enumerate()
                    .any(|(n, &on)| on && n % 16 == usize::from(source))
            {
                return Err(Error::Busy);
            }
        }
        let matches = |e: &Event, pad: u8| {
            e.deck.is_none()
                && (if drum {
                    e.note % 16 == pad
                } else {
                    e.slice && e.note == pad
                })
        };
        let events = if recording {
            &self.capture_events
        } else {
            &self.tracks[track].events
        };
        let count = events.iter().filter(|e| matches(e, source)).count();
        if count == 0 {
            return Err(Error::NoContent);
        }
        if let Some(dest) = target {
            if events.iter().any(|e| matches(e, dest)) {
                return Err(Error::NotEmpty);
            }
            let limit = if recording {
                MAX_EVENTS - 128
            } else {
                MAX_EVENTS
            };
            if events.len() + count > limit {
                return Err(Error::Capacity);
            }
        }
        if !recording {
            self.history
                .checkpoint(track, &self.tracks[track], EditKind::Midi);
        }
        let events = if recording {
            &mut self.capture_events
        } else {
            &mut self.tracks[track].events
        };
        if let Some(dest) = target {
            let len = events.len();
            for i in 0..len {
                if matches(&events[i], source) {
                    let mut copy = events[i];
                    copy.note = dest;
                    events.push(copy);
                }
            }
            events.sort_unstable_by_key(|e| (e.tick, e.on));
        } else {
            events.retain(|e| !matches(e, source));
        }
        let changed = target.unwrap_or(source);
        if let Some(r) = &mut self.recorder {
            r.edited_pads |= 1 << changed;
            if drum {
                r.touched |= 1 << changed;
            }
            if target.is_none() {
                for note in 0..128 {
                    if (drum && note % 16 == usize::from(source))
                        || (!drum && note == usize::from(source))
                    {
                        self.open_notes[track][note] = false;
                        self.capture_velocities[note] = 0;
                    }
                }
            }
        } else {
            let t = &mut self.tracks[track];
            t.generated_sounds &= !(1 << changed);
            t.last_generated_sound = None;
            t.generated = t.generated_sounds != 0;
            t.revision = t.revision.wrapping_add(1);
            let phase = t.phase(self.clock.tick());
            t.cursor = t.events.partition_point(|e| e.tick <= phase);
            t.last_phase = phase;
        }
        // End only the edited sequenced lane; other pads and live input retain ownership.
        for v in &mut self.voices {
            if v.track == track && v.source == 1 && v.drum == Some(usize::from(changed)) {
                v.release();
            }
        }
        for v in &mut self.slice_voices {
            if v.track == track
                && v.pad == usize::from(changed)
                && v.sampler.is_some_and(|s| s.source == 1 && s.slice)
            {
                v.release = true;
            }
        }
        Ok(())
    }
}
