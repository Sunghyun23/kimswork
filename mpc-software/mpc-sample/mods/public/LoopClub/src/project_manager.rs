//! Named project storage and modal UI. Filesystem work runs only on the project worker.
use crate::{engine::Engine, runtime::Runtime, ui::Snapshot, view::Painter};
use std::{
    fs, io,
    path::{Path, PathBuf},
    sync::{Mutex, OnceLock},
};

const LIMIT: usize = 256;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operation {
    Scan,
    New,
    Open(usize),
    Save,
    SaveCopy,
}
#[derive(Clone, Copy, Debug)]
pub struct Request {
    pub operation: Operation,
    pub save_first: bool,
}
#[derive(Clone, Default)]
pub struct View {
    pub current: String,
    pub names: Vec<String>,
    pub busy: bool,
    pub message: &'static str,
    pub revision: u64,
    pub replaced: u64,
}
#[derive(Default)]
struct Shared {
    view: View,
    request: Option<Request>,
}
static SHARED: OnceLock<Mutex<Shared>> = OnceLock::new();
pub fn initialize() {
    let _ = SHARED.set(Mutex::default());
}
fn publish(view: &View) {
    if let Some(m) = SHARED.get() {
        m.lock().unwrap_or_else(|e| e.into_inner()).view = view.clone();
    }
}
fn request(r: Request) -> bool {
    let Some(m) = SHARED.get() else { return false };
    let Ok(mut s) = m.try_lock() else {
        return false;
    };
    if s.view.busy || s.request.is_some() {
        return false;
    }
    s.view.busy = true;
    s.view.message = "WORKING...";
    s.request = Some(r);
    true
}
fn invalid() -> io::Error {
    io::Error::other("Invalid project name")
}
fn valid(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 48
        && name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
}
fn named_path(root: &Path, name: &str) -> io::Result<PathBuf> {
    if !valid(name) {
        return Err(invalid());
    }
    Ok(if name == "Session" {
        root.join("Session.lcproj")
    } else {
        root.join("Projects").join(format!("{name}.lcproj"))
    })
}
fn plain_file(path: &Path) -> bool {
    fs::symlink_metadata(path).is_ok_and(|m| m.file_type().is_file())
}
/// The old Session remains the startup fallback. A sidecar only selects a successfully saved/opened project.
pub fn startup_path(root: &Path) -> PathBuf {
    fs::read_to_string(root.join("active-project.txt"))
        .ok()
        .and_then(|s| named_path(root, s.trim()).ok())
        .filter(|p| plain_file(p))
        .unwrap_or_else(|| root.join("Session.lcproj"))
}
fn remember(root: &Path, name: &str) -> io::Result<()> {
    if !valid(name) {
        return Err(invalid());
    }
    let temp = root.join("active-project.txt.new");
    fs::write(&temp, name)?;
    fs::File::open(&temp)?.sync_all()?;
    fs::rename(temp, root.join("active-project.txt"))?;
    fs::File::open(root)?.sync_all()
}
fn save_with_backup(engine: &Engine, path: &Path, exists: bool) -> io::Result<()> {
    if !engine.is_idle() {
        return Err(io::Error::new(io::ErrorKind::WouldBlock, "Stop first"));
    }
    if path.exists() {
        if !exists || !plain_file(path) {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "Project exists",
            ));
        }
        let backup = path.with_extension("previous.lcproj");
        let temp = path.with_extension("previous.lcproj.new");
        fs::copy(path, &temp)?;
        fs::File::open(&temp)?.sync_all()?;
        fs::rename(temp, backup)?;
    }
    crate::project::save(engine, path)
}
pub struct Store {
    root: PathBuf,
    current: String,
    exists: bool,
    pub view: View,
}
impl Store {
    pub fn new(root: PathBuf) -> io::Result<Self> {
        fs::create_dir_all(root.join("Projects"))?;
        // Do not follow a replacement Projects directory outside the project root.
        if fs::symlink_metadata(root.join("Projects"))?
            .file_type()
            .is_symlink()
        {
            return Err(invalid());
        }
        let path = startup_path(&root);
        let current = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Session")
            .to_owned();
        let exists = plain_file(&path);
        let mut s = Self {
            root,
            current,
            exists,
            view: View::default(),
        };
        s.scan()?;
        Ok(s)
    }
    fn scan(&mut self) -> io::Result<()> {
        let mut names = Vec::new();
        if plain_file(&self.root.join("Session.lcproj")) {
            names.push("Session".to_owned());
        }
        for entry in fs::read_dir(self.root.join("Projects"))? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("lcproj")
                || !entry.file_type()?.is_file()
            {
                continue;
            }
            let Some(name) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            if valid(name) && name != "Session" {
                if names.len() == LIMIT {
                    return Err(io::Error::other("Project list full"));
                }
                names.push(name.to_owned());
            }
        }
        names.sort();
        names.dedup();
        self.view.names = names;
        self.view.current = self.current.clone();
        Ok(())
    }
    fn unused(&self) -> io::Result<String> {
        for n in 1..=9999 {
            let name = format!("Project-{n:03}");
            if name != self.current && !named_path(&self.root, &name)?.exists() {
                return Ok(name);
            }
        }
        Err(io::Error::other("No free project names"))
    }
    fn save(&mut self, engine: &Engine) -> io::Result<()> {
        save_with_backup(engine, &named_path(&self.root, &self.current)?, self.exists)?;
        self.exists = true;
        remember(&self.root, &self.current)
    }
    pub fn apply(
        &mut self,
        r: Request,
        engine: &mut Engine,
        rate: u32,
        capacity: usize,
    ) -> io::Result<()> {
        if r.operation == Operation::Scan {
            return self.scan();
        }
        if !engine.is_idle() {
            return Err(io::Error::new(
                io::ErrorKind::WouldBlock,
                "Stop and release notes",
            ));
        }
        if r.save_first {
            self.save(engine)?;
        }
        match r.operation {
            Operation::New => {
                let name = self.unused()?;
                let mut fresh = Engine::new_project(rate, capacity);
                fresh.hardware_volume = engine.hardware_volume;
                fresh.hardware_gain = engine.hardware_gain;
                *engine = fresh;
                self.current = name;
                self.exists = false;
                self.view.replaced += 1;
            }
            Operation::Open(i) => {
                let name = self.view.names.get(i).ok_or_else(invalid)?.clone();
                let path = named_path(&self.root, &name)?;
                if !plain_file(&path) {
                    return Err(invalid());
                }
                let mut loaded =
                    crate::project::load_for_replacement(&path, rate, capacity, engine)?;
                loaded.selected = 0;
                // Update startup choice before replacing live data; failure leaves the live engine intact.
                remember(&self.root, &name)?;
                *engine = loaded;
                self.current = name;
                self.exists = true;
                self.view.replaced += 1;
            }
            Operation::Save => self.save(engine)?,
            Operation::SaveCopy => {
                let name = self.unused()?;
                save_with_backup(engine, &named_path(&self.root, &name)?, false)?;
                remember(&self.root, &name)?;
                self.current = name;
                self.exists = true;
            }
            Operation::Scan => {}
        }
        self.scan()
    }
}
pub struct Worker {
    store: Option<Store>,
    pending: Option<Request>,
    view: View,
}
impl Worker {
    pub fn new(root: Option<PathBuf>) -> Self {
        let store = root.and_then(|p| Store::new(p).ok());
        let view = store
            .as_ref()
            .map(|s| s.view.clone())
            .unwrap_or_else(|| View {
                message: "PROJECT STORAGE UNAVAILABLE",
                ..View::default()
            });
        publish(&view);
        Self {
            store,
            pending: None,
            view,
        }
    }
    pub fn tick(&mut self, runtime: &Runtime, rate: u32, capacity: usize) {
        if self.pending.is_none()
            && let Some(m) = SHARED.get()
        {
            self.pending = m.lock().unwrap_or_else(|e| e.into_inner()).request.take();
        }
        let Some(req) = self.pending else { return };
        let done = if let Some(store) = &mut self.store {
            runtime.with_engine(|slot| {
                let before = store.view.replaced;
                let result = slot
                    .as_mut()
                    .ok_or_else(invalid)
                    .and_then(|e| store.apply(req, e, rate, capacity));
                if store.view.replaced != before {
                    // Synchronize external MIDI's immediate routing hint as well as Engine selection.
                    runtime.enqueue(crate::engine::Command::Select(0));
                }
                if let Some(e) = slot {
                    runtime.snapshot.publish(&Snapshot::capture(e));
                }
                result
            })
        } else {
            Some(Err(io::Error::other("Storage unavailable")))
        };
        let Some(done) = done else { return };
        if let Some(store) = &self.store {
            self.view = store.view.clone();
        }
        self.view.busy = false;
        self.view.revision += 1;
        self.view.message = match done {
            Ok(()) => match req.operation {
                Operation::New => "NEW PROJECT / NOT SAVED",
                Operation::Open(_) => "PROJECT OPENED",
                Operation::Save | Operation::SaveCopy => "PROJECT SAVED",
                _ => "",
            },
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => "STOP / RELEASE NOTES FIRST",
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => "NAME EXISTS / USE SAVE COPY",
            Err(_) => "PROJECT IO ERROR / CURRENT KEPT",
        };
        if let Some(store) = &mut self.store {
            store.view.revision = self.view.revision;
        }
        self.pending = None;
        publish(&self.view);
    }
}
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub enum Screen {
    #[default]
    Menu,
    Open,
    Confirm(Operation),
}
#[derive(Default)]
pub struct Manager {
    pub open: bool,
    pub screen: Screen,
    pub row: usize,
    pub view: View,
    pub replaced: u64,
}
impl Manager {
    pub fn refresh(&mut self) -> bool {
        if let Some(m) = SHARED.get()
            && let Ok(s) = m.try_lock()
        {
            self.view = s.view.clone();
        }
        if self.view.replaced != self.replaced {
            self.replaced = self.view.replaced;
            self.screen = Screen::Menu;
            self.row = 0;
            return true;
        }
        false
    }
    pub fn start(&mut self, operation: Operation) {
        self.open = true;
        self.screen = if operation == Operation::Open(0) {
            Screen::Open
        } else {
            Screen::Menu
        };
        self.row = 0;
        match operation {
            Operation::Save => {
                self.row = 2;
                self.submit(Operation::Scan, false);
            }
            Operation::New => {
                self.screen = Screen::Confirm(Operation::New);
                self.row = 2;
            }
            _ => self.submit(Operation::Scan, false),
        }
    }
    fn submit(&mut self, operation: Operation, save_first: bool) {
        if request(Request {
            operation,
            save_first,
        }) {
            self.view.busy = true;
            self.view.message = "WORKING...";
        } else {
            self.view.message = "PROJECT BUSY / TRY AGAIN";
        }
    }
    pub fn encoder(&mut self, delta: i32) {
        if self.view.busy {
            return;
        }
        let count = match self.screen {
            Screen::Menu => 4,
            Screen::Open => self.view.names.len(),
            Screen::Confirm(_) => 3,
        };
        self.row =
            (self.row as i64 + i64::from(delta)).clamp(0, count.saturating_sub(1) as i64) as usize;
    }
    fn back(&mut self) {
        match self.screen {
            Screen::Menu => self.open = false,
            Screen::Open => {
                self.screen = Screen::Menu;
                self.row = 1;
            }
            Screen::Confirm(op) => {
                self.screen = if matches!(op, Operation::Open(_)) {
                    Screen::Open
                } else {
                    Screen::Menu
                };
                self.row = if let Operation::Open(i) = op { i } else { 0 };
            }
        }
    }
    pub fn button(&mut self, code: u8, shift: bool) {
        if self.view.busy {
            return;
        }
        match code {
            6 | 74 | 14 => {
                self.open = false;
                return;
            }
            12 => {
                self.back();
                return;
            }
            54 | 55 => {
                let step = if shift && self.screen == Screen::Open {
                    6
                } else {
                    1
                };
                self.encoder(if code == 54 { step } else { -step });
                return;
            }
            13 | 111 => {}
            _ => return,
        }
        match self.screen {
            Screen::Menu => match self.row {
                0 => {
                    self.screen = Screen::Confirm(Operation::New);
                    self.row = 2;
                }
                1 => {
                    self.screen = Screen::Open;
                    self.row = 0;
                    self.submit(Operation::Scan, false);
                }
                2 => self.submit(Operation::Save, false),
                _ => self.submit(Operation::SaveCopy, false),
            },
            Screen::Open if !self.view.names.is_empty() => {
                self.screen = Screen::Confirm(Operation::Open(self.row));
                self.row = 2;
            }
            Screen::Confirm(op) => match self.row {
                0 => self.submit(op, true),
                1 => self.submit(op, false),
                _ => self.back(),
            },
            _ => {}
        }
    }
    pub fn render(&self, p: &mut impl Painter) {
        const BG: u32 = 0xff142620;
        const PANEL: u32 = 0xff20392e;
        const GOLD: u32 = 0xffd6b46d;
        const INK: u32 = 0xffece4cf;
        p.rect(0, 0, 320, 240, BG);
        p.text(8, 4, 304, 23, 21., INK, "PROJECT");
        let tabs = ["BACK", "SELECT", "LOOP"];
        for (i, text) in tabs.iter().enumerate() {
            let x = 5 + i as i32 * 105;
            p.rect(x, 30, 100, 29, PANEL);
            p.text(x + 2, 34, 96, 22, 16., GOLD, text);
        }
        match self.screen {
            Screen::Menu => {
                p.text(
                    9,
                    64,
                    302,
                    20,
                    16.,
                    INK,
                    &format!("CURRENT: {}", self.view.current),
                );
                for (i, name) in ["NEW PROJECT", "OPEN PROJECT", "SAVE", "SAVE A COPY"]
                    .iter()
                    .enumerate()
                {
                    let y = 91 + i as i32 * 27;
                    let selected = self.row == i;
                    p.rect(5, y, 310, 25, if selected { GOLD } else { PANEL });
                    p.text(
                        11,
                        y + 1,
                        298,
                        24,
                        20.,
                        if selected { BG } else { INK },
                        name,
                    );
                }
                p.text(8, 199, 304, 16, 14., INK, "DATA: MOVE / PRESS: SELECT");
            }
            Screen::Open => {
                let count = self.view.names.len();
                if count == 0 {
                    p.text(10, 90, 300, 26, 19., INK, "NO SAVED PROJECTS");
                }
                let first = self.row / 6 * 6;
                for (i, name) in self.view.names.iter().enumerate().skip(first).take(6) {
                    let y = 66 + (i - first) as i32 * 21;
                    let selected = i == self.row;
                    p.rect(5, y, 310, 20, if selected { GOLD } else { PANEL });
                    p.text(9, y, 302, 20, 17., if selected { BG } else { INK }, name);
                }
                p.text(
                    8,
                    195,
                    304,
                    19,
                    15.,
                    INK,
                    &format!(
                        "DATA: SELECT    {} / {}",
                        if count == 0 { 0 } else { self.row + 1 },
                        count
                    ),
                );
            }
            Screen::Confirm(op) => {
                let name = match op {
                    Operation::New => "NEW EMPTY PROJECT",
                    Operation::Open(i) => self
                        .view
                        .names
                        .get(i)
                        .map(String::as_str)
                        .unwrap_or("PROJECT"),
                    _ => "PROJECT",
                };
                p.text(9, 66, 302, 23, 18., GOLD, name);
                p.text(
                    9,
                    91,
                    302,
                    18,
                    15.,
                    INK,
                    "Save current work before leaving?",
                );
                for (i, name) in ["SAVE AND CONTINUE", "DISCARD CHANGES", "CANCEL"]
                    .iter()
                    .enumerate()
                {
                    let y = 116 + i as i32 * 27;
                    let selected = self.row == i;
                    p.rect(5, y, 310, 25, if selected { GOLD } else { PANEL });
                    p.text(
                        11,
                        y + 1,
                        298,
                        24,
                        18.,
                        if selected { BG } else { INK },
                        name,
                    );
                }
                p.text(8, 199, 304, 16, 14., INK, "DATA: MOVE / PRESS: SELECT");
            }
        }
        p.text(
            6,
            216,
            308,
            20,
            15.,
            GOLD,
            if self.view.busy {
                "WORKING... PLEASE WAIT"
            } else if self.view.message.is_empty() {
                "SAMPLE: BACK TO LOOP"
            } else {
                self.view.message
            },
        );
    }
}

