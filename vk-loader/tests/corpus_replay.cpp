#include "test_environment.h"

#include <algorithm>
#include <filesystem>
#include <iostream>
#include <string>
#include <vector>

static std::vector<std::string> corpus_files() {
    std::vector<std::string> files;
    for (const auto& entry : std::filesystem::directory_iterator(CORPUS_DIRECTORY)) {
        if (entry.is_regular_file()) files.push_back(entry.path().string());
    }
    std::sort(files.begin(), files.end());
    return files;
}

class CorpusReplay : public testing::TestWithParam<std::string> {};

static void print_bytes(const char* value, size_t capacity) {
    static constexpr char hex[] = "0123456789abcdef";
    for (size_t index = 0; index < capacity && value[index] != '\0'; ++index) {
        const auto byte = static_cast<unsigned char>(value[index]);
        std::cout.put(hex[byte >> 4]);
        std::cout.put(hex[byte & 15]);
    }
}

TEST_P(CorpusReplay, PublicApi) {
    // Every input is exercised in every role, independent of its seed name.
    for (const auto category : {ManifestCategory::implicit_layer, ManifestCategory::explicit_layer,
                               ManifestCategory::icd, ManifestCategory::settings}) {
        FrameworkEnvironment env{};
        const auto location = [&] {
            switch (category) {
                case ManifestCategory::implicit_layer: return ManifestLocation::implicit_layer;
                case ManifestCategory::explicit_layer: return ManifestLocation::explicit_layer;
                case ManifestCategory::icd: return ManifestLocation::driver;
                case ManifestCategory::settings: return ManifestLocation::settings_location;
            }
            return ManifestLocation::null;
        }();
        const char* filename = category == ManifestCategory::settings
            ? "vk_loader_settings.json" : "corpus.json";
        env.write_file_from_source(GetParam().c_str(), category, location, filename);
        uint32_t layers = 0;
        const auto layer_result = env.vulkan_functions.vkEnumerateInstanceLayerProperties(&layers, nullptr);
        if (layer_result == VK_SUCCESS && layers != 0) {
            std::vector<VkLayerProperties> properties(layers);
            auto written = layers;
            const auto result = env.vulkan_functions.vkEnumerateInstanceLayerProperties(&written, properties.data());
            ASSERT_LE(written, layers);
            std::cout << "LAYER_RESULT " << result << ' ' << written << '\n';
            for (uint32_t index = 0; index < std::min(written, layers); ++index) {
                const auto& property = properties[index];
                std::cout << "LAYER " << index << ' ';
                print_bytes(property.layerName, sizeof(property.layerName));
                std::cout << ' ' << property.specVersion << ' ' << property.implementationVersion << ' ';
                print_bytes(property.description, sizeof(property.description));
                std::cout << '\n';
            }
        }
        uint32_t extensions = 0;
        const auto extension_result = env.vulkan_functions.vkEnumerateInstanceExtensionProperties(nullptr, &extensions, nullptr);
        if (extension_result == VK_SUCCESS && extensions != 0) {
            std::vector<VkExtensionProperties> properties(extensions);
            auto written = extensions;
            const auto result = env.vulkan_functions.vkEnumerateInstanceExtensionProperties(nullptr, &written, properties.data());
            ASSERT_LE(written, extensions);
            std::cout << "EXTENSION_RESULT " << result << ' ' << written << '\n';
            for (uint32_t index = 0; index < std::min(written, extensions); ++index) {
                std::cout << "EXTENSION " << index << ' ';
                print_bytes(properties[index].extensionName, sizeof(properties[index].extensionName));
                std::cout << ' ' << properties[index].specVersion << '\n';
            }
        }
        uint32_t missing = 0;
        const auto missing_result = env.vulkan_functions.vkEnumerateInstanceExtensionProperties("test_auto", &missing, nullptr);
        VkInstanceCreateInfo info{};
        info.sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
        VkInstance instance = VK_NULL_HANDLE;
        const auto create_result = env.vulkan_functions.vkCreateInstance(&info, nullptr, &instance);
        std::cout << "RESULT " << std::filesystem::path(GetParam()).filename().string()
                  << ' ' << static_cast<int>(category)
                  << ' ' << layer_result << ' ' << layers
                  << ' ' << extension_result << ' ' << extensions
                  << ' ' << missing_result << ' ' << missing
                  << ' ' << create_result << ' ' << (instance != VK_NULL_HANDLE) << '\n';
        if (create_result == VK_SUCCESS) env.vulkan_functions.vkDestroyInstance(instance, nullptr);
    }
}

INSTANTIATE_TEST_SUITE_P(Generated, CorpusReplay, testing::ValuesIn(corpus_files()));
