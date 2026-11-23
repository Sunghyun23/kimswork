//! Bounded, worker-only MPC drum-program import. No native plugin/FX execution.
use crate::{
    bank::{DrumKit, KIT_BYTES, KitJob},
    model::*,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    io,
    path::{Path, PathBuf},
};
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct PadMeta {
    pub count: u8,
    pub ranges: [[u8; 2]; 4],
    pub mode: u8,
    pub mute_group: u8,
    pub mute_targets: [u8; 4],
    pub gate: bool,
    pub mono: bool,
    pub sensitivity: f32,
}
impl Default for PadMeta {
    fn default() -> Self {
        Self {
            count: 0,
            ranges: [[0, 127]; 4],
            mode: 1,
            mute_group: 0,
            mute_targets: [0; 4],
            gate: false,
            mono: false,
            sensitivity: 1.,
        }
    }
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Meta {
    pub bank: u8,
    pub pads: [PadMeta; 16],
}
impl Meta {
    pub fn valid(self) -> bool {
        self.bank < 8
            && self.pads.iter().all(|p| {
                p.count <= 4
                    && p.mode <= 2
                    && p.mute_group <= 32
                    && p.mute_targets.iter().all(|n| *n <= 16)
                    && p.ranges.iter().all(|r| r[0] <= r[1] && r[1] <= 127)
                    && p.sensitivity.is_finite()
                    && (0.0..=1.0).contains(&p.sensitivity)
            })
    }
}
#[derive(Clone)]
pub struct Data {
    pub meta: Meta,
    pub extra: [[Vec<Frame>; 3]; 16],
}
impl Default for Data {
    fn default() -> Self {
        Self {
            meta: Meta::default(),
            extra: std::array::from_fn(|_| std::array::from_fn(|_| Vec::new())),
        }
    }
}
impl Data {
    pub fn bytes(&self) -> usize {
        self.extra.iter().flatten().map(|c| c.len() * 8).sum()
    }
    pub fn pad_bytes(&self, p: usize) -> usize {
        self.extra[p].iter().map(|c| c.len() * 8).sum()
    }
}
#[derive(Clone)]
struct Layer {
    name: String,
    file: String,
    low: u8,
    high: u8,
    volume: f32,
    pan: f32,
    tune: f32,
    start: u64,
    end: u64,
    reverse: bool,
}
struct Pad {
    number: usize,
    layers: Vec<Layer>,
    mode: u8,
    group: u8,
    targets: [u16; 4],
    volume: f32,
    pan: f32,
    tune: f32,
    gate: bool,
    mono: bool,
    mute: bool,
    solo: bool,
    sensitivity: f32,
}
pub struct Program {
    pub name: String,
    pub banks: Vec<usize>,
    pub native_fx: bool,
    pads: Vec<Pad>,
    root: PathBuf,
    directory: PathBuf,
    volume: f32,
    pan: f32,
    tune: f32,
}
fn bad(s: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, s.into())
}
fn text<'a>(node: roxmltree::Node<'a, 'a>, name: &str) -> &'a str {
    node.children()
        .find(|n| n.is_element() && n.tag_name().name() == name)
        .and_then(|n| n.text())
        .unwrap_or("")
        .trim()
}
fn number(node: roxmltree::Node<'_, '_>, name: &str, default: f32) -> io::Result<f32> {
    let value = text(node, name);
    if value.is_empty() {
        return Ok(default);
    }
    let n = value
        .parse::<f32>()
        .map_err(|_| bad(format!("Invalid XPM {name}")))?;
    if !n.is_finite() {
        return Err(bad("Non-finite XPM value"));
    }
    Ok(n)
}
fn integer(node: roxmltree::Node<'_, '_>, name: &str, default: u64) -> io::Result<u64> {
    let v = text(node, name);
    if v.is_empty() {
        Ok(default)
    } else {
        v.parse().map_err(|_| bad(format!("Invalid XPM {name}")))
    }
}
fn boolean(node: roxmltree::Node<'_, '_>, name: &str, default: bool) -> bool {
    let v = text(node, name);
    if v.is_empty() {
        default
    } else {
        v.eq_ignore_ascii_case("true") || v == "1"
    }
}
fn bounded(n: f32, lo: f32, hi: f32) -> io::Result<f32> {
    if (lo..=hi).contains(&n) {
        Ok(n)
    } else {
        Err(bad("XPM value outside supported range"))
    }
}
fn tune(node: roxmltree::Node<'_, '_>) -> io::Result<f32> {
    bounded(
        number(node, "TuneCoarse", 0.)? + number(node, "TuneFine", 0.)? / 100.,
        -96.,
        96.,
    )
}
fn limited_name(s: &str, n: usize) -> String {
    let mut bytes = 0;
    s.chars()
        .filter(|c| !c.is_control())
        .take_while(|c| {
            bytes += c.len_utf8();
            bytes <= n
        })
        .collect()
}
impl Program {
    pub fn open(path: &Path, scope: &Path) -> io::Result<Self> {
        let scope = scope.canonicalize()?;
        let path = path.canonicalize()?;
        if !path.starts_with(&scope) {
            return Err(bad("XPM outside selected storage"));
        }
        if std::fs::metadata(&path)?.len() > 8 * 1024 * 1024 {
            return Err(bad("XPM metadata exceeds limit"));
        }
        let xml = std::fs::read_to_string(&path)?;
        let doc = roxmltree::Document::parse_with_options(
            &xml,
            roxmltree::ParsingOptions {
                allow_dtd: false,
                nodes_limit: 200000,
                ..Default::default()
            },
        )
        .map_err(|e| bad(format!("Invalid XPM XML: {e}")))?;
        let program = doc
            .root_element()
            .children()
            .find(|n| n.has_tag_name("Program"))
            .ok_or_else(|| bad("XPM Program missing"))?;
        if !program
            .attribute("type")
            .is_some_and(|s| s.eq_ignore_ascii_case("Drum"))
        {
            return Err(bad("Choose a Drum XPM; keygroups are not supported"));
        }
        let instruments = program
            .children()
            .find(|n| n.has_tag_name("Instruments"))
            .ok_or_else(|| bad("XPM instruments missing"))?;
        let mut pads = Vec::new();
        let mut seen = BTreeSet::new();
        let mut banks = BTreeSet::new();
        for instrument in instruments
            .children()
            .filter(|n| n.has_tag_name("Instrument"))
        {
            let id = instrument
                .attribute("number")
                .ok_or_else(|| bad("XPM instrument number missing"))?
                .parse::<usize>()
                .map_err(|_| bad("Invalid instrument number"))?;
            if !(1..=128).contains(&id) || !seen.insert(id) {
                return Err(bad("Duplicate or out-of-range XPM pad"));
            }
            let mut layers = Vec::new();
            let mut slots = BTreeSet::new();
            if let Some(container) = instrument.children().find(|n| n.has_tag_name("Layers")) {
                for layer in container.children().filter(|n| n.has_tag_name("Layer")) {
                    let slot = layer
                        .attribute("number")
                        .unwrap_or("1")
                        .parse::<usize>()
                        .map_err(|_| bad("Invalid layer number"))?;
                    if !(1..=4).contains(&slot) || !slots.insert(slot) {
                        return Err(bad("XPM supports four unique layers per pad"));
                    }
                    if !boolean(layer, "Active", true) {
                        continue;
                    }
                    let name = text(layer, "SampleName").to_owned();
                    let file = text(layer, "SampleFile").to_owned();
                    if name.is_empty() && file.is_empty() {
                        continue;
                    }
                    let low = integer(layer, "VelStart", 0)?;
                    let high = integer(layer, "VelEnd", 127)?;
                    if low > high || high > 127 {
                        return Err(bad("Invalid XPM velocity range"));
                    }
                    let direction = integer(layer, "Direction", 0)?;
                    if direction > 1 {
                        return Err(bad("Unsupported XPM sample direction"));
                    }
                    let mut start = integer(layer, "SliceStart", 0)?;
                    let mut end = integer(layer, "SliceEnd", 0)?;
                    if end == 0 {
                        start = integer(layer, "SampleStart", 0)?;
                        end = integer(layer, "SampleEnd", 0)?;
                    }
                    layers.push((
                        slot,
                        Layer {
                            name,
                            file,
                            low: low as u8,
                            high: high as u8,
                            volume: bounded(number(layer, "Volume", 1.)?, 0., 8.)?,
                            pan: bounded(number(layer, "Pan", 0.5)?, 0., 1.)?,
                            tune: tune(layer)?,
                            start,
                            end,
                            reverse: direction == 1,
                        },
                    ));
                }
            }
            layers.sort_by_key(|(slot, _)| *slot);
            let layers: Vec<_> = layers.into_iter().map(|(_, l)| l).collect();
            if layers.is_empty() {
                continue;
            }
            let mode = integer(instrument, "ZonePlay", 1)?;
            let group = integer(instrument, "MuteGroup", 0)?;
            if mode > 2 || group > 32 {
                return Err(bad("Unsupported XPM layer mode/mute group"));
            }
            let mut targets = [0; 4];
            for (i, t) in targets.iter_mut().enumerate() {
                let n = integer(instrument, &format!("MuteTarget{}", i + 1), 0)?;
                if n > 128 {
                    return Err(bad("Invalid mute target"));
                }
                *t = n as u16;
            }
            let trigger = integer(instrument, "TriggerMode", 0)?;
            if trigger > 2 {
                return Err(bad("Unsupported trigger mode"));
            }
            banks.insert((id - 1) / 16);
            pads.push(Pad {
                number: id - 1,
                layers,
                mode: mode as u8,
                group: group as u8,
                targets,
                volume: bounded(number(instrument, "Volume", 1.)?, 0., 8.)?,
                pan: bounded(number(instrument, "Pan", 0.5)?, 0., 1.)?,
                tune: tune(instrument)?,
                gate: trigger != 0,
                mono: boolean(instrument, "Mono", false),
                mute: boolean(instrument, "Mute", false),
                solo: boolean(instrument, "Solo", false),
                sensitivity: bounded(number(instrument, "VelocitySensitivity", 1.)?, 0., 1.)?,
            });
        }
        if pads.is_empty() {
            return Err(bad("XPM has no active sample pads"));
        }
        let parent = path.parent().ok_or_else(|| bad("XPM folder unavailable"))?;
        let mut root = parent.to_path_buf();
        for ancestor in parent.ancestors().take_while(|p| p.starts_with(&scope)) {
            if ["Expansion.xml", "expansion.xml"]
                .iter()
                .any(|n| ancestor.join(n).is_file())
            {
                root = ancestor.to_path_buf();
                break;
            }
        }
        if root == parent
            && parent.file_name().is_some_and(|n| {
                n.to_string_lossy().eq_ignore_ascii_case("Programs")
                    || n.to_string_lossy().eq_ignore_ascii_case("Presets")
            })
            && let Some(up) = parent.parent().filter(|p| p.starts_with(&scope))
        {
            root = up.to_path_buf();
        }
        Ok(Self {
            name: limited_name(text(program, "ProgramName"), 128),
            banks: banks.into_iter().collect(),
            native_fx: program.descendants().any(|n| {
                n.is_element()
                    && (n.tag_name().name().starts_with("InsertEnable")
                        || n.has_tag_name("WarpEnable"))
                    && n.text()
                        .is_some_and(|s| s.trim().eq_ignore_ascii_case("true"))
            }),
            pads,
            root,
            directory: parent.to_path_buf(),
            volume: bounded(number(program, "Volume", 1.)?, 0., 8.)?,
            pan: bounded(number(program, "Pan", 0.5)?, 0., 1.)?,
            tune: tune(program)?,
        })
    }
    pub fn prepare(&self, bank: usize, track: usize, rate: u32) -> io::Result<Box<KitJob>> {
        if track >= 16 || !self.banks.contains(&bank) {
            return Err(bad("Choose an available XPM bank"));
        }
        let index = SampleIndex::new(&self.root, &self.directory)?;
        let mut kit = Box::new(DrumKit {
            name: self.name.clone(),
            id: format!("xpm-bank-{}", (b'A' + bank as u8) as char),
            xpm: Some(Box::new(Data::default())),
            ..Default::default()
        });
        let mut drums = [Drum::default(); 16];
        for d in &mut drums {
            d.layers[0].enabled = false;
            d.layers[1].enabled = false;
        }
        kit.xpm.as_mut().unwrap().meta.bank = bank as u8;
        let mut total = 0usize;
        for pad in self.pads.iter().filter(|p| p.number / 16 == bank) {
            let out = pad.number % 16;
            let mut meta = PadMeta {
                count: pad.layers.len() as u8,
                mode: pad.mode,
                mute_group: pad.group,
                gate: pad.gate,
                mono: pad.mono,
                sensitivity: pad.sensitivity,
                ..Default::default()
            };
            for (i, target) in pad.targets.iter().enumerate() {
                if *target > 0 && usize::from(*target - 1) / 16 == bank {
                    meta.mute_targets[i] = ((*target - 1) % 16 + 1) as u8;
                }
            }
            let mut longest = 0;
            for (zone, layer) in pad.layers.iter().enumerate() {
                let path = index.resolve(&layer.file, &layer.name)?;
                let info = mpc_sample_sdk::platform::wav::Info::read(&path)
                    .ok_or_else(|| bad(format!("Invalid WAV: {}", layer.name)))?;
                let expected =
                    u64::from(info.frames) * u64::from(rate) / u64::from(info.rate.max(1)) * 8;
                if expected as usize > KIT_BYTES.saturating_sub(total) {
                    return Err(bad("XPM bank exceeds 64 MiB"));
                }
                let mut pcm = crate::sample::decode(&path, rate, rate as usize * 16)?;
                let start =
                    (layer.start as f64 * f64::from(rate) / f64::from(info.rate)).round() as usize;
                let end = if layer.end == 0 {
                    pcm.len()
                } else {
                    (layer.end as f64 * f64::from(rate) / f64::from(info.rate)).round() as usize
                };
                if start >= end || end > pcm.len() + 1 {
                    return Err(bad(format!("Invalid XPM trim: {}", layer.name)));
                }
                let end = end.min(pcm.len());
                if start > 0 {
                    pcm.drain(..start);
                }
                pcm.truncate(end - start);
                pcm = pitch_pcm(&pcm, self.tune + pad.tune + layer.tune, rate)?;
                let gain = self.volume * layer.volume * pad.volume.max(1.);
                for x in &mut pcm {
                    *x = balance(balance(*x, layer.pan), self.pan).gain(gain);
                    if !x.finite() || x.peak() > 8. {
                        return Err(bad("XPM gain exceeds supported headroom"));
                    }
                }
                if layer.reverse {
                    pcm.reverse();
                }
                total = total
                    .checked_add(pcm.len() * 8)
                    .ok_or_else(|| bad("XPM size overflow"))?;
                if total > KIT_BYTES {
                    return Err(bad("XPM bank exceeds 64 MiB"));
                }
                longest = longest.max(pcm.len());
                meta.ranges[zone] = [layer.low, layer.high];
                if zone == 0 {
                    kit.clips[out] = pcm;
                    kit.names[out][0] = limited_name(
                        if layer.name.is_empty() {
                            &layer.file
                        } else {
                            &layer.name
                        },
                        128,
                    );
                } else {
                    kit.xpm.as_mut().unwrap().extra[out][zone - 1] = pcm;
                }
            }
            kit.xpm.as_mut().unwrap().meta.pads[out] = meta;
            if meta.count > 1 {
                kit.names[out][0] = limited_name(
                    &format!("{} [{} layers]", kit.names[out][0], meta.count),
                    128,
                );
            }
            let d = &mut drums[out];
            d.mix.level = pad.volume.min(1.);
            d.mix.mute = pad.mute;
            d.mix.solo = pad.solo;
            d.mix.pan = pad.pan * 2. - 1.;
            d.tune = 0.;
            d.decay = (longest as f32 / rate as f32).clamp(0.01, 16.);
            d.layers[0].enabled = true;
            d.layers[0].level = 1.;
            d.layers[0].tune = 0.;
            d.layers[0].decay = 0.;
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
fn balance(x: Frame, p: f32) -> Frame {
    Frame {
        l: x.l * (2. * (1. - p)).min(1.),
        r: x.r * (2. * p).min(1.),
    }
}
fn pitch_pcm(audio: &[Frame], semitones: f32, rate: u32) -> io::Result<Vec<Frame>> {
    if semitones.abs() < 1e-6 {
        return Ok(audio.to_vec());
    }
    let ratio = 2f64.powf(f64::from(semitones) / 12.);
    let frames = (audio.len() as f64 / ratio).ceil() as usize;
    if frames == 0 || frames > rate as usize * 16 {
        return Err(bad("Tuned XPM sample exceeds 16 seconds"));
    }
    let cutoff = (1. / ratio).min(1.) * 0.92;
    let mut kernels = [[0f32; 32]; 128];
    for (phase, k) in kernels.iter_mut().enumerate() {
        let frac = phase as f64 / 128.;
        let mut sum = 0.;
        for (tap, w) in k.iter_mut().enumerate() {
            let d = tap as f64 - 15. - frac;
            let x = std::f64::consts::PI * d * cutoff;
            let sinc = if x.abs() < 1e-10 { 1. } else { x.sin() / x };
            let value =
                cutoff * sinc * (0.5 + 0.5 * (std::f64::consts::PI * d / 16.).cos()).max(0.);
            *w = value as f32;
            sum += value;
        }
        for w in k {
            *w /= sum as f32;
        }
    }
    let mut out = vec![Frame::ZERO; frames];
    for (i, x) in out.iter_mut().enumerate() {
        let pos = i as f64 * ratio;
        let base = pos.floor() as i64;
        let phase = (pos.fract() * 128.).floor() as usize;
        for (tap, w) in kernels[phase.min(127)].iter().enumerate() {
            *x = x.combine(
                audio[(base + tap as i64 - 15).clamp(0, audio.len() as i64 - 1) as usize].gain(*w),
            );
        }
    }
    Ok(out)
}
struct SampleIndex {
    root: PathBuf,
    directory: PathBuf,
    files: BTreeMap<String, Vec<PathBuf>>,
}
impl SampleIndex {
    fn new(root: &Path, directory: &Path) -> io::Result<Self> {
        let root = root.canonicalize()?;
        let mut out = Self {
            root: root.clone(),
            directory: directory.to_path_buf(),
            files: BTreeMap::new(),
        };
        let mut dirs = vec![root];
        let mut entries = 0usize;
        while let Some(dir) = dirs.pop() {
            for entry in std::fs::read_dir(dir)? {
                let e = entry?;
                entries += 1;
                if entries > 50000 {
                    return Err(bad("XPM sample search exceeds 50000 entries"));
                }
                let ty = e.file_type()?;
                if ty.is_symlink() || e.file_name().to_string_lossy().starts_with('.') {
                    continue;
                }
                let p = e.path();
                if ty.is_dir() {
                    dirs.push(p);
                } else if p.extension().is_some_and(|x| x.eq_ignore_ascii_case("wav")) {
                    let filename = e.file_name().to_string_lossy().to_lowercase();
                    let stem = filename
                        .strip_suffix(".wav")
                        .unwrap_or(&filename)
                        .to_owned();
                    let relative = p
                        .strip_prefix(&out.root)
                        .unwrap()
                        .to_string_lossy()
                        .replace('\\', "/")
                        .to_lowercase();
                    if relative != filename {
                        out.files.entry(relative).or_default().push(p.clone());
                    }
                    out.files.entry(filename).or_default().push(p.clone());
                    out.files.entry(stem).or_default().push(p);
                }
            }
        }
        Ok(out)
    }
    fn resolve(&self, file: &str, name: &str) -> io::Result<PathBuf> {
        for hint in [file, name] {
            if hint.is_empty() {
                continue;
            }
            let normalized = hint.replace('\\', "/");
            let relative = self.directory.join(&normalized);
            for p in [
                relative,
                self.root.join(&normalized),
                self.directory.join(format!("{normalized}.wav")),
                self.root.join(format!("{normalized}.wav")),
            ] {
                if let Ok(p) = p.canonicalize()
                    && p.starts_with(&self.root)
                    && p.is_file()
                {
                    return Ok(p);
                }
            }
            for base_dir in [&self.directory, &self.root] {
                let joined = base_dir.join(&normalized);
                if let Ok(relative) = joined.strip_prefix(&self.root) {
                    let mut parts = Vec::new();
                    let mut valid = true;
                    for part in relative.components() {
                        match part {
                            std::path::Component::Normal(n) => {
                                parts.push(n.to_string_lossy().to_lowercase())
                            }
                            std::path::Component::ParentDir => {
                                if parts.pop().is_none() {
                                    valid = false;
                                    break;
                                }
                            }
                            std::path::Component::CurDir => {}
                            _ => {
                                valid = false;
                                break;
                            }
                        }
                    }
                    if valid {
                        let key = parts.join("/");
                        if let Some(found) = self.files.get(&key) {
                            if found.len() == 1 {
                                return Ok(found[0].clone());
                            }
                            return Err(bad("Ambiguous XPM relative sample path"));
                        }
                    }
                }
            }
            let base = normalized.rsplit('/').next().unwrap_or("").to_lowercase();
            if let Some(found) = self.files.get(&base) {
                if found.len() == 1 {
                    return Ok(found[0].clone());
                }
                return Err(bad(format!("Ambiguous XPM sample: {base}")));
            }
        }
        Err(bad(format!(
            "Missing XPM sample: {}",
            if name.is_empty() { file } else { name }
        )))
    }
}
