#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
upstream_dir="$repo_root/.upstream/vulkan-loader"
build_dir="$upstream_dir/build-windows-rust-parity"
expected_revision="b1d75f38257ffa71d7aa93552d2e2793296309aa"
compiler_prefix="${VK_LOADER_MINGW_PREFIX:-x86_64-w64-mingw32}"
cc="$compiler_prefix-gcc"
cxx="$compiler_prefix-g++"
windres="$compiler_prefix-windres"

require_tools git cmake ninja "$cc" "$cxx" "$windres"

if [[ ! -d "$upstream_dir/.git" ]]; then
  git clone https://github.com/KhronosGroup/Vulkan-Loader.git "$upstream_dir"
fi
actual_revision="$(git -C "$upstream_dir" rev-parse HEAD)"
if [[ "$actual_revision" != "$expected_revision" ]]; then
  git -C "$upstream_dir" fetch origin "$expected_revision"
  git -C "$upstream_dir" checkout --detach "$expected_revision"
fi

compat="$loader_scripts/platform/mingw-test-compat.h"
case_include="$loader_scripts/platform/mingw-case-include"
cmake \
  -S "$upstream_dir" \
  -B "$build_dir" \
  -G Ninja \
  -D CMAKE_SYSTEM_NAME=Windows \
  -D CMAKE_C_COMPILER="$cc" \
  -D CMAKE_CXX_COMPILER="$cxx" \
  -D CMAKE_RC_COMPILER="$windres" \
  -D CMAKE_CXX_FLAGS="-I$case_include -include $compat" \
  -D CMAKE_BUILD_TYPE=Debug \
  -D UPDATE_DEPS=ON \
  -D BUILD_TESTS=ON \
  -D BUILD_WERROR=OFF

fixture_targets=(
  test_icd_export_none
  test_icd_export_icd_gipa
  test_icd_export_negotiate_interface_version
  test_icd_version_2
  test_icd_version_2_export_icd_enumerate_adapter_physical_devices
  test_icd_version_2_export_icd_gpdpa
  test_icd_version_6
  test_icd_version_7
  test_icd_version_7_without_exports
  test_layer_export_base
  test_layer_export_version_0
  test_layer_export_version_0_named_gpa
  test_layer_export_version_1
  test_layer_export_version_2
  test_layer_wrap_objects
  test_layer_wrap_objects_1
  test_layer_wrap_objects_2
  test_layer_wrap_objects_3
)
cmake --build "$build_dir" --parallel --target \
  test_regression test_fuzzing test_threading "${fixture_targets[@]}"

# GNU ld cannot parse upstream's Unicode `LIBRARY 🌋` module-definition
# line. Compile the unchanged fixture object and link it with export-all instead;
# the test only resolves the two Vulkan ICD entry points by name.
unicode_object="tests/framework/icd/CMakeFiles/test_unicode.dir/test_icd.cpp.obj"
cmake --build "$build_dir" --target "$unicode_object"
"$cxx" -shared -Wl,--export-all-symbols \
  -o "$build_dir/tests/framework/icd/lib🌋.dll" \
  "$build_dir/$unicode_object" \
  "$build_dir/tests/framework/util/libtesting_framework_util.a" \
  "$build_dir/lib/libgtest.a" \
  -lkernel32 -luser32 -lgdi32 -lwinspool -lshell32 \
  -lole32 -loleaut32 -luuid -lcomdlg32 -ladvapi32

echo "Prepared unchanged upstream Windows tests in $build_dir/tests"
