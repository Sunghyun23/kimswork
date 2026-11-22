//! Background SD browser; UI only clones eight visible rows and never scans files.
use crate::{
    queue::Queue,
    runtime::Runtime,
    sample::{Browser, BrowserView},
    view::Painter,
};
use std::{
    path::PathBuf,
    sync::{Mutex, OnceLock},
};
#[derive(Clone, Copy)]
struct Request {
    drum: bool,
    target: Option<(usize, usize)>,
    up: bool,
    index: usize,
    revision: u64,
    track: usize,
    open: bool,
}
static REQUESTS: OnceLock<Queue<Request, 32>> = OnceLock::new();
static VIEW: OnceLock<Mutex<BrowserView>> = OnceLock::new();
pub fn initialize() {
    let _ = REQUESTS.set(Queue::default());
    let _ = VIEW.set(Mutex::new(BrowserView::default()));
}
fn publish(view: &BrowserView) {
    let next = view.clone();
    if let Some(m) = VIEW.get() {
        let old = {
            let mut lock = m.lock().unwrap_or_else(|e| e.into_inner());
            std::mem::replace(&mut *lock, next)
        };
        drop(old)
    }
}
pub struct FileWorker {
    kit_paths: Vec<PathBuf>,
    record_indices: Vec<usize>,
    kit_bank: Option<crate::bank::Bank>,
    kit_view: BrowserView,
    browser: Option<Browser>,
    root: Option<PathBuf>,
    rate: u32,
    capacity: usize,
    pending: bool,
}
impl FileWorker {
    pub fn new(rate: u32, capacity: usize) -> Self {
        Self::with_root(
            std::env::var_os("LOOP_CLUB_CARD_DIR")
                .map(PathBuf::from)
                .filter(|p| p.is_absolute()),
            rate,
            capacity,
        )
    }
    pub fn with_root(root: Option<PathBuf>, rate: u32, capacity: usize) -> Self {
        Self {
            kit_paths: Vec::new(),
            record_indices: Vec::new(),
            kit_bank: None,
            kit_view: BrowserView::default(),
            browser: None,
            root,
            rate,
            capacity,
            pending: false,
        }
    }
    pub fn tick(&mut self, r: &Runtime) -> Option<bool> {
        let mut loaded = None;
        if let Some(done) = r.collect_import() {
            let ok = done.error.is_none();
            loaded = Some(ok);
            self.pending = false;
            if let Some(b) = self.browser.as_mut() {
                b.view.pending = false;
                b.view.error = if ok {
                    String::new()
                } else {
                    "Track changed; sample not loaded".into()
                };
                publish(&b.view);
            }
            drop(done);
        }
        if let Some(done) = r.collect_kit() {
            let ok = done.error.is_none();
            loaded = Some(ok);
            self.pending = false;
            self.kit_view.pending = false;
            self.kit_view.error = if ok {
                String::new()
            } else {
                "Track changed or kit memory limit".into()
            };
            publish(&self.kit_view);
            drop(done);
        }
        let Some(queue) = REQUESTS.get() else {
            return loaded;
        };
        // SAFETY: only this dedicated file worker consumes the request queue.
        let Some(request) = (unsafe { queue.pop_shared() }) else {
            return loaded;
        };
        if self.pending {
            return loaded;
        }
        if request.drum {
            if let Err(error) = self.handle_kit(request, r) {
                self.kit_view.pending = false;
                self.kit_view.error = error.to_string();
            }
            publish(&self.kit_view);
            return loaded;
        }
        if request.open {
            let result = self
                .root
                .as_ref()
                .ok_or_else(|| std::io::Error::other("SD root unavailable"))
                .and_then(|root| Browser::open(root));
            match result {
                Ok(browser) => self.browser = Some(browser),
                Err(_) => {
                    publish(&BrowserView {
                        error: "SD ROOT UNAVAILABLE".into(),
                        revision: 1,
                        ..BrowserView::default()
                    });
                    return loaded;
                }
            }
        }
        let Some(browser) = self.browser.as_mut() else {
            return loaded;
        };
        let result = (|| -> std::io::Result<()> {
            if request.open {
                return Ok(());
            }
            if request.revision != browser.view.revision {
                return Err(std::io::Error::other("Folder changed; select again"));
            }
            if request.up {
                return browser.up();
            }
            let path = browser.resolve(request.index)?;
            if path.is_dir() {
                return browser.enter(request.index);
            }
            let bpm = r
                .snapshot
                .read()
                .map(|s| s.bpm())
                .ok_or_else(|| std::io::Error::other("Audio not ready"))?;
            browser.view.pending = true;
            publish(&browser.view);
            let job = crate::sample::prepare(&path, request.track, self.rate, bpm, self.capacity)?;
            if r.submit_import(job).is_err() {
                return Err(std::io::Error::other("Another sample load is pending"));
            }
            self.pending = true;
            Ok(())
        })();
        if let Err(error) = result {
            browser.view.pending = false;
            browser.view.error = error.to_string();
        }
        publish(&browser.view);
        loaded
    }
    fn show_packs(&mut self) -> std::io::Result<()> {
        let root = self
            .root
            .as_ref()
            .ok_or_else(|| std::io::Error::other("SD root unavailable"))?
            .canonicalize()?;
        let folder = root.join("LoopClub/Expansions");
        self.kit_paths.clear();
        self.kit_bank = None;
        if folder.exists() {
            let folder = folder.canonicalize()?;
            if !folder.starts_with(&root) {
                return Err(std::io::Error::other("Bank folder outside SD"));
            }
            for item in std::fs::read_dir(folder)? {
                let item = item?;
                if item.file_name().to_string_lossy().starts_with('.') {
                    continue;
                }
                let ty = item.file_type()?;
                let path = item.path();
                if ty.is_file()
                    && !ty.is_symlink()
                    && path
                        .extension()
                        .is_some_and(|v| v.eq_ignore_ascii_case("lcbank"))
                {
                    self.kit_paths.push(path);
                }
            }
        }
        if self.kit_paths.len() > 128 {
            return Err(std::io::Error::other("Maximum 128 banks"));
        }
        self.kit_paths.sort();
        self.kit_view = BrowserView {
            path: "DRUM BANKS".into(),
            revision: self.kit_view.revision + 1,
            entries: self
                .kit_paths
                .iter()
                .map(|p| crate::sample::Entry {
                    name: p
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .into_owned(),
                    path: p.clone(),
                    directory: true,
                })
                .collect(),
            ..BrowserView::default()
        };
        Ok(())
    }
    fn handle_kit(&mut self, request: Request, r: &Runtime) -> std::io::Result<()> {
        if request.open {
            return self.show_packs();
        }
        if request.revision != self.kit_view.revision {
            return Err(std::io::Error::other("List changed; select again"));
        }
        if request.up {
            return self.show_packs();
        }
        if let Some(bank) = &self.kit_bank {
            let job = if let Some((sound, layer)) = request.target {
                let index = *self
                    .record_indices
                    .get(request.index)
                    .ok_or_else(|| std::io::Error::other("Choose a sample"))?;
                bank.prepare_layer(index, request.track, sound, layer, self.rate)?
            } else {
                bank.prepare(request.index, request.track, self.rate)?
            };
            if r.submit_kit(job).is_err() {
                return Err(std::io::Error::other("Kit load busy"));
            }
            self.pending = true;
            self.kit_view.pending = true;
        } else {
            let path = self
                .kit_paths
                .get(request.index)
                .ok_or_else(|| std::io::Error::other("Choose a bank"))?;
            let root = self.root.as_ref().unwrap().canonicalize()?;
            if !path.canonicalize()?.starts_with(root) {
                return Err(std::io::Error::other("Bank outside SD"));
            }
            let bank = crate::bank::Bank::open(path)?;
            self.kit_view = BrowserView {
                path: bank.manifest.name.clone(),
                revision: self.kit_view.revision + 1,
                entries: if request.target.is_some() {
                    self.record_indices = bank
                        .manifest
                        .records
                        .iter()
                        .enumerate()
                        .filter(|(_, r)| r.kind == 0)
                        .map(|(i, _)| i)
                        .collect();
                    self.record_indices.sort_by(|a, b| {
                        bank.manifest.records[*a]
                            .name
                            .cmp(&bank.manifest.records[*b].name)
                    });
                    self.record_indices
                        .iter()
                        .map(|i| crate::sample::Entry {
                            name: crate::bank::sample_name(&bank.manifest.records[*i].name),
                            path: path.clone(),
                            directory: false,
                        })
                        .collect()
                } else {
                    bank.manifest
                        .kits
                        .iter()
                        .map(|k| crate::sample::Entry {
                            name: k.name.clone(),
                            path: path.clone(),
                            directory: false,
                        })
                        .collect()
                },
                ..BrowserView::default()
            };
            self.kit_bank = Some(bank);
        }
        Ok(())
    }
}
#[derive(Default)]
pub struct BrowserUi {
    drum: bool,
    pub target: Option<(usize, usize)>,
    pub open: bool,
    pub row: usize,
    track: usize,
    revision: u64,
    total: usize,
    view: BrowserView,
}
impl BrowserUi {
    pub fn start_layer(&mut self, track: usize, sound: usize, layer: usize) {
        self.target = Some((sound, layer));
        self.start_kind(track, true);
    }
    pub fn start(&mut self, track: usize) {
        self.start_kind(track, false);
    }
    pub fn start_kind(&mut self, track: usize, drum: bool) {
        self.drum = drum;
        self.open = true;
        self.row = 0;
        self.track = track;
        self.view = BrowserView::default();
        self.send(Request {
            drum,
            target: self.target,
            up: false,
            index: 0,
            revision: 0,
            track,
            open: true,
        });
    }
    fn send(&mut self, request: Request) {
        if !REQUESTS.get().is_some_and(|q| q.push(request)) {
            self.view.error = "BROWSER BUSY".into()
        }
    }
    pub fn refresh(&mut self) {
        if !self.open {
            return;
        }
        if let Some(m) = VIEW.get()
            && let Ok(view) = m.try_lock()
        {
            if view.revision != self.revision {
                self.row = 0;
                self.revision = view.revision;
            }
            self.total = view.entries.len();
            self.row = self.row.min(self.total.saturating_sub(1));
            let first = self.row / 6 * 6;
            let end = (first + 6).min(self.total);
            self.view = BrowserView {
                path: view.path.clone(),
                entries: view.entries[first..end].to_vec(),
                error: view.error.clone(),
                revision: view.revision,
                pending: view.pending,
            };
        }
    }
    pub fn encoder(&mut self, delta: i32) {
        if self.total > 0 {
            self.row = (self.row as i64 + i64::from(delta)).rem_euclid(self.total as i64) as usize;
        }
        self.refresh();
    }
    pub fn button(&mut self, code: u32) {
        match code {
            54 => self.encoder(6),
            55 => self.encoder(-6),
            14 => self.open = false,
            12 | 13 | 111 => self.send(Request {
                drum: self.drum,
                target: self.target,
                up: code == 12,
                index: self.row,
                revision: self.revision,
                track: self.track,
                open: false,
            }),
            _ => {}
        }
    }
    pub fn render(&self, p: &mut impl Painter) {
        let bg = 0xff142620;
        let ink = 0xffece4cf;
        let gold = 0xffd6b46d;
        let panel = 0xff20392e;
        p.rect(0, 0, 320, 240, bg);
        p.text(
            6,
            3,
            308,
            23,
            19.,
            ink,
            if self.drum {
                if self.target.is_some() {
                    "LAYER SAMPLE"
                } else {
                    "DRUM KITS"
                }
            } else {
                "SAMPLES"
            },
        );
        for (i, title) in ["UP", "OPEN / LOAD", "BACK"].iter().enumerate() {
            p.rect(5 + i as i32 * 105, 27, 100, 22, panel);
            p.text(9 + i as i32 * 105, 29, 94, 19, 16., gold, title);
        }
        p.text(
            6,
            51,
            308,
            18,
            14.,
            ink,
            &format!("SD / {}", self.view.path),
        );
        for (i, entry) in self.view.entries.iter().enumerate() {
            let y = 71 + i as i32 * 22;
            let selected = self.row % 6 == i;
            if selected {
                p.rect(5, y, 310, 21, panel)
            }
            p.text(
                8,
                y,
                304,
                21,
                17.,
                if selected { gold } else { ink },
                &format!(
                    "{} {}",
                    if self.drum {
                        if entry.directory {
                            "[BANK]"
                        } else if self.target.is_some() {
                            "WAV"
                        } else {
                            "KIT"
                        }
                    } else if entry.directory {
                        "[DIR]"
                    } else {
                        "WAV"
                    },
                    entry.name
                ),
            );
        }
        p.text(
            6,
            205,
            308,
            17,
            14.,
            ink,
            &format!(
                "DATA / +/- PAGE  {}/{}  T{:02}",
                if self.total == 0 { 0 } else { self.row + 1 },
                self.total,
                self.track + 1
            ),
        );
        p.text(
            6,
            223,
            308,
            17,
            14.,
            gold,
            if self.view.pending {
                if self.drum {
                    "LOADING SELECTED KIT..."
                } else {
                    "DECODING WAV..."
                }
            } else if !self.view.error.is_empty() {
                &self.view.error
            } else {
                "ORIGINAL FILE IS PRESERVED"
            },
        );
    }
}
