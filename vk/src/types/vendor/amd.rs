#[cfg(feature = "VK_AMD_anti_lag")]
use crate::enums::VkAntiLagModeAMD;
#[cfg(feature = "VK_AMD_anti_lag")]
use crate::enums::VkAntiLagStageAMD;
#[cfg(feature = "VK_AMD_gpa_interface")]
use crate::enums::VkGpaDeviceClockModeAMD;
#[cfg(feature = "VK_AMD_gpa_interface")]
use crate::enums::VkGpaPerfBlockAMD;
#[cfg(feature = "VK_AMD_gpa_interface")]
use crate::enums::VkGpaSampleTypeAMD;
#[cfg(feature = "VK_AMD_gpa_interface")]
use crate::enums::VkGpaSqShaderStageFlagBitsAMD;
#[cfg(feature = "VK_AMD_memory_overallocation_behavior")]
use crate::enums::VkMemoryOverallocationBehaviorAMD;
#[cfg(feature = "VK_AMD_pipeline_compiler_control")]
use crate::enums::VkPipelineCompilerControlFlagBitsAMD;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkPipelineStageFlagBits;
#[cfg(feature = "VK_AMD_rasterization_order")]
use crate::enums::VkRasterizationOrderAMD;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkSampleCountFlagBits;
#[cfg(feature = "VK_AMD_shader_core_properties2")]
use crate::enums::VkShaderCorePropertiesFlagBitsAMD;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkShaderStageFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBool32;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkCommandBufferInheritanceInfo;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkComputePipelineCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceSize;
#[cfg(feature = "VK_AMDX_shader_enqueue")]
use crate::types::VkExecutionGraphPipelineCreateInfoAMDX;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkFlags;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkGraphicsPipelineCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkImageFormatProperties2;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceFeatures2;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceProperties2;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkPipelineRasterizationStateCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkPipelineStageFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkShaderStageFlags;
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
use crate::types::VkSurfaceCapabilities2KHR;
#[cfg(feature = "VK_KHR_swapchain")]
use crate::types::VkSwapchainCreateInfoKHR;
use core::ffi::c_void;
/// [VkPhysicalDeviceAntiLagFeaturesAMD](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceAntiLagFeaturesAMD.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_AMD_anti_lag")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceAntiLagFeaturesAMD<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ANTI_LAG_FEATURES_AMD
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub antiLag: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_AMD_anti_lag")]
unsafe impl<'a> Send for VkPhysicalDeviceAntiLagFeaturesAMD<'a> {}
#[cfg(feature = "VK_AMD_anti_lag")]
unsafe impl<'a> Sync for VkPhysicalDeviceAntiLagFeaturesAMD<'a> {}
#[cfg(all(feature = "VK_AMD_anti_lag", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceAntiLagFeaturesAMD<'child>
{
}
#[cfg(all(feature = "VK_AMD_anti_lag", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceAntiLagFeaturesAMD<'child>
{
}
#[cfg(feature = "VK_AMD_anti_lag")]
impl<'a> VkPhysicalDeviceAntiLagFeaturesAMD<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_ANTI_LAG_FEATURES_AMD,
    pNext: core::ptr::null_mut(),
    antiLag: 0,
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
  pub const fn with_antiLag(mut self, val: VkBool32) -> Self {
    self.antiLag = val;
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
/// [VkAntiLagDataAMD](https://docs.vulkan.org/refpages/latest/refpages/source/VkAntiLagDataAMD.html)
#[cfg(feature = "VK_AMD_anti_lag")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAntiLagDataAMD<'a> {
  /// Values: VK_STRUCTURE_TYPE_ANTI_LAG_DATA_AMD
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *const c_void,
  pub mode: VkAntiLagModeAMD,
  pub maxFPS: u32,
  /// Optional: true
  pub pPresentationInfo: *const VkAntiLagPresentationInfoAMD<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_AMD_anti_lag")]
unsafe impl<'a> Send for VkAntiLagDataAMD<'a> {}
#[cfg(feature = "VK_AMD_anti_lag")]
unsafe impl<'a> Sync for VkAntiLagDataAMD<'a> {}
#[cfg(feature = "VK_AMD_anti_lag")]
impl<'a> VkAntiLagDataAMD<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::ANTI_LAG_DATA_AMD,
    pNext: core::ptr::null(),
    mode: VkAntiLagModeAMD(0),
    maxFPS: 0,
    pPresentationInfo: core::ptr::null(),
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
  pub const fn with_mode(mut self, val: VkAntiLagModeAMD) -> Self {
    self.mode = val;
    self
  }
  #[inline]
  pub const fn with_maxFPS(mut self, val: u32) -> Self {
    self.maxFPS = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pPresentationInfo(
    mut self,
    val: *const VkAntiLagPresentationInfoAMD<'a>,
  ) -> Self {
    self.pPresentationInfo = val;
    self
  }
  #[cfg(feature = "VK_AMD_anti_lag")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkAntiLagDataAMD<
    'root,
    T: VkPNextExtends<VkAntiLagDataAMD<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkAntiLagPresentationInfoAMD](https://docs.vulkan.org/refpages/latest/refpages/source/VkAntiLagPresentationInfoAMD.html)
#[cfg(feature = "VK_AMD_anti_lag")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAntiLagPresentationInfoAMD<'a> {
  /// Values: VK_STRUCTURE_TYPE_ANTI_LAG_PRESENTATION_INFO_AMD
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub stage: VkAntiLagStageAMD,
  pub frameIndex: u64,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_AMD_anti_lag")]
unsafe impl<'a> Send for VkAntiLagPresentationInfoAMD<'a> {}
#[cfg(feature = "VK_AMD_anti_lag")]
unsafe impl<'a> Sync for VkAntiLagPresentationInfoAMD<'a> {}
#[cfg(feature = "VK_AMD_anti_lag")]
impl<'a> VkAntiLagPresentationInfoAMD<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::ANTI_LAG_PRESENTATION_INFO_AMD,
    pNext: core::ptr::null_mut(),
    stage: VkAntiLagStageAMD(0),
    frameIndex: 0,
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
  pub const fn with_stage(mut self, val: VkAntiLagStageAMD) -> Self {
    self.stage = val;
    self
  }
  #[inline]
  pub const fn with_frameIndex(mut self, val: u64) -> Self {
    self.frameIndex = val;
    self
  }
  #[cfg(feature = "VK_AMD_anti_lag")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkAntiLagPresentationInfoAMD<
    'root,
    T: VkPNextExtends<VkAntiLagPresentationInfoAMD<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceCoherentMemoryFeaturesAMD](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCoherentMemoryFeaturesAMD.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_AMD_device_coherent_memory")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceCoherentMemoryFeaturesAMD<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub deviceCoherentMemory: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_AMD_device_coherent_memory")]
