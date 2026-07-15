use core::ffi::{CStr, c_char};
use core::slice;
use core::{iter, mem};
use image::{ImageBuffer, RgbaImage, imageops};
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::{Instant, UNIX_EPOCH};
use vk::*;
use vk_alloc::{AllocationCreateInfo, Allocator};

#[repr(C, align(4))]
struct AlignedSpv<const N: usize>([u8; N]);

macro_rules! include_spirv_words {
    ($path:expr) => {{
        static SPV: AlignedSpv<{ include_bytes!($path).len() }> =
            AlignedSpv(*include_bytes!($path));
        unsafe { core::slice::from_raw_parts(SPV.0.as_ptr().cast::<u32>(), SPV.0.len() / 4) }
    }};
}

const IMAGE_EDIT_SPV: &[u32] = include_spirv_words!(concat!(env!("OUT_DIR"), "/image_edit.spv"));

const SRC_W: u32 = 1024;
const SRC_H: u32 = 1024;
const OUT_W: u32 = 2400;
const OUT_H: u32 = 1600;

struct SourceStack {
    pixels: Vec<u32>,
    image_count: u32,
}

const VALIDATION_LAYER: &CStr = c"VK_LAYER_KHRONOS_validation";
const APP_INFO: VkApplicationInfo = VkApplicationInfo::DEFAULT
    .with_apiVersion(VK_API_VERSION_1_4)
    .with_applicationVersion(VK_MAKE_VERSION(0, 1, 0))
    .with_engineVersion(VK_MAKE_VERSION(0, 1, 0))
    .with_pEngineName(c"vk-demo".as_ptr())
    .with_pApplicationName(c"Image Edit".as_ptr());
const DEVICE_CREATE_INFO: VkDeviceCreateInfo = VkDeviceCreateInfo::DEFAULT;
const BINDINGS: [VkDescriptorSetLayoutBinding; 2] = [
    VkDescriptorSetLayoutBinding::DEFAULT
        .with_binding(0)
        .with_descriptorType(VkDescriptorType::STORAGE_BUFFER)
        .with_descriptorCount(1)
        .with_stageFlags(VkShaderStageFlagBits::COMPUTE),
    VkDescriptorSetLayoutBinding::DEFAULT
        .with_binding(1)
        .with_descriptorType(VkDescriptorType::STORAGE_BUFFER)
        .with_descriptorCount(1)
        .with_stageFlags(VkShaderStageFlagBits::COMPUTE),
];
const DSL_INFO: VkDescriptorSetLayoutCreateInfo =
    VkDescriptorSetLayoutCreateInfo::DEFAULT.with_pBindings(&BINDINGS);

