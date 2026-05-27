#[cfg(feature = "VK_INTEL_performance_query")]
use crate::enums::VkPerformanceConfigurationTypeINTEL;
#[cfg(feature = "VK_INTEL_performance_query")]
use crate::enums::VkPerformanceOverrideTypeINTEL;
#[cfg(feature = "VK_INTEL_performance_query")]
use crate::enums::VkPerformanceValueTypeINTEL;
#[cfg(feature = "VK_INTEL_performance_query")]
use crate::enums::VkQueryPoolSamplingModeINTEL;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBool32;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceCreateInfo;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceFeatures2;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkQueryPoolCreateInfo;
use core::ffi::{c_char, c_void};
/// [VkPerformanceConfigurationINTEL](https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceConfigurationINTEL.html)
#[cfg(feature = "VK_INTEL_performance_query")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkPerformanceConfigurationINTEL(pub *mut c_void);
#[cfg(feature = "VK_INTEL_performance_query")]
impl VkPerformanceConfigurationINTEL {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_INTEL_performance_query")]
impl Default for VkPerformanceConfigurationINTEL {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_INTEL_performance_query")]
unsafe impl Send for VkPerformanceConfigurationINTEL {}
#[cfg(feature = "VK_INTEL_performance_query")]
unsafe impl Sync for VkPerformanceConfigurationINTEL {}
/// [VkPerformanceValueDataINTEL](https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceValueDataINTEL.html)
#[cfg(feature = "VK_INTEL_performance_query")]
#[repr(C)]
#[derive(Copy, Clone)]
pub union VkPerformanceValueDataINTEL<'a> {
  pub value32: u32,
  pub value64: u64,
  pub valueFloat: f32,
  pub valueBool: VkBool32,
  /// Length: null-terminated
  pub valueString: *const c_char,
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_INTEL_performance_query")]
unsafe impl<'a> Send for VkPerformanceValueDataINTEL<'a> {}
#[cfg(feature = "VK_INTEL_performance_query")]
unsafe impl<'a> Sync for VkPerformanceValueDataINTEL<'a> {}
#[cfg(feature = "VK_INTEL_performance_query")]
impl<'a> VkPerformanceValueDataINTEL<'a> {
  pub const DEFAULT: Self = unsafe {
    Self {
      value32: core::mem::zeroed::<u32>(),
    }
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
}
#[cfg(feature = "VK_INTEL_performance_query")]
impl<'a> core::fmt::Debug for VkPerformanceValueDataINTEL<'a> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("VkPerformanceValueDataINTEL")
      .field("value32", unsafe { &self.value32 })
      .finish()
  }
}
/// [VkPerformanceValueINTEL](https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceValueINTEL.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_INTEL_performance_query")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPerformanceValueINTEL<'a> {
  pub type_: VkPerformanceValueTypeINTEL,
  /// No Auto-Validity
  pub data: VkPerformanceValueDataINTEL<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_INTEL_performance_query")]
unsafe impl<'a> Send for VkPerformanceValueINTEL<'a> {}
#[cfg(feature = "VK_INTEL_performance_query")]
unsafe impl<'a> Sync for VkPerformanceValueINTEL<'a> {}
#[cfg(feature = "VK_INTEL_performance_query")]
impl<'a> VkPerformanceValueINTEL<'a> {
  pub const DEFAULT: Self = Self {
    type_: VkPerformanceValueTypeINTEL(0),
    data: VkPerformanceValueDataINTEL::DEFAULT,
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_type(mut self, val: VkPerformanceValueTypeINTEL) -> Self {
    self.type_ = val;
    self
  }
  #[inline]
  pub const fn with_data(mut self, val: VkPerformanceValueDataINTEL<'a>) -> Self {
    self.data = val;
    self
  }
}
/// [VkInitializePerformanceApiInfoINTEL](https://docs.vulkan.org/refpages/latest/refpages/source/VkInitializePerformanceApiInfoINTEL.html)
#[cfg(feature = "VK_INTEL_performance_query")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkInitializePerformanceApiInfoINTEL<'a> {
  /// Values: VK_STRUCTURE_TYPE_INITIALIZE_PERFORMANCE_API_INFO_INTEL
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub pUserData: *mut c_void,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_INTEL_performance_query")]
unsafe impl<'a> Send for VkInitializePerformanceApiInfoINTEL<'a> {}
#[cfg(feature = "VK_INTEL_performance_query")]
unsafe impl<'a> Sync for VkInitializePerformanceApiInfoINTEL<'a> {}
#[cfg(feature = "VK_INTEL_performance_query")]
impl<'a> VkInitializePerformanceApiInfoINTEL<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::INITIALIZE_PERFORMANCE_API_INFO_INTEL,
    pNext: core::ptr::null(),
    pUserData: core::ptr::null_mut(),
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
  pub const fn with_pUserData(mut self, val: *mut c_void) -> Self {
    self.pUserData = val;
    self
  }
  #[cfg(feature = "VK_INTEL_performance_query")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkInitializePerformanceApiInfoINTEL<
    'root,
    T: VkPNextExtends<VkInitializePerformanceApiInfoINTEL<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkQueryPoolPerformanceQueryCreateInfoINTEL](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPoolPerformanceQueryCreateInfoINTEL.html)
///
/// **Extends:** VkQueryPoolCreateInfo.
#[cfg(feature = "VK_INTEL_performance_query")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkQueryPoolPerformanceQueryCreateInfoINTEL<'a> {
  /// Values: VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub performanceCountersSampling: VkQueryPoolSamplingModeINTEL,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_INTEL_performance_query")]
unsafe impl<'a> Send for VkQueryPoolPerformanceQueryCreateInfoINTEL<'a> {}
#[cfg(feature = "VK_INTEL_performance_query")]
unsafe impl<'a> Sync for VkQueryPoolPerformanceQueryCreateInfoINTEL<'a> {}
#[cfg(all(
  feature = "VK_INTEL_performance_query",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkQueryPoolCreateInfo<'root>>
  for VkQueryPoolPerformanceQueryCreateInfoINTEL<'child>
{
}
#[cfg(feature = "VK_INTEL_performance_query")]
impl<'a> VkQueryPoolPerformanceQueryCreateInfoINTEL<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL,
    pNext: core::ptr::null(),
    performanceCountersSampling: VkQueryPoolSamplingModeINTEL(0),
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
  pub const fn with_performanceCountersSampling(
    mut self,
    val: VkQueryPoolSamplingModeINTEL,
  ) -> Self {
    self.performanceCountersSampling = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkQueryPoolCreateInfo<
    'root,
    T: VkPNextExtends<VkQueryPoolCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkQueryPoolCreateInfoINTEL](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPoolCreateInfoINTEL.html)
///
/// **Extends:** VkQueryPoolCreateInfo.
#[cfg(feature = "VK_INTEL_performance_query")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkQueryPoolCreateInfoINTEL<'a> {
  /// Values: VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub performanceCountersSampling: VkQueryPoolSamplingModeINTEL,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_INTEL_performance_query")]
unsafe impl<'a> Send for VkQueryPoolCreateInfoINTEL<'a> {}
#[cfg(feature = "VK_INTEL_performance_query")]
unsafe impl<'a> Sync for VkQueryPoolCreateInfoINTEL<'a> {}
#[cfg(all(
  feature = "VK_INTEL_performance_query",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkQueryPoolCreateInfo<'root>>
  for VkQueryPoolCreateInfoINTEL<'child>
{
}
#[cfg(feature = "VK_INTEL_performance_query")]
impl<'a> VkQueryPoolCreateInfoINTEL<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL,
    pNext: core::ptr::null(),
    performanceCountersSampling: VkQueryPoolSamplingModeINTEL(0),
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
  pub const fn with_performanceCountersSampling(
    mut self,
    val: VkQueryPoolSamplingModeINTEL,
  ) -> Self {
    self.performanceCountersSampling = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkQueryPoolCreateInfo<
    'root,
    T: VkPNextExtends<VkQueryPoolCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPerformanceMarkerInfoINTEL](https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceMarkerInfoINTEL.html)
#[cfg(feature = "VK_INTEL_performance_query")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPerformanceMarkerInfoINTEL<'a> {
  /// Values: VK_STRUCTURE_TYPE_PERFORMANCE_MARKER_INFO_INTEL
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub marker: u64,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_INTEL_performance_query")]
unsafe impl<'a> Send for VkPerformanceMarkerInfoINTEL<'a> {}
#[cfg(feature = "VK_INTEL_performance_query")]
unsafe impl<'a> Sync for VkPerformanceMarkerInfoINTEL<'a> {}
#[cfg(feature = "VK_INTEL_performance_query")]
impl<'a> VkPerformanceMarkerInfoINTEL<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PERFORMANCE_MARKER_INFO_INTEL,
    pNext: core::ptr::null(),
    marker: 0,
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
  pub const fn with_marker(mut self, val: u64) -> Self {
    self.marker = val;
    self
  }
  #[cfg(feature = "VK_INTEL_performance_query")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPerformanceMarkerInfoINTEL<
    'root,
    T: VkPNextExtends<VkPerformanceMarkerInfoINTEL<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPerformanceStreamMarkerInfoINTEL](https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceStreamMarkerInfoINTEL.html)
