//! Loader-owned WSI surface state and surface-aware terminators.

use core::ffi::{CStr, c_void};

use crate::sync::Mutex;
use vk::{
    PFN_vkCreateSharedSwapchainsKHR, PFN_vkCreateSwapchainKHR,
    PFN_vkGetDeviceGroupSurfacePresentModesKHR, VkAllocationCallbacks, VkBaseInStructure, VkDevice,
    VkDeviceGroupPresentModeFlagsKHR, VkInstance, VkResult, VkStructureType,
    VkSurfaceCapabilities2EXT, VkSurfaceCapabilities2KHR, VkSurfaceCapabilitiesKHR,
    VkSurfaceFormat2KHR, VkSurfaceKHR, VkSurfacePresentModeCompatibilityKHR,
    VkSurfacePresentModeKHR, VkSurfacePresentScalingCapabilitiesKHR,
    VkSurfaceProtectedCapabilitiesKHR, VkSwapchainCreateInfoKHR, VkSwapchainKHR,
};

use crate::{
    allocation::{LoaderAllocation, LoaderArray, LoaderBox},
    collections::ScratchArray,
    device::LoaderDevice,
    emulation::{emulate_result_array, for_each_output_chain, optional_output_slice},
    generated::EmulatedCommand,
    icd::IcdInstance,
    instance::{LoaderInstance, LoaderPhysicalDevice},
    load_typed, surface_create_info_extension_size,
};

const STACK_SURFACE_FORMATS: usize = 32;
const STACK_SWAPCHAIN_CREATE_INFOS: usize = 4;

type NativeSurfaceCreate<T> = unsafe extern "system" fn(
    VkInstance,
    *const T,
    *const VkAllocationCallbacks<'_>,
    *mut VkSurfaceKHR,
) -> VkResult;

type ErasedSurfaceCreate =
    unsafe fn(&DeferredSurface, &IcdInstance) -> Result<VkSurfaceKHR, VkResult>;

#[cfg(target_os = "android")]
const ICD_WSI_PLATFORM_ANDROID: i32 = 5;
#[cfg(target_os = "ios")]
const ICD_WSI_PLATFORM_IOS: i32 = 7;

/// ABI shared by upstream's `VkIcdSurfaceAndroid` and `VkIcdSurfaceIOS`.
/// These two platforms pass a loader-owned WSI object directly to the ICD
/// instead of creating one native `VkSurfaceKHR` per ICD.
#[repr(C)]
#[cfg(any(target_os = "android", target_os = "ios", test))]
pub(crate) struct IcdPassthroughSurface {
    platform: i32,
    object: *const c_void,
}

// The platform object lifetime and synchronization requirements are imposed
// by the corresponding Vulkan surface extension.
#[cfg(any(target_os = "android", target_os = "ios"))]
unsafe impl Send for IcdPassthroughSurface {}
#[cfg(any(target_os = "android", target_os = "ios"))]
unsafe impl Sync for IcdPassthroughSurface {}

struct OwnedCreateInfo {
    allocation: LoaderAllocation,
}