fn main() -> Result<(), String> {
    let started = Instant::now();
    let source_stack = load_images(Path::new("vk-demo/image-edit/images"))?;
    println!("Prepared source stack in {:.2?}", started.elapsed());

    let gpu_started = Instant::now();
    let library = VulkanLib::load().map_err(|e| e.to_string())?;
    let entry = Entry::new(&library);
    let layer_names = enabled_validation_layers(&entry)?;
    let instance_info = VkInstanceCreateInfo::DEFAULT
        .with_pApplicationInfo(&APP_INFO)
        .with_ppEnabledLayerNames(&layer_names);
    let instance = entry
        .vkCreateInstance(&instance_info, null())
        .map_err(|e| format!("vkCreateInstance failed: {e:?}"))?;

    let (device, physical_device, queue_family_index) = create_device(&instance)?;
    let allocator = Allocator::new(&physical_device, &device)
        .map_err(|e| format!("Allocator creation failed: {e:?}"))?;
    let queue = device.vkGetDeviceQueue(queue_family_index, 0);

    let input_size = (source_stack.pixels.len() * size_of::<u32>()) as u64;
    let output_len = (OUT_W * OUT_H) as usize;
    let output_size = (output_len * size_of::<u32>()) as u64;
    let mut input_buffer = create_storage_buffer(&allocator, input_size)?;
    let output_buffer = create_storage_buffer(&allocator, output_size)?;
    write_to_buffer(input_buffer.allocation_mut(), &source_stack.pixels)?;

    let descriptor_pool = create_descriptor_pool(&device)?;
    let descriptor_set_layout = device
        .vkCreateDescriptorSetLayout(&DSL_INFO, null())
        .map_err(|e| format!("vkCreateDescriptorSetLayout failed: {e:?}"))?;
    let descriptor_sets = create_descriptor_set(
        &descriptor_pool,
        &descriptor_set_layout,
        input_buffer.buffer(),
        output_buffer.buffer(),
        input_size,
        output_size,
    )?;
    let descriptor_set = descriptor_sets
        .first()
        .ok_or("No descriptor sets allocated")?;
    let (pipeline_layout, pipelines) = create_compute_pipeline(&device, &descriptor_set_layout)?;
    let pipeline = pipelines.first().ok_or("No compute pipeline created")?;

    run_compute(
        &device,
        &queue,
        queue_family_index,
        pipeline,
        &pipeline_layout,
        descriptor_set,
        source_stack.image_count,
    )?;
    println!("Vulkan compute finished in {:.2?}", gpu_started.elapsed());

    let save_started = Instant::now();
    let output_pixels = read_buffer(output_buffer.allocation(), output_len)?;
    let balanced_pixels = histogram_balance(output_pixels);
    let out_path = Path::new("vk-demo/image-edit/edit.png");
    save_png(out_path, &balanced_pixels)?;
    println!("Saved PNG in {:.2?}", save_started.elapsed());
    println!("Wrote {}", out_path.display());
    Ok(())
}

fn load_images(dir: &Path) -> Result<SourceStack, String> {
    let paths = image_paths(dir)?;
    let image_count: u32 = paths
        .len()
        .try_into()
        .map_err(|_| format!("Too many images in {}", dir.display()))?;
    let signature = source_signature(&paths)?;
    let cache_dir = Path::new("vk-demo/image-edit/.cache");
    let cache_pixels = cache_dir.join("source-512x512-rgba8.bin");
    let cache_manifest = cache_dir.join("source-512x512-rgba8.manifest");
    if let Some(pixels) = read_source_cache(
        &cache_pixels,
        &cache_manifest,
        &signature,
        image_count as usize,
    )? {
        return Ok(SourceStack {
            pixels,
            image_count,
        });
    }

    let mut pixels = Vec::with_capacity(paths.len() * (SRC_W * SRC_H) as usize);
    for path in paths {
        let image = image::open(&path)
            .map_err(|e| format!("Failed to open {}: {e}", path.display()))?
            .to_rgba8();
        let framed = fit_full_image(&image);
        for pixel in framed.pixels() {
            let [r, g, b, _] = pixel.0;
            pixels.push(u32::from(r) | (u32::from(g) << 8) | (u32::from(b) << 16) | 0xff000000);
        }
    }
    write_source_cache(&cache_pixels, &cache_manifest, &signature, &pixels)?;
    Ok(SourceStack {
        pixels,
        image_count,
    })
}

fn image_paths(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let mut paths: Vec<PathBuf> = fs::read_dir(dir)
        .map_err(|e| format!("Failed to read {}: {e}", dir.display()))?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<_, _>>()
        .map_err(|e| format!("Failed to inspect image dir: {e}"))?;
    paths.retain(|path| {
        path.extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| matches!(ext.to_ascii_lowercase().as_str(), "jpg" | "jpeg" | "png"))
    });
    paths.sort();
    if paths.is_empty() {
        return Err(format!("No images found in {}", dir.display()));
    }
    Ok(paths)
}

fn source_signature(paths: &[PathBuf]) -> Result<String, String> {
    let mut signature = format!("v5-full-frame-cover-fill:{SRC_W}x{SRC_H}:{}\n", paths.len());
    for path in paths {
        let metadata = fs::metadata(path)
            .map_err(|e| format!("Failed to read metadata for {}: {e}", path.display()))?;
        let modified = metadata
            .modified()
            .map_err(|e| format!("Failed to read mtime for {}: {e}", path.display()))?
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("Invalid mtime for {}: {e}", path.display()))?
            .as_nanos();
        signature.push_str(&format!(
            "{}:{}:{}\n",
            path.file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| format!("Non-UTF8 image path: {}", path.display()))?,
            metadata.len(),
            modified
        ));
    }
    Ok(signature)
}