unsafe impl<'a> Send for VkPhysicalDeviceCoherentMemoryFeaturesAMD<'a> {}
#[cfg(feature = "VK_AMD_device_coherent_memory")]
unsafe impl<'a> Sync for VkPhysicalDeviceCoherentMemoryFeaturesAMD<'a> {}
#[cfg(all(
  feature = "VK_AMD_device_coherent_memory",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceCoherentMemoryFeaturesAMD<'child>
{
}
#[cfg(all(
  feature = "VK_AMD_device_coherent_memory",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceCoherentMemoryFeaturesAMD<'child>
{
}
#[cfg(feature = "VK_AMD_device_coherent_memory")]
impl<'a> VkPhysicalDeviceCoherentMemoryFeaturesAMD<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD,
    pNext: core::ptr::null_mut(),
    deviceCoherentMemory: 0,
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
  pub const fn with_deviceCoherentMemory(mut self, val: VkBool32) -> Self {
    self.deviceCoherentMemory = val;
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
/// [VkDisplayNativeHdrSurfaceCapabilitiesAMD](https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayNativeHdrSurfaceCapabilitiesAMD.html)
///
/// *Note: This is a **returned only** struct.*
///
/// **Extends:** VkSurfaceCapabilities2KHR.
#[cfg(feature = "VK_AMD_display_native_hdr")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDisplayNativeHdrSurfaceCapabilitiesAMD<'a> {
  /// Values: VK_STRUCTURE_TYPE_DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub localDimmingSupport: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_AMD_display_native_hdr")]
unsafe impl<'a> Send for VkDisplayNativeHdrSurfaceCapabilitiesAMD<'a> {}
#[cfg(feature = "VK_AMD_display_native_hdr")]
unsafe impl<'a> Sync for VkDisplayNativeHdrSurfaceCapabilitiesAMD<'a> {}
#[cfg(all(
  feature = "VK_AMD_display_native_hdr",
  feature = "VK_KHR_get_surface_capabilities2"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkSurfaceCapabilities2KHR<'root>>
  for VkDisplayNativeHdrSurfaceCapabilitiesAMD<'child>
{
}
#[cfg(feature = "VK_AMD_display_native_hdr")]
impl<'a> VkDisplayNativeHdrSurfaceCapabilitiesAMD<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD,
    pNext: core::ptr::null_mut(),
    localDimmingSupport: 0,
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
  pub const fn with_localDimmingSupport(mut self, val: VkBool32) -> Self {
    self.localDimmingSupport = val;
    self
  }
  #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSurfaceCapabilities2KHR<
    'root,
    T: VkPNextExtends<VkSurfaceCapabilities2KHR<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkSwapchainDisplayNativeHdrCreateInfoAMD](https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainDisplayNativeHdrCreateInfoAMD.html)
///
/// **Extends:** VkSwapchainCreateInfoKHR.
#[cfg(feature = "VK_AMD_display_native_hdr")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSwapchainDisplayNativeHdrCreateInfoAMD<'a> {
  /// Values: VK_STRUCTURE_TYPE_SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub localDimmingEnable: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_AMD_display_native_hdr")]
