//! Local Vulkan allocator benchmarks.
//!
//! These benchmarks intentionally use a real Vulkan loader and physical device.
//! They measure the allocator's public API, including Vulkan resource creation,
//! memory allocation, binding, and host-visible writes. Setup is outside each
//! Criterion timing loop.

use core::ffi::CStr;
use core::ptr::null;
use core::sync::atomic::{AtomicBool, Ordering};
use criterion::{
    BatchSize, BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main,
};
use std::sync::Barrier;
use std::thread;
use vk::{
    Device, Entry, Instance, PhysicalDevice, VkApplicationInfo, VkBufferCreateInfo,
    VkDeviceCreateInfo, VkDeviceQueueCreateInfo, VkInstanceCreateInfo, VkPhysicalDeviceProperties2,
    VkQueueFamilyProperties2, VkSharingMode, VulkanLib,
};
use vk_alloc::{
    AllocationCreateInfo, AllocationStrategy, Allocator, MemoryTypePolicy, Pool, PoolCreateInfo,
};

const SMALL_BUFFER_SIZE: u64 = 64 * 1024;
const LARGE_BUFFER_SIZE: u64 = 8 * 1024 * 1024;
const WRITE_SIZE: usize = 4 * 1024 * 1024;
const CONCURRENT_OPERATIONS_PER_WORKER: usize = 256;
const BUFFER_USAGE: vk::VkBufferUsageFlags2CreateInfo<'static> =
    vk::VkBufferUsageFlags2CreateInfo::DEFAULT.with_usage(vk::VkBufferUsageFlagBits2::TRANSFER_SRC);

fn with_allocator(f: impl for<'vk> FnOnce(&Allocator<'vk>)) {
    let library = VulkanLib::load().expect("failed to load Vulkan loader");
    let entry = Entry::new(&library);
    let instance = create_instance(&entry);
    let (physical_device, queue_family_index) = select_device(&instance);
    let device = create_device(&physical_device, queue_family_index);
    let allocator = Allocator::from_create_info(
        vk_alloc::AllocatorCreateInfo::new(&physical_device, &device).with_default_pool(
            PoolCreateInfo::new()
                .with_host_visible_block_size(16 * 1024 * 1024)
                .with_device_local_block_size(64 * 1024 * 1024),
        ),
    )
    .expect("failed to create allocator");

    f(&allocator);
}

fn create_instance<'lib>(entry: &'lib Entry<'lib>) -> Instance<'lib> {
    let app_info = VkApplicationInfo::DEFAULT
        .with_apiVersion(vk::VK_API_VERSION_1_1)
        .with_pApplicationName(c"vk-alloc benchmark".as_ptr())
        .with_pEngineName(c"vk-alloc".as_ptr());
    let create_info = VkInstanceCreateInfo::DEFAULT.with_pApplicationInfo(&raw const app_info);
    entry
        .vkCreateInstance(&create_info, null())
        .expect("failed to create Vulkan instance")
}

fn select_device<'inst>(instance: &'inst Instance<'inst>) -> (PhysicalDevice<'inst>, u32) {
    let physical_devices = instance
        .vkEnumeratePhysicalDevices()
        .expect("failed to enumerate physical devices");

    for physical_device in physical_devices {
        if let Some(queue_family_index) = find_queue_family(&physical_device) {
            let mut properties = VkPhysicalDeviceProperties2::DEFAULT;
            physical_device.vkGetPhysicalDeviceProperties2(&mut properties);
            let name = unsafe { CStr::from_ptr(properties.properties.deviceName.as_ptr()) };
            eprintln!("vk-alloc benchmarks: using {}", name.to_string_lossy());
            return (physical_device, queue_family_index);
        }
    }
    panic!("no Vulkan physical device with a queue family was found");
}

fn find_queue_family(physical_device: &PhysicalDevice<'_>) -> Option<u32> {
    let mut count = 0;
    physical_device.vkGetPhysicalDeviceQueueFamilyProperties2(&mut count, core::ptr::null_mut());
    let mut properties = vec![VkQueueFamilyProperties2::DEFAULT; count as usize];
    physical_device.vkGetPhysicalDeviceQueueFamilyProperties2(&mut count, properties.as_mut_ptr());
    properties
        .iter()
        .position(|property| !property.queueFamilyProperties.queueFlags.is_empty())
        .map(|index| index as u32)
}

