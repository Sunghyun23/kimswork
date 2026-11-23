use loop_club::{
    engine::{Command, Engine},
    model::*,
    runtime::Runtime,
};
use std::sync::atomic::Ordering;
fn wav(path: &std::path::Path, rate: u32, frames: u32) {
    let mut bytes = Vec::new();
    let size = frames * 2;
    bytes.extend(b"RIFF");
    bytes.extend((36 + size).to_le_bytes());
    bytes.extend(b"WAVEfmt ");
    bytes.extend(16u32.to_le_bytes());
    bytes.extend(1u16.to_le_bytes());
    bytes.extend(1u16.to_le_bytes());
    bytes.extend(rate.to_le_bytes());
    bytes.extend((rate * 2).to_le_bytes());
    bytes.extend(2u16.to_le_bytes());
    bytes.extend(16u16.to_le_bytes());
    bytes.extend(b"data");
    bytes.extend(size.to_le_bytes());
    for i in 0..frames {
        let sample =
            ((i as f64 * 1000. * std::f64::consts::TAU / f64::from(rate)).sin() * 12000.) as i16;
        bytes.extend(sample.to_le_bytes());
    }
    std::fs::write(path, bytes).unwrap();
}
#[test]
fn wav_load_keeps_original_speed_and_pads_the_loop_boundary() {
    let path = std::env::temp_dir().join(format!("loopclub-wav-{}.wav", std::process::id()));
    wav(&path, 44100, 11025);
    let job = loop_club::sample::prepare(&path, 0, 48000, 120., 48000).unwrap();
    assert_eq!(job.source_frames, 12000);
    assert_eq!(job.frames, 96000);
    assert_eq!(job.ticks, 4 * PPQ);
    assert!(job.audio[12000..].iter().all(|s| s.peak() == 0.));
    let crossings = job.audio[..12000]
        .windows(2)
        .filter(|f| f[0].l <= 0. && f[1].l > 0.)
        .count();
    assert!((249..=251).contains(&crossings));
    std::fs::remove_file(path).unwrap();
}
#[test]
fn sample_commit_preserves_other_playing_tracks_and_rejects_overwrite() {
    let path = std::env::temp_dir().join(format!("loopclub-swap-{}.wav", std::process::id()));
    wav(&path, 8000, 2000);
    let r = Box::new(Runtime::default());
    let mut e = Box::new(Engine::new(8000, 240., 16000));
    e.running = true;
    e.tracks[1].kind = Kind::Synth;
    e.tracks[1].length_ticks = 4 * PPQ;
    e.tracks[1].playing = true;
    r.initialize(e).ok().unwrap();
    r.active.store(true, Ordering::Release);
    let job = loop_club::sample::prepare(&path, 0, 8000, 240., 16000).unwrap();
    r.submit_import(job).ok().unwrap();
    r.output(&mut [0; 128], 2, 8000);
    let done = r.collect_import().unwrap();
    assert!(done.error.is_none());
    drop(done);
    r.with_engine(|s| {
        let e = s.as_ref().unwrap();
        assert!(e.tracks[1].playing);
        assert_eq!(e.tracks[0].source_frames, 2000);
        assert_eq!(e.tracks[0].chops.slices[15].end, 2000);
    });
    let job = loop_club::sample::prepare(&path, 0, 8000, 240., 16000).unwrap();
    r.submit_import(job).ok().unwrap();
    r.output(&mut [0; 128], 2, 8000);
    assert_eq!(r.collect_import().unwrap().error, Some(Error::NotEmpty));
    r.enqueue(Command::Stop);
    std::fs::remove_file(path).unwrap();
}
#[test]
fn browser_stays_within_root_and_filters_audio() {
    let root = std::env::temp_dir().join(format!("loopclub-browser-{}", std::process::id()));
    std::fs::create_dir_all(root.join("kits")).unwrap();
    std::fs::write(root.join("notes.txt"), b"private text").unwrap();
    wav(&root.join("tone.wav"), 8000, 500);
    std::fs::write(root.join("._tone.wav"), b"macOS metadata").unwrap();
    let mut b = loop_club::sample::Browser::open(&root).unwrap();
    assert_eq!(b.view.entries.len(), 2);
    assert!(b.view.entries[0].directory);
    b.enter(0).unwrap();
    b.up().unwrap();
    b.up().unwrap();
    assert_eq!(b.current, root.canonicalize().unwrap());
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn background_picker_loads_the_selected_wav() {
    let root = std::env::temp_dir().join(format!("loopclub-picker-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    wav(&root.join("kick.wav"), 8000, 1000);
    let r = Box::new(Runtime::default());
    r.initialize(Box::new(Engine::new(8000, 240., 16000)))
        .ok()
        .unwrap();
    loop_club::browser_device::initialize();
    let mut worker =
        loop_club::browser_device::FileWorker::with_root(Some(root.clone()), 8000, 16000);
    let mut ui = loop_club::browser_device::BrowserUi::default();
    ui.start(0);
    worker.tick(&r);
    ui.refresh();
    ui.button(13);
    worker.tick(&r);
    r.output(&mut [0; 128], 2, 8000);
    assert_eq!(worker.tick(&r), Some(true));
    r.with_engine(|s| assert_eq!(s.as_ref().unwrap().tracks[0].source_frames, 1000));
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn wav_browser_first_turn_scaled_input_and_selected_file_roundtrip() {
    use loop_club::{
        browser_device::{self, BrowserUi, FileWorker},
        device::normalize_encoder,
    };
    let root = std::env::temp_dir().join(format!("lc-wave-cursor-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    for (i, name) in ["a", "b", "c", "d"].iter().enumerate() {
        wav(
            &root.join(format!("{name}.wav")),
            8000,
            800 + i as u32 * 100,
        );
    }
    let r = Runtime::default();
    assert!(
        r.initialize(Box::new(Engine::new(8000, 240., 16000)))
            .is_ok()
    );
    r.active.store(true, Ordering::Release);
    r.output(&mut [0; 128], 2, 8000);
    browser_device::initialize();
    let mut worker = FileWorker::with_root(Some(root.clone()), 8000, 16000);
    let mut ui = BrowserUi::default();
    ui.start(0);
    worker.tick(&r);
    // Data can arrive after the worker completes but before a separate UI refresh.
    ui.encoder(normalize_encoder(128));
    assert_eq!(ui.row, 1);
    assert_eq!(ui.total(), 4);
    ui.button(54);
    assert_eq!(ui.row, 2);
    ui.button(55);
    assert_eq!(ui.row, 1);
    ui.button_with_shift(54, true);
    assert_eq!(ui.row, 3);
    ui.button(55);
    assert_eq!(ui.row, 2);
    ui.button(13);
    worker.tick(&r);
    r.output(&mut [0; 128], 2, 8000);
    assert_eq!(worker.tick(&r), Some(true));
    r.with_engine(|e| {
        let t = &e.as_ref().unwrap().tracks[0];
        assert_eq!(t.source_frames, 1000);
        assert_eq!(t.name, "c");
    });
    std::fs::remove_dir_all(root).unwrap();
}
