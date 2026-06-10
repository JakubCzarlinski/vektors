use crate::allocation::Allocation;
use crate::error::AllocatorError;
use crate::group::device_mask::partition_device_mask;
use crate::group_allocator::GroupBindMode;
use alloc::boxed::Box;
use vk::{
    Buffer, Device, Image, VkBindBufferMemoryDeviceGroupInfo, VkBindBufferMemoryInfo,
    VkBindImageMemoryDeviceGroupInfo, VkBindImageMemoryInfo, VkRect2D,
};

#[inline]
fn instance0_indices() -> Box<[u32]> {
    Box::new([0])
}

pub(crate) fn bind_buffer<'vk>(
    device: &Device<'vk>,
    buffer: &Buffer<'vk>,
    allocation: &Allocation,
    mode: GroupBindMode,
    device_mask: u32,
) -> Result<(), AllocatorError> {
    let device_indices = match mode {
        GroupBindMode::Instance0 => instance0_indices(),
        GroupBindMode::PerDeviceInstance => partition_device_mask(device_mask),
        GroupBindMode::SplitInstanceRegions => {
            return Err(AllocatorError::GroupModeUnsupported);
        }
    };
    let group_info =
        VkBindBufferMemoryDeviceGroupInfo::DEFAULT.with_pDeviceIndices(&device_indices);
    let bind = VkBindBufferMemoryInfo::DEFAULT
        .with_pNext_VkBindBufferMemoryDeviceGroupInfo(&group_info)
        .with_buffer(buffer.raw())
        .with_memory(allocation.memory())
        .with_memoryOffset(allocation.offset());
    device
        .vkBindBufferMemory2(&[bind])
        .map_err(AllocatorError::Vulkan)?;
    Ok(())
}

pub(crate) fn bind_image<'vk>(
    device: &Device<'vk>,
    image: &Image<'vk>,
    allocation: &Allocation,
    mode: GroupBindMode,
    device_mask: u32,
    split_regions: &[VkRect2D],
) -> Result<(), AllocatorError> {
    let device_indices = match mode {
        GroupBindMode::Instance0 => instance0_indices(),
        GroupBindMode::PerDeviceInstance | GroupBindMode::SplitInstanceRegions => {
            partition_device_mask(device_mask)
        }
    };
    let group_info = VkBindImageMemoryDeviceGroupInfo::DEFAULT
        .with_pDeviceIndices(&device_indices)
        .with_pSplitInstanceBindRegions(split_regions);
    let bind = &[VkBindImageMemoryInfo::DEFAULT
        .with_pNext_VkBindImageMemoryDeviceGroupInfo(&group_info)
        .with_image(image.raw())
        .with_memory(allocation.memory())
        .with_memoryOffset(allocation.offset())];
    device
        .vkBindImageMemory2(bind)
        .map_err(AllocatorError::Vulkan)?;
    Ok(())
}
