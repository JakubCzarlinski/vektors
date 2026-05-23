#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(feature = "VK_SEC_throttle_hint")]
use crate::enums::VkThrottleHintTypeSEC;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBool32;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkFlags;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceFeatures2;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkSubmitInfo;
use core::ffi::c_void;
/// [VkPhysicalDeviceAmigoProfilingFeaturesSEC](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceAmigoProfilingFeaturesSEC.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_SEC_amigo_profiling")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceAmigoProfilingFeaturesSEC<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_AMIGO_PROFILING_FEATURES_SEC
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub amigoProfiling: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_SEC_amigo_profiling")]
unsafe impl<'a> Send for VkPhysicalDeviceAmigoProfilingFeaturesSEC<'a> {}
#[cfg(feature = "VK_SEC_amigo_profiling")]
unsafe impl<'a> Sync for VkPhysicalDeviceAmigoProfilingFeaturesSEC<'a> {}
#[cfg(all(feature = "VK_SEC_amigo_profiling", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceAmigoProfilingFeaturesSEC<'child>
{
}
#[cfg(all(feature = "VK_SEC_amigo_profiling", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceAmigoProfilingFeaturesSEC<'child>
{
}
#[cfg(feature = "VK_SEC_amigo_profiling")]
impl<'a> VkPhysicalDeviceAmigoProfilingFeaturesSEC<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_AMIGO_PROFILING_FEATURES_SEC,
    pNext: core::ptr::null_mut(),
    amigoProfiling: 0,
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
  pub const fn with_amigoProfiling(mut self, val: VkBool32) -> Self {
    self.amigoProfiling = val;
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
/// [VkAmigoProfilingSubmitInfoSEC](https://docs.vulkan.org/refpages/latest/refpages/source/VkAmigoProfilingSubmitInfoSEC.html)
///
/// **Extends:** VkSubmitInfo.
#[cfg(feature = "VK_SEC_amigo_profiling")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAmigoProfilingSubmitInfoSEC<'a> {
  /// Values: VK_STRUCTURE_TYPE_AMIGO_PROFILING_SUBMIT_INFO_SEC
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub firstDrawTimestamp: u64,
  pub swapBufferTimestamp: u64,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_SEC_amigo_profiling")]
unsafe impl<'a> Send for VkAmigoProfilingSubmitInfoSEC<'a> {}
#[cfg(feature = "VK_SEC_amigo_profiling")]
unsafe impl<'a> Sync for VkAmigoProfilingSubmitInfoSEC<'a> {}
#[cfg(all(feature = "VK_SEC_amigo_profiling", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkSubmitInfo<'root>>
  for VkAmigoProfilingSubmitInfoSEC<'child>
{
}
#[cfg(feature = "VK_SEC_amigo_profiling")]
impl<'a> VkAmigoProfilingSubmitInfoSEC<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_AMIGO_PROFILING_SUBMIT_INFO_SEC,
    pNext: core::ptr::null(),
    firstDrawTimestamp: 0,
    swapBufferTimestamp: 0,
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
  pub const fn with_firstDrawTimestamp(mut self, val: u64) -> Self {
    self.firstDrawTimestamp = val;
    self
  }
  #[inline]
  pub const fn with_swapBufferTimestamp(mut self, val: u64) -> Self {
    self.swapBufferTimestamp = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSubmitInfo<'root, T: VkPNextExtends<VkSubmitInfo<'root>>>(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDevicePipelineCacheIncrementalModeFeaturesSEC](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePipelineCacheIncrementalModeFeaturesSEC.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_SEC_pipeline_cache_incremental_mode")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDevicePipelineCacheIncrementalModeFeaturesSEC<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_CACHE_INCREMENTAL_MODE_FEATURES_SEC
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub pipelineCacheIncrementalMode: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_SEC_pipeline_cache_incremental_mode")]
unsafe impl<'a> Send for VkPhysicalDevicePipelineCacheIncrementalModeFeaturesSEC<'a> {}
#[cfg(feature = "VK_SEC_pipeline_cache_incremental_mode")]
unsafe impl<'a> Sync for VkPhysicalDevicePipelineCacheIncrementalModeFeaturesSEC<'a> {}
#[cfg(all(
  feature = "VK_SEC_pipeline_cache_incremental_mode",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDevicePipelineCacheIncrementalModeFeaturesSEC<'child>
{
}
#[cfg(all(
  feature = "VK_SEC_pipeline_cache_incremental_mode",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDevicePipelineCacheIncrementalModeFeaturesSEC<'child>
{
}
#[cfg(feature = "VK_SEC_pipeline_cache_incremental_mode")]
impl<'a> VkPhysicalDevicePipelineCacheIncrementalModeFeaturesSEC<'a> {
  pub const DEFAULT: Self = Self {
        sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_CACHE_INCREMENTAL_MODE_FEATURES_SEC,
        pNext: core::ptr::null_mut(),
        pipelineCacheIncrementalMode: 0,
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
  pub const fn with_pipelineCacheIncrementalMode(mut self, val: VkBool32) -> Self {
    self.pipelineCacheIncrementalMode = val;
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
/// [VkPhysicalDeviceThrottleHintFeaturesSEC](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceThrottleHintFeaturesSEC.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_SEC_throttle_hint")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceThrottleHintFeaturesSEC<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_THROTTLE_HINT_FEATURES_SEC
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub throttleHint: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_SEC_throttle_hint")]
unsafe impl<'a> Send for VkPhysicalDeviceThrottleHintFeaturesSEC<'a> {}
#[cfg(feature = "VK_SEC_throttle_hint")]
unsafe impl<'a> Sync for VkPhysicalDeviceThrottleHintFeaturesSEC<'a> {}
#[cfg(all(feature = "VK_SEC_throttle_hint", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceThrottleHintFeaturesSEC<'child>
{
}
#[cfg(all(feature = "VK_SEC_throttle_hint", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceThrottleHintFeaturesSEC<'child>
{
}
#[cfg(feature = "VK_SEC_throttle_hint")]
impl<'a> VkPhysicalDeviceThrottleHintFeaturesSEC<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_THROTTLE_HINT_FEATURES_SEC,
    pNext: core::ptr::null_mut(),
    throttleHint: 0,
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
  pub const fn with_throttleHint(mut self, val: VkBool32) -> Self {
    self.throttleHint = val;
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
/// [VkThrottleHintSubmitInfoSEC](https://docs.vulkan.org/refpages/latest/refpages/source/VkThrottleHintSubmitInfoSEC.html)
///
/// **Extends:** VkSubmitInfo.
#[cfg(feature = "VK_SEC_throttle_hint")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkThrottleHintSubmitInfoSEC<'a> {
  /// Values: VK_STRUCTURE_TYPE_THROTTLE_HINT_SUBMIT_INFO_SEC
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub throttleHint: VkThrottleHintTypeSEC,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_SEC_throttle_hint")]
unsafe impl<'a> Send for VkThrottleHintSubmitInfoSEC<'a> {}
#[cfg(feature = "VK_SEC_throttle_hint")]
unsafe impl<'a> Sync for VkThrottleHintSubmitInfoSEC<'a> {}
#[cfg(all(feature = "VK_SEC_throttle_hint", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkSubmitInfo<'root>>
  for VkThrottleHintSubmitInfoSEC<'child>
{
}
#[cfg(feature = "VK_SEC_throttle_hint")]
impl<'a> VkThrottleHintSubmitInfoSEC<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_THROTTLE_HINT_SUBMIT_INFO_SEC,
    pNext: core::ptr::null(),
    throttleHint: VkThrottleHintTypeSEC(0),
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
  pub const fn with_throttleHint(mut self, val: VkThrottleHintTypeSEC) -> Self {
    self.throttleHint = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSubmitInfo<'root, T: VkPNextExtends<VkSubmitInfo<'root>>>(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [ubm_device](https://docs.vulkan.org/refpages/latest/refpages/source/ubm_device.html)
/// Opaque platform handle - always used as a raw pointer.
#[cfg(feature = "VK_SEC_ubm_surface")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ubm_device(pub *mut c_void);
#[cfg(feature = "VK_SEC_ubm_surface")]
impl ubm_device {
  pub const NULL: Self = Self(core::ptr::null_mut());
}
#[cfg(feature = "VK_SEC_ubm_surface")]
unsafe impl Send for ubm_device {}
#[cfg(feature = "VK_SEC_ubm_surface")]
unsafe impl Sync for ubm_device {}
/// [ubm_surface](https://docs.vulkan.org/refpages/latest/refpages/source/ubm_surface.html)
/// Opaque platform handle - always used as a raw pointer.
#[cfg(feature = "VK_SEC_ubm_surface")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ubm_surface(pub *mut c_void);
#[cfg(feature = "VK_SEC_ubm_surface")]
impl ubm_surface {
  pub const NULL: Self = Self(core::ptr::null_mut());
}
#[cfg(feature = "VK_SEC_ubm_surface")]
unsafe impl Send for ubm_surface {}
#[cfg(feature = "VK_SEC_ubm_surface")]
unsafe impl Sync for ubm_surface {}
/// [VkUbmSurfaceCreateFlagsSEC](https://docs.vulkan.org/refpages/latest/refpages/source/VkUbmSurfaceCreateFlagsSEC.html)
#[cfg(feature = "VK_SEC_ubm_surface")]
pub type VkUbmSurfaceCreateFlagsSEC = VkFlags;
/// [VkUbmSurfaceCreateInfoSEC](https://docs.vulkan.org/refpages/latest/refpages/source/VkUbmSurfaceCreateInfoSEC.html)
#[cfg(feature = "VK_SEC_ubm_surface")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkUbmSurfaceCreateInfoSEC<'a> {
  /// Values: VK_STRUCTURE_TYPE_UBM_SURFACE_CREATE_INFO_SEC
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkUbmSurfaceCreateFlagsSEC,
  /// No Auto-Validity
  pub device: *mut ubm_device,
  /// No Auto-Validity
  pub surface: *mut ubm_surface,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_SEC_ubm_surface")]
unsafe impl<'a> Send for VkUbmSurfaceCreateInfoSEC<'a> {}
#[cfg(feature = "VK_SEC_ubm_surface")]
unsafe impl<'a> Sync for VkUbmSurfaceCreateInfoSEC<'a> {}
#[cfg(feature = "VK_SEC_ubm_surface")]
impl<'a> VkUbmSurfaceCreateInfoSEC<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_UBM_SURFACE_CREATE_INFO_SEC,
    pNext: core::ptr::null(),
    flags: 0,
    device: core::ptr::null_mut(),
    surface: core::ptr::null_mut(),
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
  pub const fn with_flags(mut self, val: VkUbmSurfaceCreateFlagsSEC) -> Self {
    self.flags = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_device(mut self, val: *mut ubm_device) -> Self {
    self.device = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_surface(mut self, val: *mut ubm_surface) -> Self {
    self.surface = val;
    self
  }
  #[cfg(feature = "VK_SEC_ubm_surface")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkUbmSurfaceCreateInfoSEC<
    'root,
    T: VkPNextExtends<VkUbmSurfaceCreateInfoSEC<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
