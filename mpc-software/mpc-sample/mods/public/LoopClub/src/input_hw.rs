//! Bounded, read-only ALSA mixer discovery on the management worker, never audio.
#[cfg(target_os = "linux")]
pub fn inspect() -> serde_json::Value {
    use std::ffi::{CStr, CString, c_char, c_int, c_uint, c_void};
    unsafe extern "C" {
        fn dlopen(name: *const c_char, flags: c_int) -> *mut c_void;
        fn dlsym(handle: *mut c_void, name: *const c_char) -> *mut c_void;
        fn dlclose(handle: *mut c_void) -> c_int;
    }
    // SAFETY: standard libasound API; pointers remain owned by this worker and are freed before dlclose.
    unsafe {
        let lib = dlopen(c"libasound.so.2".as_ptr(), 1);
        if lib.is_null() {
            return serde_json::json!({"error":"libasound unavailable"});
        }
        macro_rules! sym {($name:literal,$t:ty)=>{{ let ptr=dlsym(lib,concat!($name,"\0").as_ptr().cast());if ptr.is_null(){dlclose(lib);return serde_json::json!({"error":concat!("Missing ",$name)});}std::mem::transmute::<*mut c_void,$t>(ptr) }};}
        type Ptr = *mut c_void;
        let open = sym!(
            "snd_ctl_open",
            unsafe extern "C" fn(*mut Ptr, *const c_char, c_int) -> c_int
        );
        let close = sym!("snd_ctl_close", unsafe extern "C" fn(Ptr) -> c_int);
        let list_alloc = sym!(
            "snd_ctl_elem_list_malloc",
            unsafe extern "C" fn(*mut Ptr) -> c_int
        );
        let list_free = sym!("snd_ctl_elem_list_free", unsafe extern "C" fn(Ptr));
        let list = sym!("snd_ctl_elem_list", unsafe extern "C" fn(Ptr, Ptr) -> c_int);
        let count = sym!(
            "snd_ctl_elem_list_get_count",
            unsafe extern "C" fn(Ptr) -> c_uint
        );
        let space = sym!(
            "snd_ctl_elem_list_alloc_space",
            unsafe extern "C" fn(Ptr, c_uint) -> c_int
        );
        let name = sym!(
            "snd_ctl_elem_list_get_name",
            unsafe extern "C" fn(Ptr, c_uint) -> *const c_char
        );
        let numid = sym!(
            "snd_ctl_elem_list_get_numid",
            unsafe extern "C" fn(Ptr, c_uint) -> c_uint
        );
        let info_alloc = sym!(
            "snd_ctl_elem_info_malloc",
            unsafe extern "C" fn(*mut Ptr) -> c_int
        );
        let info_free = sym!("snd_ctl_elem_info_free", unsafe extern "C" fn(Ptr));
        let set_id = sym!(
            "snd_ctl_elem_info_set_numid",
            unsafe extern "C" fn(Ptr, c_uint)
        );
        let info = sym!("snd_ctl_elem_info", unsafe extern "C" fn(Ptr, Ptr) -> c_int);
        let typ = sym!(
            "snd_ctl_elem_info_get_type",
            unsafe extern "C" fn(Ptr) -> c_int
        );
        let items = sym!(
            "snd_ctl_elem_info_get_items",
            unsafe extern "C" fn(Ptr) -> c_uint
        );
        let set_item = sym!(
            "snd_ctl_elem_info_set_item",
            unsafe extern "C" fn(Ptr, c_uint)
        );
        let item_name = sym!(
            "snd_ctl_elem_info_get_item_name",
            unsafe extern "C" fn(Ptr) -> *const c_char
        );
        let mut cards = Vec::new();
        for card in 0..3 {
            let device = CString::new(format!("hw:{card}")).unwrap();
            let mut ctl = std::ptr::null_mut();
            if open(&mut ctl, device.as_ptr(), 1) < 0 {
                continue;
            }
            let mut entries = std::ptr::null_mut();
            if list_alloc(&mut entries) < 0 {
                close(ctl);
                continue;
            }
            let mut controls = Vec::new();
            if list(ctl, entries) >= 0
                && space(entries, count(entries).min(256)) >= 0
                && list(ctl, entries) >= 0
            {
                for i in 0..count(entries).min(256) {
                    let ptr = name(entries, i);
                    if ptr.is_null() {
                        continue;
                    }
                    let label = CStr::from_ptr(ptr).to_string_lossy().into_owned();
                    let id = numid(entries, i);
                    let mut detail = std::ptr::null_mut();
                    if info_alloc(&mut detail) < 0 {
                        continue;
                    }
                    set_id(detail, id);
                    let mut choices = Vec::new();
                    let mut kind = 0;
                    if info(ctl, detail) >= 0 {
                        kind = typ(detail);
                        if kind == 3 {
                            for item in 0..items(detail).min(64) {
                                set_item(detail, item);
                                if info(ctl, detail) >= 0 {
                                    let n = item_name(detail);
                                    if !n.is_null() {
                                        choices
                                            .push(CStr::from_ptr(n).to_string_lossy().into_owned());
                                    }
                                }
                            }
                        }
                    }
                    controls.push(
                        serde_json::json!({"numid":id,"name":label,"kind":kind,"choices":choices}),
                    );
                    info_free(detail);
                }
            }
            list_free(entries);
            close(ctl);
            cards.push(serde_json::json!({"card":card,"controls":controls}));
        }
        dlclose(lib);
        serde_json::json!({"scope":"read-only ALSA discovery; no source changes","cards":cards})
    }
}
#[cfg(not(target_os = "linux"))]
pub fn inspect() -> serde_json::Value {
    serde_json::json!({"error":"Device only"})
}

