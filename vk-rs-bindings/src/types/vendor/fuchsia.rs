#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_1",
  feature = "VK_KHR_sampler_ycbcr_conversion"
))]
use crate::enums::VkChromaLocation;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_1",
  feature = "VK_KHR_external_memory_capabilities",
  feature = "VK_EXT_external_memory_dma_buf",
  feature = "VK_ANDROID_external_memory_android_hardware_buffer",
  feature = "VK_EXT_external_memory_host",
  feature = "VK_FUCHSIA_external_memory",
  feature = "VK_NV_external_memory_rdma",
  feature = "VK_OHOS_external_memory",
  feature = "VK_QNX_external_memory_screen_buffer",
  feature = "VK_EXT_external_memory_metal"
))]
use crate::enums::VkExternalMemoryHandleTypeFlagBits;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_1",
  feature = "VK_KHR_external_semaphore_capabilities"
))]
use crate::enums::VkExternalSemaphoreHandleTypeFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkFormatFeatureFlagBits;
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
use crate::enums::VkImageConstraintsInfoFlagBitsFUCHSIA;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_1",
  feature = "VK_KHR_sampler_ycbcr_conversion"
))]
use crate::enums::VkSamplerYcbcrModelConversion;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_1",
  feature = "VK_KHR_sampler_ycbcr_conversion"
))]
use crate::enums::VkSamplerYcbcrRange;
#[cfg(any(feature = "VK_BASE_VERSION_1_1", feature = "VK_KHR_external_semaphore"))]
use crate::enums::VkSemaphoreImportFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBufferCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkComponentMapping;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceMemory;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkFormatFeatureFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkMemoryAllocateInfo;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkSemaphore;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkSemaphoreImportFlags;
use core::ffi::c_void;
/// [VkImageFormatConstraintsFlagsFUCHSIA](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageFormatConstraintsFlagsFUCHSIA.html)
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
pub type VkImageFormatConstraintsFlagsFUCHSIA = VkFlags;
/// [VkImageConstraintsInfoFlagsFUCHSIA](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageConstraintsInfoFlagsFUCHSIA.html)
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
pub type VkImageConstraintsInfoFlagsFUCHSIA = VkImageConstraintsInfoFlagBitsFUCHSIA;
/// [VkBufferCollectionFUCHSIA](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCollectionFUCHSIA.html)
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkBufferCollectionFUCHSIA(pub *mut c_void);
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
impl VkBufferCollectionFUCHSIA {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
impl Default for VkBufferCollectionFUCHSIA {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
unsafe impl Send for VkBufferCollectionFUCHSIA {}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
unsafe impl Sync for VkBufferCollectionFUCHSIA {}
/// [VkImportMemoryBufferCollectionFUCHSIA](https://docs.vulkan.org/refpages/latest/refpages/source/VkImportMemoryBufferCollectionFUCHSIA.html)
///
/// **Extends:** VkMemoryAllocateInfo.
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImportMemoryBufferCollectionFUCHSIA<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub collection: VkBufferCollectionFUCHSIA,
  pub index: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
unsafe impl<'a> Send for VkImportMemoryBufferCollectionFUCHSIA<'a> {}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
unsafe impl<'a> Sync for VkImportMemoryBufferCollectionFUCHSIA<'a> {}
#[cfg(all(
  feature = "VK_FUCHSIA_buffer_collection",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkMemoryAllocateInfo<'root>>
  for VkImportMemoryBufferCollectionFUCHSIA<'child>
{
}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
impl<'a> VkImportMemoryBufferCollectionFUCHSIA<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA,
    pNext: core::ptr::null(),
    collection: VkBufferCollectionFUCHSIA::DEFAULT,
    index: 0,
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_collection(mut self, val: VkBufferCollectionFUCHSIA) -> Self {
    self.collection = val;
    self
  }
  #[inline]
  pub const fn with_index(mut self, val: u32) -> Self {
    self.index = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkMemoryAllocateInfo<
    'root,
    T: VkPNextExtends<VkMemoryAllocateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkBufferCollectionImageCreateInfoFUCHSIA](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCollectionImageCreateInfoFUCHSIA.html)
///
/// **Extends:** VkImageCreateInfo.
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBufferCollectionImageCreateInfoFUCHSIA<'a> {
  /// Values: VK_STRUCTURE_TYPE_BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub collection: VkBufferCollectionFUCHSIA,
  pub index: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
unsafe impl<'a> Send for VkBufferCollectionImageCreateInfoFUCHSIA<'a> {}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
unsafe impl<'a> Sync for VkBufferCollectionImageCreateInfoFUCHSIA<'a> {}
#[cfg(all(
  feature = "VK_FUCHSIA_buffer_collection",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkImageCreateInfo<'root>>
  for VkBufferCollectionImageCreateInfoFUCHSIA<'child>
{
}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
impl<'a> VkBufferCollectionImageCreateInfoFUCHSIA<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA,
    pNext: core::ptr::null(),
    collection: VkBufferCollectionFUCHSIA::DEFAULT,
    index: 0,
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_collection(mut self, val: VkBufferCollectionFUCHSIA) -> Self {
    self.collection = val;
    self
  }
  #[inline]
  pub const fn with_index(mut self, val: u32) -> Self {
    self.index = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkImageCreateInfo<
    'root,
    T: VkPNextExtends<VkImageCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkBufferCollectionBufferCreateInfoFUCHSIA](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCollectionBufferCreateInfoFUCHSIA.html)
///
/// **Extends:** VkBufferCreateInfo.
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBufferCollectionBufferCreateInfoFUCHSIA<'a> {
  /// Values: VK_STRUCTURE_TYPE_BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub collection: VkBufferCollectionFUCHSIA,
  pub index: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
unsafe impl<'a> Send for VkBufferCollectionBufferCreateInfoFUCHSIA<'a> {}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
unsafe impl<'a> Sync for VkBufferCollectionBufferCreateInfoFUCHSIA<'a> {}
#[cfg(all(
  feature = "VK_FUCHSIA_buffer_collection",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkBufferCreateInfo<'root>>
  for VkBufferCollectionBufferCreateInfoFUCHSIA<'child>
{
}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
impl<'a> VkBufferCollectionBufferCreateInfoFUCHSIA<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA,
    pNext: core::ptr::null(),
    collection: VkBufferCollectionFUCHSIA::DEFAULT,
    index: 0,
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_collection(mut self, val: VkBufferCollectionFUCHSIA) -> Self {
    self.collection = val;
    self
  }
  #[inline]
  pub const fn with_index(mut self, val: u32) -> Self {
    self.index = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkBufferCreateInfo<
    'root,
    T: VkPNextExtends<VkBufferCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkBufferCollectionCreateInfoFUCHSIA](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCollectionCreateInfoFUCHSIA.html)
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBufferCollectionCreateInfoFUCHSIA<'a> {
  /// Values: VK_STRUCTURE_TYPE_BUFFER_COLLECTION_CREATE_INFO_FUCHSIA
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub collectionToken: zx_handle_t,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
unsafe impl<'a> Send for VkBufferCollectionCreateInfoFUCHSIA<'a> {}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
unsafe impl<'a> Sync for VkBufferCollectionCreateInfoFUCHSIA<'a> {}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
impl<'a> VkBufferCollectionCreateInfoFUCHSIA<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::BUFFER_COLLECTION_CREATE_INFO_FUCHSIA,
    pNext: core::ptr::null(),
    collectionToken: zx_handle_t::NULL,
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_collectionToken(mut self, val: zx_handle_t) -> Self {
    self.collectionToken = val;
    self
  }
  #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkBufferCollectionCreateInfoFUCHSIA<
    'root,
    T: VkPNextExtends<VkBufferCollectionCreateInfoFUCHSIA<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkBufferCollectionPropertiesFUCHSIA](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCollectionPropertiesFUCHSIA.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBufferCollectionPropertiesFUCHSIA<'a> {
  /// Values: VK_STRUCTURE_TYPE_BUFFER_COLLECTION_PROPERTIES_FUCHSIA
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub memoryTypeBits: u32,
  pub bufferCount: u32,
  pub createInfoIndex: u32,
  pub sysmemPixelFormat: u64,
  pub formatFeatures: VkFormatFeatureFlags,
  pub sysmemColorSpaceIndex: VkSysmemColorSpaceFUCHSIA<'a>,
  pub samplerYcbcrConversionComponents: VkComponentMapping,
  pub suggestedYcbcrModel: VkSamplerYcbcrModelConversion,
  pub suggestedYcbcrRange: VkSamplerYcbcrRange,
  pub suggestedXChromaOffset: VkChromaLocation,
  pub suggestedYChromaOffset: VkChromaLocation,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
unsafe impl<'a> Send for VkBufferCollectionPropertiesFUCHSIA<'a> {}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
unsafe impl<'a> Sync for VkBufferCollectionPropertiesFUCHSIA<'a> {}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
impl<'a> VkBufferCollectionPropertiesFUCHSIA<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::BUFFER_COLLECTION_PROPERTIES_FUCHSIA,
    pNext: core::ptr::null_mut(),
    memoryTypeBits: 0,
    bufferCount: 0,
    createInfoIndex: 0,
    sysmemPixelFormat: 0,
    formatFeatures: VkFormatFeatureFlagBits(0),
    sysmemColorSpaceIndex: VkSysmemColorSpaceFUCHSIA::DEFAULT,
    samplerYcbcrConversionComponents: VkComponentMapping::DEFAULT,
    suggestedYcbcrModel: VkSamplerYcbcrModelConversion(0),
    suggestedYcbcrRange: VkSamplerYcbcrRange(0),
    suggestedXChromaOffset: VkChromaLocation(0),
    suggestedYChromaOffset: VkChromaLocation(0),
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *mut c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_memoryTypeBits(mut self, val: u32) -> Self {
    self.memoryTypeBits = val;
    self
  }
  #[inline]
  pub const fn with_bufferCount(mut self, val: u32) -> Self {
    self.bufferCount = val;
    self
  }
  #[inline]
  pub const fn with_createInfoIndex(mut self, val: u32) -> Self {
    self.createInfoIndex = val;
    self
  }
  #[inline]
  pub const fn with_sysmemPixelFormat(mut self, val: u64) -> Self {
    self.sysmemPixelFormat = val;
    self
  }
  #[inline]
  pub const fn with_formatFeatures(mut self, val: VkFormatFeatureFlags) -> Self {
    self.formatFeatures = val;
    self
  }
  #[inline]
  pub const fn with_sysmemColorSpaceIndex(mut self, val: VkSysmemColorSpaceFUCHSIA<'a>) -> Self {
    self.sysmemColorSpaceIndex = val;
    self
  }
  #[inline]
  pub const fn with_samplerYcbcrConversionComponents(mut self, val: VkComponentMapping) -> Self {
    self.samplerYcbcrConversionComponents = val;
    self
  }
  #[inline]
  pub const fn with_suggestedYcbcrModel(mut self, val: VkSamplerYcbcrModelConversion) -> Self {
    self.suggestedYcbcrModel = val;
    self
  }
  #[inline]
  pub const fn with_suggestedYcbcrRange(mut self, val: VkSamplerYcbcrRange) -> Self {
    self.suggestedYcbcrRange = val;
    self
  }
  #[inline]
  pub const fn with_suggestedXChromaOffset(mut self, val: VkChromaLocation) -> Self {
    self.suggestedXChromaOffset = val;
    self
  }
  #[inline]
  pub const fn with_suggestedYChromaOffset(mut self, val: VkChromaLocation) -> Self {
    self.suggestedYChromaOffset = val;
    self
  }
  #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkBufferCollectionPropertiesFUCHSIA<
    'root,
    T: VkPNextExtends<VkBufferCollectionPropertiesFUCHSIA<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkBufferConstraintsInfoFUCHSIA](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferConstraintsInfoFUCHSIA.html)
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBufferConstraintsInfoFUCHSIA<'a> {
  /// Values: VK_STRUCTURE_TYPE_BUFFER_CONSTRAINTS_INFO_FUCHSIA
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub createInfo: VkBufferCreateInfo<'a>,
  /// Optional: true
  pub requiredFormatFeatures: VkFormatFeatureFlags,
  pub bufferCollectionConstraints: VkBufferCollectionConstraintsInfoFUCHSIA<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
unsafe impl<'a> Send for VkBufferConstraintsInfoFUCHSIA<'a> {}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
unsafe impl<'a> Sync for VkBufferConstraintsInfoFUCHSIA<'a> {}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
impl<'a> VkBufferConstraintsInfoFUCHSIA<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::BUFFER_CONSTRAINTS_INFO_FUCHSIA,
    pNext: core::ptr::null(),
    createInfo: VkBufferCreateInfo::DEFAULT,
    requiredFormatFeatures: VkFormatFeatureFlagBits(0),
    bufferCollectionConstraints: VkBufferCollectionConstraintsInfoFUCHSIA::DEFAULT,
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_createInfo(mut self, val: VkBufferCreateInfo<'a>) -> Self {
    self.createInfo = val;
    self
  }
  #[inline]
  pub const fn with_requiredFormatFeatures(mut self, val: VkFormatFeatureFlags) -> Self {
    self.requiredFormatFeatures = val;
    self
  }
  #[inline]
  pub const fn with_bufferCollectionConstraints(
    mut self,
    val: VkBufferCollectionConstraintsInfoFUCHSIA<'a>,
  ) -> Self {
    self.bufferCollectionConstraints = val;
    self
  }
  #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkBufferConstraintsInfoFUCHSIA<
    'root,
    T: VkPNextExtends<VkBufferConstraintsInfoFUCHSIA<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkSysmemColorSpaceFUCHSIA](https://docs.vulkan.org/refpages/latest/refpages/source/VkSysmemColorSpaceFUCHSIA.html)
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSysmemColorSpaceFUCHSIA<'a> {
  /// Values: VK_STRUCTURE_TYPE_SYSMEM_COLOR_SPACE_FUCHSIA
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub colorSpace: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
unsafe impl<'a> Send for VkSysmemColorSpaceFUCHSIA<'a> {}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
unsafe impl<'a> Sync for VkSysmemColorSpaceFUCHSIA<'a> {}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
impl<'a> VkSysmemColorSpaceFUCHSIA<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SYSMEM_COLOR_SPACE_FUCHSIA,
    pNext: core::ptr::null(),
    colorSpace: 0,
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_colorSpace(mut self, val: u32) -> Self {
    self.colorSpace = val;
    self
  }
  #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSysmemColorSpaceFUCHSIA<
    'root,
    T: VkPNextExtends<VkSysmemColorSpaceFUCHSIA<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkImageFormatConstraintsInfoFUCHSIA](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageFormatConstraintsInfoFUCHSIA.html)
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImageFormatConstraintsInfoFUCHSIA<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub imageCreateInfo: VkImageCreateInfo<'a>,
  pub requiredFormatFeatures: VkFormatFeatureFlags,
  /// Optional: true
  pub flags: VkImageFormatConstraintsFlagsFUCHSIA,
  /// Optional: true
  pub sysmemPixelFormat: u64,
  pub colorSpaceCount: u32,
  /// Length: colorSpaceCount
  pub pColorSpaces: *const VkSysmemColorSpaceFUCHSIA<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
