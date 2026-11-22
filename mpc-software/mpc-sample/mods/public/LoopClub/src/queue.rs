//! Bounded multi-producer / single-consumer queue. Enqueue never waits.
use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::atomic::{AtomicUsize, Ordering},
};
struct Slot<T: Copy> {
    sequence: AtomicUsize,
    value: UnsafeCell<MaybeUninit<T>>,
}
// SAFETY: sequence publication gives exclusive ownership before data access.
unsafe impl<T: Copy + Send> Sync for Slot<T> {}
pub struct Queue<T: Copy, const N: usize> {
    slots: [Slot<T>; N],
    write: AtomicUsize,
    read: AtomicUsize,
}
// SAFETY: producer reservation and consumer release synchronize each slot.
unsafe impl<T: Copy + Send, const N: usize> Sync for Queue<T, N> {}
impl<T: Copy, const N: usize> Default for Queue<T, N> {
    fn default() -> Self {
        assert!(N >= 2 && N.is_power_of_two());
        Self {
            slots: std::array::from_fn(|i| Slot {
                sequence: AtomicUsize::new(i),
                value: UnsafeCell::new(MaybeUninit::uninit()),
            }),
            write: AtomicUsize::new(0),
            read: AtomicUsize::new(0),
        }
    }
}
impl<T: Copy, const N: usize> Queue<T, N> {
    pub fn push(&self, value: T) -> bool {
        let mut p = self.write.load(Ordering::Relaxed);
        for _ in 0..8 {
            let slot = &self.slots[p & (N - 1)];
            let seq = slot.sequence.load(Ordering::Acquire);
            let diff = seq.wrapping_sub(p) as isize;
            if diff == 0 {
                match self.write.compare_exchange_weak(
                    p,
                    p.wrapping_add(1),
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => {
                        // SAFETY: this producer reserved the free slot.
                        unsafe {
                            (*slot.value.get()).write(value);
                        }
                        slot.sequence.store(p.wrapping_add(1), Ordering::Release);
                        return true;
                    }
                    Err(next) => p = next,
                }
            } else if diff < 0 {
                return false;
            } else {
                p = self.write.load(Ordering::Relaxed)
            }
        }
        false
    }
    /// Exactly one consumer may call pop at a time.
    pub fn pop(&mut self) -> Option<T> {
        self.pop_single_consumer()
    }
    /// # Safety
    /// The caller must guarantee a single consumer for this queue.
    pub unsafe fn pop_shared(&self) -> Option<T> {
        self.pop_single_consumer()
    }
    fn pop_single_consumer(&self) -> Option<T> {
        let p = self.read.load(Ordering::Relaxed);
        let slot = &self.slots[p & (N - 1)];
        if slot.sequence.load(Ordering::Acquire) != p.wrapping_add(1) {
            return None;
        } // SAFETY: the published slot is initialized and belongs to the only consumer.
        let value = unsafe { (*slot.value.get()).assume_init_read() };
        slot.sequence.store(p.wrapping_add(N), Ordering::Release);
        self.read.store(p.wrapping_add(1), Ordering::Relaxed);
        Some(value)
    }
}
