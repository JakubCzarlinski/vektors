#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_1",
  feature = "VK_KHR_sampler_ycbcr_conversion"
))]
use crate::enums::VkChromaLocation;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkFormat;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkFormatFeatureFlagBits;
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
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
use crate::types::VkAttachmentDescription2;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkCommandBufferInheritanceInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkComponentMapping;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceMemory;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceSize;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkFormatFeatureFlags;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkGraphicsPipelineCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkImageFormatProperties2;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkMemoryAllocateInfo;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
use crate::types::VkSamplerYcbcrConversionCreateInfo;
use core::ffi::c_void;
/// [OH_NativeBuffer](https://docs.vulkan.org/refpages/latest/refpages/source/OH_NativeBuffer.html)
/// Opaque platform handle - always used as a raw pointer.
#[cfg(feature = "VK_OHOS_external_memory")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OH_NativeBuffer(pub *mut c_void);
#[cfg(feature = "VK_OHOS_external_memory")]
impl OH_NativeBuffer {
  pub const NULL: Self = Self(core::ptr::null_mut());
}
#[cfg(feature = "VK_OHOS_external_memory")]
unsafe impl Send for OH_NativeBuffer {}
#[cfg(feature = "VK_OHOS_external_memory")]
unsafe impl Sync for OH_NativeBuffer {}
/// [VkNativeBufferUsageOHOS](https://docs.vulkan.org/refpages/latest/refpages/source/VkNativeBufferUsageOHOS.html)
///
/// *Note: This is a **returned only** struct.*
///
/// **Extends:** VkImageFormatProperties2.
#[cfg(feature = "VK_OHOS_external_memory")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkNativeBufferUsageOHOS<'a> {
  /// Values: VK_STRUCTURE_TYPE_NATIVE_BUFFER_USAGE_OHOS
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub OHOSNativeBufferUsage: u64,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_OHOS_external_memory")]
unsafe impl<'a> Send for VkNativeBufferUsageOHOS<'a> {}
#[cfg(feature = "VK_OHOS_external_memory")]
unsafe impl<'a> Sync for VkNativeBufferUsageOHOS<'a> {}
#[cfg(all(feature = "VK_OHOS_external_memory", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkImageFormatProperties2<'root>>
  for VkNativeBufferUsageOHOS<'child>
{
}
#[cfg(feature = "VK_OHOS_external_memory")]
impl<'a> VkNativeBufferUsageOHOS<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::NATIVE_BUFFER_USAGE_OHOS,
    pNext: core::ptr::null_mut(),
    OHOSNativeBufferUsage: 0,
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
  pub const fn with_OHOSNativeBufferUsage(mut self, val: u64) -> Self {
    self.OHOSNativeBufferUsage = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkImageFormatProperties2<
    'root,
    T: VkPNextExtends<VkImageFormatProperties2<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkNativeBufferPropertiesOHOS](https://docs.vulkan.org/refpages/latest/refpages/source/VkNativeBufferPropertiesOHOS.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_OHOS_external_memory")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkNativeBufferPropertiesOHOS<'a> {
  /// Values: VK_STRUCTURE_TYPE_NATIVE_BUFFER_PROPERTIES_OHOS
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub allocationSize: VkDeviceSize,
  pub memoryTypeBits: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_OHOS_external_memory")]
unsafe impl<'a> Send for VkNativeBufferPropertiesOHOS<'a> {}
#[cfg(feature = "VK_OHOS_external_memory")]
unsafe impl<'a> Sync for VkNativeBufferPropertiesOHOS<'a> {}
#[cfg(feature = "VK_OHOS_external_memory")]
impl<'a> VkNativeBufferPropertiesOHOS<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::NATIVE_BUFFER_PROPERTIES_OHOS,
    pNext: core::ptr::null_mut(),
    allocationSize: 0,
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
  pub const fn with_allocationSize(mut self, val: VkDeviceSize) -> Self {
    self.allocationSize = val;
    self
  }
  #[inline]
  pub const fn with_memoryTypeBits(mut self, val: u32) -> Self {
    self.memoryTypeBits = val;
    self
  }
  #[cfg(feature = "VK_OHOS_external_memory")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkNativeBufferFormatPropertiesOHOS<'child>(
    mut self,
    val: &'a mut VkNativeBufferFormatPropertiesOHOS<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkNativeBufferFormatPropertiesOHOS<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_OHOS_external_memory")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkNativeBufferPropertiesOHOS<
    'root,
    T: VkPNextExtends<VkNativeBufferPropertiesOHOS<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkNativeBufferFormatPropertiesOHOS](https://docs.vulkan.org/refpages/latest/refpages/source/VkNativeBufferFormatPropertiesOHOS.html)
///
/// *Note: This is a **returned only** struct.*
///
/// **Extends:** VkNativeBufferPropertiesOHOS.
#[cfg(feature = "VK_OHOS_external_memory")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkNativeBufferFormatPropertiesOHOS<'a> {
  /// Values: VK_STRUCTURE_TYPE_NATIVE_BUFFER_FORMAT_PROPERTIES_OHOS
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub format: VkFormat,
  pub externalFormat: u64,
  pub formatFeatures: VkFormatFeatureFlags,
  pub samplerYcbcrConversionComponents: VkComponentMapping,
  pub suggestedYcbcrModel: VkSamplerYcbcrModelConversion,
  pub suggestedYcbcrRange: VkSamplerYcbcrRange,
  pub suggestedXChromaOffset: VkChromaLocation,
  pub suggestedYChromaOffset: VkChromaLocation,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_OHOS_external_memory")]
unsafe impl<'a> Send for VkNativeBufferFormatPropertiesOHOS<'a> {}
#[cfg(feature = "VK_OHOS_external_memory")]
unsafe impl<'a> Sync for VkNativeBufferFormatPropertiesOHOS<'a> {}
#[cfg(all(
  feature = "VK_OHOS_external_memory",
  feature = "VK_OHOS_external_memory"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkNativeBufferPropertiesOHOS<'root>>
  for VkNativeBufferFormatPropertiesOHOS<'child>
{
}
#[cfg(feature = "VK_OHOS_external_memory")]
impl<'a> VkNativeBufferFormatPropertiesOHOS<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::NATIVE_BUFFER_FORMAT_PROPERTIES_OHOS,
    pNext: core::ptr::null_mut(),
    format: VkFormat(0),
    externalFormat: 0,
    formatFeatures: VkFormatFeatureFlagBits(0),
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
  pub const fn with_format(mut self, val: VkFormat) -> Self {
    self.format = val;
    self
  }
  #[inline]
  pub const fn with_externalFormat(mut self, val: u64) -> Self {
    self.externalFormat = val;
    self
  }
  #[inline]
  pub const fn with_formatFeatures(mut self, val: VkFormatFeatureFlags) -> Self {
    self.formatFeatures = val;
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
  #[cfg(feature = "VK_OHOS_external_memory")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkNativeBufferPropertiesOHOS<
    'root,
    T: VkPNextExtends<VkNativeBufferPropertiesOHOS<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkImportNativeBufferInfoOHOS](https://docs.vulkan.org/refpages/latest/refpages/source/VkImportNativeBufferInfoOHOS.html)
///
/// **Extends:** VkMemoryAllocateInfo.
#[cfg(feature = "VK_OHOS_external_memory")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImportNativeBufferInfoOHOS<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMPORT_NATIVE_BUFFER_INFO_OHOS
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub buffer: *mut OH_NativeBuffer,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_OHOS_external_memory")]
unsafe impl<'a> Send for VkImportNativeBufferInfoOHOS<'a> {}
#[cfg(feature = "VK_OHOS_external_memory")]
unsafe impl<'a> Sync for VkImportNativeBufferInfoOHOS<'a> {}
#[cfg(all(feature = "VK_OHOS_external_memory", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkMemoryAllocateInfo<'root>>
  for VkImportNativeBufferInfoOHOS<'child>
{
}
#[cfg(feature = "VK_OHOS_external_memory")]
impl<'a> VkImportNativeBufferInfoOHOS<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::IMPORT_NATIVE_BUFFER_INFO_OHOS,
    pNext: core::ptr::null(),
    buffer: core::ptr::null_mut(),
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
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_buffer(mut self, val: *mut OH_NativeBuffer) -> Self {
    self.buffer = val;
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
/// [VkMemoryGetNativeBufferInfoOHOS](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryGetNativeBufferInfoOHOS.html)
#[cfg(feature = "VK_OHOS_external_memory")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkMemoryGetNativeBufferInfoOHOS<'a> {
  /// Values: VK_STRUCTURE_TYPE_MEMORY_GET_NATIVE_BUFFER_INFO_OHOS
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub memory: VkDeviceMemory,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_OHOS_external_memory")]
unsafe impl<'a> Send for VkMemoryGetNativeBufferInfoOHOS<'a> {}
#[cfg(feature = "VK_OHOS_external_memory")]
unsafe impl<'a> Sync for VkMemoryGetNativeBufferInfoOHOS<'a> {}
#[cfg(feature = "VK_OHOS_external_memory")]
impl<'a> VkMemoryGetNativeBufferInfoOHOS<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::MEMORY_GET_NATIVE_BUFFER_INFO_OHOS,
    pNext: core::ptr::null(),
    memory: VkDeviceMemory::DEFAULT,
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
  #[cfg(feature = "VK_OHOS_external_memory")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkMemoryGetNativeBufferInfoOHOS<
    'root,
    T: VkPNextExtends<VkMemoryGetNativeBufferInfoOHOS<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkExternalFormatOHOS](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFormatOHOS.html)
///
/// **Extends:** VkImageCreateInfo, VkSamplerYcbcrConversionCreateInfo, VkAttachmentDescription2, VkGraphicsPipelineCreateInfo, VkCommandBufferInheritanceInfo.
#[cfg(feature = "VK_OHOS_external_memory")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkExternalFormatOHOS<'a> {
  /// Values: VK_STRUCTURE_TYPE_EXTERNAL_FORMAT_OHOS
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub externalFormat: u64,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_OHOS_external_memory")]
unsafe impl<'a> Send for VkExternalFormatOHOS<'a> {}
#[cfg(feature = "VK_OHOS_external_memory")]
unsafe impl<'a> Sync for VkExternalFormatOHOS<'a> {}
#[cfg(all(feature = "VK_OHOS_external_memory", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkImageCreateInfo<'root>>
  for VkExternalFormatOHOS<'child>
{
}
#[cfg(all(
  feature = "VK_OHOS_external_memory",
  feature = "VK_COMPUTE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkSamplerYcbcrConversionCreateInfo<'root>>
  for VkExternalFormatOHOS<'child>
{
}
#[cfg(all(
  feature = "VK_OHOS_external_memory",
  feature = "VK_GRAPHICS_VERSION_1_2"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkAttachmentDescription2<'root>>
  for VkExternalFormatOHOS<'child>
{
}
#[cfg(all(
  feature = "VK_OHOS_external_memory",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkGraphicsPipelineCreateInfo<'root>>
  for VkExternalFormatOHOS<'child>
{
}
#[cfg(all(feature = "VK_OHOS_external_memory", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkCommandBufferInheritanceInfo<'root>>
  for VkExternalFormatOHOS<'child>
{
}
#[cfg(feature = "VK_OHOS_external_memory")]
impl<'a> VkExternalFormatOHOS<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::EXTERNAL_FORMAT_OHOS,
    pNext: core::ptr::null_mut(),
    externalFormat: 0,
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
  pub const fn with_externalFormat(mut self, val: u64) -> Self {
    self.externalFormat = val;
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
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSamplerYcbcrConversionCreateInfo<
    'root,
    T: VkPNextExtends<VkSamplerYcbcrConversionCreateInfo<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkAttachmentDescription2<
    'root,
    T: VkPNextExtends<VkAttachmentDescription2<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkGraphicsPipelineCreateInfo<
    'root,
    T: VkPNextExtends<VkGraphicsPipelineCreateInfo<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkCommandBufferInheritanceInfo<
    'root,
    T: VkPNextExtends<VkCommandBufferInheritanceInfo<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [OHNativeWindow](https://docs.vulkan.org/refpages/latest/refpages/source/OHNativeWindow.html)
/// Opaque platform handle - always used as a raw pointer.
#[cfg(feature = "VK_OHOS_surface")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OHNativeWindow(pub *mut c_void);
#[cfg(feature = "VK_OHOS_surface")]
impl OHNativeWindow {
  pub const NULL: Self = Self(core::ptr::null_mut());
}
#[cfg(feature = "VK_OHOS_surface")]
unsafe impl Send for OHNativeWindow {}
#[cfg(feature = "VK_OHOS_surface")]
unsafe impl Sync for OHNativeWindow {}
/// [VkSurfaceCreateFlagsOHOS](https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceCreateFlagsOHOS.html)
#[cfg(feature = "VK_OHOS_surface")]
pub type VkSurfaceCreateFlagsOHOS = VkFlags;
/// [VkSurfaceCreateInfoOHOS](https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceCreateInfoOHOS.html)
#[cfg(feature = "VK_OHOS_surface")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSurfaceCreateInfoOHOS<'a> {
  /// Values: VK_STRUCTURE_TYPE_SURFACE_CREATE_INFO_OHOS
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkSurfaceCreateFlagsOHOS,
  /// No Auto-Validity
  pub window: *mut OHNativeWindow,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_OHOS_surface")]
unsafe impl<'a> Send for VkSurfaceCreateInfoOHOS<'a> {}
#[cfg(feature = "VK_OHOS_surface")]
unsafe impl<'a> Sync for VkSurfaceCreateInfoOHOS<'a> {}
#[cfg(feature = "VK_OHOS_surface")]
impl<'a> VkSurfaceCreateInfoOHOS<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SURFACE_CREATE_INFO_OHOS,
    pNext: core::ptr::null(),
    flags: 0,
    window: core::ptr::null_mut(),
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
  pub const fn with_flags(mut self, val: VkSurfaceCreateFlagsOHOS) -> Self {
    self.flags = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_window(mut self, val: *mut OHNativeWindow) -> Self {
    self.window = val;
    self
  }
  #[cfg(feature = "VK_OHOS_surface")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSurfaceCreateInfoOHOS<
    'root,
    T: VkPNextExtends<VkSurfaceCreateInfoOHOS<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
