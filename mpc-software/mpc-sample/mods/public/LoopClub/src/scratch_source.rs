//! Bundled third-party scratch excerpt; see assets/scratch/SOURCE-NOTICE.txt.
//! Prepare only on the file worker; finished PCM swaps at an audio boundary.
use crate::{model::*, sample::ImportJob};
const SOURCE: &[u8] = include_bytes!("../assets/scratch/classic-ah-yeah-44100-stereo.s16le");
const SOURCE_RATE: f64 = 44100.;
pub fn prepare(
    track: usize,
    rate: u32,
    bpm: f64,
    capacity: usize,
) -> Result<Box<ImportJob>, Error> {
    if track >= 16
        || !(8000..=96000).contains(&rate)
        || !bpm.is_finite()
        || !(40. ..=240.).contains(&bpm)
    {
        return Err(Error::InvalidValue);
    }
    let source_frames = (SOURCE.len() as f64 / 4. * rate as f64 / SOURCE_RATE).round() as usize;
    let bar = 60. / bpm * 4. * f64::from(rate);
    let bars = (source_frames as f64 / bar).ceil().max(1.) as u64;
    let frames = (bars as f64 * bar).round() as usize;
    if frames > capacity {
        return Err(Error::Capacity);
    }
    let mut audio = vec![Frame::ZERO; capacity];
    // File-worker-only 128-phase / 32-tap conversion, matching WAV import.
    let cutoff = (rate as f64 / SOURCE_RATE).min(1.) * 0.92;
    let mut kernels = [[0f32; 32]; 128];
    for (phase, kernel) in kernels.iter_mut().enumerate() {
        let fraction = phase as f64 / 128.;
        let mut total = 0.;
        for (tap, weight) in kernel.iter_mut().enumerate() {
            let d = tap as f64 - 15. - fraction;
            let x = std::f64::consts::PI * d * cutoff;
            let sinc = if x.abs() < 1e-10 { 1. } else { x.sin() / x };
            let window = (0.5 + 0.5 * (std::f64::consts::PI * d / 16.).cos()).max(0.);
            *weight = (cutoff * sinc * window) as f32;
            total += *weight;
        }
        for weight in kernel {
            *weight /= total;
        }
    }
    let count = SOURCE.len() / 4;
    let at = |i: usize| Frame {
        l: i16::from_le_bytes([SOURCE[4 * i], SOURCE[4 * i + 1]]) as f32 / 32768.,
        r: i16::from_le_bytes([SOURCE[4 * i + 2], SOURCE[4 * i + 3]]) as f32 / 32768.,
    };
    for (i, out) in audio.iter_mut().take(source_frames).enumerate() {
        let position = i as f64 * SOURCE_RATE / rate as f64;
        let base = position.floor() as i64;
        let phase = ((position.fract() * 128.) as usize).min(127);
        let value = if rate == SOURCE_RATE as u32 {
            at(i)
        } else {
            kernels[phase]
                .iter()
                .enumerate()
                .map(|(tap, w)| {
                    let index = (base + tap as i64 - 15).clamp(0, count as i64 - 1) as usize;
                    at(index).gain(*w)
                })
                .fold(Frame::ZERO, Frame::combine)
        };
        // Resampling may ring just past a zero endpoint; taper only 1ms.
        let fade = (rate as usize / 1000).max(1);
        let gain = (i as f32 / fade as f32).min(1.)
            * ((source_frames - 1 - i) as f32 / fade as f32).min(1.);
        *out = value.gain(gain);
    }
    let peak = audio[..source_frames]
        .iter()
        .map(|f| f.l.abs().max(f.r.abs()))
        .fold(0f32, f32::max);
    if peak > 0.6 {
        for frame in &mut audio[..source_frames] {
            *frame = frame.gain(0.6 / peak);
        }
    }
    let mut features = Box::new(crate::features::Features::default());
    for frame in &audio[..source_frames] {
        features.feed(*frame, rate);
    }
    Ok(Box::new(ImportJob {
        features,
        track,
        rate,
        bpm,
        audio,
        frames,
        source_frames,
        ticks: bars * 4 * PPQ,
        name: "Classic Ah Yeah".into(),
        error: None,
    }))
}

/// A user-supplied default stays outside autorun and survives app upgrades.
/// Invalid files are reported rather than silently selecting a different sound.
pub fn prepare_default(
    root: Option<&std::path::Path>,
    track: usize,
    rate: u32,
    bpm: f64,
    capacity: usize,
) -> Result<Box<ImportJob>, Error> {
    if let Some(root) = root {
        let path = root.join("LoopClub/Content/Scratch/default.wav");
        match std::fs::metadata(&path) {
            Ok(_) => {
                return crate::sample::prepare(&path, track, rate, bpm, capacity)
                    .map(|mut job| {
                        job.name = "Default Scratch".into();
                        job
                    })
                    .map_err(|_| Error::InvalidValue);
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(_) => return Err(Error::InvalidValue),
        }
    }
    prepare(track, rate, bpm, capacity)
}

/// Development-only transport helper; called by the opt-in USB QA server,
/// never the audio callback. No arbitrary path or project modification.
pub fn install_default(root: &std::path::Path, bytes: &[u8]) -> std::io::Result<usize> {
    use std::{
        fs,
        io::{Error as IoError, ErrorKind, Write},
    };
    if !(44..=512 * 1024).contains(&bytes.len()) {
        return Err(IoError::new(ErrorKind::InvalidData, "Expected bounded WAV"));
    }
    let mut dir = root.canonicalize()?;
    for part in ["LoopClub", "Content", "Scratch"] {
        dir.push(part);
        match fs::symlink_metadata(&dir) {
            Ok(m) if !m.is_dir() || m.file_type().is_symlink() => {
                return Err(IoError::new(
                    ErrorKind::InvalidInput,
                    "Refuse redirected scratch directory",
                ));
            }
            Ok(_) => {}
            Err(e) if e.kind() == ErrorKind::NotFound => fs::create_dir(&dir)?,
            Err(e) => return Err(e),
        }
    }
    let dest = dir.join("default.wav");
    if fs::symlink_metadata(&dest).is_ok_and(|m| !m.is_file() || m.file_type().is_symlink()) {
        return Err(IoError::new(
            ErrorKind::InvalidInput,
            "Refuse redirected scratch file",
        ));
    }
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(IoError::other)?
        .as_nanos();
    let temp = dir.join(format!(".default-{stamp}.wav"));
    let result = (|| {
        let mut f = fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&temp)?;
        f.write_all(bytes)?;
        f.sync_all()?;
        drop(f);
        // Four seconds maximum; the standard importer validates channels,
        // truncation, finite samples and supported rates before activation.
        let audio = crate::sample::decode(&temp, 48000, 48000 * 4)?;
        if !audio.iter().any(|s| s.l.abs().max(s.r.abs()) > 0.0001) {
            return Err(IoError::new(
                ErrorKind::InvalidData,
                "Silent default source",
            ));
        }
        if dest.exists() {
            let old = fs::read(&dest)?;
            if old == bytes {
                return Ok(audio.len());
            }
            let mut backup = fs::OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(dir.join(format!("default-previous-{stamp}.wav")))?;
            backup.write_all(&old)?;
            backup.sync_all()?;
        }
        fs::rename(&temp, &dest)?;
        if fs::read(&dest)? != bytes {
            return Err(IoError::other("Default source readback mismatch"));
        }
        Ok(audio.len())
    })();
    let _ = fs::remove_file(&temp);
    result
}
