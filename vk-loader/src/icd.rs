//! Installable Client Driver loading and interface negotiation.

use crate::LoaderPathExt;
use crate::sync::{GlobalMutex, MutexAcquire, MutexInit, ObjectMutex};
use crate::{ExtensionSet, allocation, discovery, layer, platform, unknown};
use crate::{
    InstanceDispatchTable,
    discovery::DriverManifest,
    load_typed,
    platform::{LoaderLibrary, LogFilter},
};
use core::ffi::{CStr, c_char, c_void};
use core::sync::atomic::{AtomicBool, Ordering};
use std::path::{Path, PathBuf};
use vk::{
    PFN_vkCreateInstance, PFN_vkEnumerateInstanceVersion, PFN_vkGetInstanceProcAddr,
    VK_API_VERSION_1_0, VkInstance, VkResult,
};

#[cfg(windows)]
pub(crate) type EnumerateAdapterPhysicalDevices = unsafe extern "system" fn(
    VkInstance,
    platform::AdapterLuid,
    *mut u32,
    *mut vk::VkPhysicalDevice,
) -> VkResult;

type NegotiateInterface = unsafe extern "system" fn(*mut u32) -> VkResult;
pub(crate) type GetPhysicalDeviceProcAddr =
    unsafe extern "system" fn(VkInstance, *const c_char) -> vk::PFN_vkVoidFunction;

const CURRENT_INTERFACE_VERSION: u32 = 7;

static PRELOADED_ICDS: GlobalMutex<Option<Vec<ScannedIcd>>> = GlobalMutex::new(None);

/// Retains one scanned reference to each currently discoverable ICD.
///
/// Upstream does this before global extension enumeration so repeated scans do
/// not unload and reinitialize driver modules between calls.
pub(crate) fn preload_icds() -> Result<(), VkResult> {
    let mut preloaded = PRELOADED_ICDS.try_lock()?;
    if preloaded.is_some() {
        return Ok(());
    }
    *preloaded = Some(scan_global_icds()?);
    Ok(())
}

/// Performs the transient ICD scan used by global extension enumeration.
/// Upstream retains a preloaded reference and independently scans a second
/// set so driver modules cannot unload between repeated enumeration calls.
pub(crate) fn scan_global_icds() -> Result<Vec<ScannedIcd>, VkResult> {
    let scan = discovery::scan_drivers();
    emit_global_scan_diagnostics(&scan);
    if crate::pending::json_allocation_failed() {
        return Err(VkResult::ERROR_OUT_OF_HOST_MEMORY);
    }
    let mut loaded = Vec::new();
    loaded
        .try_reserve_exact(scan.manifests.len())
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    for manifest in &scan.manifests {
        if let Some(icd) = load_global_icd(manifest)? {
            loaded.push(icd);
        }
    }
    Ok(loaded)
}

#[cold]
#[inline(never)]
fn emit_global_scan_diagnostics(scan: &discovery::DriverScan) {
    platform::write_loader_category_log(
        LogFilter::Driver,
        format_args!("Searching for driver manifest files"),
    );
    platform::write_loader_category_log(
        LogFilter::Driver,
        format_args!("   In following locations:"),
    );
    for root in &scan.search_roots {
        platform::write_loader_category_log(
            LogFilter::Driver,
            format_args!("      {}", root.loader_display()),
        );
    }
    if scan.reported_files.is_empty() {
        platform::write_loader_category_log(LogFilter::Driver, format_args!("   Found no files"));
    } else {
        platform::write_loader_category_log(
            LogFilter::Driver,
            format_args!("   Found the following files:"),
        );
        for path in &scan.reported_files {
            platform::write_loader_category_log(
                LogFilter::Driver,
                format_args!("      {}", path.loader_display()),
            );
        }
    }
}

