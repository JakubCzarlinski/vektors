//! Windows registry, adapter, and package discovery.

#[cfg(windows)]
pub(super) mod directory;
#[cfg(windows)]
pub(super) mod paths;

#[cfg(windows)]
use super::{
    AdapterLuid, ManifestDirectory, has_elevated_privileges, reinterpret_function_pointer,
};
#[cfg(windows)]
use std::path::Path;
use std::path::PathBuf;
#[cfg(windows)]
use windows_sys::{
    Win32::{
        Foundation::{ERROR_INSUFFICIENT_BUFFER, ERROR_SUCCESS},
        System::{
            LibraryLoader::{GetModuleHandleA, GetProcAddress, LoadLibraryExA},
            Registry::{
                HKEY, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_QUERY_VALUE, RegCloseKey,
                RegEnumValueA, RegOpenKeyExA,
            },
        },
    },
    core::{GUID, w},
};

#[cfg(windows)]
#[cold]
#[inline(never)]
fn registry_values(location: &core::ffi::CStr, current_user: bool) -> Box<[PathBuf]> {
    let hive = if current_user {
        HKEY_CURRENT_USER
    } else {
        HKEY_LOCAL_MACHINE
    };
    let mut key: HKEY = core::ptr::null_mut();
    // SAFETY: The registry path is NUL-terminated and `key` is writable.
    if unsafe { RegOpenKeyExA(hive, location.as_ptr().cast(), 0, KEY_QUERY_VALUE, &mut key) }
        != ERROR_SUCCESS
    {
        return Box::default();
    }

    let mut paths = Vec::new();
    let mut name = [0_u8; 2048];
    for index in 0.. {
        let mut name_length = name.len() as u32;
        let mut value = 0_u32;
        let mut value_length = core::mem::size_of_val(&value) as u32;
        // SAFETY: All buffers are writable for the lengths supplied. This is
        // the same RegEnumValueA contract used by upstream and its test shim.
        let status = unsafe {
            RegEnumValueA(
                key,
                index,
                name.as_mut_ptr(),
                &mut name_length,
                core::ptr::null(),
                core::ptr::null_mut(),
                (&mut value as *mut u32).cast(),
                &mut value_length,
            )
        };
        if status != ERROR_SUCCESS {
            break;
        }
        if value_length == core::mem::size_of_val(&value) as u32 && value == 0 {
            let length = (name_length as usize).min(name.len());
            let length = name[..length]
                .iter()
                .position(|byte| *byte == 0)
                .unwrap_or(length);
            if let Ok(path) = core::str::from_utf8(&name[..length]) {
                let inserted = crate::allocation::try_path(Path::new(path))
                    .and_then(|path| crate::allocation::try_push(&mut paths, path));
                if inserted.is_err() {
                    crate::pending::mark_json_allocation_failed();
                    break;
                }
            }
        }
    }
    // SAFETY: `key` was opened successfully and is owned by this function.
    unsafe { RegCloseKey(key) };
    crate::allocation::try_into_boxed_slice(paths).unwrap_or_else(|_| {
        crate::pending::mark_json_allocation_failed();
        Box::default()
    })
}

#[cfg(windows)]
pub(crate) struct RegistryDiagnostics {
    pub(crate) located: Box<[PathBuf]>,
    pub(crate) no_unique_files: bool,
}

