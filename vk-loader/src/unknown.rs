//! Signature-independent dispatch for commands absent from the loader's registry.
//!
//! Vulkan-Loader uses architecture-specific, frameless tail-call stubs here.
//! A Rust function cannot express the same contract because the command's
//! integer, SIMD, and stack arguments are intentionally unknown.

use alloc::ffi::CString;
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm",
    target_arch = "x86",
    target_arch = "x86_64"
))]
use core::arch::global_asm;
use core::{
    ffi::{CStr, c_void},
    sync::atomic::{AtomicPtr, Ordering},
};
use vk::PFN_vkVoidFunction;

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm",
    target_arch = "x86",
    target_arch = "x86_64"
))]
use crate::instance::{LoaderPhysicalDevice, LoaderPhysicalDeviceTrampoline};
use crate::{allocation::try_box_uninit_slice, instance::LoaderInstance};

pub(crate) const MAX_UNKNOWN_COMMANDS: usize = 250;
#[cfg(target_arch = "x86_64")]
const TRAMPOLINE_STRIDE: usize = 24;
#[cfg(target_arch = "x86_64")]
const TERMINATOR_STRIDE: usize = 40;
#[cfg(target_arch = "x86_64")]
const DEVICE_TRAMPOLINE_STRIDE: usize = 16;
#[cfg(target_arch = "aarch64")]
const TRAMPOLINE_STRIDE: usize = 20;
#[cfg(target_arch = "aarch64")]
const TERMINATOR_STRIDE: usize = 32;
#[cfg(target_arch = "aarch64")]
const DEVICE_TRAMPOLINE_STRIDE: usize = 20;
#[cfg(target_arch = "arm")]
const TRAMPOLINE_STRIDE: usize = 16;
#[cfg(target_arch = "arm")]
const TERMINATOR_STRIDE: usize = 32;
#[cfg(target_arch = "arm")]
const DEVICE_TRAMPOLINE_STRIDE: usize = 8;
#[cfg(target_arch = "x86")]
const TRAMPOLINE_STRIDE: usize = 28;
#[cfg(target_arch = "x86")]
const TERMINATOR_STRIDE: usize = 48;
#[cfg(target_arch = "x86")]
const DEVICE_TRAMPOLINE_STRIDE: usize = 20;

pub(crate) struct UnknownDispatchTable {
    entries: Box<[AtomicPtr<c_void>]>,
}

impl UnknownDispatchTable {
    pub(crate) fn try_new() -> Result<Self, vk::VkResult> {
        let mut entries = try_box_uninit_slice::<AtomicPtr<c_void>>(MAX_UNKNOWN_COMMANDS)?;
        for entry in &mut entries {
            entry.write(AtomicPtr::new(core::ptr::null_mut()));
        }
        // SAFETY: Every element was initialized exactly once above.
        Ok(Self {
            entries: unsafe { entries.assume_init() },
        })
    }

    pub(crate) fn as_ptr(&self) -> *const AtomicPtr<c_void> {
        self.entries.as_ptr()
    }

    fn store(&self, index: usize, function: PFN_vkVoidFunction) {
        self.entries[index].store(function_address(function), Ordering::Release);
    }
}

pub(crate) struct UnknownPhysicalDeviceState {
    names: Vec<CString>,
    dispatch: UnknownDispatchTable,
}

pub(crate) struct UnknownDeviceState {
    names: Vec<CString>,
}

impl UnknownDeviceState {
    pub(crate) const fn new() -> Self {
        Self { names: Vec::new() }
    }

    fn index_of(&self, name: &CStr) -> Option<usize> {
        self.names
            .iter()
            .position(|candidate| candidate.as_c_str() == name)
    }

    fn get_or_insert(&mut self, name: &CStr) -> Option<usize> {
        if let Some(index) = self.index_of(name) {
            return Some(index);
        }
        if self.names.len() == MAX_UNKNOWN_COMMANDS || self.names.try_reserve(1).is_err() {
            return None;
        }
        let bytes = name.to_bytes_with_nul();
        let mut owned = Vec::new();
        if owned.try_reserve_exact(bytes.len()).is_err() {
            return None;
        }
        owned.extend_from_slice(bytes);
        // SAFETY: This is an exact copy of a valid C string.
        self.names
            .push(unsafe { CString::from_vec_with_nul_unchecked(owned) });
        Some(self.names.len() - 1)
    }
}

impl UnknownPhysicalDeviceState {
    pub(crate) fn try_new() -> Result<Self, vk::VkResult> {
        Ok(Self {
            names: Vec::new(),
            dispatch: UnknownDispatchTable::try_new()?,
        })
    }

    pub(crate) const fn dispatch(&self) -> &UnknownDispatchTable {
        &self.dispatch
    }

