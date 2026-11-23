//! One bounded clip-edit history. PCM changes rotate preallocated buffers.
use super::*;
#[derive(Clone, Copy)]
pub(super) struct ClipState {
    length: u64,
    frames: usize,
    source: usize,
    source_ticks: u64,
    audio_sequence: bool,
    origin: u64,
    playing: bool,
    generated: bool,
    generated_sounds: u16,
    pattern: crate::music::PatternConfig,
    last_generated: Option<usize>,
    chops: crate::chop::Chops,
    wave: (u32, u32),
}
impl ClipState {
    pub(super) fn read(t: &Track) -> Self {
        Self {
            length: t.length_ticks,
            frames: t.audio_frames,
            source: t.source_frames,
            source_ticks: t.source_ticks,
            audio_sequence: t.audio_sequence,
            origin: t.origin,
            playing: t.playing,
            generated: t.generated,
            generated_sounds: t.generated_sounds,
            pattern: t.pattern,
            last_generated: t.last_generated_sound,
            chops: t.chops,
            wave: t.wave_view,
        }
    }
    fn write(self, t: &mut Track) {
        t.length_ticks = self.length;
        t.audio_frames = self.frames;
        t.source_frames = self.source;
        t.source_ticks = self.source_ticks;
        t.audio_sequence = self.audio_sequence;
        t.origin = self.origin;
        t.playing = self.playing;
        t.generated = self.generated;
        t.generated_sounds = self.generated_sounds;
        t.pattern = self.pattern;
        t.last_generated_sound = self.last_generated;
        t.chops = self.chops;
        t.wave_view = self.wave;
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum EditKind {
    Midi,
    Audio,
    Chop,
    ChopMidi,
    TrackCopy,
    TrackMove,
}
pub(super) struct History {
    pub(super) track: Option<Box<Track>>,
    pub(super) peer: Option<usize>,
    pub(super) target: Option<usize>,
    pub(super) undone: bool,
    pub(super) kind: EditKind,
    pub(super) state: Option<ClipState>,
    pub(super) audio: Vec<Frame>,
    pub(super) features: Box<crate::features::Features>,
    pub(super) events: Vec<Event>,
}
impl History {
    pub(super) fn new(capacity: usize) -> Self {
        Self {
            track: None,
            peer: None,
            target: None,
            undone: false,
            kind: EditKind::Midi,
            state: None,
            audio: vec![Frame::ZERO; capacity],
            features: Box::new(crate::features::Features::default()),
            events: Vec::with_capacity(MAX_EVENTS),
        }
    }
    pub(super) fn checkpoint(&mut self, i: usize, t: &Track, kind: EditKind) {
        self.peer = None;
        self.target = Some(i);
        self.undone = false;
        self.kind = kind;
        self.state = Some(ClipState::read(t));
        if kind != EditKind::Chop {
            self.events.clear();
            self.events.extend_from_slice(&t.events);
        }
    }
    pub(super) fn invalidate(&mut self, i: usize) {
        if self.target == Some(i) || self.peer == Some(i) {
            self.target = None;
        }
    }
}
impl Engine {
    pub fn history_status(&self) -> u64 {
        self.history.target.map_or(0, |i| {
            (if self.history.undone { 2 } else { 1 }) | ((i as u64 + 1) << 8)
        })
    }
    pub(super) fn swap_history(&mut self, redo: bool) -> Result<(), Error> {
        if self.recorder.is_some() {
            if redo {
                return Err(Error::Busy);
            }
            let committed = self.record_history_committed;
            self.cancel_capture();
            self.loop_revision = self.loop_revision.wrapping_add(1);
            if !committed {
                return Ok(());
            }
        }
        let i = self
            .history
            .target
            .ok_or(if redo { Error::NoRedo } else { Error::NoUndo })?;
        if self.history.undone != redo {
            return Err(if redo { Error::NoRedo } else { Error::NoUndo });
        }
        if matches!(self.history.kind, EditKind::TrackCopy | EditKind::TrackMove) {
            if !self.is_idle() || self.chain.running || self.held_fx != 0 {
                return Err(Error::Busy);
            }
            let peer = self.history.peer.ok_or(Error::NoUndo)?;
            if self.history.kind == EditKind::TrackMove {
                self.tracks.swap(i, peer);
                self.track_fx.swap(i, peer);
                self.remap_track_references(i, peer);
            } else {
                let old = self.history.track.as_mut().ok_or(Error::NoUndo)?;
                std::mem::swap(&mut self.tracks[i], old.as_mut());
            }
            for index in [i, peer] {
                let t = &mut self.tracks[index];
                t.playing = false;
                t.reset_audio_player();
                t.revision = t.revision.wrapping_add(1);
            }
            self.history.undone = !redo;
            self.loop_revision = self.loop_revision.wrapping_add(1);
            return Ok(());
        }
        if self.chain_protects(i) {
            return Err(Error::Busy);
        }
        self.release_track(i);
        self.clear_slice_voices(i);
        let t = &mut self.tracks[i];
        t.audio_player.reset();
        let old = ClipState::read(t);
        let saved = self.history.state.expect("history target has a state");
        match self.history.kind {
            EditKind::TrackCopy | EditKind::TrackMove => unreachable!("handled above"),
            EditKind::Chop | EditKind::ChopMidi => {
                t.chops = saved.chops;
                t.source_frames = saved.source;
                if self.history.kind == EditKind::ChopMidi {
                    std::mem::swap(&mut t.events, &mut self.history.events);
                }
            }
            EditKind::Midi => {
                std::mem::swap(&mut t.events, &mut self.history.events);
                saved.write(t);
            }
            EditKind::Audio => {
                std::mem::swap(&mut t.audio, &mut self.history.audio);
                std::mem::swap(&mut t.features, &mut self.history.features);
                std::mem::swap(&mut t.events, &mut self.history.events);
                saved.write(t);
            }
        }
        self.history.state = Some(old);
        self.history.undone = !redo;
        if !matches!(self.history.kind, EditKind::Chop | EditKind::ChopMidi) {
            t.playing = self.running && !t.empty() && saved.playing;
        }
        let phase = t.phase(self.clock.tick());
        t.cursor = t.events.partition_point(|e| e.tick < phase);
        t.last_phase = phase;
        t.revision = t.revision.wrapping_add(1);
        self.loop_revision = self.loop_revision.wrapping_add(1);
        Ok(())
    }
    pub(super) fn checkpoint_clear(&mut self, i: usize) {
        let t = &mut self.tracks[i];
        let kind = if t.kind == Kind::Audio {
            EditKind::Audio
        } else {
            EditKind::Midi
        };
        self.history.checkpoint(i, t, kind);
        if kind == EditKind::Audio {
            std::mem::swap(&mut t.audio, &mut self.history.audio);
            std::mem::swap(&mut t.features, &mut self.history.features);
            self.history.events.clear();
            self.history.events.extend_from_slice(&t.events);
        }
    }
}
