#define _POSIX_C_SOURCE 200809L

#include <vulkan/vulkan.h>

#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

static volatile uintptr_t sink;

static uint64_t now_ns(void) {
    struct timespec time;
    if (clock_gettime(CLOCK_MONOTONIC_RAW, &time) != 0) {
        perror("clock_gettime");
        exit(2);
    }
    return (uint64_t)time.tv_sec * UINT64_C(1000000000) + (uint64_t)time.tv_nsec;
}

static void require_success(VkResult result, const char *operation) {
    if (result != VK_SUCCESS) {
        fprintf(stderr, "%s failed: %d\n", operation, result);
        exit(2);
    }
}

static VkInstanceCreateInfo instance_create_info(void) {
    static const VkApplicationInfo application = {
        .sType = VK_STRUCTURE_TYPE_APPLICATION_INFO,
        .pApplicationName = "loader-benchmark",
        .applicationVersion = 1,
        .pEngineName = "none",
        .engineVersion = 1,
        .apiVersion = VK_API_VERSION_1_4,
    };
    static const char *layer;
    const char *selected_layer = getenv("VK_LOADER_BENCH_LAYER");
    layer = selected_layer != NULL && selected_layer[0] != '\0' ? selected_layer : NULL;
    VkInstanceCreateInfo create_info = {
        .sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        .pApplicationInfo = &application,
    };
    if (layer != NULL) {
        create_info.enabledLayerCount = 1;
        create_info.ppEnabledLayerNames = &layer;
    }
    return create_info;
}

static VkInstance create_instance(void) {
    VkInstance instance = VK_NULL_HANDLE;
    VkInstanceCreateInfo create_info = instance_create_info();
    require_success(vkCreateInstance(&create_info, NULL, &instance), "vkCreateInstance");
    return instance;
}

static VkPhysicalDevice first_physical_device(VkInstance instance) {
    uint32_t count = 0;
    require_success(vkEnumeratePhysicalDevices(instance, &count, NULL),
                    "vkEnumeratePhysicalDevices(count)");
    if (count == 0) {
        fputs("no Vulkan physical device\n", stderr);
        exit(2);
    }
    VkPhysicalDevice *devices = calloc(count, sizeof(*devices));
    if (devices == NULL) {
        fputs("out of host memory\n", stderr);
        exit(2);
    }
    require_success(vkEnumeratePhysicalDevices(instance, &count, devices),
                    "vkEnumeratePhysicalDevices(data)");
    VkPhysicalDevice device = devices[0];
    free(devices);
    return device;
}

static uint32_t first_queue_family(VkPhysicalDevice physical_device) {
    uint32_t count = 0;
    vkGetPhysicalDeviceQueueFamilyProperties(physical_device, &count, NULL);
    if (count == 0) {
        fputs("physical device has no queue families\n", stderr);
        exit(2);
    }
    VkQueueFamilyProperties *properties = calloc(count, sizeof(*properties));
    if (properties == NULL) {
        fputs("out of host memory\n", stderr);
        exit(2);
    }
    vkGetPhysicalDeviceQueueFamilyProperties(physical_device, &count, properties);
    uint32_t selected = 0;
    for (uint32_t index = 0; index < count; ++index) {
        if (properties[index].queueCount != 0 &&
            (properties[index].queueFlags & VK_QUEUE_GRAPHICS_BIT) != 0) {
            selected = index;
            break;
        }
    }
    free(properties);
    return selected;
}

static VkDevice create_device(VkPhysicalDevice physical_device) {
    const float priority = 1.0f;
    const VkDeviceQueueCreateInfo queue = {
        .sType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
        .queueFamilyIndex = first_queue_family(physical_device),
        .queueCount = 1,
        .pQueuePriorities = &priority,
    };
    const VkDeviceCreateInfo create_info = {
        .sType = VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
        .queueCreateInfoCount = 1,
        .pQueueCreateInfos = &queue,
    };
    VkDevice device = VK_NULL_HANDLE;
    require_success(vkCreateDevice(physical_device, &create_info, NULL, &device), "vkCreateDevice");
    return device;
}

