use crate::error::AllocatorError;
use core::ffi::c_void;
use core::ptr::{null, null_mut};
use core::sync::atomic::{AtomicPtr, Ordering};
use vk::{
    PFN_vkFlushMappedMemoryRanges, PFN_vkFreeMemory, PFN_vkInvalidateMappedMemoryRanges,
    PFN_vkMapMemory, PFN_vkUnmapMemory, VkBuffer, VkDevice, VkDeviceMemory, VkImage,
    VkMappedMemoryRange, VkMemoryAllocateFlagBits, VkMemoryMapFlagBits, VkResult,
};

#[derive(Debug)]
pub(crate) struct DeviceFns {
    pub(crate) device: VkDevice,
    pub(crate) free_memory: PFN_vkFreeMemory,
    pub(crate) map_memory: PFN_vkMapMemory,
    pub(crate) unmap_memory: PFN_vkUnmapMemory,
    pub(crate) flush_mapped_memory_ranges: PFN_vkFlushMappedMemoryRanges,
    pub(crate) invalidate_mapped_memory_ranges: PFN_vkInvalidateMappedMemoryRanges,
}

unsafe impl Send for DeviceFns {}
unsafe impl Sync for DeviceFns {}

#[derive(Debug)]
pub(crate) struct OwnedMemory {
    raw: VkDeviceMemory,
    size: u64,
    mapped: AtomicPtr<u8>,
    host_visible: bool,
    host_coherent: bool,
    non_coherent_atom_size: u64,
    device_fns: DeviceFns,
}

unsafe impl Send for OwnedMemory {}
unsafe impl Sync for OwnedMemory {}

impl OwnedMemory {
    pub(crate) fn new(
        raw: VkDeviceMemory,
        size: u64,
        host_visible: bool,
        host_coherent: bool,
        non_coherent_atom_size: u64,
        device_fns: DeviceFns,
    ) -> Self {
        Self {
            raw,
            size,
            mapped: AtomicPtr::new(null_mut()),
            host_visible,
            host_coherent,
            non_coherent_atom_size,
            device_fns,
        }
    }

    pub(crate) const fn raw(&self) -> VkDeviceMemory {
        self.raw
    }

    pub(crate) const fn size(&self) -> u64 {
        self.size
    }

    pub(crate) fn mapped_ptr(&self) -> *mut u8 {
        // A value of address 1 is an internal "mapping in progress" sentinel.
        // Vulkan mappings are aligned and can never validly have this address.
        const MAPPING: *mut u8 = core::ptr::dangling_mut::<u8>();

        if !self.host_visible {
            return null_mut();
        }

        loop {
            let cached = self.mapped.load(Ordering::Acquire);
            if cached == MAPPING {
                core::hint::spin_loop();
                continue;
            }
            if !cached.is_null() {
                return cached;
            }
            if self
                .mapped
                .compare_exchange(null_mut(), MAPPING, Ordering::AcqRel, Ordering::Acquire)
                .is_err()
            {
                continue;
            }

            let mut out: *mut c_void = null_mut();
            let result = unsafe {
                (self.device_fns.map_memory)(
                    self.device_fns.device,
                    self.raw,
                    0,
                    self.size,
                    VkMemoryMapFlagBits::EMPTY,
                    &raw mut out,
                )
            };
            let mapped = if result >= VkResult::SUCCESS {
                out.cast::<u8>()
            } else {
                null_mut()
            };
            self.mapped.store(mapped, Ordering::Release);
            return mapped;
        }
    }

    pub(crate) fn flush_range(&self, offset: u64, size: u64) -> Result<(), AllocatorError> {
        self.sync_range(offset, size, self.device_fns.flush_mapped_memory_ranges)
    }

    pub(crate) fn invalidate_range(&self, offset: u64, size: u64) -> Result<(), AllocatorError> {
        self.sync_range(
            offset,
            size,
            self.device_fns.invalidate_mapped_memory_ranges,
        )
    }