unsafe impl<'a> Send for VkImageFormatConstraintsInfoFUCHSIA<'a> {}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
unsafe impl<'a> Sync for VkImageFormatConstraintsInfoFUCHSIA<'a> {}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
impl<'a> VkImageFormatConstraintsInfoFUCHSIA<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA,
    pNext: core::ptr::null(),
    imageCreateInfo: VkImageCreateInfo::DEFAULT,
    requiredFormatFeatures: VkFormatFeatureFlagBits(0),
    flags: 0,
    sysmemPixelFormat: 0,
    colorSpaceCount: 0,
    pColorSpaces: core::ptr::null(),
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_imageCreateInfo(mut self, val: VkImageCreateInfo<'a>) -> Self {
    self.imageCreateInfo = val;
    self
  }
  #[inline]
  pub const fn with_requiredFormatFeatures(mut self, val: VkFormatFeatureFlags) -> Self {
    self.requiredFormatFeatures = val;
    self
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkImageFormatConstraintsFlagsFUCHSIA) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_sysmemPixelFormat(mut self, val: u64) -> Self {
    self.sysmemPixelFormat = val;
    self
  }
  #[inline]
  pub const fn with_colorSpaceCount(mut self, val: u32) -> Self {
    self.colorSpaceCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pColorSpaces(mut self, val: &'a [VkSysmemColorSpaceFUCHSIA<'a>]) -> Self {
    self.colorSpaceCount = val.len() as u32;
    self.pColorSpaces = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkImageFormatConstraintsInfoFUCHSIA<
    'root,
    T: VkPNextExtends<VkImageFormatConstraintsInfoFUCHSIA<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkImageConstraintsInfoFUCHSIA](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageConstraintsInfoFUCHSIA.html)
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImageConstraintsInfoFUCHSIA<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMAGE_CONSTRAINTS_INFO_FUCHSIA
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub formatConstraintsCount: u32,
  /// Length: formatConstraintsCount
  pub pFormatConstraints: *const VkImageFormatConstraintsInfoFUCHSIA<'a>,
  pub bufferCollectionConstraints: VkBufferCollectionConstraintsInfoFUCHSIA<'a>,
  /// Optional: true
  pub flags: VkImageConstraintsInfoFlagsFUCHSIA,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