fn create_device<'inst>(
    physical_device: &PhysicalDevice<'inst>,
    queue_family_index: u32,
) -> Device<'inst> {
    let priorities = [1.0_f32];
    let queue_info = VkDeviceQueueCreateInfo::DEFAULT
        .with_queueFamilyIndex(queue_family_index)
        .with_pQueuePriorities(&priorities);
    let queue_infos = [queue_info];
    let create_info = VkDeviceCreateInfo::DEFAULT.with_pQueueCreateInfos(&queue_infos);
    physical_device
        .vkCreateDevice(&create_info, null())
        .expect("failed to create Vulkan device")
}

fn buffer_info(size: u64) -> VkBufferCreateInfo<'static> {
    VkBufferCreateInfo::DEFAULT
        .with_size(size)
        .with_pNext_VkBufferUsageFlags2CreateInfo(&BUFFER_USAGE)
        .with_sharingMode(VkSharingMode::EXCLUSIVE)
}

fn bench_host_visible_resource_lifecycle(c: &mut Criterion) {
    with_allocator(|allocator| {
        let mut group = c.benchmark_group("real_vulkan_host_visible");
        group.throughput(Throughput::Bytes(SMALL_BUFFER_SIZE));
        group.bench_function("create_allocate_bind_drop_64KiB", |b| {
            b.iter(|| {
                let buffer = allocator
                    .create_buffer(
                        &buffer_info(SMALL_BUFFER_SIZE),
                        AllocationCreateInfo::new()
                            .with_memory_type_policy(MemoryTypePolicy::UPLOAD),
                    )
                    .expect("host-visible allocation failed");
                black_box(buffer);
            });
        });
        group.finish();
    });
}

fn bench_device_local_resource_lifecycle(c: &mut Criterion) {
    with_allocator(|allocator| {
        let mut group = c.benchmark_group("real_vulkan_device_local");
        group.throughput(Throughput::Bytes(SMALL_BUFFER_SIZE));
        group.bench_function("suballocated_create_allocate_bind_drop_64KiB", |b| {
            b.iter(|| {
                let buffer = allocator
                    .create_buffer(
                        &buffer_info(SMALL_BUFFER_SIZE),
                        AllocationCreateInfo::new()
                            .with_memory_type_policy(MemoryTypePolicy::DEVICE_LOCAL),
                    )
                    .expect("device-local allocation failed");
                black_box(buffer);
            });
        });
        group.throughput(Throughput::Bytes(LARGE_BUFFER_SIZE));
        group.bench_function("dedicated_create_allocate_bind_drop_8MiB", |b| {
            b.iter(|| {
                let buffer = allocator
                    .create_buffer(
                        &buffer_info(LARGE_BUFFER_SIZE),
                        AllocationCreateInfo::new()
                            .with_memory_type_policy(MemoryTypePolicy::DEVICE_LOCAL)
                            .with_strategy(AllocationStrategy::AlwaysDedicated),
                    )
                    .expect("dedicated device-local allocation failed");
                black_box(buffer);
            });
        });
        group.finish();
    });
}

fn bench_mapped_host_write(c: &mut Criterion) {
    with_allocator(|allocator| {
        let mut buffer = allocator
            .create_buffer(
                &buffer_info(WRITE_SIZE as u64),
                AllocationCreateInfo::new().with_memory_type_policy(MemoryTypePolicy::UPLOAD),
            )
            .expect("host-visible allocation failed");
        let words = WRITE_SIZE / core::mem::size_of::<u32>();
        let mut group = c.benchmark_group("real_vulkan_host_visible");
        group.throughput(Throughput::Bytes(WRITE_SIZE as u64));
        group.bench_function("write_mapped_4MiB", |b| {
            b.iter(|| {
                let slice = buffer
                    .allocation_mut()
                    .mapped_slice_mut::<u32>(words)
                    .expect("allocation is not mapped");
                slice.fill(black_box(0xA5A5_A5A5));
                black_box(slice.as_ptr());
            });
        });
        group.finish();
    });
}

