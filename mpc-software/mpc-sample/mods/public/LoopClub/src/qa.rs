//! Opt-in USB-only device QA. Screen readback follows the active DRM framebuffer.
//! Requests enqueue controller events; only the owning paint callback applies them.
use crate::{queue::Queue, ui::Snapshot};
use serde::Deserialize;
use std::{
    io::{Read, Write},
    net::{Ipv4Addr, TcpListener, TcpStream},
    sync::{
        OnceLock,
        atomic::{AtomicU64, Ordering},
    },
    time::{Duration, Instant},
};
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Input {
    TransportButton {
        code: u32,
        on: bool,
        #[serde(default)]
        shift: bool,
        #[serde(default)]
        erase: bool,
    },
    Button {
        code: u32,
        on: bool,
        #[serde(default)]
        shift: bool,
    },
    Knob {
        index: u32,
        value: f32,
        #[serde(default)]
        shift: bool,
    },
    Encoder {
        delta: i32,
    },
    Pad {
        index: usize,
        on: bool,
        velocity: u8,
        #[serde(default)]
        shift: bool,
        #[serde(default)]
        erase: bool,
    },
}
impl Input {
    pub fn valid(&self) -> bool {
        match *self {
            Self::Button { code, .. } | Self::TransportButton { code, .. } => [
                0, 1, 6, 7, 9, 12, 13, 14, 36, 40, 49, 54, 55, 73, 74, 75, 80, 81, 82, 111,
            ]
            .contains(&code),
            Self::Knob { index, value, .. } => {
                (16..=20).contains(&index) && value.is_finite() && (0.0..=1.0).contains(&value)
            }
            Self::Encoder { delta } => delta.unsigned_abs() <= 16,
            Self::Pad {
                index, velocity, ..
            } => index < 16 && velocity <= 127,
        }
    }
}
static INPUT: OnceLock<Queue<Input, 64>> = OnceLock::new();
static STATE: OnceLock<crate::runtime::AtomicSnapshot> = OnceLock::new();
static BUTTONS: AtomicU64 = AtomicU64::new(0);
static PAINTS: AtomicU64 = AtomicU64::new(0);
static APPLIED: AtomicU64 = AtomicU64::new(0);
static PAGE: AtomicU64 = AtomicU64::new(0);
static OUT_PCM: [AtomicU64; 4] = [const { AtomicU64::new(0) }; 4];
static OUT_INFO: [AtomicU64; 4] = [const { AtomicU64::new(0) }; 4];
static OUT_PEAK: [AtomicU64; 4] = [const { AtomicU64::new(0) }; 4];
static OUT_CALLS: [AtomicU64; 4] = [const { AtomicU64::new(0) }; 4];
pub fn output_probe(info: &mpc_sample_sdk::audio::OutputInfo, selected: bool) {
    if STATE.get().is_none() {
        return;
    }
    let pcm = info.pcm() as u64;
    for i in 0..4 {
        let old = OUT_PCM[i].load(Ordering::Relaxed);
        if old == pcm
            || old == 0
                && OUT_PCM[i]
                    .compare_exchange(0, pcm, Ordering::Relaxed, Ordering::Relaxed)
                    .is_ok()
        {
            OUT_INFO[i].store(
                info.block_frames() as u64
                    | (u64::from(info.period_out()) << 16)
                    | (u64::from(info.rate()) << 32)
                    | (u64::from(selected) << 63),
                Ordering::Relaxed,
            );
            OUT_CALLS[i].fetch_add(1, Ordering::Relaxed);
            break;
        }
    }
}
pub fn output_peak(pcm: usize, samples: &[i32]) {
    if STATE.get().is_none() {
        return;
    }
    for i in 0..4 {
        if OUT_PCM[i].load(Ordering::Relaxed) == pcm as u64 {
            let peak = samples
                .iter()
                .map(|x| x.unsigned_abs() as u64)
                .max()
                .unwrap_or(0);
            OUT_PEAK[i].fetch_max(peak, Ordering::Relaxed);
            break;
        }
    }
}
fn project_storage() -> serde_json::Value {
    let path = std::env::var_os("LOOP_CLUB_DATA_DIR")
        .map(std::path::PathBuf::from)
        .map(|p| crate::project::session_directory(&p, enabled()));
    serde_json::json!({"qa":enabled(),"session_id":option_env!("LOOP_CLUB_QA_SESSION"),"directory":path})
}
fn output_diagnostics() -> serde_json::Value {
    serde_json::json!((0..4).map(|i| {let info=OUT_INFO[i].load(Ordering::Relaxed);serde_json::json!({"pcm":OUT_PCM[i].load(Ordering::Relaxed),"block_frames":info&65535,"period_frames":(info>>16)&65535,"rate":(info>>32)&0x7fffffff,"selected":info>>63!=0,"calls":OUT_CALLS[i].load(Ordering::Relaxed),"mixed_peak":OUT_PEAK[i].load(Ordering::Relaxed) as f64 / 2147483648.0})}).collect::<Vec<_>>())
}
static AUDIO_TIME: [AtomicU64; 101] = [const { AtomicU64::new(0) }; 101];
static AUDIO_MAX: AtomicU64 = AtomicU64::new(0);
pub fn output_start() -> Option<Instant> {
    STATE.get().map(|_| Instant::now())
}
pub fn output_finish(start: Option<Instant>, frames: usize, rate: u32) {
    if let Some(start) = start {
        if frames == 0 || rate == 0 {
            return;
        }
        let units = (start.elapsed().as_nanos() * u128::from(rate) / frames as u128 / 1000)
            .min(u128::from(u64::MAX)) as u64;
        AUDIO_TIME[(units / 50000).min(100) as usize].fetch_add(1, Ordering::Relaxed);
        AUDIO_MAX.fetch_max(units, Ordering::Relaxed);
    }
}
fn audio_timing() -> serde_json::Value {
    let bins: Vec<_> = AUDIO_TIME
        .iter()
        .map(|v| v.load(Ordering::Relaxed))
        .collect();
    let total: u64 = bins.iter().sum();
    let target = (total * 99).div_ceil(100);
    let mut seen = 0;
    let mut p99 = None;
    for (i, n) in bins.iter().enumerate() {
        seen += n;
        if total > 0 && seen >= target {
            p99 = Some(if i == 100 {
                AUDIO_MAX.load(Ordering::Relaxed) as f64 / 10000.0
            } else {
                (i + 1) as f64 * 5.0
            });
            break;
        }
    }
    serde_json::json!({"scope":"actual primary callback elapsed time, including scheduler interruptions; QA instrumentation only","blocks":total,"p99_deadline_pct_upper":p99,"max_deadline_pct":AUDIO_MAX.load(Ordering::Relaxed) as f64/10000.0})
}
static PROFILE_TICKS: AtomicU64 = AtomicU64::new(0);
static PROFILE_TIME: [AtomicU64; 12] = [const { AtomicU64::new(0) }; 12];
static PROFILE_COUNT: [AtomicU64; 12] = [const { AtomicU64::new(0) }; 12];
pub struct DspSpan {
    last: Instant,
    band: usize,
}
pub fn dsp_start(voices: impl FnOnce() -> usize) -> Option<DspSpan> {
    if STATE.get().is_none() || PROFILE_TICKS.fetch_add(1, Ordering::Relaxed) & 255 != 0 {
        return None;
    }
    let n = voices();
    Some(DspSpan {
        last: Instant::now(),
        band: if n < 16 {
            0
        } else if n < 40 {
            1
        } else {
            2
        },
    })
}
pub fn dsp_mark(span: &mut Option<DspSpan>, section: usize) {
    if let Some(span) = span {
        let now = Instant::now();
        let i = span.band * 4 + section;
        PROFILE_TIME[i].fetch_add(
            now.duration_since(span.last).as_nanos() as u64,
            Ordering::Relaxed,
        );
        PROFILE_COUNT[i].fetch_add(1, Ordering::Relaxed);
        span.last = now;
    }
}
fn dsp_profile() -> serde_json::Value {
    serde_json::json!((0..3).map(|band|serde_json::json!({"voices":(["0-15","16-39","40-64"][band]),"sections":(0..4).map(|j|{let i=band*4+j;let n=PROFILE_COUNT[i].load(Ordering::Relaxed);serde_json::json!({"section":(["schedule","voices","track_mix_fx","shared_master_fx"][j]),"samples":n,"mean_microseconds_per_frame":if n==0{0.0}else{PROFILE_TIME[i].load(Ordering::Relaxed) as f64/n as f64/1000.0}})}).collect::<Vec<_>>() })).collect::<Vec<_>>())
}
static IO_SEQUENCE: AtomicU64 = AtomicU64::new(0);
static IO_RESULT: AtomicU64 = AtomicU64::new(0);
pub fn io_result(result: u8) {
    if STATE.get().is_some() {
        IO_RESULT.store(u64::from(result), Ordering::Release);
        IO_SEQUENCE.fetch_add(1, Ordering::AcqRel);
    }
}
thread_local! {static REMOTE_CONTROL:std::cell::Cell<bool>=const{std::cell::Cell::new(false)};}
static REMOTE_CONTROL_EDGES: AtomicU64 = AtomicU64::new(0);
static REMOTE_CONTROL_FLOW: AtomicU64 = AtomicU64::new(0);
pub fn injected_control(call: impl FnOnce() -> mpc_sample_sdk::Flow) {
    REMOTE_CONTROL_EDGES.fetch_add(1, Ordering::Relaxed);
    REMOTE_CONTROL.with(|v| v.set(true));
    let flow = call();
    REMOTE_CONTROL.with(|v| v.set(false));
    REMOTE_CONTROL_FLOW.store(
        u64::from(flow == mpc_sample_sdk::Flow::Consume),
        Ordering::Relaxed,
    );
}
static PHYSICAL_COUNT: AtomicU64 = AtomicU64::new(0);
static PHYSICAL_BUTTONS: [AtomicU64; 32] = [const { AtomicU64::new(0) }; 32];
pub fn physical_button(code: u32, pressed: bool, shift: bool) {
    if STATE.get().is_some() {
        let n = PHYSICAL_COUNT.fetch_add(1, Ordering::Relaxed);
        PHYSICAL_BUTTONS[n as usize % 32].store(
            u64::from(code)
                | (u64::from(pressed) << 32)
                | (u64::from(shift) << 33)
                | (u64::from(REMOTE_CONTROL.with(std::cell::Cell::get)) << 34),
            Ordering::Release,
        );
    }
}
static UI_POSITION: AtomicU64 = AtomicU64::new(0);
static SURFACES: AtomicU64 = AtomicU64::new(0);
static LAST_SURFACE: AtomicU64 = AtomicU64::new(0);
/// Read-only input observation outside the Mod UI mutex. Never consumes events.
pub fn surface(event: mpc_sample_sdk::SurfaceEvent) -> mpc_sample_sdk::Flow {
    if STATE.get().is_some() {
        LAST_SURFACE.store(
            (event.code as u32 as u64) << 32
                | u64::from(event.kind) << 16
                | u64::from(event.note) << 8
                | u64::from(event.velocity),
            Ordering::Relaxed,
        );
        SURFACES.fetch_add(1, Ordering::Relaxed);
    }
    mpc_sample_sdk::Flow::Forward
}
static REMOTE_BUTTONS: AtomicU64 = AtomicU64::new(0);
pub fn remote_button() {
    REMOTE_BUTTONS.fetch_add(1, Ordering::Relaxed);
}
pub fn build_id() -> &'static str {
    option_env!("LOOP_CLUB_BUILD_ID").unwrap_or("development")
}
pub fn enabled() -> bool {
    if build_id() == "qa5" {
        return true;
    }
    std::env::var("LOOP_CLUB_QA").is_ok_and(|s| s == "1")
}
pub fn button() {
    BUTTONS.fetch_add(1, Ordering::Relaxed);
}
pub fn paint(ui: &crate::ui::Ui, s: &Snapshot, browser: bool) {
    let page = ui.page as u64;
    UI_POSITION.store(
        ui.selected as u64
            | (u64::from(ui.synth_page) << 8)
            | (u64::from(ui.chord_modes[ui.selected]) << 16)
            | (u64::from(browser) << 17)
            | ((ui.layer as u64) << 18)
            | (u64::from(ui.layer_detail) << 19)
            | ((ui.sound as u64) << 20),
        Ordering::Relaxed,
    );
    if let Some(state) = STATE.get() {
        state.publish(s);
        PAGE.store(page, Ordering::Relaxed);
        PAINTS.fetch_add(1, Ordering::Relaxed);
    }
}
pub fn pop() -> Option<Input> {
    // SAFETY: only serialized owning UI callbacks consume the QA queue.
    let item = INPUT.get().and_then(|q| unsafe { q.pop_shared() });
    if item.is_some() {
        APPLIED.fetch_add(1, Ordering::Relaxed);
    }
    item
}
pub fn bmp(
    raw: &[u8],
    width: usize,
    height: usize,
    stride: usize,
    bpp: usize,
) -> Result<Vec<u8>, &'static str> {
    if width != 320
        || height != 240
        || ![16, 32].contains(&bpp)
        || stride < width * (bpp / 8)
        || stride > 8192
        || raw.len() < stride * height
    {
        return Err("Unsupported framebuffer geometry");
    }
    let mut out = vec![0; 54 + width * height * 3];
    out[..2].copy_from_slice(b"BM");
    let size = out.len() as u32;
    out[2..6].copy_from_slice(&size.to_le_bytes());
    out[10..14].copy_from_slice(&54u32.to_le_bytes());
    out[14..18].copy_from_slice(&40u32.to_le_bytes());
    out[18..22].copy_from_slice(&(width as i32).to_le_bytes());
    out[22..26].copy_from_slice(&(-(height as i32)).to_le_bytes());
    out[26..28].copy_from_slice(&1u16.to_le_bytes());
    out[28..30].copy_from_slice(&24u16.to_le_bytes());
    for y in 0..height {
        for x in 0..width {
            let a = y * stride + x * (bpp / 8);
            let p = 54 + (y * width + x) * 3;
            if bpp == 32 {
                out[p..p + 3].copy_from_slice(&raw[a..a + 3]);
            } else {
                let v = u16::from_le_bytes([raw[a], raw[a + 1]]);
                out[p] = ((v & 31) * 255 / 31) as u8;
                out[p + 1] = (((v >> 5) & 63) * 255 / 63) as u8;
                out[p + 2] = (((v >> 11) & 31) * 255 / 31) as u8;
            }
        }
    }
    Ok(out)
}
fn reply(stream: &mut TcpStream, status: &str, kind: &str, body: &[u8]) {
    let header = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {kind}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        body.len()
    );
    let _ = stream
        .write_all(header.as_bytes())
        .and_then(|_| stream.write_all(body));
}
fn serve(mut stream: TcpStream) {
    if !stream.peer_addr().is_ok_and(|a|matches!(a.ip(),std::net::IpAddr::V4(ip)if ip.octets()[..3]==[192,168,155]&&ip.octets()[3]<16)){return;}
    let _ = stream.set_read_timeout(Some(Duration::from_secs(2)));
    let _ = stream.set_write_timeout(Some(Duration::from_secs(2)));
    let mut raw = Vec::new();
    let mut b = [0; 1024];
    let header_end = loop {
        if raw.len() > 4096 {
            return;
        }
        let Ok(n) = stream.read(&mut b) else {
            return;
        };
        if n == 0 {
            return;
        }
        raw.extend_from_slice(&b[..n]);
        if let Some(i) = raw.windows(4).position(|b| b == b"\r\n\r\n") {
            break i + 4;
        }
    };
    let header = String::from_utf8_lossy(&raw[..header_end]).to_lowercase();
    let first = header.lines().next().unwrap_or("");
    if first.starts_with("get /usb-capabilities.json ") {
        let data = serde_json::to_vec(&crate::usb_bus::capabilities()).unwrap_or_default();
        reply(&mut stream, "200 OK", "application/json", &data);
        return;
    }
    if first.starts_with("get /boot-storage.json ") {
        let result = crate::qa_persist::inspect();
        match result {
            Ok(value) => reply(
                &mut stream,
                "200 OK",
                "application/json",
                &serde_json::to_vec(&value).unwrap_or_default(),
            ),
            Err(e) => reply(&mut stream, "409 Conflict", "text/plain", e.as_bytes()),
        };
        return;
    }
    if first.starts_with("post /persist ") {
        if header.contains("\r\norigin:")
            || !header
                .lines()
                .any(|s| s == "x-loop-club-management: persist-v1")
        {
            reply(
                &mut stream,
                "403 Forbidden",
                "text/plain",
                b"Explicit QA persistence header required",
            );
            return;
        }
        let len = header
            .lines()
            .find_map(|s| {
                s.strip_prefix("content-length:")?
                    .trim()
                    .parse::<usize>()
                    .ok()
            })
            .unwrap_or(0);
        if len == 0 || len > 512 {
            reply(
                &mut stream,
                "400 Bad Request",
                "text/plain",
                b"Invalid request length",
            );
            return;
        }
        while raw.len() < header_end + len {
            let Ok(n) = stream.read(&mut b) else { return };
            if n == 0 {
                return;
            };
            raw.extend_from_slice(&b[..n]);
        }
        let snapshot = STATE.get().and_then(|s| s.read());
        if !snapshot.is_some_and(|s| s.words[1] >= 8000 && s.words[4] & 1 == 0 && s.words[9] == 0) {
            reply(
                &mut stream,
                "409 Conflict",
                "text/plain",
                b"Stop playback/recording and wait for engine readiness first",
            );
            return;
        }
        #[derive(serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Commit {
            previous: String,
            runtime: String,
        }
        let result = (|| {
            let commit: Commit = serde_json::from_slice(&raw[header_end..header_end + len])
                .map_err(|e| e.to_string())?;
            let root = crate::qa_persist::device_root()?;
            crate::qa_persist::publish(
                &root,
                std::path::Path::new("/run/loop-club.so"),
                &commit.previous,
                &commit.runtime,
            )
        })();
        match result {
            Ok(value) => reply(
                &mut stream,
                "200 OK",
                "application/json",
                &serde_json::to_vec(&value).unwrap_or_default(),
            ),
            Err(e) => reply(&mut stream, "409 Conflict", "text/plain", e.as_bytes()),
        };
        return;
    }
    if first.starts_with("get /screen.bmp ") {
        match crate::drm_capture::screen() {
            Ok(b) => reply(&mut stream, "200 OK", "image/bmp", &b),
            Err(e) => reply(
                &mut stream,
                "503 Service Unavailable",
                "text/plain",
                e.as_bytes(),
            ),
        }
        return;
    }
    if first.starts_with("get /status.json ") {
        let s = STATE.get().and_then(|s| s.read()).unwrap_or_default();
        let body=serde_json::to_vec(&serde_json::json!({"source":"active DRM framebuffer + live Loop Club state","build_id":build_id(),"ui_revision":"looper-first-layers","pid":std::process::id(),"voice_limit":crate::engine::VOICES,"sdk_patch_sha256":option_env!("LOOP_CLUB_SDK_PATCH"),"input_transport":"independent SDK callbacks + owning UI queue","physical_button_edges":PHYSICAL_COUNT.load(Ordering::Acquire).saturating_sub(REMOTE_CONTROL_EDGES.load(Ordering::Acquire)),"remote_transport_edges":REMOTE_CONTROL_EDGES.load(Ordering::Acquire),"remote_transport_consumed":REMOTE_CONTROL_FLOW.load(Ordering::Acquire)!=0,"physical_button_trace":PHYSICAL_BUTTONS.iter().map(|v|v.load(Ordering::Acquire)).collect::<Vec<_>>(),"project_storage":project_storage(),"project_io":{"sequence":IO_SEQUENCE.load(Ordering::Acquire),"result":IO_RESULT.load(Ordering::Acquire)},"output_streams":output_diagnostics(),"audio_timing":audio_timing(),"dsp_profile":dsp_profile(),"ui_position":UI_POSITION.load(Ordering::Relaxed),"surface_callbacks":SURFACES.load(Ordering::Relaxed),"last_surface_packed":LAST_SURFACE.load(Ordering::Relaxed),"capture_note":"No fbdev fallback; capture fails if active DRM readback is unavailable","remote_input_scope":"controller queue; physical SDK button transport is a separate test","paint_count":PAINTS.load(Ordering::Relaxed),"button_callbacks":BUTTONS.load(Ordering::Relaxed),"remote_button_callbacks":REMOTE_BUTTONS.load(Ordering::Relaxed),"applied":APPLIED.load(Ordering::Relaxed),"page":PAGE.load(Ordering::Relaxed),"words":s.words.to_vec()})).unwrap_or_default();
        reply(&mut stream, "200 OK", "application/json", &body);
        return;
    }
    if first.starts_with("post /input ")
        && header.contains("content-type: application/json")
        && !header.contains("\r\norigin:")
    {
        let len = header
            .lines()
            .find_map(|s| {
                s.strip_prefix("content-length:")
                    .and_then(|s| s.trim().parse::<usize>().ok())
            })
            .unwrap_or(0);
        if len == 0 || len > 512 {
            return;
        }
        while raw.len() < header_end + len {
            let Ok(n) = stream.read(&mut b) else {
                return;
            };
            if n == 0 {
                return;
            }
            raw.extend_from_slice(&b[..n]);
            if raw.len() > 8192 {
                return;
            }
        }
        if let Ok(input) = serde_json::from_slice::<Input>(&raw[header_end..header_end + len])
            && input.valid()
        {
            if INPUT.get().is_some_and(|q| q.push(input)) {
                reply(&mut stream, "202 Accepted", "text/plain", b"queued");
            } else {
                reply(
                    &mut stream,
                    "503 Service Unavailable",
                    "text/plain",
                    b"queue full",
                );
            }
        } else {
            reply(
                &mut stream,
                "400 Bad Request",
                "text/plain",
                b"invalid input",
            );
        }
        return;
    }
    reply(&mut stream, "404 Not Found", "text/plain", b"USB QA only");
}
static DIAG_DIR: OnceLock<std::path::PathBuf> = OnceLock::new();
static SINCE: OnceLock<Instant> = OnceLock::new();
static CAPTURES: AtomicU64 = AtomicU64::new(0);
fn diagnostic_dir() -> Option<std::path::PathBuf> {
    if let Some(card) = std::env::var_os("LOOP_CLUB_CARD_DIR")
        .map(std::path::PathBuf::from)
        .filter(|p| p.is_absolute() && p.is_dir())
    {
        return Some(card.join("LoopClub/QA-qa5"));
    }
    let mounts = std::fs::read_to_string("/proc/mounts").ok()?;
    let mut found = Vec::new();
    for line in mounts.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        if parts.len() < 3 || !["exfat", "vfat", "fuseblk"].contains(&parts[2]) {
            continue;
        }
        let root = std::path::PathBuf::from(parts[1].replace("\\040", " "));
        if root.join("LoopClub/Expansions").is_dir() {
            found.push(root.join("LoopClub/QA-qa5"));
        }
    }
    if found.len() == 1 { found.pop() } else { None }
}
fn log(name: &str, value: serde_json::Value) {
    if let Some(dir) = DIAG_DIR.get()
        && let Ok(bytes) = serde_json::to_vec_pretty(&value)
    {
        let _ = std::fs::write(dir.join(name), bytes);
    }
}
/// File/project worker only. A bounded set of actual LCD captures survives
/// network failure. No filesystem work is performed by audio or paint here.
pub fn tick() {
    let Some(since) = SINCE.get() else {
        return;
    };
    let Some(dir) = DIAG_DIR.get() else {
        return;
    };
    let n = CAPTURES.load(Ordering::Relaxed);
    if n >= 20 || since.elapsed() < Duration::from_secs(5 + n * 3) {
        return;
    }
    CAPTURES.store(n + 1, Ordering::Relaxed);
    match crate::drm_capture::screen() {
        Ok(bytes) => {
            let path = dir.join(format!("lcd-{n:02}.bmp"));
            let saved = std::fs::write(&path, &bytes).is_ok();
            log(
                "capture-status.json",
                serde_json::json!({"source":"active DRM framebuffer","saved":saved,"frame":n,"page":PAGE.load(Ordering::Relaxed),"paints":PAINTS.load(Ordering::Relaxed)}),
            );
        }
        Err(error) => log(
            "capture-status.json",
            serde_json::json!({"source":"active DRM framebuffer","saved":false,"error":error,"frame":n}),
        ),
    }
}
pub fn start() {
    if !enabled() {
        return;
    }
    let _ = INPUT.set(Queue::default());
    let _ = STATE.set(crate::runtime::AtomicSnapshot::default());
    let _ = SINCE.set(Instant::now());
    if let Some(dir) = diagnostic_dir()
        && std::fs::create_dir_all(&dir).is_ok()
    {
        let _ = DIAG_DIR.set(dir);
    }
    log(
        "startup.json",
        serde_json::json!({"build":build_id(),"pid":std::process::id(),"voice_limit":crate::engine::VOICES,"sdk_patch_sha256":option_env!("LOOP_CLUB_SDK_PATCH"),"input_transport":"independent SDK callbacks + owning UI queue","physical_button_edges":PHYSICAL_COUNT.load(Ordering::Acquire).saturating_sub(REMOTE_CONTROL_EDGES.load(Ordering::Acquire)),"remote_transport_edges":REMOTE_CONTROL_EDGES.load(Ordering::Acquire),"remote_transport_consumed":REMOTE_CONTROL_FLOW.load(Ordering::Acquire)!=0,"physical_button_trace":PHYSICAL_BUTTONS.iter().map(|v|v.load(Ordering::Acquire)).collect::<Vec<_>>(),"project_storage":project_storage(),"project_io":{"sequence":IO_SEQUENCE.load(Ordering::Acquire),"result":IO_RESULT.load(Ordering::Acquire)},"output_streams":output_diagnostics(),"audio_timing":audio_timing(),"dsp_profile":dsp_profile(),"ui_position":UI_POSITION.load(Ordering::Relaxed),"qa_env":std::env::var("LOOP_CLUB_QA").ok(),"card_dir_present":std::env::var_os("LOOP_CLUB_CARD_DIR").is_some(),"state":"start requested"}),
    );
    let spawn=std::thread::Builder::new().name("loopclub-usb-qa".into()).spawn(||{
        for attempt in 0..120 {
            match TcpListener::bind((Ipv4Addr::new(192,168,155,1),8090)){
                Ok(listener)=>{log("network.json",serde_json::json!({"state":"listening","address":"192.168.155.1:8090","attempt":attempt}));for stream in listener.incoming().flatten(){serve(stream);}return;}
                Err(error)=>{if attempt==0||attempt==29||attempt==119 {log("network.json",serde_json::json!({"state":"bind failed","error":error.to_string(),"errno":error.raw_os_error(),"attempt":attempt}));}std::thread::sleep(Duration::from_secs(1));}
            }
        }
    });
    if let Err(error) = spawn {
        log(
            "network.json",
            serde_json::json!({"state":"thread failed","error":error.to_string()}),
        );
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn actual_framebuffer_rgb565_layout() {
        let mut raw = vec![0; 640 * 240];
        raw[..2].copy_from_slice(&0xf800u16.to_le_bytes());
        let b = bmp(&raw, 320, 240, 640, 16).unwrap();
        assert_eq!(&b[54..57], &[0, 0, 255]);
        assert_eq!(&b[..2], b"BM");
        assert!(bmp(&raw, 1280, 800, 640, 16).is_err());
    }
    #[test]
    fn input_limits_reject_invalid_values() {
        assert!(
            !Input::Knob {
                index: 16,
                value: f32::NAN,
                shift: false
            }
            .valid()
        );
        assert!(
            !Input::Button {
                code: 999,
                on: true,
                shift: false
            }
            .valid()
        );
        assert!(
            !Input::Pad {
                index: 16,
                on: true,
                velocity: 100,
                shift: false,
                erase: false
            }
            .valid()
        );
        assert!(
            Input::Button {
                code: 7,
                on: true,
                shift: false
            }
            .valid()
        );
    }
}