fn read_source_cache(
    pixels_path: &Path,
    manifest_path: &Path,
    signature: &str,
    image_count: usize,
) -> Result<Option<Vec<u32>>, String> {
    let manifest = match fs::read_to_string(manifest_path) {
        Ok(manifest) => manifest,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(format!(
                "Failed to read cache manifest {}: {error}",
                manifest_path.display()
            ));
        }
    };
    if manifest != signature {
        return Ok(None);
    }

    let expected_len = image_count * (SRC_W * SRC_H) as usize;
    let mut bytes = Vec::new();
    fs::File::open(pixels_path)
        .map_err(|e| format!("Failed to open cache {}: {e}", pixels_path.display()))?
        .read_to_end(&mut bytes)
        .map_err(|e| format!("Failed to read cache {}: {e}", pixels_path.display()))?;
    if bytes.len() != expected_len * size_of::<u32>() {
        return Ok(None);
    }

    let pixels = bytes
        .chunks_exact(4)
        .map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect();
    Ok(Some(pixels))
}

fn write_source_cache(
    pixels_path: &Path,
    manifest_path: &Path,
    signature: &str,
    pixels: &[u32],
) -> Result<(), String> {
    let cache_dir = pixels_path
        .parent()
        .ok_or_else(|| format!("Cache path has no parent: {}", pixels_path.display()))?;
    fs::create_dir_all(cache_dir)
        .map_err(|e| format!("Failed to create cache dir {}: {e}", cache_dir.display()))?;
    let mut bytes = Vec::with_capacity(mem::size_of_val(pixels));
    for &pixel in pixels {
        bytes.extend_from_slice(&pixel.to_le_bytes());
    }
    fs::File::create(pixels_path)
        .map_err(|e| format!("Failed to create cache {}: {e}", pixels_path.display()))?
        .write_all(&bytes)
        .map_err(|e| format!("Failed to write cache {}: {e}", pixels_path.display()))?;
    fs::write(manifest_path, signature).map_err(|e| {
        format!(
            "Failed to write cache manifest {}: {e}",
            manifest_path.display()
        )
    })
}

fn fit_full_image(image: &RgbaImage) -> RgbaImage {
    let cover_scale =
        (SRC_W as f32 / image.width() as f32).max(SRC_H as f32 / image.height() as f32);
    let cover_w = (image.width() as f32 * cover_scale).round().max(1.0) as u32;
    let cover_h = (image.height() as f32 * cover_scale).round().max(1.0) as u32;
    let cover = imageops::resize(image, cover_w, cover_h, imageops::FilterType::Triangle);
    let cover_x = (cover.width() - SRC_W) / 2;
    let cover_y = (cover.height() - SRC_H) / 2;
    let mut framed = imageops::crop_imm(&cover, cover_x, cover_y, SRC_W, SRC_H).to_image();
    for pixel in framed.pixels_mut() {
        pixel.0[0] = (f32::from(pixel.0[0]) * 0.36) as u8;
        pixel.0[1] = (f32::from(pixel.0[1]) * 0.36) as u8;
        pixel.0[2] = (f32::from(pixel.0[2]) * 0.4) as u8;
    }

    let contain_scale =
        (SRC_W as f32 / image.width() as f32).min(SRC_H as f32 / image.height() as f32);
    let contained_w = (image.width() as f32 * contain_scale).round().max(1.0) as u32;
    let contained_h = (image.height() as f32 * contain_scale).round().max(1.0) as u32;
    let contained = imageops::resize(
        image,
        contained_w.min(SRC_W),
        contained_h.min(SRC_H),
        imageops::FilterType::Triangle,
    );
    let x = (SRC_W - contained.width()) / 2;
    let y = (SRC_H - contained.height()) / 2;
    imageops::overlay(&mut framed, &contained, i64::from(x), i64::from(y));
    framed
}

