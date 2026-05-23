#[cfg(feature = "VK_AMDX_dense_geometry_format")]
use crate::enums::VkCompressedTriangleFormatAMDX;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_KHR_device_group",
  feature = "VK_KHR_ray_tracing_pipeline",
  feature = "VK_NV_ray_tracing",
  all(
    feature = "VK_EXT_fragment_density_map",
    feature = "VK_KHR_dynamic_rendering"
  ),
  all(
    feature = "VK_KHR_dynamic_rendering",
    feature = "VK_KHR_fragment_shading_rate"
  ),
  feature = "VK_KHR_pipeline_executable_properties",
  feature = "VK_NV_device_generated_commands",
  feature = "VK_KHR_pipeline_library",
  feature = "VK_EXT_pipeline_creation_cache_control",
  feature = "VK_EXT_descriptor_buffer",
  feature = "VK_EXT_attachment_feedback_loop_layout",
  feature = "VK_EXT_opacity_micromap",
  feature = "VK_EXT_pipeline_protected_access",
  feature = "VK_KHR_opacity_micromap"
))]
use crate::enums::VkPipelineCreateFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkAccelerationStructureGeometryKHR;
#[cfg(feature = "VK_EXT_opacity_micromap")]
use crate::types::VkAccelerationStructureTrianglesOpacityMicromapEXT;
#[cfg(feature = "VK_KHR_opacity_micromap")]
use crate::types::VkAccelerationStructureTrianglesOpacityMicromapKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBool32;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceAddress;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceCreateInfo;
#[cfg(any(
  feature = "VK_KHR_acceleration_structure",
  feature = "VK_NV_cooperative_vector"
))]
use crate::types::VkDeviceOrHostAddressConstKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceSize;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceFeatures2;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceProperties2;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPipeline;
#[cfg(feature = "VK_AMD_pipeline_compiler_control")]
use crate::types::VkPipelineCompilerControlCreateInfoAMD;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPipelineCreateFlags;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPipelineCreationFeedbackCreateInfo;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPipelineLayout;
#[cfg(feature = "VK_KHR_pipeline_library")]
use crate::types::VkPipelineLibraryCreateInfoKHR;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPipelineShaderStageCreateInfo;
use core::ffi::{c_char, c_void};
/// [VkPhysicalDeviceDenseGeometryFormatFeaturesAMDX](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDenseGeometryFormatFeaturesAMDX.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_AMDX_dense_geometry_format")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceDenseGeometryFormatFeaturesAMDX<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DENSE_GEOMETRY_FORMAT_FEATURES_AMDX
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub denseGeometryFormat: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_AMDX_dense_geometry_format")]
unsafe impl<'a> Send for VkPhysicalDeviceDenseGeometryFormatFeaturesAMDX<'a> {}
#[cfg(feature = "VK_AMDX_dense_geometry_format")]
unsafe impl<'a> Sync for VkPhysicalDeviceDenseGeometryFormatFeaturesAMDX<'a> {}
#[cfg(all(
  feature = "VK_AMDX_dense_geometry_format",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceDenseGeometryFormatFeaturesAMDX<'child>
{
}
#[cfg(all(
  feature = "VK_AMDX_dense_geometry_format",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceDenseGeometryFormatFeaturesAMDX<'child>
{
}
#[cfg(feature = "VK_AMDX_dense_geometry_format")]
impl<'a> VkPhysicalDeviceDenseGeometryFormatFeaturesAMDX<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DENSE_GEOMETRY_FORMAT_FEATURES_AMDX,
    pNext: core::ptr::null_mut(),
    denseGeometryFormat: 0,
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
  pub const fn with_denseGeometryFormat(mut self, val: VkBool32) -> Self {
    self.denseGeometryFormat = val;
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
/// [VkAccelerationStructureDenseGeometryFormatTrianglesDataAMDX](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureDenseGeometryFormatTrianglesDataAMDX.html)
///
/// **Extends:** VkAccelerationStructureGeometryKHR.
#[cfg(feature = "VK_AMDX_dense_geometry_format")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAccelerationStructureDenseGeometryFormatTrianglesDataAMDX<'a> {
  /// Values: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_DENSE_GEOMETRY_FORMAT_TRIANGLES_DATA_AMDX
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub compressedData: VkDeviceOrHostAddressConstKHR<'a>,
  pub dataSize: VkDeviceSize,
  pub numTriangles: u32,
  pub numVertices: u32,
  pub maxPrimitiveIndex: u32,
  pub maxGeometryIndex: u32,
  pub format: VkCompressedTriangleFormatAMDX,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_AMDX_dense_geometry_format")]
unsafe impl<'a> Send for VkAccelerationStructureDenseGeometryFormatTrianglesDataAMDX<'a> {}
#[cfg(feature = "VK_AMDX_dense_geometry_format")]
unsafe impl<'a> Sync for VkAccelerationStructureDenseGeometryFormatTrianglesDataAMDX<'a> {}
#[cfg(all(
  feature = "VK_AMDX_dense_geometry_format",
  feature = "VK_KHR_acceleration_structure"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkAccelerationStructureGeometryKHR<'root>>
  for VkAccelerationStructureDenseGeometryFormatTrianglesDataAMDX<'child>
{
}
#[cfg(feature = "VK_AMDX_dense_geometry_format")]
impl<'a> VkAccelerationStructureDenseGeometryFormatTrianglesDataAMDX<'a> {
  pub const DEFAULT: Self = Self {
        sType: VkStructureType::VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_DENSE_GEOMETRY_FORMAT_TRIANGLES_DATA_AMDX,
        pNext: core::ptr::null(),
        compressedData: VkDeviceOrHostAddressConstKHR::DEFAULT,
        dataSize: 0,
        numTriangles: 0,
        numVertices: 0,
        maxPrimitiveIndex: 0,
        maxGeometryIndex: 0,
        format: VkCompressedTriangleFormatAMDX(0),
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
  pub const fn with_compressedData(mut self, val: VkDeviceOrHostAddressConstKHR<'a>) -> Self {
    self.compressedData = val;
    self
  }
  #[inline]
  pub const fn with_dataSize(mut self, val: VkDeviceSize) -> Self {
    self.dataSize = val;
    self
  }
  #[inline]
  pub const fn with_numTriangles(mut self, val: u32) -> Self {
    self.numTriangles = val;
    self
  }
  #[inline]
  pub const fn with_numVertices(mut self, val: u32) -> Self {
    self.numVertices = val;
    self
  }
  #[inline]
  pub const fn with_maxPrimitiveIndex(mut self, val: u32) -> Self {
    self.maxPrimitiveIndex = val;
    self
  }
  #[inline]
  pub const fn with_maxGeometryIndex(mut self, val: u32) -> Self {
    self.maxGeometryIndex = val;
    self
  }
  #[inline]
  pub const fn with_format(mut self, val: VkCompressedTriangleFormatAMDX) -> Self {
    self.format = val;
    self
  }
  #[cfg(feature = "VK_EXT_opacity_micromap")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkAccelerationStructureTrianglesOpacityMicromapEXT<'child>(
    mut self,
    val: &'a VkAccelerationStructureTrianglesOpacityMicromapEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkAccelerationStructureTrianglesOpacityMicromapEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_opacity_micromap")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkAccelerationStructureTrianglesOpacityMicromapKHR<'child>(
    mut self,
    val: &'a VkAccelerationStructureTrianglesOpacityMicromapKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkAccelerationStructureTrianglesOpacityMicromapKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkAccelerationStructureGeometryKHR<
    'root,
    T: VkPNextExtends<VkAccelerationStructureGeometryKHR<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkDeviceOrHostAddressConstAMDX](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceOrHostAddressConstAMDX.html)
#[cfg(feature = "VK_AMDX_shader_enqueue")]
#[repr(C)]
#[derive(Copy, Clone)]
pub union VkDeviceOrHostAddressConstAMDX<'a> {
  /// No Auto-Validity
  pub deviceAddress: VkDeviceAddress,
  /// No Auto-Validity
  pub hostAddress: *const c_void,
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_AMDX_shader_enqueue")]
unsafe impl<'a> Send for VkDeviceOrHostAddressConstAMDX<'a> {}
#[cfg(feature = "VK_AMDX_shader_enqueue")]
unsafe impl<'a> Sync for VkDeviceOrHostAddressConstAMDX<'a> {}
#[cfg(feature = "VK_AMDX_shader_enqueue")]
impl<'a> VkDeviceOrHostAddressConstAMDX<'a> {
  pub const DEFAULT: Self = unsafe {
    Self {
      deviceAddress: core::mem::zeroed::<VkDeviceAddress>(),
    }
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
}
#[cfg(feature = "VK_AMDX_shader_enqueue")]
impl<'a> core::fmt::Debug for VkDeviceOrHostAddressConstAMDX<'a> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("VkDeviceOrHostAddressConstAMDX")
      .field("deviceAddress", unsafe { &self.deviceAddress })
      .finish()
  }
}
/// [VkPhysicalDeviceShaderEnqueuePropertiesAMDX](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderEnqueuePropertiesAMDX.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_AMDX_shader_enqueue")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShaderEnqueuePropertiesAMDX<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ENQUEUE_PROPERTIES_AMDX
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub maxExecutionGraphDepth: u32,
  /// Limit Type: [Max]
  pub maxExecutionGraphShaderOutputNodes: u32,
  /// Limit Type: [Max]
  pub maxExecutionGraphShaderPayloadSize: u32,
  /// Limit Type: [Max]
  pub maxExecutionGraphShaderPayloadCount: u32,
  /// Limit Type: [Noauto]
  pub executionGraphDispatchAddressAlignment: u32,
  /// Limit Type: [Max]
  pub maxExecutionGraphWorkgroupCount: [u32; 3],
  /// Limit Type: [Max]
  pub maxExecutionGraphWorkgroups: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_AMDX_shader_enqueue")]
unsafe impl<'a> Send for VkPhysicalDeviceShaderEnqueuePropertiesAMDX<'a> {}
#[cfg(feature = "VK_AMDX_shader_enqueue")]
unsafe impl<'a> Sync for VkPhysicalDeviceShaderEnqueuePropertiesAMDX<'a> {}
#[cfg(all(feature = "VK_AMDX_shader_enqueue", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceShaderEnqueuePropertiesAMDX<'child>
{
}
#[cfg(feature = "VK_AMDX_shader_enqueue")]
impl<'a> VkPhysicalDeviceShaderEnqueuePropertiesAMDX<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ENQUEUE_PROPERTIES_AMDX,
    pNext: core::ptr::null_mut(),
    maxExecutionGraphDepth: 0,
    maxExecutionGraphShaderOutputNodes: 0,
    maxExecutionGraphShaderPayloadSize: 0,
    maxExecutionGraphShaderPayloadCount: 0,
    executionGraphDispatchAddressAlignment: 0,
    maxExecutionGraphWorkgroupCount: [0u32; 3],
    maxExecutionGraphWorkgroups: 0,
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
  pub const fn with_maxExecutionGraphDepth(mut self, val: u32) -> Self {
    self.maxExecutionGraphDepth = val;
    self
  }
  #[inline]
  pub const fn with_maxExecutionGraphShaderOutputNodes(mut self, val: u32) -> Self {
    self.maxExecutionGraphShaderOutputNodes = val;
    self
  }
  #[inline]
  pub const fn with_maxExecutionGraphShaderPayloadSize(mut self, val: u32) -> Self {
    self.maxExecutionGraphShaderPayloadSize = val;
    self
  }
  #[inline]
  pub const fn with_maxExecutionGraphShaderPayloadCount(mut self, val: u32) -> Self {
    self.maxExecutionGraphShaderPayloadCount = val;
    self
  }
  #[inline]
  pub const fn with_executionGraphDispatchAddressAlignment(mut self, val: u32) -> Self {
    self.executionGraphDispatchAddressAlignment = val;
    self
  }
  #[inline]
  pub const fn with_maxExecutionGraphWorkgroupCount(mut self, val: [u32; 3]) -> Self {
    self.maxExecutionGraphWorkgroupCount = val;
    self
  }
  #[inline]
  pub const fn with_maxExecutionGraphWorkgroups(mut self, val: u32) -> Self {
    self.maxExecutionGraphWorkgroups = val;
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
/// [VkPhysicalDeviceShaderEnqueueFeaturesAMDX](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderEnqueueFeaturesAMDX.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_AMDX_shader_enqueue")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShaderEnqueueFeaturesAMDX<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ENQUEUE_FEATURES_AMDX
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub shaderEnqueue: VkBool32,
  pub shaderMeshEnqueue: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_AMDX_shader_enqueue")]
unsafe impl<'a> Send for VkPhysicalDeviceShaderEnqueueFeaturesAMDX<'a> {}
#[cfg(feature = "VK_AMDX_shader_enqueue")]
unsafe impl<'a> Sync for VkPhysicalDeviceShaderEnqueueFeaturesAMDX<'a> {}
#[cfg(all(feature = "VK_AMDX_shader_enqueue", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceShaderEnqueueFeaturesAMDX<'child>
{
}
#[cfg(all(feature = "VK_AMDX_shader_enqueue", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceShaderEnqueueFeaturesAMDX<'child>
{
}
#[cfg(feature = "VK_AMDX_shader_enqueue")]
impl<'a> VkPhysicalDeviceShaderEnqueueFeaturesAMDX<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ENQUEUE_FEATURES_AMDX,
    pNext: core::ptr::null_mut(),
    shaderEnqueue: 0,
    shaderMeshEnqueue: 0,
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
  pub const fn with_shaderEnqueue(mut self, val: VkBool32) -> Self {
    self.shaderEnqueue = val;
    self
  }
  #[inline]
  pub const fn with_shaderMeshEnqueue(mut self, val: VkBool32) -> Self {
    self.shaderMeshEnqueue = val;
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
/// [VkExecutionGraphPipelineCreateInfoAMDX](https://docs.vulkan.org/refpages/latest/refpages/source/VkExecutionGraphPipelineCreateInfoAMDX.html)
#[cfg(feature = "VK_AMDX_shader_enqueue")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkExecutionGraphPipelineCreateInfoAMDX<'a> {
  /// Values: VK_STRUCTURE_TYPE_EXECUTION_GRAPH_PIPELINE_CREATE_INFO_AMDX
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true,  No Auto-Validity
  pub flags: VkPipelineCreateFlags,
  /// Optional: true
  pub stageCount: u32,
  /// Optional: true,  Length: stageCount
  pub pStages: *const VkPipelineShaderStageCreateInfo<'a>,
  /// Optional: true
  pub pLibraryInfo: *const VkPipelineLibraryCreateInfoKHR<'a>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  /// Optional: true
  pub layout: VkPipelineLayout,
  #[cfg(not(feature = "VK_COMPUTE_VERSION_1_0"))]
  /// Optional: true
  pub layout: *mut c_void,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  /// Optional: true,  No Auto-Validity
  pub basePipelineHandle: VkPipeline,
  #[cfg(not(feature = "VK_COMPUTE_VERSION_1_0"))]
  /// Optional: true,  No Auto-Validity
  pub basePipelineHandle: *mut c_void,
  pub basePipelineIndex: i32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_AMDX_shader_enqueue")]
unsafe impl<'a> Send for VkExecutionGraphPipelineCreateInfoAMDX<'a> {}
#[cfg(feature = "VK_AMDX_shader_enqueue")]
unsafe impl<'a> Sync for VkExecutionGraphPipelineCreateInfoAMDX<'a> {}
#[cfg(feature = "VK_AMDX_shader_enqueue")]
impl<'a> VkExecutionGraphPipelineCreateInfoAMDX<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_EXECUTION_GRAPH_PIPELINE_CREATE_INFO_AMDX,
    pNext: core::ptr::null(),
    flags: VkPipelineCreateFlagBits(0),
    stageCount: 0,
    pStages: core::ptr::null(),
    pLibraryInfo: core::ptr::null(),
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    layout: VkPipelineLayout::DEFAULT,
    #[cfg(not(feature = "VK_COMPUTE_VERSION_1_0"))]
    layout: core::ptr::null_mut(),
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    basePipelineHandle: VkPipeline::DEFAULT,
    #[cfg(not(feature = "VK_COMPUTE_VERSION_1_0"))]
    basePipelineHandle: core::ptr::null_mut(),
    basePipelineIndex: 0,
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
  pub const fn with_flags(mut self, val: VkPipelineCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_stageCount(mut self, val: u32) -> Self {
    self.stageCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pStages(mut self, val: &'a [VkPipelineShaderStageCreateInfo<'a>]) -> Self {
    self.stageCount = val.len() as u32;
    self.pStages = val.as_ptr();
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pLibraryInfo(mut self, val: *const VkPipelineLibraryCreateInfoKHR<'a>) -> Self {
    self.pLibraryInfo = val;
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  #[inline]
  pub const fn with_layout(mut self, val: VkPipelineLayout) -> Self {
    self.layout = val;
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  #[inline]
  pub const fn with_basePipelineHandle(mut self, val: VkPipeline) -> Self {
    self.basePipelineHandle = val;
    self
  }
  #[inline]
  pub const fn with_basePipelineIndex(mut self, val: i32) -> Self {
    self.basePipelineIndex = val;
    self
  }
  #[cfg(feature = "VK_AMD_pipeline_compiler_control")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineCompilerControlCreateInfoAMD<'child>(
    mut self,
    val: &'a VkPipelineCompilerControlCreateInfoAMD<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPipelineCompilerControlCreateInfoAMD<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineCreationFeedbackCreateInfo<'child>(
    mut self,
    val: &'a VkPipelineCreationFeedbackCreateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPipelineCreationFeedbackCreateInfo<'child>).cast::<c_void>();
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
/// [VkPipelineShaderStageNodeCreateInfoAMDX](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineShaderStageNodeCreateInfoAMDX.html)
///
/// **Extends:** VkPipelineShaderStageCreateInfo.
#[cfg(feature = "VK_AMDX_shader_enqueue")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineShaderStageNodeCreateInfoAMDX<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_NODE_CREATE_INFO_AMDX
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true,  Length: null-terminated
  pub pName: *const c_char,
  pub index: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_AMDX_shader_enqueue")]
unsafe impl<'a> Send for VkPipelineShaderStageNodeCreateInfoAMDX<'a> {}
#[cfg(feature = "VK_AMDX_shader_enqueue")]
unsafe impl<'a> Sync for VkPipelineShaderStageNodeCreateInfoAMDX<'a> {}
#[cfg(all(feature = "VK_AMDX_shader_enqueue", feature = "VK_COMPUTE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPipelineShaderStageCreateInfo<'root>>
  for VkPipelineShaderStageNodeCreateInfoAMDX<'child>
{
}
#[cfg(feature = "VK_AMDX_shader_enqueue")]
impl<'a> VkPipelineShaderStageNodeCreateInfoAMDX<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_NODE_CREATE_INFO_AMDX,
    pNext: core::ptr::null(),
    pName: core::ptr::null(),
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
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pName(mut self, val: *const c_char) -> Self {
    self.pName = val;
    self
  }
  #[inline]
  pub const fn with_index(mut self, val: u32) -> Self {
    self.index = val;
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPipelineShaderStageCreateInfo<
    'root,
    T: VkPNextExtends<VkPipelineShaderStageCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkExecutionGraphPipelineScratchSizeAMDX](https://docs.vulkan.org/refpages/latest/refpages/source/VkExecutionGraphPipelineScratchSizeAMDX.html)
#[cfg(feature = "VK_AMDX_shader_enqueue")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkExecutionGraphPipelineScratchSizeAMDX<'a> {
  /// Values: VK_STRUCTURE_TYPE_EXECUTION_GRAPH_PIPELINE_SCRATCH_SIZE_AMDX
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub minSize: VkDeviceSize,
  pub maxSize: VkDeviceSize,
  pub sizeGranularity: VkDeviceSize,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_AMDX_shader_enqueue")]
unsafe impl<'a> Send for VkExecutionGraphPipelineScratchSizeAMDX<'a> {}
#[cfg(feature = "VK_AMDX_shader_enqueue")]
unsafe impl<'a> Sync for VkExecutionGraphPipelineScratchSizeAMDX<'a> {}
#[cfg(feature = "VK_AMDX_shader_enqueue")]
impl<'a> VkExecutionGraphPipelineScratchSizeAMDX<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_EXECUTION_GRAPH_PIPELINE_SCRATCH_SIZE_AMDX,
    pNext: core::ptr::null_mut(),
    minSize: 0,
    maxSize: 0,
    sizeGranularity: 0,
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
  pub const fn with_minSize(mut self, val: VkDeviceSize) -> Self {
    self.minSize = val;
    self
  }
  #[inline]
  pub const fn with_maxSize(mut self, val: VkDeviceSize) -> Self {
    self.maxSize = val;
    self
  }
  #[inline]
  pub const fn with_sizeGranularity(mut self, val: VkDeviceSize) -> Self {
    self.sizeGranularity = val;
    self
  }
  #[cfg(feature = "VK_AMDX_shader_enqueue")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkExecutionGraphPipelineScratchSizeAMDX<
    'root,
    T: VkPNextExtends<VkExecutionGraphPipelineScratchSizeAMDX<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkDispatchGraphInfoAMDX](https://docs.vulkan.org/refpages/latest/refpages/source/VkDispatchGraphInfoAMDX.html)
#[cfg(feature = "VK_AMDX_shader_enqueue")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDispatchGraphInfoAMDX<'a> {
  pub nodeIndex: u32,
  /// Optional: true
  pub payloadCount: u32,
  /// No Auto-Validity
  pub payloads: VkDeviceOrHostAddressConstAMDX<'a>,
  pub payloadStride: u64,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_AMDX_shader_enqueue")]
unsafe impl<'a> Send for VkDispatchGraphInfoAMDX<'a> {}
#[cfg(feature = "VK_AMDX_shader_enqueue")]
unsafe impl<'a> Sync for VkDispatchGraphInfoAMDX<'a> {}
#[cfg(feature = "VK_AMDX_shader_enqueue")]
impl<'a> VkDispatchGraphInfoAMDX<'a> {
  pub const DEFAULT: Self = Self {
    nodeIndex: 0,
    payloadCount: 0,
    payloads: VkDeviceOrHostAddressConstAMDX::DEFAULT,
    payloadStride: 0,
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_nodeIndex(mut self, val: u32) -> Self {
    self.nodeIndex = val;
    self
  }
  #[inline]
  pub const fn with_payloadCount(mut self, val: u32) -> Self {
    self.payloadCount = val;
    self
  }
  #[inline]
  pub const fn with_payloads(mut self, val: VkDeviceOrHostAddressConstAMDX<'a>) -> Self {
    self.payloads = val;
    self
  }
  #[inline]
  pub const fn with_payloadStride(mut self, val: u64) -> Self {
    self.payloadStride = val;
    self
  }
}
/// [VkDispatchGraphCountInfoAMDX](https://docs.vulkan.org/refpages/latest/refpages/source/VkDispatchGraphCountInfoAMDX.html)
#[cfg(feature = "VK_AMDX_shader_enqueue")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDispatchGraphCountInfoAMDX<'a> {
  /// Optional: true
  pub count: u32,
  /// No Auto-Validity
  pub infos: VkDeviceOrHostAddressConstAMDX<'a>,
  pub stride: u64,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_AMDX_shader_enqueue")]
unsafe impl<'a> Send for VkDispatchGraphCountInfoAMDX<'a> {}
#[cfg(feature = "VK_AMDX_shader_enqueue")]
unsafe impl<'a> Sync for VkDispatchGraphCountInfoAMDX<'a> {}
#[cfg(feature = "VK_AMDX_shader_enqueue")]
impl<'a> VkDispatchGraphCountInfoAMDX<'a> {
  pub const DEFAULT: Self = Self {
    count: 0,
    infos: VkDeviceOrHostAddressConstAMDX::DEFAULT,
    stride: 0,
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_count(mut self, val: u32) -> Self {
    self.count = val;
    self
  }
  #[inline]
  pub const fn with_infos(mut self, val: VkDeviceOrHostAddressConstAMDX<'a>) -> Self {
    self.infos = val;
    self
  }
  #[inline]
  pub const fn with_stride(mut self, val: u64) -> Self {
    self.stride = val;
    self
  }
}