#[cfg(windows)]
pub(crate) fn registry_manifest_files_with_diagnostics(
    directory: ManifestDirectory,
) -> (Box<[PathBuf]>, RegistryDiagnostics) {
    let result = (|| {
        let location = match directory {
            ManifestDirectory::Driver => c"SOFTWARE\\Khronos\\Vulkan\\Drivers",
            ManifestDirectory::ImplicitLayer => c"SOFTWARE\\Khronos\\Vulkan\\ImplicitLayers",
            ManifestDirectory::ExplicitLayer => c"SOFTWARE\\Khronos\\Vulkan\\ExplicitLayers",
        };
        let mut paths = Vec::new();
        if directory != ManifestDirectory::ExplicitLayer
            && let Some(package) = app_package_manifest_path()
        {
            crate::allocation::try_push(&mut paths, package)?;
        }
        for path in d3dkmt_manifest_files(directory) {
            crate::allocation::try_push(&mut paths, path)?;
        }
        let mut located = Vec::new();
        let count_before = paths.len();
        extend_registry_paths(&mut paths, &mut located, registry_values(location, false))?;
        if directory != ManifestDirectory::Driver && !has_elevated_privileges() {
            extend_registry_paths(&mut paths, &mut located, registry_values(location, true))?;
        }
        let no_unique_files = paths.len() == count_before;
        Ok::<_, vk::VkResult>((
            crate::allocation::try_into_boxed_slice(paths)?,
            RegistryDiagnostics {
                located: crate::allocation::try_into_boxed_slice(located)?,
                no_unique_files,
            },
        ))
    })();
    result.unwrap_or_else(|_| {
        crate::pending::mark_json_allocation_failed();
        (
            Box::default(),
            RegistryDiagnostics {
                located: Box::default(),
                no_unique_files: true,
            },
        )
    })
}

#[cfg(windows)]
pub(crate) fn registry_manifest_files(directory: ManifestDirectory) -> Box<[PathBuf]> {
    registry_manifest_files_with_diagnostics(directory).0
}

#[cfg(any(windows, test))]
pub(super) fn extend_registry_paths(
    paths: &mut Vec<PathBuf>,
    located: &mut Vec<PathBuf>,
    values: impl IntoIterator<Item = PathBuf>,
) -> Result<(), vk::VkResult> {
    for path in values {
        if !paths.contains(&path) {
            let unique_path = crate::allocation::try_path(&path)?;
            crate::allocation::try_push(paths, unique_path)?;
        }
        crate::allocation::try_push(located, path)?;
    }
    Ok(())
}

