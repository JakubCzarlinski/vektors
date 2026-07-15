use crate::allocation::{HostImportBufferCreateInfo, ImportedHostBuffer};
use crate::error::AllocatorError;
use crate::memory::select::choose_memory_type;
use crate::resource::AllocationCreateInfo;
use crate::vulkan::limits::{
    DeviceLimits, validate_allocation_size, validate_host_pointer_alignment,
};
use crate::vulkan::requirements::buffer_requirements;
use vk::{
    Device, VkBufferCreateInfo, VkExternalMemoryBufferCreateInfo, VkImportMemoryHostPointerInfoEXT,
    VkMemoryAllocateInfo, VkMemoryHostPointerPropertiesEXT, VkPhysicalDeviceMemoryProperties, null,
};

pub(crate) fn import_host_buffer<'host, 'vk>(
    device: &'vk Device<'vk>,
    memory_properties: &VkPhysicalDeviceMemoryProperties,
    limits: &DeviceLimits,
    buffer_info: &VkBufferCreateInfo,
    HostImportBufferCreateInfo {
        host_ptr,
        size,
        handle_type,
        ..
    }: HostImportBufferCreateInfo,
    alloc_info: AllocationCreateInfo,
) -> Result<ImportedHostBuffer<'host, 'vk>, AllocatorError> {
    if host_ptr.is_null() || size == 0 {
        return Err(AllocatorError::OutOfBounds);
    }
    if buffer_info.size != size {
        return Err(AllocatorError::InvalidHostImport);
    }
    validate_host_pointer_alignment(host_ptr, limits.min_imported_host_pointer_alignment)?;
    validate_allocation_size(size, limits.max_memory_allocation_size)?;

    let buffer = {
        let external_memory_info =
            VkExternalMemoryBufferCreateInfo::DEFAULT.with_handleTypes(handle_type);
        let buffer_info =
            buffer_info.with_pNext_VkExternalMemoryBufferCreateInfo(&external_memory_info);
        device
            .vkCreateBuffer(&buffer_info, null())
            .map_err(AllocatorError::Vulkan)
    }?;

    let memory = {
        let requirement = buffer_requirements(device, &buffer).requirements;

        let host_memory_type_bits = {
            let mut host_props = VkMemoryHostPointerPropertiesEXT::DEFAULT;
            device
                .vkGetMemoryHostPointerPropertiesEXT(handle_type, host_ptr.cast(), &mut host_props)
                .map_err(AllocatorError::Vulkan)?;
            host_props.memoryTypeBits
        };

        let memory_type_index = choose_memory_type(
            memory_properties,
            host_memory_type_bits & requirement.memoryTypeBits,
            &alloc_info,
        )?;
        let import = &VkImportMemoryHostPointerInfoEXT::DEFAULT
            .with_handleType(handle_type)
            .with_pHostPointer(host_ptr.cast());
        let allocate_info = &VkMemoryAllocateInfo::DEFAULT
            .with_pNext_VkImportMemoryHostPointerInfoEXT(import)
            .with_allocationSize(requirement.size)
            .with_memoryTypeIndex(memory_type_index);
        device
            .vkAllocateMemory(allocate_info, null())
            .map_err(AllocatorError::Vulkan)?
    };
    buffer
        .vkBindBufferMemory(memory.raw(), 0)
        .map_err(AllocatorError::Vulkan)?;
    Ok(ImportedHostBuffer::new(buffer, memory, host_ptr, size))
}