fn histogram_balance(pixels: &[u32]) -> Vec<u32> {
    let mut histogram = [0usize; 256];
    for &pixel in pixels {
        histogram[luma(pixel) as usize] += 1;
    }

    let clip_limit = (pixels.len() / 96).max(1);
    let mut overflow = 0usize;
    for count in &mut histogram {
        if *count > clip_limit {
            overflow += *count - clip_limit;
            *count = clip_limit;
        }
    }
    let redistribute = overflow / histogram.len();
    let remainder = overflow % histogram.len();
    for (index, count) in histogram.iter_mut().enumerate() {
        *count += redistribute + usize::from(index < remainder);
    }

    let mut cdf = [0usize; 256];
    let mut running = 0usize;
    for (index, count) in histogram.iter().enumerate() {
        running += count;
        cdf[index] = running;
    }

    let Some(cdf_min) = cdf.iter().copied().find(|&count| count != 0) else {
        return Vec::new();
    };
    let denominator = pixels.len().saturating_sub(cdf_min);
    if denominator == 0 {
        return pixels.to_vec();
    }

    let mut luma_map = [0u8; 256];
    for (index, mapped) in luma_map.iter_mut().enumerate() {
        *mapped = (((cdf[index].saturating_sub(cdf_min)) * 255) / denominator) as u8;
    }

    pixels
        .iter()
        .map(|&pixel| {
            let source_luma = luma(pixel);
            let equalized_luma = luma_map[source_luma as usize];
            let balanced_luma =
                ((u16::from(source_luma) * 9 + u16::from(equalized_luma)) / 10) as u8;
            let target_luma = compress_highlights(balanced_luma);
            scale_pixel_to_luma(pixel, source_luma, target_luma)
        })
        .collect()
}

fn compress_highlights(luma: u8) -> u8 {
    const KNEE: u16 = 164;
    const MAX_LUMA: u16 = 204;
    let luma = u16::from(luma);
    if luma <= KNEE {
        return luma as u8;
    }

    (KNEE + ((luma - KNEE) * (MAX_LUMA - KNEE)) / (255 - KNEE)) as u8
}

fn luma(pixel: u32) -> u8 {
    let r = pixel & 255;
    let g = (pixel >> 8) & 255;
    let b = (pixel >> 16) & 255;
    ((77 * r + 150 * g + 29 * b) >> 8) as u8
}

fn scale_pixel_to_luma(pixel: u32, source_luma: u8, target_luma: u8) -> u32 {
    let r = pixel & 255;
    let g = (pixel >> 8) & 255;
    let b = (pixel >> 16) & 255;

    if source_luma == 0 {
        let v = u32::from(target_luma);
        return v | (v << 8) | (v << 16) | 0xff000000;
    }

    let scale = u32::from(target_luma) * 256 / u32::from(source_luma);
    let r = ((r * scale).min(255 * 256) >> 8).min(255);
    let g = ((g * scale).min(255 * 256) >> 8).min(255);
    let b = ((b * scale).min(255 * 256) >> 8).min(255);
    r | (g << 8) | (b << 16) | 0xff000000
}

fn save_png(path: &Path, pixels: &[u32]) -> Result<(), String> {
    let mut bytes = Vec::with_capacity(pixels.len() * 4);
    for &pixel in pixels {
        bytes.push((pixel & 255) as u8);
        bytes.push(((pixel >> 8) & 255) as u8);
        bytes.push(((pixel >> 16) & 255) as u8);
        bytes.push(255);
    }
    let image: RgbaImage = ImageBuffer::from_raw(OUT_W, OUT_H, bytes)
        .ok_or("Output buffer size does not match image dimensions")?;
    image
        .save(path)
        .map_err(|e| format!("Failed to save {}: {e}", path.display()))
}

fn enabled_validation_layers(entry: &Entry<'_>) -> Result<Vec<*const c_char>, String> {
    let layers = enumerate_instance_layers(entry)?;
    let has_validation = layers.iter().any(|layer| {
        let name = unsafe { CStr::from_ptr(layer.layerName.as_ptr()) };
        name == VALIDATION_LAYER
    });
    Ok(if has_validation {
        vec![VALIDATION_LAYER.as_ptr()]
    } else {
        Vec::new()
    })
}