#[cfg(windows)]
#[cold]
#[inline(never)]
fn d3dkmt_manifest_files(directory: ManifestDirectory) -> Box<[PathBuf]> {
    const STATUS_SUCCESS: i32 = 0;
    const LOAD_LIBRARY_SEARCH_SYSTEM32: u32 = 0x0000_0800;
    const QUERY_TYPE_REGISTRY: u32 = 48;
    const QUERY_REGISTRY_ADAPTER_KEY: u32 = 1;
    const QUERY_REGISTRY_STATUS_SUCCESS: u32 = 0;
    const QUERY_REGISTRY_STATUS_BUFFER_OVERFLOW: u32 = 1;
    const REG_SZ: u32 = 1;
    const REG_MULTI_SZ: u32 = 7;

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct Luid {
        low_part: u32,
        high_part: i32,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct Adapter {
        handle: u32,
        luid: Luid,
        source_count: u32,
        present_move_regions_preferred: i32,
    }

    #[repr(C)]
    struct EnumAdapters {
        adapter_count: u32,
        adapters: *mut Adapter,
    }

    #[repr(C)]
    struct QueryAdapterInfo {
        handle: u32,
        kind: u32,
        private_data: *mut core::ffi::c_void,
        private_data_size: u32,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct QueryRegistryInfo {
        query_type: u32,
        query_flags: u32,
        value_name: [u16; 260],
        value_type: u32,
        physical_adapter_index: u32,
        output_value_size: u32,
        status: u32,
        output: u64,
    }

    type EnumAdaptersFn = unsafe extern "system" fn(*mut EnumAdapters) -> i32;
    type QueryAdapterInfoFn = unsafe extern "system" fn(*mut QueryAdapterInfo) -> i32;

    fn symbol<T: Copy>(
        module: windows_sys::Win32::Foundation::HMODULE,
        name: &core::ffi::CStr,
    ) -> Option<T> {
        // SAFETY: `module` is live and `name` is NUL-terminated.
        let address = unsafe { GetProcAddress(module, name.as_ptr().cast()) }?;
        // SAFETY: Each caller chooses the function type matching `name`.
        Some(unsafe { reinterpret_function_pointer(address) })
    }

    // Match upstream's system32-only load, avoiding DLL search-path injection.
    // SAFETY: Both module names are NUL-terminated.
    let mut gdi32 = unsafe { GetModuleHandleA(c"gdi32.dll".as_ptr().cast()) };
    if gdi32.is_null() {
        // SAFETY: The system32 search flag prevents application-directory
        // substitution, and the returned module remains process-loaded.
        gdi32 = unsafe {
            LoadLibraryExA(
                c"gdi32.dll".as_ptr().cast(),
                core::ptr::null_mut(),
                LOAD_LIBRARY_SEARCH_SYSTEM32,
            )
        };
    }
    if gdi32.is_null() {
        return Box::default();
    }
    let Some(enum_adapters) = symbol::<EnumAdaptersFn>(gdi32, c"D3DKMTEnumAdapters2") else {
        return Box::default();
    };
    let Some(query_adapter) = symbol::<QueryAdapterInfoFn>(gdi32, c"D3DKMTQueryAdapterInfo") else {
        return Box::default();
    };

    let value_name = match directory {
        ManifestDirectory::Driver => "VulkanDriverName",
        ManifestDirectory::ImplicitLayer => "VulkanImplicitLayers",
        ManifestDirectory::ExplicitLayer => "VulkanExplicitLayers",
    };
    let mut enumeration = EnumAdapters {
        adapter_count: 0,
        adapters: core::ptr::null_mut(),
    };
    // SAFETY: `enumeration` is writable for the documented sizing query.
    if unsafe { enum_adapters(&mut enumeration) } != STATUS_SUCCESS
        || enumeration.adapter_count == 0
    {
        return Box::default();
    }
    let Ok(mut adapters) =
        crate::allocation::try_box_uninit_slice::<Adapter>(enumeration.adapter_count as usize)
    else {
        crate::pending::mark_json_allocation_failed();
        return Box::default();
    };
    enumeration.adapters = adapters.as_mut_ptr().cast();
    // SAFETY: The adapter array has the count requested by the first query.
    if unsafe { enum_adapters(&mut enumeration) } != STATUS_SUCCESS {
        return Box::default();
    }
    let initialized = (enumeration.adapter_count as usize).min(adapters.len());
    // SAFETY: A successful query initialized the reported adapter prefix. The
    // MaybeUninit backing allocation remains live throughout this borrow.
    let initialized_adapters =
        unsafe { core::slice::from_raw_parts(adapters.as_ptr().cast::<Adapter>(), initialized) };

    let mut paths = Vec::new();
    for adapter in initialized_adapters {
        let mut registry = QueryRegistryInfo {
            query_type: QUERY_REGISTRY_ADAPTER_KEY,
            query_flags: 1,
            value_name: [0; 260],
            value_type: REG_MULTI_SZ,
            physical_adapter_index: 0,
            output_value_size: 0,
            status: QUERY_REGISTRY_STATUS_SUCCESS,
            output: 0,
        };
        for (destination, source) in registry
            .value_name
            .iter_mut()
            .zip(value_name.encode_utf16())
        {
            *destination = source;
        }
        let mut query = QueryAdapterInfo {
            handle: adapter.handle,
            kind: QUERY_TYPE_REGISTRY,
            private_data: (&raw mut registry).cast(),
            private_data_size: core::mem::size_of::<QueryRegistryInfo>() as u32,
        };
        // SAFETY: The query header has the exact D3DKMT registry ABI.
        let mut status = unsafe { query_adapter(&mut query) };
        if status != STATUS_SUCCESS {
            registry.value_type = REG_SZ;
            // SAFETY: Same valid header, retrying the alternate registry type.
            status = unsafe { query_adapter(&mut query) };
        }
        if status != STATUS_SUCCESS || registry.status != QUERY_REGISTRY_STATUS_BUFFER_OVERFLOW {
            continue;
        }

        let mut response = None;
        for _ in 0..4 {
            let byte_length = core::mem::size_of::<QueryRegistryInfo>()
                .checked_add(registry.output_value_size as usize);
            let Some(byte_length) = byte_length else {
                break;
            };
            let word_length = byte_length.div_ceil(core::mem::size_of::<u64>());
            let Ok(mut storage) = crate::allocation::try_boxed_slice_filled(word_length, 0_u64)
            else {
                crate::pending::mark_json_allocation_failed();
                return Box::default();
            };
            // SAFETY: `storage` is aligned for QueryRegistryInfo and large
            // enough for the header and requested variable-sized output.
            unsafe {
                storage
                    .as_mut_ptr()
                    .cast::<QueryRegistryInfo>()
                    .write(registry)
            };
            query.private_data = storage.as_mut_ptr().cast();
            if byte_length > u32::MAX as usize {
                break;
            }
            query.private_data_size = byte_length as u32;
            // SAFETY: `query` points at the aligned, writable response buffer.
            if unsafe { query_adapter(&mut query) } != STATUS_SUCCESS {
                break;
            }
            // SAFETY: D3DKMT initialized the response header on success.
            let header = unsafe { &*storage.as_ptr().cast::<QueryRegistryInfo>() };
            if header.status == QUERY_REGISTRY_STATUS_SUCCESS {
                response = Some(storage);
                break;
            }
            if header.status != QUERY_REGISTRY_STATUS_BUFFER_OVERFLOW {
                break;
            }
            registry = *header;
        }

        let Some(response) = response else { continue };
        // SAFETY: The successful response contains an initialized header.
        let header = unsafe { &*response.as_ptr().cast::<QueryRegistryInfo>() };
        let output_units = header.output_value_size as usize / core::mem::size_of::<u16>();
        // SAFETY: The allocation included `output_value_size` bytes following
        // the output union, which is aligned for UTF-16.
        let output = unsafe {
            core::slice::from_raw_parts(
                core::ptr::addr_of!(header.output).cast::<u16>(),
                output_units,
            )
        };
        let mut remaining = output;
        while let Some(length) = remaining.iter().position(|unit| *unit == 0) {
            if length == 0 {
                break;
            }
            let Ok(path) = paths::path_from_utf16(&remaining[..length]) else {
                crate::pending::mark_json_allocation_failed();
                return Box::default();
            };
            if crate::allocation::try_push(&mut paths, path).is_err() {
                crate::pending::mark_json_allocation_failed();
                return Box::default();
            }
            if header.value_type == REG_SZ {
                break;
            }
            remaining = &remaining[length + 1..];
        }
    }
    crate::allocation::try_into_boxed_slice(paths).unwrap_or_else(|_| {
        crate::pending::mark_json_allocation_failed();
        Box::default()
    })
}

#[cfg(windows)]
#[cold]
#[inline(never)]
pub(crate) fn adapter_luids() -> Result<Vec<AdapterLuid>, vk::VkResult> {
    const S_OK: i32 = 0;
    const DXGI_ERROR_NOT_FOUND: i32 = 0x887a_0002_u32 as i32;
    const LOAD_LIBRARY_SEARCH_SYSTEM32: u32 = 0x0000_0800;
    const DXGI_GPU_PREFERENCE_UNSPECIFIED: u32 = 0;
    const IID_IDXGI_FACTORY6: GUID = GUID {
        data1: 0xc1b6_694f,
        data2: 0xff09,
        data3: 0x44a9,
        data4: [0xb0, 0x3c, 0x77, 0x90, 0x0a, 0x0a, 0x1d, 0x17],
    };
    const IID_IDXGI_ADAPTER1: GUID = GUID {
        data1: 0x2903_8f61,
        data2: 0x3839,
        data3: 0x4626,
        data4: [0x91, 0xfd, 0x08, 0x68, 0x79, 0x01, 0x1a, 0x05],
    };

    #[repr(C)]
    struct AdapterDescription {
        description: [u16; 128],
        vendor_id: u32,
        device_id: u32,
        subsystem_id: u32,
        revision: u32,
        dedicated_video_memory: usize,
        dedicated_system_memory: usize,
        shared_system_memory: usize,
        adapter_luid: AdapterLuid,
        flags: u32,
    }

    type CreateFactory = unsafe extern "system" fn(*const GUID, *mut *mut core::ffi::c_void) -> i32;
    type EnumAdapter = unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *const GUID,
        *mut *mut core::ffi::c_void,
    ) -> i32;
    type GetDescription =
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut AdapterDescription) -> i32;
    type Release = unsafe extern "system" fn(*mut core::ffi::c_void) -> u32;

    unsafe fn method<T: Copy>(object: *mut core::ffi::c_void, index: usize) -> T {
        // SAFETY: All DXGI COM interfaces begin with a live vtable pointer.
        let vtable = unsafe { object.cast::<*const *const core::ffi::c_void>().read() };
        // SAFETY: The caller supplies an index and type from the exact interface ABI.
        let address = unsafe { vtable.add(index).read() };
        // SAFETY: The selected vtable slot has function-pointer representation.
        unsafe { reinterpret_function_pointer(address) }
    }

    // SAFETY: Module names are NUL-terminated. Loading only from system32
    // matches upstream's DLL search-path hardening.
    let mut dxgi = unsafe { GetModuleHandleA(c"dxgi.dll".as_ptr().cast()) };
    if dxgi.is_null() {
        // SAFETY: The system32-only flag excludes application-controlled paths.
        dxgi = unsafe {
            LoadLibraryExA(
                c"dxgi.dll".as_ptr().cast(),
                core::ptr::null_mut(),
                LOAD_LIBRARY_SEARCH_SYSTEM32,
            )
        };
    }
    if dxgi.is_null() {
        return Ok(Vec::new());
    }
    // SAFETY: `dxgi` is live and the symbol name is NUL-terminated.
    let Some(create_factory) =
        (unsafe { GetProcAddress(dxgi, c"CreateDXGIFactory1".as_ptr().cast()) })
    else {
        return Ok(Vec::new());
    };
    debug_assert_eq!(
        core::mem::size_of::<CreateFactory>(),
        core::mem::size_of_val(&create_factory)
    );
    // SAFETY: GetProcAddress returned CreateDXGIFactory1's documented ABI.
    let create_factory: CreateFactory = unsafe { core::mem::transmute(create_factory) };

    let mut factory = core::ptr::null_mut();
    // SAFETY: The IID is valid and the output interface pointer is writable.
    if unsafe { create_factory(&IID_IDXGI_FACTORY6, &mut factory) } != S_OK || factory.is_null() {
        return Ok(Vec::new());
    }
    // IDXGIFactory6 inherits 29 methods before EnumAdapterByGpuPreference;
    // IUnknown::Release remains slot 2.
    let enumerate: EnumAdapter = unsafe { method(factory, 29) };
    let release_factory: Release = unsafe { method(factory, 2) };
    let mut luids = Vec::new();
    for index in 0.. {
        let mut adapter = core::ptr::null_mut();
        // SAFETY: The factory and IID are live and `adapter` is writable.
        let result = unsafe {
            enumerate(
                factory,
                index,
                DXGI_GPU_PREFERENCE_UNSPECIFIED,
                &IID_IDXGI_ADAPTER1,
                &mut adapter,
            )
        };
        if result == DXGI_ERROR_NOT_FOUND {
            break;
        }
        if result != S_OK || adapter.is_null() {
            break;
        }
        // IDXGIAdapter1::GetDesc1 is slot 10; Release is slot 2.
        let get_description: GetDescription = unsafe { method(adapter, 10) };
        let release_adapter: Release = unsafe { method(adapter, 2) };
        let mut description = core::mem::MaybeUninit::<AdapterDescription>::uninit();
        // SAFETY: `description` is writable for the complete DXGI structure.
        let stored = if unsafe { get_description(adapter, description.as_mut_ptr()) } == S_OK {
            // SAFETY: GetDesc1 initialized the complete structure on success.
            crate::allocation::try_push(
                &mut luids,
                unsafe { description.assume_init() }.adapter_luid,
            )
        } else {
            Ok(())
        };
        // SAFETY: This function owns the reference returned by enumeration.
        unsafe { release_adapter(adapter) };
        if let Err(result) = stored {
            // SAFETY: The adapter was released above; this function still
            // owns the factory reference on the allocation-failure path.
            unsafe { release_factory(factory) };
            return Err(result);
        }
    }
    // SAFETY: This function owns the factory reference.
    unsafe { release_factory(factory) };
    Ok(luids)
}

