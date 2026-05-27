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
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBool32;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkComponentMapping;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceSize;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkFormatFeatureFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkMemoryAllocateInfo;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceFeatures2;
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
use crate::types::VkSamplerYcbcrConversionCreateInfo;
use core::ffi::c_void;
/// [_screen_buffer](https://docs.vulkan.org/refpages/latest/refpages/source/_screen_buffer.html)
/// Opaque platform handle - always used as a raw pointer.
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct _screen_buffer(pub *mut c_void);
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
impl _screen_buffer {
  pub const NULL: Self = Self(core::ptr::null_mut());
}
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
unsafe impl Send for _screen_buffer {}
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
unsafe impl Sync for _screen_buffer {}
/// [VkImportScreenBufferInfoQNX](https://docs.vulkan.org/refpages/latest/refpages/source/VkImportScreenBufferInfoQNX.html)
///
/// **Extends:** VkMemoryAllocateInfo.
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImportScreenBufferInfoQNX<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMPORT_SCREEN_BUFFER_INFO_QNX
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// No Auto-Validity
  pub buffer: *mut _screen_buffer,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
unsafe impl<'a> Send for VkImportScreenBufferInfoQNX<'a> {}
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
unsafe impl<'a> Sync for VkImportScreenBufferInfoQNX<'a> {}
#[cfg(all(
  feature = "VK_QNX_external_memory_screen_buffer",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkMemoryAllocateInfo<'root>>
  for VkImportScreenBufferInfoQNX<'child>
{
}
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
impl<'a> VkImportScreenBufferInfoQNX<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::IMPORT_SCREEN_BUFFER_INFO_QNX,
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
  pub const fn with_buffer(mut self, val: *mut _screen_buffer) -> Self {
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
/// [VkScreenBufferPropertiesQNX](https://docs.vulkan.org/refpages/latest/refpages/source/VkScreenBufferPropertiesQNX.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkScreenBufferPropertiesQNX<'a> {
  /// Values: VK_STRUCTURE_TYPE_SCREEN_BUFFER_PROPERTIES_QNX
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub allocationSize: VkDeviceSize,
  pub memoryTypeBits: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
unsafe impl<'a> Send for VkScreenBufferPropertiesQNX<'a> {}
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
unsafe impl<'a> Sync for VkScreenBufferPropertiesQNX<'a> {}
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
impl<'a> VkScreenBufferPropertiesQNX<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SCREEN_BUFFER_PROPERTIES_QNX,
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
  #[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkScreenBufferFormatPropertiesQNX<'child>(
    mut self,
    val: &'a mut VkScreenBufferFormatPropertiesQNX<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkScreenBufferFormatPropertiesQNX<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkScreenBufferPropertiesQNX<
    'root,
    T: VkPNextExtends<VkScreenBufferPropertiesQNX<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkScreenBufferFormatPropertiesQNX](https://docs.vulkan.org/refpages/latest/refpages/source/VkScreenBufferFormatPropertiesQNX.html)
///
/// *Note: This is a **returned only** struct.*
///
/// **Extends:** VkScreenBufferPropertiesQNX.
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkScreenBufferFormatPropertiesQNX<'a> {
  /// Values: VK_STRUCTURE_TYPE_SCREEN_BUFFER_FORMAT_PROPERTIES_QNX
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub format: VkFormat,
  pub externalFormat: u64,
  pub screenUsage: u64,
  pub formatFeatures: VkFormatFeatureFlags,
  pub samplerYcbcrConversionComponents: VkComponentMapping,
  pub suggestedYcbcrModel: VkSamplerYcbcrModelConversion,
  pub suggestedYcbcrRange: VkSamplerYcbcrRange,
  pub suggestedXChromaOffset: VkChromaLocation,
  pub suggestedYChromaOffset: VkChromaLocation,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
unsafe impl<'a> Send for VkScreenBufferFormatPropertiesQNX<'a> {}
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
unsafe impl<'a> Sync for VkScreenBufferFormatPropertiesQNX<'a> {}
#[cfg(all(
  feature = "VK_QNX_external_memory_screen_buffer",
  feature = "VK_QNX_external_memory_screen_buffer"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkScreenBufferPropertiesQNX<'root>>
  for VkScreenBufferFormatPropertiesQNX<'child>
{
}
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
impl<'a> VkScreenBufferFormatPropertiesQNX<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SCREEN_BUFFER_FORMAT_PROPERTIES_QNX,
    pNext: core::ptr::null_mut(),
    format: VkFormat(0),
    externalFormat: 0,
    screenUsage: 0,
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
  pub const fn with_screenUsage(mut self, val: u64) -> Self {
    self.screenUsage = val;
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
  #[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkScreenBufferPropertiesQNX<
    'root,
    T: VkPNextExtends<VkScreenBufferPropertiesQNX<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkExternalFormatQNX](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFormatQNX.html)
///
/// **Extends:** VkImageCreateInfo, VkSamplerYcbcrConversionCreateInfo.
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkExternalFormatQNX<'a> {
  /// Values: VK_STRUCTURE_TYPE_EXTERNAL_FORMAT_QNX
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub externalFormat: u64,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
unsafe impl<'a> Send for VkExternalFormatQNX<'a> {}
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
unsafe impl<'a> Sync for VkExternalFormatQNX<'a> {}
#[cfg(all(
  feature = "VK_QNX_external_memory_screen_buffer",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkImageCreateInfo<'root>>
  for VkExternalFormatQNX<'child>
{
}
#[cfg(all(
  feature = "VK_QNX_external_memory_screen_buffer",
  feature = "VK_COMPUTE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkSamplerYcbcrConversionCreateInfo<'root>>
  for VkExternalFormatQNX<'child>
{
}
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
impl<'a> VkExternalFormatQNX<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::EXTERNAL_FORMAT_QNX,
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
}
/// [VkPhysicalDeviceExternalMemoryScreenBufferFeaturesQNX](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExternalMemoryScreenBufferFeaturesQNX.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceExternalMemoryScreenBufferFeaturesQNX<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCREEN_BUFFER_FEATURES_QNX
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub screenBufferImport: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
unsafe impl<'a> Send for VkPhysicalDeviceExternalMemoryScreenBufferFeaturesQNX<'a> {}
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
unsafe impl<'a> Sync for VkPhysicalDeviceExternalMemoryScreenBufferFeaturesQNX<'a> {}
#[cfg(all(
  feature = "VK_QNX_external_memory_screen_buffer",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceExternalMemoryScreenBufferFeaturesQNX<'child>
{
}
#[cfg(all(
  feature = "VK_QNX_external_memory_screen_buffer",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceExternalMemoryScreenBufferFeaturesQNX<'child>
{
}
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
impl<'a> VkPhysicalDeviceExternalMemoryScreenBufferFeaturesQNX<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCREEN_BUFFER_FEATURES_QNX,
    pNext: core::ptr::null_mut(),
    screenBufferImport: 0,
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
  pub const fn with_screenBufferImport(mut self, val: VkBool32) -> Self {
    self.screenBufferImport = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPhysicalDeviceFeatures2<
    'root,
    T: VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>,
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
  pub const fn with_pNext_chain_VkDeviceCreateInfo<
    'root,
    T: VkPNextExtends<VkDeviceCreateInfo<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [_screen_context](https://docs.vulkan.org/refpages/latest/refpages/source/_screen_context.html)
/// Opaque platform handle - always used as a raw pointer.
#[cfg(feature = "VK_QNX_screen_surface")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct _screen_context(pub *mut c_void);
#[cfg(feature = "VK_QNX_screen_surface")]
impl _screen_context {
  pub const NULL: Self = Self(core::ptr::null_mut());
}
#[cfg(feature = "VK_QNX_screen_surface")]
unsafe impl Send for _screen_context {}
#[cfg(feature = "VK_QNX_screen_surface")]
unsafe impl Sync for _screen_context {}
/// [_screen_window](https://docs.vulkan.org/refpages/latest/refpages/source/_screen_window.html)
/// Opaque platform handle - always used as a raw pointer.
#[cfg(feature = "VK_QNX_screen_surface")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct _screen_window(pub *mut c_void);
#[cfg(feature = "VK_QNX_screen_surface")]
impl _screen_window {
  pub const NULL: Self = Self(core::ptr::null_mut());
}
#[cfg(feature = "VK_QNX_screen_surface")]
unsafe impl Send for _screen_window {}
#[cfg(feature = "VK_QNX_screen_surface")]
unsafe impl Sync for _screen_window {}
/// [VkScreenSurfaceCreateFlagsQNX](https://docs.vulkan.org/refpages/latest/refpages/source/VkScreenSurfaceCreateFlagsQNX.html)
#[cfg(feature = "VK_QNX_screen_surface")]
pub type VkScreenSurfaceCreateFlagsQNX = VkFlags;
/// [VkScreenSurfaceCreateInfoQNX](https://docs.vulkan.org/refpages/latest/refpages/source/VkScreenSurfaceCreateInfoQNX.html)
#[cfg(feature = "VK_QNX_screen_surface")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkScreenSurfaceCreateInfoQNX<'a> {
  /// Values: VK_STRUCTURE_TYPE_SCREEN_SURFACE_CREATE_INFO_QNX
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkScreenSurfaceCreateFlagsQNX,
  /// No Auto-Validity
  pub context: *mut _screen_context,
  /// No Auto-Validity
  pub window: *mut _screen_window,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QNX_screen_surface")]
unsafe impl<'a> Send for VkScreenSurfaceCreateInfoQNX<'a> {}
#[cfg(feature = "VK_QNX_screen_surface")]
unsafe impl<'a> Sync for VkScreenSurfaceCreateInfoQNX<'a> {}
#[cfg(feature = "VK_QNX_screen_surface")]
impl<'a> VkScreenSurfaceCreateInfoQNX<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SCREEN_SURFACE_CREATE_INFO_QNX,
    pNext: core::ptr::null(),
    flags: 0,
    context: core::ptr::null_mut(),
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
  pub const fn with_flags(mut self, val: VkScreenSurfaceCreateFlagsQNX) -> Self {
    self.flags = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_context(mut self, val: *mut _screen_context) -> Self {
    self.context = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_window(mut self, val: *mut _screen_window) -> Self {
    self.window = val;
    self
  }
  #[cfg(feature = "VK_QNX_screen_surface")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkScreenSurfaceCreateInfoQNX<
    'root,
    T: VkPNextExtends<VkScreenSurfaceCreateInfoQNX<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
