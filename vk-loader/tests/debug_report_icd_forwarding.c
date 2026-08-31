#include <dlfcn.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include <vulkan/vulkan.h>

typedef uint32_t(VKAPI_PTR *PFN_message_count)(void);
typedef void(VKAPI_PTR *PFN_reset_count)(void);

static uint32_t report_from_utils_count;
static uint32_t utils_from_report_count;
static char report_order[3];
static uint32_t report_order_length;
static uint32_t object_allocations;
static uint32_t object_frees;
static void *object_allocated_pointers[16];

static int object_pointer_index(void *memory) {
    for (int index = 0; index < 16; index++) {
        if (object_allocated_pointers[index] == memory) {
            return index;
        }
    }
    return -1;
}

static VKAPI_ATTR void *VKAPI_CALL object_allocate(
    void *user_data, size_t size, size_t alignment,
    VkSystemAllocationScope allocation_scope) {
    (void)user_data;
    (void)allocation_scope;
    size_t extra = alignment - 1 + sizeof(void *);
    void *base = malloc(size + extra);
    if (base == NULL) {
        return NULL;
    }
    uintptr_t aligned = ((uintptr_t)base + sizeof(void *) + alignment - 1) &
                        ~(uintptr_t)(alignment - 1);
    ((void **)aligned)[-1] = base;
    for (int index = 0; index < 16; index++) {
        if (object_allocated_pointers[index] == NULL) {
            object_allocated_pointers[index] = (void *)aligned;
            break;
        }
    }
    object_allocations++;
    return (void *)aligned;
}

static VKAPI_ATTR void *VKAPI_CALL object_reallocate(
    void *user_data, void *original, size_t size, size_t alignment,
    VkSystemAllocationScope allocation_scope) {
    if (original != NULL) {
        int index = object_pointer_index(original);
        if (index >= 0) {
            object_allocated_pointers[index] = NULL;
            free(((void **)original)[-1]);
            object_frees++;
        } else {
            free(original);
        }
    }
    return object_allocate(user_data, size, alignment, allocation_scope);
}

static VKAPI_ATTR void VKAPI_CALL object_free(void *user_data, void *memory) {
    (void)user_data;
    if (memory != NULL) {
        int index = object_pointer_index(memory);
        if (index >= 0) {
            object_allocated_pointers[index] = NULL;
            free(((void **)memory)[-1]);
            object_frees++;
        } else {
            // Upstream currently allocates the opaque index from the instance
            // heap but releases it through this object allocator.
            free(memory);
        }
    }
}

static VKAPI_ATTR void VKAPI_CALL object_internal_allocate(
    void *user_data, size_t size, VkInternalAllocationType allocation_type,
    VkSystemAllocationScope allocation_scope) {
    (void)user_data;
    (void)size;
    (void)allocation_type;
    (void)allocation_scope;
}

static VKAPI_ATTR void VKAPI_CALL object_internal_free(
    void *user_data, size_t size, VkInternalAllocationType allocation_type,
    VkSystemAllocationScope allocation_scope) {
    (void)user_data;
    (void)size;
    (void)allocation_type;
    (void)allocation_scope;
}

static VKAPI_ATTR VkBool32 VKAPI_CALL report_callback(
    VkDebugReportFlagsEXT flags, VkDebugReportObjectTypeEXT object_type,
    uint64_t object, size_t location, int32_t message_code,
    const char *layer_prefix, const char *message, void *user_data) {
    (void)user_data;
    if (flags == VK_DEBUG_REPORT_INFORMATION_BIT_EXT &&
        object_type == VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT && object == 0x1234 &&
        location == 0 && message_code == 23 && layer_prefix != NULL &&
        message != NULL) {
        report_from_utils_count++;
    }
    return VK_FALSE;
}

static VKAPI_ATTR VkBool32 VKAPI_CALL utils_callback(
    VkDebugUtilsMessageSeverityFlagBitsEXT severity,
    VkDebugUtilsMessageTypeFlagsEXT message_types,
    const VkDebugUtilsMessengerCallbackDataEXT *callback_data, void *user_data) {
    (void)user_data;
    if (severity == VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT &&
        message_types == VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT &&
        callback_data != NULL && callback_data->objectCount == 1 &&
        callback_data->pObjects != NULL &&
        callback_data->pObjects[0].objectType == VK_OBJECT_TYPE_IMAGE &&
        callback_data->pObjects[0].objectHandle == 0x5678 &&
        callback_data->messageIdNumber == 29) {
        utils_from_report_count++;
    }
    return VK_FALSE;
}