#[cfg(feature = "VK_INTEL_performance_query")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPerformanceStreamMarkerInfoINTEL<'a> {
  /// Values: VK_STRUCTURE_TYPE_PERFORMANCE_STREAM_MARKER_INFO_INTEL
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub marker: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_INTEL_performance_query")]
unsafe impl<'a> Send for VkPerformanceStreamMarkerInfoINTEL<'a> {}
#[cfg(feature = "VK_INTEL_performance_query")]
unsafe impl<'a> Sync for VkPerformanceStreamMarkerInfoINTEL<'a> {}
#[cfg(feature = "VK_INTEL_performance_query")]
impl<'a> VkPerformanceStreamMarkerInfoINTEL<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PERFORMANCE_STREAM_MARKER_INFO_INTEL,
    pNext: core::ptr::null(),
    marker: 0,
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
  pub const fn with_marker(mut self, val: u32) -> Self {
    self.marker = val;
    self
  }
  #[cfg(feature = "VK_INTEL_performance_query")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPerformanceStreamMarkerInfoINTEL<
    'root,
    T: VkPNextExtends<VkPerformanceStreamMarkerInfoINTEL<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPerformanceOverrideInfoINTEL](https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceOverrideInfoINTEL.html)
#[cfg(feature = "VK_INTEL_performance_query")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPerformanceOverrideInfoINTEL<'a> {
  /// Values: VK_STRUCTURE_TYPE_PERFORMANCE_OVERRIDE_INFO_INTEL
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub type_: VkPerformanceOverrideTypeINTEL,
  pub enable: VkBool32,
  pub parameter: u64,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_INTEL_performance_query")]
