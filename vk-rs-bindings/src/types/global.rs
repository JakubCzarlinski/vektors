/// Marker trait for Vulkan structs that are valid in the `pNext` chain rooted at `Root`.
///
/// # Safety
/// Implementors must be Vulkan structs whose `structextends` metadata includes `Root`.
pub unsafe trait VkPNextExtends<Root> {}
