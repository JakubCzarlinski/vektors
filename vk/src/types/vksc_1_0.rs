use crate::consts::VK_UUID_SIZE;
#[cfg(feature = "VKSC_VERSION_1_0")]
use crate::enums::VkFaultLevel;
#[cfg(feature = "VKSC_VERSION_1_0")]
use crate::enums::VkFaultType;
#[cfg(feature = "VKSC_VERSION_1_0")]
use crate::enums::VkPipelineCacheValidationVersion;
#[cfg(feature = "VKSC_VERSION_1_0")]
use crate::enums::VkPipelineMatchControl;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBool32;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkCommandPoolCreateInfo;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkComputePipelineCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceSize;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkGraphicsPipelineCreateInfo;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceFeatures2;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceProperties2;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPipelineCacheCreateInfo;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPipelineCacheHeaderVersionOne;
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
use crate::types::VkRayTracingPipelineCreateInfoKHR;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::types::VkRayTracingPipelineCreateInfoNV;
use core::ffi::c_void;
/// [PFN_vkFaultCallbackFunction](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkFaultCallbackFunction.html)
#[cfg(feature = "VKSC_VERSION_1_0")]
pub type PFN_vkFaultCallbackFunction = Option<
  unsafe extern "system" fn(
    unrecordedFaults: VkBool32,
    faultCount: u32,
    pFaults: *const VkFaultData,
  ),
