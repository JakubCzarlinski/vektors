#include <stdatomic.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include <vulkan/vk_icd.h>
#include <vulkan/vulkan.h>

struct fake_instance {
    void *dispatch;
};

static _Atomic uint32_t report_count;

VKAPI_ATTR uint32_t VKAPI_CALL debug_report_icd_message_count(void) {
    return atomic_load_explicit(&report_count, memory_order_relaxed);
}

VKAPI_ATTR void VKAPI_CALL debug_report_icd_reset(void) {
    atomic_store_explicit(&report_count, 0, memory_order_relaxed);
}

static VKAPI_ATTR VkResult VKAPI_CALL fake_CreateInstance(
    const VkInstanceCreateInfo *create_info,
    const VkAllocationCallbacks *allocator,
    VkInstance *instance) {
    (void)create_info;
    (void)allocator;
    struct fake_instance *created = calloc(1, sizeof(*created));
    if (created == NULL) {
        return VK_ERROR_OUT_OF_HOST_MEMORY;
    }
    *instance = (VkInstance)created;
    return VK_SUCCESS;
}

static VKAPI_ATTR void VKAPI_CALL fake_DestroyInstance(
    VkInstance instance,
    const VkAllocationCallbacks *allocator) {
    (void)allocator;
    free((void *)instance);
}

static VKAPI_ATTR VkResult VKAPI_CALL fake_EnumerateInstanceExtensionProperties(
    const char *layer_name,
    uint32_t *property_count,
    VkExtensionProperties *properties) {
    if (layer_name != NULL) {
        return VK_ERROR_LAYER_NOT_PRESENT;
    }
    if (properties == NULL) {
        *property_count = 1;
        return VK_SUCCESS;
    }
    if (*property_count == 0) {
        return VK_INCOMPLETE;
    }
    memset(properties, 0, sizeof(*properties));
    memcpy(properties->extensionName, VK_EXT_DEBUG_REPORT_EXTENSION_NAME,
           sizeof(VK_EXT_DEBUG_REPORT_EXTENSION_NAME));
    properties->specVersion = VK_EXT_DEBUG_REPORT_SPEC_VERSION;
    *property_count = 1;
    return VK_SUCCESS;
}

static VKAPI_ATTR VkResult VKAPI_CALL fake_EnumeratePhysicalDevices(
    VkInstance instance,
    uint32_t *physical_device_count,
    VkPhysicalDevice *physical_devices) {
    (void)instance;
    (void)physical_devices;
    *physical_device_count = 0;
    return VK_SUCCESS;
}

static VKAPI_ATTR void VKAPI_CALL fake_GetPhysicalDeviceFeatures(
    VkPhysicalDevice physical_device, VkPhysicalDeviceFeatures *features) {
    (void)physical_device;
    memset(features, 0, sizeof(*features));
}

static VKAPI_ATTR void VKAPI_CALL fake_GetPhysicalDeviceFormatProperties(
    VkPhysicalDevice physical_device, VkFormat format,
    VkFormatProperties *properties) {
    (void)physical_device;
    (void)format;
    memset(properties, 0, sizeof(*properties));
}

static VKAPI_ATTR VkResult VKAPI_CALL fake_GetPhysicalDeviceImageFormatProperties(
    VkPhysicalDevice physical_device, VkFormat format, VkImageType type,
    VkImageTiling tiling, VkImageUsageFlags usage, VkImageCreateFlags flags,
    VkImageFormatProperties *properties) {
    (void)physical_device;
    (void)format;
    (void)type;
    (void)tiling;
    (void)usage;
    (void)flags;
    memset(properties, 0, sizeof(*properties));
    return VK_ERROR_FORMAT_NOT_SUPPORTED;
}

static VKAPI_ATTR void VKAPI_CALL fake_GetPhysicalDeviceProperties(
    VkPhysicalDevice physical_device, VkPhysicalDeviceProperties *properties) {
    (void)physical_device;
    memset(properties, 0, sizeof(*properties));
    properties->apiVersion = VK_API_VERSION_1_0;
}

static VKAPI_ATTR void VKAPI_CALL fake_GetPhysicalDeviceQueueFamilyProperties(
    VkPhysicalDevice physical_device, uint32_t *property_count,
    VkQueueFamilyProperties *properties) {
    (void)physical_device;
    (void)properties;
    *property_count = 0;
}

static VKAPI_ATTR void VKAPI_CALL fake_GetPhysicalDeviceMemoryProperties(
    VkPhysicalDevice physical_device, VkPhysicalDeviceMemoryProperties *properties) {
    (void)physical_device;
    memset(properties, 0, sizeof(*properties));
}

static VKAPI_ATTR PFN_vkVoidFunction VKAPI_CALL fake_GetDeviceProcAddr(
    VkDevice device, const char *name) {
    (void)device;
    (void)name;
    return NULL;
}

static VKAPI_ATTR VkResult VKAPI_CALL fake_CreateDevice(
    VkPhysicalDevice physical_device, const VkDeviceCreateInfo *create_info,
    const VkAllocationCallbacks *allocator, VkDevice *device) {
    (void)physical_device;
    (void)create_info;
    (void)allocator;
    (void)device;
    return VK_ERROR_INITIALIZATION_FAILED;
}