fn enumerate_instance_layers(entry: &Entry<'_>) -> Result<Vec<VkLayerProperties>, String> {
    let mut count = 0;
    entry
        .vkEnumerateInstanceLayerProperties(&mut count, null_mut())
        .map_err(|e| format!("vkEnumerateInstanceLayerProperties count failed: {e:?}"))?;

    let mut layers = vec![VkLayerProperties::DEFAULT; count as usize];
    if count != 0 {
        entry
            .vkEnumerateInstanceLayerProperties(&mut count, layers.as_mut_ptr())
            .map_err(|e| format!("vkEnumerateInstanceLayerProperties failed: {e:?}"))?;
        layers.truncate(count as usize);
    }

    Ok(layers)
}

fn create_device<'inst>(
    instance: &'inst Instance<'inst>,
) -> Result<(Device<'inst>, PhysicalDevice<'inst>, u32), String> {
    let physical_device = instance
        .vkEnumeratePhysicalDevices()
        .map_err(|e| format!("vkEnumeratePhysicalDevices failed: {e:?}"))?
        .into_iter()
        .next()
        .ok_or("No physical devices found")?;
    let queue_family_index =
        find_compute_queue_family(&physical_device).ok_or("No compute queue family found")?;

    const PRIORITIES: &[f32; 1] = &[1.0];
    const VULKAN13_FEATURES: VkPhysicalDeviceVulkan13Features<'_> =
        VkPhysicalDeviceVulkan13Features::DEFAULT.with_synchronization2(VK_TRUE);
    let queue_infos = &[VkDeviceQueueCreateInfo::DEFAULT
        .with_queueFamilyIndex(queue_family_index)
        .with_pQueuePriorities(PRIORITIES)];
    let device_info = DEVICE_CREATE_INFO
        .with_pQueueCreateInfos(queue_infos)
        .with_pNext_VkPhysicalDeviceVulkan13Features(&VULKAN13_FEATURES);
    let device = physical_device
        .vkCreateDevice(&device_info, null())
        .map_err(|e| format!("vkCreateDevice failed: {e:?}"))?;
    Ok((device, physical_device, queue_family_index))
}

fn find_compute_queue_family(physical_device: &PhysicalDevice<'_>) -> Option<u32> {
    let mut count = 0;
    physical_device.vkGetPhysicalDeviceQueueFamilyProperties2(&mut count, null_mut());
    let mut props: Vec<_> =
        iter::repeat_n(VkQueueFamilyProperties2::DEFAULT, count as usize).collect();
    physical_device.vkGetPhysicalDeviceQueueFamilyProperties2(&mut count, props.as_mut_ptr());
    props
        .iter()
        .position(|p| {
            p.queueFamilyProperties.queueCount > 0
                && p.queueFamilyProperties
                    .queueFlags
                    .intersects(VkQueueFlagBits::COMPUTE)
        })
        .map(|index| index as u32)
}

fn create_storage_buffer<'a>(
    allocator: &'a Allocator<'a>,
    size: u64,
) -> Result<vk_alloc::AllocatedBuffer<'a>, String> {
    let buffer_usage_info =
        VkBufferUsageFlags2CreateInfo::DEFAULT.with_usage(VkBufferUsageFlagBits2::STORAGE_BUFFER);
    let buffer_info = VkBufferCreateInfo::DEFAULT
        .with_sharingMode(VkSharingMode::EXCLUSIVE)
        .with_pNext_VkBufferUsageFlags2CreateInfo(&buffer_usage_info)
        .with_size(size);
    allocator
        .create_buffer(
            &buffer_info,
            AllocationCreateInfo {
                memory_type_policy: vk_alloc::MemoryTypePolicy::HOST_VISIBLE.with_required_flags(
                    VkMemoryPropertyFlagBits::HOST_VISIBLE
                        | VkMemoryPropertyFlagBits::HOST_COHERENT,
                ),
                ..AllocationCreateInfo::new()
            },
        )
        .map_err(|e| format!("Buffer allocation failed: {e:?}"))
}

