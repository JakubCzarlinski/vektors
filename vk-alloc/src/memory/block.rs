use crate::memory::owned::OwnedMemory;
use crate::memory::range_allocator::RangeAllocator;
use crate::stats::StatsState;
use alloc::sync::Arc;
use core::ptr::null_mut;
use parking_lot::Mutex;
use vk::VkDeviceMemory;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BlockMetadata {
    Single,
    Group { device_mask: u32 },
}

#[derive(Debug)]
enum BlockRanges {
    Suballocated(Mutex<RangeAllocator>),
    Dedicated,
}

/// Concrete backing source shared by allocations.
///
/// Using one representation for normal and dedicated memory keeps
/// `Allocation` to a thin `Arc` and avoids trait-object dispatch.
#[derive(Debug)]
pub(crate) struct BlockMemory {
    pub(crate) id: u32,
    pub(crate) arena_id: u32,
    pub(crate) size: u64,
    _metadata: BlockMetadata,
    pub(crate) memory: OwnedMemory,
    ranges: BlockRanges,
    stats: Arc<StatsState>,
}

impl BlockMemory {
    pub(crate) fn new(
        id: u32,
        arena_id: u32,
        size: u64,
        metadata: BlockMetadata,
        memory: OwnedMemory,
        stats: Arc<StatsState>,
    ) -> Self {
        Self {
            id,
            arena_id,
            size,
            _metadata: metadata,
            memory,
            ranges: BlockRanges::Suballocated(Mutex::new(RangeAllocator::new(size))),
            stats,
        }
    }

    pub(crate) fn dedicated(
        id: u32,
        arena_id: u32,
        size: u64,
        memory: OwnedMemory,
        stats: Arc<StatsState>,
    ) -> Self {
        Self {
            id,
            arena_id,
            size,
            _metadata: BlockMetadata::Single,
            memory,
            ranges: BlockRanges::Dedicated,
            stats,
        }
    }

    pub(crate) fn allocate(&self, size: u64, alignment: u64) -> Option<u64> {
        match &self.ranges {
            BlockRanges::Suballocated(ranges) => ranges.lock().allocate(size, alignment),
            BlockRanges::Dedicated => None,
        }
    }

    pub(crate) fn raw_memory(&self) -> VkDeviceMemory {
        self.memory.raw()
    }

    pub(crate) fn mapped_with_offset(&self, offset: u64) -> *mut u8 {
        let base = self.memory.mapped_ptr();
        if base.is_null() {
            null_mut()
        } else {
            unsafe { base.add(offset as usize) }
        }
    }

    pub(crate) fn free_range(&self, offset: u64, size: u64) {
        if let BlockRanges::Suballocated(ranges) = &self.ranges {
            ranges.lock().free(offset, size);
        }
        self.stats.on_free();
    }

    pub(crate) fn flush_range(&self, offset: u64, size: u64) -> Result<(), crate::AllocatorError> {
        self.memory.flush_range(offset, size)
    }

    pub(crate) fn invalidate_range(
        &self,
        offset: u64,
        size: u64,
    ) -> Result<(), crate::AllocatorError> {
        self.memory.invalidate_range(offset, size)
    }
}

pub(crate) type SharedSource = Arc<BlockMemory>;
