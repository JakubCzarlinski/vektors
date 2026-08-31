//! Fast hash collections whose seed state is safe across loader unloads.
//!
//! `std::collections::hash_map::RandomState` uses a TLS cache on Windows, while
//! `ahash::RandomState::default` retains two heap allocations in process-global
//! `OnceBox` values. A Vulkan loader can be loaded and unloaded repeatedly, so
//! neither lifetime is suitable here. Keep randomized `AHash` performance while
//! storing the base seeds inline in this image instead.

use core::{hash::BuildHasher, sync::atomic::AtomicUsize};
use std::{
    sync::LazyLock,
    time::{SystemTime, UNIX_EPOCH},
};

static SEQUENCE: AtomicUsize = AtomicUsize::new(0);
static BASE_SEEDS: LazyLock<[u64; 4]> = LazyLock::new(|| {
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
});

#[derive(Clone)]
pub(crate) struct RandomState(ahash::RandomState);

impl Default for RandomState {
    #[inline]
    fn default() -> Self {
        let [k0, k1, k2, k3] = *BASE_SEEDS;
        let sequence = SEQUENCE.fetch_add(1, core::sync::atomic::Ordering::Relaxed) as u64;
        Self(ahash::RandomState::with_seeds(
            k0,
            k1 ^ sequence,
            k2,
            k3 ^ sequence.rotate_left(29),
        ))
    }
}

impl BuildHasher for RandomState {
    type Hasher = <ahash::RandomState as BuildHasher>::Hasher;

    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        self.0.build_hasher()
    }
}

pub(crate) type HashMap<K, V> = std::collections::HashMap<K, V, RandomState>;
pub(crate) type HashSet<T> = std::collections::HashSet<T, RandomState>;