fn bench_mixed_lifetime_churn(c: &mut Criterion) {
    with_allocator(|allocator| {
        let mut group = c.benchmark_group("real_vulkan_allocator_churn");
        group.throughput(Throughput::Elements(64));
        group.bench_with_input(
            BenchmarkId::new("create_drop_mixed", 64),
            &64_usize,
            |b, &count| {
                b.iter_batched(
                    || Vec::with_capacity(count),
                    |mut buffers| {
                        for index in 0..count {
                            let size = 4 * 1024 * (1 + (index % 16) as u64);
                            let policy = if index % 2 == 0 {
                                MemoryTypePolicy::UPLOAD
                            } else {
                                MemoryTypePolicy::DEVICE_LOCAL
                            };
                            buffers.push(
                                allocator
                                    .create_buffer(
                                        &buffer_info(size),
                                        AllocationCreateInfo::new().with_memory_type_policy(policy),
                                    )
                                    .expect("allocation failed"),
                            );
                        }
                        black_box(&buffers);
                        drop(buffers);
                    },
                    BatchSize::SmallInput,
                );
            },
        );
        group.finish();
    });
}

fn bench_concurrent_allocations(c: &mut Criterion) {
    with_allocator(|allocator| {
        const THREAD_COUNTS: &[usize] = &[1, 2, 4, 8];
        let worker_pools: Vec<Pool> =
            (0..*THREAD_COUNTS.last().expect("thread counts are non-empty"))
                .map(|_| {
                    allocator
                        .create_pool(
                            PoolCreateInfo::new().with_host_visible_block_size(16 * 1024 * 1024),
                        )
                        .expect("failed to create worker pool")
                })
                .collect();

        for (layout_name, separate_pools) in [("shared_arena", false), ("per_thread_pool", true)] {
            let mut group = c.benchmark_group(format!("real_vulkan_concurrent/{layout_name}"));
            group.warm_up_time(core::time::Duration::from_millis(500));
            group.measurement_time(core::time::Duration::from_secs(2));
            group.sample_size(40);

            for &worker_count in THREAD_COUNTS {
                group.throughput(Throughput::Elements(
                    (worker_count * CONCURRENT_OPERATIONS_PER_WORKER) as u64,
                ));
                group.bench_with_input(
                    BenchmarkId::from_parameter(worker_count),
                    &worker_count,
                    |b, &worker_count| {
                        let phase = Barrier::new(worker_count + 1);
                        let stop = AtomicBool::new(false);
                        thread::scope(|scope| {
                            for worker_pool in worker_pools.iter().take(worker_count) {
                                let phase = &phase;
                                let stop = &stop;
                                let pool = if separate_pools {
                                    *worker_pool
                                } else {
                                    Pool::DEFAULT
                                };
                                scope.spawn(move || {
                                    let alloc_info = AllocationCreateInfo::new()
                                        .with_memory_type_policy(MemoryTypePolicy::UPLOAD)
                                        .with_pool(pool);
                                    loop {
                                        phase.wait();
                                        if stop.load(Ordering::Acquire) {
                                            break;
                                        }
                                        for operation in 0..CONCURRENT_OPERATIONS_PER_WORKER {
                                            let size = 4 * 1024 * (1 + (operation % 16) as u64);
                                            let buffer = allocator
                                                .create_buffer(
                                                    &buffer_info(size),
                                                    alloc_info.clone(),
                                                )
                                                .expect("concurrent allocation failed");
                                            drop(black_box(buffer));
                                        }
                                        phase.wait();
                                    }
                                });
                            }

                            b.iter(|| {
                                phase.wait();
                                phase.wait();
                            });

                            stop.store(true, Ordering::Release);
                            phase.wait();
                        });
                    },
                );
            }
            group.finish();
        }
    });
}

criterion_group!(
    benches,
    bench_host_visible_resource_lifecycle,
    bench_device_local_resource_lifecycle,
    bench_mapped_host_write,
    bench_mixed_lifetime_churn,
    bench_concurrent_allocations,
);
criterion_main!(benches);