fn load_global_icd(manifest: &DriverManifest) -> Result<Option<ScannedIcd>, VkResult> {
    platform::write_loader_category_log(
        LogFilter::Driver,
        format_args!(
            "Found ICD manifest file {}, version {}",
            manifest.manifest_path.loader_display(),
            layer::ManifestVersion(manifest.manifest_version),
        ),
    );
    let displayed_library_path = manifest
        .library_path
        .to_str()
        .and_then(|path| {
            crate::rfind_bytes(path.as_bytes(), b"/./").map(|index| &path[index + 1..])
        })
        .map_or(manifest.library_path.as_path(), Path::new);
    platform::write_loader_log_with_category(
        LogFilter::Debug,
        LogFilter::Driver,
        format_args!(
            "Searching for ICD drivers named {}",
            displayed_library_path.loader_display()
        ),
    );
    if vk::VK_API_VERSION_VARIANT(manifest.api_version) != 0 || !manifest.architecture_supported {
        return Ok(None);
    }
    let icd = match ScannedIcd::load(manifest) {
        Ok((icd, _, _)) => icd,
        Err(ScannedIcdLoadError::OutOfMemory) => return Err(VkResult::ERROR_OUT_OF_HOST_MEMORY),
        Err(ScannedIcdLoadError::MutexInitialization(error)) => return Err(error),
        Err(_) => return Ok(None),
    };
    if manifest.library_path.is_absolute()
        && icd.library_path() != Some(manifest.library_path.as_path())
        && !platform::path_normalizes(&manifest.library_path)?
    {
        platform::write_loader_log(
            LogFilter::Debug,
            format_args!(
                "normalize_path: Call to realpath() failed with error code 2 when given the path {}",
                manifest.library_path.loader_display()
            ),
        );
        if let Some(loaded_path) = icd.library_path() {
            platform::write_loader_log_with_category(
                LogFilter::Warning,
                LogFilter::Layer,
                format_args!(
                    "Path to given binary {} was found to differ from OS loaded path {}",
                    manifest.library_path.loader_display(),
                    loaded_path.loader_display()
                ),
            );
        }
    }
    Ok(Some(icd))
}

pub(crate) fn unload_preloaded_icds() {
    if let Some(mut preloaded) = PRELOADED_ICDS.lock_if_initialized() {
        *preloaded = None;
    }
}

/// Releases native synchronization only after loader entry points have stopped.
#[cfg(not(all(target_vendor = "apple", feature = "apple-static-loader")))]
pub(crate) unsafe fn destroy_preloaded_icd_lock() {
    // SAFETY: The caller supplies the library-termination exclusion guarantee.
    unsafe { PRELOADED_ICDS.destroy() };
}

pub(crate) fn unload_preloaded_icd(path: &Path) {
    let Some(mut preloaded) = PRELOADED_ICDS.lock_if_initialized() else {
        return;
    };
    let Some(icds) = preloaded.as_mut() else {
        return;
    };
    icds.retain(|icd| icd.library_path() != Some(path));
}

