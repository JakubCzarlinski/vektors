#[path = "shared/mod.rs"]
mod shared;

use core::ptr::null;
use core::time::Duration;
use shared::bootstrap::{create_device, create_instance};
use shared::device_select::{device_name, select_single_device};
use std::sync::mpsc;
use std::thread;
use vk::{
    Entry, VkBufferCreateInfo, VkBufferUsageFlagBits2, VkBufferUsageFlags2CreateInfo,
    VkSharingMode, VulkanLib,
};
use vk_alloc::{
    AllocationCreateInfo, Allocator, AllocatorCreateInfo, MemoryTypePolicy, PoolCreateInfo,
};

fn pool_id_from_arena_id(arena_id: u32) -> u32 {
    (arena_id >> 16) - 1
}

fn main() -> Result<(), String> {
    const STREAM_FRAMES: usize = 120;
    const WORDS_PER_FRAME: usize = 256;

    let library = VulkanLib::load().map_err(|err| format!("failed to load Vulkan: {err:?}"))?;
    let entry = Entry::new(&library);
    let mut instance = create_instance(&entry, c"vk-alloc pool example")?;
    {
        let (physical_device, queue_family_index) = select_single_device(&instance)?;
        let physical_device_name = device_name(&physical_device);
        let mut device = create_device(&physical_device, queue_family_index)?;

        println!("Selected device: {physical_device_name}");
        println!("Queue family index: {queue_family_index}");

        {
            let allocator = Allocator::from_create_info(
                AllocatorCreateInfo::new(&physical_device, &device).with_default_pool(
                    PoolCreateInfo::new().with_host_visible_block_size(8 * 1024 * 1024),
                ),
            )
            .map_err(|err| format!("{err:?}"))?;

            let streaming_pool = allocator
                .create_pool(
                    PoolCreateInfo::new()
                        .with_host_visible_block_size(2 * 1024 * 1024)
                        .with_dedicated_threshold(512 * 1024),
                )
                .map_err(|err| format!("failed to create streaming pool: {err:?}"))?;
            let readback_pool = allocator
                .create_pool(PoolCreateInfo::new().with_host_visible_block_size(4 * 1024 * 1024))
                .map_err(|err| format!("failed to create readback pool: {err:?}"))?;

            let usage = VkBufferUsageFlags2CreateInfo::DEFAULT
                .with_usage(VkBufferUsageFlagBits2::TRANSFER_SRC);
            let buffer_info = VkBufferCreateInfo::DEFAULT
                .with_size((WORDS_PER_FRAME * core::mem::size_of::<u32>()) as u64)
                .with_pNext_VkBufferUsageFlags2CreateInfo(&usage)
                .with_sharingMode(VkSharingMode::EXCLUSIVE);
            let (tx, rx) = mpsc::sync_channel::<Vec<u32>>(8);
            let producer = thread::spawn(move || -> Result<(), String> {
                for frame in 0..STREAM_FRAMES {
                    let payload: Vec<u32> = (0..WORDS_PER_FRAME as u32)
                        .map(|v| v + frame as u32)
                        .collect();
                    tx.send(payload)
                        .map_err(|_| "consumer thread dropped stream channel".to_string())?;
                    thread::sleep(Duration::from_millis(4));
                }
                Ok(())
            });

            for (frame, payload) in rx.into_iter().enumerate() {
                let target_pool = if frame % 2 == 0 {
                    streaming_pool
                } else {
                    readback_pool
                };
                let alloc_info = AllocationCreateInfo::new()
                    .with_memory_type_policy(MemoryTypePolicy::UPLOAD)
                    .with_pool(target_pool);
                let mut buffer = allocator
                    .create_buffer(&buffer_info, alloc_info)
                    .map_err(|err| format!("failed to create stream buffer: {err:?}"))?;
                buffer
                    .allocation_mut()
                    .mapped_slice_mut::<u32>(WORDS_PER_FRAME)
                    .map_err(|err| format!("stream allocation is not writable: {err:?}"))?
                    .copy_from_slice(&payload);
                if frame % 30 == 0 {
                    let arena_id = buffer.allocation().arena_id();
                    println!(
                        "frame {frame}: pool id {}, arena id {}, decoded pool id {}",
                        target_pool.id(),
                        arena_id,
                        pool_id_from_arena_id(arena_id),
                    );
                }
            }

            producer
                .join()
                .map_err(|_| "producer thread panicked".to_string())??;
            println!("Allocator stats after streaming: {:?}", allocator.stats());
            drop(allocator);
        }

        device.vkDestroyDevice(null());
    }
    instance.vkDestroyInstance(null());
    Ok(())
}
