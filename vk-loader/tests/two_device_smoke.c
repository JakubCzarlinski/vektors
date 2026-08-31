#define VK_USE_PLATFORM_XCB_KHR
#define VK_USE_PLATFORM_WAYLAND_KHR
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <wayland-client.h>
#include <xcb/xcb.h>
#include <vulkan/vulkan.h>

static void require(VkResult result, const char *operation) {
    if (result != VK_SUCCESS) {
        fprintf(stderr, "%s failed: %d\n", operation, result);
        exit(1);
    }
}

static VKAPI_ATTR VkBool32 VKAPI_CALL debug_report(
    VkDebugReportFlagsEXT flags, VkDebugReportObjectTypeEXT object_type, uint64_t object,
    size_t location, int32_t message_code, const char *layer_prefix, const char *message,
    void *user_data) {
    (void)flags;
    (void)object_type;
    (void)object;
    (void)location;
    (void)message_code;
    (void)layer_prefix;
    (void)message;
    (void)user_data;
    return VK_FALSE;
}

static void registry_global(void *data, struct wl_registry *registry, uint32_t name,
                            const char *interface, uint32_t version) {
    struct wl_compositor **compositor = data;
    if (strcmp(interface, wl_compositor_interface.name) == 0) {
        uint32_t bind_version = version < 4 ? version : 4;
        *compositor = wl_registry_bind(registry, name, &wl_compositor_interface,
                                       bind_version);
    }
}

static void registry_global_remove(void *data, struct wl_registry *registry, uint32_t name) {
    (void)data;
    (void)registry;
    (void)name;
}

static const struct wl_registry_listener registry_listener = {
    .global = registry_global,
    .global_remove = registry_global_remove,
};

