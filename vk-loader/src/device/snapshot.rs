//! Fixed device mappings read without touching a shared lock word.

use super::LoaderDevice;
use core::{
    ptr,
    sync::atomic::{AtomicPtr, AtomicUsize, Ordering},
};

#[repr(align(64))]
struct Snapshot {
    sequence: AtomicUsize,
    canonical: AtomicUsize,
    alias: AtomicUsize,
    device: AtomicPtr<LoaderDevice>,
}

impl Snapshot {
    pub(super) const fn new() -> Self {
        Self {
            sequence: AtomicUsize::new(2),
            canonical: AtomicUsize::new(0),
            alias: AtomicUsize::new(0),
            device: AtomicPtr::new(ptr::null_mut()),
        }
    }

    /// Writers must hold the device registry lock. Zero permanently disables
    /// the snapshot on sequence exhaustion, preventing an ABA after wraparound.
    pub(super) fn publish(&self, canonical: usize, alias: usize, device: *mut LoaderDevice) {
        let sequence = self.sequence.load(Ordering::SeqCst);
        if sequence == 0 {
            return;
        }
        let Some(next) = sequence.checked_add(2) else {
            self.sequence.store(0, Ordering::SeqCst);
            return;
        };
        self.sequence.store(sequence + 1, Ordering::SeqCst);
        self.canonical.store(canonical, Ordering::SeqCst);
        self.alias.store(alias, Ordering::SeqCst);
        self.device.store(device, Ordering::SeqCst);
        self.sequence.store(next, Ordering::SeqCst);
    }

    /// Refresh an empty snapshot without bouncing it between devices on misses.
    /// The caller holds the same registry lock as all other publishers.
    fn is_empty(&self) -> bool {
        self.device.load(Ordering::SeqCst).is_null()
    }

    /// Invalidates before the registry releases ownership of this device.
    pub(super) fn remove(&self, device: *mut LoaderDevice) {
        if self.device.load(Ordering::SeqCst) == device {
            self.publish(0, 0, ptr::null_mut());
        }
    }

    pub(super) fn get(&self, key: usize) -> Option<*mut LoaderDevice> {
        if key == 0 {
            return None;
        }
        let before = self.sequence.load(Ordering::SeqCst);
        if before == 0 || before & 1 != 0 {
            return None;
        }
        if self.canonical.load(Ordering::SeqCst) != key && self.alias.load(Ordering::SeqCst) != key
        {
            return None;
        }
        let device = self.device.load(Ordering::SeqCst);
        if self.sequence.load(Ordering::SeqCst) != before || device.is_null() {
            return None;
        }
        // All fields are atomic and sequentially consistent: matching even
        // sequence reads exclude mixed publications. This does not pin memory;
        // dereferencing still requires the caller's live-device contract, just
        // as it does after releasing the ordinary registry lock.
        Some(device)
    }
}

/// Eight cache lines bound static storage; collisions use the owning registry.
/// Each entry occupies its own line so publication does not invalidate readers
/// of other mappings. No per-device or thread-local allocation is required.
pub(super) struct Cache([Snapshot; 8]);

impl Cache {
    pub(super) const fn new() -> Self {
        Self([const { Snapshot::new() }; 8])
    }

    const fn index(key: usize) -> usize {
        #[cfg(target_pointer_width = "64")]
        const MULTIPLIER: usize = 0x9e37_79b9_7f4a_7c15;
        #[cfg(target_pointer_width = "32")]
        const MULTIPLIER: usize = 0x9e37_79b9;
        key.wrapping_mul(MULTIPLIER) >> (usize::BITS - 3)
    }

    pub(super) fn publish(&self, canonical: usize, alias: usize, device: *mut LoaderDevice) {
        let slot = Self::index(canonical);
        self.0[slot].publish(canonical, alias, device);
        if alias != 0 && Self::index(alias) != slot {
            self.0[Self::index(alias)].publish(canonical, alias, device);
        }
    }

    pub(super) fn remove(&self, device: *mut LoaderDevice) {
        for slot in &self.0 {
            slot.remove(device);
        }
    }

    pub(super) fn get(&self, key: usize) -> Option<*mut LoaderDevice> {
        self.0[Self::index(key)].get(key)
    }

    pub(super) fn is_empty(&self, key: usize) -> bool {
        self.0[Self::index(key)].is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalidation_reuse_and_exhaustion() {
        let snapshot = Snapshot::new();
        let first = ptr::without_provenance_mut(100);
        let second = ptr::without_provenance_mut(200);
        snapshot.publish(1, 11, first);
        assert_eq!(snapshot.get(0), None);
        assert_eq!(snapshot.get(11), Some(first));
        snapshot.remove(second);
        assert_eq!(snapshot.get(1), Some(first));
        snapshot.remove(first);
        assert_eq!(snapshot.get(1), None);
        snapshot.publish(1, 0, second);
        assert_eq!(snapshot.get(1), Some(second));
        snapshot.sequence.store(usize::MAX - 1, Ordering::SeqCst);
        snapshot.publish(2, 0, first);
        snapshot.publish(1, 0, second);
        assert_eq!(snapshot.get(1), None);
    }

    #[test]
    fn collisions_fall_back_and_removal_clears_both_aliases() {
        let cache = Cache::new();
        for key in 1..=32 {
            cache.publish(key, key + 1000, ptr::without_provenance_mut(key * 100));
        }
        for key in 1..=32 {
            for alias in [key, key + 1000] {
                if let Some(device) = cache.get(alias) {
                    assert_eq!(device.addr(), key * 100);
                }
            }
        }
        assert!(cache.get(32).is_some());
        assert!(cache.get(1032).is_some());
        cache.remove(ptr::without_provenance_mut(3200));
        assert_eq!(cache.get(32), None);
        assert_eq!(cache.get(1032), None);
    }

    #[test]
    fn readers_never_mix_device_and_alias_publications() {
        let snapshot = Cache::new();
        std::thread::scope(|scope| {
            for _ in 0..8 {
                scope.spawn(|| {
                    for _ in 0..50_000 {
                        for key in [1, 11, 2, 22] {
                            if let Some(device) = snapshot.get(key) {
                                let expected = if key == 1 || key == 11 { 100 } else { 200 };
                                assert_eq!(device.addr(), expected);
                            }
                        }
                    }
                });
            }
            for _ in 0..50_000 {
                snapshot.publish(1, 11, ptr::without_provenance_mut(100));
                snapshot.publish(2, 22, ptr::without_provenance_mut(200));
                snapshot.remove(ptr::without_provenance_mut(200));
            }
        });
    }
}