impl OwnedCreateInfo {
    unsafe fn copy_from<T: Copy>(
        source: *const T,
        expected_structure_type: VkStructureType,
        callbacks: Option<&VkAllocationCallbacks<'static>>,
    ) -> Result<Self, VkResult> {
        const CHAIN_ALIGNMENT: usize = core::mem::size_of::<u64>();

        fn aligned_size(size: usize) -> Option<usize> {
            size.checked_add(CHAIN_ALIGNMENT - 1)
                .map(|size| size & !(CHAIN_ALIGNMENT - 1))
        }

        const {
            assert!(core::mem::align_of::<T>() <= CHAIN_ALIGNMENT);
        }

        if source.is_null() {
            return Err(VkResult::ERROR_INITIALIZATION_FAILED);
        }

        // Surface create-info chains are retained because ICD objects are
        // materialized lazily. The generator derives each command's expected
        // root and permitted extension structures from the registry.
        let root = unsafe { &*source.cast::<VkBaseInStructure<'_>>() };
        if root.sType != expected_structure_type {
            return Err(VkResult::ERROR_EXTENSION_NOT_PRESENT);
        }
        let mut total =
            aligned_size(core::mem::size_of::<T>()).ok_or(VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
        let mut next = root.pNext;
        while !next.is_null() {
            let header = unsafe { &*next };
            let size = surface_create_info_extension_size(expected_structure_type, header.sType)
                .ok_or(VkResult::ERROR_EXTENSION_NOT_PRESENT)?;
            total = total
                .checked_add(aligned_size(size).ok_or(VkResult::ERROR_OUT_OF_HOST_MEMORY)?)
                .ok_or(VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
            next = header.pNext;
        }

        let allocation =
            LoaderAllocation::new(callbacks, total, vk::VkSystemAllocationScope::OBJECT)?;

        let mut destination = allocation.as_ptr();
        let mut source_node = source.cast::<VkBaseInStructure<'_>>();
        let mut previous: *mut VkBaseInStructure<'static> = core::ptr::null_mut();
        let mut first = true;
        while !source_node.is_null() {
            let header = unsafe { &*source_node };
            let size = if first {
                core::mem::size_of::<T>()
            } else {
                // SAFETY: The immutable Vulkan input chain was validated by
                // the sizing pass immediately above.
                unsafe {
                    surface_create_info_extension_size(expected_structure_type, header.sType)
                        .unwrap_unchecked()
                }
            };
            // SAFETY: The sizing pass validated every node, `destination` has
            // room for its aligned size, and Vulkan makes each source node
            // readable according to its sType.
            unsafe { core::ptr::copy_nonoverlapping(source_node.cast::<u8>(), destination, size) };
            let copied = destination.cast::<VkBaseInStructure<'static>>();
            if !previous.is_null() {
                // SAFETY: `previous` points into the same owned allocation and
                // its pNext field is writable.
                unsafe { (*previous).pNext = copied };
            }
            previous = copied;
            destination = unsafe { destination.add(aligned_size(size).unwrap_unchecked()) };
            source_node = header.pNext;
            first = false;
        }
        Ok(Self { allocation })
    }

    const fn as_ptr(&self) -> *const c_void {
        self.allocation.as_ptr().cast_const().cast()
    }
}

// The copied Vulkan structures contain application/platform pointers whose
// cross-thread validity is governed by Vulkan's external synchronization rules.
unsafe impl Send for OwnedCreateInfo {}
unsafe impl Sync for OwnedCreateInfo {}

pub(crate) struct DeferredSurface {
    command_name: &'static CStr,
    extension_id: u16,
    create_info: OwnedCreateInfo,
    create_native: ErasedSurfaceCreate,
    allocator: Option<VkAllocationCallbacks<'static>>,
    native_surfaces: Mutex<LoaderArray<VkSurfaceKHR>>,
}

impl DeferredSurface {
    unsafe fn new<T: Copy>(
        create_info: *const T,
        expected_structure_type: VkStructureType,
        allocator: *const VkAllocationCallbacks<'_>,
        instance_allocator: Option<&VkAllocationCallbacks<'static>>,
        command_name: &'static CStr,
        extension_id: u16,
        icd_count: usize,
    ) -> Result<LoaderBox<Self>, VkResult> {
        // SAFETY: Propagates the Vulkan command's readable-pointer contract.
        let create_info = unsafe {
            OwnedCreateInfo::copy_from(create_info, expected_structure_type, instance_allocator)
        }?;
        let allocator = if allocator.is_null() {
            None
        } else {
            // SAFETY: Allocation callbacks are copied by value for the surface
            // lifetime, exactly as required for deferred native creation.
            Some(unsafe {
                core::mem::transmute::<VkAllocationCallbacks<'_>, VkAllocationCallbacks<'static>>(
                    allocator.read(),
                )
            })
        };
        let native_surfaces = LoaderArray::filled(
            instance_allocator,
            icd_count,
            VkSurfaceKHR::NULL,
            vk::VkSystemAllocationScope::INSTANCE,
        )?;
        LoaderBox::new(
            instance_allocator,
            Self {
                command_name,
                extension_id,
                create_info,
                create_native: create_native_surface::<T>,
                allocator,
                native_surfaces: Mutex::new(native_surfaces),
            },
            vk::VkSystemAllocationScope::OBJECT,
        )
    }

    fn allocator(&self) -> *const VkAllocationCallbacks<'_> {
        self.allocator
            .as_ref()
            .map_or(core::ptr::null(), core::ptr::from_ref)
    }

    pub(crate) unsafe fn native(
        &self,
        icd_index: usize,
        icd: &IcdInstance,
    ) -> Result<VkSurfaceKHR, VkResult> {
        if !icd.is_active() {
            return Err(VkResult::ERROR_SURFACE_LOST_KHR);
        }
        let mut native_surfaces = self.native_surfaces.lock();
        let Some(native) = native_surfaces.get_mut(icd_index) else {
            return Err(VkResult::ERROR_SURFACE_LOST_KHR);
        };
        if *native == VkSurfaceKHR::NULL {
            // SAFETY: The erased function was monomorphized for the copied
            // create-info type stored by this surface.
            *native = unsafe { (self.create_native)(self, icd) }?;
        }
        Ok(*native)
    }

    fn destroy_native_surfaces(&self, icds: &[IcdInstance]) {
        let mut native_surfaces = self.native_surfaces.lock();
        for (native, icd) in native_surfaces.iter_mut().zip(icds) {
            let handle = *native;
            if !icd.is_active() {
                debug_assert_eq!(handle, VkSurfaceKHR::NULL);
                continue;
            }
            if handle == VkSurfaceKHR::NULL {
                continue;
            }
            let Some(destroy) = icd.dispatch.vkDestroySurfaceKHR else {
                continue;
            };
            // SAFETY: The surface was created from this ICD instance and its
            // retained allocation callbacks match that creation.
            unsafe { destroy(icd.handle, handle, self.allocator()) };
            *native = VkSurfaceKHR::NULL;
        }
    }

    fn destroy_native_surface(&self, icd_index: usize, icd: &IcdInstance) {
        let mut native_surfaces = self.native_surfaces.lock();
        let Some(native) = native_surfaces.get_mut(icd_index) else {
            return;
        };
        if *native == VkSurfaceKHR::NULL {
            return;
        }
        if let Some(destroy) = icd.dispatch.vkDestroySurfaceKHR {
            // SAFETY: The native surface belongs to this ICD and its creation
            // allocator is retained by the loader surface.
            unsafe { destroy(icd.handle, *native, self.allocator()) };
        }
        *native = VkSurfaceKHR::NULL;
    }
}

pub(crate) enum SurfaceState {
    Deferred(LoaderBox<DeferredSurface>),
    #[cfg(any(target_os = "android", target_os = "ios"))]
    Passthrough(LoaderBox<IcdPassthroughSurface>),
}

impl SurfaceState {
    fn key(&self) -> usize {
        match self {
            Self::Deferred(surface) => surface.as_ptr() as usize,
            #[cfg(any(target_os = "android", target_os = "ios"))]
            Self::Passthrough(surface) => surface.as_ptr() as usize,
        }
    }

    unsafe fn native(&self, icd_index: usize, icd: &IcdInstance) -> Result<VkSurfaceKHR, VkResult> {
        match self {
            // SAFETY: The deferred surface and ICD are retained by the parent instance.
            Self::Deferred(surface) => unsafe { surface.native(icd_index, icd) },
            #[cfg(any(target_os = "android", target_os = "ios"))]
            Self::Passthrough(surface) => Ok(VkSurfaceKHR(surface.as_ptr() as usize as u64)),
        }
    }

    fn destroy_native_surfaces(&self, icds: &[IcdInstance]) {
        match self {
            Self::Deferred(surface) => surface.destroy_native_surfaces(icds),
            #[cfg(any(target_os = "android", target_os = "ios"))]
            Self::Passthrough(_) => {}
        }
    }

    fn destroy_native_surface(&self, icd_index: usize, icd: &IcdInstance) {
        match self {
            Self::Deferred(surface) => surface.destroy_native_surface(icd_index, icd),
            #[cfg(any(target_os = "android", target_os = "ios"))]
            Self::Passthrough(_) => {}
        }
    }
}

#[cfg(any(target_os = "android", target_os = "ios"))]
unsafe fn create_passthrough_surface<T>(
    create_info: *const T,
    expected_structure_type: VkStructureType,
    instance_allocator: Option<&VkAllocationCallbacks<'static>>,
) -> Result<Option<SurfaceState>, VkResult> {
    #[cfg(target_os = "android")]
    if expected_structure_type == VkStructureType::ANDROID_SURFACE_CREATE_INFO_KHR {
        if create_info.is_null() {
            return Err(VkResult::ERROR_INITIALIZATION_FAILED);
        }
        // SAFETY: The generated terminator pairs this structure type with the
        // concrete Android create-info ABI.
        let create_info = unsafe { &*create_info.cast::<vk::VkAndroidSurfaceCreateInfoKHR<'_>>() };
        if create_info.sType != expected_structure_type {
            return Err(VkResult::ERROR_EXTENSION_NOT_PRESENT);
        }
        return LoaderBox::new(
            instance_allocator,
            IcdPassthroughSurface {
                platform: ICD_WSI_PLATFORM_ANDROID,
                object: create_info.window.cast_const().cast(),
            },
            vk::VkSystemAllocationScope::OBJECT,
        )
        .map(SurfaceState::Passthrough)
        .map(Some);
    }

    #[cfg(target_os = "ios")]
    if expected_structure_type == VkStructureType::IOS_SURFACE_CREATE_INFO_MVK {
        if create_info.is_null() {
            return Err(VkResult::ERROR_INITIALIZATION_FAILED);
        }
        // SAFETY: The generated terminator pairs this structure type with the
        // concrete iOS create-info ABI.
        let create_info = unsafe { &*create_info.cast::<vk::VkIOSSurfaceCreateInfoMVK<'_>>() };
        if create_info.sType != expected_structure_type {
            return Err(VkResult::ERROR_EXTENSION_NOT_PRESENT);
        }
        return LoaderBox::new(
            instance_allocator,
            IcdPassthroughSurface {
                platform: ICD_WSI_PLATFORM_IOS,
                object: create_info.pView,
            },
            vk::VkSystemAllocationScope::OBJECT,
        )
        .map(SurfaceState::Passthrough)
        .map(Some);
    }

    Ok(None)
}

unsafe fn create_native_surface<T: Copy>(
    surface: &DeferredSurface,
    icd: &IcdInstance,
) -> Result<VkSurfaceKHR, VkResult> {
    if !icd.enabled_extensions.contains(surface.extension_id) {
        return Err(VkResult::ERROR_EXTENSION_NOT_PRESENT);
    }
    // SAFETY: The command name and instance are retained by the same ICD.
    let create: Option<NativeSurfaceCreate<T>> =
        unsafe { icd.icd.resolve(icd.handle, surface.command_name) };
    let Some(create) = create else {
        return Err(VkResult::ERROR_EXTENSION_NOT_PRESENT);
    };
    let mut native = VkSurfaceKHR::NULL;
    // SAFETY: The copied create info has the `T` used to monomorphize this
    // function and remains live for the call.
    let result = unsafe {
        create(
            icd.handle,
            surface.create_info.as_ptr().cast::<T>(),
            surface.allocator(),
            &raw mut native,
        )
    };
    if result == VkResult::SUCCESS {
        // The loader forwards the ICD result and handle verbatim. Validating a
        // non-null success handle here would strengthen the ICD contract beyond
        Ok(native)
    } else {
        Err(result)
    }
}

fn surface_key(surface: VkSurfaceKHR) -> Option<usize> {
    let key = surface.0 as usize;
    (key != 0).then_some(key)
}

/// Creates a loader-owned surface whose ICD objects are materialized lazily.
///
/// # Safety
///
/// All handles and pointers must satisfy the corresponding Vulkan platform
/// surface creation command's contract.
pub(crate) unsafe fn create_loader_surface<T: Copy>(
    instance: VkInstance,
    create_info: *const T,
    expected_structure_type: VkStructureType,
    allocator: *const VkAllocationCallbacks<'_>,
    surface: *mut VkSurfaceKHR,
    command_name: &'static CStr,
    extension_id: u16,
) -> VkResult {
    if surface.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    // SAFETY: The Vulkan command requires a live instance handle.
    let Some(instance) = (unsafe { LoaderInstance::from_handle(instance) }) else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    if !instance.enabled_extensions.contains(extension_id) {
        return VkResult::ERROR_EXTENSION_NOT_PRESENT;
    }
    // Android and iOS retain upstream's legacy loader-owned WSI ABI and never
    // invoke an ICD surface-creation command.
    #[cfg(any(target_os = "android", target_os = "ios"))]
    let passthrough = unsafe {
        create_passthrough_surface(create_info, expected_structure_type, instance.allocator())
    };
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    let passthrough = Ok(None);
    let loader_surface = match passthrough {
        Ok(Some(surface)) => surface,
        Ok(None) => {
            // SAFETY: The platform create-info and optional callbacks are readable by contract.
            match unsafe {
                DeferredSurface::new(
                    create_info,
                    expected_structure_type,
                    allocator,
                    instance.allocator(),
                    command_name,
                    extension_id,
                    instance.icds.len(),
                )
            } {
                Ok(surface) => SurfaceState::Deferred(surface),
                Err(result) => return result,
            }
        }
        Err(result) => return result,
    };
    let key = loader_surface.key();
    let mut surfaces = instance.surfaces.lock();
    if surfaces.try_reserve(1).is_err() {
        return VkResult::ERROR_OUT_OF_HOST_MEMORY;
    }
    surfaces.insert(key, loader_surface);
    // SAFETY: The caller supplied writable output storage. Vulkan non-dispatchable
    // handles are wide enough to retain a native pointer on supported targets.
    unsafe { surface.write(VkSurfaceKHR(key as u64)) };
    VkResult::SUCCESS
}

pub(crate) unsafe fn native_surface(
    instance: &LoaderInstance,
    icd_index: usize,
    surface: VkSurfaceKHR,
) -> Result<VkSurfaceKHR, VkResult> {
    let Some(key) = surface_key(surface) else {
        return Err(VkResult::ERROR_SURFACE_LOST_KHR);
    };
    let surfaces = instance.surfaces.lock();
    let Some(surface) = surfaces.get(&key) else {
        return Err(VkResult::ERROR_SURFACE_LOST_KHR);
    };
    let Some(icd) = instance.icds.get(icd_index) else {
        return Err(VkResult::ERROR_SURFACE_LOST_KHR);
    };
    // SAFETY: The surface and ICD are retained by this loader instance.
    unsafe { surface.native(icd_index, icd) }
}

pub(crate) unsafe fn translate_physical_device_surface(
    physical_device: vk::VkPhysicalDevice,
    surface: VkSurfaceKHR,
) -> Result<VkSurfaceKHR, VkResult> {
    if surface == VkSurfaceKHR::NULL {
        return Ok(surface);
    }
    // SAFETY: Physical-device terminators receive a live loader wrapper.
    let device = unsafe { LoaderPhysicalDevice::from_handle(physical_device) }
        .ok_or(VkResult::ERROR_INITIALIZATION_FAILED)?;
    // SAFETY: The loader instance retains both the surface and the ICD.
    unsafe { native_surface(device.instance(), device.icd_index, surface) }
}

unsafe fn native_surface_info<'a>(
    device: &LoaderPhysicalDevice,
    surface_info: &vk::VkPhysicalDeviceSurfaceInfo2KHR<'a>,
) -> Result<vk::VkPhysicalDeviceSurfaceInfo2KHR<'a>, VkResult> {
    let mut native_info = *surface_info;
    if native_info.surface != VkSurfaceKHR::NULL {
        // SAFETY: The surface and ICD index are retained by the same loader instance.
        native_info.surface =
            unsafe { native_surface(device.instance(), device.icd_index, native_info.surface) }?;
    }
    Ok(native_info)
}

/// Implements the loader's `VK_KHR_get_surface_capabilities2` ICD boundary.
///
/// The input structure is copied so its loader-owned surface can be replaced
/// without mutating application memory. Its `pNext` chain is deliberately
/// forwarded unchanged, as required by the Vulkan command contract.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceSurfaceCapabilities2KHR(
    physical_device: vk::VkPhysicalDevice,
    surface_info: *const vk::VkPhysicalDeviceSurfaceInfo2KHR<'_>,
    surface_capabilities: *mut VkSurfaceCapabilities2KHR<'_>,
) -> VkResult {
    if surface_info.is_null() || surface_capabilities.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    // SAFETY: Both pointers were validated and remain live for this call.
    let surface_info = unsafe { &*surface_info };
    let surface_capabilities = unsafe { &mut *surface_capabilities };
    // SAFETY: The terminator is entered with a live loader physical-device wrapper.
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    // SAFETY: Both structures are readable/writable by the entry-point contract.
    // SAFETY: The copied input retains its live application-owned pNext chain.
    let native_info = match unsafe { native_surface_info(device, surface_info) } {
        Ok(info) => info,
        Err(result) => return result,
    };

    let icd = device.icd();
    if let Some(command) = icd.dispatch.vkGetPhysicalDeviceSurfaceCapabilities2KHR {
        // The extension permits this output structure even when an individual
        // ICD does not implement protected surfaces.
        unsafe { initialize_protected_surface_capabilities(surface_capabilities) };
        // SAFETY: The copied input and translated handles remain live for the call.
        let result =
            unsafe { command(device.native, &raw const native_info, surface_capabilities) };
        if !icd
            .enabled_extensions
            .contains(crate::VK_KHR_SURFACE_MAINTENANCE1_EXTENSION_ID)
            && !icd
                .enabled_extensions
                .contains(crate::VK_EXT_SURFACE_MAINTENANCE1_EXTENSION_ID)
        {
            // SAFETY: The application supplied valid input and output chains.
            unsafe { emulate_surface_maintenance1(surface_info, surface_capabilities) };
        }
        return result;
    }

    device.log_icd_emulation(EmulatedCommand::GetPhysicalDeviceSurfaceCapabilities2KHR);

    let Some(command) = icd.dispatch.vkGetPhysicalDeviceSurfaceCapabilitiesKHR else {
        // SAFETY: The output pointer was validated above.
        unsafe {
            surface_capabilities.surfaceCapabilities = VkSurfaceCapabilitiesKHR::DEFAULT;
            emulate_surface_maintenance1(surface_info, surface_capabilities);
        }
        return VkResult::SUCCESS;
    };
    // SAFETY: The native physical device and surface belong to this ICD.
    let result = unsafe {
        command(
            device.native,
            native_info.surface,
            &raw mut surface_capabilities.surfaceCapabilities,
        )
    };
    if !icd
        .enabled_extensions
        .contains(crate::VK_KHR_SURFACE_MAINTENANCE1_EXTENSION_ID)
        && !icd
            .enabled_extensions
            .contains(crate::VK_EXT_SURFACE_MAINTENANCE1_EXTENSION_ID)
    {
        // SAFETY: The application provided a valid output chain.
        unsafe { emulate_surface_maintenance1(surface_info, surface_capabilities) };
    }
    result
}

pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceSurfaceFormats2KHR(
    physical_device: vk::VkPhysicalDevice,
    surface_info: *const vk::VkPhysicalDeviceSurfaceInfo2KHR<'_>,
    surface_format_count: *mut u32,
    surface_formats: *mut VkSurfaceFormat2KHR<'_>,
) -> VkResult {
    if surface_info.is_null() || surface_format_count.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    // SAFETY: Both pointers were validated and remain live for this call.
    let surface_info = unsafe { &*surface_info };
    let surface_format_count = unsafe { &mut *surface_format_count };
    // SAFETY: The terminator is entered with a live loader physical-device wrapper.
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    // SAFETY: The input structure is readable by the command contract.
    // SAFETY: The copied input retains its live application-owned pNext chain.
    let native_info = match unsafe { native_surface_info(device, surface_info) } {
        Ok(info) => info,
        Err(result) => return result,
    };

    let icd = device.icd();
    if let Some(command) = icd.dispatch.vkGetPhysicalDeviceSurfaceFormats2KHR {
        // SAFETY: The copied input and translated handles remain live for the call.
        return unsafe {
            command(
                device.native,
                &raw const native_info,
                surface_format_count,
                surface_formats,
            )
        };
    }

    device.log_icd_emulation(EmulatedCommand::GetPhysicalDeviceSurfaceFormats2KHR);

    let Some(command) = icd.dispatch.vkGetPhysicalDeviceSurfaceFormatsKHR else {
        // SAFETY: The count pointer was validated above.
        *surface_format_count = 0;
        return VkResult::SUCCESS;
    };
    // SAFETY: The count pointer was validated above.
    let capacity = *surface_format_count as usize;
    if surface_formats.is_null() || capacity == 0 {
        // SAFETY: The native handles belong to this ICD and the count is writable.
        return unsafe {
            command(
                device.native,
                native_info.surface,
                surface_format_count,
                core::ptr::null_mut(),
            )
        };
    }

    // SAFETY: A non-null Vulkan enumeration output points to `count` writable elements.
    let output = unsafe { optional_output_slice(surface_formats, *surface_format_count) };
    unsafe {
        emulate_result_array::<_, _, STACK_SURFACE_FORMATS>(
            surface_format_count,
            output,
            |count, formats| command(device.native, native_info.surface, count, formats),
            |output, format| output.surfaceFormat = format,
        )
        .unwrap_or(VkResult::ERROR_OUT_OF_HOST_MEMORY)
    }
}

pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceSurfaceSupportKHR(
    physical_device: vk::VkPhysicalDevice,
    queue_family_index: u32,
    surface: VkSurfaceKHR,
    supported: *mut vk::VkBool32,
) -> VkResult {
    if supported.is_null() {
        crate::fatal_loader_error(
            c"NULL pointer passed into vkGetPhysicalDeviceSurfaceSupportKHR for pSupported!",
        );
    }
    // SAFETY: The required output pointer was validated above.
    let supported = unsafe { &mut *supported };
    *supported = 0;
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    let Some(command) = device.icd().dispatch.vkGetPhysicalDeviceSurfaceSupportKHR else {
        return VkResult::SUCCESS;
    };
    let Ok(native_surface) =
        (unsafe { native_surface(device.instance(), device.icd_index, surface) })
    else {
        // Upstream defines a platform-incompatible ICD as simply not supporting
        // presentation for this surface.
        return VkResult::SUCCESS;
    };
    unsafe { command(device.native, queue_family_index, native_surface, supported) }
}

pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceSurfaceCapabilities2EXT(
    physical_device: vk::VkPhysicalDevice,
    surface: VkSurfaceKHR,
    capabilities: *mut VkSurfaceCapabilities2EXT<'_>,
) -> VkResult {
    if capabilities.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    // SAFETY: The required output pointer was validated above.
    let capabilities = unsafe { &mut *capabilities };
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    let surface = match unsafe { native_surface(device.instance(), device.icd_index, surface) } {
        Ok(surface) => surface,
        Err(result) => return result,
    };
    let icd = device.icd();
    if let Some(command) = icd.dispatch.vkGetPhysicalDeviceSurfaceCapabilities2EXT {
        return unsafe { command(device.native, surface, capabilities) };
    }
    let Some(command) = icd.dispatch.vkGetPhysicalDeviceSurfaceCapabilitiesKHR else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    device.log_icd_emulation(EmulatedCommand::GetPhysicalDeviceSurfaceCapabilities2EXT);
    let mut base = VkSurfaceCapabilitiesKHR::DEFAULT;
    let result = unsafe { command(device.native, surface, &raw mut base) };
    if result != VkResult::SUCCESS {
        return result;
    }
    let output = capabilities;
    output.minImageCount = base.minImageCount;
    output.maxImageCount = base.maxImageCount;
    output.currentExtent = base.currentExtent;
    output.minImageExtent = base.minImageExtent;
    output.maxImageExtent = base.maxImageExtent;
    output.maxImageArrayLayers = base.maxImageArrayLayers;
    output.supportedTransforms = base.supportedTransforms;
    output.currentTransform = base.currentTransform;
    output.supportedCompositeAlpha = base.supportedCompositeAlpha;
    output.supportedUsageFlags = base.supportedUsageFlags;
    output.supportedSurfaceCounters = vk::VkSurfaceCounterFlagBitsEXT::EMPTY;
    if !output.pNext.is_null() {
        device.instance().log_loader_message(
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
            vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
            c"vkGetPhysicalDeviceSurfaceCapabilities2EXT: Emulation found unrecognized structure type in pSurfaceCapabilities->pNext - this struct will be ignored",
        );
    }
    VkResult::SUCCESS
}

unsafe fn initialize_protected_surface_capabilities(
    capabilities: &mut VkSurfaceCapabilities2KHR<'_>,
) {
    let next = capabilities.pNext;
    // SAFETY: The caller propagates the writable pNext-chain contract.
    unsafe {
        for_each_output_chain(next, |header| {
            if header.sType == VkStructureType::SURFACE_PROTECTED_CAPABILITIES_KHR {
                // SAFETY: The matching sType determines the concrete node layout.
                let protected =
                    core::ptr::from_mut(header).cast::<VkSurfaceProtectedCapabilitiesKHR<'_>>();
                (*protected).supportsProtected = 0;
            }
        });
    }
}

unsafe fn emulate_surface_maintenance1(
    surface_info: &vk::VkPhysicalDeviceSurfaceInfo2KHR<'_>,
    capabilities: &mut VkSurfaceCapabilities2KHR<'_>,
) {
    // SAFETY: The caller propagates the readable input-chain contract.
    let Some(present_mode) = (unsafe {
        crate::emulation::find_input_chain(
            surface_info.pNext,
            VkStructureType::SURFACE_PRESENT_MODE_KHR,
        )
    }) else {
        return;
    };
    // SAFETY: The matching sType determines the concrete node layout.
    let present_mode = unsafe {
        (*core::ptr::from_ref(present_mode).cast::<VkSurfacePresentModeKHR<'_>>()).presentMode
    };

    let next_out = capabilities.pNext;
    // SAFETY: The caller propagates the writable output-chain contract.
    unsafe {
        for_each_output_chain(next_out, |header| {
            match header.sType {
                VkStructureType::SURFACE_PRESENT_MODE_COMPATIBILITY_KHR => {
                    // SAFETY: The matching sType determines the concrete node layout.
                    let compatibility = &mut *core::ptr::from_mut(header)
                        .cast::<VkSurfacePresentModeCompatibilityKHR<'_>>();
                    if compatibility.pPresentModes.is_null() {
                        compatibility.presentModeCount = 1;
                    } else if compatibility.presentModeCount != 0 {
                        // SAFETY: A nonzero count makes the first element writable.
                        compatibility.pPresentModes.write(present_mode);
                        compatibility.presentModeCount = 1;
                    }
                }
                VkStructureType::SURFACE_PRESENT_SCALING_CAPABILITIES_KHR => {
                    // SAFETY: The matching sType determines the concrete node layout.
                    let scaling = &mut *core::ptr::from_mut(header)
                        .cast::<VkSurfacePresentScalingCapabilitiesKHR<'_>>();
                    scaling.supportedPresentScaling = vk::VkPresentScalingFlagBitsKHR::EMPTY;
                    scaling.supportedPresentGravityX = vk::VkPresentGravityFlagBitsKHR::EMPTY;
                    scaling.supportedPresentGravityY = vk::VkPresentGravityFlagBitsKHR::EMPTY;
                    // SAFETY: The root output structure is writable by contract.
                    let base = &capabilities.surfaceCapabilities;
                    scaling.minScaledImageExtent = base.minImageExtent;
                    scaling.maxScaledImageExtent = base.maxImageExtent;
                }
                _ => {}
            }
        });
    }
}

pub(crate) fn destroy_all_surfaces(instance: &LoaderInstance) {
    let mut surfaces = instance.surfaces.lock();
    for (_, surface) in surfaces.drain() {
        surface.destroy_native_surfaces(&instance.icds);
    }
}

pub(crate) fn destroy_icd_surfaces(instance: &LoaderInstance, icd_index: usize) {
    let Some(icd) = instance.icds.get(icd_index) else {
        return;
    };
    let surfaces = instance.surfaces.lock();
    for surface in surfaces.values() {
        surface.destroy_native_surface(icd_index, icd);
    }
}

/// Dispatches surface destruction through active instance layers.
///
/// # Safety
///
/// `instance` and `surface` must identify a live parent/child pair.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkDestroySurfaceKHR(
    instance: VkInstance,
    surface: VkSurfaceKHR,
    allocator: *const VkAllocationCallbacks<'_>,
) {
    // SAFETY: The Vulkan command requires a live instance handle.
    let loader = unsafe { LoaderInstance::from_handle(instance) }.unwrap_or_else(|| {
        crate::fatal_loader_error(
            c"vkDestroySurfaceKHR: Invalid instance [VUID-vkDestroySurfaceKHR-instance-parameter]",
        )
    });
    // SAFETY: The stable table was populated from the active top layer or the
    // loader terminator when no layers are active.
    let dispatch = unsafe { &*loader.dispatch() };
    let destroy = dispatch.vkDestroySurfaceKHR;
    debug_assert!(destroy.is_some());
    let destroy = unsafe { destroy.unwrap_unchecked() };
    unsafe { destroy(loader.chain_handle(), surface, allocator) };
}

/// Destroys loader-owned surface state at the bottom of an instance chain.
///
/// # Safety
///
/// `instance` and `surface` must identify a live parent/child pair.
pub(crate) unsafe extern "system" fn terminator_vkDestroySurfaceKHR(
    instance: VkInstance,
    surface: VkSurfaceKHR,
    _allocator: *const VkAllocationCallbacks<'_>,
) {
    let Some(instance) = (unsafe {
        LoaderInstance::from_handle(instance)
            .or_else(|| LoaderInstance::from_internal_handle(instance))
    }) else {
        return;
    };
    if surface == VkSurfaceKHR::NULL {
        return;
    }
    let Some(key) = surface_key(surface) else {
        return;
    };
    let Some(surface) = instance.surfaces.lock().remove(&key) else {
        return;
    };
    surface.destroy_native_surfaces(&instance.icds);
}

/// Creates a swapchain after translating its loader-owned surface.
///
/// # Safety
///
/// All handles and pointers must satisfy `vkCreateSwapchainKHR`'s contract.
pub(crate) unsafe extern "system" fn terminator_create_swapchain(
    device: VkDevice,
    create_info: *const VkSwapchainCreateInfoKHR<'_>,
    allocator: *const VkAllocationCallbacks<'_>,
    swapchain: *mut VkSwapchainKHR,
) -> VkResult {
    if create_info.is_null() || swapchain.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    let create_info = unsafe { &*create_info };
    // SAFETY: The Vulkan command requires a live loader device.
    let Some(device) = (unsafe { LoaderDevice::from_handle(device) }) else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    if create_info.surface == VkSurfaceKHR::NULL {
        crate::fatal_loader_error(c"vkCreateSwapchainKHR: pCreateInfo->surface is VK_NULL_HANDLE");
    }
    let mut native_info = *create_info;
    // SAFETY: The device retains its loader instance and owning ICD index.
    native_info.surface =
        match unsafe { native_surface(device.instance(), device.icd_index(), native_info.surface) }
        {
            Ok(surface) => surface,
            Err(result) => return result,
        };
    // SAFETY: The native device and resolver belong to the same ICD.
    let create: Option<PFN_vkCreateSwapchainKHR> =
        unsafe { load_typed(device.resolve(c"vkCreateSwapchainKHR")) };
    let Some(create) = create else {
        return VkResult::ERROR_EXTENSION_NOT_PRESENT;
    };
    // SAFETY: The translated create info remains live and all remaining
    // arguments retain their original Vulkan contracts.
    unsafe {
        create(
            device.icd_device,
            &raw const native_info,
            allocator,
            swapchain,
        )
    }
}

pub(crate) unsafe extern "system" fn terminator_get_device_group_surface_present_modes(
    device: VkDevice,
    surface: VkSurfaceKHR,
    modes: *mut VkDeviceGroupPresentModeFlagsKHR,
) -> VkResult {
    // SAFETY: The Vulkan command requires a live loader device.
    let Some(device) = (unsafe { LoaderDevice::from_handle(device) }) else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    // SAFETY: The native device and retained direct resolver belong together.
    let command: Option<PFN_vkGetDeviceGroupSurfacePresentModesKHR> =
        unsafe { load_typed(device.resolve(c"vkGetDeviceGroupSurfacePresentModesKHR")) };
    let Some(command) = command else {
        crate::platform::write_loader_log(
            crate::platform::LogFilter::Error,
            format_args!(
                "vkGetDeviceGroupSurfacePresentModesKHR: Driver's function pointer was NULL, returning VK_SUCCESS. Was either Vulkan 1.1 and VK_KHR_swapchain enabled or both the VK_KHR_device_group and VK_KHR_surface extensions enabled when using Vulkan 1.0?"
            ),
        );
        return VkResult::SUCCESS;
    };
    // SAFETY: The device retains its parent instance and ICD index.
    let surface = match unsafe { native_surface(device.instance(), device.icd_index(), surface) } {
        Ok(surface) => surface,
        Err(result) => return result,
    };
    // SAFETY: The translated surface and native device belong to the same ICD;
    // output validity is inherited from the Vulkan entry-point contract.
    unsafe { command(device.icd_device, surface, modes) }
}

pub(crate) unsafe extern "system" fn terminator_create_shared_swapchains(
    device: VkDevice,
    swapchain_count: u32,
    create_infos: *const VkSwapchainCreateInfoKHR<'_>,
    allocator: *const VkAllocationCallbacks<'_>,
    swapchains: *mut VkSwapchainKHR,
) -> VkResult {
    // SAFETY: The Vulkan command requires a live loader device.
    let Some(device) = (unsafe { LoaderDevice::from_handle(device) }) else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    // SAFETY: The native device and retained direct resolver belong together.
    let command: Option<PFN_vkCreateSharedSwapchainsKHR> =
        unsafe { load_typed(device.resolve(c"vkCreateSharedSwapchainsKHR")) };
    let Some(command) = command else {
        return VkResult::SUCCESS;
    };
    let count = swapchain_count as usize;
    if count != 0 && (create_infos.is_null() || swapchains.is_null()) {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    let Ok(mut native_infos) =
        ScratchArray::<VkSwapchainCreateInfoKHR<'_>, STACK_SWAPCHAIN_CREATE_INFOS>::try_new(count)
    else {
        return VkResult::ERROR_OUT_OF_HOST_MEMORY;
    };
    if count != 0 {
        // SAFETY: The command contract guarantees `count` readable create infos.
        unsafe { core::ptr::copy_nonoverlapping(create_infos, native_infos.as_mut_ptr(), count) };
    }
    // SAFETY: The input array was copied into the complete scratch buffer.
    let native_infos = unsafe { core::slice::from_raw_parts_mut(native_infos.as_mut_ptr(), count) };
    for create_info in &mut *native_infos {
        // SAFETY: Each loader surface belongs to the retained instance.
        create_info.surface = match unsafe {
            native_surface(device.instance(), device.icd_index(), create_info.surface)
        } {
            Ok(surface) => surface,
            Err(result) => return result,
        };
    }
    // SAFETY: The translated fixed-length array remains live for the call.
    unsafe {
        command(
            device.icd_device,
            swapchain_count,
            native_infos.as_ptr(),
            allocator,
            swapchains,
        )
    }
}

/// Dispatches swapchain creation through layers to the loader WSI terminator.
///
/// # Safety
///
/// All arguments must satisfy `vkCreateSwapchainKHR`'s Vulkan contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateSwapchainKHR(
    device: VkDevice,
    create_info: *const VkSwapchainCreateInfoKHR<'_>,
    allocator: *const VkAllocationCallbacks<'_>,
    swapchain: *mut VkSwapchainKHR,
) -> VkResult {
    // SAFETY: A valid dispatchable stores the loader-compatible table pointer
    // in its first word.
    let dispatch = unsafe { crate::device_dispatch(device.0.cast()) }
        .unwrap_or_else(|| crate::invalid_device_dispatch());
    let Some(create) = dispatch.vkCreateSwapchainKHR else {
        crate::fatal_loader_error(c"vkCreateSwapchainKHR: Driver's function pointer was NULL, returning VK_SUCCESS. Was the VK_KHR_swapchain extension enabled?");
    };
    // SAFETY: Forwarded from this entry point's Vulkan contract.
    unsafe { create(device, create_info, allocator, swapchain) }
}

/// Dispatches shared swapchain creation through the device chain.
///
/// # Safety
///
/// Arguments must satisfy `vkCreateSharedSwapchainsKHR`'s Vulkan contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateSharedSwapchainsKHR(
    device: VkDevice,
    swapchain_count: u32,
    create_infos: *const VkSwapchainCreateInfoKHR<'_>,
    allocator: *const VkAllocationCallbacks<'_>,
    swapchains: *mut VkSwapchainKHR,
) -> VkResult {
    // SAFETY: A live device stores a loader-compatible dispatch table.
    let dispatch = unsafe { crate::device_dispatch(device.0.cast()) }
        .unwrap_or_else(|| crate::invalid_device_dispatch());
    let command = dispatch.vkCreateSharedSwapchainsKHR;
    debug_assert!(command.is_some());
    // SAFETY: GPA only exposes this trampoline when its extension is enabled.
    let command = unsafe { command.unwrap_unchecked() };
    // SAFETY: Forwarded from this entry point's contract.
    unsafe { command(device, swapchain_count, create_infos, allocator, swapchains) }
}

