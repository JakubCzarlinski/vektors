use crate::error::AllocatorError;
use crate::memory::block::SharedSource;
use alloc::boxed::Box;
use core::mem::{self, MaybeUninit};
use core::ptr::{self, null_mut};
use core::slice;
use vk::{Buffer, DeviceMemory, Image, VkDeviceMemory, VkExternalMemoryHandleTypeFlagBits};

pub struct Allocation {
    block_handle: u32,
    offset: u64,
    size: u64,
    mapped_ptr: *mut u8,
    arena_id: u32,
    source: SharedSource,
}

unsafe impl Send for Allocation {}
unsafe impl Sync for Allocation {}

impl Allocation {
    pub(crate) const fn new(
        block_handle: u32,
        arena_id: u32,
        offset: u64,
        size: u64,
        mapped_ptr: *mut u8,
        source: SharedSource,
    ) -> Self {
        Self {
            block_handle,
            offset,
            size,
            mapped_ptr,
            arena_id,
            source,
        }
    }

    pub const fn block_handle(&self) -> u32 {
        self.block_handle
    }

    pub const fn offset(&self) -> u64 {
        self.offset
    }

    pub const fn size(&self) -> u64 {
        self.size
    }

    pub const fn arena_id(&self) -> u32 {
        self.arena_id
    }

    pub fn memory(&self) -> VkDeviceMemory {
        self.source.raw_memory()
    }

    pub const fn mapped_ptr(&self) -> *mut u8 {
        self.mapped_ptr
    }

    /// Makes host writes to this allocation visible to the device when its
    /// memory type is not host coherent.
    pub fn flush(&self) -> Result<(), AllocatorError> {
        if self.mapped_ptr.is_null() {
            return Err(AllocatorError::HostVisibleRequired);
        }
        self.source.flush_range(self.offset, self.size)
    }

    /// Makes device writes to this allocation visible to the host when its
    /// memory type is not host coherent.
    pub fn invalidate(&self) -> Result<(), AllocatorError> {
        if self.mapped_ptr.is_null() {
            return Err(AllocatorError::HostVisibleRequired);
        }
        self.source.invalidate_range(self.offset, self.size)
    }

    /// Returns a mutable typed view over this mapped allocation.
    ///
    /// The requested slice must fit entirely within this allocation and its
    /// element alignment must be compatible with the allocation offset.
    pub fn mapped_slice_mut<T>(&mut self, len: usize) -> Result<&mut [T], AllocatorError> {
        if self.mapped_ptr.is_null() {
            return Err(AllocatorError::HostVisibleRequired);
        }
        let element_size = core::mem::size_of::<T>();
        if element_size == 0 {
            return Err(AllocatorError::InvalidMappedRange);
        }
        let byte_len = len
            .checked_mul(element_size)
            .ok_or(AllocatorError::OutOfBounds)?;
        if byte_len > self.size as usize
            || self.mapped_ptr.align_offset(core::mem::align_of::<T>()) != 0
        {
            return Err(AllocatorError::OutOfBounds);
        }
        // SAFETY: the checks above establish a non-null, aligned, in-bounds
        // region for exactly `len` values of `T`.
        Ok(unsafe { slice::from_raw_parts_mut(self.mapped_ptr.cast::<T>(), len) })
    }
}

impl Drop for Allocation {
    fn drop(&mut self) {
        self.source.free_range(self.offset, self.size);
        self.mapped_ptr = null_mut();
    }
}

pub struct AllocatedBuffer<'vk> {
    buffer: Buffer<'vk>,
    allocation: Allocation,
}

impl<'vk> AllocatedBuffer<'vk> {
    pub(crate) const fn new(buffer: Buffer<'vk>, allocation: Allocation) -> Self {
        Self { buffer, allocation }
    }

    pub const fn buffer(&self) -> &Buffer<'vk> {
        &self.buffer
    }

    pub const fn allocation(&self) -> &Allocation {
        &self.allocation
    }

    pub fn allocation_mut(&mut self) -> &mut Allocation {
        &mut self.allocation
    }
}