unsafe impl<'a> Send for VkSwapchainDisplayNativeHdrCreateInfoAMD<'a> {}
#[cfg(feature = "VK_AMD_display_native_hdr")]
unsafe impl<'a> Sync for VkSwapchainDisplayNativeHdrCreateInfoAMD<'a> {}
#[cfg(all(feature = "VK_AMD_display_native_hdr", feature = "VK_KHR_swapchain"))]
unsafe impl<'child, 'root> VkPNextExtends<VkSwapchainCreateInfoKHR<'root>>
  for VkSwapchainDisplayNativeHdrCreateInfoAMD<'child>
{
}
#[cfg(feature = "VK_AMD_display_native_hdr")]
impl<'a> VkSwapchainDisplayNativeHdrCreateInfoAMD<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD,
    pNext: core::ptr::null(),
    localDimmingEnable: 0,
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
  pub const fn with_localDimmingEnable(mut self, val: VkBool32) -> Self {
    self.localDimmingEnable = val;
    self
  }
  #[cfg(feature = "VK_KHR_swapchain")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSwapchainCreateInfoKHR<
    'root,
    T: VkPNextExtends<VkSwapchainCreateInfoKHR<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkGpaSqShaderStageFlagsAMD](https://docs.vulkan.org/refpages/latest/refpages/source/VkGpaSqShaderStageFlagsAMD.html)
#[cfg(feature = "VK_AMD_gpa_interface")]
pub type VkGpaSqShaderStageFlagsAMD = VkGpaSqShaderStageFlagBitsAMD;
/// [VkGpaPerfBlockPropertiesFlagsAMD](https://docs.vulkan.org/refpages/latest/refpages/source/VkGpaPerfBlockPropertiesFlagsAMD.html)
#[cfg(feature = "VK_AMD_gpa_interface")]
pub type VkGpaPerfBlockPropertiesFlagsAMD = VkFlags;
/// [VkPhysicalDeviceGpaPropertiesFlagsAMD](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceGpaPropertiesFlagsAMD.html)
#[cfg(feature = "VK_AMD_gpa_interface")]
pub type VkPhysicalDeviceGpaPropertiesFlagsAMD = VkFlags;
/// [VkGpaSessionAMD](https://docs.vulkan.org/refpages/latest/refpages/source/VkGpaSessionAMD.html)
#[cfg(feature = "VK_AMD_gpa_interface")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkGpaSessionAMD(pub *mut c_void);
#[cfg(feature = "VK_AMD_gpa_interface")]
impl VkGpaSessionAMD {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_AMD_gpa_interface")]
impl Default for VkGpaSessionAMD {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_AMD_gpa_interface")]
unsafe impl Send for VkGpaSessionAMD {}
#[cfg(feature = "VK_AMD_gpa_interface")]
unsafe impl Sync for VkGpaSessionAMD {}
/// [VkGpaPerfBlockPropertiesAMD](https://docs.vulkan.org/refpages/latest/refpages/source/VkGpaPerfBlockPropertiesAMD.html)
#[cfg(feature = "VK_AMD_gpa_interface")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkGpaPerfBlockPropertiesAMD {
  pub blockType: VkGpaPerfBlockAMD,
  pub flags: VkGpaPerfBlockPropertiesFlagsAMD,
  pub instanceCount: u32,
  pub maxEventID: u32,
  pub maxGlobalOnlyCounters: u32,
  pub maxGlobalSharedCounters: u32,
  pub maxStreamingCounters: u32,
}
#[cfg(feature = "VK_AMD_gpa_interface")]
unsafe impl Send for VkGpaPerfBlockPropertiesAMD {}
#[cfg(feature = "VK_AMD_gpa_interface")]
unsafe impl Sync for VkGpaPerfBlockPropertiesAMD {}
#[cfg(feature = "VK_AMD_gpa_interface")]
impl VkGpaPerfBlockPropertiesAMD {
  pub const DEFAULT: Self = Self {
    blockType: VkGpaPerfBlockAMD(0),
    flags: 0,
    instanceCount: 0,
    maxEventID: 0,
    maxGlobalOnlyCounters: 0,
    maxGlobalSharedCounters: 0,
    maxStreamingCounters: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_blockType(mut self, val: VkGpaPerfBlockAMD) -> Self {
    self.blockType = val;
    self
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkGpaPerfBlockPropertiesFlagsAMD) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_instanceCount(mut self, val: u32) -> Self {
    self.instanceCount = val;
    self
  }
  #[inline]
  pub const fn with_maxEventID(mut self, val: u32) -> Self {
    self.maxEventID = val;
    self
  }
  #[inline]
  pub const fn with_maxGlobalOnlyCounters(mut self, val: u32) -> Self {
    self.maxGlobalOnlyCounters = val;
    self
  }
  #[inline]
  pub const fn with_maxGlobalSharedCounters(mut self, val: u32) -> Self {
    self.maxGlobalSharedCounters = val;
    self
  }
  #[inline]
  pub const fn with_maxStreamingCounters(mut self, val: u32) -> Self {
    self.maxStreamingCounters = val;
    self
  }
}
/// [VkPhysicalDeviceGpaFeaturesAMD](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceGpaFeaturesAMD.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_AMD_gpa_interface")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceGpaFeaturesAMD<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GPA_FEATURES_AMD
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub perfCounters: VkBool32,
  pub streamingPerfCounters: VkBool32,
  pub sqThreadTracing: VkBool32,
  pub clockModes: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_AMD_gpa_interface")]
unsafe impl<'a> Send for VkPhysicalDeviceGpaFeaturesAMD<'a> {}
#[cfg(feature = "VK_AMD_gpa_interface")]
unsafe impl<'a> Sync for VkPhysicalDeviceGpaFeaturesAMD<'a> {}
#[cfg(all(feature = "VK_AMD_gpa_interface", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceGpaFeaturesAMD<'child>
{
}
#[cfg(all(feature = "VK_AMD_gpa_interface", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceGpaFeaturesAMD<'child>
{
}
#[cfg(feature = "VK_AMD_gpa_interface")]
impl<'a> VkPhysicalDeviceGpaFeaturesAMD<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_GPA_FEATURES_AMD,
    pNext: core::ptr::null_mut(),
    perfCounters: 0,
    streamingPerfCounters: 0,
    sqThreadTracing: 0,
    clockModes: 0,
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
  pub const fn with_perfCounters(mut self, val: VkBool32) -> Self {
    self.perfCounters = val;
    self
  }
  #[inline]
  pub const fn with_streamingPerfCounters(mut self, val: VkBool32) -> Self {
    self.streamingPerfCounters = val;
    self
  }
  #[inline]
  pub const fn with_sqThreadTracing(mut self, val: VkBool32) -> Self {
    self.sqThreadTracing = val;
    self
  }
  #[inline]
  pub const fn with_clockModes(mut self, val: VkBool32) -> Self {
    self.clockModes = val;
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
/// [VkPhysicalDeviceGpaPropertiesAMD](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceGpaPropertiesAMD.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_AMD_gpa_interface")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceGpaPropertiesAMD<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GPA_PROPERTIES_AMD
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Noauto]
  pub flags: VkPhysicalDeviceGpaPropertiesFlagsAMD,
  /// Limit Type: [Max]
  pub maxSqttSeBufferSize: VkDeviceSize,
  /// Limit Type: [Noauto]
  pub shaderEngineCount: u32,
  /// Limit Type: [Noauto]
  pub perfBlockCount: u32,
  /// Length: perfBlockCount,  Limit Type: [Noauto]
  pub pPerfBlocks: *mut VkGpaPerfBlockPropertiesAMD,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_AMD_gpa_interface")]
unsafe impl<'a> Send for VkPhysicalDeviceGpaPropertiesAMD<'a> {}
#[cfg(feature = "VK_AMD_gpa_interface")]
unsafe impl<'a> Sync for VkPhysicalDeviceGpaPropertiesAMD<'a> {}
#[cfg(all(feature = "VK_AMD_gpa_interface", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceGpaPropertiesAMD<'child>
{
}
#[cfg(feature = "VK_AMD_gpa_interface")]
impl<'a> VkPhysicalDeviceGpaPropertiesAMD<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_GPA_PROPERTIES_AMD,
    pNext: core::ptr::null_mut(),
    flags: 0,
    maxSqttSeBufferSize: 0,
    shaderEngineCount: 0,
    perfBlockCount: 0,
    pPerfBlocks: core::ptr::null_mut(),
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
  pub const fn with_flags(mut self, val: VkPhysicalDeviceGpaPropertiesFlagsAMD) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_maxSqttSeBufferSize(mut self, val: VkDeviceSize) -> Self {
    self.maxSqttSeBufferSize = val;
    self
  }
  #[inline]
  pub const fn with_shaderEngineCount(mut self, val: u32) -> Self {
    self.shaderEngineCount = val;
    self
  }
  #[inline]
  pub const fn with_perfBlockCount(mut self, val: u32) -> Self {
    self.perfBlockCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pPerfBlocks(mut self, val: &'a mut [VkGpaPerfBlockPropertiesAMD]) -> Self {
    self.perfBlockCount = val.len() as u32;
    self.pPerfBlocks = val.as_mut_ptr();
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
/// [VkPhysicalDeviceGpaProperties2AMD](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceGpaProperties2AMD.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_AMD_gpa_interface")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceGpaProperties2AMD<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GPA_PROPERTIES_2_AMD
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Noauto]
  pub revisionId: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_AMD_gpa_interface")]
unsafe impl<'a> Send for VkPhysicalDeviceGpaProperties2AMD<'a> {}
#[cfg(feature = "VK_AMD_gpa_interface")]
unsafe impl<'a> Sync for VkPhysicalDeviceGpaProperties2AMD<'a> {}
#[cfg(all(feature = "VK_AMD_gpa_interface", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceGpaProperties2AMD<'child>
{
}
#[cfg(feature = "VK_AMD_gpa_interface")]
impl<'a> VkPhysicalDeviceGpaProperties2AMD<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_GPA_PROPERTIES_2_AMD,
    pNext: core::ptr::null_mut(),
    revisionId: 0,
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
  pub const fn with_revisionId(mut self, val: u32) -> Self {
    self.revisionId = val;
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
/// [VkGpaPerfCounterAMD](https://docs.vulkan.org/refpages/latest/refpages/source/VkGpaPerfCounterAMD.html)
#[cfg(feature = "VK_AMD_gpa_interface")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkGpaPerfCounterAMD {
  pub blockType: VkGpaPerfBlockAMD,
  pub blockInstance: u32,
  pub eventID: u32,
}
#[cfg(feature = "VK_AMD_gpa_interface")]
unsafe impl Send for VkGpaPerfCounterAMD {}
#[cfg(feature = "VK_AMD_gpa_interface")]
unsafe impl Sync for VkGpaPerfCounterAMD {}
#[cfg(feature = "VK_AMD_gpa_interface")]
impl VkGpaPerfCounterAMD {
  pub const DEFAULT: Self = Self {
    blockType: VkGpaPerfBlockAMD(0),
    blockInstance: 0,
    eventID: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_blockType(mut self, val: VkGpaPerfBlockAMD) -> Self {
    self.blockType = val;
    self
  }
  #[inline]
  pub const fn with_blockInstance(mut self, val: u32) -> Self {
    self.blockInstance = val;
    self
  }
  #[inline]
  pub const fn with_eventID(mut self, val: u32) -> Self {
    self.eventID = val;
    self
  }
}
/// [VkGpaSampleBeginInfoAMD](https://docs.vulkan.org/refpages/latest/refpages/source/VkGpaSampleBeginInfoAMD.html)
#[cfg(feature = "VK_AMD_gpa_interface")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkGpaSampleBeginInfoAMD<'a> {
  /// Values: VK_STRUCTURE_TYPE_GPA_SAMPLE_BEGIN_INFO_AMD
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub sampleType: VkGpaSampleTypeAMD,
  pub sampleInternalOperations: VkBool32,
  pub cacheFlushOnCounterCollection: VkBool32,
  pub sqShaderMaskEnable: VkBool32,
  pub sqShaderMask: VkGpaSqShaderStageFlagsAMD,
  pub perfCounterCount: u32,
  /// Length: perfCounterCount
  pub pPerfCounters: *const VkGpaPerfCounterAMD,
  pub streamingPerfTraceSampleInterval: u32,
  pub perfCounterDeviceMemoryLimit: VkDeviceSize,
  pub sqThreadTraceEnable: VkBool32,
  pub sqThreadTraceSuppressInstructionTokens: VkBool32,
  pub sqThreadTraceDeviceMemoryLimit: VkDeviceSize,
  pub timingPreSample: VkPipelineStageFlags,
  pub timingPostSample: VkPipelineStageFlags,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_AMD_gpa_interface")]
unsafe impl<'a> Send for VkGpaSampleBeginInfoAMD<'a> {}
#[cfg(feature = "VK_AMD_gpa_interface")]
unsafe impl<'a> Sync for VkGpaSampleBeginInfoAMD<'a> {}
#[cfg(feature = "VK_AMD_gpa_interface")]
impl<'a> VkGpaSampleBeginInfoAMD<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::GPA_SAMPLE_BEGIN_INFO_AMD,
    pNext: core::ptr::null(),
    sampleType: VkGpaSampleTypeAMD(0),
    sampleInternalOperations: 0,
    cacheFlushOnCounterCollection: 0,
    sqShaderMaskEnable: 0,
    sqShaderMask: VkGpaSqShaderStageFlagBitsAMD(0),
    perfCounterCount: 0,
    pPerfCounters: core::ptr::null(),
    streamingPerfTraceSampleInterval: 0,
    perfCounterDeviceMemoryLimit: 0,
    sqThreadTraceEnable: 0,
    sqThreadTraceSuppressInstructionTokens: 0,
    sqThreadTraceDeviceMemoryLimit: 0,
    timingPreSample: VkPipelineStageFlagBits(0),
    timingPostSample: VkPipelineStageFlagBits(0),
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
  pub const fn with_sampleType(mut self, val: VkGpaSampleTypeAMD) -> Self {
    self.sampleType = val;
    self
  }
  #[inline]
  pub const fn with_sampleInternalOperations(mut self, val: VkBool32) -> Self {
    self.sampleInternalOperations = val;
    self
  }
  #[inline]
  pub const fn with_cacheFlushOnCounterCollection(mut self, val: VkBool32) -> Self {
    self.cacheFlushOnCounterCollection = val;
    self
  }
  #[inline]
  pub const fn with_sqShaderMaskEnable(mut self, val: VkBool32) -> Self {
    self.sqShaderMaskEnable = val;
    self
  }
  #[inline]
  pub const fn with_sqShaderMask(mut self, val: VkGpaSqShaderStageFlagsAMD) -> Self {
    self.sqShaderMask = val;
    self
  }
  #[inline]
  pub const fn with_perfCounterCount(mut self, val: u32) -> Self {
    self.perfCounterCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pPerfCounters(mut self, val: &'a [VkGpaPerfCounterAMD]) -> Self {
    self.perfCounterCount = val.len() as u32;
    self.pPerfCounters = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_streamingPerfTraceSampleInterval(mut self, val: u32) -> Self {
    self.streamingPerfTraceSampleInterval = val;
    self
  }
  #[inline]
  pub const fn with_perfCounterDeviceMemoryLimit(mut self, val: VkDeviceSize) -> Self {
    self.perfCounterDeviceMemoryLimit = val;
    self
  }
  #[inline]
  pub const fn with_sqThreadTraceEnable(mut self, val: VkBool32) -> Self {
    self.sqThreadTraceEnable = val;
    self
  }
  #[inline]
  pub const fn with_sqThreadTraceSuppressInstructionTokens(mut self, val: VkBool32) -> Self {
    self.sqThreadTraceSuppressInstructionTokens = val;
    self
  }
  #[inline]
  pub const fn with_sqThreadTraceDeviceMemoryLimit(mut self, val: VkDeviceSize) -> Self {
    self.sqThreadTraceDeviceMemoryLimit = val;
    self
  }
  #[inline]
  pub const fn with_timingPreSample(mut self, val: VkPipelineStageFlags) -> Self {
    self.timingPreSample = val;
    self
  }
  #[inline]
  pub const fn with_timingPostSample(mut self, val: VkPipelineStageFlags) -> Self {
    self.timingPostSample = val;
    self
  }
  #[cfg(feature = "VK_AMD_gpa_interface")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkGpaSampleBeginInfoAMD<
    'root,
    T: VkPNextExtends<VkGpaSampleBeginInfoAMD<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkGpaDeviceClockModeInfoAMD](https://docs.vulkan.org/refpages/latest/refpages/source/VkGpaDeviceClockModeInfoAMD.html)
#[cfg(feature = "VK_AMD_gpa_interface")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkGpaDeviceClockModeInfoAMD<'a> {
  /// Values: VK_STRUCTURE_TYPE_GPA_DEVICE_CLOCK_MODE_INFO_AMD
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub clockMode: VkGpaDeviceClockModeAMD,
  pub memoryClockRatioToPeak: f32,
  pub engineClockRatioToPeak: f32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_AMD_gpa_interface")]
unsafe impl<'a> Send for VkGpaDeviceClockModeInfoAMD<'a> {}
#[cfg(feature = "VK_AMD_gpa_interface")]
unsafe impl<'a> Sync for VkGpaDeviceClockModeInfoAMD<'a> {}
#[cfg(feature = "VK_AMD_gpa_interface")]
impl<'a> VkGpaDeviceClockModeInfoAMD<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::GPA_DEVICE_CLOCK_MODE_INFO_AMD,
    pNext: core::ptr::null(),
    clockMode: VkGpaDeviceClockModeAMD(0),
    memoryClockRatioToPeak: 0.0f32,
    engineClockRatioToPeak: 0.0f32,
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
  pub const fn with_clockMode(mut self, val: VkGpaDeviceClockModeAMD) -> Self {
    self.clockMode = val;
    self
  }
  #[inline]
  pub const fn with_memoryClockRatioToPeak(mut self, val: f32) -> Self {
    self.memoryClockRatioToPeak = val;
    self
  }
  #[inline]
  pub const fn with_engineClockRatioToPeak(mut self, val: f32) -> Self {
    self.engineClockRatioToPeak = val;
    self
  }
  #[cfg(feature = "VK_AMD_gpa_interface")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkGpaDeviceClockModeInfoAMD<
    'root,
    T: VkPNextExtends<VkGpaDeviceClockModeInfoAMD<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkGpaDeviceGetClockInfoAMD](https://docs.vulkan.org/refpages/latest/refpages/source/VkGpaDeviceGetClockInfoAMD.html)
#[cfg(feature = "VK_AMD_gpa_interface")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkGpaDeviceGetClockInfoAMD<'a> {
  /// Values: VK_STRUCTURE_TYPE_GPA_DEVICE_GET_CLOCK_INFO_AMD
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub memoryClockRatioToPeak: f32,
  pub engineClockRatioToPeak: f32,
  pub memoryClockFrequency: u32,
  pub engineClockFrequency: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_AMD_gpa_interface")]
unsafe impl<'a> Send for VkGpaDeviceGetClockInfoAMD<'a> {}
#[cfg(feature = "VK_AMD_gpa_interface")]
unsafe impl<'a> Sync for VkGpaDeviceGetClockInfoAMD<'a> {}
#[cfg(feature = "VK_AMD_gpa_interface")]
impl<'a> VkGpaDeviceGetClockInfoAMD<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::GPA_DEVICE_GET_CLOCK_INFO_AMD,
    pNext: core::ptr::null_mut(),
    memoryClockRatioToPeak: 0.0f32,
    engineClockRatioToPeak: 0.0f32,
    memoryClockFrequency: 0,
    engineClockFrequency: 0,
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
  pub const fn with_memoryClockRatioToPeak(mut self, val: f32) -> Self {
    self.memoryClockRatioToPeak = val;
    self
  }
  #[inline]
  pub const fn with_engineClockRatioToPeak(mut self, val: f32) -> Self {
    self.engineClockRatioToPeak = val;
    self
  }
  #[inline]
  pub const fn with_memoryClockFrequency(mut self, val: u32) -> Self {
    self.memoryClockFrequency = val;
    self
  }
  #[inline]
  pub const fn with_engineClockFrequency(mut self, val: u32) -> Self {
    self.engineClockFrequency = val;
    self
  }
  #[cfg(feature = "VK_AMD_gpa_interface")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkGpaDeviceGetClockInfoAMD<
    'root,
    T: VkPNextExtends<VkGpaDeviceGetClockInfoAMD<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkGpaSessionCreateInfoAMD](https://docs.vulkan.org/refpages/latest/refpages/source/VkGpaSessionCreateInfoAMD.html)
#[cfg(feature = "VK_AMD_gpa_interface")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkGpaSessionCreateInfoAMD<'a> {
  /// Values: VK_STRUCTURE_TYPE_GPA_SESSION_CREATE_INFO_AMD
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub secondaryCopySource: VkGpaSessionAMD,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_AMD_gpa_interface")]
unsafe impl<'a> Send for VkGpaSessionCreateInfoAMD<'a> {}
#[cfg(feature = "VK_AMD_gpa_interface")]
unsafe impl<'a> Sync for VkGpaSessionCreateInfoAMD<'a> {}
#[cfg(feature = "VK_AMD_gpa_interface")]
impl<'a> VkGpaSessionCreateInfoAMD<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::GPA_SESSION_CREATE_INFO_AMD,
    pNext: core::ptr::null(),
    secondaryCopySource: VkGpaSessionAMD::DEFAULT,
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
  pub const fn with_secondaryCopySource(mut self, val: VkGpaSessionAMD) -> Self {
    self.secondaryCopySource = val;
    self
  }
  #[cfg(feature = "VK_AMD_gpa_interface")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkGpaSessionCreateInfoAMD<
    'root,
    T: VkPNextExtends<VkGpaSessionCreateInfoAMD<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkDeviceMemoryOverallocationCreateInfoAMD](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceMemoryOverallocationCreateInfoAMD.html)
///
/// **Extends:** VkDeviceCreateInfo.
#[cfg(feature = "VK_AMD_memory_overallocation_behavior")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDeviceMemoryOverallocationCreateInfoAMD<'a> {
  /// Values: VK_STRUCTURE_TYPE_DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub overallocationBehavior: VkMemoryOverallocationBehaviorAMD,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_AMD_memory_overallocation_behavior")]
unsafe impl<'a> Send for VkDeviceMemoryOverallocationCreateInfoAMD<'a> {}
#[cfg(feature = "VK_AMD_memory_overallocation_behavior")]
unsafe impl<'a> Sync for VkDeviceMemoryOverallocationCreateInfoAMD<'a> {}
#[cfg(all(
  feature = "VK_AMD_memory_overallocation_behavior",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkDeviceMemoryOverallocationCreateInfoAMD<'child>
{
}
#[cfg(feature = "VK_AMD_memory_overallocation_behavior")]
impl<'a> VkDeviceMemoryOverallocationCreateInfoAMD<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD,
    pNext: core::ptr::null(),
    overallocationBehavior: VkMemoryOverallocationBehaviorAMD(0),
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
  pub const fn with_overallocationBehavior(
    mut self,
    val: VkMemoryOverallocationBehaviorAMD,
  ) -> Self {
    self.overallocationBehavior = val;
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
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkAttachmentSampleCountInfoAMD](https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentSampleCountInfoAMD.html)
///
/// **Extends:** VkCommandBufferInheritanceInfo, VkGraphicsPipelineCreateInfo.
///
/// **Availability:** depends on `VK_VERSION_1_3 + VK_KHR_dynamic_rendering`.
#[cfg(any(
  all(
    feature = "VK_AMD_mixed_attachment_samples",
    feature = "VK_VERSION_1_3"
  ),
  all(
    feature = "VK_AMD_mixed_attachment_samples",
    feature = "VK_KHR_dynamic_rendering"
  )
))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAttachmentSampleCountInfoAMD<'a> {
  /// Values: VK_STRUCTURE_TYPE_ATTACHMENT_SAMPLE_COUNT_INFO_AMD
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub colorAttachmentCount: u32,
  /// Length: colorAttachmentCount,  No Auto-Validity
  pub pColorAttachmentSamples: *const VkSampleCountFlagBits,
  /// Optional: true,  No Auto-Validity
  pub depthStencilAttachmentSamples: VkSampleCountFlagBits,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(any(
  all(
    feature = "VK_AMD_mixed_attachment_samples",
    feature = "VK_VERSION_1_3"
  ),
  all(
    feature = "VK_AMD_mixed_attachment_samples",
    feature = "VK_KHR_dynamic_rendering"
  )
))]
unsafe impl<'a> Send for VkAttachmentSampleCountInfoAMD<'a> {}
#[cfg(any(
  all(
    feature = "VK_AMD_mixed_attachment_samples",
    feature = "VK_VERSION_1_3"
  ),
  all(
    feature = "VK_AMD_mixed_attachment_samples",
    feature = "VK_KHR_dynamic_rendering"
  )
))]
unsafe impl<'a> Sync for VkAttachmentSampleCountInfoAMD<'a> {}
#[cfg(all(
  any(
    all(
      feature = "VK_AMD_mixed_attachment_samples",
      feature = "VK_VERSION_1_3"
    ),
    all(
      feature = "VK_AMD_mixed_attachment_samples",
      feature = "VK_KHR_dynamic_rendering"
    )
  ),
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkCommandBufferInheritanceInfo<'root>>
  for VkAttachmentSampleCountInfoAMD<'child>
{
}
#[cfg(all(
  any(
    all(
      feature = "VK_AMD_mixed_attachment_samples",
      feature = "VK_VERSION_1_3"
    ),
    all(
      feature = "VK_AMD_mixed_attachment_samples",
      feature = "VK_KHR_dynamic_rendering"
    )
  ),
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkGraphicsPipelineCreateInfo<'root>>
  for VkAttachmentSampleCountInfoAMD<'child>
{
}
#[cfg(any(
  all(
    feature = "VK_AMD_mixed_attachment_samples",
    feature = "VK_VERSION_1_3"
  ),
  all(
    feature = "VK_AMD_mixed_attachment_samples",
    feature = "VK_KHR_dynamic_rendering"
  )
))]
impl<'a> VkAttachmentSampleCountInfoAMD<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::ATTACHMENT_SAMPLE_COUNT_INFO_AMD,
    pNext: core::ptr::null(),
    colorAttachmentCount: 0,
    pColorAttachmentSamples: core::ptr::null(),
    depthStencilAttachmentSamples: VkSampleCountFlagBits(0),
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
  pub const fn with_colorAttachmentCount(mut self, val: u32) -> Self {
    self.colorAttachmentCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pColorAttachmentSamples(mut self, val: &'a [VkSampleCountFlagBits]) -> Self {
    self.colorAttachmentCount = val.len() as u32;
    self.pColorAttachmentSamples = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_depthStencilAttachmentSamples(mut self, val: VkSampleCountFlagBits) -> Self {
    self.depthStencilAttachmentSamples = val;
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
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
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
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPipelineCompilerControlFlagsAMD](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCompilerControlFlagsAMD.html)
#[cfg(feature = "VK_AMD_pipeline_compiler_control")]
pub type VkPipelineCompilerControlFlagsAMD = VkPipelineCompilerControlFlagBitsAMD;
/// [VkPipelineCompilerControlCreateInfoAMD](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCompilerControlCreateInfoAMD.html)
///
/// **Extends:** VkGraphicsPipelineCreateInfo, VkComputePipelineCreateInfo, VkExecutionGraphPipelineCreateInfoAMDX.
#[cfg(feature = "VK_AMD_pipeline_compiler_control")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineCompilerControlCreateInfoAMD<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub compilerControlFlags: VkPipelineCompilerControlFlagsAMD,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_AMD_pipeline_compiler_control")]
unsafe impl<'a> Send for VkPipelineCompilerControlCreateInfoAMD<'a> {}
#[cfg(feature = "VK_AMD_pipeline_compiler_control")]
unsafe impl<'a> Sync for VkPipelineCompilerControlCreateInfoAMD<'a> {}
#[cfg(all(
  feature = "VK_AMD_pipeline_compiler_control",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkGraphicsPipelineCreateInfo<'root>>
  for VkPipelineCompilerControlCreateInfoAMD<'child>
{
}
#[cfg(all(
  feature = "VK_AMD_pipeline_compiler_control",
  feature = "VK_COMPUTE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkComputePipelineCreateInfo<'root>>
  for VkPipelineCompilerControlCreateInfoAMD<'child>
{
}
#[cfg(all(
  feature = "VK_AMD_pipeline_compiler_control",
  feature = "VK_AMDX_shader_enqueue"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkExecutionGraphPipelineCreateInfoAMDX<'root>>
  for VkPipelineCompilerControlCreateInfoAMD<'child>
{
}
#[cfg(feature = "VK_AMD_pipeline_compiler_control")]
impl<'a> VkPipelineCompilerControlCreateInfoAMD<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD,
    pNext: core::ptr::null(),
    compilerControlFlags: VkPipelineCompilerControlFlagBitsAMD(0),
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
  pub const fn with_compilerControlFlags(mut self, val: VkPipelineCompilerControlFlagsAMD) -> Self {
    self.compilerControlFlags = val;
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
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkComputePipelineCreateInfo<
    'root,
    T: VkPNextExtends<VkComputePipelineCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_AMDX_shader_enqueue")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkExecutionGraphPipelineCreateInfoAMDX<
    'root,
    T: VkPNextExtends<VkExecutionGraphPipelineCreateInfoAMDX<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPipelineRasterizationStateRasterizationOrderAMD](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationStateRasterizationOrderAMD.html)
///
/// **Extends:** VkPipelineRasterizationStateCreateInfo.
#[cfg(feature = "VK_AMD_rasterization_order")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineRasterizationStateRasterizationOrderAMD<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub rasterizationOrder: VkRasterizationOrderAMD,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_AMD_rasterization_order")]
unsafe impl<'a> Send for VkPipelineRasterizationStateRasterizationOrderAMD<'a> {}
#[cfg(feature = "VK_AMD_rasterization_order")]
unsafe impl<'a> Sync for VkPipelineRasterizationStateRasterizationOrderAMD<'a> {}
#[cfg(all(
  feature = "VK_AMD_rasterization_order",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPipelineRasterizationStateCreateInfo<'root>>
  for VkPipelineRasterizationStateRasterizationOrderAMD<'child>
{
}
#[cfg(feature = "VK_AMD_rasterization_order")]
impl<'a> VkPipelineRasterizationStateRasterizationOrderAMD<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD,
    pNext: core::ptr::null(),
    rasterizationOrder: VkRasterizationOrderAMD(0),
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
  pub const fn with_rasterizationOrder(mut self, val: VkRasterizationOrderAMD) -> Self {
    self.rasterizationOrder = val;
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPipelineRasterizationStateCreateInfo<
    'root,
    T: VkPNextExtends<VkPipelineRasterizationStateCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceShaderCorePropertiesAMD](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderCorePropertiesAMD.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_AMD_shader_core_properties")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShaderCorePropertiesAMD<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Exact]
  pub shaderEngineCount: u32,
  /// Limit Type: [Exact]
  pub shaderArraysPerEngineCount: u32,
  /// Limit Type: [Exact]
  pub computeUnitsPerShaderArray: u32,
  /// Limit Type: [Exact]
  pub simdPerComputeUnit: u32,
  /// Limit Type: [Exact]
  pub wavefrontsPerSimd: u32,
  /// Limit Type: [Max]
  pub wavefrontSize: u32,
  /// Limit Type: [Exact]
  pub sgprsPerSimd: u32,
  /// Limit Type: [Min]
  pub minSgprAllocation: u32,
  /// Limit Type: [Max]
  pub maxSgprAllocation: u32,
  /// Limit Type: [Min, Mul]
  pub sgprAllocationGranularity: u32,
  /// Limit Type: [Exact]
  pub vgprsPerSimd: u32,
  /// Limit Type: [Min]
  pub minVgprAllocation: u32,
  /// Limit Type: [Max]
  pub maxVgprAllocation: u32,
  /// Limit Type: [Min, Mul]
  pub vgprAllocationGranularity: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_AMD_shader_core_properties")]
unsafe impl<'a> Send for VkPhysicalDeviceShaderCorePropertiesAMD<'a> {}
#[cfg(feature = "VK_AMD_shader_core_properties")]
unsafe impl<'a> Sync for VkPhysicalDeviceShaderCorePropertiesAMD<'a> {}
#[cfg(all(
  feature = "VK_AMD_shader_core_properties",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceShaderCorePropertiesAMD<'child>
{
}
#[cfg(feature = "VK_AMD_shader_core_properties")]
impl<'a> VkPhysicalDeviceShaderCorePropertiesAMD<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD,
    pNext: core::ptr::null_mut(),
    shaderEngineCount: 0,
    shaderArraysPerEngineCount: 0,
    computeUnitsPerShaderArray: 0,
    simdPerComputeUnit: 0,
    wavefrontsPerSimd: 0,
    wavefrontSize: 0,
    sgprsPerSimd: 0,
    minSgprAllocation: 0,
    maxSgprAllocation: 0,
    sgprAllocationGranularity: 0,
    vgprsPerSimd: 0,
    minVgprAllocation: 0,
    maxVgprAllocation: 0,
    vgprAllocationGranularity: 0,
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
  pub const fn with_shaderEngineCount(mut self, val: u32) -> Self {
    self.shaderEngineCount = val;
    self
  }
  #[inline]
  pub const fn with_shaderArraysPerEngineCount(mut self, val: u32) -> Self {
    self.shaderArraysPerEngineCount = val;
    self
  }
  #[inline]
  pub const fn with_computeUnitsPerShaderArray(mut self, val: u32) -> Self {
    self.computeUnitsPerShaderArray = val;
    self
  }
  #[inline]
  pub const fn with_simdPerComputeUnit(mut self, val: u32) -> Self {
    self.simdPerComputeUnit = val;
    self
  }
  #[inline]
  pub const fn with_wavefrontsPerSimd(mut self, val: u32) -> Self {
    self.wavefrontsPerSimd = val;
    self
  }
  #[inline]
  pub const fn with_wavefrontSize(mut self, val: u32) -> Self {
    self.wavefrontSize = val;
    self
  }
  #[inline]
  pub const fn with_sgprsPerSimd(mut self, val: u32) -> Self {
    self.sgprsPerSimd = val;
    self
  }
  #[inline]
  pub const fn with_minSgprAllocation(mut self, val: u32) -> Self {
    self.minSgprAllocation = val;
    self
  }
  #[inline]
  pub const fn with_maxSgprAllocation(mut self, val: u32) -> Self {
    self.maxSgprAllocation = val;
    self
  }
  #[inline]
  pub const fn with_sgprAllocationGranularity(mut self, val: u32) -> Self {
    self.sgprAllocationGranularity = val;
    self
  }
  #[inline]
  pub const fn with_vgprsPerSimd(mut self, val: u32) -> Self {
    self.vgprsPerSimd = val;
    self
  }
  #[inline]
  pub const fn with_minVgprAllocation(mut self, val: u32) -> Self {
    self.minVgprAllocation = val;
    self
  }
  #[inline]
  pub const fn with_maxVgprAllocation(mut self, val: u32) -> Self {
    self.maxVgprAllocation = val;
    self
  }
  #[inline]
  pub const fn with_vgprAllocationGranularity(mut self, val: u32) -> Self {
    self.vgprAllocationGranularity = val;
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
/// [VkShaderCorePropertiesFlagsAMD](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderCorePropertiesFlagsAMD.html)
#[cfg(feature = "VK_AMD_shader_core_properties2")]
pub type VkShaderCorePropertiesFlagsAMD = VkShaderCorePropertiesFlagBitsAMD;
/// [VkPhysicalDeviceShaderCoreProperties2AMD](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderCoreProperties2AMD.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_AMD_shader_core_properties2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShaderCoreProperties2AMD<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Bitmask]
  pub shaderCoreFeatures: VkShaderCorePropertiesFlagsAMD,
  /// Limit Type: [Max]
  pub activeComputeUnitCount: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_AMD_shader_core_properties2")]
unsafe impl<'a> Send for VkPhysicalDeviceShaderCoreProperties2AMD<'a> {}
#[cfg(feature = "VK_AMD_shader_core_properties2")]
unsafe impl<'a> Sync for VkPhysicalDeviceShaderCoreProperties2AMD<'a> {}
#[cfg(all(
  feature = "VK_AMD_shader_core_properties2",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceShaderCoreProperties2AMD<'child>
{
}
#[cfg(feature = "VK_AMD_shader_core_properties2")]
impl<'a> VkPhysicalDeviceShaderCoreProperties2AMD<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD,
    pNext: core::ptr::null_mut(),
    shaderCoreFeatures: VkShaderCorePropertiesFlagBitsAMD(0),
    activeComputeUnitCount: 0,
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
  pub const fn with_shaderCoreFeatures(mut self, val: VkShaderCorePropertiesFlagsAMD) -> Self {
    self.shaderCoreFeatures = val;
    self
  }
  #[inline]
  pub const fn with_activeComputeUnitCount(mut self, val: u32) -> Self {
    self.activeComputeUnitCount = val;
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
/// [VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_AMD_shader_early_and_late_fragment_tests")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_FEATURES_AMD
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub shaderEarlyAndLateFragmentTests: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_AMD_shader_early_and_late_fragment_tests")]
unsafe impl<'a> Send for VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD<'a> {}
#[cfg(feature = "VK_AMD_shader_early_and_late_fragment_tests")]
unsafe impl<'a> Sync for VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD<'a> {}
#[cfg(all(
  feature = "VK_AMD_shader_early_and_late_fragment_tests",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD<'child>
{
}
#[cfg(all(
  feature = "VK_AMD_shader_early_and_late_fragment_tests",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD<'child>
{
}
#[cfg(feature = "VK_AMD_shader_early_and_late_fragment_tests")]
impl<'a> VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_FEATURES_AMD,
    pNext: core::ptr::null_mut(),
    shaderEarlyAndLateFragmentTests: 0,
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
  pub const fn with_shaderEarlyAndLateFragmentTests(mut self, val: VkBool32) -> Self {
    self.shaderEarlyAndLateFragmentTests = val;
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
/// [VkShaderResourceUsageAMD](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderResourceUsageAMD.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_AMD_shader_info")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkShaderResourceUsageAMD {
  pub numUsedVgprs: u32,
  pub numUsedSgprs: u32,
  pub ldsSizePerLocalWorkGroup: u32,
  pub ldsUsageSizeInBytes: usize,
  pub scratchMemUsageInBytes: usize,
}
#[cfg(feature = "VK_AMD_shader_info")]
unsafe impl Send for VkShaderResourceUsageAMD {}
#[cfg(feature = "VK_AMD_shader_info")]
unsafe impl Sync for VkShaderResourceUsageAMD {}
#[cfg(feature = "VK_AMD_shader_info")]
impl VkShaderResourceUsageAMD {
  pub const DEFAULT: Self = Self {
    numUsedVgprs: 0,
    numUsedSgprs: 0,
    ldsSizePerLocalWorkGroup: 0,
    ldsUsageSizeInBytes: 0,
    scratchMemUsageInBytes: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_numUsedVgprs(mut self, val: u32) -> Self {
    self.numUsedVgprs = val;
    self
  }
  #[inline]
  pub const fn with_numUsedSgprs(mut self, val: u32) -> Self {
    self.numUsedSgprs = val;
    self
  }
  #[inline]
  pub const fn with_ldsSizePerLocalWorkGroup(mut self, val: u32) -> Self {
    self.ldsSizePerLocalWorkGroup = val;
    self
  }
  #[inline]
  pub const fn with_ldsUsageSizeInBytes(mut self, val: usize) -> Self {
    self.ldsUsageSizeInBytes = val;
    self
  }
  #[inline]
  pub const fn with_scratchMemUsageInBytes(mut self, val: usize) -> Self {
    self.scratchMemUsageInBytes = val;
    self
  }
}
/// [VkShaderStatisticsInfoAMD](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderStatisticsInfoAMD.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_AMD_shader_info")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkShaderStatisticsInfoAMD {
  pub shaderStageMask: VkShaderStageFlags,
  pub resourceUsage: VkShaderResourceUsageAMD,
  pub numPhysicalVgprs: u32,
  pub numPhysicalSgprs: u32,
  pub numAvailableVgprs: u32,
  pub numAvailableSgprs: u32,
  pub computeWorkGroupSize: [u32; 3],
}
#[cfg(feature = "VK_AMD_shader_info")]
unsafe impl Send for VkShaderStatisticsInfoAMD {}
#[cfg(feature = "VK_AMD_shader_info")]
unsafe impl Sync for VkShaderStatisticsInfoAMD {}
#[cfg(feature = "VK_AMD_shader_info")]
impl VkShaderStatisticsInfoAMD {
  pub const DEFAULT: Self = Self {
    shaderStageMask: VkShaderStageFlagBits(0),
    resourceUsage: VkShaderResourceUsageAMD::DEFAULT,
    numPhysicalVgprs: 0,
    numPhysicalSgprs: 0,
    numAvailableVgprs: 0,
    numAvailableSgprs: 0,
    computeWorkGroupSize: [0u32; 3],
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_shaderStageMask(mut self, val: VkShaderStageFlags) -> Self {
    self.shaderStageMask = val;
    self
  }
  #[inline]
  pub const fn with_resourceUsage(mut self, val: VkShaderResourceUsageAMD) -> Self {
    self.resourceUsage = val;
    self
  }
  #[inline]
  pub const fn with_numPhysicalVgprs(mut self, val: u32) -> Self {
    self.numPhysicalVgprs = val;
    self
  }
  #[inline]
  pub const fn with_numPhysicalSgprs(mut self, val: u32) -> Self {
    self.numPhysicalSgprs = val;
    self
  }
  #[inline]
  pub const fn with_numAvailableVgprs(mut self, val: u32) -> Self {
    self.numAvailableVgprs = val;
    self
  }
  #[inline]
  pub const fn with_numAvailableSgprs(mut self, val: u32) -> Self {
    self.numAvailableSgprs = val;
    self
  }
  #[inline]
  pub const fn with_computeWorkGroupSize(mut self, val: [u32; 3]) -> Self {
    self.computeWorkGroupSize = val;
    self
  }
}
/// [VkTextureLODGatherFormatPropertiesAMD](https://docs.vulkan.org/refpages/latest/refpages/source/VkTextureLODGatherFormatPropertiesAMD.html)
///
/// *Note: This is a **returned only** struct.*
///
/// **Extends:** VkImageFormatProperties2.
#[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkTextureLODGatherFormatPropertiesAMD<'a> {
  /// Values: VK_STRUCTURE_TYPE_TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub supportsTextureGatherLODBiasAMD: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
unsafe impl<'a> Send for VkTextureLODGatherFormatPropertiesAMD<'a> {}
#[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
unsafe impl<'a> Sync for VkTextureLODGatherFormatPropertiesAMD<'a> {}
#[cfg(all(
  feature = "VK_AMD_texture_gather_bias_lod",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkImageFormatProperties2<'root>>
  for VkTextureLODGatherFormatPropertiesAMD<'child>
{
}
#[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
impl<'a> VkTextureLODGatherFormatPropertiesAMD<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD,
    pNext: core::ptr::null_mut(),
    supportsTextureGatherLODBiasAMD: 0,
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
  pub const fn with_supportsTextureGatherLODBiasAMD(mut self, val: VkBool32) -> Self {
    self.supportsTextureGatherLODBiasAMD = val;
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