static VKAPI_ATTR VkResult VKAPI_CALL fake_EnumerateDeviceExtensionProperties(
    VkPhysicalDevice physical_device, const char *layer_name,
    uint32_t *property_count, VkExtensionProperties *properties) {
    (void)physical_device;
    (void)layer_name;
    (void)properties;
    *property_count = 0;
    return VK_SUCCESS;
}

static VKAPI_ATTR void VKAPI_CALL fake_GetPhysicalDeviceSparseImageFormatProperties(
    VkPhysicalDevice physical_device, VkFormat format, VkImageType type,
    VkSampleCountFlagBits samples, VkImageUsageFlags usage, VkImageTiling tiling,
    uint32_t *property_count, VkSparseImageFormatProperties *properties) {
    (void)physical_device;
    (void)format;
    (void)type;
    (void)samples;
    (void)usage;
    (void)tiling;
    (void)properties;
    *property_count = 0;
}

static VKAPI_ATTR void VKAPI_CALL fake_DebugReportMessageEXT(
    VkInstance instance,
    VkDebugReportFlagsEXT flags,
    VkDebugReportObjectTypeEXT object_type,
    uint64_t object,
    size_t location,
    int32_t message_code,
    const char *layer_prefix,
    const char *message) {
    (void)instance;
    (void)flags;
    (void)object_type;
    (void)object;
    (void)location;
    (void)message_code;
    (void)layer_prefix;
    (void)message;
    atomic_fetch_add_explicit(&report_count, 1, memory_order_relaxed);
}

VKAPI_ATTR VkResult VKAPI_CALL
vk_icdNegotiateLoaderICDInterfaceVersion(uint32_t *supported_version) {
    if (*supported_version > 7) {
        *supported_version = 7;
    }
    return VK_SUCCESS;
}

VKAPI_ATTR PFN_vkVoidFunction VKAPI_CALL
vk_icdGetInstanceProcAddr(VkInstance instance, const char *name) {
    (void)instance;
    if (strcmp(name, "vkCreateInstance") == 0) {
        return (PFN_vkVoidFunction)fake_CreateInstance;
    }
    if (strcmp(name, "vkDestroyInstance") == 0) {
        return (PFN_vkVoidFunction)fake_DestroyInstance;
    }
    if (strcmp(name, "vkEnumerateInstanceExtensionProperties") == 0) {
        return (PFN_vkVoidFunction)fake_EnumerateInstanceExtensionProperties;
    }
    if (strcmp(name, "vkEnumeratePhysicalDevices") == 0) {
        return (PFN_vkVoidFunction)fake_EnumeratePhysicalDevices;
    }
    if (strcmp(name, "vkGetPhysicalDeviceFeatures") == 0) {
        return (PFN_vkVoidFunction)fake_GetPhysicalDeviceFeatures;
    }
    if (strcmp(name, "vkGetPhysicalDeviceFormatProperties") == 0) {
        return (PFN_vkVoidFunction)fake_GetPhysicalDeviceFormatProperties;
    }
    if (strcmp(name, "vkGetPhysicalDeviceImageFormatProperties") == 0) {
        return (PFN_vkVoidFunction)fake_GetPhysicalDeviceImageFormatProperties;
    }
    if (strcmp(name, "vkGetPhysicalDeviceProperties") == 0) {
        return (PFN_vkVoidFunction)fake_GetPhysicalDeviceProperties;
    }
    if (strcmp(name, "vkGetPhysicalDeviceQueueFamilyProperties") == 0) {
        return (PFN_vkVoidFunction)fake_GetPhysicalDeviceQueueFamilyProperties;
    }
    if (strcmp(name, "vkGetPhysicalDeviceMemoryProperties") == 0) {
        return (PFN_vkVoidFunction)fake_GetPhysicalDeviceMemoryProperties;
    }
    if (strcmp(name, "vkGetDeviceProcAddr") == 0) {
        return (PFN_vkVoidFunction)fake_GetDeviceProcAddr;
    }
    if (strcmp(name, "vkCreateDevice") == 0) {
        return (PFN_vkVoidFunction)fake_CreateDevice;
    }
    if (strcmp(name, "vkEnumerateDeviceExtensionProperties") == 0) {
        return (PFN_vkVoidFunction)fake_EnumerateDeviceExtensionProperties;
    }
    if (strcmp(name, "vkGetPhysicalDeviceSparseImageFormatProperties") == 0) {
        return (PFN_vkVoidFunction)fake_GetPhysicalDeviceSparseImageFormatProperties;
    }
    if (strcmp(name, "vkDebugReportMessageEXT") == 0) {
        return (PFN_vkVoidFunction)fake_DebugReportMessageEXT;
    }
    return NULL;
}

VKAPI_ATTR PFN_vkVoidFunction VKAPI_CALL
vkGetInstanceProcAddr(VkInstance instance, const char *name) {
    return vk_icdGetInstanceProcAddr(instance, name);
}
