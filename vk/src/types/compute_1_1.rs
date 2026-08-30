#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_1",
  feature = "VK_KHR_sampler_ycbcr_conversion"
))]
use crate::enums::VkChromaLocation;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_EXT_inline_uniform_block",
  feature = "VK_KHR_acceleration_structure",
  feature = "VK_NV_ray_tracing",
  feature = "VK_VALVE_mutable_descriptor_type",
  feature = "VK_QCOM_image_processing",
  feature = "VK_EXT_mutable_descriptor_type"
))]
use crate::enums::VkDescriptorType;
#[cfg(any(
  any(
    feature = "VK_COMPUTE_VERSION_1_4",
    all(feature = "VK_KHR_push_descriptor", feature = "VK_VERSION_1_1"),
    feature = "VK_KHR_descriptor_update_template"
  ),
  all(feature = "VK_COMPUTE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0"))
))]
use crate::enums::VkDescriptorUpdateTemplateType;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_IMG_filter_cubic",
  feature = "VK_EXT_filter_cubic"
))]
use crate::enums::VkFilter;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkFormat;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_AMDX_shader_enqueue",
  feature = "VK_KHR_ray_tracing_pipeline",
  feature = "VK_NV_ray_tracing",
  feature = "VK_HUAWEI_subpass_shading"
))]
use crate::enums::VkPipelineBindPoint;
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
use crate::enums::VkShaderStageFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_1",
  feature = "VK_KHR_shader_subgroup_rotate",
  feature = "VK_EXT_shader_subgroup_partitioned"
))]
use crate::enums::VkSubgroupFeatureFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBool32;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkComponentMapping;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkDescriptorSetLayout;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkDescriptorSetVariableDescriptorCountLayoutSupport;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceSize;
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
use crate::types::VkExternalFormatANDROID;
#[cfg(feature = "VK_OHOS_external_memory")]
use crate::types::VkExternalFormatOHOS;
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
use crate::types::VkExternalFormatQNX;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkFlags;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkImageFormatProperties2;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageViewCreateInfo;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceFeatures2;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceProperties2;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPipelineLayout;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkSamplerCreateInfo;
#[cfg(feature = "VK_QCOM_ycbcr_degamma")]
use crate::types::VkSamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkShaderStageFlags;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkSubgroupFeatureFlags;
use core::ffi::c_void;
/// [VkDescriptorUpdateTemplateCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorUpdateTemplateCreateFlags.html)
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
pub type VkDescriptorUpdateTemplateCreateFlags = VkFlags;
/// [VkDescriptorUpdateTemplate](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorUpdateTemplate.html)
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkDescriptorUpdateTemplate(pub u64);
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
impl VkDescriptorUpdateTemplate {
  pub const NULL: Self = Self(0);
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
impl Default for VkDescriptorUpdateTemplate {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl Send for VkDescriptorUpdateTemplate {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl Sync for VkDescriptorUpdateTemplate {}
/// [VkSamplerYcbcrConversion](https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrConversion.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkSamplerYcbcrConversion(pub u64);
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
impl VkSamplerYcbcrConversion {
  pub const NULL: Self = Self(0);
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
impl Default for VkSamplerYcbcrConversion {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
unsafe impl Send for VkSamplerYcbcrConversion {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
unsafe impl Sync for VkSamplerYcbcrConversion {}
/// [VkPhysicalDeviceVariablePointersFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVariablePointersFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceVariablePointersFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub variablePointersStorageBuffer: VkBool32,
  pub variablePointers: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
unsafe impl<'a> Send for VkPhysicalDeviceVariablePointersFeatures<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
unsafe impl<'a> Sync for VkPhysicalDeviceVariablePointersFeatures<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceVariablePointersFeatures<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceVariablePointersFeatures<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
impl<'a> VkPhysicalDeviceVariablePointersFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES,
    pNext: core::ptr::null_mut(),
    variablePointersStorageBuffer: 0,
    variablePointers: 0,
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
  pub const fn with_variablePointersStorageBuffer(mut self, val: VkBool32) -> Self {
    self.variablePointersStorageBuffer = val;
    self
  }
  #[inline]
  pub const fn with_variablePointers(mut self, val: VkBool32) -> Self {
    self.variablePointers = val;
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
/// [VkPhysicalDeviceVariablePointerFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVariablePointerFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceVariablePointerFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub variablePointersStorageBuffer: VkBool32,
  pub variablePointers: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
unsafe impl<'a> Send for VkPhysicalDeviceVariablePointerFeatures<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
unsafe impl<'a> Sync for VkPhysicalDeviceVariablePointerFeatures<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceVariablePointerFeatures<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceVariablePointerFeatures<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
impl<'a> VkPhysicalDeviceVariablePointerFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES,
    pNext: core::ptr::null_mut(),
    variablePointersStorageBuffer: 0,
    variablePointers: 0,
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
  pub const fn with_variablePointersStorageBuffer(mut self, val: VkBool32) -> Self {
    self.variablePointersStorageBuffer = val;
    self
  }
  #[inline]
  pub const fn with_variablePointers(mut self, val: VkBool32) -> Self {
    self.variablePointers = val;
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
/// [VkDescriptorUpdateTemplateEntry](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorUpdateTemplateEntry.html)
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDescriptorUpdateTemplateEntry {
  pub dstBinding: u32,
  pub dstArrayElement: u32,
  pub descriptorCount: u32,
  pub descriptorType: VkDescriptorType,
  pub offset: usize,
  pub stride: usize,
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl Send for VkDescriptorUpdateTemplateEntry {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl Sync for VkDescriptorUpdateTemplateEntry {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
impl VkDescriptorUpdateTemplateEntry {
  pub const DEFAULT: Self = Self {
    dstBinding: 0,
    dstArrayElement: 0,
    descriptorCount: 0,
    descriptorType: VkDescriptorType(0),
    offset: 0,
    stride: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_dstBinding(mut self, val: u32) -> Self {
    self.dstBinding = val;
    self
  }
  #[inline]
  pub const fn with_dstArrayElement(mut self, val: u32) -> Self {
    self.dstArrayElement = val;
    self
  }
  #[inline]
  pub const fn with_descriptorCount(mut self, val: u32) -> Self {
    self.descriptorCount = val;
    self
  }
  #[inline]
  pub const fn with_descriptorType(mut self, val: VkDescriptorType) -> Self {
    self.descriptorType = val;
    self
  }
  #[inline]
  pub const fn with_offset(mut self, val: usize) -> Self {
    self.offset = val;
    self
  }
  #[inline]
  pub const fn with_stride(mut self, val: usize) -> Self {
    self.stride = val;
    self
  }
}
/// [VkDescriptorUpdateTemplateCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorUpdateTemplateCreateInfo.html)
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDescriptorUpdateTemplateCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkDescriptorUpdateTemplateCreateFlags,
  pub descriptorUpdateEntryCount: u32,
  /// Length: descriptorUpdateEntryCount
  pub pDescriptorUpdateEntries: *const VkDescriptorUpdateTemplateEntry,
  pub templateType: VkDescriptorUpdateTemplateType,
  /// No Auto-Validity
  pub descriptorSetLayout: VkDescriptorSetLayout,
  /// No Auto-Validity
  pub pipelineBindPoint: VkPipelineBindPoint,
  /// No Auto-Validity
  pub pipelineLayout: VkPipelineLayout,
  /// No Auto-Validity
  pub set: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl<'a> Send for VkDescriptorUpdateTemplateCreateInfo<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl<'a> Sync for VkDescriptorUpdateTemplateCreateInfo<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
impl<'a> VkDescriptorUpdateTemplateCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: 0,
    descriptorUpdateEntryCount: 0,
    pDescriptorUpdateEntries: core::ptr::null(),
    templateType: VkDescriptorUpdateTemplateType(0),
    descriptorSetLayout: VkDescriptorSetLayout::DEFAULT,
    pipelineBindPoint: VkPipelineBindPoint(0),
    pipelineLayout: VkPipelineLayout::DEFAULT,
    set: 0,
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
  pub const fn with_flags(mut self, val: VkDescriptorUpdateTemplateCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_descriptorUpdateEntryCount(mut self, val: u32) -> Self {
    self.descriptorUpdateEntryCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pDescriptorUpdateEntries(
    mut self,
    val: &'a [VkDescriptorUpdateTemplateEntry],
  ) -> Self {
    self.descriptorUpdateEntryCount = val.len() as u32;
    self.pDescriptorUpdateEntries = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_templateType(mut self, val: VkDescriptorUpdateTemplateType) -> Self {
    self.templateType = val;
    self
  }
  #[inline]
  pub const fn with_descriptorSetLayout(mut self, val: VkDescriptorSetLayout) -> Self {
    self.descriptorSetLayout = val;
    self
  }
  #[inline]
  pub const fn with_pipelineBindPoint(mut self, val: VkPipelineBindPoint) -> Self {
    self.pipelineBindPoint = val;
    self
  }
  #[inline]
  pub const fn with_pipelineLayout(mut self, val: VkPipelineLayout) -> Self {
    self.pipelineLayout = val;
    self
  }
  #[inline]
  pub const fn with_set(mut self, val: u32) -> Self {
    self.set = val;
    self
  }
  #[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDescriptorUpdateTemplateCreateInfo<
    'root,
    T: VkPNextExtends<VkDescriptorUpdateTemplateCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDevice16BitStorageFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevice16BitStorageFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDevice16BitStorageFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub storageBuffer16BitAccess: VkBool32,
  pub uniformAndStorageBuffer16BitAccess: VkBool32,
  pub storagePushConstant16: VkBool32,
  pub storageInputOutput16: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
unsafe impl<'a> Send for VkPhysicalDevice16BitStorageFeatures<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
unsafe impl<'a> Sync for VkPhysicalDevice16BitStorageFeatures<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDevice16BitStorageFeatures<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDevice16BitStorageFeatures<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
impl<'a> VkPhysicalDevice16BitStorageFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES,
    pNext: core::ptr::null_mut(),
    storageBuffer16BitAccess: 0,
    uniformAndStorageBuffer16BitAccess: 0,
    storagePushConstant16: 0,
    storageInputOutput16: 0,
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
  pub const fn with_storageBuffer16BitAccess(mut self, val: VkBool32) -> Self {
    self.storageBuffer16BitAccess = val;
    self
  }
  #[inline]
  pub const fn with_uniformAndStorageBuffer16BitAccess(mut self, val: VkBool32) -> Self {
    self.uniformAndStorageBuffer16BitAccess = val;
    self
  }
  #[inline]
  pub const fn with_storagePushConstant16(mut self, val: VkBool32) -> Self {
    self.storagePushConstant16 = val;
    self
  }
  #[inline]
  pub const fn with_storageInputOutput16(mut self, val: VkBool32) -> Self {
    self.storageInputOutput16 = val;
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
/// [VkPhysicalDeviceSubgroupProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSubgroupProperties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceSubgroupProperties<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max, Pot],  No Auto-Validity
  pub subgroupSize: u32,
  /// Limit Type: [Bitmask],  No Auto-Validity
  pub supportedStages: VkShaderStageFlags,
  /// Limit Type: [Bitmask],  No Auto-Validity
  pub supportedOperations: VkSubgroupFeatureFlags,
  /// Limit Type: [Max],  No Auto-Validity
  pub quadOperationsInAllStages: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
unsafe impl<'a> Send for VkPhysicalDeviceSubgroupProperties<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
unsafe impl<'a> Sync for VkPhysicalDeviceSubgroupProperties<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceSubgroupProperties<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
impl<'a> VkPhysicalDeviceSubgroupProperties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_SUBGROUP_PROPERTIES,
    pNext: core::ptr::null_mut(),
    subgroupSize: 0,
    supportedStages: VkShaderStageFlagBits(0),
    supportedOperations: VkSubgroupFeatureFlagBits(0),
    quadOperationsInAllStages: 0,
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
  pub const fn with_subgroupSize(mut self, val: u32) -> Self {
    self.subgroupSize = val;
    self
  }
  #[inline]
  pub const fn with_supportedStages(mut self, val: VkShaderStageFlags) -> Self {
    self.supportedStages = val;
    self
  }
  #[inline]
  pub const fn with_supportedOperations(mut self, val: VkSubgroupFeatureFlags) -> Self {
    self.supportedOperations = val;
    self
  }
  #[inline]
  pub const fn with_quadOperationsInAllStages(mut self, val: VkBool32) -> Self {
    self.quadOperationsInAllStages = val;
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
/// [VkSamplerYcbcrConversionInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrConversionInfo.html)
///
/// **Extends:** VkSamplerCreateInfo, VkImageViewCreateInfo.
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSamplerYcbcrConversionInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub conversion: VkSamplerYcbcrConversion,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
unsafe impl<'a> Send for VkSamplerYcbcrConversionInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
unsafe impl<'a> Sync for VkSamplerYcbcrConversionInfo<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", feature = "VK_COMPUTE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkSamplerCreateInfo<'root>>
  for VkSamplerYcbcrConversionInfo<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkImageViewCreateInfo<'root>>
  for VkSamplerYcbcrConversionInfo<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
impl<'a> VkSamplerYcbcrConversionInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SAMPLER_YCBCR_CONVERSION_INFO,
    pNext: core::ptr::null(),
    conversion: VkSamplerYcbcrConversion::DEFAULT,
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
  pub const fn with_conversion(mut self, val: VkSamplerYcbcrConversion) -> Self {
    self.conversion = val;
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
/// [VkSamplerYcbcrConversionCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrConversionCreateInfo.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSamplerYcbcrConversionCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub format: VkFormat,
  pub ycbcrModel: VkSamplerYcbcrModelConversion,
  pub ycbcrRange: VkSamplerYcbcrRange,
  pub components: VkComponentMapping,
  pub xChromaOffset: VkChromaLocation,
  pub yChromaOffset: VkChromaLocation,
  pub chromaFilter: VkFilter,
  pub forceExplicitReconstruction: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
unsafe impl<'a> Send for VkSamplerYcbcrConversionCreateInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
unsafe impl<'a> Sync for VkSamplerYcbcrConversionCreateInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
impl<'a> VkSamplerYcbcrConversionCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SAMPLER_YCBCR_CONVERSION_CREATE_INFO,
    pNext: core::ptr::null(),
    format: VkFormat(0),
    ycbcrModel: VkSamplerYcbcrModelConversion(0),
    ycbcrRange: VkSamplerYcbcrRange(0),
    components: VkComponentMapping::DEFAULT,
    xChromaOffset: VkChromaLocation(0),
    yChromaOffset: VkChromaLocation(0),
    chromaFilter: VkFilter(0),
    forceExplicitReconstruction: 0,
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
  pub const fn with_format(mut self, val: VkFormat) -> Self {
    self.format = val;
    self
  }
  #[inline]
  pub const fn with_ycbcrModel(mut self, val: VkSamplerYcbcrModelConversion) -> Self {
    self.ycbcrModel = val;
    self
  }
  #[inline]
  pub const fn with_ycbcrRange(mut self, val: VkSamplerYcbcrRange) -> Self {
    self.ycbcrRange = val;
    self
  }
  #[inline]
  pub const fn with_components(mut self, val: VkComponentMapping) -> Self {
    self.components = val;
    self
  }
  #[inline]
  pub const fn with_xChromaOffset(mut self, val: VkChromaLocation) -> Self {
    self.xChromaOffset = val;
    self
  }
  #[inline]
  pub const fn with_yChromaOffset(mut self, val: VkChromaLocation) -> Self {
    self.yChromaOffset = val;
    self
  }
  #[inline]
  pub const fn with_chromaFilter(mut self, val: VkFilter) -> Self {
    self.chromaFilter = val;
    self
  }
  #[inline]
  pub const fn with_forceExplicitReconstruction(mut self, val: VkBool32) -> Self {
    self.forceExplicitReconstruction = val;
    self
  }
  #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExternalFormatANDROID<'child>(
    mut self,
    val: &'a VkExternalFormatANDROID<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExternalFormatANDROID<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_OHOS_external_memory")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExternalFormatOHOS<'child>(
    mut self,
    val: &'a VkExternalFormatOHOS<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExternalFormatOHOS<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExternalFormatQNX<'child>(
    mut self,
    val: &'a VkExternalFormatQNX<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExternalFormatQNX<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_ycbcr_degamma")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkSamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM<'child>(
    mut self,
    val: &'a VkSamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkSamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM<'child>).cast::<c_void>();
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
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceSamplerYcbcrConversionFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSamplerYcbcrConversionFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceSamplerYcbcrConversionFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub samplerYcbcrConversion: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
unsafe impl<'a> Send for VkPhysicalDeviceSamplerYcbcrConversionFeatures<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
unsafe impl<'a> Sync for VkPhysicalDeviceSamplerYcbcrConversionFeatures<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceSamplerYcbcrConversionFeatures<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceSamplerYcbcrConversionFeatures<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
impl<'a> VkPhysicalDeviceSamplerYcbcrConversionFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES,
    pNext: core::ptr::null_mut(),
    samplerYcbcrConversion: 0,
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
  pub const fn with_samplerYcbcrConversion(mut self, val: VkBool32) -> Self {
    self.samplerYcbcrConversion = val;
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
/// [VkSamplerYcbcrConversionImageFormatProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrConversionImageFormatProperties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// **Extends:** VkImageFormatProperties2.
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSamplerYcbcrConversionImageFormatProperties<'a> {
  /// Values: VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub combinedImageSamplerDescriptorCount: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
unsafe impl<'a> Send for VkSamplerYcbcrConversionImageFormatProperties<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
unsafe impl<'a> Sync for VkSamplerYcbcrConversionImageFormatProperties<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkImageFormatProperties2<'root>>
  for VkSamplerYcbcrConversionImageFormatProperties<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
impl<'a> VkSamplerYcbcrConversionImageFormatProperties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES,
    pNext: core::ptr::null_mut(),
    combinedImageSamplerDescriptorCount: 0,
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
  pub const fn with_combinedImageSamplerDescriptorCount(mut self, val: u32) -> Self {
    self.combinedImageSamplerDescriptorCount = val;
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
/// [VkPhysicalDeviceMaintenance3Properties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMaintenance3Properties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceMaintenance3Properties<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub maxPerSetDescriptors: u32,
  /// Limit Type: [Max]
  pub maxMemoryAllocationSize: VkDeviceSize,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
unsafe impl<'a> Send for VkPhysicalDeviceMaintenance3Properties<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
unsafe impl<'a> Sync for VkPhysicalDeviceMaintenance3Properties<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceMaintenance3Properties<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
impl<'a> VkPhysicalDeviceMaintenance3Properties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES,
    pNext: core::ptr::null_mut(),
    maxPerSetDescriptors: 0,
    maxMemoryAllocationSize: 0,
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
  pub const fn with_maxPerSetDescriptors(mut self, val: u32) -> Self {
    self.maxPerSetDescriptors = val;
    self
  }
  #[inline]
  pub const fn with_maxMemoryAllocationSize(mut self, val: VkDeviceSize) -> Self {
    self.maxMemoryAllocationSize = val;
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
/// [VkDescriptorSetLayoutSupport](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayoutSupport.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDescriptorSetLayoutSupport<'a> {
  /// Values: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub supported: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
unsafe impl<'a> Send for VkDescriptorSetLayoutSupport<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
unsafe impl<'a> Sync for VkDescriptorSetLayoutSupport<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
impl<'a> VkDescriptorSetLayoutSupport<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::DESCRIPTOR_SET_LAYOUT_SUPPORT,
    pNext: core::ptr::null_mut(),
    supported: 0,
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
  pub const fn with_supported(mut self, val: VkBool32) -> Self {
    self.supported = val;
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDescriptorSetVariableDescriptorCountLayoutSupport<'child>(
    mut self,
    val: &'a mut VkDescriptorSetVariableDescriptorCountLayoutSupport<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkDescriptorSetVariableDescriptorCountLayoutSupport<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDescriptorSetLayoutSupport<
    'root,
    T: VkPNextExtends<VkDescriptorSetLayoutSupport<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