/// Dispatches device-group surface mode queries through the device chain.
///
/// # Safety
///
/// Arguments must satisfy `vkGetDeviceGroupSurfacePresentModesKHR`'s contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetDeviceGroupSurfacePresentModesKHR(
    device: VkDevice,
    surface: VkSurfaceKHR,
    modes: *mut VkDeviceGroupPresentModeFlagsKHR,
) -> VkResult {
    // SAFETY: A live device stores a loader-compatible dispatch table.
    let dispatch = unsafe { crate::device_dispatch(device.0.cast()) }
        .unwrap_or_else(|| crate::invalid_device_dispatch());
    let command = dispatch.vkGetDeviceGroupSurfacePresentModesKHR;
    debug_assert!(command.is_some());
    // SAFETY: GPA only exposes this trampoline when a provider is enabled.
    let command = unsafe { command.unwrap_unchecked() };
    // SAFETY: Forwarded from this entry point's contract.
    unsafe { command(device, surface, modes) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passthrough_surface_matches_vk_icd_layout() {
        let object_offset = core::mem::offset_of!(IcdPassthroughSurface, object);
        assert_eq!(object_offset, core::mem::align_of::<*const c_void>());
        assert_eq!(
            core::mem::size_of::<IcdPassthroughSurface>(),
            object_offset + core::mem::size_of::<*const c_void>()
        );
    }

    #[test]
    fn retained_display_surface_chain_owns_its_pnext_nodes() {
        let stereo = vk::VkDisplaySurfaceStereoCreateInfoNV {
            stereoType: vk::VkDisplaySurfaceStereoTypeNV(1),
            ..vk::VkDisplaySurfaceStereoCreateInfoNV::DEFAULT
        };
        let root = vk::VkDisplaySurfaceCreateInfoKHR {
            pNext: core::ptr::from_ref(&stereo).cast(),
            planeIndex: 7,
            ..vk::VkDisplaySurfaceCreateInfoKHR::DEFAULT
        };
        let owned = unsafe {
            OwnedCreateInfo::copy_from(
                &raw const root,
                VkStructureType::DISPLAY_SURFACE_CREATE_INFO_KHR,
                None,
            )
        }
        .unwrap();
        let copied_root = unsafe {
            &*owned
                .as_ptr()
                .cast::<vk::VkDisplaySurfaceCreateInfoKHR<'_>>()
        };
        assert_eq!(copied_root.planeIndex, 7);
        assert_ne!(copied_root.pNext, root.pNext);
        let copied_stereo = unsafe {
            &*copied_root
                .pNext
                .cast::<vk::VkDisplaySurfaceStereoCreateInfoNV<'_>>()
        };
        assert_eq!(copied_stereo.stereoType.0, 1);
        assert!(copied_stereo.pNext.is_null());
    }

    #[test]
    fn unsupported_surface_chain_node_is_rejected() {
        let unsupported = VkBaseInStructure {
            sType: VkStructureType::APPLICATION_INFO,
            ..VkBaseInStructure::DEFAULT
        };
        let root = vk::VkHeadlessSurfaceCreateInfoEXT {
            pNext: core::ptr::from_ref(&unsupported).cast(),
            ..vk::VkHeadlessSurfaceCreateInfoEXT::DEFAULT
        };
        assert!(matches!(
            unsafe {
                OwnedCreateInfo::copy_from(
                    &raw const root,
                    VkStructureType::HEADLESS_SURFACE_CREATE_INFO_EXT,
                    None,
                )
            },
            Err(VkResult::ERROR_EXTENSION_NOT_PRESENT)
        ));
    }

    #[test]
    fn incorrect_surface_root_type_is_rejected() {
        let root = vk::VkHeadlessSurfaceCreateInfoEXT {
            sType: VkStructureType::APPLICATION_INFO,
            ..vk::VkHeadlessSurfaceCreateInfoEXT::DEFAULT
        };
        assert!(matches!(
            unsafe {
                OwnedCreateInfo::copy_from(
                    &raw const root,
                    VkStructureType::HEADLESS_SURFACE_CREATE_INFO_EXT,
                    None,
                )
            },
            Err(VkResult::ERROR_EXTENSION_NOT_PRESENT)
        ));
    }
}
