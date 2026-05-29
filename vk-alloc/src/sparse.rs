use crate::allocation::Allocation;
use crate::allocator::Allocator;
use crate::error::AllocatorError;
use crate::group_allocator::{GroupAllocator, GroupBindMode};
use crate::resource::{AllocationCreateInfo, SparseAllocationCreateInfo};
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::vec::Vec;
use parking_lot::RwLock;
use vk::{
    Buffer, Device, Image, VkBindSparseInfo, VkBuffer, VkBufferCreateInfo, VkExtent3D, VkImage,
    VkImageCreateFlagBits, VkImageCreateInfo, VkMemoryRequirements, VkOffset3D,
    VkSparseBufferMemoryBindInfo, VkSparseImageMemoryBind, VkSparseImageMemoryBindInfo,
    VkSparseMemoryBind,
};

#[derive(Debug, Clone)]
pub(crate) struct PageTable<K, V> {
    pages: BTreeMap<K, V>,
}

impl<K, V> PageTable<K, V> {
    pub(crate) const fn new() -> Self {
        Self {
            pages: BTreeMap::new(),
        }
    }
}

impl<K, V> Default for PageTable<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> PageTable<K, V>
where
    K: Ord,
{
    pub(crate) fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.pages.insert(key, value)
    }

    pub(crate) fn remove(&mut self, key: &K) -> Option<V> {
        self.pages.remove(key)
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn get(&self, key: &K) -> Option<&V> {
        self.pages.get(key)
    }

    pub(crate) fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(&K, &V),
    {
        for (key, value) in &self.pages {
            f(key, value);
        }
    }
}

#[derive(Debug, Clone)]
pub struct SparseBufferBindList {
    binds: Box<[VkSparseMemoryBind]>,
}

impl SparseBufferBindList {
    pub fn binds(&self) -> &[VkSparseMemoryBind] {
        &self.binds
    }
}

#[derive(Debug, Clone)]
pub struct SparseImageBindList {
    binds: Box<[VkSparseImageMemoryBind]>,
}

impl SparseImageBindList {
    pub fn binds(&self) -> &[VkSparseImageMemoryBind] {
        &self.binds
    }
}

#[derive(Debug, Clone)]
pub struct PreparedBindSparseInfo {
    buffer_binds: Box<[VkSparseMemoryBind]>,
    buffer: Option<VkBuffer>,
    image_binds: Box<[VkSparseImageMemoryBind]>,
    image: Option<VkImage>,
}