pub struct AllocatedImage<'vk> {
    image: Image<'vk>,
    allocation: Allocation,
}

impl<'vk> AllocatedImage<'vk> {
    pub(crate) fn new(image: Image<'vk>, allocation: Allocation) -> Self {
        Self { image, allocation }
    }

    pub fn image(&self) -> &Image<'vk> {
        &self.image
    }

    pub fn allocation(&self) -> &Allocation {
        &self.allocation
    }
}

#[derive(Debug)]
pub struct HostImportBufferCreateInfo<'host> {
    pub(crate) host_ptr: *mut u8,
    pub(crate) size: u64,
    pub(crate) handle_type: VkExternalMemoryHandleTypeFlagBits,
    _host: core::marker::PhantomData<&'host mut [u8]>,
}

impl<'host> HostImportBufferCreateInfo<'host> {
    /// Imports a mutable host allocation for the lifetime of the returned
    /// buffer. Vulkan's required host-pointer alignment is validated when the
    /// allocation is created.
    pub fn from_slice(host: &'host mut [u8]) -> Self {
        Self {
            host_ptr: host.as_mut_ptr(),
            size: host.len() as u64,
            handle_type: VkExternalMemoryHandleTypeFlagBits::HOST_ALLOCATION_BIT_EXT,
            _host: core::marker::PhantomData,
        }
    }

    /// Creates an import description from raw host memory.
    ///
    /// # Safety
    /// `host_ptr..host_ptr + size` must remain allocated, writable, and valid
    /// for Vulkan access for the entire `'host` lifetime.
    pub const unsafe fn from_raw_parts(host_ptr: *mut u8, size: u64) -> Self {
        Self {
            host_ptr,
            size,
            handle_type: VkExternalMemoryHandleTypeFlagBits::HOST_ALLOCATION_BIT_EXT,
            _host: core::marker::PhantomData,
        }
    }

    #[must_use]
    pub const fn with_handle_type(
        mut self,
        handle_type: VkExternalMemoryHandleTypeFlagBits,
    ) -> Self {
        self.handle_type = handle_type;
        self
    }
}

pub struct ImportedHostBuffer<'host, 'vk> {
    buffer: Buffer<'vk>,
    memory: DeviceMemory<'vk>,
    host_ptr: *mut u8,
    size: u64,
    _host: core::marker::PhantomData<&'host mut [u8]>,
}

unsafe impl Send for ImportedHostBuffer<'_, '_> {}
unsafe impl Sync for ImportedHostBuffer<'_, '_> {}

#[allow(clippy::elidable_lifetime_names)]
impl<'host, 'vk> ImportedHostBuffer<'host, 'vk> {
    pub(crate) const fn new(
        buffer: Buffer<'vk>,
        memory: DeviceMemory<'vk>,
        host_ptr: *mut u8,
        size: u64,
    ) -> Self {
        Self {
            buffer,
            memory,
            host_ptr,
            size,
            _host: core::marker::PhantomData,
        }
    }

