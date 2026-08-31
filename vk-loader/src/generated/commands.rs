// Generated from registry/vk.xml by vk-loader-codegen. Do not edit.

use super::dispatch_tables::LayerDeviceDispatchTable;
use super::extensions::COMMAND_DEVICE_EXTENSION_IDS;
use super::extensions::COMMAND_DEVICE_EXTENSION_RANGES;
use super::extensions::COMMAND_INSTANCE_EXTENSION_IDS;
use super::extensions::COMMAND_INSTANCE_EXTENSION_RANGES;
use super::extensions::ExtensionSet;
use crate::CStr;
use crate::CommandLookup;
use crate::CommandProviderRange;
use crate::CommandRecord;
use crate::CommandScope;
use crate::HandleInfo;
use crate::command_hash;
use crate::command_name_eq;
use crate::command_slot_hash;
use crate::dispatch_offset;
const _: () = assert!(core::mem::size_of::<LayerDeviceDispatchTable>() <= 65_535);
pub(super) const VK_GET_DEVICE_PROC_ADDR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetDeviceProcAddr),
);
pub(super) const VK_DESTROY_DEVICE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyDevice),
);
pub(super) const VK_GET_DEVICE_QUEUE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetDeviceQueue),
);
pub(super) const VK_QUEUE_SUBMIT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkQueueSubmit),
);
pub(super) const VK_QUEUE_WAIT_IDLE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkQueueWaitIdle),
);
pub(super) const VK_DEVICE_WAIT_IDLE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDeviceWaitIdle),
);
pub(super) const VK_ALLOCATE_MEMORY_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkAllocateMemory),
);
pub(super) const VK_FREE_MEMORY_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkFreeMemory),
);
pub(super) const VK_MAP_MEMORY_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(LayerDeviceDispatchTable, vkMapMemory));
pub(super) const VK_UNMAP_MEMORY_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkUnmapMemory),
);
pub(super) const VK_FLUSH_MAPPED_MEMORY_RANGES_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkFlushMappedMemoryRanges),
);
pub(super) const VK_INVALIDATE_MAPPED_MEMORY_RANGES_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkInvalidateMappedMemoryRanges),
);
pub(super) const VK_GET_DEVICE_MEMORY_COMMITMENT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetDeviceMemoryCommitment),
);
pub(super) const VK_BIND_BUFFER_MEMORY_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkBindBufferMemory),
);
pub(super) const VK_BIND_IMAGE_MEMORY_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkBindImageMemory),
);
pub(super) const VK_GET_BUFFER_MEMORY_REQUIREMENTS_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetBufferMemoryRequirements),
);
pub(super) const VK_GET_IMAGE_MEMORY_REQUIREMENTS_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetImageMemoryRequirements),
);
pub(super) const VK_GET_IMAGE_SPARSE_MEMORY_REQUIREMENTS_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetImageSparseMemoryRequirements
    ));
pub(super) const VK_QUEUE_BIND_SPARSE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkQueueBindSparse),
);
pub(super) const VK_CREATE_FENCE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateFence),
);
pub(super) const VK_DESTROY_FENCE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyFence),
);
pub(super) const VK_RESET_FENCES_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkResetFences),
);
pub(super) const VK_GET_FENCE_STATUS_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetFenceStatus),
);
pub(super) const VK_WAIT_FOR_FENCES_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkWaitForFences),
);
pub(super) const VK_CREATE_SEMAPHORE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateSemaphore),
);
pub(super) const VK_DESTROY_SEMAPHORE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroySemaphore),
);
pub(super) const VK_CREATE_QUERY_POOL_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateQueryPool),
);
pub(super) const VK_DESTROY_QUERY_POOL_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyQueryPool),
);
pub(super) const VK_GET_QUERY_POOL_RESULTS_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetQueryPoolResults),
);
pub(super) const VK_CREATE_BUFFER_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateBuffer),
);
pub(super) const VK_DESTROY_BUFFER_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyBuffer),
);
pub(super) const VK_CREATE_IMAGE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateImage),
);
pub(super) const VK_DESTROY_IMAGE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyImage),
);
pub(super) const VK_GET_IMAGE_SUBRESOURCE_LAYOUT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetImageSubresourceLayout),
);
pub(super) const VK_CREATE_IMAGE_VIEW_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateImageView),
);
pub(super) const VK_DESTROY_IMAGE_VIEW_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyImageView),
);
pub(super) const VK_CREATE_COMMAND_POOL_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateCommandPool),
);
pub(super) const VK_DESTROY_COMMAND_POOL_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyCommandPool),
);
pub(super) const VK_RESET_COMMAND_POOL_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkResetCommandPool),
);
pub(super) const VK_ALLOCATE_COMMAND_BUFFERS_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkAllocateCommandBuffers),
);
pub(super) const VK_FREE_COMMAND_BUFFERS_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkFreeCommandBuffers),
);
pub(super) const VK_BEGIN_COMMAND_BUFFER_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkBeginCommandBuffer),
);
pub(super) const VK_END_COMMAND_BUFFER_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkEndCommandBuffer),
);
pub(super) const VK_RESET_COMMAND_BUFFER_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkResetCommandBuffer),
);
pub(super) const VK_CMD_COPY_BUFFER_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdCopyBuffer),
);
pub(super) const VK_CMD_COPY_IMAGE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdCopyImage),
);
pub(super) const VK_CMD_COPY_BUFFER_TO_IMAGE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdCopyBufferToImage),
);
pub(super) const VK_CMD_COPY_IMAGE_TO_BUFFER_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdCopyImageToBuffer),
);
pub(super) const VK_CMD_UPDATE_BUFFER_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdUpdateBuffer),
);
pub(super) const VK_CMD_FILL_BUFFER_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdFillBuffer),
);
pub(super) const VK_CMD_PIPELINE_BARRIER_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdPipelineBarrier),
);
pub(super) const VK_CMD_BEGIN_QUERY_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBeginQuery),
);
pub(super) const VK_CMD_END_QUERY_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdEndQuery),
);
pub(super) const VK_CMD_RESET_QUERY_POOL_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdResetQueryPool),
);
pub(super) const VK_CMD_WRITE_TIMESTAMP_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdWriteTimestamp),
);
pub(super) const VK_CMD_COPY_QUERY_POOL_RESULTS_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdCopyQueryPoolResults),
);
pub(super) const VK_CMD_EXECUTE_COMMANDS_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdExecuteCommands),
);
pub(super) const VK_CREATE_EVENT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateEvent),
);
pub(super) const VK_DESTROY_EVENT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyEvent),
);
pub(super) const VK_GET_EVENT_STATUS_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetEventStatus),
);
pub(super) const VK_SET_EVENT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(LayerDeviceDispatchTable, vkSetEvent));
pub(super) const VK_RESET_EVENT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkResetEvent),
);
pub(super) const VK_CREATE_BUFFER_VIEW_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateBufferView),
);
pub(super) const VK_DESTROY_BUFFER_VIEW_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyBufferView),
);
pub(super) const VK_CREATE_SHADER_MODULE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateShaderModule),
);
pub(super) const VK_DESTROY_SHADER_MODULE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyShaderModule),
);
pub(super) const VK_CREATE_PIPELINE_CACHE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreatePipelineCache),
);
pub(super) const VK_DESTROY_PIPELINE_CACHE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyPipelineCache),
);
pub(super) const VK_GET_PIPELINE_CACHE_DATA_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetPipelineCacheData),
);
pub(super) const VK_MERGE_PIPELINE_CACHES_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkMergePipelineCaches),
);
pub(super) const VK_CREATE_COMPUTE_PIPELINES_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateComputePipelines),
);
pub(super) const VK_DESTROY_PIPELINE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyPipeline),
);
pub(super) const VK_CREATE_PIPELINE_LAYOUT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreatePipelineLayout),
);
pub(super) const VK_DESTROY_PIPELINE_LAYOUT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyPipelineLayout),
);
pub(super) const VK_CREATE_SAMPLER_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateSampler),
);
pub(super) const VK_DESTROY_SAMPLER_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroySampler),
);
pub(super) const VK_CREATE_DESCRIPTOR_SET_LAYOUT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateDescriptorSetLayout),
);
pub(super) const VK_DESTROY_DESCRIPTOR_SET_LAYOUT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyDescriptorSetLayout),
);
pub(super) const VK_CREATE_DESCRIPTOR_POOL_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateDescriptorPool),
);
pub(super) const VK_DESTROY_DESCRIPTOR_POOL_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyDescriptorPool),
);
pub(super) const VK_RESET_DESCRIPTOR_POOL_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkResetDescriptorPool),
);
pub(super) const VK_ALLOCATE_DESCRIPTOR_SETS_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkAllocateDescriptorSets),
);
pub(super) const VK_FREE_DESCRIPTOR_SETS_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkFreeDescriptorSets),
);
pub(super) const VK_UPDATE_DESCRIPTOR_SETS_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkUpdateDescriptorSets),
);
pub(super) const VK_CMD_BIND_PIPELINE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBindPipeline),
);
pub(super) const VK_CMD_BIND_DESCRIPTOR_SETS_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBindDescriptorSets),
);
pub(super) const VK_CMD_CLEAR_COLOR_IMAGE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdClearColorImage),
);
pub(super) const VK_CMD_DISPATCH_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDispatch),
);
pub(super) const VK_CMD_DISPATCH_INDIRECT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDispatchIndirect),
);
pub(super) const VK_CMD_SET_EVENT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetEvent),
);
pub(super) const VK_CMD_RESET_EVENT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdResetEvent),
);
pub(super) const VK_CMD_WAIT_EVENTS_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdWaitEvents),
);
pub(super) const VK_CMD_PUSH_CONSTANTS_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdPushConstants),
);
pub(super) const VK_CREATE_GRAPHICS_PIPELINES_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateGraphicsPipelines),
);
pub(super) const VK_CREATE_FRAMEBUFFER_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateFramebuffer),
);
pub(super) const VK_DESTROY_FRAMEBUFFER_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyFramebuffer),
);
pub(super) const VK_CREATE_RENDER_PASS_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateRenderPass),
);
pub(super) const VK_DESTROY_RENDER_PASS_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyRenderPass),
);
pub(super) const VK_GET_RENDER_AREA_GRANULARITY_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetRenderAreaGranularity),
);
pub(super) const VK_CMD_SET_VIEWPORT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetViewport),
);
pub(super) const VK_CMD_SET_SCISSOR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetScissor),
);
pub(super) const VK_CMD_SET_LINE_WIDTH_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetLineWidth),
);
pub(super) const VK_CMD_SET_DEPTH_BIAS_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetDepthBias),
);
pub(super) const VK_CMD_SET_BLEND_CONSTANTS_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetBlendConstants),
);
pub(super) const VK_CMD_SET_DEPTH_BOUNDS_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetDepthBounds),
);
pub(super) const VK_CMD_SET_STENCIL_COMPARE_MASK_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetStencilCompareMask),
);
pub(super) const VK_CMD_SET_STENCIL_WRITE_MASK_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetStencilWriteMask),
);
pub(super) const VK_CMD_SET_STENCIL_REFERENCE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetStencilReference),
);
pub(super) const VK_CMD_BIND_INDEX_BUFFER_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBindIndexBuffer),
);
pub(super) const VK_CMD_BIND_VERTEX_BUFFERS_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBindVertexBuffers),
);
pub(super) const VK_CMD_DRAW_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDraw));
pub(super) const VK_CMD_DRAW_INDEXED_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDrawIndexed),
);
pub(super) const VK_CMD_DRAW_INDIRECT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDrawIndirect),
);
pub(super) const VK_CMD_DRAW_INDEXED_INDIRECT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDrawIndexedIndirect),
);
pub(super) const VK_CMD_BLIT_IMAGE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBlitImage),
);
pub(super) const VK_CMD_CLEAR_DEPTH_STENCIL_IMAGE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdClearDepthStencilImage),
);
pub(super) const VK_CMD_CLEAR_ATTACHMENTS_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdClearAttachments),
);
pub(super) const VK_CMD_RESOLVE_IMAGE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdResolveImage),
);
pub(super) const VK_CMD_BEGIN_RENDER_PASS_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBeginRenderPass),
);
pub(super) const VK_CMD_NEXT_SUBPASS_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdNextSubpass),
);
pub(super) const VK_CMD_END_RENDER_PASS_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdEndRenderPass),
);
pub(super) const VK_BIND_BUFFER_MEMORY2_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkBindBufferMemory2),
);
pub(super) const VK_BIND_IMAGE_MEMORY2_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkBindImageMemory2),
);
pub(super) const VK_GET_DEVICE_GROUP_PEER_MEMORY_FEATURES_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetDeviceGroupPeerMemoryFeatures
    ));
pub(super) const VK_CMD_SET_DEVICE_MASK_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetDeviceMask),
);
pub(super) const VK_GET_IMAGE_MEMORY_REQUIREMENTS2_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetImageMemoryRequirements2),
);
pub(super) const VK_GET_BUFFER_MEMORY_REQUIREMENTS2_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetBufferMemoryRequirements2),
);
pub(super) const VK_GET_IMAGE_SPARSE_MEMORY_REQUIREMENTS2_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetImageSparseMemoryRequirements2
    ));
pub(super) const VK_TRIM_COMMAND_POOL_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkTrimCommandPool),
);
pub(super) const VK_GET_DEVICE_QUEUE2_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetDeviceQueue2),
);
pub(super) const VK_CMD_DISPATCH_BASE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDispatchBase),
);
pub(super) const VK_CREATE_DESCRIPTOR_UPDATE_TEMPLATE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateDescriptorUpdateTemplate),
);
pub(super) const VK_DESTROY_DESCRIPTOR_UPDATE_TEMPLATE_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkDestroyDescriptorUpdateTemplate
    ));
pub(super) const VK_UPDATE_DESCRIPTOR_SET_WITH_TEMPLATE_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkUpdateDescriptorSetWithTemplate
    ));
pub(super) const VK_GET_DESCRIPTOR_SET_LAYOUT_SUPPORT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetDescriptorSetLayoutSupport),
);
pub(super) const VK_CREATE_SAMPLER_YCBCR_CONVERSION_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateSamplerYcbcrConversion),
);
pub(super) const VK_DESTROY_SAMPLER_YCBCR_CONVERSION_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroySamplerYcbcrConversion),
);
pub(super) const VK_RESET_QUERY_POOL_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkResetQueryPool),
);
pub(super) const VK_GET_SEMAPHORE_COUNTER_VALUE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetSemaphoreCounterValue),
);
pub(super) const VK_WAIT_SEMAPHORES_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkWaitSemaphores),
);
pub(super) const VK_SIGNAL_SEMAPHORE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkSignalSemaphore),
);
pub(super) const VK_GET_BUFFER_DEVICE_ADDRESS_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetBufferDeviceAddress),
);
pub(super) const VK_GET_BUFFER_OPAQUE_CAPTURE_ADDRESS_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetBufferOpaqueCaptureAddress),
);
pub(super) const VK_GET_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetDeviceMemoryOpaqueCaptureAddress
    ));
pub(super) const VK_CMD_DRAW_INDIRECT_COUNT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDrawIndirectCount),
);
pub(super) const VK_CMD_DRAW_INDEXED_INDIRECT_COUNT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDrawIndexedIndirectCount),
);
pub(super) const VK_CREATE_RENDER_PASS2_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateRenderPass2),
);
pub(super) const VK_CMD_BEGIN_RENDER_PASS2_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBeginRenderPass2),
);
pub(super) const VK_CMD_NEXT_SUBPASS2_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdNextSubpass2),
);
pub(super) const VK_CMD_END_RENDER_PASS2_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdEndRenderPass2),
);
pub(super) const VK_CREATE_PRIVATE_DATA_SLOT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreatePrivateDataSlot),
);
pub(super) const VK_DESTROY_PRIVATE_DATA_SLOT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyPrivateDataSlot),
);
pub(super) const VK_SET_PRIVATE_DATA_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkSetPrivateData),
);
pub(super) const VK_GET_PRIVATE_DATA_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetPrivateData),
);
pub(super) const VK_CMD_PIPELINE_BARRIER2_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdPipelineBarrier2),
);
pub(super) const VK_CMD_WRITE_TIMESTAMP2_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdWriteTimestamp2),
);
pub(super) const VK_QUEUE_SUBMIT2_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkQueueSubmit2),
);
pub(super) const VK_CMD_COPY_BUFFER2_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdCopyBuffer2),
);
pub(super) const VK_CMD_COPY_IMAGE2_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdCopyImage2),
);
pub(super) const VK_CMD_COPY_BUFFER_TO_IMAGE2_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdCopyBufferToImage2),
);
pub(super) const VK_CMD_COPY_IMAGE_TO_BUFFER2_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdCopyImageToBuffer2),
);
pub(super) const VK_GET_DEVICE_BUFFER_MEMORY_REQUIREMENTS_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetDeviceBufferMemoryRequirements
    ));
pub(super) const VK_GET_DEVICE_IMAGE_MEMORY_REQUIREMENTS_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetDeviceImageMemoryRequirements
    ));
pub(super) const VK_GET_DEVICE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetDeviceImageSparseMemoryRequirements
    ));
pub(super) const VK_CMD_SET_EVENT2_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetEvent2),
);
pub(super) const VK_CMD_RESET_EVENT2_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdResetEvent2),
);
pub(super) const VK_CMD_WAIT_EVENTS2_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdWaitEvents2),
);
pub(super) const VK_CMD_BLIT_IMAGE2_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBlitImage2),
);
pub(super) const VK_CMD_RESOLVE_IMAGE2_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdResolveImage2),
);
pub(super) const VK_CMD_BEGIN_RENDERING_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBeginRendering),
);
pub(super) const VK_CMD_END_RENDERING_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdEndRendering),
);
pub(super) const VK_CMD_SET_CULL_MODE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetCullMode),
);
pub(super) const VK_CMD_SET_FRONT_FACE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetFrontFace),
);
pub(super) const VK_CMD_SET_PRIMITIVE_TOPOLOGY_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetPrimitiveTopology),
);
pub(super) const VK_CMD_SET_VIEWPORT_WITH_COUNT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetViewportWithCount),
);
pub(super) const VK_CMD_SET_SCISSOR_WITH_COUNT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetScissorWithCount),
);
pub(super) const VK_CMD_BIND_VERTEX_BUFFERS2_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBindVertexBuffers2),
);
pub(super) const VK_CMD_SET_DEPTH_TEST_ENABLE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetDepthTestEnable),
);
pub(super) const VK_CMD_SET_DEPTH_WRITE_ENABLE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetDepthWriteEnable),
);
pub(super) const VK_CMD_SET_DEPTH_COMPARE_OP_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetDepthCompareOp),
);
pub(super) const VK_CMD_SET_DEPTH_BOUNDS_TEST_ENABLE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetDepthBoundsTestEnable),
);
pub(super) const VK_CMD_SET_STENCIL_TEST_ENABLE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetStencilTestEnable),
);
pub(super) const VK_CMD_SET_STENCIL_OP_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetStencilOp),
);
pub(super) const VK_CMD_SET_RASTERIZER_DISCARD_ENABLE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetRasterizerDiscardEnable),
);
pub(super) const VK_CMD_SET_DEPTH_BIAS_ENABLE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetDepthBiasEnable),
);
pub(super) const VK_CMD_SET_PRIMITIVE_RESTART_ENABLE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetPrimitiveRestartEnable),
);
pub(super) const VK_MAP_MEMORY2_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkMapMemory2),
);
pub(super) const VK_UNMAP_MEMORY2_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkUnmapMemory2),
);
pub(super) const VK_GET_DEVICE_IMAGE_SUBRESOURCE_LAYOUT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetDeviceImageSubresourceLayout
    ));
pub(super) const VK_GET_IMAGE_SUBRESOURCE_LAYOUT2_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetImageSubresourceLayout2),
);
pub(super) const VK_COPY_MEMORY_TO_IMAGE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCopyMemoryToImage),
);
pub(super) const VK_COPY_IMAGE_TO_MEMORY_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCopyImageToMemory),
);
pub(super) const VK_COPY_IMAGE_TO_IMAGE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCopyImageToImage),
);
pub(super) const VK_TRANSITION_IMAGE_LAYOUT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkTransitionImageLayout),
);
pub(super) const VK_CMD_PUSH_DESCRIPTOR_SET_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdPushDescriptorSet),
);
pub(super) const VK_CMD_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdPushDescriptorSetWithTemplate
    ));
pub(super) const VK_CMD_BIND_DESCRIPTOR_SETS2_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBindDescriptorSets2),
);
pub(super) const VK_CMD_PUSH_CONSTANTS2_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdPushConstants2),
);
pub(super) const VK_CMD_PUSH_DESCRIPTOR_SET2_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdPushDescriptorSet2),
);
pub(super) const VK_CMD_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE2_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdPushDescriptorSetWithTemplate2
    ));
pub(super) const VK_CMD_SET_LINE_STIPPLE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetLineStipple),
);
pub(super) const VK_CMD_BIND_INDEX_BUFFER2_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBindIndexBuffer2),
);
pub(super) const VK_GET_RENDERING_AREA_GRANULARITY_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetRenderingAreaGranularity),
);
pub(super) const VK_CMD_SET_RENDERING_ATTACHMENT_LOCATIONS_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetRenderingAttachmentLocations
    ));
pub(super) const VK_CMD_SET_RENDERING_INPUT_ATTACHMENT_INDICES_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetRenderingInputAttachmentIndices
    ));
pub(super) const VK_CREATE_SWAPCHAIN_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateSwapchainKHR),
);
pub(super) const VK_DESTROY_SWAPCHAIN_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroySwapchainKHR),
);
pub(super) const VK_GET_SWAPCHAIN_IMAGES_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetSwapchainImagesKHR),
);
pub(super) const VK_ACQUIRE_NEXT_IMAGE_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkAcquireNextImageKHR),
);
pub(super) const VK_QUEUE_PRESENT_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkQueuePresentKHR),
);
pub(super) const VK_GET_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetDeviceGroupPresentCapabilitiesKHR
    ));
pub(super) const VK_GET_DEVICE_GROUP_SURFACE_PRESENT_MODES_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetDeviceGroupSurfacePresentModesKHR
    ));
pub(super) const VK_ACQUIRE_NEXT_IMAGE2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkAcquireNextImage2KHR),
);
pub(super) const VK_CREATE_SHARED_SWAPCHAINS_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateSharedSwapchainsKHR),
);
pub(super) const VK_CREATE_VIDEO_SESSION_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateVideoSessionKHR),
);
pub(super) const VK_DESTROY_VIDEO_SESSION_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyVideoSessionKHR),
);
pub(super) const VK_GET_VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetVideoSessionMemoryRequirementsKHR
    ));
pub(super) const VK_BIND_VIDEO_SESSION_MEMORY_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkBindVideoSessionMemoryKHR),
);
pub(super) const VK_CREATE_VIDEO_SESSION_PARAMETERS_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCreateVideoSessionParametersKHR
    ));
pub(super) const VK_UPDATE_VIDEO_SESSION_PARAMETERS_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkUpdateVideoSessionParametersKHR
    ));
pub(super) const VK_DESTROY_VIDEO_SESSION_PARAMETERS_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkDestroyVideoSessionParametersKHR
    ));
pub(super) const VK_CMD_BEGIN_VIDEO_CODING_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBeginVideoCodingKHR),
);
pub(super) const VK_CMD_END_VIDEO_CODING_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdEndVideoCodingKHR),
);
pub(super) const VK_CMD_CONTROL_VIDEO_CODING_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdControlVideoCodingKHR),
);
pub(super) const VK_CMD_DECODE_VIDEO_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDecodeVideoKHR),
);
pub(super) const VK_CMD_BEGIN_RENDERING_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBeginRenderingKHR),
);
pub(super) const VK_CMD_END_RENDERING_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdEndRenderingKHR),
);
pub(super) const VK_GET_DEVICE_GROUP_PEER_MEMORY_FEATURES_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetDeviceGroupPeerMemoryFeaturesKHR
    ));
pub(super) const VK_CMD_SET_DEVICE_MASK_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetDeviceMaskKHR),
);
pub(super) const VK_CMD_DISPATCH_BASE_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDispatchBaseKHR),
);
pub(super) const VK_TRIM_COMMAND_POOL_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkTrimCommandPoolKHR),
);
#[cfg(target_os = "windows")]
pub(super) const VK_GET_MEMORY_WIN32HANDLE_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetMemoryWin32HandleKHR),
);
#[cfg(not(target_os = "windows"))]
pub(super) const VK_GET_MEMORY_WIN32HANDLE_KHR_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
#[cfg(target_os = "windows")]
pub(super) const VK_GET_MEMORY_WIN32HANDLE_PROPERTIES_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetMemoryWin32HandlePropertiesKHR
    ));
#[cfg(not(target_os = "windows"))]
pub(super) const VK_GET_MEMORY_WIN32HANDLE_PROPERTIES_KHR_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
pub(super) const VK_GET_MEMORY_FD_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetMemoryFdKHR),
);
pub(super) const VK_GET_MEMORY_FD_PROPERTIES_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetMemoryFdPropertiesKHR),
);
#[cfg(target_os = "windows")]
pub(super) const VK_IMPORT_SEMAPHORE_WIN32HANDLE_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkImportSemaphoreWin32HandleKHR),
);
#[cfg(not(target_os = "windows"))]
pub(super) const VK_IMPORT_SEMAPHORE_WIN32HANDLE_KHR_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
#[cfg(target_os = "windows")]
pub(super) const VK_GET_SEMAPHORE_WIN32HANDLE_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetSemaphoreWin32HandleKHR),
);
#[cfg(not(target_os = "windows"))]
pub(super) const VK_GET_SEMAPHORE_WIN32HANDLE_KHR_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
pub(super) const VK_IMPORT_SEMAPHORE_FD_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkImportSemaphoreFdKHR),
);
pub(super) const VK_GET_SEMAPHORE_FD_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetSemaphoreFdKHR),
);
pub(super) const VK_CMD_PUSH_DESCRIPTOR_SET_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdPushDescriptorSetKHR),
);
pub(super) const VK_CMD_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdPushDescriptorSetWithTemplateKHR
    ));
pub(super) const VK_CREATE_DESCRIPTOR_UPDATE_TEMPLATE_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCreateDescriptorUpdateTemplateKHR
    ));
pub(super) const VK_DESTROY_DESCRIPTOR_UPDATE_TEMPLATE_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkDestroyDescriptorUpdateTemplateKHR
    ));
pub(super) const VK_UPDATE_DESCRIPTOR_SET_WITH_TEMPLATE_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkUpdateDescriptorSetWithTemplateKHR
    ));
pub(super) const VK_CREATE_RENDER_PASS2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateRenderPass2KHR),
);
pub(super) const VK_CMD_BEGIN_RENDER_PASS2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBeginRenderPass2KHR),
);
pub(super) const VK_CMD_NEXT_SUBPASS2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdNextSubpass2KHR),
);
pub(super) const VK_CMD_END_RENDER_PASS2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdEndRenderPass2KHR),
);
pub(super) const VK_GET_SWAPCHAIN_STATUS_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetSwapchainStatusKHR),
);
#[cfg(target_os = "windows")]
pub(super) const VK_IMPORT_FENCE_WIN32HANDLE_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkImportFenceWin32HandleKHR),
);
#[cfg(not(target_os = "windows"))]
pub(super) const VK_IMPORT_FENCE_WIN32HANDLE_KHR_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
#[cfg(target_os = "windows")]
pub(super) const VK_GET_FENCE_WIN32HANDLE_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetFenceWin32HandleKHR),
);
#[cfg(not(target_os = "windows"))]
pub(super) const VK_GET_FENCE_WIN32HANDLE_KHR_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
pub(super) const VK_IMPORT_FENCE_FD_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkImportFenceFdKHR),
);
pub(super) const VK_GET_FENCE_FD_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetFenceFdKHR),
);
pub(super) const VK_ACQUIRE_PROFILING_LOCK_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkAcquireProfilingLockKHR),
);
pub(super) const VK_RELEASE_PROFILING_LOCK_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkReleaseProfilingLockKHR),
);
pub(super) const VK_GET_IMAGE_MEMORY_REQUIREMENTS2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetImageMemoryRequirements2KHR),
);
pub(super) const VK_GET_BUFFER_MEMORY_REQUIREMENTS2KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetBufferMemoryRequirements2KHR
    ));
pub(super) const VK_GET_IMAGE_SPARSE_MEMORY_REQUIREMENTS2KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetImageSparseMemoryRequirements2KHR
    ));
pub(super) const VK_CREATE_SAMPLER_YCBCR_CONVERSION_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCreateSamplerYcbcrConversionKHR
    ));
pub(super) const VK_DESTROY_SAMPLER_YCBCR_CONVERSION_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkDestroySamplerYcbcrConversionKHR
    ));
pub(super) const VK_BIND_BUFFER_MEMORY2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkBindBufferMemory2KHR),
);
pub(super) const VK_BIND_IMAGE_MEMORY2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkBindImageMemory2KHR),
);
pub(super) const VK_GET_DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetDescriptorSetLayoutSupportKHR
    ));
pub(super) const VK_CMD_DRAW_INDIRECT_COUNT_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDrawIndirectCountKHR),
);
pub(super) const VK_CMD_DRAW_INDEXED_INDIRECT_COUNT_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdDrawIndexedIndirectCountKHR
    ));
pub(super) const VK_GET_SEMAPHORE_COUNTER_VALUE_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetSemaphoreCounterValueKHR),
);
pub(super) const VK_WAIT_SEMAPHORES_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkWaitSemaphoresKHR),
);
pub(super) const VK_SIGNAL_SEMAPHORE_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkSignalSemaphoreKHR),
);
pub(super) const VK_CMD_SET_FRAGMENT_SHADING_RATE_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetFragmentShadingRateKHR),
);
pub(super) const VK_CMD_SET_RENDERING_ATTACHMENT_LOCATIONS_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetRenderingAttachmentLocationsKHR
    ));
pub(super) const VK_CMD_SET_RENDERING_INPUT_ATTACHMENT_INDICES_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetRenderingInputAttachmentIndicesKHR
    ));
pub(super) const VK_WAIT_FOR_PRESENT_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkWaitForPresentKHR),
);
pub(super) const VK_GET_BUFFER_DEVICE_ADDRESS_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetBufferDeviceAddressKHR),
);
pub(super) const VK_GET_BUFFER_OPAQUE_CAPTURE_ADDRESS_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetBufferOpaqueCaptureAddressKHR
    ));
pub(super) const VK_GET_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetDeviceMemoryOpaqueCaptureAddressKHR
    ));
pub(super) const VK_CREATE_DEFERRED_OPERATION_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateDeferredOperationKHR),
);
pub(super) const VK_DESTROY_DEFERRED_OPERATION_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyDeferredOperationKHR),
);
pub(super) const VK_GET_DEFERRED_OPERATION_MAX_CONCURRENCY_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetDeferredOperationMaxConcurrencyKHR
    ));
pub(super) const VK_GET_DEFERRED_OPERATION_RESULT_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetDeferredOperationResultKHR),
);
pub(super) const VK_DEFERRED_OPERATION_JOIN_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDeferredOperationJoinKHR),
);
pub(super) const VK_GET_PIPELINE_EXECUTABLE_PROPERTIES_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetPipelineExecutablePropertiesKHR
    ));
pub(super) const VK_GET_PIPELINE_EXECUTABLE_STATISTICS_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetPipelineExecutableStatisticsKHR
    ));
pub(super) const VK_GET_PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATIONS_KHR_DEVICE_DISPATCH_OFFSET:
    u16 = dispatch_offset(core::mem::offset_of!(
    LayerDeviceDispatchTable,
    vkGetPipelineExecutableInternalRepresentationsKHR
));
pub(super) const VK_MAP_MEMORY2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkMapMemory2KHR),
);
pub(super) const VK_UNMAP_MEMORY2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkUnmapMemory2KHR),
);
pub(super) const VK_GET_ENCODED_VIDEO_SESSION_PARAMETERS_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetEncodedVideoSessionParametersKHR
    ));
pub(super) const VK_CMD_ENCODE_VIDEO_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdEncodeVideoKHR),
);
pub(super) const VK_CMD_SET_EVENT2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetEvent2KHR),
);
pub(super) const VK_CMD_RESET_EVENT2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdResetEvent2KHR),
);
pub(super) const VK_CMD_WAIT_EVENTS2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdWaitEvents2KHR),
);
pub(super) const VK_CMD_PIPELINE_BARRIER2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdPipelineBarrier2KHR),
);
pub(super) const VK_CMD_WRITE_TIMESTAMP2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdWriteTimestamp2KHR),
);
pub(super) const VK_QUEUE_SUBMIT2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkQueueSubmit2KHR),
);
pub(super) const VK_CMD_BIND_INDEX_BUFFER3KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBindIndexBuffer3KHR),
);
pub(super) const VK_CMD_BIND_VERTEX_BUFFERS3KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBindVertexBuffers3KHR),
);
pub(super) const VK_CMD_DRAW_INDIRECT2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDrawIndirect2KHR),
);
pub(super) const VK_CMD_DRAW_INDEXED_INDIRECT2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDrawIndexedIndirect2KHR),
);
pub(super) const VK_CMD_DISPATCH_INDIRECT2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDispatchIndirect2KHR),
);
pub(super) const VK_CMD_COPY_MEMORY_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdCopyMemoryKHR),
);
pub(super) const VK_CMD_COPY_MEMORY_TO_IMAGE_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdCopyMemoryToImageKHR),
);
pub(super) const VK_CMD_COPY_IMAGE_TO_MEMORY_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdCopyImageToMemoryKHR),
);
pub(super) const VK_CMD_UPDATE_MEMORY_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdUpdateMemoryKHR),
);
pub(super) const VK_CMD_FILL_MEMORY_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdFillMemoryKHR),
);
pub(super) const VK_CMD_COPY_QUERY_POOL_RESULTS_TO_MEMORY_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdCopyQueryPoolResultsToMemoryKHR
    ));
pub(super) const VK_CMD_DRAW_INDIRECT_COUNT2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDrawIndirectCount2KHR),
);
pub(super) const VK_CMD_DRAW_INDEXED_INDIRECT_COUNT2KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdDrawIndexedIndirectCount2KHR
    ));
pub(super) const VK_CMD_BEGIN_CONDITIONAL_RENDERING2EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdBeginConditionalRendering2EXT
    ));
pub(super) const VK_CMD_BIND_TRANSFORM_FEEDBACK_BUFFERS2EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdBindTransformFeedbackBuffers2EXT
    ));
pub(super) const VK_CMD_BEGIN_TRANSFORM_FEEDBACK2EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBeginTransformFeedback2EXT),
);
pub(super) const VK_CMD_END_TRANSFORM_FEEDBACK2EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdEndTransformFeedback2EXT),
);
pub(super) const VK_CMD_DRAW_INDIRECT_BYTE_COUNT2EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDrawIndirectByteCount2EXT),
);
pub(super) const VK_CMD_DRAW_MESH_TASKS_INDIRECT2EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDrawMeshTasksIndirect2EXT),
);
pub(super) const VK_CMD_DRAW_MESH_TASKS_INDIRECT_COUNT2EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdDrawMeshTasksIndirectCount2EXT
    ));
pub(super) const VK_CMD_WRITE_MARKER_TO_MEMORY_AMD_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdWriteMarkerToMemoryAMD),
);
pub(super) const VK_CREATE_ACCELERATION_STRUCTURE2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateAccelerationStructure2KHR),
);
pub(super) const VK_CMD_COPY_BUFFER2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdCopyBuffer2KHR),
);
pub(super) const VK_CMD_COPY_IMAGE2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdCopyImage2KHR),
);
pub(super) const VK_CMD_COPY_BUFFER_TO_IMAGE2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdCopyBufferToImage2KHR),
);
pub(super) const VK_CMD_COPY_IMAGE_TO_BUFFER2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdCopyImageToBuffer2KHR),
);
pub(super) const VK_CMD_BLIT_IMAGE2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBlitImage2KHR),
);
pub(super) const VK_CMD_RESOLVE_IMAGE2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdResolveImage2KHR),
);
pub(super) const VK_CMD_TRACE_RAYS_INDIRECT2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdTraceRaysIndirect2KHR),
);
pub(super) const VK_GET_DEVICE_BUFFER_MEMORY_REQUIREMENTS_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetDeviceBufferMemoryRequirementsKHR
    ));
pub(super) const VK_GET_DEVICE_IMAGE_MEMORY_REQUIREMENTS_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetDeviceImageMemoryRequirementsKHR
    ));
pub(super) const VK_GET_DEVICE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetDeviceImageSparseMemoryRequirementsKHR
    ));
pub(super) const VK_CMD_BIND_INDEX_BUFFER2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBindIndexBuffer2KHR),
);
pub(super) const VK_GET_RENDERING_AREA_GRANULARITY_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetRenderingAreaGranularityKHR
    ));
pub(super) const VK_GET_DEVICE_IMAGE_SUBRESOURCE_LAYOUT_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetDeviceImageSubresourceLayoutKHR
    ));
pub(super) const VK_GET_IMAGE_SUBRESOURCE_LAYOUT2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetImageSubresourceLayout2KHR),
);
pub(super) const VK_WAIT_FOR_PRESENT2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkWaitForPresent2KHR),
);
pub(super) const VK_CREATE_PIPELINE_BINARIES_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreatePipelineBinariesKHR),
);
pub(super) const VK_DESTROY_PIPELINE_BINARY_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyPipelineBinaryKHR),
);
pub(super) const VK_GET_PIPELINE_KEY_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetPipelineKeyKHR),
);
pub(super) const VK_GET_PIPELINE_BINARY_DATA_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetPipelineBinaryDataKHR),
);
pub(super) const VK_RELEASE_CAPTURED_PIPELINE_DATA_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkReleaseCapturedPipelineDataKHR
    ));
pub(super) const VK_RELEASE_SWAPCHAIN_IMAGES_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkReleaseSwapchainImagesKHR),
);
pub(super) const VK_CMD_SET_LINE_STIPPLE_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetLineStippleKHR),
);
pub(super) const VK_GET_CALIBRATED_TIMESTAMPS_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetCalibratedTimestampsKHR),
);
pub(super) const VK_CMD_BIND_DESCRIPTOR_SETS2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBindDescriptorSets2KHR),
);
pub(super) const VK_CMD_PUSH_CONSTANTS2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdPushConstants2KHR),
);
pub(super) const VK_CMD_PUSH_DESCRIPTOR_SET2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdPushDescriptorSet2KHR),
);
pub(super) const VK_CMD_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE2KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdPushDescriptorSetWithTemplate2KHR
    ));
pub(super) const VK_CMD_SET_DESCRIPTOR_BUFFER_OFFSETS2EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetDescriptorBufferOffsets2EXT
    ));
pub(super) const VK_CMD_BIND_DESCRIPTOR_BUFFER_EMBEDDED_SAMPLERS2EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdBindDescriptorBufferEmbeddedSamplers2EXT
    ));
pub(super) const VK_CMD_COPY_MEMORY_INDIRECT_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdCopyMemoryIndirectKHR),
);
pub(super) const VK_CMD_COPY_MEMORY_TO_IMAGE_INDIRECT_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdCopyMemoryToImageIndirectKHR
    ));
pub(super) const VK_GET_DEVICE_FAULT_REPORTS_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetDeviceFaultReportsKHR),
);
pub(super) const VK_GET_DEVICE_FAULT_DEBUG_INFO_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetDeviceFaultDebugInfoKHR),
);
pub(super) const VK_CMD_END_RENDERING2KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdEndRendering2KHR),
);
pub(super) const VK_DEBUG_MARKER_SET_OBJECT_TAG_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDebugMarkerSetObjectTagEXT),
);
pub(super) const VK_DEBUG_MARKER_SET_OBJECT_NAME_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDebugMarkerSetObjectNameEXT),
);
pub(super) const VK_CMD_DEBUG_MARKER_BEGIN_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDebugMarkerBeginEXT),
);
pub(super) const VK_CMD_DEBUG_MARKER_END_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDebugMarkerEndEXT),
);
pub(super) const VK_CMD_DEBUG_MARKER_INSERT_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDebugMarkerInsertEXT),
);
pub(super) const VK_CMD_BIND_TRANSFORM_FEEDBACK_BUFFERS_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdBindTransformFeedbackBuffersEXT
    ));
pub(super) const VK_CMD_BEGIN_TRANSFORM_FEEDBACK_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBeginTransformFeedbackEXT),
);
pub(super) const VK_CMD_END_TRANSFORM_FEEDBACK_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdEndTransformFeedbackEXT),
);
pub(super) const VK_CMD_BEGIN_QUERY_INDEXED_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBeginQueryIndexedEXT),
);
pub(super) const VK_CMD_END_QUERY_INDEXED_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdEndQueryIndexedEXT),
);
pub(super) const VK_CMD_DRAW_INDIRECT_BYTE_COUNT_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDrawIndirectByteCountEXT),
);
pub(super) const VK_CREATE_CU_MODULE_NVX_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateCuModuleNVX),
);
pub(super) const VK_CREATE_CU_FUNCTION_NVX_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateCuFunctionNVX),
);
pub(super) const VK_DESTROY_CU_MODULE_NVX_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyCuModuleNVX),
);
pub(super) const VK_DESTROY_CU_FUNCTION_NVX_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyCuFunctionNVX),
);
pub(super) const VK_CMD_CU_LAUNCH_KERNEL_NVX_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdCuLaunchKernelNVX),
);
pub(super) const VK_GET_IMAGE_VIEW_HANDLE_NVX_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetImageViewHandleNVX),
);
pub(super) const VK_GET_IMAGE_VIEW_HANDLE64NVX_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetImageViewHandle64NVX),
);
pub(super) const VK_GET_IMAGE_VIEW_ADDRESS_NVX_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetImageViewAddressNVX),
);
pub(super) const VK_GET_DEVICE_COMBINED_IMAGE_SAMPLER_INDEX_NVX_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetDeviceCombinedImageSamplerIndexNVX
    ));
pub(super) const VK_CMD_DRAW_INDIRECT_COUNT_AMD_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDrawIndirectCountAMD),
);
pub(super) const VK_CMD_DRAW_INDEXED_INDIRECT_COUNT_AMD_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdDrawIndexedIndirectCountAMD
    ));
pub(super) const VK_GET_SHADER_INFO_AMD_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetShaderInfoAMD),
);
#[cfg(target_os = "windows")]
pub(super) const VK_GET_MEMORY_WIN32HANDLE_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetMemoryWin32HandleNV),
);
#[cfg(not(target_os = "windows"))]
pub(super) const VK_GET_MEMORY_WIN32HANDLE_NV_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
pub(super) const VK_CMD_BEGIN_CONDITIONAL_RENDERING_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdBeginConditionalRenderingEXT
    ));
pub(super) const VK_CMD_END_CONDITIONAL_RENDERING_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdEndConditionalRenderingEXT),
);
pub(super) const VK_CMD_SET_VIEWPORT_W_SCALING_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetViewportWScalingNV),
);
pub(super) const VK_DISPLAY_POWER_CONTROL_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDisplayPowerControlEXT),
);
pub(super) const VK_REGISTER_DEVICE_EVENT_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkRegisterDeviceEventEXT),
);
pub(super) const VK_REGISTER_DISPLAY_EVENT_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkRegisterDisplayEventEXT),
);
pub(super) const VK_GET_SWAPCHAIN_COUNTER_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetSwapchainCounterEXT),
);
pub(super) const VK_GET_REFRESH_CYCLE_DURATION_GOOGLE_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetRefreshCycleDurationGOOGLE),
);
pub(super) const VK_GET_PAST_PRESENTATION_TIMING_GOOGLE_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetPastPresentationTimingGOOGLE
    ));
pub(super) const VK_CMD_SET_DISCARD_RECTANGLE_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetDiscardRectangleEXT),
);
pub(super) const VK_CMD_SET_DISCARD_RECTANGLE_ENABLE_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetDiscardRectangleEnableEXT
    ));
pub(super) const VK_CMD_SET_DISCARD_RECTANGLE_MODE_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetDiscardRectangleModeEXT
    ));
pub(super) const VK_SET_HDR_METADATA_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkSetHdrMetadataEXT),
);
pub(super) const VK_SET_DEBUG_UTILS_OBJECT_NAME_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkSetDebugUtilsObjectNameEXT),
);
pub(super) const VK_SET_DEBUG_UTILS_OBJECT_TAG_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkSetDebugUtilsObjectTagEXT),
);
pub(super) const VK_QUEUE_BEGIN_DEBUG_UTILS_LABEL_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkQueueBeginDebugUtilsLabelEXT),
);
pub(super) const VK_QUEUE_END_DEBUG_UTILS_LABEL_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkQueueEndDebugUtilsLabelEXT),
);
pub(super) const VK_QUEUE_INSERT_DEBUG_UTILS_LABEL_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkQueueInsertDebugUtilsLabelEXT
    ));
pub(super) const VK_CMD_BEGIN_DEBUG_UTILS_LABEL_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBeginDebugUtilsLabelEXT),
);
pub(super) const VK_CMD_END_DEBUG_UTILS_LABEL_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdEndDebugUtilsLabelEXT),
);
pub(super) const VK_CMD_INSERT_DEBUG_UTILS_LABEL_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdInsertDebugUtilsLabelEXT),
);
#[cfg(target_os = "android")]
pub(super) const VK_GET_ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetAndroidHardwareBufferPropertiesANDROID
    ));
#[cfg(not(target_os = "android"))]
pub(super) const VK_GET_ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID_DEVICE_DISPATCH_OFFSET: u16 =
    u16::MAX;
#[cfg(target_os = "android")]
pub(super) const VK_GET_MEMORY_ANDROID_HARDWARE_BUFFER_ANDROID_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetMemoryAndroidHardwareBufferANDROID
    ));
#[cfg(not(target_os = "android"))]
pub(super) const VK_GET_MEMORY_ANDROID_HARDWARE_BUFFER_ANDROID_DEVICE_DISPATCH_OFFSET: u16 =
    u16::MAX;
pub(super) const VK_CREATE_GPA_SESSION_AMD_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateGpaSessionAMD),
);
pub(super) const VK_DESTROY_GPA_SESSION_AMD_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyGpaSessionAMD),
);
pub(super) const VK_SET_GPA_DEVICE_CLOCK_MODE_AMD_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkSetGpaDeviceClockModeAMD),
);
pub(super) const VK_GET_GPA_DEVICE_CLOCK_INFO_AMD_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetGpaDeviceClockInfoAMD),
);
pub(super) const VK_CMD_BEGIN_GPA_SESSION_AMD_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBeginGpaSessionAMD),
);
pub(super) const VK_CMD_END_GPA_SESSION_AMD_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdEndGpaSessionAMD),
);
pub(super) const VK_CMD_BEGIN_GPA_SAMPLE_AMD_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBeginGpaSampleAMD),
);
pub(super) const VK_CMD_END_GPA_SAMPLE_AMD_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdEndGpaSampleAMD),
);
pub(super) const VK_GET_GPA_SESSION_STATUS_AMD_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetGpaSessionStatusAMD),
);
pub(super) const VK_GET_GPA_SESSION_RESULTS_AMD_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetGpaSessionResultsAMD),
);
pub(super) const VK_RESET_GPA_SESSION_AMD_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkResetGpaSessionAMD),
);
pub(super) const VK_CMD_COPY_GPA_SESSION_RESULTS_AMD_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdCopyGpaSessionResultsAMD),
);
#[cfg(feature = "beta-extensions")]
pub(super) const VK_CREATE_EXECUTION_GRAPH_PIPELINES_AMDX_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCreateExecutionGraphPipelinesAMDX
    ));
#[cfg(not(feature = "beta-extensions"))]
pub(super) const VK_CREATE_EXECUTION_GRAPH_PIPELINES_AMDX_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
#[cfg(feature = "beta-extensions")]
pub(super) const VK_GET_EXECUTION_GRAPH_PIPELINE_SCRATCH_SIZE_AMDX_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetExecutionGraphPipelineScratchSizeAMDX
    ));
#[cfg(not(feature = "beta-extensions"))]
pub(super) const VK_GET_EXECUTION_GRAPH_PIPELINE_SCRATCH_SIZE_AMDX_DEVICE_DISPATCH_OFFSET: u16 =
    u16::MAX;
#[cfg(feature = "beta-extensions")]
pub(super) const VK_GET_EXECUTION_GRAPH_PIPELINE_NODE_INDEX_AMDX_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetExecutionGraphPipelineNodeIndexAMDX
    ));
#[cfg(not(feature = "beta-extensions"))]
pub(super) const VK_GET_EXECUTION_GRAPH_PIPELINE_NODE_INDEX_AMDX_DEVICE_DISPATCH_OFFSET: u16 =
    u16::MAX;
#[cfg(feature = "beta-extensions")]
pub(super) const VK_CMD_INITIALIZE_GRAPH_SCRATCH_MEMORY_AMDX_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdInitializeGraphScratchMemoryAMDX
    ));
#[cfg(not(feature = "beta-extensions"))]
pub(super) const VK_CMD_INITIALIZE_GRAPH_SCRATCH_MEMORY_AMDX_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
#[cfg(feature = "beta-extensions")]
pub(super) const VK_CMD_DISPATCH_GRAPH_AMDX_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDispatchGraphAMDX),
);
#[cfg(not(feature = "beta-extensions"))]
pub(super) const VK_CMD_DISPATCH_GRAPH_AMDX_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
#[cfg(feature = "beta-extensions")]
pub(super) const VK_CMD_DISPATCH_GRAPH_INDIRECT_AMDX_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDispatchGraphIndirectAMDX),
);
#[cfg(not(feature = "beta-extensions"))]
pub(super) const VK_CMD_DISPATCH_GRAPH_INDIRECT_AMDX_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
#[cfg(feature = "beta-extensions")]
pub(super) const VK_CMD_DISPATCH_GRAPH_INDIRECT_COUNT_AMDX_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdDispatchGraphIndirectCountAMDX
    ));
#[cfg(not(feature = "beta-extensions"))]
pub(super) const VK_CMD_DISPATCH_GRAPH_INDIRECT_COUNT_AMDX_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
pub(super) const VK_WRITE_SAMPLER_DESCRIPTORS_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkWriteSamplerDescriptorsEXT),
);
pub(super) const VK_WRITE_RESOURCE_DESCRIPTORS_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkWriteResourceDescriptorsEXT),
);
pub(super) const VK_CMD_BIND_SAMPLER_HEAP_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBindSamplerHeapEXT),
);
pub(super) const VK_CMD_BIND_RESOURCE_HEAP_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBindResourceHeapEXT),
);
pub(super) const VK_CMD_PUSH_DATA_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdPushDataEXT),
);
pub(super) const VK_GET_IMAGE_OPAQUE_CAPTURE_DATA_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetImageOpaqueCaptureDataEXT),
);
pub(super) const VK_REGISTER_CUSTOM_BORDER_COLOR_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkRegisterCustomBorderColorEXT),
);
pub(super) const VK_UNREGISTER_CUSTOM_BORDER_COLOR_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkUnregisterCustomBorderColorEXT
    ));
pub(super) const VK_GET_TENSOR_OPAQUE_CAPTURE_DATA_ARM_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetTensorOpaqueCaptureDataARM
    ));
pub(super) const VK_CMD_SET_SAMPLE_LOCATIONS_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetSampleLocationsEXT),
);
pub(super) const VK_GET_IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetImageDrmFormatModifierPropertiesEXT
    ));
pub(super) const VK_CREATE_VALIDATION_CACHE_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateValidationCacheEXT),
);
pub(super) const VK_DESTROY_VALIDATION_CACHE_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyValidationCacheEXT),
);
pub(super) const VK_MERGE_VALIDATION_CACHES_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkMergeValidationCachesEXT),
);
pub(super) const VK_GET_VALIDATION_CACHE_DATA_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetValidationCacheDataEXT),
);
pub(super) const VK_CMD_BIND_SHADING_RATE_IMAGE_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBindShadingRateImageNV),
);
pub(super) const VK_CMD_SET_VIEWPORT_SHADING_RATE_PALETTE_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetViewportShadingRatePaletteNV
    ));
pub(super) const VK_CMD_SET_COARSE_SAMPLE_ORDER_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetCoarseSampleOrderNV),
);
pub(super) const VK_CREATE_ACCELERATION_STRUCTURE_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateAccelerationStructureNV),
);
pub(super) const VK_DESTROY_ACCELERATION_STRUCTURE_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyAccelerationStructureNV),
);
pub(super) const VK_GET_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetAccelerationStructureMemoryRequirementsNV
    ));
pub(super) const VK_BIND_ACCELERATION_STRUCTURE_MEMORY_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkBindAccelerationStructureMemoryNV
    ));
pub(super) const VK_CMD_BUILD_ACCELERATION_STRUCTURE_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdBuildAccelerationStructureNV
    ));
pub(super) const VK_CMD_COPY_ACCELERATION_STRUCTURE_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdCopyAccelerationStructureNV
    ));
pub(super) const VK_CMD_TRACE_RAYS_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdTraceRaysNV),
);
pub(super) const VK_CREATE_RAY_TRACING_PIPELINES_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateRayTracingPipelinesNV),
);
pub(super) const VK_GET_RAY_TRACING_SHADER_GROUP_HANDLES_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetRayTracingShaderGroupHandlesKHR
    ));
pub(super) const VK_GET_RAY_TRACING_SHADER_GROUP_HANDLES_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetRayTracingShaderGroupHandlesNV
    ));
pub(super) const VK_GET_ACCELERATION_STRUCTURE_HANDLE_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetAccelerationStructureHandleNV
    ));
pub(super) const VK_CMD_WRITE_ACCELERATION_STRUCTURES_PROPERTIES_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdWriteAccelerationStructuresPropertiesNV
    ));
pub(super) const VK_COMPILE_DEFERRED_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCompileDeferredNV),
);
pub(super) const VK_GET_MEMORY_HOST_POINTER_PROPERTIES_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetMemoryHostPointerPropertiesEXT
    ));
pub(super) const VK_CMD_WRITE_BUFFER_MARKER_AMD_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdWriteBufferMarkerAMD),
);
pub(super) const VK_CMD_WRITE_BUFFER_MARKER2AMD_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdWriteBufferMarker2AMD),
);
pub(super) const VK_GET_CALIBRATED_TIMESTAMPS_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetCalibratedTimestampsEXT),
);
pub(super) const VK_CMD_DRAW_MESH_TASKS_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDrawMeshTasksNV),
);
pub(super) const VK_CMD_DRAW_MESH_TASKS_INDIRECT_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDrawMeshTasksIndirectNV),
);
pub(super) const VK_CMD_DRAW_MESH_TASKS_INDIRECT_COUNT_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdDrawMeshTasksIndirectCountNV
    ));
pub(super) const VK_CMD_SET_EXCLUSIVE_SCISSOR_ENABLE_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetExclusiveScissorEnableNV
    ));
pub(super) const VK_CMD_SET_EXCLUSIVE_SCISSOR_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetExclusiveScissorNV),
);
pub(super) const VK_CMD_SET_CHECKPOINT_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetCheckpointNV),
);
pub(super) const VK_GET_QUEUE_CHECKPOINT_DATA_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetQueueCheckpointDataNV),
);
pub(super) const VK_GET_QUEUE_CHECKPOINT_DATA2NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetQueueCheckpointData2NV),
);
pub(super) const VK_SET_SWAPCHAIN_PRESENT_TIMING_QUEUE_SIZE_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkSetSwapchainPresentTimingQueueSizeEXT
    ));
pub(super) const VK_GET_SWAPCHAIN_TIMING_PROPERTIES_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetSwapchainTimingPropertiesEXT
    ));
pub(super) const VK_GET_SWAPCHAIN_TIME_DOMAIN_PROPERTIES_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetSwapchainTimeDomainPropertiesEXT
    ));
pub(super) const VK_GET_PAST_PRESENTATION_TIMING_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetPastPresentationTimingEXT),
);
pub(super) const VK_INITIALIZE_PERFORMANCE_API_INTEL_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkInitializePerformanceApiINTEL),
);
pub(super) const VK_UNINITIALIZE_PERFORMANCE_API_INTEL_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkUninitializePerformanceApiINTEL
    ));
pub(super) const VK_CMD_SET_PERFORMANCE_MARKER_INTEL_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetPerformanceMarkerINTEL),
);
pub(super) const VK_CMD_SET_PERFORMANCE_STREAM_MARKER_INTEL_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetPerformanceStreamMarkerINTEL
    ));
pub(super) const VK_CMD_SET_PERFORMANCE_OVERRIDE_INTEL_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetPerformanceOverrideINTEL
    ));
pub(super) const VK_ACQUIRE_PERFORMANCE_CONFIGURATION_INTEL_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkAcquirePerformanceConfigurationINTEL
    ));
pub(super) const VK_RELEASE_PERFORMANCE_CONFIGURATION_INTEL_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkReleasePerformanceConfigurationINTEL
    ));
pub(super) const VK_QUEUE_SET_PERFORMANCE_CONFIGURATION_INTEL_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkQueueSetPerformanceConfigurationINTEL
    ));
pub(super) const VK_GET_PERFORMANCE_PARAMETER_INTEL_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetPerformanceParameterINTEL),
);
pub(super) const VK_SET_LOCAL_DIMMING_AMD_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkSetLocalDimmingAMD),
);
pub(super) const VK_GET_BUFFER_DEVICE_ADDRESS_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetBufferDeviceAddressEXT),
);
#[cfg(target_os = "windows")]
pub(super) const VK_ACQUIRE_FULL_SCREEN_EXCLUSIVE_MODE_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkAcquireFullScreenExclusiveModeEXT
    ));
#[cfg(not(target_os = "windows"))]
pub(super) const VK_ACQUIRE_FULL_SCREEN_EXCLUSIVE_MODE_EXT_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
#[cfg(target_os = "windows")]
pub(super) const VK_RELEASE_FULL_SCREEN_EXCLUSIVE_MODE_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkReleaseFullScreenExclusiveModeEXT
    ));
#[cfg(not(target_os = "windows"))]
pub(super) const VK_RELEASE_FULL_SCREEN_EXCLUSIVE_MODE_EXT_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
#[cfg(target_os = "windows")]
pub(super) const VK_GET_DEVICE_GROUP_SURFACE_PRESENT_MODES2EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetDeviceGroupSurfacePresentModes2EXT
    ));
#[cfg(not(target_os = "windows"))]
pub(super) const VK_GET_DEVICE_GROUP_SURFACE_PRESENT_MODES2EXT_DEVICE_DISPATCH_OFFSET: u16 =
    u16::MAX;
pub(super) const VK_CMD_SET_LINE_STIPPLE_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetLineStippleEXT),
);
pub(super) const VK_RESET_QUERY_POOL_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkResetQueryPoolEXT),
);
pub(super) const VK_CMD_SET_CULL_MODE_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetCullModeEXT),
);
pub(super) const VK_CMD_SET_FRONT_FACE_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetFrontFaceEXT),
);
pub(super) const VK_CMD_SET_PRIMITIVE_TOPOLOGY_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetPrimitiveTopologyEXT),
);
pub(super) const VK_CMD_SET_VIEWPORT_WITH_COUNT_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetViewportWithCountEXT),
);
pub(super) const VK_CMD_SET_SCISSOR_WITH_COUNT_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetScissorWithCountEXT),
);
pub(super) const VK_CMD_BIND_VERTEX_BUFFERS2EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBindVertexBuffers2EXT),
);
pub(super) const VK_CMD_SET_DEPTH_TEST_ENABLE_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetDepthTestEnableEXT),
);
pub(super) const VK_CMD_SET_DEPTH_WRITE_ENABLE_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetDepthWriteEnableEXT),
);
pub(super) const VK_CMD_SET_DEPTH_COMPARE_OP_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetDepthCompareOpEXT),
);
pub(super) const VK_CMD_SET_DEPTH_BOUNDS_TEST_ENABLE_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetDepthBoundsTestEnableEXT
    ));
pub(super) const VK_CMD_SET_STENCIL_TEST_ENABLE_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetStencilTestEnableEXT),
);
pub(super) const VK_CMD_SET_STENCIL_OP_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetStencilOpEXT),
);
pub(super) const VK_COPY_MEMORY_TO_IMAGE_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCopyMemoryToImageEXT),
);
pub(super) const VK_COPY_IMAGE_TO_MEMORY_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCopyImageToMemoryEXT),
);
pub(super) const VK_COPY_IMAGE_TO_IMAGE_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCopyImageToImageEXT),
);
pub(super) const VK_TRANSITION_IMAGE_LAYOUT_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkTransitionImageLayoutEXT),
);
pub(super) const VK_GET_IMAGE_SUBRESOURCE_LAYOUT2EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetImageSubresourceLayout2EXT),
);
pub(super) const VK_RELEASE_SWAPCHAIN_IMAGES_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkReleaseSwapchainImagesEXT),
);
pub(super) const VK_GET_GENERATED_COMMANDS_MEMORY_REQUIREMENTS_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetGeneratedCommandsMemoryRequirementsNV
    ));
pub(super) const VK_CMD_PREPROCESS_GENERATED_COMMANDS_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdPreprocessGeneratedCommandsNV
    ));
pub(super) const VK_CMD_EXECUTE_GENERATED_COMMANDS_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdExecuteGeneratedCommandsNV),
);
pub(super) const VK_CMD_BIND_PIPELINE_SHADER_GROUP_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBindPipelineShaderGroupNV),
);
pub(super) const VK_CREATE_INDIRECT_COMMANDS_LAYOUT_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCreateIndirectCommandsLayoutNV
    ));
pub(super) const VK_DESTROY_INDIRECT_COMMANDS_LAYOUT_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkDestroyIndirectCommandsLayoutNV
    ));
pub(super) const VK_CMD_SET_DEPTH_BIAS2EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetDepthBias2EXT),
);
pub(super) const VK_CREATE_PRIVATE_DATA_SLOT_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreatePrivateDataSlotEXT),
);
pub(super) const VK_DESTROY_PRIVATE_DATA_SLOT_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyPrivateDataSlotEXT),
);
pub(super) const VK_SET_PRIVATE_DATA_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkSetPrivateDataEXT),
);
pub(super) const VK_GET_PRIVATE_DATA_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetPrivateDataEXT),
);
pub(super) const VK_QUEUE_SET_PERF_HINT_QCOM_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkQueueSetPerfHintQCOM),
);
#[cfg(feature = "beta-extensions")]
pub(super) const VK_CREATE_CUDA_MODULE_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateCudaModuleNV),
);
#[cfg(not(feature = "beta-extensions"))]
pub(super) const VK_CREATE_CUDA_MODULE_NV_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
#[cfg(feature = "beta-extensions")]
pub(super) const VK_GET_CUDA_MODULE_CACHE_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetCudaModuleCacheNV),
);
#[cfg(not(feature = "beta-extensions"))]
pub(super) const VK_GET_CUDA_MODULE_CACHE_NV_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
#[cfg(feature = "beta-extensions")]
pub(super) const VK_CREATE_CUDA_FUNCTION_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateCudaFunctionNV),
);
#[cfg(not(feature = "beta-extensions"))]
pub(super) const VK_CREATE_CUDA_FUNCTION_NV_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
#[cfg(feature = "beta-extensions")]
pub(super) const VK_DESTROY_CUDA_MODULE_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyCudaModuleNV),
);
#[cfg(not(feature = "beta-extensions"))]
pub(super) const VK_DESTROY_CUDA_MODULE_NV_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
#[cfg(feature = "beta-extensions")]
pub(super) const VK_DESTROY_CUDA_FUNCTION_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyCudaFunctionNV),
);
#[cfg(not(feature = "beta-extensions"))]
pub(super) const VK_DESTROY_CUDA_FUNCTION_NV_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
#[cfg(feature = "beta-extensions")]
pub(super) const VK_CMD_CUDA_LAUNCH_KERNEL_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdCudaLaunchKernelNV),
);
#[cfg(not(feature = "beta-extensions"))]
pub(super) const VK_CMD_CUDA_LAUNCH_KERNEL_NV_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
pub(super) const VK_CMD_DISPATCH_TILE_QCOM_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDispatchTileQCOM),
);
pub(super) const VK_CMD_BEGIN_PER_TILE_EXECUTION_QCOM_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBeginPerTileExecutionQCOM),
);
pub(super) const VK_CMD_END_PER_TILE_EXECUTION_QCOM_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdEndPerTileExecutionQCOM),
);
pub(super) const VK_SET_LATENCY_SLEEP_MODE_LEGACY_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkSetLatencySleepModeLegacyNV),
);
pub(super) const VK_LATENCY_SLEEP_LEGACY_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkLatencySleepLegacyNV),
);
pub(super) const VK_SET_LATENCY_MARKER_LEGACY_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkSetLatencyMarkerLegacyNV),
);
pub(super) const VK_GET_LATENCY_TIMINGS_LEGACY_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetLatencyTimingsLegacyNV),
);
pub(super) const VK_QUEUE_NOTIFY_OUT_OF_BAND_LEGACY_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkQueueNotifyOutOfBandLegacyNV
    ));
pub(super) const VK_GET_SLEEP_STATUS_LEGACY_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetSleepStatusLegacyNV),
);
pub(super) const VK_SHUTDOWN_LATENCY_DEVICE_LEGACY_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkShutdownLatencyDeviceLegacyNV),
);
#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "visionos"
))]
pub(super) const VK_EXPORT_METAL_OBJECTS_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkExportMetalObjectsEXT),
);
#[cfg(not(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "visionos"
)))]
pub(super) const VK_EXPORT_METAL_OBJECTS_EXT_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
pub(super) const VK_GET_DESCRIPTOR_SET_LAYOUT_SIZE_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetDescriptorSetLayoutSizeEXT
    ));
pub(super) const VK_GET_DESCRIPTOR_SET_LAYOUT_BINDING_OFFSET_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetDescriptorSetLayoutBindingOffsetEXT
    ));
pub(super) const VK_GET_DESCRIPTOR_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetDescriptorEXT),
);
pub(super) const VK_CMD_BIND_DESCRIPTOR_BUFFERS_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBindDescriptorBuffersEXT),
);
pub(super) const VK_CMD_SET_DESCRIPTOR_BUFFER_OFFSETS_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetDescriptorBufferOffsetsEXT
    ));
pub(super) const VK_CMD_BIND_DESCRIPTOR_BUFFER_EMBEDDED_SAMPLERS_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdBindDescriptorBufferEmbeddedSamplersEXT
    ));
pub(super) const VK_GET_BUFFER_OPAQUE_CAPTURE_DESCRIPTOR_DATA_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetBufferOpaqueCaptureDescriptorDataEXT
    ));
pub(super) const VK_GET_IMAGE_OPAQUE_CAPTURE_DESCRIPTOR_DATA_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetImageOpaqueCaptureDescriptorDataEXT
    ));
pub(super) const VK_GET_IMAGE_VIEW_OPAQUE_CAPTURE_DESCRIPTOR_DATA_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetImageViewOpaqueCaptureDescriptorDataEXT
    ));
pub(super) const VK_GET_SAMPLER_OPAQUE_CAPTURE_DESCRIPTOR_DATA_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetSamplerOpaqueCaptureDescriptorDataEXT
    ));
pub(super) const VK_GET_ACCELERATION_STRUCTURE_OPAQUE_CAPTURE_DESCRIPTOR_DATA_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT
    ),
);
pub(super) const VK_CMD_SET_FRAGMENT_SHADING_RATE_ENUM_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetFragmentShadingRateEnumNV
    ));
pub(super) const VK_GET_DEVICE_FAULT_INFO_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetDeviceFaultInfoEXT),
);
pub(super) const VK_CMD_SET_VERTEX_INPUT_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetVertexInputEXT),
);
#[cfg(target_os = "fuchsia")]
pub(super) const VK_GET_MEMORY_ZIRCON_HANDLE_FUCHSIA_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetMemoryZirconHandleFUCHSIA),
);
#[cfg(not(target_os = "fuchsia"))]
pub(super) const VK_GET_MEMORY_ZIRCON_HANDLE_FUCHSIA_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
#[cfg(target_os = "fuchsia")]
pub(super) const VK_GET_MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetMemoryZirconHandlePropertiesFUCHSIA
    ));
#[cfg(not(target_os = "fuchsia"))]
pub(super) const VK_GET_MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA_DEVICE_DISPATCH_OFFSET: u16 =
    u16::MAX;
#[cfg(target_os = "fuchsia")]
pub(super) const VK_IMPORT_SEMAPHORE_ZIRCON_HANDLE_FUCHSIA_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkImportSemaphoreZirconHandleFUCHSIA
    ));
#[cfg(not(target_os = "fuchsia"))]
pub(super) const VK_IMPORT_SEMAPHORE_ZIRCON_HANDLE_FUCHSIA_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
#[cfg(target_os = "fuchsia")]
pub(super) const VK_GET_SEMAPHORE_ZIRCON_HANDLE_FUCHSIA_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetSemaphoreZirconHandleFUCHSIA
    ));
#[cfg(not(target_os = "fuchsia"))]
pub(super) const VK_GET_SEMAPHORE_ZIRCON_HANDLE_FUCHSIA_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
#[cfg(target_os = "fuchsia")]
pub(super) const VK_CREATE_BUFFER_COLLECTION_FUCHSIA_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateBufferCollectionFUCHSIA),
);
#[cfg(not(target_os = "fuchsia"))]
pub(super) const VK_CREATE_BUFFER_COLLECTION_FUCHSIA_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
#[cfg(target_os = "fuchsia")]
pub(super) const VK_SET_BUFFER_COLLECTION_IMAGE_CONSTRAINTS_FUCHSIA_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkSetBufferCollectionImageConstraintsFUCHSIA
    ));
#[cfg(not(target_os = "fuchsia"))]
pub(super) const VK_SET_BUFFER_COLLECTION_IMAGE_CONSTRAINTS_FUCHSIA_DEVICE_DISPATCH_OFFSET: u16 =
    u16::MAX;
#[cfg(target_os = "fuchsia")]
pub(super) const VK_SET_BUFFER_COLLECTION_BUFFER_CONSTRAINTS_FUCHSIA_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkSetBufferCollectionBufferConstraintsFUCHSIA
    ));
#[cfg(not(target_os = "fuchsia"))]
pub(super) const VK_SET_BUFFER_COLLECTION_BUFFER_CONSTRAINTS_FUCHSIA_DEVICE_DISPATCH_OFFSET: u16 =
    u16::MAX;
#[cfg(target_os = "fuchsia")]
pub(super) const VK_DESTROY_BUFFER_COLLECTION_FUCHSIA_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyBufferCollectionFUCHSIA),
);
#[cfg(not(target_os = "fuchsia"))]
pub(super) const VK_DESTROY_BUFFER_COLLECTION_FUCHSIA_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
#[cfg(target_os = "fuchsia")]
pub(super) const VK_GET_BUFFER_COLLECTION_PROPERTIES_FUCHSIA_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetBufferCollectionPropertiesFUCHSIA
    ));
#[cfg(not(target_os = "fuchsia"))]
pub(super) const VK_GET_BUFFER_COLLECTION_PROPERTIES_FUCHSIA_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
pub(super) const VK_GET_DEVICE_SUBPASS_SHADING_MAX_WORKGROUP_SIZE_HUAWEI_DEVICE_DISPATCH_OFFSET:
    u16 = dispatch_offset(core::mem::offset_of!(
    LayerDeviceDispatchTable,
    vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI
));
pub(super) const VK_CMD_SUBPASS_SHADING_HUAWEI_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSubpassShadingHUAWEI),
);
pub(super) const VK_CMD_BIND_INVOCATION_MASK_HUAWEI_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBindInvocationMaskHUAWEI),
);
pub(super) const VK_GET_MEMORY_REMOTE_ADDRESS_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetMemoryRemoteAddressNV),
);
pub(super) const VK_GET_PIPELINE_PROPERTIES_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetPipelinePropertiesEXT),
);
pub(super) const VK_CMD_SET_PATCH_CONTROL_POINTS_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetPatchControlPointsEXT),
);
pub(super) const VK_CMD_SET_RASTERIZER_DISCARD_ENABLE_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetRasterizerDiscardEnableEXT
    ));
pub(super) const VK_CMD_SET_DEPTH_BIAS_ENABLE_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetDepthBiasEnableEXT),
);
pub(super) const VK_CMD_SET_LOGIC_OP_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetLogicOpEXT),
);
pub(super) const VK_CMD_SET_PRIMITIVE_RESTART_ENABLE_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetPrimitiveRestartEnableEXT
    ));
pub(super) const VK_CMD_SET_COLOR_WRITE_ENABLE_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetColorWriteEnableEXT),
);
pub(super) const VK_CMD_DRAW_MULTI_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDrawMultiEXT),
);
pub(super) const VK_CMD_DRAW_MULTI_INDEXED_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDrawMultiIndexedEXT),
);
pub(super) const VK_CREATE_MICROMAP_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateMicromapEXT),
);
pub(super) const VK_DESTROY_MICROMAP_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyMicromapEXT),
);
pub(super) const VK_CMD_BUILD_MICROMAPS_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBuildMicromapsEXT),
);
pub(super) const VK_BUILD_MICROMAPS_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkBuildMicromapsEXT),
);
pub(super) const VK_COPY_MICROMAP_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCopyMicromapEXT),
);
pub(super) const VK_COPY_MICROMAP_TO_MEMORY_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCopyMicromapToMemoryEXT),
);
pub(super) const VK_COPY_MEMORY_TO_MICROMAP_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCopyMemoryToMicromapEXT),
);
pub(super) const VK_WRITE_MICROMAPS_PROPERTIES_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkWriteMicromapsPropertiesEXT),
);
pub(super) const VK_CMD_COPY_MICROMAP_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdCopyMicromapEXT),
);
pub(super) const VK_CMD_COPY_MICROMAP_TO_MEMORY_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdCopyMicromapToMemoryEXT),
);
pub(super) const VK_CMD_COPY_MEMORY_TO_MICROMAP_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdCopyMemoryToMicromapEXT),
);
pub(super) const VK_CMD_WRITE_MICROMAPS_PROPERTIES_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdWriteMicromapsPropertiesEXT
    ));
pub(super) const VK_GET_DEVICE_MICROMAP_COMPATIBILITY_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetDeviceMicromapCompatibilityEXT
    ));
pub(super) const VK_GET_MICROMAP_BUILD_SIZES_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetMicromapBuildSizesEXT),
);
pub(super) const VK_CMD_DRAW_CLUSTER_HUAWEI_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDrawClusterHUAWEI),
);
pub(super) const VK_CMD_DRAW_CLUSTER_INDIRECT_HUAWEI_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDrawClusterIndirectHUAWEI),
);
pub(super) const VK_SET_DEVICE_MEMORY_PRIORITY_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkSetDeviceMemoryPriorityEXT),
);
pub(super) const VK_CMD_SET_DISPATCH_PARAMETERS_ARM_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetDispatchParametersARM),
);
pub(super) const VK_GET_DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetDescriptorSetLayoutHostMappingInfoVALVE
    ));
pub(super) const VK_GET_DESCRIPTOR_SET_HOST_MAPPING_VALVE_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetDescriptorSetHostMappingVALVE
    ));
pub(super) const VK_CMD_COPY_MEMORY_INDIRECT_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdCopyMemoryIndirectNV),
);
pub(super) const VK_CMD_COPY_MEMORY_TO_IMAGE_INDIRECT_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdCopyMemoryToImageIndirectNV
    ));
pub(super) const VK_CMD_DECOMPRESS_MEMORY_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDecompressMemoryNV),
);
pub(super) const VK_CMD_DECOMPRESS_MEMORY_INDIRECT_COUNT_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdDecompressMemoryIndirectCountNV
    ));
pub(super) const VK_GET_PIPELINE_INDIRECT_MEMORY_REQUIREMENTS_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetPipelineIndirectMemoryRequirementsNV
    ));
pub(super) const VK_CMD_UPDATE_PIPELINE_INDIRECT_BUFFER_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdUpdatePipelineIndirectBufferNV
    ));
pub(super) const VK_GET_PIPELINE_INDIRECT_DEVICE_ADDRESS_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetPipelineIndirectDeviceAddressNV
    ));
#[cfg(target_env = "ohos")]
pub(super) const VK_GET_NATIVE_BUFFER_PROPERTIES_OHOS_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetNativeBufferPropertiesOHOS),
);
#[cfg(not(target_env = "ohos"))]
pub(super) const VK_GET_NATIVE_BUFFER_PROPERTIES_OHOS_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
#[cfg(target_env = "ohos")]
pub(super) const VK_GET_MEMORY_NATIVE_BUFFER_OHOS_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetMemoryNativeBufferOHOS),
);
#[cfg(not(target_env = "ohos"))]
pub(super) const VK_GET_MEMORY_NATIVE_BUFFER_OHOS_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
pub(super) const VK_CMD_SET_DEPTH_CLAMP_ENABLE_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetDepthClampEnableEXT),
);
pub(super) const VK_CMD_SET_POLYGON_MODE_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetPolygonModeEXT),
);
pub(super) const VK_CMD_SET_RASTERIZATION_SAMPLES_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetRasterizationSamplesEXT),
);
pub(super) const VK_CMD_SET_SAMPLE_MASK_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetSampleMaskEXT),
);
pub(super) const VK_CMD_SET_ALPHA_TO_COVERAGE_ENABLE_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetAlphaToCoverageEnableEXT
    ));
pub(super) const VK_CMD_SET_ALPHA_TO_ONE_ENABLE_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetAlphaToOneEnableEXT),
);
pub(super) const VK_CMD_SET_LOGIC_OP_ENABLE_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetLogicOpEnableEXT),
);
pub(super) const VK_CMD_SET_COLOR_BLEND_ENABLE_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetColorBlendEnableEXT),
);
pub(super) const VK_CMD_SET_COLOR_BLEND_EQUATION_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetColorBlendEquationEXT),
);
pub(super) const VK_CMD_SET_COLOR_WRITE_MASK_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetColorWriteMaskEXT),
);
pub(super) const VK_CMD_SET_TESSELLATION_DOMAIN_ORIGIN_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetTessellationDomainOriginEXT
    ));
pub(super) const VK_CMD_SET_RASTERIZATION_STREAM_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetRasterizationStreamEXT),
);
pub(super) const VK_CMD_SET_CONSERVATIVE_RASTERIZATION_MODE_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetConservativeRasterizationModeEXT
    ));
pub(super) const VK_CMD_SET_EXTRA_PRIMITIVE_OVERESTIMATION_SIZE_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetExtraPrimitiveOverestimationSizeEXT
    ));
pub(super) const VK_CMD_SET_DEPTH_CLIP_ENABLE_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetDepthClipEnableEXT),
);
pub(super) const VK_CMD_SET_SAMPLE_LOCATIONS_ENABLE_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetSampleLocationsEnableEXT
    ));
pub(super) const VK_CMD_SET_COLOR_BLEND_ADVANCED_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetColorBlendAdvancedEXT),
);
pub(super) const VK_CMD_SET_PROVOKING_VERTEX_MODE_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetProvokingVertexModeEXT),
);
pub(super) const VK_CMD_SET_LINE_RASTERIZATION_MODE_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetLineRasterizationModeEXT
    ));
pub(super) const VK_CMD_SET_LINE_STIPPLE_ENABLE_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetLineStippleEnableEXT),
);
pub(super) const VK_CMD_SET_DEPTH_CLIP_NEGATIVE_ONE_TO_ONE_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetDepthClipNegativeOneToOneEXT
    ));
pub(super) const VK_CMD_SET_VIEWPORT_W_SCALING_ENABLE_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetViewportWScalingEnableNV
    ));
pub(super) const VK_CMD_SET_VIEWPORT_SWIZZLE_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetViewportSwizzleNV),
);
pub(super) const VK_CMD_SET_COVERAGE_TO_COLOR_ENABLE_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetCoverageToColorEnableNV
    ));
pub(super) const VK_CMD_SET_COVERAGE_TO_COLOR_LOCATION_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetCoverageToColorLocationNV
    ));
pub(super) const VK_CMD_SET_COVERAGE_MODULATION_MODE_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetCoverageModulationModeNV
    ));
pub(super) const VK_CMD_SET_COVERAGE_MODULATION_TABLE_ENABLE_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetCoverageModulationTableEnableNV
    ));
pub(super) const VK_CMD_SET_COVERAGE_MODULATION_TABLE_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetCoverageModulationTableNV
    ));
pub(super) const VK_CMD_SET_SHADING_RATE_IMAGE_ENABLE_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetShadingRateImageEnableNV
    ));
pub(super) const VK_CMD_SET_REPRESENTATIVE_FRAGMENT_TEST_ENABLE_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetRepresentativeFragmentTestEnableNV
    ));
pub(super) const VK_CMD_SET_COVERAGE_REDUCTION_MODE_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetCoverageReductionModeNV
    ));
pub(super) const VK_CREATE_TENSOR_ARM_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateTensorARM),
);
pub(super) const VK_DESTROY_TENSOR_ARM_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyTensorARM),
);
pub(super) const VK_CREATE_TENSOR_VIEW_ARM_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateTensorViewARM),
);
pub(super) const VK_DESTROY_TENSOR_VIEW_ARM_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyTensorViewARM),
);
pub(super) const VK_GET_TENSOR_MEMORY_REQUIREMENTS_ARM_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetTensorMemoryRequirementsARM
    ));
pub(super) const VK_BIND_TENSOR_MEMORY_ARM_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkBindTensorMemoryARM),
);
pub(super) const VK_GET_DEVICE_TENSOR_MEMORY_REQUIREMENTS_ARM_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetDeviceTensorMemoryRequirementsARM
    ));
pub(super) const VK_CMD_COPY_TENSOR_ARM_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdCopyTensorARM),
);
pub(super) const VK_GET_TENSOR_OPAQUE_CAPTURE_DESCRIPTOR_DATA_ARM_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetTensorOpaqueCaptureDescriptorDataARM
    ));
pub(super) const VK_GET_TENSOR_VIEW_OPAQUE_CAPTURE_DESCRIPTOR_DATA_ARM_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetTensorViewOpaqueCaptureDescriptorDataARM
    ));
pub(super) const VK_GET_SHADER_MODULE_IDENTIFIER_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetShaderModuleIdentifierEXT),
);
pub(super) const VK_GET_SHADER_MODULE_CREATE_INFO_IDENTIFIER_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetShaderModuleCreateInfoIdentifierEXT
    ));
pub(super) const VK_CREATE_OPTICAL_FLOW_SESSION_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateOpticalFlowSessionNV),
);
pub(super) const VK_DESTROY_OPTICAL_FLOW_SESSION_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyOpticalFlowSessionNV),
);
pub(super) const VK_BIND_OPTICAL_FLOW_SESSION_IMAGE_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkBindOpticalFlowSessionImageNV
    ));
pub(super) const VK_CMD_OPTICAL_FLOW_EXECUTE_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdOpticalFlowExecuteNV),
);
pub(super) const VK_ANTI_LAG_UPDATE_AMD_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkAntiLagUpdateAMD),
);
pub(super) const VK_CREATE_SHADERS_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateShadersEXT),
);
pub(super) const VK_DESTROY_SHADER_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyShaderEXT),
);
pub(super) const VK_GET_SHADER_BINARY_DATA_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetShaderBinaryDataEXT),
);
pub(super) const VK_CMD_BIND_SHADERS_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBindShadersEXT),
);
pub(super) const VK_CMD_SET_DEPTH_CLAMP_RANGE_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdSetDepthClampRangeEXT),
);
pub(super) const VK_GET_FRAMEBUFFER_TILE_PROPERTIES_QCOM_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetFramebufferTilePropertiesQCOM
    ));
pub(super) const VK_GET_DYNAMIC_RENDERING_TILE_PROPERTIES_QCOM_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetDynamicRenderingTilePropertiesQCOM
    ));
pub(super) const VK_CONVERT_COOPERATIVE_VECTOR_MATRIX_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkConvertCooperativeVectorMatrixNV
    ));
pub(super) const VK_CMD_CONVERT_COOPERATIVE_VECTOR_MATRIX_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdConvertCooperativeVectorMatrixNV
    ));
pub(super) const VK_SET_LATENCY_SLEEP_MODE_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkSetLatencySleepModeNV),
);
pub(super) const VK_LATENCY_SLEEP_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkLatencySleepNV),
);
pub(super) const VK_SET_LATENCY_MARKER_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkSetLatencyMarkerNV),
);
pub(super) const VK_GET_LATENCY_TIMINGS_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetLatencyTimingsNV),
);
pub(super) const VK_QUEUE_NOTIFY_OUT_OF_BAND_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkQueueNotifyOutOfBandNV),
);
pub(super) const VK_CREATE_DATA_GRAPH_PIPELINES_ARM_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateDataGraphPipelinesARM),
);
pub(super) const VK_CREATE_DATA_GRAPH_PIPELINE_SESSION_ARM_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCreateDataGraphPipelineSessionARM
    ));
pub(super) const VK_GET_DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENTS_ARM_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(
        LayerDeviceDispatchTable, vkGetDataGraphPipelineSessionBindPointRequirementsARM
    ),
);
pub(super) const VK_GET_DATA_GRAPH_PIPELINE_SESSION_MEMORY_REQUIREMENTS_ARM_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(
        LayerDeviceDispatchTable, vkGetDataGraphPipelineSessionMemoryRequirementsARM
    ),
);
pub(super) const VK_BIND_DATA_GRAPH_PIPELINE_SESSION_MEMORY_ARM_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkBindDataGraphPipelineSessionMemoryARM
    ));
pub(super) const VK_DESTROY_DATA_GRAPH_PIPELINE_SESSION_ARM_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkDestroyDataGraphPipelineSessionARM
    ));
pub(super) const VK_CMD_DISPATCH_DATA_GRAPH_ARM_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDispatchDataGraphARM),
);
pub(super) const VK_GET_DATA_GRAPH_PIPELINE_AVAILABLE_PROPERTIES_ARM_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetDataGraphPipelineAvailablePropertiesARM
    ));
pub(super) const VK_GET_DATA_GRAPH_PIPELINE_PROPERTIES_ARM_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetDataGraphPipelinePropertiesARM
    ));
pub(super) const VK_CMD_SET_ATTACHMENT_FEEDBACK_LOOP_ENABLE_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetAttachmentFeedbackLoopEnableEXT
    ));
#[cfg(any(target_os = "nto", target_os = "qnx"))]
pub(super) const VK_GET_SCREEN_BUFFER_PROPERTIES_QNX_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetScreenBufferPropertiesQNX),
);
#[cfg(not(any(target_os = "nto", target_os = "qnx")))]
pub(super) const VK_GET_SCREEN_BUFFER_PROPERTIES_QNX_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
pub(super) const VK_CMD_BIND_TILE_MEMORY_QCOM_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBindTileMemoryQCOM),
);
pub(super) const VK_CMD_DECOMPRESS_MEMORY_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDecompressMemoryEXT),
);
pub(super) const VK_CMD_DECOMPRESS_MEMORY_INDIRECT_COUNT_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdDecompressMemoryIndirectCountEXT
    ));
pub(super) const VK_CREATE_EXTERNAL_COMPUTE_QUEUE_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateExternalComputeQueueNV),
);
pub(super) const VK_DESTROY_EXTERNAL_COMPUTE_QUEUE_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkDestroyExternalComputeQueueNV),
);
pub(super) const VK_GET_EXTERNAL_COMPUTE_QUEUE_DATA_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetExternalComputeQueueDataNV
    ));
pub(super) const VK_GET_CLUSTER_ACCELERATION_STRUCTURE_BUILD_SIZES_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetClusterAccelerationStructureBuildSizesNV
    ));
pub(super) const VK_CMD_BUILD_CLUSTER_ACCELERATION_STRUCTURE_INDIRECT_NV_DEVICE_DISPATCH_OFFSET:
    u16 = dispatch_offset(core::mem::offset_of!(
    LayerDeviceDispatchTable,
    vkCmdBuildClusterAccelerationStructureIndirectNV
));
pub(super) const VK_GET_PARTITIONED_ACCELERATION_STRUCTURES_BUILD_SIZES_NV_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(
        LayerDeviceDispatchTable, vkGetPartitionedAccelerationStructuresBuildSizesNV
    ),
);
pub(super) const VK_CMD_BUILD_PARTITIONED_ACCELERATION_STRUCTURES_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdBuildPartitionedAccelerationStructuresNV
    ));
pub(super) const VK_GET_GENERATED_COMMANDS_MEMORY_REQUIREMENTS_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetGeneratedCommandsMemoryRequirementsEXT
    ));
pub(super) const VK_CMD_PREPROCESS_GENERATED_COMMANDS_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdPreprocessGeneratedCommandsEXT
    ));
pub(super) const VK_CMD_EXECUTE_GENERATED_COMMANDS_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdExecuteGeneratedCommandsEXT
    ));
pub(super) const VK_CREATE_INDIRECT_COMMANDS_LAYOUT_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCreateIndirectCommandsLayoutEXT
    ));
pub(super) const VK_DESTROY_INDIRECT_COMMANDS_LAYOUT_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkDestroyIndirectCommandsLayoutEXT
    ));
pub(super) const VK_CREATE_INDIRECT_EXECUTION_SET_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateIndirectExecutionSetEXT),
);
pub(super) const VK_DESTROY_INDIRECT_EXECUTION_SET_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkDestroyIndirectExecutionSetEXT
    ));
pub(super) const VK_UPDATE_INDIRECT_EXECUTION_SET_PIPELINE_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkUpdateIndirectExecutionSetPipelineEXT
    ));
pub(super) const VK_UPDATE_INDIRECT_EXECUTION_SET_SHADER_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkUpdateIndirectExecutionSetShaderEXT
    ));
#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "visionos"
))]
pub(super) const VK_GET_MEMORY_METAL_HANDLE_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkGetMemoryMetalHandleEXT),
);
#[cfg(not(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "visionos"
)))]
pub(super) const VK_GET_MEMORY_METAL_HANDLE_EXT_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "visionos"
))]
pub(super) const VK_GET_MEMORY_METAL_HANDLE_PROPERTIES_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetMemoryMetalHandlePropertiesEXT
    ));
#[cfg(not(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "visionos"
)))]
pub(super) const VK_GET_MEMORY_METAL_HANDLE_PROPERTIES_EXT_DEVICE_DISPATCH_OFFSET: u16 = u16::MAX;
pub(super) const VK_CREATE_SHADER_INSTRUMENTATION_ARM_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateShaderInstrumentationARM),
);
pub(super) const VK_DESTROY_SHADER_INSTRUMENTATION_ARM_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkDestroyShaderInstrumentationARM
    ));
pub(super) const VK_CMD_BEGIN_SHADER_INSTRUMENTATION_ARM_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdBeginShaderInstrumentationARM
    ));
pub(super) const VK_CMD_END_SHADER_INSTRUMENTATION_ARM_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdEndShaderInstrumentationARM
    ));
pub(super) const VK_GET_SHADER_INSTRUMENTATION_VALUES_ARM_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetShaderInstrumentationValuesARM
    ));
pub(super) const VK_CLEAR_SHADER_INSTRUMENTATION_METRICS_ARM_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkClearShaderInstrumentationMetricsARM
    ));
pub(super) const VK_CMD_END_RENDERING2EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdEndRendering2EXT),
);
pub(super) const VK_CMD_BEGIN_CUSTOM_RESOLVE_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdBeginCustomResolveEXT),
);
pub(super) const VK_CMD_SET_COMPUTE_OCCUPANCY_PRIORITY_NV_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetComputeOccupancyPriorityNV
    ));
pub(super) const VK_CMD_SET_PRIMITIVE_RESTART_INDEX_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetPrimitiveRestartIndexEXT
    ));
pub(super) const VK_CREATE_ACCELERATION_STRUCTURE_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateAccelerationStructureKHR),
);
pub(super) const VK_DESTROY_ACCELERATION_STRUCTURE_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkDestroyAccelerationStructureKHR
    ));
pub(super) const VK_CMD_BUILD_ACCELERATION_STRUCTURES_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdBuildAccelerationStructuresKHR
    ));
pub(super) const VK_CMD_BUILD_ACCELERATION_STRUCTURES_INDIRECT_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdBuildAccelerationStructuresIndirectKHR
    ));
pub(super) const VK_BUILD_ACCELERATION_STRUCTURES_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkBuildAccelerationStructuresKHR),
);
pub(super) const VK_COPY_ACCELERATION_STRUCTURE_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCopyAccelerationStructureKHR),
);
pub(super) const VK_COPY_ACCELERATION_STRUCTURE_TO_MEMORY_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCopyAccelerationStructureToMemoryKHR
    ));
pub(super) const VK_COPY_MEMORY_TO_ACCELERATION_STRUCTURE_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCopyMemoryToAccelerationStructureKHR
    ));
pub(super) const VK_WRITE_ACCELERATION_STRUCTURES_PROPERTIES_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkWriteAccelerationStructuresPropertiesKHR
    ));
pub(super) const VK_CMD_COPY_ACCELERATION_STRUCTURE_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdCopyAccelerationStructureKHR
    ));
pub(super) const VK_CMD_COPY_ACCELERATION_STRUCTURE_TO_MEMORY_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdCopyAccelerationStructureToMemoryKHR
    ));
pub(super) const VK_CMD_COPY_MEMORY_TO_ACCELERATION_STRUCTURE_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdCopyMemoryToAccelerationStructureKHR
    ));
pub(super) const VK_GET_ACCELERATION_STRUCTURE_DEVICE_ADDRESS_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetAccelerationStructureDeviceAddressKHR
    ));
pub(super) const VK_CMD_WRITE_ACCELERATION_STRUCTURES_PROPERTIES_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdWriteAccelerationStructuresPropertiesKHR
    ));
pub(super) const VK_GET_DEVICE_ACCELERATION_STRUCTURE_COMPATIBILITY_KHR_DEVICE_DISPATCH_OFFSET:
    u16 = dispatch_offset(core::mem::offset_of!(
    LayerDeviceDispatchTable,
    vkGetDeviceAccelerationStructureCompatibilityKHR
));
pub(super) const VK_GET_ACCELERATION_STRUCTURE_BUILD_SIZES_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetAccelerationStructureBuildSizesKHR
    ));
pub(super) const VK_CMD_TRACE_RAYS_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdTraceRaysKHR),
);
pub(super) const VK_CREATE_RAY_TRACING_PIPELINES_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCreateRayTracingPipelinesKHR),
);
pub(super) const VK_GET_RAY_TRACING_CAPTURE_REPLAY_SHADER_GROUP_HANDLES_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(
        LayerDeviceDispatchTable, vkGetRayTracingCaptureReplayShaderGroupHandlesKHR
    ),
);
pub(super) const VK_CMD_TRACE_RAYS_INDIRECT_KHR_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdTraceRaysIndirectKHR),
);
pub(super) const VK_GET_RAY_TRACING_SHADER_GROUP_STACK_SIZE_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkGetRayTracingShaderGroupStackSizeKHR
    ));
pub(super) const VK_CMD_SET_RAY_TRACING_PIPELINE_STACK_SIZE_KHR_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdSetRayTracingPipelineStackSizeKHR
    ));
pub(super) const VK_CMD_DRAW_MESH_TASKS_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDrawMeshTasksEXT),
);
pub(super) const VK_CMD_DRAW_MESH_TASKS_INDIRECT_EXT_DEVICE_DISPATCH_OFFSET: u16 = dispatch_offset(
    core::mem::offset_of!(LayerDeviceDispatchTable, vkCmdDrawMeshTasksIndirectEXT),
);
pub(super) const VK_CMD_DRAW_MESH_TASKS_INDIRECT_COUNT_EXT_DEVICE_DISPATCH_OFFSET: u16 =
    dispatch_offset(core::mem::offset_of!(
        LayerDeviceDispatchTable,
        vkCmdDrawMeshTasksIndirectCountEXT
    ));
#[allow(dead_code)]
pub(super) const HANDLE_INFOS: [HandleInfo; 63] = [
    HandleInfo {
        name: "VkAccelerationStructureKHR",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_ACCELERATION_STRUCTURE_KHR"),
        alias: None,
    },
    HandleInfo {
        name: "VkAccelerationStructureNV",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_ACCELERATION_STRUCTURE_NV"),
        alias: None,
    },
    HandleInfo {
        name: "VkBuffer",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_BUFFER"),
        alias: None,
    },
    HandleInfo {
        name: "VkBufferCollectionFUCHSIA",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_BUFFER_COLLECTION_FUCHSIA"),
        alias: None,
    },
    HandleInfo {
        name: "VkBufferView",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_BUFFER_VIEW"),
        alias: None,
    },
    HandleInfo {
        name: "VkCommandBuffer",
        dispatchable: true,
        parent: Some("VkCommandPool"),
        object_type: Some("VK_OBJECT_TYPE_COMMAND_BUFFER"),
        alias: None,
    },
    HandleInfo {
        name: "VkCommandPool",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_COMMAND_POOL"),
        alias: None,
    },
    HandleInfo {
        name: "VkCuFunctionNVX",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_CU_FUNCTION_NVX"),
        alias: None,
    },
    HandleInfo {
        name: "VkCuModuleNVX",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_CU_MODULE_NVX"),
        alias: None,
    },
    HandleInfo {
        name: "VkCudaFunctionNV",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_CUDA_FUNCTION_NV"),
        alias: None,
    },
    HandleInfo {
        name: "VkCudaModuleNV",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_CUDA_MODULE_NV"),
        alias: None,
    },
    HandleInfo {
        name: "VkDataGraphPipelineSessionARM",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_DATA_GRAPH_PIPELINE_SESSION_ARM"),
        alias: None,
    },
    HandleInfo {
        name: "VkDebugReportCallbackEXT",
        dispatchable: false,
        parent: Some("VkInstance"),
        object_type: Some("VK_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT"),
        alias: None,
    },
    HandleInfo {
        name: "VkDebugUtilsMessengerEXT",
        dispatchable: false,
        parent: Some("VkInstance"),
        object_type: Some("VK_OBJECT_TYPE_DEBUG_UTILS_MESSENGER_EXT"),
        alias: None,
    },
    HandleInfo {
        name: "VkDeferredOperationKHR",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_DEFERRED_OPERATION_KHR"),
        alias: None,
    },
    HandleInfo {
        name: "VkDescriptorPool",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_DESCRIPTOR_POOL"),
        alias: None,
    },
    HandleInfo {
        name: "VkDescriptorSet",
        dispatchable: false,
        parent: Some("VkDescriptorPool"),
        object_type: Some("VK_OBJECT_TYPE_DESCRIPTOR_SET"),
        alias: None,
    },
    HandleInfo {
        name: "VkDescriptorSetLayout",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT"),
        alias: None,
    },
    HandleInfo {
        name: "VkDescriptorUpdateTemplate",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE"),
        alias: None,
    },
    HandleInfo {
        name: "VkDescriptorUpdateTemplateKHR",
        dispatchable: true,
        parent: None,
        object_type: None,
        alias: Some("VkDescriptorUpdateTemplate"),
    },
    HandleInfo {
        name: "VkDevice",
        dispatchable: true,
        parent: Some("VkPhysicalDevice"),
        object_type: Some("VK_OBJECT_TYPE_DEVICE"),
        alias: None,
    },
    HandleInfo {
        name: "VkDeviceMemory",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_DEVICE_MEMORY"),
        alias: None,
    },
    HandleInfo {
        name: "VkDisplayKHR",
        dispatchable: false,
        parent: Some("VkPhysicalDevice"),
        object_type: Some("VK_OBJECT_TYPE_DISPLAY_KHR"),
        alias: None,
    },
    HandleInfo {
        name: "VkDisplayModeKHR",
        dispatchable: false,
        parent: Some("VkDisplayKHR"),
        object_type: Some("VK_OBJECT_TYPE_DISPLAY_MODE_KHR"),
        alias: None,
    },
    HandleInfo {
        name: "VkEvent",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_EVENT"),
        alias: None,
    },
    HandleInfo {
        name: "VkExternalComputeQueueNV",
        dispatchable: true,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_EXTERNAL_COMPUTE_QUEUE_NV"),
        alias: None,
    },
    HandleInfo {
        name: "VkFence",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_FENCE"),
        alias: None,
    },
    HandleInfo {
        name: "VkFramebuffer",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_FRAMEBUFFER"),
        alias: None,
    },
    HandleInfo {
        name: "VkGpaSessionAMD",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_GPA_SESSION_AMD"),
        alias: None,
    },
    HandleInfo {
        name: "VkImage",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_IMAGE"),
        alias: None,
    },
    HandleInfo {
        name: "VkImageView",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_IMAGE_VIEW"),
        alias: None,
    },
    HandleInfo {
        name: "VkIndirectCommandsLayoutEXT",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_INDIRECT_COMMANDS_LAYOUT_EXT"),
        alias: None,
    },
    HandleInfo {
        name: "VkIndirectCommandsLayoutNV",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_INDIRECT_COMMANDS_LAYOUT_NV"),
        alias: None,
    },
    HandleInfo {
        name: "VkIndirectExecutionSetEXT",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_INDIRECT_EXECUTION_SET_EXT"),
        alias: None,
    },
    HandleInfo {
        name: "VkInstance",
        dispatchable: true,
        parent: None,
        object_type: Some("VK_OBJECT_TYPE_INSTANCE"),
        alias: None,
    },
    HandleInfo {
        name: "VkMicromapEXT",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_MICROMAP_EXT"),
        alias: None,
    },
    HandleInfo {
        name: "VkOpticalFlowSessionNV",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_OPTICAL_FLOW_SESSION_NV"),
        alias: None,
    },
    HandleInfo {
        name: "VkPerformanceConfigurationINTEL",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_PERFORMANCE_CONFIGURATION_INTEL"),
        alias: None,
    },
    HandleInfo {
        name: "VkPhysicalDevice",
        dispatchable: true,
        parent: Some("VkInstance"),
        object_type: Some("VK_OBJECT_TYPE_PHYSICAL_DEVICE"),
        alias: None,
    },
    HandleInfo {
        name: "VkPipeline",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_PIPELINE"),
        alias: None,
    },
    HandleInfo {
        name: "VkPipelineBinaryKHR",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_PIPELINE_BINARY_KHR"),
        alias: None,
    },
    HandleInfo {
        name: "VkPipelineCache",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_PIPELINE_CACHE"),
        alias: None,
    },
    HandleInfo {
        name: "VkPipelineLayout",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_PIPELINE_LAYOUT"),
        alias: None,
    },
    HandleInfo {
        name: "VkPrivateDataSlot",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_PRIVATE_DATA_SLOT"),
        alias: None,
    },
    HandleInfo {
        name: "VkPrivateDataSlotEXT",
        dispatchable: true,
        parent: None,
        object_type: None,
        alias: Some("VkPrivateDataSlot"),
    },
    HandleInfo {
        name: "VkQueryPool",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_QUERY_POOL"),
        alias: None,
    },
    HandleInfo {
        name: "VkQueue",
        dispatchable: true,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_QUEUE"),
        alias: None,
    },
    HandleInfo {
        name: "VkRenderPass",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_RENDER_PASS"),
        alias: None,
    },
    HandleInfo {
        name: "VkSampler",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_SAMPLER"),
        alias: None,
    },
    HandleInfo {
        name: "VkSamplerYcbcrConversion",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION"),
        alias: None,
    },
    HandleInfo {
        name: "VkSamplerYcbcrConversionKHR",
        dispatchable: true,
        parent: None,
        object_type: None,
        alias: Some("VkSamplerYcbcrConversion"),
    },
    HandleInfo {
        name: "VkSemaphore",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_SEMAPHORE"),
        alias: None,
    },
    HandleInfo {
        name: "VkSemaphoreSciSyncPoolNV",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_SEMAPHORE_SCI_SYNC_POOL_NV"),
        alias: None,
    },
    HandleInfo {
        name: "VkShaderEXT",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_SHADER_EXT"),
        alias: None,
    },
    HandleInfo {
        name: "VkShaderInstrumentationARM",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_SHADER_INSTRUMENTATION_ARM"),
        alias: None,
    },
    HandleInfo {
        name: "VkShaderModule",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_SHADER_MODULE"),
        alias: None,
    },
    HandleInfo {
        name: "VkSurfaceKHR",
        dispatchable: false,
        parent: Some("VkInstance"),
        object_type: Some("VK_OBJECT_TYPE_SURFACE_KHR"),
        alias: None,
    },
    HandleInfo {
        name: "VkSwapchainKHR",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_SWAPCHAIN_KHR"),
        alias: None,
    },
    HandleInfo {
        name: "VkTensorARM",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_TENSOR_ARM"),
        alias: None,
    },
    HandleInfo {
        name: "VkTensorViewARM",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_TENSOR_VIEW_ARM"),
        alias: None,
    },
    HandleInfo {
        name: "VkValidationCacheEXT",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_VALIDATION_CACHE_EXT"),
        alias: None,
    },
    HandleInfo {
        name: "VkVideoSessionKHR",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_VIDEO_SESSION_KHR"),
        alias: None,
    },
    HandleInfo {
        name: "VkVideoSessionParametersKHR",
        dispatchable: false,
        parent: Some("VkDevice"),
        object_type: Some("VK_OBJECT_TYPE_VIDEO_SESSION_PARAMETERS_KHR"),
        alias: None,
    },
];
#[allow(dead_code)]
pub(super) const VK_ACQUIRE_FULL_SCREEN_EXCLUSIVE_MODE_EXT_COMMAND_ID: u16 = 1;
#[allow(dead_code)]
pub(super) const VK_ACQUIRE_NEXT_IMAGE2KHR_COMMAND_ID: u16 = 2;
#[allow(dead_code)]
pub(super) const VK_ACQUIRE_NEXT_IMAGE_KHR_COMMAND_ID: u16 = 3;
#[allow(dead_code)]
pub(super) const VK_ACQUIRE_PERFORMANCE_CONFIGURATION_INTEL_COMMAND_ID: u16 = 4;
#[allow(dead_code)]
pub(super) const VK_ACQUIRE_PROFILING_LOCK_KHR_COMMAND_ID: u16 = 5;
#[allow(dead_code)]
pub(super) const VK_ALLOCATE_COMMAND_BUFFERS_COMMAND_ID: u16 = 8;
#[allow(dead_code)]
pub(super) const VK_ALLOCATE_DESCRIPTOR_SETS_COMMAND_ID: u16 = 9;
#[allow(dead_code)]
pub(super) const VK_ALLOCATE_MEMORY_COMMAND_ID: u16 = 10;
#[allow(dead_code)]
pub(super) const VK_ANTI_LAG_UPDATE_AMD_COMMAND_ID: u16 = 11;
#[allow(dead_code)]
pub(super) const VK_BEGIN_COMMAND_BUFFER_COMMAND_ID: u16 = 12;
#[allow(dead_code)]
pub(super) const VK_BIND_ACCELERATION_STRUCTURE_MEMORY_NV_COMMAND_ID: u16 = 13;
#[allow(dead_code)]
pub(super) const VK_BIND_BUFFER_MEMORY_COMMAND_ID: u16 = 14;
#[allow(dead_code)]
pub(super) const VK_BIND_BUFFER_MEMORY2_COMMAND_ID: u16 = 15;
#[allow(dead_code)]
pub(super) const VK_BIND_BUFFER_MEMORY2KHR_COMMAND_ID: u16 = 16;
#[allow(dead_code)]
pub(super) const VK_BIND_DATA_GRAPH_PIPELINE_SESSION_MEMORY_ARM_COMMAND_ID: u16 = 17;
#[allow(dead_code)]
pub(super) const VK_BIND_IMAGE_MEMORY_COMMAND_ID: u16 = 18;
#[allow(dead_code)]
pub(super) const VK_BIND_IMAGE_MEMORY2_COMMAND_ID: u16 = 19;
#[allow(dead_code)]
pub(super) const VK_BIND_IMAGE_MEMORY2KHR_COMMAND_ID: u16 = 20;
#[allow(dead_code)]
pub(super) const VK_BIND_OPTICAL_FLOW_SESSION_IMAGE_NV_COMMAND_ID: u16 = 21;
#[allow(dead_code)]
pub(super) const VK_BIND_TENSOR_MEMORY_ARM_COMMAND_ID: u16 = 22;
#[allow(dead_code)]
pub(super) const VK_BIND_VIDEO_SESSION_MEMORY_KHR_COMMAND_ID: u16 = 23;
#[allow(dead_code)]
pub(super) const VK_BUILD_ACCELERATION_STRUCTURES_KHR_COMMAND_ID: u16 = 24;
#[allow(dead_code)]
pub(super) const VK_BUILD_MICROMAPS_EXT_COMMAND_ID: u16 = 25;
#[allow(dead_code)]
pub(super) const VK_CLEAR_SHADER_INSTRUMENTATION_METRICS_ARM_COMMAND_ID: u16 = 26;
#[allow(dead_code)]
pub(super) const VK_CMD_BEGIN_CONDITIONAL_RENDERING2EXT_COMMAND_ID: u16 = 27;
#[allow(dead_code)]
pub(super) const VK_CMD_BEGIN_CONDITIONAL_RENDERING_EXT_COMMAND_ID: u16 = 28;
#[allow(dead_code)]
pub(super) const VK_CMD_BEGIN_CUSTOM_RESOLVE_EXT_COMMAND_ID: u16 = 29;
#[allow(dead_code)]
pub(super) const VK_CMD_BEGIN_DEBUG_UTILS_LABEL_EXT_COMMAND_ID: u16 = 30;
#[allow(dead_code)]
pub(super) const VK_CMD_BEGIN_GPA_SAMPLE_AMD_COMMAND_ID: u16 = 31;
#[allow(dead_code)]
pub(super) const VK_CMD_BEGIN_GPA_SESSION_AMD_COMMAND_ID: u16 = 32;
#[allow(dead_code)]
pub(super) const VK_CMD_BEGIN_PER_TILE_EXECUTION_QCOM_COMMAND_ID: u16 = 33;
#[allow(dead_code)]
pub(super) const VK_CMD_BEGIN_QUERY_COMMAND_ID: u16 = 34;
#[allow(dead_code)]
pub(super) const VK_CMD_BEGIN_QUERY_INDEXED_EXT_COMMAND_ID: u16 = 35;
#[allow(dead_code)]
pub(super) const VK_CMD_BEGIN_RENDER_PASS_COMMAND_ID: u16 = 36;
#[allow(dead_code)]
pub(super) const VK_CMD_BEGIN_RENDER_PASS2_COMMAND_ID: u16 = 37;
#[allow(dead_code)]
pub(super) const VK_CMD_BEGIN_RENDER_PASS2KHR_COMMAND_ID: u16 = 38;
#[allow(dead_code)]
pub(super) const VK_CMD_BEGIN_RENDERING_COMMAND_ID: u16 = 39;
#[allow(dead_code)]
pub(super) const VK_CMD_BEGIN_RENDERING_KHR_COMMAND_ID: u16 = 40;
#[allow(dead_code)]
pub(super) const VK_CMD_BEGIN_SHADER_INSTRUMENTATION_ARM_COMMAND_ID: u16 = 41;
#[allow(dead_code)]
pub(super) const VK_CMD_BEGIN_TRANSFORM_FEEDBACK2EXT_COMMAND_ID: u16 = 42;
#[allow(dead_code)]
pub(super) const VK_CMD_BEGIN_TRANSFORM_FEEDBACK_EXT_COMMAND_ID: u16 = 43;
#[allow(dead_code)]
pub(super) const VK_CMD_BEGIN_VIDEO_CODING_KHR_COMMAND_ID: u16 = 44;
#[allow(dead_code)]
pub(super) const VK_CMD_BIND_DESCRIPTOR_BUFFER_EMBEDDED_SAMPLERS2EXT_COMMAND_ID: u16 = 45;
#[allow(dead_code)]
pub(super) const VK_CMD_BIND_DESCRIPTOR_BUFFER_EMBEDDED_SAMPLERS_EXT_COMMAND_ID: u16 = 46;
#[allow(dead_code)]
pub(super) const VK_CMD_BIND_DESCRIPTOR_BUFFERS_EXT_COMMAND_ID: u16 = 47;
#[allow(dead_code)]
pub(super) const VK_CMD_BIND_DESCRIPTOR_SETS_COMMAND_ID: u16 = 48;
#[allow(dead_code)]
pub(super) const VK_CMD_BIND_DESCRIPTOR_SETS2_COMMAND_ID: u16 = 49;
#[allow(dead_code)]
pub(super) const VK_CMD_BIND_DESCRIPTOR_SETS2KHR_COMMAND_ID: u16 = 50;
#[allow(dead_code)]
pub(super) const VK_CMD_BIND_INDEX_BUFFER_COMMAND_ID: u16 = 51;
#[allow(dead_code)]
pub(super) const VK_CMD_BIND_INDEX_BUFFER2_COMMAND_ID: u16 = 52;
#[allow(dead_code)]
pub(super) const VK_CMD_BIND_INDEX_BUFFER2KHR_COMMAND_ID: u16 = 53;
#[allow(dead_code)]
pub(super) const VK_CMD_BIND_INDEX_BUFFER3KHR_COMMAND_ID: u16 = 54;
#[allow(dead_code)]
pub(super) const VK_CMD_BIND_INVOCATION_MASK_HUAWEI_COMMAND_ID: u16 = 55;
#[allow(dead_code)]
pub(super) const VK_CMD_BIND_PIPELINE_COMMAND_ID: u16 = 56;
#[allow(dead_code)]
pub(super) const VK_CMD_BIND_PIPELINE_SHADER_GROUP_NV_COMMAND_ID: u16 = 57;
#[allow(dead_code)]
pub(super) const VK_CMD_BIND_RESOURCE_HEAP_EXT_COMMAND_ID: u16 = 58;
#[allow(dead_code)]
pub(super) const VK_CMD_BIND_SAMPLER_HEAP_EXT_COMMAND_ID: u16 = 59;
#[allow(dead_code)]
pub(super) const VK_CMD_BIND_SHADERS_EXT_COMMAND_ID: u16 = 60;
#[allow(dead_code)]
pub(super) const VK_CMD_BIND_SHADING_RATE_IMAGE_NV_COMMAND_ID: u16 = 61;
#[allow(dead_code)]
pub(super) const VK_CMD_BIND_TILE_MEMORY_QCOM_COMMAND_ID: u16 = 62;
#[allow(dead_code)]
pub(super) const VK_CMD_BIND_TRANSFORM_FEEDBACK_BUFFERS2EXT_COMMAND_ID: u16 = 63;
#[allow(dead_code)]
pub(super) const VK_CMD_BIND_TRANSFORM_FEEDBACK_BUFFERS_EXT_COMMAND_ID: u16 = 64;
#[allow(dead_code)]
pub(super) const VK_CMD_BIND_VERTEX_BUFFERS_COMMAND_ID: u16 = 65;
#[allow(dead_code)]
pub(super) const VK_CMD_BIND_VERTEX_BUFFERS2_COMMAND_ID: u16 = 66;
#[allow(dead_code)]
pub(super) const VK_CMD_BIND_VERTEX_BUFFERS2EXT_COMMAND_ID: u16 = 67;
#[allow(dead_code)]
pub(super) const VK_CMD_BIND_VERTEX_BUFFERS3KHR_COMMAND_ID: u16 = 68;
#[allow(dead_code)]
pub(super) const VK_CMD_BLIT_IMAGE_COMMAND_ID: u16 = 69;
#[allow(dead_code)]
pub(super) const VK_CMD_BLIT_IMAGE2_COMMAND_ID: u16 = 70;
#[allow(dead_code)]
pub(super) const VK_CMD_BLIT_IMAGE2KHR_COMMAND_ID: u16 = 71;
#[allow(dead_code)]
pub(super) const VK_CMD_BUILD_ACCELERATION_STRUCTURE_NV_COMMAND_ID: u16 = 72;
#[allow(dead_code)]
pub(super) const VK_CMD_BUILD_ACCELERATION_STRUCTURES_INDIRECT_KHR_COMMAND_ID: u16 = 73;
#[allow(dead_code)]
pub(super) const VK_CMD_BUILD_ACCELERATION_STRUCTURES_KHR_COMMAND_ID: u16 = 74;
#[allow(dead_code)]
pub(super) const VK_CMD_BUILD_CLUSTER_ACCELERATION_STRUCTURE_INDIRECT_NV_COMMAND_ID: u16 = 75;
#[allow(dead_code)]
pub(super) const VK_CMD_BUILD_MICROMAPS_EXT_COMMAND_ID: u16 = 76;
#[allow(dead_code)]
pub(super) const VK_CMD_BUILD_PARTITIONED_ACCELERATION_STRUCTURES_NV_COMMAND_ID: u16 = 77;
#[allow(dead_code)]
pub(super) const VK_CMD_CLEAR_ATTACHMENTS_COMMAND_ID: u16 = 78;
#[allow(dead_code)]
pub(super) const VK_CMD_CLEAR_COLOR_IMAGE_COMMAND_ID: u16 = 79;
#[allow(dead_code)]
pub(super) const VK_CMD_CLEAR_DEPTH_STENCIL_IMAGE_COMMAND_ID: u16 = 80;
#[allow(dead_code)]
pub(super) const VK_CMD_CONTROL_VIDEO_CODING_KHR_COMMAND_ID: u16 = 81;
#[allow(dead_code)]
pub(super) const VK_CMD_CONVERT_COOPERATIVE_VECTOR_MATRIX_NV_COMMAND_ID: u16 = 82;
#[allow(dead_code)]
pub(super) const VK_CMD_COPY_ACCELERATION_STRUCTURE_KHR_COMMAND_ID: u16 = 83;
#[allow(dead_code)]
pub(super) const VK_CMD_COPY_ACCELERATION_STRUCTURE_NV_COMMAND_ID: u16 = 84;
#[allow(dead_code)]
pub(super) const VK_CMD_COPY_ACCELERATION_STRUCTURE_TO_MEMORY_KHR_COMMAND_ID: u16 = 85;
#[allow(dead_code)]
pub(super) const VK_CMD_COPY_BUFFER_COMMAND_ID: u16 = 86;
#[allow(dead_code)]
pub(super) const VK_CMD_COPY_BUFFER2_COMMAND_ID: u16 = 87;
#[allow(dead_code)]
pub(super) const VK_CMD_COPY_BUFFER2KHR_COMMAND_ID: u16 = 88;
#[allow(dead_code)]
pub(super) const VK_CMD_COPY_BUFFER_TO_IMAGE_COMMAND_ID: u16 = 89;
#[allow(dead_code)]
pub(super) const VK_CMD_COPY_BUFFER_TO_IMAGE2_COMMAND_ID: u16 = 90;
#[allow(dead_code)]
pub(super) const VK_CMD_COPY_BUFFER_TO_IMAGE2KHR_COMMAND_ID: u16 = 91;
#[allow(dead_code)]
pub(super) const VK_CMD_COPY_GPA_SESSION_RESULTS_AMD_COMMAND_ID: u16 = 92;
#[allow(dead_code)]
pub(super) const VK_CMD_COPY_IMAGE_COMMAND_ID: u16 = 93;
#[allow(dead_code)]
pub(super) const VK_CMD_COPY_IMAGE2_COMMAND_ID: u16 = 94;
#[allow(dead_code)]
pub(super) const VK_CMD_COPY_IMAGE2KHR_COMMAND_ID: u16 = 95;
#[allow(dead_code)]
pub(super) const VK_CMD_COPY_IMAGE_TO_BUFFER_COMMAND_ID: u16 = 96;
#[allow(dead_code)]
pub(super) const VK_CMD_COPY_IMAGE_TO_BUFFER2_COMMAND_ID: u16 = 97;
#[allow(dead_code)]
pub(super) const VK_CMD_COPY_IMAGE_TO_BUFFER2KHR_COMMAND_ID: u16 = 98;
#[allow(dead_code)]
pub(super) const VK_CMD_COPY_IMAGE_TO_MEMORY_KHR_COMMAND_ID: u16 = 99;
#[allow(dead_code)]
pub(super) const VK_CMD_COPY_MEMORY_INDIRECT_KHR_COMMAND_ID: u16 = 100;
#[allow(dead_code)]
pub(super) const VK_CMD_COPY_MEMORY_INDIRECT_NV_COMMAND_ID: u16 = 101;
#[allow(dead_code)]
pub(super) const VK_CMD_COPY_MEMORY_KHR_COMMAND_ID: u16 = 102;
#[allow(dead_code)]
pub(super) const VK_CMD_COPY_MEMORY_TO_ACCELERATION_STRUCTURE_KHR_COMMAND_ID: u16 = 103;
#[allow(dead_code)]
pub(super) const VK_CMD_COPY_MEMORY_TO_IMAGE_INDIRECT_KHR_COMMAND_ID: u16 = 104;
#[allow(dead_code)]
pub(super) const VK_CMD_COPY_MEMORY_TO_IMAGE_INDIRECT_NV_COMMAND_ID: u16 = 105;
#[allow(dead_code)]
pub(super) const VK_CMD_COPY_MEMORY_TO_IMAGE_KHR_COMMAND_ID: u16 = 106;
#[allow(dead_code)]
pub(super) const VK_CMD_COPY_MEMORY_TO_MICROMAP_EXT_COMMAND_ID: u16 = 107;
#[allow(dead_code)]
pub(super) const VK_CMD_COPY_MICROMAP_EXT_COMMAND_ID: u16 = 108;
#[allow(dead_code)]
pub(super) const VK_CMD_COPY_MICROMAP_TO_MEMORY_EXT_COMMAND_ID: u16 = 109;
#[allow(dead_code)]
pub(super) const VK_CMD_COPY_QUERY_POOL_RESULTS_COMMAND_ID: u16 = 110;
#[allow(dead_code)]
pub(super) const VK_CMD_COPY_QUERY_POOL_RESULTS_TO_MEMORY_KHR_COMMAND_ID: u16 = 111;
#[allow(dead_code)]
pub(super) const VK_CMD_COPY_TENSOR_ARM_COMMAND_ID: u16 = 112;
#[allow(dead_code)]
pub(super) const VK_CMD_CU_LAUNCH_KERNEL_NVX_COMMAND_ID: u16 = 113;
#[allow(dead_code)]
pub(super) const VK_CMD_CUDA_LAUNCH_KERNEL_NV_COMMAND_ID: u16 = 114;
#[allow(dead_code)]
pub(super) const VK_CMD_DEBUG_MARKER_BEGIN_EXT_COMMAND_ID: u16 = 115;
#[allow(dead_code)]
pub(super) const VK_CMD_DEBUG_MARKER_END_EXT_COMMAND_ID: u16 = 116;
#[allow(dead_code)]
pub(super) const VK_CMD_DEBUG_MARKER_INSERT_EXT_COMMAND_ID: u16 = 117;
#[allow(dead_code)]
pub(super) const VK_CMD_DECODE_VIDEO_KHR_COMMAND_ID: u16 = 118;
#[allow(dead_code)]
pub(super) const VK_CMD_DECOMPRESS_MEMORY_EXT_COMMAND_ID: u16 = 119;
#[allow(dead_code)]
pub(super) const VK_CMD_DECOMPRESS_MEMORY_INDIRECT_COUNT_EXT_COMMAND_ID: u16 = 120;
#[allow(dead_code)]
pub(super) const VK_CMD_DECOMPRESS_MEMORY_INDIRECT_COUNT_NV_COMMAND_ID: u16 = 121;
#[allow(dead_code)]
pub(super) const VK_CMD_DECOMPRESS_MEMORY_NV_COMMAND_ID: u16 = 122;
#[allow(dead_code)]
pub(super) const VK_CMD_DISPATCH_COMMAND_ID: u16 = 123;
#[allow(dead_code)]
pub(super) const VK_CMD_DISPATCH_BASE_COMMAND_ID: u16 = 124;
#[allow(dead_code)]
pub(super) const VK_CMD_DISPATCH_BASE_KHR_COMMAND_ID: u16 = 125;
#[allow(dead_code)]
pub(super) const VK_CMD_DISPATCH_DATA_GRAPH_ARM_COMMAND_ID: u16 = 126;
#[allow(dead_code)]
pub(super) const VK_CMD_DISPATCH_GRAPH_AMDX_COMMAND_ID: u16 = 127;
#[allow(dead_code)]
pub(super) const VK_CMD_DISPATCH_GRAPH_INDIRECT_AMDX_COMMAND_ID: u16 = 128;
#[allow(dead_code)]
pub(super) const VK_CMD_DISPATCH_GRAPH_INDIRECT_COUNT_AMDX_COMMAND_ID: u16 = 129;
#[allow(dead_code)]
pub(super) const VK_CMD_DISPATCH_INDIRECT_COMMAND_ID: u16 = 130;
#[allow(dead_code)]
pub(super) const VK_CMD_DISPATCH_INDIRECT2KHR_COMMAND_ID: u16 = 131;
#[allow(dead_code)]
pub(super) const VK_CMD_DISPATCH_TILE_QCOM_COMMAND_ID: u16 = 132;
#[allow(dead_code)]
pub(super) const VK_CMD_DRAW_COMMAND_ID: u16 = 133;
#[allow(dead_code)]
pub(super) const VK_CMD_DRAW_CLUSTER_HUAWEI_COMMAND_ID: u16 = 134;
#[allow(dead_code)]
pub(super) const VK_CMD_DRAW_CLUSTER_INDIRECT_HUAWEI_COMMAND_ID: u16 = 135;
#[allow(dead_code)]
pub(super) const VK_CMD_DRAW_INDEXED_COMMAND_ID: u16 = 136;
#[allow(dead_code)]
pub(super) const VK_CMD_DRAW_INDEXED_INDIRECT_COMMAND_ID: u16 = 137;
#[allow(dead_code)]
pub(super) const VK_CMD_DRAW_INDEXED_INDIRECT2KHR_COMMAND_ID: u16 = 138;
#[allow(dead_code)]
pub(super) const VK_CMD_DRAW_INDEXED_INDIRECT_COUNT_COMMAND_ID: u16 = 139;
#[allow(dead_code)]
pub(super) const VK_CMD_DRAW_INDEXED_INDIRECT_COUNT2KHR_COMMAND_ID: u16 = 140;
#[allow(dead_code)]
pub(super) const VK_CMD_DRAW_INDEXED_INDIRECT_COUNT_AMD_COMMAND_ID: u16 = 141;
#[allow(dead_code)]
pub(super) const VK_CMD_DRAW_INDEXED_INDIRECT_COUNT_KHR_COMMAND_ID: u16 = 142;
#[allow(dead_code)]
pub(super) const VK_CMD_DRAW_INDIRECT_COMMAND_ID: u16 = 143;
#[allow(dead_code)]
pub(super) const VK_CMD_DRAW_INDIRECT2KHR_COMMAND_ID: u16 = 144;
#[allow(dead_code)]
pub(super) const VK_CMD_DRAW_INDIRECT_BYTE_COUNT2EXT_COMMAND_ID: u16 = 145;
#[allow(dead_code)]
pub(super) const VK_CMD_DRAW_INDIRECT_BYTE_COUNT_EXT_COMMAND_ID: u16 = 146;
#[allow(dead_code)]
pub(super) const VK_CMD_DRAW_INDIRECT_COUNT_COMMAND_ID: u16 = 147;
#[allow(dead_code)]
pub(super) const VK_CMD_DRAW_INDIRECT_COUNT2KHR_COMMAND_ID: u16 = 148;
#[allow(dead_code)]
pub(super) const VK_CMD_DRAW_INDIRECT_COUNT_AMD_COMMAND_ID: u16 = 149;
#[allow(dead_code)]
pub(super) const VK_CMD_DRAW_INDIRECT_COUNT_KHR_COMMAND_ID: u16 = 150;
#[allow(dead_code)]
pub(super) const VK_CMD_DRAW_MESH_TASKS_EXT_COMMAND_ID: u16 = 151;
#[allow(dead_code)]
pub(super) const VK_CMD_DRAW_MESH_TASKS_INDIRECT2EXT_COMMAND_ID: u16 = 152;
#[allow(dead_code)]
pub(super) const VK_CMD_DRAW_MESH_TASKS_INDIRECT_COUNT2EXT_COMMAND_ID: u16 = 153;
#[allow(dead_code)]
pub(super) const VK_CMD_DRAW_MESH_TASKS_INDIRECT_COUNT_EXT_COMMAND_ID: u16 = 154;
#[allow(dead_code)]
pub(super) const VK_CMD_DRAW_MESH_TASKS_INDIRECT_COUNT_NV_COMMAND_ID: u16 = 155;
#[allow(dead_code)]
pub(super) const VK_CMD_DRAW_MESH_TASKS_INDIRECT_EXT_COMMAND_ID: u16 = 156;
#[allow(dead_code)]
pub(super) const VK_CMD_DRAW_MESH_TASKS_INDIRECT_NV_COMMAND_ID: u16 = 157;
#[allow(dead_code)]
pub(super) const VK_CMD_DRAW_MESH_TASKS_NV_COMMAND_ID: u16 = 158;
#[allow(dead_code)]
pub(super) const VK_CMD_DRAW_MULTI_EXT_COMMAND_ID: u16 = 159;
#[allow(dead_code)]
pub(super) const VK_CMD_DRAW_MULTI_INDEXED_EXT_COMMAND_ID: u16 = 160;
#[allow(dead_code)]
pub(super) const VK_CMD_ENCODE_VIDEO_KHR_COMMAND_ID: u16 = 161;
#[allow(dead_code)]
pub(super) const VK_CMD_END_CONDITIONAL_RENDERING_EXT_COMMAND_ID: u16 = 162;
#[allow(dead_code)]
pub(super) const VK_CMD_END_DEBUG_UTILS_LABEL_EXT_COMMAND_ID: u16 = 163;
#[allow(dead_code)]
pub(super) const VK_CMD_END_GPA_SAMPLE_AMD_COMMAND_ID: u16 = 164;
#[allow(dead_code)]
pub(super) const VK_CMD_END_GPA_SESSION_AMD_COMMAND_ID: u16 = 165;
#[allow(dead_code)]
pub(super) const VK_CMD_END_PER_TILE_EXECUTION_QCOM_COMMAND_ID: u16 = 166;
#[allow(dead_code)]
pub(super) const VK_CMD_END_QUERY_COMMAND_ID: u16 = 167;
#[allow(dead_code)]
pub(super) const VK_CMD_END_QUERY_INDEXED_EXT_COMMAND_ID: u16 = 168;
#[allow(dead_code)]
pub(super) const VK_CMD_END_RENDER_PASS_COMMAND_ID: u16 = 169;
#[allow(dead_code)]
pub(super) const VK_CMD_END_RENDER_PASS2_COMMAND_ID: u16 = 170;
#[allow(dead_code)]
pub(super) const VK_CMD_END_RENDER_PASS2KHR_COMMAND_ID: u16 = 171;
#[allow(dead_code)]
pub(super) const VK_CMD_END_RENDERING_COMMAND_ID: u16 = 172;
#[allow(dead_code)]
pub(super) const VK_CMD_END_RENDERING2EXT_COMMAND_ID: u16 = 173;
#[allow(dead_code)]
pub(super) const VK_CMD_END_RENDERING2KHR_COMMAND_ID: u16 = 174;
#[allow(dead_code)]
pub(super) const VK_CMD_END_RENDERING_KHR_COMMAND_ID: u16 = 175;
#[allow(dead_code)]
pub(super) const VK_CMD_END_SHADER_INSTRUMENTATION_ARM_COMMAND_ID: u16 = 176;
#[allow(dead_code)]
pub(super) const VK_CMD_END_TRANSFORM_FEEDBACK2EXT_COMMAND_ID: u16 = 177;
#[allow(dead_code)]
pub(super) const VK_CMD_END_TRANSFORM_FEEDBACK_EXT_COMMAND_ID: u16 = 178;
#[allow(dead_code)]
pub(super) const VK_CMD_END_VIDEO_CODING_KHR_COMMAND_ID: u16 = 179;
#[allow(dead_code)]
pub(super) const VK_CMD_EXECUTE_COMMANDS_COMMAND_ID: u16 = 180;
#[allow(dead_code)]
pub(super) const VK_CMD_EXECUTE_GENERATED_COMMANDS_EXT_COMMAND_ID: u16 = 181;
#[allow(dead_code)]
pub(super) const VK_CMD_EXECUTE_GENERATED_COMMANDS_NV_COMMAND_ID: u16 = 182;
#[allow(dead_code)]
pub(super) const VK_CMD_FILL_BUFFER_COMMAND_ID: u16 = 183;
#[allow(dead_code)]
pub(super) const VK_CMD_FILL_MEMORY_KHR_COMMAND_ID: u16 = 184;
#[allow(dead_code)]
pub(super) const VK_CMD_INITIALIZE_GRAPH_SCRATCH_MEMORY_AMDX_COMMAND_ID: u16 = 185;
#[allow(dead_code)]
pub(super) const VK_CMD_INSERT_DEBUG_UTILS_LABEL_EXT_COMMAND_ID: u16 = 186;
#[allow(dead_code)]
pub(super) const VK_CMD_NEXT_SUBPASS_COMMAND_ID: u16 = 187;
#[allow(dead_code)]
pub(super) const VK_CMD_NEXT_SUBPASS2_COMMAND_ID: u16 = 188;
#[allow(dead_code)]
pub(super) const VK_CMD_NEXT_SUBPASS2KHR_COMMAND_ID: u16 = 189;
#[allow(dead_code)]
pub(super) const VK_CMD_OPTICAL_FLOW_EXECUTE_NV_COMMAND_ID: u16 = 190;
#[allow(dead_code)]
pub(super) const VK_CMD_PIPELINE_BARRIER_COMMAND_ID: u16 = 191;
#[allow(dead_code)]
pub(super) const VK_CMD_PIPELINE_BARRIER2_COMMAND_ID: u16 = 192;
#[allow(dead_code)]
pub(super) const VK_CMD_PIPELINE_BARRIER2KHR_COMMAND_ID: u16 = 193;
#[allow(dead_code)]
pub(super) const VK_CMD_PREPROCESS_GENERATED_COMMANDS_EXT_COMMAND_ID: u16 = 194;
#[allow(dead_code)]
pub(super) const VK_CMD_PREPROCESS_GENERATED_COMMANDS_NV_COMMAND_ID: u16 = 195;
#[allow(dead_code)]
pub(super) const VK_CMD_PUSH_CONSTANTS_COMMAND_ID: u16 = 196;
#[allow(dead_code)]
pub(super) const VK_CMD_PUSH_CONSTANTS2_COMMAND_ID: u16 = 197;
#[allow(dead_code)]
pub(super) const VK_CMD_PUSH_CONSTANTS2KHR_COMMAND_ID: u16 = 198;
#[allow(dead_code)]
pub(super) const VK_CMD_PUSH_DATA_EXT_COMMAND_ID: u16 = 199;
#[allow(dead_code)]
pub(super) const VK_CMD_PUSH_DESCRIPTOR_SET_COMMAND_ID: u16 = 200;
#[allow(dead_code)]
pub(super) const VK_CMD_PUSH_DESCRIPTOR_SET2_COMMAND_ID: u16 = 201;
#[allow(dead_code)]
pub(super) const VK_CMD_PUSH_DESCRIPTOR_SET2KHR_COMMAND_ID: u16 = 202;
#[allow(dead_code)]
pub(super) const VK_CMD_PUSH_DESCRIPTOR_SET_KHR_COMMAND_ID: u16 = 203;
#[allow(dead_code)]
pub(super) const VK_CMD_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_COMMAND_ID: u16 = 204;
#[allow(dead_code)]
pub(super) const VK_CMD_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE2_COMMAND_ID: u16 = 205;
#[allow(dead_code)]
pub(super) const VK_CMD_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE2KHR_COMMAND_ID: u16 = 206;
#[allow(dead_code)]
pub(super) const VK_CMD_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_KHR_COMMAND_ID: u16 = 207;
#[allow(dead_code)]
pub(super) const VK_CMD_RESET_EVENT_COMMAND_ID: u16 = 208;
#[allow(dead_code)]
pub(super) const VK_CMD_RESET_EVENT2_COMMAND_ID: u16 = 209;
#[allow(dead_code)]
pub(super) const VK_CMD_RESET_EVENT2KHR_COMMAND_ID: u16 = 210;
#[allow(dead_code)]
pub(super) const VK_CMD_RESET_QUERY_POOL_COMMAND_ID: u16 = 211;
#[allow(dead_code)]
pub(super) const VK_CMD_RESOLVE_IMAGE_COMMAND_ID: u16 = 212;
#[allow(dead_code)]
pub(super) const VK_CMD_RESOLVE_IMAGE2_COMMAND_ID: u16 = 213;
#[allow(dead_code)]
pub(super) const VK_CMD_RESOLVE_IMAGE2KHR_COMMAND_ID: u16 = 214;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_ALPHA_TO_COVERAGE_ENABLE_EXT_COMMAND_ID: u16 = 215;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_ALPHA_TO_ONE_ENABLE_EXT_COMMAND_ID: u16 = 216;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_ATTACHMENT_FEEDBACK_LOOP_ENABLE_EXT_COMMAND_ID: u16 = 217;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_BLEND_CONSTANTS_COMMAND_ID: u16 = 218;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_CHECKPOINT_NV_COMMAND_ID: u16 = 219;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_COARSE_SAMPLE_ORDER_NV_COMMAND_ID: u16 = 220;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_COLOR_BLEND_ADVANCED_EXT_COMMAND_ID: u16 = 221;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_COLOR_BLEND_ENABLE_EXT_COMMAND_ID: u16 = 222;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_COLOR_BLEND_EQUATION_EXT_COMMAND_ID: u16 = 223;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_COLOR_WRITE_ENABLE_EXT_COMMAND_ID: u16 = 224;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_COLOR_WRITE_MASK_EXT_COMMAND_ID: u16 = 225;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_COMPUTE_OCCUPANCY_PRIORITY_NV_COMMAND_ID: u16 = 226;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_CONSERVATIVE_RASTERIZATION_MODE_EXT_COMMAND_ID: u16 = 227;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_COVERAGE_MODULATION_MODE_NV_COMMAND_ID: u16 = 228;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_COVERAGE_MODULATION_TABLE_ENABLE_NV_COMMAND_ID: u16 = 229;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_COVERAGE_MODULATION_TABLE_NV_COMMAND_ID: u16 = 230;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_COVERAGE_REDUCTION_MODE_NV_COMMAND_ID: u16 = 231;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_COVERAGE_TO_COLOR_ENABLE_NV_COMMAND_ID: u16 = 232;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_COVERAGE_TO_COLOR_LOCATION_NV_COMMAND_ID: u16 = 233;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_CULL_MODE_COMMAND_ID: u16 = 234;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_CULL_MODE_EXT_COMMAND_ID: u16 = 235;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_DEPTH_BIAS_COMMAND_ID: u16 = 236;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_DEPTH_BIAS2EXT_COMMAND_ID: u16 = 237;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_DEPTH_BIAS_ENABLE_COMMAND_ID: u16 = 238;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_DEPTH_BIAS_ENABLE_EXT_COMMAND_ID: u16 = 239;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_DEPTH_BOUNDS_COMMAND_ID: u16 = 240;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_DEPTH_BOUNDS_TEST_ENABLE_COMMAND_ID: u16 = 241;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_DEPTH_BOUNDS_TEST_ENABLE_EXT_COMMAND_ID: u16 = 242;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_DEPTH_CLAMP_ENABLE_EXT_COMMAND_ID: u16 = 243;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_DEPTH_CLAMP_RANGE_EXT_COMMAND_ID: u16 = 244;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_DEPTH_CLIP_ENABLE_EXT_COMMAND_ID: u16 = 245;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_DEPTH_CLIP_NEGATIVE_ONE_TO_ONE_EXT_COMMAND_ID: u16 = 246;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_DEPTH_COMPARE_OP_COMMAND_ID: u16 = 247;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_DEPTH_COMPARE_OP_EXT_COMMAND_ID: u16 = 248;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_DEPTH_TEST_ENABLE_COMMAND_ID: u16 = 249;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_DEPTH_TEST_ENABLE_EXT_COMMAND_ID: u16 = 250;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_DEPTH_WRITE_ENABLE_COMMAND_ID: u16 = 251;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_DEPTH_WRITE_ENABLE_EXT_COMMAND_ID: u16 = 252;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_DESCRIPTOR_BUFFER_OFFSETS2EXT_COMMAND_ID: u16 = 253;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_DESCRIPTOR_BUFFER_OFFSETS_EXT_COMMAND_ID: u16 = 254;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_DEVICE_MASK_COMMAND_ID: u16 = 255;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_DEVICE_MASK_KHR_COMMAND_ID: u16 = 256;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_DISCARD_RECTANGLE_EXT_COMMAND_ID: u16 = 257;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_DISCARD_RECTANGLE_ENABLE_EXT_COMMAND_ID: u16 = 258;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_DISCARD_RECTANGLE_MODE_EXT_COMMAND_ID: u16 = 259;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_DISPATCH_PARAMETERS_ARM_COMMAND_ID: u16 = 260;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_EVENT_COMMAND_ID: u16 = 261;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_EVENT2_COMMAND_ID: u16 = 262;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_EVENT2KHR_COMMAND_ID: u16 = 263;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_EXCLUSIVE_SCISSOR_ENABLE_NV_COMMAND_ID: u16 = 264;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_EXCLUSIVE_SCISSOR_NV_COMMAND_ID: u16 = 265;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_EXTRA_PRIMITIVE_OVERESTIMATION_SIZE_EXT_COMMAND_ID: u16 = 266;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_FRAGMENT_SHADING_RATE_ENUM_NV_COMMAND_ID: u16 = 267;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_FRAGMENT_SHADING_RATE_KHR_COMMAND_ID: u16 = 268;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_FRONT_FACE_COMMAND_ID: u16 = 269;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_FRONT_FACE_EXT_COMMAND_ID: u16 = 270;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_LINE_RASTERIZATION_MODE_EXT_COMMAND_ID: u16 = 271;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_LINE_STIPPLE_COMMAND_ID: u16 = 272;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_LINE_STIPPLE_EXT_COMMAND_ID: u16 = 273;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_LINE_STIPPLE_ENABLE_EXT_COMMAND_ID: u16 = 274;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_LINE_STIPPLE_KHR_COMMAND_ID: u16 = 275;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_LINE_WIDTH_COMMAND_ID: u16 = 276;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_LOGIC_OP_EXT_COMMAND_ID: u16 = 277;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_LOGIC_OP_ENABLE_EXT_COMMAND_ID: u16 = 278;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_PATCH_CONTROL_POINTS_EXT_COMMAND_ID: u16 = 279;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_PERFORMANCE_MARKER_INTEL_COMMAND_ID: u16 = 280;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_PERFORMANCE_OVERRIDE_INTEL_COMMAND_ID: u16 = 281;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_PERFORMANCE_STREAM_MARKER_INTEL_COMMAND_ID: u16 = 282;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_POLYGON_MODE_EXT_COMMAND_ID: u16 = 283;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_PRIMITIVE_RESTART_ENABLE_COMMAND_ID: u16 = 284;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_PRIMITIVE_RESTART_ENABLE_EXT_COMMAND_ID: u16 = 285;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_PRIMITIVE_RESTART_INDEX_EXT_COMMAND_ID: u16 = 286;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_PRIMITIVE_TOPOLOGY_COMMAND_ID: u16 = 287;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_PRIMITIVE_TOPOLOGY_EXT_COMMAND_ID: u16 = 288;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_PROVOKING_VERTEX_MODE_EXT_COMMAND_ID: u16 = 289;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_RASTERIZATION_SAMPLES_EXT_COMMAND_ID: u16 = 290;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_RASTERIZATION_STREAM_EXT_COMMAND_ID: u16 = 291;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_RASTERIZER_DISCARD_ENABLE_COMMAND_ID: u16 = 292;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_RASTERIZER_DISCARD_ENABLE_EXT_COMMAND_ID: u16 = 293;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_RAY_TRACING_PIPELINE_STACK_SIZE_KHR_COMMAND_ID: u16 = 294;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_RENDERING_ATTACHMENT_LOCATIONS_COMMAND_ID: u16 = 295;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_RENDERING_ATTACHMENT_LOCATIONS_KHR_COMMAND_ID: u16 = 296;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_RENDERING_INPUT_ATTACHMENT_INDICES_COMMAND_ID: u16 = 297;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_RENDERING_INPUT_ATTACHMENT_INDICES_KHR_COMMAND_ID: u16 = 298;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_REPRESENTATIVE_FRAGMENT_TEST_ENABLE_NV_COMMAND_ID: u16 = 299;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_SAMPLE_LOCATIONS_EXT_COMMAND_ID: u16 = 300;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_SAMPLE_LOCATIONS_ENABLE_EXT_COMMAND_ID: u16 = 301;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_SAMPLE_MASK_EXT_COMMAND_ID: u16 = 302;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_SCISSOR_COMMAND_ID: u16 = 303;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_SCISSOR_WITH_COUNT_COMMAND_ID: u16 = 304;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_SCISSOR_WITH_COUNT_EXT_COMMAND_ID: u16 = 305;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_SHADING_RATE_IMAGE_ENABLE_NV_COMMAND_ID: u16 = 306;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_STENCIL_COMPARE_MASK_COMMAND_ID: u16 = 307;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_STENCIL_OP_COMMAND_ID: u16 = 308;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_STENCIL_OP_EXT_COMMAND_ID: u16 = 309;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_STENCIL_REFERENCE_COMMAND_ID: u16 = 310;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_STENCIL_TEST_ENABLE_COMMAND_ID: u16 = 311;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_STENCIL_TEST_ENABLE_EXT_COMMAND_ID: u16 = 312;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_STENCIL_WRITE_MASK_COMMAND_ID: u16 = 313;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_TESSELLATION_DOMAIN_ORIGIN_EXT_COMMAND_ID: u16 = 314;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_VERTEX_INPUT_EXT_COMMAND_ID: u16 = 315;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_VIEWPORT_COMMAND_ID: u16 = 316;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_VIEWPORT_SHADING_RATE_PALETTE_NV_COMMAND_ID: u16 = 317;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_VIEWPORT_SWIZZLE_NV_COMMAND_ID: u16 = 318;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_VIEWPORT_W_SCALING_ENABLE_NV_COMMAND_ID: u16 = 319;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_VIEWPORT_W_SCALING_NV_COMMAND_ID: u16 = 320;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_VIEWPORT_WITH_COUNT_COMMAND_ID: u16 = 321;
#[allow(dead_code)]
pub(super) const VK_CMD_SET_VIEWPORT_WITH_COUNT_EXT_COMMAND_ID: u16 = 322;
#[allow(dead_code)]
pub(super) const VK_CMD_SUBPASS_SHADING_HUAWEI_COMMAND_ID: u16 = 323;
#[allow(dead_code)]
pub(super) const VK_CMD_TRACE_RAYS_INDIRECT2KHR_COMMAND_ID: u16 = 324;
#[allow(dead_code)]
pub(super) const VK_CMD_TRACE_RAYS_INDIRECT_KHR_COMMAND_ID: u16 = 325;
#[allow(dead_code)]
pub(super) const VK_CMD_TRACE_RAYS_KHR_COMMAND_ID: u16 = 326;
#[allow(dead_code)]
pub(super) const VK_CMD_TRACE_RAYS_NV_COMMAND_ID: u16 = 327;
#[allow(dead_code)]
pub(super) const VK_CMD_UPDATE_BUFFER_COMMAND_ID: u16 = 328;
#[allow(dead_code)]
pub(super) const VK_CMD_UPDATE_MEMORY_KHR_COMMAND_ID: u16 = 329;
#[allow(dead_code)]
pub(super) const VK_CMD_UPDATE_PIPELINE_INDIRECT_BUFFER_NV_COMMAND_ID: u16 = 330;
#[allow(dead_code)]
pub(super) const VK_CMD_WAIT_EVENTS_COMMAND_ID: u16 = 331;
#[allow(dead_code)]
pub(super) const VK_CMD_WAIT_EVENTS2_COMMAND_ID: u16 = 332;
#[allow(dead_code)]
pub(super) const VK_CMD_WAIT_EVENTS2KHR_COMMAND_ID: u16 = 333;
#[allow(dead_code)]
pub(super) const VK_CMD_WRITE_ACCELERATION_STRUCTURES_PROPERTIES_KHR_COMMAND_ID: u16 = 334;
#[allow(dead_code)]
pub(super) const VK_CMD_WRITE_ACCELERATION_STRUCTURES_PROPERTIES_NV_COMMAND_ID: u16 = 335;
#[allow(dead_code)]
pub(super) const VK_CMD_WRITE_BUFFER_MARKER2AMD_COMMAND_ID: u16 = 336;
#[allow(dead_code)]
pub(super) const VK_CMD_WRITE_BUFFER_MARKER_AMD_COMMAND_ID: u16 = 337;
#[allow(dead_code)]
pub(super) const VK_CMD_WRITE_MARKER_TO_MEMORY_AMD_COMMAND_ID: u16 = 338;
#[allow(dead_code)]
pub(super) const VK_CMD_WRITE_MICROMAPS_PROPERTIES_EXT_COMMAND_ID: u16 = 339;
#[allow(dead_code)]
pub(super) const VK_CMD_WRITE_TIMESTAMP_COMMAND_ID: u16 = 340;
#[allow(dead_code)]
pub(super) const VK_CMD_WRITE_TIMESTAMP2_COMMAND_ID: u16 = 341;
#[allow(dead_code)]
pub(super) const VK_CMD_WRITE_TIMESTAMP2KHR_COMMAND_ID: u16 = 342;
#[allow(dead_code)]
pub(super) const VK_COMPILE_DEFERRED_NV_COMMAND_ID: u16 = 343;
#[allow(dead_code)]
pub(super) const VK_CONVERT_COOPERATIVE_VECTOR_MATRIX_NV_COMMAND_ID: u16 = 344;
#[allow(dead_code)]
pub(super) const VK_COPY_ACCELERATION_STRUCTURE_KHR_COMMAND_ID: u16 = 345;
#[allow(dead_code)]
pub(super) const VK_COPY_ACCELERATION_STRUCTURE_TO_MEMORY_KHR_COMMAND_ID: u16 = 346;
#[allow(dead_code)]
pub(super) const VK_COPY_IMAGE_TO_IMAGE_COMMAND_ID: u16 = 347;
#[allow(dead_code)]
pub(super) const VK_COPY_IMAGE_TO_IMAGE_EXT_COMMAND_ID: u16 = 348;
#[allow(dead_code)]
pub(super) const VK_COPY_IMAGE_TO_MEMORY_COMMAND_ID: u16 = 349;
#[allow(dead_code)]
pub(super) const VK_COPY_IMAGE_TO_MEMORY_EXT_COMMAND_ID: u16 = 350;
#[allow(dead_code)]
pub(super) const VK_COPY_MEMORY_TO_ACCELERATION_STRUCTURE_KHR_COMMAND_ID: u16 = 351;
#[allow(dead_code)]
pub(super) const VK_COPY_MEMORY_TO_IMAGE_COMMAND_ID: u16 = 352;
#[allow(dead_code)]
pub(super) const VK_COPY_MEMORY_TO_IMAGE_EXT_COMMAND_ID: u16 = 353;
#[allow(dead_code)]
pub(super) const VK_COPY_MEMORY_TO_MICROMAP_EXT_COMMAND_ID: u16 = 354;
#[allow(dead_code)]
pub(super) const VK_COPY_MICROMAP_EXT_COMMAND_ID: u16 = 355;
#[allow(dead_code)]
pub(super) const VK_COPY_MICROMAP_TO_MEMORY_EXT_COMMAND_ID: u16 = 356;
#[allow(dead_code)]
pub(super) const VK_CREATE_ACCELERATION_STRUCTURE2KHR_COMMAND_ID: u16 = 357;
#[allow(dead_code)]
pub(super) const VK_CREATE_ACCELERATION_STRUCTURE_KHR_COMMAND_ID: u16 = 358;
#[allow(dead_code)]
pub(super) const VK_CREATE_ACCELERATION_STRUCTURE_NV_COMMAND_ID: u16 = 359;
#[allow(dead_code)]
pub(super) const VK_CREATE_BUFFER_COMMAND_ID: u16 = 361;
#[allow(dead_code)]
pub(super) const VK_CREATE_BUFFER_COLLECTION_FUCHSIA_COMMAND_ID: u16 = 362;
#[allow(dead_code)]
pub(super) const VK_CREATE_BUFFER_VIEW_COMMAND_ID: u16 = 363;
#[allow(dead_code)]
pub(super) const VK_CREATE_COMMAND_POOL_COMMAND_ID: u16 = 364;
#[allow(dead_code)]
pub(super) const VK_CREATE_COMPUTE_PIPELINES_COMMAND_ID: u16 = 365;
#[allow(dead_code)]
pub(super) const VK_CREATE_CU_FUNCTION_NVX_COMMAND_ID: u16 = 366;
#[allow(dead_code)]
pub(super) const VK_CREATE_CU_MODULE_NVX_COMMAND_ID: u16 = 367;
#[allow(dead_code)]
pub(super) const VK_CREATE_CUDA_FUNCTION_NV_COMMAND_ID: u16 = 368;
#[allow(dead_code)]
pub(super) const VK_CREATE_CUDA_MODULE_NV_COMMAND_ID: u16 = 369;
#[allow(dead_code)]
pub(super) const VK_CREATE_DATA_GRAPH_PIPELINE_SESSION_ARM_COMMAND_ID: u16 = 370;
#[allow(dead_code)]
pub(super) const VK_CREATE_DATA_GRAPH_PIPELINES_ARM_COMMAND_ID: u16 = 371;
#[allow(dead_code)]
pub(super) const VK_CREATE_DEFERRED_OPERATION_KHR_COMMAND_ID: u16 = 374;
#[allow(dead_code)]
pub(super) const VK_CREATE_DESCRIPTOR_POOL_COMMAND_ID: u16 = 375;
#[allow(dead_code)]
pub(super) const VK_CREATE_DESCRIPTOR_SET_LAYOUT_COMMAND_ID: u16 = 376;
#[allow(dead_code)]
pub(super) const VK_CREATE_DESCRIPTOR_UPDATE_TEMPLATE_COMMAND_ID: u16 = 377;
#[allow(dead_code)]
pub(super) const VK_CREATE_DESCRIPTOR_UPDATE_TEMPLATE_KHR_COMMAND_ID: u16 = 378;
#[allow(dead_code)]
pub(super) const VK_CREATE_EVENT_COMMAND_ID: u16 = 383;
#[allow(dead_code)]
pub(super) const VK_CREATE_EXECUTION_GRAPH_PIPELINES_AMDX_COMMAND_ID: u16 = 384;
#[allow(dead_code)]
pub(super) const VK_CREATE_EXTERNAL_COMPUTE_QUEUE_NV_COMMAND_ID: u16 = 385;
#[allow(dead_code)]
pub(super) const VK_CREATE_FENCE_COMMAND_ID: u16 = 386;
#[allow(dead_code)]
pub(super) const VK_CREATE_FRAMEBUFFER_COMMAND_ID: u16 = 387;
#[allow(dead_code)]
pub(super) const VK_CREATE_GPA_SESSION_AMD_COMMAND_ID: u16 = 388;
#[allow(dead_code)]
pub(super) const VK_CREATE_GRAPHICS_PIPELINES_COMMAND_ID: u16 = 389;
#[allow(dead_code)]
pub(super) const VK_CREATE_IMAGE_COMMAND_ID: u16 = 392;
#[allow(dead_code)]
pub(super) const VK_CREATE_IMAGE_VIEW_COMMAND_ID: u16 = 394;
#[allow(dead_code)]
pub(super) const VK_CREATE_INDIRECT_COMMANDS_LAYOUT_EXT_COMMAND_ID: u16 = 395;
#[allow(dead_code)]
pub(super) const VK_CREATE_INDIRECT_COMMANDS_LAYOUT_NV_COMMAND_ID: u16 = 396;
#[allow(dead_code)]
pub(super) const VK_CREATE_INDIRECT_EXECUTION_SET_EXT_COMMAND_ID: u16 = 397;
#[allow(dead_code)]
pub(super) const VK_CREATE_MICROMAP_EXT_COMMAND_ID: u16 = 401;
#[allow(dead_code)]
pub(super) const VK_CREATE_OPTICAL_FLOW_SESSION_NV_COMMAND_ID: u16 = 402;
#[allow(dead_code)]
pub(super) const VK_CREATE_PIPELINE_BINARIES_KHR_COMMAND_ID: u16 = 403;
#[allow(dead_code)]
pub(super) const VK_CREATE_PIPELINE_CACHE_COMMAND_ID: u16 = 404;
#[allow(dead_code)]
pub(super) const VK_CREATE_PIPELINE_LAYOUT_COMMAND_ID: u16 = 405;
#[allow(dead_code)]
pub(super) const VK_CREATE_PRIVATE_DATA_SLOT_COMMAND_ID: u16 = 406;
#[allow(dead_code)]
pub(super) const VK_CREATE_PRIVATE_DATA_SLOT_EXT_COMMAND_ID: u16 = 407;
#[allow(dead_code)]
pub(super) const VK_CREATE_QUERY_POOL_COMMAND_ID: u16 = 408;
#[allow(dead_code)]
pub(super) const VK_CREATE_RAY_TRACING_PIPELINES_KHR_COMMAND_ID: u16 = 409;
#[allow(dead_code)]
pub(super) const VK_CREATE_RAY_TRACING_PIPELINES_NV_COMMAND_ID: u16 = 410;
#[allow(dead_code)]
pub(super) const VK_CREATE_RENDER_PASS_COMMAND_ID: u16 = 411;
#[allow(dead_code)]
pub(super) const VK_CREATE_RENDER_PASS2_COMMAND_ID: u16 = 412;
#[allow(dead_code)]
pub(super) const VK_CREATE_RENDER_PASS2KHR_COMMAND_ID: u16 = 413;
#[allow(dead_code)]
pub(super) const VK_CREATE_SAMPLER_COMMAND_ID: u16 = 414;
#[allow(dead_code)]
pub(super) const VK_CREATE_SAMPLER_YCBCR_CONVERSION_COMMAND_ID: u16 = 415;
#[allow(dead_code)]
pub(super) const VK_CREATE_SAMPLER_YCBCR_CONVERSION_KHR_COMMAND_ID: u16 = 416;
#[allow(dead_code)]
pub(super) const VK_CREATE_SEMAPHORE_COMMAND_ID: u16 = 418;
#[allow(dead_code)]
pub(super) const VK_CREATE_SHADER_INSTRUMENTATION_ARM_COMMAND_ID: u16 = 419;
#[allow(dead_code)]
pub(super) const VK_CREATE_SHADER_MODULE_COMMAND_ID: u16 = 420;
#[allow(dead_code)]
pub(super) const VK_CREATE_SHADERS_EXT_COMMAND_ID: u16 = 421;
#[allow(dead_code)]
pub(super) const VK_CREATE_SHARED_SWAPCHAINS_KHR_COMMAND_ID: u16 = 422;
#[allow(dead_code)]
pub(super) const VK_CREATE_SWAPCHAIN_KHR_COMMAND_ID: u16 = 425;
#[allow(dead_code)]
pub(super) const VK_CREATE_TENSOR_ARM_COMMAND_ID: u16 = 426;
#[allow(dead_code)]
pub(super) const VK_CREATE_TENSOR_VIEW_ARM_COMMAND_ID: u16 = 427;
#[allow(dead_code)]
pub(super) const VK_CREATE_VALIDATION_CACHE_EXT_COMMAND_ID: u16 = 429;
#[allow(dead_code)]
pub(super) const VK_CREATE_VIDEO_SESSION_KHR_COMMAND_ID: u16 = 431;
#[allow(dead_code)]
pub(super) const VK_CREATE_VIDEO_SESSION_PARAMETERS_KHR_COMMAND_ID: u16 = 432;
#[allow(dead_code)]
pub(super) const VK_DEBUG_MARKER_SET_OBJECT_NAME_EXT_COMMAND_ID: u16 = 437;
#[allow(dead_code)]
pub(super) const VK_DEBUG_MARKER_SET_OBJECT_TAG_EXT_COMMAND_ID: u16 = 438;
#[allow(dead_code)]
pub(super) const VK_DEFERRED_OPERATION_JOIN_KHR_COMMAND_ID: u16 = 440;
#[allow(dead_code)]
pub(super) const VK_DESTROY_ACCELERATION_STRUCTURE_KHR_COMMAND_ID: u16 = 441;
#[allow(dead_code)]
pub(super) const VK_DESTROY_ACCELERATION_STRUCTURE_NV_COMMAND_ID: u16 = 442;
#[allow(dead_code)]
pub(super) const VK_DESTROY_BUFFER_COMMAND_ID: u16 = 443;
#[allow(dead_code)]
pub(super) const VK_DESTROY_BUFFER_COLLECTION_FUCHSIA_COMMAND_ID: u16 = 444;
#[allow(dead_code)]
pub(super) const VK_DESTROY_BUFFER_VIEW_COMMAND_ID: u16 = 445;
#[allow(dead_code)]
pub(super) const VK_DESTROY_COMMAND_POOL_COMMAND_ID: u16 = 446;
#[allow(dead_code)]
pub(super) const VK_DESTROY_CU_FUNCTION_NVX_COMMAND_ID: u16 = 447;
#[allow(dead_code)]
pub(super) const VK_DESTROY_CU_MODULE_NVX_COMMAND_ID: u16 = 448;
#[allow(dead_code)]
pub(super) const VK_DESTROY_CUDA_FUNCTION_NV_COMMAND_ID: u16 = 449;
#[allow(dead_code)]
pub(super) const VK_DESTROY_CUDA_MODULE_NV_COMMAND_ID: u16 = 450;
#[allow(dead_code)]
pub(super) const VK_DESTROY_DATA_GRAPH_PIPELINE_SESSION_ARM_COMMAND_ID: u16 = 451;
#[allow(dead_code)]
pub(super) const VK_DESTROY_DEFERRED_OPERATION_KHR_COMMAND_ID: u16 = 454;
#[allow(dead_code)]
pub(super) const VK_DESTROY_DESCRIPTOR_POOL_COMMAND_ID: u16 = 455;
#[allow(dead_code)]
pub(super) const VK_DESTROY_DESCRIPTOR_SET_LAYOUT_COMMAND_ID: u16 = 456;
#[allow(dead_code)]
pub(super) const VK_DESTROY_DESCRIPTOR_UPDATE_TEMPLATE_COMMAND_ID: u16 = 457;
#[allow(dead_code)]
pub(super) const VK_DESTROY_DESCRIPTOR_UPDATE_TEMPLATE_KHR_COMMAND_ID: u16 = 458;
#[allow(dead_code)]
pub(super) const VK_DESTROY_DEVICE_COMMAND_ID: u16 = 459;
#[allow(dead_code)]
pub(super) const VK_DESTROY_EVENT_COMMAND_ID: u16 = 460;
#[allow(dead_code)]
pub(super) const VK_DESTROY_EXTERNAL_COMPUTE_QUEUE_NV_COMMAND_ID: u16 = 461;
#[allow(dead_code)]
pub(super) const VK_DESTROY_FENCE_COMMAND_ID: u16 = 462;
#[allow(dead_code)]
pub(super) const VK_DESTROY_FRAMEBUFFER_COMMAND_ID: u16 = 463;
#[allow(dead_code)]
pub(super) const VK_DESTROY_GPA_SESSION_AMD_COMMAND_ID: u16 = 464;
#[allow(dead_code)]
pub(super) const VK_DESTROY_IMAGE_COMMAND_ID: u16 = 465;
#[allow(dead_code)]
pub(super) const VK_DESTROY_IMAGE_VIEW_COMMAND_ID: u16 = 466;
#[allow(dead_code)]
pub(super) const VK_DESTROY_INDIRECT_COMMANDS_LAYOUT_EXT_COMMAND_ID: u16 = 467;
#[allow(dead_code)]
pub(super) const VK_DESTROY_INDIRECT_COMMANDS_LAYOUT_NV_COMMAND_ID: u16 = 468;
#[allow(dead_code)]
pub(super) const VK_DESTROY_INDIRECT_EXECUTION_SET_EXT_COMMAND_ID: u16 = 469;
#[allow(dead_code)]
pub(super) const VK_DESTROY_MICROMAP_EXT_COMMAND_ID: u16 = 471;
#[allow(dead_code)]
pub(super) const VK_DESTROY_OPTICAL_FLOW_SESSION_NV_COMMAND_ID: u16 = 472;
#[allow(dead_code)]
pub(super) const VK_DESTROY_PIPELINE_COMMAND_ID: u16 = 473;
#[allow(dead_code)]
pub(super) const VK_DESTROY_PIPELINE_BINARY_KHR_COMMAND_ID: u16 = 474;
#[allow(dead_code)]
pub(super) const VK_DESTROY_PIPELINE_CACHE_COMMAND_ID: u16 = 475;
#[allow(dead_code)]
pub(super) const VK_DESTROY_PIPELINE_LAYOUT_COMMAND_ID: u16 = 476;
#[allow(dead_code)]
pub(super) const VK_DESTROY_PRIVATE_DATA_SLOT_COMMAND_ID: u16 = 477;
#[allow(dead_code)]
pub(super) const VK_DESTROY_PRIVATE_DATA_SLOT_EXT_COMMAND_ID: u16 = 478;
#[allow(dead_code)]
pub(super) const VK_DESTROY_QUERY_POOL_COMMAND_ID: u16 = 479;
#[allow(dead_code)]
pub(super) const VK_DESTROY_RENDER_PASS_COMMAND_ID: u16 = 480;
#[allow(dead_code)]
pub(super) const VK_DESTROY_SAMPLER_COMMAND_ID: u16 = 481;
#[allow(dead_code)]
pub(super) const VK_DESTROY_SAMPLER_YCBCR_CONVERSION_COMMAND_ID: u16 = 482;
#[allow(dead_code)]
pub(super) const VK_DESTROY_SAMPLER_YCBCR_CONVERSION_KHR_COMMAND_ID: u16 = 483;
#[allow(dead_code)]
pub(super) const VK_DESTROY_SEMAPHORE_COMMAND_ID: u16 = 484;
#[allow(dead_code)]
pub(super) const VK_DESTROY_SHADER_EXT_COMMAND_ID: u16 = 485;
#[allow(dead_code)]
pub(super) const VK_DESTROY_SHADER_INSTRUMENTATION_ARM_COMMAND_ID: u16 = 486;
#[allow(dead_code)]
pub(super) const VK_DESTROY_SHADER_MODULE_COMMAND_ID: u16 = 487;
#[allow(dead_code)]
pub(super) const VK_DESTROY_SWAPCHAIN_KHR_COMMAND_ID: u16 = 489;
#[allow(dead_code)]
pub(super) const VK_DESTROY_TENSOR_ARM_COMMAND_ID: u16 = 490;
#[allow(dead_code)]
pub(super) const VK_DESTROY_TENSOR_VIEW_ARM_COMMAND_ID: u16 = 491;
#[allow(dead_code)]
pub(super) const VK_DESTROY_VALIDATION_CACHE_EXT_COMMAND_ID: u16 = 492;
#[allow(dead_code)]
pub(super) const VK_DESTROY_VIDEO_SESSION_KHR_COMMAND_ID: u16 = 493;
#[allow(dead_code)]
pub(super) const VK_DESTROY_VIDEO_SESSION_PARAMETERS_KHR_COMMAND_ID: u16 = 494;
#[allow(dead_code)]
pub(super) const VK_DEVICE_WAIT_IDLE_COMMAND_ID: u16 = 495;
#[allow(dead_code)]
pub(super) const VK_DISPLAY_POWER_CONTROL_EXT_COMMAND_ID: u16 = 496;
#[allow(dead_code)]
pub(super) const VK_END_COMMAND_BUFFER_COMMAND_ID: u16 = 497;
#[allow(dead_code)]
pub(super) const VK_EXPORT_METAL_OBJECTS_EXT_COMMAND_ID: u16 = 509;
#[allow(dead_code)]
pub(super) const VK_FLUSH_MAPPED_MEMORY_RANGES_COMMAND_ID: u16 = 510;
#[allow(dead_code)]
pub(super) const VK_FREE_COMMAND_BUFFERS_COMMAND_ID: u16 = 511;
#[allow(dead_code)]
pub(super) const VK_FREE_DESCRIPTOR_SETS_COMMAND_ID: u16 = 512;
#[allow(dead_code)]
pub(super) const VK_FREE_MEMORY_COMMAND_ID: u16 = 513;
#[allow(dead_code)]
pub(super) const VK_GET_ACCELERATION_STRUCTURE_BUILD_SIZES_KHR_COMMAND_ID: u16 = 514;
#[allow(dead_code)]
pub(super) const VK_GET_ACCELERATION_STRUCTURE_DEVICE_ADDRESS_KHR_COMMAND_ID: u16 = 515;
#[allow(dead_code)]
pub(super) const VK_GET_ACCELERATION_STRUCTURE_HANDLE_NV_COMMAND_ID: u16 = 516;
#[allow(dead_code)]
pub(super) const VK_GET_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_NV_COMMAND_ID: u16 = 517;
#[allow(dead_code)]
pub(super) const VK_GET_ACCELERATION_STRUCTURE_OPAQUE_CAPTURE_DESCRIPTOR_DATA_EXT_COMMAND_ID: u16 =
    518;
#[allow(dead_code)]
pub(super) const VK_GET_ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID_COMMAND_ID: u16 = 519;
#[allow(dead_code)]
pub(super) const VK_GET_BUFFER_COLLECTION_PROPERTIES_FUCHSIA_COMMAND_ID: u16 = 520;
#[allow(dead_code)]
pub(super) const VK_GET_BUFFER_DEVICE_ADDRESS_COMMAND_ID: u16 = 521;
#[allow(dead_code)]
pub(super) const VK_GET_BUFFER_DEVICE_ADDRESS_EXT_COMMAND_ID: u16 = 522;
#[allow(dead_code)]
pub(super) const VK_GET_BUFFER_DEVICE_ADDRESS_KHR_COMMAND_ID: u16 = 523;
#[allow(dead_code)]
pub(super) const VK_GET_BUFFER_MEMORY_REQUIREMENTS_COMMAND_ID: u16 = 524;
#[allow(dead_code)]
pub(super) const VK_GET_BUFFER_MEMORY_REQUIREMENTS2_COMMAND_ID: u16 = 525;
#[allow(dead_code)]
pub(super) const VK_GET_BUFFER_MEMORY_REQUIREMENTS2KHR_COMMAND_ID: u16 = 526;
#[allow(dead_code)]
pub(super) const VK_GET_BUFFER_OPAQUE_CAPTURE_ADDRESS_COMMAND_ID: u16 = 527;
#[allow(dead_code)]
pub(super) const VK_GET_BUFFER_OPAQUE_CAPTURE_ADDRESS_KHR_COMMAND_ID: u16 = 528;
#[allow(dead_code)]
pub(super) const VK_GET_BUFFER_OPAQUE_CAPTURE_DESCRIPTOR_DATA_EXT_COMMAND_ID: u16 = 529;
#[allow(dead_code)]
pub(super) const VK_GET_CALIBRATED_TIMESTAMPS_EXT_COMMAND_ID: u16 = 530;
#[allow(dead_code)]
pub(super) const VK_GET_CALIBRATED_TIMESTAMPS_KHR_COMMAND_ID: u16 = 531;
#[allow(dead_code)]
pub(super) const VK_GET_CLUSTER_ACCELERATION_STRUCTURE_BUILD_SIZES_NV_COMMAND_ID: u16 = 532;
#[allow(dead_code)]
pub(super) const VK_GET_CUDA_MODULE_CACHE_NV_COMMAND_ID: u16 = 533;
#[allow(dead_code)]
pub(super) const VK_GET_DATA_GRAPH_PIPELINE_AVAILABLE_PROPERTIES_ARM_COMMAND_ID: u16 = 534;
#[allow(dead_code)]
pub(super) const VK_GET_DATA_GRAPH_PIPELINE_PROPERTIES_ARM_COMMAND_ID: u16 = 535;
#[allow(dead_code)]
pub(super) const VK_GET_DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENTS_ARM_COMMAND_ID: u16 =
    536;
#[allow(dead_code)]
pub(super) const VK_GET_DATA_GRAPH_PIPELINE_SESSION_MEMORY_REQUIREMENTS_ARM_COMMAND_ID: u16 = 537;
#[allow(dead_code)]
pub(super) const VK_GET_DEFERRED_OPERATION_MAX_CONCURRENCY_KHR_COMMAND_ID: u16 = 538;
#[allow(dead_code)]
pub(super) const VK_GET_DEFERRED_OPERATION_RESULT_KHR_COMMAND_ID: u16 = 539;
#[allow(dead_code)]
pub(super) const VK_GET_DESCRIPTOR_EXT_COMMAND_ID: u16 = 540;
#[allow(dead_code)]
pub(super) const VK_GET_DESCRIPTOR_SET_HOST_MAPPING_VALVE_COMMAND_ID: u16 = 541;
#[allow(dead_code)]
pub(super) const VK_GET_DESCRIPTOR_SET_LAYOUT_BINDING_OFFSET_EXT_COMMAND_ID: u16 = 542;
#[allow(dead_code)]
pub(super) const VK_GET_DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE_COMMAND_ID: u16 = 543;
#[allow(dead_code)]
pub(super) const VK_GET_DESCRIPTOR_SET_LAYOUT_SIZE_EXT_COMMAND_ID: u16 = 544;
#[allow(dead_code)]
pub(super) const VK_GET_DESCRIPTOR_SET_LAYOUT_SUPPORT_COMMAND_ID: u16 = 545;
#[allow(dead_code)]
pub(super) const VK_GET_DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR_COMMAND_ID: u16 = 546;
#[allow(dead_code)]
pub(super) const VK_GET_DEVICE_ACCELERATION_STRUCTURE_COMPATIBILITY_KHR_COMMAND_ID: u16 = 547;
#[allow(dead_code)]
pub(super) const VK_GET_DEVICE_BUFFER_MEMORY_REQUIREMENTS_COMMAND_ID: u16 = 548;
#[allow(dead_code)]
pub(super) const VK_GET_DEVICE_BUFFER_MEMORY_REQUIREMENTS_KHR_COMMAND_ID: u16 = 549;
#[allow(dead_code)]
pub(super) const VK_GET_DEVICE_COMBINED_IMAGE_SAMPLER_INDEX_NVX_COMMAND_ID: u16 = 550;
#[allow(dead_code)]
pub(super) const VK_GET_DEVICE_FAULT_DEBUG_INFO_KHR_COMMAND_ID: u16 = 551;
#[allow(dead_code)]
pub(super) const VK_GET_DEVICE_FAULT_INFO_EXT_COMMAND_ID: u16 = 552;
#[allow(dead_code)]
pub(super) const VK_GET_DEVICE_FAULT_REPORTS_KHR_COMMAND_ID: u16 = 553;
#[allow(dead_code)]
pub(super) const VK_GET_DEVICE_GROUP_PEER_MEMORY_FEATURES_COMMAND_ID: u16 = 554;
#[allow(dead_code)]
pub(super) const VK_GET_DEVICE_GROUP_PEER_MEMORY_FEATURES_KHR_COMMAND_ID: u16 = 555;
#[allow(dead_code)]
pub(super) const VK_GET_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR_COMMAND_ID: u16 = 556;
#[allow(dead_code)]
pub(super) const VK_GET_DEVICE_GROUP_SURFACE_PRESENT_MODES2EXT_COMMAND_ID: u16 = 557;
#[allow(dead_code)]
pub(super) const VK_GET_DEVICE_GROUP_SURFACE_PRESENT_MODES_KHR_COMMAND_ID: u16 = 558;
#[allow(dead_code)]
pub(super) const VK_GET_DEVICE_IMAGE_MEMORY_REQUIREMENTS_COMMAND_ID: u16 = 559;
#[allow(dead_code)]
pub(super) const VK_GET_DEVICE_IMAGE_MEMORY_REQUIREMENTS_KHR_COMMAND_ID: u16 = 560;
#[allow(dead_code)]
pub(super) const VK_GET_DEVICE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_COMMAND_ID: u16 = 561;
#[allow(dead_code)]
pub(super) const VK_GET_DEVICE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_KHR_COMMAND_ID: u16 = 562;
#[allow(dead_code)]
pub(super) const VK_GET_DEVICE_IMAGE_SUBRESOURCE_LAYOUT_COMMAND_ID: u16 = 563;
#[allow(dead_code)]
pub(super) const VK_GET_DEVICE_IMAGE_SUBRESOURCE_LAYOUT_KHR_COMMAND_ID: u16 = 564;
#[allow(dead_code)]
pub(super) const VK_GET_DEVICE_MEMORY_COMMITMENT_COMMAND_ID: u16 = 565;
#[allow(dead_code)]
pub(super) const VK_GET_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_COMMAND_ID: u16 = 566;
#[allow(dead_code)]
pub(super) const VK_GET_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_KHR_COMMAND_ID: u16 = 567;
#[allow(dead_code)]
pub(super) const VK_GET_DEVICE_MICROMAP_COMPATIBILITY_EXT_COMMAND_ID: u16 = 568;
#[allow(dead_code)]
pub(super) const VK_GET_DEVICE_PROC_ADDR_COMMAND_ID: u16 = 569;
#[allow(dead_code)]
pub(super) const VK_GET_DEVICE_QUEUE_COMMAND_ID: u16 = 570;
#[allow(dead_code)]
pub(super) const VK_GET_DEVICE_QUEUE2_COMMAND_ID: u16 = 571;
#[allow(dead_code)]
pub(super) const VK_GET_DEVICE_SUBPASS_SHADING_MAX_WORKGROUP_SIZE_HUAWEI_COMMAND_ID: u16 = 572;
#[allow(dead_code)]
pub(super) const VK_GET_DEVICE_TENSOR_MEMORY_REQUIREMENTS_ARM_COMMAND_ID: u16 = 573;
#[allow(dead_code)]
pub(super) const VK_GET_DYNAMIC_RENDERING_TILE_PROPERTIES_QCOM_COMMAND_ID: u16 = 580;
#[allow(dead_code)]
pub(super) const VK_GET_ENCODED_VIDEO_SESSION_PARAMETERS_KHR_COMMAND_ID: u16 = 581;
#[allow(dead_code)]
pub(super) const VK_GET_EVENT_STATUS_COMMAND_ID: u16 = 582;
#[allow(dead_code)]
pub(super) const VK_GET_EXECUTION_GRAPH_PIPELINE_NODE_INDEX_AMDX_COMMAND_ID: u16 = 583;
#[allow(dead_code)]
pub(super) const VK_GET_EXECUTION_GRAPH_PIPELINE_SCRATCH_SIZE_AMDX_COMMAND_ID: u16 = 584;
#[allow(dead_code)]
pub(super) const VK_GET_EXTERNAL_COMPUTE_QUEUE_DATA_NV_COMMAND_ID: u16 = 585;
#[allow(dead_code)]
pub(super) const VK_GET_FENCE_FD_KHR_COMMAND_ID: u16 = 586;
#[allow(dead_code)]
pub(super) const VK_GET_FENCE_STATUS_COMMAND_ID: u16 = 587;
#[allow(dead_code)]
pub(super) const VK_GET_FENCE_WIN32HANDLE_KHR_COMMAND_ID: u16 = 588;
#[allow(dead_code)]
pub(super) const VK_GET_FRAMEBUFFER_TILE_PROPERTIES_QCOM_COMMAND_ID: u16 = 589;
#[allow(dead_code)]
pub(super) const VK_GET_GENERATED_COMMANDS_MEMORY_REQUIREMENTS_EXT_COMMAND_ID: u16 = 590;
#[allow(dead_code)]
pub(super) const VK_GET_GENERATED_COMMANDS_MEMORY_REQUIREMENTS_NV_COMMAND_ID: u16 = 591;
#[allow(dead_code)]
pub(super) const VK_GET_GPA_DEVICE_CLOCK_INFO_AMD_COMMAND_ID: u16 = 592;
#[allow(dead_code)]
pub(super) const VK_GET_GPA_SESSION_RESULTS_AMD_COMMAND_ID: u16 = 593;
#[allow(dead_code)]
pub(super) const VK_GET_GPA_SESSION_STATUS_AMD_COMMAND_ID: u16 = 594;
#[allow(dead_code)]
pub(super) const VK_GET_IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT_COMMAND_ID: u16 = 595;
#[allow(dead_code)]
pub(super) const VK_GET_IMAGE_MEMORY_REQUIREMENTS_COMMAND_ID: u16 = 596;
#[allow(dead_code)]
pub(super) const VK_GET_IMAGE_MEMORY_REQUIREMENTS2_COMMAND_ID: u16 = 597;
#[allow(dead_code)]
pub(super) const VK_GET_IMAGE_MEMORY_REQUIREMENTS2KHR_COMMAND_ID: u16 = 598;
#[allow(dead_code)]
pub(super) const VK_GET_IMAGE_OPAQUE_CAPTURE_DATA_EXT_COMMAND_ID: u16 = 599;
#[allow(dead_code)]
pub(super) const VK_GET_IMAGE_OPAQUE_CAPTURE_DESCRIPTOR_DATA_EXT_COMMAND_ID: u16 = 600;
#[allow(dead_code)]
pub(super) const VK_GET_IMAGE_SPARSE_MEMORY_REQUIREMENTS_COMMAND_ID: u16 = 601;
#[allow(dead_code)]
pub(super) const VK_GET_IMAGE_SPARSE_MEMORY_REQUIREMENTS2_COMMAND_ID: u16 = 602;
#[allow(dead_code)]
pub(super) const VK_GET_IMAGE_SPARSE_MEMORY_REQUIREMENTS2KHR_COMMAND_ID: u16 = 603;
#[allow(dead_code)]
pub(super) const VK_GET_IMAGE_SUBRESOURCE_LAYOUT_COMMAND_ID: u16 = 604;
#[allow(dead_code)]
pub(super) const VK_GET_IMAGE_SUBRESOURCE_LAYOUT2_COMMAND_ID: u16 = 605;
#[allow(dead_code)]
pub(super) const VK_GET_IMAGE_SUBRESOURCE_LAYOUT2EXT_COMMAND_ID: u16 = 606;
#[allow(dead_code)]
pub(super) const VK_GET_IMAGE_SUBRESOURCE_LAYOUT2KHR_COMMAND_ID: u16 = 607;
#[allow(dead_code)]
pub(super) const VK_GET_IMAGE_VIEW_ADDRESS_NVX_COMMAND_ID: u16 = 608;
#[allow(dead_code)]
pub(super) const VK_GET_IMAGE_VIEW_HANDLE64NVX_COMMAND_ID: u16 = 609;
#[allow(dead_code)]
pub(super) const VK_GET_IMAGE_VIEW_HANDLE_NVX_COMMAND_ID: u16 = 610;
#[allow(dead_code)]
pub(super) const VK_GET_IMAGE_VIEW_OPAQUE_CAPTURE_DESCRIPTOR_DATA_EXT_COMMAND_ID: u16 = 611;
#[allow(dead_code)]
pub(super) const VK_GET_LATENCY_TIMINGS_LEGACY_NV_COMMAND_ID: u16 = 613;
#[allow(dead_code)]
pub(super) const VK_GET_LATENCY_TIMINGS_NV_COMMAND_ID: u16 = 614;
#[allow(dead_code)]
pub(super) const VK_GET_MEMORY_ANDROID_HARDWARE_BUFFER_ANDROID_COMMAND_ID: u16 = 615;
#[allow(dead_code)]
pub(super) const VK_GET_MEMORY_FD_KHR_COMMAND_ID: u16 = 616;
#[allow(dead_code)]
pub(super) const VK_GET_MEMORY_FD_PROPERTIES_KHR_COMMAND_ID: u16 = 617;
#[allow(dead_code)]
pub(super) const VK_GET_MEMORY_HOST_POINTER_PROPERTIES_EXT_COMMAND_ID: u16 = 618;
#[allow(dead_code)]
pub(super) const VK_GET_MEMORY_METAL_HANDLE_EXT_COMMAND_ID: u16 = 619;
#[allow(dead_code)]
pub(super) const VK_GET_MEMORY_METAL_HANDLE_PROPERTIES_EXT_COMMAND_ID: u16 = 620;
#[allow(dead_code)]
pub(super) const VK_GET_MEMORY_NATIVE_BUFFER_OHOS_COMMAND_ID: u16 = 621;
#[allow(dead_code)]
pub(super) const VK_GET_MEMORY_REMOTE_ADDRESS_NV_COMMAND_ID: u16 = 622;
#[allow(dead_code)]
pub(super) const VK_GET_MEMORY_WIN32HANDLE_KHR_COMMAND_ID: u16 = 623;
#[allow(dead_code)]
pub(super) const VK_GET_MEMORY_WIN32HANDLE_NV_COMMAND_ID: u16 = 624;
#[allow(dead_code)]
pub(super) const VK_GET_MEMORY_WIN32HANDLE_PROPERTIES_KHR_COMMAND_ID: u16 = 625;
#[allow(dead_code)]
pub(super) const VK_GET_MEMORY_ZIRCON_HANDLE_FUCHSIA_COMMAND_ID: u16 = 626;
#[allow(dead_code)]
pub(super) const VK_GET_MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA_COMMAND_ID: u16 = 627;
#[allow(dead_code)]
pub(super) const VK_GET_MICROMAP_BUILD_SIZES_EXT_COMMAND_ID: u16 = 628;
#[allow(dead_code)]
pub(super) const VK_GET_NATIVE_BUFFER_PROPERTIES_OHOS_COMMAND_ID: u16 = 629;
#[allow(dead_code)]
pub(super) const VK_GET_PARTITIONED_ACCELERATION_STRUCTURES_BUILD_SIZES_NV_COMMAND_ID: u16 = 630;
#[allow(dead_code)]
pub(super) const VK_GET_PAST_PRESENTATION_TIMING_EXT_COMMAND_ID: u16 = 631;
#[allow(dead_code)]
pub(super) const VK_GET_PAST_PRESENTATION_TIMING_GOOGLE_COMMAND_ID: u16 = 632;
#[allow(dead_code)]
pub(super) const VK_GET_PERFORMANCE_PARAMETER_INTEL_COMMAND_ID: u16 = 633;
#[allow(dead_code)]
pub(super) const VK_GET_PIPELINE_BINARY_DATA_KHR_COMMAND_ID: u16 = 705;
#[allow(dead_code)]
pub(super) const VK_GET_PIPELINE_CACHE_DATA_COMMAND_ID: u16 = 706;
#[allow(dead_code)]
pub(super) const VK_GET_PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATIONS_KHR_COMMAND_ID: u16 = 707;
#[allow(dead_code)]
pub(super) const VK_GET_PIPELINE_EXECUTABLE_PROPERTIES_KHR_COMMAND_ID: u16 = 708;
#[allow(dead_code)]
pub(super) const VK_GET_PIPELINE_EXECUTABLE_STATISTICS_KHR_COMMAND_ID: u16 = 709;
#[allow(dead_code)]
pub(super) const VK_GET_PIPELINE_INDIRECT_DEVICE_ADDRESS_NV_COMMAND_ID: u16 = 710;
#[allow(dead_code)]
pub(super) const VK_GET_PIPELINE_INDIRECT_MEMORY_REQUIREMENTS_NV_COMMAND_ID: u16 = 711;
#[allow(dead_code)]
pub(super) const VK_GET_PIPELINE_KEY_KHR_COMMAND_ID: u16 = 712;
#[allow(dead_code)]
pub(super) const VK_GET_PIPELINE_PROPERTIES_EXT_COMMAND_ID: u16 = 713;
#[allow(dead_code)]
pub(super) const VK_GET_PRIVATE_DATA_COMMAND_ID: u16 = 714;
#[allow(dead_code)]
pub(super) const VK_GET_PRIVATE_DATA_EXT_COMMAND_ID: u16 = 715;
#[allow(dead_code)]
pub(super) const VK_GET_QUERY_POOL_RESULTS_COMMAND_ID: u16 = 716;
#[allow(dead_code)]
pub(super) const VK_GET_QUEUE_CHECKPOINT_DATA2NV_COMMAND_ID: u16 = 717;
#[allow(dead_code)]
pub(super) const VK_GET_QUEUE_CHECKPOINT_DATA_NV_COMMAND_ID: u16 = 718;
#[allow(dead_code)]
pub(super) const VK_GET_RAY_TRACING_CAPTURE_REPLAY_SHADER_GROUP_HANDLES_KHR_COMMAND_ID: u16 = 720;
#[allow(dead_code)]
pub(super) const VK_GET_RAY_TRACING_SHADER_GROUP_HANDLES_KHR_COMMAND_ID: u16 = 721;
#[allow(dead_code)]
pub(super) const VK_GET_RAY_TRACING_SHADER_GROUP_HANDLES_NV_COMMAND_ID: u16 = 722;
#[allow(dead_code)]
pub(super) const VK_GET_RAY_TRACING_SHADER_GROUP_STACK_SIZE_KHR_COMMAND_ID: u16 = 723;
#[allow(dead_code)]
pub(super) const VK_GET_REFRESH_CYCLE_DURATION_GOOGLE_COMMAND_ID: u16 = 724;
#[allow(dead_code)]
pub(super) const VK_GET_RENDER_AREA_GRANULARITY_COMMAND_ID: u16 = 725;
#[allow(dead_code)]
pub(super) const VK_GET_RENDERING_AREA_GRANULARITY_COMMAND_ID: u16 = 726;
#[allow(dead_code)]
pub(super) const VK_GET_RENDERING_AREA_GRANULARITY_KHR_COMMAND_ID: u16 = 727;
#[allow(dead_code)]
pub(super) const VK_GET_SAMPLER_OPAQUE_CAPTURE_DESCRIPTOR_DATA_EXT_COMMAND_ID: u16 = 728;
#[allow(dead_code)]
pub(super) const VK_GET_SCREEN_BUFFER_PROPERTIES_QNX_COMMAND_ID: u16 = 729;
#[allow(dead_code)]
pub(super) const VK_GET_SEMAPHORE_COUNTER_VALUE_COMMAND_ID: u16 = 730;
#[allow(dead_code)]
pub(super) const VK_GET_SEMAPHORE_COUNTER_VALUE_KHR_COMMAND_ID: u16 = 731;
#[allow(dead_code)]
pub(super) const VK_GET_SEMAPHORE_FD_KHR_COMMAND_ID: u16 = 732;
#[allow(dead_code)]
pub(super) const VK_GET_SEMAPHORE_WIN32HANDLE_KHR_COMMAND_ID: u16 = 733;
#[allow(dead_code)]
pub(super) const VK_GET_SEMAPHORE_ZIRCON_HANDLE_FUCHSIA_COMMAND_ID: u16 = 734;
#[allow(dead_code)]
pub(super) const VK_GET_SHADER_BINARY_DATA_EXT_COMMAND_ID: u16 = 735;
#[allow(dead_code)]
pub(super) const VK_GET_SHADER_INFO_AMD_COMMAND_ID: u16 = 736;
#[allow(dead_code)]
pub(super) const VK_GET_SHADER_INSTRUMENTATION_VALUES_ARM_COMMAND_ID: u16 = 737;
#[allow(dead_code)]
pub(super) const VK_GET_SHADER_MODULE_CREATE_INFO_IDENTIFIER_EXT_COMMAND_ID: u16 = 738;
#[allow(dead_code)]
pub(super) const VK_GET_SHADER_MODULE_IDENTIFIER_EXT_COMMAND_ID: u16 = 739;
#[allow(dead_code)]
pub(super) const VK_GET_SLEEP_STATUS_LEGACY_NV_COMMAND_ID: u16 = 740;
#[allow(dead_code)]
pub(super) const VK_GET_SWAPCHAIN_COUNTER_EXT_COMMAND_ID: u16 = 741;
#[allow(dead_code)]
pub(super) const VK_GET_SWAPCHAIN_IMAGES_KHR_COMMAND_ID: u16 = 742;
#[allow(dead_code)]
pub(super) const VK_GET_SWAPCHAIN_STATUS_KHR_COMMAND_ID: u16 = 743;
#[allow(dead_code)]
pub(super) const VK_GET_SWAPCHAIN_TIME_DOMAIN_PROPERTIES_EXT_COMMAND_ID: u16 = 744;
#[allow(dead_code)]
pub(super) const VK_GET_SWAPCHAIN_TIMING_PROPERTIES_EXT_COMMAND_ID: u16 = 745;
#[allow(dead_code)]
pub(super) const VK_GET_TENSOR_MEMORY_REQUIREMENTS_ARM_COMMAND_ID: u16 = 746;
#[allow(dead_code)]
pub(super) const VK_GET_TENSOR_OPAQUE_CAPTURE_DATA_ARM_COMMAND_ID: u16 = 747;
#[allow(dead_code)]
pub(super) const VK_GET_TENSOR_OPAQUE_CAPTURE_DESCRIPTOR_DATA_ARM_COMMAND_ID: u16 = 748;
#[allow(dead_code)]
pub(super) const VK_GET_TENSOR_VIEW_OPAQUE_CAPTURE_DESCRIPTOR_DATA_ARM_COMMAND_ID: u16 = 749;
#[allow(dead_code)]
pub(super) const VK_GET_VALIDATION_CACHE_DATA_EXT_COMMAND_ID: u16 = 750;
#[allow(dead_code)]
pub(super) const VK_GET_VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR_COMMAND_ID: u16 = 751;
#[allow(dead_code)]
pub(super) const VK_IMPORT_FENCE_FD_KHR_COMMAND_ID: u16 = 753;
#[allow(dead_code)]
pub(super) const VK_IMPORT_FENCE_WIN32HANDLE_KHR_COMMAND_ID: u16 = 754;
#[allow(dead_code)]
pub(super) const VK_IMPORT_SEMAPHORE_FD_KHR_COMMAND_ID: u16 = 755;
#[allow(dead_code)]
pub(super) const VK_IMPORT_SEMAPHORE_WIN32HANDLE_KHR_COMMAND_ID: u16 = 756;
#[allow(dead_code)]
pub(super) const VK_IMPORT_SEMAPHORE_ZIRCON_HANDLE_FUCHSIA_COMMAND_ID: u16 = 757;
#[allow(dead_code)]
pub(super) const VK_INITIALIZE_PERFORMANCE_API_INTEL_COMMAND_ID: u16 = 758;
#[allow(dead_code)]
pub(super) const VK_INVALIDATE_MAPPED_MEMORY_RANGES_COMMAND_ID: u16 = 759;
#[allow(dead_code)]
pub(super) const VK_LATENCY_SLEEP_LEGACY_NV_COMMAND_ID: u16 = 760;
#[allow(dead_code)]
pub(super) const VK_LATENCY_SLEEP_NV_COMMAND_ID: u16 = 761;
#[allow(dead_code)]
pub(super) const VK_MAP_MEMORY_COMMAND_ID: u16 = 762;
#[allow(dead_code)]
pub(super) const VK_MAP_MEMORY2_COMMAND_ID: u16 = 763;
#[allow(dead_code)]
pub(super) const VK_MAP_MEMORY2KHR_COMMAND_ID: u16 = 764;
#[allow(dead_code)]
pub(super) const VK_MERGE_PIPELINE_CACHES_COMMAND_ID: u16 = 765;
#[allow(dead_code)]
pub(super) const VK_MERGE_VALIDATION_CACHES_EXT_COMMAND_ID: u16 = 766;
#[allow(dead_code)]
pub(super) const VK_QUEUE_BEGIN_DEBUG_UTILS_LABEL_EXT_COMMAND_ID: u16 = 767;
#[allow(dead_code)]
pub(super) const VK_QUEUE_BIND_SPARSE_COMMAND_ID: u16 = 768;
#[allow(dead_code)]
pub(super) const VK_QUEUE_END_DEBUG_UTILS_LABEL_EXT_COMMAND_ID: u16 = 769;
#[allow(dead_code)]
pub(super) const VK_QUEUE_INSERT_DEBUG_UTILS_LABEL_EXT_COMMAND_ID: u16 = 770;
#[allow(dead_code)]
pub(super) const VK_QUEUE_NOTIFY_OUT_OF_BAND_LEGACY_NV_COMMAND_ID: u16 = 771;
#[allow(dead_code)]
pub(super) const VK_QUEUE_NOTIFY_OUT_OF_BAND_NV_COMMAND_ID: u16 = 772;
#[allow(dead_code)]
pub(super) const VK_QUEUE_PRESENT_KHR_COMMAND_ID: u16 = 773;
#[allow(dead_code)]
pub(super) const VK_QUEUE_SET_PERF_HINT_QCOM_COMMAND_ID: u16 = 774;
#[allow(dead_code)]
pub(super) const VK_QUEUE_SET_PERFORMANCE_CONFIGURATION_INTEL_COMMAND_ID: u16 = 775;
#[allow(dead_code)]
pub(super) const VK_QUEUE_SUBMIT_COMMAND_ID: u16 = 776;
#[allow(dead_code)]
pub(super) const VK_QUEUE_SUBMIT2_COMMAND_ID: u16 = 777;
#[allow(dead_code)]
pub(super) const VK_QUEUE_SUBMIT2KHR_COMMAND_ID: u16 = 778;
#[allow(dead_code)]
pub(super) const VK_QUEUE_WAIT_IDLE_COMMAND_ID: u16 = 779;
#[allow(dead_code)]
pub(super) const VK_REGISTER_CUSTOM_BORDER_COLOR_EXT_COMMAND_ID: u16 = 780;
#[allow(dead_code)]
pub(super) const VK_REGISTER_DEVICE_EVENT_EXT_COMMAND_ID: u16 = 781;
#[allow(dead_code)]
pub(super) const VK_REGISTER_DISPLAY_EVENT_EXT_COMMAND_ID: u16 = 782;
#[allow(dead_code)]
pub(super) const VK_RELEASE_CAPTURED_PIPELINE_DATA_KHR_COMMAND_ID: u16 = 783;
#[allow(dead_code)]
pub(super) const VK_RELEASE_FULL_SCREEN_EXCLUSIVE_MODE_EXT_COMMAND_ID: u16 = 785;
#[allow(dead_code)]
pub(super) const VK_RELEASE_PERFORMANCE_CONFIGURATION_INTEL_COMMAND_ID: u16 = 786;
#[allow(dead_code)]
pub(super) const VK_RELEASE_PROFILING_LOCK_KHR_COMMAND_ID: u16 = 787;
#[allow(dead_code)]
pub(super) const VK_RELEASE_SWAPCHAIN_IMAGES_EXT_COMMAND_ID: u16 = 788;
#[allow(dead_code)]
pub(super) const VK_RELEASE_SWAPCHAIN_IMAGES_KHR_COMMAND_ID: u16 = 789;
#[allow(dead_code)]
pub(super) const VK_RESET_COMMAND_BUFFER_COMMAND_ID: u16 = 790;
#[allow(dead_code)]
pub(super) const VK_RESET_COMMAND_POOL_COMMAND_ID: u16 = 791;
#[allow(dead_code)]
pub(super) const VK_RESET_DESCRIPTOR_POOL_COMMAND_ID: u16 = 792;
#[allow(dead_code)]
pub(super) const VK_RESET_EVENT_COMMAND_ID: u16 = 793;
#[allow(dead_code)]
pub(super) const VK_RESET_FENCES_COMMAND_ID: u16 = 794;
#[allow(dead_code)]
pub(super) const VK_RESET_GPA_SESSION_AMD_COMMAND_ID: u16 = 795;
#[allow(dead_code)]
pub(super) const VK_RESET_QUERY_POOL_COMMAND_ID: u16 = 796;
#[allow(dead_code)]
pub(super) const VK_RESET_QUERY_POOL_EXT_COMMAND_ID: u16 = 797;
#[allow(dead_code)]
pub(super) const VK_SET_BUFFER_COLLECTION_BUFFER_CONSTRAINTS_FUCHSIA_COMMAND_ID: u16 = 798;
#[allow(dead_code)]
pub(super) const VK_SET_BUFFER_COLLECTION_IMAGE_CONSTRAINTS_FUCHSIA_COMMAND_ID: u16 = 799;
#[allow(dead_code)]
pub(super) const VK_SET_DEBUG_UTILS_OBJECT_NAME_EXT_COMMAND_ID: u16 = 800;
#[allow(dead_code)]
pub(super) const VK_SET_DEBUG_UTILS_OBJECT_TAG_EXT_COMMAND_ID: u16 = 801;
#[allow(dead_code)]
pub(super) const VK_SET_DEVICE_MEMORY_PRIORITY_EXT_COMMAND_ID: u16 = 802;
#[allow(dead_code)]
pub(super) const VK_SET_EVENT_COMMAND_ID: u16 = 803;
#[allow(dead_code)]
pub(super) const VK_SET_GPA_DEVICE_CLOCK_MODE_AMD_COMMAND_ID: u16 = 804;
#[allow(dead_code)]
pub(super) const VK_SET_HDR_METADATA_EXT_COMMAND_ID: u16 = 805;
#[allow(dead_code)]
pub(super) const VK_SET_LATENCY_MARKER_LEGACY_NV_COMMAND_ID: u16 = 806;
#[allow(dead_code)]
pub(super) const VK_SET_LATENCY_MARKER_NV_COMMAND_ID: u16 = 807;
#[allow(dead_code)]
pub(super) const VK_SET_LATENCY_SLEEP_MODE_LEGACY_NV_COMMAND_ID: u16 = 808;
#[allow(dead_code)]
pub(super) const VK_SET_LATENCY_SLEEP_MODE_NV_COMMAND_ID: u16 = 809;
#[allow(dead_code)]
pub(super) const VK_SET_LOCAL_DIMMING_AMD_COMMAND_ID: u16 = 810;
#[allow(dead_code)]
pub(super) const VK_SET_PRIVATE_DATA_COMMAND_ID: u16 = 811;
#[allow(dead_code)]
pub(super) const VK_SET_PRIVATE_DATA_EXT_COMMAND_ID: u16 = 812;
#[allow(dead_code)]
pub(super) const VK_SET_SWAPCHAIN_PRESENT_TIMING_QUEUE_SIZE_EXT_COMMAND_ID: u16 = 813;
#[allow(dead_code)]
pub(super) const VK_SHUTDOWN_LATENCY_DEVICE_LEGACY_NV_COMMAND_ID: u16 = 814;
#[allow(dead_code)]
pub(super) const VK_SIGNAL_SEMAPHORE_COMMAND_ID: u16 = 815;
#[allow(dead_code)]
pub(super) const VK_SIGNAL_SEMAPHORE_KHR_COMMAND_ID: u16 = 816;
#[allow(dead_code)]
pub(super) const VK_TRANSITION_IMAGE_LAYOUT_COMMAND_ID: u16 = 818;
#[allow(dead_code)]
pub(super) const VK_TRANSITION_IMAGE_LAYOUT_EXT_COMMAND_ID: u16 = 819;
#[allow(dead_code)]
pub(super) const VK_TRIM_COMMAND_POOL_COMMAND_ID: u16 = 820;
#[allow(dead_code)]
pub(super) const VK_TRIM_COMMAND_POOL_KHR_COMMAND_ID: u16 = 821;
#[allow(dead_code)]
pub(super) const VK_UNINITIALIZE_PERFORMANCE_API_INTEL_COMMAND_ID: u16 = 822;
#[allow(dead_code)]
pub(super) const VK_UNMAP_MEMORY_COMMAND_ID: u16 = 823;
#[allow(dead_code)]
pub(super) const VK_UNMAP_MEMORY2_COMMAND_ID: u16 = 824;
#[allow(dead_code)]
pub(super) const VK_UNMAP_MEMORY2KHR_COMMAND_ID: u16 = 825;
#[allow(dead_code)]
pub(super) const VK_UNREGISTER_CUSTOM_BORDER_COLOR_EXT_COMMAND_ID: u16 = 826;
#[allow(dead_code)]
pub(super) const VK_UPDATE_DESCRIPTOR_SET_WITH_TEMPLATE_COMMAND_ID: u16 = 827;
#[allow(dead_code)]
pub(super) const VK_UPDATE_DESCRIPTOR_SET_WITH_TEMPLATE_KHR_COMMAND_ID: u16 = 828;
#[allow(dead_code)]
pub(super) const VK_UPDATE_DESCRIPTOR_SETS_COMMAND_ID: u16 = 829;
#[allow(dead_code)]
pub(super) const VK_UPDATE_INDIRECT_EXECUTION_SET_PIPELINE_EXT_COMMAND_ID: u16 = 830;
#[allow(dead_code)]
pub(super) const VK_UPDATE_INDIRECT_EXECUTION_SET_SHADER_EXT_COMMAND_ID: u16 = 831;
#[allow(dead_code)]
pub(super) const VK_UPDATE_VIDEO_SESSION_PARAMETERS_KHR_COMMAND_ID: u16 = 832;
#[allow(dead_code)]
pub(super) const VK_WAIT_FOR_FENCES_COMMAND_ID: u16 = 833;
#[allow(dead_code)]
pub(super) const VK_WAIT_FOR_PRESENT2KHR_COMMAND_ID: u16 = 834;
#[allow(dead_code)]
pub(super) const VK_WAIT_FOR_PRESENT_KHR_COMMAND_ID: u16 = 835;
#[allow(dead_code)]
pub(super) const VK_WAIT_SEMAPHORES_COMMAND_ID: u16 = 836;
#[allow(dead_code)]
pub(super) const VK_WAIT_SEMAPHORES_KHR_COMMAND_ID: u16 = 837;
#[allow(dead_code)]
pub(super) const VK_WRITE_ACCELERATION_STRUCTURES_PROPERTIES_KHR_COMMAND_ID: u16 = 838;
#[allow(dead_code)]
pub(super) const VK_WRITE_MICROMAPS_PROPERTIES_EXT_COMMAND_ID: u16 = 839;
#[allow(dead_code)]
pub(super) const VK_WRITE_RESOURCE_DESCRIPTORS_EXT_COMMAND_ID: u16 = 840;
#[allow(dead_code)]
pub(super) const VK_WRITE_SAMPLER_DESCRIPTORS_EXT_COMMAND_ID: u16 = 841;
pub(crate) static COMMAND_NAMES: &[u8] = b"AcquireDrmDisplayEXTAcquireFullScreenExclusiveModeEXTAcquireNextImage2KHRAcquireNextImageKHRAcquirePerformanceConfigurationINTELAcquireProfilingLockKHRAcquireWinrtDisplayNVAcquireXlibDisplayEXTAllocateCommandBuffersAllocateDescriptorSetsAllocateMemoryAntiLagUpdateAMDBeginCommandBufferBindAccelerationStructureMemoryNVBindBufferMemoryBindBufferMemory2BindBufferMemory2KHRBindDataGraphPipelineSessionMemoryARMBindImageMemoryBindImageMemory2BindImageMemory2KHRBindOpticalFlowSessionImageNVBindTensorMemoryARMBindVideoSessionMemoryKHRBuildAccelerationStructuresKHRBuildMicromapsEXTClearShaderInstrumentationMetricsARMCmdBeginConditionalRendering2EXTCmdBeginConditionalRenderingEXTCmdBeginCustomResolveEXTCmdBeginDebugUtilsLabelEXTCmdBeginGpaSampleAMDCmdBeginGpaSessionAMDCmdBeginPerTileExecutionQCOMCmdBeginQueryCmdBeginQueryIndexedEXTCmdBeginRenderPassCmdBeginRenderPass2CmdBeginRenderPass2KHRCmdBeginRenderingCmdBeginRenderingKHRCmdBeginShaderInstrumentationARMCmdBeginTransformFeedback2EXTCmdBeginTransformFeedbackEXTCmdBeginVideoCodingKHRCmdBindDescriptorBufferEmbeddedSamplers2EXTCmdBindDescriptorBufferEmbeddedSamplersEXTCmdBindDescriptorBuffersEXTCmdBindDescriptorSetsCmdBindDescriptorSets2CmdBindDescriptorSets2KHRCmdBindIndexBufferCmdBindIndexBuffer2CmdBindIndexBuffer2KHRCmdBindIndexBuffer3KHRCmdBindInvocationMaskHUAWEICmdBindPipelineCmdBindPipelineShaderGroupNVCmdBindResourceHeapEXTCmdBindSamplerHeapEXTCmdBindShadersEXTCmdBindShadingRateImageNVCmdBindTileMemoryQCOMCmdBindTransformFeedbackBuffers2EXTCmdBindTransformFeedbackBuffersEXTCmdBindVertexBuffersCmdBindVertexBuffers2CmdBindVertexBuffers2EXTCmdBindVertexBuffers3KHRCmdBlitImageCmdBlitImage2CmdBlitImage2KHRCmdBuildAccelerationStructureNVCmdBuildAccelerationStructuresIndirectKHRCmdBuildAccelerationStructuresKHRCmdBuildClusterAccelerationStructureIndirectNVCmdBuildMicromapsEXTCmdBuildPartitionedAccelerationStructuresNVCmdClearAttachmentsCmdClearColorImageCmdClearDepthStencilImageCmdControlVideoCodingKHRCmdConvertCooperativeVectorMatrixNVCmdCopyAccelerationStructureKHRCmdCopyAccelerationStructureNVCmdCopyAccelerationStructureToMemoryKHRCmdCopyBufferCmdCopyBuffer2CmdCopyBuffer2KHRCmdCopyBufferToImageCmdCopyBufferToImage2CmdCopyBufferToImage2KHRCmdCopyGpaSessionResultsAMDCmdCopyImageCmdCopyImage2CmdCopyImage2KHRCmdCopyImageToBufferCmdCopyImageToBuffer2CmdCopyImageToBuffer2KHRCmdCopyImageToMemoryKHRCmdCopyMemoryIndirectKHRCmdCopyMemoryIndirectNVCmdCopyMemoryKHRCmdCopyMemoryToAccelerationStructureKHRCmdCopyMemoryToImageIndirectKHRCmdCopyMemoryToImageIndirectNVCmdCopyMemoryToImageKHRCmdCopyMemoryToMicromapEXTCmdCopyMicromapEXTCmdCopyMicromapToMemoryEXTCmdCopyQueryPoolResultsCmdCopyQueryPoolResultsToMemoryKHRCmdCopyTensorARMCmdCuLaunchKernelNVXCmdCudaLaunchKernelNVCmdDebugMarkerBeginEXTCmdDebugMarkerEndEXTCmdDebugMarkerInsertEXTCmdDecodeVideoKHRCmdDecompressMemoryEXTCmdDecompressMemoryIndirectCountEXTCmdDecompressMemoryIndirectCountNVCmdDecompressMemoryNVCmdDispatchCmdDispatchBaseCmdDispatchBaseKHRCmdDispatchDataGraphARMCmdDispatchGraphAMDXCmdDispatchGraphIndirectAMDXCmdDispatchGraphIndirectCountAMDXCmdDispatchIndirectCmdDispatchIndirect2KHRCmdDispatchTileQCOMCmdDrawCmdDrawClusterHUAWEICmdDrawClusterIndirectHUAWEICmdDrawIndexedCmdDrawIndexedIndirectCmdDrawIndexedIndirect2KHRCmdDrawIndexedIndirectCountCmdDrawIndexedIndirectCount2KHRCmdDrawIndexedIndirectCountAMDCmdDrawIndexedIndirectCountKHRCmdDrawIndirectCmdDrawIndirect2KHRCmdDrawIndirectByteCount2EXTCmdDrawIndirectByteCountEXTCmdDrawIndirectCountCmdDrawIndirectCount2KHRCmdDrawIndirectCountAMDCmdDrawIndirectCountKHRCmdDrawMeshTasksEXTCmdDrawMeshTasksIndirect2EXTCmdDrawMeshTasksIndirectCount2EXTCmdDrawMeshTasksIndirectCountEXTCmdDrawMeshTasksIndirectCountNVCmdDrawMeshTasksIndirectEXTCmdDrawMeshTasksIndirectNVCmdDrawMeshTasksNVCmdDrawMultiEXTCmdDrawMultiIndexedEXTCmdEncodeVideoKHRCmdEndConditionalRenderingEXTCmdEndDebugUtilsLabelEXTCmdEndGpaSampleAMDCmdEndGpaSessionAMDCmdEndPerTileExecutionQCOMCmdEndQueryCmdEndQueryIndexedEXTCmdEndRenderPassCmdEndRenderPass2CmdEndRenderPass2KHRCmdEndRenderingCmdEndRendering2EXTCmdEndRendering2KHRCmdEndRenderingKHRCmdEndShaderInstrumentationARMCmdEndTransformFeedback2EXTCmdEndTransformFeedbackEXTCmdEndVideoCodingKHRCmdExecuteCommandsCmdExecuteGeneratedCommandsEXTCmdExecuteGeneratedCommandsNVCmdFillBufferCmdFillMemoryKHRCmdInitializeGraphScratchMemoryAMDXCmdInsertDebugUtilsLabelEXTCmdNextSubpassCmdNextSubpass2CmdNextSubpass2KHRCmdOpticalFlowExecuteNVCmdPipelineBarrierCmdPipelineBarrier2CmdPipelineBarrier2KHRCmdPreprocessGeneratedCommandsEXTCmdPreprocessGeneratedCommandsNVCmdPushConstantsCmdPushConstants2CmdPushConstants2KHRCmdPushDataEXTCmdPushDescriptorSetCmdPushDescriptorSet2CmdPushDescriptorSet2KHRCmdPushDescriptorSetKHRCmdPushDescriptorSetWithTemplateCmdPushDescriptorSetWithTemplate2CmdPushDescriptorSetWithTemplate2KHRCmdPushDescriptorSetWithTemplateKHRCmdResetEventCmdResetEvent2CmdResetEvent2KHRCmdResetQueryPoolCmdResolveImageCmdResolveImage2CmdResolveImage2KHRCmdSetAlphaToCoverageEnableEXTCmdSetAlphaToOneEnableEXTCmdSetAttachmentFeedbackLoopEnableEXTCmdSetBlendConstantsCmdSetCheckpointNVCmdSetCoarseSampleOrderNVCmdSetColorBlendAdvancedEXTCmdSetColorBlendEnableEXTCmdSetColorBlendEquationEXTCmdSetColorWriteEnableEXTCmdSetColorWriteMaskEXTCmdSetComputeOccupancyPriorityNVCmdSetConservativeRasterizationModeEXTCmdSetCoverageModulationModeNVCmdSetCoverageModulationTableEnableNVCmdSetCoverageModulationTableNVCmdSetCoverageReductionModeNVCmdSetCoverageToColorEnableNVCmdSetCoverageToColorLocationNVCmdSetCullModeCmdSetCullModeEXTCmdSetDepthBiasCmdSetDepthBias2EXTCmdSetDepthBiasEnableCmdSetDepthBiasEnableEXTCmdSetDepthBoundsCmdSetDepthBoundsTestEnableCmdSetDepthBoundsTestEnableEXTCmdSetDepthClampEnableEXTCmdSetDepthClampRangeEXTCmdSetDepthClipEnableEXTCmdSetDepthClipNegativeOneToOneEXTCmdSetDepthCompareOpCmdSetDepthCompareOpEXTCmdSetDepthTestEnableCmdSetDepthTestEnableEXTCmdSetDepthWriteEnableCmdSetDepthWriteEnableEXTCmdSetDescriptorBufferOffsets2EXTCmdSetDescriptorBufferOffsetsEXTCmdSetDeviceMaskCmdSetDeviceMaskKHRCmdSetDiscardRectangleEXTCmdSetDiscardRectangleEnableEXTCmdSetDiscardRectangleModeEXTCmdSetDispatchParametersARMCmdSetEventCmdSetEvent2CmdSetEvent2KHRCmdSetExclusiveScissorEnableNVCmdSetExclusiveScissorNVCmdSetExtraPrimitiveOverestimationSizeEXTCmdSetFragmentShadingRateEnumNVCmdSetFragmentShadingRateKHRCmdSetFrontFaceCmdSetFrontFaceEXTCmdSetLineRasterizationModeEXTCmdSetLineStippleCmdSetLineStippleEXTCmdSetLineStippleEnableEXTCmdSetLineStippleKHRCmdSetLineWidthCmdSetLogicOpEXTCmdSetLogicOpEnableEXTCmdSetPatchControlPointsEXTCmdSetPerformanceMarkerINTELCmdSetPerformanceOverrideINTELCmdSetPerformanceStreamMarkerINTELCmdSetPolygonModeEXTCmdSetPrimitiveRestartEnableCmdSetPrimitiveRestartEnableEXTCmdSetPrimitiveRestartIndexEXTCmdSetPrimitiveTopologyCmdSetPrimitiveTopologyEXTCmdSetProvokingVertexModeEXTCmdSetRasterizationSamplesEXTCmdSetRasterizationStreamEXTCmdSetRasterizerDiscardEnableCmdSetRasterizerDiscardEnableEXTCmdSetRayTracingPipelineStackSizeKHRCmdSetRenderingAttachmentLocationsCmdSetRenderingAttachmentLocationsKHRCmdSetRenderingInputAttachmentIndicesCmdSetRenderingInputAttachmentIndicesKHRCmdSetRepresentativeFragmentTestEnableNVCmdSetSampleLocationsEXTCmdSetSampleLocationsEnableEXTCmdSetSampleMaskEXTCmdSetScissorCmdSetScissorWithCountCmdSetScissorWithCountEXTCmdSetShadingRateImageEnableNVCmdSetStencilCompareMaskCmdSetStencilOpCmdSetStencilOpEXTCmdSetStencilReferenceCmdSetStencilTestEnableCmdSetStencilTestEnableEXTCmdSetStencilWriteMaskCmdSetTessellationDomainOriginEXTCmdSetVertexInputEXTCmdSetViewportCmdSetViewportShadingRatePaletteNVCmdSetViewportSwizzleNVCmdSetViewportWScalingEnableNVCmdSetViewportWScalingNVCmdSetViewportWithCountCmdSetViewportWithCountEXTCmdSubpassShadingHUAWEICmdTraceRaysIndirect2KHRCmdTraceRaysIndirectKHRCmdTraceRaysKHRCmdTraceRaysNVCmdUpdateBufferCmdUpdateMemoryKHRCmdUpdatePipelineIndirectBufferNVCmdWaitEventsCmdWaitEvents2CmdWaitEvents2KHRCmdWriteAccelerationStructuresPropertiesKHRCmdWriteAccelerationStructuresPropertiesNVCmdWriteBufferMarker2AMDCmdWriteBufferMarkerAMDCmdWriteMarkerToMemoryAMDCmdWriteMicromapsPropertiesEXTCmdWriteTimestampCmdWriteTimestamp2CmdWriteTimestamp2KHRCompileDeferredNVConvertCooperativeVectorMatrixNVCopyAccelerationStructureKHRCopyAccelerationStructureToMemoryKHRCopyImageToImageCopyImageToImageEXTCopyImageToMemoryCopyImageToMemoryEXTCopyMemoryToAccelerationStructureKHRCopyMemoryToImageCopyMemoryToImageEXTCopyMemoryToMicromapEXTCopyMicromapEXTCopyMicromapToMemoryEXTCreateAccelerationStructure2KHRCreateAccelerationStructureKHRCreateAccelerationStructureNVCreateAndroidSurfaceKHRCreateBufferCreateBufferCollectionFUCHSIACreateBufferViewCreateCommandPoolCreateComputePipelinesCreateCuFunctionNVXCreateCuModuleNVXCreateCudaFunctionNVCreateCudaModuleNVCreateDataGraphPipelineSessionARMCreateDataGraphPipelinesARMCreateDebugReportCallbackEXTCreateDebugUtilsMessengerEXTCreateDeferredOperationKHRCreateDescriptorPoolCreateDescriptorSetLayoutCreateDescriptorUpdateTemplateCreateDescriptorUpdateTemplateKHRCreateDeviceCreateDirectFBSurfaceEXTCreateDisplayModeKHRCreateDisplayPlaneSurfaceKHRCreateEventCreateExecutionGraphPipelinesAMDXCreateExternalComputeQueueNVCreateFenceCreateFramebufferCreateGpaSessionAMDCreateGraphicsPipelinesCreateHeadlessSurfaceEXTCreateIOSSurfaceMVKCreateImageCreateImagePipeSurfaceFUCHSIACreateImageViewCreateIndirectCommandsLayoutEXTCreateIndirectCommandsLayoutNVCreateIndirectExecutionSetEXTCreateInstanceCreateMacOSSurfaceMVKCreateMetalSurfaceEXTCreateMicromapEXTCreateOpticalFlowSessionNVCreatePipelineBinariesKHRCreatePipelineCacheCreatePipelineLayoutCreatePrivateDataSlotCreatePrivateDataSlotEXTCreateQueryPoolCreateRayTracingPipelinesKHRCreateRayTracingPipelinesNVCreateRenderPassCreateRenderPass2CreateRenderPass2KHRCreateSamplerCreateSamplerYcbcrConversionCreateSamplerYcbcrConversionKHRCreateScreenSurfaceQNXCreateSemaphoreCreateShaderInstrumentationARMCreateShaderModuleCreateShadersEXTCreateSharedSwapchainsKHRCreateStreamDescriptorSurfaceGGPCreateSurfaceOHOSCreateSwapchainKHRCreateTensorARMCreateTensorViewARMCreateUbmSurfaceSECCreateValidationCacheEXTCreateViSurfaceNNCreateVideoSessionKHRCreateVideoSessionParametersKHRCreateWaylandSurfaceKHRCreateWin32SurfaceKHRCreateXcbSurfaceKHRCreateXlibSurfaceKHRDebugMarkerSetObjectNameEXTDebugMarkerSetObjectTagEXTDebugReportMessageEXTDeferredOperationJoinKHRDestroyAccelerationStructureKHRDestroyAccelerationStructureNVDestroyBufferDestroyBufferCollectionFUCHSIADestroyBufferViewDestroyCommandPoolDestroyCuFunctionNVXDestroyCuModuleNVXDestroyCudaFunctionNVDestroyCudaModuleNVDestroyDataGraphPipelineSessionARMDestroyDebugReportCallbackEXTDestroyDebugUtilsMessengerEXTDestroyDeferredOperationKHRDestroyDescriptorPoolDestroyDescriptorSetLayoutDestroyDescriptorUpdateTemplateDestroyDescriptorUpdateTemplateKHRDestroyDeviceDestroyEventDestroyExternalComputeQueueNVDestroyFenceDestroyFramebufferDestroyGpaSessionAMDDestroyImageDestroyImageViewDestroyIndirectCommandsLayoutEXTDestroyIndirectCommandsLayoutNVDestroyIndirectExecutionSetEXTDestroyInstanceDestroyMicromapEXTDestroyOpticalFlowSessionNVDestroyPipelineDestroyPipelineBinaryKHRDestroyPipelineCacheDestroyPipelineLayoutDestroyPrivateDataSlotDestroyPrivateDataSlotEXTDestroyQueryPoolDestroyRenderPassDestroySamplerDestroySamplerYcbcrConversionDestroySamplerYcbcrConversionKHRDestroySemaphoreDestroyShaderEXTDestroyShaderInstrumentationARMDestroyShaderModuleDestroySurfaceKHRDestroySwapchainKHRDestroyTensorARMDestroyTensorViewARMDestroyValidationCacheEXTDestroyVideoSessionKHRDestroyVideoSessionParametersKHRDeviceWaitIdleDisplayPowerControlEXTEndCommandBufferEnumerateDeviceExtensionPropertiesEnumerateDeviceLayerPropertiesEnumerateInstanceExtensionPropertiesEnumerateInstanceLayerPropertiesEnumerateInstanceVersionEnumeratePhysicalDeviceGroupsEnumeratePhysicalDeviceGroupsKHREnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARMEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHREnumeratePhysicalDeviceShaderInstrumentationMetricsARMEnumeratePhysicalDevicesExportMetalObjectsEXTFlushMappedMemoryRangesFreeCommandBuffersFreeDescriptorSetsFreeMemoryGetAccelerationStructureBuildSizesKHRGetAccelerationStructureDeviceAddressKHRGetAccelerationStructureHandleNVGetAccelerationStructureMemoryRequirementsNVGetAccelerationStructureOpaqueCaptureDescriptorDataEXTGetAndroidHardwareBufferPropertiesANDROIDGetBufferCollectionPropertiesFUCHSIAGetBufferDeviceAddressGetBufferDeviceAddressEXTGetBufferDeviceAddressKHRGetBufferMemoryRequirementsGetBufferMemoryRequirements2GetBufferMemoryRequirements2KHRGetBufferOpaqueCaptureAddressGetBufferOpaqueCaptureAddressKHRGetBufferOpaqueCaptureDescriptorDataEXTGetCalibratedTimestampsEXTGetCalibratedTimestampsKHRGetClusterAccelerationStructureBuildSizesNVGetCudaModuleCacheNVGetDataGraphPipelineAvailablePropertiesARMGetDataGraphPipelinePropertiesARMGetDataGraphPipelineSessionBindPointRequirementsARMGetDataGraphPipelineSessionMemoryRequirementsARMGetDeferredOperationMaxConcurrencyKHRGetDeferredOperationResultKHRGetDescriptorEXTGetDescriptorSetHostMappingVALVEGetDescriptorSetLayoutBindingOffsetEXTGetDescriptorSetLayoutHostMappingInfoVALVEGetDescriptorSetLayoutSizeEXTGetDescriptorSetLayoutSupportGetDescriptorSetLayoutSupportKHRGetDeviceAccelerationStructureCompatibilityKHRGetDeviceBufferMemoryRequirementsGetDeviceBufferMemoryRequirementsKHRGetDeviceCombinedImageSamplerIndexNVXGetDeviceFaultDebugInfoKHRGetDeviceFaultInfoEXTGetDeviceFaultReportsKHRGetDeviceGroupPeerMemoryFeaturesGetDeviceGroupPeerMemoryFeaturesKHRGetDeviceGroupPresentCapabilitiesKHRGetDeviceGroupSurfacePresentModes2EXTGetDeviceGroupSurfacePresentModesKHRGetDeviceImageMemoryRequirementsGetDeviceImageMemoryRequirementsKHRGetDeviceImageSparseMemoryRequirementsGetDeviceImageSparseMemoryRequirementsKHRGetDeviceImageSubresourceLayoutGetDeviceImageSubresourceLayoutKHRGetDeviceMemoryCommitmentGetDeviceMemoryOpaqueCaptureAddressGetDeviceMemoryOpaqueCaptureAddressKHRGetDeviceMicromapCompatibilityEXTGetDeviceProcAddrGetDeviceQueueGetDeviceQueue2GetDeviceSubpassShadingMaxWorkgroupSizeHUAWEIGetDeviceTensorMemoryRequirementsARMGetDisplayModeProperties2KHRGetDisplayModePropertiesKHRGetDisplayPlaneCapabilities2KHRGetDisplayPlaneCapabilitiesKHRGetDisplayPlaneSupportedDisplaysKHRGetDrmDisplayEXTGetDynamicRenderingTilePropertiesQCOMGetEncodedVideoSessionParametersKHRGetEventStatusGetExecutionGraphPipelineNodeIndexAMDXGetExecutionGraphPipelineScratchSizeAMDXGetExternalComputeQueueDataNVGetFenceFdKHRGetFenceStatusGetFenceWin32HandleKHRGetFramebufferTilePropertiesQCOMGetGeneratedCommandsMemoryRequirementsEXTGetGeneratedCommandsMemoryRequirementsNVGetGpaDeviceClockInfoAMDGetGpaSessionResultsAMDGetGpaSessionStatusAMDGetImageDrmFormatModifierPropertiesEXTGetImageMemoryRequirementsGetImageMemoryRequirements2GetImageMemoryRequirements2KHRGetImageOpaqueCaptureDataEXTGetImageOpaqueCaptureDescriptorDataEXTGetImageSparseMemoryRequirementsGetImageSparseMemoryRequirements2GetImageSparseMemoryRequirements2KHRGetImageSubresourceLayoutGetImageSubresourceLayout2GetImageSubresourceLayout2EXTGetImageSubresourceLayout2KHRGetImageViewAddressNVXGetImageViewHandle64NVXGetImageViewHandleNVXGetImageViewOpaqueCaptureDescriptorDataEXTGetInstanceProcAddrGetLatencyTimingsLegacyNVGetLatencyTimingsNVGetMemoryAndroidHardwareBufferANDROIDGetMemoryFdKHRGetMemoryFdPropertiesKHRGetMemoryHostPointerPropertiesEXTGetMemoryMetalHandleEXTGetMemoryMetalHandlePropertiesEXTGetMemoryNativeBufferOHOSGetMemoryRemoteAddressNVGetMemoryWin32HandleKHRGetMemoryWin32HandleNVGetMemoryWin32HandlePropertiesKHRGetMemoryZirconHandleFUCHSIAGetMemoryZirconHandlePropertiesFUCHSIAGetMicromapBuildSizesEXTGetNativeBufferPropertiesOHOSGetPartitionedAccelerationStructuresBuildSizesNVGetPastPresentationTimingEXTGetPastPresentationTimingGOOGLEGetPerformanceParameterINTELGetPhysicalDeviceCalibrateableTimeDomainsEXTGetPhysicalDeviceCalibrateableTimeDomainsKHRGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNVGetPhysicalDeviceCooperativeMatrixProperties2EXTGetPhysicalDeviceCooperativeMatrixPropertiesKHRGetPhysicalDeviceCooperativeMatrixPropertiesNVGetPhysicalDeviceCooperativeVectorPropertiesNVGetPhysicalDeviceDescriptorSizeEXTGetPhysicalDeviceDirectFBPresentationSupportEXTGetPhysicalDeviceDisplayPlaneProperties2KHRGetPhysicalDeviceDisplayPlanePropertiesKHRGetPhysicalDeviceDisplayProperties2KHRGetPhysicalDeviceDisplayPropertiesKHRGetPhysicalDeviceExternalBufferPropertiesGetPhysicalDeviceExternalBufferPropertiesKHRGetPhysicalDeviceExternalFencePropertiesGetPhysicalDeviceExternalFencePropertiesKHRGetPhysicalDeviceExternalImageFormatPropertiesNVGetPhysicalDeviceExternalSemaphorePropertiesGetPhysicalDeviceExternalSemaphorePropertiesKHRGetPhysicalDeviceExternalTensorPropertiesARMGetPhysicalDeviceFeaturesGetPhysicalDeviceFeatures2GetPhysicalDeviceFeatures2KHRGetPhysicalDeviceFormatPropertiesGetPhysicalDeviceFormatProperties2GetPhysicalDeviceFormatProperties2KHRGetPhysicalDeviceFragmentShadingRatesKHRGetPhysicalDeviceImageFormatPropertiesGetPhysicalDeviceImageFormatProperties2GetPhysicalDeviceImageFormatProperties2KHRGetPhysicalDeviceMemoryPropertiesGetPhysicalDeviceMemoryProperties2GetPhysicalDeviceMemoryProperties2KHRGetPhysicalDeviceMultisamplePropertiesEXTGetPhysicalDeviceOpticalFlowImageFormatsNVGetPhysicalDevicePresentRectanglesKHRGetPhysicalDevicePropertiesGetPhysicalDeviceProperties2GetPhysicalDeviceProperties2KHRGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARMGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARMGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARMGetPhysicalDeviceQueueFamilyDataGraphPropertiesARMGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHRGetPhysicalDeviceQueueFamilyPropertiesGetPhysicalDeviceQueueFamilyProperties2GetPhysicalDeviceQueueFamilyProperties2KHRGetPhysicalDeviceScreenPresentationSupportQNXGetPhysicalDeviceSparseImageFormatPropertiesGetPhysicalDeviceSparseImageFormatProperties2GetPhysicalDeviceSparseImageFormatProperties2KHRGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNVGetPhysicalDeviceSurfaceCapabilities2EXTGetPhysicalDeviceSurfaceCapabilities2KHRGetPhysicalDeviceSurfaceCapabilitiesKHRGetPhysicalDeviceSurfaceFormats2KHRGetPhysicalDeviceSurfaceFormatsKHRGetPhysicalDeviceSurfacePresentModes2EXTGetPhysicalDeviceSurfacePresentModesKHRGetPhysicalDeviceSurfaceSupportKHRGetPhysicalDeviceToolPropertiesGetPhysicalDeviceToolPropertiesEXTGetPhysicalDeviceUbmPresentationSupportSECGetPhysicalDeviceVideoCapabilitiesKHRGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHRGetPhysicalDeviceVideoFormatPropertiesKHRGetPhysicalDeviceWaylandPresentationSupportKHRGetPhysicalDeviceWin32PresentationSupportKHRGetPhysicalDeviceXcbPresentationSupportKHRGetPhysicalDeviceXlibPresentationSupportKHRGetPipelineBinaryDataKHRGetPipelineCacheDataGetPipelineExecutableInternalRepresentationsKHRGetPipelineExecutablePropertiesKHRGetPipelineExecutableStatisticsKHRGetPipelineIndirectDeviceAddressNVGetPipelineIndirectMemoryRequirementsNVGetPipelineKeyKHRGetPipelinePropertiesEXTGetPrivateDataGetPrivateDataEXTGetQueryPoolResultsGetQueueCheckpointData2NVGetQueueCheckpointDataNVGetRandROutputDisplayEXTGetRayTracingCaptureReplayShaderGroupHandlesKHRGetRayTracingShaderGroupHandlesKHRGetRayTracingShaderGroupHandlesNVGetRayTracingShaderGroupStackSizeKHRGetRefreshCycleDurationGOOGLEGetRenderAreaGranularityGetRenderingAreaGranularityGetRenderingAreaGranularityKHRGetSamplerOpaqueCaptureDescriptorDataEXTGetScreenBufferPropertiesQNXGetSemaphoreCounterValueGetSemaphoreCounterValueKHRGetSemaphoreFdKHRGetSemaphoreWin32HandleKHRGetSemaphoreZirconHandleFUCHSIAGetShaderBinaryDataEXTGetShaderInfoAMDGetShaderInstrumentationValuesARMGetShaderModuleCreateInfoIdentifierEXTGetShaderModuleIdentifierEXTGetSleepStatusLegacyNVGetSwapchainCounterEXTGetSwapchainImagesKHRGetSwapchainStatusKHRGetSwapchainTimeDomainPropertiesEXTGetSwapchainTimingPropertiesEXTGetTensorMemoryRequirementsARMGetTensorOpaqueCaptureDataARMGetTensorOpaqueCaptureDescriptorDataARMGetTensorViewOpaqueCaptureDescriptorDataARMGetValidationCacheDataEXTGetVideoSessionMemoryRequirementsKHRGetWinrtDisplayNVImportFenceFdKHRImportFenceWin32HandleKHRImportSemaphoreFdKHRImportSemaphoreWin32HandleKHRImportSemaphoreZirconHandleFUCHSIAInitializePerformanceApiINTELInvalidateMappedMemoryRangesLatencySleepLegacyNVLatencySleepNVMapMemoryMapMemory2MapMemory2KHRMergePipelineCachesMergeValidationCachesEXTQueueBeginDebugUtilsLabelEXTQueueBindSparseQueueEndDebugUtilsLabelEXTQueueInsertDebugUtilsLabelEXTQueueNotifyOutOfBandLegacyNVQueueNotifyOutOfBandNVQueuePresentKHRQueueSetPerfHintQCOMQueueSetPerformanceConfigurationINTELQueueSubmitQueueSubmit2QueueSubmit2KHRQueueWaitIdleRegisterCustomBorderColorEXTRegisterDeviceEventEXTRegisterDisplayEventEXTReleaseCapturedPipelineDataKHRReleaseDisplayEXTReleaseFullScreenExclusiveModeEXTReleasePerformanceConfigurationINTELReleaseProfilingLockKHRReleaseSwapchainImagesEXTReleaseSwapchainImagesKHRResetCommandBufferResetCommandPoolResetDescriptorPoolResetEventResetFencesResetGpaSessionAMDResetQueryPoolResetQueryPoolEXTSetBufferCollectionBufferConstraintsFUCHSIASetBufferCollectionImageConstraintsFUCHSIASetDebugUtilsObjectNameEXTSetDebugUtilsObjectTagEXTSetDeviceMemoryPriorityEXTSetEventSetGpaDeviceClockModeAMDSetHdrMetadataEXTSetLatencyMarkerLegacyNVSetLatencyMarkerNVSetLatencySleepModeLegacyNVSetLatencySleepModeNVSetLocalDimmingAMDSetPrivateDataSetPrivateDataEXTSetSwapchainPresentTimingQueueSizeEXTShutdownLatencyDeviceLegacyNVSignalSemaphoreSignalSemaphoreKHRSubmitDebugUtilsMessageEXTTransitionImageLayoutTransitionImageLayoutEXTTrimCommandPoolTrimCommandPoolKHRUninitializePerformanceApiINTELUnmapMemoryUnmapMemory2UnmapMemory2KHRUnregisterCustomBorderColorEXTUpdateDescriptorSetWithTemplateUpdateDescriptorSetWithTemplateKHRUpdateDescriptorSetsUpdateIndirectExecutionSetPipelineEXTUpdateIndirectExecutionSetShaderEXTUpdateVideoSessionParametersKHRWaitForFencesWaitForPresent2KHRWaitForPresentKHRWaitSemaphoresWaitSemaphoresKHRWriteAccelerationStructuresPropertiesKHRWriteMicromapsPropertiesEXTWriteResourceDescriptorsEXTWriteSamplerDescriptorsEXT";
pub(crate) static COMMAND_TABLE: [CommandRecord; 1024] = [
    CommandRecord {
        name_offset: 5573,
        id: 238,
        name_len: 21,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 10977,
        id: 468,
        name_len: 31,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 2849,
        id: 120,
        name_len: 35,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 8090,
        id: 339,
        name_len: 30,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 53,
        id: 2,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20114,
        id: 752,
        name_len: 17,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 16797,
        id: 657,
        name_len: 29,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 16520,
        id: 650,
        name_len: 43,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 9626,
        id: 407,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 10917,
        id: 465,
        name_len: 12,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 14702,
        id: 595,
        name_len: 38,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 11178,
        id: 477,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21630,
        id: 822,
        name_len: 31,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21794,
        id: 829,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 17643,
        id: 678,
        name_len: 53,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 5999,
        id: 255,
        name_len: 16,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 8324,
        id: 349,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 11365,
        id: 486,
        name_len: 31,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 4373,
        id: 187,
        name_len: 14,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 10053,
        id: 427,
        name_len: 19,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 10207,
        id: 434,
        name_len: 21,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 19732,
        id: 739,
        name_len: 28,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 11803,
        id: 504,
        name_len: 32,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 18731,
        id: 704,
        name_len: 43,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 4402,
        id: 189,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 19430,
        id: 728,
        name_len: 40,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 8866,
        id: 373,
        name_len: 28,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 16826,
        id: 658,
        name_len: 33,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 6438,
        id: 274,
        name_len: 26,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 19284,
        id: 723,
        name_len: 36,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 13853,
        id: 565,
        name_len: 25,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 1926,
        id: 80,
        name_len: 25,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 11960,
        id: 507,
        name_len: 54,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 928,
        id: 41,
        name_len: 32,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 1780,
        id: 75,
        name_len: 46,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 9753,
        id: 413,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 9498,
        id: 401,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 4969,
        id: 215,
        name_len: 30,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3776,
        id: 159,
        name_len: 15,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 14990,
        id: 604,
        name_len: 25,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20421,
        id: 767,
        name_len: 28,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 5887,
        id: 251,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 13819,
        id: 564,
        name_len: 34,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 12459,
        id: 523,
        name_len: 25,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 11566,
        id: 495,
        name_len: 14,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 16930,
        id: 661,
        name_len: 40,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 1586,
        id: 67,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 4346,
        id: 186,
        name_len: 27,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 9231,
        id: 389,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 6676,
        id: 284,
        name_len: 28,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 12670,
        id: 530,
        name_len: 26,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 11241,
        id: 480,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 1826,
        id: 76,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 8042,
        id: 337,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 17156,
        id: 667,
        name_len: 37,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 7144,
        id: 299,
        name_len: 40,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 12434,
        id: 522,
        name_len: 25,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 12014,
        id: 508,
        name_len: 24,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 13289,
        id: 548,
        name_len: 33,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20692,
        id: 780,
        name_len: 28,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 15627,
        id: 628,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 16480,
        id: 649,
        name_len: 40,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 15165,
        id: 611,
        name_len: 42,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 11750,
        id: 502,
        name_len: 24,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 92,
        id: 4,
        name_len: 36,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 2307,
        id: 97,
        name_len: 21,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 10867,
        id: 462,
        name_len: 12,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20519,
        id: 771,
        name_len: 28,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3830,
        id: 162,
        name_len: 29,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 5594,
        id: 239,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 7902,
        id: 332,
        name_len: 14,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3292,
        id: 140,
        name_len: 31,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 18599,
        id: 701,
        name_len: 46,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 891,
        id: 39,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 7933,
        id: 334,
        name_len: 43,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 18689,
        id: 703,
        name_len: 42,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 16893,
        id: 660,
        name_len: 37,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 2538,
        id: 106,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 19023,
        id: 713,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 5692,
        id: 243,
        name_len: 25,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 20881,
        id: 787,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 10645,
        id: 453,
        name_len: 29,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 6090,
        id: 259,
        name_len: 29,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 15756,
        id: 632,
        name_len: 31,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 14015,
        id: 571,
        name_len: 15,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20255,
        id: 758,
        name_len: 29,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 11652,
        id: 499,
        name_len: 30,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 19522,
        id: 731,
        name_len: 27,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 13322,
        id: 549,
        name_len: 36,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 12631,
        id: 529,
        name_len: 39,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 10320,
        id: 439,
        name_len: 21,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 10504,
        id: 447,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 14633,
        id: 592,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 14657,
        id: 593,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 10247,
        id: 436,
        name_len: 20,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 1256,
        id: 53,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 1646,
        id: 70,
        name_len: 13,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 15144,
        id: 610,
        name_len: 21,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 18774,
        id: 705,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 7404,
        id: 310,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 6871,
        id: 291,
        name_len: 28,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 17340,
        id: 672,
        name_len: 28,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 9084,
        id: 382,
        name_len: 28,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 10469,
        id: 445,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 15815,
        id: 634,
        name_len: 44,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 19373,
        id: 726,
        name_len: 27,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 4567,
        id: 196,
        name_len: 16,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 4282,
        id: 183,
        name_len: 13,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 20490,
        id: 770,
        name_len: 29,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 21996,
        id: 838,
        name_len: 40,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 16154,
        id: 641,
        name_len: 34,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 5765,
        id: 246,
        name_len: 34,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 15787,
        id: 633,
        name_len: 28,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 4699,
        id: 203,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20147,
        id: 754,
        name_len: 25,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 6371,
        id: 271,
        name_len: 30,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 2767,
        id: 116,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 4223,
        id: 181,
        name_len: 30,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20449,
        id: 768,
        name_len: 15,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 4675,
        id: 202,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 14954,
        id: 603,
        name_len: 36,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 16746,
        id: 655,
        name_len: 25,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 3957,
        id: 168,
        name_len: 21,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 11038,
        id: 470,
        name_len: 15,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 14921,
        id: 602,
        name_len: 33,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 2787,
        id: 117,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20679,
        id: 779,
        name_len: 13,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 1124,
        id: 47,
        name_len: 27,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3323,
        id: 141,
        name_len: 30,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 6656,
        id: 283,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 8120,
        id: 340,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20929,
        id: 789,
        name_len: 25,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 16859,
        id: 659,
        name_len: 34,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 9773,
        id: 414,
        name_len: 13,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 334,
        id: 15,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 5061,
        id: 218,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 18361,
        id: 695,
        name_len: 31,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 7371,
        id: 308,
        name_len: 15,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 11618,
        id: 498,
        name_len: 34,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 6592,
        id: 281,
        name_len: 30,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 17368,
        id: 673,
        name_len: 31,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 20172,
        id: 755,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 5176,
        id: 223,
        name_len: 27,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 768,
        id: 33,
        name_len: 28,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 2918,
        id: 122,
        name_len: 21,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 19971,
        id: 748,
        name_len: 39,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 2246,
        id: 93,
        name_len: 12,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 5741,
        id: 245,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 1392,
        id: 59,
        name_len: 21,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 19122,
        id: 718,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 13041,
        id: 541,
        name_len: 32,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 10879,
        id: 463,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 12281,
        id: 518,
        name_len: 54,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3217,
        id: 137,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 1476,
        id: 63,
        name_len: 35,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 9930,
        id: 421,
        name_len: 16,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 6184,
        id: 264,
        name_len: 30,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 531,
        id: 24,
        name_len: 30,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 2965,
        id: 125,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 19470,
        id: 729,
        name_len: 28,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 2561,
        id: 107,
        name_len: 26,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 21612,
        id: 821,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 1300,
        id: 55,
        name_len: 27,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 14740,
        id: 596,
        name_len: 26,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 10826,
        id: 460,
        name_len: 12,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21684,
        id: 825,
        name_len: 15,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21017,
        id: 794,
        name_len: 11,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 11487,
        id: 492,
        name_len: 25,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 4205,
        id: 180,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 7889,
        id: 331,
        name_len: 13,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 12539,
        id: 526,
        name_len: 31,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 2422,
        id: 102,
        name_len: 16,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 6928,
        id: 293,
        name_len: 32,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 8920,
        id: 375,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 15207,
        id: 612,
        name_len: 19,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 2704,
        id: 113,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 8811,
        id: 371,
        name_len: 27,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 16108,
        id: 640,
        name_len: 46,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 73,
        id: 3,
        name_len: 19,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3859,
        id: 163,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 10426,
        id: 443,
        name_len: 13,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 2123,
        id: 87,
        name_len: 14,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20812,
        id: 785,
        name_len: 33,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21552,
        id: 818,
        name_len: 21,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 2195,
        id: 91,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 4787,
        id: 206,
        name_len: 36,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 12376,
        id: 520,
        name_len: 36,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20284,
        id: 759,
        name_len: 28,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 1565,
        id: 66,
        name_len: 21,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 10486,
        id: 446,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 10779,
        id: 458,
        name_len: 34,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 8289,
        id: 347,
        name_len: 16,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 22090,
        id: 841,
        name_len: 26,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 18288,
        id: 693,
        name_len: 39,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 1975,
        id: 82,
        name_len: 35,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 7675,
        id: 321,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 5508,
        id: 234,
        name_len: 14,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 16611,
        id: 652,
        name_len: 44,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 21699,
        id: 826,
        name_len: 30,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 10341,
        id: 440,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 701,
        id: 30,
        name_len: 26,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 6704,
        id: 285,
        name_len: 31,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 2154,
        id: 89,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 1889,
        id: 78,
        name_len: 19,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 10945,
        id: 467,
        name_len: 32,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 9845,
        id: 417,
        name_len: 22,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 9112,
        id: 383,
        name_len: 11,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 1951,
        id: 81,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3175,
        id: 135,
        name_len: 28,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 7238,
        id: 302,
        name_len: 19,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 12959,
        id: 538,
        name_len: 37,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 4885,
        id: 210,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 12059,
        id: 510,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 15728,
        id: 631,
        name_len: 28,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 17313,
        id: 671,
        name_len: 27,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 6353,
        id: 270,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20365,
        id: 764,
        name_len: 13,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 16702,
        id: 654,
        name_len: 44,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 423,
        id: 19,
        name_len: 16,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 13073,
        id: 542,
        name_len: 38,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 10365,
        id: 441,
        name_len: 31,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 6484,
        id: 276,
        name_len: 15,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 1172,
        id: 49,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 18899,
        id: 709,
        name_len: 34,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 16655,
        id: 653,
        name_len: 47,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 10582,
        id: 451,
        name_len: 34,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 7270,
        id: 304,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 8723,
        id: 367,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 15528,
        id: 625,
        name_len: 33,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21247,
        id: 804,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 2219,
        id: 92,
        name_len: 27,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 7497,
        id: 314,
        name_len: 33,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 19881,
        id: 745,
        name_len: 31,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3239,
        id: 138,
        name_len: 26,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 17696,
        id: 679,
        name_len: 38,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 12785,
        id: 534,
        name_len: 42,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 11349,
        id: 485,
        name_len: 16,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 18060,
        id: 687,
        name_len: 40,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 267,
        id: 12,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 7550,
        id: 316,
        name_len: 14,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21597,
        id: 820,
        name_len: 15,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 15680,
        id: 630,
        name_len: 48,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 19061,
        id: 715,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 9736,
        id: 412,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 14484,
        id: 587,
        name_len: 14,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 9693,
        id: 410,
        name_len: 27,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21312,
        id: 807,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3492,
        id: 148,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 16358,
        id: 646,
        name_len: 37,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 8995,
        id: 378,
        name_len: 33,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 13243,
        id: 547,
        name_len: 46,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 17276,
        id: 670,
        name_len: 37,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 3006,
        id: 127,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 11451,
        id: 490,
        name_len: 16,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 7724,
        id: 323,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 5251,
        id: 226,
        name_len: 32,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 16436,
        id: 648,
        name_len: 44,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 9515,
        id: 402,
        name_len: 26,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3901,
        id: 165,
        name_len: 19,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 11602,
        id: 497,
        name_len: 16,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 19078,
        id: 716,
        name_len: 19,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 15589,
        id: 627,
        name_len: 38,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 7426,
        id: 311,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 17949,
        id: 685,
        name_len: 48,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 7916,
        id: 333,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 15459,
        id: 622,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 12038,
        id: 509,
        name_len: 21,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 8018,
        id: 336,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 2352,
        id: 99,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21526,
        id: 817,
        name_len: 26,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 4253,
        id: 182,
        name_len: 29,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 11071,
        id: 472,
        name_len: 27,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 8361,
        id: 351,
        name_len: 36,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 19006,
        id: 712,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 5203,
        id: 224,
        name_len: 25,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 14075,
        id: 573,
        name_len: 36,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20,
        id: 1,
        name_len: 33,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 1430,
        id: 61,
        name_len: 25,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 4600,
        id: 198,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 561,
        id: 25,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 8704,
        id: 366,
        name_len: 19,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 21162,
        id: 800,
        name_len: 26,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21077,
        id: 798,
        name_len: 43,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 5024,
        id: 217,
        name_len: 37,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 18426,
        id: 697,
        name_len: 42,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 6735,
        id: 286,
        name_len: 30,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20192,
        id: 756,
        name_len: 29,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 4634,
        id: 200,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 20604,
        id: 775,
        name_len: 37,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 960,
        id: 42,
        name_len: 29,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 13466,
        id: 554,
        name_len: 32,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 8155,
        id: 342,
        name_len: 21,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 4084,
        id: 175,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 12765,
        id: 533,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 19549,
        id: 732,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 18558,
        id: 700,
        name_len: 41,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 4754,
        id: 205,
        name_len: 33,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 8940,
        id: 376,
        name_len: 25,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 5388,
        id: 230,
        name_len: 31,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 14030,
        id: 572,
        name_len: 45,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 458,
        id: 21,
        name_len: 29,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20845,
        id: 786,
        name_len: 36,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 1634,
        id: 69,
        name_len: 12,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 1082,
        id: 46,
        name_len: 42,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 15307,
        id: 616,
        name_len: 14,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 14520,
        id: 589,
        name_len: 32,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 16970,
        id: 662,
        name_len: 38,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 19400,
        id: 727,
        name_len: 30,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 4823,
        id: 207,
        name_len: 35,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 7651,
        id: 320,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 10228,
        id: 435,
        name_len: 19,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 3417,
        id: 145,
        name_len: 28,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 14001,
        id: 570,
        name_len: 14,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 6464,
        id: 275,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 22036,
        id: 839,
        name_len: 27,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20378,
        id: 765,
        name_len: 19,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 677,
        id: 29,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 10294,
        id: 438,
        name_len: 26,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 16771,
        id: 656,
        name_len: 26,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 9184,
        id: 386,
        name_len: 11,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 19097,
        id: 717,
        name_len: 25,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 5124,
        id: 221,
        name_len: 27,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 8649,
        id: 363,
        name_len: 16,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 14851,
        id: 600,
        name_len: 38,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 4654,
        id: 201,
        name_len: 21,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 12722,
        id: 532,
        name_len: 43,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 6499,
        id: 277,
        name_len: 16,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 12335,
        id: 519,
        name_len: 41,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 5283,
        id: 227,
        name_len: 38,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 2041,
        id: 84,
        name_len: 30,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21007,
        id: 793,
        name_len: 10,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 15251,
        id: 614,
        name_len: 19,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 11008,
        id: 469,
        name_len: 30,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 18798,
        id: 706,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 371,
        id: 17,
        name_len: 37,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 6996,
        id: 295,
        name_len: 34,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 7976,
        id: 335,
        name_len: 42,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 2939,
        id: 123,
        name_len: 11,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 2010,
        id: 83,
        name_len: 31,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 1413,
        id: 60,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 2258,
        id: 94,
        name_len: 13,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 20795,
        id: 784,
        name_len: 17,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 4871,
        id: 209,
        name_len: 14,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 8682,
        id: 365,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 9971,
        id: 423,
        name_len: 32,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 172,
        id: 7,
        name_len: 21,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 578,
        id: 26,
        name_len: 36,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 1846,
        id: 77,
        name_len: 43,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 1278,
        id: 54,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 8620,
        id: 362,
        name_len: 29,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20584,
        id: 774,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3946,
        id: 167,
        name_len: 11,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 9278,
        id: 391,
        name_len: 19,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 2375,
        id: 100,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 351,
        id: 16,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 869,
        id: 38,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 15378,
        id: 619,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 13025,
        id: 540,
        name_len: 16,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 11682,
        id: 500,
        name_len: 36,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 12996,
        id: 539,
        name_len: 29,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 10838,
        id: 461,
        name_len: 29,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 12165,
        id: 515,
        name_len: 40,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 6401,
        id: 272,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 128,
        id: 5,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 13951,
        id: 568,
        name_len: 33,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 6899,
        id: 292,
        name_len: 29,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20312,
        id: 760,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 4902,
        id: 211,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3920,
        id: 166,
        name_len: 26,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 2688,
        id: 112,
        name_len: 16,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 1747,
        id: 74,
        name_len: 33,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 6564,
        id: 280,
        name_len: 28,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 19251,
        id: 722,
        name_len: 33,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 5863,
        id: 250,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 9352,
        id: 395,
        name_len: 31,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3472,
        id: 147,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 15903,
        id: 636,
        name_len: 64,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 17047,
        id: 664,
        name_len: 42,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 4950,
        id: 214,
        name_len: 19,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 10020,
        id: 425,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21330,
        id: 808,
        name_len: 27,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 9308,
        id: 393,
        name_len: 29,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 4461,
        id: 192,
        name_len: 19,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 17773,
        id: 681,
        name_len: 42,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 12128,
        id: 514,
        name_len: 37,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3562,
        id: 151,
        name_len: 19,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 2654,
        id: 111,
        name_len: 34,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 13674,
        id: 560,
        name_len: 35,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 6015,
        id: 256,
        name_len: 19,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21661,
        id: 823,
        name_len: 11,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 4185,
        id: 179,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 11718,
        id: 501,
        name_len: 32,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 21378,
        id: 810,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: 0,
        name_len: 20,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 21046,
        id: 796,
        name_len: 14,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 20332,
        id: 761,
        name_len: 14,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 21028,
        id: 795,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 17593,
        id: 677,
        name_len: 50,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 19661,
        id: 737,
        name_len: 33,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 10115,
        id: 430,
        name_len: 17,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 8341,
        id: 350,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 13498,
        id: 555,
        name_len: 35,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 20664,
        id: 778,
        name_len: 15,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 7856,
        id: 330,
        name_len: 33,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 6214,
        id: 265,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 14593,
        id: 591,
        name_len: 40,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 15506,
        id: 624,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 13442,
        id: 553,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 14766,
        id: 597,
        name_len: 27,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 10396,
        id: 442,
        name_len: 30,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 5448,
        id: 232,
        name_len: 29,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 1194,
        id: 50,
        name_len: 25,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 13642,
        id: 559,
        name_len: 32,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 16563,
        id: 651,
        name_len: 48,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 10091,
        id: 429,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 21729,
        id: 827,
        name_len: 31,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 8397,
        id: 352,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 14278,
        id: 580,
        name_len: 37,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 6338,
        id: 269,
        name_len: 15,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 15015,
        id: 605,
        name_len: 26,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 9720,
        id: 411,
        name_len: 16,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 9650,
        id: 408,
        name_len: 15,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 12860,
        id: 536,
        name_len: 51,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 19047,
        id: 714,
        name_len: 14,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20904,
        id: 788,
        name_len: 25,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 8434,
        id: 354,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 9605,
        id: 406,
        name_len: 21,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 6765,
        id: 287,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 15321,
        id: 617,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3155,
        id: 134,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 1908,
        id: 79,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 19566,
        id: 733,
        name_len: 26,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 13788,
        id: 563,
        name_len: 31,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 12082,
        id: 511,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 18645,
        id: 702,
        name_len: 44,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 1455,
        id: 62,
        name_len: 21,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21357,
        id: 809,
        name_len: 21,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 5321,
        id: 228,
        name_len: 30,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 6146,
        id: 261,
        name_len: 11,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 408,
        id: 18,
        name_len: 15,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 19912,
        id: 746,
        name_len: 30,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 8838,
        id: 372,
        name_len: 28,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 2745,
        id: 115,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3054,
        id: 129,
        name_len: 33,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 10701,
        id: 455,
        name_len: 21,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 796,
        id: 34,
        name_len: 13,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 4102,
        id: 176,
        name_len: 30,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 16278,
        id: 644,
        name_len: 42,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 17904,
        id: 684,
        name_len: 45,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 2477,
        id: 104,
        name_len: 31,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 14471,
        id: 586,
        name_len: 13,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 10132,
        id: 431,
        name_len: 21,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 2110,
        id: 86,
        name_len: 13,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 9212,
        id: 388,
        name_len: 19,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20397,
        id: 766,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 16320,
        id: 645,
        name_len: 38,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 3758,
        id: 158,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 7449,
        id: 312,
        name_len: 26,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 13211,
        id: 546,
        name_len: 32,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 7257,
        id: 303,
        name_len: 13,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 8778,
        id: 370,
        name_len: 33,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 14680,
        id: 594,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 11333,
        id: 484,
        name_len: 16,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 1219,
        id: 51,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 9442,
        id: 398,
        name_len: 14,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 251,
        id: 11,
        name_len: 16,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 7067,
        id: 297,
        name_len: 37,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 7475,
        id: 313,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 6059,
        id: 258,
        name_len: 31,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 12511,
        id: 525,
        name_len: 28,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 19349,
        id: 725,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 8608,
        id: 361,
        name_len: 12,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 18505,
        id: 699,
        name_len: 53,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 19217,
        id: 721,
        name_len: 34,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 2724,
        id: 114,
        name_len: 21,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 10897,
        id: 464,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 5539,
        id: 236,
        name_len: 15,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 2071,
        id: 85,
        name_len: 39,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 5228,
        id: 225,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 2508,
        id: 105,
        name_len: 30,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 6310,
        id: 268,
        name_len: 28,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 747,
        id: 32,
        name_len: 21,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 18865,
        id: 708,
        name_len: 34,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21410,
        id: 812,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 15401,
        id: 620,
        name_len: 33,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 19320,
        id: 724,
        name_len: 29,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 3445,
        id: 146,
        name_len: 27,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 12118,
        id: 513,
        name_len: 10,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 5934,
        id: 253,
        name_len: 33,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 21464,
        id: 814,
        name_len: 29,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 9156,
        id: 385,
        name_len: 28,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 989,
        id: 43,
        name_len: 28,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 8556,
        id: 359,
        name_len: 29,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 727,
        id: 31,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 19694,
        id: 738,
        name_len: 38,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 9541,
        id: 403,
        name_len: 25,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3813,
        id: 161,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 19170,
        id: 720,
        name_len: 47,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3978,
        id: 169,
        name_len: 16,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 2399,
        id: 101,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 2328,
        id: 98,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21979,
        id: 837,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 10616,
        id: 452,
        name_len: 29,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 3516,
        id: 149,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 5081,
        id: 219,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3791,
        id: 160,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 11534,
        id: 494,
        name_len: 32,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 11899,
        id: 506,
        name_len: 61,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 10003,
        id: 424,
        name_len: 17,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 14262,
        id: 579,
        name_len: 16,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 809,
        id: 35,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 21493,
        id: 815,
        name_len: 15,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 18179,
        id: 690,
        name_len: 35,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 8760,
        id: 369,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20221,
        id: 757,
        name_len: 34,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 1342,
        id: 57,
        name_len: 28,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 15270,
        id: 615,
        name_len: 37,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 10267,
        id: 437,
        name_len: 27,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 17997,
        id: 686,
        name_len: 63,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 2287,
        id: 96,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 12599,
        id: 528,
        name_len: 32,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 850,
        id: 37,
        name_len: 19,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3581,
        id: 152,
        name_len: 28,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3383,
        id: 143,
        name_len: 15,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3265,
        id: 139,
        name_len: 27,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 17399,
        id: 674,
        name_len: 65,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 11113,
        id: 474,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20355,
        id: 763,
        name_len: 10,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 19760,
        id: 740,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 19592,
        id: 734,
        name_len: 31,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20641,
        id: 776,
        name_len: 11,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 487,
        id: 22,
        name_len: 19,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 4999,
        id: 216,
        name_len: 25,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 10748,
        id: 457,
        name_len: 31,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 5967,
        id: 254,
        name_len: 32,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 10813,
        id: 459,
        name_len: 13,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 12237,
        id: 517,
        name_len: 44,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 15070,
        id: 607,
        name_len: 29,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 13878,
        id: 566,
        name_len: 35,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 8176,
        id: 343,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 17193,
        id: 668,
        name_len: 41,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 9040,
        id: 380,
        name_len: 24,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 13358,
        id: 550,
        name_len: 37,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 14823,
        id: 599,
        name_len: 28,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 8740,
        id: 368,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 4311,
        id: 185,
        name_len: 35,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 832,
        id: 36,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 9028,
        id: 379,
        name_len: 12,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 9383,
        id: 396,
        name_len: 30,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 6034,
        id: 257,
        name_len: 25,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20053,
        id: 750,
        name_len: 25,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 9456,
        id: 399,
        name_len: 21,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 10439,
        id: 444,
        name_len: 30,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 14552,
        id: 590,
        name_len: 41,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 4583,
        id: 197,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 15434,
        id: 621,
        name_len: 25,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20078,
        id: 751,
        name_len: 36,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 6169,
        id: 263,
        name_len: 15,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 9867,
        id: 418,
        name_len: 15,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 19804,
        id: 742,
        name_len: 21,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3994,
        id: 170,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3883,
        id: 164,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 18967,
        id: 711,
        name_len: 39,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 646,
        id: 28,
        name_len: 31,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 2271,
        id: 95,
        name_len: 16,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 11272,
        id: 482,
        name_len: 29,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 11053,
        id: 471,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 11301,
        id: 483,
        name_len: 32,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 11157,
        id: 476,
        name_len: 21,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 506,
        id: 23,
        name_len: 25,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 12205,
        id: 516,
        name_len: 32,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 13421,
        id: 552,
        name_len: 21,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 9912,
        id: 420,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 908,
        id: 40,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 7698,
        id: 322,
        name_len: 26,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 16062,
        id: 639,
        name_len: 46,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 21965,
        id: 836,
        name_len: 14,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 13153,
        id: 544,
        name_len: 29,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21573,
        id: 819,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 285,
        id: 13,
        name_len: 33,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 8414,
        id: 353,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 2174,
        id: 90,
        name_len: 21,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 13913,
        id: 567,
        name_len: 38,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 4919,
        id: 212,
        name_len: 15,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 11512,
        id: 493,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 11432,
        id: 489,
        name_len: 19,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20765,
        id: 783,
        name_len: 30,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 8193,
        id: 344,
        name_len: 32,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 8137,
        id: 341,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 4858,
        id: 208,
        name_len: 13,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 4159,
        id: 178,
        name_len: 26,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 7809,
        id: 327,
        name_len: 14,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 13395,
        id: 551,
        name_len: 26,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 14350,
        id: 582,
        name_len: 14,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 14227,
        id: 578,
        name_len: 35,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 11098,
        id: 473,
        name_len: 15,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 10722,
        id: 456,
        name_len: 26,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 15651,
        id: 629,
        name_len: 29,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 4046,
        id: 173,
        name_len: 19,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 1706,
        id: 73,
        name_len: 41,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 3106,
        id: 131,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 3539,
        id: 150,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21060,
        id: 797,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 18140,
        id: 689,
        name_len: 39,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 20720,
        id: 781,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 8305,
        id: 348,
        name_len: 19,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3087,
        id: 130,
        name_len: 19,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3026,
        id: 128,
        name_len: 28,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 16395,
        id: 647,
        name_len: 41,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 2884,
        id: 121,
        name_len: 34,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 8472,
        id: 356,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21508,
        id: 816,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 9814,
        id: 416,
        name_len: 31,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 10929,
        id: 466,
        name_len: 16,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 5909,
        id: 252,
        name_len: 25,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20988,
        id: 792,
        name_len: 19,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 11396,
        id: 487,
        name_len: 19,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 2810,
        id: 118,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21814,
        id: 830,
        name_len: 37,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 7386,
        id: 309,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 19942,
        id: 747,
        name_len: 29,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 7317,
        id: 306,
        name_len: 30,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 6119,
        id: 260,
        name_len: 27,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21851,
        id: 831,
        name_len: 35,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 12696,
        id: 531,
        name_len: 26,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 11467,
        id: 491,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3674,
        id: 155,
        name_len: 31,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 10542,
        id: 449,
        name_len: 21,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 6418,
        id: 273,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3148,
        id: 133,
        name_len: 7,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3732,
        id: 157,
        name_len: 26,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 14197,
        id: 577,
        name_len: 30,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 15041,
        id: 606,
        name_len: 29,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 17815,
        id: 682,
        name_len: 45,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 4535,
        id: 195,
        name_len: 32,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 2631,
        id: 110,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 1370,
        id: 58,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 16188,
        id: 642,
        name_len: 47,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 5477,
        id: 233,
        name_len: 31,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 15967,
        id: 637,
        name_len: 48,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 20972,
        id: 791,
        name_len: 16,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 19645,
        id: 736,
        name_len: 16,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 12570,
        id: 527,
        name_len: 29,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 1237,
        id: 52,
        name_len: 19,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 1327,
        id: 56,
        name_len: 15,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 7292,
        id: 305,
        name_len: 25,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 17089,
        id: 665,
        name_len: 33,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 15099,
        id: 608,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 7530,
        id: 315,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 10072,
        id: 428,
        name_len: 19,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 1017,
        id: 44,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 5419,
        id: 231,
        name_len: 29,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 5351,
        id: 229,
        name_len: 37,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 9123,
        id: 384,
        name_len: 33,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 15121,
        id: 609,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 8526,
        id: 358,
        name_len: 30,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 5819,
        id: 248,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 7771,
        id: 325,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 3609,
        id: 153,
        name_len: 33,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 4502,
        id: 194,
        name_len: 33,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 7184,
        id: 300,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 22063,
        id: 840,
        name_len: 27,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 18933,
        id: 710,
        name_len: 34,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3129,
        id: 132,
        name_len: 19,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21213,
        id: 802,
        name_len: 26,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21271,
        id: 805,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 6537,
        id: 279,
        name_len: 27,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21239,
        id: 803,
        name_len: 8,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21948,
        id: 835,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20346,
        id: 762,
        name_len: 9,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 4480,
        id: 193,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 7030,
        id: 296,
        name_len: 37,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 8894,
        id: 374,
        name_len: 26,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21917,
        id: 833,
        name_len: 13,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 6157,
        id: 262,
        name_len: 12,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 10038,
        id: 426,
        name_len: 15,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 14315,
        id: 581,
        name_len: 35,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 13606,
        id: 558,
        name_len: 36,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3398,
        id: 144,
        name_len: 19,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 4420,
        id: 190,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 5522,
        id: 235,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 1610,
        id: 68,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21760,
        id: 828,
        name_len: 34,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3203,
        id: 136,
        name_len: 14,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 17234,
        id: 669,
        name_len: 42,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 318,
        id: 14,
        name_len: 16,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 6238,
        id: 266,
        name_len: 41,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 2587,
        id: 108,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 19782,
        id: 741,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 237,
        id: 10,
        name_len: 14,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 10524,
        id: 448,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 11580,
        id: 496,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 4295,
        id: 184,
        name_len: 16,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 11415,
        id: 488,
        name_len: 17,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 5717,
        id: 244,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 4387,
        id: 188,
        name_len: 15,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 439,
        id: 20,
        name_len: 19,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 19846,
        id: 744,
        name_len: 35,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 14442,
        id: 585,
        name_len: 29,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 9946,
        id: 422,
        name_len: 25,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 14793,
        id: 598,
        name_len: 30,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 4011,
        id: 171,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 18248,
        id: 692,
        name_len: 40,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 7104,
        id: 298,
        name_len: 40,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 1675,
        id: 72,
        name_len: 31,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 5099,
        id: 220,
        name_len: 25,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 12911,
        id: 537,
        name_len: 48,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 13569,
        id: 557,
        name_len: 37,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 19825,
        id: 743,
        name_len: 21,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 9064,
        id: 381,
        name_len: 20,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 11137,
        id: 475,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 19623,
        id: 735,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 5662,
        id: 242,
        name_len: 30,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 8965,
        id: 377,
        name_len: 30,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 4620,
        id: 199,
        name_len: 14,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 11225,
        id: 479,
        name_len: 16,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 4722,
        id: 204,
        name_len: 32,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 1039,
        id: 45,
        name_len: 43,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 16235,
        id: 643,
        name_len: 43,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 2983,
        id: 126,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 18468,
        id: 698,
        name_len: 37,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 21672,
        id: 824,
        name_len: 12,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21886,
        id: 832,
        name_len: 31,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20547,
        id: 772,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 7747,
        id: 324,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 9297,
        id: 392,
        name_len: 11,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 5554,
        id: 237,
        name_len: 19,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 17734,
        id: 680,
        name_len: 39,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 7838,
        id: 329,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 20010,
        id: 749,
        name_len: 43,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 11835,
        id: 505,
        name_len: 64,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 10184,
        id: 433,
        name_len: 23,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 13747,
        id: 562,
        name_len: 41,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 7823,
        id: 328,
        name_len: 15,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 6622,
        id: 282,
        name_len: 34,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 18214,
        id: 691,
        name_len: 34,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 3642,
        id: 154,
        name_len: 32,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 18327,
        id: 694,
        name_len: 34,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 13533,
        id: 556,
        name_len: 36,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 11258,
        id: 481,
        name_len: 14,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 8457,
        id: 355,
        name_len: 15,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 12100,
        id: 512,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 18392,
        id: 696,
        name_len: 34,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 13111,
        id: 543,
        name_len: 42,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 10153,
        id: 432,
        name_len: 31,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20131,
        id: 753,
        name_len: 16,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 5842,
        id: 249,
        name_len: 21,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 9337,
        id: 394,
        name_len: 15,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20954,
        id: 790,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 7564,
        id: 317,
        name_len: 34,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 21120,
        id: 799,
        name_len: 42,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 9254,
        id: 390,
        name_len: 24,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 2950,
        id: 124,
        name_len: 15,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20569,
        id: 773,
        name_len: 15,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 6814,
        id: 289,
        name_len: 28,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 2827,
        id: 119,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 6842,
        id: 290,
        name_len: 29,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 14111,
        id: 574,
        name_len: 28,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 5635,
        id: 241,
        name_len: 27,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 9195,
        id: 387,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 9566,
        id: 404,
        name_len: 19,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20464,
        id: 769,
        name_len: 26,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21396,
        id: 811,
        name_len: 14,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 2438,
        id: 103,
        name_len: 39,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 6279,
        id: 267,
        name_len: 31,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 14166,
        id: 576,
        name_len: 31,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 17464,
        id: 675,
        name_len: 63,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 8065,
        id: 338,
        name_len: 25,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 19498,
        id: 730,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20652,
        id: 777,
        name_len: 12,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 20742,
        id: 782,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 7347,
        id: 307,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 215,
        id: 9,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 14364,
        id: 583,
        name_len: 38,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 19146,
        id: 719,
        name_len: 24,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 13984,
        id: 569,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 15226,
        id: 613,
        name_len: 25,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 6960,
        id: 294,
        name_len: 36,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 7794,
        id: 326,
        name_len: 15,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 8225,
        id: 345,
        name_len: 28,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 8495,
        id: 357,
        name_len: 31,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 10674,
        id: 454,
        name_len: 27,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 17860,
        id: 683,
        name_len: 44,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 12827,
        id: 535,
        name_len: 33,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 6515,
        id: 278,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 17527,
        id: 676,
        name_len: 66,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 12412,
        id: 521,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 9665,
        id: 409,
        name_len: 28,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 11774,
        id: 503,
        name_len: 29,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 2605,
        id: 109,
        name_len: 26,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 1151,
        id: 48,
        name_len: 21,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 12484,
        id: 524,
        name_len: 27,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 5151,
        id: 222,
        name_len: 25,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 4065,
        id: 174,
        name_len: 19,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 14889,
        id: 601,
        name_len: 32,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 7208,
        id: 301,
        name_len: 30,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3353,
        id: 142,
        name_len: 30,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 4934,
        id: 213,
        name_len: 16,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 1511,
        id: 64,
        name_len: 34,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 7621,
        id: 319,
        name_len: 30,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 8665,
        id: 364,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 5799,
        id: 247,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 9585,
        id: 405,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 13709,
        id: 561,
        name_len: 38,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21930,
        id: 834,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 15859,
        id: 635,
        name_len: 44,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 15561,
        id: 626,
        name_len: 28,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 1659,
        id: 71,
        name_len: 16,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 3705,
        id: 156,
        name_len: 27,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 193,
        id: 8,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 6788,
        id: 288,
        name_len: 26,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 14402,
        id: 584,
        name_len: 40,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 10563,
        id: 450,
        name_len: 19,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 9477,
        id: 400,
        name_len: 21,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 9413,
        id: 397,
        name_len: 29,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 1545,
        id: 65,
        name_len: 20,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 8253,
        id: 346,
        name_len: 36,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 18818,
        id: 707,
        name_len: 47,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 4132,
        id: 177,
        name_len: 27,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 4031,
        id: 172,
        name_len: 15,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 13182,
        id: 545,
        name_len: 29,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 15483,
        id: 623,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 11200,
        id: 478,
        name_len: 25,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21188,
        id: 801,
        name_len: 25,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 14498,
        id: 588,
        name_len: 22,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 17008,
        id: 663,
        name_len: 39,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 21427,
        id: 813,
        name_len: 37,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 21288,
        id: 806,
        name_len: 24,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 5618,
        id: 240,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 14139,
        id: 575,
        name_len: 27,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 614,
        id: 27,
        name_len: 32,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 4443,
        id: 191,
        name_len: 18,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 7598,
        id: 318,
        name_len: 23,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 16015,
        id: 638,
        name_len: 47,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 9882,
        id: 419,
        name_len: 30,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 18100,
        id: 688,
        name_len: 40,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 9786,
        id: 415,
        name_len: 28,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 2137,
        id: 88,
        name_len: 17,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 15345,
        id: 618,
        name_len: 33,
        scope: CommandScope::Device,
    },
    CommandRecord {
        name_offset: 17122,
        id: 666,
        name_len: 34,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 151,
        id: 6,
        name_len: 21,
        scope: CommandScope::Instance,
    },
    CommandRecord {
        name_offset: 0,
        id: u16::MAX,
        name_len: 0,
        scope: CommandScope::Global,
    },
    CommandRecord {
        name_offset: 8585,
        id: 360,
        name_len: 23,
        scope: CommandScope::Instance,
    },
];
pub(super) static COMMAND_DISPLACEMENTS: [u16; 512] = [
    2, 0, 1, 0, 0, 0, 1, 0, 7, 0, 1, 4, 1, 8, 2, 0, 0, 6, 0, 1, 4, 0, 0, 2, 0, 0, 0, 0, 3, 7, 0, 3,
    0, 0, 4, 0, 2, 0, 1, 6, 1, 1, 0, 2, 2, 10, 10, 9, 7, 1, 5, 4, 1, 11, 1, 0, 13, 1, 0, 1, 2, 1,
    0, 5, 2, 0, 0, 0, 0, 2, 0, 0, 2, 0, 0, 0, 0, 4, 0, 0, 0, 7, 0, 0, 0, 4, 9, 7, 3, 0, 0, 1, 8, 0,
    2, 0, 10, 2, 8, 4, 3, 6, 0, 1, 7, 4, 0, 10, 10, 0, 0, 4, 0, 0, 0, 0, 1, 10, 3, 0, 1, 0, 0, 2,
    0, 0, 9, 0, 4, 0, 0, 0, 0, 1, 4, 4, 7, 6, 3, 5, 3, 1, 0, 1, 4, 0, 1, 2, 8, 0, 0, 0, 0, 0, 0, 0,
    1, 4, 5, 0, 0, 7, 3, 0, 3, 2, 4, 2, 0, 1, 0, 7, 2, 0, 3, 0, 24, 1, 3, 11, 0, 1, 5, 0, 6, 0, 3,
    0, 0, 0, 5, 14, 0, 0, 5, 6, 0, 0, 8, 5, 0, 5, 1, 0, 2, 0, 2, 0, 0, 0, 3, 28, 15, 0, 3, 8, 2, 0,
    2, 1, 0, 0, 24, 1, 1, 0, 4, 13, 5, 5, 8, 1, 0, 0, 0, 0, 9, 0, 5, 5, 6, 13, 0, 0, 6, 0, 2, 0, 4,
    5, 5, 16, 0, 11, 4, 0, 0, 5, 6, 0, 0, 0, 3, 1, 4, 0, 0, 0, 1, 3, 0, 1, 3, 0, 2, 7, 7, 2, 10, 0,
    0, 4, 0, 10, 2, 0, 0, 2, 1, 0, 0, 0, 2, 0, 6, 0, 0, 12, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0,
    0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 2, 0, 0, 8, 3, 4, 1, 0, 1, 0, 0, 5, 0, 0, 1, 6, 0, 10, 1, 2, 0,
    6, 11, 0, 2, 0, 1, 0, 0, 0, 5, 0, 0, 7, 5, 4, 4, 0, 0, 0, 1, 1, 0, 0, 2, 0, 11, 0, 3, 2, 0, 0,
    2, 0, 2, 0, 2, 0, 0, 3, 6, 2, 3, 0, 3, 6, 4, 0, 2, 0, 2, 0, 0, 1, 2, 0, 0, 0, 0, 13, 0, 6, 6,
    4, 5, 2, 9, 0, 0, 11, 2, 16, 3, 0, 5, 0, 0, 5, 0, 4, 2, 0, 0, 11, 1, 5, 0, 1, 0, 1, 0, 0, 0, 0,
    8, 0, 0, 0, 8, 4, 0, 1, 1, 10, 1, 4, 5, 7, 0, 6, 0, 2, 1, 10, 14, 1, 9, 2, 0, 6, 1, 1, 0, 0,
    10, 5, 0, 5, 1, 0, 8, 7, 16, 24, 6, 3, 0, 2, 0, 0, 0, 2, 5, 0, 6, 0, 2, 4, 26, 2, 2, 0, 0, 0,
    0, 0, 13, 1, 1, 5, 9, 14, 1, 21, 1, 0, 20, 0, 0, 0, 5,
];
pub(super) static COMMAND_CORE_LEVELS: [u16; COMMAND_COUNT] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1024, 1024, 1024, 0, 1024, 0, 1024, 1025, 0, 0, 1024, 1025, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1024, 0, 1024, 1026, 0, 1027, 0, 0, 0, 0, 0, 0, 0, 0, 1024, 1028,
    0, 1024, 1028, 0, 0, 0, 1024, 0, 0, 0, 0, 0, 0, 0, 0, 1024, 1027, 0, 0, 1024, 1027, 0, 0, 0, 0,
    0, 0, 0, 1024, 1024, 1024, 0, 0, 0, 0, 0, 1024, 1027, 0, 1024, 1027, 0, 0, 1024, 1027, 0, 1024,
    1027, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1024, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1024, 1025,
    0, 0, 0, 0, 0, 1024, 0, 0, 1024, 0, 0, 1024, 1024, 0, 1026, 0, 0, 0, 1024, 0, 0, 0, 1026, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1024, 0, 1024, 1026, 0, 1027, 0, 0, 0, 0, 0,
    0, 0, 1024, 0, 0, 1024, 0, 0, 0, 1024, 1026, 0, 0, 1024, 1027, 0, 0, 0, 1024, 1028, 0, 0, 1028,
    1028, 0, 0, 1028, 1028, 0, 0, 1024, 1027, 0, 1024, 1024, 1027, 0, 0, 0, 0, 1024, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1027, 0, 1024, 0, 1027, 0, 1024, 1027, 0, 0, 0, 0, 0, 1027, 0,
    1027, 0, 1027, 0, 0, 0, 1025, 0, 0, 0, 0, 0, 1024, 1027, 0, 0, 0, 0, 0, 0, 1027, 0, 0, 1028, 0,
    0, 0, 1024, 0, 0, 0, 0, 0, 0, 0, 1027, 0, 0, 1027, 0, 0, 0, 0, 1027, 0, 0, 1028, 0, 1028, 0, 0,
    0, 0, 0, 1024, 1027, 0, 0, 1024, 1027, 0, 1024, 1027, 0, 1024, 0, 0, 1024, 0, 0, 0, 0, 1027, 0,
    0, 0, 0, 0, 0, 1024, 0, 0, 1024, 1027, 0, 0, 0, 0, 0, 0, 0, 1024, 1027, 0, 0, 0, 0, 0, 1028, 0,
    1028, 0, 0, 1028, 0, 0, 0, 0, 0, 0, 0, 0, 1024, 0, 1024, 1024, 1024, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1024, 1024, 1025, 0, 1024, 0, 0, 0, 1024, 0, 0, 1024, 1024, 0, 1024, 0, 0, 1024, 0, 1024, 0, 0,
    0, 1024, 0, 0, 0, 0, 0, 1024, 1024, 1027, 0, 1024, 0, 0, 1024, 1026, 0, 1024, 1025, 0, 0, 1024,
    0, 1024, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1024, 0, 1024, 1024,
    0, 0, 0, 0, 0, 0, 0, 0, 1024, 1024, 1025, 0, 1024, 1024, 0, 1024, 1024, 0, 1024, 1024, 0, 0, 0,
    1024, 0, 0, 1024, 0, 1024, 1024, 1027, 0, 1024, 1024, 1024, 1025, 0, 1024, 0, 0, 1024, 0, 0, 0,
    0, 0, 0, 0, 1024, 0, 1024, 1024, 1024, 1024, 1024, 1025, 1025, 0, 0, 0, 0, 1024, 0, 1024, 1024,
    1024, 1024, 0, 0, 0, 0, 0, 0, 0, 1026, 0, 0, 1024, 1025, 0, 1026, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 1025, 0, 0, 1027, 0, 0, 0, 0, 0, 1025, 0, 0, 0, 0, 1027, 0, 1027, 0, 1028,
    0, 1024, 1026, 0, 0, 1024, 1024, 1025, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1024, 0, 0, 0, 0, 1024, 0,
    0, 0, 0, 0, 0, 0, 0, 1024, 1025, 0, 0, 0, 1024, 1025, 0, 1024, 1028, 0, 0, 0, 0, 0, 0, 1024, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 1025, 0, 1025, 0, 0, 1025, 0, 0, 1024, 1025, 0, 1024, 1025, 0, 0, 1024, 1025, 0, 1024, 1025,
    0, 0, 0, 0, 1024, 1025, 0, 0, 0, 0, 0, 0, 1024, 1025, 0, 0, 1024, 1025, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 1027, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1024, 0, 0, 0, 0, 0, 0, 0, 1027, 0, 1024, 0, 0, 0, 0,
    0, 0, 0, 0, 1024, 1028, 0, 0, 0, 1026, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 1024, 0, 0, 1024, 1028, 0, 1024, 0, 0, 1024, 0, 0, 0, 0, 0, 0, 0,
    1024, 1027, 0, 1024, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1024, 1024, 1024, 1024, 1024, 0, 1026, 0, 0,
    0, 0, 0, 0, 1024, 0, 0, 0, 0, 0, 0, 0, 1027, 0, 0, 0, 1026, 0, 0, 1028, 0, 1025, 0, 0, 1024,
    1028, 0, 0, 1025, 0, 1024, 0, 0, 0, 1024, 0, 0, 1026, 0, 0, 0, 0, 0,
];
pub(super) static COMMAND_DEVICE_DISPATCH_OFFSETS: [u16; COMMAND_COUNT] = [
    u16::MAX,
    VK_ACQUIRE_FULL_SCREEN_EXCLUSIVE_MODE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_ACQUIRE_NEXT_IMAGE2KHR_DEVICE_DISPATCH_OFFSET,
    VK_ACQUIRE_NEXT_IMAGE_KHR_DEVICE_DISPATCH_OFFSET,
    VK_ACQUIRE_PERFORMANCE_CONFIGURATION_INTEL_DEVICE_DISPATCH_OFFSET,
    VK_ACQUIRE_PROFILING_LOCK_KHR_DEVICE_DISPATCH_OFFSET,
    u16::MAX,
    u16::MAX,
    VK_ALLOCATE_COMMAND_BUFFERS_DEVICE_DISPATCH_OFFSET,
    VK_ALLOCATE_DESCRIPTOR_SETS_DEVICE_DISPATCH_OFFSET,
    VK_ALLOCATE_MEMORY_DEVICE_DISPATCH_OFFSET,
    VK_ANTI_LAG_UPDATE_AMD_DEVICE_DISPATCH_OFFSET,
    VK_BEGIN_COMMAND_BUFFER_DEVICE_DISPATCH_OFFSET,
    VK_BIND_ACCELERATION_STRUCTURE_MEMORY_NV_DEVICE_DISPATCH_OFFSET,
    VK_BIND_BUFFER_MEMORY_DEVICE_DISPATCH_OFFSET,
    VK_BIND_BUFFER_MEMORY2_DEVICE_DISPATCH_OFFSET,
    VK_BIND_BUFFER_MEMORY2KHR_DEVICE_DISPATCH_OFFSET,
    VK_BIND_DATA_GRAPH_PIPELINE_SESSION_MEMORY_ARM_DEVICE_DISPATCH_OFFSET,
    VK_BIND_IMAGE_MEMORY_DEVICE_DISPATCH_OFFSET,
    VK_BIND_IMAGE_MEMORY2_DEVICE_DISPATCH_OFFSET,
    VK_BIND_IMAGE_MEMORY2KHR_DEVICE_DISPATCH_OFFSET,
    VK_BIND_OPTICAL_FLOW_SESSION_IMAGE_NV_DEVICE_DISPATCH_OFFSET,
    VK_BIND_TENSOR_MEMORY_ARM_DEVICE_DISPATCH_OFFSET,
    VK_BIND_VIDEO_SESSION_MEMORY_KHR_DEVICE_DISPATCH_OFFSET,
    VK_BUILD_ACCELERATION_STRUCTURES_KHR_DEVICE_DISPATCH_OFFSET,
    VK_BUILD_MICROMAPS_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CLEAR_SHADER_INSTRUMENTATION_METRICS_ARM_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BEGIN_CONDITIONAL_RENDERING2EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BEGIN_CONDITIONAL_RENDERING_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BEGIN_CUSTOM_RESOLVE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BEGIN_DEBUG_UTILS_LABEL_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BEGIN_GPA_SAMPLE_AMD_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BEGIN_GPA_SESSION_AMD_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BEGIN_PER_TILE_EXECUTION_QCOM_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BEGIN_QUERY_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BEGIN_QUERY_INDEXED_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BEGIN_RENDER_PASS_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BEGIN_RENDER_PASS2_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BEGIN_RENDER_PASS2KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BEGIN_RENDERING_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BEGIN_RENDERING_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BEGIN_SHADER_INSTRUMENTATION_ARM_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BEGIN_TRANSFORM_FEEDBACK2EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BEGIN_TRANSFORM_FEEDBACK_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BEGIN_VIDEO_CODING_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BIND_DESCRIPTOR_BUFFER_EMBEDDED_SAMPLERS2EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BIND_DESCRIPTOR_BUFFER_EMBEDDED_SAMPLERS_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BIND_DESCRIPTOR_BUFFERS_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BIND_DESCRIPTOR_SETS_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BIND_DESCRIPTOR_SETS2_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BIND_DESCRIPTOR_SETS2KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BIND_INDEX_BUFFER_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BIND_INDEX_BUFFER2_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BIND_INDEX_BUFFER2KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BIND_INDEX_BUFFER3KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BIND_INVOCATION_MASK_HUAWEI_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BIND_PIPELINE_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BIND_PIPELINE_SHADER_GROUP_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BIND_RESOURCE_HEAP_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BIND_SAMPLER_HEAP_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BIND_SHADERS_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BIND_SHADING_RATE_IMAGE_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BIND_TILE_MEMORY_QCOM_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BIND_TRANSFORM_FEEDBACK_BUFFERS2EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BIND_TRANSFORM_FEEDBACK_BUFFERS_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BIND_VERTEX_BUFFERS_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BIND_VERTEX_BUFFERS2_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BIND_VERTEX_BUFFERS2EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BIND_VERTEX_BUFFERS3KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BLIT_IMAGE_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BLIT_IMAGE2_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BLIT_IMAGE2KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BUILD_ACCELERATION_STRUCTURE_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BUILD_ACCELERATION_STRUCTURES_INDIRECT_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BUILD_ACCELERATION_STRUCTURES_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BUILD_CLUSTER_ACCELERATION_STRUCTURE_INDIRECT_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BUILD_MICROMAPS_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_BUILD_PARTITIONED_ACCELERATION_STRUCTURES_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_CLEAR_ATTACHMENTS_DEVICE_DISPATCH_OFFSET,
    VK_CMD_CLEAR_COLOR_IMAGE_DEVICE_DISPATCH_OFFSET,
    VK_CMD_CLEAR_DEPTH_STENCIL_IMAGE_DEVICE_DISPATCH_OFFSET,
    VK_CMD_CONTROL_VIDEO_CODING_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_CONVERT_COOPERATIVE_VECTOR_MATRIX_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_COPY_ACCELERATION_STRUCTURE_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_COPY_ACCELERATION_STRUCTURE_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_COPY_ACCELERATION_STRUCTURE_TO_MEMORY_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_COPY_BUFFER_DEVICE_DISPATCH_OFFSET,
    VK_CMD_COPY_BUFFER2_DEVICE_DISPATCH_OFFSET,
    VK_CMD_COPY_BUFFER2KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_COPY_BUFFER_TO_IMAGE_DEVICE_DISPATCH_OFFSET,
    VK_CMD_COPY_BUFFER_TO_IMAGE2_DEVICE_DISPATCH_OFFSET,
    VK_CMD_COPY_BUFFER_TO_IMAGE2KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_COPY_GPA_SESSION_RESULTS_AMD_DEVICE_DISPATCH_OFFSET,
    VK_CMD_COPY_IMAGE_DEVICE_DISPATCH_OFFSET,
    VK_CMD_COPY_IMAGE2_DEVICE_DISPATCH_OFFSET,
    VK_CMD_COPY_IMAGE2KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_COPY_IMAGE_TO_BUFFER_DEVICE_DISPATCH_OFFSET,
    VK_CMD_COPY_IMAGE_TO_BUFFER2_DEVICE_DISPATCH_OFFSET,
    VK_CMD_COPY_IMAGE_TO_BUFFER2KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_COPY_IMAGE_TO_MEMORY_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_COPY_MEMORY_INDIRECT_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_COPY_MEMORY_INDIRECT_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_COPY_MEMORY_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_COPY_MEMORY_TO_ACCELERATION_STRUCTURE_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_COPY_MEMORY_TO_IMAGE_INDIRECT_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_COPY_MEMORY_TO_IMAGE_INDIRECT_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_COPY_MEMORY_TO_IMAGE_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_COPY_MEMORY_TO_MICROMAP_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_COPY_MICROMAP_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_COPY_MICROMAP_TO_MEMORY_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_COPY_QUERY_POOL_RESULTS_DEVICE_DISPATCH_OFFSET,
    VK_CMD_COPY_QUERY_POOL_RESULTS_TO_MEMORY_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_COPY_TENSOR_ARM_DEVICE_DISPATCH_OFFSET,
    VK_CMD_CU_LAUNCH_KERNEL_NVX_DEVICE_DISPATCH_OFFSET,
    VK_CMD_CUDA_LAUNCH_KERNEL_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DEBUG_MARKER_BEGIN_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DEBUG_MARKER_END_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DEBUG_MARKER_INSERT_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DECODE_VIDEO_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DECOMPRESS_MEMORY_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DECOMPRESS_MEMORY_INDIRECT_COUNT_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DECOMPRESS_MEMORY_INDIRECT_COUNT_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DECOMPRESS_MEMORY_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DISPATCH_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DISPATCH_BASE_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DISPATCH_BASE_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DISPATCH_DATA_GRAPH_ARM_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DISPATCH_GRAPH_AMDX_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DISPATCH_GRAPH_INDIRECT_AMDX_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DISPATCH_GRAPH_INDIRECT_COUNT_AMDX_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DISPATCH_INDIRECT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DISPATCH_INDIRECT2KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DISPATCH_TILE_QCOM_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DRAW_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DRAW_CLUSTER_HUAWEI_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DRAW_CLUSTER_INDIRECT_HUAWEI_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DRAW_INDEXED_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DRAW_INDEXED_INDIRECT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DRAW_INDEXED_INDIRECT2KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DRAW_INDEXED_INDIRECT_COUNT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DRAW_INDEXED_INDIRECT_COUNT2KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DRAW_INDEXED_INDIRECT_COUNT_AMD_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DRAW_INDEXED_INDIRECT_COUNT_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DRAW_INDIRECT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DRAW_INDIRECT2KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DRAW_INDIRECT_BYTE_COUNT2EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DRAW_INDIRECT_BYTE_COUNT_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DRAW_INDIRECT_COUNT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DRAW_INDIRECT_COUNT2KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DRAW_INDIRECT_COUNT_AMD_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DRAW_INDIRECT_COUNT_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DRAW_MESH_TASKS_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DRAW_MESH_TASKS_INDIRECT2EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DRAW_MESH_TASKS_INDIRECT_COUNT2EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DRAW_MESH_TASKS_INDIRECT_COUNT_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DRAW_MESH_TASKS_INDIRECT_COUNT_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DRAW_MESH_TASKS_INDIRECT_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DRAW_MESH_TASKS_INDIRECT_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DRAW_MESH_TASKS_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DRAW_MULTI_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_DRAW_MULTI_INDEXED_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_ENCODE_VIDEO_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_END_CONDITIONAL_RENDERING_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_END_DEBUG_UTILS_LABEL_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_END_GPA_SAMPLE_AMD_DEVICE_DISPATCH_OFFSET,
    VK_CMD_END_GPA_SESSION_AMD_DEVICE_DISPATCH_OFFSET,
    VK_CMD_END_PER_TILE_EXECUTION_QCOM_DEVICE_DISPATCH_OFFSET,
    VK_CMD_END_QUERY_DEVICE_DISPATCH_OFFSET,
    VK_CMD_END_QUERY_INDEXED_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_END_RENDER_PASS_DEVICE_DISPATCH_OFFSET,
    VK_CMD_END_RENDER_PASS2_DEVICE_DISPATCH_OFFSET,
    VK_CMD_END_RENDER_PASS2KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_END_RENDERING_DEVICE_DISPATCH_OFFSET,
    VK_CMD_END_RENDERING2EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_END_RENDERING2KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_END_RENDERING_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_END_SHADER_INSTRUMENTATION_ARM_DEVICE_DISPATCH_OFFSET,
    VK_CMD_END_TRANSFORM_FEEDBACK2EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_END_TRANSFORM_FEEDBACK_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_END_VIDEO_CODING_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_EXECUTE_COMMANDS_DEVICE_DISPATCH_OFFSET,
    VK_CMD_EXECUTE_GENERATED_COMMANDS_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_EXECUTE_GENERATED_COMMANDS_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_FILL_BUFFER_DEVICE_DISPATCH_OFFSET,
    VK_CMD_FILL_MEMORY_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_INITIALIZE_GRAPH_SCRATCH_MEMORY_AMDX_DEVICE_DISPATCH_OFFSET,
    VK_CMD_INSERT_DEBUG_UTILS_LABEL_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_NEXT_SUBPASS_DEVICE_DISPATCH_OFFSET,
    VK_CMD_NEXT_SUBPASS2_DEVICE_DISPATCH_OFFSET,
    VK_CMD_NEXT_SUBPASS2KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_OPTICAL_FLOW_EXECUTE_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_PIPELINE_BARRIER_DEVICE_DISPATCH_OFFSET,
    VK_CMD_PIPELINE_BARRIER2_DEVICE_DISPATCH_OFFSET,
    VK_CMD_PIPELINE_BARRIER2KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_PREPROCESS_GENERATED_COMMANDS_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_PREPROCESS_GENERATED_COMMANDS_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_PUSH_CONSTANTS_DEVICE_DISPATCH_OFFSET,
    VK_CMD_PUSH_CONSTANTS2_DEVICE_DISPATCH_OFFSET,
    VK_CMD_PUSH_CONSTANTS2KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_PUSH_DATA_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_PUSH_DESCRIPTOR_SET_DEVICE_DISPATCH_OFFSET,
    VK_CMD_PUSH_DESCRIPTOR_SET2_DEVICE_DISPATCH_OFFSET,
    VK_CMD_PUSH_DESCRIPTOR_SET2KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_PUSH_DESCRIPTOR_SET_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_DEVICE_DISPATCH_OFFSET,
    VK_CMD_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE2_DEVICE_DISPATCH_OFFSET,
    VK_CMD_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE2KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_RESET_EVENT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_RESET_EVENT2_DEVICE_DISPATCH_OFFSET,
    VK_CMD_RESET_EVENT2KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_RESET_QUERY_POOL_DEVICE_DISPATCH_OFFSET,
    VK_CMD_RESOLVE_IMAGE_DEVICE_DISPATCH_OFFSET,
    VK_CMD_RESOLVE_IMAGE2_DEVICE_DISPATCH_OFFSET,
    VK_CMD_RESOLVE_IMAGE2KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_ALPHA_TO_COVERAGE_ENABLE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_ALPHA_TO_ONE_ENABLE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_ATTACHMENT_FEEDBACK_LOOP_ENABLE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_BLEND_CONSTANTS_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_CHECKPOINT_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_COARSE_SAMPLE_ORDER_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_COLOR_BLEND_ADVANCED_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_COLOR_BLEND_ENABLE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_COLOR_BLEND_EQUATION_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_COLOR_WRITE_ENABLE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_COLOR_WRITE_MASK_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_COMPUTE_OCCUPANCY_PRIORITY_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_CONSERVATIVE_RASTERIZATION_MODE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_COVERAGE_MODULATION_MODE_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_COVERAGE_MODULATION_TABLE_ENABLE_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_COVERAGE_MODULATION_TABLE_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_COVERAGE_REDUCTION_MODE_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_COVERAGE_TO_COLOR_ENABLE_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_COVERAGE_TO_COLOR_LOCATION_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_CULL_MODE_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_CULL_MODE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_DEPTH_BIAS_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_DEPTH_BIAS2EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_DEPTH_BIAS_ENABLE_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_DEPTH_BIAS_ENABLE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_DEPTH_BOUNDS_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_DEPTH_BOUNDS_TEST_ENABLE_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_DEPTH_BOUNDS_TEST_ENABLE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_DEPTH_CLAMP_ENABLE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_DEPTH_CLAMP_RANGE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_DEPTH_CLIP_ENABLE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_DEPTH_CLIP_NEGATIVE_ONE_TO_ONE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_DEPTH_COMPARE_OP_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_DEPTH_COMPARE_OP_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_DEPTH_TEST_ENABLE_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_DEPTH_TEST_ENABLE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_DEPTH_WRITE_ENABLE_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_DEPTH_WRITE_ENABLE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_DESCRIPTOR_BUFFER_OFFSETS2EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_DESCRIPTOR_BUFFER_OFFSETS_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_DEVICE_MASK_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_DEVICE_MASK_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_DISCARD_RECTANGLE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_DISCARD_RECTANGLE_ENABLE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_DISCARD_RECTANGLE_MODE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_DISPATCH_PARAMETERS_ARM_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_EVENT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_EVENT2_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_EVENT2KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_EXCLUSIVE_SCISSOR_ENABLE_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_EXCLUSIVE_SCISSOR_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_EXTRA_PRIMITIVE_OVERESTIMATION_SIZE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_FRAGMENT_SHADING_RATE_ENUM_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_FRAGMENT_SHADING_RATE_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_FRONT_FACE_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_FRONT_FACE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_LINE_RASTERIZATION_MODE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_LINE_STIPPLE_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_LINE_STIPPLE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_LINE_STIPPLE_ENABLE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_LINE_STIPPLE_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_LINE_WIDTH_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_LOGIC_OP_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_LOGIC_OP_ENABLE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_PATCH_CONTROL_POINTS_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_PERFORMANCE_MARKER_INTEL_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_PERFORMANCE_OVERRIDE_INTEL_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_PERFORMANCE_STREAM_MARKER_INTEL_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_POLYGON_MODE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_PRIMITIVE_RESTART_ENABLE_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_PRIMITIVE_RESTART_ENABLE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_PRIMITIVE_RESTART_INDEX_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_PRIMITIVE_TOPOLOGY_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_PRIMITIVE_TOPOLOGY_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_PROVOKING_VERTEX_MODE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_RASTERIZATION_SAMPLES_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_RASTERIZATION_STREAM_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_RASTERIZER_DISCARD_ENABLE_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_RASTERIZER_DISCARD_ENABLE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_RAY_TRACING_PIPELINE_STACK_SIZE_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_RENDERING_ATTACHMENT_LOCATIONS_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_RENDERING_ATTACHMENT_LOCATIONS_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_RENDERING_INPUT_ATTACHMENT_INDICES_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_RENDERING_INPUT_ATTACHMENT_INDICES_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_REPRESENTATIVE_FRAGMENT_TEST_ENABLE_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_SAMPLE_LOCATIONS_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_SAMPLE_LOCATIONS_ENABLE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_SAMPLE_MASK_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_SCISSOR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_SCISSOR_WITH_COUNT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_SCISSOR_WITH_COUNT_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_SHADING_RATE_IMAGE_ENABLE_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_STENCIL_COMPARE_MASK_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_STENCIL_OP_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_STENCIL_OP_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_STENCIL_REFERENCE_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_STENCIL_TEST_ENABLE_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_STENCIL_TEST_ENABLE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_STENCIL_WRITE_MASK_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_TESSELLATION_DOMAIN_ORIGIN_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_VERTEX_INPUT_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_VIEWPORT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_VIEWPORT_SHADING_RATE_PALETTE_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_VIEWPORT_SWIZZLE_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_VIEWPORT_W_SCALING_ENABLE_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_VIEWPORT_W_SCALING_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_VIEWPORT_WITH_COUNT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SET_VIEWPORT_WITH_COUNT_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_SUBPASS_SHADING_HUAWEI_DEVICE_DISPATCH_OFFSET,
    VK_CMD_TRACE_RAYS_INDIRECT2KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_TRACE_RAYS_INDIRECT_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_TRACE_RAYS_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_TRACE_RAYS_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_UPDATE_BUFFER_DEVICE_DISPATCH_OFFSET,
    VK_CMD_UPDATE_MEMORY_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_UPDATE_PIPELINE_INDIRECT_BUFFER_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_WAIT_EVENTS_DEVICE_DISPATCH_OFFSET,
    VK_CMD_WAIT_EVENTS2_DEVICE_DISPATCH_OFFSET,
    VK_CMD_WAIT_EVENTS2KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_WRITE_ACCELERATION_STRUCTURES_PROPERTIES_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CMD_WRITE_ACCELERATION_STRUCTURES_PROPERTIES_NV_DEVICE_DISPATCH_OFFSET,
    VK_CMD_WRITE_BUFFER_MARKER2AMD_DEVICE_DISPATCH_OFFSET,
    VK_CMD_WRITE_BUFFER_MARKER_AMD_DEVICE_DISPATCH_OFFSET,
    VK_CMD_WRITE_MARKER_TO_MEMORY_AMD_DEVICE_DISPATCH_OFFSET,
    VK_CMD_WRITE_MICROMAPS_PROPERTIES_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CMD_WRITE_TIMESTAMP_DEVICE_DISPATCH_OFFSET,
    VK_CMD_WRITE_TIMESTAMP2_DEVICE_DISPATCH_OFFSET,
    VK_CMD_WRITE_TIMESTAMP2KHR_DEVICE_DISPATCH_OFFSET,
    VK_COMPILE_DEFERRED_NV_DEVICE_DISPATCH_OFFSET,
    VK_CONVERT_COOPERATIVE_VECTOR_MATRIX_NV_DEVICE_DISPATCH_OFFSET,
    VK_COPY_ACCELERATION_STRUCTURE_KHR_DEVICE_DISPATCH_OFFSET,
    VK_COPY_ACCELERATION_STRUCTURE_TO_MEMORY_KHR_DEVICE_DISPATCH_OFFSET,
    VK_COPY_IMAGE_TO_IMAGE_DEVICE_DISPATCH_OFFSET,
    VK_COPY_IMAGE_TO_IMAGE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_COPY_IMAGE_TO_MEMORY_DEVICE_DISPATCH_OFFSET,
    VK_COPY_IMAGE_TO_MEMORY_EXT_DEVICE_DISPATCH_OFFSET,
    VK_COPY_MEMORY_TO_ACCELERATION_STRUCTURE_KHR_DEVICE_DISPATCH_OFFSET,
    VK_COPY_MEMORY_TO_IMAGE_DEVICE_DISPATCH_OFFSET,
    VK_COPY_MEMORY_TO_IMAGE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_COPY_MEMORY_TO_MICROMAP_EXT_DEVICE_DISPATCH_OFFSET,
    VK_COPY_MICROMAP_EXT_DEVICE_DISPATCH_OFFSET,
    VK_COPY_MICROMAP_TO_MEMORY_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_ACCELERATION_STRUCTURE2KHR_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_ACCELERATION_STRUCTURE_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_ACCELERATION_STRUCTURE_NV_DEVICE_DISPATCH_OFFSET,
    u16::MAX,
    VK_CREATE_BUFFER_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_BUFFER_COLLECTION_FUCHSIA_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_BUFFER_VIEW_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_COMMAND_POOL_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_COMPUTE_PIPELINES_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_CU_FUNCTION_NVX_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_CU_MODULE_NVX_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_CUDA_FUNCTION_NV_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_CUDA_MODULE_NV_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_DATA_GRAPH_PIPELINE_SESSION_ARM_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_DATA_GRAPH_PIPELINES_ARM_DEVICE_DISPATCH_OFFSET,
    u16::MAX,
    u16::MAX,
    VK_CREATE_DEFERRED_OPERATION_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_DESCRIPTOR_POOL_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_DESCRIPTOR_SET_LAYOUT_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_DESCRIPTOR_UPDATE_TEMPLATE_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_DESCRIPTOR_UPDATE_TEMPLATE_KHR_DEVICE_DISPATCH_OFFSET,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    VK_CREATE_EVENT_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_EXECUTION_GRAPH_PIPELINES_AMDX_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_EXTERNAL_COMPUTE_QUEUE_NV_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_FENCE_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_FRAMEBUFFER_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_GPA_SESSION_AMD_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_GRAPHICS_PIPELINES_DEVICE_DISPATCH_OFFSET,
    u16::MAX,
    u16::MAX,
    VK_CREATE_IMAGE_DEVICE_DISPATCH_OFFSET,
    u16::MAX,
    VK_CREATE_IMAGE_VIEW_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_INDIRECT_COMMANDS_LAYOUT_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_INDIRECT_COMMANDS_LAYOUT_NV_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_INDIRECT_EXECUTION_SET_EXT_DEVICE_DISPATCH_OFFSET,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    VK_CREATE_MICROMAP_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_OPTICAL_FLOW_SESSION_NV_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_PIPELINE_BINARIES_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_PIPELINE_CACHE_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_PIPELINE_LAYOUT_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_PRIVATE_DATA_SLOT_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_PRIVATE_DATA_SLOT_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_QUERY_POOL_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_RAY_TRACING_PIPELINES_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_RAY_TRACING_PIPELINES_NV_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_RENDER_PASS_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_RENDER_PASS2_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_RENDER_PASS2KHR_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_SAMPLER_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_SAMPLER_YCBCR_CONVERSION_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_SAMPLER_YCBCR_CONVERSION_KHR_DEVICE_DISPATCH_OFFSET,
    u16::MAX,
    VK_CREATE_SEMAPHORE_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_SHADER_INSTRUMENTATION_ARM_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_SHADER_MODULE_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_SHADERS_EXT_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_SHARED_SWAPCHAINS_KHR_DEVICE_DISPATCH_OFFSET,
    u16::MAX,
    u16::MAX,
    VK_CREATE_SWAPCHAIN_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_TENSOR_ARM_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_TENSOR_VIEW_ARM_DEVICE_DISPATCH_OFFSET,
    u16::MAX,
    VK_CREATE_VALIDATION_CACHE_EXT_DEVICE_DISPATCH_OFFSET,
    u16::MAX,
    VK_CREATE_VIDEO_SESSION_KHR_DEVICE_DISPATCH_OFFSET,
    VK_CREATE_VIDEO_SESSION_PARAMETERS_KHR_DEVICE_DISPATCH_OFFSET,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    VK_DEBUG_MARKER_SET_OBJECT_NAME_EXT_DEVICE_DISPATCH_OFFSET,
    VK_DEBUG_MARKER_SET_OBJECT_TAG_EXT_DEVICE_DISPATCH_OFFSET,
    u16::MAX,
    VK_DEFERRED_OPERATION_JOIN_KHR_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_ACCELERATION_STRUCTURE_KHR_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_ACCELERATION_STRUCTURE_NV_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_BUFFER_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_BUFFER_COLLECTION_FUCHSIA_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_BUFFER_VIEW_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_COMMAND_POOL_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_CU_FUNCTION_NVX_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_CU_MODULE_NVX_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_CUDA_FUNCTION_NV_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_CUDA_MODULE_NV_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_DATA_GRAPH_PIPELINE_SESSION_ARM_DEVICE_DISPATCH_OFFSET,
    u16::MAX,
    u16::MAX,
    VK_DESTROY_DEFERRED_OPERATION_KHR_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_DESCRIPTOR_POOL_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_DESCRIPTOR_SET_LAYOUT_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_DESCRIPTOR_UPDATE_TEMPLATE_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_DESCRIPTOR_UPDATE_TEMPLATE_KHR_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_DEVICE_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_EVENT_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_EXTERNAL_COMPUTE_QUEUE_NV_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_FENCE_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_FRAMEBUFFER_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_GPA_SESSION_AMD_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_IMAGE_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_IMAGE_VIEW_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_INDIRECT_COMMANDS_LAYOUT_EXT_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_INDIRECT_COMMANDS_LAYOUT_NV_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_INDIRECT_EXECUTION_SET_EXT_DEVICE_DISPATCH_OFFSET,
    u16::MAX,
    VK_DESTROY_MICROMAP_EXT_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_OPTICAL_FLOW_SESSION_NV_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_PIPELINE_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_PIPELINE_BINARY_KHR_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_PIPELINE_CACHE_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_PIPELINE_LAYOUT_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_PRIVATE_DATA_SLOT_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_PRIVATE_DATA_SLOT_EXT_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_QUERY_POOL_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_RENDER_PASS_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_SAMPLER_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_SAMPLER_YCBCR_CONVERSION_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_SAMPLER_YCBCR_CONVERSION_KHR_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_SEMAPHORE_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_SHADER_EXT_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_SHADER_INSTRUMENTATION_ARM_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_SHADER_MODULE_DEVICE_DISPATCH_OFFSET,
    u16::MAX,
    VK_DESTROY_SWAPCHAIN_KHR_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_TENSOR_ARM_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_TENSOR_VIEW_ARM_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_VALIDATION_CACHE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_VIDEO_SESSION_KHR_DEVICE_DISPATCH_OFFSET,
    VK_DESTROY_VIDEO_SESSION_PARAMETERS_KHR_DEVICE_DISPATCH_OFFSET,
    VK_DEVICE_WAIT_IDLE_DEVICE_DISPATCH_OFFSET,
    VK_DISPLAY_POWER_CONTROL_EXT_DEVICE_DISPATCH_OFFSET,
    VK_END_COMMAND_BUFFER_DEVICE_DISPATCH_OFFSET,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    VK_EXPORT_METAL_OBJECTS_EXT_DEVICE_DISPATCH_OFFSET,
    VK_FLUSH_MAPPED_MEMORY_RANGES_DEVICE_DISPATCH_OFFSET,
    VK_FREE_COMMAND_BUFFERS_DEVICE_DISPATCH_OFFSET,
    VK_FREE_DESCRIPTOR_SETS_DEVICE_DISPATCH_OFFSET,
    VK_FREE_MEMORY_DEVICE_DISPATCH_OFFSET,
    VK_GET_ACCELERATION_STRUCTURE_BUILD_SIZES_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_ACCELERATION_STRUCTURE_DEVICE_ADDRESS_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_ACCELERATION_STRUCTURE_HANDLE_NV_DEVICE_DISPATCH_OFFSET,
    VK_GET_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_NV_DEVICE_DISPATCH_OFFSET,
    VK_GET_ACCELERATION_STRUCTURE_OPAQUE_CAPTURE_DESCRIPTOR_DATA_EXT_DEVICE_DISPATCH_OFFSET,
    VK_GET_ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID_DEVICE_DISPATCH_OFFSET,
    VK_GET_BUFFER_COLLECTION_PROPERTIES_FUCHSIA_DEVICE_DISPATCH_OFFSET,
    VK_GET_BUFFER_DEVICE_ADDRESS_DEVICE_DISPATCH_OFFSET,
    VK_GET_BUFFER_DEVICE_ADDRESS_EXT_DEVICE_DISPATCH_OFFSET,
    VK_GET_BUFFER_DEVICE_ADDRESS_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_BUFFER_MEMORY_REQUIREMENTS_DEVICE_DISPATCH_OFFSET,
    VK_GET_BUFFER_MEMORY_REQUIREMENTS2_DEVICE_DISPATCH_OFFSET,
    VK_GET_BUFFER_MEMORY_REQUIREMENTS2KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_BUFFER_OPAQUE_CAPTURE_ADDRESS_DEVICE_DISPATCH_OFFSET,
    VK_GET_BUFFER_OPAQUE_CAPTURE_ADDRESS_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_BUFFER_OPAQUE_CAPTURE_DESCRIPTOR_DATA_EXT_DEVICE_DISPATCH_OFFSET,
    VK_GET_CALIBRATED_TIMESTAMPS_EXT_DEVICE_DISPATCH_OFFSET,
    VK_GET_CALIBRATED_TIMESTAMPS_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_CLUSTER_ACCELERATION_STRUCTURE_BUILD_SIZES_NV_DEVICE_DISPATCH_OFFSET,
    VK_GET_CUDA_MODULE_CACHE_NV_DEVICE_DISPATCH_OFFSET,
    VK_GET_DATA_GRAPH_PIPELINE_AVAILABLE_PROPERTIES_ARM_DEVICE_DISPATCH_OFFSET,
    VK_GET_DATA_GRAPH_PIPELINE_PROPERTIES_ARM_DEVICE_DISPATCH_OFFSET,
    VK_GET_DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENTS_ARM_DEVICE_DISPATCH_OFFSET,
    VK_GET_DATA_GRAPH_PIPELINE_SESSION_MEMORY_REQUIREMENTS_ARM_DEVICE_DISPATCH_OFFSET,
    VK_GET_DEFERRED_OPERATION_MAX_CONCURRENCY_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_DEFERRED_OPERATION_RESULT_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_DESCRIPTOR_EXT_DEVICE_DISPATCH_OFFSET,
    VK_GET_DESCRIPTOR_SET_HOST_MAPPING_VALVE_DEVICE_DISPATCH_OFFSET,
    VK_GET_DESCRIPTOR_SET_LAYOUT_BINDING_OFFSET_EXT_DEVICE_DISPATCH_OFFSET,
    VK_GET_DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE_DEVICE_DISPATCH_OFFSET,
    VK_GET_DESCRIPTOR_SET_LAYOUT_SIZE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_GET_DESCRIPTOR_SET_LAYOUT_SUPPORT_DEVICE_DISPATCH_OFFSET,
    VK_GET_DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_DEVICE_ACCELERATION_STRUCTURE_COMPATIBILITY_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_DEVICE_BUFFER_MEMORY_REQUIREMENTS_DEVICE_DISPATCH_OFFSET,
    VK_GET_DEVICE_BUFFER_MEMORY_REQUIREMENTS_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_DEVICE_COMBINED_IMAGE_SAMPLER_INDEX_NVX_DEVICE_DISPATCH_OFFSET,
    VK_GET_DEVICE_FAULT_DEBUG_INFO_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_DEVICE_FAULT_INFO_EXT_DEVICE_DISPATCH_OFFSET,
    VK_GET_DEVICE_FAULT_REPORTS_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_DEVICE_GROUP_PEER_MEMORY_FEATURES_DEVICE_DISPATCH_OFFSET,
    VK_GET_DEVICE_GROUP_PEER_MEMORY_FEATURES_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_DEVICE_GROUP_SURFACE_PRESENT_MODES2EXT_DEVICE_DISPATCH_OFFSET,
    VK_GET_DEVICE_GROUP_SURFACE_PRESENT_MODES_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_DEVICE_IMAGE_MEMORY_REQUIREMENTS_DEVICE_DISPATCH_OFFSET,
    VK_GET_DEVICE_IMAGE_MEMORY_REQUIREMENTS_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_DEVICE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_DEVICE_DISPATCH_OFFSET,
    VK_GET_DEVICE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_DEVICE_IMAGE_SUBRESOURCE_LAYOUT_DEVICE_DISPATCH_OFFSET,
    VK_GET_DEVICE_IMAGE_SUBRESOURCE_LAYOUT_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_DEVICE_MEMORY_COMMITMENT_DEVICE_DISPATCH_OFFSET,
    VK_GET_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_DEVICE_DISPATCH_OFFSET,
    VK_GET_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_DEVICE_MICROMAP_COMPATIBILITY_EXT_DEVICE_DISPATCH_OFFSET,
    VK_GET_DEVICE_PROC_ADDR_DEVICE_DISPATCH_OFFSET,
    VK_GET_DEVICE_QUEUE_DEVICE_DISPATCH_OFFSET,
    VK_GET_DEVICE_QUEUE2_DEVICE_DISPATCH_OFFSET,
    VK_GET_DEVICE_SUBPASS_SHADING_MAX_WORKGROUP_SIZE_HUAWEI_DEVICE_DISPATCH_OFFSET,
    VK_GET_DEVICE_TENSOR_MEMORY_REQUIREMENTS_ARM_DEVICE_DISPATCH_OFFSET,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    VK_GET_DYNAMIC_RENDERING_TILE_PROPERTIES_QCOM_DEVICE_DISPATCH_OFFSET,
    VK_GET_ENCODED_VIDEO_SESSION_PARAMETERS_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_EVENT_STATUS_DEVICE_DISPATCH_OFFSET,
    VK_GET_EXECUTION_GRAPH_PIPELINE_NODE_INDEX_AMDX_DEVICE_DISPATCH_OFFSET,
    VK_GET_EXECUTION_GRAPH_PIPELINE_SCRATCH_SIZE_AMDX_DEVICE_DISPATCH_OFFSET,
    VK_GET_EXTERNAL_COMPUTE_QUEUE_DATA_NV_DEVICE_DISPATCH_OFFSET,
    VK_GET_FENCE_FD_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_FENCE_STATUS_DEVICE_DISPATCH_OFFSET,
    VK_GET_FENCE_WIN32HANDLE_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_FRAMEBUFFER_TILE_PROPERTIES_QCOM_DEVICE_DISPATCH_OFFSET,
    VK_GET_GENERATED_COMMANDS_MEMORY_REQUIREMENTS_EXT_DEVICE_DISPATCH_OFFSET,
    VK_GET_GENERATED_COMMANDS_MEMORY_REQUIREMENTS_NV_DEVICE_DISPATCH_OFFSET,
    VK_GET_GPA_DEVICE_CLOCK_INFO_AMD_DEVICE_DISPATCH_OFFSET,
    VK_GET_GPA_SESSION_RESULTS_AMD_DEVICE_DISPATCH_OFFSET,
    VK_GET_GPA_SESSION_STATUS_AMD_DEVICE_DISPATCH_OFFSET,
    VK_GET_IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT_DEVICE_DISPATCH_OFFSET,
    VK_GET_IMAGE_MEMORY_REQUIREMENTS_DEVICE_DISPATCH_OFFSET,
    VK_GET_IMAGE_MEMORY_REQUIREMENTS2_DEVICE_DISPATCH_OFFSET,
    VK_GET_IMAGE_MEMORY_REQUIREMENTS2KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_IMAGE_OPAQUE_CAPTURE_DATA_EXT_DEVICE_DISPATCH_OFFSET,
    VK_GET_IMAGE_OPAQUE_CAPTURE_DESCRIPTOR_DATA_EXT_DEVICE_DISPATCH_OFFSET,
    VK_GET_IMAGE_SPARSE_MEMORY_REQUIREMENTS_DEVICE_DISPATCH_OFFSET,
    VK_GET_IMAGE_SPARSE_MEMORY_REQUIREMENTS2_DEVICE_DISPATCH_OFFSET,
    VK_GET_IMAGE_SPARSE_MEMORY_REQUIREMENTS2KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_IMAGE_SUBRESOURCE_LAYOUT_DEVICE_DISPATCH_OFFSET,
    VK_GET_IMAGE_SUBRESOURCE_LAYOUT2_DEVICE_DISPATCH_OFFSET,
    VK_GET_IMAGE_SUBRESOURCE_LAYOUT2EXT_DEVICE_DISPATCH_OFFSET,
    VK_GET_IMAGE_SUBRESOURCE_LAYOUT2KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_IMAGE_VIEW_ADDRESS_NVX_DEVICE_DISPATCH_OFFSET,
    VK_GET_IMAGE_VIEW_HANDLE64NVX_DEVICE_DISPATCH_OFFSET,
    VK_GET_IMAGE_VIEW_HANDLE_NVX_DEVICE_DISPATCH_OFFSET,
    VK_GET_IMAGE_VIEW_OPAQUE_CAPTURE_DESCRIPTOR_DATA_EXT_DEVICE_DISPATCH_OFFSET,
    u16::MAX,
    VK_GET_LATENCY_TIMINGS_LEGACY_NV_DEVICE_DISPATCH_OFFSET,
    VK_GET_LATENCY_TIMINGS_NV_DEVICE_DISPATCH_OFFSET,
    VK_GET_MEMORY_ANDROID_HARDWARE_BUFFER_ANDROID_DEVICE_DISPATCH_OFFSET,
    VK_GET_MEMORY_FD_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_MEMORY_FD_PROPERTIES_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_MEMORY_HOST_POINTER_PROPERTIES_EXT_DEVICE_DISPATCH_OFFSET,
    VK_GET_MEMORY_METAL_HANDLE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_GET_MEMORY_METAL_HANDLE_PROPERTIES_EXT_DEVICE_DISPATCH_OFFSET,
    VK_GET_MEMORY_NATIVE_BUFFER_OHOS_DEVICE_DISPATCH_OFFSET,
    VK_GET_MEMORY_REMOTE_ADDRESS_NV_DEVICE_DISPATCH_OFFSET,
    VK_GET_MEMORY_WIN32HANDLE_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_MEMORY_WIN32HANDLE_NV_DEVICE_DISPATCH_OFFSET,
    VK_GET_MEMORY_WIN32HANDLE_PROPERTIES_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_MEMORY_ZIRCON_HANDLE_FUCHSIA_DEVICE_DISPATCH_OFFSET,
    VK_GET_MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA_DEVICE_DISPATCH_OFFSET,
    VK_GET_MICROMAP_BUILD_SIZES_EXT_DEVICE_DISPATCH_OFFSET,
    VK_GET_NATIVE_BUFFER_PROPERTIES_OHOS_DEVICE_DISPATCH_OFFSET,
    VK_GET_PARTITIONED_ACCELERATION_STRUCTURES_BUILD_SIZES_NV_DEVICE_DISPATCH_OFFSET,
    VK_GET_PAST_PRESENTATION_TIMING_EXT_DEVICE_DISPATCH_OFFSET,
    VK_GET_PAST_PRESENTATION_TIMING_GOOGLE_DEVICE_DISPATCH_OFFSET,
    VK_GET_PERFORMANCE_PARAMETER_INTEL_DEVICE_DISPATCH_OFFSET,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    u16::MAX,
    VK_GET_PIPELINE_BINARY_DATA_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_PIPELINE_CACHE_DATA_DEVICE_DISPATCH_OFFSET,
    VK_GET_PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATIONS_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_PIPELINE_EXECUTABLE_PROPERTIES_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_PIPELINE_EXECUTABLE_STATISTICS_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_PIPELINE_INDIRECT_DEVICE_ADDRESS_NV_DEVICE_DISPATCH_OFFSET,
    VK_GET_PIPELINE_INDIRECT_MEMORY_REQUIREMENTS_NV_DEVICE_DISPATCH_OFFSET,
    VK_GET_PIPELINE_KEY_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_PIPELINE_PROPERTIES_EXT_DEVICE_DISPATCH_OFFSET,
    VK_GET_PRIVATE_DATA_DEVICE_DISPATCH_OFFSET,
    VK_GET_PRIVATE_DATA_EXT_DEVICE_DISPATCH_OFFSET,
    VK_GET_QUERY_POOL_RESULTS_DEVICE_DISPATCH_OFFSET,
    VK_GET_QUEUE_CHECKPOINT_DATA2NV_DEVICE_DISPATCH_OFFSET,
    VK_GET_QUEUE_CHECKPOINT_DATA_NV_DEVICE_DISPATCH_OFFSET,
    u16::MAX,
    VK_GET_RAY_TRACING_CAPTURE_REPLAY_SHADER_GROUP_HANDLES_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_RAY_TRACING_SHADER_GROUP_HANDLES_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_RAY_TRACING_SHADER_GROUP_HANDLES_NV_DEVICE_DISPATCH_OFFSET,
    VK_GET_RAY_TRACING_SHADER_GROUP_STACK_SIZE_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_REFRESH_CYCLE_DURATION_GOOGLE_DEVICE_DISPATCH_OFFSET,
    VK_GET_RENDER_AREA_GRANULARITY_DEVICE_DISPATCH_OFFSET,
    VK_GET_RENDERING_AREA_GRANULARITY_DEVICE_DISPATCH_OFFSET,
    VK_GET_RENDERING_AREA_GRANULARITY_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_SAMPLER_OPAQUE_CAPTURE_DESCRIPTOR_DATA_EXT_DEVICE_DISPATCH_OFFSET,
    VK_GET_SCREEN_BUFFER_PROPERTIES_QNX_DEVICE_DISPATCH_OFFSET,
    VK_GET_SEMAPHORE_COUNTER_VALUE_DEVICE_DISPATCH_OFFSET,
    VK_GET_SEMAPHORE_COUNTER_VALUE_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_SEMAPHORE_FD_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_SEMAPHORE_WIN32HANDLE_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_SEMAPHORE_ZIRCON_HANDLE_FUCHSIA_DEVICE_DISPATCH_OFFSET,
    VK_GET_SHADER_BINARY_DATA_EXT_DEVICE_DISPATCH_OFFSET,
    VK_GET_SHADER_INFO_AMD_DEVICE_DISPATCH_OFFSET,
    VK_GET_SHADER_INSTRUMENTATION_VALUES_ARM_DEVICE_DISPATCH_OFFSET,
    VK_GET_SHADER_MODULE_CREATE_INFO_IDENTIFIER_EXT_DEVICE_DISPATCH_OFFSET,
    VK_GET_SHADER_MODULE_IDENTIFIER_EXT_DEVICE_DISPATCH_OFFSET,
    VK_GET_SLEEP_STATUS_LEGACY_NV_DEVICE_DISPATCH_OFFSET,
    VK_GET_SWAPCHAIN_COUNTER_EXT_DEVICE_DISPATCH_OFFSET,
    VK_GET_SWAPCHAIN_IMAGES_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_SWAPCHAIN_STATUS_KHR_DEVICE_DISPATCH_OFFSET,
    VK_GET_SWAPCHAIN_TIME_DOMAIN_PROPERTIES_EXT_DEVICE_DISPATCH_OFFSET,
    VK_GET_SWAPCHAIN_TIMING_PROPERTIES_EXT_DEVICE_DISPATCH_OFFSET,
    VK_GET_TENSOR_MEMORY_REQUIREMENTS_ARM_DEVICE_DISPATCH_OFFSET,
    VK_GET_TENSOR_OPAQUE_CAPTURE_DATA_ARM_DEVICE_DISPATCH_OFFSET,
    VK_GET_TENSOR_OPAQUE_CAPTURE_DESCRIPTOR_DATA_ARM_DEVICE_DISPATCH_OFFSET,
    VK_GET_TENSOR_VIEW_OPAQUE_CAPTURE_DESCRIPTOR_DATA_ARM_DEVICE_DISPATCH_OFFSET,
    VK_GET_VALIDATION_CACHE_DATA_EXT_DEVICE_DISPATCH_OFFSET,
    VK_GET_VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR_DEVICE_DISPATCH_OFFSET,
    u16::MAX,
    VK_IMPORT_FENCE_FD_KHR_DEVICE_DISPATCH_OFFSET,
    VK_IMPORT_FENCE_WIN32HANDLE_KHR_DEVICE_DISPATCH_OFFSET,
    VK_IMPORT_SEMAPHORE_FD_KHR_DEVICE_DISPATCH_OFFSET,
    VK_IMPORT_SEMAPHORE_WIN32HANDLE_KHR_DEVICE_DISPATCH_OFFSET,
    VK_IMPORT_SEMAPHORE_ZIRCON_HANDLE_FUCHSIA_DEVICE_DISPATCH_OFFSET,
    VK_INITIALIZE_PERFORMANCE_API_INTEL_DEVICE_DISPATCH_OFFSET,
    VK_INVALIDATE_MAPPED_MEMORY_RANGES_DEVICE_DISPATCH_OFFSET,
    VK_LATENCY_SLEEP_LEGACY_NV_DEVICE_DISPATCH_OFFSET,
    VK_LATENCY_SLEEP_NV_DEVICE_DISPATCH_OFFSET,
    VK_MAP_MEMORY_DEVICE_DISPATCH_OFFSET,
    VK_MAP_MEMORY2_DEVICE_DISPATCH_OFFSET,
    VK_MAP_MEMORY2KHR_DEVICE_DISPATCH_OFFSET,
    VK_MERGE_PIPELINE_CACHES_DEVICE_DISPATCH_OFFSET,
    VK_MERGE_VALIDATION_CACHES_EXT_DEVICE_DISPATCH_OFFSET,
    VK_QUEUE_BEGIN_DEBUG_UTILS_LABEL_EXT_DEVICE_DISPATCH_OFFSET,
    VK_QUEUE_BIND_SPARSE_DEVICE_DISPATCH_OFFSET,
    VK_QUEUE_END_DEBUG_UTILS_LABEL_EXT_DEVICE_DISPATCH_OFFSET,
    VK_QUEUE_INSERT_DEBUG_UTILS_LABEL_EXT_DEVICE_DISPATCH_OFFSET,
    VK_QUEUE_NOTIFY_OUT_OF_BAND_LEGACY_NV_DEVICE_DISPATCH_OFFSET,
    VK_QUEUE_NOTIFY_OUT_OF_BAND_NV_DEVICE_DISPATCH_OFFSET,
    VK_QUEUE_PRESENT_KHR_DEVICE_DISPATCH_OFFSET,
    VK_QUEUE_SET_PERF_HINT_QCOM_DEVICE_DISPATCH_OFFSET,
    VK_QUEUE_SET_PERFORMANCE_CONFIGURATION_INTEL_DEVICE_DISPATCH_OFFSET,
    VK_QUEUE_SUBMIT_DEVICE_DISPATCH_OFFSET,
    VK_QUEUE_SUBMIT2_DEVICE_DISPATCH_OFFSET,
    VK_QUEUE_SUBMIT2KHR_DEVICE_DISPATCH_OFFSET,
    VK_QUEUE_WAIT_IDLE_DEVICE_DISPATCH_OFFSET,
    VK_REGISTER_CUSTOM_BORDER_COLOR_EXT_DEVICE_DISPATCH_OFFSET,
    VK_REGISTER_DEVICE_EVENT_EXT_DEVICE_DISPATCH_OFFSET,
    VK_REGISTER_DISPLAY_EVENT_EXT_DEVICE_DISPATCH_OFFSET,
    VK_RELEASE_CAPTURED_PIPELINE_DATA_KHR_DEVICE_DISPATCH_OFFSET,
    u16::MAX,
    VK_RELEASE_FULL_SCREEN_EXCLUSIVE_MODE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_RELEASE_PERFORMANCE_CONFIGURATION_INTEL_DEVICE_DISPATCH_OFFSET,
    VK_RELEASE_PROFILING_LOCK_KHR_DEVICE_DISPATCH_OFFSET,
    VK_RELEASE_SWAPCHAIN_IMAGES_EXT_DEVICE_DISPATCH_OFFSET,
    VK_RELEASE_SWAPCHAIN_IMAGES_KHR_DEVICE_DISPATCH_OFFSET,
    VK_RESET_COMMAND_BUFFER_DEVICE_DISPATCH_OFFSET,
    VK_RESET_COMMAND_POOL_DEVICE_DISPATCH_OFFSET,
    VK_RESET_DESCRIPTOR_POOL_DEVICE_DISPATCH_OFFSET,
    VK_RESET_EVENT_DEVICE_DISPATCH_OFFSET,
    VK_RESET_FENCES_DEVICE_DISPATCH_OFFSET,
    VK_RESET_GPA_SESSION_AMD_DEVICE_DISPATCH_OFFSET,
    VK_RESET_QUERY_POOL_DEVICE_DISPATCH_OFFSET,
    VK_RESET_QUERY_POOL_EXT_DEVICE_DISPATCH_OFFSET,
    VK_SET_BUFFER_COLLECTION_BUFFER_CONSTRAINTS_FUCHSIA_DEVICE_DISPATCH_OFFSET,
    VK_SET_BUFFER_COLLECTION_IMAGE_CONSTRAINTS_FUCHSIA_DEVICE_DISPATCH_OFFSET,
    VK_SET_DEBUG_UTILS_OBJECT_NAME_EXT_DEVICE_DISPATCH_OFFSET,
    VK_SET_DEBUG_UTILS_OBJECT_TAG_EXT_DEVICE_DISPATCH_OFFSET,
    VK_SET_DEVICE_MEMORY_PRIORITY_EXT_DEVICE_DISPATCH_OFFSET,
    VK_SET_EVENT_DEVICE_DISPATCH_OFFSET,
    VK_SET_GPA_DEVICE_CLOCK_MODE_AMD_DEVICE_DISPATCH_OFFSET,
    VK_SET_HDR_METADATA_EXT_DEVICE_DISPATCH_OFFSET,
    VK_SET_LATENCY_MARKER_LEGACY_NV_DEVICE_DISPATCH_OFFSET,
    VK_SET_LATENCY_MARKER_NV_DEVICE_DISPATCH_OFFSET,
    VK_SET_LATENCY_SLEEP_MODE_LEGACY_NV_DEVICE_DISPATCH_OFFSET,
    VK_SET_LATENCY_SLEEP_MODE_NV_DEVICE_DISPATCH_OFFSET,
    VK_SET_LOCAL_DIMMING_AMD_DEVICE_DISPATCH_OFFSET,
    VK_SET_PRIVATE_DATA_DEVICE_DISPATCH_OFFSET,
    VK_SET_PRIVATE_DATA_EXT_DEVICE_DISPATCH_OFFSET,
    VK_SET_SWAPCHAIN_PRESENT_TIMING_QUEUE_SIZE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_SHUTDOWN_LATENCY_DEVICE_LEGACY_NV_DEVICE_DISPATCH_OFFSET,
    VK_SIGNAL_SEMAPHORE_DEVICE_DISPATCH_OFFSET,
    VK_SIGNAL_SEMAPHORE_KHR_DEVICE_DISPATCH_OFFSET,
    u16::MAX,
    VK_TRANSITION_IMAGE_LAYOUT_DEVICE_DISPATCH_OFFSET,
    VK_TRANSITION_IMAGE_LAYOUT_EXT_DEVICE_DISPATCH_OFFSET,
    VK_TRIM_COMMAND_POOL_DEVICE_DISPATCH_OFFSET,
    VK_TRIM_COMMAND_POOL_KHR_DEVICE_DISPATCH_OFFSET,
    VK_UNINITIALIZE_PERFORMANCE_API_INTEL_DEVICE_DISPATCH_OFFSET,
    VK_UNMAP_MEMORY_DEVICE_DISPATCH_OFFSET,
    VK_UNMAP_MEMORY2_DEVICE_DISPATCH_OFFSET,
    VK_UNMAP_MEMORY2KHR_DEVICE_DISPATCH_OFFSET,
    VK_UNREGISTER_CUSTOM_BORDER_COLOR_EXT_DEVICE_DISPATCH_OFFSET,
    VK_UPDATE_DESCRIPTOR_SET_WITH_TEMPLATE_DEVICE_DISPATCH_OFFSET,
    VK_UPDATE_DESCRIPTOR_SET_WITH_TEMPLATE_KHR_DEVICE_DISPATCH_OFFSET,
    VK_UPDATE_DESCRIPTOR_SETS_DEVICE_DISPATCH_OFFSET,
    VK_UPDATE_INDIRECT_EXECUTION_SET_PIPELINE_EXT_DEVICE_DISPATCH_OFFSET,
    VK_UPDATE_INDIRECT_EXECUTION_SET_SHADER_EXT_DEVICE_DISPATCH_OFFSET,
    VK_UPDATE_VIDEO_SESSION_PARAMETERS_KHR_DEVICE_DISPATCH_OFFSET,
    VK_WAIT_FOR_FENCES_DEVICE_DISPATCH_OFFSET,
    VK_WAIT_FOR_PRESENT2KHR_DEVICE_DISPATCH_OFFSET,
    VK_WAIT_FOR_PRESENT_KHR_DEVICE_DISPATCH_OFFSET,
    VK_WAIT_SEMAPHORES_DEVICE_DISPATCH_OFFSET,
    VK_WAIT_SEMAPHORES_KHR_DEVICE_DISPATCH_OFFSET,
    VK_WRITE_ACCELERATION_STRUCTURES_PROPERTIES_KHR_DEVICE_DISPATCH_OFFSET,
    VK_WRITE_MICROMAPS_PROPERTIES_EXT_DEVICE_DISPATCH_OFFSET,
    VK_WRITE_RESOURCE_DESCRIPTORS_EXT_DEVICE_DISPATCH_OFFSET,
    VK_WRITE_SAMPLER_DESCRIPTORS_EXT_DEVICE_DISPATCH_OFFSET,
];
pub(super) static COMMAND_LOADER_TRAMPOLINE_WORDS: [u64; 14] = [
    0x0000_0000_0000_0100,
    0x0000_0000_0000_0000,
    0x0000_0000_0000_0000,
    0x0000_0000_0000_0000,
    0x0000_0000_0000_0000,
    0x0000_0000_0000_0000,
    0x0060_0240_0000_0000,
    0x0000_0000_0000_0800,
    0x0c00_4000_0000_0000,
    0x0000_0000_0000_0000,
    0x0000_0000_0000_0000,
    0x0000_0000_0000_0000,
    0x0000_0003_0000_0000,
    0x0000_0000_0000_0000,
];
pub(crate) const COMMAND_COUNT: usize = 842;
#[cfg(test)]
pub(crate) const COMMAND_MAX_DISPLACEMENT: u16 = 28;
#[inline(never)]
pub(crate) fn command_lookup(name: &CStr) -> Option<CommandLookup> {
    let suffix = name.to_bytes().strip_prefix(b"vk")?;
    let hash = command_hash(suffix);
    let bucket_mask = (COMMAND_DISPLACEMENTS.len() - 1) as u64;
    let bucket = (hash & bucket_mask) as usize;
    let displacement = COMMAND_DISPLACEMENTS[bucket];
    let slot_mask = (COMMAND_TABLE.len() - 1) as u64;
    let slot = (command_slot_hash(hash ^ u64::from(displacement)) & slot_mask) as usize;
    let record = COMMAND_TABLE[slot];
    if record.id == u16::MAX {
        return None;
    }
    let start = usize::from(record.name_offset);
    let end = start + usize::from(record.name_len);
    debug_assert!(end <= COMMAND_NAMES.len());
    let stored_suffix = unsafe { COMMAND_NAMES.get_unchecked(start..end) };
    command_name_eq(stored_suffix, suffix).then_some(CommandLookup {
        id: record.id,
        scope: record.scope,
    })
}
#[inline]
pub(crate) fn command_must_use_loader_trampoline(id: u16) -> bool {
    let index = usize::from(id);
    debug_assert!(index < COMMAND_COUNT);
    let word =
        unsafe { *COMMAND_LOADER_TRAMPOLINE_WORDS.get_unchecked(index / u64::BITS as usize) };
    word & (1_u64 << (index % u64::BITS as usize)) != 0
}
#[inline]
pub(crate) fn command_core_level(id: u16) -> u16 {
    let index = usize::from(id);
    debug_assert!(index < COMMAND_CORE_LEVELS.len());
    unsafe { *COMMAND_CORE_LEVELS.get_unchecked(index) }
}
#[inline]
pub(super) fn command_extension_enabled(
    id: u16,
    ranges: &[CommandProviderRange; COMMAND_COUNT],
    ids: &[u16],
    enabled: &ExtensionSet,
) -> bool {
    let index = usize::from(id);
    debug_assert!(index < ranges.len());
    let range = unsafe { *ranges.get_unchecked(index) };
    let start = usize::from(range.offset);
    let end = start + usize::from(range.len);
    debug_assert!(end <= ids.len());
    unsafe { ids.get_unchecked(start..end) }
        .iter()
        .copied()
        .any(|extension| enabled.contains(extension))
}
#[inline]
pub(crate) fn command_has_enabled_instance_extension(id: u16, enabled: &ExtensionSet) -> bool {
    command_extension_enabled(
        id,
        &COMMAND_INSTANCE_EXTENSION_RANGES,
        &COMMAND_INSTANCE_EXTENSION_IDS,
        enabled,
    )
}
#[inline]
pub(crate) fn command_has_enabled_device_extension(id: u16, enabled: &ExtensionSet) -> bool {
    command_extension_enabled(
        id,
        &COMMAND_DEVICE_EXTENSION_RANGES,
        &COMMAND_DEVICE_EXTENSION_IDS,
        enabled,
    )
}
#[inline]
pub(crate) fn command_has_device_extension_provider(id: u16) -> bool {
    let index = usize::from(id);
    debug_assert!(index < COMMAND_DEVICE_EXTENSION_RANGES.len());
    unsafe { COMMAND_DEVICE_EXTENSION_RANGES.get_unchecked(index) }.len != 0
}
