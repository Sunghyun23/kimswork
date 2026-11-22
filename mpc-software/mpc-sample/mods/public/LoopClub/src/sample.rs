//! Offline WAV decoding, rate conversion and bounded SD folder browsing.
use crate::{chop::Chops, model::*};
use mpc_sample_sdk::platform::wav;
use std::{
    io,
    path::{Path, PathBuf},
};
pub struct ImportJob {
    pub features: Box<crate::features::Features>,
    pub track: usize,
    pub rate: u32,
    pub bpm: f64,
    pub audio: Vec<Frame>,
    pub frames: usize,
    pub source_frames: usize,
    pub ticks: u64,
    pub name: String,
    pub error: Option<Error>,
}
fn invalid(text: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, text)
}
pub fn decode(path: &Path, rate: u32, capacity: usize) -> io::Result<Vec<Frame>> {
    let info = wav::Info::read(path).ok_or_else(|| invalid("Unsupported WAV"))?;
    if !(8000..=96000).contains(&rate)
        || !(1..=2).contains(&info.channels)
        || !(8000..=192000).contains(&info.rate)
        || info.frames == 0
        || u64::from(info.frames) * u64::from(rate) > capacity as u64 * u64::from(info.rate)
    {
        return Err(invalid("Unsupported sample rate/channels/duration"));
    }
    let raw = wav::read_f32(path, info.frames, info.channels, info.rate)
        .ok_or_else(|| invalid("Truncated WAV"))?;
    if raw.iter().any(|v| !v.is_finite() || v.abs() > 8.) {
        return Err(invalid("Invalid audio samples"));
    }
    let count = (u64::from(info.frames) * u64::from(rate) + u64::from(info.rate) / 2)
        / u64::from(info.rate);
    let count = count.max(1) as usize;
    let mut audio = vec![Frame::ZERO; count];
    let at = |i: usize| {
        let offset = i * usize::from(info.channels);
        Frame {
            l: raw[offset],
            r: raw[offset + usize::from(info.channels) - 1],
        }
    };
    if info.rate == rate {
        for (i, out) in audio[..count].iter_mut().enumerate() {
            *out = at(i)
        }
    } else {
        // 128-phase, 32-tap windowed-sinc conversion; pitch/time follow the source.
        let cutoff = (f64::from(rate) / f64::from(info.rate)).min(1.) * 0.92;
        let mut kernels = [[0f32; 32]; 128];
        for (phase, kernel) in kernels.iter_mut().enumerate() {
            let fraction = phase as f64 / 128.;
            let mut sum = 0.;
            for (tap, value) in kernel.iter_mut().enumerate() {
                let d = tap as f64 - 15. - fraction;
                let x = std::f64::consts::PI * d * cutoff;
                let sinc = if x.abs() < 1e-10 { 1. } else { x.sin() / x };
                let window = (0.5 + 0.5 * (std::f64::consts::PI * d / 16.).cos()).max(0.);
                let w = cutoff * sinc * window;
                *value = w as f32;
                sum += w;
            }
            for value in kernel {
                *value /= sum as f32;
            }
        }
        for (i, out) in audio[..count].iter_mut().enumerate() {
            let position = i as f64 * f64::from(info.rate) / f64::from(rate);
            let base = position.floor() as i64;
            let phase = (position.fract() * 128.).floor() as usize;
            let mut sum = Frame::ZERO;
            for (tap, weight) in kernels[phase.min(127)].iter().enumerate() {
                let source = (base + tap as i64 - 15).clamp(0, i64::from(info.frames) - 1) as usize;
                sum = sum.combine(at(source).gain(*weight));
            }
            *out = sum;
        }
    }
    Ok(audio)
}
pub fn prepare(
    path: &Path,
    track: usize,
    rate: u32,
    bpm: f64,
    capacity: usize,
) -> io::Result<Box<ImportJob>> {
    if track >= 16 || !bpm.is_finite() || !(40.0..=240.0).contains(&bpm) {
        return Err(invalid("Invalid target/tempo"));
    }
    let mut audio = decode(path, rate, capacity)?;
    let count = audio.len();
    let bar = 60. / bpm * 4. * f64::from(rate);
    let bars = (count as f64 / bar).ceil().max(1.) as u64;
    let ticks = bars * 4 * PPQ;
    let frames = (bars as f64 * bar).round() as usize;
    audio.resize(frames.max(capacity), Frame::ZERO);
    let name: String = path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .chars()
        .filter(|c| !c.is_control())
        .scan(0usize, |len, c| {
            *len += c.len_utf8();
            (*len <= 32).then_some(c)
        })
        .collect();
    let mut features = Box::new(crate::features::Features::default());
    for frame in &audio[..count] {
        features.feed(*frame, rate);
    }
    Ok(Box::new(ImportJob {
        features,
        track,
        rate,
        bpm,
        audio,
        frames,
        source_frames: count,
        ticks,
        name,
        error: None,
    }))
}
impl ImportJob {
    pub fn commit(&mut self, engine: &mut crate::engine::Engine) {
        let error = if self.track >= 16 {
            Some(Error::InvalidTrack)
        } else if engine.clock.rate != self.rate || engine.clock.bpm != self.bpm {
            Some(Error::InvalidValue)
        } else if engine.recorder.is_some_and(|r| r.track == self.track) {
            Some(Error::Busy)
        } else if engine.tracks[self.track].kind != Kind::Audio
            || !engine.tracks[self.track].empty()
        {
            Some(Error::NotEmpty)
        } else {
            None
        };
        self.error = error;
        if let Some(error) = error {
            engine.error = Some(error);
            return;
        }
        engine.clear_slice_voices(self.track);
        engine.reset_chop_history(self.track);
        let t = &mut engine.tracks[self.track];
        std::mem::swap(&mut t.audio, &mut self.audio);
        std::mem::swap(&mut t.name, &mut self.name);
        std::mem::swap(&mut t.features, &mut self.features);
        t.wave_view = (0, 0);
        t.audio_frames = self.frames;
        t.source_frames = self.source_frames;
        t.length_ticks = self.ticks;
        t.origin = 0;
        t.playing = false;
        t.events.clear();
        t.chops = Chops::default();
        t.chops.divide(self.source_frames);
        t.revision += 1;
        engine.error = None;
    }
}
#[derive(Clone)]
pub struct Entry {
    pub name: String,
    pub path: PathBuf,
    pub directory: bool,
}
#[derive(Clone, Default)]
pub struct BrowserView {
    pub path: String,
    pub entries: Vec<Entry>,
    pub error: String,
    pub revision: u64,
    pub pending: bool,
}
pub struct Browser {
    pub root: PathBuf,
    pub current: PathBuf,
    pub view: BrowserView,
}
impl Browser {
    pub fn open(root: &Path) -> io::Result<Self> {
        let root = root.canonicalize()?;
        let mut b = Self {
            current: root.clone(),
            root,
            view: BrowserView::default(),
        };
        b.refresh()?;
        Ok(b)
    }
    pub fn refresh(&mut self) -> io::Result<()> {
        let mut entries = Vec::new();
        let mut capped = false;
        for entry in std::fs::read_dir(&self.current)? {
            let e = entry?;
            let file_type = e.file_type()?;
            if file_type.is_symlink() {
                continue;
            }
            let path = e.path();
            let name = e.file_name().to_string_lossy().into_owned();
            if name.starts_with('.') {
                continue;
            }
            let dir = file_type.is_dir();
            if !dir
                && !path
                    .extension()
                    .is_some_and(|v| v.eq_ignore_ascii_case("wav"))
            {
                continue;
            }
            if entries.len() >= 8192 {
                capped = true;
                break;
            }
            entries.push(Entry {
                name,
                path,
                directory: dir,
            })
        }
        entries.sort_by(|a, b| {
            b.directory
                .cmp(&a.directory)
                .then(a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        });
        self.view = BrowserView {
            path: self
                .current
                .strip_prefix(&self.root)
                .unwrap_or(Path::new(""))
                .to_string_lossy()
                .into_owned(),
            entries,
            error: if capped {
                "8192 entries limit; use subfolders".into()
            } else {
                String::new()
            },
            revision: self.view.revision + 1,
            pending: false,
        };
        Ok(())
    }
    pub fn up(&mut self) -> io::Result<()> {
        if self.current != self.root {
            let parent = self.current.parent().unwrap_or(&self.root);
            if parent.starts_with(&self.root) {
                self.current = parent.to_path_buf();
            }
        }
        self.refresh()
    }
    pub fn resolve(&self, index: usize) -> io::Result<PathBuf> {
        let e = self
            .view
            .entries
            .get(index)
            .ok_or_else(|| invalid("Entry no longer exists"))?;
        let path = e.path.canonicalize()?;
        if !path.starts_with(&self.root) {
            return Err(invalid("Path leaves SD root"));
        }
        Ok(path)
    }
    pub fn enter(&mut self, index: usize) -> io::Result<()> {
        let path = self.resolve(index)?;
        if !path.is_dir() {
            return Err(invalid("Not a folder"));
        }
        self.current = path;
        self.refresh()
    }
}