    fn index_of(&self, name: &CStr) -> Option<usize> {
        self.names
            .iter()
            .position(|candidate| candidate.as_c_str() == name)
    }

    fn get_or_insert(&mut self, name: &CStr) -> Option<usize> {
        if let Some(index) = self.index_of(name) {
            return Some(index);
        }
        if self.names.len() == MAX_UNKNOWN_COMMANDS || self.names.try_reserve(1).is_err() {
            return None;
        }
        let bytes = name.to_bytes_with_nul();
        let mut owned = Vec::new();
        if owned.try_reserve_exact(bytes.len()).is_err() {
            return None;
        }
        owned.extend_from_slice(bytes);
        // SAFETY: `name` is a C string, so this copied buffer has one trailing
        // NUL and no interior NUL bytes.
        self.names
            .push(unsafe { CString::from_vec_with_nul_unchecked(owned) });
        Some(self.names.len() - 1)
    }
}

fn function_address(function: PFN_vkVoidFunction) -> *mut c_void {
    function.map_or(core::ptr::null_mut(), |function| {
        function as *const () as *mut c_void
    })
}

fn query_icd(instance: &LoaderInstance, index: usize, name: &CStr) -> PFN_vkVoidFunction {
    let icd = &instance.icds[index];
    if !icd.is_active() || icd.icd.interface_version < 3 {
        return None;
    }
    let get = icd.icd.get_physical_device_proc_addr?;
    // SAFETY: The resolver and instance handle originate from this live ICD.
    unsafe { get(icd.handle, name.as_ptr()) }
}

fn top_layer_physical_device_proc_addr(
    instance: &LoaderInstance,
    name: &CStr,
) -> PFN_vkVoidFunction {
    let layer = instance
        .layers
        .iter()
        .find_map(|layer| layer.get_physical_device_proc_addr)?;
    // The first layer with GPDPA support owns traversal of the remaining chain.
    // SAFETY: The layer and top-of-chain instance remain live together.
    unsafe { layer(instance.chain_handle(), name.as_ptr()) }
}

fn first_layer_physical_device_proc_addr<'a>(
    instance: &'a LoaderInstance,
    name: &CStr,
) -> Option<(&'a crate::layer::LoadedLayer, unsafe extern "system" fn())> {
    instance.layers.iter().find_map(|layer| {
        let get = layer.get_physical_device_proc_addr?;
        // SAFETY: The layer and top-of-chain instance remain live together.
        unsafe { get(instance.chain_handle(), name.as_ptr()) }.map(|function| (layer, function))
    })
}

fn emit_unknown_debug(message: core::fmt::Arguments<'_>) {
    crate::platform::write_loader_log(crate::platform::LogFilter::Debug, message);
}

/// Resolves a physical-device command unknown to the compiled registry.
///
/// `trampoline` selects the application-facing wrapper or the bottom-of-layer
/// terminator, matching Vulkan-Loader's two GPDPA paths.
pub(crate) fn physical_device_proc_addr(
    instance: &LoaderInstance,
    name: &CStr,
    trampoline: bool,
) -> PFN_vkVoidFunction {
    let mut supported_by_icd = false;
    for index in 0..instance.icds.len() {
        if query_icd(instance, index, name).is_some() {
            supported_by_icd = true;
            break;
        }
    }
    if !supported_by_icd
        && (!trampoline || top_layer_physical_device_proc_addr(instance, name).is_none())
    {
        return None;
    }

    let index = {
        let mut state = instance.unknown_physical_devices.lock();
        if let Some(index) = state.index_of(name) {
            index
        } else {
            let index = state.get_or_insert(name)?;
            emit_unknown_debug(format_args!(
                "loader_phys_dev_ext_gpa: Adding unknown physical function {} to internal store at index {index}",
                name.to_string_lossy()
            ));
            index
        }
    };

    let mut terminator_needed = false;
    for icd_index in 0..instance.icds.len() {
        let function = query_icd(instance, icd_index, name);
        instance.icds[icd_index]
            .unknown_physical_device_dispatch
            .store(index, function);
        if function.is_some() {
            let path = instance.icds[icd_index]
                .icd
                .library_path()
                .map_or_else(|| "<direct_driver>".into(), |path| path.to_string_lossy());
            emit_unknown_debug(format_args!(
                "loader_phys_dev_ext_gpa: Driver {path} returned ptr {:p} for {}",
                function_address(terminator_function(index)),
                name.to_string_lossy()
            ));
        }
        terminator_needed |= function.is_some();
    }

    let state = instance.unknown_physical_devices.lock();
    if terminator_needed {
        state.dispatch.store(index, terminator_function(index));
    }
    drop(state);

    if trampoline {
        if let Some((layer, function)) = first_layer_physical_device_proc_addr(instance, name) {
            instance
                .unknown_physical_devices
                .lock()
                .dispatch
                .store(index, Some(function));
            emit_unknown_debug(format_args!(
                "loader_phys_dev_ext_gpa: Layer {} returned ptr {:p} for {}",
                layer.name.to_string_lossy(),
                function_address(Some(function)),
                name.to_string_lossy()
            ));
        }
        trampoline_function(index)
    } else {
        terminator_function(index)
    }
}

