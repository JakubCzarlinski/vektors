use crate::vulkan::limits::DeviceLimits;
use vk::{
    Device, Image, VkBaseInStructure, VkBufferCreateInfo, VkBufferMemoryRequirementsInfo2,
    VkBufferUsageFlagBits2, VkBufferUsageFlags, VkBufferUsageFlags2, VkBufferUsageFlags2CreateInfo,
    VkImageMemoryRequirementsInfo2, VkMemoryDedicatedRequirements, VkMemoryRequirements,
    VkMemoryRequirements2,
};

#[derive(Debug, Clone, Copy)]
pub(crate) struct RequirementInfo {
    pub(crate) requirements: VkMemoryRequirements,
    pub(crate) dedicated_required: bool,
    pub(crate) dedicated_preferred: bool,
}

pub(crate) fn buffer_requirements(device: &Device<'_>, buffer: &vk::Buffer<'_>) -> RequirementInfo {
    let mut dedicated = VkMemoryDedicatedRequirements::DEFAULT;
    let mut req =
        VkMemoryRequirements2::DEFAULT.with_pNext_VkMemoryDedicatedRequirements(&mut dedicated);
    {
        let info = VkBufferMemoryRequirementsInfo2::DEFAULT.with_buffer(buffer.raw());
        device.vkGetBufferMemoryRequirements2(&info, &mut req);
    }
    RequirementInfo {
        requirements: req.memoryRequirements,
        dedicated_required: dedicated.requiresDedicatedAllocation == vk::VK_TRUE,
        dedicated_preferred: dedicated.prefersDedicatedAllocation == vk::VK_TRUE,
    }
}

pub(crate) fn image_requirements(device: &Device<'_>, image: &Image<'_>) -> RequirementInfo {
    let mut dedicated = VkMemoryDedicatedRequirements::DEFAULT;
    let mut req =
        VkMemoryRequirements2::DEFAULT.with_pNext_VkMemoryDedicatedRequirements(&mut dedicated);
    {
        let info = VkImageMemoryRequirementsInfo2::DEFAULT.with_image(image.raw());
        device.vkGetImageMemoryRequirements2(&info, &mut req);
    }
    RequirementInfo {
        requirements: req.memoryRequirements,
        dedicated_required: dedicated.requiresDedicatedAllocation == vk::VK_TRUE,
        dedicated_preferred: dedicated.prefersDedicatedAllocation == vk::VK_TRUE,
    }
}

pub(crate) fn buffer_usage_flags2(buffer_info: &VkBufferCreateInfo) -> VkBufferUsageFlags2 {
    let mut next = buffer_info.pNext.cast::<VkBaseInStructure>();
    while !next.is_null() {
        let base = unsafe { &*next };
        if base.sType == vk::VkStructureType::BUFFER_USAGE_FLAGS_2_CREATE_INFO {
            let usage_info = unsafe { &*next.cast::<VkBufferUsageFlags2CreateInfo>() };
            return usage_info.usage;
        }
        next = base.pNext;
    }
    legacy_buffer_usage_flags2(buffer_info.usage)
}

#[allow(deprecated)]
fn legacy_buffer_usage_flags2(usage: VkBufferUsageFlags) -> VkBufferUsageFlags2 {
    VkBufferUsageFlagBits2(usage.0 as u64)
}

pub(crate) fn recommended_buffer_chunk_size(
    total_size: u64,
    usage_flags: VkBufferUsageFlags2,
    limits: DeviceLimits,
) -> u64 {
    let mut chunk = if limits.max_memory_allocation_size != 0 {
        limits.max_memory_allocation_size
    } else {
        total_size
    };
    if usage_flags.intersects(VkBufferUsageFlagBits2::STORAGE_BUFFER)
        && limits.max_storage_buffer_range != 0
    {
        chunk = chunk.min(u64::from(limits.max_storage_buffer_range));
    }
    if usage_flags.intersects(VkBufferUsageFlagBits2::UNIFORM_BUFFER)
        && limits.max_uniform_buffer_range != 0
    {
        chunk = chunk.min(u64::from(limits.max_uniform_buffer_range));
    }
    chunk.max(1)
}
