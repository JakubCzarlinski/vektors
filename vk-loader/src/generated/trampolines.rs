// Generated from registry/vk.xml by vk-loader-codegen. Do not edit.

use crate::LoaderInstance;
use crate::c_void;
use crate::device_dispatch;
use crate::fatal_loader_error;
use crate::invalid_device_dispatch;
use crate::resolve_trampoline_physical_device;
use crate::set_device_dispatchable;
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkAcquireDrmDisplayEXT(
    physicalDevice: vk::VkPhysicalDevice,
    drmFd: i32,
    display: vk::VkDisplayKHR,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkAcquireDrmDisplayEXT: Invalid physicalDevice [VUID-vkAcquireDrmDisplayEXT-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkAcquireDrmDisplayEXT
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, drmFd, display) },
    )
}
#[cfg(target_os = "windows")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkAcquireFullScreenExclusiveModeEXT(
    device: vk::VkDevice,
    swapchain: vk::VkSwapchainKHR,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkAcquireFullScreenExclusiveModeEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, swapchain) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkAcquireNextImage2KHR(
    device: vk::VkDevice,
    pAcquireInfo: *const vk::VkAcquireNextImageInfoKHR<'_>,
    pImageIndex: *mut u32,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkAcquireNextImage2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pAcquireInfo, pImageIndex) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkAcquireNextImageKHR(
    device: vk::VkDevice,
    swapchain: vk::VkSwapchainKHR,
    timeout: u64,
    semaphore: vk::VkSemaphore,
    fence: vk::VkFence,
    pImageIndex: *mut u32,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkAcquireNextImageKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, swapchain, timeout, semaphore, fence, pImageIndex) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkAcquirePerformanceConfigurationINTEL(
    device: vk::VkDevice,
    pAcquireInfo: *const vk::VkPerformanceConfigurationAcquireInfoINTEL<'_>,
    pConfiguration: *mut vk::VkPerformanceConfigurationINTEL,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkAcquirePerformanceConfigurationINTEL;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pAcquireInfo, pConfiguration) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkAcquireProfilingLockKHR(
    device: vk::VkDevice,
    pInfo: *const vk::VkAcquireProfilingLockInfoKHR<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkAcquireProfilingLockKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pInfo) }
}
#[cfg(target_os = "windows")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkAcquireWinrtDisplayNV(
    physicalDevice: vk::VkPhysicalDevice,
    display: vk::VkDisplayKHR,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkAcquireWinrtDisplayNV: Invalid physicalDevice [VUID-vkAcquireWinrtDisplayNV-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkAcquireWinrtDisplayNV
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, display) },
    )
}
#[cfg(all(
    feature = "wsi-xlib-xrandr",
    any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "hurd",
        target_os = "cygwin"
    )
))]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkAcquireXlibDisplayEXT(
    physicalDevice: vk::VkPhysicalDevice,
    dpy: *mut vk::Display,
    display: vk::VkDisplayKHR,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkAcquireXlibDisplayEXT: Invalid physicalDevice [VUID-vkAcquireXlibDisplayEXT-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkAcquireXlibDisplayEXT
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, dpy, display) },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkAllocateCommandBuffers(
    device: vk::VkDevice,
    pAllocateInfo: *const vk::VkCommandBufferAllocateInfo<'_>,
    pCommandBuffers: *mut vk::VkCommandBuffer,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkAllocateCommandBuffers;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    let result = unsafe { command(device, pAllocateInfo, pCommandBuffers) };
    if result == vk::VkResult::SUCCESS {
        let count = unsafe { (*pAllocateInfo).commandBufferCount } as usize;
        for index in 0..count {
            let command_buffer = unsafe { pCommandBuffers.add(index).read() };
            if command_buffer != vk::VkCommandBuffer::NULL {
                unsafe {
                    set_device_dispatchable(command_buffer.0.cast(), core::ptr::from_ref(dispatch));
                }
            }
        }
    }
    result
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkAllocateDescriptorSets(
    device: vk::VkDevice,
    pAllocateInfo: *const vk::VkDescriptorSetAllocateInfo<'_>,
    pDescriptorSets: *mut vk::VkDescriptorSet,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkAllocateDescriptorSets;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pAllocateInfo, pDescriptorSets) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkAllocateMemory(
    device: vk::VkDevice,
    pAllocateInfo: *const vk::VkMemoryAllocateInfo<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pMemory: *mut vk::VkDeviceMemory,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkAllocateMemory;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pAllocateInfo, pAllocator, pMemory) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkAntiLagUpdateAMD(
    device: vk::VkDevice,
    pData: *const vk::VkAntiLagDataAMD<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkAntiLagUpdateAMD;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pData);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkBeginCommandBuffer(
    commandBuffer: vk::VkCommandBuffer,
    pBeginInfo: *const vk::VkCommandBufferBeginInfo<'_>,
) -> vk::VkResult {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkBeginCommandBuffer;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(commandBuffer, pBeginInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkBindAccelerationStructureMemoryNV(
    device: vk::VkDevice,
    bindInfoCount: u32,
    pBindInfos: *const vk::VkBindAccelerationStructureMemoryInfoNV<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkBindAccelerationStructureMemoryNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, bindInfoCount, pBindInfos) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkBindBufferMemory(
    device: vk::VkDevice,
    buffer: vk::VkBuffer,
    memory: vk::VkDeviceMemory,
    memoryOffset: vk::VkDeviceSize,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkBindBufferMemory;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, buffer, memory, memoryOffset) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkBindBufferMemory2(
    device: vk::VkDevice,
    bindInfoCount: u32,
    pBindInfos: *const vk::VkBindBufferMemoryInfo<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkBindBufferMemory2;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, bindInfoCount, pBindInfos) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkBindBufferMemory2KHR(
    device: vk::VkDevice,
    bindInfoCount: u32,
    pBindInfos: *const vk::VkBindBufferMemoryInfoKHR<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkBindBufferMemory2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, bindInfoCount, pBindInfos) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkBindDataGraphPipelineSessionMemoryARM(
    device: vk::VkDevice,
    bindInfoCount: u32,
    pBindInfos: *const vk::VkBindDataGraphPipelineSessionMemoryInfoARM<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkBindDataGraphPipelineSessionMemoryARM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, bindInfoCount, pBindInfos) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkBindImageMemory(
    device: vk::VkDevice,
    image: vk::VkImage,
    memory: vk::VkDeviceMemory,
    memoryOffset: vk::VkDeviceSize,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkBindImageMemory;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, image, memory, memoryOffset) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkBindImageMemory2(
    device: vk::VkDevice,
    bindInfoCount: u32,
    pBindInfos: *const vk::VkBindImageMemoryInfo<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkBindImageMemory2;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, bindInfoCount, pBindInfos) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkBindImageMemory2KHR(
    device: vk::VkDevice,
    bindInfoCount: u32,
    pBindInfos: *const vk::VkBindImageMemoryInfoKHR<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkBindImageMemory2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, bindInfoCount, pBindInfos) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkBindOpticalFlowSessionImageNV(
    device: vk::VkDevice,
    session: vk::VkOpticalFlowSessionNV,
    bindingPoint: vk::VkOpticalFlowSessionBindingPointNV,
    view: vk::VkImageView,
    layout: vk::VkImageLayout,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkBindOpticalFlowSessionImageNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, session, bindingPoint, view, layout) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkBindTensorMemoryARM(
    device: vk::VkDevice,
    bindInfoCount: u32,
    pBindInfos: *const vk::VkBindTensorMemoryInfoARM<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkBindTensorMemoryARM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, bindInfoCount, pBindInfos) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkBindVideoSessionMemoryKHR(
    device: vk::VkDevice,
    videoSession: vk::VkVideoSessionKHR,
    bindSessionMemoryInfoCount: u32,
    pBindSessionMemoryInfos: *const vk::VkBindVideoSessionMemoryInfoKHR<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkBindVideoSessionMemoryKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            videoSession,
            bindSessionMemoryInfoCount,
            pBindSessionMemoryInfos,
        )
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkBuildAccelerationStructuresKHR(
    device: vk::VkDevice,
    deferredOperation: vk::VkDeferredOperationKHR,
    infoCount: u32,
    pInfos: *const vk::VkAccelerationStructureBuildGeometryInfoKHR<'_>,
    ppBuildRangeInfos: *const *const vk::VkAccelerationStructureBuildRangeInfoKHR,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkBuildAccelerationStructuresKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            deferredOperation,
            infoCount,
            pInfos,
            ppBuildRangeInfos,
        )
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkBuildMicromapsEXT(
    device: vk::VkDevice,
    deferredOperation: vk::VkDeferredOperationKHR,
    infoCount: u32,
    pInfos: *const vk::VkMicromapBuildInfoEXT<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkBuildMicromapsEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, deferredOperation, infoCount, pInfos) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkClearShaderInstrumentationMetricsARM(
    device: vk::VkDevice,
    instrumentation: vk::VkShaderInstrumentationARM,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkClearShaderInstrumentationMetricsARM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, instrumentation);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBeginConditionalRendering2EXT(
    commandBuffer: vk::VkCommandBuffer,
    pConditionalRenderingBegin: *const vk::VkConditionalRenderingBeginInfo2EXT<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBeginConditionalRendering2EXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pConditionalRenderingBegin);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBeginConditionalRenderingEXT(
    commandBuffer: vk::VkCommandBuffer,
    pConditionalRenderingBegin: *const vk::VkConditionalRenderingBeginInfoEXT<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBeginConditionalRenderingEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pConditionalRenderingBegin);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBeginCustomResolveEXT(
    commandBuffer: vk::VkCommandBuffer,
    pBeginCustomResolveInfo: *const vk::VkBeginCustomResolveInfoEXT<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBeginCustomResolveEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pBeginCustomResolveInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBeginDebugUtilsLabelEXT(
    commandBuffer: vk::VkCommandBuffer,
    pLabelInfo: *const vk::VkDebugUtilsLabelEXT<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBeginDebugUtilsLabelEXT;
    if let Some(command) = command {
        unsafe {
            command(commandBuffer, pLabelInfo);
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBeginGpaSampleAMD(
    commandBuffer: vk::VkCommandBuffer,
    gpaSession: vk::VkGpaSessionAMD,
    pGpaSampleBeginInfo: *const vk::VkGpaSampleBeginInfoAMD<'_>,
    pSampleID: *mut u32,
) -> vk::VkResult {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBeginGpaSampleAMD;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(commandBuffer, gpaSession, pGpaSampleBeginInfo, pSampleID) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBeginGpaSessionAMD(
    commandBuffer: vk::VkCommandBuffer,
    gpaSession: vk::VkGpaSessionAMD,
) -> vk::VkResult {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBeginGpaSessionAMD;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(commandBuffer, gpaSession) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBeginPerTileExecutionQCOM(
    commandBuffer: vk::VkCommandBuffer,
    pPerTileBeginInfo: *const vk::VkPerTileBeginInfoQCOM<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBeginPerTileExecutionQCOM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pPerTileBeginInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdBeginQuery(
    commandBuffer: vk::VkCommandBuffer,
    queryPool: vk::VkQueryPool,
    query: u32,
    flags: vk::VkQueryControlFlags,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBeginQuery;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, queryPool, query, flags);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBeginQueryIndexedEXT(
    commandBuffer: vk::VkCommandBuffer,
    queryPool: vk::VkQueryPool,
    query: u32,
    flags: vk::VkQueryControlFlags,
    index: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBeginQueryIndexedEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, queryPool, query, flags, index);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdBeginRenderPass(
    commandBuffer: vk::VkCommandBuffer,
    pRenderPassBegin: *const vk::VkRenderPassBeginInfo<'_>,
    contents: vk::VkSubpassContents,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBeginRenderPass;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pRenderPassBegin, contents);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdBeginRenderPass2(
    commandBuffer: vk::VkCommandBuffer,
    pRenderPassBegin: *const vk::VkRenderPassBeginInfo<'_>,
    pSubpassBeginInfo: *const vk::VkSubpassBeginInfo<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBeginRenderPass2;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pRenderPassBegin, pSubpassBeginInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBeginRenderPass2KHR(
    commandBuffer: vk::VkCommandBuffer,
    pRenderPassBegin: *const vk::VkRenderPassBeginInfo<'_>,
    pSubpassBeginInfo: *const vk::VkSubpassBeginInfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBeginRenderPass2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pRenderPassBegin, pSubpassBeginInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdBeginRendering(
    commandBuffer: vk::VkCommandBuffer,
    pRenderingInfo: *const vk::VkRenderingInfo<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBeginRendering;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pRenderingInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBeginRenderingKHR(
    commandBuffer: vk::VkCommandBuffer,
    pRenderingInfo: *const vk::VkRenderingInfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBeginRenderingKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pRenderingInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBeginShaderInstrumentationARM(
    commandBuffer: vk::VkCommandBuffer,
    instrumentation: vk::VkShaderInstrumentationARM,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBeginShaderInstrumentationARM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, instrumentation);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBeginTransformFeedback2EXT(
    commandBuffer: vk::VkCommandBuffer,
    firstCounterRange: u32,
    counterRangeCount: u32,
    pCounterInfos: *const vk::VkBindTransformFeedbackBuffer2InfoEXT<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBeginTransformFeedback2EXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            firstCounterRange,
            counterRangeCount,
            pCounterInfos,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBeginTransformFeedbackEXT(
    commandBuffer: vk::VkCommandBuffer,
    firstCounterBuffer: u32,
    counterBufferCount: u32,
    pCounterBuffers: *const vk::VkBuffer,
    pCounterBufferOffsets: *const vk::VkDeviceSize,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBeginTransformFeedbackEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            firstCounterBuffer,
            counterBufferCount,
            pCounterBuffers,
            pCounterBufferOffsets,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBeginVideoCodingKHR(
    commandBuffer: vk::VkCommandBuffer,
    pBeginInfo: *const vk::VkVideoBeginCodingInfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBeginVideoCodingKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pBeginInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBindDescriptorBufferEmbeddedSamplers2EXT(
    commandBuffer: vk::VkCommandBuffer,
    pBindDescriptorBufferEmbeddedSamplersInfo: *const vk::VkBindDescriptorBufferEmbeddedSamplersInfoEXT<
        '_,
    >,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBindDescriptorBufferEmbeddedSamplers2EXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pBindDescriptorBufferEmbeddedSamplersInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBindDescriptorBufferEmbeddedSamplersEXT(
    commandBuffer: vk::VkCommandBuffer,
    pipelineBindPoint: vk::VkPipelineBindPoint,
    layout: vk::VkPipelineLayout,
    set: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBindDescriptorBufferEmbeddedSamplersEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pipelineBindPoint, layout, set);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBindDescriptorBuffersEXT(
    commandBuffer: vk::VkCommandBuffer,
    bufferCount: u32,
    pBindingInfos: *const vk::VkDescriptorBufferBindingInfoEXT<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBindDescriptorBuffersEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, bufferCount, pBindingInfos);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdBindDescriptorSets(
    commandBuffer: vk::VkCommandBuffer,
    pipelineBindPoint: vk::VkPipelineBindPoint,
    layout: vk::VkPipelineLayout,
    firstSet: u32,
    descriptorSetCount: u32,
    pDescriptorSets: *const vk::VkDescriptorSet,
    dynamicOffsetCount: u32,
    pDynamicOffsets: *const u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBindDescriptorSets;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            pipelineBindPoint,
            layout,
            firstSet,
            descriptorSetCount,
            pDescriptorSets,
            dynamicOffsetCount,
            pDynamicOffsets,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdBindDescriptorSets2(
    commandBuffer: vk::VkCommandBuffer,
    pBindDescriptorSetsInfo: *const vk::VkBindDescriptorSetsInfo<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBindDescriptorSets2;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pBindDescriptorSetsInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBindDescriptorSets2KHR(
    commandBuffer: vk::VkCommandBuffer,
    pBindDescriptorSetsInfo: *const vk::VkBindDescriptorSetsInfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBindDescriptorSets2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pBindDescriptorSetsInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdBindIndexBuffer(
    commandBuffer: vk::VkCommandBuffer,
    buffer: vk::VkBuffer,
    offset: vk::VkDeviceSize,
    indexType: vk::VkIndexType,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBindIndexBuffer;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, buffer, offset, indexType);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdBindIndexBuffer2(
    commandBuffer: vk::VkCommandBuffer,
    buffer: vk::VkBuffer,
    offset: vk::VkDeviceSize,
    size: vk::VkDeviceSize,
    indexType: vk::VkIndexType,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBindIndexBuffer2;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, buffer, offset, size, indexType);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBindIndexBuffer2KHR(
    commandBuffer: vk::VkCommandBuffer,
    buffer: vk::VkBuffer,
    offset: vk::VkDeviceSize,
    size: vk::VkDeviceSize,
    indexType: vk::VkIndexType,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBindIndexBuffer2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, buffer, offset, size, indexType);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBindIndexBuffer3KHR(
    commandBuffer: vk::VkCommandBuffer,
    pInfo: *const vk::VkBindIndexBuffer3InfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBindIndexBuffer3KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBindInvocationMaskHUAWEI(
    commandBuffer: vk::VkCommandBuffer,
    imageView: vk::VkImageView,
    imageLayout: vk::VkImageLayout,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBindInvocationMaskHUAWEI;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, imageView, imageLayout);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdBindPipeline(
    commandBuffer: vk::VkCommandBuffer,
    pipelineBindPoint: vk::VkPipelineBindPoint,
    pipeline: vk::VkPipeline,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBindPipeline;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pipelineBindPoint, pipeline);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBindPipelineShaderGroupNV(
    commandBuffer: vk::VkCommandBuffer,
    pipelineBindPoint: vk::VkPipelineBindPoint,
    pipeline: vk::VkPipeline,
    groupIndex: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBindPipelineShaderGroupNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pipelineBindPoint, pipeline, groupIndex);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBindResourceHeapEXT(
    commandBuffer: vk::VkCommandBuffer,
    pBindInfo: *const vk::VkBindHeapInfoEXT<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBindResourceHeapEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pBindInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBindSamplerHeapEXT(
    commandBuffer: vk::VkCommandBuffer,
    pBindInfo: *const vk::VkBindHeapInfoEXT<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBindSamplerHeapEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pBindInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBindShadersEXT(
    commandBuffer: vk::VkCommandBuffer,
    stageCount: u32,
    pStages: *const vk::VkShaderStageFlagBits,
    pShaders: *const vk::VkShaderEXT,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBindShadersEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, stageCount, pStages, pShaders);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBindShadingRateImageNV(
    commandBuffer: vk::VkCommandBuffer,
    imageView: vk::VkImageView,
    imageLayout: vk::VkImageLayout,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBindShadingRateImageNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, imageView, imageLayout);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBindTileMemoryQCOM(
    commandBuffer: vk::VkCommandBuffer,
    pTileMemoryBindInfo: *const vk::VkTileMemoryBindInfoQCOM<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBindTileMemoryQCOM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pTileMemoryBindInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBindTransformFeedbackBuffers2EXT(
    commandBuffer: vk::VkCommandBuffer,
    firstBinding: u32,
    bindingCount: u32,
    pBindingInfos: *const vk::VkBindTransformFeedbackBuffer2InfoEXT<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBindTransformFeedbackBuffers2EXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, firstBinding, bindingCount, pBindingInfos);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBindTransformFeedbackBuffersEXT(
    commandBuffer: vk::VkCommandBuffer,
    firstBinding: u32,
    bindingCount: u32,
    pBuffers: *const vk::VkBuffer,
    pOffsets: *const vk::VkDeviceSize,
    pSizes: *const vk::VkDeviceSize,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBindTransformFeedbackBuffersEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            firstBinding,
            bindingCount,
            pBuffers,
            pOffsets,
            pSizes,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdBindVertexBuffers(
    commandBuffer: vk::VkCommandBuffer,
    firstBinding: u32,
    bindingCount: u32,
    pBuffers: *const vk::VkBuffer,
    pOffsets: *const vk::VkDeviceSize,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBindVertexBuffers;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            firstBinding,
            bindingCount,
            pBuffers,
            pOffsets,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdBindVertexBuffers2(
    commandBuffer: vk::VkCommandBuffer,
    firstBinding: u32,
    bindingCount: u32,
    pBuffers: *const vk::VkBuffer,
    pOffsets: *const vk::VkDeviceSize,
    pSizes: *const vk::VkDeviceSize,
    pStrides: *const vk::VkDeviceSize,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBindVertexBuffers2;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            firstBinding,
            bindingCount,
            pBuffers,
            pOffsets,
            pSizes,
            pStrides,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBindVertexBuffers2EXT(
    commandBuffer: vk::VkCommandBuffer,
    firstBinding: u32,
    bindingCount: u32,
    pBuffers: *const vk::VkBuffer,
    pOffsets: *const vk::VkDeviceSize,
    pSizes: *const vk::VkDeviceSize,
    pStrides: *const vk::VkDeviceSize,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBindVertexBuffers2EXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            firstBinding,
            bindingCount,
            pBuffers,
            pOffsets,
            pSizes,
            pStrides,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBindVertexBuffers3KHR(
    commandBuffer: vk::VkCommandBuffer,
    firstBinding: u32,
    bindingCount: u32,
    pBindingInfos: *const vk::VkBindVertexBuffer3InfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBindVertexBuffers3KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, firstBinding, bindingCount, pBindingInfos);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdBlitImage(
    commandBuffer: vk::VkCommandBuffer,
    srcImage: vk::VkImage,
    srcImageLayout: vk::VkImageLayout,
    dstImage: vk::VkImage,
    dstImageLayout: vk::VkImageLayout,
    regionCount: u32,
    pRegions: *const vk::VkImageBlit,
    filter: vk::VkFilter,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBlitImage;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            srcImage,
            srcImageLayout,
            dstImage,
            dstImageLayout,
            regionCount,
            pRegions,
            filter,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdBlitImage2(
    commandBuffer: vk::VkCommandBuffer,
    pBlitImageInfo: *const vk::VkBlitImageInfo2<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBlitImage2;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pBlitImageInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBlitImage2KHR(
    commandBuffer: vk::VkCommandBuffer,
    pBlitImageInfo: *const vk::VkBlitImageInfo2KHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBlitImage2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pBlitImageInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBuildAccelerationStructureNV(
    commandBuffer: vk::VkCommandBuffer,
    pInfo: *const vk::VkAccelerationStructureInfoNV<'_>,
    instanceData: vk::VkBuffer,
    instanceOffset: vk::VkDeviceSize,
    update: vk::VkBool32,
    dst: vk::VkAccelerationStructureNV,
    src: vk::VkAccelerationStructureNV,
    scratch: vk::VkBuffer,
    scratchOffset: vk::VkDeviceSize,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBuildAccelerationStructureNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            pInfo,
            instanceData,
            instanceOffset,
            update,
            dst,
            src,
            scratch,
            scratchOffset,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBuildAccelerationStructuresIndirectKHR(
    commandBuffer: vk::VkCommandBuffer,
    infoCount: u32,
    pInfos: *const vk::VkAccelerationStructureBuildGeometryInfoKHR<'_>,
    pIndirectDeviceAddresses: *const vk::VkDeviceAddress,
    pIndirectStrides: *const u32,
    ppMaxPrimitiveCounts: *const *const u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBuildAccelerationStructuresIndirectKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            infoCount,
            pInfos,
            pIndirectDeviceAddresses,
            pIndirectStrides,
            ppMaxPrimitiveCounts,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBuildAccelerationStructuresKHR(
    commandBuffer: vk::VkCommandBuffer,
    infoCount: u32,
    pInfos: *const vk::VkAccelerationStructureBuildGeometryInfoKHR<'_>,
    ppBuildRangeInfos: *const *const vk::VkAccelerationStructureBuildRangeInfoKHR,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBuildAccelerationStructuresKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, infoCount, pInfos, ppBuildRangeInfos);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBuildClusterAccelerationStructureIndirectNV(
    commandBuffer: vk::VkCommandBuffer,
    pCommandInfos: *const vk::VkClusterAccelerationStructureCommandsInfoNV<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBuildClusterAccelerationStructureIndirectNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pCommandInfos);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBuildMicromapsEXT(
    commandBuffer: vk::VkCommandBuffer,
    infoCount: u32,
    pInfos: *const vk::VkMicromapBuildInfoEXT<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBuildMicromapsEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, infoCount, pInfos);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdBuildPartitionedAccelerationStructuresNV(
    commandBuffer: vk::VkCommandBuffer,
    pBuildInfo: *const vk::VkBuildPartitionedAccelerationStructureInfoNV<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdBuildPartitionedAccelerationStructuresNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pBuildInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdClearAttachments(
    commandBuffer: vk::VkCommandBuffer,
    attachmentCount: u32,
    pAttachments: *const vk::VkClearAttachment,
    rectCount: u32,
    pRects: *const vk::VkClearRect,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdClearAttachments;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            attachmentCount,
            pAttachments,
            rectCount,
            pRects,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdClearColorImage(
    commandBuffer: vk::VkCommandBuffer,
    image: vk::VkImage,
    imageLayout: vk::VkImageLayout,
    pColor: *const vk::VkClearColorValue,
    rangeCount: u32,
    pRanges: *const vk::VkImageSubresourceRange,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdClearColorImage;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            image,
            imageLayout,
            pColor,
            rangeCount,
            pRanges,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdClearDepthStencilImage(
    commandBuffer: vk::VkCommandBuffer,
    image: vk::VkImage,
    imageLayout: vk::VkImageLayout,
    pDepthStencil: *const vk::VkClearDepthStencilValue,
    rangeCount: u32,
    pRanges: *const vk::VkImageSubresourceRange,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdClearDepthStencilImage;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            image,
            imageLayout,
            pDepthStencil,
            rangeCount,
            pRanges,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdControlVideoCodingKHR(
    commandBuffer: vk::VkCommandBuffer,
    pCodingControlInfo: *const vk::VkVideoCodingControlInfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdControlVideoCodingKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pCodingControlInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdConvertCooperativeVectorMatrixNV(
    commandBuffer: vk::VkCommandBuffer,
    infoCount: u32,
    pInfos: *const vk::VkConvertCooperativeVectorMatrixInfoNV<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdConvertCooperativeVectorMatrixNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, infoCount, pInfos);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdCopyAccelerationStructureKHR(
    commandBuffer: vk::VkCommandBuffer,
    pInfo: *const vk::VkCopyAccelerationStructureInfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCopyAccelerationStructureKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdCopyAccelerationStructureNV(
    commandBuffer: vk::VkCommandBuffer,
    dst: vk::VkAccelerationStructureNV,
    src: vk::VkAccelerationStructureNV,
    mode: vk::VkCopyAccelerationStructureModeKHR,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCopyAccelerationStructureNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, dst, src, mode);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdCopyAccelerationStructureToMemoryKHR(
    commandBuffer: vk::VkCommandBuffer,
    pInfo: *const vk::VkCopyAccelerationStructureToMemoryInfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCopyAccelerationStructureToMemoryKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdCopyBuffer(
    commandBuffer: vk::VkCommandBuffer,
    srcBuffer: vk::VkBuffer,
    dstBuffer: vk::VkBuffer,
    regionCount: u32,
    pRegions: *const vk::VkBufferCopy,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCopyBuffer;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, srcBuffer, dstBuffer, regionCount, pRegions);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdCopyBuffer2(
    commandBuffer: vk::VkCommandBuffer,
    pCopyBufferInfo: *const vk::VkCopyBufferInfo2<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCopyBuffer2;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pCopyBufferInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdCopyBuffer2KHR(
    commandBuffer: vk::VkCommandBuffer,
    pCopyBufferInfo: *const vk::VkCopyBufferInfo2KHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCopyBuffer2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pCopyBufferInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdCopyBufferToImage(
    commandBuffer: vk::VkCommandBuffer,
    srcBuffer: vk::VkBuffer,
    dstImage: vk::VkImage,
    dstImageLayout: vk::VkImageLayout,
    regionCount: u32,
    pRegions: *const vk::VkBufferImageCopy,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCopyBufferToImage;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            srcBuffer,
            dstImage,
            dstImageLayout,
            regionCount,
            pRegions,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdCopyBufferToImage2(
    commandBuffer: vk::VkCommandBuffer,
    pCopyBufferToImageInfo: *const vk::VkCopyBufferToImageInfo2<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCopyBufferToImage2;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pCopyBufferToImageInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdCopyBufferToImage2KHR(
    commandBuffer: vk::VkCommandBuffer,
    pCopyBufferToImageInfo: *const vk::VkCopyBufferToImageInfo2KHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCopyBufferToImage2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pCopyBufferToImageInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdCopyGpaSessionResultsAMD(
    commandBuffer: vk::VkCommandBuffer,
    gpaSession: vk::VkGpaSessionAMD,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCopyGpaSessionResultsAMD;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, gpaSession);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdCopyImage(
    commandBuffer: vk::VkCommandBuffer,
    srcImage: vk::VkImage,
    srcImageLayout: vk::VkImageLayout,
    dstImage: vk::VkImage,
    dstImageLayout: vk::VkImageLayout,
    regionCount: u32,
    pRegions: *const vk::VkImageCopy,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCopyImage;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            srcImage,
            srcImageLayout,
            dstImage,
            dstImageLayout,
            regionCount,
            pRegions,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdCopyImage2(
    commandBuffer: vk::VkCommandBuffer,
    pCopyImageInfo: *const vk::VkCopyImageInfo2<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCopyImage2;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pCopyImageInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdCopyImage2KHR(
    commandBuffer: vk::VkCommandBuffer,
    pCopyImageInfo: *const vk::VkCopyImageInfo2KHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCopyImage2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pCopyImageInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdCopyImageToBuffer(
    commandBuffer: vk::VkCommandBuffer,
    srcImage: vk::VkImage,
    srcImageLayout: vk::VkImageLayout,
    dstBuffer: vk::VkBuffer,
    regionCount: u32,
    pRegions: *const vk::VkBufferImageCopy,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCopyImageToBuffer;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            srcImage,
            srcImageLayout,
            dstBuffer,
            regionCount,
            pRegions,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdCopyImageToBuffer2(
    commandBuffer: vk::VkCommandBuffer,
    pCopyImageToBufferInfo: *const vk::VkCopyImageToBufferInfo2<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCopyImageToBuffer2;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pCopyImageToBufferInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdCopyImageToBuffer2KHR(
    commandBuffer: vk::VkCommandBuffer,
    pCopyImageToBufferInfo: *const vk::VkCopyImageToBufferInfo2KHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCopyImageToBuffer2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pCopyImageToBufferInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdCopyImageToMemoryKHR(
    commandBuffer: vk::VkCommandBuffer,
    pCopyMemoryInfo: *const vk::VkCopyDeviceMemoryImageInfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCopyImageToMemoryKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pCopyMemoryInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdCopyMemoryIndirectKHR(
    commandBuffer: vk::VkCommandBuffer,
    pCopyMemoryIndirectInfo: *const vk::VkCopyMemoryIndirectInfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCopyMemoryIndirectKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pCopyMemoryIndirectInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdCopyMemoryIndirectNV(
    commandBuffer: vk::VkCommandBuffer,
    copyBufferAddress: vk::VkDeviceAddress,
    copyCount: u32,
    stride: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCopyMemoryIndirectNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, copyBufferAddress, copyCount, stride);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdCopyMemoryKHR(
    commandBuffer: vk::VkCommandBuffer,
    pCopyMemoryInfo: *const vk::VkCopyDeviceMemoryInfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCopyMemoryKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pCopyMemoryInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdCopyMemoryToAccelerationStructureKHR(
    commandBuffer: vk::VkCommandBuffer,
    pInfo: *const vk::VkCopyMemoryToAccelerationStructureInfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCopyMemoryToAccelerationStructureKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdCopyMemoryToImageIndirectKHR(
    commandBuffer: vk::VkCommandBuffer,
    pCopyMemoryToImageIndirectInfo: *const vk::VkCopyMemoryToImageIndirectInfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCopyMemoryToImageIndirectKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pCopyMemoryToImageIndirectInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdCopyMemoryToImageIndirectNV(
    commandBuffer: vk::VkCommandBuffer,
    copyBufferAddress: vk::VkDeviceAddress,
    copyCount: u32,
    stride: u32,
    dstImage: vk::VkImage,
    dstImageLayout: vk::VkImageLayout,
    pImageSubresources: *const vk::VkImageSubresourceLayers,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCopyMemoryToImageIndirectNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            copyBufferAddress,
            copyCount,
            stride,
            dstImage,
            dstImageLayout,
            pImageSubresources,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdCopyMemoryToImageKHR(
    commandBuffer: vk::VkCommandBuffer,
    pCopyMemoryInfo: *const vk::VkCopyDeviceMemoryImageInfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCopyMemoryToImageKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pCopyMemoryInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdCopyMemoryToMicromapEXT(
    commandBuffer: vk::VkCommandBuffer,
    pInfo: *const vk::VkCopyMemoryToMicromapInfoEXT<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCopyMemoryToMicromapEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdCopyMicromapEXT(
    commandBuffer: vk::VkCommandBuffer,
    pInfo: *const vk::VkCopyMicromapInfoEXT<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCopyMicromapEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdCopyMicromapToMemoryEXT(
    commandBuffer: vk::VkCommandBuffer,
    pInfo: *const vk::VkCopyMicromapToMemoryInfoEXT<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCopyMicromapToMemoryEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdCopyQueryPoolResults(
    commandBuffer: vk::VkCommandBuffer,
    queryPool: vk::VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
    dstBuffer: vk::VkBuffer,
    dstOffset: vk::VkDeviceSize,
    stride: vk::VkDeviceSize,
    flags: vk::VkQueryResultFlags,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCopyQueryPoolResults;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            queryPool,
            firstQuery,
            queryCount,
            dstBuffer,
            dstOffset,
            stride,
            flags,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdCopyQueryPoolResultsToMemoryKHR(
    commandBuffer: vk::VkCommandBuffer,
    queryPool: vk::VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
    pDstRange: *const vk::VkStridedDeviceAddressRangeKHR,
    dstFlags: vk::VkAddressCommandFlagsKHR,
    queryResultFlags: vk::VkQueryResultFlags,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCopyQueryPoolResultsToMemoryKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            queryPool,
            firstQuery,
            queryCount,
            pDstRange,
            dstFlags,
            queryResultFlags,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdCopyTensorARM(
    commandBuffer: vk::VkCommandBuffer,
    pCopyTensorInfo: *const vk::VkCopyTensorInfoARM<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCopyTensorARM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pCopyTensorInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdCuLaunchKernelNVX(
    commandBuffer: vk::VkCommandBuffer,
    pLaunchInfo: *const vk::VkCuLaunchInfoNVX<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCuLaunchKernelNVX;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pLaunchInfo);
    }
}
#[cfg(feature = "beta-extensions")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdCudaLaunchKernelNV(
    commandBuffer: vk::VkCommandBuffer,
    pLaunchInfo: *const vk::VkCudaLaunchInfoNV<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdCudaLaunchKernelNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pLaunchInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDebugMarkerBeginEXT(
    commandBuffer: vk::VkCommandBuffer,
    pMarkerInfo: *const vk::VkDebugMarkerMarkerInfoEXT<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDebugMarkerBeginEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pMarkerInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDebugMarkerEndEXT(commandBuffer: vk::VkCommandBuffer) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDebugMarkerEndEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDebugMarkerInsertEXT(
    commandBuffer: vk::VkCommandBuffer,
    pMarkerInfo: *const vk::VkDebugMarkerMarkerInfoEXT<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDebugMarkerInsertEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pMarkerInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDecodeVideoKHR(
    commandBuffer: vk::VkCommandBuffer,
    pDecodeInfo: *const vk::VkVideoDecodeInfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDecodeVideoKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pDecodeInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDecompressMemoryEXT(
    commandBuffer: vk::VkCommandBuffer,
    pDecompressMemoryInfoEXT: *const vk::VkDecompressMemoryInfoEXT<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDecompressMemoryEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pDecompressMemoryInfoEXT);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDecompressMemoryIndirectCountEXT(
    commandBuffer: vk::VkCommandBuffer,
    decompressionMethod: vk::VkMemoryDecompressionMethodFlagsEXT,
    indirectCommandsAddress: vk::VkDeviceAddress,
    indirectCommandsCountAddress: vk::VkDeviceAddress,
    maxDecompressionCount: u32,
    stride: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDecompressMemoryIndirectCountEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            decompressionMethod,
            indirectCommandsAddress,
            indirectCommandsCountAddress,
            maxDecompressionCount,
            stride,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDecompressMemoryIndirectCountNV(
    commandBuffer: vk::VkCommandBuffer,
    indirectCommandsAddress: vk::VkDeviceAddress,
    indirectCommandsCountAddress: vk::VkDeviceAddress,
    stride: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDecompressMemoryIndirectCountNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            indirectCommandsAddress,
            indirectCommandsCountAddress,
            stride,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDecompressMemoryNV(
    commandBuffer: vk::VkCommandBuffer,
    decompressRegionCount: u32,
    pDecompressMemoryRegions: *const vk::VkDecompressMemoryRegionNV,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDecompressMemoryNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            decompressRegionCount,
            pDecompressMemoryRegions,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdDispatch(
    commandBuffer: vk::VkCommandBuffer,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDispatch;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, groupCountX, groupCountY, groupCountZ);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdDispatchBase(
    commandBuffer: vk::VkCommandBuffer,
    baseGroupX: u32,
    baseGroupY: u32,
    baseGroupZ: u32,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDispatchBase;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            baseGroupX,
            baseGroupY,
            baseGroupZ,
            groupCountX,
            groupCountY,
            groupCountZ,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDispatchBaseKHR(
    commandBuffer: vk::VkCommandBuffer,
    baseGroupX: u32,
    baseGroupY: u32,
    baseGroupZ: u32,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDispatchBaseKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            baseGroupX,
            baseGroupY,
            baseGroupZ,
            groupCountX,
            groupCountY,
            groupCountZ,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDispatchDataGraphARM(
    commandBuffer: vk::VkCommandBuffer,
    session: vk::VkDataGraphPipelineSessionARM,
    pInfo: *const vk::VkDataGraphPipelineDispatchInfoARM<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDispatchDataGraphARM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, session, pInfo);
    }
}
#[cfg(feature = "beta-extensions")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDispatchGraphAMDX(
    commandBuffer: vk::VkCommandBuffer,
    scratch: vk::VkDeviceAddress,
    scratchSize: vk::VkDeviceSize,
    pCountInfo: *const vk::VkDispatchGraphCountInfoAMDX<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDispatchGraphAMDX;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, scratch, scratchSize, pCountInfo);
    }
}
#[cfg(feature = "beta-extensions")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDispatchGraphIndirectAMDX(
    commandBuffer: vk::VkCommandBuffer,
    scratch: vk::VkDeviceAddress,
    scratchSize: vk::VkDeviceSize,
    pCountInfo: *const vk::VkDispatchGraphCountInfoAMDX<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDispatchGraphIndirectAMDX;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, scratch, scratchSize, pCountInfo);
    }
}
#[cfg(feature = "beta-extensions")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDispatchGraphIndirectCountAMDX(
    commandBuffer: vk::VkCommandBuffer,
    scratch: vk::VkDeviceAddress,
    scratchSize: vk::VkDeviceSize,
    countInfo: vk::VkDeviceAddress,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDispatchGraphIndirectCountAMDX;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, scratch, scratchSize, countInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdDispatchIndirect(
    commandBuffer: vk::VkCommandBuffer,
    buffer: vk::VkBuffer,
    offset: vk::VkDeviceSize,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDispatchIndirect;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, buffer, offset);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDispatchIndirect2KHR(
    commandBuffer: vk::VkCommandBuffer,
    pInfo: *const vk::VkDispatchIndirect2InfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDispatchIndirect2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDispatchTileQCOM(
    commandBuffer: vk::VkCommandBuffer,
    pDispatchTileInfo: *const vk::VkDispatchTileInfoQCOM<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDispatchTileQCOM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pDispatchTileInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdDraw(
    commandBuffer: vk::VkCommandBuffer,
    vertexCount: u32,
    instanceCount: u32,
    firstVertex: u32,
    firstInstance: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDraw;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            vertexCount,
            instanceCount,
            firstVertex,
            firstInstance,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDrawClusterHUAWEI(
    commandBuffer: vk::VkCommandBuffer,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDrawClusterHUAWEI;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, groupCountX, groupCountY, groupCountZ);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDrawClusterIndirectHUAWEI(
    commandBuffer: vk::VkCommandBuffer,
    buffer: vk::VkBuffer,
    offset: vk::VkDeviceSize,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDrawClusterIndirectHUAWEI;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, buffer, offset);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdDrawIndexed(
    commandBuffer: vk::VkCommandBuffer,
    indexCount: u32,
    instanceCount: u32,
    firstIndex: u32,
    vertexOffset: i32,
    firstInstance: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDrawIndexed;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            indexCount,
            instanceCount,
            firstIndex,
            vertexOffset,
            firstInstance,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdDrawIndexedIndirect(
    commandBuffer: vk::VkCommandBuffer,
    buffer: vk::VkBuffer,
    offset: vk::VkDeviceSize,
    drawCount: u32,
    stride: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDrawIndexedIndirect;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, buffer, offset, drawCount, stride);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDrawIndexedIndirect2KHR(
    commandBuffer: vk::VkCommandBuffer,
    pInfo: *const vk::VkDrawIndirect2InfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDrawIndexedIndirect2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdDrawIndexedIndirectCount(
    commandBuffer: vk::VkCommandBuffer,
    buffer: vk::VkBuffer,
    offset: vk::VkDeviceSize,
    countBuffer: vk::VkBuffer,
    countBufferOffset: vk::VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDrawIndexedIndirectCount;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            buffer,
            offset,
            countBuffer,
            countBufferOffset,
            maxDrawCount,
            stride,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDrawIndexedIndirectCount2KHR(
    commandBuffer: vk::VkCommandBuffer,
    pInfo: *const vk::VkDrawIndirectCount2InfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDrawIndexedIndirectCount2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDrawIndexedIndirectCountAMD(
    commandBuffer: vk::VkCommandBuffer,
    buffer: vk::VkBuffer,
    offset: vk::VkDeviceSize,
    countBuffer: vk::VkBuffer,
    countBufferOffset: vk::VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDrawIndexedIndirectCountAMD;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            buffer,
            offset,
            countBuffer,
            countBufferOffset,
            maxDrawCount,
            stride,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDrawIndexedIndirectCountKHR(
    commandBuffer: vk::VkCommandBuffer,
    buffer: vk::VkBuffer,
    offset: vk::VkDeviceSize,
    countBuffer: vk::VkBuffer,
    countBufferOffset: vk::VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDrawIndexedIndirectCountKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            buffer,
            offset,
            countBuffer,
            countBufferOffset,
            maxDrawCount,
            stride,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdDrawIndirect(
    commandBuffer: vk::VkCommandBuffer,
    buffer: vk::VkBuffer,
    offset: vk::VkDeviceSize,
    drawCount: u32,
    stride: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDrawIndirect;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, buffer, offset, drawCount, stride);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDrawIndirect2KHR(
    commandBuffer: vk::VkCommandBuffer,
    pInfo: *const vk::VkDrawIndirect2InfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDrawIndirect2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDrawIndirectByteCount2EXT(
    commandBuffer: vk::VkCommandBuffer,
    instanceCount: u32,
    firstInstance: u32,
    pCounterInfo: *const vk::VkBindTransformFeedbackBuffer2InfoEXT<'_>,
    counterOffset: u32,
    vertexStride: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDrawIndirectByteCount2EXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            instanceCount,
            firstInstance,
            pCounterInfo,
            counterOffset,
            vertexStride,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDrawIndirectByteCountEXT(
    commandBuffer: vk::VkCommandBuffer,
    instanceCount: u32,
    firstInstance: u32,
    counterBuffer: vk::VkBuffer,
    counterBufferOffset: vk::VkDeviceSize,
    counterOffset: u32,
    vertexStride: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDrawIndirectByteCountEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            instanceCount,
            firstInstance,
            counterBuffer,
            counterBufferOffset,
            counterOffset,
            vertexStride,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdDrawIndirectCount(
    commandBuffer: vk::VkCommandBuffer,
    buffer: vk::VkBuffer,
    offset: vk::VkDeviceSize,
    countBuffer: vk::VkBuffer,
    countBufferOffset: vk::VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDrawIndirectCount;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            buffer,
            offset,
            countBuffer,
            countBufferOffset,
            maxDrawCount,
            stride,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDrawIndirectCount2KHR(
    commandBuffer: vk::VkCommandBuffer,
    pInfo: *const vk::VkDrawIndirectCount2InfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDrawIndirectCount2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDrawIndirectCountAMD(
    commandBuffer: vk::VkCommandBuffer,
    buffer: vk::VkBuffer,
    offset: vk::VkDeviceSize,
    countBuffer: vk::VkBuffer,
    countBufferOffset: vk::VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDrawIndirectCountAMD;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            buffer,
            offset,
            countBuffer,
            countBufferOffset,
            maxDrawCount,
            stride,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDrawIndirectCountKHR(
    commandBuffer: vk::VkCommandBuffer,
    buffer: vk::VkBuffer,
    offset: vk::VkDeviceSize,
    countBuffer: vk::VkBuffer,
    countBufferOffset: vk::VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDrawIndirectCountKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            buffer,
            offset,
            countBuffer,
            countBufferOffset,
            maxDrawCount,
            stride,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDrawMeshTasksEXT(
    commandBuffer: vk::VkCommandBuffer,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDrawMeshTasksEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, groupCountX, groupCountY, groupCountZ);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDrawMeshTasksIndirect2EXT(
    commandBuffer: vk::VkCommandBuffer,
    pInfo: *const vk::VkDrawIndirect2InfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDrawMeshTasksIndirect2EXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDrawMeshTasksIndirectCount2EXT(
    commandBuffer: vk::VkCommandBuffer,
    pInfo: *const vk::VkDrawIndirectCount2InfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDrawMeshTasksIndirectCount2EXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDrawMeshTasksIndirectCountEXT(
    commandBuffer: vk::VkCommandBuffer,
    buffer: vk::VkBuffer,
    offset: vk::VkDeviceSize,
    countBuffer: vk::VkBuffer,
    countBufferOffset: vk::VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDrawMeshTasksIndirectCountEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            buffer,
            offset,
            countBuffer,
            countBufferOffset,
            maxDrawCount,
            stride,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDrawMeshTasksIndirectCountNV(
    commandBuffer: vk::VkCommandBuffer,
    buffer: vk::VkBuffer,
    offset: vk::VkDeviceSize,
    countBuffer: vk::VkBuffer,
    countBufferOffset: vk::VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDrawMeshTasksIndirectCountNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            buffer,
            offset,
            countBuffer,
            countBufferOffset,
            maxDrawCount,
            stride,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDrawMeshTasksIndirectEXT(
    commandBuffer: vk::VkCommandBuffer,
    buffer: vk::VkBuffer,
    offset: vk::VkDeviceSize,
    drawCount: u32,
    stride: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDrawMeshTasksIndirectEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, buffer, offset, drawCount, stride);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDrawMeshTasksIndirectNV(
    commandBuffer: vk::VkCommandBuffer,
    buffer: vk::VkBuffer,
    offset: vk::VkDeviceSize,
    drawCount: u32,
    stride: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDrawMeshTasksIndirectNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, buffer, offset, drawCount, stride);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDrawMeshTasksNV(
    commandBuffer: vk::VkCommandBuffer,
    taskCount: u32,
    firstTask: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDrawMeshTasksNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, taskCount, firstTask);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDrawMultiEXT(
    commandBuffer: vk::VkCommandBuffer,
    drawCount: u32,
    pVertexInfo: *const vk::VkMultiDrawInfoEXT,
    instanceCount: u32,
    firstInstance: u32,
    stride: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDrawMultiEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            drawCount,
            pVertexInfo,
            instanceCount,
            firstInstance,
            stride,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdDrawMultiIndexedEXT(
    commandBuffer: vk::VkCommandBuffer,
    drawCount: u32,
    pIndexInfo: *const vk::VkMultiDrawIndexedInfoEXT,
    instanceCount: u32,
    firstInstance: u32,
    stride: u32,
    pVertexOffset: *const i32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdDrawMultiIndexedEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            drawCount,
            pIndexInfo,
            instanceCount,
            firstInstance,
            stride,
            pVertexOffset,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdEncodeVideoKHR(
    commandBuffer: vk::VkCommandBuffer,
    pEncodeInfo: *const vk::VkVideoEncodeInfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdEncodeVideoKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pEncodeInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdEndConditionalRenderingEXT(
    commandBuffer: vk::VkCommandBuffer,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdEndConditionalRenderingEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdEndDebugUtilsLabelEXT(
    commandBuffer: vk::VkCommandBuffer,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdEndDebugUtilsLabelEXT;
    if let Some(command) = command {
        unsafe {
            command(commandBuffer);
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdEndGpaSampleAMD(
    commandBuffer: vk::VkCommandBuffer,
    gpaSession: vk::VkGpaSessionAMD,
    sampleID: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdEndGpaSampleAMD;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, gpaSession, sampleID);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdEndGpaSessionAMD(
    commandBuffer: vk::VkCommandBuffer,
    gpaSession: vk::VkGpaSessionAMD,
) -> vk::VkResult {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdEndGpaSessionAMD;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(commandBuffer, gpaSession) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdEndPerTileExecutionQCOM(
    commandBuffer: vk::VkCommandBuffer,
    pPerTileEndInfo: *const vk::VkPerTileEndInfoQCOM<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdEndPerTileExecutionQCOM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pPerTileEndInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdEndQuery(
    commandBuffer: vk::VkCommandBuffer,
    queryPool: vk::VkQueryPool,
    query: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdEndQuery;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, queryPool, query);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdEndQueryIndexedEXT(
    commandBuffer: vk::VkCommandBuffer,
    queryPool: vk::VkQueryPool,
    query: u32,
    index: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdEndQueryIndexedEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, queryPool, query, index);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdEndRenderPass(commandBuffer: vk::VkCommandBuffer) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdEndRenderPass;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdEndRenderPass2(
    commandBuffer: vk::VkCommandBuffer,
    pSubpassEndInfo: *const vk::VkSubpassEndInfo<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdEndRenderPass2;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pSubpassEndInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdEndRenderPass2KHR(
    commandBuffer: vk::VkCommandBuffer,
    pSubpassEndInfo: *const vk::VkSubpassEndInfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdEndRenderPass2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pSubpassEndInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdEndRendering(commandBuffer: vk::VkCommandBuffer) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdEndRendering;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdEndRendering2EXT(
    commandBuffer: vk::VkCommandBuffer,
    pRenderingEndInfo: *const vk::VkRenderingEndInfoEXT<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdEndRendering2EXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pRenderingEndInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdEndRendering2KHR(
    commandBuffer: vk::VkCommandBuffer,
    pRenderingEndInfo: *const vk::VkRenderingEndInfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdEndRendering2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pRenderingEndInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdEndRenderingKHR(commandBuffer: vk::VkCommandBuffer) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdEndRenderingKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdEndShaderInstrumentationARM(
    commandBuffer: vk::VkCommandBuffer,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdEndShaderInstrumentationARM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdEndTransformFeedback2EXT(
    commandBuffer: vk::VkCommandBuffer,
    firstCounterRange: u32,
    counterRangeCount: u32,
    pCounterInfos: *const vk::VkBindTransformFeedbackBuffer2InfoEXT<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdEndTransformFeedback2EXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            firstCounterRange,
            counterRangeCount,
            pCounterInfos,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdEndTransformFeedbackEXT(
    commandBuffer: vk::VkCommandBuffer,
    firstCounterBuffer: u32,
    counterBufferCount: u32,
    pCounterBuffers: *const vk::VkBuffer,
    pCounterBufferOffsets: *const vk::VkDeviceSize,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdEndTransformFeedbackEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            firstCounterBuffer,
            counterBufferCount,
            pCounterBuffers,
            pCounterBufferOffsets,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdEndVideoCodingKHR(
    commandBuffer: vk::VkCommandBuffer,
    pEndCodingInfo: *const vk::VkVideoEndCodingInfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdEndVideoCodingKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pEndCodingInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdExecuteCommands(
    commandBuffer: vk::VkCommandBuffer,
    commandBufferCount: u32,
    pCommandBuffers: *const vk::VkCommandBuffer,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdExecuteCommands;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, commandBufferCount, pCommandBuffers);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdExecuteGeneratedCommandsEXT(
    commandBuffer: vk::VkCommandBuffer,
    isPreprocessed: vk::VkBool32,
    pGeneratedCommandsInfo: *const vk::VkGeneratedCommandsInfoEXT<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdExecuteGeneratedCommandsEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, isPreprocessed, pGeneratedCommandsInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdExecuteGeneratedCommandsNV(
    commandBuffer: vk::VkCommandBuffer,
    isPreprocessed: vk::VkBool32,
    pGeneratedCommandsInfo: *const vk::VkGeneratedCommandsInfoNV<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdExecuteGeneratedCommandsNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, isPreprocessed, pGeneratedCommandsInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdFillBuffer(
    commandBuffer: vk::VkCommandBuffer,
    dstBuffer: vk::VkBuffer,
    dstOffset: vk::VkDeviceSize,
    size: vk::VkDeviceSize,
    data: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdFillBuffer;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, dstBuffer, dstOffset, size, data);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdFillMemoryKHR(
    commandBuffer: vk::VkCommandBuffer,
    pDstRange: *const vk::VkDeviceAddressRangeKHR,
    dstFlags: vk::VkAddressCommandFlagsKHR,
    data: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdFillMemoryKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pDstRange, dstFlags, data);
    }
}
#[cfg(feature = "beta-extensions")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdInitializeGraphScratchMemoryAMDX(
    commandBuffer: vk::VkCommandBuffer,
    executionGraph: vk::VkPipeline,
    scratch: vk::VkDeviceAddress,
    scratchSize: vk::VkDeviceSize,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdInitializeGraphScratchMemoryAMDX;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, executionGraph, scratch, scratchSize);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdInsertDebugUtilsLabelEXT(
    commandBuffer: vk::VkCommandBuffer,
    pLabelInfo: *const vk::VkDebugUtilsLabelEXT<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdInsertDebugUtilsLabelEXT;
    if let Some(command) = command {
        unsafe {
            command(commandBuffer, pLabelInfo);
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdNextSubpass(
    commandBuffer: vk::VkCommandBuffer,
    contents: vk::VkSubpassContents,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdNextSubpass;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, contents);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdNextSubpass2(
    commandBuffer: vk::VkCommandBuffer,
    pSubpassBeginInfo: *const vk::VkSubpassBeginInfo<'_>,
    pSubpassEndInfo: *const vk::VkSubpassEndInfo<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdNextSubpass2;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pSubpassBeginInfo, pSubpassEndInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdNextSubpass2KHR(
    commandBuffer: vk::VkCommandBuffer,
    pSubpassBeginInfo: *const vk::VkSubpassBeginInfoKHR<'_>,
    pSubpassEndInfo: *const vk::VkSubpassEndInfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdNextSubpass2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pSubpassBeginInfo, pSubpassEndInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdOpticalFlowExecuteNV(
    commandBuffer: vk::VkCommandBuffer,
    session: vk::VkOpticalFlowSessionNV,
    pExecuteInfo: *const vk::VkOpticalFlowExecuteInfoNV<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdOpticalFlowExecuteNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, session, pExecuteInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdPipelineBarrier(
    commandBuffer: vk::VkCommandBuffer,
    srcStageMask: vk::VkPipelineStageFlags,
    dstStageMask: vk::VkPipelineStageFlags,
    dependencyFlags: vk::VkDependencyFlags,
    memoryBarrierCount: u32,
    pMemoryBarriers: *const vk::VkMemoryBarrier<'_>,
    bufferMemoryBarrierCount: u32,
    pBufferMemoryBarriers: *const vk::VkBufferMemoryBarrier<'_>,
    imageMemoryBarrierCount: u32,
    pImageMemoryBarriers: *const vk::VkImageMemoryBarrier<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdPipelineBarrier;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            srcStageMask,
            dstStageMask,
            dependencyFlags,
            memoryBarrierCount,
            pMemoryBarriers,
            bufferMemoryBarrierCount,
            pBufferMemoryBarriers,
            imageMemoryBarrierCount,
            pImageMemoryBarriers,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdPipelineBarrier2(
    commandBuffer: vk::VkCommandBuffer,
    pDependencyInfo: *const vk::VkDependencyInfo<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdPipelineBarrier2;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pDependencyInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdPipelineBarrier2KHR(
    commandBuffer: vk::VkCommandBuffer,
    pDependencyInfo: *const vk::VkDependencyInfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdPipelineBarrier2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pDependencyInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdPreprocessGeneratedCommandsEXT(
    commandBuffer: vk::VkCommandBuffer,
    pGeneratedCommandsInfo: *const vk::VkGeneratedCommandsInfoEXT<'_>,
    stateCommandBuffer: vk::VkCommandBuffer,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdPreprocessGeneratedCommandsEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pGeneratedCommandsInfo, stateCommandBuffer);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdPreprocessGeneratedCommandsNV(
    commandBuffer: vk::VkCommandBuffer,
    pGeneratedCommandsInfo: *const vk::VkGeneratedCommandsInfoNV<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdPreprocessGeneratedCommandsNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pGeneratedCommandsInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdPushConstants(
    commandBuffer: vk::VkCommandBuffer,
    layout: vk::VkPipelineLayout,
    stageFlags: vk::VkShaderStageFlags,
    offset: u32,
    size: u32,
    pValues: *const c_void,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdPushConstants;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, layout, stageFlags, offset, size, pValues);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdPushConstants2(
    commandBuffer: vk::VkCommandBuffer,
    pPushConstantsInfo: *const vk::VkPushConstantsInfo<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdPushConstants2;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pPushConstantsInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdPushConstants2KHR(
    commandBuffer: vk::VkCommandBuffer,
    pPushConstantsInfo: *const vk::VkPushConstantsInfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdPushConstants2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pPushConstantsInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdPushDataEXT(
    commandBuffer: vk::VkCommandBuffer,
    pPushDataInfo: *const vk::VkPushDataInfoEXT<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdPushDataEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pPushDataInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdPushDescriptorSet(
    commandBuffer: vk::VkCommandBuffer,
    pipelineBindPoint: vk::VkPipelineBindPoint,
    layout: vk::VkPipelineLayout,
    set: u32,
    descriptorWriteCount: u32,
    pDescriptorWrites: *const vk::VkWriteDescriptorSet<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdPushDescriptorSet;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            pipelineBindPoint,
            layout,
            set,
            descriptorWriteCount,
            pDescriptorWrites,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdPushDescriptorSet2(
    commandBuffer: vk::VkCommandBuffer,
    pPushDescriptorSetInfo: *const vk::VkPushDescriptorSetInfo<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdPushDescriptorSet2;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pPushDescriptorSetInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdPushDescriptorSet2KHR(
    commandBuffer: vk::VkCommandBuffer,
    pPushDescriptorSetInfo: *const vk::VkPushDescriptorSetInfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdPushDescriptorSet2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pPushDescriptorSetInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdPushDescriptorSetKHR(
    commandBuffer: vk::VkCommandBuffer,
    pipelineBindPoint: vk::VkPipelineBindPoint,
    layout: vk::VkPipelineLayout,
    set: u32,
    descriptorWriteCount: u32,
    pDescriptorWrites: *const vk::VkWriteDescriptorSet<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdPushDescriptorSetKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            pipelineBindPoint,
            layout,
            set,
            descriptorWriteCount,
            pDescriptorWrites,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdPushDescriptorSetWithTemplate(
    commandBuffer: vk::VkCommandBuffer,
    descriptorUpdateTemplate: vk::VkDescriptorUpdateTemplate,
    layout: vk::VkPipelineLayout,
    set: u32,
    pData: *const c_void,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdPushDescriptorSetWithTemplate;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, descriptorUpdateTemplate, layout, set, pData);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdPushDescriptorSetWithTemplate2(
    commandBuffer: vk::VkCommandBuffer,
    pPushDescriptorSetWithTemplateInfo: *const vk::VkPushDescriptorSetWithTemplateInfo<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdPushDescriptorSetWithTemplate2;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pPushDescriptorSetWithTemplateInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdPushDescriptorSetWithTemplate2KHR(
    commandBuffer: vk::VkCommandBuffer,
    pPushDescriptorSetWithTemplateInfo: *const vk::VkPushDescriptorSetWithTemplateInfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdPushDescriptorSetWithTemplate2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pPushDescriptorSetWithTemplateInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdPushDescriptorSetWithTemplateKHR(
    commandBuffer: vk::VkCommandBuffer,
    descriptorUpdateTemplate: vk::VkDescriptorUpdateTemplate,
    layout: vk::VkPipelineLayout,
    set: u32,
    pData: *const c_void,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdPushDescriptorSetWithTemplateKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, descriptorUpdateTemplate, layout, set, pData);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdResetEvent(
    commandBuffer: vk::VkCommandBuffer,
    event: vk::VkEvent,
    stageMask: vk::VkPipelineStageFlags,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdResetEvent;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, event, stageMask);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdResetEvent2(
    commandBuffer: vk::VkCommandBuffer,
    event: vk::VkEvent,
    stageMask: vk::VkPipelineStageFlags2,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdResetEvent2;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, event, stageMask);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdResetEvent2KHR(
    commandBuffer: vk::VkCommandBuffer,
    event: vk::VkEvent,
    stageMask: vk::VkPipelineStageFlags2KHR,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdResetEvent2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, event, stageMask);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdResetQueryPool(
    commandBuffer: vk::VkCommandBuffer,
    queryPool: vk::VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdResetQueryPool;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, queryPool, firstQuery, queryCount);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdResolveImage(
    commandBuffer: vk::VkCommandBuffer,
    srcImage: vk::VkImage,
    srcImageLayout: vk::VkImageLayout,
    dstImage: vk::VkImage,
    dstImageLayout: vk::VkImageLayout,
    regionCount: u32,
    pRegions: *const vk::VkImageResolve,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdResolveImage;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            srcImage,
            srcImageLayout,
            dstImage,
            dstImageLayout,
            regionCount,
            pRegions,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdResolveImage2(
    commandBuffer: vk::VkCommandBuffer,
    pResolveImageInfo: *const vk::VkResolveImageInfo2<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdResolveImage2;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pResolveImageInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdResolveImage2KHR(
    commandBuffer: vk::VkCommandBuffer,
    pResolveImageInfo: *const vk::VkResolveImageInfo2KHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdResolveImage2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pResolveImageInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetAlphaToCoverageEnableEXT(
    commandBuffer: vk::VkCommandBuffer,
    alphaToCoverageEnable: vk::VkBool32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetAlphaToCoverageEnableEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, alphaToCoverageEnable);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetAlphaToOneEnableEXT(
    commandBuffer: vk::VkCommandBuffer,
    alphaToOneEnable: vk::VkBool32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetAlphaToOneEnableEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, alphaToOneEnable);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetAttachmentFeedbackLoopEnableEXT(
    commandBuffer: vk::VkCommandBuffer,
    aspectMask: vk::VkImageAspectFlags,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetAttachmentFeedbackLoopEnableEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, aspectMask);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdSetBlendConstants(
    commandBuffer: vk::VkCommandBuffer,
    blendConstants: &[f32; 4],
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetBlendConstants;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, blendConstants);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetCheckpointNV(
    commandBuffer: vk::VkCommandBuffer,
    pCheckpointMarker: *const c_void,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetCheckpointNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pCheckpointMarker);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetCoarseSampleOrderNV(
    commandBuffer: vk::VkCommandBuffer,
    sampleOrderType: vk::VkCoarseSampleOrderTypeNV,
    customSampleOrderCount: u32,
    pCustomSampleOrders: *const vk::VkCoarseSampleOrderCustomNV<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetCoarseSampleOrderNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            sampleOrderType,
            customSampleOrderCount,
            pCustomSampleOrders,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetColorBlendAdvancedEXT(
    commandBuffer: vk::VkCommandBuffer,
    firstAttachment: u32,
    attachmentCount: u32,
    pColorBlendAdvanced: *const vk::VkColorBlendAdvancedEXT,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetColorBlendAdvancedEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            firstAttachment,
            attachmentCount,
            pColorBlendAdvanced,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetColorBlendEnableEXT(
    commandBuffer: vk::VkCommandBuffer,
    firstAttachment: u32,
    attachmentCount: u32,
    pColorBlendEnables: *const vk::VkBool32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetColorBlendEnableEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            firstAttachment,
            attachmentCount,
            pColorBlendEnables,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetColorBlendEquationEXT(
    commandBuffer: vk::VkCommandBuffer,
    firstAttachment: u32,
    attachmentCount: u32,
    pColorBlendEquations: *const vk::VkColorBlendEquationEXT,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetColorBlendEquationEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            firstAttachment,
            attachmentCount,
            pColorBlendEquations,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetColorWriteEnableEXT(
    commandBuffer: vk::VkCommandBuffer,
    attachmentCount: u32,
    pColorWriteEnables: *const vk::VkBool32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetColorWriteEnableEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, attachmentCount, pColorWriteEnables);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetColorWriteMaskEXT(
    commandBuffer: vk::VkCommandBuffer,
    firstAttachment: u32,
    attachmentCount: u32,
    pColorWriteMasks: *const vk::VkColorComponentFlags,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetColorWriteMaskEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            firstAttachment,
            attachmentCount,
            pColorWriteMasks,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetComputeOccupancyPriorityNV(
    commandBuffer: vk::VkCommandBuffer,
    pParameters: *const vk::VkComputeOccupancyPriorityParametersNV<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetComputeOccupancyPriorityNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pParameters);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetConservativeRasterizationModeEXT(
    commandBuffer: vk::VkCommandBuffer,
    conservativeRasterizationMode: vk::VkConservativeRasterizationModeEXT,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetConservativeRasterizationModeEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, conservativeRasterizationMode);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetCoverageModulationModeNV(
    commandBuffer: vk::VkCommandBuffer,
    coverageModulationMode: vk::VkCoverageModulationModeNV,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetCoverageModulationModeNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, coverageModulationMode);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetCoverageModulationTableEnableNV(
    commandBuffer: vk::VkCommandBuffer,
    coverageModulationTableEnable: vk::VkBool32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetCoverageModulationTableEnableNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, coverageModulationTableEnable);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetCoverageModulationTableNV(
    commandBuffer: vk::VkCommandBuffer,
    coverageModulationTableCount: u32,
    pCoverageModulationTable: *const f32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetCoverageModulationTableNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            coverageModulationTableCount,
            pCoverageModulationTable,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetCoverageReductionModeNV(
    commandBuffer: vk::VkCommandBuffer,
    coverageReductionMode: vk::VkCoverageReductionModeNV,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetCoverageReductionModeNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, coverageReductionMode);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetCoverageToColorEnableNV(
    commandBuffer: vk::VkCommandBuffer,
    coverageToColorEnable: vk::VkBool32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetCoverageToColorEnableNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, coverageToColorEnable);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetCoverageToColorLocationNV(
    commandBuffer: vk::VkCommandBuffer,
    coverageToColorLocation: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetCoverageToColorLocationNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, coverageToColorLocation);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdSetCullMode(
    commandBuffer: vk::VkCommandBuffer,
    cullMode: vk::VkCullModeFlags,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetCullMode;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, cullMode);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetCullModeEXT(
    commandBuffer: vk::VkCommandBuffer,
    cullMode: vk::VkCullModeFlags,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetCullModeEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, cullMode);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdSetDepthBias(
    commandBuffer: vk::VkCommandBuffer,
    depthBiasConstantFactor: f32,
    depthBiasClamp: f32,
    depthBiasSlopeFactor: f32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetDepthBias;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            depthBiasConstantFactor,
            depthBiasClamp,
            depthBiasSlopeFactor,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetDepthBias2EXT(
    commandBuffer: vk::VkCommandBuffer,
    pDepthBiasInfo: *const vk::VkDepthBiasInfoEXT<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetDepthBias2EXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pDepthBiasInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdSetDepthBiasEnable(
    commandBuffer: vk::VkCommandBuffer,
    depthBiasEnable: vk::VkBool32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetDepthBiasEnable;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, depthBiasEnable);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetDepthBiasEnableEXT(
    commandBuffer: vk::VkCommandBuffer,
    depthBiasEnable: vk::VkBool32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetDepthBiasEnableEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, depthBiasEnable);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdSetDepthBounds(
    commandBuffer: vk::VkCommandBuffer,
    minDepthBounds: f32,
    maxDepthBounds: f32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetDepthBounds;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, minDepthBounds, maxDepthBounds);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdSetDepthBoundsTestEnable(
    commandBuffer: vk::VkCommandBuffer,
    depthBoundsTestEnable: vk::VkBool32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetDepthBoundsTestEnable;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, depthBoundsTestEnable);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetDepthBoundsTestEnableEXT(
    commandBuffer: vk::VkCommandBuffer,
    depthBoundsTestEnable: vk::VkBool32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetDepthBoundsTestEnableEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, depthBoundsTestEnable);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetDepthClampEnableEXT(
    commandBuffer: vk::VkCommandBuffer,
    depthClampEnable: vk::VkBool32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetDepthClampEnableEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, depthClampEnable);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetDepthClampRangeEXT(
    commandBuffer: vk::VkCommandBuffer,
    depthClampMode: vk::VkDepthClampModeEXT,
    pDepthClampRange: *const vk::VkDepthClampRangeEXT,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetDepthClampRangeEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, depthClampMode, pDepthClampRange);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetDepthClipEnableEXT(
    commandBuffer: vk::VkCommandBuffer,
    depthClipEnable: vk::VkBool32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetDepthClipEnableEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, depthClipEnable);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetDepthClipNegativeOneToOneEXT(
    commandBuffer: vk::VkCommandBuffer,
    negativeOneToOne: vk::VkBool32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetDepthClipNegativeOneToOneEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, negativeOneToOne);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdSetDepthCompareOp(
    commandBuffer: vk::VkCommandBuffer,
    depthCompareOp: vk::VkCompareOp,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetDepthCompareOp;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, depthCompareOp);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetDepthCompareOpEXT(
    commandBuffer: vk::VkCommandBuffer,
    depthCompareOp: vk::VkCompareOp,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetDepthCompareOpEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, depthCompareOp);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdSetDepthTestEnable(
    commandBuffer: vk::VkCommandBuffer,
    depthTestEnable: vk::VkBool32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetDepthTestEnable;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, depthTestEnable);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetDepthTestEnableEXT(
    commandBuffer: vk::VkCommandBuffer,
    depthTestEnable: vk::VkBool32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetDepthTestEnableEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, depthTestEnable);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdSetDepthWriteEnable(
    commandBuffer: vk::VkCommandBuffer,
    depthWriteEnable: vk::VkBool32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetDepthWriteEnable;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, depthWriteEnable);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetDepthWriteEnableEXT(
    commandBuffer: vk::VkCommandBuffer,
    depthWriteEnable: vk::VkBool32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetDepthWriteEnableEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, depthWriteEnable);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetDescriptorBufferOffsets2EXT(
    commandBuffer: vk::VkCommandBuffer,
    pSetDescriptorBufferOffsetsInfo: *const vk::VkSetDescriptorBufferOffsetsInfoEXT<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetDescriptorBufferOffsets2EXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pSetDescriptorBufferOffsetsInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetDescriptorBufferOffsetsEXT(
    commandBuffer: vk::VkCommandBuffer,
    pipelineBindPoint: vk::VkPipelineBindPoint,
    layout: vk::VkPipelineLayout,
    firstSet: u32,
    setCount: u32,
    pBufferIndices: *const u32,
    pOffsets: *const vk::VkDeviceSize,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetDescriptorBufferOffsetsEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            pipelineBindPoint,
            layout,
            firstSet,
            setCount,
            pBufferIndices,
            pOffsets,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdSetDeviceMask(
    commandBuffer: vk::VkCommandBuffer,
    deviceMask: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetDeviceMask;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, deviceMask);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetDeviceMaskKHR(
    commandBuffer: vk::VkCommandBuffer,
    deviceMask: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetDeviceMaskKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, deviceMask);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetDiscardRectangleEXT(
    commandBuffer: vk::VkCommandBuffer,
    firstDiscardRectangle: u32,
    discardRectangleCount: u32,
    pDiscardRectangles: *const vk::VkRect2D,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetDiscardRectangleEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            firstDiscardRectangle,
            discardRectangleCount,
            pDiscardRectangles,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetDiscardRectangleEnableEXT(
    commandBuffer: vk::VkCommandBuffer,
    discardRectangleEnable: vk::VkBool32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetDiscardRectangleEnableEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, discardRectangleEnable);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetDiscardRectangleModeEXT(
    commandBuffer: vk::VkCommandBuffer,
    discardRectangleMode: vk::VkDiscardRectangleModeEXT,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetDiscardRectangleModeEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, discardRectangleMode);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetDispatchParametersARM(
    commandBuffer: vk::VkCommandBuffer,
    pDispatchParameters: *const vk::VkDispatchParametersARM<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetDispatchParametersARM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pDispatchParameters);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdSetEvent(
    commandBuffer: vk::VkCommandBuffer,
    event: vk::VkEvent,
    stageMask: vk::VkPipelineStageFlags,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetEvent;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, event, stageMask);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdSetEvent2(
    commandBuffer: vk::VkCommandBuffer,
    event: vk::VkEvent,
    pDependencyInfo: *const vk::VkDependencyInfo<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetEvent2;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, event, pDependencyInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetEvent2KHR(
    commandBuffer: vk::VkCommandBuffer,
    event: vk::VkEvent,
    pDependencyInfo: *const vk::VkDependencyInfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetEvent2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, event, pDependencyInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetExclusiveScissorEnableNV(
    commandBuffer: vk::VkCommandBuffer,
    firstExclusiveScissor: u32,
    exclusiveScissorCount: u32,
    pExclusiveScissorEnables: *const vk::VkBool32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetExclusiveScissorEnableNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            firstExclusiveScissor,
            exclusiveScissorCount,
            pExclusiveScissorEnables,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetExclusiveScissorNV(
    commandBuffer: vk::VkCommandBuffer,
    firstExclusiveScissor: u32,
    exclusiveScissorCount: u32,
    pExclusiveScissors: *const vk::VkRect2D,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetExclusiveScissorNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            firstExclusiveScissor,
            exclusiveScissorCount,
            pExclusiveScissors,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetExtraPrimitiveOverestimationSizeEXT(
    commandBuffer: vk::VkCommandBuffer,
    extraPrimitiveOverestimationSize: f32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetExtraPrimitiveOverestimationSizeEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, extraPrimitiveOverestimationSize);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetFragmentShadingRateEnumNV(
    commandBuffer: vk::VkCommandBuffer,
    shadingRate: vk::VkFragmentShadingRateNV,
    combinerOps: &[vk::VkFragmentShadingRateCombinerOpKHR; 2],
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetFragmentShadingRateEnumNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, shadingRate, combinerOps);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetFragmentShadingRateKHR(
    commandBuffer: vk::VkCommandBuffer,
    pFragmentSize: *const vk::VkExtent2D,
    combinerOps: &[vk::VkFragmentShadingRateCombinerOpKHR; 2],
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetFragmentShadingRateKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pFragmentSize, combinerOps);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdSetFrontFace(
    commandBuffer: vk::VkCommandBuffer,
    frontFace: vk::VkFrontFace,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetFrontFace;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, frontFace);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetFrontFaceEXT(
    commandBuffer: vk::VkCommandBuffer,
    frontFace: vk::VkFrontFace,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetFrontFaceEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, frontFace);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetLineRasterizationModeEXT(
    commandBuffer: vk::VkCommandBuffer,
    lineRasterizationMode: vk::VkLineRasterizationModeEXT,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetLineRasterizationModeEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, lineRasterizationMode);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdSetLineStipple(
    commandBuffer: vk::VkCommandBuffer,
    lineStippleFactor: u32,
    lineStipplePattern: u16,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetLineStipple;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, lineStippleFactor, lineStipplePattern);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetLineStippleEXT(
    commandBuffer: vk::VkCommandBuffer,
    lineStippleFactor: u32,
    lineStipplePattern: u16,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetLineStippleEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, lineStippleFactor, lineStipplePattern);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetLineStippleEnableEXT(
    commandBuffer: vk::VkCommandBuffer,
    stippledLineEnable: vk::VkBool32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetLineStippleEnableEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, stippledLineEnable);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetLineStippleKHR(
    commandBuffer: vk::VkCommandBuffer,
    lineStippleFactor: u32,
    lineStipplePattern: u16,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetLineStippleKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, lineStippleFactor, lineStipplePattern);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdSetLineWidth(
    commandBuffer: vk::VkCommandBuffer,
    lineWidth: f32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetLineWidth;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, lineWidth);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetLogicOpEXT(
    commandBuffer: vk::VkCommandBuffer,
    logicOp: vk::VkLogicOp,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetLogicOpEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, logicOp);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetLogicOpEnableEXT(
    commandBuffer: vk::VkCommandBuffer,
    logicOpEnable: vk::VkBool32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetLogicOpEnableEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, logicOpEnable);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetPatchControlPointsEXT(
    commandBuffer: vk::VkCommandBuffer,
    patchControlPoints: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetPatchControlPointsEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, patchControlPoints);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetPerformanceMarkerINTEL(
    commandBuffer: vk::VkCommandBuffer,
    pMarkerInfo: *const vk::VkPerformanceMarkerInfoINTEL<'_>,
) -> vk::VkResult {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetPerformanceMarkerINTEL;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(commandBuffer, pMarkerInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetPerformanceOverrideINTEL(
    commandBuffer: vk::VkCommandBuffer,
    pOverrideInfo: *const vk::VkPerformanceOverrideInfoINTEL<'_>,
) -> vk::VkResult {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetPerformanceOverrideINTEL;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(commandBuffer, pOverrideInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetPerformanceStreamMarkerINTEL(
    commandBuffer: vk::VkCommandBuffer,
    pMarkerInfo: *const vk::VkPerformanceStreamMarkerInfoINTEL<'_>,
) -> vk::VkResult {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetPerformanceStreamMarkerINTEL;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(commandBuffer, pMarkerInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetPolygonModeEXT(
    commandBuffer: vk::VkCommandBuffer,
    polygonMode: vk::VkPolygonMode,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetPolygonModeEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, polygonMode);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdSetPrimitiveRestartEnable(
    commandBuffer: vk::VkCommandBuffer,
    primitiveRestartEnable: vk::VkBool32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetPrimitiveRestartEnable;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, primitiveRestartEnable);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetPrimitiveRestartEnableEXT(
    commandBuffer: vk::VkCommandBuffer,
    primitiveRestartEnable: vk::VkBool32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetPrimitiveRestartEnableEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, primitiveRestartEnable);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetPrimitiveRestartIndexEXT(
    commandBuffer: vk::VkCommandBuffer,
    primitiveRestartIndex: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetPrimitiveRestartIndexEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, primitiveRestartIndex);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdSetPrimitiveTopology(
    commandBuffer: vk::VkCommandBuffer,
    primitiveTopology: vk::VkPrimitiveTopology,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetPrimitiveTopology;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, primitiveTopology);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetPrimitiveTopologyEXT(
    commandBuffer: vk::VkCommandBuffer,
    primitiveTopology: vk::VkPrimitiveTopology,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetPrimitiveTopologyEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, primitiveTopology);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetProvokingVertexModeEXT(
    commandBuffer: vk::VkCommandBuffer,
    provokingVertexMode: vk::VkProvokingVertexModeEXT,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetProvokingVertexModeEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, provokingVertexMode);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetRasterizationSamplesEXT(
    commandBuffer: vk::VkCommandBuffer,
    rasterizationSamples: vk::VkSampleCountFlagBits,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetRasterizationSamplesEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, rasterizationSamples);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetRasterizationStreamEXT(
    commandBuffer: vk::VkCommandBuffer,
    rasterizationStream: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetRasterizationStreamEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, rasterizationStream);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdSetRasterizerDiscardEnable(
    commandBuffer: vk::VkCommandBuffer,
    rasterizerDiscardEnable: vk::VkBool32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetRasterizerDiscardEnable;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, rasterizerDiscardEnable);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetRasterizerDiscardEnableEXT(
    commandBuffer: vk::VkCommandBuffer,
    rasterizerDiscardEnable: vk::VkBool32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetRasterizerDiscardEnableEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, rasterizerDiscardEnable);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetRayTracingPipelineStackSizeKHR(
    commandBuffer: vk::VkCommandBuffer,
    pipelineStackSize: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetRayTracingPipelineStackSizeKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pipelineStackSize);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdSetRenderingAttachmentLocations(
    commandBuffer: vk::VkCommandBuffer,
    pLocationInfo: *const vk::VkRenderingAttachmentLocationInfo<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetRenderingAttachmentLocations;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pLocationInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetRenderingAttachmentLocationsKHR(
    commandBuffer: vk::VkCommandBuffer,
    pLocationInfo: *const vk::VkRenderingAttachmentLocationInfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetRenderingAttachmentLocationsKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pLocationInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdSetRenderingInputAttachmentIndices(
    commandBuffer: vk::VkCommandBuffer,
    pInputAttachmentIndexInfo: *const vk::VkRenderingInputAttachmentIndexInfo<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetRenderingInputAttachmentIndices;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pInputAttachmentIndexInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetRenderingInputAttachmentIndicesKHR(
    commandBuffer: vk::VkCommandBuffer,
    pInputAttachmentIndexInfo: *const vk::VkRenderingInputAttachmentIndexInfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetRenderingInputAttachmentIndicesKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pInputAttachmentIndexInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetRepresentativeFragmentTestEnableNV(
    commandBuffer: vk::VkCommandBuffer,
    representativeFragmentTestEnable: vk::VkBool32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetRepresentativeFragmentTestEnableNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, representativeFragmentTestEnable);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetSampleLocationsEXT(
    commandBuffer: vk::VkCommandBuffer,
    pSampleLocationsInfo: *const vk::VkSampleLocationsInfoEXT<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetSampleLocationsEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pSampleLocationsInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetSampleLocationsEnableEXT(
    commandBuffer: vk::VkCommandBuffer,
    sampleLocationsEnable: vk::VkBool32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetSampleLocationsEnableEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, sampleLocationsEnable);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetSampleMaskEXT(
    commandBuffer: vk::VkCommandBuffer,
    samples: vk::VkSampleCountFlagBits,
    pSampleMask: *const vk::VkSampleMask,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetSampleMaskEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, samples, pSampleMask);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdSetScissor(
    commandBuffer: vk::VkCommandBuffer,
    firstScissor: u32,
    scissorCount: u32,
    pScissors: *const vk::VkRect2D,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetScissor;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, firstScissor, scissorCount, pScissors);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdSetScissorWithCount(
    commandBuffer: vk::VkCommandBuffer,
    scissorCount: u32,
    pScissors: *const vk::VkRect2D,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetScissorWithCount;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, scissorCount, pScissors);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetScissorWithCountEXT(
    commandBuffer: vk::VkCommandBuffer,
    scissorCount: u32,
    pScissors: *const vk::VkRect2D,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetScissorWithCountEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, scissorCount, pScissors);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetShadingRateImageEnableNV(
    commandBuffer: vk::VkCommandBuffer,
    shadingRateImageEnable: vk::VkBool32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetShadingRateImageEnableNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, shadingRateImageEnable);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdSetStencilCompareMask(
    commandBuffer: vk::VkCommandBuffer,
    faceMask: vk::VkStencilFaceFlags,
    compareMask: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetStencilCompareMask;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, faceMask, compareMask);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdSetStencilOp(
    commandBuffer: vk::VkCommandBuffer,
    faceMask: vk::VkStencilFaceFlags,
    failOp: vk::VkStencilOp,
    passOp: vk::VkStencilOp,
    depthFailOp: vk::VkStencilOp,
    compareOp: vk::VkCompareOp,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetStencilOp;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            faceMask,
            failOp,
            passOp,
            depthFailOp,
            compareOp,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetStencilOpEXT(
    commandBuffer: vk::VkCommandBuffer,
    faceMask: vk::VkStencilFaceFlags,
    failOp: vk::VkStencilOp,
    passOp: vk::VkStencilOp,
    depthFailOp: vk::VkStencilOp,
    compareOp: vk::VkCompareOp,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetStencilOpEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            faceMask,
            failOp,
            passOp,
            depthFailOp,
            compareOp,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdSetStencilReference(
    commandBuffer: vk::VkCommandBuffer,
    faceMask: vk::VkStencilFaceFlags,
    reference: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetStencilReference;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, faceMask, reference);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdSetStencilTestEnable(
    commandBuffer: vk::VkCommandBuffer,
    stencilTestEnable: vk::VkBool32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetStencilTestEnable;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, stencilTestEnable);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetStencilTestEnableEXT(
    commandBuffer: vk::VkCommandBuffer,
    stencilTestEnable: vk::VkBool32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetStencilTestEnableEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, stencilTestEnable);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdSetStencilWriteMask(
    commandBuffer: vk::VkCommandBuffer,
    faceMask: vk::VkStencilFaceFlags,
    writeMask: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetStencilWriteMask;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, faceMask, writeMask);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetTessellationDomainOriginEXT(
    commandBuffer: vk::VkCommandBuffer,
    domainOrigin: vk::VkTessellationDomainOrigin,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetTessellationDomainOriginEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, domainOrigin);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetVertexInputEXT(
    commandBuffer: vk::VkCommandBuffer,
    vertexBindingDescriptionCount: u32,
    pVertexBindingDescriptions: *const vk::VkVertexInputBindingDescription2EXT<'_>,
    vertexAttributeDescriptionCount: u32,
    pVertexAttributeDescriptions: *const vk::VkVertexInputAttributeDescription2EXT<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetVertexInputEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            vertexBindingDescriptionCount,
            pVertexBindingDescriptions,
            vertexAttributeDescriptionCount,
            pVertexAttributeDescriptions,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdSetViewport(
    commandBuffer: vk::VkCommandBuffer,
    firstViewport: u32,
    viewportCount: u32,
    pViewports: *const vk::VkViewport,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetViewport;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, firstViewport, viewportCount, pViewports);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetViewportShadingRatePaletteNV(
    commandBuffer: vk::VkCommandBuffer,
    firstViewport: u32,
    viewportCount: u32,
    pShadingRatePalettes: *const vk::VkShadingRatePaletteNV<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetViewportShadingRatePaletteNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            firstViewport,
            viewportCount,
            pShadingRatePalettes,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetViewportSwizzleNV(
    commandBuffer: vk::VkCommandBuffer,
    firstViewport: u32,
    viewportCount: u32,
    pViewportSwizzles: *const vk::VkViewportSwizzleNV,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetViewportSwizzleNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            firstViewport,
            viewportCount,
            pViewportSwizzles,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetViewportWScalingEnableNV(
    commandBuffer: vk::VkCommandBuffer,
    viewportWScalingEnable: vk::VkBool32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetViewportWScalingEnableNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, viewportWScalingEnable);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetViewportWScalingNV(
    commandBuffer: vk::VkCommandBuffer,
    firstViewport: u32,
    viewportCount: u32,
    pViewportWScalings: *const vk::VkViewportWScalingNV,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetViewportWScalingNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            firstViewport,
            viewportCount,
            pViewportWScalings,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdSetViewportWithCount(
    commandBuffer: vk::VkCommandBuffer,
    viewportCount: u32,
    pViewports: *const vk::VkViewport,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetViewportWithCount;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, viewportCount, pViewports);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSetViewportWithCountEXT(
    commandBuffer: vk::VkCommandBuffer,
    viewportCount: u32,
    pViewports: *const vk::VkViewport,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSetViewportWithCountEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, viewportCount, pViewports);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdSubpassShadingHUAWEI(commandBuffer: vk::VkCommandBuffer) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdSubpassShadingHUAWEI;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdTraceRaysIndirect2KHR(
    commandBuffer: vk::VkCommandBuffer,
    indirectDeviceAddress: vk::VkDeviceAddress,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdTraceRaysIndirect2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, indirectDeviceAddress);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdTraceRaysIndirectKHR(
    commandBuffer: vk::VkCommandBuffer,
    pRaygenShaderBindingTable: *const vk::VkStridedDeviceAddressRegionKHR,
    pMissShaderBindingTable: *const vk::VkStridedDeviceAddressRegionKHR,
    pHitShaderBindingTable: *const vk::VkStridedDeviceAddressRegionKHR,
    pCallableShaderBindingTable: *const vk::VkStridedDeviceAddressRegionKHR,
    indirectDeviceAddress: vk::VkDeviceAddress,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdTraceRaysIndirectKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            pRaygenShaderBindingTable,
            pMissShaderBindingTable,
            pHitShaderBindingTable,
            pCallableShaderBindingTable,
            indirectDeviceAddress,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdTraceRaysKHR(
    commandBuffer: vk::VkCommandBuffer,
    pRaygenShaderBindingTable: *const vk::VkStridedDeviceAddressRegionKHR,
    pMissShaderBindingTable: *const vk::VkStridedDeviceAddressRegionKHR,
    pHitShaderBindingTable: *const vk::VkStridedDeviceAddressRegionKHR,
    pCallableShaderBindingTable: *const vk::VkStridedDeviceAddressRegionKHR,
    width: u32,
    height: u32,
    depth: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdTraceRaysKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            pRaygenShaderBindingTable,
            pMissShaderBindingTable,
            pHitShaderBindingTable,
            pCallableShaderBindingTable,
            width,
            height,
            depth,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdTraceRaysNV(
    commandBuffer: vk::VkCommandBuffer,
    raygenShaderBindingTableBuffer: vk::VkBuffer,
    raygenShaderBindingOffset: vk::VkDeviceSize,
    missShaderBindingTableBuffer: vk::VkBuffer,
    missShaderBindingOffset: vk::VkDeviceSize,
    missShaderBindingStride: vk::VkDeviceSize,
    hitShaderBindingTableBuffer: vk::VkBuffer,
    hitShaderBindingOffset: vk::VkDeviceSize,
    hitShaderBindingStride: vk::VkDeviceSize,
    callableShaderBindingTableBuffer: vk::VkBuffer,
    callableShaderBindingOffset: vk::VkDeviceSize,
    callableShaderBindingStride: vk::VkDeviceSize,
    width: u32,
    height: u32,
    depth: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdTraceRaysNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            raygenShaderBindingTableBuffer,
            raygenShaderBindingOffset,
            missShaderBindingTableBuffer,
            missShaderBindingOffset,
            missShaderBindingStride,
            hitShaderBindingTableBuffer,
            hitShaderBindingOffset,
            hitShaderBindingStride,
            callableShaderBindingTableBuffer,
            callableShaderBindingOffset,
            callableShaderBindingStride,
            width,
            height,
            depth,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdUpdateBuffer(
    commandBuffer: vk::VkCommandBuffer,
    dstBuffer: vk::VkBuffer,
    dstOffset: vk::VkDeviceSize,
    dataSize: vk::VkDeviceSize,
    pData: *const c_void,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdUpdateBuffer;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, dstBuffer, dstOffset, dataSize, pData);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdUpdateMemoryKHR(
    commandBuffer: vk::VkCommandBuffer,
    pDstRange: *const vk::VkDeviceAddressRangeKHR,
    dstFlags: vk::VkAddressCommandFlagsKHR,
    dataSize: vk::VkDeviceSize,
    pData: *const c_void,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdUpdateMemoryKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pDstRange, dstFlags, dataSize, pData);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdUpdatePipelineIndirectBufferNV(
    commandBuffer: vk::VkCommandBuffer,
    pipelineBindPoint: vk::VkPipelineBindPoint,
    pipeline: vk::VkPipeline,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdUpdatePipelineIndirectBufferNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pipelineBindPoint, pipeline);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdWaitEvents(
    commandBuffer: vk::VkCommandBuffer,
    eventCount: u32,
    pEvents: *const vk::VkEvent,
    srcStageMask: vk::VkPipelineStageFlags,
    dstStageMask: vk::VkPipelineStageFlags,
    memoryBarrierCount: u32,
    pMemoryBarriers: *const vk::VkMemoryBarrier<'_>,
    bufferMemoryBarrierCount: u32,
    pBufferMemoryBarriers: *const vk::VkBufferMemoryBarrier<'_>,
    imageMemoryBarrierCount: u32,
    pImageMemoryBarriers: *const vk::VkImageMemoryBarrier<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdWaitEvents;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            eventCount,
            pEvents,
            srcStageMask,
            dstStageMask,
            memoryBarrierCount,
            pMemoryBarriers,
            bufferMemoryBarrierCount,
            pBufferMemoryBarriers,
            imageMemoryBarrierCount,
            pImageMemoryBarriers,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdWaitEvents2(
    commandBuffer: vk::VkCommandBuffer,
    eventCount: u32,
    pEvents: *const vk::VkEvent,
    pDependencyInfos: *const vk::VkDependencyInfo<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdWaitEvents2;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, eventCount, pEvents, pDependencyInfos);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdWaitEvents2KHR(
    commandBuffer: vk::VkCommandBuffer,
    eventCount: u32,
    pEvents: *const vk::VkEvent,
    pDependencyInfos: *const vk::VkDependencyInfoKHR<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdWaitEvents2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, eventCount, pEvents, pDependencyInfos);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdWriteAccelerationStructuresPropertiesKHR(
    commandBuffer: vk::VkCommandBuffer,
    accelerationStructureCount: u32,
    pAccelerationStructures: *const vk::VkAccelerationStructureKHR,
    queryType: vk::VkQueryType,
    queryPool: vk::VkQueryPool,
    firstQuery: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdWriteAccelerationStructuresPropertiesKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            accelerationStructureCount,
            pAccelerationStructures,
            queryType,
            queryPool,
            firstQuery,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdWriteAccelerationStructuresPropertiesNV(
    commandBuffer: vk::VkCommandBuffer,
    accelerationStructureCount: u32,
    pAccelerationStructures: *const vk::VkAccelerationStructureNV,
    queryType: vk::VkQueryType,
    queryPool: vk::VkQueryPool,
    firstQuery: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdWriteAccelerationStructuresPropertiesNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            accelerationStructureCount,
            pAccelerationStructures,
            queryType,
            queryPool,
            firstQuery,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdWriteBufferMarker2AMD(
    commandBuffer: vk::VkCommandBuffer,
    stage: vk::VkPipelineStageFlags2,
    dstBuffer: vk::VkBuffer,
    dstOffset: vk::VkDeviceSize,
    marker: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdWriteBufferMarker2AMD;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, stage, dstBuffer, dstOffset, marker);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdWriteBufferMarkerAMD(
    commandBuffer: vk::VkCommandBuffer,
    pipelineStage: vk::VkPipelineStageFlagBits,
    dstBuffer: vk::VkBuffer,
    dstOffset: vk::VkDeviceSize,
    marker: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdWriteBufferMarkerAMD;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pipelineStage, dstBuffer, dstOffset, marker);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdWriteMarkerToMemoryAMD(
    commandBuffer: vk::VkCommandBuffer,
    pInfo: *const vk::VkMemoryMarkerInfoAMD<'_>,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdWriteMarkerToMemoryAMD;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdWriteMicromapsPropertiesEXT(
    commandBuffer: vk::VkCommandBuffer,
    micromapCount: u32,
    pMicromaps: *const vk::VkMicromapEXT,
    queryType: vk::VkQueryType,
    queryPool: vk::VkQueryPool,
    firstQuery: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdWriteMicromapsPropertiesEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            commandBuffer,
            micromapCount,
            pMicromaps,
            queryType,
            queryPool,
            firstQuery,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdWriteTimestamp(
    commandBuffer: vk::VkCommandBuffer,
    pipelineStage: vk::VkPipelineStageFlagBits,
    queryPool: vk::VkQueryPool,
    query: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdWriteTimestamp;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, pipelineStage, queryPool, query);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCmdWriteTimestamp2(
    commandBuffer: vk::VkCommandBuffer,
    stage: vk::VkPipelineStageFlags2,
    queryPool: vk::VkQueryPool,
    query: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdWriteTimestamp2;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, stage, queryPool, query);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCmdWriteTimestamp2KHR(
    commandBuffer: vk::VkCommandBuffer,
    stage: vk::VkPipelineStageFlags2KHR,
    queryPool: vk::VkQueryPool,
    query: u32,
) {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCmdWriteTimestamp2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(commandBuffer, stage, queryPool, query);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCompileDeferredNV(
    device: vk::VkDevice,
    pipeline: vk::VkPipeline,
    shader: u32,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCompileDeferredNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pipeline, shader) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkConvertCooperativeVectorMatrixNV(
    device: vk::VkDevice,
    pInfo: *const vk::VkConvertCooperativeVectorMatrixInfoNV<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkConvertCooperativeVectorMatrixNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCopyAccelerationStructureKHR(
    device: vk::VkDevice,
    deferredOperation: vk::VkDeferredOperationKHR,
    pInfo: *const vk::VkCopyAccelerationStructureInfoKHR<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCopyAccelerationStructureKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, deferredOperation, pInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCopyAccelerationStructureToMemoryKHR(
    device: vk::VkDevice,
    deferredOperation: vk::VkDeferredOperationKHR,
    pInfo: *const vk::VkCopyAccelerationStructureToMemoryInfoKHR<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCopyAccelerationStructureToMemoryKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, deferredOperation, pInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCopyImageToImage(
    device: vk::VkDevice,
    pCopyImageToImageInfo: *const vk::VkCopyImageToImageInfo<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCopyImageToImage;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCopyImageToImageInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCopyImageToImageEXT(
    device: vk::VkDevice,
    pCopyImageToImageInfo: *const vk::VkCopyImageToImageInfoEXT<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCopyImageToImageEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCopyImageToImageInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCopyImageToMemory(
    device: vk::VkDevice,
    pCopyImageToMemoryInfo: *const vk::VkCopyImageToMemoryInfo<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCopyImageToMemory;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCopyImageToMemoryInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCopyImageToMemoryEXT(
    device: vk::VkDevice,
    pCopyImageToMemoryInfo: *const vk::VkCopyImageToMemoryInfoEXT<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCopyImageToMemoryEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCopyImageToMemoryInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCopyMemoryToAccelerationStructureKHR(
    device: vk::VkDevice,
    deferredOperation: vk::VkDeferredOperationKHR,
    pInfo: *const vk::VkCopyMemoryToAccelerationStructureInfoKHR<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCopyMemoryToAccelerationStructureKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, deferredOperation, pInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCopyMemoryToImage(
    device: vk::VkDevice,
    pCopyMemoryToImageInfo: *const vk::VkCopyMemoryToImageInfo<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCopyMemoryToImage;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCopyMemoryToImageInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCopyMemoryToImageEXT(
    device: vk::VkDevice,
    pCopyMemoryToImageInfo: *const vk::VkCopyMemoryToImageInfoEXT<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCopyMemoryToImageEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCopyMemoryToImageInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCopyMemoryToMicromapEXT(
    device: vk::VkDevice,
    deferredOperation: vk::VkDeferredOperationKHR,
    pInfo: *const vk::VkCopyMemoryToMicromapInfoEXT<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCopyMemoryToMicromapEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, deferredOperation, pInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCopyMicromapEXT(
    device: vk::VkDevice,
    deferredOperation: vk::VkDeferredOperationKHR,
    pInfo: *const vk::VkCopyMicromapInfoEXT<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCopyMicromapEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, deferredOperation, pInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCopyMicromapToMemoryEXT(
    device: vk::VkDevice,
    deferredOperation: vk::VkDeferredOperationKHR,
    pInfo: *const vk::VkCopyMicromapToMemoryInfoEXT<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCopyMicromapToMemoryEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, deferredOperation, pInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateAccelerationStructure2KHR(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkAccelerationStructureCreateInfo2KHR<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pAccelerationStructure: *mut vk::VkAccelerationStructureKHR,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateAccelerationStructure2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pAccelerationStructure) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateAccelerationStructureKHR(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkAccelerationStructureCreateInfoKHR<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pAccelerationStructure: *mut vk::VkAccelerationStructureKHR,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateAccelerationStructureKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pAccelerationStructure) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateAccelerationStructureNV(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkAccelerationStructureCreateInfoNV<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pAccelerationStructure: *mut vk::VkAccelerationStructureNV,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateAccelerationStructureNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pAccelerationStructure) }
}
#[cfg(target_os = "android")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateAndroidSurfaceKHR(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkAndroidSurfaceCreateInfoKHR<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    let loader = unsafe { LoaderInstance::from_handle(instance) }
        .unwrap_or_else(|| fatal_loader_error(
            c"vkCreateAndroidSurfaceKHR: Invalid instance [VUID-vkCreateAndroidSurfaceKHR-instance-parameter]",
        ));
    let dispatch = unsafe { &*loader.dispatch() };
    let command = dispatch.vkCreateAndroidSurfaceKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(loader.chain_handle(), pCreateInfo, pAllocator, pSurface) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateBuffer(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkBufferCreateInfo<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pBuffer: *mut vk::VkBuffer,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateBuffer;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pBuffer) }
}
#[cfg(target_os = "fuchsia")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateBufferCollectionFUCHSIA(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkBufferCollectionCreateInfoFUCHSIA<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pCollection: *mut vk::VkBufferCollectionFUCHSIA,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateBufferCollectionFUCHSIA;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pCollection) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateBufferView(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkBufferViewCreateInfo<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pView: *mut vk::VkBufferView,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateBufferView;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pView) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateCommandPool(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkCommandPoolCreateInfo<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pCommandPool: *mut vk::VkCommandPool,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateCommandPool;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pCommandPool) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateComputePipelines(
    device: vk::VkDevice,
    pipelineCache: vk::VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const vk::VkComputePipelineCreateInfo<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pPipelines: *mut vk::VkPipeline,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateComputePipelines;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            pipelineCache,
            createInfoCount,
            pCreateInfos,
            pAllocator,
            pPipelines,
        )
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateCuFunctionNVX(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkCuFunctionCreateInfoNVX<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pFunction: *mut vk::VkCuFunctionNVX,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateCuFunctionNVX;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pFunction) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateCuModuleNVX(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkCuModuleCreateInfoNVX<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pModule: *mut vk::VkCuModuleNVX,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateCuModuleNVX;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pModule) }
}
#[cfg(feature = "beta-extensions")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateCudaFunctionNV(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkCudaFunctionCreateInfoNV<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pFunction: *mut vk::VkCudaFunctionNV,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateCudaFunctionNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pFunction) }
}
#[cfg(feature = "beta-extensions")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateCudaModuleNV(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkCudaModuleCreateInfoNV<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pModule: *mut vk::VkCudaModuleNV,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateCudaModuleNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pModule) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateDataGraphPipelineSessionARM(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkDataGraphPipelineSessionCreateInfoARM<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSession: *mut vk::VkDataGraphPipelineSessionARM,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateDataGraphPipelineSessionARM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pSession) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateDataGraphPipelinesARM(
    device: vk::VkDevice,
    deferredOperation: vk::VkDeferredOperationKHR,
    pipelineCache: vk::VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const vk::VkDataGraphPipelineCreateInfoARM<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pPipelines: *mut vk::VkPipeline,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateDataGraphPipelinesARM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            deferredOperation,
            pipelineCache,
            createInfoCount,
            pCreateInfos,
            pAllocator,
            pPipelines,
        )
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateDeferredOperationKHR(
    device: vk::VkDevice,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pDeferredOperation: *mut vk::VkDeferredOperationKHR,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateDeferredOperationKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pAllocator, pDeferredOperation) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateDescriptorPool(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkDescriptorPoolCreateInfo<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pDescriptorPool: *mut vk::VkDescriptorPool,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateDescriptorPool;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pDescriptorPool) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateDescriptorSetLayout(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkDescriptorSetLayoutCreateInfo<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSetLayout: *mut vk::VkDescriptorSetLayout,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateDescriptorSetLayout;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pSetLayout) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateDescriptorUpdateTemplate(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkDescriptorUpdateTemplateCreateInfo<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pDescriptorUpdateTemplate: *mut vk::VkDescriptorUpdateTemplate,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateDescriptorUpdateTemplate;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pDescriptorUpdateTemplate) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateDescriptorUpdateTemplateKHR(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkDescriptorUpdateTemplateCreateInfoKHR<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pDescriptorUpdateTemplate: *mut vk::VkDescriptorUpdateTemplateKHR,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateDescriptorUpdateTemplateKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pDescriptorUpdateTemplate) }
}
#[cfg(feature = "wsi-directfb")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateDirectFBSurfaceEXT(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkDirectFBSurfaceCreateInfoEXT<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    let loader = unsafe { LoaderInstance::from_handle(instance) }
        .unwrap_or_else(|| fatal_loader_error(
            c"vkCreateDirectFBSurfaceEXT: Invalid instance [VUID-vkCreateDirectFBSurfaceEXT-instance-parameter]",
        ));
    let dispatch = unsafe { &*loader.dispatch() };
    let command = dispatch.vkCreateDirectFBSurfaceEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(loader.chain_handle(), pCreateInfo, pAllocator, pSurface) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateDisplayModeKHR(
    physicalDevice: vk::VkPhysicalDevice,
    display: vk::VkDisplayKHR,
    pCreateInfo: *const vk::VkDisplayModeCreateInfoKHR<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pMode: *mut vk::VkDisplayModeKHR,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkCreateDisplayModeKHR: Invalid physicalDevice [VUID-vkCreateDisplayModeKHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkCreateDisplayModeKHR
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(physicalDevice, display, pCreateInfo, pAllocator, pMode)
        },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateDisplayPlaneSurfaceKHR(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkDisplaySurfaceCreateInfoKHR<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    let loader = unsafe { LoaderInstance::from_handle(instance) }
        .unwrap_or_else(|| fatal_loader_error(
            c"vkCreateDisplayPlaneSurfaceKHR: Invalid instance [VUID-vkCreateDisplayPlaneSurfaceKHR-instance-parameter]",
        ));
    let dispatch = unsafe { &*loader.dispatch() };
    let command = dispatch.vkCreateDisplayPlaneSurfaceKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(loader.chain_handle(), pCreateInfo, pAllocator, pSurface) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateEvent(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkEventCreateInfo<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pEvent: *mut vk::VkEvent,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateEvent;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pEvent) }
}
#[cfg(feature = "beta-extensions")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateExecutionGraphPipelinesAMDX(
    device: vk::VkDevice,
    pipelineCache: vk::VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const vk::VkExecutionGraphPipelineCreateInfoAMDX<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pPipelines: *mut vk::VkPipeline,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateExecutionGraphPipelinesAMDX;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            pipelineCache,
            createInfoCount,
            pCreateInfos,
            pAllocator,
            pPipelines,
        )
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateExternalComputeQueueNV(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkExternalComputeQueueCreateInfoNV<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pExternalQueue: *mut vk::VkExternalComputeQueueNV,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateExternalComputeQueueNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pExternalQueue) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateFence(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkFenceCreateInfo<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pFence: *mut vk::VkFence,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateFence;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pFence) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateFramebuffer(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkFramebufferCreateInfo<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pFramebuffer: *mut vk::VkFramebuffer,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateFramebuffer;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pFramebuffer) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateGpaSessionAMD(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkGpaSessionCreateInfoAMD<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pGpaSession: *mut vk::VkGpaSessionAMD,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateGpaSessionAMD;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pGpaSession) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateGraphicsPipelines(
    device: vk::VkDevice,
    pipelineCache: vk::VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const vk::VkGraphicsPipelineCreateInfo<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pPipelines: *mut vk::VkPipeline,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateGraphicsPipelines;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            pipelineCache,
            createInfoCount,
            pCreateInfos,
            pAllocator,
            pPipelines,
        )
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateHeadlessSurfaceEXT(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkHeadlessSurfaceCreateInfoEXT<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    let loader = unsafe { LoaderInstance::from_handle(instance) }
        .unwrap_or_else(|| fatal_loader_error(
            c"vkCreateHeadlessSurfaceEXT: Invalid instance [VUID-vkCreateHeadlessSurfaceEXT-instance-parameter]",
        ));
    let dispatch = unsafe { &*loader.dispatch() };
    let command = dispatch.vkCreateHeadlessSurfaceEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(loader.chain_handle(), pCreateInfo, pAllocator, pSurface) }
}
#[cfg(target_os = "ios")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateIOSSurfaceMVK(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkIOSSurfaceCreateInfoMVK<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    let loader = unsafe { LoaderInstance::from_handle(instance) }
        .unwrap_or_else(|| fatal_loader_error(
            c"vkCreateIOSSurfaceMVK: Invalid instance [VUID-vkCreateIOSSurfaceMVK-instance-parameter]",
        ));
    let dispatch = unsafe { &*loader.dispatch() };
    let command = dispatch.vkCreateIOSSurfaceMVK;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(loader.chain_handle(), pCreateInfo, pAllocator, pSurface) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateImage(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkImageCreateInfo<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pImage: *mut vk::VkImage,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateImage;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pImage) }
}
#[cfg(target_os = "fuchsia")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateImagePipeSurfaceFUCHSIA(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkImagePipeSurfaceCreateInfoFUCHSIA<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    let loader = unsafe { LoaderInstance::from_handle(instance) }
        .unwrap_or_else(|| fatal_loader_error(
            c"vkCreateImagePipeSurfaceFUCHSIA: Invalid instance [VUID-vkCreateImagePipeSurfaceFUCHSIA-instance-parameter]",
        ));
    let dispatch = unsafe { &*loader.dispatch() };
    let command = dispatch.vkCreateImagePipeSurfaceFUCHSIA;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(loader.chain_handle(), pCreateInfo, pAllocator, pSurface) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateImageView(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkImageViewCreateInfo<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pView: *mut vk::VkImageView,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateImageView;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pView) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateIndirectCommandsLayoutEXT(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkIndirectCommandsLayoutCreateInfoEXT<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pIndirectCommandsLayout: *mut vk::VkIndirectCommandsLayoutEXT,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateIndirectCommandsLayoutEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pIndirectCommandsLayout) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateIndirectCommandsLayoutNV(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkIndirectCommandsLayoutCreateInfoNV<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pIndirectCommandsLayout: *mut vk::VkIndirectCommandsLayoutNV,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateIndirectCommandsLayoutNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pIndirectCommandsLayout) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateIndirectExecutionSetEXT(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkIndirectExecutionSetCreateInfoEXT<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pIndirectExecutionSet: *mut vk::VkIndirectExecutionSetEXT,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateIndirectExecutionSetEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pIndirectExecutionSet) }
}
#[cfg(target_os = "macos")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateMacOSSurfaceMVK(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkMacOSSurfaceCreateInfoMVK<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    let loader = unsafe { LoaderInstance::from_handle(instance) }
        .unwrap_or_else(|| fatal_loader_error(
            c"vkCreateMacOSSurfaceMVK: Invalid instance [VUID-vkCreateMacOSSurfaceMVK-instance-parameter]",
        ));
    let dispatch = unsafe { &*loader.dispatch() };
    let command = dispatch.vkCreateMacOSSurfaceMVK;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(loader.chain_handle(), pCreateInfo, pAllocator, pSurface) }
}
#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "visionos"
))]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateMetalSurfaceEXT(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkMetalSurfaceCreateInfoEXT<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    let loader = unsafe { LoaderInstance::from_handle(instance) }
        .unwrap_or_else(|| fatal_loader_error(
            c"vkCreateMetalSurfaceEXT: Invalid instance [VUID-vkCreateMetalSurfaceEXT-instance-parameter]",
        ));
    let dispatch = unsafe { &*loader.dispatch() };
    let command = dispatch.vkCreateMetalSurfaceEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(loader.chain_handle(), pCreateInfo, pAllocator, pSurface) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateMicromapEXT(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkMicromapCreateInfoEXT<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pMicromap: *mut vk::VkMicromapEXT,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateMicromapEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pMicromap) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateOpticalFlowSessionNV(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkOpticalFlowSessionCreateInfoNV<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSession: *mut vk::VkOpticalFlowSessionNV,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateOpticalFlowSessionNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pSession) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreatePipelineBinariesKHR(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkPipelineBinaryCreateInfoKHR<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pBinaries: *mut vk::VkPipelineBinaryHandlesInfoKHR<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreatePipelineBinariesKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pBinaries) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreatePipelineCache(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkPipelineCacheCreateInfo<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pPipelineCache: *mut vk::VkPipelineCache,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreatePipelineCache;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pPipelineCache) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreatePipelineLayout(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkPipelineLayoutCreateInfo<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pPipelineLayout: *mut vk::VkPipelineLayout,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreatePipelineLayout;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pPipelineLayout) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreatePrivateDataSlot(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkPrivateDataSlotCreateInfo<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pPrivateDataSlot: *mut vk::VkPrivateDataSlot,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreatePrivateDataSlot;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pPrivateDataSlot) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreatePrivateDataSlotEXT(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkPrivateDataSlotCreateInfoEXT<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pPrivateDataSlot: *mut vk::VkPrivateDataSlotEXT,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreatePrivateDataSlotEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pPrivateDataSlot) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateQueryPool(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkQueryPoolCreateInfo<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pQueryPool: *mut vk::VkQueryPool,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateQueryPool;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pQueryPool) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateRayTracingPipelinesKHR(
    device: vk::VkDevice,
    deferredOperation: vk::VkDeferredOperationKHR,
    pipelineCache: vk::VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const vk::VkRayTracingPipelineCreateInfoKHR<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pPipelines: *mut vk::VkPipeline,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateRayTracingPipelinesKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            deferredOperation,
            pipelineCache,
            createInfoCount,
            pCreateInfos,
            pAllocator,
            pPipelines,
        )
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateRayTracingPipelinesNV(
    device: vk::VkDevice,
    pipelineCache: vk::VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const vk::VkRayTracingPipelineCreateInfoNV<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pPipelines: *mut vk::VkPipeline,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateRayTracingPipelinesNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            pipelineCache,
            createInfoCount,
            pCreateInfos,
            pAllocator,
            pPipelines,
        )
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateRenderPass(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkRenderPassCreateInfo<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pRenderPass: *mut vk::VkRenderPass,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateRenderPass;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pRenderPass) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateRenderPass2(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkRenderPassCreateInfo2<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pRenderPass: *mut vk::VkRenderPass,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateRenderPass2;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pRenderPass) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateRenderPass2KHR(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkRenderPassCreateInfo2KHR<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pRenderPass: *mut vk::VkRenderPass,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateRenderPass2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pRenderPass) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateSampler(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkSamplerCreateInfo<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSampler: *mut vk::VkSampler,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateSampler;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pSampler) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateSamplerYcbcrConversion(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkSamplerYcbcrConversionCreateInfo<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pYcbcrConversion: *mut vk::VkSamplerYcbcrConversion,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateSamplerYcbcrConversion;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pYcbcrConversion) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateSamplerYcbcrConversionKHR(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkSamplerYcbcrConversionCreateInfoKHR<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pYcbcrConversion: *mut vk::VkSamplerYcbcrConversionKHR,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateSamplerYcbcrConversionKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pYcbcrConversion) }
}
#[cfg(any(target_os = "nto", target_os = "qnx"))]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateScreenSurfaceQNX(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkScreenSurfaceCreateInfoQNX<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    let loader = unsafe { LoaderInstance::from_handle(instance) }
        .unwrap_or_else(|| fatal_loader_error(
            c"vkCreateScreenSurfaceQNX: Invalid instance [VUID-vkCreateScreenSurfaceQNX-instance-parameter]",
        ));
    let dispatch = unsafe { &*loader.dispatch() };
    let command = dispatch.vkCreateScreenSurfaceQNX;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(loader.chain_handle(), pCreateInfo, pAllocator, pSurface) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateSemaphore(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkSemaphoreCreateInfo<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSemaphore: *mut vk::VkSemaphore,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateSemaphore;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pSemaphore) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateShaderInstrumentationARM(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkShaderInstrumentationCreateInfoARM<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pInstrumentation: *mut vk::VkShaderInstrumentationARM,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateShaderInstrumentationARM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pInstrumentation) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateShaderModule(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkShaderModuleCreateInfo<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pShaderModule: *mut vk::VkShaderModule,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateShaderModule;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pShaderModule) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateShadersEXT(
    device: vk::VkDevice,
    createInfoCount: u32,
    pCreateInfos: *const vk::VkShaderCreateInfoEXT<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pShaders: *mut vk::VkShaderEXT,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateShadersEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, createInfoCount, pCreateInfos, pAllocator, pShaders) }
}
#[cfg(feature = "platform-ggp")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateStreamDescriptorSurfaceGGP(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkStreamDescriptorSurfaceCreateInfoGGP<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    let loader = unsafe { LoaderInstance::from_handle(instance) }
        .unwrap_or_else(|| fatal_loader_error(
            c"vkCreateStreamDescriptorSurfaceGGP: Invalid instance [VUID-vkCreateStreamDescriptorSurfaceGGP-instance-parameter]",
        ));
    let dispatch = unsafe { &*loader.dispatch() };
    let command = dispatch.vkCreateStreamDescriptorSurfaceGGP;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(loader.chain_handle(), pCreateInfo, pAllocator, pSurface) }
}
#[cfg(target_env = "ohos")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateSurfaceOHOS(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkSurfaceCreateInfoOHOS<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    let loader = unsafe { LoaderInstance::from_handle(instance) }.unwrap_or_else(|| {
        fatal_loader_error(
            c"vkCreateSurfaceOHOS: Invalid instance [VUID-vkCreateSurfaceOHOS-instance-parameter]",
        )
    });
    let dispatch = unsafe { &*loader.dispatch() };
    let command = dispatch.vkCreateSurfaceOHOS;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(loader.chain_handle(), pCreateInfo, pAllocator, pSurface) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateTensorARM(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkTensorCreateInfoARM<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pTensor: *mut vk::VkTensorARM,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateTensorARM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pTensor) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateTensorViewARM(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkTensorViewCreateInfoARM<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pView: *mut vk::VkTensorViewARM,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateTensorViewARM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pView) }
}
#[cfg(feature = "platform-ubm")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateUbmSurfaceSEC(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkUbmSurfaceCreateInfoSEC<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    let loader = unsafe { LoaderInstance::from_handle(instance) }
        .unwrap_or_else(|| fatal_loader_error(
            c"vkCreateUbmSurfaceSEC: Invalid instance [VUID-vkCreateUbmSurfaceSEC-instance-parameter]",
        ));
    let dispatch = unsafe { &*loader.dispatch() };
    let command = dispatch.vkCreateUbmSurfaceSEC;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(loader.chain_handle(), pCreateInfo, pAllocator, pSurface) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateValidationCacheEXT(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkValidationCacheCreateInfoEXT<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pValidationCache: *mut vk::VkValidationCacheEXT,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateValidationCacheEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pValidationCache) }
}
#[cfg(feature = "platform-vi")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateViSurfaceNN(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkViSurfaceCreateInfoNN<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    let loader = unsafe { LoaderInstance::from_handle(instance) }.unwrap_or_else(|| {
        fatal_loader_error(
            c"vkCreateViSurfaceNN: Invalid instance [VUID-vkCreateViSurfaceNN-instance-parameter]",
        )
    });
    let dispatch = unsafe { &*loader.dispatch() };
    let command = dispatch.vkCreateViSurfaceNN;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(loader.chain_handle(), pCreateInfo, pAllocator, pSurface) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateVideoSessionKHR(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkVideoSessionCreateInfoKHR<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pVideoSession: *mut vk::VkVideoSessionKHR,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateVideoSessionKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pVideoSession) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkCreateVideoSessionParametersKHR(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkVideoSessionParametersCreateInfoKHR<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pVideoSessionParameters: *mut vk::VkVideoSessionParametersKHR,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkCreateVideoSessionParametersKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pCreateInfo, pAllocator, pVideoSessionParameters) }
}
#[cfg(all(
    feature = "wsi-wayland",
    any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "hurd",
        target_os = "cygwin"
    )
))]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateWaylandSurfaceKHR(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkWaylandSurfaceCreateInfoKHR<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    let loader = unsafe { LoaderInstance::from_handle(instance) }
        .unwrap_or_else(|| fatal_loader_error(
            c"vkCreateWaylandSurfaceKHR: Invalid instance [VUID-vkCreateWaylandSurfaceKHR-instance-parameter]",
        ));
    let dispatch = unsafe { &*loader.dispatch() };
    let command = dispatch.vkCreateWaylandSurfaceKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(loader.chain_handle(), pCreateInfo, pAllocator, pSurface) }
}
#[cfg(target_os = "windows")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateWin32SurfaceKHR(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkWin32SurfaceCreateInfoKHR<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    let loader = unsafe { LoaderInstance::from_handle(instance) }
        .unwrap_or_else(|| fatal_loader_error(
            c"vkCreateWin32SurfaceKHR: Invalid instance [VUID-vkCreateWin32SurfaceKHR-instance-parameter]",
        ));
    let dispatch = unsafe { &*loader.dispatch() };
    let command = dispatch.vkCreateWin32SurfaceKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(loader.chain_handle(), pCreateInfo, pAllocator, pSurface) }
}
#[cfg(all(
    feature = "wsi-xcb",
    any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "hurd",
        target_os = "cygwin"
    )
))]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateXcbSurfaceKHR(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkXcbSurfaceCreateInfoKHR<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    let loader = unsafe { LoaderInstance::from_handle(instance) }
        .unwrap_or_else(|| fatal_loader_error(
            c"vkCreateXcbSurfaceKHR: Invalid instance [VUID-vkCreateXcbSurfaceKHR-instance-parameter]",
        ));
    let dispatch = unsafe { &*loader.dispatch() };
    let command = dispatch.vkCreateXcbSurfaceKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(loader.chain_handle(), pCreateInfo, pAllocator, pSurface) }
}
#[cfg(all(
    feature = "wsi-xlib",
    any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "hurd",
        target_os = "cygwin"
    )
))]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkCreateXlibSurfaceKHR(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkXlibSurfaceCreateInfoKHR<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    let loader = unsafe { LoaderInstance::from_handle(instance) }
        .unwrap_or_else(|| fatal_loader_error(
            c"vkCreateXlibSurfaceKHR: Invalid instance [VUID-vkCreateXlibSurfaceKHR-instance-parameter]",
        ));
    let dispatch = unsafe { &*loader.dispatch() };
    let command = dispatch.vkCreateXlibSurfaceKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(loader.chain_handle(), pCreateInfo, pAllocator, pSurface) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkDeferredOperationJoinKHR(
    device: vk::VkDevice,
    operation: vk::VkDeferredOperationKHR,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDeferredOperationJoinKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, operation) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkDestroyAccelerationStructureKHR(
    device: vk::VkDevice,
    accelerationStructure: vk::VkAccelerationStructureKHR,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyAccelerationStructureKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, accelerationStructure, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkDestroyAccelerationStructureNV(
    device: vk::VkDevice,
    accelerationStructure: vk::VkAccelerationStructureNV,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyAccelerationStructureNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, accelerationStructure, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkDestroyBuffer(
    device: vk::VkDevice,
    buffer: vk::VkBuffer,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyBuffer;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, buffer, pAllocator);
    }
}
#[cfg(target_os = "fuchsia")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkDestroyBufferCollectionFUCHSIA(
    device: vk::VkDevice,
    collection: vk::VkBufferCollectionFUCHSIA,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyBufferCollectionFUCHSIA;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, collection, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkDestroyBufferView(
    device: vk::VkDevice,
    bufferView: vk::VkBufferView,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyBufferView;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, bufferView, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkDestroyCommandPool(
    device: vk::VkDevice,
    commandPool: vk::VkCommandPool,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyCommandPool;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, commandPool, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkDestroyCuFunctionNVX(
    device: vk::VkDevice,
    function: vk::VkCuFunctionNVX,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyCuFunctionNVX;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, function, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkDestroyCuModuleNVX(
    device: vk::VkDevice,
    module: vk::VkCuModuleNVX,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyCuModuleNVX;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, module, pAllocator);
    }
}
#[cfg(feature = "beta-extensions")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkDestroyCudaFunctionNV(
    device: vk::VkDevice,
    function: vk::VkCudaFunctionNV,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyCudaFunctionNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, function, pAllocator);
    }
}
#[cfg(feature = "beta-extensions")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkDestroyCudaModuleNV(
    device: vk::VkDevice,
    module: vk::VkCudaModuleNV,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyCudaModuleNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, module, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkDestroyDataGraphPipelineSessionARM(
    device: vk::VkDevice,
    session: vk::VkDataGraphPipelineSessionARM,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyDataGraphPipelineSessionARM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, session, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkDestroyDeferredOperationKHR(
    device: vk::VkDevice,
    operation: vk::VkDeferredOperationKHR,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyDeferredOperationKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, operation, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkDestroyDescriptorPool(
    device: vk::VkDevice,
    descriptorPool: vk::VkDescriptorPool,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyDescriptorPool;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, descriptorPool, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkDestroyDescriptorSetLayout(
    device: vk::VkDevice,
    descriptorSetLayout: vk::VkDescriptorSetLayout,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyDescriptorSetLayout;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, descriptorSetLayout, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkDestroyDescriptorUpdateTemplate(
    device: vk::VkDevice,
    descriptorUpdateTemplate: vk::VkDescriptorUpdateTemplate,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyDescriptorUpdateTemplate;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, descriptorUpdateTemplate, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkDestroyDescriptorUpdateTemplateKHR(
    device: vk::VkDevice,
    descriptorUpdateTemplate: vk::VkDescriptorUpdateTemplateKHR,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyDescriptorUpdateTemplateKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, descriptorUpdateTemplate, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkDestroyEvent(
    device: vk::VkDevice,
    event: vk::VkEvent,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyEvent;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, event, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkDestroyExternalComputeQueueNV(
    device: vk::VkDevice,
    externalQueue: vk::VkExternalComputeQueueNV,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyExternalComputeQueueNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, externalQueue, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkDestroyFence(
    device: vk::VkDevice,
    fence: vk::VkFence,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyFence;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, fence, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkDestroyFramebuffer(
    device: vk::VkDevice,
    framebuffer: vk::VkFramebuffer,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyFramebuffer;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, framebuffer, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkDestroyGpaSessionAMD(
    device: vk::VkDevice,
    gpaSession: vk::VkGpaSessionAMD,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyGpaSessionAMD;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, gpaSession, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkDestroyImage(
    device: vk::VkDevice,
    image: vk::VkImage,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyImage;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, image, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkDestroyImageView(
    device: vk::VkDevice,
    imageView: vk::VkImageView,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyImageView;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, imageView, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkDestroyIndirectCommandsLayoutEXT(
    device: vk::VkDevice,
    indirectCommandsLayout: vk::VkIndirectCommandsLayoutEXT,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyIndirectCommandsLayoutEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, indirectCommandsLayout, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkDestroyIndirectCommandsLayoutNV(
    device: vk::VkDevice,
    indirectCommandsLayout: vk::VkIndirectCommandsLayoutNV,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyIndirectCommandsLayoutNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, indirectCommandsLayout, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkDestroyIndirectExecutionSetEXT(
    device: vk::VkDevice,
    indirectExecutionSet: vk::VkIndirectExecutionSetEXT,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyIndirectExecutionSetEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, indirectExecutionSet, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkDestroyMicromapEXT(
    device: vk::VkDevice,
    micromap: vk::VkMicromapEXT,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyMicromapEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, micromap, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkDestroyOpticalFlowSessionNV(
    device: vk::VkDevice,
    session: vk::VkOpticalFlowSessionNV,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyOpticalFlowSessionNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, session, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkDestroyPipeline(
    device: vk::VkDevice,
    pipeline: vk::VkPipeline,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyPipeline;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pipeline, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkDestroyPipelineBinaryKHR(
    device: vk::VkDevice,
    pipelineBinary: vk::VkPipelineBinaryKHR,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyPipelineBinaryKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pipelineBinary, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkDestroyPipelineCache(
    device: vk::VkDevice,
    pipelineCache: vk::VkPipelineCache,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyPipelineCache;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pipelineCache, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkDestroyPipelineLayout(
    device: vk::VkDevice,
    pipelineLayout: vk::VkPipelineLayout,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyPipelineLayout;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pipelineLayout, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkDestroyPrivateDataSlot(
    device: vk::VkDevice,
    privateDataSlot: vk::VkPrivateDataSlot,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyPrivateDataSlot;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, privateDataSlot, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkDestroyPrivateDataSlotEXT(
    device: vk::VkDevice,
    privateDataSlot: vk::VkPrivateDataSlotEXT,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyPrivateDataSlotEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, privateDataSlot, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkDestroyQueryPool(
    device: vk::VkDevice,
    queryPool: vk::VkQueryPool,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyQueryPool;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, queryPool, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkDestroyRenderPass(
    device: vk::VkDevice,
    renderPass: vk::VkRenderPass,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyRenderPass;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, renderPass, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkDestroySampler(
    device: vk::VkDevice,
    sampler: vk::VkSampler,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroySampler;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, sampler, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkDestroySamplerYcbcrConversion(
    device: vk::VkDevice,
    ycbcrConversion: vk::VkSamplerYcbcrConversion,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroySamplerYcbcrConversion;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, ycbcrConversion, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkDestroySamplerYcbcrConversionKHR(
    device: vk::VkDevice,
    ycbcrConversion: vk::VkSamplerYcbcrConversionKHR,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroySamplerYcbcrConversionKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, ycbcrConversion, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkDestroySemaphore(
    device: vk::VkDevice,
    semaphore: vk::VkSemaphore,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroySemaphore;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, semaphore, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkDestroyShaderEXT(
    device: vk::VkDevice,
    shader: vk::VkShaderEXT,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyShaderEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, shader, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkDestroyShaderInstrumentationARM(
    device: vk::VkDevice,
    instrumentation: vk::VkShaderInstrumentationARM,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyShaderInstrumentationARM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, instrumentation, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkDestroyShaderModule(
    device: vk::VkDevice,
    shaderModule: vk::VkShaderModule,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyShaderModule;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, shaderModule, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkDestroySwapchainKHR(
    device: vk::VkDevice,
    swapchain: vk::VkSwapchainKHR,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroySwapchainKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, swapchain, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkDestroyTensorARM(
    device: vk::VkDevice,
    tensor: vk::VkTensorARM,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyTensorARM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, tensor, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkDestroyTensorViewARM(
    device: vk::VkDevice,
    tensorView: vk::VkTensorViewARM,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyTensorViewARM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, tensorView, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkDestroyValidationCacheEXT(
    device: vk::VkDevice,
    validationCache: vk::VkValidationCacheEXT,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyValidationCacheEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, validationCache, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkDestroyVideoSessionKHR(
    device: vk::VkDevice,
    videoSession: vk::VkVideoSessionKHR,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyVideoSessionKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, videoSession, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkDestroyVideoSessionParametersKHR(
    device: vk::VkDevice,
    videoSessionParameters: vk::VkVideoSessionParametersKHR,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDestroyVideoSessionParametersKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, videoSessionParameters, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkDeviceWaitIdle(device: vk::VkDevice) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDeviceWaitIdle;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkDisplayPowerControlEXT(
    device: vk::VkDevice,
    display: vk::VkDisplayKHR,
    pDisplayPowerInfo: *const vk::VkDisplayPowerInfoEXT<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkDisplayPowerControlEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, display, pDisplayPowerInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkEndCommandBuffer(
    commandBuffer: vk::VkCommandBuffer,
) -> vk::VkResult {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkEndCommandBuffer;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(commandBuffer) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM(
    physicalDevice: vk::VkPhysicalDevice,
    queueFamilyIndex: u32,
    pCounterCount: *mut u32,
    pCounters: *mut vk::VkPerformanceCounterARM<'_>,
    pCounterDescriptions: *mut vk::VkPerformanceCounterDescriptionARM<'_>,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM: Invalid physicalDevice [VUID-vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(
                physicalDevice,
                queueFamilyIndex,
                pCounterCount,
                pCounters,
                pCounterDescriptions,
            )
        },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR(
    physicalDevice: vk::VkPhysicalDevice,
    queueFamilyIndex: u32,
    pCounterCount: *mut u32,
    pCounters: *mut vk::VkPerformanceCounterKHR<'_>,
    pCounterDescriptions: *mut vk::VkPerformanceCounterDescriptionKHR<'_>,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR: Invalid physicalDevice [VUID-vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(
                physicalDevice,
                queueFamilyIndex,
                pCounterCount,
                pCounters,
                pCounterDescriptions,
            )
        },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM(
    physicalDevice: vk::VkPhysicalDevice,
    pDescriptionCount: *mut u32,
    pDescriptions: *mut vk::VkShaderInstrumentationMetricDescriptionARM<'_>,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM: Invalid physicalDevice [VUID-vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(physicalDevice, pDescriptionCount, pDescriptions)
        },
    )
}
#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "visionos"
))]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkExportMetalObjectsEXT(
    device: vk::VkDevice,
    pMetalObjectsInfo: *mut vk::VkExportMetalObjectsInfoEXT<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkExportMetalObjectsEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pMetalObjectsInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkFlushMappedMemoryRanges(
    device: vk::VkDevice,
    memoryRangeCount: u32,
    pMemoryRanges: *const vk::VkMappedMemoryRange<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkFlushMappedMemoryRanges;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, memoryRangeCount, pMemoryRanges) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkFreeCommandBuffers(
    device: vk::VkDevice,
    commandPool: vk::VkCommandPool,
    commandBufferCount: u32,
    pCommandBuffers: *const vk::VkCommandBuffer,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkFreeCommandBuffers;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, commandPool, commandBufferCount, pCommandBuffers);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkFreeDescriptorSets(
    device: vk::VkDevice,
    descriptorPool: vk::VkDescriptorPool,
    descriptorSetCount: u32,
    pDescriptorSets: *const vk::VkDescriptorSet,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkFreeDescriptorSets;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, descriptorPool, descriptorSetCount, pDescriptorSets) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkFreeMemory(
    device: vk::VkDevice,
    memory: vk::VkDeviceMemory,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkFreeMemory;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, memory, pAllocator);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetAccelerationStructureBuildSizesKHR(
    device: vk::VkDevice,
    buildType: vk::VkAccelerationStructureBuildTypeKHR,
    pBuildInfo: *const vk::VkAccelerationStructureBuildGeometryInfoKHR<'_>,
    pMaxPrimitiveCounts: *const u32,
    pSizeInfo: *mut vk::VkAccelerationStructureBuildSizesInfoKHR<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetAccelerationStructureBuildSizesKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            buildType,
            pBuildInfo,
            pMaxPrimitiveCounts,
            pSizeInfo,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetAccelerationStructureDeviceAddressKHR(
    device: vk::VkDevice,
    pInfo: *const vk::VkAccelerationStructureDeviceAddressInfoKHR<'_>,
) -> vk::VkDeviceAddress {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetAccelerationStructureDeviceAddressKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetAccelerationStructureHandleNV(
    device: vk::VkDevice,
    accelerationStructure: vk::VkAccelerationStructureNV,
    dataSize: usize,
    pData: *mut c_void,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetAccelerationStructureHandleNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, accelerationStructure, dataSize, pData) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetAccelerationStructureMemoryRequirementsNV(
    device: vk::VkDevice,
    pInfo: *const vk::VkAccelerationStructureMemoryRequirementsInfoNV<'_>,
    pMemoryRequirements: *mut vk::VkMemoryRequirements2<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetAccelerationStructureMemoryRequirementsNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pInfo, pMemoryRequirements);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT(
    device: vk::VkDevice,
    pInfo: *const vk::VkAccelerationStructureCaptureDescriptorDataInfoEXT<'_>,
    pData: *mut c_void,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pInfo, pData) }
}
#[cfg(target_os = "android")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetAndroidHardwareBufferPropertiesANDROID(
    device: vk::VkDevice,
    buffer: *const vk::AHardwareBuffer,
    pProperties: *mut vk::VkAndroidHardwareBufferPropertiesANDROID<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetAndroidHardwareBufferPropertiesANDROID;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, buffer, pProperties) }
}
#[cfg(target_os = "fuchsia")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetBufferCollectionPropertiesFUCHSIA(
    device: vk::VkDevice,
    collection: vk::VkBufferCollectionFUCHSIA,
    pProperties: *mut vk::VkBufferCollectionPropertiesFUCHSIA<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetBufferCollectionPropertiesFUCHSIA;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, collection, pProperties) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetBufferDeviceAddress(
    device: vk::VkDevice,
    pInfo: *const vk::VkBufferDeviceAddressInfo<'_>,
) -> vk::VkDeviceAddress {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetBufferDeviceAddress;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetBufferDeviceAddressEXT(
    device: vk::VkDevice,
    pInfo: *const vk::VkBufferDeviceAddressInfoEXT<'_>,
) -> vk::VkDeviceAddress {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetBufferDeviceAddressEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetBufferDeviceAddressKHR(
    device: vk::VkDevice,
    pInfo: *const vk::VkBufferDeviceAddressInfoKHR<'_>,
) -> vk::VkDeviceAddress {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetBufferDeviceAddressKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetBufferMemoryRequirements(
    device: vk::VkDevice,
    buffer: vk::VkBuffer,
    pMemoryRequirements: *mut vk::VkMemoryRequirements,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetBufferMemoryRequirements;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, buffer, pMemoryRequirements);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetBufferMemoryRequirements2(
    device: vk::VkDevice,
    pInfo: *const vk::VkBufferMemoryRequirementsInfo2<'_>,
    pMemoryRequirements: *mut vk::VkMemoryRequirements2<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetBufferMemoryRequirements2;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pInfo, pMemoryRequirements);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetBufferMemoryRequirements2KHR(
    device: vk::VkDevice,
    pInfo: *const vk::VkBufferMemoryRequirementsInfo2KHR<'_>,
    pMemoryRequirements: *mut vk::VkMemoryRequirements2KHR<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetBufferMemoryRequirements2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pInfo, pMemoryRequirements);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetBufferOpaqueCaptureAddress(
    device: vk::VkDevice,
    pInfo: *const vk::VkBufferDeviceAddressInfo<'_>,
) -> u64 {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetBufferOpaqueCaptureAddress;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetBufferOpaqueCaptureAddressKHR(
    device: vk::VkDevice,
    pInfo: *const vk::VkBufferDeviceAddressInfoKHR<'_>,
) -> u64 {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetBufferOpaqueCaptureAddressKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetBufferOpaqueCaptureDescriptorDataEXT(
    device: vk::VkDevice,
    pInfo: *const vk::VkBufferCaptureDescriptorDataInfoEXT<'_>,
    pData: *mut c_void,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetBufferOpaqueCaptureDescriptorDataEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pInfo, pData) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetCalibratedTimestampsEXT(
    device: vk::VkDevice,
    timestampCount: u32,
    pTimestampInfos: *const vk::VkCalibratedTimestampInfoEXT<'_>,
    pTimestamps: *mut u64,
    pMaxDeviation: *mut u64,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetCalibratedTimestampsEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            timestampCount,
            pTimestampInfos,
            pTimestamps,
            pMaxDeviation,
        )
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetCalibratedTimestampsKHR(
    device: vk::VkDevice,
    timestampCount: u32,
    pTimestampInfos: *const vk::VkCalibratedTimestampInfoKHR<'_>,
    pTimestamps: *mut u64,
    pMaxDeviation: *mut u64,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetCalibratedTimestampsKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            timestampCount,
            pTimestampInfos,
            pTimestamps,
            pMaxDeviation,
        )
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetClusterAccelerationStructureBuildSizesNV(
    device: vk::VkDevice,
    pInfo: *const vk::VkClusterAccelerationStructureInputInfoNV<'_>,
    pSizeInfo: *mut vk::VkAccelerationStructureBuildSizesInfoKHR<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetClusterAccelerationStructureBuildSizesNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pInfo, pSizeInfo);
    }
}
#[cfg(feature = "beta-extensions")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetCudaModuleCacheNV(
    device: vk::VkDevice,
    module: vk::VkCudaModuleNV,
    pCacheSize: *mut usize,
    pCacheData: *mut c_void,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetCudaModuleCacheNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, module, pCacheSize, pCacheData) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetDataGraphPipelineAvailablePropertiesARM(
    device: vk::VkDevice,
    pPipelineInfo: *const vk::VkDataGraphPipelineInfoARM<'_>,
    pPropertiesCount: *mut u32,
    pProperties: *mut vk::VkDataGraphPipelinePropertyARM,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDataGraphPipelineAvailablePropertiesARM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pPipelineInfo, pPropertiesCount, pProperties) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetDataGraphPipelinePropertiesARM(
    device: vk::VkDevice,
    pPipelineInfo: *const vk::VkDataGraphPipelineInfoARM<'_>,
    propertiesCount: u32,
    pProperties: *mut vk::VkDataGraphPipelinePropertyQueryResultARM<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDataGraphPipelinePropertiesARM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pPipelineInfo, propertiesCount, pProperties) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetDataGraphPipelineSessionBindPointRequirementsARM(
    device: vk::VkDevice,
    pInfo: *const vk::VkDataGraphPipelineSessionBindPointRequirementsInfoARM<'_>,
    pBindPointRequirementCount: *mut u32,
    pBindPointRequirements: *mut vk::VkDataGraphPipelineSessionBindPointRequirementARM<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDataGraphPipelineSessionBindPointRequirementsARM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            pInfo,
            pBindPointRequirementCount,
            pBindPointRequirements,
        )
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetDataGraphPipelineSessionMemoryRequirementsARM(
    device: vk::VkDevice,
    pInfo: *const vk::VkDataGraphPipelineSessionMemoryRequirementsInfoARM<'_>,
    pMemoryRequirements: *mut vk::VkMemoryRequirements2<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDataGraphPipelineSessionMemoryRequirementsARM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pInfo, pMemoryRequirements);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetDeferredOperationMaxConcurrencyKHR(
    device: vk::VkDevice,
    operation: vk::VkDeferredOperationKHR,
) -> u32 {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDeferredOperationMaxConcurrencyKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, operation) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetDeferredOperationResultKHR(
    device: vk::VkDevice,
    operation: vk::VkDeferredOperationKHR,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDeferredOperationResultKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, operation) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetDescriptorEXT(
    device: vk::VkDevice,
    pDescriptorInfo: *const vk::VkDescriptorGetInfoEXT<'_>,
    dataSize: usize,
    pDescriptor: *mut c_void,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDescriptorEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pDescriptorInfo, dataSize, pDescriptor);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetDescriptorSetHostMappingVALVE(
    device: vk::VkDevice,
    descriptorSet: vk::VkDescriptorSet,
    ppData: *mut *mut c_void,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDescriptorSetHostMappingVALVE;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, descriptorSet, ppData);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetDescriptorSetLayoutBindingOffsetEXT(
    device: vk::VkDevice,
    layout: vk::VkDescriptorSetLayout,
    binding: u32,
    pOffset: *mut vk::VkDeviceSize,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDescriptorSetLayoutBindingOffsetEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, layout, binding, pOffset);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetDescriptorSetLayoutHostMappingInfoVALVE(
    device: vk::VkDevice,
    pBindingReference: *const vk::VkDescriptorSetBindingReferenceVALVE<'_>,
    pHostMapping: *mut vk::VkDescriptorSetLayoutHostMappingInfoVALVE<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDescriptorSetLayoutHostMappingInfoVALVE;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pBindingReference, pHostMapping);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetDescriptorSetLayoutSizeEXT(
    device: vk::VkDevice,
    layout: vk::VkDescriptorSetLayout,
    pLayoutSizeInBytes: *mut vk::VkDeviceSize,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDescriptorSetLayoutSizeEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, layout, pLayoutSizeInBytes);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetDescriptorSetLayoutSupport(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkDescriptorSetLayoutCreateInfo<'_>,
    pSupport: *mut vk::VkDescriptorSetLayoutSupport<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDescriptorSetLayoutSupport;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pCreateInfo, pSupport);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetDescriptorSetLayoutSupportKHR(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkDescriptorSetLayoutCreateInfo<'_>,
    pSupport: *mut vk::VkDescriptorSetLayoutSupportKHR<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDescriptorSetLayoutSupportKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pCreateInfo, pSupport);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetDeviceAccelerationStructureCompatibilityKHR(
    device: vk::VkDevice,
    pVersionInfo: *const vk::VkAccelerationStructureVersionInfoKHR<'_>,
    pCompatibility: *mut vk::VkAccelerationStructureCompatibilityKHR,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDeviceAccelerationStructureCompatibilityKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pVersionInfo, pCompatibility);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetDeviceBufferMemoryRequirements(
    device: vk::VkDevice,
    pInfo: *const vk::VkDeviceBufferMemoryRequirements<'_>,
    pMemoryRequirements: *mut vk::VkMemoryRequirements2<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDeviceBufferMemoryRequirements;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pInfo, pMemoryRequirements);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetDeviceBufferMemoryRequirementsKHR(
    device: vk::VkDevice,
    pInfo: *const vk::VkDeviceBufferMemoryRequirementsKHR<'_>,
    pMemoryRequirements: *mut vk::VkMemoryRequirements2<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDeviceBufferMemoryRequirementsKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pInfo, pMemoryRequirements);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetDeviceCombinedImageSamplerIndexNVX(
    device: vk::VkDevice,
    imageViewIndex: u64,
    samplerIndex: u64,
) -> u64 {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDeviceCombinedImageSamplerIndexNVX;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, imageViewIndex, samplerIndex) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetDeviceFaultDebugInfoKHR(
    device: vk::VkDevice,
    pDebugInfo: *mut vk::VkDeviceFaultDebugInfoKHR<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDeviceFaultDebugInfoKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pDebugInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetDeviceFaultInfoEXT(
    device: vk::VkDevice,
    pFaultCounts: *mut vk::VkDeviceFaultCountsEXT<'_>,
    pFaultInfo: *mut vk::VkDeviceFaultInfoEXT<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDeviceFaultInfoEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pFaultCounts, pFaultInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetDeviceFaultReportsKHR(
    device: vk::VkDevice,
    timeout: u64,
    pFaultCounts: *mut u32,
    pFaultInfo: *mut vk::VkDeviceFaultInfoKHR<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDeviceFaultReportsKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, timeout, pFaultCounts, pFaultInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetDeviceGroupPeerMemoryFeatures(
    device: vk::VkDevice,
    heapIndex: u32,
    localDeviceIndex: u32,
    remoteDeviceIndex: u32,
    pPeerMemoryFeatures: *mut vk::VkPeerMemoryFeatureFlags,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDeviceGroupPeerMemoryFeatures;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            heapIndex,
            localDeviceIndex,
            remoteDeviceIndex,
            pPeerMemoryFeatures,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetDeviceGroupPeerMemoryFeaturesKHR(
    device: vk::VkDevice,
    heapIndex: u32,
    localDeviceIndex: u32,
    remoteDeviceIndex: u32,
    pPeerMemoryFeatures: *mut vk::VkPeerMemoryFeatureFlagsKHR,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDeviceGroupPeerMemoryFeaturesKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            heapIndex,
            localDeviceIndex,
            remoteDeviceIndex,
            pPeerMemoryFeatures,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetDeviceGroupPresentCapabilitiesKHR(
    device: vk::VkDevice,
    pDeviceGroupPresentCapabilities: *mut vk::VkDeviceGroupPresentCapabilitiesKHR<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDeviceGroupPresentCapabilitiesKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pDeviceGroupPresentCapabilities) }
}
#[cfg(target_os = "windows")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetDeviceGroupSurfacePresentModes2EXT(
    device: vk::VkDevice,
    pSurfaceInfo: *const vk::VkPhysicalDeviceSurfaceInfo2KHR<'_>,
    pModes: *mut vk::VkDeviceGroupPresentModeFlagsKHR,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDeviceGroupSurfacePresentModes2EXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pSurfaceInfo, pModes) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetDeviceImageMemoryRequirements(
    device: vk::VkDevice,
    pInfo: *const vk::VkDeviceImageMemoryRequirements<'_>,
    pMemoryRequirements: *mut vk::VkMemoryRequirements2<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDeviceImageMemoryRequirements;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pInfo, pMemoryRequirements);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetDeviceImageMemoryRequirementsKHR(
    device: vk::VkDevice,
    pInfo: *const vk::VkDeviceImageMemoryRequirementsKHR<'_>,
    pMemoryRequirements: *mut vk::VkMemoryRequirements2<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDeviceImageMemoryRequirementsKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pInfo, pMemoryRequirements);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetDeviceImageSparseMemoryRequirements(
    device: vk::VkDevice,
    pInfo: *const vk::VkDeviceImageMemoryRequirements<'_>,
    pSparseMemoryRequirementCount: *mut u32,
    pSparseMemoryRequirements: *mut vk::VkSparseImageMemoryRequirements2<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDeviceImageSparseMemoryRequirements;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            pInfo,
            pSparseMemoryRequirementCount,
            pSparseMemoryRequirements,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetDeviceImageSparseMemoryRequirementsKHR(
    device: vk::VkDevice,
    pInfo: *const vk::VkDeviceImageMemoryRequirementsKHR<'_>,
    pSparseMemoryRequirementCount: *mut u32,
    pSparseMemoryRequirements: *mut vk::VkSparseImageMemoryRequirements2<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDeviceImageSparseMemoryRequirementsKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            pInfo,
            pSparseMemoryRequirementCount,
            pSparseMemoryRequirements,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetDeviceImageSubresourceLayout(
    device: vk::VkDevice,
    pInfo: *const vk::VkDeviceImageSubresourceInfo<'_>,
    pLayout: *mut vk::VkSubresourceLayout2<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDeviceImageSubresourceLayout;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pInfo, pLayout);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetDeviceImageSubresourceLayoutKHR(
    device: vk::VkDevice,
    pInfo: *const vk::VkDeviceImageSubresourceInfoKHR<'_>,
    pLayout: *mut vk::VkSubresourceLayout2KHR<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDeviceImageSubresourceLayoutKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pInfo, pLayout);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetDeviceMemoryCommitment(
    device: vk::VkDevice,
    memory: vk::VkDeviceMemory,
    pCommittedMemoryInBytes: *mut vk::VkDeviceSize,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDeviceMemoryCommitment;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, memory, pCommittedMemoryInBytes);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetDeviceMemoryOpaqueCaptureAddress(
    device: vk::VkDevice,
    pInfo: *const vk::VkDeviceMemoryOpaqueCaptureAddressInfo<'_>,
) -> u64 {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDeviceMemoryOpaqueCaptureAddress;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetDeviceMemoryOpaqueCaptureAddressKHR(
    device: vk::VkDevice,
    pInfo: *const vk::VkDeviceMemoryOpaqueCaptureAddressInfoKHR<'_>,
) -> u64 {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDeviceMemoryOpaqueCaptureAddressKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetDeviceMicromapCompatibilityEXT(
    device: vk::VkDevice,
    pVersionInfo: *const vk::VkMicromapVersionInfoEXT<'_>,
    pCompatibility: *mut vk::VkAccelerationStructureCompatibilityKHR,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDeviceMicromapCompatibilityEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pVersionInfo, pCompatibility);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetDeviceQueue(
    device: vk::VkDevice,
    queueFamilyIndex: u32,
    queueIndex: u32,
    pQueue: *mut vk::VkQueue,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDeviceQueue;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, queueFamilyIndex, queueIndex, pQueue);
    }
    if !pQueue.is_null() {
        let queue = unsafe { pQueue.read() };
        if queue != vk::VkQueue::NULL {
            unsafe {
                set_device_dispatchable(queue.0.cast(), core::ptr::from_ref(dispatch));
            }
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetDeviceQueue2(
    device: vk::VkDevice,
    pQueueInfo: *const vk::VkDeviceQueueInfo2<'_>,
    pQueue: *mut vk::VkQueue,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDeviceQueue2;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pQueueInfo, pQueue);
    }
    if !pQueue.is_null() {
        let queue = unsafe { pQueue.read() };
        if queue != vk::VkQueue::NULL {
            unsafe {
                set_device_dispatchable(queue.0.cast(), core::ptr::from_ref(dispatch));
            }
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI(
    device: vk::VkDevice,
    renderpass: vk::VkRenderPass,
    pMaxWorkgroupSize: *mut vk::VkExtent2D,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, renderpass, pMaxWorkgroupSize) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetDeviceTensorMemoryRequirementsARM(
    device: vk::VkDevice,
    pInfo: *const vk::VkDeviceTensorMemoryRequirementsARM<'_>,
    pMemoryRequirements: *mut vk::VkMemoryRequirements2<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDeviceTensorMemoryRequirementsARM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pInfo, pMemoryRequirements);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetDisplayModeProperties2KHR(
    physicalDevice: vk::VkPhysicalDevice,
    display: vk::VkDisplayKHR,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::VkDisplayModeProperties2KHR<'_>,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetDisplayModeProperties2KHR: Invalid physicalDevice [VUID-vkGetDisplayModeProperties2KHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetDisplayModeProperties2KHR
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(physicalDevice, display, pPropertyCount, pProperties)
        },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetDisplayModePropertiesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    display: vk::VkDisplayKHR,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::VkDisplayModePropertiesKHR,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetDisplayModePropertiesKHR: Invalid physicalDevice [VUID-vkGetDisplayModePropertiesKHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetDisplayModePropertiesKHR
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(physicalDevice, display, pPropertyCount, pProperties)
        },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetDisplayPlaneCapabilities2KHR(
    physicalDevice: vk::VkPhysicalDevice,
    pDisplayPlaneInfo: *const vk::VkDisplayPlaneInfo2KHR<'_>,
    pCapabilities: *mut vk::VkDisplayPlaneCapabilities2KHR<'_>,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetDisplayPlaneCapabilities2KHR: Invalid physicalDevice [VUID-vkGetDisplayPlaneCapabilities2KHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetDisplayPlaneCapabilities2KHR
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(physicalDevice, pDisplayPlaneInfo, pCapabilities)
        },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetDisplayPlaneCapabilitiesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    mode: vk::VkDisplayModeKHR,
    planeIndex: u32,
    pCapabilities: *mut vk::VkDisplayPlaneCapabilitiesKHR,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetDisplayPlaneCapabilitiesKHR: Invalid physicalDevice [VUID-vkGetDisplayPlaneCapabilitiesKHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetDisplayPlaneCapabilitiesKHR
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(physicalDevice, mode, planeIndex, pCapabilities)
        },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetDisplayPlaneSupportedDisplaysKHR(
    physicalDevice: vk::VkPhysicalDevice,
    planeIndex: u32,
    pDisplayCount: *mut u32,
    pDisplays: *mut vk::VkDisplayKHR,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetDisplayPlaneSupportedDisplaysKHR: Invalid physicalDevice [VUID-vkGetDisplayPlaneSupportedDisplaysKHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetDisplayPlaneSupportedDisplaysKHR
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(physicalDevice, planeIndex, pDisplayCount, pDisplays)
        },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetDrmDisplayEXT(
    physicalDevice: vk::VkPhysicalDevice,
    drmFd: i32,
    connectorId: u32,
    display: *mut vk::VkDisplayKHR,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetDrmDisplayEXT: Invalid physicalDevice [VUID-vkGetDrmDisplayEXT-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetDrmDisplayEXT
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, drmFd, connectorId, display) },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetDynamicRenderingTilePropertiesQCOM(
    device: vk::VkDevice,
    pRenderingInfo: *const vk::VkRenderingInfo<'_>,
    pProperties: *mut vk::VkTilePropertiesQCOM<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetDynamicRenderingTilePropertiesQCOM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pRenderingInfo, pProperties) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetEncodedVideoSessionParametersKHR(
    device: vk::VkDevice,
    pVideoSessionParametersInfo: *const vk::VkVideoEncodeSessionParametersGetInfoKHR<'_>,
    pFeedbackInfo: *mut vk::VkVideoEncodeSessionParametersFeedbackInfoKHR<'_>,
    pDataSize: *mut usize,
    pData: *mut c_void,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetEncodedVideoSessionParametersKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            pVideoSessionParametersInfo,
            pFeedbackInfo,
            pDataSize,
            pData,
        )
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetEventStatus(
    device: vk::VkDevice,
    event: vk::VkEvent,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetEventStatus;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, event) }
}
#[cfg(feature = "beta-extensions")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetExecutionGraphPipelineNodeIndexAMDX(
    device: vk::VkDevice,
    executionGraph: vk::VkPipeline,
    pNodeInfo: *const vk::VkPipelineShaderStageNodeCreateInfoAMDX<'_>,
    pNodeIndex: *mut u32,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetExecutionGraphPipelineNodeIndexAMDX;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, executionGraph, pNodeInfo, pNodeIndex) }
}
#[cfg(feature = "beta-extensions")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetExecutionGraphPipelineScratchSizeAMDX(
    device: vk::VkDevice,
    executionGraph: vk::VkPipeline,
    pSizeInfo: *mut vk::VkExecutionGraphPipelineScratchSizeAMDX<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetExecutionGraphPipelineScratchSizeAMDX;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, executionGraph, pSizeInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetExternalComputeQueueDataNV(
    externalQueue: vk::VkExternalComputeQueueNV,
    params: *mut vk::VkExternalComputeQueueDataParamsNV<'_>,
    pData: *mut c_void,
) {
    let dispatch = unsafe { device_dispatch(externalQueue.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetExternalComputeQueueDataNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(externalQueue, params, pData);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetFenceFdKHR(
    device: vk::VkDevice,
    pGetFdInfo: *const vk::VkFenceGetFdInfoKHR<'_>,
    pFd: *mut core::ffi::c_int,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetFenceFdKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pGetFdInfo, pFd) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetFenceStatus(
    device: vk::VkDevice,
    fence: vk::VkFence,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetFenceStatus;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, fence) }
}
#[cfg(target_os = "windows")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetFenceWin32HandleKHR(
    device: vk::VkDevice,
    pGetWin32HandleInfo: *const vk::VkFenceGetWin32HandleInfoKHR<'_>,
    pHandle: *mut vk::HANDLE,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetFenceWin32HandleKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pGetWin32HandleInfo, pHandle) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetFramebufferTilePropertiesQCOM(
    device: vk::VkDevice,
    framebuffer: vk::VkFramebuffer,
    pPropertiesCount: *mut u32,
    pProperties: *mut vk::VkTilePropertiesQCOM<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetFramebufferTilePropertiesQCOM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, framebuffer, pPropertiesCount, pProperties) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetGeneratedCommandsMemoryRequirementsEXT(
    device: vk::VkDevice,
    pInfo: *const vk::VkGeneratedCommandsMemoryRequirementsInfoEXT<'_>,
    pMemoryRequirements: *mut vk::VkMemoryRequirements2<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetGeneratedCommandsMemoryRequirementsEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pInfo, pMemoryRequirements);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetGeneratedCommandsMemoryRequirementsNV(
    device: vk::VkDevice,
    pInfo: *const vk::VkGeneratedCommandsMemoryRequirementsInfoNV<'_>,
    pMemoryRequirements: *mut vk::VkMemoryRequirements2<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetGeneratedCommandsMemoryRequirementsNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pInfo, pMemoryRequirements);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetGpaDeviceClockInfoAMD(
    device: vk::VkDevice,
    pInfo: *mut vk::VkGpaDeviceGetClockInfoAMD<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetGpaDeviceClockInfoAMD;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetGpaSessionResultsAMD(
    device: vk::VkDevice,
    gpaSession: vk::VkGpaSessionAMD,
    sampleID: u32,
    pSizeInBytes: *mut usize,
    pData: *mut c_void,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetGpaSessionResultsAMD;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, gpaSession, sampleID, pSizeInBytes, pData) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetGpaSessionStatusAMD(
    device: vk::VkDevice,
    gpaSession: vk::VkGpaSessionAMD,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetGpaSessionStatusAMD;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, gpaSession) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetImageDrmFormatModifierPropertiesEXT(
    device: vk::VkDevice,
    image: vk::VkImage,
    pProperties: *mut vk::VkImageDrmFormatModifierPropertiesEXT<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetImageDrmFormatModifierPropertiesEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, image, pProperties) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetImageMemoryRequirements(
    device: vk::VkDevice,
    image: vk::VkImage,
    pMemoryRequirements: *mut vk::VkMemoryRequirements,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetImageMemoryRequirements;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, image, pMemoryRequirements);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetImageMemoryRequirements2(
    device: vk::VkDevice,
    pInfo: *const vk::VkImageMemoryRequirementsInfo2<'_>,
    pMemoryRequirements: *mut vk::VkMemoryRequirements2<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetImageMemoryRequirements2;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pInfo, pMemoryRequirements);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetImageMemoryRequirements2KHR(
    device: vk::VkDevice,
    pInfo: *const vk::VkImageMemoryRequirementsInfo2KHR<'_>,
    pMemoryRequirements: *mut vk::VkMemoryRequirements2KHR<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetImageMemoryRequirements2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pInfo, pMemoryRequirements);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetImageOpaqueCaptureDataEXT(
    device: vk::VkDevice,
    imageCount: u32,
    pImages: *const vk::VkImage,
    pDatas: *mut vk::VkHostAddressRangeEXT<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetImageOpaqueCaptureDataEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, imageCount, pImages, pDatas) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetImageOpaqueCaptureDescriptorDataEXT(
    device: vk::VkDevice,
    pInfo: *const vk::VkImageCaptureDescriptorDataInfoEXT<'_>,
    pData: *mut c_void,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetImageOpaqueCaptureDescriptorDataEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pInfo, pData) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetImageSparseMemoryRequirements(
    device: vk::VkDevice,
    image: vk::VkImage,
    pSparseMemoryRequirementCount: *mut u32,
    pSparseMemoryRequirements: *mut vk::VkSparseImageMemoryRequirements,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetImageSparseMemoryRequirements;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            image,
            pSparseMemoryRequirementCount,
            pSparseMemoryRequirements,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetImageSparseMemoryRequirements2(
    device: vk::VkDevice,
    pInfo: *const vk::VkImageSparseMemoryRequirementsInfo2<'_>,
    pSparseMemoryRequirementCount: *mut u32,
    pSparseMemoryRequirements: *mut vk::VkSparseImageMemoryRequirements2<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetImageSparseMemoryRequirements2;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            pInfo,
            pSparseMemoryRequirementCount,
            pSparseMemoryRequirements,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetImageSparseMemoryRequirements2KHR(
    device: vk::VkDevice,
    pInfo: *const vk::VkImageSparseMemoryRequirementsInfo2KHR<'_>,
    pSparseMemoryRequirementCount: *mut u32,
    pSparseMemoryRequirements: *mut vk::VkSparseImageMemoryRequirements2KHR<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetImageSparseMemoryRequirements2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            pInfo,
            pSparseMemoryRequirementCount,
            pSparseMemoryRequirements,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetImageSubresourceLayout(
    device: vk::VkDevice,
    image: vk::VkImage,
    pSubresource: *const vk::VkImageSubresource,
    pLayout: *mut vk::VkSubresourceLayout,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetImageSubresourceLayout;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, image, pSubresource, pLayout);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetImageSubresourceLayout2(
    device: vk::VkDevice,
    image: vk::VkImage,
    pSubresource: *const vk::VkImageSubresource2<'_>,
    pLayout: *mut vk::VkSubresourceLayout2<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetImageSubresourceLayout2;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, image, pSubresource, pLayout);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetImageSubresourceLayout2EXT(
    device: vk::VkDevice,
    image: vk::VkImage,
    pSubresource: *const vk::VkImageSubresource2EXT<'_>,
    pLayout: *mut vk::VkSubresourceLayout2EXT<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetImageSubresourceLayout2EXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, image, pSubresource, pLayout);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetImageSubresourceLayout2KHR(
    device: vk::VkDevice,
    image: vk::VkImage,
    pSubresource: *const vk::VkImageSubresource2KHR<'_>,
    pLayout: *mut vk::VkSubresourceLayout2KHR<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetImageSubresourceLayout2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, image, pSubresource, pLayout);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetImageViewAddressNVX(
    device: vk::VkDevice,
    imageView: vk::VkImageView,
    pProperties: *mut vk::VkImageViewAddressPropertiesNVX<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetImageViewAddressNVX;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, imageView, pProperties) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetImageViewHandle64NVX(
    device: vk::VkDevice,
    pInfo: *const vk::VkImageViewHandleInfoNVX<'_>,
) -> u64 {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetImageViewHandle64NVX;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetImageViewHandleNVX(
    device: vk::VkDevice,
    pInfo: *const vk::VkImageViewHandleInfoNVX<'_>,
) -> u32 {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetImageViewHandleNVX;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetImageViewOpaqueCaptureDescriptorDataEXT(
    device: vk::VkDevice,
    pInfo: *const vk::VkImageViewCaptureDescriptorDataInfoEXT<'_>,
    pData: *mut c_void,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetImageViewOpaqueCaptureDescriptorDataEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pInfo, pData) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetLatencyTimingsLegacyNV(
    device: vk::VkDevice,
    pTimings: *mut c_void,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetLatencyTimingsLegacyNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pTimings);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetLatencyTimingsNV(
    device: vk::VkDevice,
    swapchain: vk::VkSwapchainKHR,
    pLatencyMarkerInfo: *mut vk::VkGetLatencyMarkerInfoNV<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetLatencyTimingsNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, swapchain, pLatencyMarkerInfo);
    }
}
#[cfg(target_os = "android")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetMemoryAndroidHardwareBufferANDROID(
    device: vk::VkDevice,
    pInfo: *const vk::VkMemoryGetAndroidHardwareBufferInfoANDROID<'_>,
    pBuffer: *mut *mut vk::AHardwareBuffer,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetMemoryAndroidHardwareBufferANDROID;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pInfo, pBuffer) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetMemoryFdKHR(
    device: vk::VkDevice,
    pGetFdInfo: *const vk::VkMemoryGetFdInfoKHR<'_>,
    pFd: *mut core::ffi::c_int,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetMemoryFdKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pGetFdInfo, pFd) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetMemoryFdPropertiesKHR(
    device: vk::VkDevice,
    handleType: vk::VkExternalMemoryHandleTypeFlagBits,
    fd: core::ffi::c_int,
    pMemoryFdProperties: *mut vk::VkMemoryFdPropertiesKHR<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetMemoryFdPropertiesKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, handleType, fd, pMemoryFdProperties) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetMemoryHostPointerPropertiesEXT(
    device: vk::VkDevice,
    handleType: vk::VkExternalMemoryHandleTypeFlagBits,
    pHostPointer: *const c_void,
    pMemoryHostPointerProperties: *mut vk::VkMemoryHostPointerPropertiesEXT<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetMemoryHostPointerPropertiesEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            handleType,
            pHostPointer,
            pMemoryHostPointerProperties,
        )
    }
}
#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "visionos"
))]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetMemoryMetalHandleEXT(
    device: vk::VkDevice,
    pGetMetalHandleInfo: *const vk::VkMemoryGetMetalHandleInfoEXT<'_>,
    pHandle: *mut *mut c_void,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetMemoryMetalHandleEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pGetMetalHandleInfo, pHandle) }
}
#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "visionos"
))]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetMemoryMetalHandlePropertiesEXT(
    device: vk::VkDevice,
    handleType: vk::VkExternalMemoryHandleTypeFlagBits,
    pHandle: *const c_void,
    pMemoryMetalHandleProperties: *mut vk::VkMemoryMetalHandlePropertiesEXT<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetMemoryMetalHandlePropertiesEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, handleType, pHandle, pMemoryMetalHandleProperties) }
}
#[cfg(target_env = "ohos")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetMemoryNativeBufferOHOS(
    device: vk::VkDevice,
    pInfo: *const vk::VkMemoryGetNativeBufferInfoOHOS<'_>,
    pBuffer: *mut *mut vk::OH_NativeBuffer,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetMemoryNativeBufferOHOS;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pInfo, pBuffer) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetMemoryRemoteAddressNV(
    device: vk::VkDevice,
    pMemoryGetRemoteAddressInfo: *const vk::VkMemoryGetRemoteAddressInfoNV<'_>,
    pAddress: *mut vk::VkRemoteAddressNV,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetMemoryRemoteAddressNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pMemoryGetRemoteAddressInfo, pAddress) }
}
#[cfg(target_os = "windows")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetMemoryWin32HandleKHR(
    device: vk::VkDevice,
    pGetWin32HandleInfo: *const vk::VkMemoryGetWin32HandleInfoKHR<'_>,
    pHandle: *mut vk::HANDLE,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetMemoryWin32HandleKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pGetWin32HandleInfo, pHandle) }
}
#[cfg(target_os = "windows")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetMemoryWin32HandleNV(
    device: vk::VkDevice,
    memory: vk::VkDeviceMemory,
    handleType: vk::VkExternalMemoryHandleTypeFlagsNV,
    pHandle: *mut vk::HANDLE,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetMemoryWin32HandleNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, memory, handleType, pHandle) }
}
#[cfg(target_os = "windows")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetMemoryWin32HandlePropertiesKHR(
    device: vk::VkDevice,
    handleType: vk::VkExternalMemoryHandleTypeFlagBits,
    handle: vk::HANDLE,
    pMemoryWin32HandleProperties: *mut vk::VkMemoryWin32HandlePropertiesKHR<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetMemoryWin32HandlePropertiesKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, handleType, handle, pMemoryWin32HandleProperties) }
}
#[cfg(target_os = "fuchsia")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetMemoryZirconHandleFUCHSIA(
    device: vk::VkDevice,
    pGetZirconHandleInfo: *const vk::VkMemoryGetZirconHandleInfoFUCHSIA<'_>,
    pZirconHandle: *mut vk::zx_handle_t,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetMemoryZirconHandleFUCHSIA;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pGetZirconHandleInfo, pZirconHandle) }
}
#[cfg(target_os = "fuchsia")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetMemoryZirconHandlePropertiesFUCHSIA(
    device: vk::VkDevice,
    handleType: vk::VkExternalMemoryHandleTypeFlagBits,
    zirconHandle: vk::zx_handle_t,
    pMemoryZirconHandleProperties: *mut vk::VkMemoryZirconHandlePropertiesFUCHSIA<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetMemoryZirconHandlePropertiesFUCHSIA;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            handleType,
            zirconHandle,
            pMemoryZirconHandleProperties,
        )
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetMicromapBuildSizesEXT(
    device: vk::VkDevice,
    buildType: vk::VkAccelerationStructureBuildTypeKHR,
    pBuildInfo: *const vk::VkMicromapBuildInfoEXT<'_>,
    pSizeInfo: *mut vk::VkMicromapBuildSizesInfoEXT<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetMicromapBuildSizesEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, buildType, pBuildInfo, pSizeInfo);
    }
}
#[cfg(target_env = "ohos")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetNativeBufferPropertiesOHOS(
    device: vk::VkDevice,
    buffer: *const vk::OH_NativeBuffer,
    pProperties: *mut vk::VkNativeBufferPropertiesOHOS<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetNativeBufferPropertiesOHOS;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, buffer, pProperties) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPartitionedAccelerationStructuresBuildSizesNV(
    device: vk::VkDevice,
    pInfo: *const vk::VkPartitionedAccelerationStructureInstancesInputNV<'_>,
    pSizeInfo: *mut vk::VkAccelerationStructureBuildSizesInfoKHR<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetPartitionedAccelerationStructuresBuildSizesNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pInfo, pSizeInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPastPresentationTimingEXT(
    device: vk::VkDevice,
    pPastPresentationTimingInfo: *const vk::VkPastPresentationTimingInfoEXT<'_>,
    pPastPresentationTimingProperties: *mut vk::VkPastPresentationTimingPropertiesEXT<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetPastPresentationTimingEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            pPastPresentationTimingInfo,
            pPastPresentationTimingProperties,
        )
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPastPresentationTimingGOOGLE(
    device: vk::VkDevice,
    swapchain: vk::VkSwapchainKHR,
    pPresentationTimingCount: *mut u32,
    pPresentationTimings: *mut vk::VkPastPresentationTimingGOOGLE,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetPastPresentationTimingGOOGLE;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            swapchain,
            pPresentationTimingCount,
            pPresentationTimings,
        )
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPerformanceParameterINTEL(
    device: vk::VkDevice,
    parameter: vk::VkPerformanceParameterTypeINTEL,
    pValue: *mut vk::VkPerformanceValueINTEL<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetPerformanceParameterINTEL;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, parameter, pValue) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceCalibrateableTimeDomainsEXT(
    physicalDevice: vk::VkPhysicalDevice,
    pTimeDomainCount: *mut u32,
    pTimeDomains: *mut vk::VkTimeDomainEXT,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceCalibrateableTimeDomainsEXT: Invalid physicalDevice [VUID-vkGetPhysicalDeviceCalibrateableTimeDomainsEXT-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceCalibrateableTimeDomainsEXT
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(physicalDevice, pTimeDomainCount, pTimeDomains)
        },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceCalibrateableTimeDomainsKHR(
    physicalDevice: vk::VkPhysicalDevice,
    pTimeDomainCount: *mut u32,
    pTimeDomains: *mut vk::VkTimeDomainKHR,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceCalibrateableTimeDomainsKHR: Invalid physicalDevice [VUID-vkGetPhysicalDeviceCalibrateableTimeDomainsKHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceCalibrateableTimeDomainsKHR
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(physicalDevice, pTimeDomainCount, pTimeDomains)
        },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV(
    physicalDevice: vk::VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::VkCooperativeMatrixFlexibleDimensionsPropertiesNV<'_>,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV: Invalid physicalDevice [VUID-vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, pPropertyCount, pProperties) },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceCooperativeMatrixProperties2EXT(
    physicalDevice: vk::VkPhysicalDevice,
    pCooperativeMatrixInfo: *const vk::VkPhysicalDeviceCooperativeMatrixInfo2EXT<'_>,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::VkCooperativeMatrixProperties2EXT<'_>,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceCooperativeMatrixProperties2EXT: Invalid physicalDevice [VUID-vkGetPhysicalDeviceCooperativeMatrixProperties2EXT-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceCooperativeMatrixProperties2EXT
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(
                physicalDevice,
                pCooperativeMatrixInfo,
                pPropertyCount,
                pProperties,
            )
        },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::VkCooperativeMatrixPropertiesKHR<'_>,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR: Invalid physicalDevice [VUID-vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, pPropertyCount, pProperties) },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceCooperativeMatrixPropertiesNV(
    physicalDevice: vk::VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::VkCooperativeMatrixPropertiesNV<'_>,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceCooperativeMatrixPropertiesNV: Invalid physicalDevice [VUID-vkGetPhysicalDeviceCooperativeMatrixPropertiesNV-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceCooperativeMatrixPropertiesNV
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, pPropertyCount, pProperties) },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceCooperativeVectorPropertiesNV(
    physicalDevice: vk::VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::VkCooperativeVectorPropertiesNV<'_>,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceCooperativeVectorPropertiesNV: Invalid physicalDevice [VUID-vkGetPhysicalDeviceCooperativeVectorPropertiesNV-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceCooperativeVectorPropertiesNV
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, pPropertyCount, pProperties) },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceDescriptorSizeEXT(
    physicalDevice: vk::VkPhysicalDevice,
    descriptorType: vk::VkDescriptorType,
) -> vk::VkDeviceSize {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceDescriptorSizeEXT: Invalid physicalDevice [VUID-vkGetPhysicalDeviceDescriptorSizeEXT-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceDescriptorSizeEXT
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            unsafe { core::mem::zeroed::<vk::VkDeviceSize>() }
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, descriptorType) },
    )
}
#[cfg(feature = "wsi-directfb")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceDirectFBPresentationSupportEXT(
    physicalDevice: vk::VkPhysicalDevice,
    queueFamilyIndex: u32,
    dfb: *mut vk::IDirectFB,
) -> vk::VkBool32 {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceDirectFBPresentationSupportEXT: Invalid physicalDevice [VUID-vkGetPhysicalDeviceDirectFBPresentationSupportEXT-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceDirectFBPresentationSupportEXT
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            unsafe { core::mem::zeroed::<vk::VkBool32>() }
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, queueFamilyIndex, dfb) },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceDisplayPlaneProperties2KHR(
    physicalDevice: vk::VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::VkDisplayPlaneProperties2KHR<'_>,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceDisplayPlaneProperties2KHR: Invalid physicalDevice [VUID-vkGetPhysicalDeviceDisplayPlaneProperties2KHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceDisplayPlaneProperties2KHR
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, pPropertyCount, pProperties) },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceDisplayPlanePropertiesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::VkDisplayPlanePropertiesKHR,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceDisplayPlanePropertiesKHR: Invalid physicalDevice [VUID-vkGetPhysicalDeviceDisplayPlanePropertiesKHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceDisplayPlanePropertiesKHR
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, pPropertyCount, pProperties) },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceDisplayProperties2KHR(
    physicalDevice: vk::VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::VkDisplayProperties2KHR<'_>,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceDisplayProperties2KHR: Invalid physicalDevice [VUID-vkGetPhysicalDeviceDisplayProperties2KHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceDisplayProperties2KHR
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, pPropertyCount, pProperties) },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceDisplayPropertiesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::VkDisplayPropertiesKHR<'_>,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceDisplayPropertiesKHR: Invalid physicalDevice [VUID-vkGetPhysicalDeviceDisplayPropertiesKHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceDisplayPropertiesKHR
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, pPropertyCount, pProperties) },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceExternalBufferProperties(
    physicalDevice: vk::VkPhysicalDevice,
    pExternalBufferInfo: *const vk::VkPhysicalDeviceExternalBufferInfo<'_>,
    pExternalBufferProperties: *mut vk::VkExternalBufferProperties<'_>,
) {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceExternalBufferProperties: Invalid physicalDevice [VUID-vkGetPhysicalDeviceExternalBufferProperties-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceExternalBufferProperties
        .map(|command| (command, physicalDevice));
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(
                physicalDevice,
                pExternalBufferInfo,
                pExternalBufferProperties,
            );
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceExternalBufferPropertiesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    pExternalBufferInfo: *const vk::VkPhysicalDeviceExternalBufferInfoKHR<'_>,
    pExternalBufferProperties: *mut vk::VkExternalBufferPropertiesKHR<'_>,
) {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceExternalBufferPropertiesKHR: Invalid physicalDevice [VUID-vkGetPhysicalDeviceExternalBufferPropertiesKHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceExternalBufferPropertiesKHR
        .map(|command| (command, physicalDevice));
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(
                physicalDevice,
                pExternalBufferInfo,
                pExternalBufferProperties,
            );
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceExternalFenceProperties(
    physicalDevice: vk::VkPhysicalDevice,
    pExternalFenceInfo: *const vk::VkPhysicalDeviceExternalFenceInfo<'_>,
    pExternalFenceProperties: *mut vk::VkExternalFenceProperties<'_>,
) {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceExternalFenceProperties: Invalid physicalDevice [VUID-vkGetPhysicalDeviceExternalFenceProperties-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceExternalFenceProperties
        .map(|command| (command, physicalDevice));
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(physicalDevice, pExternalFenceInfo, pExternalFenceProperties);
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceExternalFencePropertiesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    pExternalFenceInfo: *const vk::VkPhysicalDeviceExternalFenceInfoKHR<'_>,
    pExternalFenceProperties: *mut vk::VkExternalFencePropertiesKHR<'_>,
) {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceExternalFencePropertiesKHR: Invalid physicalDevice [VUID-vkGetPhysicalDeviceExternalFencePropertiesKHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceExternalFencePropertiesKHR
        .map(|command| (command, physicalDevice));
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(physicalDevice, pExternalFenceInfo, pExternalFenceProperties);
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceExternalImageFormatPropertiesNV(
    physicalDevice: vk::VkPhysicalDevice,
    format: vk::VkFormat,
    type_: vk::VkImageType,
    tiling: vk::VkImageTiling,
    usage: vk::VkImageUsageFlags,
    flags: vk::VkImageCreateFlags,
    externalHandleType: vk::VkExternalMemoryHandleTypeFlagsNV,
    pExternalImageFormatProperties: *mut vk::VkExternalImageFormatPropertiesNV,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceExternalImageFormatPropertiesNV: Invalid physicalDevice [VUID-vkGetPhysicalDeviceExternalImageFormatPropertiesNV-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceExternalImageFormatPropertiesNV
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(
                physicalDevice,
                format,
                type_,
                tiling,
                usage,
                flags,
                externalHandleType,
                pExternalImageFormatProperties,
            )
        },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceExternalSemaphoreProperties(
    physicalDevice: vk::VkPhysicalDevice,
    pExternalSemaphoreInfo: *const vk::VkPhysicalDeviceExternalSemaphoreInfo<'_>,
    pExternalSemaphoreProperties: *mut vk::VkExternalSemaphoreProperties<'_>,
) {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceExternalSemaphoreProperties: Invalid physicalDevice [VUID-vkGetPhysicalDeviceExternalSemaphoreProperties-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceExternalSemaphoreProperties
        .map(|command| (command, physicalDevice));
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(
                physicalDevice,
                pExternalSemaphoreInfo,
                pExternalSemaphoreProperties,
            );
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceExternalSemaphorePropertiesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    pExternalSemaphoreInfo: *const vk::VkPhysicalDeviceExternalSemaphoreInfoKHR<'_>,
    pExternalSemaphoreProperties: *mut vk::VkExternalSemaphorePropertiesKHR<'_>,
) {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceExternalSemaphorePropertiesKHR: Invalid physicalDevice [VUID-vkGetPhysicalDeviceExternalSemaphorePropertiesKHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceExternalSemaphorePropertiesKHR
        .map(|command| (command, physicalDevice));
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(
                physicalDevice,
                pExternalSemaphoreInfo,
                pExternalSemaphoreProperties,
            );
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceExternalTensorPropertiesARM(
    physicalDevice: vk::VkPhysicalDevice,
    pExternalTensorInfo: *const vk::VkPhysicalDeviceExternalTensorInfoARM<'_>,
    pExternalTensorProperties: *mut vk::VkExternalTensorPropertiesARM<'_>,
) {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceExternalTensorPropertiesARM: Invalid physicalDevice [VUID-vkGetPhysicalDeviceExternalTensorPropertiesARM-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceExternalTensorPropertiesARM
        .map(|command| (command, physicalDevice));
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(
                physicalDevice,
                pExternalTensorInfo,
                pExternalTensorProperties,
            );
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceFeatures(
    physicalDevice: vk::VkPhysicalDevice,
    pFeatures: *mut vk::VkPhysicalDeviceFeatures,
) {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceFeatures: Invalid physicalDevice [VUID-vkGetPhysicalDeviceFeatures-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceFeatures
        .map(|command| (command, physicalDevice));
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(physicalDevice, pFeatures);
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceFeatures2(
    physicalDevice: vk::VkPhysicalDevice,
    pFeatures: *mut vk::VkPhysicalDeviceFeatures2<'_>,
) {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceFeatures2: Invalid physicalDevice [VUID-vkGetPhysicalDeviceFeatures2-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceFeatures2
        .map(|command| (command, physicalDevice));
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(physicalDevice, pFeatures);
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceFeatures2KHR(
    physicalDevice: vk::VkPhysicalDevice,
    pFeatures: *mut vk::VkPhysicalDeviceFeatures2KHR<'_>,
) {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceFeatures2KHR: Invalid physicalDevice [VUID-vkGetPhysicalDeviceFeatures2KHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceFeatures2KHR
        .map(|command| (command, physicalDevice));
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(physicalDevice, pFeatures);
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceFormatProperties(
    physicalDevice: vk::VkPhysicalDevice,
    format: vk::VkFormat,
    pFormatProperties: *mut vk::VkFormatProperties,
) {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceFormatProperties: Invalid physicalDevice [VUID-vkGetPhysicalDeviceFormatProperties-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceFormatProperties
        .map(|command| (command, physicalDevice));
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(physicalDevice, format, pFormatProperties);
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceFormatProperties2(
    physicalDevice: vk::VkPhysicalDevice,
    format: vk::VkFormat,
    pFormatProperties: *mut vk::VkFormatProperties2<'_>,
) {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceFormatProperties2: Invalid physicalDevice [VUID-vkGetPhysicalDeviceFormatProperties2-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceFormatProperties2
        .map(|command| (command, physicalDevice));
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(physicalDevice, format, pFormatProperties);
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceFormatProperties2KHR(
    physicalDevice: vk::VkPhysicalDevice,
    format: vk::VkFormat,
    pFormatProperties: *mut vk::VkFormatProperties2KHR<'_>,
) {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceFormatProperties2KHR: Invalid physicalDevice [VUID-vkGetPhysicalDeviceFormatProperties2KHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceFormatProperties2KHR
        .map(|command| (command, physicalDevice));
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(physicalDevice, format, pFormatProperties);
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceFragmentShadingRatesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    pFragmentShadingRateCount: *mut u32,
    pFragmentShadingRates: *mut vk::VkPhysicalDeviceFragmentShadingRateKHR<'_>,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceFragmentShadingRatesKHR: Invalid physicalDevice [VUID-vkGetPhysicalDeviceFragmentShadingRatesKHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceFragmentShadingRatesKHR
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(
                physicalDevice,
                pFragmentShadingRateCount,
                pFragmentShadingRates,
            )
        },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceImageFormatProperties(
    physicalDevice: vk::VkPhysicalDevice,
    format: vk::VkFormat,
    type_: vk::VkImageType,
    tiling: vk::VkImageTiling,
    usage: vk::VkImageUsageFlags,
    flags: vk::VkImageCreateFlags,
    pImageFormatProperties: *mut vk::VkImageFormatProperties,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceImageFormatProperties: Invalid physicalDevice [VUID-vkGetPhysicalDeviceImageFormatProperties-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceImageFormatProperties
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(
                physicalDevice,
                format,
                type_,
                tiling,
                usage,
                flags,
                pImageFormatProperties,
            )
        },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceImageFormatProperties2(
    physicalDevice: vk::VkPhysicalDevice,
    pImageFormatInfo: *const vk::VkPhysicalDeviceImageFormatInfo2<'_>,
    pImageFormatProperties: *mut vk::VkImageFormatProperties2<'_>,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceImageFormatProperties2: Invalid physicalDevice [VUID-vkGetPhysicalDeviceImageFormatProperties2-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceImageFormatProperties2
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(physicalDevice, pImageFormatInfo, pImageFormatProperties)
        },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceImageFormatProperties2KHR(
    physicalDevice: vk::VkPhysicalDevice,
    pImageFormatInfo: *const vk::VkPhysicalDeviceImageFormatInfo2KHR<'_>,
    pImageFormatProperties: *mut vk::VkImageFormatProperties2KHR<'_>,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceImageFormatProperties2KHR: Invalid physicalDevice [VUID-vkGetPhysicalDeviceImageFormatProperties2KHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceImageFormatProperties2KHR
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(physicalDevice, pImageFormatInfo, pImageFormatProperties)
        },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceMemoryProperties(
    physicalDevice: vk::VkPhysicalDevice,
    pMemoryProperties: *mut vk::VkPhysicalDeviceMemoryProperties,
) {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceMemoryProperties: Invalid physicalDevice [VUID-vkGetPhysicalDeviceMemoryProperties-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceMemoryProperties
        .map(|command| (command, physicalDevice));
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(physicalDevice, pMemoryProperties);
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceMemoryProperties2(
    physicalDevice: vk::VkPhysicalDevice,
    pMemoryProperties: *mut vk::VkPhysicalDeviceMemoryProperties2<'_>,
) {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceMemoryProperties2: Invalid physicalDevice [VUID-vkGetPhysicalDeviceMemoryProperties2-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceMemoryProperties2
        .map(|command| (command, physicalDevice));
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(physicalDevice, pMemoryProperties);
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceMemoryProperties2KHR(
    physicalDevice: vk::VkPhysicalDevice,
    pMemoryProperties: *mut vk::VkPhysicalDeviceMemoryProperties2KHR<'_>,
) {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceMemoryProperties2KHR: Invalid physicalDevice [VUID-vkGetPhysicalDeviceMemoryProperties2KHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceMemoryProperties2KHR
        .map(|command| (command, physicalDevice));
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(physicalDevice, pMemoryProperties);
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceMultisamplePropertiesEXT(
    physicalDevice: vk::VkPhysicalDevice,
    samples: vk::VkSampleCountFlagBits,
    pMultisampleProperties: *mut vk::VkMultisamplePropertiesEXT<'_>,
) {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceMultisamplePropertiesEXT: Invalid physicalDevice [VUID-vkGetPhysicalDeviceMultisamplePropertiesEXT-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceMultisamplePropertiesEXT
        .map(|command| (command, physicalDevice));
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(physicalDevice, samples, pMultisampleProperties);
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceOpticalFlowImageFormatsNV(
    physicalDevice: vk::VkPhysicalDevice,
    pOpticalFlowImageFormatInfo: *const vk::VkOpticalFlowImageFormatInfoNV<'_>,
    pFormatCount: *mut u32,
    pImageFormatProperties: *mut vk::VkOpticalFlowImageFormatPropertiesNV<'_>,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceOpticalFlowImageFormatsNV: Invalid physicalDevice [VUID-vkGetPhysicalDeviceOpticalFlowImageFormatsNV-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceOpticalFlowImageFormatsNV
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(
                physicalDevice,
                pOpticalFlowImageFormatInfo,
                pFormatCount,
                pImageFormatProperties,
            )
        },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDevicePresentRectanglesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    surface: vk::VkSurfaceKHR,
    pRectCount: *mut u32,
    pRects: *mut vk::VkRect2D,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDevicePresentRectanglesKHR: Invalid physicalDevice [VUID-vkGetPhysicalDevicePresentRectanglesKHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDevicePresentRectanglesKHR
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, surface, pRectCount, pRects) },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceProperties(
    physicalDevice: vk::VkPhysicalDevice,
    pProperties: *mut vk::VkPhysicalDeviceProperties,
) {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceProperties: Invalid physicalDevice [VUID-vkGetPhysicalDeviceProperties-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceProperties
        .map(|command| (command, physicalDevice));
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(physicalDevice, pProperties);
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceProperties2(
    physicalDevice: vk::VkPhysicalDevice,
    pProperties: *mut vk::VkPhysicalDeviceProperties2<'_>,
) {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceProperties2: Invalid physicalDevice [VUID-vkGetPhysicalDeviceProperties2-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceProperties2
        .map(|command| (command, physicalDevice));
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(physicalDevice, pProperties);
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceProperties2KHR(
    physicalDevice: vk::VkPhysicalDevice,
    pProperties: *mut vk::VkPhysicalDeviceProperties2KHR<'_>,
) {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceProperties2KHR: Invalid physicalDevice [VUID-vkGetPhysicalDeviceProperties2KHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceProperties2KHR
        .map(|command| (command, physicalDevice));
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(physicalDevice, pProperties);
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM(
    physicalDevice: vk::VkPhysicalDevice,
    queueFamilyIndex: u32,
    pQueueFamilyDataGraphProperties: *const vk::VkQueueFamilyDataGraphPropertiesARM<'_>,
    pProperties: *mut vk::VkBaseOutStructure<'_>,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM: Invalid physicalDevice [VUID-vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(
                physicalDevice,
                queueFamilyIndex,
                pQueueFamilyDataGraphProperties,
                pProperties,
            )
        },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM(
    physicalDevice: vk::VkPhysicalDevice,
    queueFamilyIndex: u32,
    pQueueFamilyDataGraphProperties: *const vk::VkQueueFamilyDataGraphPropertiesARM<'_>,
    pOpticalFlowImageFormatInfo: *const vk::VkDataGraphOpticalFlowImageFormatInfoARM<'_>,
    pFormatCount: *mut u32,
    pImageFormatProperties: *mut vk::VkDataGraphOpticalFlowImageFormatPropertiesARM<'_>,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM: Invalid physicalDevice [VUID-vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(
                physicalDevice,
                queueFamilyIndex,
                pQueueFamilyDataGraphProperties,
                pOpticalFlowImageFormatInfo,
                pFormatCount,
                pImageFormatProperties,
            )
        },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM(
    physicalDevice: vk::VkPhysicalDevice,
    pQueueFamilyDataGraphProcessingEngineInfo: *const vk::VkPhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM<
        '_,
    >,
    pQueueFamilyDataGraphProcessingEngineProperties: *mut vk::VkQueueFamilyDataGraphProcessingEnginePropertiesARM<
        '_,
    >,
) {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM: Invalid physicalDevice [VUID-vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM
        .map(|command| (command, physicalDevice));
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(
                physicalDevice,
                pQueueFamilyDataGraphProcessingEngineInfo,
                pQueueFamilyDataGraphProcessingEngineProperties,
            );
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM(
    physicalDevice: vk::VkPhysicalDevice,
    queueFamilyIndex: u32,
    pQueueFamilyDataGraphPropertyCount: *mut u32,
    pQueueFamilyDataGraphProperties: *mut vk::VkQueueFamilyDataGraphPropertiesARM<'_>,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM: Invalid physicalDevice [VUID-vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(
                physicalDevice,
                queueFamilyIndex,
                pQueueFamilyDataGraphPropertyCount,
                pQueueFamilyDataGraphProperties,
            )
        },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    pPerformanceQueryCreateInfo: *const vk::VkQueryPoolPerformanceCreateInfoKHR<'_>,
    pNumPasses: *mut u32,
) {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR: Invalid physicalDevice [VUID-vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR
        .map(|command| (command, physicalDevice));
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(physicalDevice, pPerformanceQueryCreateInfo, pNumPasses);
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceQueueFamilyProperties(
    physicalDevice: vk::VkPhysicalDevice,
    pQueueFamilyPropertyCount: *mut u32,
    pQueueFamilyProperties: *mut vk::VkQueueFamilyProperties,
) {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceQueueFamilyProperties: Invalid physicalDevice [VUID-vkGetPhysicalDeviceQueueFamilyProperties-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceQueueFamilyProperties
        .map(|command| (command, physicalDevice));
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(
                physicalDevice,
                pQueueFamilyPropertyCount,
                pQueueFamilyProperties,
            );
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceQueueFamilyProperties2(
    physicalDevice: vk::VkPhysicalDevice,
    pQueueFamilyPropertyCount: *mut u32,
    pQueueFamilyProperties: *mut vk::VkQueueFamilyProperties2<'_>,
) {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceQueueFamilyProperties2: Invalid physicalDevice [VUID-vkGetPhysicalDeviceQueueFamilyProperties2-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceQueueFamilyProperties2
        .map(|command| (command, physicalDevice));
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(
                physicalDevice,
                pQueueFamilyPropertyCount,
                pQueueFamilyProperties,
            );
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceQueueFamilyProperties2KHR(
    physicalDevice: vk::VkPhysicalDevice,
    pQueueFamilyPropertyCount: *mut u32,
    pQueueFamilyProperties: *mut vk::VkQueueFamilyProperties2KHR<'_>,
) {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceQueueFamilyProperties2KHR: Invalid physicalDevice [VUID-vkGetPhysicalDeviceQueueFamilyProperties2KHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceQueueFamilyProperties2KHR
        .map(|command| (command, physicalDevice));
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(
                physicalDevice,
                pQueueFamilyPropertyCount,
                pQueueFamilyProperties,
            );
        }
    }
}
#[cfg(any(target_os = "nto", target_os = "qnx"))]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceScreenPresentationSupportQNX(
    physicalDevice: vk::VkPhysicalDevice,
    queueFamilyIndex: u32,
    window: *mut vk::_screen_window,
) -> vk::VkBool32 {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceScreenPresentationSupportQNX: Invalid physicalDevice [VUID-vkGetPhysicalDeviceScreenPresentationSupportQNX-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceScreenPresentationSupportQNX
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            unsafe { core::mem::zeroed::<vk::VkBool32>() }
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, queueFamilyIndex, window) },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceSparseImageFormatProperties(
    physicalDevice: vk::VkPhysicalDevice,
    format: vk::VkFormat,
    type_: vk::VkImageType,
    samples: vk::VkSampleCountFlagBits,
    usage: vk::VkImageUsageFlags,
    tiling: vk::VkImageTiling,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::VkSparseImageFormatProperties,
) {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceSparseImageFormatProperties: Invalid physicalDevice [VUID-vkGetPhysicalDeviceSparseImageFormatProperties-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceSparseImageFormatProperties
        .map(|command| (command, physicalDevice));
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(
                physicalDevice,
                format,
                type_,
                samples,
                usage,
                tiling,
                pPropertyCount,
                pProperties,
            );
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceSparseImageFormatProperties2(
    physicalDevice: vk::VkPhysicalDevice,
    pFormatInfo: *const vk::VkPhysicalDeviceSparseImageFormatInfo2<'_>,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::VkSparseImageFormatProperties2<'_>,
) {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceSparseImageFormatProperties2: Invalid physicalDevice [VUID-vkGetPhysicalDeviceSparseImageFormatProperties2-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceSparseImageFormatProperties2
        .map(|command| (command, physicalDevice));
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(physicalDevice, pFormatInfo, pPropertyCount, pProperties);
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceSparseImageFormatProperties2KHR(
    physicalDevice: vk::VkPhysicalDevice,
    pFormatInfo: *const vk::VkPhysicalDeviceSparseImageFormatInfo2KHR<'_>,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::VkSparseImageFormatProperties2KHR<'_>,
) {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceSparseImageFormatProperties2KHR: Invalid physicalDevice [VUID-vkGetPhysicalDeviceSparseImageFormatProperties2KHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceSparseImageFormatProperties2KHR
        .map(|command| (command, physicalDevice));
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(physicalDevice, pFormatInfo, pPropertyCount, pProperties);
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV(
    physicalDevice: vk::VkPhysicalDevice,
    pCombinationCount: *mut u32,
    pCombinations: *mut vk::VkFramebufferMixedSamplesCombinationNV<'_>,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV: Invalid physicalDevice [VUID-vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(physicalDevice, pCombinationCount, pCombinations)
        },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceSurfaceCapabilities2EXT(
    physicalDevice: vk::VkPhysicalDevice,
    surface: vk::VkSurfaceKHR,
    pSurfaceCapabilities: *mut vk::VkSurfaceCapabilities2EXT<'_>,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceSurfaceCapabilities2EXT: Invalid physicalDevice [VUID-vkGetPhysicalDeviceSurfaceCapabilities2EXT-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceSurfaceCapabilities2EXT
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(physicalDevice, surface, pSurfaceCapabilities)
        },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceSurfaceCapabilities2KHR(
    physicalDevice: vk::VkPhysicalDevice,
    pSurfaceInfo: *const vk::VkPhysicalDeviceSurfaceInfo2KHR<'_>,
    pSurfaceCapabilities: *mut vk::VkSurfaceCapabilities2KHR<'_>,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceSurfaceCapabilities2KHR: Invalid physicalDevice [VUID-vkGetPhysicalDeviceSurfaceCapabilities2KHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceSurfaceCapabilities2KHR
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(physicalDevice, pSurfaceInfo, pSurfaceCapabilities)
        },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    surface: vk::VkSurfaceKHR,
    pSurfaceCapabilities: *mut vk::VkSurfaceCapabilitiesKHR,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceSurfaceCapabilitiesKHR: Invalid physicalDevice [VUID-vkGetPhysicalDeviceSurfaceCapabilitiesKHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceSurfaceCapabilitiesKHR
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(physicalDevice, surface, pSurfaceCapabilities)
        },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceSurfaceFormats2KHR(
    physicalDevice: vk::VkPhysicalDevice,
    pSurfaceInfo: *const vk::VkPhysicalDeviceSurfaceInfo2KHR<'_>,
    pSurfaceFormatCount: *mut u32,
    pSurfaceFormats: *mut vk::VkSurfaceFormat2KHR<'_>,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceSurfaceFormats2KHR: Invalid physicalDevice [VUID-vkGetPhysicalDeviceSurfaceFormats2KHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceSurfaceFormats2KHR
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(
                physicalDevice,
                pSurfaceInfo,
                pSurfaceFormatCount,
                pSurfaceFormats,
            )
        },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceSurfaceFormatsKHR(
    physicalDevice: vk::VkPhysicalDevice,
    surface: vk::VkSurfaceKHR,
    pSurfaceFormatCount: *mut u32,
    pSurfaceFormats: *mut vk::VkSurfaceFormatKHR,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceSurfaceFormatsKHR: Invalid physicalDevice [VUID-vkGetPhysicalDeviceSurfaceFormatsKHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceSurfaceFormatsKHR
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(
                physicalDevice,
                surface,
                pSurfaceFormatCount,
                pSurfaceFormats,
            )
        },
    )
}
#[cfg(target_os = "windows")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceSurfacePresentModes2EXT(
    physicalDevice: vk::VkPhysicalDevice,
    pSurfaceInfo: *const vk::VkPhysicalDeviceSurfaceInfo2KHR<'_>,
    pPresentModeCount: *mut u32,
    pPresentModes: *mut vk::VkPresentModeKHR,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceSurfacePresentModes2EXT: Invalid physicalDevice [VUID-vkGetPhysicalDeviceSurfacePresentModes2EXT-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceSurfacePresentModes2EXT
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(
                physicalDevice,
                pSurfaceInfo,
                pPresentModeCount,
                pPresentModes,
            )
        },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceSurfacePresentModesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    surface: vk::VkSurfaceKHR,
    pPresentModeCount: *mut u32,
    pPresentModes: *mut vk::VkPresentModeKHR,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceSurfacePresentModesKHR: Invalid physicalDevice [VUID-vkGetPhysicalDeviceSurfacePresentModesKHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceSurfacePresentModesKHR
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(physicalDevice, surface, pPresentModeCount, pPresentModes)
        },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceSurfaceSupportKHR(
    physicalDevice: vk::VkPhysicalDevice,
    queueFamilyIndex: u32,
    surface: vk::VkSurfaceKHR,
    pSupported: *mut vk::VkBool32,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceSurfaceSupportKHR: Invalid physicalDevice [VUID-vkGetPhysicalDeviceSurfaceSupportKHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceSurfaceSupportKHR
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(physicalDevice, queueFamilyIndex, surface, pSupported)
        },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceToolProperties(
    physicalDevice: vk::VkPhysicalDevice,
    pToolCount: *mut u32,
    pToolProperties: *mut vk::VkPhysicalDeviceToolProperties<'_>,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceToolProperties: Invalid physicalDevice [VUID-vkGetPhysicalDeviceToolProperties-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceToolProperties
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, pToolCount, pToolProperties) },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceToolPropertiesEXT(
    physicalDevice: vk::VkPhysicalDevice,
    pToolCount: *mut u32,
    pToolProperties: *mut vk::VkPhysicalDeviceToolPropertiesEXT<'_>,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceToolPropertiesEXT: Invalid physicalDevice [VUID-vkGetPhysicalDeviceToolPropertiesEXT-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceToolPropertiesEXT
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, pToolCount, pToolProperties) },
    )
}
#[cfg(feature = "platform-ubm")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceUbmPresentationSupportSEC(
    physicalDevice: vk::VkPhysicalDevice,
    queueFamilyIndex: u32,
    device: *mut vk::ubm_device,
) -> vk::VkBool32 {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceUbmPresentationSupportSEC: Invalid physicalDevice [VUID-vkGetPhysicalDeviceUbmPresentationSupportSEC-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceUbmPresentationSupportSEC
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            unsafe { core::mem::zeroed::<vk::VkBool32>() }
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, queueFamilyIndex, device) },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceVideoCapabilitiesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    pVideoProfile: *const vk::VkVideoProfileInfoKHR<'_>,
    pCapabilities: *mut vk::VkVideoCapabilitiesKHR<'_>,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceVideoCapabilitiesKHR: Invalid physicalDevice [VUID-vkGetPhysicalDeviceVideoCapabilitiesKHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceVideoCapabilitiesKHR
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(physicalDevice, pVideoProfile, pCapabilities)
        },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    pQualityLevelInfo: *const vk::VkPhysicalDeviceVideoEncodeQualityLevelInfoKHR<'_>,
    pQualityLevelProperties: *mut vk::VkVideoEncodeQualityLevelPropertiesKHR<'_>,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR: Invalid physicalDevice [VUID-vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(physicalDevice, pQualityLevelInfo, pQualityLevelProperties)
        },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceVideoFormatPropertiesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    pVideoFormatInfo: *const vk::VkPhysicalDeviceVideoFormatInfoKHR<'_>,
    pVideoFormatPropertyCount: *mut u32,
    pVideoFormatProperties: *mut vk::VkVideoFormatPropertiesKHR<'_>,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceVideoFormatPropertiesKHR: Invalid physicalDevice [VUID-vkGetPhysicalDeviceVideoFormatPropertiesKHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceVideoFormatPropertiesKHR
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(
                physicalDevice,
                pVideoFormatInfo,
                pVideoFormatPropertyCount,
                pVideoFormatProperties,
            )
        },
    )
}
#[cfg(all(
    feature = "wsi-wayland",
    any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "hurd",
        target_os = "cygwin"
    )
))]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceWaylandPresentationSupportKHR(
    physicalDevice: vk::VkPhysicalDevice,
    queueFamilyIndex: u32,
    display: *mut vk::wl_display,
) -> vk::VkBool32 {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceWaylandPresentationSupportKHR: Invalid physicalDevice [VUID-vkGetPhysicalDeviceWaylandPresentationSupportKHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceWaylandPresentationSupportKHR
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            unsafe { core::mem::zeroed::<vk::VkBool32>() }
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, queueFamilyIndex, display) },
    )
}
#[cfg(target_os = "windows")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceWin32PresentationSupportKHR(
    physicalDevice: vk::VkPhysicalDevice,
    queueFamilyIndex: u32,
) -> vk::VkBool32 {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceWin32PresentationSupportKHR: Invalid physicalDevice [VUID-vkGetPhysicalDeviceWin32PresentationSupportKHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceWin32PresentationSupportKHR
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            unsafe { core::mem::zeroed::<vk::VkBool32>() }
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, queueFamilyIndex) },
    )
}
#[cfg(all(
    feature = "wsi-xcb",
    any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "hurd",
        target_os = "cygwin"
    )
))]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceXcbPresentationSupportKHR(
    physicalDevice: vk::VkPhysicalDevice,
    queueFamilyIndex: u32,
    connection: *mut vk::xcb_connection_t,
    visual_id: vk::xcb_visualid_t,
) -> vk::VkBool32 {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceXcbPresentationSupportKHR: Invalid physicalDevice [VUID-vkGetPhysicalDeviceXcbPresentationSupportKHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceXcbPresentationSupportKHR
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            unsafe { core::mem::zeroed::<vk::VkBool32>() }
        },
        |(command, physicalDevice)| unsafe {
            command(physicalDevice, queueFamilyIndex, connection, visual_id)
        },
    )
}
#[cfg(all(
    feature = "wsi-xlib",
    any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "hurd",
        target_os = "cygwin"
    )
))]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPhysicalDeviceXlibPresentationSupportKHR(
    physicalDevice: vk::VkPhysicalDevice,
    queueFamilyIndex: u32,
    dpy: *mut vk::Display,
    visualID: vk::VisualID,
) -> vk::VkBool32 {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceXlibPresentationSupportKHR: Invalid physicalDevice [VUID-vkGetPhysicalDeviceXlibPresentationSupportKHR-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetPhysicalDeviceXlibPresentationSupportKHR
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            unsafe { core::mem::zeroed::<vk::VkBool32>() }
        },
        |(command, physicalDevice)| unsafe {
            command(physicalDevice, queueFamilyIndex, dpy, visualID)
        },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPipelineBinaryDataKHR(
    device: vk::VkDevice,
    pInfo: *const vk::VkPipelineBinaryDataInfoKHR<'_>,
    pPipelineBinaryKey: *mut vk::VkPipelineBinaryKeyKHR<'_>,
    pPipelineBinaryDataSize: *mut usize,
    pPipelineBinaryData: *mut c_void,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetPipelineBinaryDataKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            pInfo,
            pPipelineBinaryKey,
            pPipelineBinaryDataSize,
            pPipelineBinaryData,
        )
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPipelineCacheData(
    device: vk::VkDevice,
    pipelineCache: vk::VkPipelineCache,
    pDataSize: *mut usize,
    pData: *mut c_void,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetPipelineCacheData;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pipelineCache, pDataSize, pData) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPipelineExecutableInternalRepresentationsKHR(
    device: vk::VkDevice,
    pExecutableInfo: *const vk::VkPipelineExecutableInfoKHR<'_>,
    pInternalRepresentationCount: *mut u32,
    pInternalRepresentations: *mut vk::VkPipelineExecutableInternalRepresentationKHR<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetPipelineExecutableInternalRepresentationsKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            pExecutableInfo,
            pInternalRepresentationCount,
            pInternalRepresentations,
        )
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPipelineExecutablePropertiesKHR(
    device: vk::VkDevice,
    pPipelineInfo: *const vk::VkPipelineInfoKHR<'_>,
    pExecutableCount: *mut u32,
    pProperties: *mut vk::VkPipelineExecutablePropertiesKHR<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetPipelineExecutablePropertiesKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pPipelineInfo, pExecutableCount, pProperties) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPipelineExecutableStatisticsKHR(
    device: vk::VkDevice,
    pExecutableInfo: *const vk::VkPipelineExecutableInfoKHR<'_>,
    pStatisticCount: *mut u32,
    pStatistics: *mut vk::VkPipelineExecutableStatisticKHR<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetPipelineExecutableStatisticsKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pExecutableInfo, pStatisticCount, pStatistics) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPipelineIndirectDeviceAddressNV(
    device: vk::VkDevice,
    pInfo: *const vk::VkPipelineIndirectDeviceAddressInfoNV<'_>,
) -> vk::VkDeviceAddress {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetPipelineIndirectDeviceAddressNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPipelineIndirectMemoryRequirementsNV(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkComputePipelineCreateInfo<'_>,
    pMemoryRequirements: *mut vk::VkMemoryRequirements2<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetPipelineIndirectMemoryRequirementsNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pCreateInfo, pMemoryRequirements);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPipelineKeyKHR(
    device: vk::VkDevice,
    pPipelineCreateInfo: *const vk::VkPipelineCreateInfoKHR<'_>,
    pPipelineKey: *mut vk::VkPipelineBinaryKeyKHR<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetPipelineKeyKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pPipelineCreateInfo, pPipelineKey) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPipelinePropertiesEXT(
    device: vk::VkDevice,
    pPipelineInfo: *const vk::VkPipelineInfoKHR<'_>,
    pPipelineProperties: *mut vk::VkBaseOutStructure<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetPipelinePropertiesEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pPipelineInfo, pPipelineProperties) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetPrivateData(
    device: vk::VkDevice,
    objectType: vk::VkObjectType,
    objectHandle: u64,
    privateDataSlot: vk::VkPrivateDataSlot,
    pData: *mut u64,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetPrivateData;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, objectType, objectHandle, privateDataSlot, pData);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetPrivateDataEXT(
    device: vk::VkDevice,
    objectType: vk::VkObjectType,
    objectHandle: u64,
    privateDataSlot: vk::VkPrivateDataSlotEXT,
    pData: *mut u64,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetPrivateDataEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, objectType, objectHandle, privateDataSlot, pData);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetQueryPoolResults(
    device: vk::VkDevice,
    queryPool: vk::VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
    dataSize: usize,
    pData: *mut c_void,
    stride: vk::VkDeviceSize,
    flags: vk::VkQueryResultFlags,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetQueryPoolResults;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device, queryPool, firstQuery, queryCount, dataSize, pData, stride, flags,
        )
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetQueueCheckpointData2NV(
    queue: vk::VkQueue,
    pCheckpointDataCount: *mut u32,
    pCheckpointData: *mut vk::VkCheckpointData2NV<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(queue.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetQueueCheckpointData2NV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(queue, pCheckpointDataCount, pCheckpointData);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetQueueCheckpointDataNV(
    queue: vk::VkQueue,
    pCheckpointDataCount: *mut u32,
    pCheckpointData: *mut vk::VkCheckpointDataNV<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(queue.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetQueueCheckpointDataNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(queue, pCheckpointDataCount, pCheckpointData);
    }
}
#[cfg(all(
    feature = "wsi-xlib-xrandr",
    any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "hurd",
        target_os = "cygwin"
    )
))]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetRandROutputDisplayEXT(
    physicalDevice: vk::VkPhysicalDevice,
    dpy: *mut vk::Display,
    rrOutput: vk::RROutput,
    pDisplay: *mut vk::VkDisplayKHR,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetRandROutputDisplayEXT: Invalid physicalDevice [VUID-vkGetRandROutputDisplayEXT-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetRandROutputDisplayEXT
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, dpy, rrOutput, pDisplay) },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetRayTracingCaptureReplayShaderGroupHandlesKHR(
    device: vk::VkDevice,
    pipeline: vk::VkPipeline,
    firstGroup: u32,
    groupCount: u32,
    dataSize: usize,
    pData: *mut c_void,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetRayTracingCaptureReplayShaderGroupHandlesKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pipeline, firstGroup, groupCount, dataSize, pData) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetRayTracingShaderGroupHandlesKHR(
    device: vk::VkDevice,
    pipeline: vk::VkPipeline,
    firstGroup: u32,
    groupCount: u32,
    dataSize: usize,
    pData: *mut c_void,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetRayTracingShaderGroupHandlesKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pipeline, firstGroup, groupCount, dataSize, pData) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetRayTracingShaderGroupHandlesNV(
    device: vk::VkDevice,
    pipeline: vk::VkPipeline,
    firstGroup: u32,
    groupCount: u32,
    dataSize: usize,
    pData: *mut c_void,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetRayTracingShaderGroupHandlesNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pipeline, firstGroup, groupCount, dataSize, pData) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetRayTracingShaderGroupStackSizeKHR(
    device: vk::VkDevice,
    pipeline: vk::VkPipeline,
    group: u32,
    groupShader: vk::VkShaderGroupShaderKHR,
) -> vk::VkDeviceSize {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetRayTracingShaderGroupStackSizeKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pipeline, group, groupShader) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetRefreshCycleDurationGOOGLE(
    device: vk::VkDevice,
    swapchain: vk::VkSwapchainKHR,
    pDisplayTimingProperties: *mut vk::VkRefreshCycleDurationGOOGLE,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetRefreshCycleDurationGOOGLE;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, swapchain, pDisplayTimingProperties) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetRenderAreaGranularity(
    device: vk::VkDevice,
    renderPass: vk::VkRenderPass,
    pGranularity: *mut vk::VkExtent2D,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetRenderAreaGranularity;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, renderPass, pGranularity);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetRenderingAreaGranularity(
    device: vk::VkDevice,
    pRenderingAreaInfo: *const vk::VkRenderingAreaInfo<'_>,
    pGranularity: *mut vk::VkExtent2D,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetRenderingAreaGranularity;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pRenderingAreaInfo, pGranularity);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetRenderingAreaGranularityKHR(
    device: vk::VkDevice,
    pRenderingAreaInfo: *const vk::VkRenderingAreaInfoKHR<'_>,
    pGranularity: *mut vk::VkExtent2D,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetRenderingAreaGranularityKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pRenderingAreaInfo, pGranularity);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetSamplerOpaqueCaptureDescriptorDataEXT(
    device: vk::VkDevice,
    pInfo: *const vk::VkSamplerCaptureDescriptorDataInfoEXT<'_>,
    pData: *mut c_void,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetSamplerOpaqueCaptureDescriptorDataEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pInfo, pData) }
}
#[cfg(any(target_os = "nto", target_os = "qnx"))]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetScreenBufferPropertiesQNX(
    device: vk::VkDevice,
    buffer: *const vk::_screen_buffer,
    pProperties: *mut vk::VkScreenBufferPropertiesQNX<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetScreenBufferPropertiesQNX;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, buffer, pProperties) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetSemaphoreCounterValue(
    device: vk::VkDevice,
    semaphore: vk::VkSemaphore,
    pValue: *mut u64,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetSemaphoreCounterValue;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, semaphore, pValue) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetSemaphoreCounterValueKHR(
    device: vk::VkDevice,
    semaphore: vk::VkSemaphore,
    pValue: *mut u64,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetSemaphoreCounterValueKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, semaphore, pValue) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetSemaphoreFdKHR(
    device: vk::VkDevice,
    pGetFdInfo: *const vk::VkSemaphoreGetFdInfoKHR<'_>,
    pFd: *mut core::ffi::c_int,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetSemaphoreFdKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pGetFdInfo, pFd) }
}
#[cfg(target_os = "windows")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetSemaphoreWin32HandleKHR(
    device: vk::VkDevice,
    pGetWin32HandleInfo: *const vk::VkSemaphoreGetWin32HandleInfoKHR<'_>,
    pHandle: *mut vk::HANDLE,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetSemaphoreWin32HandleKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pGetWin32HandleInfo, pHandle) }
}
#[cfg(target_os = "fuchsia")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetSemaphoreZirconHandleFUCHSIA(
    device: vk::VkDevice,
    pGetZirconHandleInfo: *const vk::VkSemaphoreGetZirconHandleInfoFUCHSIA<'_>,
    pZirconHandle: *mut vk::zx_handle_t,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetSemaphoreZirconHandleFUCHSIA;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pGetZirconHandleInfo, pZirconHandle) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetShaderBinaryDataEXT(
    device: vk::VkDevice,
    shader: vk::VkShaderEXT,
    pDataSize: *mut usize,
    pData: *mut c_void,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetShaderBinaryDataEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, shader, pDataSize, pData) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetShaderInfoAMD(
    device: vk::VkDevice,
    pipeline: vk::VkPipeline,
    shaderStage: vk::VkShaderStageFlagBits,
    infoType: vk::VkShaderInfoTypeAMD,
    pInfoSize: *mut usize,
    pInfo: *mut c_void,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetShaderInfoAMD;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pipeline, shaderStage, infoType, pInfoSize, pInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetShaderInstrumentationValuesARM(
    device: vk::VkDevice,
    instrumentation: vk::VkShaderInstrumentationARM,
    pMetricBlockCount: *mut u32,
    pMetricValues: *mut c_void,
    flags: vk::VkShaderInstrumentationValuesFlagsARM,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetShaderInstrumentationValuesARM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            instrumentation,
            pMetricBlockCount,
            pMetricValues,
            flags,
        )
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetShaderModuleCreateInfoIdentifierEXT(
    device: vk::VkDevice,
    pCreateInfo: *const vk::VkShaderModuleCreateInfo<'_>,
    pIdentifier: *mut vk::VkShaderModuleIdentifierEXT<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetShaderModuleCreateInfoIdentifierEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pCreateInfo, pIdentifier);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetShaderModuleIdentifierEXT(
    device: vk::VkDevice,
    shaderModule: vk::VkShaderModule,
    pIdentifier: *mut vk::VkShaderModuleIdentifierEXT<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetShaderModuleIdentifierEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, shaderModule, pIdentifier);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetSleepStatusLegacyNV(
    device: vk::VkDevice,
    pLowLatencyMode: *mut vk::VkBool32,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetSleepStatusLegacyNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pLowLatencyMode);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetSwapchainCounterEXT(
    device: vk::VkDevice,
    swapchain: vk::VkSwapchainKHR,
    counter: vk::VkSurfaceCounterFlagBitsEXT,
    pCounterValue: *mut u64,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetSwapchainCounterEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, swapchain, counter, pCounterValue) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkGetSwapchainImagesKHR(
    device: vk::VkDevice,
    swapchain: vk::VkSwapchainKHR,
    pSwapchainImageCount: *mut u32,
    pSwapchainImages: *mut vk::VkImage,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetSwapchainImagesKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, swapchain, pSwapchainImageCount, pSwapchainImages) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetSwapchainStatusKHR(
    device: vk::VkDevice,
    swapchain: vk::VkSwapchainKHR,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetSwapchainStatusKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, swapchain) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetSwapchainTimeDomainPropertiesEXT(
    device: vk::VkDevice,
    swapchain: vk::VkSwapchainKHR,
    pSwapchainTimeDomainProperties: *mut vk::VkSwapchainTimeDomainPropertiesEXT<'_>,
    pTimeDomainsCounter: *mut u64,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetSwapchainTimeDomainPropertiesEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            swapchain,
            pSwapchainTimeDomainProperties,
            pTimeDomainsCounter,
        )
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetSwapchainTimingPropertiesEXT(
    device: vk::VkDevice,
    swapchain: vk::VkSwapchainKHR,
    pSwapchainTimingProperties: *mut vk::VkSwapchainTimingPropertiesEXT<'_>,
    pSwapchainTimingPropertiesCounter: *mut u64,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetSwapchainTimingPropertiesEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            swapchain,
            pSwapchainTimingProperties,
            pSwapchainTimingPropertiesCounter,
        )
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetTensorMemoryRequirementsARM(
    device: vk::VkDevice,
    pInfo: *const vk::VkTensorMemoryRequirementsInfoARM<'_>,
    pMemoryRequirements: *mut vk::VkMemoryRequirements2<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetTensorMemoryRequirementsARM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, pInfo, pMemoryRequirements);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetTensorOpaqueCaptureDataARM(
    device: vk::VkDevice,
    tensorCount: u32,
    pTensors: *const vk::VkTensorARM,
    pDatas: *mut vk::VkHostAddressRangeEXT<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetTensorOpaqueCaptureDataARM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, tensorCount, pTensors, pDatas) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetTensorOpaqueCaptureDescriptorDataARM(
    device: vk::VkDevice,
    pInfo: *const vk::VkTensorCaptureDescriptorDataInfoARM<'_>,
    pData: *mut c_void,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetTensorOpaqueCaptureDescriptorDataARM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pInfo, pData) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetTensorViewOpaqueCaptureDescriptorDataARM(
    device: vk::VkDevice,
    pInfo: *const vk::VkTensorViewCaptureDescriptorDataInfoARM<'_>,
    pData: *mut c_void,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetTensorViewOpaqueCaptureDescriptorDataARM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pInfo, pData) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetValidationCacheDataEXT(
    device: vk::VkDevice,
    validationCache: vk::VkValidationCacheEXT,
    pDataSize: *mut usize,
    pData: *mut c_void,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetValidationCacheDataEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, validationCache, pDataSize, pData) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetVideoSessionMemoryRequirementsKHR(
    device: vk::VkDevice,
    videoSession: vk::VkVideoSessionKHR,
    pMemoryRequirementsCount: *mut u32,
    pMemoryRequirements: *mut vk::VkVideoSessionMemoryRequirementsKHR<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkGetVideoSessionMemoryRequirementsKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            videoSession,
            pMemoryRequirementsCount,
            pMemoryRequirements,
        )
    }
}
#[cfg(target_os = "windows")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkGetWinrtDisplayNV(
    physicalDevice: vk::VkPhysicalDevice,
    deviceRelativeId: u32,
    pDisplay: *mut vk::VkDisplayKHR,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkGetWinrtDisplayNV: Invalid physicalDevice [VUID-vkGetWinrtDisplayNV-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkGetWinrtDisplayNV
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, deviceRelativeId, pDisplay) },
    )
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkImportFenceFdKHR(
    device: vk::VkDevice,
    pImportFenceFdInfo: *const vk::VkImportFenceFdInfoKHR<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkImportFenceFdKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pImportFenceFdInfo) }
}
#[cfg(target_os = "windows")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkImportFenceWin32HandleKHR(
    device: vk::VkDevice,
    pImportFenceWin32HandleInfo: *const vk::VkImportFenceWin32HandleInfoKHR<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkImportFenceWin32HandleKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pImportFenceWin32HandleInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkImportSemaphoreFdKHR(
    device: vk::VkDevice,
    pImportSemaphoreFdInfo: *const vk::VkImportSemaphoreFdInfoKHR<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkImportSemaphoreFdKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pImportSemaphoreFdInfo) }
}
#[cfg(target_os = "windows")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkImportSemaphoreWin32HandleKHR(
    device: vk::VkDevice,
    pImportSemaphoreWin32HandleInfo: *const vk::VkImportSemaphoreWin32HandleInfoKHR<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkImportSemaphoreWin32HandleKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pImportSemaphoreWin32HandleInfo) }
}
#[cfg(target_os = "fuchsia")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkImportSemaphoreZirconHandleFUCHSIA(
    device: vk::VkDevice,
    pImportSemaphoreZirconHandleInfo: *const vk::VkImportSemaphoreZirconHandleInfoFUCHSIA<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkImportSemaphoreZirconHandleFUCHSIA;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pImportSemaphoreZirconHandleInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkInitializePerformanceApiINTEL(
    device: vk::VkDevice,
    pInitializeInfo: *const vk::VkInitializePerformanceApiInfoINTEL<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkInitializePerformanceApiINTEL;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pInitializeInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkInvalidateMappedMemoryRanges(
    device: vk::VkDevice,
    memoryRangeCount: u32,
    pMemoryRanges: *const vk::VkMappedMemoryRange<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkInvalidateMappedMemoryRanges;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, memoryRangeCount, pMemoryRanges) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkLatencySleepLegacyNV(
    device: vk::VkDevice,
    signalSemaphore: vk::VkSemaphore,
    value: u64,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkLatencySleepLegacyNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, signalSemaphore, value);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkLatencySleepNV(
    device: vk::VkDevice,
    swapchain: vk::VkSwapchainKHR,
    pSleepInfo: *const vk::VkLatencySleepInfoNV<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkLatencySleepNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, swapchain, pSleepInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkMapMemory(
    device: vk::VkDevice,
    memory: vk::VkDeviceMemory,
    offset: vk::VkDeviceSize,
    size: vk::VkDeviceSize,
    flags: vk::VkMemoryMapFlags,
    ppData: *mut *mut c_void,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkMapMemory;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, memory, offset, size, flags, ppData) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkMapMemory2(
    device: vk::VkDevice,
    pMemoryMapInfo: *const vk::VkMemoryMapInfo<'_>,
    ppData: *mut *mut c_void,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkMapMemory2;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pMemoryMapInfo, ppData) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkMapMemory2KHR(
    device: vk::VkDevice,
    pMemoryMapInfo: *const vk::VkMemoryMapInfoKHR<'_>,
    ppData: *mut *mut c_void,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkMapMemory2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pMemoryMapInfo, ppData) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkMergePipelineCaches(
    device: vk::VkDevice,
    dstCache: vk::VkPipelineCache,
    srcCacheCount: u32,
    pSrcCaches: *const vk::VkPipelineCache,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkMergePipelineCaches;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, dstCache, srcCacheCount, pSrcCaches) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkMergeValidationCachesEXT(
    device: vk::VkDevice,
    dstCache: vk::VkValidationCacheEXT,
    srcCacheCount: u32,
    pSrcCaches: *const vk::VkValidationCacheEXT,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkMergeValidationCachesEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, dstCache, srcCacheCount, pSrcCaches) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkQueueBeginDebugUtilsLabelEXT(
    queue: vk::VkQueue,
    pLabelInfo: *const vk::VkDebugUtilsLabelEXT<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(queue.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkQueueBeginDebugUtilsLabelEXT;
    if let Some(command) = command {
        unsafe {
            command(queue, pLabelInfo);
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkQueueBindSparse(
    queue: vk::VkQueue,
    bindInfoCount: u32,
    pBindInfo: *const vk::VkBindSparseInfo<'_>,
    fence: vk::VkFence,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(queue.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkQueueBindSparse;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(queue, bindInfoCount, pBindInfo, fence) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkQueueEndDebugUtilsLabelEXT(queue: vk::VkQueue) {
    let dispatch =
        unsafe { device_dispatch(queue.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkQueueEndDebugUtilsLabelEXT;
    if let Some(command) = command {
        unsafe {
            command(queue);
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkQueueInsertDebugUtilsLabelEXT(
    queue: vk::VkQueue,
    pLabelInfo: *const vk::VkDebugUtilsLabelEXT<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(queue.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkQueueInsertDebugUtilsLabelEXT;
    if let Some(command) = command {
        unsafe {
            command(queue, pLabelInfo);
        }
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkQueueNotifyOutOfBandLegacyNV(
    queue: vk::VkQueue,
    queueType: u32,
) {
    let dispatch =
        unsafe { device_dispatch(queue.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkQueueNotifyOutOfBandLegacyNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(queue, queueType);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkQueueNotifyOutOfBandNV(
    queue: vk::VkQueue,
    pQueueTypeInfo: *const vk::VkOutOfBandQueueTypeInfoNV<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(queue.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkQueueNotifyOutOfBandNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(queue, pQueueTypeInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkQueuePresentKHR(
    queue: vk::VkQueue,
    pPresentInfo: *const vk::VkPresentInfoKHR<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(queue.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkQueuePresentKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(queue, pPresentInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkQueueSetPerfHintQCOM(
    queue: vk::VkQueue,
    pPerfHintInfo: *const vk::VkPerfHintInfoQCOM<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(queue.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkQueueSetPerfHintQCOM;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(queue, pPerfHintInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkQueueSetPerformanceConfigurationINTEL(
    queue: vk::VkQueue,
    configuration: vk::VkPerformanceConfigurationINTEL,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(queue.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkQueueSetPerformanceConfigurationINTEL;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(queue, configuration) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkQueueSubmit(
    queue: vk::VkQueue,
    submitCount: u32,
    pSubmits: *const vk::VkSubmitInfo<'_>,
    fence: vk::VkFence,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(queue.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkQueueSubmit;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(queue, submitCount, pSubmits, fence) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkQueueSubmit2(
    queue: vk::VkQueue,
    submitCount: u32,
    pSubmits: *const vk::VkSubmitInfo2<'_>,
    fence: vk::VkFence,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(queue.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkQueueSubmit2;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(queue, submitCount, pSubmits, fence) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkQueueSubmit2KHR(
    queue: vk::VkQueue,
    submitCount: u32,
    pSubmits: *const vk::VkSubmitInfo2KHR<'_>,
    fence: vk::VkFence,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(queue.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkQueueSubmit2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(queue, submitCount, pSubmits, fence) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkQueueWaitIdle(queue: vk::VkQueue) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(queue.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkQueueWaitIdle;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(queue) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkRegisterCustomBorderColorEXT(
    device: vk::VkDevice,
    pBorderColor: *const vk::VkSamplerCustomBorderColorCreateInfoEXT<'_>,
    requestIndex: vk::VkBool32,
    pIndex: *mut u32,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkRegisterCustomBorderColorEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pBorderColor, requestIndex, pIndex) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkRegisterDeviceEventEXT(
    device: vk::VkDevice,
    pDeviceEventInfo: *const vk::VkDeviceEventInfoEXT<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pFence: *mut vk::VkFence,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkRegisterDeviceEventEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pDeviceEventInfo, pAllocator, pFence) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkRegisterDisplayEventEXT(
    device: vk::VkDevice,
    display: vk::VkDisplayKHR,
    pDisplayEventInfo: *const vk::VkDisplayEventInfoEXT<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pFence: *mut vk::VkFence,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkRegisterDisplayEventEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, display, pDisplayEventInfo, pAllocator, pFence) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkReleaseCapturedPipelineDataKHR(
    device: vk::VkDevice,
    pInfo: *const vk::VkReleaseCapturedPipelineDataInfoKHR<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkReleaseCapturedPipelineDataKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pInfo, pAllocator) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkReleaseDisplayEXT(
    physicalDevice: vk::VkPhysicalDevice,
    display: vk::VkDisplayKHR,
) -> vk::VkResult {
    let Some((dispatch, physicalDevice)) =
        (unsafe { resolve_trampoline_physical_device(physicalDevice) })
    else {
        fatal_loader_error(
            c"vkReleaseDisplayEXT: Invalid physicalDevice [VUID-vkReleaseDisplayEXT-physicalDevice-parameter]",
        )
    };
    let command = dispatch
        .vkReleaseDisplayEXT
        .map(|command| (command, physicalDevice));
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, display) },
    )
}
#[cfg(target_os = "windows")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkReleaseFullScreenExclusiveModeEXT(
    device: vk::VkDevice,
    swapchain: vk::VkSwapchainKHR,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkReleaseFullScreenExclusiveModeEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, swapchain) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkReleasePerformanceConfigurationINTEL(
    device: vk::VkDevice,
    configuration: vk::VkPerformanceConfigurationINTEL,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkReleasePerformanceConfigurationINTEL;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, configuration) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkReleaseProfilingLockKHR(device: vk::VkDevice) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkReleaseProfilingLockKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkReleaseSwapchainImagesEXT(
    device: vk::VkDevice,
    pReleaseInfo: *const vk::VkReleaseSwapchainImagesInfoEXT<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkReleaseSwapchainImagesEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pReleaseInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkReleaseSwapchainImagesKHR(
    device: vk::VkDevice,
    pReleaseInfo: *const vk::VkReleaseSwapchainImagesInfoKHR<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkReleaseSwapchainImagesKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pReleaseInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkResetCommandBuffer(
    commandBuffer: vk::VkCommandBuffer,
    flags: vk::VkCommandBufferResetFlags,
) -> vk::VkResult {
    let dispatch = unsafe { device_dispatch(commandBuffer.0.cast()) }
        .unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkResetCommandBuffer;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(commandBuffer, flags) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkResetCommandPool(
    device: vk::VkDevice,
    commandPool: vk::VkCommandPool,
    flags: vk::VkCommandPoolResetFlags,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkResetCommandPool;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, commandPool, flags) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkResetDescriptorPool(
    device: vk::VkDevice,
    descriptorPool: vk::VkDescriptorPool,
    flags: vk::VkDescriptorPoolResetFlags,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkResetDescriptorPool;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, descriptorPool, flags) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkResetEvent(
    device: vk::VkDevice,
    event: vk::VkEvent,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkResetEvent;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, event) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkResetFences(
    device: vk::VkDevice,
    fenceCount: u32,
    pFences: *const vk::VkFence,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkResetFences;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, fenceCount, pFences) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkResetGpaSessionAMD(
    device: vk::VkDevice,
    gpaSession: vk::VkGpaSessionAMD,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkResetGpaSessionAMD;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, gpaSession) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkResetQueryPool(
    device: vk::VkDevice,
    queryPool: vk::VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkResetQueryPool;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, queryPool, firstQuery, queryCount);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkResetQueryPoolEXT(
    device: vk::VkDevice,
    queryPool: vk::VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkResetQueryPoolEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, queryPool, firstQuery, queryCount);
    }
}
#[cfg(target_os = "fuchsia")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkSetBufferCollectionBufferConstraintsFUCHSIA(
    device: vk::VkDevice,
    collection: vk::VkBufferCollectionFUCHSIA,
    pBufferConstraintsInfo: *const vk::VkBufferConstraintsInfoFUCHSIA<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkSetBufferCollectionBufferConstraintsFUCHSIA;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, collection, pBufferConstraintsInfo) }
}
#[cfg(target_os = "fuchsia")]
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkSetBufferCollectionImageConstraintsFUCHSIA(
    device: vk::VkDevice,
    collection: vk::VkBufferCollectionFUCHSIA,
    pImageConstraintsInfo: *const vk::VkImageConstraintsInfoFUCHSIA<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkSetBufferCollectionImageConstraintsFUCHSIA;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, collection, pImageConstraintsInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkSetDeviceMemoryPriorityEXT(
    device: vk::VkDevice,
    memory: vk::VkDeviceMemory,
    priority: f32,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkSetDeviceMemoryPriorityEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, memory, priority);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkSetEvent(
    device: vk::VkDevice,
    event: vk::VkEvent,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkSetEvent;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, event) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkSetGpaDeviceClockModeAMD(
    device: vk::VkDevice,
    pInfo: *mut vk::VkGpaDeviceClockModeInfoAMD<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkSetGpaDeviceClockModeAMD;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkSetHdrMetadataEXT(
    device: vk::VkDevice,
    swapchainCount: u32,
    pSwapchains: *const vk::VkSwapchainKHR,
    pMetadata: *const vk::VkHdrMetadataEXT<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkSetHdrMetadataEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, swapchainCount, pSwapchains, pMetadata);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkSetLatencyMarkerLegacyNV(
    device: vk::VkDevice,
    frameID: u64,
    marker: u32,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkSetLatencyMarkerLegacyNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, frameID, marker);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkSetLatencyMarkerNV(
    device: vk::VkDevice,
    swapchain: vk::VkSwapchainKHR,
    pLatencyMarkerInfo: *const vk::VkSetLatencyMarkerInfoNV<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkSetLatencyMarkerNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, swapchain, pLatencyMarkerInfo);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkSetLatencySleepModeLegacyNV(
    device: vk::VkDevice,
    lowLatencyMode: vk::VkBool32,
    lowLatencyBoost: vk::VkBool32,
    minimumIntervalUs: u32,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkSetLatencySleepModeLegacyNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, lowLatencyMode, lowLatencyBoost, minimumIntervalUs);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkSetLatencySleepModeNV(
    device: vk::VkDevice,
    swapchain: vk::VkSwapchainKHR,
    pSleepModeInfo: *const vk::VkLatencySleepModeInfoNV<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkSetLatencySleepModeNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, swapchain, pSleepModeInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkSetLocalDimmingAMD(
    device: vk::VkDevice,
    swapChain: vk::VkSwapchainKHR,
    localDimmingEnable: vk::VkBool32,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkSetLocalDimmingAMD;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, swapChain, localDimmingEnable);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkSetPrivateData(
    device: vk::VkDevice,
    objectType: vk::VkObjectType,
    objectHandle: u64,
    privateDataSlot: vk::VkPrivateDataSlot,
    data: u64,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkSetPrivateData;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, objectType, objectHandle, privateDataSlot, data) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkSetPrivateDataEXT(
    device: vk::VkDevice,
    objectType: vk::VkObjectType,
    objectHandle: u64,
    privateDataSlot: vk::VkPrivateDataSlotEXT,
    data: u64,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkSetPrivateDataEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, objectType, objectHandle, privateDataSlot, data) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkSetSwapchainPresentTimingQueueSizeEXT(
    device: vk::VkDevice,
    swapchain: vk::VkSwapchainKHR,
    size: u32,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkSetSwapchainPresentTimingQueueSizeEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, swapchain, size) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkShutdownLatencyDeviceLegacyNV(device: vk::VkDevice) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkShutdownLatencyDeviceLegacyNV;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkSignalSemaphore(
    device: vk::VkDevice,
    pSignalInfo: *const vk::VkSemaphoreSignalInfo<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkSignalSemaphore;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pSignalInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkSignalSemaphoreKHR(
    device: vk::VkDevice,
    pSignalInfo: *const vk::VkSemaphoreSignalInfoKHR<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkSignalSemaphoreKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pSignalInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkTransitionImageLayout(
    device: vk::VkDevice,
    transitionCount: u32,
    pTransitions: *const vk::VkHostImageLayoutTransitionInfo<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkTransitionImageLayout;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, transitionCount, pTransitions) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkTransitionImageLayoutEXT(
    device: vk::VkDevice,
    transitionCount: u32,
    pTransitions: *const vk::VkHostImageLayoutTransitionInfoEXT<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkTransitionImageLayoutEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, transitionCount, pTransitions) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkTrimCommandPool(
    device: vk::VkDevice,
    commandPool: vk::VkCommandPool,
    flags: vk::VkCommandPoolTrimFlags,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkTrimCommandPool;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, commandPool, flags);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkTrimCommandPoolKHR(
    device: vk::VkDevice,
    commandPool: vk::VkCommandPool,
    flags: vk::VkCommandPoolTrimFlagsKHR,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkTrimCommandPoolKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, commandPool, flags);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkUninitializePerformanceApiINTEL(device: vk::VkDevice) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkUninitializePerformanceApiINTEL;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkUnmapMemory(
    device: vk::VkDevice,
    memory: vk::VkDeviceMemory,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkUnmapMemory;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, memory);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkUnmapMemory2(
    device: vk::VkDevice,
    pMemoryUnmapInfo: *const vk::VkMemoryUnmapInfo<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkUnmapMemory2;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pMemoryUnmapInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkUnmapMemory2KHR(
    device: vk::VkDevice,
    pMemoryUnmapInfo: *const vk::VkMemoryUnmapInfoKHR<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkUnmapMemory2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pMemoryUnmapInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkUnregisterCustomBorderColorEXT(
    device: vk::VkDevice,
    index: u32,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkUnregisterCustomBorderColorEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, index);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkUpdateDescriptorSetWithTemplate(
    device: vk::VkDevice,
    descriptorSet: vk::VkDescriptorSet,
    descriptorUpdateTemplate: vk::VkDescriptorUpdateTemplate,
    pData: *const c_void,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkUpdateDescriptorSetWithTemplate;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, descriptorSet, descriptorUpdateTemplate, pData);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkUpdateDescriptorSetWithTemplateKHR(
    device: vk::VkDevice,
    descriptorSet: vk::VkDescriptorSet,
    descriptorUpdateTemplate: vk::VkDescriptorUpdateTemplateKHR,
    pData: *const c_void,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkUpdateDescriptorSetWithTemplateKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(device, descriptorSet, descriptorUpdateTemplate, pData);
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkUpdateDescriptorSets(
    device: vk::VkDevice,
    descriptorWriteCount: u32,
    pDescriptorWrites: *const vk::VkWriteDescriptorSet<'_>,
    descriptorCopyCount: u32,
    pDescriptorCopies: *const vk::VkCopyDescriptorSet<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkUpdateDescriptorSets;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            descriptorWriteCount,
            pDescriptorWrites,
            descriptorCopyCount,
            pDescriptorCopies,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkUpdateIndirectExecutionSetPipelineEXT(
    device: vk::VkDevice,
    indirectExecutionSet: vk::VkIndirectExecutionSetEXT,
    executionSetWriteCount: u32,
    pExecutionSetWrites: *const vk::VkWriteIndirectExecutionSetPipelineEXT<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkUpdateIndirectExecutionSetPipelineEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            indirectExecutionSet,
            executionSetWriteCount,
            pExecutionSetWrites,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkUpdateIndirectExecutionSetShaderEXT(
    device: vk::VkDevice,
    indirectExecutionSet: vk::VkIndirectExecutionSetEXT,
    executionSetWriteCount: u32,
    pExecutionSetWrites: *const vk::VkWriteIndirectExecutionSetShaderEXT<'_>,
) {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkUpdateIndirectExecutionSetShaderEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            indirectExecutionSet,
            executionSetWriteCount,
            pExecutionSetWrites,
        );
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkUpdateVideoSessionParametersKHR(
    device: vk::VkDevice,
    videoSessionParameters: vk::VkVideoSessionParametersKHR,
    pUpdateInfo: *const vk::VkVideoSessionParametersUpdateInfoKHR<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkUpdateVideoSessionParametersKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, videoSessionParameters, pUpdateInfo) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkWaitForFences(
    device: vk::VkDevice,
    fenceCount: u32,
    pFences: *const vk::VkFence,
    waitAll: vk::VkBool32,
    timeout: u64,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkWaitForFences;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, fenceCount, pFences, waitAll, timeout) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkWaitForPresent2KHR(
    device: vk::VkDevice,
    swapchain: vk::VkSwapchainKHR,
    pPresentWait2Info: *const vk::VkPresentWait2InfoKHR<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkWaitForPresent2KHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, swapchain, pPresentWait2Info) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkWaitForPresentKHR(
    device: vk::VkDevice,
    swapchain: vk::VkSwapchainKHR,
    presentId: u64,
    timeout: u64,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkWaitForPresentKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, swapchain, presentId, timeout) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "system" fn vkWaitSemaphores(
    device: vk::VkDevice,
    pWaitInfo: *const vk::VkSemaphoreWaitInfo<'_>,
    timeout: u64,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkWaitSemaphores;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pWaitInfo, timeout) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkWaitSemaphoresKHR(
    device: vk::VkDevice,
    pWaitInfo: *const vk::VkSemaphoreWaitInfoKHR<'_>,
    timeout: u64,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkWaitSemaphoresKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, pWaitInfo, timeout) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkWriteAccelerationStructuresPropertiesKHR(
    device: vk::VkDevice,
    accelerationStructureCount: u32,
    pAccelerationStructures: *const vk::VkAccelerationStructureKHR,
    queryType: vk::VkQueryType,
    dataSize: usize,
    pData: *mut c_void,
    stride: usize,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkWriteAccelerationStructuresPropertiesKHR;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            accelerationStructureCount,
            pAccelerationStructures,
            queryType,
            dataSize,
            pData,
            stride,
        )
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkWriteMicromapsPropertiesEXT(
    device: vk::VkDevice,
    micromapCount: u32,
    pMicromaps: *const vk::VkMicromapEXT,
    queryType: vk::VkQueryType,
    dataSize: usize,
    pData: *mut c_void,
    stride: usize,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkWriteMicromapsPropertiesEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe {
        command(
            device,
            micromapCount,
            pMicromaps,
            queryType,
            dataSize,
            pData,
            stride,
        )
    }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkWriteResourceDescriptorsEXT(
    device: vk::VkDevice,
    resourceCount: u32,
    pResources: *const vk::VkResourceDescriptorInfoEXT<'_>,
    pDescriptors: *const vk::VkHostAddressRangeEXT<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkWriteResourceDescriptorsEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, resourceCount, pResources, pDescriptors) }
}
/// Forwards a Vulkan command to the dispatch chain.
///
/// # Safety
///
/// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
pub(crate) unsafe extern "system" fn vkWriteSamplerDescriptorsEXT(
    device: vk::VkDevice,
    samplerCount: u32,
    pSamplers: *const vk::VkSamplerCreateInfo<'_>,
    pDescriptors: *const vk::VkHostAddressRangeEXT<'_>,
) -> vk::VkResult {
    let dispatch =
        unsafe { device_dispatch(device.0.cast()) }.unwrap_or_else(|| invalid_device_dispatch());
    let command = dispatch.vkWriteSamplerDescriptorsEXT;
    debug_assert!(command.is_some());
    let command = unsafe { command.unwrap_unchecked() };
    unsafe { command(device, samplerCount, pSamplers, pDescriptors) }
}