fn icd_supports_device_command(instance: &LoaderInstance, name: &CStr) -> bool {
    instance.active_icds().any(|(_, icd)| {
        // SAFETY: The instance handle and resolver originate from this live ICD.
        unsafe { (icd.icd.get_instance_proc_addr)(icd.handle, name.as_ptr()) }.is_some()
    })
}

fn layers_support_device_command(instance: &LoaderInstance, name: &CStr) -> bool {
    if instance.layers.iter().any(|layer| {
        layer.device_extensions.iter().any(|extension| {
            extension
                .entrypoints
                .iter()
                .any(|entrypoint| entrypoint.as_c_str() == name)
        })
    }) {
        return true;
    }
    let Some(top) = instance.layers.first() else {
        return false;
    };
    // SAFETY: The top layer and chain instance are retained by `instance`.
    unsafe { (top.get_instance_proc_addr)(instance.chain_handle(), name.as_ptr()) }.is_some()
}

pub(crate) fn initialize_device_dispatch(device: &crate::device::LoaderDevice) {
    let instance = device.instance();
    let count = instance.unknown_devices.lock().names.len();
    for index in 0..count {
        let name = {
            let state = instance.unknown_devices.lock();
            state.names[index].as_ptr()
        };
        // SAFETY: Names are only appended during the instance lifetime, and a
        // CString's allocation does not move when its owning Vec grows.
        let name = unsafe { CStr::from_ptr(name) };
        let function = device.resolve_chain(name);
        device.store_unknown_dispatch(index, function);
    }
}

pub(crate) fn device_proc_addr(
    instance: &LoaderInstance,
    name: &CStr,
    trampoline: bool,
) -> PFN_vkVoidFunction {
    {
        let state = instance.unknown_devices.lock();
        if let Some(index) = state.index_of(name) {
            return device_trampoline_function(index);
        }
    }

    let supported_by_icd = icd_supports_device_command(instance, name);
    if !supported_by_icd && (!trampoline || !layers_support_device_command(instance, name)) {
        return None;
    }

    let index = instance.unknown_devices.lock().get_or_insert(name)?;
    // Existing logical devices must gain the new slot immediately. A device
    // created later initializes every recorded slot in `LoaderDevice::new`.
    crate::device::initialize_unknown_dispatches(instance, index, name);
    device_trampoline_function(index)
}

#[cfg(all(target_arch = "x86_64", not(target_os = "windows")))]
global_asm!(
    r#"
    .section .text.vk_loader_unknown,"ax",@progbits
    .p2align 4
    .global vk_loader_unknown_phys_trampoline_base
    .hidden vk_loader_unknown_phys_trampoline_base
vk_loader_unknown_phys_trampoline_base:
    .set slot, 0
    .rept 250
0:
    endbr64
    mov rax, [rdi + {trampoline_table}]
    mov rdi, [rdi + {trampoline_chain}]
    jmp [rax + 8 * slot]
    .fill {trampoline_stride} - (. - 0b), 1, 0x90
    .set slot, slot + 1
    .endr

    .p2align 4
    .global vk_loader_unknown_phys_terminator_base
    .hidden vk_loader_unknown_phys_terminator_base
vk_loader_unknown_phys_terminator_base:
    .set slot, 0
    .rept 250
0:
    endbr64
    mov rax, [rdi + {terminator_table}]
    mov r11, [rax + 8 * slot]
    test r11, r11
    jz 1f
    mov rdi, [rdi + {terminator_native}]
    jmp r11
1:
    mov esi, slot
    jmp {terminator_error}
    .fill {terminator_stride} - (. - 0b), 1, 0x90
    .set slot, slot + 1
    .endr

    .p2align 4
    .global vk_loader_unknown_device_trampoline_base
    .hidden vk_loader_unknown_device_trampoline_base
vk_loader_unknown_device_trampoline_base:
    .set slot, 0
    .rept 250
0:
    endbr64
    mov rax, [rdi]
    jmp [rax + {device_dispatch_offset} + 8 * slot]
    .fill {device_trampoline_stride} - (. - 0b), 1, 0x90
    .set slot, slot + 1
    .endr
    "#,
    trampoline_table = const core::mem::offset_of!(LoaderPhysicalDeviceTrampoline, unknown_dispatch),
    trampoline_chain = const core::mem::offset_of!(LoaderPhysicalDeviceTrampoline, chain),
    trampoline_stride = const TRAMPOLINE_STRIDE,
    terminator_table = const core::mem::offset_of!(LoaderPhysicalDevice, unknown_dispatch),
    terminator_native = const core::mem::offset_of!(LoaderPhysicalDevice, native),
    terminator_stride = const TERMINATOR_STRIDE,
    terminator_error = sym vk_loader_unknown_phys_terminator_error,
    device_dispatch_offset = const crate::device::UNKNOWN_DEVICE_DISPATCH_OFFSET,
    device_trampoline_stride = const DEVICE_TRAMPOLINE_STRIDE,
);

