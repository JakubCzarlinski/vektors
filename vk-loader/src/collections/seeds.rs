//! Inline hash seeds without a waiter queue or thread-local allocation.

use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
};
use std::time::{SystemTime, UNIX_EPOCH};

static BASE_SEEDS: SeedCache = SeedCache::new();

struct SeedCache {
    claimed: AtomicBool,
    ready: AtomicBool,
    value: UnsafeCell<[u64; 4]>,
}

// SAFETY: Exactly one publisher writes value, before publishing ready with
// release ordering. Readers access value only after an acquire observes ready;
// it is never modified afterwards. Losing publishers use their local seeds.
unsafe impl Sync for SeedCache {}

impl SeedCache {
    const fn new() -> Self {
        Self {
            claimed: AtomicBool::new(false),
            ready: AtomicBool::new(false),
            value: UnsafeCell::new([0; 4]),
        }
    }

    fn get_or_init(&self, initialize: impl FnOnce() -> [u64; 4]) -> [u64; 4] {
        if self.ready.load(Ordering::Acquire) {
            // SAFETY: Acquire observes publication of the complete seed array.
            // There are no writes after publication, and self remains borrowed.
            return unsafe { *self.value.get() };
        }
        // Compute before claiming publication: even a panicking initializer
        // cannot strand the cache. Independent maps may use different seeds.
        let seeds = initialize();
        if self
            .claimed
            .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
        {
            // SAFETY: The successful claim grants exclusive write access.
            // Readers cannot access value until the following release store.
            unsafe { *self.value.get() = seeds };
            self.ready.store(true, Ordering::Release);
        }
        // Do not wait on an in-flight publisher: this map owns its seed copy.
        seeds
    }
}

pub(super) fn base_seeds() -> [u64; 4] {
    BASE_SEEDS.get_or_init(generate_seeds)
}

fn generate_seeds() -> [u64; 4] {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos());
    let image_address = core::ptr::addr_of!(BASE_SEEDS) as usize as u64;
    let process = u64::from(std::process::id());
    [
        time as u64,
        (time >> 64) as u64 ^ image_address.rotate_left(17),
        process ^ image_address.rotate_right(13),
        crate::platform::current_thread_key() as u64 ^ time as u64,
    ]
}

#[cfg(test)]
mod tests {
    use super::{SeedCache, generate_seeds};
    use crate::allocation::fault::sweep_operation;

    #[test]
    fn seed_initialization_and_cached_reads_do_not_allocate() {
        sweep_operation(|| {
            let cache = SeedCache::new();
            let seeds = cache.get_or_init(generate_seeds);
            assert_eq!(cache.get_or_init(|| panic!("already initialized")), seeds);
            vk::VkResult::SUCCESS
        });
    }

    #[test]
    fn concurrent_initialization_publishes_one_complete_seed_set() {
        let cache = SeedCache::new();
        let start = std::sync::Barrier::new(4);
        std::thread::scope(|scope| {
            for seed in 1..=4 {
                let cache = &cache;
                let start = &start;
                scope.spawn(move || {
                    let expected = [seed, seed + 10, seed + 20, seed + 30];
                    let actual = cache.get_or_init(|| {
                        // All callers enter initialization before any can publish.
                        start.wait();
                        expected
                    });
                    assert_eq!(actual, expected);
                });
            }
        });
        let published = cache.get_or_init(|| panic!("publication missing"));
        assert!((1..=4).contains(&published[0]));
        assert_eq!(
            published,
            [
                published[0],
                published[0] + 10,
                published[0] + 20,
                published[0] + 30
            ]
        );
    }

    #[test]
    fn initializer_panic_does_not_prevent_publication() {
        let cache = SeedCache::new();
        let panic = std::panic::catch_unwind(core::panic::AssertUnwindSafe(|| {
            cache.get_or_init(|| panic!("initializer failed"));
        }));
        assert!(panic.is_err());
        assert_eq!(cache.get_or_init(|| [1, 2, 3, 4]), [1, 2, 3, 4]);
        assert_eq!(
            cache.get_or_init(|| panic!("already initialized")),
            [1, 2, 3, 4]
        );
    }
}
