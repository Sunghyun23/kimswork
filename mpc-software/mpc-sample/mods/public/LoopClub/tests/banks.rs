use loop_club::{
    bank::{self, Bank, DrumKit, Kit, KitJob, Manifest, Record},
    browser_device::{self, BrowserUi, FileWorker},
    engine::{Command, Engine},
    model::*,
    project,
    runtime::Runtime,
};
use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};
fn temp() -> PathBuf {
    static N: AtomicU64 = AtomicU64::new(0);
    let p = std::env::temp_dir().join(format!(
        "lc-bank-test-{}-{}",
        std::process::id(),
        N.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir_all(&p).unwrap();
    p
}
fn fixture() -> (PathBuf, Vec<u8>) {
    let root = temp();
    let path = root.join("fixture.lcbank");
    let frames = 800u32;
    let size = frames * 4;
    let mut wav = Vec::new();
    wav.extend(b"RIFF");
    wav.extend((36 + size).to_le_bytes());
    wav.extend(b"WAVEfmt ");
    wav.extend(16u32.to_le_bytes());
    wav.extend(1u16.to_le_bytes());
    wav.extend(2u16.to_le_bytes());
    wav.extend(8000u32.to_le_bytes());
    wav.extend(32000u32.to_le_bytes());
    wav.extend(4u16.to_le_bytes());
    wav.extend(16u16.to_le_bytes());
    wav.extend(b"data");
    wav.extend(size.to_le_bytes());
    for _ in 0..frames {
        wav.extend(12000i16.to_le_bytes());
        wav.extend((-6000i16).to_le_bytes());
    }
    let mut sounds = [None; 16];
    sounds[0] = Some(0);
    sounds[3] = Some(0);
    let m = Manifest {
        format: "LCBANK01".into(),
        id: "fixture".into(),
        name: "Fixture Bank".into(),
        redistributable: true,
        payload_bytes: (8 + wav.len()) as u64,
        records: vec![Record {
            id: "a".repeat(64),
            name: "Fixture Kick.wav".into(),
            offset: 8,
            bytes: wav.len(),
            crc: bank::crc32(&wav),
            kind: 0,
            role: 0,
            rate: 8000,
            channels: 2,
            frames,
        }],
        kits: vec![Kit {
            name: "Test Kit".into(),
            sounds,
            levels: [0.7; 16],
            tunes: [0.; 16],
        }],
    };
    let json = serde_json::to_vec(&m).unwrap();
    let mut file = b"LCBANK01".to_vec();
    file.extend((json.len() as u64).to_le_bytes());
    file.extend(json);
    file.extend(b"GDBANK01");
    file.extend(&wav);
    std::fs::write(&path, &file).unwrap();
    (path, file)
}
#[test]
fn selected_kit_decodes_stereo_pcm_and_replays_through_existing_mix() {
    let (path, _) = fixture();
    let b = Bank::open(&path).unwrap();
    let mut job = b.prepare(0, 0, 8000).unwrap();
    assert_eq!(job.kit.clips[0].len(), 800);
    assert!(job.kit.clips[0][0].l > 0.3);
    assert!(job.kit.clips[0][0].r < 0.);
    let mut e = Engine::new(8000, 240., 16000);
    e.tracks[0].kind = Kind::Drum;
    e.tracks[1].length_ticks = 4 * PPQ;
    e.tracks[1].playing = true;
    job.commit(&mut e);
    assert!(job.error.is_none());
    assert!(e.tracks[1].playing);
    assert_eq!(e.tracks[0].kit.name, "Test Kit");
    e.command(Command::Note {
        track: 0,
        note: 0,
        velocity: 100,
        on: true,
        source: 2,
    })
    .unwrap();
    let audio = e.process(Frame::ZERO);
    assert!(audio.l > 0. && audio.r < 0.);
    e.command(Command::Note {
        track: 0,
        note: 3,
        velocity: 100,
        on: true,
        source: 2,
    })
    .unwrap();
    e.command(Command::Note {
        track: 0,
        note: 2,
        velocity: 100,
        on: true,
        source: 2,
    })
    .unwrap();
    assert_eq!(e.active_voices(), 2);
    e.command(Command::Stop).unwrap();
    for _ in 0..900 {
        e.process(Frame::ZERO);
    }
    assert_eq!(e.active_voices(), 0);
    std::fs::remove_dir_all(path.parent().unwrap()).unwrap();
}
#[test]
fn bank_corruption_is_rejected_without_changing_existing_track() {
    let (path, mut bytes) = fixture();
    let b = Bank::open(&path).unwrap();
    if let Some(v) = bytes.last_mut() {
        *v ^= 1;
    }
    std::fs::write(&path, bytes).unwrap();
    assert!(b.prepare(0, 0, 8000).is_err());
    let mut e = Engine::new(8000, 240., 16000);
    e.tracks[0].kind = Kind::Synth;
    let mut job = KitJob {
        track: 0,
        rate: 8000,
        kit: Box::new(DrumKit::default()),
        drums: [Drum::default(); 16],
        target: None,
        error: None,
    };
    job.commit(&mut e);
    assert_eq!(job.error, Some(Error::WrongKind));
    assert_eq!(e.tracks[0].kind, Kind::Synth);
    std::fs::remove_dir_all(path.parent().unwrap()).unwrap();
}
#[test]
fn project_embeds_loaded_samples_and_reloads_without_bank_file() {
    let (path, _) = fixture();
    let mut e = Engine::new(8000, 240., 16000);
    e.tracks[0].kind = Kind::Drum;
    Bank::open(&path)
        .unwrap()
        .prepare(0, 0, 8000)
        .unwrap()
        .commit(&mut e);
    let project_path = path.with_extension("lcproj");
    project::save(&e, &project_path).unwrap();
    std::fs::remove_file(&path).unwrap();
    let loaded = project::load(&project_path, 8000, 16000).unwrap();
    assert_eq!(loaded.tracks[0].kit.clips[0], e.tracks[0].kit.clips[0]);
    assert_eq!(loaded.tracks[0].kit.id, "fixture:0");
    std::fs::remove_dir_all(path.parent().unwrap()).unwrap();
}
#[test]
fn background_browser_selects_bank_then_kit() {
    let (source, _) = fixture();
    let root = source.parent().unwrap();
    let folder = root.join("LoopClub/Expansions");
    std::fs::create_dir_all(&folder).unwrap();
    std::fs::rename(&source, folder.join("fixture.lcbank")).unwrap();
    std::fs::write(
        folder.join("._fixture.lcbank"),
        b"AppleDouble, not a sound bank",
    )
    .unwrap();
    std::fs::write(folder.join(".hidden.lcbank"), b"hidden non-bank").unwrap();
    browser_device::initialize();
    let r = Runtime::default();
    let mut e = Box::new(Engine::new(8000, 240., 16000));
    e.tracks[0].kind = Kind::Drum;
    r.initialize(e).ok().unwrap();
    r.active.store(true, Ordering::Release);
    r.output(&mut [0; 128], 2, 8000);
    let mut worker = FileWorker::with_root(Some(root.into()), 8000, 16000);
    let mut ui = BrowserUi::default();
    ui.start_kind(0, true);
    worker.tick(&r);
    ui.refresh();
    ui.button(13);
    worker.tick(&r);
    ui.refresh();
    ui.button(13);
    worker.tick(&r);
    r.output(&mut [0; 128], 2, 8000);
    assert_eq!(worker.tick(&r), Some(true));
    r.with_engine(|e| {
        assert_eq!(e.as_ref().unwrap().tracks[0].kit.name, "Test Kit");
    });
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn bank_sample_replaces_only_selected_layer_and_roundtrips() {
    let (path, _) = fixture();
    let bank = Bank::open(&path).unwrap();
    let mut e = Engine::new(8000, 240., 16000);
    e.tracks[0].kind = Kind::Drum;
    e.tracks[0].kit.clips[0] = vec![Frame::mono(0.25); 100];
    let original = e.tracks[0].kit.clips[0].clone();
    let mut job = bank.prepare_layer(0, 0, 0, 1, 8000).unwrap();
    job.commit(&mut e);
    assert!(job.error.is_none());
    assert_eq!(e.tracks[0].kit.clips[0], original);
    assert_eq!(e.tracks[0].kit.secondary[0].len(), 800);
    assert!(e.tracks[0].drums[0].layers[1].enabled);
    assert_eq!(e.tracks[0].kit.names[0][1], "Fixture Kick");
    let saved = path.with_extension("lcproj");
    project::save(&e, &saved).unwrap();
    let loaded = project::load(&saved, 8000, 16000).unwrap();
    assert_eq!(
        loaded.tracks[0].kit.secondary[0],
        e.tracks[0].kit.secondary[0]
    );
    std::fs::remove_dir_all(path.parent().unwrap()).unwrap();
}
