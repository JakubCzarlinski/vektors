#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBool32;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkComputePipelineCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceSize;
#[cfg(feature = "VK_EXT_hdr_metadata")]
use crate::types::VkHdrMetadataEXT;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceFeatures2;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceProperties2;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkRenderPass;
use core::ffi::c_void;
/// [VkPhysicalDeviceClusterCullingShaderPropertiesHUAWEI](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceClusterCullingShaderPropertiesHUAWEI.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceClusterCullingShaderPropertiesHUAWEI<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_PROPERTIES_HUAWEI
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max, Pot]
  pub maxWorkGroupCount: [u32; 3],
  /// Limit Type: [Max, Pot]
  pub maxWorkGroupSize: [u32; 3],
  /// Limit Type: [Max]
  pub maxOutputClusterCount: u32,
  /// Limit Type: [Exact]
  pub indirectBufferOffsetAlignment: VkDeviceSize,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
unsafe impl<'a> Send for VkPhysicalDeviceClusterCullingShaderPropertiesHUAWEI<'a> {}
#[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
unsafe impl<'a> Sync for VkPhysicalDeviceClusterCullingShaderPropertiesHUAWEI<'a> {}
#[cfg(all(
  feature = "VK_HUAWEI_cluster_culling_shader",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceClusterCullingShaderPropertiesHUAWEI<'child>
{
}
#[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
impl<'a> VkPhysicalDeviceClusterCullingShaderPropertiesHUAWEI<'a> {
  pub const DEFAULT: Self = Self {
    sType:
      VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_PROPERTIES_HUAWEI,
    pNext: core::ptr::null_mut(),
    maxWorkGroupCount: [0u32; 3],
    maxWorkGroupSize: [0u32; 3],
    maxOutputClusterCount: 0,
    indirectBufferOffsetAlignment: 0,
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
  pub const fn with_maxWorkGroupCount(mut self, val: [u32; 3]) -> Self {
    self.maxWorkGroupCount = val;
    self
  }
  #[inline]
  pub const fn with_maxWorkGroupSize(mut self, val: [u32; 3]) -> Self {
    self.maxWorkGroupSize = val;
    self
  }
  #[inline]
  pub const fn with_maxOutputClusterCount(mut self, val: u32) -> Self {
    self.maxOutputClusterCount = val;
    self
  }
  #[inline]
  pub const fn with_indirectBufferOffsetAlignment(mut self, val: VkDeviceSize) -> Self {
    self.indirectBufferOffsetAlignment = val;
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
/// [VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_FEATURES_HUAWEI
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub clustercullingShader: VkBool32,
  pub multiviewClusterCullingShader: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
unsafe impl<'a> Send for VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI<'a> {}
#[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
unsafe impl<'a> Sync for VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI<'a> {}
#[cfg(all(
  feature = "VK_HUAWEI_cluster_culling_shader",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI<'child>
{
}
#[cfg(all(
  feature = "VK_HUAWEI_cluster_culling_shader",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI<'child>
{
}
#[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
impl<'a> VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI<'a> {
  pub const DEFAULT: Self = Self {
    sType:
      VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_FEATURES_HUAWEI,
    pNext: core::ptr::null_mut(),
    clustercullingShader: 0,
    multiviewClusterCullingShader: 0,
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
  pub const fn with_clustercullingShader(mut self, val: VkBool32) -> Self {
    self.clustercullingShader = val;
    self
  }
  #[inline]
  pub const fn with_multiviewClusterCullingShader(mut self, val: VkBool32) -> Self {
    self.multiviewClusterCullingShader = val;
    self
  }
  #[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI<'child>).cast::<c_void>();
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
/// [VkPhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI.html)
///
/// **Extends:** VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI.
#[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_VRS_FEATURES_HUAWEI
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub clusterShadingRate: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
unsafe impl<'a> Send for VkPhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI<'a> {}
#[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
unsafe impl<'a> Sync for VkPhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI<'a> {}
#[cfg(all(
  feature = "VK_HUAWEI_cluster_culling_shader",
  feature = "VK_HUAWEI_cluster_culling_shader"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI<'root>>
  for VkPhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI<'child>
{
}
#[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
impl<'a> VkPhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI<'a> {
  pub const DEFAULT: Self = Self {
    sType:
      VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_VRS_FEATURES_HUAWEI,
    pNext: core::ptr::null_mut(),
    clusterShadingRate: 0,
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
  pub const fn with_clusterShadingRate(mut self, val: VkBool32) -> Self {
    self.clusterShadingRate = val;
    self
  }
  #[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI<
    'root,
    T: VkPNextExtends<VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkHdrVividDynamicMetadataHUAWEI](https://docs.vulkan.org/refpages/latest/refpages/source/VkHdrVividDynamicMetadataHUAWEI.html)
///
/// **Extends:** VkHdrMetadataEXT.
#[cfg(feature = "VK_HUAWEI_hdr_vivid")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkHdrVividDynamicMetadataHUAWEI<'a> {
  /// Values: VK_STRUCTURE_TYPE_HDR_VIVID_DYNAMIC_METADATA_HUAWEI
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub dynamicMetadataSize: usize,
  /// Length: dynamicMetadataSize
  pub pDynamicMetadata: *const c_void,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_HUAWEI_hdr_vivid")]
unsafe impl<'a> Send for VkHdrVividDynamicMetadataHUAWEI<'a> {}
#[cfg(feature = "VK_HUAWEI_hdr_vivid")]
unsafe impl<'a> Sync for VkHdrVividDynamicMetadataHUAWEI<'a> {}
#[cfg(all(feature = "VK_HUAWEI_hdr_vivid", feature = "VK_EXT_hdr_metadata"))]
unsafe impl<'child, 'root> VkPNextExtends<VkHdrMetadataEXT<'root>>
  for VkHdrVividDynamicMetadataHUAWEI<'child>
{
}
#[cfg(feature = "VK_HUAWEI_hdr_vivid")]
impl<'a> VkHdrVividDynamicMetadataHUAWEI<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_HDR_VIVID_DYNAMIC_METADATA_HUAWEI,
    pNext: core::ptr::null(),
    dynamicMetadataSize: 0,
    pDynamicMetadata: core::ptr::null(),
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
  pub const fn with_dynamicMetadataSize(mut self, val: usize) -> Self {
    self.dynamicMetadataSize = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pDynamicMetadata(mut self, val: &'a [u8]) -> Self {
    self.dynamicMetadataSize = val.len() as usize;
    self.pDynamicMetadata = val.as_ptr().cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_hdr_metadata")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkHdrMetadataEXT<
    'root,
    T: VkPNextExtends<VkHdrMetadataEXT<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceHdrVividFeaturesHUAWEI](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceHdrVividFeaturesHUAWEI.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_HUAWEI_hdr_vivid")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceHdrVividFeaturesHUAWEI<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HDR_VIVID_FEATURES_HUAWEI
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub hdrVivid: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_HUAWEI_hdr_vivid")]
unsafe impl<'a> Send for VkPhysicalDeviceHdrVividFeaturesHUAWEI<'a> {}
#[cfg(feature = "VK_HUAWEI_hdr_vivid")]
unsafe impl<'a> Sync for VkPhysicalDeviceHdrVividFeaturesHUAWEI<'a> {}
#[cfg(all(feature = "VK_HUAWEI_hdr_vivid", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceHdrVividFeaturesHUAWEI<'child>
{
}
#[cfg(all(feature = "VK_HUAWEI_hdr_vivid", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceHdrVividFeaturesHUAWEI<'child>
{
}
#[cfg(feature = "VK_HUAWEI_hdr_vivid")]
impl<'a> VkPhysicalDeviceHdrVividFeaturesHUAWEI<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HDR_VIVID_FEATURES_HUAWEI,
    pNext: core::ptr::null_mut(),
    hdrVivid: 0,
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
  pub const fn with_hdrVivid(mut self, val: VkBool32) -> Self {
    self.hdrVivid = val;
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
/// [VkPhysicalDeviceInvocationMaskFeaturesHUAWEI](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceInvocationMaskFeaturesHUAWEI.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_HUAWEI_invocation_mask")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceInvocationMaskFeaturesHUAWEI<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub invocationMask: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_HUAWEI_invocation_mask")]
unsafe impl<'a> Send for VkPhysicalDeviceInvocationMaskFeaturesHUAWEI<'a> {}
#[cfg(feature = "VK_HUAWEI_invocation_mask")]
unsafe impl<'a> Sync for VkPhysicalDeviceInvocationMaskFeaturesHUAWEI<'a> {}
#[cfg(all(feature = "VK_HUAWEI_invocation_mask", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceInvocationMaskFeaturesHUAWEI<'child>
{
}
#[cfg(all(feature = "VK_HUAWEI_invocation_mask", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceInvocationMaskFeaturesHUAWEI<'child>
{
}
#[cfg(feature = "VK_HUAWEI_invocation_mask")]
impl<'a> VkPhysicalDeviceInvocationMaskFeaturesHUAWEI<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI,
    pNext: core::ptr::null_mut(),
    invocationMask: 0,
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
  pub const fn with_invocationMask(mut self, val: VkBool32) -> Self {
    self.invocationMask = val;
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
/// [VkSubpassShadingPipelineCreateInfoHUAWEI](https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassShadingPipelineCreateInfoHUAWEI.html)
///
/// **Extends:** VkComputePipelineCreateInfo.
#[cfg(feature = "VK_HUAWEI_subpass_shading")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSubpassShadingPipelineCreateInfoHUAWEI<'a> {
  /// Values: VK_STRUCTURE_TYPE_SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  pub renderPass: VkRenderPass,
  #[cfg(not(feature = "VK_GRAPHICS_VERSION_1_0"))]
  pub renderPass: *mut c_void,
  pub subpass: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_HUAWEI_subpass_shading")]
unsafe impl<'a> Send for VkSubpassShadingPipelineCreateInfoHUAWEI<'a> {}
#[cfg(feature = "VK_HUAWEI_subpass_shading")]
unsafe impl<'a> Sync for VkSubpassShadingPipelineCreateInfoHUAWEI<'a> {}
#[cfg(all(
  feature = "VK_HUAWEI_subpass_shading",
  feature = "VK_COMPUTE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkComputePipelineCreateInfo<'root>>
  for VkSubpassShadingPipelineCreateInfoHUAWEI<'child>
{
}
#[cfg(feature = "VK_HUAWEI_subpass_shading")]
impl<'a> VkSubpassShadingPipelineCreateInfoHUAWEI<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI,
    pNext: core::ptr::null_mut(),
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    renderPass: VkRenderPass::DEFAULT,
    #[cfg(not(feature = "VK_GRAPHICS_VERSION_1_0"))]
    renderPass: core::ptr::null_mut(),
    subpass: 0,
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
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  #[inline]
  pub const fn with_renderPass(mut self, val: VkRenderPass) -> Self {
    self.renderPass = val;
    self
  }
  #[inline]
  pub const fn with_subpass(mut self, val: u32) -> Self {
    self.subpass = val;
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
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceSubpassShadingPropertiesHUAWEI](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSubpassShadingPropertiesHUAWEI.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_HUAWEI_subpass_shading")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceSubpassShadingPropertiesHUAWEI<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max, Pot]
  pub maxSubpassShadingWorkgroupSizeAspectRatio: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_HUAWEI_subpass_shading")]
unsafe impl<'a> Send for VkPhysicalDeviceSubpassShadingPropertiesHUAWEI<'a> {}
#[cfg(feature = "VK_HUAWEI_subpass_shading")]
unsafe impl<'a> Sync for VkPhysicalDeviceSubpassShadingPropertiesHUAWEI<'a> {}
#[cfg(all(feature = "VK_HUAWEI_subpass_shading", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceSubpassShadingPropertiesHUAWEI<'child>
{
}
#[cfg(feature = "VK_HUAWEI_subpass_shading")]
impl<'a> VkPhysicalDeviceSubpassShadingPropertiesHUAWEI<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI,
    pNext: core::ptr::null_mut(),
    maxSubpassShadingWorkgroupSizeAspectRatio: 0,
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
  pub const fn with_maxSubpassShadingWorkgroupSizeAspectRatio(mut self, val: u32) -> Self {
    self.maxSubpassShadingWorkgroupSizeAspectRatio = val;
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
/// [VkPhysicalDeviceSubpassShadingFeaturesHUAWEI](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSubpassShadingFeaturesHUAWEI.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_HUAWEI_subpass_shading")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceSubpassShadingFeaturesHUAWEI<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub subpassShading: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_HUAWEI_subpass_shading")]
unsafe impl<'a> Send for VkPhysicalDeviceSubpassShadingFeaturesHUAWEI<'a> {}
#[cfg(feature = "VK_HUAWEI_subpass_shading")]
unsafe impl<'a> Sync for VkPhysicalDeviceSubpassShadingFeaturesHUAWEI<'a> {}
#[cfg(all(feature = "VK_HUAWEI_subpass_shading", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceSubpassShadingFeaturesHUAWEI<'child>
{
}
#[cfg(all(feature = "VK_HUAWEI_subpass_shading", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceSubpassShadingFeaturesHUAWEI<'child>
{
}
#[cfg(feature = "VK_HUAWEI_subpass_shading")]
impl<'a> VkPhysicalDeviceSubpassShadingFeaturesHUAWEI<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI,
    pNext: core::ptr::null_mut(),
    subpassShading: 0,
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
  pub const fn with_subpassShading(mut self, val: VkBool32) -> Self {
    self.subpassShading = val;
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
