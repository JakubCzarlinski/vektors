use crate::allocation::Allocation;
use crate::error::AllocatorError;
use crate::memory::arena_key::ArenaKey;
use crate::memory::block::BlockMemory;
use crate::memory::owned::OwnedMemory;
use crate::stats::StatsState;
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::vec::Vec;
use parking_lot::RwLock;

#[derive(Debug)]
pub(crate) struct ArenaState {
    pub(crate) next_block_id: u32,
    blocks: Vec<Arc<BlockMemory>>,
}

impl ArenaState {
    pub(crate) const fn new() -> Self {
        Self {
            next_block_id: 1,
            blocks: Vec::new(),
        }
    }

    pub(crate) fn allocate_from_existing_locked(
        &self,
        request_size: u64,
        alignment: u64,
    ) -> Option<Allocation> {
        self.blocks.iter().find_map(|block| {
            let offset = block.allocate(request_size, alignment)?;
            Some(Allocation::new(
                block.id,
                block.arena_id,
                offset,
                request_size,
                block.mapped_with_offset(offset),
                block.clone(),
            ))
        })
    }

    pub(crate) fn push_block(&mut self, block: Arc<BlockMemory>) {
        self.blocks.push(block);
        self.next_block_id += 1;
    }
}

pub(crate) type SharedArena = Arc<RwLock<ArenaState>>;
pub(crate) type ArenaRegistry = BTreeMap<ArenaKey, SharedArena>;

/// Tries existing blocks without holding the arena lock while mutating a
/// block's range allocator. Blocks are never removed, so cloning one under a
/// short read lock is sufficient to keep it alive for the attempt.
pub(crate) fn allocate_from_existing(
    arena: &SharedArena,
    request_size: u64,
    alignment: u64,
) -> Option<Allocation> {
    let mut index = 0;
    loop {
        let block = {
            let arena = arena.read();
            arena.blocks.get(index).cloned()
        }?;
        if let Some(offset) = block.allocate(request_size, alignment) {
            return Some(Allocation::new(
                block.id,
                block.arena_id,
                offset,
                request_size,
                block.mapped_with_offset(offset),
                block,
            ));
        }
        index += 1;
    }
}

pub(crate) fn make_block_allocation(
    block: Arc<BlockMemory>,
    request_size: u64,
    alignment: u64,
) -> Result<Allocation, AllocatorError> {
    let offset = block
        .allocate(request_size, alignment)
        .ok_or(AllocatorError::OutOfAllocatorMetadata)?;
    Ok(Allocation::new(
        block.id,
        block.arena_id,
        offset,
        request_size,
        block.mapped_with_offset(offset),
        block,
    ))
}

pub(crate) fn make_dedicated_allocation(
    block_id: u32,
    arena_id: u32,
    size: u64,
    memory: OwnedMemory,
    stats: Arc<StatsState>,
) -> Allocation {
    stats.on_dedicated();
    let source = Arc::new(BlockMemory::dedicated(
        block_id, arena_id, size, memory, stats,
    ));
    Allocation::new(
        block_id,
        arena_id,
        0,
        size,
        source.mapped_with_offset(0),
        source,
    )
}