#[derive(Debug, Clone)]
pub struct PreparedBindSparseInfoView<'a> {
    buffer_infos: Box<[VkSparseBufferMemoryBindInfo<'a>]>,
    image_infos: Box<[VkSparseImageMemoryBindInfo<'a>]>,
}

impl<'a> PreparedBindSparseInfoView<'a> {
    pub fn buffer_infos(&self) -> &[VkSparseBufferMemoryBindInfo<'a>] {
        &self.buffer_infos
    }

    pub fn image_infos(&self) -> &[VkSparseImageMemoryBindInfo<'a>] {
        &self.image_infos
    }

    pub fn with_vk_info<R>(&self, f: impl FnOnce(&VkBindSparseInfo<'_>) -> R) -> R {
        let info = VkBindSparseInfo::DEFAULT
            .with_pBufferBinds(&self.buffer_infos)
            .with_pImageBinds(&self.image_infos);
        f(&info)
    }
}

impl PreparedBindSparseInfo {
    pub fn as_vk_info(&self) -> PreparedBindSparseInfoView<'_> {
        let buffer_infos = self
            .buffer
            .map(|buffer| {
                VkSparseBufferMemoryBindInfo::DEFAULT
                    .with_buffer(buffer)
                    .with_pBinds(&self.buffer_binds)
            })
            .into_iter()
            .collect::<Box<[_]>>();
        let image_infos = self
            .image
            .map(|image| {
                VkSparseImageMemoryBindInfo::DEFAULT
                    .with_image(image)
                    .with_pBinds(&self.image_binds)
            })
            .into_iter()
            .collect::<Box<[_]>>();
        PreparedBindSparseInfoView {
            buffer_infos,
            image_infos,
        }
    }

    pub fn buffer_binds(&self) -> &[VkSparseMemoryBind] {
        &self.buffer_binds
    }

    pub fn image_binds(&self) -> &[VkSparseImageMemoryBind] {
        &self.image_binds
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct SparsePageKey(u64);

type SparsePageTable = Arc<RwLock<PageTable<SparsePageKey, Allocation>>>;

struct SparseBase {
    page_size: u64,
    pages: SparsePageTable,
    base_alloc_info: AllocationCreateInfo,
}

fn empty_box<T>() -> Box<[T]> {
    Box::new([])
}

fn sparse_buffer_base<'vk>(
    device: &'vk Device<'vk>,
    buffer_info: &VkBufferCreateInfo,
    sparse_info: SparseAllocationCreateInfo,
    group_allocator: bool,
) -> Result<(Buffer<'vk>, SparseBase), AllocatorError> {
    if group_allocator && sparse_info.group_bind_mode == Some(GroupBindMode::SplitInstanceRegions) {
        return Err(AllocatorError::GroupModeUnsupported);
    }
    if !buffer_info
        .flags
        .intersects(vk::VkBufferCreateFlagBits::SPARSE_BINDING)
    {
        return Err(AllocatorError::SparseBindingUnsupported);
    }
    let buffer = device
        .vkCreateBuffer(buffer_info, vk::null())
        .map_err(AllocatorError::Vulkan)?;
    let mut requirements = VkMemoryRequirements::DEFAULT;
    buffer.vkGetBufferMemoryRequirements(&mut requirements);
    Ok((
        buffer,
        SparseBase {
            page_size: sparse_info
                .page_size
                .unwrap_or(requirements.alignment.max(1)),
            pages: Arc::new(RwLock::new(PageTable::new())),
            base_alloc_info: sparse_info.into_allocation_info(),
        },
    ))
}

fn sparse_image_base<'vk>(
    device: &'vk Device<'vk>,
    image_info: &VkImageCreateInfo,
    sparse_info: SparseAllocationCreateInfo,
    group_allocator: bool,
) -> Result<(Image<'vk>, SparseBase), AllocatorError> {
    if group_allocator
        && sparse_info.group_bind_mode == Some(GroupBindMode::SplitInstanceRegions)
        && sparse_info.split_instance_regions.is_empty()
    {
        return Err(AllocatorError::InvalidSparseRegion);
    }
    if !image_info
        .flags
        .intersects(VkImageCreateFlagBits::SPARSE_BINDING)
    {
        return Err(AllocatorError::SparseBindingUnsupported);
    }
    let image = device
        .vkCreateImage(image_info, vk::null())
        .map_err(AllocatorError::Vulkan)?;
    let page_size = if let Some(page_size) = sparse_info.page_size {
        page_size
    } else if !group_allocator {
        let mut count = 0;
        image.vkGetImageSparseMemoryRequirements(&mut count, core::ptr::null_mut());
        64 * 1024
    } else {
        64 * 1024
    };
    Ok((
        image,
        SparseBase {
            page_size,
            pages: Arc::new(RwLock::new(PageTable::default())),
            base_alloc_info: sparse_info.into_allocation_info(),
        },
    ))
}

pub struct SparseBufferAllocation<'vk> {
    buffer: Buffer<'vk>,
    page_size: u64,
    pages: SparsePageTable,
    base_alloc_info: AllocationCreateInfo,
}

impl<'vk> SparseBufferAllocation<'vk> {
    pub(crate) fn new(
        device: &'vk Device<'vk>,
        _allocator: &Allocator<'vk>,
        buffer_info: &VkBufferCreateInfo,
        sparse_info: SparseAllocationCreateInfo,
    ) -> Result<Self, AllocatorError> {
        let (buffer, base) = sparse_buffer_base(device, buffer_info, sparse_info, false)?;
        Ok(Self {
            buffer,
            page_size: base.page_size,
            pages: base.pages,
            base_alloc_info: base.base_alloc_info,
        })
    }

    pub(crate) fn new_group(
        device: &'vk Device<'vk>,
        _allocator: &GroupAllocator<'vk>,
        buffer_info: &VkBufferCreateInfo,
        sparse_info: SparseAllocationCreateInfo,
    ) -> Result<Self, AllocatorError> {
        let (buffer, base) = sparse_buffer_base(device, buffer_info, sparse_info, true)?;
        Ok(Self {
            buffer,
            page_size: base.page_size,
            pages: base.pages,
            base_alloc_info: base.base_alloc_info,
        })
    }

    pub fn buffer(&self) -> &Buffer<'vk> {
        &self.buffer
    }

    pub fn update_page(
        &self,
        allocator: &Allocator<'vk>,
        page_index: u64,
        resident: bool,
    ) -> Result<(), AllocatorError> {
        update_sparse_page(
            &self.pages,
            self.page_size,
            self.base_alloc_info.clone(),
            page_index,
            resident,
            |requirements, alloc_info| allocator.allocate_page(requirements, alloc_info),
        )
    }

    pub fn update_page_group(
        &self,
        allocator: &GroupAllocator<'vk>,
        page_index: u64,
        resident: bool,
    ) -> Result<(), AllocatorError> {
        update_sparse_page(
            &self.pages,
            self.page_size,
            self.base_alloc_info.clone(),
            page_index,
            resident,
            |requirements, alloc_info| allocator.allocate_page(requirements, alloc_info),
        )
    }

    pub fn build_bind_list(&self) -> SparseBufferBindList {
        let mut binds = Vec::new();
        self.pages.read().for_each(|page, allocation| {
            binds.push(
                VkSparseMemoryBind::DEFAULT
                    .with_resourceOffset(page.0 * self.page_size)
                    .with_size(self.page_size)
                    .with_memory(allocation.memory())
                    .with_memoryOffset(allocation.offset()),
            );
        });
        SparseBufferBindList {
            binds: binds.into_boxed_slice(),
        }
    }

    pub fn prepare_bind_info(&self) -> PreparedBindSparseInfo {
        let buffer_binds = self.build_bind_list().binds;
        PreparedBindSparseInfo {
            buffer_binds,
            buffer: Some(self.buffer.raw()),
            image_binds: empty_box(),
            image: None,
        }
    }
}

