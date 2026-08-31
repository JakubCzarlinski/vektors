// Generated from registry/vk.xml by vk-loader-codegen. Do not edit.

#[allow(dead_code)]
#[inline]
pub(crate) const fn convert_debug_report_object_to_core_object(
    object_type: vk::VkDebugReportObjectTypeEXT,
) -> vk::VkObjectType {
    match object_type {
        vk::VkDebugReportObjectTypeEXT::ACCELERATION_STRUCTURE_KHR => {
            vk::VkObjectType::ACCELERATION_STRUCTURE_KHR
        }
        vk::VkDebugReportObjectTypeEXT::ACCELERATION_STRUCTURE_NV => {
            vk::VkObjectType::ACCELERATION_STRUCTURE_NV
        }
        vk::VkDebugReportObjectTypeEXT::BUFFER => vk::VkObjectType::BUFFER,
        vk::VkDebugReportObjectTypeEXT::BUFFER_COLLECTION_FUCHSIA => {
            vk::VkObjectType::BUFFER_COLLECTION_FUCHSIA
        }
        vk::VkDebugReportObjectTypeEXT::BUFFER_VIEW => vk::VkObjectType::BUFFER_VIEW,
        vk::VkDebugReportObjectTypeEXT::COMMAND_BUFFER => vk::VkObjectType::COMMAND_BUFFER,
        vk::VkDebugReportObjectTypeEXT::COMMAND_POOL => vk::VkObjectType::COMMAND_POOL,
        vk::VkDebugReportObjectTypeEXT::CUDA_FUNCTION_NV => vk::VkObjectType::CUDA_FUNCTION_NV,
        vk::VkDebugReportObjectTypeEXT::CUDA_MODULE_NV => vk::VkObjectType::CUDA_MODULE_NV,
        vk::VkDebugReportObjectTypeEXT::CU_FUNCTION_NVX => vk::VkObjectType::CU_FUNCTION_NVX,
        vk::VkDebugReportObjectTypeEXT::CU_MODULE_NVX => vk::VkObjectType::CU_MODULE_NVX,
        vk::VkDebugReportObjectTypeEXT::DEBUG_REPORT_CALLBACK_EXT => {
            vk::VkObjectType::DEBUG_REPORT_CALLBACK_EXT
        }
        vk::VkDebugReportObjectTypeEXT::DESCRIPTOR_POOL => vk::VkObjectType::DESCRIPTOR_POOL,
        vk::VkDebugReportObjectTypeEXT::DESCRIPTOR_SET => vk::VkObjectType::DESCRIPTOR_SET,
        vk::VkDebugReportObjectTypeEXT::DESCRIPTOR_SET_LAYOUT => {
            vk::VkObjectType::DESCRIPTOR_SET_LAYOUT
        }
        vk::VkDebugReportObjectTypeEXT::DESCRIPTOR_UPDATE_TEMPLATE => {
            vk::VkObjectType::DESCRIPTOR_UPDATE_TEMPLATE
        }
        vk::VkDebugReportObjectTypeEXT::DEVICE => vk::VkObjectType::DEVICE,
        vk::VkDebugReportObjectTypeEXT::DEVICE_MEMORY => vk::VkObjectType::DEVICE_MEMORY,
        vk::VkDebugReportObjectTypeEXT::DISPLAY_KHR => vk::VkObjectType::DISPLAY_KHR,
        vk::VkDebugReportObjectTypeEXT::DISPLAY_MODE_KHR => vk::VkObjectType::DISPLAY_MODE_KHR,
        vk::VkDebugReportObjectTypeEXT::EVENT => vk::VkObjectType::EVENT,
        vk::VkDebugReportObjectTypeEXT::FENCE => vk::VkObjectType::FENCE,
        vk::VkDebugReportObjectTypeEXT::FRAMEBUFFER => vk::VkObjectType::FRAMEBUFFER,
        vk::VkDebugReportObjectTypeEXT::IMAGE => vk::VkObjectType::IMAGE,
        vk::VkDebugReportObjectTypeEXT::IMAGE_VIEW => vk::VkObjectType::IMAGE_VIEW,
        vk::VkDebugReportObjectTypeEXT::INSTANCE => vk::VkObjectType::INSTANCE,
        vk::VkDebugReportObjectTypeEXT::PHYSICAL_DEVICE => vk::VkObjectType::PHYSICAL_DEVICE,
        vk::VkDebugReportObjectTypeEXT::PIPELINE => vk::VkObjectType::PIPELINE,
        vk::VkDebugReportObjectTypeEXT::PIPELINE_CACHE => vk::VkObjectType::PIPELINE_CACHE,
        vk::VkDebugReportObjectTypeEXT::PIPELINE_LAYOUT => vk::VkObjectType::PIPELINE_LAYOUT,
        vk::VkDebugReportObjectTypeEXT::QUERY_POOL => vk::VkObjectType::QUERY_POOL,
        vk::VkDebugReportObjectTypeEXT::QUEUE => vk::VkObjectType::QUEUE,
        vk::VkDebugReportObjectTypeEXT::RENDER_PASS => vk::VkObjectType::RENDER_PASS,
        vk::VkDebugReportObjectTypeEXT::SAMPLER => vk::VkObjectType::SAMPLER,
        vk::VkDebugReportObjectTypeEXT::SAMPLER_YCBCR_CONVERSION => {
            vk::VkObjectType::SAMPLER_YCBCR_CONVERSION
        }
        vk::VkDebugReportObjectTypeEXT::SEMAPHORE => vk::VkObjectType::SEMAPHORE,
        vk::VkDebugReportObjectTypeEXT::SHADER_MODULE => vk::VkObjectType::SHADER_MODULE,
        vk::VkDebugReportObjectTypeEXT::SURFACE_KHR => vk::VkObjectType::SURFACE_KHR,
        vk::VkDebugReportObjectTypeEXT::SWAPCHAIN_KHR => vk::VkObjectType::SWAPCHAIN_KHR,
        vk::VkDebugReportObjectTypeEXT::VALIDATION_CACHE_EXT => {
            vk::VkObjectType::VALIDATION_CACHE_EXT
        }
        _ => vk::VkObjectType::UNKNOWN,
    }
}
#[allow(dead_code)]
#[inline]
pub(crate) const fn convert_core_object_to_debug_report_object(
    object_type: vk::VkObjectType,
) -> vk::VkDebugReportObjectTypeEXT {
    match object_type {
        vk::VkObjectType::ACCELERATION_STRUCTURE_KHR => {
            vk::VkDebugReportObjectTypeEXT::ACCELERATION_STRUCTURE_KHR
        }
        vk::VkObjectType::ACCELERATION_STRUCTURE_NV => {
            vk::VkDebugReportObjectTypeEXT::ACCELERATION_STRUCTURE_NV
        }
        vk::VkObjectType::BUFFER => vk::VkDebugReportObjectTypeEXT::BUFFER,
        vk::VkObjectType::BUFFER_COLLECTION_FUCHSIA => {
            vk::VkDebugReportObjectTypeEXT::BUFFER_COLLECTION_FUCHSIA
        }
        vk::VkObjectType::BUFFER_VIEW => vk::VkDebugReportObjectTypeEXT::BUFFER_VIEW,
        vk::VkObjectType::COMMAND_BUFFER => vk::VkDebugReportObjectTypeEXT::COMMAND_BUFFER,
        vk::VkObjectType::COMMAND_POOL => vk::VkDebugReportObjectTypeEXT::COMMAND_POOL,
        vk::VkObjectType::CUDA_FUNCTION_NV => vk::VkDebugReportObjectTypeEXT::CUDA_FUNCTION_NV,
        vk::VkObjectType::CUDA_MODULE_NV => vk::VkDebugReportObjectTypeEXT::CUDA_MODULE_NV,
        vk::VkObjectType::CU_FUNCTION_NVX => vk::VkDebugReportObjectTypeEXT::CU_FUNCTION_NVX,
        vk::VkObjectType::CU_MODULE_NVX => vk::VkDebugReportObjectTypeEXT::CU_MODULE_NVX,
        vk::VkObjectType::DEBUG_REPORT_CALLBACK_EXT => {
            vk::VkDebugReportObjectTypeEXT::DEBUG_REPORT_CALLBACK_EXT
        }
        vk::VkObjectType::DESCRIPTOR_POOL => vk::VkDebugReportObjectTypeEXT::DESCRIPTOR_POOL,
        vk::VkObjectType::DESCRIPTOR_SET => vk::VkDebugReportObjectTypeEXT::DESCRIPTOR_SET,
        vk::VkObjectType::DESCRIPTOR_SET_LAYOUT => {
            vk::VkDebugReportObjectTypeEXT::DESCRIPTOR_SET_LAYOUT
        }
        vk::VkObjectType::DESCRIPTOR_UPDATE_TEMPLATE => {
            vk::VkDebugReportObjectTypeEXT::DESCRIPTOR_UPDATE_TEMPLATE
        }
        vk::VkObjectType::DEVICE => vk::VkDebugReportObjectTypeEXT::DEVICE,
        vk::VkObjectType::DEVICE_MEMORY => vk::VkDebugReportObjectTypeEXT::DEVICE_MEMORY,
        vk::VkObjectType::DISPLAY_KHR => vk::VkDebugReportObjectTypeEXT::DISPLAY_KHR,
        vk::VkObjectType::DISPLAY_MODE_KHR => vk::VkDebugReportObjectTypeEXT::DISPLAY_MODE_KHR,
        vk::VkObjectType::EVENT => vk::VkDebugReportObjectTypeEXT::EVENT,
        vk::VkObjectType::FENCE => vk::VkDebugReportObjectTypeEXT::FENCE,
        vk::VkObjectType::FRAMEBUFFER => vk::VkDebugReportObjectTypeEXT::FRAMEBUFFER,
        vk::VkObjectType::IMAGE => vk::VkDebugReportObjectTypeEXT::IMAGE,
        vk::VkObjectType::IMAGE_VIEW => vk::VkDebugReportObjectTypeEXT::IMAGE_VIEW,
        vk::VkObjectType::INSTANCE => vk::VkDebugReportObjectTypeEXT::INSTANCE,
        vk::VkObjectType::PHYSICAL_DEVICE => vk::VkDebugReportObjectTypeEXT::PHYSICAL_DEVICE,
        vk::VkObjectType::PIPELINE => vk::VkDebugReportObjectTypeEXT::PIPELINE,
        vk::VkObjectType::PIPELINE_CACHE => vk::VkDebugReportObjectTypeEXT::PIPELINE_CACHE,
        vk::VkObjectType::PIPELINE_LAYOUT => vk::VkDebugReportObjectTypeEXT::PIPELINE_LAYOUT,
        vk::VkObjectType::QUERY_POOL => vk::VkDebugReportObjectTypeEXT::QUERY_POOL,
        vk::VkObjectType::QUEUE => vk::VkDebugReportObjectTypeEXT::QUEUE,
        vk::VkObjectType::RENDER_PASS => vk::VkDebugReportObjectTypeEXT::RENDER_PASS,
        vk::VkObjectType::SAMPLER => vk::VkDebugReportObjectTypeEXT::SAMPLER,
        vk::VkObjectType::SAMPLER_YCBCR_CONVERSION => {
            vk::VkDebugReportObjectTypeEXT::SAMPLER_YCBCR_CONVERSION
        }
        vk::VkObjectType::SEMAPHORE => vk::VkDebugReportObjectTypeEXT::SEMAPHORE,
        vk::VkObjectType::SHADER_MODULE => vk::VkDebugReportObjectTypeEXT::SHADER_MODULE,
        vk::VkObjectType::SURFACE_KHR => vk::VkDebugReportObjectTypeEXT::SURFACE_KHR,
        vk::VkObjectType::SWAPCHAIN_KHR => vk::VkDebugReportObjectTypeEXT::SWAPCHAIN_KHR,
        vk::VkObjectType::VALIDATION_CACHE_EXT => {
            vk::VkDebugReportObjectTypeEXT::VALIDATION_CACHE_EXT
        }
        _ => vk::VkDebugReportObjectTypeEXT::UNKNOWN,
    }
}
