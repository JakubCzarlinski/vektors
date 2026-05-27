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
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBool32;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkCommandBufferInheritanceInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceAddress;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceSize;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkGraphicsPipelineCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageView;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceProperties2;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
use crate::types::VkRenderingInfo;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkSampler;
use core::ffi::{c_char, c_void};
/// [VkCuModuleNVX](https://docs.vulkan.org/refpages/latest/refpages/source/VkCuModuleNVX.html)
#[cfg(feature = "VK_NVX_binary_import")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkCuModuleNVX(pub *mut c_void);
#[cfg(feature = "VK_NVX_binary_import")]
impl VkCuModuleNVX {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_NVX_binary_import")]
impl Default for VkCuModuleNVX {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_NVX_binary_import")]
unsafe impl Send for VkCuModuleNVX {}
#[cfg(feature = "VK_NVX_binary_import")]
unsafe impl Sync for VkCuModuleNVX {}
/// [VkCuFunctionNVX](https://docs.vulkan.org/refpages/latest/refpages/source/VkCuFunctionNVX.html)
#[cfg(feature = "VK_NVX_binary_import")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkCuFunctionNVX(pub *mut c_void);
#[cfg(feature = "VK_NVX_binary_import")]
impl VkCuFunctionNVX {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_NVX_binary_import")]
impl Default for VkCuFunctionNVX {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_NVX_binary_import")]
unsafe impl Send for VkCuFunctionNVX {}
#[cfg(feature = "VK_NVX_binary_import")]
unsafe impl Sync for VkCuFunctionNVX {}
/// [VkCuModuleCreateInfoNVX](https://docs.vulkan.org/refpages/latest/refpages/source/VkCuModuleCreateInfoNVX.html)
#[cfg(feature = "VK_NVX_binary_import")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCuModuleCreateInfoNVX<'a> {
  /// Values: VK_STRUCTURE_TYPE_CU_MODULE_CREATE_INFO_NVX
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub dataSize: usize,
  /// Length: dataSize
  pub pData: *const c_void,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NVX_binary_import")]
unsafe impl<'a> Send for VkCuModuleCreateInfoNVX<'a> {}
#[cfg(feature = "VK_NVX_binary_import")]
unsafe impl<'a> Sync for VkCuModuleCreateInfoNVX<'a> {}
#[cfg(feature = "VK_NVX_binary_import")]
impl<'a> VkCuModuleCreateInfoNVX<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::CU_MODULE_CREATE_INFO_NVX,
    pNext: core::ptr::null(),
    dataSize: 0,
    pData: core::ptr::null(),
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
  pub const fn with_dataSize(mut self, val: usize) -> Self {
    self.dataSize = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pData(mut self, val: &'a [u8]) -> Self {
    self.dataSize = val.len() as usize;
    self.pData = val.as_ptr().cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NVX_binary_import")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkCuModuleTexturingModeCreateInfoNVX<'child>(
    mut self,
    val: &'a VkCuModuleTexturingModeCreateInfoNVX<'child>,
  ) -> Self {
    self.pNext = (val as *const VkCuModuleTexturingModeCreateInfoNVX<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NVX_binary_import")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkCuModuleCreateInfoNVX<
    'root,
    T: VkPNextExtends<VkCuModuleCreateInfoNVX<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkCuModuleTexturingModeCreateInfoNVX](https://docs.vulkan.org/refpages/latest/refpages/source/VkCuModuleTexturingModeCreateInfoNVX.html)
///
/// **Extends:** VkCuModuleCreateInfoNVX.
#[cfg(feature = "VK_NVX_binary_import")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCuModuleTexturingModeCreateInfoNVX<'a> {
  /// Values: VK_STRUCTURE_TYPE_CU_MODULE_TEXTURING_MODE_CREATE_INFO_NVX
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub use64bitTexturing: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NVX_binary_import")]
unsafe impl<'a> Send for VkCuModuleTexturingModeCreateInfoNVX<'a> {}
#[cfg(feature = "VK_NVX_binary_import")]
unsafe impl<'a> Sync for VkCuModuleTexturingModeCreateInfoNVX<'a> {}
#[cfg(all(feature = "VK_NVX_binary_import", feature = "VK_NVX_binary_import"))]
unsafe impl<'child, 'root> VkPNextExtends<VkCuModuleCreateInfoNVX<'root>>
  for VkCuModuleTexturingModeCreateInfoNVX<'child>
{
}
#[cfg(feature = "VK_NVX_binary_import")]
impl<'a> VkCuModuleTexturingModeCreateInfoNVX<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::CU_MODULE_TEXTURING_MODE_CREATE_INFO_NVX,
    pNext: core::ptr::null(),
    use64bitTexturing: 0,
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
  pub const fn with_use64bitTexturing(mut self, val: VkBool32) -> Self {
    self.use64bitTexturing = val;
    self
  }
  #[cfg(feature = "VK_NVX_binary_import")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkCuModuleCreateInfoNVX<
    'root,
    T: VkPNextExtends<VkCuModuleCreateInfoNVX<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkCuFunctionCreateInfoNVX](https://docs.vulkan.org/refpages/latest/refpages/source/VkCuFunctionCreateInfoNVX.html)
#[cfg(feature = "VK_NVX_binary_import")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCuFunctionCreateInfoNVX<'a> {
  /// Values: VK_STRUCTURE_TYPE_CU_FUNCTION_CREATE_INFO_NVX
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub module: VkCuModuleNVX,
  /// Length: null-terminated
  pub pName: *const c_char,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NVX_binary_import")]
unsafe impl<'a> Send for VkCuFunctionCreateInfoNVX<'a> {}
#[cfg(feature = "VK_NVX_binary_import")]
unsafe impl<'a> Sync for VkCuFunctionCreateInfoNVX<'a> {}
#[cfg(feature = "VK_NVX_binary_import")]
impl<'a> VkCuFunctionCreateInfoNVX<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::CU_FUNCTION_CREATE_INFO_NVX,
    pNext: core::ptr::null(),
    module: VkCuModuleNVX::DEFAULT,
    pName: core::ptr::null(),
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
  pub const fn with_module(mut self, val: VkCuModuleNVX) -> Self {
    self.module = val;
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
  #[cfg(feature = "VK_NVX_binary_import")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkCuFunctionCreateInfoNVX<
    'root,
    T: VkPNextExtends<VkCuFunctionCreateInfoNVX<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkCuLaunchInfoNVX](https://docs.vulkan.org/refpages/latest/refpages/source/VkCuLaunchInfoNVX.html)
#[cfg(feature = "VK_NVX_binary_import")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCuLaunchInfoNVX<'a> {
  /// Values: VK_STRUCTURE_TYPE_CU_LAUNCH_INFO_NVX
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub function: VkCuFunctionNVX,
  pub gridDimX: u32,
  pub gridDimY: u32,
  pub gridDimZ: u32,
  pub blockDimX: u32,
  pub blockDimY: u32,
  pub blockDimZ: u32,
  pub sharedMemBytes: u32,
  /// Optional: true
  pub paramCount: usize,
  /// Length: paramCount
  pub pParams: *const *const c_void,
  /// Optional: true
  pub extraCount: usize,
  /// Length: extraCount
  pub pExtras: *const *const c_void,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NVX_binary_import")]
unsafe impl<'a> Send for VkCuLaunchInfoNVX<'a> {}
#[cfg(feature = "VK_NVX_binary_import")]
unsafe impl<'a> Sync for VkCuLaunchInfoNVX<'a> {}
#[cfg(feature = "VK_NVX_binary_import")]
impl<'a> VkCuLaunchInfoNVX<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::CU_LAUNCH_INFO_NVX,
    pNext: core::ptr::null(),
    function: VkCuFunctionNVX::DEFAULT,
    gridDimX: 0,
    gridDimY: 0,
    gridDimZ: 0,
    blockDimX: 0,
    blockDimY: 0,
    blockDimZ: 0,
    sharedMemBytes: 0,
    paramCount: 0,
    pParams: core::ptr::null(),
    extraCount: 0,
    pExtras: core::ptr::null(),
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
  pub const fn with_function(mut self, val: VkCuFunctionNVX) -> Self {
    self.function = val;
    self
  }
  #[inline]
  pub const fn with_gridDimX(mut self, val: u32) -> Self {
    self.gridDimX = val;
    self
  }
  #[inline]
  pub const fn with_gridDimY(mut self, val: u32) -> Self {
    self.gridDimY = val;
    self
  }
  #[inline]
  pub const fn with_gridDimZ(mut self, val: u32) -> Self {
    self.gridDimZ = val;
    self
  }
  #[inline]
  pub const fn with_blockDimX(mut self, val: u32) -> Self {
    self.blockDimX = val;
    self
  }
  #[inline]
  pub const fn with_blockDimY(mut self, val: u32) -> Self {
    self.blockDimY = val;
    self
  }
  #[inline]
  pub const fn with_blockDimZ(mut self, val: u32) -> Self {
    self.blockDimZ = val;
    self
  }
  #[inline]
  pub const fn with_sharedMemBytes(mut self, val: u32) -> Self {
    self.sharedMemBytes = val;
    self
  }
  #[inline]
  pub const fn with_paramCount(mut self, val: usize) -> Self {
    self.paramCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pParams(mut self, val: &'a [*const c_void]) -> Self {
    self.paramCount = val.len() as usize;
    self.pParams = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_extraCount(mut self, val: usize) -> Self {
    self.extraCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pExtras(mut self, val: &'a [*const c_void]) -> Self {
    self.extraCount = val.len() as usize;
    self.pExtras = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_NVX_binary_import")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkCuLaunchInfoNVX<
    'root,
    T: VkPNextExtends<VkCuLaunchInfoNVX<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkImageViewHandleInfoNVX](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageViewHandleInfoNVX.html)
#[cfg(feature = "VK_NVX_image_view_handle")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImageViewHandleInfoNVX<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMAGE_VIEW_HANDLE_INFO_NVX
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub imageView: VkImageView,
  pub descriptorType: VkDescriptorType,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  /// Optional: true
  pub sampler: VkSampler,
  #[cfg(not(feature = "VK_COMPUTE_VERSION_1_0"))]
  /// Optional: true
  pub sampler: *mut c_void,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NVX_image_view_handle")]
unsafe impl<'a> Send for VkImageViewHandleInfoNVX<'a> {}
#[cfg(feature = "VK_NVX_image_view_handle")]
unsafe impl<'a> Sync for VkImageViewHandleInfoNVX<'a> {}
#[cfg(feature = "VK_NVX_image_view_handle")]
impl<'a> VkImageViewHandleInfoNVX<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::IMAGE_VIEW_HANDLE_INFO_NVX,
    pNext: core::ptr::null(),
    imageView: VkImageView::DEFAULT,
    descriptorType: VkDescriptorType(0),
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    sampler: VkSampler::DEFAULT,
    #[cfg(not(feature = "VK_COMPUTE_VERSION_1_0"))]
    sampler: core::ptr::null_mut(),
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
  pub const fn with_imageView(mut self, val: VkImageView) -> Self {
    self.imageView = val;
    self
  }
  #[inline]
  pub const fn with_descriptorType(mut self, val: VkDescriptorType) -> Self {
    self.descriptorType = val;
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  #[inline]
  pub const fn with_sampler(mut self, val: VkSampler) -> Self {
    self.sampler = val;
    self
  }
  #[cfg(feature = "VK_NVX_image_view_handle")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkImageViewHandleInfoNVX<
    'root,
    T: VkPNextExtends<VkImageViewHandleInfoNVX<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkImageViewAddressPropertiesNVX](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageViewAddressPropertiesNVX.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_NVX_image_view_handle")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImageViewAddressPropertiesNVX<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMAGE_VIEW_ADDRESS_PROPERTIES_NVX
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub deviceAddress: VkDeviceAddress,
  pub size: VkDeviceSize,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NVX_image_view_handle")]
unsafe impl<'a> Send for VkImageViewAddressPropertiesNVX<'a> {}
#[cfg(feature = "VK_NVX_image_view_handle")]
unsafe impl<'a> Sync for VkImageViewAddressPropertiesNVX<'a> {}
#[cfg(feature = "VK_NVX_image_view_handle")]
impl<'a> VkImageViewAddressPropertiesNVX<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::IMAGE_VIEW_ADDRESS_PROPERTIES_NVX,
    pNext: core::ptr::null_mut(),
    deviceAddress: 0,
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
  pub const fn with_pNext(mut self, val: *mut c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_deviceAddress(mut self, val: VkDeviceAddress) -> Self {
    self.deviceAddress = val;
    self
  }
  #[inline]
  pub const fn with_size(mut self, val: VkDeviceSize) -> Self {
    self.size = val;
    self
  }
  #[cfg(feature = "VK_NVX_image_view_handle")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkImageViewAddressPropertiesNVX<
    'root,
    T: VkPNextExtends<VkImageViewAddressPropertiesNVX<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub perViewPositionAllComponents: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
unsafe impl<'a> Send for VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX<'a> {}
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
unsafe impl<'a> Sync for VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX<'a> {}
#[cfg(all(
  feature = "VK_NVX_multiview_per_view_attributes",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX<'child>
{
}
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
impl<'a> VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX,
    pNext: core::ptr::null_mut(),
    perViewPositionAllComponents: 0,
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
  pub const fn with_perViewPositionAllComponents(mut self, val: VkBool32) -> Self {
    self.perViewPositionAllComponents = val;
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
/// [VkMultiviewPerViewAttributesInfoNVX](https://docs.vulkan.org/refpages/latest/refpages/source/VkMultiviewPerViewAttributesInfoNVX.html)
///
/// **Extends:** VkCommandBufferInheritanceInfo, VkGraphicsPipelineCreateInfo, VkRenderingInfo.
///
/// **Availability:** depends on `VK_VERSION_1_3 + VK_KHR_dynamic_rendering`.
#[cfg(any(
  all(
    feature = "VK_NVX_multiview_per_view_attributes",
    feature = "VK_VERSION_1_3"
  ),
  all(
    feature = "VK_KHR_dynamic_rendering",
    feature = "VK_NVX_multiview_per_view_attributes"
  )
))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkMultiviewPerViewAttributesInfoNVX<'a> {
  /// Values: VK_STRUCTURE_TYPE_MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub perViewAttributes: VkBool32,
  pub perViewAttributesPositionXOnly: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(any(
  all(
    feature = "VK_NVX_multiview_per_view_attributes",
    feature = "VK_VERSION_1_3"
  ),
  all(
    feature = "VK_KHR_dynamic_rendering",
    feature = "VK_NVX_multiview_per_view_attributes"
  )
))]
unsafe impl<'a> Send for VkMultiviewPerViewAttributesInfoNVX<'a> {}
#[cfg(any(
  all(
    feature = "VK_NVX_multiview_per_view_attributes",
    feature = "VK_VERSION_1_3"
  ),
  all(
    feature = "VK_KHR_dynamic_rendering",
    feature = "VK_NVX_multiview_per_view_attributes"
  )
))]
unsafe impl<'a> Sync for VkMultiviewPerViewAttributesInfoNVX<'a> {}
#[cfg(all(
  any(
    all(
      feature = "VK_NVX_multiview_per_view_attributes",
      feature = "VK_VERSION_1_3"
    ),
    all(
      feature = "VK_KHR_dynamic_rendering",
      feature = "VK_NVX_multiview_per_view_attributes"
    )
  ),
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkCommandBufferInheritanceInfo<'root>>
  for VkMultiviewPerViewAttributesInfoNVX<'child>
{
}
#[cfg(all(
  any(
    all(
      feature = "VK_NVX_multiview_per_view_attributes",
      feature = "VK_VERSION_1_3"
    ),
    all(
      feature = "VK_KHR_dynamic_rendering",
      feature = "VK_NVX_multiview_per_view_attributes"
    )
  ),
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkGraphicsPipelineCreateInfo<'root>>
  for VkMultiviewPerViewAttributesInfoNVX<'child>
{
}
#[cfg(all(
  any(
    all(
      feature = "VK_NVX_multiview_per_view_attributes",
      feature = "VK_VERSION_1_3"
    ),
    all(
      feature = "VK_KHR_dynamic_rendering",
      feature = "VK_NVX_multiview_per_view_attributes"
    )
  ),
  feature = "VK_GRAPHICS_VERSION_1_3"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkRenderingInfo<'root>>
  for VkMultiviewPerViewAttributesInfoNVX<'child>
{
}
#[cfg(any(
  all(
    feature = "VK_NVX_multiview_per_view_attributes",
    feature = "VK_VERSION_1_3"
  ),
  all(
    feature = "VK_KHR_dynamic_rendering",
    feature = "VK_NVX_multiview_per_view_attributes"
  )
))]
impl<'a> VkMultiviewPerViewAttributesInfoNVX<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX,
    pNext: core::ptr::null(),
    perViewAttributes: 0,
    perViewAttributesPositionXOnly: 0,
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
  pub const fn with_perViewAttributes(mut self, val: VkBool32) -> Self {
    self.perViewAttributes = val;
    self
  }
  #[inline]
  pub const fn with_perViewAttributesPositionXOnly(mut self, val: VkBool32) -> Self {
    self.perViewAttributesPositionXOnly = val;
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