fn write_to_buffer<T: Copy>(
    allocation: &mut vk_alloc::Allocation,
    data: &[T],
) -> Result<(), String> {
    let slice = allocation
        .mapped_slice_mut::<T>(data.len())
        .map_err(|err| format!("Allocation is not host mapped: {err:?}"))?;
    slice.copy_from_slice(data);
    Ok(())
}

fn read_buffer<T: Copy>(allocation: &vk_alloc::Allocation, len: usize) -> Result<&[T], String> {
    let ptr = allocation.mapped_ptr().cast::<T>();
    if ptr.is_null() {
        return Err("Allocation is not host mapped".into());
    }
    Ok(unsafe { slice::from_raw_parts(ptr, len) })
}

fn create_descriptor_pool<'a>(device: &'a Device<'a>) -> Result<DescriptorPool<'a>, String> {
    let pool_sizes = [VkDescriptorPoolSize::DEFAULT
        .with_type(VkDescriptorType::STORAGE_BUFFER)
        .with_descriptorCount(2)];
    let pool_info = VkDescriptorPoolCreateInfo::DEFAULT
        .with_maxSets(1)
        .with_flags(VkDescriptorPoolCreateFlagBits::FREE_DESCRIPTOR_SET)
        .with_pPoolSizes(&pool_sizes);
    device
        .vkCreateDescriptorPool(&pool_info, null())
        .map_err(|e| format!("vkCreateDescriptorPool failed: {e:?}"))
}

fn create_descriptor_set<'a>(
    descriptor_pool: &'a DescriptorPool<'a>,
    layout: &DescriptorSetLayout<'a>,
    input: &Buffer<'a>,
    output: &Buffer<'a>,
    input_size: u64,
    output_size: u64,
) -> Result<Box<[DescriptorSet<'a>]>, String> {
    let layouts = [layout.raw()];
    let alloc_info = VkDescriptorSetAllocateInfo::DEFAULT
        .with_descriptorPool(descriptor_pool.raw())
        .with_pSetLayouts(&layouts);
    let descriptor_sets = descriptor_pool
        .vkAllocateDescriptorSets(&alloc_info)
        .map_err(|e| format!("vkAllocateDescriptorSets failed: {e:?}"))?;
    let descriptor_set = descriptor_sets
        .first()
        .ok_or("No descriptor sets allocated")?;

    let buffer_infos = [
        VkDescriptorBufferInfo::DEFAULT
            .with_buffer(input.raw())
            .with_offset(0)
            .with_range(input_size),
        VkDescriptorBufferInfo::DEFAULT
            .with_buffer(output.raw())
            .with_offset(0)
            .with_range(output_size),
    ];
    let writes = [
        VkWriteDescriptorSet::DEFAULT
            .with_descriptorType(VkDescriptorType::STORAGE_BUFFER)
            .with_dstBinding(0)
            .with_pBufferInfo(&buffer_infos[0..1])
            .with_dstSet(descriptor_set.raw()),
        VkWriteDescriptorSet::DEFAULT
            .with_descriptorType(VkDescriptorType::STORAGE_BUFFER)
            .with_dstBinding(1)
            .with_pBufferInfo(&buffer_infos[1..2])
            .with_dstSet(descriptor_set.raw()),
    ];
    descriptor_pool
        .device()
        .vkUpdateDescriptorSets(&writes, &[]);
    Ok(descriptor_sets)
}