#[cfg(all(target_arch = "x86_64", target_os = "windows"))]
global_asm!(
    r#"
    .section .text$vk_loader_unknown,"xr"
    .p2align 4
    .globl vk_loader_unknown_phys_trampoline_base
vk_loader_unknown_phys_trampoline_base:
    .set slot, 0
    .rept 250
0:
    endbr64
    mov rax, [rcx + {trampoline_table}]
    mov rcx, [rcx + {trampoline_chain}]
    jmp [rax + 8 * slot]
    .fill {trampoline_stride} - (. - 0b), 1, 0x90
    .set slot, slot + 1
    .endr

    .p2align 4
    .globl vk_loader_unknown_phys_terminator_base
vk_loader_unknown_phys_terminator_base:
    .set slot, 0
    .rept 250
0:
    endbr64
    mov rax, [rcx + {terminator_table}]
    mov r11, [rax + 8 * slot]
    test r11, r11
    jz 1f
    mov rcx, [rcx + {terminator_native}]
    jmp r11
1:
    mov edx, slot
    jmp {terminator_error}
    .fill {terminator_stride} - (. - 0b), 1, 0x90
    .set slot, slot + 1
    .endr

    .p2align 4
    .globl vk_loader_unknown_device_trampoline_base
vk_loader_unknown_device_trampoline_base:
    .set slot, 0
    .rept 250
0:
    endbr64
    mov rax, [rcx]
    jmp [rax + {device_dispatch_offset} + 8 * slot]
    .fill {device_trampoline_stride} - (. - 0b), 1, 0x90
    .set slot, slot + 1
    .endr
    "#,
    trampoline_table = const core::mem::offset_of!(LoaderPhysicalDeviceTrampoline, unknown_dispatch),
    trampoline_chain = const core::mem::offset_of!(LoaderPhysicalDeviceTrampoline, chain),
    trampoline_stride = const TRAMPOLINE_STRIDE,
    terminator_table = const core::mem::offset_of!(LoaderPhysicalDevice, unknown_dispatch),
    terminator_native = const core::mem::offset_of!(LoaderPhysicalDevice, native),
    terminator_stride = const TERMINATOR_STRIDE,
    terminator_error = sym vk_loader_unknown_phys_terminator_error,
    device_dispatch_offset = const crate::device::UNKNOWN_DEVICE_DISPATCH_OFFSET,
    device_trampoline_stride = const DEVICE_TRAMPOLINE_STRIDE,
);

#[cfg(target_arch = "x86")]
macro_rules! x86_unknown_asm {
    ($visibility:literal) => {
        global_asm!(
            concat!(
                r#"
                .text
                .p2align 4
                .globl {trampoline_base}
                .globl {terminator_base}
                .globl {device_base}
                "#,
                $visibility,
                r#"
{trampoline_base}:
                .set slot, 0
                .rept 250
0:
                .byte 0xf3, 0x0f, 0x1e, 0xfb
                mov eax, [esp + 4]
                mov ecx, [eax + {trampoline_chain}]
                mov [esp + 4], ecx
                mov eax, [eax + {trampoline_table}]
                mov edx, (4 * slot)
                jmp [eax + edx]
                .fill {trampoline_stride} - (. - 0b), 1, 0x90
                .set slot, slot + 1
                .endr

                .p2align 4
{terminator_base}:
                .set slot, 0
                .rept 250
0:
                .byte 0xf3, 0x0f, 0x1e, 0xfb
                mov ecx, [esp + 4]
                mov eax, [ecx + {terminator_table}]
                mov edx, (4 * slot)
                mov edx, [eax + edx]
                test edx, edx
                jz 1f
                mov ecx, [ecx + {terminator_native}]
                mov [esp + 4], ecx
                jmp edx
1:
                mov dword ptr [esp + 8], slot
                jmp {terminator_error}
                .fill {terminator_stride} - (. - 0b), 1, 0x90
                .set slot, slot + 1
                .endr

                .p2align 4
{device_base}:
                .set slot, 0
                .rept 250
0:
                .byte 0xf3, 0x0f, 0x1e, 0xfb
                mov eax, [esp + 4]
                mov eax, [eax]
                mov edx, ({device_dispatch_offset} + 4 * slot)
                jmp [eax + edx]
                .fill {device_trampoline_stride} - (. - 0b), 1, 0x90
                .set slot, slot + 1
                .endr
                "#
            ),
            trampoline_base = sym vk_loader_unknown_phys_trampoline_base,
            terminator_base = sym vk_loader_unknown_phys_terminator_base,
            device_base = sym vk_loader_unknown_device_trampoline_base,
            trampoline_table = const core::mem::offset_of!(LoaderPhysicalDeviceTrampoline, unknown_dispatch),
            trampoline_chain = const core::mem::offset_of!(LoaderPhysicalDeviceTrampoline, chain),
            trampoline_stride = const TRAMPOLINE_STRIDE,
            terminator_table = const core::mem::offset_of!(LoaderPhysicalDevice, unknown_dispatch),
            terminator_native = const core::mem::offset_of!(LoaderPhysicalDevice, native),
            terminator_stride = const TERMINATOR_STRIDE,
            terminator_error = sym vk_loader_unknown_phys_terminator_error,
            device_dispatch_offset = const crate::device::UNKNOWN_DEVICE_DISPATCH_OFFSET,
            device_trampoline_stride = const DEVICE_TRAMPOLINE_STRIDE,
        );
    };
}

