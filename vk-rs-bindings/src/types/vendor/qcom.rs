use crate::consts::VK_DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM;
#[cfg(feature = "VK_QCOM_image_processing2")]
use crate::enums::VkBlockMatchWindowCompareModeQCOM;
#[cfg(feature = "VK_QCOM_filter_cubic_weights")]
use crate::enums::VkCubicFilterWeightsQCOM;
#[cfg(feature = "VK_QCOM_data_graph_model")]
use crate::enums::VkDataGraphModelCacheTypeQCOM;
#[cfg(feature = "VK_QCOM_queue_perf_hint")]
use crate::enums::VkPerfHintTypeQCOM;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::enums::VkPipelineCacheHeaderVersion;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkQueueFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(feature = "VK_KHR_surface")]
use crate::enums::VkSurfaceTransformFlagBitsKHR;
#[cfg(feature = "VK_QCOM_tile_shading")]
use crate::enums::VkTileShadingRenderPassFlagBitsQCOM;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
use crate::types::VkBlitImageInfo2;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBool32;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkBufferImageCopy2;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkCommandBufferInheritanceInfo;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkDataGraphPipelineCreateInfoARM;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceMemory;
#[cfg(feature = "VK_KHR_device_address_commands")]
use crate::types::VkDeviceMemoryImageCopyKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceSize;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkExtent2D;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkExtent3D;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
use crate::types::VkImageBlit2;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageViewCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkMemoryRequirements2;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkOffset2D;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkPhysicalDeviceDataGraphOperationSupportARM;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceFeatures2;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceProperties2;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkQueueFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkRect2D;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkRenderPassBeginInfo;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkRenderPassCreateInfo;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
use crate::types::VkRenderPassCreateInfo2;
#[cfg(feature = "VK_KHR_maintenance10")]
use crate::types::VkRenderingEndInfoKHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
use crate::types::VkRenderingInfo;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkSamplerCreateInfo;
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
use crate::types::VkSamplerYcbcrConversionCreateInfo;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
use crate::types::VkSubpassEndInfo;
use core::ffi::c_void;
/// [VkPhysicalDeviceCooperativeMatrixConversionFeaturesQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCooperativeMatrixConversionFeaturesQCOM.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_QCOM_cooperative_matrix_conversion")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceCooperativeMatrixConversionFeaturesQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_CONVERSION_FEATURES_QCOM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub cooperativeMatrixConversion: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_cooperative_matrix_conversion")]
unsafe impl<'a> Send for VkPhysicalDeviceCooperativeMatrixConversionFeaturesQCOM<'a> {}
#[cfg(feature = "VK_QCOM_cooperative_matrix_conversion")]
unsafe impl<'a> Sync for VkPhysicalDeviceCooperativeMatrixConversionFeaturesQCOM<'a> {}
#[cfg(all(
  feature = "VK_QCOM_cooperative_matrix_conversion",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceCooperativeMatrixConversionFeaturesQCOM<'child>
{
}
#[cfg(all(
  feature = "VK_QCOM_cooperative_matrix_conversion",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceCooperativeMatrixConversionFeaturesQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_cooperative_matrix_conversion")]
impl<'a> VkPhysicalDeviceCooperativeMatrixConversionFeaturesQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_CONVERSION_FEATURES_QCOM,
    pNext: core::ptr::null_mut(),
    cooperativeMatrixConversion: 0,
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
  pub const fn with_cooperativeMatrixConversion(mut self, val: VkBool32) -> Self {
    self.cooperativeMatrixConversion = val;
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
/// [VkPipelineCacheHeaderVersionDataGraphQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCacheHeaderVersionDataGraphQCOM.html)
#[cfg(feature = "VK_QCOM_data_graph_model")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineCacheHeaderVersionDataGraphQCOM {
  pub headerSize: u32,
  pub headerVersion: VkPipelineCacheHeaderVersion,
  pub cacheType: VkDataGraphModelCacheTypeQCOM,
  pub cacheVersion: u32,
  pub toolchainVersion: [u32; VK_DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM as usize],
}
#[cfg(feature = "VK_QCOM_data_graph_model")]
unsafe impl Send for VkPipelineCacheHeaderVersionDataGraphQCOM {}
#[cfg(feature = "VK_QCOM_data_graph_model")]
unsafe impl Sync for VkPipelineCacheHeaderVersionDataGraphQCOM {}
#[cfg(feature = "VK_QCOM_data_graph_model")]
impl VkPipelineCacheHeaderVersionDataGraphQCOM {
  pub const DEFAULT: Self = Self {
    headerSize: 0,
    headerVersion: VkPipelineCacheHeaderVersion(0),
    cacheType: VkDataGraphModelCacheTypeQCOM(0),
    cacheVersion: 0,
    toolchainVersion: [0u32; VK_DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM as usize],
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_headerSize(mut self, val: u32) -> Self {
    self.headerSize = val;
    self
  }
  #[inline]
  pub const fn with_headerVersion(mut self, val: VkPipelineCacheHeaderVersion) -> Self {
    self.headerVersion = val;
    self
  }
  #[inline]
  pub const fn with_cacheType(mut self, val: VkDataGraphModelCacheTypeQCOM) -> Self {
    self.cacheType = val;
    self
  }
  #[inline]
  pub const fn with_cacheVersion(mut self, val: u32) -> Self {
    self.cacheVersion = val;
    self
  }
  #[inline]
  pub const fn with_toolchainVersion(
    mut self,
    val: [u32; VK_DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM as usize],
  ) -> Self {
    self.toolchainVersion = val;
    self
  }
}
/// [VkDataGraphPipelineBuiltinModelCreateInfoQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineBuiltinModelCreateInfoQCOM.html)
///
/// **Extends:** VkDataGraphPipelineCreateInfoARM.
#[cfg(feature = "VK_QCOM_data_graph_model")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDataGraphPipelineBuiltinModelCreateInfoQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_BUILTIN_MODEL_CREATE_INFO_QCOM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub pOperation: *const VkPhysicalDeviceDataGraphOperationSupportARM,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_data_graph_model")]
unsafe impl<'a> Send for VkDataGraphPipelineBuiltinModelCreateInfoQCOM<'a> {}
#[cfg(feature = "VK_QCOM_data_graph_model")]
unsafe impl<'a> Sync for VkDataGraphPipelineBuiltinModelCreateInfoQCOM<'a> {}
#[cfg(all(feature = "VK_QCOM_data_graph_model", feature = "VK_ARM_data_graph"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDataGraphPipelineCreateInfoARM<'root>>
  for VkDataGraphPipelineBuiltinModelCreateInfoQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_data_graph_model")]
impl<'a> VkDataGraphPipelineBuiltinModelCreateInfoQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::DATA_GRAPH_PIPELINE_BUILTIN_MODEL_CREATE_INFO_QCOM,
    pNext: core::ptr::null(),
    pOperation: core::ptr::null(),
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
  pub const fn with_pOperation(
    mut self,
    val: &'a VkPhysicalDeviceDataGraphOperationSupportARM,
  ) -> Self {
    self.pOperation = val as *const VkPhysicalDeviceDataGraphOperationSupportARM;
    self
  }
  #[cfg(feature = "VK_ARM_data_graph")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDataGraphPipelineCreateInfoARM<
    'root,
    T: VkPNextExtends<VkDataGraphPipelineCreateInfoARM<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceDataGraphModelFeaturesQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDataGraphModelFeaturesQCOM.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_QCOM_data_graph_model")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceDataGraphModelFeaturesQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DATA_GRAPH_MODEL_FEATURES_QCOM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub dataGraphModel: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_data_graph_model")]