#[cfg(windows)]
#[cold]
#[inline(never)]
fn app_package_manifest_path() -> Option<PathBuf> {
    type GetPackagesByPackageFamily =
        unsafe extern "system" fn(*const u16, *mut u32, *mut *mut u16, *mut u32, *mut u16) -> u32;
    type GetPackagePathByFullName =
        unsafe extern "system" fn(*const u16, *mut u32, *mut u16) -> u32;

    // These APIs were introduced after Windows 7, so resolve them lazily just
    // as upstream does instead of adding hard loader imports.
    // SAFETY: The module name is NUL-terminated.
    let kernel = unsafe { GetModuleHandleA(c"kernel32.dll".as_ptr().cast()) };
    if kernel.is_null() {
        return None;
    }
    // SAFETY: The symbol names are NUL-terminated and `kernel` is live.
    let get_packages =
        unsafe { GetProcAddress(kernel, c"GetPackagesByPackageFamily".as_ptr().cast()) }?;
    // SAFETY: Same module and symbol-name contract as above.
    let get_path = unsafe { GetProcAddress(kernel, c"GetPackagePathByFullName".as_ptr().cast()) }?;
    debug_assert_eq!(
        core::mem::size_of::<GetPackagesByPackageFamily>(),
        core::mem::size_of_val(&get_packages)
    );
    debug_assert_eq!(
        core::mem::size_of::<GetPackagePathByFullName>(),
        core::mem::size_of_val(&get_path)
    );
    // SAFETY: GetProcAddress returned these exact Win32 entry points.
    let get_packages: GetPackagesByPackageFamily = unsafe { core::mem::transmute(get_packages) };
    // SAFETY: GetProcAddress returned this exact Win32 entry point.
    let get_path: GetPackagePathByFullName = unsafe { core::mem::transmute(get_path) };

    let family = w!("Microsoft.D3DMappingLayers_8wekyb3d8bbwe");
    let (mut count, mut buffer_length) = (0, 0);
    // SAFETY: This is the documented sizing query; output buffers are null.
    if unsafe {
        get_packages(
            family,
            &mut count,
            core::ptr::null_mut(),
            &mut buffer_length,
            core::ptr::null_mut(),
        )
    } != ERROR_INSUFFICIENT_BUFFER
        || count == 0
        || buffer_length == 0
    {
        return None;
    }

    let (Ok(mut names), Ok(mut packages)) = (
        crate::allocation::try_box_uninit_slice::<u16>(buffer_length as usize),
        crate::allocation::try_box_uninit_slice::<*mut u16>(count as usize),
    ) else {
        crate::pending::mark_json_allocation_failed();
        return None;
    };
    // SAFETY: Both buffers have exactly the capacities returned by the sizing
    // query, and the API initializes them on success.
    if unsafe {
        get_packages(
            family,
            &mut count,
            packages.as_mut_ptr().cast(),
            &mut buffer_length,
            names.as_mut_ptr().cast(),
        )
    } != ERROR_SUCCESS
        || count == 0
        || count as usize > packages.len()
        || buffer_length as usize > names.len()
    {
        return None;
    }
    // SAFETY: The successful second query reports at least one initialized
    // pointer. Its count may shrink since sizing, so do not assume the entire
    // allocation was initialized.
    let package = unsafe { packages.first()?.assume_init() };

    let mut path_length = 0;
    // SAFETY: This is the documented path sizing query.
    if unsafe { get_path(package, &mut path_length, core::ptr::null_mut()) }
        != ERROR_INSUFFICIENT_BUFFER
        || path_length == 0
        || path_length > 260
    {
        return None;
    }
    // Upstream zero-initializes MAX_PATH before the call. The API's returned
    // length includes room for NUL, but providers (including the parity shim)
    // need not overwrite that final unit themselves.
    let mut path = [0_u16; 260];
    // SAFETY: `path` has the capacity returned by the sizing query.
    if unsafe { get_path(package, &mut path_length, path.as_mut_ptr()) } != ERROR_SUCCESS {
        return None;
    }
    paths::package_path_from_utf16(&path).unwrap_or_else(|_| {
        crate::pending::mark_json_allocation_failed();
        None
    })
}

#[cfg(windows)]
pub(crate) fn settings_files() -> Box<[PathBuf]> {
    let location = c"SOFTWARE\\Khronos\\Vulkan\\LoaderSettings";
    let user_paths = if has_elevated_privileges() {
        Box::default()
    } else {
        registry_values(location, true)
    };
    crate::allocation::try_collect(
        user_paths
            .into_iter()
            .chain(registry_values(location, false)),
    )
    .unwrap_or_else(|_| {
        crate::pending::mark_json_allocation_failed();
        Box::default()
    })
}