use std::sync::atomic::{AtomicU8, Ordering};
static SELECTED: AtomicU8 = AtomicU8::new(0);
static PENDING: AtomicU8 = AtomicU8::new(0);
static RESULT: AtomicU8 = AtomicU8::new(0);
pub const LABELS: [&str; 3] = ["LINE", "MIC", "USB"];
pub fn selected() -> u8 {
    SELECTED.load(Ordering::Acquire)
}
pub fn pending() -> bool {
    PENDING.load(Ordering::Acquire) != 0
}
pub fn result() -> u8 {
    RESULT.load(Ordering::Acquire)
}
pub fn request(source: u8) -> bool {
    source < 3
        && PENDING
            .compare_exchange(0, source + 1, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
}
pub fn accepts_period(source: u8, period: u32) -> bool {
    if source == 2 {
        period == 24
    } else {
        period != 24
    }
}

#[cfg(target_os = "linux")]
fn switch(label: &str, write: Option<bool>) -> Result<bool, String> {
    use std::ffi::{CStr, c_char, c_int, c_long, c_uint, c_void};
    unsafe extern "C" {
        fn dlopen(name: *const c_char, flags: c_int) -> *mut c_void;
        fn dlsym(handle: *mut c_void, name: *const c_char) -> *mut c_void;
        fn dlclose(handle: *mut c_void) -> c_int;
    }
    // SAFETY: bounded standard ALSA calls on the file worker; names/types validated before writes.
    unsafe {
        let lib = dlopen(c"libasound.so.2".as_ptr(), 1);
        if lib.is_null() {
            return Err("No ALSA".into());
        }
        macro_rules! sym {
            ($name:literal,$t:ty) => {{
                let ptr = dlsym(lib, concat!($name, "\0").as_ptr().cast());
                if ptr.is_null() {
                    dlclose(lib);
                    return Err(concat!("Missing ", $name).into());
                }
                std::mem::transmute::<*mut c_void, $t>(ptr)
            }};
        }
        type Ptr = *mut c_void;
        let open = sym!(
            "snd_ctl_open",
            unsafe extern "C" fn(*mut Ptr, *const c_char, c_int) -> c_int
        );
        let close = sym!("snd_ctl_close", unsafe extern "C" fn(Ptr) -> c_int);
        let ia = sym!(
            "snd_ctl_elem_info_malloc",
            unsafe extern "C" fn(*mut Ptr) -> c_int
        );
        let free = sym!("snd_ctl_elem_info_free", unsafe extern "C" fn(Ptr));
        let sid = sym!(
            "snd_ctl_elem_info_set_numid",
            unsafe extern "C" fn(Ptr, c_uint)
        );
        let info = sym!("snd_ctl_elem_info", unsafe extern "C" fn(Ptr, Ptr) -> c_int);
        let name = sym!(
            "snd_ctl_elem_info_get_name",
            unsafe extern "C" fn(Ptr) -> *const c_char
        );
        let typ = sym!(
            "snd_ctl_elem_info_get_type",
            unsafe extern "C" fn(Ptr) -> c_int
        );
        let writable = sym!(
            "snd_ctl_elem_info_is_writable",
            unsafe extern "C" fn(Ptr) -> c_int
        );
        let va = sym!(
            "snd_ctl_elem_value_malloc",
            unsafe extern "C" fn(*mut Ptr) -> c_int
        );
        let vf = sym!("snd_ctl_elem_value_free", unsafe extern "C" fn(Ptr));
        let vi = sym!(
            "snd_ctl_elem_value_set_numid",
            unsafe extern "C" fn(Ptr, c_uint)
        );
        let read = sym!("snd_ctl_elem_read", unsafe extern "C" fn(Ptr, Ptr) -> c_int);
        let get = sym!(
            "snd_ctl_elem_value_get_boolean",
            unsafe extern "C" fn(Ptr, c_uint) -> c_long
        );
        let set = sym!(
            "snd_ctl_elem_value_set_boolean",
            unsafe extern "C" fn(Ptr, c_uint, c_long)
        );
        let put = sym!(
            "snd_ctl_elem_write",
            unsafe extern "C" fn(Ptr, Ptr) -> c_int
        );
        let id = match label {
            "Builtin Speaker Switch" => 9,
            "Builtin Microphone Switch" => 10,
            "TRS Switch" => 11,
            _ => {
                dlclose(lib);
                return Err("Unknown control".into());
            }
        };
        let mut ctl = std::ptr::null_mut();
        if open(&mut ctl, c"hw:2".as_ptr(), 0) < 0 {
            dlclose(lib);
            return Err("Codec unavailable".into());
        }
        let mut detail = std::ptr::null_mut();
        if ia(&mut detail) < 0 {
            close(ctl);
            dlclose(lib);
            return Err("No control info".into());
        }
        let mut value = std::ptr::null_mut();
        if va(&mut value) < 0 {
            free(detail);
            close(ctl);
            dlclose(lib);
            return Err("No control value".into());
        }
        let result = (|| {
            sid(detail, id);
            if info(ctl, detail) < 0
                || name(detail).is_null()
                || CStr::from_ptr(name(detail)).to_bytes() != label.as_bytes()
                || typ(detail) != 1
            {
                return Err("Codec identity mismatch".into());
            }
            vi(value, id);
            if read(ctl, value) < 0 {
                return Err("Input read failed".into());
            }
            if let Some(on) = write {
                if writable(detail) == 0 {
                    return Err("Input is read-only".into());
                }
                set(value, 0, c_long::from(on));
                if put(ctl, value) < 0 || read(ctl, value) < 0 || (get(value, 0) != 0) != on {
                    return Err("Input readback mismatch".into());
                }
            }
            Ok(get(value, 0) != 0)
        })();
        vf(value);
        free(detail);
        close(ctl);
        dlclose(lib);
        result
    }
}
#[cfg(not(target_os = "linux"))]
fn switch(_: &str, _: Option<bool>) -> Result<bool, String> {
    Err("Device only".into())
}

fn apply(source: u8, speaker: &mut Option<bool>) -> Result<(), String> {
    let mic = switch("Builtin Microphone Switch", None)?;
    let trs = switch("TRS Switch", None)?;
    let sp = switch("Builtin Speaker Switch", None)?;
    let result = (|| {
        if source == 1 {
            if !mic && speaker.is_none() {
                *speaker = Some(sp);
            }
            switch("Builtin Speaker Switch", Some(false))?;
        }
        switch("Builtin Microphone Switch", Some(false))?;
        switch("TRS Switch", Some(source == 0))?;
        if source == 1 {
            switch("Builtin Microphone Switch", Some(true))?;
        } else if mic && let Some(old) = *speaker {
            switch("Builtin Speaker Switch", Some(old))?;
        }
        Ok(())
    })();
    if result.is_err() {
        let _ = switch("Builtin Microphone Switch", Some(mic));
        let _ = switch("TRS Switch", Some(trs));
        let _ = switch("Builtin Speaker Switch", Some(sp));
    }
    result
}

#[derive(serde::Serialize, serde::Deserialize)]
struct InputPrefs {
    source: u8,
    speaker: Option<bool>,
}
static LAST_ERROR: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());
pub struct InputWorker {
    guard_ticks: u32,
    path: Option<std::path::PathBuf>,
    speaker: Option<bool>,
}
impl InputWorker {
    pub fn new(directory: Option<&std::path::Path>) -> Self {
        let mut worker = Self {
            guard_ticks: 0,
            path: directory.map(|p| p.join("InputSource.json")),
            speaker: None,
        };
        let mic = switch("Builtin Microphone Switch", None).unwrap_or(false);
        let trs = switch("TRS Switch", None).unwrap_or(true);
        SELECTED.store(
            if mic {
                1
            } else if trs {
                0
            } else {
                2
            },
            Ordering::Release,
        );
        if let Some(pref) = worker
            .path
            .as_ref()
            .filter(|p| std::fs::metadata(p).is_ok_and(|m| m.len() < 256))
            .and_then(|p| std::fs::read(p).ok())
            .and_then(|b| serde_json::from_slice::<InputPrefs>(&b).ok())
            .filter(|p| p.source < 3)
        {
            worker.speaker = pref.speaker;
            if apply(pref.source, &mut worker.speaker).is_ok() {
                SELECTED.store(pref.source, Ordering::Release);
            }
        }
        worker
    }
    pub fn tick(&mut self, active: bool) {
        if !active {
            return;
        }
        self.guard_ticks = self.guard_ticks.wrapping_add(1);
        // The native app can re-enable its speaker late during startup/page changes.
        // Enforce the MIC policy on this worker; no ALSA calls touch the audio thread.
        if selected() == 1
            && (self.guard_ticks < 150 || self.guard_ticks.is_multiple_of(5))
            && switch("Builtin Speaker Switch", None).is_ok_and(|enabled| enabled)
            && let Err(error) = switch("Builtin Speaker Switch", Some(false))
        {
            RESULT.store(2, Ordering::Release);
            if let Ok(mut last) = LAST_ERROR.lock() {
                *last = error;
            }
        }
        let request = PENDING.load(Ordering::Acquire);
        if request == 0 {
            return;
        }
        let source = request - 1;
        match apply(source, &mut self.speaker) {
            Ok(()) => {
                SELECTED.store(source, Ordering::Release);
                RESULT.store(1, Ordering::Release);
                if let Some(path) = &self.path {
                    let temporary = path.with_extension("json.tmp");
                    let bytes = serde_json::to_vec(&InputPrefs {
                        source,
                        speaker: self.speaker,
                    })
                    .unwrap_or_default();
                    if std::fs::write(&temporary, bytes)
                        .and_then(|()| std::fs::rename(&temporary, path))
                        .is_err()
                    {
                        RESULT.store(3, Ordering::Release);
                    }
                }
            }
            Err(error) => {
                RESULT.store(2, Ordering::Release);
                if let Ok(mut last) = LAST_ERROR.lock() {
                    *last = error;
                }
            }
        }
        PENDING.store(0, Ordering::Release);
    }
}

pub fn status() -> serde_json::Value {
    serde_json::json!({"selected":selected(),"pending":pending(),"result":result(),"error":LAST_ERROR.lock().map(|s|s.clone()).ok(),"mic":switch("Builtin Microphone Switch",None).ok(),"line":switch("TRS Switch",None).ok(),"speaker":switch("Builtin Speaker Switch",None).ok()})
}
