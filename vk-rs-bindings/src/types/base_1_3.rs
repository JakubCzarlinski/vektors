use crate::consts::VK_MAX_DESCRIPTION_SIZE;
use crate::consts::VK_MAX_EXTENSION_NAME_SIZE;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_3",
  feature = "VK_KHR_video_decode_queue",
  feature = "VK_EXT_descriptor_heap",
  feature = "VK_KHR_video_encode_queue",
  feature = "VK_QCOM_tile_shading",
  feature = "VK_KHR_synchronization2",
  feature = "VK_EXT_descriptor_buffer",
  feature = "VK_HUAWEI_invocation_mask",
  feature = "VK_EXT_opacity_micromap",
  feature = "VK_NV_optical_flow",
  feature = "VK_EXT_memory_decompression"
))]
use crate::enums::VkAccessFlagBits2;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkDependencyFlagBits;
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
  feature = "VK_KHR_video_encode_quantization_map"
))]
use crate::enums::VkFormatFeatureFlagBits2;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageAspectFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageLayout;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_3",
  feature = "VK_KHR_video_decode_queue",
  feature = "VK_KHR_video_encode_queue",
  feature = "VK_KHR_synchronization2",
  feature = "VK_HUAWEI_subpass_shading",
  feature = "VK_HUAWEI_invocation_mask",
  feature = "VK_EXT_opacity_micromap",
  feature = "VK_HUAWEI_cluster_culling_shader",
  feature = "VK_NV_optical_flow",
  feature = "VK_NV_cooperative_vector",
  feature = "VK_KHR_copy_memory_indirect",
  feature = "VK_EXT_memory_decompression"
))]
use crate::enums::VkPipelineStageFlagBits2;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkShaderStageFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(any(feature = "VK_BASE_VERSION_1_3", feature = "VK_KHR_synchronization2"))]
use crate::enums::VkSubmitFlagBits;
#[cfg(any(feature = "VK_BASE_VERSION_1_3", feature = "VK_EXT_tooling_info"))]
use crate::enums::VkToolPurposeFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBool32;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBuffer;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBufferCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkCommandBuffer;
#[cfg(feature = "VK_QCOM_rotated_copy_commands")]
use crate::types::VkCopyCommandTransformInfoQCOM;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDependencyFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceSize;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkExtent3D;
#[cfg(feature = "VK_EXT_external_memory_acquire_unmodified")]
use crate::types::VkExternalMemoryAcquireUnmodifiedEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkFlags;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkFormatProperties2;
#[cfg(feature = "VK_EXT_frame_boundary")]
use crate::types::VkFrameBoundaryEXT;
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_frame_boundary"))]
use crate::types::VkFrameBoundaryTensorsARM;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImage;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageSubresourceLayers;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageSubresourceRange;
#[cfg(feature = "VK_NV_low_latency2")]
use crate::types::VkLatencySubmissionPresentIdNV;
#[cfg(feature = "VK_KHR_maintenance8")]
use crate::types::VkMemoryBarrierAccessFlags3KHR;
#[cfg(feature = "VK_KHR_device_address_commands")]
use crate::types::VkMemoryRangeBarriersInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkOffset3D;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::types::VkPerformanceQuerySubmitInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceFeatures2;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceProperties2;
#[cfg(feature = "VK_ARM_render_pass_striped")]
use crate::types::VkRenderPassStripeSubmitInfoARM;
#[cfg(feature = "VK_EXT_sample_locations")]
use crate::types::VkSampleLocationsInfoEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkSemaphore;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkShaderStageFlags;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
use crate::types::VkSubpassDependency2;
#[cfg(feature = "VK_ARM_tensors")]
use crate::types::VkTensorDependencyInfoARM;
#[cfg(feature = "VK_ARM_tensors")]
use crate::types::VkTensorMemoryBarrierARM;
#[cfg(feature = "VK_KHR_win32_keyed_mutex")]
use crate::types::VkWin32KeyedMutexAcquireReleaseInfoKHR;
#[cfg(feature = "VK_NV_win32_keyed_mutex")]
use crate::types::VkWin32KeyedMutexAcquireReleaseInfoNV;
use core::ffi::{c_char, c_void};
/// [VkPrivateDataSlotCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkPrivateDataSlotCreateFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_3")]
pub type VkPrivateDataSlotCreateFlags = VkFlags;
/// [VkAccessFlags2](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccessFlags2.html)
#[cfg(feature = "VK_BASE_VERSION_1_3")]
pub type VkAccessFlags2 = VkAccessFlagBits2;
/// [VkPipelineStageFlags2](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineStageFlags2.html)
#[cfg(feature = "VK_BASE_VERSION_1_3")]
pub type VkPipelineStageFlags2 = VkPipelineStageFlagBits2;
/// [VkFormatFeatureFlags2](https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatFeatureFlags2.html)
#[cfg(feature = "VK_BASE_VERSION_1_3")]
pub type VkFormatFeatureFlags2 = VkFormatFeatureFlagBits2;
/// [VkToolPurposeFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkToolPurposeFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_3")]
pub type VkToolPurposeFlags = VkToolPurposeFlagBits;
/// [VkSubmitFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkSubmitFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_3")]
pub type VkSubmitFlags = VkSubmitFlagBits;
/// [VkPrivateDataSlot](https://docs.vulkan.org/refpages/latest/refpages/source/VkPrivateDataSlot.html)
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkPrivateDataSlot(pub *mut c_void);
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl VkPrivateDataSlot {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl Default for VkPrivateDataSlot {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl Send for VkPrivateDataSlot {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl Sync for VkPrivateDataSlot {}
/// [VkDevicePrivateDataCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkDevicePrivateDataCreateInfo.html)
///
/// **Extends:** VkDeviceCreateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDevicePrivateDataCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_DEVICE_PRIVATE_DATA_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub privateDataSlotRequestCount: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Send for VkDevicePrivateDataCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Sync for VkDevicePrivateDataCreateInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_3", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkDevicePrivateDataCreateInfo<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl<'a> VkDevicePrivateDataCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DEVICE_PRIVATE_DATA_CREATE_INFO,
    pNext: core::ptr::null(),
    privateDataSlotRequestCount: 0,
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
  pub const fn with_privateDataSlotRequestCount(mut self, val: u32) -> Self {
    self.privateDataSlotRequestCount = val;
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
/// [VkPrivateDataSlotCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkPrivateDataSlotCreateInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPrivateDataSlotCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_PRIVATE_DATA_SLOT_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub flags: VkPrivateDataSlotCreateFlags,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Send for VkPrivateDataSlotCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Sync for VkPrivateDataSlotCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl<'a> VkPrivateDataSlotCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PRIVATE_DATA_SLOT_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: 0,
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
  pub const fn with_flags(mut self, val: VkPrivateDataSlotCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPrivateDataSlotCreateInfo<
    'root,
    T: VkPNextExtends<VkPrivateDataSlotCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDevicePrivateDataFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePrivateDataFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDevicePrivateDataFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub privateData: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Send for VkPhysicalDevicePrivateDataFeatures<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Sync for VkPhysicalDevicePrivateDataFeatures<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_3", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDevicePrivateDataFeatures<'child>
{
}
#[cfg(all(feature = "VK_BASE_VERSION_1_3", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDevicePrivateDataFeatures<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl<'a> VkPhysicalDevicePrivateDataFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES,
    pNext: core::ptr::null_mut(),
    privateData: 0,
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
  pub const fn with_privateData(mut self, val: VkBool32) -> Self {
    self.privateData = val;
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
/// [VkDeviceBufferMemoryRequirements](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceBufferMemoryRequirements.html)
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDeviceBufferMemoryRequirements<'a> {
  /// Values: VK_STRUCTURE_TYPE_DEVICE_BUFFER_MEMORY_REQUIREMENTS
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub pCreateInfo: *const VkBufferCreateInfo<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Send for VkDeviceBufferMemoryRequirements<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Sync for VkDeviceBufferMemoryRequirements<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl<'a> VkDeviceBufferMemoryRequirements<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DEVICE_BUFFER_MEMORY_REQUIREMENTS,
    pNext: core::ptr::null(),
    pCreateInfo: core::ptr::null(),
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
  pub const fn with_pCreateInfo(mut self, val: &'a VkBufferCreateInfo<'a>) -> Self {
    self.pCreateInfo = val as *const VkBufferCreateInfo<'a>;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDeviceBufferMemoryRequirements<
    'root,
    T: VkPNextExtends<VkDeviceBufferMemoryRequirements<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkDeviceImageMemoryRequirements](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceImageMemoryRequirements.html)
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDeviceImageMemoryRequirements<'a> {
  /// Values: VK_STRUCTURE_TYPE_DEVICE_IMAGE_MEMORY_REQUIREMENTS
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub pCreateInfo: *const VkImageCreateInfo<'a>,
  /// Optional: true,  No Auto-Validity
  pub planeAspect: VkImageAspectFlagBits,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Send for VkDeviceImageMemoryRequirements<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Sync for VkDeviceImageMemoryRequirements<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl<'a> VkDeviceImageMemoryRequirements<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DEVICE_IMAGE_MEMORY_REQUIREMENTS,
    pNext: core::ptr::null(),
    pCreateInfo: core::ptr::null(),
    planeAspect: VkImageAspectFlagBits(0),
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
  pub const fn with_pCreateInfo(mut self, val: &'a VkImageCreateInfo<'a>) -> Self {
    self.pCreateInfo = val as *const VkImageCreateInfo<'a>;
    self
  }
  #[inline]
  pub const fn with_planeAspect(mut self, val: VkImageAspectFlagBits) -> Self {
    self.planeAspect = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDeviceImageMemoryRequirements<
    'root,
    T: VkPNextExtends<VkDeviceImageMemoryRequirements<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceMaintenance4Features](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMaintenance4Features.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceMaintenance4Features<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub maintenance4: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Send for VkPhysicalDeviceMaintenance4Features<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Sync for VkPhysicalDeviceMaintenance4Features<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_3", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceMaintenance4Features<'child>
{
}
#[cfg(all(feature = "VK_BASE_VERSION_1_3", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceMaintenance4Features<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl<'a> VkPhysicalDeviceMaintenance4Features<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES,
    pNext: core::ptr::null_mut(),
    maintenance4: 0,
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
  pub const fn with_maintenance4(mut self, val: VkBool32) -> Self {
    self.maintenance4 = val;
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
/// [VkPhysicalDeviceMaintenance4Properties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMaintenance4Properties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceMaintenance4Properties<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub maxBufferSize: VkDeviceSize,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Send for VkPhysicalDeviceMaintenance4Properties<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Sync for VkPhysicalDeviceMaintenance4Properties<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_3", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceMaintenance4Properties<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl<'a> VkPhysicalDeviceMaintenance4Properties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES,
    pNext: core::ptr::null_mut(),
    maxBufferSize: 0,
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
  pub const fn with_maxBufferSize(mut self, val: VkDeviceSize) -> Self {
    self.maxBufferSize = val;
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
/// [VkPhysicalDeviceTextureCompressionASTCHDRFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceTextureCompressionASTCHDRFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceTextureCompressionASTCHDRFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub textureCompressionASTC_HDR: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Send for VkPhysicalDeviceTextureCompressionASTCHDRFeatures<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Sync for VkPhysicalDeviceTextureCompressionASTCHDRFeatures<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_3", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceTextureCompressionASTCHDRFeatures<'child>
{
}
#[cfg(all(feature = "VK_BASE_VERSION_1_3", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceTextureCompressionASTCHDRFeatures<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl<'a> VkPhysicalDeviceTextureCompressionASTCHDRFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES,
    pNext: core::ptr::null_mut(),
    textureCompressionASTC_HDR: 0,
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
  pub const fn with_textureCompressionASTC_HDR(mut self, val: VkBool32) -> Self {
    self.textureCompressionASTC_HDR = val;
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
/// [VkPhysicalDeviceVulkan13Features](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVulkan13Features.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceVulkan13Features<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub robustImageAccess: VkBool32,
  pub inlineUniformBlock: VkBool32,
  pub descriptorBindingInlineUniformBlockUpdateAfterBind: VkBool32,
  pub pipelineCreationCacheControl: VkBool32,
  pub privateData: VkBool32,
  pub shaderDemoteToHelperInvocation: VkBool32,
  pub shaderTerminateInvocation: VkBool32,
  pub subgroupSizeControl: VkBool32,
  pub computeFullSubgroups: VkBool32,
  pub synchronization2: VkBool32,
  pub textureCompressionASTC_HDR: VkBool32,
  pub shaderZeroInitializeWorkgroupMemory: VkBool32,
  pub dynamicRendering: VkBool32,
  pub shaderIntegerDotProduct: VkBool32,
  pub maintenance4: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Send for VkPhysicalDeviceVulkan13Features<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Sync for VkPhysicalDeviceVulkan13Features<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_3", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceVulkan13Features<'child>
{
}
#[cfg(all(feature = "VK_BASE_VERSION_1_3", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceVulkan13Features<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl<'a> VkPhysicalDeviceVulkan13Features<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_FEATURES,
    pNext: core::ptr::null_mut(),
    robustImageAccess: 0,
    inlineUniformBlock: 0,
    descriptorBindingInlineUniformBlockUpdateAfterBind: 0,
    pipelineCreationCacheControl: 0,
    privateData: 0,
    shaderDemoteToHelperInvocation: 0,
    shaderTerminateInvocation: 0,
    subgroupSizeControl: 0,
    computeFullSubgroups: 0,
    synchronization2: 0,
    textureCompressionASTC_HDR: 0,
    shaderZeroInitializeWorkgroupMemory: 0,
    dynamicRendering: 0,
    shaderIntegerDotProduct: 0,
    maintenance4: 0,
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
  pub const fn with_robustImageAccess(mut self, val: VkBool32) -> Self {
    self.robustImageAccess = val;
    self
  }
  #[inline]
  pub const fn with_inlineUniformBlock(mut self, val: VkBool32) -> Self {
    self.inlineUniformBlock = val;
    self
  }
  #[inline]
  pub const fn with_descriptorBindingInlineUniformBlockUpdateAfterBind(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.descriptorBindingInlineUniformBlockUpdateAfterBind = val;
    self
  }
  #[inline]
  pub const fn with_pipelineCreationCacheControl(mut self, val: VkBool32) -> Self {
    self.pipelineCreationCacheControl = val;
    self
  }
  #[inline]
  pub const fn with_privateData(mut self, val: VkBool32) -> Self {
    self.privateData = val;
    self
  }
  #[inline]
  pub const fn with_shaderDemoteToHelperInvocation(mut self, val: VkBool32) -> Self {
    self.shaderDemoteToHelperInvocation = val;
    self
  }
  #[inline]
  pub const fn with_shaderTerminateInvocation(mut self, val: VkBool32) -> Self {
    self.shaderTerminateInvocation = val;
    self
  }
  #[inline]
  pub const fn with_subgroupSizeControl(mut self, val: VkBool32) -> Self {
    self.subgroupSizeControl = val;
    self
  }
  #[inline]
  pub const fn with_computeFullSubgroups(mut self, val: VkBool32) -> Self {
    self.computeFullSubgroups = val;
    self
  }
  #[inline]
  pub const fn with_synchronization2(mut self, val: VkBool32) -> Self {
    self.synchronization2 = val;
    self
  }
  #[inline]
  pub const fn with_textureCompressionASTC_HDR(mut self, val: VkBool32) -> Self {
    self.textureCompressionASTC_HDR = val;
    self
  }
  #[inline]
  pub const fn with_shaderZeroInitializeWorkgroupMemory(mut self, val: VkBool32) -> Self {
    self.shaderZeroInitializeWorkgroupMemory = val;
    self
  }
  #[inline]
  pub const fn with_dynamicRendering(mut self, val: VkBool32) -> Self {
    self.dynamicRendering = val;
    self
  }
  #[inline]
  pub const fn with_shaderIntegerDotProduct(mut self, val: VkBool32) -> Self {
    self.shaderIntegerDotProduct = val;
    self
  }
  #[inline]
  pub const fn with_maintenance4(mut self, val: VkBool32) -> Self {
    self.maintenance4 = val;
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
/// [VkPhysicalDeviceVulkan13Properties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVulkan13Properties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceVulkan13Properties<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Min, Pot],  No Auto-Validity
  pub minSubgroupSize: u32,
  /// Limit Type: [Max, Pot],  No Auto-Validity
  pub maxSubgroupSize: u32,
  /// Limit Type: [Max],  No Auto-Validity
  pub maxComputeWorkgroupSubgroups: u32,
  /// Limit Type: [Bitmask]
  pub requiredSubgroupSizeStages: VkShaderStageFlags,
  /// Limit Type: [Max]
  pub maxInlineUniformBlockSize: u32,
  /// Limit Type: [Max]
  pub maxPerStageDescriptorInlineUniformBlocks: u32,
  /// Limit Type: [Max]
  pub maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks: u32,
  /// Limit Type: [Max]
  pub maxDescriptorSetInlineUniformBlocks: u32,
  /// Limit Type: [Max]
  pub maxDescriptorSetUpdateAfterBindInlineUniformBlocks: u32,
  /// Limit Type: [Max]
  pub maxInlineUniformTotalSize: u32,
  /// Limit Type: [Max]
  pub integerDotProduct8BitUnsignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProduct8BitSignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProduct8BitMixedSignednessAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProduct4x8BitPackedUnsignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProduct4x8BitPackedSignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProduct4x8BitPackedMixedSignednessAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProduct16BitUnsignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProduct16BitSignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProduct16BitMixedSignednessAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProduct32BitUnsignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProduct32BitSignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProduct32BitMixedSignednessAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProduct64BitUnsignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProduct64BitSignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProduct64BitMixedSignednessAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProductAccumulatingSaturating8BitUnsignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProductAccumulatingSaturating8BitSignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProductAccumulatingSaturating16BitUnsignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProductAccumulatingSaturating16BitSignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProductAccumulatingSaturating32BitUnsignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProductAccumulatingSaturating32BitSignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProductAccumulatingSaturating64BitUnsignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProductAccumulatingSaturating64BitSignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated: VkBool32,
  /// Limit Type: [Min, Pot]
  pub storageTexelBufferOffsetAlignmentBytes: VkDeviceSize,
  /// Limit Type: [Exact]
  pub storageTexelBufferOffsetSingleTexelAlignment: VkBool32,
  /// Limit Type: [Min, Pot]
  pub uniformTexelBufferOffsetAlignmentBytes: VkDeviceSize,
  /// Limit Type: [Exact]
  pub uniformTexelBufferOffsetSingleTexelAlignment: VkBool32,
  /// Limit Type: [Max]
  pub maxBufferSize: VkDeviceSize,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Send for VkPhysicalDeviceVulkan13Properties<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Sync for VkPhysicalDeviceVulkan13Properties<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_3", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceVulkan13Properties<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl<'a> VkPhysicalDeviceVulkan13Properties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES,
    pNext: core::ptr::null_mut(),
    minSubgroupSize: 0,
    maxSubgroupSize: 0,
    maxComputeWorkgroupSubgroups: 0,
    requiredSubgroupSizeStages: VkShaderStageFlagBits(0),
    maxInlineUniformBlockSize: 0,
    maxPerStageDescriptorInlineUniformBlocks: 0,
    maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks: 0,
    maxDescriptorSetInlineUniformBlocks: 0,
    maxDescriptorSetUpdateAfterBindInlineUniformBlocks: 0,
    maxInlineUniformTotalSize: 0,
    integerDotProduct8BitUnsignedAccelerated: 0,
    integerDotProduct8BitSignedAccelerated: 0,
    integerDotProduct8BitMixedSignednessAccelerated: 0,
    integerDotProduct4x8BitPackedUnsignedAccelerated: 0,
    integerDotProduct4x8BitPackedSignedAccelerated: 0,
    integerDotProduct4x8BitPackedMixedSignednessAccelerated: 0,
    integerDotProduct16BitUnsignedAccelerated: 0,
    integerDotProduct16BitSignedAccelerated: 0,
    integerDotProduct16BitMixedSignednessAccelerated: 0,
    integerDotProduct32BitUnsignedAccelerated: 0,
    integerDotProduct32BitSignedAccelerated: 0,
    integerDotProduct32BitMixedSignednessAccelerated: 0,
    integerDotProduct64BitUnsignedAccelerated: 0,
    integerDotProduct64BitSignedAccelerated: 0,
    integerDotProduct64BitMixedSignednessAccelerated: 0,
    integerDotProductAccumulatingSaturating8BitUnsignedAccelerated: 0,
    integerDotProductAccumulatingSaturating8BitSignedAccelerated: 0,
    integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated: 0,
    integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated: 0,
    integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated: 0,
    integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated: 0,
    integerDotProductAccumulatingSaturating16BitUnsignedAccelerated: 0,
    integerDotProductAccumulatingSaturating16BitSignedAccelerated: 0,
    integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated: 0,
    integerDotProductAccumulatingSaturating32BitUnsignedAccelerated: 0,
    integerDotProductAccumulatingSaturating32BitSignedAccelerated: 0,
    integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated: 0,
    integerDotProductAccumulatingSaturating64BitUnsignedAccelerated: 0,
    integerDotProductAccumulatingSaturating64BitSignedAccelerated: 0,
    integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated: 0,
    storageTexelBufferOffsetAlignmentBytes: 0,
    storageTexelBufferOffsetSingleTexelAlignment: 0,
    uniformTexelBufferOffsetAlignmentBytes: 0,
    uniformTexelBufferOffsetSingleTexelAlignment: 0,
    maxBufferSize: 0,
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
  pub const fn with_minSubgroupSize(mut self, val: u32) -> Self {
    self.minSubgroupSize = val;
    self
  }
  #[inline]
  pub const fn with_maxSubgroupSize(mut self, val: u32) -> Self {
    self.maxSubgroupSize = val;
    self
  }
  #[inline]
  pub const fn with_maxComputeWorkgroupSubgroups(mut self, val: u32) -> Self {
    self.maxComputeWorkgroupSubgroups = val;
    self
  }
  #[inline]
  pub const fn with_requiredSubgroupSizeStages(mut self, val: VkShaderStageFlags) -> Self {
    self.requiredSubgroupSizeStages = val;
    self
  }
  #[inline]
  pub const fn with_maxInlineUniformBlockSize(mut self, val: u32) -> Self {
    self.maxInlineUniformBlockSize = val;
    self
  }
  #[inline]
  pub const fn with_maxPerStageDescriptorInlineUniformBlocks(mut self, val: u32) -> Self {
    self.maxPerStageDescriptorInlineUniformBlocks = val;
    self
  }
  #[inline]
  pub const fn with_maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks(
    mut self,
    val: u32,
  ) -> Self {
    self.maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks = val;
    self
  }
  #[inline]
  pub const fn with_maxDescriptorSetInlineUniformBlocks(mut self, val: u32) -> Self {
    self.maxDescriptorSetInlineUniformBlocks = val;
    self
  }
  #[inline]
  pub const fn with_maxDescriptorSetUpdateAfterBindInlineUniformBlocks(mut self, val: u32) -> Self {
    self.maxDescriptorSetUpdateAfterBindInlineUniformBlocks = val;
    self
  }
  #[inline]
  pub const fn with_maxInlineUniformTotalSize(mut self, val: u32) -> Self {
    self.maxInlineUniformTotalSize = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProduct8BitUnsignedAccelerated(mut self, val: VkBool32) -> Self {
    self.integerDotProduct8BitUnsignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProduct8BitSignedAccelerated(mut self, val: VkBool32) -> Self {
    self.integerDotProduct8BitSignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProduct8BitMixedSignednessAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProduct8BitMixedSignednessAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProduct4x8BitPackedUnsignedAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProduct4x8BitPackedUnsignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProduct4x8BitPackedSignedAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProduct4x8BitPackedSignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProduct4x8BitPackedMixedSignednessAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProduct4x8BitPackedMixedSignednessAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProduct16BitUnsignedAccelerated(mut self, val: VkBool32) -> Self {
    self.integerDotProduct16BitUnsignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProduct16BitSignedAccelerated(mut self, val: VkBool32) -> Self {
    self.integerDotProduct16BitSignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProduct16BitMixedSignednessAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProduct16BitMixedSignednessAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProduct32BitUnsignedAccelerated(mut self, val: VkBool32) -> Self {
    self.integerDotProduct32BitUnsignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProduct32BitSignedAccelerated(mut self, val: VkBool32) -> Self {
    self.integerDotProduct32BitSignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProduct32BitMixedSignednessAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProduct32BitMixedSignednessAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProduct64BitUnsignedAccelerated(mut self, val: VkBool32) -> Self {
    self.integerDotProduct64BitUnsignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProduct64BitSignedAccelerated(mut self, val: VkBool32) -> Self {
    self.integerDotProduct64BitSignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProduct64BitMixedSignednessAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProduct64BitMixedSignednessAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProductAccumulatingSaturating8BitUnsignedAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProductAccumulatingSaturating8BitUnsignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProductAccumulatingSaturating8BitSignedAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProductAccumulatingSaturating8BitSignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProductAccumulatingSaturating16BitUnsignedAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProductAccumulatingSaturating16BitUnsignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProductAccumulatingSaturating16BitSignedAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProductAccumulatingSaturating16BitSignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProductAccumulatingSaturating32BitUnsignedAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProductAccumulatingSaturating32BitUnsignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProductAccumulatingSaturating32BitSignedAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProductAccumulatingSaturating32BitSignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProductAccumulatingSaturating64BitUnsignedAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProductAccumulatingSaturating64BitUnsignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProductAccumulatingSaturating64BitSignedAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProductAccumulatingSaturating64BitSignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_storageTexelBufferOffsetAlignmentBytes(mut self, val: VkDeviceSize) -> Self {
    self.storageTexelBufferOffsetAlignmentBytes = val;
    self
  }
  #[inline]
  pub const fn with_storageTexelBufferOffsetSingleTexelAlignment(mut self, val: VkBool32) -> Self {
    self.storageTexelBufferOffsetSingleTexelAlignment = val;
    self
  }
  #[inline]
  pub const fn with_uniformTexelBufferOffsetAlignmentBytes(mut self, val: VkDeviceSize) -> Self {
    self.uniformTexelBufferOffsetAlignmentBytes = val;
    self
  }
  #[inline]
  pub const fn with_uniformTexelBufferOffsetSingleTexelAlignment(mut self, val: VkBool32) -> Self {
    self.uniformTexelBufferOffsetSingleTexelAlignment = val;
    self
  }
  #[inline]
  pub const fn with_maxBufferSize(mut self, val: VkDeviceSize) -> Self {
    self.maxBufferSize = val;
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
/// [VkPhysicalDeviceToolProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceToolProperties.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceToolProperties<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TOOL_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Length: null-terminated
  pub name: [c_char; VK_MAX_EXTENSION_NAME_SIZE as usize],
  /// Length: null-terminated
  pub version: [c_char; VK_MAX_EXTENSION_NAME_SIZE as usize],
  pub purposes: VkToolPurposeFlags,
  /// Length: null-terminated
  pub description: [c_char; VK_MAX_DESCRIPTION_SIZE as usize],
  /// Length: null-terminated
  pub layer: [c_char; VK_MAX_EXTENSION_NAME_SIZE as usize],
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Send for VkPhysicalDeviceToolProperties<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Sync for VkPhysicalDeviceToolProperties<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl<'a> VkPhysicalDeviceToolProperties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TOOL_PROPERTIES,
    pNext: core::ptr::null_mut(),
    name: [0i8; VK_MAX_EXTENSION_NAME_SIZE as usize],
    version: [0i8; VK_MAX_EXTENSION_NAME_SIZE as usize],
    purposes: VkToolPurposeFlagBits(0),
    description: [0i8; VK_MAX_DESCRIPTION_SIZE as usize],
    layer: [0i8; VK_MAX_EXTENSION_NAME_SIZE as usize],
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
  pub const fn with_name(mut self, val: [c_char; VK_MAX_EXTENSION_NAME_SIZE as usize]) -> Self {
    self.name = val;
    self
  }
  #[inline]
  pub const fn with_version(mut self, val: [c_char; VK_MAX_EXTENSION_NAME_SIZE as usize]) -> Self {
    self.version = val;
    self
  }
  #[inline]
  pub const fn with_purposes(mut self, val: VkToolPurposeFlags) -> Self {
    self.purposes = val;
    self
  }
  #[inline]
  pub const fn with_description(mut self, val: [c_char; VK_MAX_DESCRIPTION_SIZE as usize]) -> Self {
    self.description = val;
    self
  }
  #[inline]
  pub const fn with_layer(mut self, val: [c_char; VK_MAX_EXTENSION_NAME_SIZE as usize]) -> Self {
    self.layer = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPhysicalDeviceToolProperties<
    'root,
    T: VkPNextExtends<VkPhysicalDeviceToolProperties<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkBufferCopy2](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCopy2.html)
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[deprecated(note = "superseded by `VkDeviceMemoryCopyKHR`")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBufferCopy2<'a> {
  /// Values: VK_STRUCTURE_TYPE_BUFFER_COPY_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub srcOffset: VkDeviceSize,
  pub dstOffset: VkDeviceSize,
  /// No Auto-Validity
  pub size: VkDeviceSize,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Send for VkBufferCopy2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Sync for VkBufferCopy2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl<'a> VkBufferCopy2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_BUFFER_COPY_2,
    pNext: core::ptr::null(),
    srcOffset: 0,
    dstOffset: 0,
    size: 0,
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
  pub const fn with_srcOffset(mut self, val: VkDeviceSize) -> Self {
    self.srcOffset = val;
    self
  }
  #[inline]
  pub const fn with_dstOffset(mut self, val: VkDeviceSize) -> Self {
    self.dstOffset = val;
    self
  }
  #[inline]
  pub const fn with_size(mut self, val: VkDeviceSize) -> Self {
    self.size = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkBufferCopy2<'root, T: VkPNextExtends<VkBufferCopy2<'root>>>(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkImageCopy2](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCopy2.html)
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImageCopy2<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMAGE_COPY_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub srcSubresource: VkImageSubresourceLayers,
  pub srcOffset: VkOffset3D,
  pub dstSubresource: VkImageSubresourceLayers,
  pub dstOffset: VkOffset3D,
  pub extent: VkExtent3D,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Send for VkImageCopy2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Sync for VkImageCopy2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl<'a> VkImageCopy2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_IMAGE_COPY_2,
    pNext: core::ptr::null(),
    srcSubresource: VkImageSubresourceLayers::DEFAULT,
    srcOffset: VkOffset3D::DEFAULT,
    dstSubresource: VkImageSubresourceLayers::DEFAULT,
    dstOffset: VkOffset3D::DEFAULT,
    extent: VkExtent3D::DEFAULT,
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
  pub const fn with_srcSubresource(mut self, val: VkImageSubresourceLayers) -> Self {
    self.srcSubresource = val;
    self
  }
  #[inline]
  pub const fn with_srcOffset(mut self, val: VkOffset3D) -> Self {
    self.srcOffset = val;
    self
  }
  #[inline]
  pub const fn with_dstSubresource(mut self, val: VkImageSubresourceLayers) -> Self {
    self.dstSubresource = val;
    self
  }
  #[inline]
  pub const fn with_dstOffset(mut self, val: VkOffset3D) -> Self {
    self.dstOffset = val;
    self
  }
  #[inline]
  pub const fn with_extent(mut self, val: VkExtent3D) -> Self {
    self.extent = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkImageCopy2<'root, T: VkPNextExtends<VkImageCopy2<'root>>>(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkBufferImageCopy2](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferImageCopy2.html)
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[deprecated(note = "superseded by `VkDeviceMemoryImageCopyKHR`")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBufferImageCopy2<'a> {
  /// Values: VK_STRUCTURE_TYPE_BUFFER_IMAGE_COPY_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub bufferOffset: VkDeviceSize,
  pub bufferRowLength: u32,
  pub bufferImageHeight: u32,
  pub imageSubresource: VkImageSubresourceLayers,
  pub imageOffset: VkOffset3D,
  pub imageExtent: VkExtent3D,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Send for VkBufferImageCopy2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Sync for VkBufferImageCopy2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl<'a> VkBufferImageCopy2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_BUFFER_IMAGE_COPY_2,
    pNext: core::ptr::null(),
    bufferOffset: 0,
    bufferRowLength: 0,
    bufferImageHeight: 0,
    imageSubresource: VkImageSubresourceLayers::DEFAULT,
    imageOffset: VkOffset3D::DEFAULT,
    imageExtent: VkExtent3D::DEFAULT,
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
  pub const fn with_bufferOffset(mut self, val: VkDeviceSize) -> Self {
    self.bufferOffset = val;
    self
  }
  #[inline]
  pub const fn with_bufferRowLength(mut self, val: u32) -> Self {
    self.bufferRowLength = val;
    self
  }
  #[inline]
  pub const fn with_bufferImageHeight(mut self, val: u32) -> Self {
    self.bufferImageHeight = val;
    self
  }
  #[inline]
  pub const fn with_imageSubresource(mut self, val: VkImageSubresourceLayers) -> Self {
    self.imageSubresource = val;
    self
  }
  #[inline]
  pub const fn with_imageOffset(mut self, val: VkOffset3D) -> Self {
    self.imageOffset = val;
    self
  }
  #[inline]
  pub const fn with_imageExtent(mut self, val: VkExtent3D) -> Self {
    self.imageExtent = val;
    self
  }
  #[cfg(feature = "VK_QCOM_rotated_copy_commands")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkCopyCommandTransformInfoQCOM<'child>(
    mut self,
    val: &'a VkCopyCommandTransformInfoQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkCopyCommandTransformInfoQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkBufferImageCopy2<
    'root,
    T: VkPNextExtends<VkBufferImageCopy2<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkCopyBufferInfo2](https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyBufferInfo2.html)
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[deprecated(note = "superseded by `VkCopyDeviceMemoryInfoKHR`")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCopyBufferInfo2<'a> {
  /// Values: VK_STRUCTURE_TYPE_COPY_BUFFER_INFO_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub srcBuffer: VkBuffer,
  pub dstBuffer: VkBuffer,
  pub regionCount: u32,
  /// Length: regionCount
  pub pRegions: *const VkBufferCopy2<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Send for VkCopyBufferInfo2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Sync for VkCopyBufferInfo2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl<'a> VkCopyBufferInfo2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_COPY_BUFFER_INFO_2,
    pNext: core::ptr::null(),
    srcBuffer: VkBuffer::DEFAULT,
    dstBuffer: VkBuffer::DEFAULT,
    regionCount: 0,
    pRegions: core::ptr::null(),
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
  pub const fn with_srcBuffer(mut self, val: VkBuffer) -> Self {
    self.srcBuffer = val;
    self
  }
  #[inline]
  pub const fn with_dstBuffer(mut self, val: VkBuffer) -> Self {
    self.dstBuffer = val;
    self
  }
  #[inline]
  pub const fn with_regionCount(mut self, val: u32) -> Self {
    self.regionCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pRegions(mut self, val: &'a [VkBufferCopy2<'a>]) -> Self {
    self.regionCount = val.len() as u32;
    self.pRegions = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkCopyBufferInfo2<
    'root,
    T: VkPNextExtends<VkCopyBufferInfo2<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkCopyImageInfo2](https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyImageInfo2.html)
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCopyImageInfo2<'a> {
  /// Values: VK_STRUCTURE_TYPE_COPY_IMAGE_INFO_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub srcImage: VkImage,
  pub srcImageLayout: VkImageLayout,
  pub dstImage: VkImage,
  pub dstImageLayout: VkImageLayout,
  pub regionCount: u32,
  /// Length: regionCount
  pub pRegions: *const VkImageCopy2<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Send for VkCopyImageInfo2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Sync for VkCopyImageInfo2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl<'a> VkCopyImageInfo2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_COPY_IMAGE_INFO_2,
    pNext: core::ptr::null(),
    srcImage: VkImage::DEFAULT,
    srcImageLayout: VkImageLayout(0),
    dstImage: VkImage::DEFAULT,
    dstImageLayout: VkImageLayout(0),
    regionCount: 0,
    pRegions: core::ptr::null(),
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
  pub const fn with_srcImage(mut self, val: VkImage) -> Self {
    self.srcImage = val;
    self
  }
  #[inline]
  pub const fn with_srcImageLayout(mut self, val: VkImageLayout) -> Self {
    self.srcImageLayout = val;
    self
  }
  #[inline]
  pub const fn with_dstImage(mut self, val: VkImage) -> Self {
    self.dstImage = val;
    self
  }
  #[inline]
  pub const fn with_dstImageLayout(mut self, val: VkImageLayout) -> Self {
    self.dstImageLayout = val;
    self
  }
  #[inline]
  pub const fn with_regionCount(mut self, val: u32) -> Self {
    self.regionCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pRegions(mut self, val: &'a [VkImageCopy2<'a>]) -> Self {
    self.regionCount = val.len() as u32;
    self.pRegions = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkCopyImageInfo2<
    'root,
    T: VkPNextExtends<VkCopyImageInfo2<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkCopyBufferToImageInfo2](https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyBufferToImageInfo2.html)
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[deprecated(note = "superseded by `VkCopyDeviceMemoryImageInfoKHR`")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCopyBufferToImageInfo2<'a> {
  /// Values: VK_STRUCTURE_TYPE_COPY_BUFFER_TO_IMAGE_INFO_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub srcBuffer: VkBuffer,
  pub dstImage: VkImage,
  pub dstImageLayout: VkImageLayout,
  pub regionCount: u32,
  /// Length: regionCount
  pub pRegions: *const VkBufferImageCopy2<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Send for VkCopyBufferToImageInfo2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Sync for VkCopyBufferToImageInfo2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl<'a> VkCopyBufferToImageInfo2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_COPY_BUFFER_TO_IMAGE_INFO_2,
    pNext: core::ptr::null(),
    srcBuffer: VkBuffer::DEFAULT,
    dstImage: VkImage::DEFAULT,
    dstImageLayout: VkImageLayout(0),
    regionCount: 0,
    pRegions: core::ptr::null(),
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
  pub const fn with_srcBuffer(mut self, val: VkBuffer) -> Self {
    self.srcBuffer = val;
    self
  }
  #[inline]
  pub const fn with_dstImage(mut self, val: VkImage) -> Self {
    self.dstImage = val;
    self
  }
  #[inline]
  pub const fn with_dstImageLayout(mut self, val: VkImageLayout) -> Self {
    self.dstImageLayout = val;
    self
  }
  #[inline]
  pub const fn with_regionCount(mut self, val: u32) -> Self {
    self.regionCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pRegions(mut self, val: &'a [VkBufferImageCopy2<'a>]) -> Self {
    self.regionCount = val.len() as u32;
    self.pRegions = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkCopyBufferToImageInfo2<
    'root,
    T: VkPNextExtends<VkCopyBufferToImageInfo2<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkCopyImageToBufferInfo2](https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyImageToBufferInfo2.html)
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[deprecated(note = "superseded by `VkCopyDeviceMemoryImageInfoKHR`")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCopyImageToBufferInfo2<'a> {
  /// Values: VK_STRUCTURE_TYPE_COPY_IMAGE_TO_BUFFER_INFO_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub srcImage: VkImage,
  pub srcImageLayout: VkImageLayout,
  pub dstBuffer: VkBuffer,
  pub regionCount: u32,
  /// Length: regionCount
  pub pRegions: *const VkBufferImageCopy2<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Send for VkCopyImageToBufferInfo2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Sync for VkCopyImageToBufferInfo2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl<'a> VkCopyImageToBufferInfo2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_COPY_IMAGE_TO_BUFFER_INFO_2,
    pNext: core::ptr::null(),
    srcImage: VkImage::DEFAULT,
    srcImageLayout: VkImageLayout(0),
    dstBuffer: VkBuffer::DEFAULT,
    regionCount: 0,
    pRegions: core::ptr::null(),
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
  pub const fn with_srcImage(mut self, val: VkImage) -> Self {
    self.srcImage = val;
    self
  }
  #[inline]
  pub const fn with_srcImageLayout(mut self, val: VkImageLayout) -> Self {
    self.srcImageLayout = val;
    self
  }
  #[inline]
  pub const fn with_dstBuffer(mut self, val: VkBuffer) -> Self {
    self.dstBuffer = val;
    self
  }
  #[inline]
  pub const fn with_regionCount(mut self, val: u32) -> Self {
    self.regionCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pRegions(mut self, val: &'a [VkBufferImageCopy2<'a>]) -> Self {
    self.regionCount = val.len() as u32;
    self.pRegions = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkCopyImageToBufferInfo2<
    'root,
    T: VkPNextExtends<VkCopyImageToBufferInfo2<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkMemoryBarrier2](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryBarrier2.html)
///
/// **Extends:** VkSubpassDependency2.
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkMemoryBarrier2<'a> {
  /// Values: VK_STRUCTURE_TYPE_MEMORY_BARRIER_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub srcStageMask: VkPipelineStageFlags2,
  /// Optional: true
  pub srcAccessMask: VkAccessFlags2,
  /// Optional: true
  pub dstStageMask: VkPipelineStageFlags2,
  /// Optional: true
  pub dstAccessMask: VkAccessFlags2,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Send for VkMemoryBarrier2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Sync for VkMemoryBarrier2<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_3", feature = "VK_GRAPHICS_VERSION_1_2"))]
unsafe impl<'child, 'root> VkPNextExtends<VkSubpassDependency2<'root>>
  for VkMemoryBarrier2<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl<'a> VkMemoryBarrier2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_MEMORY_BARRIER_2,
    pNext: core::ptr::null(),
    srcStageMask: VkPipelineStageFlagBits2(0),
    srcAccessMask: VkAccessFlagBits2(0),
    dstStageMask: VkPipelineStageFlagBits2(0),
    dstAccessMask: VkAccessFlagBits2(0),
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
  pub const fn with_srcStageMask(mut self, val: VkPipelineStageFlags2) -> Self {
    self.srcStageMask = val;
    self
  }
  #[inline]
  pub const fn with_srcAccessMask(mut self, val: VkAccessFlags2) -> Self {
    self.srcAccessMask = val;
    self
  }
  #[inline]
  pub const fn with_dstStageMask(mut self, val: VkPipelineStageFlags2) -> Self {
    self.dstStageMask = val;
    self
  }
  #[inline]
  pub const fn with_dstAccessMask(mut self, val: VkAccessFlags2) -> Self {
    self.dstAccessMask = val;
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSubpassDependency2<
    'root,
    T: VkPNextExtends<VkSubpassDependency2<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkImageMemoryBarrier2](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageMemoryBarrier2.html)
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImageMemoryBarrier2<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub srcStageMask: VkPipelineStageFlags2,
  /// Optional: true
  pub srcAccessMask: VkAccessFlags2,
  /// Optional: true
  pub dstStageMask: VkPipelineStageFlags2,
  /// Optional: true
  pub dstAccessMask: VkAccessFlags2,
  pub oldLayout: VkImageLayout,
  pub newLayout: VkImageLayout,
  pub srcQueueFamilyIndex: u32,
  pub dstQueueFamilyIndex: u32,
  pub image: VkImage,
  pub subresourceRange: VkImageSubresourceRange,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Send for VkImageMemoryBarrier2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Sync for VkImageMemoryBarrier2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl<'a> VkImageMemoryBarrier2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2,
    pNext: core::ptr::null(),
    srcStageMask: VkPipelineStageFlagBits2(0),
    srcAccessMask: VkAccessFlagBits2(0),
    dstStageMask: VkPipelineStageFlagBits2(0),
    dstAccessMask: VkAccessFlagBits2(0),
    oldLayout: VkImageLayout(0),
    newLayout: VkImageLayout(0),
    srcQueueFamilyIndex: 0,
    dstQueueFamilyIndex: 0,
    image: VkImage::DEFAULT,
    subresourceRange: VkImageSubresourceRange::DEFAULT,
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
  pub const fn with_srcStageMask(mut self, val: VkPipelineStageFlags2) -> Self {
    self.srcStageMask = val;
    self
  }
  #[inline]
  pub const fn with_srcAccessMask(mut self, val: VkAccessFlags2) -> Self {
    self.srcAccessMask = val;
    self
  }
  #[inline]
  pub const fn with_dstStageMask(mut self, val: VkPipelineStageFlags2) -> Self {
    self.dstStageMask = val;
    self
  }
  #[inline]
  pub const fn with_dstAccessMask(mut self, val: VkAccessFlags2) -> Self {
    self.dstAccessMask = val;
    self
  }
  #[inline]
  pub const fn with_oldLayout(mut self, val: VkImageLayout) -> Self {
    self.oldLayout = val;
    self
  }
  #[inline]
  pub const fn with_newLayout(mut self, val: VkImageLayout) -> Self {
    self.newLayout = val;
    self
  }
  #[inline]
  pub const fn with_srcQueueFamilyIndex(mut self, val: u32) -> Self {
    self.srcQueueFamilyIndex = val;
    self
  }
  #[inline]
  pub const fn with_dstQueueFamilyIndex(mut self, val: u32) -> Self {
    self.dstQueueFamilyIndex = val;
    self
  }
  #[inline]
  pub const fn with_image(mut self, val: VkImage) -> Self {
    self.image = val;
    self
  }
  #[inline]
  pub const fn with_subresourceRange(mut self, val: VkImageSubresourceRange) -> Self {
    self.subresourceRange = val;
    self
  }
  #[cfg(feature = "VK_EXT_external_memory_acquire_unmodified")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExternalMemoryAcquireUnmodifiedEXT<'child>(
    mut self,
    val: &'a VkExternalMemoryAcquireUnmodifiedEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExternalMemoryAcquireUnmodifiedEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_maintenance8")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkMemoryBarrierAccessFlags3KHR<'child>(
    mut self,
    val: &'a VkMemoryBarrierAccessFlags3KHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkMemoryBarrierAccessFlags3KHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_sample_locations")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkSampleLocationsInfoEXT<'child>(
    mut self,
    val: &'a VkSampleLocationsInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkSampleLocationsInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkImageMemoryBarrier2<
    'root,
    T: VkPNextExtends<VkImageMemoryBarrier2<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkBufferMemoryBarrier2](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferMemoryBarrier2.html)
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[deprecated(note = "superseded by `VkMemoryRangeBarrierKHR`")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBufferMemoryBarrier2<'a> {
  /// Values: VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub srcStageMask: VkPipelineStageFlags2,
  /// Optional: true
  pub srcAccessMask: VkAccessFlags2,
  /// Optional: true
  pub dstStageMask: VkPipelineStageFlags2,
  /// Optional: true
  pub dstAccessMask: VkAccessFlags2,
  pub srcQueueFamilyIndex: u32,
  pub dstQueueFamilyIndex: u32,
  pub buffer: VkBuffer,
  pub offset: VkDeviceSize,
  pub size: VkDeviceSize,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Send for VkBufferMemoryBarrier2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Sync for VkBufferMemoryBarrier2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl<'a> VkBufferMemoryBarrier2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2,
    pNext: core::ptr::null(),
    srcStageMask: VkPipelineStageFlagBits2(0),
    srcAccessMask: VkAccessFlagBits2(0),
    dstStageMask: VkPipelineStageFlagBits2(0),
    dstAccessMask: VkAccessFlagBits2(0),
    srcQueueFamilyIndex: 0,
    dstQueueFamilyIndex: 0,
    buffer: VkBuffer::DEFAULT,
    offset: 0,
    size: 0,
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
  pub const fn with_srcStageMask(mut self, val: VkPipelineStageFlags2) -> Self {
    self.srcStageMask = val;
    self
  }
  #[inline]
  pub const fn with_srcAccessMask(mut self, val: VkAccessFlags2) -> Self {
    self.srcAccessMask = val;
    self
  }
  #[inline]
  pub const fn with_dstStageMask(mut self, val: VkPipelineStageFlags2) -> Self {
    self.dstStageMask = val;
    self
  }
  #[inline]
  pub const fn with_dstAccessMask(mut self, val: VkAccessFlags2) -> Self {
    self.dstAccessMask = val;
    self
  }
  #[inline]
  pub const fn with_srcQueueFamilyIndex(mut self, val: u32) -> Self {
    self.srcQueueFamilyIndex = val;
    self
  }
  #[inline]
  pub const fn with_dstQueueFamilyIndex(mut self, val: u32) -> Self {
    self.dstQueueFamilyIndex = val;
    self
  }
  #[inline]
  pub const fn with_buffer(mut self, val: VkBuffer) -> Self {
    self.buffer = val;
    self
  }
  #[inline]
  pub const fn with_offset(mut self, val: VkDeviceSize) -> Self {
    self.offset = val;
    self
  }
  #[inline]
  pub const fn with_size(mut self, val: VkDeviceSize) -> Self {
    self.size = val;
    self
  }
  #[cfg(feature = "VK_EXT_external_memory_acquire_unmodified")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExternalMemoryAcquireUnmodifiedEXT<'child>(
    mut self,
    val: &'a VkExternalMemoryAcquireUnmodifiedEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExternalMemoryAcquireUnmodifiedEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_maintenance8")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkMemoryBarrierAccessFlags3KHR<'child>(
    mut self,
    val: &'a VkMemoryBarrierAccessFlags3KHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkMemoryBarrierAccessFlags3KHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkBufferMemoryBarrier2<
    'root,
    T: VkPNextExtends<VkBufferMemoryBarrier2<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkDependencyInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkDependencyInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDependencyInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_DEPENDENCY_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub dependencyFlags: VkDependencyFlags,
  /// Optional: true
  pub memoryBarrierCount: u32,
  /// Length: memoryBarrierCount
  pub pMemoryBarriers: *const VkMemoryBarrier2<'a>,
  /// Optional: true
  pub bufferMemoryBarrierCount: u32,
  /// Length: bufferMemoryBarrierCount
  pub pBufferMemoryBarriers: *const VkBufferMemoryBarrier2<'a>,
  /// Optional: true
  pub imageMemoryBarrierCount: u32,
  /// Length: imageMemoryBarrierCount
  pub pImageMemoryBarriers: *const VkImageMemoryBarrier2<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Send for VkDependencyInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Sync for VkDependencyInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl<'a> VkDependencyInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DEPENDENCY_INFO,
    pNext: core::ptr::null(),
    dependencyFlags: VkDependencyFlagBits(0),
    memoryBarrierCount: 0,
    pMemoryBarriers: core::ptr::null(),
    bufferMemoryBarrierCount: 0,
    pBufferMemoryBarriers: core::ptr::null(),
    imageMemoryBarrierCount: 0,
    pImageMemoryBarriers: core::ptr::null(),
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
  pub const fn with_dependencyFlags(mut self, val: VkDependencyFlags) -> Self {
    self.dependencyFlags = val;
    self
  }
  #[inline]
  pub const fn with_memoryBarrierCount(mut self, val: u32) -> Self {
    self.memoryBarrierCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pMemoryBarriers(mut self, val: &'a [VkMemoryBarrier2<'a>]) -> Self {
    self.memoryBarrierCount = val.len() as u32;
    self.pMemoryBarriers = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_bufferMemoryBarrierCount(mut self, val: u32) -> Self {
    self.bufferMemoryBarrierCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pBufferMemoryBarriers(mut self, val: &'a [VkBufferMemoryBarrier2<'a>]) -> Self {
    self.bufferMemoryBarrierCount = val.len() as u32;
    self.pBufferMemoryBarriers = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_imageMemoryBarrierCount(mut self, val: u32) -> Self {
    self.imageMemoryBarrierCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pImageMemoryBarriers(mut self, val: &'a [VkImageMemoryBarrier2<'a>]) -> Self {
    self.imageMemoryBarrierCount = val.len() as u32;
    self.pImageMemoryBarriers = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_KHR_device_address_commands")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkMemoryRangeBarriersInfoKHR<'child>(
    mut self,
    val: &'a VkMemoryRangeBarriersInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkMemoryRangeBarriersInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_tensors")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkTensorDependencyInfoARM<'child>(
    mut self,
    val: &'a VkTensorDependencyInfoARM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkTensorDependencyInfoARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_tensors")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkTensorMemoryBarrierARM<'child>(
    mut self,
    val: &'a VkTensorMemoryBarrierARM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkTensorMemoryBarrierARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDependencyInfo<
    'root,
    T: VkPNextExtends<VkDependencyInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkSemaphoreSubmitInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreSubmitInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSemaphoreSubmitInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_SEMAPHORE_SUBMIT_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub semaphore: VkSemaphore,
  pub value: u64,
  /// Optional: true
  pub stageMask: VkPipelineStageFlags2,
  pub deviceIndex: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Send for VkSemaphoreSubmitInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Sync for VkSemaphoreSubmitInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl<'a> VkSemaphoreSubmitInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_SEMAPHORE_SUBMIT_INFO,
    pNext: core::ptr::null(),
    semaphore: VkSemaphore::DEFAULT,
    value: 0,
    stageMask: VkPipelineStageFlagBits2(0),
    deviceIndex: 0,
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
  pub const fn with_value(mut self, val: u64) -> Self {
    self.value = val;
    self
  }
  #[inline]
  pub const fn with_stageMask(mut self, val: VkPipelineStageFlags2) -> Self {
    self.stageMask = val;
    self
  }
  #[inline]
  pub const fn with_deviceIndex(mut self, val: u32) -> Self {
    self.deviceIndex = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSemaphoreSubmitInfo<
    'root,
    T: VkPNextExtends<VkSemaphoreSubmitInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkCommandBufferSubmitInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferSubmitInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCommandBufferSubmitInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub commandBuffer: VkCommandBuffer,
  pub deviceMask: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Send for VkCommandBufferSubmitInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Sync for VkCommandBufferSubmitInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl<'a> VkCommandBufferSubmitInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO,
    pNext: core::ptr::null(),
    commandBuffer: VkCommandBuffer::DEFAULT,
    deviceMask: 0,
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
  pub const fn with_commandBuffer(mut self, val: VkCommandBuffer) -> Self {
    self.commandBuffer = val;
    self
  }
  #[inline]
  pub const fn with_deviceMask(mut self, val: u32) -> Self {
    self.deviceMask = val;
    self
  }
  #[cfg(feature = "VK_ARM_render_pass_striped")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkRenderPassStripeSubmitInfoARM<'child>(
    mut self,
    val: &'a VkRenderPassStripeSubmitInfoARM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkRenderPassStripeSubmitInfoARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkCommandBufferSubmitInfo<
    'root,
    T: VkPNextExtends<VkCommandBufferSubmitInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkSubmitInfo2](https://docs.vulkan.org/refpages/latest/refpages/source/VkSubmitInfo2.html)
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSubmitInfo2<'a> {
  /// Values: VK_STRUCTURE_TYPE_SUBMIT_INFO_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkSubmitFlags,
  /// Optional: true
  pub waitSemaphoreInfoCount: u32,
  /// Length: waitSemaphoreInfoCount
  pub pWaitSemaphoreInfos: *const VkSemaphoreSubmitInfo<'a>,
  /// Optional: true
  pub commandBufferInfoCount: u32,
  /// Length: commandBufferInfoCount
  pub pCommandBufferInfos: *const VkCommandBufferSubmitInfo<'a>,
  /// Optional: true
  pub signalSemaphoreInfoCount: u32,
  /// Length: signalSemaphoreInfoCount
  pub pSignalSemaphoreInfos: *const VkSemaphoreSubmitInfo<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Send for VkSubmitInfo2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Sync for VkSubmitInfo2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl<'a> VkSubmitInfo2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_SUBMIT_INFO_2,
    pNext: core::ptr::null(),
    flags: VkSubmitFlagBits(0),
    waitSemaphoreInfoCount: 0,
    pWaitSemaphoreInfos: core::ptr::null(),
    commandBufferInfoCount: 0,
    pCommandBufferInfos: core::ptr::null(),
    signalSemaphoreInfoCount: 0,
    pSignalSemaphoreInfos: core::ptr::null(),
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
  pub const fn with_flags(mut self, val: VkSubmitFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_waitSemaphoreInfoCount(mut self, val: u32) -> Self {
    self.waitSemaphoreInfoCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pWaitSemaphoreInfos(mut self, val: &'a [VkSemaphoreSubmitInfo<'a>]) -> Self {
    self.waitSemaphoreInfoCount = val.len() as u32;
    self.pWaitSemaphoreInfos = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_commandBufferInfoCount(mut self, val: u32) -> Self {
    self.commandBufferInfoCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pCommandBufferInfos(
    mut self,
    val: &'a [VkCommandBufferSubmitInfo<'a>],
  ) -> Self {
    self.commandBufferInfoCount = val.len() as u32;
    self.pCommandBufferInfos = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_signalSemaphoreInfoCount(mut self, val: u32) -> Self {
    self.signalSemaphoreInfoCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pSignalSemaphoreInfos(mut self, val: &'a [VkSemaphoreSubmitInfo<'a>]) -> Self {
    self.signalSemaphoreInfoCount = val.len() as u32;
    self.pSignalSemaphoreInfos = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_EXT_frame_boundary")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkFrameBoundaryEXT<'child>(
    mut self,
    val: &'a VkFrameBoundaryEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkFrameBoundaryEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_frame_boundary"))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkFrameBoundaryTensorsARM<'child>(
    mut self,
    val: &'a VkFrameBoundaryTensorsARM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkFrameBoundaryTensorsARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_low_latency2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkLatencySubmissionPresentIdNV<'child>(
    mut self,
    val: &'a VkLatencySubmissionPresentIdNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkLatencySubmissionPresentIdNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_performance_query")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPerformanceQuerySubmitInfoKHR<'child>(
    mut self,
    val: &'a VkPerformanceQuerySubmitInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPerformanceQuerySubmitInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_win32_keyed_mutex")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkWin32KeyedMutexAcquireReleaseInfoKHR<'child>(
    mut self,
    val: &'a VkWin32KeyedMutexAcquireReleaseInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkWin32KeyedMutexAcquireReleaseInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_win32_keyed_mutex")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkWin32KeyedMutexAcquireReleaseInfoNV<'child>(
    mut self,
    val: &'a VkWin32KeyedMutexAcquireReleaseInfoNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkWin32KeyedMutexAcquireReleaseInfoNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSubmitInfo2<'root, T: VkPNextExtends<VkSubmitInfo2<'root>>>(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceSynchronization2Features](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSynchronization2Features.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceSynchronization2Features<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub synchronization2: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Send for VkPhysicalDeviceSynchronization2Features<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Sync for VkPhysicalDeviceSynchronization2Features<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_3", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceSynchronization2Features<'child>
{
}
#[cfg(all(feature = "VK_BASE_VERSION_1_3", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceSynchronization2Features<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl<'a> VkPhysicalDeviceSynchronization2Features<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES,
    pNext: core::ptr::null_mut(),
    synchronization2: 0,
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
  pub const fn with_synchronization2(mut self, val: VkBool32) -> Self {
    self.synchronization2 = val;
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
/// [VkFormatProperties3](https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatProperties3.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkFormatProperties2.
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkFormatProperties3<'a> {
  /// Values: VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_3
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Optional: true,  Limit Type: [Bitmask]
  pub linearTilingFeatures: VkFormatFeatureFlags2,
  /// Optional: true,  Limit Type: [Bitmask]
  pub optimalTilingFeatures: VkFormatFeatureFlags2,
  /// Optional: true,  Limit Type: [Bitmask]
  pub bufferFeatures: VkFormatFeatureFlags2,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Send for VkFormatProperties3<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'a> Sync for VkFormatProperties3<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_3", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkFormatProperties2<'root>>
  for VkFormatProperties3<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl<'a> VkFormatProperties3<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_3,
    pNext: core::ptr::null_mut(),
    linearTilingFeatures: VkFormatFeatureFlagBits2(0),
    optimalTilingFeatures: VkFormatFeatureFlagBits2(0),
    bufferFeatures: VkFormatFeatureFlagBits2(0),
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
  pub const fn with_linearTilingFeatures(mut self, val: VkFormatFeatureFlags2) -> Self {
    self.linearTilingFeatures = val;
    self
  }
  #[inline]
  pub const fn with_optimalTilingFeatures(mut self, val: VkFormatFeatureFlags2) -> Self {
    self.optimalTilingFeatures = val;
    self
  }
  #[inline]
  pub const fn with_bufferFeatures(mut self, val: VkFormatFeatureFlags2) -> Self {
    self.bufferFeatures = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkFormatProperties2<
    'root,
    T: VkPNextExtends<VkFormatProperties2<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkFlags64](https://docs.vulkan.org/refpages/latest/refpages/source/VkFlags64.html)
#[cfg(any(feature = "VK_BASE_VERSION_1_3", feature = "VK_KHR_synchronization2"))]
pub type VkFlags64 = u64;