unsafe impl<'a> Send for VkImageConstraintsInfoFUCHSIA<'a> {}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
unsafe impl<'a> Sync for VkImageConstraintsInfoFUCHSIA<'a> {}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
impl<'a> VkImageConstraintsInfoFUCHSIA<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::IMAGE_CONSTRAINTS_INFO_FUCHSIA,
    pNext: core::ptr::null(),
    formatConstraintsCount: 0,
    pFormatConstraints: core::ptr::null(),
    bufferCollectionConstraints: VkBufferCollectionConstraintsInfoFUCHSIA::DEFAULT,
    flags: VkImageConstraintsInfoFlagBitsFUCHSIA(0),
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_formatConstraintsCount(mut self, val: u32) -> Self {
    self.formatConstraintsCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pFormatConstraints(
    mut self,
    val: &'a [VkImageFormatConstraintsInfoFUCHSIA<'a>],
  ) -> Self {
    self.formatConstraintsCount = val.len() as u32;
    self.pFormatConstraints = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_bufferCollectionConstraints(
    mut self,
    val: VkBufferCollectionConstraintsInfoFUCHSIA<'a>,
  ) -> Self {
    self.bufferCollectionConstraints = val;
    self
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkImageConstraintsInfoFlagsFUCHSIA) -> Self {
    self.flags = val;
    self
  }
  #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkImageConstraintsInfoFUCHSIA<
    'root,
    T: VkPNextExtends<VkImageConstraintsInfoFUCHSIA<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkBufferCollectionConstraintsInfoFUCHSIA](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCollectionConstraintsInfoFUCHSIA.html)
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBufferCollectionConstraintsInfoFUCHSIA<'a> {
  /// Values: VK_STRUCTURE_TYPE_BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub minBufferCount: u32,
  pub maxBufferCount: u32,
  pub minBufferCountForCamping: u32,
  pub minBufferCountForDedicatedSlack: u32,
  pub minBufferCountForSharedSlack: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
