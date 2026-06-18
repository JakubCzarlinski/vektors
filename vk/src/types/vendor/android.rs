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
  feature = "VK_BASE_VERSION_1_3",
  feature = "VK_EXT_host_image_copy",
  all(
    feature = "VK_QCOM_image_processing",
    feature = "VK_QCOM_image_processing3"
  ),
  feature = "VK_KHR_format_feature_flags2",
  feature = "VK_NV_ray_tracing_linear_swept_spheres",
  feature = "VK_NV_optical_flow",
  feature = "VK_KHR_copy_memory_indirect",
  feature = "VK_KHR_video_encode_quantization_map",
  feature = "VK_IMG_filter_linear_2d"
))]
use crate::enums::VkFormatFeatureFlagBits2;
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
use crate::types::VkBool32;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkCommandBufferInheritanceInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkComponentMapping;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceMemory;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceSize;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkFormatFeatureFlags;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkFormatFeatureFlags2;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkGraphicsPipelineCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkImageFormatProperties2;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkMemoryAllocateInfo;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceFeatures2;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceProperties2;
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
use crate::types::VkSamplerYcbcrConversionCreateInfo;
use core::ffi::c_void;
/// [VkPhysicalDeviceExternalFormatResolveFeaturesANDROID](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExternalFormatResolveFeaturesANDROID.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_ANDROID_external_format_resolve")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceExternalFormatResolveFeaturesANDROID<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_FEATURES_ANDROID
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub externalFormatResolve: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ANDROID_external_format_resolve")]
unsafe impl<'a> Send for VkPhysicalDeviceExternalFormatResolveFeaturesANDROID<'a> {}
#[cfg(feature = "VK_ANDROID_external_format_resolve")]
unsafe impl<'a> Sync for VkPhysicalDeviceExternalFormatResolveFeaturesANDROID<'a> {}
#[cfg(all(
  feature = "VK_ANDROID_external_format_resolve",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceExternalFormatResolveFeaturesANDROID<'child>
{
}
#[cfg(all(
  feature = "VK_ANDROID_external_format_resolve",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceExternalFormatResolveFeaturesANDROID<'child>
{
}
#[cfg(feature = "VK_ANDROID_external_format_resolve")]
impl<'a> VkPhysicalDeviceExternalFormatResolveFeaturesANDROID<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_FEATURES_ANDROID,
    pNext: core::ptr::null_mut(),
    externalFormatResolve: 0,
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
  pub const fn with_externalFormatResolve(mut self, val: VkBool32) -> Self {
    self.externalFormatResolve = val;
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
/// [VkPhysicalDeviceExternalFormatResolvePropertiesANDROID](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExternalFormatResolvePropertiesANDROID.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_ANDROID_external_format_resolve")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceExternalFormatResolvePropertiesANDROID<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_PROPERTIES_ANDROID
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Min]
  pub nullColorAttachmentWithExternalFormatResolve: VkBool32,
  /// Limit Type: [Noauto]
  pub externalFormatResolveChromaOffsetX: VkChromaLocation,
  /// Limit Type: [Noauto]
  pub externalFormatResolveChromaOffsetY: VkChromaLocation,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ANDROID_external_format_resolve")]