pub(crate) struct ScannedIcd {
    pub(crate) get_instance_proc_addr: PFN_vkGetInstanceProcAddr,
    pub(crate) create_instance: PFN_vkCreateInstance,
    pub(crate) get_physical_device_proc_addr: Option<GetPhysicalDeviceProcAddr>,
    pub(crate) api_version: u32,
    pub(crate) interface_version: u32,
    #[cfg(windows)]
    pub(crate) enumerate_adapter_physical_devices: Option<EnumerateAdapterPhysicalDevices>,
    library_path: Option<PathBuf>,
    library: ObjectMutex<Option<LoaderLibrary>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ManifestApiVersionStatus {
    Consistent,
    EnumerateInstanceVersionMissing,
    EnumerateInstanceVersionReturned(u32),
}

pub(crate) enum ScannedIcdLoadError {
    OutOfMemory,
    MutexInitialization(VkResult),
    OpenLibrary {
        message: String,
        wrong_bit_type: bool,
    },
    MissingPrefixedGetInstanceProcAddr(u32),
    InvalidInterface,
}

pub(crate) enum DirectIcdError {
    MutexInitialization(VkResult),
    MissingNegotiate,
    IncompatibleInterface(u32),
    MissingCreateInstance,
    MissingEnumerateExtensions,
    EnumerateVersion(VkResult),
}

pub(crate) struct IcdInstance {
    pub(crate) icd: ScannedIcd,
    pub(crate) handle: VkInstance,
    pub(crate) dispatch: InstanceDispatchTable,
    pub(crate) enabled_extensions: ExtensionSet,
    pub(crate) unknown_physical_device_dispatch: unknown::UnknownDispatchTable,
    active: AtomicBool,
}

impl ScannedIcd {
    pub(crate) fn load(
        manifest: &DriverManifest,
    ) -> Result<(Self, ManifestApiVersionStatus, bool), ScannedIcdLoadError> {
        // SAFETY: Driver lifetime is retained by `ScannedIcd` and all queried
        // symbols are copied function pointers with Vulkan-defined ABIs.
        let library =
            unsafe { LoaderLibrary::open_driver(&manifest.library_path) }.map_err(|error| {
                let wrong_bit_type = error.is_wrong_bit_type();
                match error.into_message(&manifest.library_path) {
                    Ok(message) => ScannedIcdLoadError::OpenLibrary {
                        wrong_bit_type,
                        message,
                    },
                    Err(VkResult::ERROR_OUT_OF_HOST_MEMORY) => ScannedIcdLoadError::OutOfMemory,
                    Err(error) => ScannedIcdLoadError::MutexInitialization(error),
                }
            })?;
        let (interface_version, get_instance_proc_addr, uses_deprecated_interface) =
            negotiate_icd_interface(&library)?;
        let mut get_physical_device_proc_addr = if interface_version >= 7 {
            // SAFETY: Interface 7 exposes the ICD GPDPA through ICD GIPA.
            unsafe {
                load_typed(get_instance_proc_addr(
                    VkInstance::NULL,
                    c"vk_icdGetPhysicalDeviceProcAddr".as_ptr(),
                ))
            }
        } else {
            None
        };
        if get_physical_device_proc_addr.is_none() && interface_version >= 3 {
            // SAFETY: Interfaces 3 through 6 export this symbol directly;
            // interface 7 drivers may also retain the export as a fallback.
            get_physical_device_proc_addr = unsafe {
                library
                    .get::<GetPhysicalDeviceProcAddr>(b"vk_icdGetPhysicalDeviceProcAddr\0")
                    .ok()
                    .map(|symbol| *symbol)
            };
        }
        #[cfg(windows)]
        let enumerate_adapter_physical_devices = {
            let mut enumerate = if interface_version >= 7 {
                // SAFETY: Interface 7 exposes private loader-driver entry
                // points through ICD GIPA.
                unsafe {
                    load_typed(get_instance_proc_addr(
                        VkInstance::NULL,
                        c"vk_icdEnumerateAdapterPhysicalDevices".as_ptr(),
                    ))
                }
            } else {
                None
            };
            if enumerate.is_none() && interface_version >= 6 {
                // SAFETY: Interface 6 exports this private entry point.
                enumerate = unsafe {
                    library
                        .get::<EnumerateAdapterPhysicalDevices>(
                            b"vk_icdEnumerateAdapterPhysicalDevices\0",
                        )
                        .ok()
                        .map(|symbol| *symbol)
                };
            }
            enumerate
        };
        // SAFETY: Null-instance ICD queries are required by every supported interface.
        let create_instance = unsafe {
            load_typed(get_instance_proc_addr(
                VkInstance::NULL,
                c"vkCreateInstance".as_ptr(),
            ))
        }
        .ok_or(ScannedIcdLoadError::InvalidInterface)?;

        let (api_version, version_status) =
            unsafe { query_manifest_api_version(get_instance_proc_addr, manifest) };
        Ok((
            Self {
                get_instance_proc_addr,
                create_instance,
                get_physical_device_proc_addr,
                api_version,
                interface_version,
                #[cfg(windows)]
                enumerate_adapter_physical_devices,
                library_path: platform::loaded_library_path(
                    (get_instance_proc_addr as *const ()).cast::<c_void>(),
                )
                .map_err(|_| ScannedIcdLoadError::OutOfMemory)?
                .map_or_else(|| allocation::try_path(&manifest.library_path), Ok)
                .map(Some)
                .map_err(|_| ScannedIcdLoadError::OutOfMemory)?,
                library: ObjectMutex::try_new(Some(library))
                    .map_err(ScannedIcdLoadError::MutexInitialization)?,
            },
            version_status,
            uses_deprecated_interface,
        ))
    }