    fn sync_range(
        &self,
        offset: u64,
        size: u64,
        operation: unsafe extern "system" fn(
            VkDevice,
            u32,
            *const VkMappedMemoryRange<'_>,
        ) -> VkResult,
    ) -> Result<(), AllocatorError> {
        if !self.host_visible {
            return Err(AllocatorError::HostVisibleRequired);
        }
        if self.host_coherent {
            return Ok(());
        }
        let end = offset
            .checked_add(size)
            .ok_or(AllocatorError::OutOfBounds)?;
        if end > self.size {
            return Err(AllocatorError::OutOfBounds);
        }
        let atom = self.non_coherent_atom_size.max(1);
        let start = offset - offset % atom;
        let aligned_end = if end == self.size {
            end
        } else {
            end.checked_add(atom - 1)
                .ok_or(AllocatorError::OutOfBounds)?
                / atom
                * atom
        };
        let range = VkMappedMemoryRange::DEFAULT
            .with_memory(self.raw)
            .with_offset(start)
            .with_size(aligned_end - start);
        let result = unsafe { operation(self.device_fns.device, 1, &raw const range) };
        if result >= VkResult::SUCCESS {
            Ok(())
        } else {
            Err(AllocatorError::Vulkan(result))
        }
    }
}

impl Drop for OwnedMemory {
    fn drop(&mut self) {
        let mapped = self.mapped.swap(null_mut(), Ordering::AcqRel);
        if !mapped.is_null() {
            unsafe {
                (self.device_fns.unmap_memory)(self.device_fns.device, self.raw);
            }
        }
        unsafe {
            (self.device_fns.free_memory)(self.device_fns.device, self.raw, null());
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn allocate_owned_memory(
    device: &vk::Device<'_>,
    requirements: &vk::VkMemoryRequirements,
    memory_type_index: u32,
    dedicated_buffer: Option<VkBuffer>,
    dedicated_image: Option<VkImage>,
    device_mask: Option<u32>,
    host_visible: bool,
    host_coherent: bool,
    non_coherent_atom_size: u64,
) -> Result<OwnedMemory, AllocatorError> {
    let mut dedicated_info = vk::VkMemoryDedicatedAllocateInfo::DEFAULT;
    let mut flags_info = vk::VkMemoryAllocateFlagsInfo::DEFAULT;
    let mut allocate_info = vk::VkMemoryAllocateInfo::DEFAULT
        .with_allocationSize(requirements.size)
        .with_memoryTypeIndex(memory_type_index);

    let mut next = null::<c_void>();
    if dedicated_buffer.is_some() || dedicated_image.is_some() {
        dedicated_info = dedicated_info
            .with_buffer(dedicated_buffer.unwrap_or(VkBuffer::NULL))
            .with_image(dedicated_image.unwrap_or(VkImage::NULL));
        next = (&raw const dedicated_info).cast::<c_void>();
    }
    if let Some(mask) = device_mask {
        flags_info = flags_info
            .with_flags(VkMemoryAllocateFlagBits::DEVICE_MASK)
            .with_deviceMask(mask)
            .with_pNext(next);
        next = (&raw const flags_info).cast::<c_void>();
    }
    if !next.is_null() {
        allocate_info = allocate_info.with_pNext(next);
    }

    let memory = device
        .vkAllocateMemory(&allocate_info, null())
        .map_err(AllocatorError::Vulkan)?;
    let free_memory = memory.table().vkFreeMemory.ok_or(AllocatorError::Vulkan(
        VkResult::ERROR_INITIALIZATION_FAILED,
    ))?;
    let map_memory = memory.table().vkMapMemory.ok_or(AllocatorError::Vulkan(
        VkResult::ERROR_INITIALIZATION_FAILED,
    ))?;
    let unmap_memory = memory.table().vkUnmapMemory.ok_or(AllocatorError::Vulkan(
        VkResult::ERROR_INITIALIZATION_FAILED,
    ))?;
    let flush_mapped_memory_ranges =
        device
            .table()
            .vkFlushMappedMemoryRanges
            .ok_or(AllocatorError::Vulkan(
                VkResult::ERROR_INITIALIZATION_FAILED,
            ))?;
    let invalidate_mapped_memory_ranges =
        device
            .table()
            .vkInvalidateMappedMemoryRanges
            .ok_or(AllocatorError::Vulkan(
                VkResult::ERROR_INITIALIZATION_FAILED,
            ))?;
    let device_fns = DeviceFns {
        device: memory.parent().raw(),
        free_memory,
        map_memory,
        unmap_memory,
        flush_mapped_memory_ranges,
        invalidate_mapped_memory_ranges,
    };
    let raw = memory.raw();
    core::mem::forget(memory);
    Ok(OwnedMemory::new(
        raw,
        requirements.size,
        host_visible,
        host_coherent,
        non_coherent_atom_size,
        device_fns,
    ))
}