unsafe impl<'a> Send for VkPhysicalDeviceExternalFormatResolvePropertiesANDROID<'a> {}
#[cfg(feature = "VK_ANDROID_external_format_resolve")]
unsafe impl<'a> Sync for VkPhysicalDeviceExternalFormatResolvePropertiesANDROID<'a> {}
#[cfg(all(
  feature = "VK_ANDROID_external_format_resolve",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceExternalFormatResolvePropertiesANDROID<'child>
{
}
#[cfg(feature = "VK_ANDROID_external_format_resolve")]
impl<'a> VkPhysicalDeviceExternalFormatResolvePropertiesANDROID<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_PROPERTIES_ANDROID,
    pNext: core::ptr::null_mut(),
    nullColorAttachmentWithExternalFormatResolve: 0,
    externalFormatResolveChromaOffsetX: VkChromaLocation(0),
    externalFormatResolveChromaOffsetY: VkChromaLocation(0),
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
  pub const fn with_nullColorAttachmentWithExternalFormatResolve(mut self, val: VkBool32) -> Self {
    self.nullColorAttachmentWithExternalFormatResolve = val;
    self
  }
  #[inline]
  pub const fn with_externalFormatResolveChromaOffsetX(mut self, val: VkChromaLocation) -> Self {
    self.externalFormatResolveChromaOffsetX = val;
    self
  }
  #[inline]
  pub const fn with_externalFormatResolveChromaOffsetY(mut self, val: VkChromaLocation) -> Self {
    self.externalFormatResolveChromaOffsetY = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPhysicalDeviceProperties2<
    'root,
    T: VkPNextExtends<VkPhysicalDeviceProperties2<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkAndroidHardwareBufferFormatResolvePropertiesANDROID](https://docs.vulkan.org/refpages/latest/refpages/source/VkAndroidHardwareBufferFormatResolvePropertiesANDROID.html)
///
/// *Note: This is a **returned only** struct.*
///
/// **Extends:** VkAndroidHardwareBufferPropertiesANDROID.
#[cfg(feature = "VK_ANDROID_external_format_resolve")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAndroidHardwareBufferFormatResolvePropertiesANDROID<'a> {
  /// Values: VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_RESOLVE_PROPERTIES_ANDROID
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub colorAttachmentFormat: VkFormat,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ANDROID_external_format_resolve")]
unsafe impl<'a> Send for VkAndroidHardwareBufferFormatResolvePropertiesANDROID<'a> {}
#[cfg(feature = "VK_ANDROID_external_format_resolve")]
unsafe impl<'a> Sync for VkAndroidHardwareBufferFormatResolvePropertiesANDROID<'a> {}
#[cfg(all(
  feature = "VK_ANDROID_external_format_resolve",
  feature = "VK_ANDROID_external_memory_android_hardware_buffer"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkAndroidHardwareBufferPropertiesANDROID<'root>>
  for VkAndroidHardwareBufferFormatResolvePropertiesANDROID<'child>
{
}
#[cfg(feature = "VK_ANDROID_external_format_resolve")]
impl<'a> VkAndroidHardwareBufferFormatResolvePropertiesANDROID<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::ANDROID_HARDWARE_BUFFER_FORMAT_RESOLVE_PROPERTIES_ANDROID,
    pNext: core::ptr::null_mut(),
    colorAttachmentFormat: VkFormat(0),
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
  pub const fn with_colorAttachmentFormat(mut self, val: VkFormat) -> Self {
    self.colorAttachmentFormat = val;
    self
  }
  #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkAndroidHardwareBufferPropertiesANDROID<
    'root,
    T: VkPNextExtends<VkAndroidHardwareBufferPropertiesANDROID<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [AHardwareBuffer](https://docs.vulkan.org/refpages/latest/refpages/source/AHardwareBuffer.html)
/// Opaque platform handle - always used as a raw pointer.
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AHardwareBuffer(pub *mut c_void);
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
impl AHardwareBuffer {
  pub const NULL: Self = Self(core::ptr::null_mut());
}
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
unsafe impl Send for AHardwareBuffer {}
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
unsafe impl Sync for AHardwareBuffer {}
/// [VkImportAndroidHardwareBufferInfoANDROID](https://docs.vulkan.org/refpages/latest/refpages/source/VkImportAndroidHardwareBufferInfoANDROID.html)
///
/// **Extends:** VkMemoryAllocateInfo.
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImportAndroidHardwareBufferInfoANDROID<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub buffer: *mut AHardwareBuffer,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
unsafe impl<'a> Send for VkImportAndroidHardwareBufferInfoANDROID<'a> {}
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
unsafe impl<'a> Sync for VkImportAndroidHardwareBufferInfoANDROID<'a> {}
#[cfg(all(
  feature = "VK_ANDROID_external_memory_android_hardware_buffer",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkMemoryAllocateInfo<'root>>
  for VkImportAndroidHardwareBufferInfoANDROID<'child>
{
}
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
impl<'a> VkImportAndroidHardwareBufferInfoANDROID<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID,
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
  pub const fn with_buffer(mut self, val: *mut AHardwareBuffer) -> Self {
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
/// [VkAndroidHardwareBufferUsageANDROID](https://docs.vulkan.org/refpages/latest/refpages/source/VkAndroidHardwareBufferUsageANDROID.html)
///
/// *Note: This is a **returned only** struct.*
///
/// **Extends:** VkImageFormatProperties2.
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAndroidHardwareBufferUsageANDROID<'a> {
  /// Values: VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_USAGE_ANDROID
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub androidHardwareBufferUsage: u64,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
unsafe impl<'a> Send for VkAndroidHardwareBufferUsageANDROID<'a> {}
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
unsafe impl<'a> Sync for VkAndroidHardwareBufferUsageANDROID<'a> {}
#[cfg(all(
  feature = "VK_ANDROID_external_memory_android_hardware_buffer",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkImageFormatProperties2<'root>>
  for VkAndroidHardwareBufferUsageANDROID<'child>
{
}
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
impl<'a> VkAndroidHardwareBufferUsageANDROID<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::ANDROID_HARDWARE_BUFFER_USAGE_ANDROID,
    pNext: core::ptr::null_mut(),
    androidHardwareBufferUsage: 0,
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
  pub const fn with_androidHardwareBufferUsage(mut self, val: u64) -> Self {
    self.androidHardwareBufferUsage = val;
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
/// [VkAndroidHardwareBufferPropertiesANDROID](https://docs.vulkan.org/refpages/latest/refpages/source/VkAndroidHardwareBufferPropertiesANDROID.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAndroidHardwareBufferPropertiesANDROID<'a> {
  /// Values: VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub allocationSize: VkDeviceSize,
  pub memoryTypeBits: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
unsafe impl<'a> Send for VkAndroidHardwareBufferPropertiesANDROID<'a> {}
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
unsafe impl<'a> Sync for VkAndroidHardwareBufferPropertiesANDROID<'a> {}
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
impl<'a> VkAndroidHardwareBufferPropertiesANDROID<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID,
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
  #[cfg(any(
    all(
      feature = "VK_ANDROID_external_memory_android_hardware_buffer",
      feature = "VK_KHR_format_feature_flags2"
    ),
    all(
      feature = "VK_ANDROID_external_memory_android_hardware_buffer",
      feature = "VK_VERSION_1_3"
    )
  ))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkAndroidHardwareBufferFormatProperties2ANDROID<'child>(
    mut self,
    val: &'a mut VkAndroidHardwareBufferFormatProperties2ANDROID<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkAndroidHardwareBufferFormatProperties2ANDROID<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkAndroidHardwareBufferFormatPropertiesANDROID<'child>(
    mut self,
    val: &'a mut VkAndroidHardwareBufferFormatPropertiesANDROID<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkAndroidHardwareBufferFormatPropertiesANDROID<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ANDROID_external_format_resolve")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkAndroidHardwareBufferFormatResolvePropertiesANDROID<'child>(
    mut self,
    val: &'a mut VkAndroidHardwareBufferFormatResolvePropertiesANDROID<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkAndroidHardwareBufferFormatResolvePropertiesANDROID<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkAndroidHardwareBufferPropertiesANDROID<
    'root,
    T: VkPNextExtends<VkAndroidHardwareBufferPropertiesANDROID<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkMemoryGetAndroidHardwareBufferInfoANDROID](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryGetAndroidHardwareBufferInfoANDROID.html)
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkMemoryGetAndroidHardwareBufferInfoANDROID<'a> {
  /// Values: VK_STRUCTURE_TYPE_MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub memory: VkDeviceMemory,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
unsafe impl<'a> Send for VkMemoryGetAndroidHardwareBufferInfoANDROID<'a> {}
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
unsafe impl<'a> Sync for VkMemoryGetAndroidHardwareBufferInfoANDROID<'a> {}
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
impl<'a> VkMemoryGetAndroidHardwareBufferInfoANDROID<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID,
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
  #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkMemoryGetAndroidHardwareBufferInfoANDROID<
    'root,
    T: VkPNextExtends<VkMemoryGetAndroidHardwareBufferInfoANDROID<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkAndroidHardwareBufferFormatPropertiesANDROID](https://docs.vulkan.org/refpages/latest/refpages/source/VkAndroidHardwareBufferFormatPropertiesANDROID.html)
///
/// *Note: This is a **returned only** struct.*
///
/// **Extends:** VkAndroidHardwareBufferPropertiesANDROID.
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAndroidHardwareBufferFormatPropertiesANDROID<'a> {
  /// Values: VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID
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
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
unsafe impl<'a> Send for VkAndroidHardwareBufferFormatPropertiesANDROID<'a> {}
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
unsafe impl<'a> Sync for VkAndroidHardwareBufferFormatPropertiesANDROID<'a> {}
#[cfg(all(
  feature = "VK_ANDROID_external_memory_android_hardware_buffer",
  feature = "VK_ANDROID_external_memory_android_hardware_buffer"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkAndroidHardwareBufferPropertiesANDROID<'root>>
  for VkAndroidHardwareBufferFormatPropertiesANDROID<'child>
{
}
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
impl<'a> VkAndroidHardwareBufferFormatPropertiesANDROID<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID,
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
  #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkAndroidHardwareBufferPropertiesANDROID<
    'root,
    T: VkPNextExtends<VkAndroidHardwareBufferPropertiesANDROID<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkExternalFormatANDROID](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFormatANDROID.html)
///
/// **Extends:** VkImageCreateInfo, VkSamplerYcbcrConversionCreateInfo, VkAttachmentDescription2, VkGraphicsPipelineCreateInfo, VkCommandBufferInheritanceInfo.
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkExternalFormatANDROID<'a> {
  /// Values: VK_STRUCTURE_TYPE_EXTERNAL_FORMAT_ANDROID
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub externalFormat: u64,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
unsafe impl<'a> Send for VkExternalFormatANDROID<'a> {}
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
unsafe impl<'a> Sync for VkExternalFormatANDROID<'a> {}
#[cfg(all(
  feature = "VK_ANDROID_external_memory_android_hardware_buffer",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkImageCreateInfo<'root>>
  for VkExternalFormatANDROID<'child>
{
}
#[cfg(all(
  feature = "VK_ANDROID_external_memory_android_hardware_buffer",
  feature = "VK_COMPUTE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkSamplerYcbcrConversionCreateInfo<'root>>
  for VkExternalFormatANDROID<'child>
{
}
#[cfg(all(
  feature = "VK_ANDROID_external_memory_android_hardware_buffer",
  feature = "VK_GRAPHICS_VERSION_1_2"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkAttachmentDescription2<'root>>
  for VkExternalFormatANDROID<'child>
{
}
#[cfg(all(
  feature = "VK_ANDROID_external_memory_android_hardware_buffer",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkGraphicsPipelineCreateInfo<'root>>
  for VkExternalFormatANDROID<'child>
{
}
#[cfg(all(
  feature = "VK_ANDROID_external_memory_android_hardware_buffer",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkCommandBufferInheritanceInfo<'root>>
  for VkExternalFormatANDROID<'child>
{
}
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
impl<'a> VkExternalFormatANDROID<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::EXTERNAL_FORMAT_ANDROID,
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
/// [VkAndroidHardwareBufferFormatProperties2ANDROID](https://docs.vulkan.org/refpages/latest/refpages/source/VkAndroidHardwareBufferFormatProperties2ANDROID.html)
///
/// *Note: This is a **returned only** struct.*
///
/// **Extends:** VkAndroidHardwareBufferPropertiesANDROID.
///
/// **Availability:** depends on `VK_KHR_format_feature_flags2 + VK_VERSION_1_3`.
#[cfg(any(
  all(
    feature = "VK_ANDROID_external_memory_android_hardware_buffer",
    feature = "VK_KHR_format_feature_flags2"
  ),
  all(
    feature = "VK_ANDROID_external_memory_android_hardware_buffer",
    feature = "VK_VERSION_1_3"
  )
))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAndroidHardwareBufferFormatProperties2ANDROID<'a> {
  /// Values: VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub format: VkFormat,
  pub externalFormat: u64,
  pub formatFeatures: VkFormatFeatureFlags2,
  pub samplerYcbcrConversionComponents: VkComponentMapping,
  pub suggestedYcbcrModel: VkSamplerYcbcrModelConversion,
  pub suggestedYcbcrRange: VkSamplerYcbcrRange,
  pub suggestedXChromaOffset: VkChromaLocation,
  pub suggestedYChromaOffset: VkChromaLocation,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(any(
  all(
    feature = "VK_ANDROID_external_memory_android_hardware_buffer",
    feature = "VK_KHR_format_feature_flags2"
  ),
  all(
    feature = "VK_ANDROID_external_memory_android_hardware_buffer",
    feature = "VK_VERSION_1_3"
  )
))]
unsafe impl<'a> Send for VkAndroidHardwareBufferFormatProperties2ANDROID<'a> {}
#[cfg(any(
  all(
    feature = "VK_ANDROID_external_memory_android_hardware_buffer",
    feature = "VK_KHR_format_feature_flags2"
  ),
  all(
    feature = "VK_ANDROID_external_memory_android_hardware_buffer",
    feature = "VK_VERSION_1_3"
  )
))]
unsafe impl<'a> Sync for VkAndroidHardwareBufferFormatProperties2ANDROID<'a> {}
#[cfg(all(
  any(
    all(
      feature = "VK_ANDROID_external_memory_android_hardware_buffer",
      feature = "VK_KHR_format_feature_flags2"
    ),
    all(
      feature = "VK_ANDROID_external_memory_android_hardware_buffer",
      feature = "VK_VERSION_1_3"
    )
  ),
  feature = "VK_ANDROID_external_memory_android_hardware_buffer"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkAndroidHardwareBufferPropertiesANDROID<'root>>
  for VkAndroidHardwareBufferFormatProperties2ANDROID<'child>
{
}
#[cfg(any(
  all(
    feature = "VK_ANDROID_external_memory_android_hardware_buffer",
    feature = "VK_KHR_format_feature_flags2"
  ),
  all(
    feature = "VK_ANDROID_external_memory_android_hardware_buffer",
    feature = "VK_VERSION_1_3"
  )
))]
impl<'a> VkAndroidHardwareBufferFormatProperties2ANDROID<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID,
    pNext: core::ptr::null_mut(),
    format: VkFormat(0),
    externalFormat: 0,
    formatFeatures: VkFormatFeatureFlagBits2(0),
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
  pub const fn with_formatFeatures(mut self, val: VkFormatFeatureFlags2) -> Self {
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
  #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkAndroidHardwareBufferPropertiesANDROID<
    'root,
    T: VkPNextExtends<VkAndroidHardwareBufferPropertiesANDROID<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