static void benchmark_enumerate_extensions(uint64_t iterations) {
    uint32_t count = 0;
    require_success(vkEnumerateInstanceExtensionProperties(NULL, &count, NULL),
                    "vkEnumerateInstanceExtensionProperties(warmup)");
    const uint64_t start = now_ns();
    for (uint64_t index = 0; index < iterations; ++index) {
        count = 0;
        require_success(vkEnumerateInstanceExtensionProperties(NULL, &count, NULL),
                        "vkEnumerateInstanceExtensionProperties");
        sink ^= count;
    }
    const uint64_t elapsed = now_ns() - start;
    printf("enumerate-extensions,%" PRIu64 ",%" PRIu64 ",%.3f,%" PRIuPTR "\n", iterations,
           elapsed, (double)elapsed / (double)iterations, sink);
}

static void benchmark_instance_cycle(uint64_t iterations) {
    VkInstance warmup = create_instance();
    vkDestroyInstance(warmup, NULL);
    const uint64_t start = now_ns();
    for (uint64_t index = 0; index < iterations; ++index) {
        VkInstance instance = create_instance();
        sink ^= (uintptr_t)instance;
        vkDestroyInstance(instance, NULL);
    }
    const uint64_t elapsed = now_ns() - start;
    printf("instance-cycle,%" PRIu64 ",%" PRIu64 ",%.3f,%" PRIuPTR "\n", iterations, elapsed,
           (double)elapsed / (double)iterations, sink);
}

static void benchmark_device_cycle(uint64_t iterations) {
    VkInstance instance = create_instance();
    VkPhysicalDevice physical_device = first_physical_device(instance);
    VkDevice warmup = create_device(physical_device);
    vkDestroyDevice(warmup, NULL);
    const uint64_t start = now_ns();
    for (uint64_t index = 0; index < iterations; ++index) {
        VkDevice device = create_device(physical_device);
        sink ^= (uintptr_t)device;
        vkDestroyDevice(device, NULL);
    }
    const uint64_t elapsed = now_ns() - start;
    vkDestroyInstance(instance, NULL);
    printf("device-cycle,%" PRIu64 ",%" PRIu64 ",%.3f,%" PRIuPTR "\n", iterations, elapsed,
           (double)elapsed / (double)iterations, sink);
}

static void benchmark_instance_gpa(uint64_t iterations, int missing) {
    static const char *known_names[] = {
        "vkDestroyInstance",
        "vkEnumeratePhysicalDevices",
        "vkGetPhysicalDeviceFeatures",
        "vkGetPhysicalDeviceProperties",
        "vkGetPhysicalDeviceQueueFamilyProperties",
        "vkEnumerateDeviceExtensionProperties",
        "vkCreateDevice",
        "vkGetPhysicalDeviceFeatures2",
    };
    static const char *missing_names[] = {
        "vkNotACommand000", "vkNotACommand001", "vkNotACommand002", "vkNotACommand003",
        "vkNotACommand004", "vkNotACommand005", "vkNotACommand006", "vkNotACommand007",
    };
    VkInstance instance = create_instance();
    const char **names = missing ? missing_names : known_names;
    for (uint32_t index = 0; index < 10000; ++index) {
        sink ^= (uintptr_t)vkGetInstanceProcAddr(instance, names[index & 7]);
    }
    const uint64_t start = now_ns();
    for (uint64_t index = 0; index < iterations; ++index) {
        sink ^= (uintptr_t)vkGetInstanceProcAddr(instance, names[index & 7]);
    }
    const uint64_t elapsed = now_ns() - start;
    vkDestroyInstance(instance, NULL);
    printf("instance-gpa-%s,%" PRIu64 ",%" PRIu64 ",%.3f,%" PRIuPTR "\n",
           missing ? "missing" : "known", iterations, elapsed,
           (double)elapsed / (double)iterations, sink);
}