#[cfg(all(target_arch = "x86", not(windows)))]
x86_unknown_asm!(
    r#"
                .hidden {trampoline_base}
                .hidden {terminator_base}
                .hidden {device_base}
                "#
);

#[cfg(all(target_arch = "x86", windows))]
x86_unknown_asm!("");

#[cfg(target_arch = "arm")]
macro_rules! arm_unknown_asm {
    ($visibility:literal) => {
        global_asm!(
            concat!(
                r#"
                .syntax unified
                .arm
                .text
                .p2align 2
                .globl {trampoline_base}
                .globl {terminator_base}
                .globl {device_base}
                "#,
                $visibility,
                r#"
{trampoline_base}:
                .set slot, 0
                .rept 250
0:
                ldr r12, [r0, #{trampoline_table}]
                ldr r0, [r0, #{trampoline_chain}]
                ldr r12, [r12, #(4 * slot)]
                bx r12
                .fill {trampoline_stride} - (. - 0b), 1, 0
                .set slot, slot + 1
                .endr

                .p2align 2
{terminator_base}:
                .set slot, 0
                .rept 250
0:
                ldr r12, [r0, #{terminator_table}]
                ldr r12, [r12, #(4 * slot)]
                cmp r12, #0
                beq 1f
                ldr r0, [r0, #{terminator_native}]
                bx r12
1:
                mov r1, slot
                b {terminator_error}
                .fill {terminator_stride} - (. - 0b), 1, 0
                .set slot, slot + 1
                .endr

                .p2align 2
{device_base}:
                .set slot, 0
                .rept 250
0:
                ldr r12, [r0]
                ldr pc, [r12, #({device_dispatch_offset} + 4 * slot)]
                .fill {device_trampoline_stride} - (. - 0b), 1, 0
                .set slot, slot + 1
                .endr
                "#
            ),
            trampoline_base = sym vk_loader_unknown_phys_trampoline_base,
            terminator_base = sym vk_loader_unknown_phys_terminator_base,
            device_base = sym vk_loader_unknown_device_trampoline_base,
            trampoline_table = const core::mem::offset_of!(LoaderPhysicalDeviceTrampoline, unknown_dispatch),
            trampoline_chain = const core::mem::offset_of!(LoaderPhysicalDeviceTrampoline, chain),
            trampoline_stride = const TRAMPOLINE_STRIDE,
            terminator_table = const core::mem::offset_of!(LoaderPhysicalDevice, unknown_dispatch),
            terminator_native = const core::mem::offset_of!(LoaderPhysicalDevice, native),
            terminator_stride = const TERMINATOR_STRIDE,
            terminator_error = sym vk_loader_unknown_phys_terminator_error,
            device_dispatch_offset = const crate::device::UNKNOWN_DEVICE_DISPATCH_OFFSET,
            device_trampoline_stride = const DEVICE_TRAMPOLINE_STRIDE,
        );
    };
}

#[cfg(all(target_arch = "arm", target_vendor = "apple"))]
arm_unknown_asm!(
    r#"
                .private_extern {trampoline_base}
                .private_extern {terminator_base}
                .private_extern {device_base}
                "#
);

#[cfg(all(target_arch = "arm", not(target_vendor = "apple")))]
arm_unknown_asm!(
    r#"
                .hidden {trampoline_base}
                .hidden {terminator_base}
                .hidden {device_base}
                "#
);

#[cfg(target_arch = "aarch64")]
macro_rules! aarch64_unknown_asm {
    ($visibility:literal) => {
        global_asm!(
            concat!(
                r#"
                .text
                .p2align 2
                .globl {trampoline_base}
                .globl {terminator_base}
                .globl {device_base}
                "#,
                $visibility,
                r#"
{trampoline_base}:
                .set slot, 0
                .rept 250
0:
                hint #34
                ldr x9, [x0, #{trampoline_table}]
                ldr x0, [x0, #{trampoline_chain}]
                ldr x16, [x9, #(8 * slot)]
                br x16
                .fill {trampoline_stride} - (. - 0b), 1, 0
                .set slot, slot + 1
                .endr

                .p2align 2
{terminator_base}:
                .set slot, 0
                .rept 250
0:
                hint #34
                ldr x9, [x0, #{terminator_table}]
                ldr x16, [x9, #(8 * slot)]
                cbz x16, 1f
                ldr x0, [x0, #{terminator_native}]
                br x16
1:
                mov x1, slot
                b {terminator_error}
                .fill {terminator_stride} - (. - 0b), 1, 0
                .set slot, slot + 1
                .endr

                .p2align 2
{device_base}:
                .set slot, 0
                .rept 250
0:
                hint #34
                ldr x9, [x0]
                mov x10, ({device_dispatch_offset} + 8 * slot)
                ldr x16, [x9, x10]
                br x16
                .fill {device_trampoline_stride} - (. - 0b), 1, 0
                .set slot, slot + 1
                .endr
                "#
            ),
            trampoline_base = sym vk_loader_unknown_phys_trampoline_base,
            terminator_base = sym vk_loader_unknown_phys_terminator_base,
            device_base = sym vk_loader_unknown_device_trampoline_base,
            trampoline_table = const core::mem::offset_of!(LoaderPhysicalDeviceTrampoline, unknown_dispatch),
            trampoline_chain = const core::mem::offset_of!(LoaderPhysicalDeviceTrampoline, chain),
            trampoline_stride = const TRAMPOLINE_STRIDE,
            terminator_table = const core::mem::offset_of!(LoaderPhysicalDevice, unknown_dispatch),
            terminator_native = const core::mem::offset_of!(LoaderPhysicalDevice, native),
            terminator_stride = const TERMINATOR_STRIDE,
            terminator_error = sym vk_loader_unknown_phys_terminator_error,
            device_dispatch_offset = const crate::device::UNKNOWN_DEVICE_DISPATCH_OFFSET,
            device_trampoline_stride = const DEVICE_TRAMPOLINE_STRIDE,
        );
    };
}

#[cfg(all(target_arch = "aarch64", target_vendor = "apple"))]
aarch64_unknown_asm!(
    r#"
                .private_extern {trampoline_base}
                .private_extern {terminator_base}
                .private_extern {device_base}
                "#
);

#[cfg(all(target_arch = "aarch64", not(target_vendor = "apple")))]
aarch64_unknown_asm!(
    r#"
                .hidden {trampoline_base}
                .hidden {terminator_base}
                .hidden {device_base}
                "#
);

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm",
    target_arch = "x86",
    target_arch = "x86_64"
))]
unsafe extern "C" {
    static vk_loader_unknown_phys_trampoline_base: u8;
    static vk_loader_unknown_phys_terminator_base: u8;
    static vk_loader_unknown_device_trampoline_base: u8;
}

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm",
    target_arch = "x86",
    target_arch = "x86_64"
))]
#[allow(clippy::unnecessary_wraps)] // `PFN_vkVoidFunction` is an optional function pointer.
fn trampoline_function(index: usize) -> PFN_vkVoidFunction {
    debug_assert!(index < MAX_UNKNOWN_COMMANDS);
    // SAFETY: The assembly emits exactly one fixed-stride stub per slot.
    let address = unsafe {
        core::ptr::addr_of!(vk_loader_unknown_phys_trampoline_base).add(index * TRAMPOLINE_STRIDE)
    };
    // SAFETY: The selected symbol is a Vulkan-callable frameless trampoline.
    Some(unsafe { core::mem::transmute::<*const u8, unsafe extern "system" fn()>(address) })
}

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm",
    target_arch = "x86",
    target_arch = "x86_64"
))]
#[allow(clippy::unnecessary_wraps)] // `PFN_vkVoidFunction` is an optional function pointer.
fn terminator_function(index: usize) -> PFN_vkVoidFunction {
    debug_assert!(index < MAX_UNKNOWN_COMMANDS);
    // SAFETY: The assembly emits exactly one fixed-stride stub per slot.
    let address = unsafe {
        core::ptr::addr_of!(vk_loader_unknown_phys_terminator_base).add(index * TERMINATOR_STRIDE)
    };
    // SAFETY: The selected symbol is a Vulkan-callable frameless terminator.
    Some(unsafe { core::mem::transmute::<*const u8, unsafe extern "system" fn()>(address) })
}

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm",
    target_arch = "x86",
    target_arch = "x86_64"
))]
#[allow(clippy::unnecessary_wraps)] // `PFN_vkVoidFunction` is an optional function pointer.
fn device_trampoline_function(index: usize) -> PFN_vkVoidFunction {
    debug_assert!(index < MAX_UNKNOWN_COMMANDS);
    // SAFETY: The assembly emits exactly one fixed-stride stub per slot.
    let address = unsafe {
        core::ptr::addr_of!(vk_loader_unknown_device_trampoline_base)
            .add(index * DEVICE_TRAMPOLINE_STRIDE)
    };
    // SAFETY: The selected symbol is a Vulkan-callable frameless trampoline.
    Some(unsafe { core::mem::transmute::<*const u8, unsafe extern "system" fn()>(address) })
}

#[cfg(not(any(
    target_arch = "aarch64",
    target_arch = "arm",
    target_arch = "x86",
    target_arch = "x86_64"
)))]
fn trampoline_function(_index: usize) -> PFN_vkVoidFunction {
    None
}