int main(void) {
    const VkApplicationInfo application_info = {
        .sType = VK_STRUCTURE_TYPE_APPLICATION_INFO,
        .pApplicationName = "two-device-smoke",
        .apiVersion = VK_API_VERSION_1_4,
    };
    const VkDebugReportCallbackCreateInfoEXT debug_info = {
        .sType = VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT,
        .flags = VK_DEBUG_REPORT_WARNING_BIT_EXT | VK_DEBUG_REPORT_ERROR_BIT_EXT,
        .pfnCallback = debug_report,
    };
    const char *instance_extensions[] = {VK_EXT_DEBUG_REPORT_EXTENSION_NAME,
                                         VK_KHR_SURFACE_EXTENSION_NAME,
                                         VK_KHR_XCB_SURFACE_EXTENSION_NAME,
                                         VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME,
                                         VK_KHR_DISPLAY_EXTENSION_NAME,
                                         VK_KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME,
                                         VK_KHR_SURFACE_PROTECTED_CAPABILITIES_EXTENSION_NAME,
                                         VK_EXT_SURFACE_MAINTENANCE_1_EXTENSION_NAME,
                                         VK_KHR_DEVICE_GROUP_CREATION_EXTENSION_NAME};
    const VkInstanceCreateInfo instance_info = {
        .sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        .pNext = &debug_info,
        .pApplicationInfo = &application_info,
        .enabledExtensionCount = 9,
        .ppEnabledExtensionNames = instance_extensions,
    };
    VkInstance instance = VK_NULL_HANDLE;
    require(vkCreateInstance(&instance_info, NULL, &instance), "vkCreateInstance");
    PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR get_video_capabilities =
        (PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR)vkGetInstanceProcAddr(
            instance, "vkGetPhysicalDeviceVideoCapabilitiesKHR");
    PFN_vkCreateDebugReportCallbackEXT create_debug_report =
        (PFN_vkCreateDebugReportCallbackEXT)vkGetInstanceProcAddr(
            instance, "vkCreateDebugReportCallbackEXT");
    PFN_vkDestroyDebugReportCallbackEXT destroy_debug_report =
        (PFN_vkDestroyDebugReportCallbackEXT)vkGetInstanceProcAddr(
            instance, "vkDestroyDebugReportCallbackEXT");
    PFN_vkEnumeratePhysicalDeviceGroupsKHR enumerate_groups_khr =
        (PFN_vkEnumeratePhysicalDeviceGroupsKHR)vkGetInstanceProcAddr(
            instance, "vkEnumeratePhysicalDeviceGroupsKHR");
    VkDebugReportCallbackEXT debug_callback = VK_NULL_HANDLE;
    if (create_debug_report != NULL) {
        require(create_debug_report(instance, &debug_info, NULL, &debug_callback),
                "vkCreateDebugReportCallbackEXT");
    }
    xcb_connection_t *connection = xcb_connect(NULL, NULL);
    if (connection == NULL || xcb_connection_has_error(connection)) {
        fputs("no XCB connection\n", stderr);
        return 77;
    }
    const xcb_setup_t *setup = xcb_get_setup(connection);
    xcb_screen_iterator_t screens = xcb_setup_roots_iterator(setup);
    xcb_screen_t *screen = screens.data;
    if (screen == NULL) {
        return 77;
    }
    xcb_window_t window = xcb_generate_id(connection);
    xcb_create_window(connection, XCB_COPY_FROM_PARENT, window, screen->root, 0, 0, 64, 64,
                      0, XCB_WINDOW_CLASS_INPUT_OUTPUT, screen->root_visual, 0, NULL);
    xcb_flush(connection);
    PFN_vkCreateXcbSurfaceKHR create_xcb_surface =
        (PFN_vkCreateXcbSurfaceKHR)vkGetInstanceProcAddr(instance, "vkCreateXcbSurfaceKHR");
    PFN_vkDestroySurfaceKHR destroy_surface =
        (PFN_vkDestroySurfaceKHR)vkGetInstanceProcAddr(instance, "vkDestroySurfaceKHR");
    VkSurfaceKHR surface = VK_NULL_HANDLE;
    const VkXcbSurfaceCreateInfoKHR surface_info = {
        .sType = VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR,
        .connection = connection,
        .window = window,
    };
    require(create_xcb_surface(instance, &surface_info, NULL, &surface),
            "vkCreateXcbSurfaceKHR");
    struct wl_display *wayland_display = wl_display_connect(NULL);
    if (wayland_display == NULL) {
        return 77;
    }
    struct wl_registry *registry = wl_display_get_registry(wayland_display);
    struct wl_compositor *compositor = NULL;
    wl_registry_add_listener(registry, &registry_listener, &compositor);
    wl_display_roundtrip(wayland_display);
    if (compositor == NULL) {
        return 77;
    }
    struct wl_surface *wayland_window = wl_compositor_create_surface(compositor);
    PFN_vkCreateWaylandSurfaceKHR create_wayland_surface =
        (PFN_vkCreateWaylandSurfaceKHR)vkGetInstanceProcAddr(
            instance, "vkCreateWaylandSurfaceKHR");
    const VkWaylandSurfaceCreateInfoKHR wayland_surface_info = {
        .sType = VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR,
        .display = wayland_display,
        .surface = wayland_window,
    };
    VkSurfaceKHR wayland_surface = VK_NULL_HANDLE;
    require(create_wayland_surface(instance, &wayland_surface_info, NULL,
                                   &wayland_surface),
            "vkCreateWaylandSurfaceKHR");

    uint32_t physical_device_count = 1;
    VkPhysicalDevice physical_device = VK_NULL_HANDLE;
    require(vkEnumeratePhysicalDevices(instance, &physical_device_count, &physical_device),
            "vkEnumeratePhysicalDevices");
    if (physical_device_count == 0) {
        fputs("no physical devices\n", stderr);
        return 77;
    }

    uint32_t queue_family_count = 0;
    vkGetPhysicalDeviceQueueFamilyProperties(physical_device, &queue_family_count, NULL);
    VkQueueFamilyProperties *queue_families =
        calloc(queue_family_count, sizeof(*queue_families));
    if (queue_families == NULL) {
        return 1;
    }
    vkGetPhysicalDeviceQueueFamilyProperties(physical_device, &queue_family_count,
                                             queue_families);
    uint32_t queue_family = UINT32_MAX;
    for (uint32_t index = 0; index < queue_family_count; ++index) {
        if (queue_families[index].queueCount != 0) {
            queue_family = index;
            break;
        }
    }
    free(queue_families);
    if (queue_family == UINT32_MAX) {
        fputs("no queue families\n", stderr);
        return 77;
    }
    for (uint32_t index = 0; index < queue_family_count; ++index) {
        VkBool32 supported = VK_FALSE;
        require(vkGetPhysicalDeviceSurfaceSupportKHR(physical_device, index, surface,
                                                     &supported),
                "vkGetPhysicalDeviceSurfaceSupportKHR");
    }
    VkSurfaceCapabilitiesKHR surface_capabilities;
    require(vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physical_device, surface,
                                                      &surface_capabilities),
            "vkGetPhysicalDeviceSurfaceCapabilitiesKHR");
    uint32_t surface_format_count = 0;
    require(vkGetPhysicalDeviceSurfaceFormatsKHR(physical_device, surface,
                                                 &surface_format_count, NULL),
            "vkGetPhysicalDeviceSurfaceFormatsKHR");
    uint32_t present_mode_count = 0;
    require(vkGetPhysicalDeviceSurfacePresentModesKHR(physical_device, surface,
                                                      &present_mode_count, NULL),
            "vkGetPhysicalDeviceSurfacePresentModesKHR");
    VkPresentModeKHR *present_modes = calloc(present_mode_count, sizeof(*present_modes));
    if (present_mode_count != 0 && present_modes == NULL) {
        return 1;
    }
    require(vkGetPhysicalDeviceSurfacePresentModesKHR(
                physical_device, surface, &present_mode_count, present_modes),
            "vkGetPhysicalDeviceSurfacePresentModesKHR");
    PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR get_surface_capabilities2 =
        (PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR)vkGetInstanceProcAddr(
            instance, "vkGetPhysicalDeviceSurfaceCapabilities2KHR");
    for (uint32_t mode = 0; get_surface_capabilities2 != NULL && mode < present_mode_count;
         ++mode) {
        const VkSurfacePresentModeKHR present_mode = {
            .sType = VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_KHR,
            .presentMode = present_modes[mode],
        };
        const VkPhysicalDeviceSurfaceInfo2KHR info2 = {
            .sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR,
            .pNext = &present_mode,
            .surface = surface,
        };
        VkPresentModeKHR compatible_modes[16];
        VkSurfacePresentModeCompatibilityKHR compatibility = {
            .sType = VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_COMPATIBILITY_KHR,
            .presentModeCount = 16,
            .pPresentModes = compatible_modes,
        };
        VkSurfacePresentScalingCapabilitiesKHR scaling = {
            .sType = VK_STRUCTURE_TYPE_SURFACE_PRESENT_SCALING_CAPABILITIES_KHR,
            .pNext = &compatibility,
        };
        VkSurfaceProtectedCapabilitiesKHR protected_capabilities = {
            .sType = VK_STRUCTURE_TYPE_SURFACE_PROTECTED_CAPABILITIES_KHR,
            .pNext = &scaling,
        };
        VkSurfaceCapabilities2KHR capabilities2 = {
            .sType = VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR,
            .pNext = &protected_capabilities,
        };
        require(get_surface_capabilities2(physical_device, &info2, &capabilities2),
                "vkGetPhysicalDeviceSurfaceCapabilities2KHR");
    }
    for (uint32_t index = 0; index < queue_family_count; ++index) {
        VkBool32 supported = VK_FALSE;
        require(vkGetPhysicalDeviceSurfaceSupportKHR(
                    physical_device, index, wayland_surface, &supported),
                "vkGetPhysicalDeviceSurfaceSupportKHR(Wayland)");
    }
    require(vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
                physical_device, wayland_surface, &surface_capabilities),
            "vkGetPhysicalDeviceSurfaceCapabilitiesKHR(Wayland)");
    uint32_t display_count = 0;
    require(vkGetPhysicalDeviceDisplayPropertiesKHR(physical_device, &display_count, NULL),
            "vkGetPhysicalDeviceDisplayPropertiesKHR");
    VkDisplayPropertiesKHR *displays = calloc(display_count, sizeof(*displays));
    if (display_count != 0 && displays == NULL) {
        return 1;
    }
    require(vkGetPhysicalDeviceDisplayPropertiesKHR(physical_device, &display_count,
                                                    displays),
            "vkGetPhysicalDeviceDisplayPropertiesKHR");
    uint32_t plane_count = 0;
    require(vkGetPhysicalDeviceDisplayPlanePropertiesKHR(physical_device, &plane_count,
                                                         NULL),
            "vkGetPhysicalDeviceDisplayPlanePropertiesKHR");
    VkDisplayPlanePropertiesKHR *planes = calloc(plane_count, sizeof(*planes));
    if (plane_count != 0 && planes == NULL) {
        return 1;
    }
    require(vkGetPhysicalDeviceDisplayPlanePropertiesKHR(physical_device, &plane_count,
                                                         planes),
            "vkGetPhysicalDeviceDisplayPlanePropertiesKHR");
    VkSurfaceKHR display_surface = VK_NULL_HANDLE;
    if (display_count != 0) {
        uint32_t mode_count = 0;
        require(vkGetDisplayModePropertiesKHR(physical_device, displays[0].display,
                                              &mode_count, NULL),
                "vkGetDisplayModePropertiesKHR");
        VkDisplayModePropertiesKHR *modes = calloc(mode_count, sizeof(*modes));
        if (mode_count != 0 && modes == NULL) {
            return 1;
        }
        require(vkGetDisplayModePropertiesKHR(physical_device, displays[0].display,
                                              &mode_count, modes),
                "vkGetDisplayModePropertiesKHR");
        uint32_t selected_plane = UINT32_MAX;
        for (uint32_t plane = 0; plane < plane_count && selected_plane == UINT32_MAX;
             ++plane) {
            uint32_t supported_count = 0;
            require(vkGetDisplayPlaneSupportedDisplaysKHR(
                        physical_device, plane, &supported_count, NULL),
                    "vkGetDisplayPlaneSupportedDisplaysKHR");
            VkDisplayKHR *supported = calloc(supported_count, sizeof(*supported));
            if (supported_count != 0 && supported == NULL) {
                return 1;
            }
            require(vkGetDisplayPlaneSupportedDisplaysKHR(
                        physical_device, plane, &supported_count, supported),
                    "vkGetDisplayPlaneSupportedDisplaysKHR");
            for (uint32_t index = 0; index < supported_count; ++index) {
                if (supported[index] == displays[0].display) {
                    selected_plane = plane;
                    break;
                }
            }
            free(supported);
        }
        if (mode_count != 0 && selected_plane != UINT32_MAX) {
            VkDisplayPlaneCapabilitiesKHR plane_capabilities;
            require(vkGetDisplayPlaneCapabilitiesKHR(physical_device, modes[0].displayMode,
                                                     selected_plane,
                                                     &plane_capabilities),
                    "vkGetDisplayPlaneCapabilitiesKHR");
            const VkDisplaySurfaceCreateInfoKHR display_surface_info = {
                .sType = VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR,
                .displayMode = modes[0].displayMode,
                .planeIndex = selected_plane,
                .planeStackIndex = planes[selected_plane].currentStackIndex,
                .transform = VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR,
                .globalAlpha = 1.0f,
                .alphaMode = VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR,
                .imageExtent = modes[0].parameters.visibleRegion,
            };
            require(vkCreateDisplayPlaneSurfaceKHR(instance, &display_surface_info, NULL,
                                                   &display_surface),
                    "vkCreateDisplayPlaneSurfaceKHR");
        }
        free(modes);
    }

    const float priority = 1.0f;
    const VkDeviceQueueCreateInfo queue_info = {
        .sType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
        .queueFamilyIndex = queue_family,
        .queueCount = 1,
        .pQueuePriorities = &priority,
    };
    const VkPhysicalDeviceFeatures features = {0};
    const VkDeviceCreateInfo device_info = {
        .sType = VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
        .queueCreateInfoCount = 1,
        .pQueueCreateInfos = &queue_info,
        .pEnabledFeatures = &features,
    };
    for (unsigned iteration = 0; iteration < 100; ++iteration) {
        VkDevice device = VK_NULL_HANDLE;
        require(vkCreateDevice(physical_device, &device_info, NULL, &device),
                "vkCreateDevice");
        static const struct {
            VkFormat format;
            VkImageUsageFlags usage;
            VkImageCreateFlags flags;
            VkImageTiling tiling;
        } images[] = {
            {VK_FORMAT_R8G8B8A8_UNORM, VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT, 0,
             VK_IMAGE_TILING_OPTIMAL},
            {VK_FORMAT_R8G8B8A8_UNORM,
             VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT | VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT, 0,
             VK_IMAGE_TILING_OPTIMAL},
            {VK_FORMAT_R8G8B8A8_UNORM, VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
             VK_IMAGE_CREATE_SPARSE_BINDING_BIT, VK_IMAGE_TILING_OPTIMAL},
            {VK_FORMAT_D16_UNORM, VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT, 0,
             VK_IMAGE_TILING_OPTIMAL},
            {VK_FORMAT_D16_UNORM,
             VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT |
                 VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT,
             0, VK_IMAGE_TILING_OPTIMAL},
            {VK_FORMAT_X8_D24_UNORM_PACK32, VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT, 0,
             VK_IMAGE_TILING_OPTIMAL},
            {VK_FORMAT_X8_D24_UNORM_PACK32,
             VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT |
                 VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT,
             0, VK_IMAGE_TILING_OPTIMAL},
            {VK_FORMAT_D32_SFLOAT, VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT, 0,
             VK_IMAGE_TILING_OPTIMAL},
            {VK_FORMAT_D32_SFLOAT,
             VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT |
                 VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT,
             0, VK_IMAGE_TILING_OPTIMAL},
            {VK_FORMAT_S8_UINT, VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT, 0,
             VK_IMAGE_TILING_OPTIMAL},
            {VK_FORMAT_S8_UINT,
             VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT |
                 VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT,
             0, VK_IMAGE_TILING_OPTIMAL},
            {VK_FORMAT_D24_UNORM_S8_UINT, VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT, 0,
             VK_IMAGE_TILING_OPTIMAL},
            {VK_FORMAT_D24_UNORM_S8_UINT,
             VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT |
                 VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT,
             0, VK_IMAGE_TILING_OPTIMAL},
            {VK_FORMAT_D32_SFLOAT_S8_UINT, VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT, 0,
             VK_IMAGE_TILING_OPTIMAL},
            {VK_FORMAT_D32_SFLOAT_S8_UINT,
             VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT |
                 VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT,
             0, VK_IMAGE_TILING_OPTIMAL},
            {VK_FORMAT_R8G8B8A8_UNORM, VK_IMAGE_USAGE_TRANSFER_SRC_BIT, 0,
             VK_IMAGE_TILING_LINEAR},
        };
        for (unsigned image_index = 0; image_index < sizeof(images) / sizeof(images[0]);
             ++image_index) {
            const VkImageCreateInfo image_info = {
                .sType = VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO,
                .flags = images[image_index].flags,
                .imageType = VK_IMAGE_TYPE_2D,
                .format = images[image_index].format,
                .extent = {8, 8, 1},
                .mipLevels = 1,
                .arrayLayers = 1,
                .samples = VK_SAMPLE_COUNT_1_BIT,
                .tiling = images[image_index].tiling,
                .usage = images[image_index].usage,
                .sharingMode = VK_SHARING_MODE_EXCLUSIVE,
                .initialLayout = VK_IMAGE_LAYOUT_UNDEFINED,
            };
            VkImage image = VK_NULL_HANDLE;
            require(vkCreateImage(device, &image_info, NULL, &image), "vkCreateImage");
            VkMemoryRequirements requirements;
            vkGetImageMemoryRequirements(device, image, &requirements);
            vkDestroyImage(device, image, NULL);
        }
        const VkVideoDecodeH264ProfileInfoKHR h264_profile = {
            .sType = VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_PROFILE_INFO_KHR,
            .stdProfileIdc = STD_VIDEO_H264_PROFILE_IDC_BASELINE,
            .pictureLayout = VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_PROGRESSIVE_KHR,
        };
        const VkVideoProfileInfoKHR video_profile = {
            .sType = VK_STRUCTURE_TYPE_VIDEO_PROFILE_INFO_KHR,
            .pNext = &h264_profile,
            .videoCodecOperation = VK_VIDEO_CODEC_OPERATION_DECODE_H264_BIT_KHR,
            .chromaSubsampling = VK_VIDEO_CHROMA_SUBSAMPLING_420_BIT_KHR,
            .lumaBitDepth = VK_VIDEO_COMPONENT_BIT_DEPTH_8_BIT_KHR,
            .chromaBitDepth = VK_VIDEO_COMPONENT_BIT_DEPTH_8_BIT_KHR,
        };
        for (unsigned query = 0; get_video_capabilities != NULL && query < 1410; ++query) {
            VkVideoDecodeH264CapabilitiesKHR h264_capabilities = {
                .sType = VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_CAPABILITIES_KHR,
            };
            VkVideoDecodeCapabilitiesKHR decode_capabilities = {
                .sType = VK_STRUCTURE_TYPE_VIDEO_DECODE_CAPABILITIES_KHR,
                .pNext = &h264_capabilities,
            };
            VkVideoCapabilitiesKHR capabilities = {
                .sType = VK_STRUCTURE_TYPE_VIDEO_CAPABILITIES_KHR,
                .pNext = &decode_capabilities,
            };
            VkResult result = get_video_capabilities(
                physical_device, &video_profile, &capabilities);
            if (result != VK_SUCCESS &&
                result != VK_ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR &&
                result != VK_ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR &&
                result != VK_ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR) {
                require(result, "vkGetPhysicalDeviceVideoCapabilitiesKHR");
            }
        }
        vkDestroyDevice(device, NULL);
        uint32_t group_count = 0;
        require(enumerate_groups_khr(instance, &group_count, NULL),
                "vkEnumeratePhysicalDeviceGroupsKHR");
        VkPhysicalDeviceGroupProperties *groups = calloc(group_count, sizeof(*groups));
        if (group_count != 0 && groups == NULL) {
            return 1;
        }
        for (uint32_t group = 0; group < group_count; ++group) {
            groups[group].sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES;
        }
        require(enumerate_groups_khr(instance, &group_count,
                                     (VkPhysicalDeviceGroupPropertiesKHR *)groups),
                "vkEnumeratePhysicalDeviceGroupsKHR");
        free(groups);
    }
    uint32_t final_group_count = 0;
    require(enumerate_groups_khr(instance, &final_group_count, NULL),
            "vkEnumeratePhysicalDeviceGroupsKHR");
    VkPhysicalDeviceGroupPropertiesKHR *final_groups =
        calloc(final_group_count, sizeof(*final_groups));
    if (final_group_count != 0 && final_groups == NULL) {
        return 1;
    }
    for (uint32_t group = 0; group < final_group_count; ++group) {
        final_groups[group].sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES_KHR;
    }
    require(enumerate_groups_khr(instance, &final_group_count, final_groups),
            "vkEnumeratePhysicalDeviceGroupsKHR");
    if (final_group_count != 0) {
        const char *group_extensions[] = {VK_KHR_SWAPCHAIN_EXTENSION_NAME,
                                          VK_KHR_DEVICE_GROUP_EXTENSION_NAME};
        const VkDeviceGroupDeviceCreateInfoKHR group_info = {
            .sType = VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO_KHR,
            .physicalDeviceCount = final_groups[0].physicalDeviceCount,
            .pPhysicalDevices = final_groups[0].physicalDevices,
        };
        const VkDeviceCreateInfo group_device_info = {
            .sType = VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
            .pNext = &group_info,
            .queueCreateInfoCount = 1,
            .pQueueCreateInfos = &queue_info,
            .enabledExtensionCount = 2,
            .ppEnabledExtensionNames = group_extensions,
        };
        VkDevice group_device = VK_NULL_HANDLE;
        VkResult group_result =
            vkCreateDevice(final_groups[0].physicalDevices[0], &group_device_info, NULL,
                           &group_device);
        if (group_result == VK_SUCCESS) {
            PFN_vkGetDeviceGroupPresentCapabilitiesKHR get_group_capabilities =
                (PFN_vkGetDeviceGroupPresentCapabilitiesKHR)vkGetDeviceProcAddr(
                    group_device, "vkGetDeviceGroupPresentCapabilitiesKHR");
            VkDeviceGroupPresentCapabilitiesKHR capabilities = {
                .sType = VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR,
            };
            require(get_group_capabilities(group_device, &capabilities),
                    "vkGetDeviceGroupPresentCapabilitiesKHR");
            vkDestroyDevice(group_device, NULL);
        } else if (group_result != VK_ERROR_EXTENSION_NOT_PRESENT) {
            require(group_result, "vkCreateDevice(device group)");
        }
    }
    free(final_groups);
    if (debug_callback != VK_NULL_HANDLE && destroy_debug_report != NULL) {
        destroy_debug_report(instance, debug_callback, NULL);
    }
    destroy_surface(instance, surface, NULL);
    destroy_surface(instance, wayland_surface, NULL);
    if (display_surface != VK_NULL_HANDLE) {
        destroy_surface(instance, display_surface, NULL);
    }
    free(planes);
    free(displays);
    free(present_modes);
    vkDestroyInstance(instance, NULL);
    xcb_destroy_window(connection, window);
    xcb_disconnect(connection);
    wl_surface_destroy(wayland_window);
    wl_compositor_destroy(compositor);
    wl_registry_destroy(registry);
    wl_display_disconnect(wayland_display);
    return 0;
}