unsafe impl<'a> Send for VkPhysicalDeviceDataGraphModelFeaturesQCOM<'a> {}
#[cfg(feature = "VK_QCOM_data_graph_model")]
unsafe impl<'a> Sync for VkPhysicalDeviceDataGraphModelFeaturesQCOM<'a> {}
#[cfg(all(feature = "VK_QCOM_data_graph_model", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceDataGraphModelFeaturesQCOM<'child>
{
}
#[cfg(all(feature = "VK_QCOM_data_graph_model", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceDataGraphModelFeaturesQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_data_graph_model")]
impl<'a> VkPhysicalDeviceDataGraphModelFeaturesQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_DATA_GRAPH_MODEL_FEATURES_QCOM,
    pNext: core::ptr::null_mut(),
    dataGraphModel: 0,
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
  pub const fn with_dataGraphModel(mut self, val: VkBool32) -> Self {
    self.dataGraphModel = val;
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
/// [VkPhysicalDeviceElapsedTimerQueryFeaturesQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceElapsedTimerQueryFeaturesQCOM.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_QCOM_elapsed_timer_query")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceElapsedTimerQueryFeaturesQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ELAPSED_TIMER_QUERY_FEATURES_QCOM
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub elapsedTimerQuery: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_elapsed_timer_query")]
unsafe impl<'a> Send for VkPhysicalDeviceElapsedTimerQueryFeaturesQCOM<'a> {}
#[cfg(feature = "VK_QCOM_elapsed_timer_query")]
unsafe impl<'a> Sync for VkPhysicalDeviceElapsedTimerQueryFeaturesQCOM<'a> {}
#[cfg(all(
  feature = "VK_QCOM_elapsed_timer_query",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceElapsedTimerQueryFeaturesQCOM<'child>
{
}
#[cfg(all(
  feature = "VK_QCOM_elapsed_timer_query",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceElapsedTimerQueryFeaturesQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_elapsed_timer_query")]
impl<'a> VkPhysicalDeviceElapsedTimerQueryFeaturesQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_ELAPSED_TIMER_QUERY_FEATURES_QCOM,
    pNext: core::ptr::null_mut(),
    elapsedTimerQuery: 0,
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
  pub const fn with_elapsedTimerQuery(mut self, val: VkBool32) -> Self {
    self.elapsedTimerQuery = val;
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
/// [VkPhysicalDeviceCubicClampFeaturesQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCubicClampFeaturesQCOM.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_QCOM_filter_cubic_clamp")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceCubicClampFeaturesQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUBIC_CLAMP_FEATURES_QCOM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub cubicRangeClamp: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_filter_cubic_clamp")]
unsafe impl<'a> Send for VkPhysicalDeviceCubicClampFeaturesQCOM<'a> {}
#[cfg(feature = "VK_QCOM_filter_cubic_clamp")]
unsafe impl<'a> Sync for VkPhysicalDeviceCubicClampFeaturesQCOM<'a> {}
#[cfg(all(
  feature = "VK_QCOM_filter_cubic_clamp",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceCubicClampFeaturesQCOM<'child>
{
}
#[cfg(all(
  feature = "VK_QCOM_filter_cubic_clamp",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceCubicClampFeaturesQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_filter_cubic_clamp")]
impl<'a> VkPhysicalDeviceCubicClampFeaturesQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_CUBIC_CLAMP_FEATURES_QCOM,
    pNext: core::ptr::null_mut(),
    cubicRangeClamp: 0,
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
  pub const fn with_cubicRangeClamp(mut self, val: VkBool32) -> Self {
    self.cubicRangeClamp = val;
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
/// [VkPhysicalDeviceCubicWeightsFeaturesQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCubicWeightsFeaturesQCOM.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_QCOM_filter_cubic_weights")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceCubicWeightsFeaturesQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUBIC_WEIGHTS_FEATURES_QCOM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub selectableCubicWeights: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_filter_cubic_weights")]
unsafe impl<'a> Send for VkPhysicalDeviceCubicWeightsFeaturesQCOM<'a> {}
#[cfg(feature = "VK_QCOM_filter_cubic_weights")]
unsafe impl<'a> Sync for VkPhysicalDeviceCubicWeightsFeaturesQCOM<'a> {}
#[cfg(all(
  feature = "VK_QCOM_filter_cubic_weights",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceCubicWeightsFeaturesQCOM<'child>
{
}
#[cfg(all(
  feature = "VK_QCOM_filter_cubic_weights",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceCubicWeightsFeaturesQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_filter_cubic_weights")]
impl<'a> VkPhysicalDeviceCubicWeightsFeaturesQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_CUBIC_WEIGHTS_FEATURES_QCOM,
    pNext: core::ptr::null_mut(),
    selectableCubicWeights: 0,
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
  pub const fn with_selectableCubicWeights(mut self, val: VkBool32) -> Self {
    self.selectableCubicWeights = val;
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
/// [VkSamplerCubicWeightsCreateInfoQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerCubicWeightsCreateInfoQCOM.html)
///
/// **Extends:** VkSamplerCreateInfo.
#[cfg(feature = "VK_QCOM_filter_cubic_weights")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSamplerCubicWeightsCreateInfoQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_SAMPLER_CUBIC_WEIGHTS_CREATE_INFO_QCOM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub cubicWeights: VkCubicFilterWeightsQCOM,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_filter_cubic_weights")]
unsafe impl<'a> Send for VkSamplerCubicWeightsCreateInfoQCOM<'a> {}
#[cfg(feature = "VK_QCOM_filter_cubic_weights")]
unsafe impl<'a> Sync for VkSamplerCubicWeightsCreateInfoQCOM<'a> {}
#[cfg(all(
  feature = "VK_QCOM_filter_cubic_weights",
  feature = "VK_COMPUTE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkSamplerCreateInfo<'root>>
  for VkSamplerCubicWeightsCreateInfoQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_filter_cubic_weights")]
impl<'a> VkSamplerCubicWeightsCreateInfoQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SAMPLER_CUBIC_WEIGHTS_CREATE_INFO_QCOM,
    pNext: core::ptr::null(),
    cubicWeights: VkCubicFilterWeightsQCOM(0),
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
  pub const fn with_cubicWeights(mut self, val: VkCubicFilterWeightsQCOM) -> Self {
    self.cubicWeights = val;
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSamplerCreateInfo<
    'root,
    T: VkPNextExtends<VkSamplerCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkBlitImageCubicWeightsInfoQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkBlitImageCubicWeightsInfoQCOM.html)
///
/// **Extends:** VkBlitImageInfo2.
#[cfg(feature = "VK_QCOM_filter_cubic_weights")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBlitImageCubicWeightsInfoQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_BLIT_IMAGE_CUBIC_WEIGHTS_INFO_QCOM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub cubicWeights: VkCubicFilterWeightsQCOM,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_filter_cubic_weights")]
unsafe impl<'a> Send for VkBlitImageCubicWeightsInfoQCOM<'a> {}
#[cfg(feature = "VK_QCOM_filter_cubic_weights")]
unsafe impl<'a> Sync for VkBlitImageCubicWeightsInfoQCOM<'a> {}
#[cfg(all(
  feature = "VK_QCOM_filter_cubic_weights",
  feature = "VK_GRAPHICS_VERSION_1_3"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkBlitImageInfo2<'root>>
  for VkBlitImageCubicWeightsInfoQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_filter_cubic_weights")]
impl<'a> VkBlitImageCubicWeightsInfoQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::BLIT_IMAGE_CUBIC_WEIGHTS_INFO_QCOM,
    pNext: core::ptr::null(),
    cubicWeights: VkCubicFilterWeightsQCOM(0),
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
  pub const fn with_cubicWeights(mut self, val: VkCubicFilterWeightsQCOM) -> Self {
    self.cubicWeights = val;
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkBlitImageInfo2<
    'root,
    T: VkPNextExtends<VkBlitImageInfo2<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_QCOM_fragment_density_map_offset")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_EXT
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub fragmentDensityMapOffset: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_fragment_density_map_offset")]
unsafe impl<'a> Send for VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM<'a> {}
#[cfg(feature = "VK_QCOM_fragment_density_map_offset")]
unsafe impl<'a> Sync for VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM<'a> {}
#[cfg(all(
  feature = "VK_QCOM_fragment_density_map_offset",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM<'child>
{
}
#[cfg(all(
  feature = "VK_QCOM_fragment_density_map_offset",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_fragment_density_map_offset")]
impl<'a> VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_QCOM,
    pNext: core::ptr::null_mut(),
    fragmentDensityMapOffset: 0,
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
  pub const fn with_fragmentDensityMapOffset(mut self, val: VkBool32) -> Self {
    self.fragmentDensityMapOffset = val;
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
/// [VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_QCOM_fragment_density_map_offset")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_EXT
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Min, Mul]
  pub fragmentDensityOffsetGranularity: VkExtent2D,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_fragment_density_map_offset")]
unsafe impl<'a> Send for VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM<'a> {}
#[cfg(feature = "VK_QCOM_fragment_density_map_offset")]
unsafe impl<'a> Sync for VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM<'a> {}
#[cfg(all(
  feature = "VK_QCOM_fragment_density_map_offset",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_fragment_density_map_offset")]
impl<'a> VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_QCOM,
    pNext: core::ptr::null_mut(),
    fragmentDensityOffsetGranularity: VkExtent2D::DEFAULT,
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
  pub const fn with_fragmentDensityOffsetGranularity(mut self, val: VkExtent2D) -> Self {
    self.fragmentDensityOffsetGranularity = val;
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
/// [VkSubpassFragmentDensityMapOffsetEndInfoQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassFragmentDensityMapOffsetEndInfoQCOM.html)
///
/// **Extends:** VkSubpassEndInfo, VkRenderingEndInfoKHR.
#[cfg(feature = "VK_QCOM_fragment_density_map_offset")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSubpassFragmentDensityMapOffsetEndInfoQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_RENDER_PASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_EXT
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub fragmentDensityOffsetCount: u32,
  /// Length: fragmentDensityOffsetCount
  pub pFragmentDensityOffsets: *const VkOffset2D,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_fragment_density_map_offset")]
unsafe impl<'a> Send for VkSubpassFragmentDensityMapOffsetEndInfoQCOM<'a> {}
#[cfg(feature = "VK_QCOM_fragment_density_map_offset")]
unsafe impl<'a> Sync for VkSubpassFragmentDensityMapOffsetEndInfoQCOM<'a> {}
#[cfg(all(
  feature = "VK_QCOM_fragment_density_map_offset",
  feature = "VK_GRAPHICS_VERSION_1_2"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkSubpassEndInfo<'root>>
  for VkSubpassFragmentDensityMapOffsetEndInfoQCOM<'child>
{
}
#[cfg(all(
  feature = "VK_QCOM_fragment_density_map_offset",
  feature = "VK_KHR_maintenance10"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkRenderingEndInfoKHR<'root>>
  for VkSubpassFragmentDensityMapOffsetEndInfoQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_fragment_density_map_offset")]
impl<'a> VkSubpassFragmentDensityMapOffsetEndInfoQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SUBPASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_QCOM,
    pNext: core::ptr::null(),
    fragmentDensityOffsetCount: 0,
    pFragmentDensityOffsets: core::ptr::null(),
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
  pub const fn with_fragmentDensityOffsetCount(mut self, val: u32) -> Self {
    self.fragmentDensityOffsetCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pFragmentDensityOffsets(mut self, val: &'a [VkOffset2D]) -> Self {
    self.fragmentDensityOffsetCount = val.len() as u32;
    self.pFragmentDensityOffsets = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSubpassEndInfo<
    'root,
    T: VkPNextExtends<VkSubpassEndInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_maintenance10")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkRenderingEndInfoKHR<
    'root,
    T: VkPNextExtends<VkRenderingEndInfoKHR<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkImageViewSampleWeightCreateInfoQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageViewSampleWeightCreateInfoQCOM.html)
///
/// **Extends:** VkImageViewCreateInfo.
#[cfg(feature = "VK_QCOM_image_processing")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImageViewSampleWeightCreateInfoQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMAGE_VIEW_SAMPLE_WEIGHT_CREATE_INFO_QCOM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub filterCenter: VkOffset2D,
  pub filterSize: VkExtent2D,
  pub numPhases: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_image_processing")]
unsafe impl<'a> Send for VkImageViewSampleWeightCreateInfoQCOM<'a> {}
#[cfg(feature = "VK_QCOM_image_processing")]
unsafe impl<'a> Sync for VkImageViewSampleWeightCreateInfoQCOM<'a> {}
#[cfg(all(feature = "VK_QCOM_image_processing", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkImageViewCreateInfo<'root>>
  for VkImageViewSampleWeightCreateInfoQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_image_processing")]
impl<'a> VkImageViewSampleWeightCreateInfoQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::IMAGE_VIEW_SAMPLE_WEIGHT_CREATE_INFO_QCOM,
    pNext: core::ptr::null(),
    filterCenter: VkOffset2D::DEFAULT,
    filterSize: VkExtent2D::DEFAULT,
    numPhases: 0,
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
  pub const fn with_filterCenter(mut self, val: VkOffset2D) -> Self {
    self.filterCenter = val;
    self
  }
  #[inline]
  pub const fn with_filterSize(mut self, val: VkExtent2D) -> Self {
    self.filterSize = val;
    self
  }
  #[inline]
  pub const fn with_numPhases(mut self, val: u32) -> Self {
    self.numPhases = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkImageViewCreateInfo<
    'root,
    T: VkPNextExtends<VkImageViewCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceImageProcessingFeaturesQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceImageProcessingFeaturesQCOM.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_QCOM_image_processing")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceImageProcessingFeaturesQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_PROCESSING_FEATURES_QCOM
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub textureSampleWeighted: VkBool32,
  pub textureBoxFilter: VkBool32,
  pub textureBlockMatch: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_image_processing")]
unsafe impl<'a> Send for VkPhysicalDeviceImageProcessingFeaturesQCOM<'a> {}
#[cfg(feature = "VK_QCOM_image_processing")]
unsafe impl<'a> Sync for VkPhysicalDeviceImageProcessingFeaturesQCOM<'a> {}
#[cfg(all(feature = "VK_QCOM_image_processing", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceImageProcessingFeaturesQCOM<'child>
{
}
#[cfg(all(feature = "VK_QCOM_image_processing", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceImageProcessingFeaturesQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_image_processing")]
impl<'a> VkPhysicalDeviceImageProcessingFeaturesQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_IMAGE_PROCESSING_FEATURES_QCOM,
    pNext: core::ptr::null_mut(),
    textureSampleWeighted: 0,
    textureBoxFilter: 0,
    textureBlockMatch: 0,
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
  pub const fn with_textureSampleWeighted(mut self, val: VkBool32) -> Self {
    self.textureSampleWeighted = val;
    self
  }
  #[inline]
  pub const fn with_textureBoxFilter(mut self, val: VkBool32) -> Self {
    self.textureBoxFilter = val;
    self
  }
  #[inline]
  pub const fn with_textureBlockMatch(mut self, val: VkBool32) -> Self {
    self.textureBlockMatch = val;
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
/// [VkPhysicalDeviceImageProcessingPropertiesQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceImageProcessingPropertiesQCOM.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_QCOM_image_processing")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceImageProcessingPropertiesQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_PROCESSING_PROPERTIES_QCOM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Optional: true,  Limit Type: [Max]
  pub maxWeightFilterPhases: u32,
  /// Optional: true,  Limit Type: [Max]
  pub maxWeightFilterDimension: VkExtent2D,
  /// Optional: true,  Limit Type: [Max]
  pub maxBlockMatchRegion: VkExtent2D,
  /// Optional: true,  Limit Type: [Max]
  pub maxBoxFilterBlockSize: VkExtent2D,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_image_processing")]
unsafe impl<'a> Send for VkPhysicalDeviceImageProcessingPropertiesQCOM<'a> {}
#[cfg(feature = "VK_QCOM_image_processing")]
unsafe impl<'a> Sync for VkPhysicalDeviceImageProcessingPropertiesQCOM<'a> {}
#[cfg(all(feature = "VK_QCOM_image_processing", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceImageProcessingPropertiesQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_image_processing")]
impl<'a> VkPhysicalDeviceImageProcessingPropertiesQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_IMAGE_PROCESSING_PROPERTIES_QCOM,
    pNext: core::ptr::null_mut(),
    maxWeightFilterPhases: 0,
    maxWeightFilterDimension: VkExtent2D::DEFAULT,
    maxBlockMatchRegion: VkExtent2D::DEFAULT,
    maxBoxFilterBlockSize: VkExtent2D::DEFAULT,
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
  pub const fn with_maxWeightFilterPhases(mut self, val: u32) -> Self {
    self.maxWeightFilterPhases = val;
    self
  }
  #[inline]
  pub const fn with_maxWeightFilterDimension(mut self, val: VkExtent2D) -> Self {
    self.maxWeightFilterDimension = val;
    self
  }
  #[inline]
  pub const fn with_maxBlockMatchRegion(mut self, val: VkExtent2D) -> Self {
    self.maxBlockMatchRegion = val;
    self
  }
  #[inline]
  pub const fn with_maxBoxFilterBlockSize(mut self, val: VkExtent2D) -> Self {
    self.maxBoxFilterBlockSize = val;
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
/// [VkPhysicalDeviceImageProcessing2FeaturesQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceImageProcessing2FeaturesQCOM.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_QCOM_image_processing2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceImageProcessing2FeaturesQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_PROCESSING_2_FEATURES_QCOM
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub textureBlockMatch2: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_image_processing2")]
unsafe impl<'a> Send for VkPhysicalDeviceImageProcessing2FeaturesQCOM<'a> {}
#[cfg(feature = "VK_QCOM_image_processing2")]
unsafe impl<'a> Sync for VkPhysicalDeviceImageProcessing2FeaturesQCOM<'a> {}
#[cfg(all(feature = "VK_QCOM_image_processing2", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceImageProcessing2FeaturesQCOM<'child>
{
}
#[cfg(all(feature = "VK_QCOM_image_processing2", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceImageProcessing2FeaturesQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_image_processing2")]
impl<'a> VkPhysicalDeviceImageProcessing2FeaturesQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_IMAGE_PROCESSING_2_FEATURES_QCOM,
    pNext: core::ptr::null_mut(),
    textureBlockMatch2: 0,
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
  pub const fn with_textureBlockMatch2(mut self, val: VkBool32) -> Self {
    self.textureBlockMatch2 = val;
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
/// [VkPhysicalDeviceImageProcessing2PropertiesQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceImageProcessing2PropertiesQCOM.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_QCOM_image_processing2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceImageProcessing2PropertiesQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_PROCESSING_2_PROPERTIES_QCOM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Optional: true,  Limit Type: [Max]
  pub maxBlockMatchWindow: VkExtent2D,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_image_processing2")]
unsafe impl<'a> Send for VkPhysicalDeviceImageProcessing2PropertiesQCOM<'a> {}
#[cfg(feature = "VK_QCOM_image_processing2")]
unsafe impl<'a> Sync for VkPhysicalDeviceImageProcessing2PropertiesQCOM<'a> {}
#[cfg(all(feature = "VK_QCOM_image_processing2", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceImageProcessing2PropertiesQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_image_processing2")]
impl<'a> VkPhysicalDeviceImageProcessing2PropertiesQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_IMAGE_PROCESSING_2_PROPERTIES_QCOM,
    pNext: core::ptr::null_mut(),
    maxBlockMatchWindow: VkExtent2D::DEFAULT,
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
  pub const fn with_maxBlockMatchWindow(mut self, val: VkExtent2D) -> Self {
    self.maxBlockMatchWindow = val;
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
/// [VkSamplerBlockMatchWindowCreateInfoQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerBlockMatchWindowCreateInfoQCOM.html)
///
/// **Extends:** VkSamplerCreateInfo.
#[cfg(feature = "VK_QCOM_image_processing2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSamplerBlockMatchWindowCreateInfoQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_SAMPLER_BLOCK_MATCH_WINDOW_CREATE_INFO_QCOM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub windowExtent: VkExtent2D,
  pub windowCompareMode: VkBlockMatchWindowCompareModeQCOM,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_image_processing2")]
unsafe impl<'a> Send for VkSamplerBlockMatchWindowCreateInfoQCOM<'a> {}
#[cfg(feature = "VK_QCOM_image_processing2")]
unsafe impl<'a> Sync for VkSamplerBlockMatchWindowCreateInfoQCOM<'a> {}
#[cfg(all(
  feature = "VK_QCOM_image_processing2",
  feature = "VK_COMPUTE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkSamplerCreateInfo<'root>>
  for VkSamplerBlockMatchWindowCreateInfoQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_image_processing2")]
impl<'a> VkSamplerBlockMatchWindowCreateInfoQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SAMPLER_BLOCK_MATCH_WINDOW_CREATE_INFO_QCOM,
    pNext: core::ptr::null(),
    windowExtent: VkExtent2D::DEFAULT,
    windowCompareMode: VkBlockMatchWindowCompareModeQCOM(0),
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
  pub const fn with_windowExtent(mut self, val: VkExtent2D) -> Self {
    self.windowExtent = val;
    self
  }
  #[inline]
  pub const fn with_windowCompareMode(mut self, val: VkBlockMatchWindowCompareModeQCOM) -> Self {
    self.windowCompareMode = val;
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSamplerCreateInfo<
    'root,
    T: VkPNextExtends<VkSamplerCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceImageProcessing3FeaturesQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceImageProcessing3FeaturesQCOM.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_QCOM_image_processing3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceImageProcessing3FeaturesQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_PROCESSING_3_FEATURES_QCOM
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub imageGatherLinear: VkBool32,
  pub imageGatherExtendedModes: VkBool32,
  pub blockMatchExtendedClampToEdge: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_image_processing3")]
unsafe impl<'a> Send for VkPhysicalDeviceImageProcessing3FeaturesQCOM<'a> {}
#[cfg(feature = "VK_QCOM_image_processing3")]
unsafe impl<'a> Sync for VkPhysicalDeviceImageProcessing3FeaturesQCOM<'a> {}
#[cfg(all(feature = "VK_QCOM_image_processing3", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceImageProcessing3FeaturesQCOM<'child>
{
}
#[cfg(all(feature = "VK_QCOM_image_processing3", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceImageProcessing3FeaturesQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_image_processing3")]
impl<'a> VkPhysicalDeviceImageProcessing3FeaturesQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_IMAGE_PROCESSING_3_FEATURES_QCOM,
    pNext: core::ptr::null_mut(),
    imageGatherLinear: 0,
    imageGatherExtendedModes: 0,
    blockMatchExtendedClampToEdge: 0,
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
  pub const fn with_imageGatherLinear(mut self, val: VkBool32) -> Self {
    self.imageGatherLinear = val;
    self
  }
  #[inline]
  pub const fn with_imageGatherExtendedModes(mut self, val: VkBool32) -> Self {
    self.imageGatherExtendedModes = val;
    self
  }
  #[inline]
  pub const fn with_blockMatchExtendedClampToEdge(mut self, val: VkBool32) -> Self {
    self.blockMatchExtendedClampToEdge = val;
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
/// [VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_QCOM_multiview_per_view_render_areas")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_RENDER_AREAS_FEATURES_QCOM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub multiviewPerViewRenderAreas: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_multiview_per_view_render_areas")]
unsafe impl<'a> Send for VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM<'a> {}
#[cfg(feature = "VK_QCOM_multiview_per_view_render_areas")]
unsafe impl<'a> Sync for VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM<'a> {}
#[cfg(all(
  feature = "VK_QCOM_multiview_per_view_render_areas",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM<'child>
{
}
#[cfg(all(
  feature = "VK_QCOM_multiview_per_view_render_areas",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_multiview_per_view_render_areas")]
impl<'a> VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_RENDER_AREAS_FEATURES_QCOM,
    pNext: core::ptr::null_mut(),
    multiviewPerViewRenderAreas: 0,
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
  pub const fn with_multiviewPerViewRenderAreas(mut self, val: VkBool32) -> Self {
    self.multiviewPerViewRenderAreas = val;
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
/// [VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM.html)
///
/// **Extends:** VkRenderPassBeginInfo, VkRenderingInfo.
#[cfg(feature = "VK_QCOM_multiview_per_view_render_areas")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_MULTIVIEW_PER_VIEW_RENDER_AREAS_RENDER_PASS_BEGIN_INFO_QCOM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub perViewRenderAreaCount: u32,
  /// Length: perViewRenderAreaCount
  pub pPerViewRenderAreas: *const VkRect2D,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_multiview_per_view_render_areas")]
unsafe impl<'a> Send for VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM<'a> {}
#[cfg(feature = "VK_QCOM_multiview_per_view_render_areas")]
unsafe impl<'a> Sync for VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM<'a> {}
#[cfg(all(
  feature = "VK_QCOM_multiview_per_view_render_areas",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkRenderPassBeginInfo<'root>>
  for VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM<'child>
{
}
#[cfg(all(
  feature = "VK_QCOM_multiview_per_view_render_areas",
  feature = "VK_GRAPHICS_VERSION_1_3"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkRenderingInfo<'root>>
  for VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_multiview_per_view_render_areas")]
impl<'a> VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::MULTIVIEW_PER_VIEW_RENDER_AREAS_RENDER_PASS_BEGIN_INFO_QCOM,
    pNext: core::ptr::null(),
    perViewRenderAreaCount: 0,
    pPerViewRenderAreas: core::ptr::null(),
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
  pub const fn with_perViewRenderAreaCount(mut self, val: u32) -> Self {
    self.perViewRenderAreaCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pPerViewRenderAreas(mut self, val: &'a [VkRect2D]) -> Self {
    self.perViewRenderAreaCount = val.len() as u32;
    self.pPerViewRenderAreas = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkRenderPassBeginInfo<
    'root,
    T: VkPNextExtends<VkRenderPassBeginInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkRenderingInfo<
    'root,
    T: VkPNextExtends<VkRenderingInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_QCOM_multiview_per_view_viewports")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_VIEWPORTS_FEATURES_QCOM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub multiviewPerViewViewports: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_multiview_per_view_viewports")]
unsafe impl<'a> Send for VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM<'a> {}
#[cfg(feature = "VK_QCOM_multiview_per_view_viewports")]
unsafe impl<'a> Sync for VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM<'a> {}
#[cfg(all(
  feature = "VK_QCOM_multiview_per_view_viewports",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM<'child>
{
}
#[cfg(all(
  feature = "VK_QCOM_multiview_per_view_viewports",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_multiview_per_view_viewports")]
impl<'a> VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_VIEWPORTS_FEATURES_QCOM,
    pNext: core::ptr::null_mut(),
    multiviewPerViewViewports: 0,
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
  pub const fn with_multiviewPerViewViewports(mut self, val: VkBool32) -> Self {
    self.multiviewPerViewViewports = val;
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
/// [VkPerfHintInfoQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPerfHintInfoQCOM.html)
#[cfg(feature = "VK_QCOM_queue_perf_hint")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPerfHintInfoQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PERF_HINT_INFO_QCOM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub type_: VkPerfHintTypeQCOM,
  pub scale: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_queue_perf_hint")]
unsafe impl<'a> Send for VkPerfHintInfoQCOM<'a> {}
#[cfg(feature = "VK_QCOM_queue_perf_hint")]
unsafe impl<'a> Sync for VkPerfHintInfoQCOM<'a> {}
#[cfg(feature = "VK_QCOM_queue_perf_hint")]
impl<'a> VkPerfHintInfoQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PERF_HINT_INFO_QCOM,
    pNext: core::ptr::null_mut(),
    type_: VkPerfHintTypeQCOM(0),
    scale: 0,
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
  pub const fn with_type(mut self, val: VkPerfHintTypeQCOM) -> Self {
    self.type_ = val;
    self
  }
  #[inline]
  pub const fn with_scale(mut self, val: u32) -> Self {
    self.scale = val;
    self
  }
  #[cfg(feature = "VK_QCOM_queue_perf_hint")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPerfHintInfoQCOM<
    'root,
    T: VkPNextExtends<VkPerfHintInfoQCOM<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceQueuePerfHintFeaturesQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceQueuePerfHintFeaturesQCOM.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_QCOM_queue_perf_hint")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceQueuePerfHintFeaturesQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_QUEUE_PERF_HINT_FEATURES_QCOM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub queuePerfHint: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_queue_perf_hint")]
unsafe impl<'a> Send for VkPhysicalDeviceQueuePerfHintFeaturesQCOM<'a> {}
#[cfg(feature = "VK_QCOM_queue_perf_hint")]
unsafe impl<'a> Sync for VkPhysicalDeviceQueuePerfHintFeaturesQCOM<'a> {}
#[cfg(all(feature = "VK_QCOM_queue_perf_hint", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceQueuePerfHintFeaturesQCOM<'child>
{
}
#[cfg(all(feature = "VK_QCOM_queue_perf_hint", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceQueuePerfHintFeaturesQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_queue_perf_hint")]
impl<'a> VkPhysicalDeviceQueuePerfHintFeaturesQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_QUEUE_PERF_HINT_FEATURES_QCOM,
    pNext: core::ptr::null_mut(),
    queuePerfHint: 0,
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
  pub const fn with_queuePerfHint(mut self, val: VkBool32) -> Self {
    self.queuePerfHint = val;
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
/// [VkPhysicalDeviceQueuePerfHintPropertiesQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceQueuePerfHintPropertiesQCOM.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_QCOM_queue_perf_hint")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceQueuePerfHintPropertiesQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_QUEUE_PERF_HINT_PROPERTIES_QCOM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Bitmask],  No Auto-Validity
  pub supportedQueues: VkQueueFlags,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_queue_perf_hint")]
unsafe impl<'a> Send for VkPhysicalDeviceQueuePerfHintPropertiesQCOM<'a> {}
#[cfg(feature = "VK_QCOM_queue_perf_hint")]
unsafe impl<'a> Sync for VkPhysicalDeviceQueuePerfHintPropertiesQCOM<'a> {}
#[cfg(all(feature = "VK_QCOM_queue_perf_hint", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceQueuePerfHintPropertiesQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_queue_perf_hint")]
impl<'a> VkPhysicalDeviceQueuePerfHintPropertiesQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_QUEUE_PERF_HINT_PROPERTIES_QCOM,
    pNext: core::ptr::null_mut(),
    supportedQueues: VkQueueFlagBits(0),
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
  pub const fn with_supportedQueues(mut self, val: VkQueueFlags) -> Self {
    self.supportedQueues = val;
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
/// [VkRenderPassTransformBeginInfoQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassTransformBeginInfoQCOM.html)
///
/// **Extends:** VkRenderPassBeginInfo.
#[cfg(feature = "VK_QCOM_render_pass_transform")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkRenderPassTransformBeginInfoQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// No Auto-Validity
  pub transform: VkSurfaceTransformFlagBitsKHR,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_render_pass_transform")]
unsafe impl<'a> Send for VkRenderPassTransformBeginInfoQCOM<'a> {}
#[cfg(feature = "VK_QCOM_render_pass_transform")]
unsafe impl<'a> Sync for VkRenderPassTransformBeginInfoQCOM<'a> {}
#[cfg(all(
  feature = "VK_QCOM_render_pass_transform",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkRenderPassBeginInfo<'root>>
  for VkRenderPassTransformBeginInfoQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_render_pass_transform")]
impl<'a> VkRenderPassTransformBeginInfoQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM,
    pNext: core::ptr::null(),
    transform: VkSurfaceTransformFlagBitsKHR(0),
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
  pub const fn with_transform(mut self, val: VkSurfaceTransformFlagBitsKHR) -> Self {
    self.transform = val;
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkRenderPassBeginInfo<
    'root,
    T: VkPNextExtends<VkRenderPassBeginInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkCommandBufferInheritanceRenderPassTransformInfoQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferInheritanceRenderPassTransformInfoQCOM.html)
///
/// **Extends:** VkCommandBufferInheritanceInfo.
#[cfg(feature = "VK_QCOM_render_pass_transform")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCommandBufferInheritanceRenderPassTransformInfoQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// No Auto-Validity
  pub transform: VkSurfaceTransformFlagBitsKHR,
  pub renderArea: VkRect2D,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_render_pass_transform")]
unsafe impl<'a> Send for VkCommandBufferInheritanceRenderPassTransformInfoQCOM<'a> {}
#[cfg(feature = "VK_QCOM_render_pass_transform")]
unsafe impl<'a> Sync for VkCommandBufferInheritanceRenderPassTransformInfoQCOM<'a> {}
#[cfg(all(
  feature = "VK_QCOM_render_pass_transform",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkCommandBufferInheritanceInfo<'root>>
  for VkCommandBufferInheritanceRenderPassTransformInfoQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_render_pass_transform")]
impl<'a> VkCommandBufferInheritanceRenderPassTransformInfoQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM,
    pNext: core::ptr::null(),
    transform: VkSurfaceTransformFlagBitsKHR(0),
    renderArea: VkRect2D::DEFAULT,
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
  pub const fn with_transform(mut self, val: VkSurfaceTransformFlagBitsKHR) -> Self {
    self.transform = val;
    self
  }
  #[inline]
  pub const fn with_renderArea(mut self, val: VkRect2D) -> Self {
    self.renderArea = val;
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
}
/// [VkCopyCommandTransformInfoQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyCommandTransformInfoQCOM.html)
///
/// **Extends:** VkBufferImageCopy2, VkImageBlit2, VkDeviceMemoryImageCopyKHR.
#[cfg(feature = "VK_QCOM_rotated_copy_commands")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCopyCommandTransformInfoQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_COPY_COMMAND_TRANSFORM_INFO_QCOM
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *const c_void,
  /// No Auto-Validity
  pub transform: VkSurfaceTransformFlagBitsKHR,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_rotated_copy_commands")]
unsafe impl<'a> Send for VkCopyCommandTransformInfoQCOM<'a> {}
#[cfg(feature = "VK_QCOM_rotated_copy_commands")]
unsafe impl<'a> Sync for VkCopyCommandTransformInfoQCOM<'a> {}
#[cfg(all(
  feature = "VK_QCOM_rotated_copy_commands",
  feature = "VK_BASE_VERSION_1_3"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkBufferImageCopy2<'root>>
  for VkCopyCommandTransformInfoQCOM<'child>
{
}
#[cfg(all(
  feature = "VK_QCOM_rotated_copy_commands",
  feature = "VK_GRAPHICS_VERSION_1_3"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkImageBlit2<'root>>
  for VkCopyCommandTransformInfoQCOM<'child>
{
}
#[cfg(all(
  feature = "VK_QCOM_rotated_copy_commands",
  feature = "VK_KHR_device_address_commands"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceMemoryImageCopyKHR<'root>>
  for VkCopyCommandTransformInfoQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_rotated_copy_commands")]
impl<'a> VkCopyCommandTransformInfoQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::COPY_COMMAND_TRANSFORM_INFO_QCOM,
    pNext: core::ptr::null(),
    transform: VkSurfaceTransformFlagBitsKHR(0),
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
  pub const fn with_transform(mut self, val: VkSurfaceTransformFlagBitsKHR) -> Self {
    self.transform = val;
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
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkImageBlit2<'root, T: VkPNextExtends<VkImageBlit2<'root>>>(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_device_address_commands")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDeviceMemoryImageCopyKHR<
    'root,
    T: VkPNextExtends<VkDeviceMemoryImageCopyKHR<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_QCOM_shader_multiple_wait_queues")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_MULTIPLE_WAIT_QUEUES_FEATURES_QCOM
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub shaderMultipleWaitQueues: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_shader_multiple_wait_queues")]
unsafe impl<'a> Send for VkPhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM<'a> {}
#[cfg(feature = "VK_QCOM_shader_multiple_wait_queues")]
unsafe impl<'a> Sync for VkPhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM<'a> {}
#[cfg(all(
  feature = "VK_QCOM_shader_multiple_wait_queues",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM<'child>
{
}
#[cfg(all(
  feature = "VK_QCOM_shader_multiple_wait_queues",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_shader_multiple_wait_queues")]
impl<'a> VkPhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_SHADER_MULTIPLE_WAIT_QUEUES_FEATURES_QCOM,
    pNext: core::ptr::null_mut(),
    shaderMultipleWaitQueues: 0,
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
  pub const fn with_shaderMultipleWaitQueues(mut self, val: VkBool32) -> Self {
    self.shaderMultipleWaitQueues = val;
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
/// [VkPhysicalDeviceShaderMultipleWaitQueuesPropertiesQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderMultipleWaitQueuesPropertiesQCOM.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_QCOM_shader_multiple_wait_queues")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShaderMultipleWaitQueuesPropertiesQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_MULTIPLE_WAIT_QUEUES_PROPERTIES_QCOM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Optional: true,  Limit Type: [Max]
  pub maxShaderWaitQueues: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_shader_multiple_wait_queues")]
unsafe impl<'a> Send for VkPhysicalDeviceShaderMultipleWaitQueuesPropertiesQCOM<'a> {}
#[cfg(feature = "VK_QCOM_shader_multiple_wait_queues")]
unsafe impl<'a> Sync for VkPhysicalDeviceShaderMultipleWaitQueuesPropertiesQCOM<'a> {}
#[cfg(all(
  feature = "VK_QCOM_shader_multiple_wait_queues",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceShaderMultipleWaitQueuesPropertiesQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_shader_multiple_wait_queues")]
impl<'a> VkPhysicalDeviceShaderMultipleWaitQueuesPropertiesQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_SHADER_MULTIPLE_WAIT_QUEUES_PROPERTIES_QCOM,
    pNext: core::ptr::null_mut(),
    maxShaderWaitQueues: 0,
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
  pub const fn with_maxShaderWaitQueues(mut self, val: u32) -> Self {
    self.maxShaderWaitQueues = val;
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
/// [VkTileMemoryBindInfoQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkTileMemoryBindInfoQCOM.html)
///
/// **Extends:** VkCommandBufferInheritanceInfo.
#[cfg(feature = "VK_QCOM_tile_memory_heap")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkTileMemoryBindInfoQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_TILE_MEMORY_BIND_INFO_QCOM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub memory: VkDeviceMemory,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_tile_memory_heap")]
unsafe impl<'a> Send for VkTileMemoryBindInfoQCOM<'a> {}
#[cfg(feature = "VK_QCOM_tile_memory_heap")]
unsafe impl<'a> Sync for VkTileMemoryBindInfoQCOM<'a> {}
#[cfg(all(feature = "VK_QCOM_tile_memory_heap", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkCommandBufferInheritanceInfo<'root>>
  for VkTileMemoryBindInfoQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_tile_memory_heap")]
impl<'a> VkTileMemoryBindInfoQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::TILE_MEMORY_BIND_INFO_QCOM,
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
}
/// [VkPhysicalDeviceTileMemoryHeapFeaturesQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceTileMemoryHeapFeaturesQCOM.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_QCOM_tile_memory_heap")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceTileMemoryHeapFeaturesQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TILE_MEMORY_HEAP_FEATURES_QCOM
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub tileMemoryHeap: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_tile_memory_heap")]
unsafe impl<'a> Send for VkPhysicalDeviceTileMemoryHeapFeaturesQCOM<'a> {}
#[cfg(feature = "VK_QCOM_tile_memory_heap")]
unsafe impl<'a> Sync for VkPhysicalDeviceTileMemoryHeapFeaturesQCOM<'a> {}
#[cfg(all(feature = "VK_QCOM_tile_memory_heap", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceTileMemoryHeapFeaturesQCOM<'child>
{
}
#[cfg(all(feature = "VK_QCOM_tile_memory_heap", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceTileMemoryHeapFeaturesQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_tile_memory_heap")]
impl<'a> VkPhysicalDeviceTileMemoryHeapFeaturesQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_TILE_MEMORY_HEAP_FEATURES_QCOM,
    pNext: core::ptr::null_mut(),
    tileMemoryHeap: 0,
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
  pub const fn with_tileMemoryHeap(mut self, val: VkBool32) -> Self {
    self.tileMemoryHeap = val;
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
/// [VkPhysicalDeviceTileMemoryHeapPropertiesQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceTileMemoryHeapPropertiesQCOM.html)
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_QCOM_tile_memory_heap")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceTileMemoryHeapPropertiesQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TILE_MEMORY_HEAP_PROPERTIES_QCOM
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub queueSubmitBoundary: VkBool32,
  /// Limit Type: [Max]
  pub tileBufferTransfers: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_tile_memory_heap")]
unsafe impl<'a> Send for VkPhysicalDeviceTileMemoryHeapPropertiesQCOM<'a> {}
#[cfg(feature = "VK_QCOM_tile_memory_heap")]
unsafe impl<'a> Sync for VkPhysicalDeviceTileMemoryHeapPropertiesQCOM<'a> {}
#[cfg(all(feature = "VK_QCOM_tile_memory_heap", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceTileMemoryHeapPropertiesQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_tile_memory_heap")]
impl<'a> VkPhysicalDeviceTileMemoryHeapPropertiesQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_TILE_MEMORY_HEAP_PROPERTIES_QCOM,
    pNext: core::ptr::null_mut(),
    queueSubmitBoundary: 0,
    tileBufferTransfers: 0,
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
  pub const fn with_queueSubmitBoundary(mut self, val: VkBool32) -> Self {
    self.queueSubmitBoundary = val;
    self
  }
  #[inline]
  pub const fn with_tileBufferTransfers(mut self, val: VkBool32) -> Self {
    self.tileBufferTransfers = val;
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
/// [VkTileMemorySizeInfoQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkTileMemorySizeInfoQCOM.html)
///
/// **Extends:** VkRenderPassCreateInfo, VkRenderPassCreateInfo2, VkRenderingInfo.
///
/// **Availability:** depends on `VK_QCOM_tile_properties`.
#[cfg(all(
  feature = "VK_QCOM_tile_memory_heap",
  feature = "VK_QCOM_tile_properties"
))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkTileMemorySizeInfoQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_TILE_MEMORY_SIZE_INFO_QCOM
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *const c_void,
  pub size: VkDeviceSize,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(all(
  feature = "VK_QCOM_tile_memory_heap",
  feature = "VK_QCOM_tile_properties"
))]
unsafe impl<'a> Send for VkTileMemorySizeInfoQCOM<'a> {}
#[cfg(all(
  feature = "VK_QCOM_tile_memory_heap",
  feature = "VK_QCOM_tile_properties"
))]
unsafe impl<'a> Sync for VkTileMemorySizeInfoQCOM<'a> {}
#[cfg(all(
  all(
    feature = "VK_QCOM_tile_memory_heap",
    feature = "VK_QCOM_tile_properties"
  ),
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkRenderPassCreateInfo<'root>>
  for VkTileMemorySizeInfoQCOM<'child>
{
}
#[cfg(all(
  all(
    feature = "VK_QCOM_tile_memory_heap",
    feature = "VK_QCOM_tile_properties"
  ),
  feature = "VK_GRAPHICS_VERSION_1_2"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkRenderPassCreateInfo2<'root>>
  for VkTileMemorySizeInfoQCOM<'child>
{
}
#[cfg(all(
  all(
    feature = "VK_QCOM_tile_memory_heap",
    feature = "VK_QCOM_tile_properties"
  ),
  feature = "VK_GRAPHICS_VERSION_1_3"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkRenderingInfo<'root>>
  for VkTileMemorySizeInfoQCOM<'child>
{
}
#[cfg(all(
  feature = "VK_QCOM_tile_memory_heap",
  feature = "VK_QCOM_tile_properties"
))]
impl<'a> VkTileMemorySizeInfoQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::TILE_MEMORY_SIZE_INFO_QCOM,
    pNext: core::ptr::null(),
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
  pub const fn with_size(mut self, val: VkDeviceSize) -> Self {
    self.size = val;
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkRenderPassCreateInfo<
    'root,
    T: VkPNextExtends<VkRenderPassCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkRenderPassCreateInfo2<
    'root,
    T: VkPNextExtends<VkRenderPassCreateInfo2<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkRenderingInfo<
    'root,
    T: VkPNextExtends<VkRenderingInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkTileMemoryRequirementsQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkTileMemoryRequirementsQCOM.html)
///
/// **Extends:** VkMemoryRequirements2.
#[cfg(feature = "VK_QCOM_tile_memory_heap")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkTileMemoryRequirementsQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_TILE_MEMORY_REQUIREMENTS_QCOM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub size: VkDeviceSize,
  pub alignment: VkDeviceSize,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_tile_memory_heap")]
unsafe impl<'a> Send for VkTileMemoryRequirementsQCOM<'a> {}
#[cfg(feature = "VK_QCOM_tile_memory_heap")]
unsafe impl<'a> Sync for VkTileMemoryRequirementsQCOM<'a> {}
#[cfg(all(feature = "VK_QCOM_tile_memory_heap", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkMemoryRequirements2<'root>>
  for VkTileMemoryRequirementsQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_tile_memory_heap")]
impl<'a> VkTileMemoryRequirementsQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::TILE_MEMORY_REQUIREMENTS_QCOM,
    pNext: core::ptr::null_mut(),
    size: 0,
    alignment: 0,
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
  pub const fn with_size(mut self, val: VkDeviceSize) -> Self {
    self.size = val;
    self
  }
  #[inline]
  pub const fn with_alignment(mut self, val: VkDeviceSize) -> Self {
    self.alignment = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkMemoryRequirements2<
    'root,
    T: VkPNextExtends<VkMemoryRequirements2<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceTilePropertiesFeaturesQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceTilePropertiesFeaturesQCOM.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_QCOM_tile_properties")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceTilePropertiesFeaturesQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TILE_PROPERTIES_FEATURES_QCOM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub tileProperties: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_tile_properties")]
unsafe impl<'a> Send for VkPhysicalDeviceTilePropertiesFeaturesQCOM<'a> {}
#[cfg(feature = "VK_QCOM_tile_properties")]
unsafe impl<'a> Sync for VkPhysicalDeviceTilePropertiesFeaturesQCOM<'a> {}
#[cfg(all(feature = "VK_QCOM_tile_properties", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceTilePropertiesFeaturesQCOM<'child>
{
}
#[cfg(all(feature = "VK_QCOM_tile_properties", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceTilePropertiesFeaturesQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_tile_properties")]
impl<'a> VkPhysicalDeviceTilePropertiesFeaturesQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_TILE_PROPERTIES_FEATURES_QCOM,
    pNext: core::ptr::null_mut(),
    tileProperties: 0,
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
  pub const fn with_tileProperties(mut self, val: VkBool32) -> Self {
    self.tileProperties = val;
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
/// [VkTilePropertiesQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkTilePropertiesQCOM.html)
#[cfg(feature = "VK_QCOM_tile_properties")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkTilePropertiesQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_TILE_PROPERTIES_QCOM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub tileSize: VkExtent3D,
  pub apronSize: VkExtent2D,
  pub origin: VkOffset2D,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_tile_properties")]
unsafe impl<'a> Send for VkTilePropertiesQCOM<'a> {}
#[cfg(feature = "VK_QCOM_tile_properties")]
unsafe impl<'a> Sync for VkTilePropertiesQCOM<'a> {}
#[cfg(feature = "VK_QCOM_tile_properties")]
impl<'a> VkTilePropertiesQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::TILE_PROPERTIES_QCOM,
    pNext: core::ptr::null_mut(),
    tileSize: VkExtent3D::DEFAULT,
    apronSize: VkExtent2D::DEFAULT,
    origin: VkOffset2D::DEFAULT,
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
  pub const fn with_tileSize(mut self, val: VkExtent3D) -> Self {
    self.tileSize = val;
    self
  }
  #[inline]
  pub const fn with_apronSize(mut self, val: VkExtent2D) -> Self {
    self.apronSize = val;
    self
  }
  #[inline]
  pub const fn with_origin(mut self, val: VkOffset2D) -> Self {
    self.origin = val;
    self
  }
  #[cfg(feature = "VK_QCOM_tile_properties")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkTilePropertiesQCOM<
    'root,
    T: VkPNextExtends<VkTilePropertiesQCOM<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkTileShadingRenderPassFlagsQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkTileShadingRenderPassFlagsQCOM.html)
#[cfg(feature = "VK_QCOM_tile_shading")]
pub type VkTileShadingRenderPassFlagsQCOM = VkTileShadingRenderPassFlagBitsQCOM;
/// [VkPhysicalDeviceTileShadingFeaturesQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceTileShadingFeaturesQCOM.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_QCOM_tile_shading")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceTileShadingFeaturesQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TILE_SHADING_FEATURES_QCOM
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub tileShading: VkBool32,
  pub tileShadingFragmentStage: VkBool32,
  pub tileShadingColorAttachments: VkBool32,
  pub tileShadingDepthAttachments: VkBool32,
  pub tileShadingStencilAttachments: VkBool32,
  pub tileShadingInputAttachments: VkBool32,
  pub tileShadingSampledAttachments: VkBool32,
  pub tileShadingPerTileDraw: VkBool32,
  pub tileShadingPerTileDispatch: VkBool32,
  pub tileShadingDispatchTile: VkBool32,
  pub tileShadingApron: VkBool32,
  pub tileShadingAnisotropicApron: VkBool32,
  pub tileShadingAtomicOps: VkBool32,
  pub tileShadingImageProcessing: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_tile_shading")]
unsafe impl<'a> Send for VkPhysicalDeviceTileShadingFeaturesQCOM<'a> {}
#[cfg(feature = "VK_QCOM_tile_shading")]
unsafe impl<'a> Sync for VkPhysicalDeviceTileShadingFeaturesQCOM<'a> {}
#[cfg(all(feature = "VK_QCOM_tile_shading", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceTileShadingFeaturesQCOM<'child>
{
}
#[cfg(all(feature = "VK_QCOM_tile_shading", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceTileShadingFeaturesQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_tile_shading")]
impl<'a> VkPhysicalDeviceTileShadingFeaturesQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_TILE_SHADING_FEATURES_QCOM,
    pNext: core::ptr::null_mut(),
    tileShading: 0,
    tileShadingFragmentStage: 0,
    tileShadingColorAttachments: 0,
    tileShadingDepthAttachments: 0,
    tileShadingStencilAttachments: 0,
    tileShadingInputAttachments: 0,
    tileShadingSampledAttachments: 0,
    tileShadingPerTileDraw: 0,
    tileShadingPerTileDispatch: 0,
    tileShadingDispatchTile: 0,
    tileShadingApron: 0,
    tileShadingAnisotropicApron: 0,
    tileShadingAtomicOps: 0,
    tileShadingImageProcessing: 0,
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
  pub const fn with_tileShading(mut self, val: VkBool32) -> Self {
    self.tileShading = val;
    self
  }
  #[inline]
  pub const fn with_tileShadingFragmentStage(mut self, val: VkBool32) -> Self {
    self.tileShadingFragmentStage = val;
    self
  }
  #[inline]
  pub const fn with_tileShadingColorAttachments(mut self, val: VkBool32) -> Self {
    self.tileShadingColorAttachments = val;
    self
  }
  #[inline]
  pub const fn with_tileShadingDepthAttachments(mut self, val: VkBool32) -> Self {
    self.tileShadingDepthAttachments = val;
    self
  }
  #[inline]
  pub const fn with_tileShadingStencilAttachments(mut self, val: VkBool32) -> Self {
    self.tileShadingStencilAttachments = val;
    self
  }
  #[inline]
  pub const fn with_tileShadingInputAttachments(mut self, val: VkBool32) -> Self {
    self.tileShadingInputAttachments = val;
    self
  }
  #[inline]
  pub const fn with_tileShadingSampledAttachments(mut self, val: VkBool32) -> Self {
    self.tileShadingSampledAttachments = val;
    self
  }
  #[inline]
  pub const fn with_tileShadingPerTileDraw(mut self, val: VkBool32) -> Self {
    self.tileShadingPerTileDraw = val;
    self
  }
  #[inline]
  pub const fn with_tileShadingPerTileDispatch(mut self, val: VkBool32) -> Self {
    self.tileShadingPerTileDispatch = val;
    self
  }
  #[inline]
  pub const fn with_tileShadingDispatchTile(mut self, val: VkBool32) -> Self {
    self.tileShadingDispatchTile = val;
    self
  }
  #[inline]
  pub const fn with_tileShadingApron(mut self, val: VkBool32) -> Self {
    self.tileShadingApron = val;
    self
  }
  #[inline]
  pub const fn with_tileShadingAnisotropicApron(mut self, val: VkBool32) -> Self {
    self.tileShadingAnisotropicApron = val;
    self
  }
  #[inline]
  pub const fn with_tileShadingAtomicOps(mut self, val: VkBool32) -> Self {
    self.tileShadingAtomicOps = val;
    self
  }
  #[inline]
  pub const fn with_tileShadingImageProcessing(mut self, val: VkBool32) -> Self {
    self.tileShadingImageProcessing = val;
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
/// [VkPhysicalDeviceTileShadingPropertiesQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceTileShadingPropertiesQCOM.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_QCOM_tile_shading")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceTileShadingPropertiesQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TILE_SHADING_PROPERTIES_QCOM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub maxApronSize: u32,
  /// Limit Type: [Max]
  pub preferNonCoherent: VkBool32,
  /// Limit Type: [Exact]
  pub tileGranularity: VkExtent2D,
  /// Limit Type: [Max]
  pub maxTileShadingRate: VkExtent2D,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_tile_shading")]
unsafe impl<'a> Send for VkPhysicalDeviceTileShadingPropertiesQCOM<'a> {}
#[cfg(feature = "VK_QCOM_tile_shading")]
unsafe impl<'a> Sync for VkPhysicalDeviceTileShadingPropertiesQCOM<'a> {}
#[cfg(all(feature = "VK_QCOM_tile_shading", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceTileShadingPropertiesQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_tile_shading")]
impl<'a> VkPhysicalDeviceTileShadingPropertiesQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_TILE_SHADING_PROPERTIES_QCOM,
    pNext: core::ptr::null_mut(),
    maxApronSize: 0,
    preferNonCoherent: 0,
    tileGranularity: VkExtent2D::DEFAULT,
    maxTileShadingRate: VkExtent2D::DEFAULT,
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
  pub const fn with_maxApronSize(mut self, val: u32) -> Self {
    self.maxApronSize = val;
    self
  }
  #[inline]
  pub const fn with_preferNonCoherent(mut self, val: VkBool32) -> Self {
    self.preferNonCoherent = val;
    self
  }
  #[inline]
  pub const fn with_tileGranularity(mut self, val: VkExtent2D) -> Self {
    self.tileGranularity = val;
    self
  }
  #[inline]
  pub const fn with_maxTileShadingRate(mut self, val: VkExtent2D) -> Self {
    self.maxTileShadingRate = val;
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
/// [VkRenderPassTileShadingCreateInfoQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassTileShadingCreateInfoQCOM.html)
///
/// **Extends:** VkRenderPassCreateInfo, VkRenderPassCreateInfo2, VkRenderingInfo, VkCommandBufferInheritanceInfo.
#[cfg(feature = "VK_QCOM_tile_shading")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkRenderPassTileShadingCreateInfoQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_RENDER_PASS_TILE_SHADING_CREATE_INFO_QCOM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkTileShadingRenderPassFlagsQCOM,
  /// Optional: true
  pub tileApronSize: VkExtent2D,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_tile_shading")]
unsafe impl<'a> Send for VkRenderPassTileShadingCreateInfoQCOM<'a> {}
#[cfg(feature = "VK_QCOM_tile_shading")]
unsafe impl<'a> Sync for VkRenderPassTileShadingCreateInfoQCOM<'a> {}
#[cfg(all(feature = "VK_QCOM_tile_shading", feature = "VK_GRAPHICS_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkRenderPassCreateInfo<'root>>
  for VkRenderPassTileShadingCreateInfoQCOM<'child>
{
}
#[cfg(all(feature = "VK_QCOM_tile_shading", feature = "VK_GRAPHICS_VERSION_1_2"))]
unsafe impl<'child, 'root> VkPNextExtends<VkRenderPassCreateInfo2<'root>>
  for VkRenderPassTileShadingCreateInfoQCOM<'child>
{
}
#[cfg(all(feature = "VK_QCOM_tile_shading", feature = "VK_GRAPHICS_VERSION_1_3"))]
unsafe impl<'child, 'root> VkPNextExtends<VkRenderingInfo<'root>>
  for VkRenderPassTileShadingCreateInfoQCOM<'child>
{
}
#[cfg(all(feature = "VK_QCOM_tile_shading", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkCommandBufferInheritanceInfo<'root>>
  for VkRenderPassTileShadingCreateInfoQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_tile_shading")]
impl<'a> VkRenderPassTileShadingCreateInfoQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::RENDER_PASS_TILE_SHADING_CREATE_INFO_QCOM,
    pNext: core::ptr::null(),
    flags: VkTileShadingRenderPassFlagBitsQCOM(0),
    tileApronSize: VkExtent2D::DEFAULT,
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
  pub const fn with_flags(mut self, val: VkTileShadingRenderPassFlagsQCOM) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_tileApronSize(mut self, val: VkExtent2D) -> Self {
    self.tileApronSize = val;
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkRenderPassCreateInfo<
    'root,
    T: VkPNextExtends<VkRenderPassCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkRenderPassCreateInfo2<
    'root,
    T: VkPNextExtends<VkRenderPassCreateInfo2<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkRenderingInfo<
    'root,
    T: VkPNextExtends<VkRenderingInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
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
}
/// [VkPerTileBeginInfoQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPerTileBeginInfoQCOM.html)
#[cfg(feature = "VK_QCOM_tile_shading")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPerTileBeginInfoQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PER_TILE_BEGIN_INFO_QCOM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_tile_shading")]
unsafe impl<'a> Send for VkPerTileBeginInfoQCOM<'a> {}
#[cfg(feature = "VK_QCOM_tile_shading")]
unsafe impl<'a> Sync for VkPerTileBeginInfoQCOM<'a> {}
#[cfg(feature = "VK_QCOM_tile_shading")]
impl<'a> VkPerTileBeginInfoQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PER_TILE_BEGIN_INFO_QCOM,
    pNext: core::ptr::null(),
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
  #[cfg(feature = "VK_QCOM_tile_shading")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPerTileBeginInfoQCOM<
    'root,
    T: VkPNextExtends<VkPerTileBeginInfoQCOM<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPerTileEndInfoQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPerTileEndInfoQCOM.html)
#[cfg(feature = "VK_QCOM_tile_shading")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPerTileEndInfoQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PER_TILE_END_INFO_QCOM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_tile_shading")]
unsafe impl<'a> Send for VkPerTileEndInfoQCOM<'a> {}
#[cfg(feature = "VK_QCOM_tile_shading")]
unsafe impl<'a> Sync for VkPerTileEndInfoQCOM<'a> {}
#[cfg(feature = "VK_QCOM_tile_shading")]
impl<'a> VkPerTileEndInfoQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PER_TILE_END_INFO_QCOM,
    pNext: core::ptr::null(),
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
  #[cfg(feature = "VK_QCOM_tile_shading")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPerTileEndInfoQCOM<
    'root,
    T: VkPNextExtends<VkPerTileEndInfoQCOM<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkDispatchTileInfoQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDispatchTileInfoQCOM.html)
#[cfg(feature = "VK_QCOM_tile_shading")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDispatchTileInfoQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_DISPATCH_TILE_INFO_QCOM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_tile_shading")]
unsafe impl<'a> Send for VkDispatchTileInfoQCOM<'a> {}
#[cfg(feature = "VK_QCOM_tile_shading")]
unsafe impl<'a> Sync for VkDispatchTileInfoQCOM<'a> {}
#[cfg(feature = "VK_QCOM_tile_shading")]
impl<'a> VkDispatchTileInfoQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::DISPATCH_TILE_INFO_QCOM,
    pNext: core::ptr::null(),
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
  #[cfg(feature = "VK_QCOM_tile_shading")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDispatchTileInfoQCOM<
    'root,
    T: VkPNextExtends<VkDispatchTileInfoQCOM<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceYcbcrDegammaFeaturesQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceYcbcrDegammaFeaturesQCOM.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_QCOM_ycbcr_degamma")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceYcbcrDegammaFeaturesQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_YCBCR_DEGAMMA_FEATURES_QCOM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub ycbcrDegamma: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_ycbcr_degamma")]
unsafe impl<'a> Send for VkPhysicalDeviceYcbcrDegammaFeaturesQCOM<'a> {}
#[cfg(feature = "VK_QCOM_ycbcr_degamma")]
unsafe impl<'a> Sync for VkPhysicalDeviceYcbcrDegammaFeaturesQCOM<'a> {}
#[cfg(all(feature = "VK_QCOM_ycbcr_degamma", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceYcbcrDegammaFeaturesQCOM<'child>
{
}
#[cfg(all(feature = "VK_QCOM_ycbcr_degamma", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceYcbcrDegammaFeaturesQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_ycbcr_degamma")]
impl<'a> VkPhysicalDeviceYcbcrDegammaFeaturesQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_YCBCR_DEGAMMA_FEATURES_QCOM,
    pNext: core::ptr::null_mut(),
    ycbcrDegamma: 0,
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
  pub const fn with_ycbcrDegamma(mut self, val: VkBool32) -> Self {
    self.ycbcrDegamma = val;
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
/// [VkSamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM.html)
///
/// **Extends:** VkSamplerYcbcrConversionCreateInfo.
#[cfg(feature = "VK_QCOM_ycbcr_degamma")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM<'a> {
  /// Values: VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_YCBCR_DEGAMMA_CREATE_INFO_QCOM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub enableYDegamma: VkBool32,
  pub enableCbCrDegamma: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_QCOM_ycbcr_degamma")]
unsafe impl<'a> Send for VkSamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM<'a> {}
#[cfg(feature = "VK_QCOM_ycbcr_degamma")]
unsafe impl<'a> Sync for VkSamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM<'a> {}
#[cfg(all(feature = "VK_QCOM_ycbcr_degamma", feature = "VK_COMPUTE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkSamplerYcbcrConversionCreateInfo<'root>>
  for VkSamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM<'child>
{
}
#[cfg(feature = "VK_QCOM_ycbcr_degamma")]
impl<'a> VkSamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SAMPLER_YCBCR_CONVERSION_YCBCR_DEGAMMA_CREATE_INFO_QCOM,
    pNext: core::ptr::null_mut(),
    enableYDegamma: 0,
    enableCbCrDegamma: 0,
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
  pub const fn with_enableYDegamma(mut self, val: VkBool32) -> Self {
    self.enableYDegamma = val;
    self
  }
  #[inline]
  pub const fn with_enableCbCrDegamma(mut self, val: VkBool32) -> Self {
    self.enableCbCrDegamma = val;
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