unsafe impl<'a> Send for VkPerformanceOverrideInfoINTEL<'a> {}
#[cfg(feature = "VK_INTEL_performance_query")]
unsafe impl<'a> Sync for VkPerformanceOverrideInfoINTEL<'a> {}
#[cfg(feature = "VK_INTEL_performance_query")]
impl<'a> VkPerformanceOverrideInfoINTEL<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PERFORMANCE_OVERRIDE_INFO_INTEL,
    pNext: core::ptr::null(),
    type_: VkPerformanceOverrideTypeINTEL(0),
    enable: 0,
    parameter: 0,
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
  pub const fn with_type(mut self, val: VkPerformanceOverrideTypeINTEL) -> Self {
    self.type_ = val;
    self
  }
  #[inline]
  pub const fn with_enable(mut self, val: VkBool32) -> Self {
    self.enable = val;
    self
  }
  #[inline]
  pub const fn with_parameter(mut self, val: u64) -> Self {
    self.parameter = val;
    self
  }
  #[cfg(feature = "VK_INTEL_performance_query")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPerformanceOverrideInfoINTEL<
    'root,
    T: VkPNextExtends<VkPerformanceOverrideInfoINTEL<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPerformanceConfigurationAcquireInfoINTEL](https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceConfigurationAcquireInfoINTEL.html)
#[cfg(feature = "VK_INTEL_performance_query")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPerformanceConfigurationAcquireInfoINTEL<'a> {
  /// Values: VK_STRUCTURE_TYPE_PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub type_: VkPerformanceConfigurationTypeINTEL,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_INTEL_performance_query")]
unsafe impl<'a> Send for VkPerformanceConfigurationAcquireInfoINTEL<'a> {}
#[cfg(feature = "VK_INTEL_performance_query")]
unsafe impl<'a> Sync for VkPerformanceConfigurationAcquireInfoINTEL<'a> {}
#[cfg(feature = "VK_INTEL_performance_query")]
impl<'a> VkPerformanceConfigurationAcquireInfoINTEL<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL,
    pNext: core::ptr::null(),
    type_: VkPerformanceConfigurationTypeINTEL(0),
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
  pub const fn with_type(mut self, val: VkPerformanceConfigurationTypeINTEL) -> Self {
    self.type_ = val;
    self
  }
  #[cfg(feature = "VK_INTEL_performance_query")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPerformanceConfigurationAcquireInfoINTEL<
    'root,
    T: VkPNextExtends<VkPerformanceConfigurationAcquireInfoINTEL<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_INTEL_shader_integer_functions2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub shaderIntegerFunctions2: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_INTEL_shader_integer_functions2")]
unsafe impl<'a> Send for VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL<'a> {}
#[cfg(feature = "VK_INTEL_shader_integer_functions2")]
unsafe impl<'a> Sync for VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL<'a> {}
#[cfg(all(
  feature = "VK_INTEL_shader_integer_functions2",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL<'child>
{
}
#[cfg(all(
  feature = "VK_INTEL_shader_integer_functions2",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL<'child>
{
}
#[cfg(feature = "VK_INTEL_shader_integer_functions2")]
impl<'a> VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL,
    pNext: core::ptr::null_mut(),
    shaderIntegerFunctions2: 0,
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
  pub const fn with_shaderIntegerFunctions2(mut self, val: VkBool32) -> Self {
    self.shaderIntegerFunctions2 = val;
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
