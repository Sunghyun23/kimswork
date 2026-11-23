use loop_club::{
    engine::{Command, Engine},
    model::*,
    project,
    runtime::Runtime,
    ui::{Snapshot, Ui},
};
use std::sync::atomic::Ordering;
fn core(kind: Kind) -> Engine {
    let mut e = Engine::new(8000, 240., 24000);
    e.preroll = 0;
    e.tracks[0].kind = kind;
    e
}
fn frames(e: &mut Engine, n: usize) {
    for _ in 0..n {
        e.process(Frame::mono(0.2));
    }
}
fn note(e: &mut Engine, on: bool) {
    e.command(Command::Note {
        track: 0,
        note: 60,
        velocity: if on { 80 } else { 0 },
        on,
        source: 2,
    })
    .unwrap();
}
#[test]
fn canceled_audio_replace_preserves_original_playback_frames_and_save_validity() {
    let mut e = core(Kind::Audio);
    e.tracks[0].length_ticks = 8 * PPQ;
    e.tracks[0].audio_frames = 16000;
    e.tracks[0].source_frames = 16000;
    e.tracks[0].audio[..16000].fill(Frame::mono(0.1));
    e.tracks[0].chops.divide(16000);
    e.command(Command::LoopRecord(0)).unwrap();
    frames(&mut e, 100);
    assert_eq!(
        e.tracks[0].audio_frames, 16000,
        "unfinished REPLACE must not shrink the old loop"
    );
    e.command(Command::Stop).unwrap();
    assert_eq!(e.tracks[0].audio_frames, 16000);
    assert_eq!(e.tracks[0].audio[15000], Frame::mono(0.1));
    let dir = std::env::temp_dir().join(format!("lc-audio-cancel-{}", std::process::id()));
    let _ = std::fs::remove_file(&dir);
    project::save(&e, &dir).unwrap();
    project::load(&dir, 8000, 24000).unwrap();
    std::fs::remove_file(dir).unwrap();
}
#[test]
fn held_synth_note_during_count_in_has_note_on_at_the_first_downbeat() {
    let mut e = core(Kind::Synth);
    e.preroll = 1;
    e.command(Command::LoopRecord(0)).unwrap();
    frames(&mut e, 100);
    note(&mut e, true);
    frames(&mut e, 8100);
    note(&mut e, false);
    assert!(
        e.visible_events(0)
            .iter()
            .any(|e| e.on && e.note == 60 && e.tick == 0),
        "a held count-in note must have an onset"
    );
}
#[test]
fn midi_target_after_automatic_handoff_and_finish_is_the_latest_instrument() {
    let r = Runtime::default();
    let mut e = Box::new(core(Kind::Synth));
    e.tracks[1].kind = Kind::Synth;
    r.initialize(e).ok().unwrap();
    r.active.store(true, Ordering::Release);
    r.enqueue(Command::LoopRecord(0));
    r.output(&mut [0; 256], 2, 8000);
    r.enqueue(Command::LoopRecord(1));
    for _ in 0..65 {
        r.output(&mut [0; 256], 2, 8000);
    }
    assert_eq!(r.snapshot.read().unwrap().words[9], 2);
    r.enqueue(Command::LoopRecord(1));
    for _ in 0..65 {
        r.output(&mut [0; 256], 2, 8000);
    }
    assert_eq!(r.snapshot.read().unwrap().words[9], 0);
    r.external_note(9, 72, 90, true);
    r.output(&mut [0; 256], 2, 8000);
    let s = r.snapshot.read().unwrap();
    assert_eq!(s.words[3], 1);
    assert!(
        s.words[1045] & (1 << 8) != 0,
        "post-record MIDI should sound on selected track1"
    );
    r.external_note(9, 72, 0, false);
}
#[test]
fn synth_chord_performance_mode_survives_project_reload() {
    let mut e = core(Kind::Synth);
    let mut ui = Ui::default();
    ui.page = Page::Play;
    ui.soft(1, false, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    let dir = std::env::temp_dir().join(format!("lc-mode-save-{}", std::process::id()));
    let _ = std::fs::remove_file(&dir);
    project::save(&e, &dir).unwrap();
    let loaded = project::load(&dir, 8000, 24000).unwrap();
    let mut restored_ui = Ui::default();
    restored_ui.sync_chord_modes(&Snapshot::capture(&loaded));
    restored_ui.sync_loop_role(&Snapshot::capture(&loaded));
    assert!(restored_ui.chord_modes[0]);
    std::fs::remove_file(dir).unwrap();
}

#[test]
fn rejected_audio_handoff_does_not_change_selected_recording_instrument() {
    let r = Runtime::default();
    let mut e = Box::new(core(Kind::Synth));
    e.tracks[1].kind = Kind::Audio;
    r.initialize(e).ok().unwrap();
    r.active.store(true, Ordering::Release);
    r.enqueue(Command::LoopRecord(0));
    r.output(&mut [0; 256], 2, 8000);
    r.enqueue(Command::LoopRecord(1));
    r.output(&mut [0; 256], 2, 8000);
    let s = r.snapshot.read().unwrap();
    assert_eq!(s.words[3], 0);
    assert_eq!(s.words[9], 1);
    assert_eq!((s.words[12] >> 8) & 255, 0);
    assert_eq!(s.words[7], Error::InputUnavailable as u64 + 1);
}
#[test]
fn count_in_released_synth_note_and_drum_hit_are_not_recorded_as_held_notes() {
    for kind in [Kind::Synth, Kind::Drum] {
        let mut e = core(kind);
        e.preroll = 1;
        e.command(Command::LoopRecord(0)).unwrap();
        frames(&mut e, 100);
        note(&mut e, true);
        if kind == Kind::Synth {
            note(&mut e, false);
        }
        frames(&mut e, 8100);
        assert!(e.visible_events(0).is_empty());
    }
}
#[test]
fn each_held_chord_tone_is_seeded_once_and_closed_before_the_recorded_loop_end() {
    let mut e = core(Kind::Synth);
    e.preroll = 1;
    e.command(Command::LoopRecord(0)).unwrap();
    frames(&mut e, 100);
    e.command(Command::Chord {
        track: 0,
        pad: 0,
        velocity: 80,
        on: true,
    })
    .unwrap();
    frames(&mut e, 8100);
    e.command(Command::LoopRecord(0)).unwrap();
    while e.recorder.is_some() {
        e.process(Frame::ZERO);
    }
    let events = &e.tracks[0].events;
    assert_eq!(events.iter().filter(|e| e.on).count(), 4);
    assert_eq!(events.iter().filter(|e| !e.on).count(), 4);
    assert!(events.iter().filter(|e| e.on).all(|e| e.tick == 0));
    assert!(events.iter().all(|v| v.tick < e.tracks[0].length_ticks));
}
#[test]
fn pad_note_off_reaches_its_original_owner_after_page_and_track_changes() {
    for page in [
        Page::Loop,
        Page::Play,
        Page::Grid,
        Page::Mix,
        Page::TrackFx,
        Page::Texture,
        Page::Source,
        Page::Setup,
        Page::Output,
        Page::Sidechain,
    ] {
        let mut e = core(Kind::Synth);
        let mut ui = Ui::default();
        ui.pad(4, true, 90, false, false, &Snapshot::capture(&e), |c| {
            e.command(c).unwrap();
        });
        ui.navigate(page, &Snapshot::capture(&e));
        ui.selected = 1;
        e.command(Command::Select(1)).unwrap();
        ui.pad(4, false, 0, true, false, &Snapshot::capture(&e), |c| {
            e.command(c).unwrap();
        });
        frames(&mut e, 8000);
        assert_eq!(e.active_voices(), 0, "note remained after {page:?}");
    }
}
#[test]
fn randomized_record_transport_and_edits_keep_bounded_valid_clip_state() {
    let mut e = core(Kind::Synth);
    e.tracks[1].kind = Kind::Drum;
    e.tracks[2].kind = Kind::Audio;
    let mut seed = 0x719ada_u64;
    for step in 0..4000 {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let track = ((seed >> 32) % 3) as usize;
        let c = match (seed >> 40) % 13 {
            0 => Command::LoopRecord(track),
            1 => Command::Stop,
            2 => Command::PlayAll,
            3 => Command::Undo,
            4 => Command::Redo,
            5 => Command::RecordMode(track, RecordMode::Overdub),
            6 => Command::RecordMode(track, RecordMode::Replace),
            7 => Command::Select(track),
            8 => Command::Clear(track),
            9 => Command::GenerateDrumPart {
                track,
                sound: step % 16,
            },
            _ => Command::Note {
                track,
                note: (48 + step % 12) as u8,
                velocity: if step % 2 == 0 { 80 } else { 0 },
                on: step % 2 == 0,
                source: 2,
            },
        };
        let _ = e.command(c);
        for _ in 0..32 {
            assert!(e.process(Frame::mono(0.1)).finite());
        }
        if let Some(next) = e.next_record {
            let r = e.recorder.expect("pending handoff without recorder");
            assert_ne!(next, r.track);
            assert!(r.stop_after_cycle);
        }
        for t in &e.tracks {
            assert!(t.audio_frames <= t.audio.len());
            assert!(t.source_frames <= t.audio_frames);
            assert!(t.events.len() <= 4096);
            if !e.recorder.is_some_and(|r| r.track == track) {
                assert!(t.events.windows(2).all(|w| w[0].tick <= w[1].tick));
            }
        }
    }
    e.command(Command::Stop).unwrap();
    frames(&mut e, 16000);
    assert_eq!(e.active_voices(), 0);
    assert!(e.recorder.is_none());
    assert!(e.next_record.is_none());
}
