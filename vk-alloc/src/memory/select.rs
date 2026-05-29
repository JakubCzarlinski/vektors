use crate::error::AllocatorError;
use crate::pool::PoolConfig;
use crate::resource::{AllocationCreateInfo, AllocationStrategy, MemoryTypePolicy};
use crate::vulkan::requirements::RequirementInfo;
use vk::{VkMemoryPropertyFlagBits, VkMemoryPropertyFlags, VkPhysicalDeviceMemoryProperties};

pub(crate) fn score_memory_type(
    policy: MemoryTypePolicy,
    property_flags: VkMemoryPropertyFlags,
) -> Option<i32> {
    if !property_flags.contains(policy.required_flags) {
        return None;
    }
    let mut score = 0;
    score += ((property_flags & policy.preferred_flags).0.count_ones() as i32) * 16;
    score -= ((property_flags & policy.avoid_flags).0.count_ones() as i32) * 8;
    score += (property_flags.0.count_ones() as i32) * 2;
    Some(score)
}

pub(crate) fn choose_memory_type(
    properties: &VkPhysicalDeviceMemoryProperties,
    memory_type_bits: u32,
    alloc_info: &AllocationCreateInfo,
) -> Result<u32, AllocatorError> {
    let mut best = None;
    for index in 0..properties.memoryTypeCount {
        let mask = 1u32 << index;
        if memory_type_bits & mask == 0 {
            continue;
        }
        let property_flags = properties.memoryTypes[index as usize].propertyFlags;
        if let Some(score) = score_memory_type(alloc_info.memory_type_policy, property_flags) {
            match best {
                Some((_, best_score)) if best_score >= score => {}
                _ => best = Some((index, score)),
            }
        }
    }
    best.map(|(index, _)| index)
        .ok_or(AllocatorError::NoCompatibleMemoryType)
}

pub(crate) const fn is_host_visible(
    properties: &VkPhysicalDeviceMemoryProperties,
    memory_type_index: u32,
) -> bool {
    properties.memoryTypes[memory_type_index as usize]
        .propertyFlags
        .intersects(VkMemoryPropertyFlagBits::HOST_VISIBLE)
}

pub(crate) const fn block_size_for(
    properties: &VkPhysicalDeviceMemoryProperties,
    memory_type_index: u32,
    pool: &PoolConfig,
) -> u64 {
    let memory_type = properties.memoryTypes[memory_type_index as usize].propertyFlags;
    if memory_type.intersects(VkMemoryPropertyFlagBits::HOST_VISIBLE) {
        pool.host_visible_block_size
    } else if memory_type.intersects(VkMemoryPropertyFlagBits::DEVICE_LOCAL) {
        pool.device_local_block_size
    } else {
        pool.mixed_block_size
    }
}

pub(crate) fn should_dedicate(
    alloc_info: &AllocationCreateInfo,
    requirements: RequirementInfo,
    arena_block_size: u64,
) -> Result<bool, AllocatorError> {
    if requirements.dedicated_required {
        return match alloc_info.strategy {
            AllocationStrategy::NeverDedicated => Err(AllocatorError::DedicatedAllocationRequired),
            _ => Ok(true),
        };
    }
    match alloc_info.strategy {
        AllocationStrategy::AlwaysDedicated => Ok(true),
        AllocationStrategy::NeverDedicated => Ok(false),
        AllocationStrategy::Auto => {
            let threshold = alloc_info
                .dedicated_threshold
                .unwrap_or(arena_block_size / 2);
            Ok(requirements.requirements.size >= threshold || requirements.dedicated_preferred)
        }
    }
}