    pub(crate) unsafe fn load_direct(
        get_instance_proc_addr: PFN_vkGetInstanceProcAddr,
    ) -> Result<Self, DirectIcdError> {
        // SAFETY: Direct-driver-loading requires the application-provided GIPA
        // and all functions returned from it to remain live for the instance.
        let negotiate: NegotiateInterface = unsafe {
            load_typed(get_instance_proc_addr(
                VkInstance::NULL,
                c"vk_icdNegotiateLoaderICDInterfaceVersion".as_ptr(),
            ))
        }
        .ok_or(DirectIcdError::MissingNegotiate)?;
        let mut interface_version = CURRENT_INTERFACE_VERSION;
        // SAFETY: The writable version follows the loader/driver interface ABI.
        if unsafe { negotiate(&raw mut interface_version) } != VkResult::SUCCESS
            || interface_version < CURRENT_INTERFACE_VERSION
        {
            return Err(DirectIcdError::IncompatibleInterface(interface_version));
        }

        // SAFETY: Interface 7 makes these null-instance queries mandatory.
        let create_instance = unsafe {
            load_typed(get_instance_proc_addr(
                VkInstance::NULL,
                c"vkCreateInstance".as_ptr(),
            ))
        }
        .ok_or(DirectIcdError::MissingCreateInstance)?;
        let _: vk::PFN_vkEnumerateInstanceExtensionProperties = unsafe {
            load_typed(get_instance_proc_addr(
                VkInstance::NULL,
                c"vkEnumerateInstanceExtensionProperties".as_ptr(),
            ))
        }
        .ok_or(DirectIcdError::MissingEnumerateExtensions)?;
        let get_physical_device_proc_addr = unsafe {
            load_typed(get_instance_proc_addr(
                VkInstance::NULL,
                c"vk_icdGetPhysicalDeviceProcAddr".as_ptr(),
            ))
        };
        #[cfg(windows)]
        let enumerate_adapter_physical_devices = unsafe {
            load_typed(get_instance_proc_addr(
                VkInstance::NULL,
                c"vk_icdEnumerateAdapterPhysicalDevices".as_ptr(),
            ))
        };
        let enumerate_version: Option<PFN_vkEnumerateInstanceVersion> = unsafe {
            load_typed(get_instance_proc_addr(
                VkInstance::NULL,
                c"vkEnumerateInstanceVersion".as_ptr(),
            ))
        };
        let mut api_version = vk::VK_API_VERSION_1_1;
        if let Some(enumerate_version) = enumerate_version {
            // SAFETY: `api_version` is writable for the duration of the call.
            let result = unsafe { enumerate_version(&raw mut api_version) };
            if result != VkResult::SUCCESS {
                return Err(DirectIcdError::EnumerateVersion(result));
            }
        }

        Ok(Self {
            get_instance_proc_addr,
            create_instance,
            get_physical_device_proc_addr,
            api_version,
            interface_version,
            #[cfg(windows)]
            enumerate_adapter_physical_devices,
            library_path: None,
            library: ObjectMutex::try_new(None).map_err(DirectIcdError::MutexInitialization)?,
        })
    }

    pub(crate) unsafe fn resolve<T: Copy>(&self, instance: VkInstance, name: &CStr) -> Option<T> {
        // SAFETY: Caller supplies an instance created by this ICD and a live C string.
        unsafe {
            load_typed((self.get_instance_proc_addr)(
                instance,
                name.as_ptr().cast::<c_char>(),
            ))
        }
    }

    pub(crate) fn library_path(&self) -> Option<&Path> {
        self.library_path.as_deref()
    }

    pub(crate) fn unload_library(&self) {
        *self.library.lock() = None;
    }
}

impl IcdInstance {
    pub(crate) fn library_path(&self) -> Option<&Path> {
        self.icd.library_path()
    }

    /// Initializes the activation flag in reserved instance storage.
    ///
    /// # Safety
    /// `output` must point to writable, aligned storage for an `IcdInstance`.
    pub(crate) unsafe fn initialize_active(output: *mut Self) {
        // SAFETY: The caller supplies the uninitialized reserved vector slot
        // after every preceding field has been written.
        unsafe { core::ptr::addr_of_mut!((*output).active).write(AtomicBool::new(true)) };
    }

    #[inline]
    pub(crate) fn is_active(&self) -> bool {
        self.active.load(Ordering::Acquire)
    }

