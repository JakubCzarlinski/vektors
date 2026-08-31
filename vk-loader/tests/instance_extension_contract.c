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
    const VkApplicationInfo application_info = {
        .sType = VK_STRUCTURE_TYPE_APPLICATION_INFO,
        .pApplicationName = "instance-extension-contract",
        .apiVersion = VK_API_VERSION_1_0,
    };
    VkInstanceCreateInfo create_info = {
        .sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        .pApplicationInfo = &application_info,
    };
    VkInstance instance = VK_NULL_HANDLE;

    const char *missing_extension = "VK_EXTENSION_DOES_NOT_EXIST";
    create_info.enabledExtensionCount = 1;
    create_info.ppEnabledExtensionNames = &missing_extension;
    require_result(vkCreateInstance(&create_info, NULL, &instance),
                   VK_ERROR_EXTENSION_NOT_PRESENT,
                   "create-with-missing-extension-and-valid-driver");

    create_info.enabledExtensionCount = 0;
    create_info.ppEnabledExtensionNames = NULL;
    require_result(vkCreateInstance(&create_info, NULL, &instance), VK_SUCCESS,
                   "create-with-valid-driver");
    vkDestroyInstance(instance, NULL);

    puts("instance-extension-contract=pass");
    return 0;
}