#[cfg(not(any(
    target_arch = "aarch64",
    target_arch = "arm",
    target_arch = "x86",
    target_arch = "x86_64"
)))]
fn terminator_function(_index: usize) -> PFN_vkVoidFunction {
    None
}

#[cfg(not(any(
    target_arch = "aarch64",
    target_arch = "arm",
    target_arch = "x86",
    target_arch = "x86_64"
)))]
fn device_trampoline_function(_index: usize) -> PFN_vkVoidFunction {
    None
}

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm",
    target_arch = "x86",
    target_arch = "x86_64"
))]
unsafe extern "C" fn vk_loader_unknown_phys_terminator_error(
    physical_device: *const LoaderPhysicalDevice,
    index: usize,
) -> ! {
    let name = if physical_device.is_null() {
        None
    } else {
        // SAFETY: Only a terminator stub branches here, retaining its live wrapper.
        let physical_device = unsafe { &*physical_device };
        physical_device
            .instance()
            .unknown_physical_devices
            .lock()
            .names
            .get(index)
            .map(|name| name.to_string_lossy().into_owned())
    };
    let name = name.as_deref().unwrap_or("<unknown>");
    crate::platform::write_stderr(&format!(
        "Function {name} not supported for this physical device\n"
    ));
    // SAFETY: This is the required terminal path for invoking an unsupported
    // unknown physical-device command; no Rust destructors may be relied on.
    unsafe { libc::abort() }
}