fn create_compute_pipeline<'a>(
    device: &'a Device<'a>,
    descriptor_set_layout: &DescriptorSetLayout<'a>,
) -> Result<(PipelineLayout<'a>, Box<[Pipeline<'a>]>), String> {
    let layouts = [descriptor_set_layout.raw()];
    let push_constant_ranges = [VkPushConstantRange::DEFAULT
        .with_stageFlags(VkShaderStageFlagBits::COMPUTE)
        .with_offset(0)
        .with_size(size_of::<u32>() as u32)];
    let pipeline_layout_info = VkPipelineLayoutCreateInfo::DEFAULT
        .with_setLayoutCount(1)
        .with_pSetLayouts(&layouts)
        .with_pPushConstantRanges(&push_constant_ranges);
    let pipeline_layout = device
        .vkCreatePipelineLayout(&pipeline_layout_info, null())
        .map_err(|e| format!("vkCreatePipelineLayout failed: {e:?}"))?;
    let shader_module_info = VkShaderModuleCreateInfo::DEFAULT.with_pCode(IMAGE_EDIT_SPV);
    let shader_module = device
        .vkCreateShaderModule(&shader_module_info, null())
        .map_err(|e| format!("vkCreateShaderModule failed: {e:?}"))?;
    let stage = VkPipelineShaderStageCreateInfo::DEFAULT
        .with_stage(VkShaderStageFlagBits::COMPUTE)
        .with_pName(c"main".as_ptr())
        .with_module(shader_module.raw());
    let pipeline_info = VkComputePipelineCreateInfo::DEFAULT
        .with_stage(stage)
        .with_layout(pipeline_layout.raw());
    let pipelines = device
        .vkCreateComputePipelines(VkPipelineCache::NULL, &[pipeline_info], null())
        .map_err(|e| format!("vkCreateComputePipelines failed: {e:?}"))?;
    Ok((pipeline_layout, pipelines))
}

fn run_compute<'a>(
    device: &Device<'a>,
    queue: &Queue<'a>,
    queue_family_index: u32,
    pipeline: &Pipeline<'a>,
    layout: &PipelineLayout<'a>,
    descriptor_set: &DescriptorSet<'a>,
    image_count: u32,
) -> Result<(), String> {
    let pool_info = VkCommandPoolCreateInfo::DEFAULT
        .with_flags(VkCommandPoolCreateFlagBits::RESET_COMMAND_BUFFER)
        .with_queueFamilyIndex(queue_family_index);
    let command_pool = device
        .vkCreateCommandPool(&pool_info, null())
        .map_err(|e| format!("vkCreateCommandPool failed: {e:?}"))?;
    let command_buffer_info = VkCommandBufferAllocateInfo::DEFAULT
        .with_level(VkCommandBufferLevel::PRIMARY)
        .with_commandBufferCount(1)
        .with_commandPool(command_pool.raw());
    let command_buffers = command_pool
        .vkAllocateCommandBuffers(&command_buffer_info)
        .map_err(|e| format!("vkAllocateCommandBuffers failed: {e:?}"))?;
    let command_buffer = &command_buffers[0];

    command_buffer
        .vkBeginCommandBuffer(&VkCommandBufferBeginInfo::DEFAULT)
        .map_err(|e| format!("vkBeginCommandBuffer failed: {e:?}"))?;
    command_buffer.vkCmdBindPipeline(VkPipelineBindPoint::COMPUTE, pipeline.raw());
    let raw_descriptor_sets = [descriptor_set.raw()];
    let bind_descriptor_sets_info = VkBindDescriptorSetsInfo::DEFAULT
        .with_stageFlags(VkShaderStageFlagBits::COMPUTE)
        .with_pDescriptorSets(&raw_descriptor_sets)
        .with_layout(layout.raw());
    command_buffer.vkCmdBindDescriptorSets2(&bind_descriptor_sets_info);
    command_buffer.vkCmdPushConstants(
        layout.raw(),
        VkShaderStageFlagBits::COMPUTE,
        0,
        &image_count.to_ne_bytes(),
    );
    command_buffer.vkCmdDispatch(OUT_W.div_ceil(16), OUT_H.div_ceil(16), 1);
    command_buffer
        .vkEndCommandBuffer()
        .map_err(|e| format!("vkEndCommandBuffer failed: {e:?}"))?;

    let command_buffer_infos =
        [VkCommandBufferSubmitInfo::DEFAULT.with_commandBuffer(command_buffer.raw())];
    let submit = VkSubmitInfo2::DEFAULT.with_pCommandBufferInfos(&command_buffer_infos);
    queue
        .vkQueueSubmit2(&[submit], VkFence::NULL)
        .map_err(|e| format!("vkQueueSubmit2 failed: {e:?}"))?;
    queue
        .vkQueueWaitIdle()
        .map_err(|e| format!("vkQueueWaitIdle failed: {e:?}"))?;
    Ok(())
}