static void benchmark_device_gpa(uint64_t iterations, int missing) {
    static const char *known_names[] = {
        "vkDestroyDevice", "vkGetDeviceQueue", "vkQueueSubmit", "vkDeviceWaitIdle",
        "vkAllocateMemory", "vkCreateBuffer", "vkCreateImage", "vkCmdDraw",
    };
    static const char *missing_names[] = {
        "vkNotADeviceCommand000", "vkNotADeviceCommand001", "vkNotADeviceCommand002",
        "vkNotADeviceCommand003", "vkNotADeviceCommand004", "vkNotADeviceCommand005",
        "vkNotADeviceCommand006", "vkNotADeviceCommand007",
    };
    VkInstance instance = create_instance();
    VkDevice device = create_device(first_physical_device(instance));
    const char **names = missing ? missing_names : known_names;
    for (uint32_t index = 0; index < 10000; ++index) {
        sink ^= (uintptr_t)vkGetDeviceProcAddr(device, names[index & 7]);
    }
    const uint64_t start = now_ns();
    for (uint64_t index = 0; index < iterations; ++index) {
        sink ^= (uintptr_t)vkGetDeviceProcAddr(device, names[index & 7]);
    }
    const uint64_t elapsed = now_ns() - start;
    vkDestroyDevice(device, NULL);
    vkDestroyInstance(instance, NULL);
    printf("device-gpa-%s,%" PRIu64 ",%" PRIu64 ",%.3f,%" PRIuPTR "\n",
           missing ? "missing" : "known", iterations, elapsed,
           (double)elapsed / (double)iterations, sink);
}

static void benchmark_physical_device_properties(uint64_t iterations) {
    VkInstance instance = create_instance();
    VkPhysicalDevice physical_device = first_physical_device(instance);
    VkPhysicalDeviceProperties properties;
    for (uint32_t index = 0; index < 10000; ++index) {
        vkGetPhysicalDeviceProperties(physical_device, &properties);
        sink ^= properties.vendorID;
    }
    const uint64_t start = now_ns();
    for (uint64_t index = 0; index < iterations; ++index) {
        vkGetPhysicalDeviceProperties(physical_device, &properties);
        sink ^= properties.vendorID;
    }
    const uint64_t elapsed = now_ns() - start;
    vkDestroyInstance(instance, NULL);
    printf("physical-device-properties,%" PRIu64 ",%" PRIu64 ",%.3f,%" PRIuPTR "\n",
           iterations, elapsed, (double)elapsed / (double)iterations, sink);
}

int main(int argc, char **argv) {
    if (argc != 3) {
        fprintf(stderr, "usage: %s MODE ITERATIONS\n", argv[0]);
        return 2;
    }
    char *end = NULL;
    uint64_t iterations = strtoull(argv[2], &end, 10);
    if (end == argv[2] || *end != '\0' || iterations == 0) {
        fputs("ITERATIONS must be a positive integer\n", stderr);
        return 2;
    }
    if (strcmp(argv[1], "enumerate-extensions") == 0) {
        benchmark_enumerate_extensions(iterations);
    } else if (strcmp(argv[1], "instance-cycle") == 0) {
        benchmark_instance_cycle(iterations);
    } else if (strcmp(argv[1], "device-cycle") == 0) {
        benchmark_device_cycle(iterations);
    } else if (strcmp(argv[1], "instance-gpa-known") == 0) {
        benchmark_instance_gpa(iterations, 0);
    } else if (strcmp(argv[1], "instance-gpa-missing") == 0) {
        benchmark_instance_gpa(iterations, 1);
    } else if (strcmp(argv[1], "device-gpa-known") == 0) {
        benchmark_device_gpa(iterations, 0);
    } else if (strcmp(argv[1], "device-gpa-missing") == 0) {
        benchmark_device_gpa(iterations, 1);
    } else if (strcmp(argv[1], "physical-device-properties") == 0) {
        benchmark_physical_device_properties(iterations);
    } else {
        fprintf(stderr, "unknown mode: %s\n", argv[1]);
        return 2;
    }
    return 0;
}