    pub const fn buffer(&self) -> &Buffer<'vk> {
        &self.buffer
    }

    pub const fn memory(&self) -> &DeviceMemory<'vk> {
        &self.memory
    }

    pub const fn host_ptr(&self) -> *mut u8 {
        self.host_ptr
    }

    pub const fn size(&self) -> u64 {
        self.size
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LargeBufferCreateInfo {
    pub preferred_chunk_size: Option<u64>,
}

impl Default for LargeBufferCreateInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl LargeBufferCreateInfo {
    pub const DEFAULT: Self = Self {
        preferred_chunk_size: None,
    };

    pub const fn new() -> Self {
        Self::DEFAULT
    }

    #[must_use]
    pub const fn with_preferred_chunk_size(mut self, preferred_chunk_size: u64) -> Self {
        self.preferred_chunk_size = Some(preferred_chunk_size);
        self
    }
}

pub struct LargeBuffer<'vk> {
    total_size: u64,
    chunk_size: u64,
    segments: Box<[AllocatedBuffer<'vk>]>,
}

pub(crate) struct LargeBufferSegmentsBuilder<'vk> {
    segments: Box<[MaybeUninit<AllocatedBuffer<'vk>>]>,
    initialized: usize,
}

impl<'vk> LargeBufferSegmentsBuilder<'vk> {
    pub(crate) fn new(segment_count: usize) -> Self {
        Self {
            segments: Box::new_uninit_slice(segment_count),
            initialized: 0,
        }
    }

    pub(crate) fn push(&mut self, segment: AllocatedBuffer<'vk>) {
        debug_assert!(self.initialized < self.segments.len());
        self.segments[self.initialized].write(segment);
        self.initialized += 1;
    }

    pub(crate) fn finish(mut self) -> Box<[AllocatedBuffer<'vk>]> {
        debug_assert_eq!(self.initialized, self.segments.len());
        self.initialized = 0;
        let segments = mem::replace(&mut self.segments, Box::new_uninit_slice(0));
        unsafe { segments.assume_init() }
    }
}

impl Drop for LargeBufferSegmentsBuilder<'_> {
    fn drop(&mut self) {
        for segment in &mut self.segments[..self.initialized] {
            unsafe {
                segment.assume_init_drop();
            }
        }
    }
}

impl<'vk> LargeBuffer<'vk> {
    pub(crate) fn new(
        total_size: u64,
        chunk_size: u64,
        segments: Box<[AllocatedBuffer<'vk>]>,
    ) -> Self {
        Self {
            total_size,
            chunk_size,
            segments,
        }
    }

    pub fn total_size(&self) -> u64 {
        self.total_size
    }

    pub fn chunk_size(&self) -> u64 {
        self.chunk_size
    }

    pub fn segment_count(&self) -> usize {
        self.segments.len()
    }

    pub fn segments(&self) -> &[AllocatedBuffer<'vk>] {
        &self.segments
    }

    pub fn segments_mut(&mut self) -> &mut [AllocatedBuffer<'vk>] {
        &mut self.segments
    }

    pub fn segment_for_offset(&self, offset: u64) -> Option<LargeBufferSegment<'_, 'vk>> {
        if offset >= self.total_size {
            return None;
        }
        let index = (offset / self.chunk_size) as usize;
        let global_offset = (index as u64) * self.chunk_size;
        let local_offset = offset - global_offset;
        let segment = self.segments.get(index)?;
        Some(LargeBufferSegment {
            index,
            global_offset,
            local_offset,
            size: segment.allocation().size(),
            segment,
        })
    }

    pub fn write_bytes(&mut self, offset: u64, bytes: &[u8]) -> Result<(), AllocatorError> {
        if offset + bytes.len() as u64 > self.total_size {
            return Err(AllocatorError::OutOfBounds);
        }
        let mut remaining = bytes;
        let mut cursor = offset;
        while !remaining.is_empty() {
            let index = (cursor / self.chunk_size) as usize;
            let global_offset = (index as u64) * self.chunk_size;
            let local_offset = (cursor - global_offset) as usize;
            let segment = self
                .segments
                .get_mut(index)
                .ok_or(AllocatorError::OutOfBounds)?;
            let ptr = segment.allocation().mapped_ptr();
            if ptr.is_null() {
                return Err(AllocatorError::HostVisibleRequired);
            }
            let writable =
                (segment.allocation().size() as usize - local_offset).min(remaining.len());
            unsafe {
                ptr::copy_nonoverlapping(remaining.as_ptr(), ptr.add(local_offset), writable);
            }
            remaining = &remaining[writable..];
            cursor += writable as u64;
        }
        Ok(())
    }
}

pub struct LargeBufferSegment<'a, 'vk> {
    pub index: usize,
    pub global_offset: u64,
    pub local_offset: u64,
    pub size: u64,
    pub segment: &'a AllocatedBuffer<'vk>,
}