static POSITION: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub fn observe(manager: &Manager) {
    use std::sync::atomic::Ordering;
    let screen = match manager.screen {
        Screen::Menu => 0,
        Screen::Open => 1,
        Screen::Confirm(_) => 2,
    };
    POSITION.store(
        u64::from(manager.open) | (screen << 1) | ((manager.row as u64) << 8),
        Ordering::Relaxed,
    );
}
pub fn diagnostics() -> serde_json::Value {
    let pos = POSITION.load(std::sync::atomic::Ordering::Relaxed);
    let view = SHARED
        .get()
        .and_then(|m| m.try_lock().ok().map(|s| s.view.clone()))
        .unwrap_or_default();
    serde_json::json!({"open":pos&1!=0,"screen":(pos>>1)&3,"row":pos>>8,"current":view.current,"names":view.names,"busy":view.busy,"message":view.message,"revision":view.revision,"replaced":view.replaced})
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{engine::Command, model::Kind};
    struct Sandbox(PathBuf);
    impl Sandbox {
        fn new() -> Self {
            let p = std::env::temp_dir().join(format!(
                "loop-club-projects-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
            fs::create_dir_all(&p).unwrap();
            Self(p)
        }
    }
    impl Drop for Sandbox {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    fn apply(s: &mut Store, e: &mut Engine, op: Operation, save_first: bool) -> io::Result<()> {
        s.apply(
            Request {
                operation: op,
                save_first,
            },
            e,
            8000,
            128,
        )
    }
    fn engine() -> Engine {
        Engine::new_project(8000, 128)
    }
    #[test]
    fn new_save_open_and_copy_preserve_old_session_and_own_sample_data() {
        let dir = Sandbox::new();
        let mut e = engine();
        e.tracks[0].name = "Owner".into();
        e.tracks[0].audio_frames = 1;
        e.tracks[0].length_ticks = crate::model::PPQ * 4;
        e.tracks[0].audio[0].l = 0.375;
        crate::project::save(&e, &dir.0.join("Session.lcproj")).unwrap();
        let original = fs::read(dir.0.join("Session.lcproj")).unwrap();
        let mut s = Store::new(dir.0.clone()).unwrap();
        e.hardware_volume = 0.37;
        e.hardware_gain = 0.32;
        apply(&mut s, &mut e, Operation::New, false).unwrap();
        assert_eq!(s.current, "Project-001");
        assert!(!s.exists);
        assert_eq!(e.clock.bpm, 100.);
        assert_eq!(e.tracks[0].audio_frames, 0);
        assert_eq!(e.hardware_volume, 0.37);
        assert_eq!(e.hardware_gain, 0.32);
        assert_eq!(fs::read(dir.0.join("Session.lcproj")).unwrap(), original);
        assert_eq!(startup_path(&dir.0), dir.0.join("Session.lcproj"));
        e.tracks[0].name = "New song".into();
        apply(&mut s, &mut e, Operation::Save, false).unwrap();
        assert_eq!(
            startup_path(&dir.0),
            dir.0.join("Projects/Project-001.lcproj")
        );
        let saved = fs::read(startup_path(&dir.0)).unwrap();
        apply(&mut s, &mut e, Operation::SaveCopy, false).unwrap();
        assert_eq!(s.current, "Project-002");
        assert_eq!(
            fs::read(dir.0.join("Projects/Project-001.lcproj")).unwrap(),
            saved
        );
        let i = s.view.names.iter().position(|n| n == "Session").unwrap();
        apply(&mut s, &mut e, Operation::Open(i), false).unwrap();
        assert_eq!(e.tracks[0].name, "Owner");
        assert_eq!(e.tracks[0].audio[0].l, 0.375);
        assert!(!e.running);
        assert_eq!(e.hardware_volume, 0.37);
        assert_eq!(startup_path(&dir.0), dir.0.join("Session.lcproj"));
    }
    #[test]
    fn overwrite_keeps_previous_and_save_before_new_is_transactional() {
        let dir = Sandbox::new();
        let mut e = engine();
        let mut s = Store::new(dir.0.clone()).unwrap();
        apply(&mut s, &mut e, Operation::Save, false).unwrap();
        let original = fs::read(dir.0.join("Session.lcproj")).unwrap();
        e.tracks[0].name = "Second".into();
        apply(&mut s, &mut e, Operation::New, true).unwrap();
        assert_eq!(
            fs::read(dir.0.join("Session.previous.lcproj")).unwrap(),
            original
        );
        assert_eq!(
            crate::project::load(&dir.0.join("Session.lcproj"), 8000, 128)
                .unwrap()
                .tracks[0]
                .name,
            "Second"
        );
        // A colliding/symlink destination must stop SAVE+NEW before replacing the live engine.
        let name = s.current.clone();
        fs::write(named_path(&dir.0, &name).unwrap(), b"another process").unwrap();
        e.tracks[0].name = "Keep live".into();
        assert!(apply(&mut s, &mut e, Operation::New, true).is_err());
        assert_eq!(s.current, name);
        assert_eq!(e.tracks[0].name, "Keep live");
    }
    #[test]
    fn corrupt_open_busy_and_invalid_indices_leave_live_project_and_pointer_unchanged() {
        let dir = Sandbox::new();
        let mut e = engine();
        let mut s = Store::new(dir.0.clone()).unwrap();
        e.tracks[0].name = "Keep me".into();
        apply(&mut s, &mut e, Operation::Save, false).unwrap();
        fs::write(dir.0.join("Projects/Broken.lcproj"), b"bad").unwrap();
        apply(&mut s, &mut e, Operation::Scan, false).unwrap();
        let i = s.view.names.iter().position(|n| n == "Broken").unwrap();
        assert!(apply(&mut s, &mut e, Operation::Open(i), false).is_err());
        assert!(apply(&mut s, &mut e, Operation::Open(999), false).is_err());
        assert_eq!(e.tracks[0].name, "Keep me");
        assert_eq!(s.current, "Session");
        assert_eq!(startup_path(&dir.0), dir.0.join("Session.lcproj"));
        e.running = true;
        for op in [
            Operation::New,
            Operation::Open(i),
            Operation::Save,
            Operation::SaveCopy,
        ] {
            assert_eq!(
                apply(&mut s, &mut e, op, false).unwrap_err().kind(),
                io::ErrorKind::WouldBlock
            );
        }
        assert_eq!(s.current, "Session");
        e.running = false;
        e.command(Command::SetKind(0, Kind::Synth)).unwrap();
        e.command(Command::Note {
            track: 0,
            note: 60,
            velocity: 100,
            on: true,
            source: 1,
        })
        .unwrap();
        assert_eq!(
            apply(&mut s, &mut e, Operation::New, false)
                .unwrap_err()
                .kind(),
            io::ErrorKind::WouldBlock
        );
    }
    #[test]
    fn replacement_resets_immediate_external_midi_target_from_previous_selected_track() {
        let dir = Sandbox::new();
        let runtime = Runtime::default();
        assert!(runtime.initialize(Box::new(engine())).is_ok());
        runtime
            .active
            .store(true, std::sync::atomic::Ordering::Release);
        runtime.enqueue(Command::Select(7));
        let mut audio = [0; 128];
        runtime.output(&mut audio, 2, 8000);
        let mut worker = Worker {
            store: Some(Store::new(dir.0.clone()).unwrap()),
            pending: Some(Request {
                operation: Operation::New,
                save_first: false,
            }),
            view: View::default(),
        };
        worker.tick(&runtime, 8000, 128);
        runtime
            .with_engine(|slot| {
                slot.as_mut()
                    .unwrap()
                    .command(Command::SetKind(0, Kind::Synth))
                    .unwrap();
            })
            .unwrap();
        runtime.external_note(55, 60, 100, true);
        runtime.output(&mut audio, 2, 8000);
        runtime
            .with_engine(|slot| {
                let e = slot.as_ref().unwrap();
                assert_eq!(e.selected, 0);
                assert_eq!(e.active_voices(), 1);
            })
            .unwrap();
    }
    #[test]
    fn untrusted_pointer_backup_names_and_symlinks_are_never_opened() {
        let dir = Sandbox::new();
        let mut s = Store::new(dir.0.clone()).unwrap();
        for name in [
            "../escape",
            "/tmp/escape",
            "Session.previous",
            "",
            "has space",
        ] {
            assert!(named_path(&dir.0, name).is_err());
        }
        fs::write(dir.0.join("active-project.txt"), "../escape").unwrap();
        assert_eq!(startup_path(&dir.0), dir.0.join("Session.lcproj"));
        fs::write(dir.0.join("Projects/Project-001.previous.lcproj"), b"old").unwrap();
        #[cfg(unix)]
        std::os::unix::fs::symlink("/tmp", dir.0.join("Projects/Escape.lcproj")).unwrap();
        s.scan().unwrap();
        assert!(s.view.names.is_empty());
    }
    #[test]
    fn confirmation_cancel_and_list_navigation_never_issue_background_play_or_edits() {
        let mut m = Manager {
            open: true,
            view: View {
                names: (0..15).map(|i| format!("Song-{i:02}")).collect(),
                ..View::default()
            },
            ..Manager::default()
        };
        m.button(111, false);
        assert_eq!(m.screen, Screen::Confirm(Operation::New));
        assert_eq!(m.row, 2);
        m.button(111, false);
        assert_eq!(m.screen, Screen::Menu);
        m.encoder(1);
        assert_eq!(m.row, 1);
        m.screen = Screen::Open;
        m.row = 0;
        m.encoder(8);
        assert_eq!(m.row, 8);
        m.button(111, false);
        assert_eq!(m.screen, Screen::Confirm(Operation::Open(8)));
        assert_eq!(m.row, 2);
        m.button(111, false);
        assert_eq!(m.screen, Screen::Open);
        assert_eq!(m.row, 8);
        m.encoder(i32::MAX);
        assert_eq!(m.row, 14);
        m.encoder(i32::MIN);
        assert_eq!(m.row, 0);
        m.view.busy = true;
        m.button(6, false);
        assert!(m.open);
        m.view.busy = false;
        m.button(6, false);
        assert!(!m.open);
    }
}
