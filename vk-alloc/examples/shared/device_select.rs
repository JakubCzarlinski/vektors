use core::ffi::CStr;

use vk::{
    Instance, PhysicalDevice, VkPhysicalDeviceProperties2, VkQueueFamilyProperties2, VkQueueFlags,
};

pub(crate) fn queue_family_properties<'a>(
    physical_device: &'a PhysicalDevice<'a>,
) -> Vec<VkQueueFamilyProperties2<'a>> {
    let mut count = 0;
    physical_device.vkGetPhysicalDeviceQueueFamilyProperties2(&mut count, std::ptr::null_mut());
    let mut properties = vec![VkQueueFamilyProperties2::DEFAULT; count as usize];
    physical_device.vkGetPhysicalDeviceQueueFamilyProperties2(&mut count, properties.as_mut_ptr());
    properties.truncate(count as usize);
    properties
}

pub(crate) fn supports_queue_family(
    physical_device: &PhysicalDevice<'_>,
    queue_family_index: u32,
    required_flags: VkQueueFlags,
) -> bool {
    queue_family_properties(physical_device)
        .get(queue_family_index as usize)
        .is_some_and(|properties| {
            properties
                .queueFamilyProperties
                .queueFlags
                .contains(required_flags)
        })
}

pub(crate) fn find_compute_queue_family(physical_device: &PhysicalDevice<'_>) -> Option<u32> {
    queue_family_properties(physical_device)
        .iter()
        .enumerate()
        .find_map(|(index, properties)| {
            if properties
                .queueFamilyProperties
                .queueFlags
                .intersects(vk::VkQueueFlagBits::COMPUTE)
            {
                Some(index as u32)
            } else {
                None
            }
        })
}

pub(crate) fn device_name(physical_device: &PhysicalDevice<'_>) -> String {
    let mut properties = VkPhysicalDeviceProperties2::DEFAULT;
    physical_device.vkGetPhysicalDeviceProperties2(&mut properties);
    let name = unsafe { CStr::from_ptr(properties.properties.deviceName.as_ptr()) };
    name.to_string_lossy().into_owned()
}

pub(crate) fn select_single_device<'inst>(
    instance: &'inst Instance<'inst>,
) -> Result<(PhysicalDevice<'inst>, u32), String> {
    let physical_devices = instance
        .vkEnumeratePhysicalDevices()
        .map_err(|err| format!("vkEnumeratePhysicalDevices failed: {err:?}"))?;
    for physical_device in physical_devices {
        if let Some(queue_family_index) = find_compute_queue_family(&physical_device) {
            return Ok((physical_device, queue_family_index));
        }
    }
    Err("no physical device with a compute-capable queue family was found".into())
}