unsafe impl<'a> Send for VkBufferCollectionConstraintsInfoFUCHSIA<'a> {}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
unsafe impl<'a> Sync for VkBufferCollectionConstraintsInfoFUCHSIA<'a> {}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
impl<'a> VkBufferCollectionConstraintsInfoFUCHSIA<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA,
    pNext: core::ptr::null(),
    minBufferCount: 0,
    maxBufferCount: 0,
    minBufferCountForCamping: 0,
    minBufferCountForDedicatedSlack: 0,
    minBufferCountForSharedSlack: 0,
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_minBufferCount(mut self, val: u32) -> Self {
    self.minBufferCount = val;
    self
  }
  #[inline]
  pub const fn with_maxBufferCount(mut self, val: u32) -> Self {
    self.maxBufferCount = val;
    self
  }
  #[inline]
  pub const fn with_minBufferCountForCamping(mut self, val: u32) -> Self {
    self.minBufferCountForCamping = val;
    self
  }
  #[inline]
  pub const fn with_minBufferCountForDedicatedSlack(mut self, val: u32) -> Self {
    self.minBufferCountForDedicatedSlack = val;
    self
  }
  #[inline]
  pub const fn with_minBufferCountForSharedSlack(mut self, val: u32) -> Self {
    self.minBufferCountForSharedSlack = val;
    self
  }
  #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkBufferCollectionConstraintsInfoFUCHSIA<
    'root,
    T: VkPNextExtends<VkBufferCollectionConstraintsInfoFUCHSIA<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkImportMemoryZirconHandleInfoFUCHSIA](https://docs.vulkan.org/refpages/latest/refpages/source/VkImportMemoryZirconHandleInfoFUCHSIA.html)
///
/// **Extends:** VkMemoryAllocateInfo.
#[cfg(feature = "VK_FUCHSIA_external_memory")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImportMemoryZirconHandleInfoFUCHSIA<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub handleType: VkExternalMemoryHandleTypeFlagBits,
  /// Optional: true
  pub handle: zx_handle_t,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_FUCHSIA_external_memory")]
unsafe impl<'a> Send for VkImportMemoryZirconHandleInfoFUCHSIA<'a> {}
#[cfg(feature = "VK_FUCHSIA_external_memory")]
unsafe impl<'a> Sync for VkImportMemoryZirconHandleInfoFUCHSIA<'a> {}
#[cfg(all(
  feature = "VK_FUCHSIA_external_memory",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkMemoryAllocateInfo<'root>>
  for VkImportMemoryZirconHandleInfoFUCHSIA<'child>
{
}
#[cfg(feature = "VK_FUCHSIA_external_memory")]
impl<'a> VkImportMemoryZirconHandleInfoFUCHSIA<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA,
    pNext: core::ptr::null(),
    handleType: VkExternalMemoryHandleTypeFlagBits(0),
    handle: zx_handle_t::NULL,
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_handleType(mut self, val: VkExternalMemoryHandleTypeFlagBits) -> Self {
    self.handleType = val;
    self
  }
  #[inline]
  pub const fn with_handle(mut self, val: zx_handle_t) -> Self {
    self.handle = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkMemoryAllocateInfo<
    'root,
    T: VkPNextExtends<VkMemoryAllocateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkMemoryZirconHandlePropertiesFUCHSIA](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryZirconHandlePropertiesFUCHSIA.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_FUCHSIA_external_memory")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkMemoryZirconHandlePropertiesFUCHSIA<'a> {
  /// Values: VK_STRUCTURE_TYPE_MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub memoryTypeBits: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_FUCHSIA_external_memory")]
unsafe impl<'a> Send for VkMemoryZirconHandlePropertiesFUCHSIA<'a> {}
#[cfg(feature = "VK_FUCHSIA_external_memory")]
unsafe impl<'a> Sync for VkMemoryZirconHandlePropertiesFUCHSIA<'a> {}
#[cfg(feature = "VK_FUCHSIA_external_memory")]
impl<'a> VkMemoryZirconHandlePropertiesFUCHSIA<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA,
    pNext: core::ptr::null_mut(),
    memoryTypeBits: 0,
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *mut c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_memoryTypeBits(mut self, val: u32) -> Self {
    self.memoryTypeBits = val;
    self
  }
  #[cfg(feature = "VK_FUCHSIA_external_memory")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkMemoryZirconHandlePropertiesFUCHSIA<
    'root,
    T: VkPNextExtends<VkMemoryZirconHandlePropertiesFUCHSIA<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkMemoryGetZirconHandleInfoFUCHSIA](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryGetZirconHandleInfoFUCHSIA.html)
#[cfg(feature = "VK_FUCHSIA_external_memory")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkMemoryGetZirconHandleInfoFUCHSIA<'a> {
  /// Values: VK_STRUCTURE_TYPE_MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub memory: VkDeviceMemory,
  pub handleType: VkExternalMemoryHandleTypeFlagBits,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_FUCHSIA_external_memory")]
unsafe impl<'a> Send for VkMemoryGetZirconHandleInfoFUCHSIA<'a> {}
#[cfg(feature = "VK_FUCHSIA_external_memory")]
unsafe impl<'a> Sync for VkMemoryGetZirconHandleInfoFUCHSIA<'a> {}
#[cfg(feature = "VK_FUCHSIA_external_memory")]
impl<'a> VkMemoryGetZirconHandleInfoFUCHSIA<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA,
    pNext: core::ptr::null(),
    memory: VkDeviceMemory::DEFAULT,
    handleType: VkExternalMemoryHandleTypeFlagBits(0),
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_memory(mut self, val: VkDeviceMemory) -> Self {
    self.memory = val;
    self
  }
  #[inline]
  pub const fn with_handleType(mut self, val: VkExternalMemoryHandleTypeFlagBits) -> Self {
    self.handleType = val;
    self
  }
  #[cfg(feature = "VK_FUCHSIA_external_memory")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkMemoryGetZirconHandleInfoFUCHSIA<
    'root,
    T: VkPNextExtends<VkMemoryGetZirconHandleInfoFUCHSIA<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [zx_handle_t](https://docs.vulkan.org/refpages/latest/refpages/source/zx_handle_t.html)
/// Opaque platform handle - always used as a raw pointer.
#[cfg(any(
  feature = "VK_FUCHSIA_imagepipe_surface",
  feature = "VK_FUCHSIA_external_memory",
  feature = "VK_FUCHSIA_external_semaphore"
))]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct zx_handle_t(pub *mut c_void);
#[cfg(any(
  feature = "VK_FUCHSIA_imagepipe_surface",
  feature = "VK_FUCHSIA_external_memory",
  feature = "VK_FUCHSIA_external_semaphore"
))]
impl zx_handle_t {
  pub const NULL: Self = Self(core::ptr::null_mut());
}
#[cfg(any(
  feature = "VK_FUCHSIA_imagepipe_surface",
  feature = "VK_FUCHSIA_external_memory",
  feature = "VK_FUCHSIA_external_semaphore"
))]
unsafe impl Send for zx_handle_t {}
#[cfg(any(
  feature = "VK_FUCHSIA_imagepipe_surface",
  feature = "VK_FUCHSIA_external_memory",
  feature = "VK_FUCHSIA_external_semaphore"
))]
unsafe impl Sync for zx_handle_t {}
/// [VkImportSemaphoreZirconHandleInfoFUCHSIA](https://docs.vulkan.org/refpages/latest/refpages/source/VkImportSemaphoreZirconHandleInfoFUCHSIA.html)
#[cfg(feature = "VK_FUCHSIA_external_semaphore")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImportSemaphoreZirconHandleInfoFUCHSIA<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub semaphore: VkSemaphore,
  /// Optional: true
  pub flags: VkSemaphoreImportFlags,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
  pub zirconHandle: zx_handle_t,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_FUCHSIA_external_semaphore")]
