use loop_club::{browser_device::FileWorker, engine::Engine, runtime::Runtime, scratch_source};
use std::{
    io::Write,
    path::{Path, PathBuf},
};
struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "lc-default-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&p).unwrap();
        Self(p)
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn wav(hz: f32) -> Vec<u8> {
    let n = 1600u32;
    let bytes = n * 2;
    let mut out = Vec::new();
    out.write_all(b"RIFF").unwrap();
    out.extend((36 + bytes).to_le_bytes());
    out.extend(b"WAVEfmt ");
    out.extend(16u32.to_le_bytes());
    out.extend(1u16.to_le_bytes());
    out.extend(1u16.to_le_bytes());
    out.extend(8000u32.to_le_bytes());
    out.extend(16000u32.to_le_bytes());
    out.extend(2u16.to_le_bytes());
    out.extend(16u16.to_le_bytes());
    out.extend(b"data");
    out.extend(bytes.to_le_bytes());
    for i in 0..n {
        out.extend(
            (((i as f32 * hz / 8000. * std::f32::consts::TAU).sin() * 9000.) as i16).to_le_bytes(),
        );
    }
    out
}
fn file(root: &Path) -> PathBuf {
    root.join("LoopClub/Content/Scratch/default.wav")
}
#[test]
fn installed_default_is_decoded_by_the_file_worker_and_preserves_existing_sources() {
    let t = Temp::new();
    let bytes = wav(330.);
    assert_eq!(scratch_source::install_default(&t.0, &bytes).unwrap(), 9600);
    assert_eq!(std::fs::read(file(&t.0)).unwrap(), bytes);
    let r = Runtime::default();
    assert!(
        r.initialize(Box::new(Engine::new(8000, 100., 32000)))
            .is_ok()
    );
    r.active.store(true, std::sync::atomic::Ordering::Release);
    let mut w = FileWorker::with_root(Some(t.0.clone()), 8000, 32000);
    w.load_builtin(&r, 0).unwrap();
    let mut out = [0; 128];
    r.output(&mut out, 2, 8000);
    assert_eq!(w.tick(&r), Some(true));
    r.output(&mut out, 2, 8000);
    r.with_engine(|e| {
        let t = &e.as_ref().unwrap().tracks[0];
        assert_eq!(t.source_frames, 1600);
        assert_eq!(t.audio_target, loop_club::audio::RecordTarget::Turntable);
        assert!(t.audio.iter().any(|x| x.l.abs() > 0.1));
    })
    .unwrap();
    assert!(w.load_builtin(&r, 0).is_err());
}
#[test]
fn invalid_or_silent_upload_does_not_replace_default_and_replacement_retains_backup() {
    let t = Temp::new();
    let a = wav(330.);
    let b = wav(440.);
    scratch_source::install_default(&t.0, &a).unwrap();
    assert!(scratch_source::install_default(&t.0, b"not a wave").is_err());
    let mut silent = a.clone();
    silent[44..].fill(0);
    assert!(scratch_source::install_default(&t.0, &silent).is_err());
    assert_eq!(std::fs::read(file(&t.0)).unwrap(), a);
    scratch_source::install_default(&t.0, &b).unwrap();
    scratch_source::install_default(&t.0, &b).unwrap();
    let files: Vec<_> = std::fs::read_dir(file(&t.0).parent().unwrap())
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    assert_eq!(files.len(), 2);
    assert!(files.iter().any(|p| std::fs::read(p).unwrap() == a));
    assert_eq!(std::fs::read(file(&t.0)).unwrap(), b);
}
#[test]
fn missing_default_uses_bundled_clip_but_corrupt_selected_default_is_an_error() {
    let t = Temp::new();
    assert_eq!(
        scratch_source::prepare_default(Some(&t.0), 0, 8000, 100., 32000)
            .unwrap()
            .name,
        "Classic Ah Yeah"
    );
    scratch_source::install_default(&t.0, &wav(330.)).unwrap();
    std::fs::write(file(&t.0), b"broken").unwrap();
    assert!(scratch_source::prepare_default(Some(&t.0), 0, 8000, 100., 32000).is_err());
}
#[cfg(unix)]
#[test]
fn install_refuses_symlinked_directories_or_destination() {
    use std::os::unix::fs::symlink;
    let t = Temp::new();
    let other = Temp::new();
    symlink(&other.0, t.0.join("LoopClub")).unwrap();
    assert!(scratch_source::install_default(&t.0, &wav(330.)).is_err());
    std::fs::remove_file(t.0.join("LoopClub")).unwrap();
    scratch_source::install_default(&t.0, &wav(330.)).unwrap();
    let f = file(&t.0);
    std::fs::remove_file(&f).unwrap();
    let target = other.0.join("owner.wav");
    std::fs::write(&target, b"preserve").unwrap();
    symlink(&target, &f).unwrap();
    assert!(scratch_source::install_default(&t.0, &wav(440.)).is_err());
    assert_eq!(std::fs::read(target).unwrap(), b"preserve");
}
