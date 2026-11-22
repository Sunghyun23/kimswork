//! Bounded lock-free fanout of generated audio to secondary native outputs.
//! The primary stream alone advances Engine. Each USB stream has its own reader;
//! missing data produces silence rather than repeating a stale block.
use crate::model::Frame;
use std::sync::atomic::{AtomicU64, Ordering};
const CAPACITY: usize = 4096;
struct Slot {
    stamp: AtomicU64,
    value: AtomicU64,
}
pub struct OutputMirror {
    frames: Box<[Slot]>,
    write: AtomicU64,
    readers: [Reader; 8],
    pub underruns: AtomicU64,
    pub overruns: AtomicU64,
}
struct Reader {
    pcm: AtomicU64,
    next: AtomicU64,
}
impl Default for OutputMirror {
    fn default() -> Self {
        Self {
            frames: (0..CAPACITY)
                .map(|_| Slot {
                    stamp: AtomicU64::new(0),
                    value: AtomicU64::new(0),
                })
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            write: AtomicU64::new(0),
            readers: std::array::from_fn(|_| Reader {
                pcm: AtomicU64::new(0),
                next: AtomicU64::new(0),
            }),
            underruns: AtomicU64::new(0),
            overruns: AtomicU64::new(0),
        }
    }
}
impl OutputMirror {
    /// Single producer: Runtime's exclusive Engine owner only.
    pub(crate) fn push(&self, frame: Frame) {
        let n = self.write.load(Ordering::Relaxed);
        let slot = &self.frames[n as usize % CAPACITY];
        slot.stamp.store(0, Ordering::Release);
        slot.value.store(
            u64::from(frame.l.to_bits()) | (u64::from(frame.r.to_bits()) << 32),
            Ordering::Release,
        );
        slot.stamp.store(n + 1, Ordering::Release);
        self.write.store(n + 1, Ordering::Release);
    }
    /// Device output callbacks are serialized by the SDK's output-tap guard.
    pub fn add(&self, pcm: usize, samples: &mut [i32], channels: usize) {
        if pcm == 0 || channels == 0 || !samples.len().is_multiple_of(channels) {
            return;
        }
        let count = samples.len() / channels;
        let write = self.write.load(Ordering::Acquire);
        let mut found = None;
        for (i, r) in self.readers.iter().enumerate() {
            let old = r.pcm.load(Ordering::Acquire);
            if old == pcm as u64 {
                found = Some((i, false));
                break;
            }
            if old == 0
                && r.pcm
                    .compare_exchange(0, pcm as u64, Ordering::AcqRel, Ordering::Relaxed)
                    .is_ok()
            {
                found = Some((i, true));
                break;
            }
        }
        let (index, fresh) = found.unwrap_or_else(|| {
            let index = self
                .readers
                .iter()
                .enumerate()
                .min_by_key(|(_, r)| r.next.load(Ordering::Relaxed))
                .map_or(0, |(i, _)| i);
            self.readers[index].pcm.store(pcm as u64, Ordering::Release);
            (index, true)
        });
        let reader = &self.readers[index];
        let mut next = if fresh {
            write.saturating_sub(count as u64)
        } else {
            reader.next.load(Ordering::Acquire)
        };
        if write.saturating_sub(next) > CAPACITY as u64 {
            next = write.saturating_sub(count.min(CAPACITY) as u64);
            self.overruns.fetch_add(1, Ordering::Relaxed);
        }
        for chunk in samples.chunks_exact_mut(channels) {
            if next >= write {
                self.underruns.fetch_add(1, Ordering::Relaxed);
                continue;
            }
            let slot = &self.frames[next as usize % CAPACITY];
            let wanted = next + 1;
            let before = slot.stamp.load(Ordering::Acquire);
            let bits = slot.value.load(Ordering::Acquire);
            let after = slot.stamp.load(Ordering::Acquire);
            next += 1;
            if before != wanted || after != wanted {
                self.underruns.fetch_add(1, Ordering::Relaxed);
                continue;
            }
            let l = f32::from_bits(bits as u32);
            let r = f32::from_bits((bits >> 32) as u32);
            for (ch, dst) in chunk.iter_mut().enumerate().take(2) {
                let signal = if channels == 1 {
                    (l + r) * 0.5
                } else if ch == 0 {
                    l
                } else {
                    r
                };
                *dst = (f64::from(*dst) + f64::from(signal) * f64::from(i32::MAX))
                    .clamp(f64::from(i32::MIN), f64::from(i32::MAX)) as i32;
            }
        }
        reader.next.store(next, Ordering::Release);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn independent_outputs_get_same_audio_without_advancing_producer() {
        let m = OutputMirror::default();
        for i in 0..128 {
            m.push(Frame {
                l: i as f32 / 512.,
                r: -(i as f32) / 512.,
            });
        }
        let mut a = [100; 256];
        let mut b = [100; 256];
        m.add(1, &mut a, 2);
        m.add(2, &mut b, 2);
        assert_eq!(a, b);
        assert!(a.iter().any(|v| *v != 100));
        assert_eq!(m.write.load(Ordering::Relaxed), 128);
        let mut dry = [37; 256];
        m.add(1, &mut dry, 2);
        assert_eq!(dry, [37; 256]);
    }
    #[test]
    fn stalled_output_recovers_to_recent_audio_not_old_notes() {
        let m = OutputMirror::default();
        for _ in 0..128 {
            m.push(Frame::mono(0.2));
        }
        m.add(1, &mut [0; 256], 2);
        for _ in 0..CAPACITY + 256 {
            m.push(Frame::ZERO);
        }
        let mut output = [0; 256];
        m.add(1, &mut output, 2);
        assert_eq!(output, [0; 256]);
        assert_eq!(m.overruns.load(Ordering::Relaxed), 1);
    }
}