    /// Claims this ICD for one-time native teardown.
    pub(crate) fn begin_retire(&self) -> bool {
        self.active.swap(false, Ordering::AcqRel)
    }
}

#[cold]
unsafe fn query_manifest_api_version(
    get_instance_proc_addr: PFN_vkGetInstanceProcAddr,
    manifest: &DriverManifest,
) -> (u32, ManifestApiVersionStatus) {
    if manifest.api_version >= vk::VK_API_VERSION_1_1 {
        // SAFETY: Null-instance ICD queries are required by the interface.
        let enumerate_version: Option<PFN_vkEnumerateInstanceVersion> = unsafe {
            load_typed(get_instance_proc_addr(
                VkInstance::NULL,
                c"vkEnumerateInstanceVersion".as_ptr(),
            ))
        };
        let mut version = VK_API_VERSION_1_0;
        match enumerate_version {
            Some(enumerate_version) => {
                // SAFETY: `version` points to writable local storage.
                let result = unsafe { enumerate_version(&raw mut version) };
                if result == VkResult::SUCCESS {
                    let status = if version >= vk::VK_API_VERSION_1_1 {
                        ManifestApiVersionStatus::Consistent
                    } else {
                        ManifestApiVersionStatus::EnumerateInstanceVersionReturned(version)
                    };
                    (version, status)
                } else {
                    (
                        VK_API_VERSION_1_0,
                        ManifestApiVersionStatus::EnumerateInstanceVersionReturned(
                            VK_API_VERSION_1_0,
                        ),
                    )
                }
            }
            None => (
                VK_API_VERSION_1_0,
                ManifestApiVersionStatus::EnumerateInstanceVersionMissing,
            ),
        }
    } else {
        (VK_API_VERSION_1_0, ManifestApiVersionStatus::Consistent)
    }
}

#[cold]
fn negotiate_icd_interface(
    library: &LoaderLibrary,
) -> Result<(u32, PFN_vkGetInstanceProcAddr, bool), ScannedIcdLoadError> {
    // SAFETY: Symbol type is defined by the loader-driver interface.
    let direct_gipa = unsafe {
        library
            .get::<PFN_vkGetInstanceProcAddr>(b"vk_icdGetInstanceProcAddr\0")
            .ok()
            .map(|symbol| *symbol)
    };
    // SAFETY: Symbol type is defined by the loader-driver interface.
    let mut negotiate = unsafe {
        library
            .get::<NegotiateInterface>(b"vk_icdNegotiateLoaderICDInterfaceVersion\0")
            .ok()
            .map(|symbol| *symbol)
    };
    if negotiate.is_none()
        && let Some(gipa) = direct_gipa
    {
        // SAFETY: Null-instance ICD queries are required by the interface.
        negotiate = unsafe {
            load_typed(gipa(
                VkInstance::NULL,
                c"vk_icdNegotiateLoaderICDInterfaceVersion".as_ptr(),
            ))
        };
    }

    let mut interface_version = u32::from(direct_gipa.is_some());
    if let Some(negotiate) = negotiate {
        interface_version = CURRENT_INTERFACE_VERSION;
        // SAFETY: `interface_version` is writable and negotiation owns no pointer.
        if unsafe { negotiate(&raw mut interface_version) } != VkResult::SUCCESS {
            return Err(ScannedIcdLoadError::InvalidInterface);
        }
        // Interface version 2 and newer require the ICD-prefixed GIPA.
        // Falling back to Vulkan's public GIPA here would incorrectly
        // accept a driver that violates the loader/driver ABI contract.
        if interface_version != 0 && direct_gipa.is_none() {
            return Err(ScannedIcdLoadError::MissingPrefixedGetInstanceProcAddr(
                interface_version,
            ));
        }
    }

    let (get_instance_proc_addr, uses_deprecated_interface) = if let Some(gipa) = direct_gipa {
        (gipa, false)
    } else {
        // SAFETY: Version-zero ICDs export the Vulkan-named entry point.
        unsafe {
            library
                .get::<PFN_vkGetInstanceProcAddr>(b"vkGetInstanceProcAddr\0")
                .ok()
                .map(|symbol| *symbol)
        }
        .ok_or(ScannedIcdLoadError::InvalidInterface)
        .map(|gipa| (gipa, true))?
    };
    Ok((
        interface_version,
        get_instance_proc_addr,
        uses_deprecated_interface,
    ))
}