#[cfg(all(
    test,
    any(
        target_arch = "aarch64",
        target_arch = "arm",
        target_arch = "x86",
        target_arch = "x86_64"
    )
))]
mod tests {
    #![allow(clippy::many_single_char_names)]

    use super::*;
    use crate::{device::UNKNOWN_DEVICE_DISPATCH_OFFSET, instance::LoaderPhysicalDeviceTrampoline};
    use vk::{VkDevice, VkPhysicalDevice};

    const SLOT: usize = 137;

    type PhysicalCommand =
        unsafe extern "system" fn(VkPhysicalDevice, u32, u64, f32, f64, u32, u32, u64, f32) -> u64;
    type DeviceCommand =
        unsafe extern "system" fn(VkDevice, u32, u64, f32, f64, u32, u32, u64, f32) -> u64;

    struct Arguments {
        a: u32,
        b: u64,
        c: f32,
        d: f64,
        e: u32,
        f: u32,
        g: u64,
        h: f32,
    }

    #[repr(C)]
    struct FakeDevice {
        dispatch: *const usize,
    }

    fn checksum(handle: usize, arguments: &Arguments) -> u64 {
        (handle as u64)
            .rotate_left(7)
            .wrapping_add(u64::from(arguments.a))
            .wrapping_add(arguments.b.rotate_left(11))
            .wrapping_add(u64::from(arguments.c.to_bits()).rotate_left(19))
            .wrapping_add(arguments.d.to_bits().rotate_left(23))
            .wrapping_add(u64::from(arguments.e).rotate_left(29))
            .wrapping_add(u64::from(arguments.f).rotate_left(31))
            .wrapping_add(arguments.g.rotate_left(37))
            .wrapping_add(u64::from(arguments.h.to_bits()).rotate_left(41))
    }

    unsafe extern "system" fn physical_target(
        handle: VkPhysicalDevice,
        a: u32,
        b: u64,
        c: f32,
        d: f64,
        e: u32,
        f: u32,
        g: u64,
        h: f32,
    ) -> u64 {
        checksum(
            handle.0 as usize,
            &Arguments {
                a,
                b,
                c,
                d,
                e,
                f,
                g,
                h,
            },
        )
    }

    unsafe extern "system" fn device_target(
        handle: VkDevice,
        a: u32,
        b: u64,
        c: f32,
        d: f64,
        e: u32,
        f: u32,
        g: u64,
        h: f32,
    ) -> u64 {
        checksum(
            handle.0 as usize,
            &Arguments {
                a,
                b,
                c,
                d,
                e,
                f,
                g,
                h,
            },
        )
    }

