//! Standalone optional sample banks. All parsing/decoding happens on the file worker.
use crate::{engine::Engine, model::*};
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};
pub const SESSION_KIT_BYTES: usize = 256 * 1024 * 1024;
pub const KIT_BYTES: usize = 64 * 1024 * 1024;
#[derive(Default)]
pub struct DrumKit {
    pub name: String,
    pub id: String,
    pub clips: [Vec<Frame>; 16],
    pub secondary: [Vec<Frame>; 16],
    pub names: [[String; 2]; 16],
}
#[derive(Serialize, Deserialize)]
pub struct Record {
    pub id: String,
    pub name: String,
    pub offset: u64,
    pub bytes: usize,
    pub crc: u32,
    pub kind: u32,
    pub role: u32,
    pub rate: u32,
    pub channels: u32,
    pub frames: u32,
}
#[derive(Serialize, Deserialize)]
pub struct Kit {
    pub name: String,
    pub sounds: [Option<usize>; 16],
    pub levels: [f32; 16],
    pub tunes: [f32; 16],
}
#[derive(Serialize, Deserialize)]
pub struct Manifest {
    pub format: String,
    pub id: String,
    pub name: String,
    pub redistributable: bool,
    pub payload_bytes: u64,
    pub records: Vec<Record>,
    pub kits: Vec<Kit>,
}
pub struct Bank {
    pub manifest: Manifest,
    path: PathBuf,
    payload: u64,
}
fn bad(s: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, s)
}
pub fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = !0u32;
    for &b in bytes {
        crc ^= u32::from(b);
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xedb88320u32 & 0u32.wrapping_sub(crc & 1));
        }
    }
    !crc
}
pub fn sample_name(name: &str) -> String {
    let base = name.rsplit(['/', '\\']).next().unwrap_or(name);
    let base = base
        .rsplit_once('.')
        .filter(|(_, ext)| ext.eq_ignore_ascii_case("wav"))
        .map_or(base, |(stem, _)| stem);
    base.chars().take(32).collect()
}
impl DrumKit {
    pub fn bytes(&self) -> usize {
        self.clips
            .iter()
            .chain(self.secondary.iter())
            .map(|c| c.len() * 8)
            .sum()
    }
    pub fn clip(&self, sound: usize, layer: usize) -> &[Frame] {
        if layer == 0 {
            &self.clips[sound]
        } else {
            &self.secondary[sound]
        }
    }
}
impl Bank {
    pub fn open(path: &Path) -> io::Result<Self> {
        let mut f = File::open(path)?;
        let len = f.metadata()?.len();
        let mut header = [0; 16];
        f.read_exact(&mut header)?;
        if &header[..8] != b"LCBANK01" {
            return Err(bad("Not a Loop Club bank"));
        }
        let size = u64::from_le_bytes(header[8..].try_into().unwrap());
        if size > 16 * 1024 * 1024 || size > len.saturating_sub(16) {
            return Err(bad("Invalid metadata length"));
        }
        let mut data = vec![0; size as usize];
        f.read_exact(&mut data)?;
        let manifest: Manifest =
            serde_json::from_slice(&data).map_err(|_| bad("Invalid bank metadata"))?;
        let m = &manifest;
        if m.format != "LCBANK01"
            || m.id.is_empty()
            || m.id.len() > 63
            || !m
                .id
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || b"_-".contains(&c))
            || m.name.is_empty()
            || m.name.len() > 128
            || m.name.chars().any(char::is_control)
            || m.payload_bytes != len - 16 - size
            || m.payload_bytes > 4 * 1024 * 1024 * 1024
            || m.records.is_empty()
            || m.records.len() > 50000
            || m.kits.is_empty()
            || m.kits.len() > 512
        {
            return Err(bad("Invalid bank bounds/identity"));
        }
        for r in &m.records {
            if r.bytes > 64 * 1024 * 1024
                || r.offset > m.payload_bytes
                || r.bytes as u64 > m.payload_bytes - r.offset
            {
                return Err(bad("Invalid sample bounds"));
            }
        }
        for k in &m.kits {
            if k.name.is_empty()
                || k.name.len() > 128
                || k.name.chars().any(char::is_control)
                || !k
                    .levels
                    .iter()
                    .all(|v| v.is_finite() && (0.0..=1.0).contains(v))
                || !k
                    .tunes
                    .iter()
                    .all(|v| v.is_finite() && (-24.0..=24.0).contains(v))
                || k.sounds
                    .iter()
                    .flatten()
                    .any(|i| *i >= m.records.len() || m.records[*i].kind != 0)
            {
                return Err(bad("Invalid kit"));
            }
        }
        let mut signature = [0; 8];
        f.read_exact(&mut signature)?;
        if &signature != b"GDBANK01" {
            return Err(bad("Invalid payload signature"));
        }
        Ok(Self {
            manifest,
            path: path.into(),
            payload: 16 + size,
        })
    }
    pub fn prepare_layer(
        &self,
        index: usize,
        track: usize,
        sound: usize,
        layer: usize,
        rate: u32,
    ) -> io::Result<Box<KitJob>> {
        if track >= 16 || sound >= 16 || layer >= 2 {
            return Err(bad("Invalid layer target"));
        }
        let record = self
            .manifest
            .records
            .get(index)
            .filter(|r| r.kind == 0)
            .ok_or_else(|| bad("Not an audio sample"))?;
        let mut f = File::open(&self.path)?;
        f.seek(SeekFrom::Start(self.payload + record.offset))?;
        let mut bytes = vec![0; record.bytes];
        f.read_exact(&mut bytes)?;
        if crc32(&bytes) != record.crc {
            return Err(bad("Sample integrity check failed"));
        }
        let tmp = TempWav::write(&bytes)?;
        let pcm = crate::sample::decode(&tmp.0, rate, rate as usize * 16)?;
        let mut kit = Box::<DrumKit>::default();
        kit.clips[0] = pcm;
        kit.names[0][0] = sample_name(&record.name);
        Ok(Box::new(KitJob {
            track,
            rate,
            kit,
            drums: [Drum::default(); 16],
            target: Some((sound, layer)),
            error: None,
        }))
    }
    pub fn prepare(&self, index: usize, track: usize, rate: u32) -> io::Result<Box<KitJob>> {
        if track >= 16 || !(8000..=96000).contains(&rate) {
            return Err(bad("Invalid kit destination"));
        }
        let k = self
            .manifest
            .kits
            .get(index)
            .ok_or_else(|| bad("Kit not found"))?;
        let mut kit = Box::new(DrumKit {
            name: k.name.clone(),
            id: format!("{}:{}", self.manifest.id, index),
            ..DrumKit::default()
        });
        let mut f = File::open(&self.path)?;
        let mut total = 0usize;
        for (pad, entry) in k.sounds.iter().enumerate() {
            let Some(index) = entry else {
                continue;
            };
            let r = &self.manifest.records[*index];
            let mut bytes = vec![0; r.bytes];
            f.seek(SeekFrom::Start(self.payload + r.offset))?;
            f.read_exact(&mut bytes)?;
            if crc32(&bytes) != r.crc {
                return Err(bad("Sample integrity check failed"));
            }
            let tmp = TempWav::write(&bytes)?;
            let pcm = crate::sample::decode(&tmp.0, rate, rate as usize * 16)?;
            total = total
                .checked_add(pcm.len() * 8)
                .ok_or_else(|| bad("Kit size overflow"))?;
            if total > KIT_BYTES {
                return Err(bad("Selected kit exceeds 64 MiB"));
            }
            kit.clips[pad] = pcm;
            kit.names[pad][0] = sample_name(&r.name);
        }
        let mut drums = [Drum::default(); 16];
        for (pad, drum) in drums.iter_mut().enumerate() {
            if !kit.clips[pad].is_empty() {
                drum.mix.level = k.levels[pad];
                drum.tune = k.tunes[pad];
                drum.decay = (kit.clips[pad].len() as f32 / rate as f32).clamp(0.01, 16.);
            }
        }
        Ok(Box::new(KitJob {
            track,
            rate,
            kit,
            drums,
            target: None,
            error: None,
        }))
    }
}
struct TempWav(PathBuf);
impl TempWav {
    fn write(bytes: &[u8]) -> io::Result<Self> {
        static SERIAL: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!(
            "loop-club-kit-{}-{}.wav",
            std::process::id(),
            SERIAL.fetch_add(1, Ordering::Relaxed)
        ));
        let mut f = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)?;
        let temp = Self(path);
        f.write_all(bytes)?;
        drop(f);
        Ok(temp)
    }
}
impl Drop for TempWav {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}
pub struct KitJob {
    pub track: usize,
    pub rate: u32,
    pub kit: Box<DrumKit>,
    pub drums: [Drum; 16],
    pub target: Option<(usize, usize)>,
    pub error: Option<Error>,
}
impl KitJob {
    pub fn commit(&mut self, e: &mut Engine) {
        self.error = if self.track >= 16 {
            Some(Error::InvalidTrack)
        } else if e.clock.rate != self.rate {
            Some(Error::InvalidValue)
        } else if e.tracks[self.track].kind != Kind::Drum {
            Some(Error::WrongKind)
        } else if e.recorder.is_some_and(|r| r.track == self.track) {
            Some(Error::Busy)
        } else {
            None
        };
        if let Some(err) = self.error {
            e.error = Some(err);
            return;
        }
        let used = e
            .tracks
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != self.track)
            .map(|(_, t)| t.kit.bytes())
            .sum::<usize>();
        let incoming = if let Some((sound, layer)) = self.target {
            if sound >= 16 || layer >= 2 {
                self.error = Some(Error::InvalidValue);
                return;
            }
            e.tracks[self.track].kit.bytes() - e.tracks[self.track].kit.clip(sound, layer).len() * 8
                + self.kit.clips[0].len() * 8
        } else {
            self.kit.bytes()
        };
        if incoming > KIT_BYTES || used + incoming > SESSION_KIT_BYTES {
            self.error = Some(Error::Capacity);
            e.error = self.error;
            return;
        }
        if let Some((sound, layer)) = self.target {
            e.kill_layer_voices(self.track, sound, layer);
        } else {
            e.kill_track_voices(self.track);
        }
        let t = &mut e.tracks[self.track];
        if let Some((sound, layer)) = self.target {
            let clip = if layer == 0 {
                &mut t.kit.clips[sound]
            } else {
                &mut t.kit.secondary[sound]
            };
            std::mem::swap(clip, &mut self.kit.clips[0]);
            std::mem::swap(&mut t.kit.names[sound][layer], &mut self.kit.names[0][0]);
            let cfg = &mut t.drums[sound].layers[layer];
            cfg.enabled = true;
            cfg.reverse = false;
            cfg.decay = (clip.len() as f32 / self.rate as f32).clamp(0.01, 16.);
        } else {
            std::mem::swap(&mut t.kit, &mut self.kit);
            t.drums = self.drums;
            t.sound_gain = t.drums.map(|d| d.mix.level);
        }
        t.revision += 1;
        e.error = None;
    }
}