>;
/// [VkPipelineCacheStageValidationIndexEntry](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCacheStageValidationIndexEntry.html)
#[cfg(feature = "VKSC_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineCacheStageValidationIndexEntry {
  pub codeSize: u64,
  pub codeOffset: u64,
}
#[cfg(feature = "VKSC_VERSION_1_0")]
unsafe impl Send for VkPipelineCacheStageValidationIndexEntry {}
#[cfg(feature = "VKSC_VERSION_1_0")]
unsafe impl Sync for VkPipelineCacheStageValidationIndexEntry {}
#[cfg(feature = "VKSC_VERSION_1_0")]
impl VkPipelineCacheStageValidationIndexEntry {
  pub const DEFAULT: Self = Self {
    codeSize: 0,
    codeOffset: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_codeSize(mut self, val: u64) -> Self {
    self.codeSize = val;
    self
  }
  #[inline]
  pub const fn with_codeOffset(mut self, val: u64) -> Self {
    self.codeOffset = val;
    self
  }
}
/// [VkPipelineCacheSafetyCriticalIndexEntry](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCacheSafetyCriticalIndexEntry.html)
#[cfg(feature = "VKSC_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineCacheSafetyCriticalIndexEntry {
  pub pipelineIdentifier: [u8; VK_UUID_SIZE as usize],
  pub pipelineMemorySize: u64,
  pub jsonSize: u64,
  pub jsonOffset: u64,
  pub stageIndexCount: u32,
  pub stageIndexStride: u32,
  pub stageIndexOffset: u64,
}
#[cfg(feature = "VKSC_VERSION_1_0")]
unsafe impl Send for VkPipelineCacheSafetyCriticalIndexEntry {}
#[cfg(feature = "VKSC_VERSION_1_0")]
unsafe impl Sync for VkPipelineCacheSafetyCriticalIndexEntry {}
#[cfg(feature = "VKSC_VERSION_1_0")]
impl VkPipelineCacheSafetyCriticalIndexEntry {
  pub const DEFAULT: Self = Self {
    pipelineIdentifier: [0u8; VK_UUID_SIZE as usize],
    pipelineMemorySize: 0,
    jsonSize: 0,
    jsonOffset: 0,
    stageIndexCount: 0,
    stageIndexStride: 0,
    stageIndexOffset: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_pipelineIdentifier(mut self, val: [u8; VK_UUID_SIZE as usize]) -> Self {
    self.pipelineIdentifier = val;
    self
  }
  #[inline]
  pub const fn with_pipelineMemorySize(mut self, val: u64) -> Self {
    self.pipelineMemorySize = val;
    self
  }
  #[inline]
  pub const fn with_jsonSize(mut self, val: u64) -> Self {
    self.jsonSize = val;
    self
  }
  #[inline]
  pub const fn with_jsonOffset(mut self, val: u64) -> Self {
    self.jsonOffset = val;
    self
  }
  #[inline]
  pub const fn with_stageIndexCount(mut self, val: u32) -> Self {
    self.stageIndexCount = val;
    self
  }
  #[inline]
  pub const fn with_stageIndexStride(mut self, val: u32) -> Self {
    self.stageIndexStride = val;
    self
  }
  #[inline]
  pub const fn with_stageIndexOffset(mut self, val: u64) -> Self {
    self.stageIndexOffset = val;
    self
  }
}
/// [VkPipelineCacheHeaderVersionSafetyCriticalOne](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCacheHeaderVersionSafetyCriticalOne.html)
#[cfg(feature = "VKSC_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineCacheHeaderVersionSafetyCriticalOne {
  pub headerVersionOne: VkPipelineCacheHeaderVersionOne,
  pub validationVersion: VkPipelineCacheValidationVersion,
  pub implementationData: u32,
  pub pipelineIndexCount: u32,
  pub pipelineIndexStride: u32,
  pub pipelineIndexOffset: u64,
}
#[cfg(feature = "VKSC_VERSION_1_0")]
unsafe impl Send for VkPipelineCacheHeaderVersionSafetyCriticalOne {}
#[cfg(feature = "VKSC_VERSION_1_0")]
unsafe impl Sync for VkPipelineCacheHeaderVersionSafetyCriticalOne {}
#[cfg(feature = "VKSC_VERSION_1_0")]
impl VkPipelineCacheHeaderVersionSafetyCriticalOne {
  pub const DEFAULT: Self = Self {
    headerVersionOne: VkPipelineCacheHeaderVersionOne::DEFAULT,
    validationVersion: VkPipelineCacheValidationVersion(0),
    implementationData: 0,
    pipelineIndexCount: 0,
    pipelineIndexStride: 0,
    pipelineIndexOffset: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_headerVersionOne(mut self, val: VkPipelineCacheHeaderVersionOne) -> Self {
    self.headerVersionOne = val;
    self
  }
  #[inline]
  pub const fn with_validationVersion(mut self, val: VkPipelineCacheValidationVersion) -> Self {
    self.validationVersion = val;
    self
  }
  #[inline]
  pub const fn with_implementationData(mut self, val: u32) -> Self {
    self.implementationData = val;
    self
  }
  #[inline]
  pub const fn with_pipelineIndexCount(mut self, val: u32) -> Self {
    self.pipelineIndexCount = val;
    self
  }
  #[inline]
  pub const fn with_pipelineIndexStride(mut self, val: u32) -> Self {
    self.pipelineIndexStride = val;
    self
  }
  #[inline]
  pub const fn with_pipelineIndexOffset(mut self, val: u64) -> Self {
    self.pipelineIndexOffset = val;
    self
  }
}
/// [VkFaultData](https://docs.vulkan.org/refpages/latest/refpages/source/VkFaultData.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VKSC_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkFaultData<'a> {
  /// Values: VK_STRUCTURE_TYPE_FAULT_DATA
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub faultLevel: VkFaultLevel,
  pub faultType: VkFaultType,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VKSC_VERSION_1_0")]
unsafe impl<'a> Send for VkFaultData<'a> {}
#[cfg(feature = "VKSC_VERSION_1_0")]
unsafe impl<'a> Sync for VkFaultData<'a> {}
#[cfg(feature = "VKSC_VERSION_1_0")]
impl<'a> VkFaultData<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::FAULT_DATA,
    pNext: core::ptr::null_mut(),
    faultLevel: VkFaultLevel(0),
    faultType: VkFaultType(0),
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
  pub const fn with_faultLevel(mut self, val: VkFaultLevel) -> Self {
    self.faultLevel = val;
    self
  }
  #[inline]
  pub const fn with_faultType(mut self, val: VkFaultType) -> Self {
    self.faultType = val;
    self
  }
  #[cfg(feature = "VKSC_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkFaultData<'root, T: VkPNextExtends<VkFaultData<'root>>>(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkFaultCallbackInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkFaultCallbackInfo.html)
///
/// **Extends:** VkDeviceCreateInfo.
#[cfg(feature = "VKSC_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkFaultCallbackInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_FAULT_CALLBACK_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub faultCount: u32,
  /// Optional: true,  Length: faultCount
  pub pFaults: *mut VkFaultData<'a>,
  pub pfnFaultCallback: PFN_vkFaultCallbackFunction,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VKSC_VERSION_1_0")]
unsafe impl<'a> Send for VkFaultCallbackInfo<'a> {}
#[cfg(feature = "VKSC_VERSION_1_0")]
unsafe impl<'a> Sync for VkFaultCallbackInfo<'a> {}
#[cfg(all(feature = "VKSC_VERSION_1_0", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkFaultCallbackInfo<'child>
{
}
#[cfg(feature = "VKSC_VERSION_1_0")]
impl<'a> VkFaultCallbackInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::FAULT_CALLBACK_INFO,
    pNext: core::ptr::null(),
    faultCount: 0,
    pFaults: core::ptr::null_mut(),
    pfnFaultCallback: None,
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
  pub const fn with_faultCount(mut self, val: u32) -> Self {
    self.faultCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pFaults(mut self, val: &'a mut [VkFaultData<'a>]) -> Self {
    self.faultCount = val.len() as u32;
    self.pFaults = val.as_mut_ptr();
    self
  }
  #[inline]
  pub const fn with_pfnFaultCallback(mut self, val: PFN_vkFaultCallbackFunction) -> Self {
    self.pfnFaultCallback = val;
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
/// [VkPipelineOfflineCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineOfflineCreateInfo.html)
///
/// **Extends:** VkGraphicsPipelineCreateInfo, VkComputePipelineCreateInfo, VkRayTracingPipelineCreateInfoKHR, VkRayTracingPipelineCreateInfoNV.
#[cfg(feature = "VKSC_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineOfflineCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_OFFLINE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub pipelineIdentifier: [u8; VK_UUID_SIZE as usize],
  pub matchControl: VkPipelineMatchControl,
  pub poolEntrySize: VkDeviceSize,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VKSC_VERSION_1_0")]
unsafe impl<'a> Send for VkPipelineOfflineCreateInfo<'a> {}
#[cfg(feature = "VKSC_VERSION_1_0")]
unsafe impl<'a> Sync for VkPipelineOfflineCreateInfo<'a> {}
#[cfg(all(feature = "VKSC_VERSION_1_0", feature = "VK_GRAPHICS_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkGraphicsPipelineCreateInfo<'root>>
  for VkPipelineOfflineCreateInfo<'child>
{
}
#[cfg(all(feature = "VKSC_VERSION_1_0", feature = "VK_COMPUTE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkComputePipelineCreateInfo<'root>>
  for VkPipelineOfflineCreateInfo<'child>
{
}
#[cfg(all(feature = "VKSC_VERSION_1_0", feature = "VK_KHR_ray_tracing_pipeline"))]
unsafe impl<'child, 'root> VkPNextExtends<VkRayTracingPipelineCreateInfoKHR<'root>>
  for VkPipelineOfflineCreateInfo<'child>
{
}
#[cfg(all(feature = "VKSC_VERSION_1_0", feature = "VK_NV_ray_tracing"))]
unsafe impl<'child, 'root> VkPNextExtends<VkRayTracingPipelineCreateInfoNV<'root>>
  for VkPipelineOfflineCreateInfo<'child>
{
}
#[cfg(feature = "VKSC_VERSION_1_0")]
impl<'a> VkPipelineOfflineCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_OFFLINE_CREATE_INFO,
    pNext: core::ptr::null(),
    pipelineIdentifier: [0u8; VK_UUID_SIZE as usize],
    matchControl: VkPipelineMatchControl(0),
    poolEntrySize: 0,
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
  pub const fn with_pipelineIdentifier(mut self, val: [u8; VK_UUID_SIZE as usize]) -> Self {
    self.pipelineIdentifier = val;
    self
  }
  #[inline]
  pub const fn with_matchControl(mut self, val: VkPipelineMatchControl) -> Self {
    self.matchControl = val;
    self
  }
  #[inline]
  pub const fn with_poolEntrySize(mut self, val: VkDeviceSize) -> Self {
    self.poolEntrySize = val;
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
  #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkRayTracingPipelineCreateInfoKHR<
    'root,
    T: VkPNextExtends<VkRayTracingPipelineCreateInfoKHR<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_ray_tracing")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkRayTracingPipelineCreateInfoNV<
    'root,
    T: VkPNextExtends<VkRayTracingPipelineCreateInfoNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceVulkanSC10Properties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVulkanSC10Properties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VKSC_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceVulkanSC10Properties<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_SC_1_0_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub deviceNoDynamicHostAllocations: VkBool32,
  /// Limit Type: [Max]
  pub deviceDestroyFreesMemory: VkBool32,
  /// Limit Type: [Max]
  pub commandPoolMultipleCommandBuffersRecording: VkBool32,
  /// Limit Type: [Max]
  pub commandPoolResetCommandBuffer: VkBool32,
  /// Limit Type: [Max]
  pub commandBufferSimultaneousUse: VkBool32,
  /// Limit Type: [Max]
  pub secondaryCommandBufferNullOrImagelessFramebuffer: VkBool32,
  /// Limit Type: [Max]
  pub recycleDescriptorSetMemory: VkBool32,
  /// Limit Type: [Max]
  pub recyclePipelineMemory: VkBool32,
  /// Limit Type: [Max]
  pub maxRenderPassSubpasses: u32,
  /// Limit Type: [Max]
  pub maxRenderPassDependencies: u32,
  /// Limit Type: [Max]
  pub maxSubpassInputAttachments: u32,
  /// Limit Type: [Max]
  pub maxSubpassPreserveAttachments: u32,
  /// Limit Type: [Max]
  pub maxFramebufferAttachments: u32,
  /// Limit Type: [Max]
  pub maxDescriptorSetLayoutBindings: u32,
  /// Limit Type: [Max]
  pub maxQueryFaultCount: u32,
  /// Limit Type: [Max]
  pub maxCallbackFaultCount: u32,
  /// Limit Type: [Max]
  pub maxCommandPoolCommandBuffers: u32,
  /// Limit Type: [Max]
  pub maxCommandBufferSize: VkDeviceSize,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VKSC_VERSION_1_0")]
unsafe impl<'a> Send for VkPhysicalDeviceVulkanSC10Properties<'a> {}
#[cfg(feature = "VKSC_VERSION_1_0")]
unsafe impl<'a> Sync for VkPhysicalDeviceVulkanSC10Properties<'a> {}
#[cfg(all(feature = "VKSC_VERSION_1_0", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceVulkanSC10Properties<'child>
{
}
#[cfg(feature = "VKSC_VERSION_1_0")]
impl<'a> VkPhysicalDeviceVulkanSC10Properties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_VULKAN_SC_1_0_PROPERTIES,
    pNext: core::ptr::null_mut(),
    deviceNoDynamicHostAllocations: 0,
    deviceDestroyFreesMemory: 0,
    commandPoolMultipleCommandBuffersRecording: 0,
    commandPoolResetCommandBuffer: 0,
    commandBufferSimultaneousUse: 0,
    secondaryCommandBufferNullOrImagelessFramebuffer: 0,
    recycleDescriptorSetMemory: 0,
    recyclePipelineMemory: 0,
    maxRenderPassSubpasses: 0,
    maxRenderPassDependencies: 0,
    maxSubpassInputAttachments: 0,
    maxSubpassPreserveAttachments: 0,
    maxFramebufferAttachments: 0,
    maxDescriptorSetLayoutBindings: 0,
    maxQueryFaultCount: 0,
    maxCallbackFaultCount: 0,
    maxCommandPoolCommandBuffers: 0,
    maxCommandBufferSize: 0,
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
  pub const fn with_deviceNoDynamicHostAllocations(mut self, val: VkBool32) -> Self {
    self.deviceNoDynamicHostAllocations = val;
    self
  }
  #[inline]
  pub const fn with_deviceDestroyFreesMemory(mut self, val: VkBool32) -> Self {
    self.deviceDestroyFreesMemory = val;
    self
  }
  #[inline]
  pub const fn with_commandPoolMultipleCommandBuffersRecording(mut self, val: VkBool32) -> Self {
    self.commandPoolMultipleCommandBuffersRecording = val;
    self
  }
  #[inline]
  pub const fn with_commandPoolResetCommandBuffer(mut self, val: VkBool32) -> Self {
    self.commandPoolResetCommandBuffer = val;
    self
  }
  #[inline]
  pub const fn with_commandBufferSimultaneousUse(mut self, val: VkBool32) -> Self {
    self.commandBufferSimultaneousUse = val;
    self
  }
  #[inline]
  pub const fn with_secondaryCommandBufferNullOrImagelessFramebuffer(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.secondaryCommandBufferNullOrImagelessFramebuffer = val;
    self
  }
  #[inline]
  pub const fn with_recycleDescriptorSetMemory(mut self, val: VkBool32) -> Self {
    self.recycleDescriptorSetMemory = val;
    self
  }
  #[inline]
  pub const fn with_recyclePipelineMemory(mut self, val: VkBool32) -> Self {
    self.recyclePipelineMemory = val;
    self
  }
  #[inline]
  pub const fn with_maxRenderPassSubpasses(mut self, val: u32) -> Self {
    self.maxRenderPassSubpasses = val;
    self
  }
  #[inline]
  pub const fn with_maxRenderPassDependencies(mut self, val: u32) -> Self {
    self.maxRenderPassDependencies = val;
    self
  }
  #[inline]
  pub const fn with_maxSubpassInputAttachments(mut self, val: u32) -> Self {
    self.maxSubpassInputAttachments = val;
    self
  }
  #[inline]
  pub const fn with_maxSubpassPreserveAttachments(mut self, val: u32) -> Self {
    self.maxSubpassPreserveAttachments = val;
    self
  }
  #[inline]
  pub const fn with_maxFramebufferAttachments(mut self, val: u32) -> Self {
    self.maxFramebufferAttachments = val;
    self
  }
  #[inline]
  pub const fn with_maxDescriptorSetLayoutBindings(mut self, val: u32) -> Self {
    self.maxDescriptorSetLayoutBindings = val;
    self
  }
  #[inline]
  pub const fn with_maxQueryFaultCount(mut self, val: u32) -> Self {
    self.maxQueryFaultCount = val;
    self
  }
  #[inline]
  pub const fn with_maxCallbackFaultCount(mut self, val: u32) -> Self {
    self.maxCallbackFaultCount = val;
    self
  }
  #[inline]
  pub const fn with_maxCommandPoolCommandBuffers(mut self, val: u32) -> Self {
    self.maxCommandPoolCommandBuffers = val;
    self
  }
  #[inline]
  pub const fn with_maxCommandBufferSize(mut self, val: VkDeviceSize) -> Self {
    self.maxCommandBufferSize = val;
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
/// [VkPipelinePoolSize](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelinePoolSize.html)
#[cfg(feature = "VKSC_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelinePoolSize<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_POOL_SIZE
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub poolEntrySize: VkDeviceSize,
  pub poolEntryCount: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VKSC_VERSION_1_0")]
unsafe impl<'a> Send for VkPipelinePoolSize<'a> {}
#[cfg(feature = "VKSC_VERSION_1_0")]
unsafe impl<'a> Sync for VkPipelinePoolSize<'a> {}
#[cfg(feature = "VKSC_VERSION_1_0")]
impl<'a> VkPipelinePoolSize<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_POOL_SIZE,
    pNext: core::ptr::null(),
    poolEntrySize: 0,
    poolEntryCount: 0,
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
  pub const fn with_poolEntrySize(mut self, val: VkDeviceSize) -> Self {
    self.poolEntrySize = val;
    self
  }
  #[inline]
  pub const fn with_poolEntryCount(mut self, val: u32) -> Self {
    self.poolEntryCount = val;
    self
  }
  #[cfg(feature = "VKSC_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPipelinePoolSize<
    'root,
    T: VkPNextExtends<VkPipelinePoolSize<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkDeviceObjectReservationCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceObjectReservationCreateInfo.html)
///
/// **Extends:** VkDeviceCreateInfo.
#[cfg(feature = "VKSC_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDeviceObjectReservationCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_DEVICE_OBJECT_RESERVATION_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub pipelineCacheCreateInfoCount: u32,
  /// Length: pipelineCacheCreateInfoCount
  pub pPipelineCacheCreateInfos: *const VkPipelineCacheCreateInfo<'a>,
  /// Optional: true
  pub pipelinePoolSizeCount: u32,
  /// Length: pipelinePoolSizeCount
  pub pPipelinePoolSizes: *const VkPipelinePoolSize<'a>,
  /// Optional: true
  pub semaphoreRequestCount: u32,
  /// Optional: true
  pub commandBufferRequestCount: u32,
  /// Optional: true
  pub fenceRequestCount: u32,
  /// Optional: true
  pub deviceMemoryRequestCount: u32,
  /// Optional: true
  pub bufferRequestCount: u32,
  /// Optional: true
  pub imageRequestCount: u32,
  /// Optional: true
  pub eventRequestCount: u32,
  /// Optional: true
  pub queryPoolRequestCount: u32,
  /// Optional: true
  pub bufferViewRequestCount: u32,
  /// Optional: true
  pub imageViewRequestCount: u32,
  /// Optional: true
  pub layeredImageViewRequestCount: u32,
  /// Optional: true
  pub pipelineCacheRequestCount: u32,
  /// Optional: true
  pub pipelineLayoutRequestCount: u32,
  /// Optional: true
  pub renderPassRequestCount: u32,
  /// Optional: true
  pub graphicsPipelineRequestCount: u32,
  /// Optional: true
  pub computePipelineRequestCount: u32,
  /// Optional: true
  pub descriptorSetLayoutRequestCount: u32,
  /// Optional: true
  pub samplerRequestCount: u32,
  /// Optional: true
  pub descriptorPoolRequestCount: u32,
  /// Optional: true
  pub descriptorSetRequestCount: u32,
  /// Optional: true
  pub framebufferRequestCount: u32,
  /// Optional: true
  pub commandPoolRequestCount: u32,
  /// Optional: true
  pub samplerYcbcrConversionRequestCount: u32,
  /// Optional: true
  pub surfaceRequestCount: u32,
  /// Optional: true
  pub swapchainRequestCount: u32,
  /// Optional: true
  pub displayModeRequestCount: u32,
  /// Optional: true
  pub subpassDescriptionRequestCount: u32,
  /// Optional: true
  pub attachmentDescriptionRequestCount: u32,
  /// Optional: true
  pub descriptorSetLayoutBindingRequestCount: u32,
  pub descriptorSetLayoutBindingLimit: u32,
  pub maxImageViewMipLevels: u32,
  pub maxImageViewArrayLayers: u32,
  pub maxLayeredImageViewMipLevels: u32,
  pub maxOcclusionQueriesPerPool: u32,
  pub maxPipelineStatisticsQueriesPerPool: u32,
  pub maxTimestampQueriesPerPool: u32,
  pub maxImmutableSamplersPerDescriptorSetLayout: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VKSC_VERSION_1_0")]
unsafe impl<'a> Send for VkDeviceObjectReservationCreateInfo<'a> {}
#[cfg(feature = "VKSC_VERSION_1_0")]
unsafe impl<'a> Sync for VkDeviceObjectReservationCreateInfo<'a> {}
#[cfg(all(feature = "VKSC_VERSION_1_0", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkDeviceObjectReservationCreateInfo<'child>
{
}
#[cfg(feature = "VKSC_VERSION_1_0")]
impl<'a> VkDeviceObjectReservationCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::DEVICE_OBJECT_RESERVATION_CREATE_INFO,
    pNext: core::ptr::null(),
    pipelineCacheCreateInfoCount: 0,
    pPipelineCacheCreateInfos: core::ptr::null(),
    pipelinePoolSizeCount: 0,
    pPipelinePoolSizes: core::ptr::null(),
    semaphoreRequestCount: 0,
    commandBufferRequestCount: 0,
    fenceRequestCount: 0,
    deviceMemoryRequestCount: 0,
    bufferRequestCount: 0,
    imageRequestCount: 0,
    eventRequestCount: 0,
    queryPoolRequestCount: 0,
    bufferViewRequestCount: 0,
    imageViewRequestCount: 0,
    layeredImageViewRequestCount: 0,
    pipelineCacheRequestCount: 0,
    pipelineLayoutRequestCount: 0,
    renderPassRequestCount: 0,
    graphicsPipelineRequestCount: 0,
    computePipelineRequestCount: 0,
    descriptorSetLayoutRequestCount: 0,
    samplerRequestCount: 0,
    descriptorPoolRequestCount: 0,
    descriptorSetRequestCount: 0,
    framebufferRequestCount: 0,
    commandPoolRequestCount: 0,
    samplerYcbcrConversionRequestCount: 0,
    surfaceRequestCount: 0,
    swapchainRequestCount: 0,
    displayModeRequestCount: 0,
    subpassDescriptionRequestCount: 0,
    attachmentDescriptionRequestCount: 0,
    descriptorSetLayoutBindingRequestCount: 0,
    descriptorSetLayoutBindingLimit: 0,
    maxImageViewMipLevels: 0,
    maxImageViewArrayLayers: 0,
    maxLayeredImageViewMipLevels: 0,
    maxOcclusionQueriesPerPool: 0,
    maxPipelineStatisticsQueriesPerPool: 0,
    maxTimestampQueriesPerPool: 0,
    maxImmutableSamplersPerDescriptorSetLayout: 0,
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
  pub const fn with_pipelineCacheCreateInfoCount(mut self, val: u32) -> Self {
    self.pipelineCacheCreateInfoCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pPipelineCacheCreateInfos(
    mut self,
    val: &'a [VkPipelineCacheCreateInfo<'a>],
  ) -> Self {
    self.pipelineCacheCreateInfoCount = val.len() as u32;
    self.pPipelineCacheCreateInfos = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_pipelinePoolSizeCount(mut self, val: u32) -> Self {
    self.pipelinePoolSizeCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pPipelinePoolSizes(mut self, val: &'a [VkPipelinePoolSize<'a>]) -> Self {
    self.pipelinePoolSizeCount = val.len() as u32;
    self.pPipelinePoolSizes = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_semaphoreRequestCount(mut self, val: u32) -> Self {
    self.semaphoreRequestCount = val;
    self
  }
  #[inline]
  pub const fn with_commandBufferRequestCount(mut self, val: u32) -> Self {
    self.commandBufferRequestCount = val;
    self
  }
  #[inline]
  pub const fn with_fenceRequestCount(mut self, val: u32) -> Self {
    self.fenceRequestCount = val;
    self
  }
  #[inline]
  pub const fn with_deviceMemoryRequestCount(mut self, val: u32) -> Self {
    self.deviceMemoryRequestCount = val;
    self
  }
  #[inline]
  pub const fn with_bufferRequestCount(mut self, val: u32) -> Self {
    self.bufferRequestCount = val;
    self
  }
  #[inline]
  pub const fn with_imageRequestCount(mut self, val: u32) -> Self {
    self.imageRequestCount = val;
    self
  }
  #[inline]
  pub const fn with_eventRequestCount(mut self, val: u32) -> Self {
    self.eventRequestCount = val;
    self
  }
  #[inline]
  pub const fn with_queryPoolRequestCount(mut self, val: u32) -> Self {
    self.queryPoolRequestCount = val;
    self
  }
  #[inline]
  pub const fn with_bufferViewRequestCount(mut self, val: u32) -> Self {
    self.bufferViewRequestCount = val;
    self
  }
  #[inline]
  pub const fn with_imageViewRequestCount(mut self, val: u32) -> Self {
    self.imageViewRequestCount = val;
    self
  }
  #[inline]
  pub const fn with_layeredImageViewRequestCount(mut self, val: u32) -> Self {
    self.layeredImageViewRequestCount = val;
    self
  }
  #[inline]
  pub const fn with_pipelineCacheRequestCount(mut self, val: u32) -> Self {
    self.pipelineCacheRequestCount = val;
    self
  }
  #[inline]
  pub const fn with_pipelineLayoutRequestCount(mut self, val: u32) -> Self {
    self.pipelineLayoutRequestCount = val;
    self
  }
  #[inline]
  pub const fn with_renderPassRequestCount(mut self, val: u32) -> Self {
    self.renderPassRequestCount = val;
    self
  }
  #[inline]
  pub const fn with_graphicsPipelineRequestCount(mut self, val: u32) -> Self {
    self.graphicsPipelineRequestCount = val;
    self
  }
  #[inline]
  pub const fn with_computePipelineRequestCount(mut self, val: u32) -> Self {
    self.computePipelineRequestCount = val;
    self
  }
  #[inline]
  pub const fn with_descriptorSetLayoutRequestCount(mut self, val: u32) -> Self {
    self.descriptorSetLayoutRequestCount = val;
    self
  }
  #[inline]
  pub const fn with_samplerRequestCount(mut self, val: u32) -> Self {
    self.samplerRequestCount = val;
    self
  }
  #[inline]
  pub const fn with_descriptorPoolRequestCount(mut self, val: u32) -> Self {
    self.descriptorPoolRequestCount = val;
    self
  }
  #[inline]
  pub const fn with_descriptorSetRequestCount(mut self, val: u32) -> Self {
    self.descriptorSetRequestCount = val;
    self
  }
  #[inline]
  pub const fn with_framebufferRequestCount(mut self, val: u32) -> Self {
    self.framebufferRequestCount = val;
    self
  }
  #[inline]
  pub const fn with_commandPoolRequestCount(mut self, val: u32) -> Self {
    self.commandPoolRequestCount = val;
    self
  }
  #[inline]
  pub const fn with_samplerYcbcrConversionRequestCount(mut self, val: u32) -> Self {
    self.samplerYcbcrConversionRequestCount = val;
    self
  }
  #[inline]
  pub const fn with_surfaceRequestCount(mut self, val: u32) -> Self {
    self.surfaceRequestCount = val;
    self
  }
  #[inline]
  pub const fn with_swapchainRequestCount(mut self, val: u32) -> Self {
    self.swapchainRequestCount = val;
    self
  }
  #[inline]
  pub const fn with_displayModeRequestCount(mut self, val: u32) -> Self {
    self.displayModeRequestCount = val;
    self
  }
  #[inline]
  pub const fn with_subpassDescriptionRequestCount(mut self, val: u32) -> Self {
    self.subpassDescriptionRequestCount = val;
    self
  }
  #[inline]
  pub const fn with_attachmentDescriptionRequestCount(mut self, val: u32) -> Self {
    self.attachmentDescriptionRequestCount = val;
    self
  }
  #[inline]
  pub const fn with_descriptorSetLayoutBindingRequestCount(mut self, val: u32) -> Self {
    self.descriptorSetLayoutBindingRequestCount = val;
    self
  }
  #[inline]
  pub const fn with_descriptorSetLayoutBindingLimit(mut self, val: u32) -> Self {
    self.descriptorSetLayoutBindingLimit = val;
    self
  }
  #[inline]
  pub const fn with_maxImageViewMipLevels(mut self, val: u32) -> Self {
    self.maxImageViewMipLevels = val;
    self
  }
  #[inline]
  pub const fn with_maxImageViewArrayLayers(mut self, val: u32) -> Self {
    self.maxImageViewArrayLayers = val;
    self
  }
  #[inline]
  pub const fn with_maxLayeredImageViewMipLevels(mut self, val: u32) -> Self {
    self.maxLayeredImageViewMipLevels = val;
    self
  }
  #[inline]
  pub const fn with_maxOcclusionQueriesPerPool(mut self, val: u32) -> Self {
    self.maxOcclusionQueriesPerPool = val;
    self
  }
  #[inline]
  pub const fn with_maxPipelineStatisticsQueriesPerPool(mut self, val: u32) -> Self {
    self.maxPipelineStatisticsQueriesPerPool = val;
    self
  }
  #[inline]
  pub const fn with_maxTimestampQueriesPerPool(mut self, val: u32) -> Self {
    self.maxTimestampQueriesPerPool = val;
    self
  }
  #[inline]
  pub const fn with_maxImmutableSamplersPerDescriptorSetLayout(mut self, val: u32) -> Self {
    self.maxImmutableSamplersPerDescriptorSetLayout = val;
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
/// [VkCommandPoolMemoryReservationCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPoolMemoryReservationCreateInfo.html)
///
/// **Extends:** VkCommandPoolCreateInfo.
#[cfg(feature = "VKSC_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCommandPoolMemoryReservationCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_COMMAND_POOL_MEMORY_RESERVATION_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub commandPoolReservedSize: VkDeviceSize,
  pub commandPoolMaxCommandBuffers: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VKSC_VERSION_1_0")]
unsafe impl<'a> Send for VkCommandPoolMemoryReservationCreateInfo<'a> {}
#[cfg(feature = "VKSC_VERSION_1_0")]
unsafe impl<'a> Sync for VkCommandPoolMemoryReservationCreateInfo<'a> {}
#[cfg(all(feature = "VKSC_VERSION_1_0", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkCommandPoolCreateInfo<'root>>
  for VkCommandPoolMemoryReservationCreateInfo<'child>
{
}
#[cfg(feature = "VKSC_VERSION_1_0")]
impl<'a> VkCommandPoolMemoryReservationCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::COMMAND_POOL_MEMORY_RESERVATION_CREATE_INFO,
    pNext: core::ptr::null(),
    commandPoolReservedSize: 0,
    commandPoolMaxCommandBuffers: 0,
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
  pub const fn with_commandPoolReservedSize(mut self, val: VkDeviceSize) -> Self {
    self.commandPoolReservedSize = val;
    self
  }
  #[inline]
  pub const fn with_commandPoolMaxCommandBuffers(mut self, val: u32) -> Self {
    self.commandPoolMaxCommandBuffers = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkCommandPoolCreateInfo<
    'root,
    T: VkPNextExtends<VkCommandPoolCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkCommandPoolMemoryConsumption](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPoolMemoryConsumption.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VKSC_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCommandPoolMemoryConsumption<'a> {
  /// Values: VK_STRUCTURE_TYPE_COMMAND_POOL_MEMORY_CONSUMPTION
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub commandPoolAllocated: VkDeviceSize,
  pub commandPoolReservedSize: VkDeviceSize,
  pub commandBufferAllocated: VkDeviceSize,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VKSC_VERSION_1_0")]
unsafe impl<'a> Send for VkCommandPoolMemoryConsumption<'a> {}
#[cfg(feature = "VKSC_VERSION_1_0")]
unsafe impl<'a> Sync for VkCommandPoolMemoryConsumption<'a> {}
#[cfg(feature = "VKSC_VERSION_1_0")]
impl<'a> VkCommandPoolMemoryConsumption<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::COMMAND_POOL_MEMORY_CONSUMPTION,
    pNext: core::ptr::null_mut(),
    commandPoolAllocated: 0,
    commandPoolReservedSize: 0,
    commandBufferAllocated: 0,
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
  pub const fn with_commandPoolAllocated(mut self, val: VkDeviceSize) -> Self {
    self.commandPoolAllocated = val;
    self
  }
  #[inline]
  pub const fn with_commandPoolReservedSize(mut self, val: VkDeviceSize) -> Self {
    self.commandPoolReservedSize = val;
    self
  }
  #[inline]
  pub const fn with_commandBufferAllocated(mut self, val: VkDeviceSize) -> Self {
    self.commandBufferAllocated = val;
    self
  }
  #[cfg(feature = "VKSC_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkCommandPoolMemoryConsumption<
    'root,
    T: VkPNextExtends<VkCommandPoolMemoryConsumption<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceVulkanSC10Features](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVulkanSC10Features.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VKSC_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceVulkanSC10Features<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_SC_1_0_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub shaderAtomicInstructions: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VKSC_VERSION_1_0")]
unsafe impl<'a> Send for VkPhysicalDeviceVulkanSC10Features<'a> {}
#[cfg(feature = "VKSC_VERSION_1_0")]
unsafe impl<'a> Sync for VkPhysicalDeviceVulkanSC10Features<'a> {}
#[cfg(all(feature = "VKSC_VERSION_1_0", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceVulkanSC10Features<'child>
{
}
#[cfg(all(feature = "VKSC_VERSION_1_0", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceVulkanSC10Features<'child>
{
}
#[cfg(feature = "VKSC_VERSION_1_0")]
impl<'a> VkPhysicalDeviceVulkanSC10Features<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_VULKAN_SC_1_0_FEATURES,
    pNext: core::ptr::null_mut(),
    shaderAtomicInstructions: 0,
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
  pub const fn with_shaderAtomicInstructions(mut self, val: VkBool32) -> Self {
    self.shaderAtomicInstructions = val;
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
