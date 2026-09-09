#include "test_environment.h"

#include <algorithm>
#include <cstdlib>
#include <filesystem>
#include <iostream>
#include <string>
#include <vector>

static std::vector<std::string> corpus_files() {
  std::vector<std::string> files;
  for (const auto &entry : std::filesystem::directory_iterator(
           std::getenv("VK_LOADER_CORPUS_DIRECTORY")
               ? std::getenv("VK_LOADER_CORPUS_DIRECTORY")
               : CORPUS_DIRECTORY)) {
    if (entry.is_regular_file())
      files.push_back(entry.path().string());
  }
  std::sort(files.begin(), files.end());
  return files;
}

class CorpusReplay : public testing::TestWithParam<std::string> {};

static void print_bytes(const char *value, size_t capacity) {
  static constexpr char hex[] = "0123456789abcdef";
  for (size_t index = 0; index < capacity && value[index] != '\0'; ++index) {
    const auto byte = static_cast<unsigned char>(value[index]);
    std::cout.put(hex[byte >> 4]);
    std::cout.put(hex[byte & 15]);
  }
}

TEST_P(CorpusReplay, PublicApi) {
  for (bool with_driver : {false, true}) {
    // Every input is exercised in every role, independent of its seed name.
    for (const auto category :
         {ManifestCategory::implicit_layer, ManifestCategory::explicit_layer,
          ManifestCategory::icd, ManifestCategory::settings}) {
      FrameworkEnvironment env{};
#if TESTING_COMMON_UNIX_PLATFORMS
      // Historical fuzz seeds name the host validation library. Keep replay
      // independent of the installed version and its behavior on malformed
      // chains.
      env.platform_shim->add_system_library(
          "libVkLayer_khronos_validation.so",
          "/nonexistent-corpus-library/libVkLayer_khronos_validation.so");
#endif
      if (with_driver) {
        auto &driver =
            env.add_icd(TEST_ICD_PATH_VERSION_2, {},
                        ManifestICD{}.set_api_version(VK_API_VERSION_1_1))
                .set_min_icd_interface_version(5)
                .set_icd_api_version(VK_API_VERSION_1_1);
        for (int index = 0; index < 2; ++index)
          driver.add_and_get_physical_device("corpus_" + std::to_string(index))
              .add_queue_family_properties(
                  {{VK_QUEUE_GRAPHICS_BIT, 1, 0, {1, 1, 1}}, true});
        driver.add_physical_device_group(0);
      }
      const auto location = [&] {
        switch (category) {
        case ManifestCategory::implicit_layer:
          return ManifestLocation::implicit_layer;
        case ManifestCategory::explicit_layer:
          return ManifestLocation::explicit_layer;
        case ManifestCategory::icd:
          return ManifestLocation::driver;
        case ManifestCategory::settings:
          return ManifestLocation::settings_location;
        }
        return ManifestLocation::null;
      }();
      const char *filename = category == ManifestCategory::settings
                                 ? "vk_loader_settings.json"
                                 : "corpus.json";
      env.write_file_from_source(GetParam().c_str(), category, location,
                                 filename);
      uint32_t layers = 0;
      const auto layer_result =
          env.vulkan_functions.vkEnumerateInstanceLayerProperties(&layers,
                                                                  nullptr);
      if (layer_result == VK_SUCCESS && layers != 0) {
        std::vector<VkLayerProperties> properties(layers);
        auto written = layers;
        const auto result =
            env.vulkan_functions.vkEnumerateInstanceLayerProperties(
                &written, properties.data());
        ASSERT_LE(written, layers);
        std::cout << "LAYER_RESULT " << result << ' ' << written << '\n';
        for (uint32_t index = 0; index < std::min(written, layers); ++index) {
          const auto &property = properties[index];
          std::cout << "LAYER " << index << ' ';
          print_bytes(property.layerName, sizeof(property.layerName));
          std::cout << ' ' << property.specVersion << ' '
                    << property.implementationVersion << ' ';
          print_bytes(property.description, sizeof(property.description));
          std::cout << '\n';
        }
      }
      uint32_t extensions = 0;
      const auto extension_result =
          env.vulkan_functions.vkEnumerateInstanceExtensionProperties(
              nullptr, &extensions, nullptr);
      if (extension_result == VK_SUCCESS && extensions != 0) {
        std::vector<VkExtensionProperties> properties(extensions);
        auto written = extensions;
        const auto result =
            env.vulkan_functions.vkEnumerateInstanceExtensionProperties(
                nullptr, &written, properties.data());
        ASSERT_LE(written, extensions);
        std::cout << "EXTENSION_RESULT " << result << ' ' << written << '\n';
        for (uint32_t index = 0; index < std::min(written, extensions);
             ++index) {
          std::cout << "EXTENSION " << index << ' ';
          print_bytes(properties[index].extensionName,
                      sizeof(properties[index].extensionName));
          std::cout << ' ' << properties[index].specVersion << '\n';
        }
      }
      uint32_t missing = 0;
      const auto missing_result =
          env.vulkan_functions.vkEnumerateInstanceExtensionProperties(
              "test_auto", &missing, nullptr);
      VkInstanceCreateInfo info{};
      info.sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
      VkApplicationInfo application{};
      application.sType = VK_STRUCTURE_TYPE_APPLICATION_INFO;
      application.apiVersion = VK_API_VERSION_1_1;
      if (with_driver)
        info.pApplicationInfo = &application;
      VkInstance instance = VK_NULL_HANDLE;
      const auto create_result =
          env.vulkan_functions.vkCreateInstance(&info, nullptr, &instance);
      if (create_result == VK_SUCCESS && with_driver) {
        uint32_t count = 0;
        auto result = env.vulkan_functions.vkEnumeratePhysicalDevices(
            instance, &count, nullptr);
        std::cout << "PHYSICAL_RESULT " << result << ' ' << count << '\n';
        std::vector<VkPhysicalDevice> devices(count);
        if (count) {
          auto written = count;
          result = env.vulkan_functions.vkEnumeratePhysicalDevices(
              instance, &written, devices.data());
          ASSERT_LE(written, count);
          devices.resize(written);
          std::cout << "PHYSICAL_RESULT " << result << ' ' << written << '\n';
        }
        for (int phase = 0; phase < 3; ++phase) {
          if (phase == 1)
            env.get_test_icd().add_physical_device_group(1);
          uint32_t groups = 0;
          result = env.vulkan_functions.vkEnumeratePhysicalDeviceGroups(
              instance, &groups, nullptr);
          std::cout << "GROUP_RESULT " << phase << ' ' << result << ' '
                    << groups << '\n';
          if (groups) {
            std::vector<VkPhysicalDeviceGroupProperties> properties(groups);
            for (auto &group : properties)
              group.sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES;
            auto written = groups;
            result = env.vulkan_functions.vkEnumeratePhysicalDeviceGroups(
                instance, &written, properties.data());
            ASSERT_LE(written, groups);
            std::cout << "GROUP_RESULT " << phase << ' ' << result << ' '
                      << written << '\n';
            for (uint32_t index = 0; index < written; ++index) {
              const auto &group = properties[index];
              ASSERT_LE(group.physicalDeviceCount, VK_MAX_DEVICE_GROUP_SIZE);
              std::cout << "GROUP " << group.physicalDeviceCount << ' '
                        << group.subsetAllocation;
              for (uint32_t gpu = 0; gpu < group.physicalDeviceCount; ++gpu)
                std::cout << ' '
                          << (std::find(devices.begin(), devices.end(),
                                        group.physicalDevices[gpu]) -
                              devices.begin());
              std::cout << '\n';
            }
          }
        }
        if (!devices.empty()) {
          float priority = 1.0f;
          VkDeviceQueueCreateInfo queue{};
          queue.sType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
          queue.queueCount = 1;
          queue.pQueuePriorities = &priority;
          VkDeviceCreateInfo device_info{};
          device_info.sType = VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO;
          device_info.queueCreateInfoCount = 1;
          device_info.pQueueCreateInfos = &queue;
          VkDevice device = VK_NULL_HANDLE;
          result = env.vulkan_functions.vkCreateDevice(
              devices.front(), &device_info, nullptr, &device);
          std::cout << "DEVICE_RESULT " << result << ' '
                    << (device != VK_NULL_HANDLE) << '\n';
          if (result == VK_SUCCESS)
            env.vulkan_functions.vkDestroyDevice(device, nullptr);
        }
      }
      std::cout << "RESULT "
                << std::filesystem::path(GetParam()).filename().string() << ' '
                << (static_cast<int>(category) + (with_driver ? 4 : 0)) << ' '
                << layer_result << ' ' << layers << ' ' << extension_result
                << ' ' << extensions << ' ' << missing_result << ' ' << missing
                << ' ' << create_result << ' ' << (instance != VK_NULL_HANDLE)
                << '\n';
      if (create_result == VK_SUCCESS)
        env.vulkan_functions.vkDestroyInstance(instance, nullptr);
    }
  }
}

INSTANTIATE_TEST_SUITE_P(Generated, CorpusReplay,
                         testing::ValuesIn(corpus_files()));