unsafe impl<'a> Send for VkImportSemaphoreZirconHandleInfoFUCHSIA<'a> {}
#[cfg(feature = "VK_FUCHSIA_external_semaphore")]
unsafe impl<'a> Sync for VkImportSemaphoreZirconHandleInfoFUCHSIA<'a> {}
#[cfg(feature = "VK_FUCHSIA_external_semaphore")]
impl<'a> VkImportSemaphoreZirconHandleInfoFUCHSIA<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA,
    pNext: core::ptr::null(),
    semaphore: VkSemaphore::DEFAULT,
    flags: VkSemaphoreImportFlagBits(0),
    handleType: VkExternalSemaphoreHandleTypeFlagBits(0),
    zirconHandle: zx_handle_t::NULL,
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_semaphore(mut self, val: VkSemaphore) -> Self {
    self.semaphore = val;
    self
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkSemaphoreImportFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_handleType(mut self, val: VkExternalSemaphoreHandleTypeFlagBits) -> Self {
    self.handleType = val;
    self
  }
  #[inline]
  pub const fn with_zirconHandle(mut self, val: zx_handle_t) -> Self {
    self.zirconHandle = val;
    self
  }
  #[cfg(feature = "VK_FUCHSIA_external_semaphore")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkImportSemaphoreZirconHandleInfoFUCHSIA<
    'root,
    T: VkPNextExtends<VkImportSemaphoreZirconHandleInfoFUCHSIA<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkSemaphoreGetZirconHandleInfoFUCHSIA](https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreGetZirconHandleInfoFUCHSIA.html)