static VKAPI_ATTR VkBool32 VKAPI_CALL ordering_report_callback(
    VkDebugReportFlagsEXT flags, VkDebugReportObjectTypeEXT object_type,
    uint64_t object, size_t location, int32_t message_code,
    const char *layer_prefix, const char *message, void *user_data) {
    (void)flags;
    (void)object_type;
    (void)object;
    (void)location;
    (void)message_code;
    (void)layer_prefix;
    (void)message;
    if (report_order_length < 2) {
        report_order[report_order_length++] = *(const char *)user_data;
        report_order[report_order_length] = '\0';
    }
    return VK_FALSE;
}

int main(void) {
    const char *icd_path = getenv("VK_LOADER_FORWARDING_ICD");
    if (icd_path == NULL) {
        return 2;
    }
    void *icd = dlopen(icd_path, RTLD_NOW | RTLD_LOCAL);
    if (icd == NULL) {
        return 3;
    }
    PFN_message_count message_count =
        (PFN_message_count)dlsym(icd, "debug_report_icd_message_count");
    PFN_reset_count reset_count =
        (PFN_reset_count)dlsym(icd, "debug_report_icd_reset");
    if (message_count == NULL || reset_count == NULL) {
        return 4;
    }
    reset_count();

    const char *extensions[] = {
        VK_EXT_DEBUG_REPORT_EXTENSION_NAME,
        VK_EXT_DEBUG_UTILS_EXTENSION_NAME,
    };
    VkApplicationInfo application = {
        .sType = VK_STRUCTURE_TYPE_APPLICATION_INFO,
        .apiVersion = VK_API_VERSION_1_0,
    };
    VkInstanceCreateInfo create_info = {
        .sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        .pApplicationInfo = &application,
        .enabledExtensionCount = 2,
        .ppEnabledExtensionNames = extensions,
    };
    VkInstance instance = VK_NULL_HANDLE;
    VkResult result = vkCreateInstance(&create_info, NULL, &instance);
    if (result != VK_SUCCESS) {
        printf("create=%d\n", result);
        return 5;
    }
    PFN_vkCreateDebugReportCallbackEXT create_report_callback =
        (PFN_vkCreateDebugReportCallbackEXT)vkGetInstanceProcAddr(
            instance, "vkCreateDebugReportCallbackEXT");
    PFN_vkDestroyDebugReportCallbackEXT destroy_report_callback =
        (PFN_vkDestroyDebugReportCallbackEXT)vkGetInstanceProcAddr(
            instance, "vkDestroyDebugReportCallbackEXT");
    PFN_vkCreateDebugUtilsMessengerEXT create_utils_messenger =
        (PFN_vkCreateDebugUtilsMessengerEXT)vkGetInstanceProcAddr(
            instance, "vkCreateDebugUtilsMessengerEXT");
    PFN_vkDestroyDebugUtilsMessengerEXT destroy_utils_messenger =
        (PFN_vkDestroyDebugUtilsMessengerEXT)vkGetInstanceProcAddr(
            instance, "vkDestroyDebugUtilsMessengerEXT");
    PFN_vkSubmitDebugUtilsMessageEXT submit_utils =
        (PFN_vkSubmitDebugUtilsMessageEXT)vkGetInstanceProcAddr(
            instance, "vkSubmitDebugUtilsMessageEXT");
    PFN_vkDebugReportMessageEXT report =
        (PFN_vkDebugReportMessageEXT)vkGetInstanceProcAddr(instance, "vkDebugReportMessageEXT");
    if (create_report_callback == NULL || destroy_report_callback == NULL ||
        create_utils_messenger == NULL || destroy_utils_messenger == NULL ||
        submit_utils == NULL || report == NULL) {
        vkDestroyInstance(instance, NULL);
        return 6;
    }

    VkDebugReportCallbackCreateInfoEXT report_info = {
        .sType = VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT,
        .flags = VK_DEBUG_REPORT_INFORMATION_BIT_EXT,
        .pfnCallback = report_callback,
    };
    VkDebugReportCallbackEXT report_handle = VK_NULL_HANDLE;
    if (create_report_callback(instance, &report_info, NULL, &report_handle) != VK_SUCCESS) {
        vkDestroyInstance(instance, NULL);
        return 7;
    }
    VkDebugUtilsObjectNameInfoEXT submit_object = {
        .sType = VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_NAME_INFO_EXT,
        .objectType = VK_OBJECT_TYPE_BUFFER,
        .objectHandle = 0x1234,
    };
    VkDebugUtilsMessengerCallbackDataEXT submit_data = {
        .sType = VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT,
        .pMessageIdName = "utils-to-report",
        .messageIdNumber = 23,
        .pMessage = "converted report",
        .objectCount = 1,
        .pObjects = &submit_object,
    };
    submit_utils(instance, VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT,
                 VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT, &submit_data);

    VkDebugUtilsMessengerCreateInfoEXT utils_info = {
        .sType = VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
        .messageSeverity = VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT,
        .messageType = VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT,
        .pfnUserCallback = utils_callback,
    };
    VkDebugUtilsMessengerEXT utils_handle = VK_NULL_HANDLE;
    if (create_utils_messenger(instance, &utils_info, NULL, &utils_handle) != VK_SUCCESS) {
        destroy_report_callback(instance, report_handle, NULL);
        vkDestroyInstance(instance, NULL);
        return 8;
    }
    report(instance, VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT,
           VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT, 0x5678, 0, 29,
           "report-to-utils", "converted utils");

    char first_name = 'A';
    char second_name = 'B';
    VkDebugReportCallbackCreateInfoEXT first_order_info = {
        .sType = VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT,
        .flags = VK_DEBUG_REPORT_INFORMATION_BIT_EXT,
        .pfnCallback = ordering_report_callback,
        .pUserData = &first_name,
    };
    VkDebugReportCallbackCreateInfoEXT second_order_info = first_order_info;
    second_order_info.pUserData = &second_name;
    VkDebugReportCallbackEXT first_order_handle = VK_NULL_HANDLE;
    VkDebugReportCallbackEXT second_order_handle = VK_NULL_HANDLE;
    if (create_report_callback(instance, &first_order_info, NULL,
                               &first_order_handle) != VK_SUCCESS ||
        create_report_callback(instance, &second_order_info, NULL,
                               &second_order_handle) != VK_SUCCESS) {
        destroy_utils_messenger(instance, utils_handle, NULL);
        destroy_report_callback(instance, report_handle, NULL);
        vkDestroyInstance(instance, NULL);
        return 9;
    }
    report(instance, VK_DEBUG_REPORT_INFORMATION_BIT_EXT,
           VK_DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT, 0x9abc, 0, 31,
           "ordering", "ordering message");

    VkAllocationCallbacks object_allocator = {
        .pfnAllocation = object_allocate,
        .pfnReallocation = object_reallocate,
        .pfnFree = object_free,
        .pfnInternalAllocation = object_internal_allocate,
        .pfnInternalFree = object_internal_free,
    };
    VkDebugReportCallbackCreateInfoEXT allocated_report_info = {
        .sType = VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT,
        .flags = VK_DEBUG_REPORT_ERROR_BIT_EXT,
        .pfnCallback = report_callback,
    };
    VkDebugReportCallbackEXT allocated_report_handle = VK_NULL_HANDLE;
    if (create_report_callback(instance, &allocated_report_info, &object_allocator,
                               &allocated_report_handle) != VK_SUCCESS) {
        return 10;
    }
    destroy_report_callback(instance, allocated_report_handle, &object_allocator);

    printf("icd=%u report_from_utils=%u utils_from_report=%u order=%s alloc=%u free=%u\n",
           message_count(), report_from_utils_count, utils_from_report_count,
           report_order, object_allocations, object_frees);
    destroy_report_callback(instance, second_order_handle, NULL);
    destroy_report_callback(instance, first_order_handle, NULL);
    destroy_utils_messenger(instance, utils_handle, NULL);
    destroy_report_callback(instance, report_handle, NULL);
    vkDestroyInstance(instance, NULL);
    dlclose(icd);
    return 0;
}