    fn erase_physical(function: PhysicalCommand) -> unsafe extern "system" fn() {
        // SAFETY: Vulkan proc-address values erase the signature without changing
        // the function address; the caller restores this exact test signature.
        unsafe { core::mem::transmute::<PhysicalCommand, unsafe extern "system" fn()>(function) }
    }

    fn erase_device(function: DeviceCommand) -> *mut c_void {
        function as *const () as *mut c_void
    }

    unsafe fn call_physical(function: PFN_vkVoidFunction, handle: VkPhysicalDevice) -> u64 {
        // SAFETY: The selected test slot dispatches to `physical_target`.
        let function = unsafe {
            core::mem::transmute::<unsafe extern "system" fn(), PhysicalCommand>(
                function.unwrap_unchecked(),
            )
        };
        unsafe {
            function(
                handle,
                0x1357_9bdf,
                0x0123_4567_89ab_cdef,
                -17.25,
                1.0 / 3.0,
                0x2468_ace0,
                0xfedc_ba98,
                0xf0e1_d2c3_b4a5_9687,
                4097.5,
            )
        }
    }

    unsafe fn call_device(function: PFN_vkVoidFunction, handle: VkDevice) -> u64 {
        // SAFETY: The selected test slot dispatches to `device_target`.
        let function = unsafe {
            core::mem::transmute::<unsafe extern "system" fn(), DeviceCommand>(
                function.unwrap_unchecked(),
            )
        };
        unsafe {
            function(
                handle,
                0x1357_9bdf,
                0x0123_4567_89ab_cdef,
                -17.25,
                1.0 / 3.0,
                0x2468_ace0,
                0xfedc_ba98,
                0xf0e1_d2c3_b4a5_9687,
                4097.5,
            )
        }
    }

    #[test]
    fn physical_trampoline_preserves_unknown_signature_and_unwraps_handle() {
        let dispatch = UnknownDispatchTable::try_new().unwrap();
        dispatch.store(SLOT, Some(erase_physical(physical_target)));
        let chain = VkPhysicalDevice(0x1234_5678usize as *mut c_void);
        let wrapper = LoaderPhysicalDeviceTrampoline::test_stub(chain, dispatch.as_ptr());
        let handle = VkPhysicalDevice(core::ptr::from_ref(&wrapper).cast_mut().cast());

        // SAFETY: `wrapper`, its dispatch storage, and the target are live.
        let actual = unsafe { call_physical(trampoline_function(SLOT), handle) };
        // SAFETY: Direct invocation supplies the same valid scalar arguments.
        let expected = unsafe { call_physical(Some(erase_physical(physical_target)), chain) };
        assert_eq!(actual, expected);
    }

    #[test]
    fn physical_terminator_preserves_unknown_signature_and_unwraps_handle() {
        let dispatch = UnknownDispatchTable::try_new().unwrap();
        dispatch.store(SLOT, Some(erase_physical(physical_target)));
        let native = VkPhysicalDevice(0x7654_3210usize as *mut c_void);
        let wrapper = LoaderPhysicalDevice::test_stub(native, dispatch.as_ptr());
        let handle = VkPhysicalDevice(core::ptr::from_ref(&wrapper).cast_mut().cast());

        // SAFETY: `wrapper`, its dispatch storage, and the target are live.
        let actual = unsafe { call_physical(terminator_function(SLOT), handle) };
        // SAFETY: Direct invocation supplies the same valid scalar arguments.
        let expected = unsafe { call_physical(Some(erase_physical(physical_target)), native) };
        assert_eq!(actual, expected);
    }

    #[test]
    fn device_trampoline_preserves_the_complete_unknown_signature() {
        assert_eq!(UNKNOWN_DEVICE_DISPATCH_OFFSET % size_of::<usize>(), 0);
        let slot = UNKNOWN_DEVICE_DISPATCH_OFFSET / size_of::<usize>() + SLOT;
        let mut dispatch = vec![0usize; slot + 1];
        dispatch[slot] = erase_device(device_target) as usize;

        let device = FakeDevice {
            dispatch: dispatch.as_ptr(),
        };
        let handle = VkDevice(core::ptr::from_ref(&device).cast_mut().cast());
        // SAFETY: `device`, its dispatch storage, and the target are live.
        let actual = unsafe { call_device(device_trampoline_function(SLOT), handle) };
        // SAFETY: Direct invocation supplies the same valid scalar arguments.
        let expected = unsafe {
            call_device(
                Some(core::mem::transmute::<
                    DeviceCommand,
                    unsafe extern "system" fn(),
                >(device_target)),
                handle,
            )
        };
        assert_eq!(actual, expected);
    }
}
