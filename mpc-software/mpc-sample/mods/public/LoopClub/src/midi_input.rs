//! Native ALSA sequencer observation; caller bytes/results are never changed.
//! ABI: alsa-project/alsa-lib include/seq_event.h (fixed header and note data).
use std::sync::atomic::{AtomicBool, Ordering};
static OBSERVED: AtomicBool = AtomicBool::new(false);
pub(crate) fn observed() -> bool {
    OBSERVED.load(Ordering::Acquire)
}
#[cfg(any(test, all(target_os = "linux", target_arch = "aarch64")))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Note {
    source: u32,
    pitch: u8,
    velocity: u8,
    on: bool,
}
#[cfg(any(test, all(target_os = "linux", target_arch = "aarch64")))]
fn decode(bytes: &[u8]) -> Option<Note> {
    if bytes.len() < 20
        || !matches!(bytes[0], 6 | 7)
        || bytes[1] & 0x20 != 0
        || bytes[16] > 15
        || bytes[17] > 127
        || bytes[18] > 127
    {
        return None;
    }
    let on = bytes[0] == 6 && bytes[18] != 0;
    Some(Note {
        // Distinguish ALSA client, port and MIDI channel; stable within a run.
        source: 0x4c00_0000
            | (u32::from(bytes[12]) << 12)
            | (u32::from(bytes[13]) << 4)
            | u32::from(bytes[16]),
        pitch: bytes[17],
        velocity: if on { bytes[18] } else { 0 },
        on,
    })
}
#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
mod native {
    use std::{
        ffi::{c_char, c_void},
        sync::OnceLock,
    };
    type Read = unsafe extern "C" fn(usize, *mut *mut c_void) -> i32;
    unsafe extern "C" {
        fn dlsym(handle: *mut c_void, name: *const c_char) -> *mut c_void;
    }
    static ORIGINAL: OnceLock<Option<Read>> = OnceLock::new();
    /// # Safety
    /// The ALSA caller supplies a live sequencer handle and writable event-result
    /// pointer. The original function receives those arguments unchanged.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn snd_seq_event_input(handle: usize, event: *mut *mut c_void) -> i32 {
        let original = *ORIGINAL.get_or_init(|| {
            // SAFETY: static NUL-terminated ALSA symbol, resolved after this mod.
            let ptr = unsafe { dlsym((-1isize) as *mut c_void, c"snd_seq_event_input".as_ptr()) };
            if ptr.is_null() {
                None
            } else {
                // SAFETY: the resolved symbol has ALSA's declared read signature.
                Some(unsafe { std::mem::transmute::<*mut c_void, Read>(ptr) })
            }
        });
        let Some(read) = original else {
            return -1;
        };
        // SAFETY: delegate the original caller arguments unchanged.
        let result = unsafe { read(handle, event) };
        if result >= 0 && !event.is_null() {
            // SAFETY: successful ALSA read initialized this result pointer.
            let ptr = unsafe { *event };
            if !ptr.is_null() {
                // SAFETY: snd_seq_event_t's initialized fixed header/note data
                // has at least20bytes, including for non-note event variants.
                let bytes = unsafe { std::slice::from_raw_parts(ptr.cast::<u8>(), 20) };
                if let Some(note) = super::decode(bytes) {
                    super::OBSERVED.store(true, super::Ordering::Release);
                    crate::qa::midi_input_trace(
                        4,
                        note.source,
                        note.pitch,
                        u8::from(note.on),
                        note.velocity,
                    );
                    crate::device::sequencer_note(note.source, note.pitch, note.velocity, note.on);
                }
            }
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn event(kind: u8, channel: u8, note: u8, velocity: u8) -> [u8; 28] {
        let mut b = [0; 28];
        b[0] = kind;
        b[12] = 32;
        b[13] = 1;
        b[16] = channel;
        b[17] = note;
        b[18] = velocity;
        b
    }
    #[test]
    fn both_note_off_forms_close_the_same_owner_and_ignore_release_velocity() {
        let on = decode(&event(6, 3, 60, 85)).unwrap();
        for bytes in [event(7, 3, 60, 64), event(6, 3, 60, 0)] {
            let off = decode(&bytes).unwrap();
            assert_eq!(off.source, on.source);
            assert!(!off.on);
            assert_eq!(off.velocity, 0);
            assert_eq!(off.pitch, 60);
        }
    }
    #[test]
    fn ports_channels_and_clients_have_independent_ownership() {
        let mut b = event(6, 0, 60, 80);
        let first = decode(&b).unwrap().source;
        b[16] = 1;
        assert_ne!(decode(&b).unwrap().source, first);
        b[16] = 0;
        b[13] = 2;
        assert_ne!(decode(&b).unwrap().source, first);
        b[13] = 1;
        b[12] = 33;
        assert_ne!(decode(&b).unwrap().source, first);
    }
    #[test]
    fn malformed_non_note_and_ump_events_are_ignored() {
        assert_eq!(decode(&[]), None);
        assert_eq!(decode(&event(10, 0, 60, 80)), None);
        assert_eq!(decode(&event(6, 16, 60, 80)), None);
        assert_eq!(decode(&event(6, 0, 128, 80)), None);
        assert_eq!(decode(&event(6, 0, 60, 128)), None);
        let mut ump = event(6, 0, 60, 80);
        ump[1] = 0x20;
        assert_eq!(decode(&ump), None);
    }
}
