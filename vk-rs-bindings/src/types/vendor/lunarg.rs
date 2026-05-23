#[cfg(feature = "VK_LUNARG_direct_driver_loading")]
use crate::enums::VkDirectDriverLoadingModeLUNARG;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::PFN_vkVoidFunction;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkInstance;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkInstanceCreateInfo;
use crate::types::VkPNextExtends;
use core::ffi::{c_char, c_void};
/// [VkDirectDriverLoadingFlagsLUNARG](https://docs.vulkan.org/refpages/latest/refpages/source/VkDirectDriverLoadingFlagsLUNARG.html)
#[cfg(feature = "VK_LUNARG_direct_driver_loading")]
pub type VkDirectDriverLoadingFlagsLUNARG = VkFlags;
/// [PFN_vkGetInstanceProcAddrLUNARG](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetInstanceProcAddrLUNARG.html)
#[cfg(feature = "VK_LUNARG_direct_driver_loading")]
pub type PFN_vkGetInstanceProcAddrLUNARG = Option<
  unsafe extern "system" fn(instance: VkInstance, pName: *const c_char) -> PFN_vkVoidFunction,
>;
/// [VkDirectDriverLoadingInfoLUNARG](https://docs.vulkan.org/refpages/latest/refpages/source/VkDirectDriverLoadingInfoLUNARG.html)
#[cfg(feature = "VK_LUNARG_direct_driver_loading")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDirectDriverLoadingInfoLUNARG<'a> {
  /// Values: VK_STRUCTURE_TYPE_DIRECT_DRIVER_LOADING_INFO_LUNARG
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub flags: VkDirectDriverLoadingFlagsLUNARG,
  /// No Auto-Validity
  pub pfnGetInstanceProcAddr: PFN_vkGetInstanceProcAddrLUNARG,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_LUNARG_direct_driver_loading")]
unsafe impl<'a> Send for VkDirectDriverLoadingInfoLUNARG<'a> {}
#[cfg(feature = "VK_LUNARG_direct_driver_loading")]
unsafe impl<'a> Sync for VkDirectDriverLoadingInfoLUNARG<'a> {}
#[cfg(feature = "VK_LUNARG_direct_driver_loading")]
impl<'a> VkDirectDriverLoadingInfoLUNARG<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DIRECT_DRIVER_LOADING_INFO_LUNARG,
    pNext: core::ptr::null_mut(),
    flags: 0,
    pfnGetInstanceProcAddr: None,
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
  pub const fn with_flags(mut self, val: VkDirectDriverLoadingFlagsLUNARG) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_pfnGetInstanceProcAddr(mut self, val: PFN_vkGetInstanceProcAddrLUNARG) -> Self {
    self.pfnGetInstanceProcAddr = val;
    self
  }
  #[cfg(feature = "VK_LUNARG_direct_driver_loading")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDirectDriverLoadingInfoLUNARG<
    'root,
    T: VkPNextExtends<VkDirectDriverLoadingInfoLUNARG<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkDirectDriverLoadingListLUNARG](https://docs.vulkan.org/refpages/latest/refpages/source/VkDirectDriverLoadingListLUNARG.html)
///
/// **Extends:** VkInstanceCreateInfo.
#[cfg(feature = "VK_LUNARG_direct_driver_loading")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDirectDriverLoadingListLUNARG<'a> {
  /// Values: VK_STRUCTURE_TYPE_DIRECT_DRIVER_LOADING_LIST_LUNARG
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *const c_void,
  pub mode: VkDirectDriverLoadingModeLUNARG,
  pub driverCount: u32,
  /// Length: driverCount
  pub pDrivers: *const VkDirectDriverLoadingInfoLUNARG<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_LUNARG_direct_driver_loading")]
unsafe impl<'a> Send for VkDirectDriverLoadingListLUNARG<'a> {}
#[cfg(feature = "VK_LUNARG_direct_driver_loading")]
unsafe impl<'a> Sync for VkDirectDriverLoadingListLUNARG<'a> {}
#[cfg(all(
  feature = "VK_LUNARG_direct_driver_loading",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkInstanceCreateInfo<'root>>
  for VkDirectDriverLoadingListLUNARG<'child>
{
}
#[cfg(feature = "VK_LUNARG_direct_driver_loading")]
impl<'a> VkDirectDriverLoadingListLUNARG<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DIRECT_DRIVER_LOADING_LIST_LUNARG,
    pNext: core::ptr::null(),
    mode: VkDirectDriverLoadingModeLUNARG(0),
    driverCount: 0,
    pDrivers: core::ptr::null(),
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
  pub const fn with_mode(mut self, val: VkDirectDriverLoadingModeLUNARG) -> Self {
    self.mode = val;
    self
  }
  #[inline]
  pub const fn with_driverCount(mut self, val: u32) -> Self {
    self.driverCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pDrivers(mut self, val: &'a [VkDirectDriverLoadingInfoLUNARG<'a>]) -> Self {
    self.driverCount = val.len() as u32;
    self.pDrivers = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkInstanceCreateInfo<
    'root,
    T: VkPNextExtends<VkInstanceCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
