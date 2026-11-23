use loop_club::{
    engine::{Command, Engine},
    model::*,
    runtime::Runtime,
};
use std::sync::atomic::Ordering;
use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::Cell,
};
thread_local! {static TRACK:Cell<bool>=const{Cell::new(false)};static COUNT:Cell<usize>=const{Cell::new(0)};}
struct Alloc;
unsafe impl GlobalAlloc for Alloc {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        let _ = TRACK.try_with(|t| {
            if t.get() {
                COUNT.with(|c| c.set(c.get() + 1))
            }
        });
        unsafe { System.alloc(l) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        let _ = TRACK.try_with(|t| {
            if t.get() {
                COUNT.with(|c| c.set(c.get() + 1))
            }
        });
        unsafe { System.dealloc(p, l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        let _ = TRACK.try_with(|t| {
            if t.get() {
                COUNT.with(|c| c.set(c.get() + 1))
            }
        });
        unsafe { System.realloc(p, l, n) }
    }
}
#[global_allocator]
static ALLOC: Alloc = Alloc;
#[test]
fn callback_commands_capture_and_snapshot_do_not_allocate() {
    let r = Box::new(Runtime::default());
    let mut e = Box::new(Engine::new(8000, 240., 16000));
    e.tracks[0].kind = Kind::Synth;
    r.initialize(e).ok().unwrap();
    r.active.store(true, Ordering::Release);
    r.output(&mut [0; 256], 2, 8000);
    let mut block = [0; 256];
    COUNT.with(|c| c.set(0));
    TRACK.with(|t| t.set(true));
    for _ in 0..20 {
        r.enqueue(Command::Note {
            track: 0,
            note: 60,
            velocity: 90,
            on: true,
            source: 2,
        });
        for _ in 0..128 {
            r.capture(Frame::ZERO, 8000)
        }
        r.output(&mut block, 2, 8000);
        r.enqueue(Command::Note {
            track: 0,
            note: 60,
            velocity: 0,
            on: false,
            source: 2,
        });
        r.output(&mut block, 2, 8000);
    }
    TRACK.with(|t| t.set(false));
    assert_eq!(COUNT.with(Cell::get), 0);
}

#[test]
fn generation_chords_chops_and_import_commit_do_not_allocate_or_free() {
    let r = Box::new(Runtime::default());
    let mut e = Box::new(Engine::new(8000, 240., 16000));
    e.tracks[0].kind = Kind::Drum;
    e.tracks[1].kind = Kind::Synth;
    e.tracks[1].synth.lfo_pan = 1.;
    e.sidechain.depth = 0.7;
    e.texture.enabled = [true; 5];
    e.tracks[1].synth.lfo_pitch = 120.;
    e.tracks[1].synth.lfo_filter = 2.;
    e.tracks[1].synth.lfo_volume = 0.6;
    let mut audio = vec![Frame::ZERO; 16000];
    audio[..8000].fill(Frame::mono(0.1));
    let job = Box::new(loop_club::sample::ImportJob {
        features: Box::new(loop_club::features::Features::default()),
        track: 2,
        rate: 8000,
        bpm: 240.,
        audio,
        frames: 8000,
        source_frames: 8000,
        ticks: 4 * PPQ,
        name: "Fixture".into(),
        error: None,
    });
    r.initialize(e).ok().unwrap();
    r.active.store(true, Ordering::Release);
    r.output(&mut [0; 128], 2, 8000);
    r.submit_import(job).ok().unwrap();
    COUNT.with(|c| c.set(0));
    TRACK.with(|t| t.set(true));
    r.enqueue(Command::GenerateDrumPart { track: 0, sound: 0 });
    r.enqueue(Command::Chord {
        track: 1,
        pad: 0,
        velocity: 80,
        on: true,
    });
    r.output(&mut [0; 128], 2, 8000);
    r.enqueue(Command::Slice {
        track: 2,
        pad: 0,
        velocity: 80,
        on: true,
    });
    r.output(&mut [0; 128], 2, 8000);
    TRACK.with(|t| t.set(false));
    assert_eq!(COUNT.with(Cell::get), 0);
    drop(r.collect_import());
}

#[test]
fn replace_overdub_wrap_and_mode_commands_do_not_allocate_or_free() {
    for kind in [Kind::Audio, Kind::Synth, Kind::Drum] {
        let mut e = Box::new(Engine::new(8000, 240., 16000));
        e.preroll = 0;
        e.tracks[0].kind = kind;
        e.command(Command::Record(0)).unwrap();
        COUNT.with(|c| c.set(0));
        TRACK.with(|t| t.set(true));
        for frame in 0..25000 {
            if frame == 100 {
                e.command(Command::Record(0)).unwrap();
            }
            if frame == 9000 {
                e.command(Command::RecordMode(0, RecordMode::Overdub))
                    .unwrap();
            }
            if frame % 1000 == 500 && kind != Kind::Audio {
                e.command(Command::Note {
                    track: 0,
                    note: if kind == Kind::Drum { 1 } else { 60 },
                    velocity: 80,
                    on: true,
                    source: 2,
                })
                .unwrap();
                e.command(Command::Note {
                    track: 0,
                    note: if kind == Kind::Drum { 1 } else { 60 },
                    velocity: 0,
                    on: false,
                    source: 2,
                })
                .unwrap();
            }
            e.process(Frame::mono(0.1));
        }
        TRACK.with(|t| t.set(false));
        assert_eq!(COUNT.with(Cell::get), 0, "{kind:?}");
    }
}

#[test]
fn kit_swap_playback_and_snapshot_do_not_allocate_or_free() {
    let r = Runtime::default();
    let mut e = Box::new(Engine::new(8000, 240., 16000));
    e.tracks[0].kind = Kind::Drum;
    r.initialize(e).ok().unwrap();
    r.active.store(true, Ordering::Release);
    let mut kit = Box::new(loop_club::bank::DrumKit::default());
    kit.name = "Fixture Kit".into();
    kit.id = "fixture:0".into();
    kit.clips[0] = vec![Frame { l: 0.2, r: 0.1 }; 800];
    kit.secondary[0] = vec![Frame::mono(0.1); 400];
    let mut drums = [Drum::default(); 16];
    drums[0].layers[1].enabled = true;
    let job = Box::new(loop_club::bank::KitJob {
        track: 0,
        rate: 8000,
        kit,
        drums,
        target: None,
        error: None,
    });
    r.submit_kit(job).ok().unwrap();
    COUNT.with(|c| c.set(0));
    TRACK.with(|t| t.set(true));
    r.output(&mut [0; 128], 2, 8000);
    r.enqueue(Command::Note {
        track: 0,
        note: 0,
        velocity: 90,
        on: true,
        source: 2,
    });
    for _ in 0..20 {
        r.output(&mut [0; 128], 2, 8000);
    }
    TRACK.with(|t| t.set(false));
    assert_eq!(COUNT.with(Cell::get), 0);
    assert!(r.collect_kit().unwrap().error.is_none());
}

#[test]
fn repeat_arming_and_live_hardware_volume_changes_do_not_allocate() {
    let r = Runtime::default();
    let mut e = Box::new(Engine::new(8000, 240., 8000));
    e.tracks[0].kind = Kind::Synth;
    e.performance[0] = Fx {
        effect: Effect::Repeat,
        a: 0.,
        b: 0.1,
        mix: 1.,
        ..Fx::default()
    };
    assert!(r.initialize(e).is_ok());
    r.active.store(true, Ordering::Release);
    let mut block = [0; 256];
    block.fill(0);
    r.output(&mut block, 2, 8000);
    COUNT.with(|c| c.set(0));
    TRACK.with(|t| t.set(true));
    for _ in 0..10 {
        block.fill(0);
        r.output(&mut block, 2, 8000);
    }
    r.enqueue(Command::HardwareVolume(0.6));
    r.enqueue(Command::Note {
        track: 0,
        note: 60,
        velocity: 90,
        on: true,
        source: 1,
    });
    let mut peak = 0i32;
    for _ in 0..100 {
        block.fill(0);
        r.output(&mut block, 2, 8000);
        peak = peak.max(block.iter().map(|s| s.abs()).max().unwrap());
    }
    r.enqueue(Command::HardwareVolume(0.));
    block.fill(0);
    r.output(&mut block, 2, 8000);
    r.enqueue(Command::HardwareVolume(1.));
    block.fill(0);
    r.output(&mut block, 2, 8000);
    TRACK.with(|t| t.set(false));
    assert_eq!(COUNT.with(Cell::get), 0);
    assert!(peak > 1000);
}

#[test]
fn external_midi_stays_with_recording_track_even_when_mixer_selects_another() {
    let r = Box::new(Runtime::default());
    let mut e = Box::new(Engine::new(8000, 240., 40000));
    e.preroll = 0;
    e.tracks[0].kind = Kind::Synth;
    e.tracks[1].kind = Kind::Synth;
    r.initialize(e).ok().unwrap();
    r.active.store(true, Ordering::Release);
    let mut block = [0; 256];
    r.enqueue(Command::LoopRecord(0));
    r.output(&mut block, 2, 8000);
    r.enqueue(Command::Select(1));
    r.output(&mut block, 2, 8000);
    assert!(r.external_note(99, 60, 90, true));
    r.output(&mut block, 2, 8000);
    let s = r.snapshot.read().unwrap();
    assert_eq!(s.words[22], 1);
    assert_eq!(s.words[34], 0);
    assert!(r.external_note(99, 60, 0, false));
    r.output(&mut block, 2, 8000);
    assert_eq!(r.snapshot.read().unwrap().words[22], 2);
}

#[test]
fn partial_overdub_tail_preparation_and_commit_do_not_allocate() {
    let mut e = Box::new(Engine::new(8000, 240., 40000));
    e.preroll = 0;
    let t = &mut e.tracks[0];
    t.audio_frames = 32000;
    t.source_frames = 32000;
    t.length_ticks = 16 * PPQ;
    t.record_mode = RecordMode::Overdub;
    for f in &mut t.audio[..32000] {
        *f = Frame::mono(0.1);
        t.features.feed(*f, 8000);
    }
    COUNT.with(|c| c.set(0));
    TRACK.with(|t| t.set(true));
    e.command(Command::LoopRecord(0)).unwrap();
    for _ in 0..1000 {
        e.process(Frame::mono(0.2));
    }
    e.command(Command::LoopRecord(0)).unwrap();
    for _ in 0..7001 {
        e.process(Frame::mono(0.2));
    }
    TRACK.with(|t| t.set(false));
    assert_eq!(COUNT.with(Cell::get), 0);
    assert!(e.recorder.is_none());
    assert_eq!(e.tracks[0].audio_frames, 32000);
}

#[test]
fn chain_boundaries_and_commands_do_not_allocate() {
    let mut e = Box::new(Engine::new(8000, 240., 16000));
    for i in 0..2 {
        e.tracks[i].kind = Kind::Synth;
        e.tracks[i].length_ticks = 4 * PPQ;
    }
    COUNT.with(|c| c.set(0));
    TRACK.with(|t| t.set(true));
    e.command(Command::ChainAppend(0)).unwrap();
    e.command(Command::ChainAppend(1)).unwrap();
    e.command(Command::ChainToggle).unwrap();
    for _ in 0..24001 {
        e.process(Frame::ZERO);
    }
    e.command(Command::ChainClear).unwrap();
    TRACK.with(|t| t.set(false));
    assert_eq!(COUNT.with(|c| c.get()), 0);
}

#[test]
fn modulation_effects_and_parameter_changes_allocate_nothing() {
    let mut e = Box::new(Engine::new(8000, 120., 16000));
    COUNT.with(|c| c.set(0));
    TRACK.with(|t| t.set(true));
    for effect in [
        Effect::Tremolo,
        Effect::AutoPan,
        Effect::AutoFilter,
        Effect::Wobble,
    ] {
        e.command(Command::FxSelect {
            target: 1,
            track: 0,
            slot: 0,
            effect,
            temporary: false,
        })
        .unwrap();
        for (index, value) in [(3, 1.), (4, 0.5), (5, 0.8)] {
            e.command(Command::FxValue {
                target: 1,
                track: 0,
                slot: 0,
                index,
                value,
            })
            .unwrap();
        }
        for _ in 0..8000 {
            e.process(Frame::ZERO);
        }
    }
    TRACK.with(|t| t.set(false));
    assert_eq!(COUNT.with(|c| c.get()), 0);
}

#[test]
fn record_history_rotation_undo_redo_and_clear_do_not_allocate_or_free() {
    for kind in [Kind::Audio, Kind::Synth, Kind::Drum] {
        let mut e = Box::new(Engine::new(8000, 240., 16000));
        e.preroll = 0;
        e.tracks[0].kind = kind;
        COUNT.with(|c| c.set(0));
        TRACK.with(|t| t.set(true));
        for _ in 0..2 {
            e.command(Command::LoopRecord(0)).unwrap();
            let start = e.clock.frame_at(e.recorder.unwrap().start);
            while e.clock.frame <= start + 100 {
                e.process(Frame::mono(0.1));
            }
            e.command(Command::LoopRecord(0)).unwrap();
            while e.recorder.is_some() {
                e.process(Frame::mono(0.1));
            }
            e.command(Command::Undo).unwrap();
            e.command(Command::Redo).unwrap();
        }
        e.command(Command::Clear(0)).unwrap();
        e.command(Command::Undo).unwrap();
        e.command(Command::Redo).unwrap();
        TRACK.with(|t| t.set(false));
        assert_eq!(COUNT.with(Cell::get), 0, "{kind:?}");
    }
}

#[test]
fn recording_handoffs_rotate_buffers_without_allocation_or_deallocation() {
    let mut e = Box::new(Engine::new(8000, 240., 16000));
    e.preroll = 0;
    e.tracks[1].kind = Kind::Synth;
    e.tracks[2].kind = Kind::Drum;
    e.command(Command::LoopRecord(0)).unwrap();
    COUNT.with(|c| c.set(0));
    TRACK.with(|t| t.set(true));
    for frame in 0..17000 {
        if frame == 100 {
            e.command(Command::LoopRecord(1)).unwrap();
        }
        if frame == 8100 {
            e.command(Command::LoopRecord(2)).unwrap();
        }
        e.process(Frame::mono(0.1));
    }
    TRACK.with(|t| t.set(false));
    assert_eq!(COUNT.with(Cell::get), 0);
    assert_eq!(e.recorder.unwrap().track, 2);
}

#[test]
fn chromatic_chop_warp_capture_history_and_snapshot_have_no_callback_allocations_or_frees() {
    use loop_club::audio::{RecordTarget, Settings};
    let mut e = Engine::new(8000, 240., 32000);
    e.preroll = 0;
    let t = &mut e.tracks[0];
    t.audio_frames = 8000;
    t.source_frames = 8000;
    t.source_ticks = 4 * PPQ;
    t.length_ticks = 4 * PPQ;
    t.audio[..8000].fill(Frame::mono(0.1));
    t.chops.divide(8000);
    COUNT.with(|c| c.set(0));
    TRACK.with(|t| t.set(true));
    for target in [RecordTarget::Chromatic, RecordTarget::Chop] {
        e.command(Command::AudioTarget(0, target)).unwrap();
        e.command(Command::AudioSettings(
            0,
            Settings {
                warp: true,
                ..Default::default()
            },
        ))
        .unwrap();
        e.command(Command::LoopRecord(0)).unwrap();
        let start = e.recorder.unwrap().start;
        while e.clock.tick() < start {
            e.process(Frame::ZERO);
        }
        for note in if target == RecordTarget::Chop {
            [0, 1, 2, 3]
        } else {
            [48, 55, 60, 64]
        } {
            e.command(Command::Note {
                track: 0,
                note,
                velocity: 85,
                on: true,
                source: 100 + u16::from(note),
            })
            .unwrap();
            for _ in 0..100 {
                e.process(Frame::ZERO);
            }
            e.command(Command::Note {
                track: 0,
                note,
                velocity: 0,
                on: false,
                source: 100 + u16::from(note),
            })
            .unwrap();
        }
        e.command(Command::LoopRecord(0)).unwrap();
        while e.recorder.is_some() {
            e.process(Frame::ZERO);
            let _ = loop_club::ui::Snapshot::capture(&e);
        }
        e.command(Command::Undo).unwrap();
        e.command(Command::Redo).unwrap();
        e.command(Command::Stop).unwrap();
        for _ in 0..100 {
            e.process(Frame::ZERO);
        }
    }
    TRACK.with(|t| t.set(false));
    assert_eq!(COUNT.with(Cell::get), 0);
}

#[test]
fn xpm_four_zones_playback_choke_and_replacement_do_not_allocate_or_free() {
    use loop_club::{
        bank::{DrumKit, KitJob},
        xpm::Data,
    };
    let mut e = Engine::new(8000, 100., 16000);
    e.tracks[0].kind = Kind::Drum;
    let mut kit = Box::new(DrumKit::default());
    kit.xpm = Some(Box::new(Data::default()));
    for d in 0..2 {
        kit.clips[d] = vec![Frame::mono(0.03); 800];
        let x = kit.xpm.as_mut().unwrap();
        x.meta.pads[d].count = 4;
        x.meta.pads[d].mute_group = 1;
        x.meta.pads[d].gate = true;
        for z in 0..3 {
            x.extra[d][z] = vec![Frame::mono(0.03); 800];
        }
    }
    let mut job = KitJob {
        track: 0,
        rate: 8000,
        kit,
        drums: [Drum::default(); 16],
        target: None,
        error: None,
    };
    let mut replacement = KitJob {
        track: 0,
        rate: 8000,
        kit: Box::new(DrumKit {
            xpm: Some(Box::new(Data::default())),
            ..DrumKit::default()
        }),
        drums: [Drum::default(); 16],
        target: Some((0, 0)),
        error: None,
    };
    replacement.kit.clips[0] = vec![Frame::mono(0.1); 400];
    COUNT.with(|c| c.set(0));
    TRACK.with(|t| t.set(true));
    job.commit(&mut e);
    for n in [0, 1, 0] {
        e.command(Command::Note {
            track: 0,
            note: n,
            velocity: 100,
            on: true,
            source: 2,
        })
        .unwrap();
        for _ in 0..100 {
            let _ = e.process(Frame::ZERO);
        }
        e.command(Command::Note {
            track: 0,
            note: n,
            velocity: 0,
            on: false,
            source: 2,
        })
        .unwrap();
        for _ in 0..50 {
            let _ = e.process(Frame::ZERO);
        }
    }
    replacement.commit(&mut e);
    TRACK.with(|t| t.set(false));
    assert_eq!(COUNT.with(Cell::get), 0);
    assert!(job.error.is_none() && replacement.error.is_none());
    assert_eq!(e.tracks[0].kit.xpm.as_ref().unwrap().meta.pads[0].count, 0);
    assert_eq!(replacement.kit.xpm.as_ref().unwrap().extra[0][2].len(), 800);
}

#[test]
fn full_track_copy_move_undo_redo_swap_without_callback_allocation_or_free() {
    let mut e = Engine::new(8000, 100., 16000);
    e.tracks[0].kind = Kind::Audio;
    e.tracks[0].audio_frames = 800;
    e.tracks[0].source_frames = 800;
    e.tracks[0].length_ticks = 4 * PPQ;
    e.tracks[0].audio[..800].fill(Frame::mono(0.2));
    e.transfer_track_stopped(0, 1, false).unwrap();
    COUNT.with(|c| c.set(0));
    TRACK.with(|t| t.set(true));
    e.command(Command::Undo).unwrap();
    e.command(Command::Redo).unwrap();
    TRACK.with(|t| t.set(false));
    assert_eq!(COUNT.with(Cell::get), 0);
    e.transfer_track_stopped(1, 2, true).unwrap();
    COUNT.with(|c| c.set(0));
    TRACK.with(|t| t.set(true));
    e.command(Command::Undo).unwrap();
    e.command(Command::Redo).unwrap();
    TRACK.with(|t| t.set(false));
    assert_eq!(COUNT.with(Cell::get), 0);
}
#[test]
fn mono_glide_strum_and_trigger_commands_do_not_allocate_or_free() {
    let mut e = Box::new(Engine::new(8000, 240., 16000));
    e.tracks[0].kind = Kind::Synth;
    e.tracks[0].play.voice = 2;
    e.tracks[0].play.glide_ms = 200.;
    e.tracks[1].kind = Kind::Synth;
    e.tracks[1].chord.strum_ms = 20.;
    COUNT.with(|c| c.set(0));
    TRACK.with(|t| t.set(true));
    for i in 0..16 {
        e.command(Command::Note {
            track: 0,
            note: 48 + i,
            velocity: 90,
            on: true,
            source: 2 + i as u16,
        })
        .unwrap();
        for _ in 0..64 {
            e.process(Frame::ZERO);
        }
        e.command(Command::Note {
            track: 0,
            note: 48 + i,
            velocity: 0,
            on: false,
            source: 2 + i as u16,
        })
        .unwrap();
    }
    e.command(Command::Chord {
        track: 1,
        pad: 0,
        velocity: 90,
        on: true,
    })
    .unwrap();
    for _ in 0..500 {
        e.process(Frame::ZERO);
    }
    e.command(Command::Chord {
        track: 1,
        pad: 0,
        velocity: 0,
        on: false,
    })
    .unwrap();
    e.command(Command::PlayValue {
        track: 0,
        index: 2,
        value: 2.,
    })
    .unwrap();
    e.command(Command::Stop).unwrap();
    TRACK.with(|t| t.set(false));
    assert_eq!(COUNT.with(Cell::get), 0);
}
#[test]
fn swing_reschedule_and_turntable_capture_never_allocate_or_free() {
    let mut e = Engine::new(8000, 240., 16000);
    e.preroll = 0;
    let t = &mut e.tracks[0];
    t.source_frames = 8000;
    t.audio_frames = 8000;
    t.source_ticks = 4 * PPQ;
    t.length_ticks = 4 * PPQ;
    t.audio_target = loop_club::audio::RecordTarget::Turntable;
    t.audio[..8000].fill(Frame::mono(0.2));
    e.tracks[1].kind = Kind::Drum;
    COUNT.with(|c| c.set(0));
    TRACK.with(|t| t.set(true));
    e.command(Command::Swing(Some(loop_club::swing::Basis {
        amount: 66.,
        sixteenth: false,
    })))
    .unwrap();
    e.command(Command::GenerateDrumPart { track: 1, sound: 2 })
        .unwrap();
    e.process(Frame::ZERO);
    e.command(Command::Stop).unwrap();
    e.command(Command::LoopRecord(0)).unwrap();
    e.process(Frame::ZERO);
    e.command(Command::DeckGrab(0, true, Some(0.5))).unwrap();
    for n in 0..200 {
        e.command(Command::DeckFader(0, (n % 50) as f32 / 50.))
            .unwrap();
        for _ in 0..20 {
            e.process(Frame::ZERO);
        }
    }
    e.command(Command::DeckGrab(0, false, None)).unwrap();
    e.command(Command::LoopRecord(0)).unwrap();
    for _ in 0..8100 {
        e.process(Frame::ZERO);
    }
    e.command(Command::Stop).unwrap();
    let _ = loop_club::ui::Snapshot::capture(&e);
    e.command(Command::Undo).unwrap();
    e.command(Command::Redo).unwrap();
    TRACK.with(|t| t.set(false));
    assert_eq!(COUNT.with(Cell::get), 0);
}

#[test]
fn full_scratch_overdub_refuses_seed_without_growing_or_mutating_completed_clip() {
    let mut e = Engine::new(8000, 240., 16000);
    e.preroll = 0;
    let t = &mut e.tracks[0];
    t.source_frames = 8000;
    t.audio_frames = 8000;
    t.source_ticks = 4 * PPQ;
    t.length_ticks = 4 * PPQ;
    t.audio_sequence = true;
    t.audio_target = loop_club::audio::RecordTarget::Turntable;
    t.record_mode = RecordMode::Overdub;
    for tick in 0..loop_club::engine::MAX_EVENTS {
        t.events.push(Event {
            deck: Some(Default::default()),
            timing: None,
            slice: false,
            tick: tick as u64,
            note: 0,
            velocity: 0,
            on: false,
        });
    }
    e.command(Command::DeckGrab(0, true, Some(0.5))).unwrap();
    COUNT.with(|c| c.set(0));
    TRACK.with(|t| t.set(true));
    e.command(Command::LoopRecord(0)).unwrap();
    e.process(Frame::ZERO);
    TRACK.with(|t| t.set(false));
    assert_eq!(COUNT.with(Cell::get), 0);
    assert!(e.recorder.is_none());
    assert_eq!(e.error, Some(Error::Capacity));
    assert_eq!(e.tracks[0].events.len(), loop_club::engine::MAX_EVENTS);
    assert_eq!(e.tracks[0].source_frames, 8000);
}