pub struct SparseImageAllocation<'vk> {
    image: Image<'vk>,
    page_size: u64,
    pages: SparsePageTable,
    base_alloc_info: AllocationCreateInfo,
}

impl<'vk> SparseImageAllocation<'vk> {
    pub(crate) fn new(
        device: &'vk Device<'vk>,
        _allocator: &Allocator<'vk>,
        image_info: &VkImageCreateInfo,
        sparse_info: SparseAllocationCreateInfo,
    ) -> Result<Self, AllocatorError> {
        let (image, base) = sparse_image_base(device, image_info, sparse_info, false)?;
        Ok(Self {
            image,
            page_size: base.page_size,
            pages: base.pages,
            base_alloc_info: base.base_alloc_info,
        })
    }

    pub(crate) fn new_group(
        device: &'vk Device<'vk>,
        _allocator: &GroupAllocator<'vk>,
        image_info: &VkImageCreateInfo,
        sparse_info: SparseAllocationCreateInfo,
    ) -> Result<Self, AllocatorError> {
        let (image, base) = sparse_image_base(device, image_info, sparse_info, true)?;
        Ok(Self {
            image,
            page_size: base.page_size,
            pages: base.pages,
            base_alloc_info: base.base_alloc_info,
        })
    }

    pub fn image(&self) -> &Image<'vk> {
        &self.image
    }

    pub fn update_page(
        &self,
        allocator: &Allocator<'vk>,
        page_index: u64,
        resident: bool,
    ) -> Result<(), AllocatorError> {
        update_sparse_page(
            &self.pages,
            self.page_size,
            self.base_alloc_info.clone(),
            page_index,
            resident,
            |requirements, alloc_info| allocator.allocate_page(requirements, alloc_info),
        )
    }

    pub fn update_page_group(
        &self,
        allocator: &GroupAllocator<'vk>,
        page_index: u64,
        resident: bool,
    ) -> Result<(), AllocatorError> {
        update_sparse_page(
            &self.pages,
            self.page_size,
            self.base_alloc_info.clone(),
            page_index,
            resident,
            |requirements, alloc_info| allocator.allocate_page(requirements, alloc_info),
        )
    }

    pub fn build_bind_list(&self) -> SparseImageBindList {
        let mut binds = Vec::new();
        self.pages.read().for_each(|page, allocation| {
            binds.push(
                VkSparseImageMemoryBind::DEFAULT
                    .with_offset(VkOffset3D::DEFAULT.with_x((page.0 * self.page_size) as i32))
                    .with_extent(
                        VkExtent3D::DEFAULT
                            .with_width(self.page_size as u32)
                            .with_height(1)
                            .with_depth(1),
                    )
                    .with_memory(allocation.memory())
                    .with_memoryOffset(allocation.offset()),
            );
        });
        SparseImageBindList {
            binds: binds.into_boxed_slice(),
        }
    }

    pub fn prepare_bind_info(&self) -> PreparedBindSparseInfo {
        let image_binds = self.build_bind_list().binds;
        PreparedBindSparseInfo {
            buffer_binds: empty_box(),
            buffer: None,
            image_binds,
            image: Some(self.image.raw()),
        }
    }
}

fn update_sparse_page(
    pages: &SparsePageTable,
    page_size: u64,
    base_alloc_info: AllocationCreateInfo,
    page_index: u64,
    resident: bool,
    allocate: impl Fn(VkMemoryRequirements, AllocationCreateInfo) -> Result<Allocation, AllocatorError>, // TODO(czarlinski): this is awkward, inline.
) -> Result<(), AllocatorError> {
    let key = SparsePageKey(page_index);
    if resident {
        let requirements = VkMemoryRequirements::DEFAULT
            .with_size(page_size)
            .with_alignment(page_size)
            .with_memoryTypeBits(u32::MAX);
        let allocation = allocate(requirements, base_alloc_info)?;
        pages.write().insert(key, allocation);
    } else {
        pages.write().remove(&key);
    }
    Ok(())
}
