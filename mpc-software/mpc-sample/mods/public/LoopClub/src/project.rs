//! Versioned project container. Only call outside realtime, with exclusive engine ownership.
use crate::{
    engine::{Engine, MAX_EVENTS},
    model::*,
};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File, OpenOptions},
    io::{self, BufReader, BufWriter, Read, Write},
    path::Path,
};
#[derive(Serialize, Deserialize)]
struct TrackState {
    #[serde(default)]
    deck: crate::deck::Settings,
    #[serde(default)]
    origin: u64,
    #[serde(default)]
    kit_frames: [usize; 16],
    #[serde(default)]
    xpm: Option<crate::xpm::Meta>,
    #[serde(default)]
    xpm_frames: [[usize; 3]; 16],
    #[serde(default)]
    secondary_frames: [usize; 16],
    #[serde(default)]
    layer_names: [[String; 2]; 16],
    #[serde(default)]
    kit_name: String,
    #[serde(default)]
    kit_id: String,
    #[serde(default)]
    record_mode: RecordMode,
    #[serde(default)]
    source_frames: usize,
    #[serde(default)]
    source_ticks: u64,
    #[serde(default)]
    audio_sequence: bool,
    #[serde(default)]
    audio_settings: crate::audio::Settings,
    #[serde(default)]
    audio_target: crate::audio::RecordTarget,
    #[serde(default)]
    chops: crate::chop::Chops,
    #[serde(default)]
    generated: bool,
    #[serde(default)]
    generated_sounds: u16,
    #[serde(default)]
    pattern: crate::music::PatternConfig,
    #[serde(default)]
    chord: crate::music::ChordConfig,
    #[serde(default)]
    chord_mode: bool,
    #[serde(default)]
    play: crate::performance::Settings,
    kind: Kind,
    name: String,
    mix: Mix,
    synth: Synth,
    drums: [Drum; 16],
    fx: [Fx; 2],
    route: Route,
    #[serde(default)]
    output: OutputTarget,
    length_ticks: u64,
    audio_frames: usize,
    events: Vec<Event>,
}
#[derive(Serialize, Deserialize)]
struct State {
    #[serde(default)]
    timing_revision: u8,
    #[serde(default)]
    swing: Option<crate::swing::Basis>,
    #[serde(default)]
    transport_frame: u64,
    #[serde(default)]
    chain: crate::chain::Config,
    version: u32,
    rate: u32,
    bpm: f64,
    preroll: u8,
    master: f32,
    performance: [Fx; 2],
    bus_fx: [Fx; 2],
    master_fx: Fx,
    #[serde(default)]
    sidechain: crate::motion::Sidechain,
    #[serde(default)]
    texture: crate::motion::Texture,
    tracks: Vec<TrackState>,
}
fn invalid(s: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, s)
}
pub fn save(engine: &Engine, path: &Path) -> io::Result<()> {
    if !engine.is_idle() {
        return Err(io::Error::new(
            io::ErrorKind::WouldBlock,
            "Stop before saving",
        ));
    }
    let state = State {
        chain: engine.chain.config,
        version: 10,
        timing_revision: 1,
        swing: engine.swing,
        transport_frame: engine.clock.frame,
        rate: engine.clock.rate,
        bpm: engine.clock.bpm,
        preroll: engine.preroll,
        master: engine.master,
        performance: engine.performance,
        bus_fx: engine.saved_bus_fx(),
        master_fx: engine.master_fx,
        sidechain: engine.saved_sidechain(),
        texture: engine.saved_texture(),
        tracks: (0..16)
            .map(|i| {
                let t = &engine.tracks[i];
                TrackState {
                    deck: t.deck,
                    origin: t.origin,
                    kit_frames: t.kit.clips.each_ref().map(Vec::len),
                    xpm: t.kit.xpm.as_ref().map(|x| x.meta),
                    xpm_frames: t.kit.xpm.as_ref().map_or([[0; 3]; 16], |x| {
                        x.extra.each_ref().map(|a| a.each_ref().map(Vec::len))
                    }),
                    secondary_frames: t.kit.secondary.each_ref().map(Vec::len),
                    layer_names: t.kit.names.clone(),
                    kit_name: t.kit.name.clone(),
                    kit_id: t.kit.id.clone(),
                    record_mode: t.record_mode,
                    source_frames: t.source_frames,
                    source_ticks: t.source_ticks,
                    audio_sequence: t.audio_sequence,
                    audio_settings: t.audio_settings,
                    audio_target: t.audio_target,
                    chops: t.chops,
                    generated: t.generated,
                    generated_sounds: t.generated_sounds,
                    pattern: t.pattern,
                    chord: t.chord,
                    chord_mode: t.chord_mode,
                    play: t.play,
                    kind: t.kind,
                    name: t.name.clone(),
                    mix: t.mix,
                    synth: t.synth,
                    drums: t.drums,
                    fx: engine.saved_track_fx(i),
                    route: t.route,
                    output: t.output,
                    length_ticks: t.length_ticks,
                    audio_frames: t.audio_frames,
                    events: t.events.clone(),
                }
            })
            .collect(),
    };
    let bytes = serde_json::to_vec(&state)?;
    if bytes.len() > 8 * 1024 * 1024 {
        return Err(invalid("Metadata exceeds limit"));
    }
    let parent = path.parent().unwrap_or(Path::new("."));
    fs::create_dir_all(parent)?;
    let mut filename = path
        .file_name()
        .ok_or_else(|| invalid("Missing filename"))?
        .to_os_string();
    filename.push(format!(
        ".{}.{}.new",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    ));
    let temp = parent.join(filename);
    let result = (|| {
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp)?;
        let mut f = BufWriter::with_capacity(65536, file);
        f.write_all(b"LC01")?;
        f.write_all(&(bytes.len() as u32).to_le_bytes())?;
        f.write_all(&bytes)?;
        for t in &engine.tracks {
            for s in &t.audio[..t.audio_frames] {
                f.write_all(&s.l.to_le_bytes())?;
                f.write_all(&s.r.to_le_bytes())?;
            }
            for clip in t.kit.clips.iter().chain(t.kit.secondary.iter()) {
                for sample in clip {
                    f.write_all(&sample.l.to_le_bytes())?;
                    f.write_all(&sample.r.to_le_bytes())?;
                }
            }
            if let Some(x) = &t.kit.xpm {
                for clip in x.extra.iter().flatten() {
                    for sample in clip {
                        f.write_all(&sample.l.to_le_bytes())?;
                        f.write_all(&sample.r.to_le_bytes())?;
                    }
                }
            }
        }
        f.flush()?;
        f.get_ref().sync_all()?;
        drop(f);
        fs::rename(&temp, path)?;
        if let Ok(dir) = File::open(parent) {
            let _ = dir.sync_all();
        }
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temp);
    }
    result
}
pub fn load(path: &Path, expected_rate: u32, capacity: usize) -> io::Result<Engine> {
    let mut f = BufReader::with_capacity(65536, File::open(path)?);
    let mut header = [0; 8];
    f.read_exact(&mut header)?;
    if &header[..4] != b"LC01" {
        return Err(invalid("Not a Loop Club project"));
    }
    let size = u32::from_le_bytes(header[4..].try_into().unwrap()) as usize;
    if size > 8 * 1024 * 1024 {
        return Err(invalid("Metadata exceeds limit"));
    }
    let mut bytes = vec![0; size];
    f.read_exact(&mut bytes)?;
    let s: State = serde_json::from_slice(&bytes)?;
    if s.tracks.len() != 16
        || !(1..=10).contains(&s.version)
        || s.transport_frame > u64::from(s.rate) * u64::from(u32::MAX)
        || s.rate != expected_rate
        || !s.bpm.is_finite()
        || !(40.0..=240.0).contains(&s.bpm)
        || s.preroll > 2
        || s.timing_revision > 1
        || s.swing.is_some_and(|v| !v.valid())
        || !s.chain.valid()
        || !s.master.is_finite()
        || !(0.0..=1.0).contains(&s.master)
        || !s.performance.iter().all(Fx::valid)
        || !s.bus_fx.iter().all(Fx::valid)
        || !s.master_fx.valid()
        || !s.sidechain.valid()
        || !s.texture.valid()
    {
        return Err(invalid("Unsupported project version/rate/settings"));
    }
    let mut total = 0_u64;
    let mut kit_total = 0usize;
    for t in &s.tracks {
        if s.version >= 7
            && t.origin
                > crate::clock::Clock::new(s.rate, s.bpm)
                    .tick_at(s.transport_frame)
                    .saturating_add(16 * PPQ)
        {
            return Err(invalid("Invalid saved loop origin"));
        }
        let frames = t
            .kit_frames
            .iter()
            .chain(t.secondary_frames.iter())
            .chain(t.xpm_frames.iter().flatten())
            .try_fold(0usize, |a, b| a.checked_add(*b))
            .ok_or_else(|| invalid("Kit size overflow"))?;
        if frames > crate::bank::KIT_BYTES / 8
            || t.kit_frames
                .iter()
                .chain(t.secondary_frames.iter())
                .chain(t.xpm_frames.iter().flatten())
                .any(|n| *n > expected_rate as usize * 16)
            || t.layer_names
                .iter()
                .flatten()
                .any(|n| n.len() > 128 || n.chars().any(char::is_control))
            || s.version < 3 && t.secondary_frames.iter().any(|n| *n != 0)
            || t.kit_name.len() > 128
            || t.kit_id.len() > 192
            || t.kit_name.chars().any(char::is_control)
            || t.kit_id.chars().any(char::is_control)
        {
            return Err(invalid("Invalid saved kit"));
        }
        if let Some(meta) = t.xpm {
            // Kit data survives SOURCE changes, just like the pre-XPM kit layers.
            // Validate all dormant zones too; they become audible only on Drum tracks.
            if s.version < 6 || !meta.valid() {
                return Err(invalid("Invalid XPM metadata"));
            }
            for pad in 0..16 {
                let count = usize::from(meta.pads[pad].count);
                if count > 0 && t.kit_frames[pad] == 0 {
                    return Err(invalid("Missing XPM primary layer"));
                }
                for z in 0..3 {
                    if (z + 1 < count) != (t.xpm_frames[pad][z] > 0) {
                        return Err(invalid("XPM layer frame count mismatch"));
                    }
                }
            }
        } else if t.xpm_frames.iter().flatten().any(|n| *n > 0) {
            return Err(invalid("XPM PCM without metadata"));
        }
        kit_total = kit_total
            .checked_add(frames * 8)
            .ok_or_else(|| invalid("Kit memory overflow"))?;
        if kit_total > crate::bank::SESSION_KIT_BYTES {
            return Err(invalid("Saved kits exceed memory limit"));
        }
        total = total
            .checked_add(frames as u64 * 8)
            .ok_or_else(|| invalid("Kit file size overflow"))?;
        if !t.deck.valid()
            || !t.pattern.valid()
            || !t.chord.valid()
            || !t.play.valid()
            || t.generated && t.kind != Kind::Drum
        {
            return Err(invalid("Invalid musical mode"));
        }
        if t.name.len() > 32
            || t.name.chars().any(char::is_control)
            || !t.mix.valid()
            || !t.synth.valid()
            || !t.drums.iter().all(Drum::valid)
            || !t.fx.iter().all(Fx::valid)
            || !t.audio_settings.valid()
            || t.source_ticks > PPQ * 4 * 4096
            || t.source_frames > capacity
            || t.audio_frames > capacity + (s.rate as usize * 6)
            || t.events.len() > MAX_EVENTS
            || t.length_ticks > PPQ * 4 * 4096
        {
            return Err(invalid("Invalid track settings"));
        }
        if (t.kind != Kind::Audio && t.audio_frames != 0)
            || (t.kind == Kind::Audio && !t.audio_sequence && !t.events.is_empty())
            || (t.audio_sequence
                && (t.kind != Kind::Audio || t.audio_frames == 0 || t.source_ticks == 0))
        {
            return Err(invalid("Content does not match track kind"));
        }
        let source = if t.source_frames == 0 {
            t.audio_frames
        } else {
            t.source_frames
        };
        if source > capacity || source > t.audio_frames {
            return Err(invalid("Invalid source length"));
        }
        if t.kind == Kind::Audio
            && source > 0
            && !t.chops.slices.iter().all(|r| r.start == 0 && r.end == 0)
            && !t.chops.valid(source)
        {
            return Err(invalid("Invalid chop regions"));
        }
        if t.length_ticks == 0 && (t.audio_frames != 0 || !t.events.is_empty()) {
            return Err(invalid("Content in empty track"));
        }
        if t.events.windows(2).any(|e| e[0].tick > e[1].tick)
            || t.events.iter().any(|e| {
                e.deck.is_some_and(|d| {
                    !d.valid() || t.kind != Kind::Audio || !t.audio_sequence || e.slice
                }) || e.timing.is_some_and(|v| !v.valid())
                    || e.note > 127
                    || e.velocity > 127
                    || e.tick >= t.length_ticks
                    || (e.slice && (t.kind != Kind::Audio || e.note >= t.chops.count))
            })
        {
            return Err(invalid("Invalid MIDI events"));
        }
        total = total
            .checked_add(t.audio_frames as u64 * 8)
            .ok_or_else(|| invalid("Size overflow"))?;
    }
    if f.get_ref().metadata()?.len() != 8 + size as u64 + total {
        return Err(invalid("Truncated or trailing project data"));
    }
    let mut e = Engine::new(expected_rate, s.bpm, capacity);
    e.swing = s.swing;
    if s.version >= 7 {
        e.clock.frame = s.transport_frame;
    }
    e.preroll = s.preroll;
    e.chain.config = s.chain;
    e.master = s.master;
    e.master_gain = s.master;
    e.performance = if s.version >= 4 {
        s.performance
    } else {
        s.performance.map(|fx| Fx {
            effect: Effect::Off,
            ..fx
        })
    };
    e.bus_fx = s.bus_fx;
    e.master_fx = s.master_fx;
    e.sidechain = s.sidechain;
    e.texture = s.texture;
    for (i, t) in s.tracks.into_iter().enumerate() {
        let dst = &mut e.tracks[i];
        dst.generated = t.generated;
        dst.generated_sounds = t.generated_sounds;
        dst.deck = t.deck;
        dst.pattern = t.pattern;
        dst.chord = t.chord;
        dst.chord_mode = t.chord_mode;
        dst.play = t.play;
        dst.audio_settings = t.audio_settings;
        dst.audio_target = t.audio_target;
        dst.source_ticks = if t.kind == Kind::Audio && t.source_ticks == 0 {
            t.length_ticks
        } else {
            t.source_ticks
        };
        dst.audio_sequence = t.audio_sequence;
        dst.source_frames = if t.source_frames == 0 {
            t.audio_frames
        } else {
            t.source_frames
        };
        dst.chops = t.chops;
        if t.kind == Kind::Audio
            && dst.source_frames > 0
            && dst.chops.slices.iter().all(|r| r.start == 0 && r.end == 0)
        {
            dst.chops.divide(dst.source_frames);
        }
        if t.audio_frames > dst.audio.len() {
            dst.audio.resize(t.audio_frames, Frame::ZERO);
        }
        dst.origin = if s.version >= 7 { t.origin } else { 0 };
        dst.kind = t.kind;
        dst.record_mode = t.record_mode;
        dst.name = t.name;
        dst.mix = t.mix;
        dst.gain = t.mix.level;
        dst.synth = t.synth;
        dst.drums = t.drums;
        dst.sound_gain = t.drums.map(|d| d.mix.level);
        dst.fx = t.fx;
        dst.route = t.route;
        dst.output = t.output;
        dst.main_gain = if t.output == OutputTarget::Usb {
            0.
        } else {
            1.
        };
        dst.usb_gain = if t.output == OutputTarget::Main {
            0.
        } else {
            1.
        };
        e.separate_usb |= t.output != OutputTarget::Both;
        dst.length_ticks = t.length_ticks;
        dst.audio_frames = t.audio_frames;
        let mut previous_deck: Option<crate::deck::Control> = None;
        dst.events.extend(t.events.into_iter().map(|mut event| {
            if s.version < 10
                && let Some(mut control) = event.deck
            {
                control.returning = control.grabbed
                    && previous_deck.map_or(control.fader > control.anchor_fader, |prev| {
                        if prev.grabbed {
                            if control.fader == prev.fader {
                                prev.returning
                            } else {
                                control.fader > prev.fader
                            }
                        } else {
                            false
                        }
                    });
                previous_deck = Some(control);
                event.deck = Some(control);
            }
            let lane = event.note % 16;
            if t.kind == Kind::Drum && t.generated_sounds & (1 << lane) != 0 {
                if s.version < 9 {
                    event.timing = crate::music::swing_basis(t.pattern, lane);
                }
                if s.timing_revision == 0 {
                    if lane == 3 {
                        event.timing = None;
                    } else if matches!(lane, 12 | 14)
                        && (event.timing.is_none() || s.version < 9)
                        && t.pattern.swing > 50.
                    {
                        // Original ride/bell generation used only quarter/eighth grids.
                        event.timing = Some(crate::swing::Basis {
                            amount: t.pattern.swing,
                            sixteenth: false,
                        });
                    }
                }
            }
            event
        }));
        for (index, frame) in dst.audio[..dst.audio_frames].iter_mut().enumerate() {
            let mut sample = [0; 8];
            f.read_exact(&mut sample)?;
            *frame = Frame {
                l: f32::from_le_bytes(sample[..4].try_into().unwrap()),
                r: f32::from_le_bytes(sample[4..].try_into().unwrap()),
            };
            if index < dst.source_frames {
                dst.features.feed(*frame, s.rate);
            }
            if !frame.finite() || frame.peak() > 8. {
                return Err(invalid("Invalid audio sample"));
            }
        }
        dst.kit.names = t.layer_names;
        dst.kit.name = t.kit_name;
        dst.kit.id = t.kit_id;
        for (index, frames) in t
            .kit_frames
            .iter()
            .chain(t.secondary_frames.iter())
            .enumerate()
        {
            let clip = if index < 16 {
                &mut dst.kit.clips[index]
            } else {
                &mut dst.kit.secondary[index - 16]
            };
            clip.resize(*frames, Frame::ZERO);
            for sample in clip {
                let mut raw = [0; 8];
                f.read_exact(&mut raw)?;
                *sample = Frame {
                    l: f32::from_le_bytes(raw[..4].try_into().unwrap()),
                    r: f32::from_le_bytes(raw[4..].try_into().unwrap()),
                };
                if !sample.finite() || sample.peak() > 8. {
                    return Err(invalid("Invalid saved kit PCM"));
                }
            }
        }
        if let Some(meta) = t.xpm {
            let mut x = Box::new(crate::xpm::Data {
                meta,
                ..Default::default()
            });
            for pad in 0..16 {
                for zone in 0..3 {
                    let clip = &mut x.extra[pad][zone];
                    clip.resize(t.xpm_frames[pad][zone], Frame::ZERO);
                    for sample in clip {
                        let mut raw = [0; 8];
                        f.read_exact(&mut raw)?;
                        *sample = Frame {
                            l: f32::from_le_bytes(raw[..4].try_into().unwrap()),
                            r: f32::from_le_bytes(raw[4..].try_into().unwrap()),
                        };
                        if !sample.finite() || sample.peak() > 8. {
                            return Err(invalid("Invalid XPM PCM"));
                        }
                    }
                }
            }
            dst.kit.xpm = Some(x);
        }
    }
    Ok(e)
}

/// Development QA save/load must never target the owner's regular session file.
pub fn session_directory(base: &std::path::Path, qa: bool) -> std::path::PathBuf {
    if qa {
        let root = base.join("QA");
        if let Some(id) = option_env!("LOOP_CLUB_QA_SESSION")
            && !id.is_empty()
            && id.len() <= 48
            && id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
        {
            return root.join(id);
        }
        root
    } else {
        base.to_path_buf()
    }
}

/// Project selection never resets the physical MAIN VOLUME control. Device
/// controls are independent of the saved mix; preserve their current smoothing.
pub fn load_for_replacement(
    path: &Path,
    rate: u32,
    capacity: usize,
    live: &Engine,
) -> io::Result<Engine> {
    let mut loaded = load(path, rate, capacity)?;
    loaded.selected = live.selected.min(ACTIVE_TRACKS - 1);
    loaded.hardware_volume = live.hardware_volume;
    loaded.hardware_gain = live.hardware_gain;
    Ok(loaded)
}
