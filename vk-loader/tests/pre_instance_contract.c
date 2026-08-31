#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <vulkan/vulkan.h>

static void require_result(VkResult actual, VkResult expected, const char *operation) {
    printf("%s=%d\n", operation, actual);
    if (actual != expected) {
        fprintf(stderr, "%s returned %d, expected %d\n", operation, actual, expected);
        exit(1);
    }
}

int main(void) {
    uint32_t version = 0;
    require_result(vkEnumerateInstanceVersion(&version), VK_SUCCESS,
                   "enumerate-instance-version");
    printf("instance-version=%u.%u.%u\n", VK_API_VERSION_MAJOR(version),
           VK_API_VERSION_MINOR(version), VK_API_VERSION_PATCH(version));

    uint32_t extension_count = 0;
    require_result(vkEnumerateInstanceExtensionProperties(NULL, &extension_count, NULL),
                   VK_SUCCESS, "enumerate-extensions-count");
    printf("extension-count=%u\n", extension_count);
    if (extension_count != 0) {
        VkExtensionProperties property;
        uint32_t capacity = 1;
        VkResult result =
            vkEnumerateInstanceExtensionProperties(NULL, &capacity, &property);
        require_result(result, extension_count == 1 ? VK_SUCCESS : VK_INCOMPLETE,
                       "enumerate-extensions-one");
        printf("extension-written=%u\n", capacity);
    }

    uint32_t missing_layer_count = 0;
    require_result(vkEnumerateInstanceExtensionProperties(
                       "VK_LAYER_DOES_NOT_EXIST", &missing_layer_count, NULL),
                   VK_ERROR_LAYER_NOT_PRESENT, "enumerate-missing-layer-extensions");

    const VkApplicationInfo application_info = {
        .sType = VK_STRUCTURE_TYPE_APPLICATION_INFO,
        .pApplicationName = "pre-instance-contract",
        .apiVersion = VK_API_VERSION_1_4,
    };
    VkInstanceCreateInfo create_info = {
        .sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        .pApplicationInfo = &application_info,
    };
    VkInstance instance = VK_NULL_HANDLE;
    require_result(vkCreateInstance(&create_info, NULL, &instance),
                   VK_ERROR_INCOMPATIBLE_DRIVER, "create-without-driver");

    const char *missing_layer = "VK_LAYER_DOES_NOT_EXIST";
    create_info.enabledLayerCount = 1;
    create_info.ppEnabledLayerNames = &missing_layer;
    require_result(vkCreateInstance(&create_info, NULL, &instance),
                   VK_ERROR_LAYER_NOT_PRESENT, "create-with-missing-layer");

    const char *missing_extension = "VK_EXTENSION_DOES_NOT_EXIST";
    create_info.enabledLayerCount = 0;
    create_info.ppEnabledLayerNames = NULL;
    create_info.enabledExtensionCount = 1;
    create_info.ppEnabledExtensionNames = &missing_extension;
    require_result(vkCreateInstance(&create_info, NULL, &instance),
                   VK_ERROR_INCOMPATIBLE_DRIVER,
                   "create-with-missing-extension-and-no-driver");

    puts("pre-instance-contract=pass");
    return 0;
}