#[cfg(feature = "VK_FUCHSIA_external_semaphore")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSemaphoreGetZirconHandleInfoFUCHSIA<'a> {
  /// Values: VK_STRUCTURE_TYPE_SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub semaphore: VkSemaphore,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_FUCHSIA_external_semaphore")]
unsafe impl<'a> Send for VkSemaphoreGetZirconHandleInfoFUCHSIA<'a> {}
#[cfg(feature = "VK_FUCHSIA_external_semaphore")]
unsafe impl<'a> Sync for VkSemaphoreGetZirconHandleInfoFUCHSIA<'a> {}
#[cfg(feature = "VK_FUCHSIA_external_semaphore")]
impl<'a> VkSemaphoreGetZirconHandleInfoFUCHSIA<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA,
    pNext: core::ptr::null(),
    semaphore: VkSemaphore::DEFAULT,
    handleType: VkExternalSemaphoreHandleTypeFlagBits(0),
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_semaphore(mut self, val: VkSemaphore) -> Self {
    self.semaphore = val;
    self
  }
  #[inline]
  pub const fn with_handleType(mut self, val: VkExternalSemaphoreHandleTypeFlagBits) -> Self {
    self.handleType = val;
    self
  }
  #[cfg(feature = "VK_FUCHSIA_external_semaphore")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSemaphoreGetZirconHandleInfoFUCHSIA<
    'root,
    T: VkPNextExtends<VkSemaphoreGetZirconHandleInfoFUCHSIA<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkImagePipeSurfaceCreateFlagsFUCHSIA](https://docs.vulkan.org/refpages/latest/refpages/source/VkImagePipeSurfaceCreateFlagsFUCHSIA.html)
#[cfg(feature = "VK_FUCHSIA_imagepipe_surface")]
pub type VkImagePipeSurfaceCreateFlagsFUCHSIA = VkFlags;
/// [VkImagePipeSurfaceCreateInfoFUCHSIA](https://docs.vulkan.org/refpages/latest/refpages/source/VkImagePipeSurfaceCreateInfoFUCHSIA.html)
#[cfg(feature = "VK_FUCHSIA_imagepipe_surface")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImagePipeSurfaceCreateInfoFUCHSIA<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkImagePipeSurfaceCreateFlagsFUCHSIA,
  pub imagePipeHandle: zx_handle_t,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_FUCHSIA_imagepipe_surface")]
unsafe impl<'a> Send for VkImagePipeSurfaceCreateInfoFUCHSIA<'a> {}
#[cfg(feature = "VK_FUCHSIA_imagepipe_surface")]
unsafe impl<'a> Sync for VkImagePipeSurfaceCreateInfoFUCHSIA<'a> {}
#[cfg(feature = "VK_FUCHSIA_imagepipe_surface")]
impl<'a> VkImagePipeSurfaceCreateInfoFUCHSIA<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA,
    pNext: core::ptr::null(),
    flags: 0,
    imagePipeHandle: zx_handle_t::NULL,
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkImagePipeSurfaceCreateFlagsFUCHSIA) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_imagePipeHandle(mut self, val: zx_handle_t) -> Self {
    self.imagePipeHandle = val;
    self
  }
  #[cfg(feature = "VK_FUCHSIA_imagepipe_surface")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkImagePipeSurfaceCreateInfoFUCHSIA<
    'root,
    T: VkPNextExtends<VkImagePipeSurfaceCreateInfoFUCHSIA<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
