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
