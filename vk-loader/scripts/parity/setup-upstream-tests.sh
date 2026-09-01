#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
expected_revision="b1d75f38257ffa71d7aa93552d2e2793296309aa"

require_tools cmake git mold ninja

coverage_args=()
if [[ "${VK_LOADER_UPSTREAM_CODE_COVERAGE:-0}" == 1 ]]; then
  require_tools clang clang++ llvm-cov llvm-profdata
  coverage_args+=(
    -D CODE_COVERAGE=ON
    -D CMAKE_C_COMPILER=clang
    -D CMAKE_CXX_COMPILER=clang++
  )
fi

if [[ ! -d "$upstream_dir/.git" ]]; then
  git clone https://github.com/KhronosGroup/Vulkan-Loader.git "$upstream_dir"
fi

actual_revision="$(git -C "$upstream_dir" rev-parse HEAD)"
if [[ "$actual_revision" != "$expected_revision" ]]; then
  git -C "$upstream_dir" fetch origin "$expected_revision"
  git -C "$upstream_dir" checkout --detach "$expected_revision"
fi

cmake \
  -S "$upstream_dir" \
  -B "$upstream_build_dir" \
  -G Ninja \
  -D CMAKE_EXE_LINKER_FLAGS=-fuse-ld=mold \
  -D CMAKE_MODULE_LINKER_FLAGS=-fuse-ld=mold \
  -D CMAKE_SHARED_LINKER_FLAGS=-fuse-ld=mold \
  -D CMAKE_BUILD_TYPE=Debug \
  -D UPDATE_DEPS=ON \
  -D BUILD_TESTS=ON \
  -D BUILD_WERROR=OFF \
  "${coverage_args[@]}"
cmake --build "$upstream_build_dir" --parallel
