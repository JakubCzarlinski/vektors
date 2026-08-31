#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
upstream_loader="${VK_LOADER_PARITY_UPSTREAM_LIBRARY:-$repo_root/.upstream/vulkan-loader/build-rust-parity/loader/libvulkan.so.1.4.361}"
output_dir="${VK_LOADER_CONTRACT_PARITY_DIR:-$repo_root/target/pre-instance-contract-parity}"

require_tools cargo cc
ensure_upstream_tests test_regression "$upstream_loader"
rust_loader="$(resolve_rust_loader "${VK_LOADER_PARITY_RUST_LIBRARY:-}" release)"
require_files "$rust_loader" "$upstream_loader"

mkdir -p "$output_dir"
probe="$output_dir/pre-instance-contract"
empty_dir="$output_dir/empty"
rust_dir="$output_dir/rust-loader"
upstream_dir="$output_dir/upstream-loader"
mkdir -p "$empty_dir" "$rust_dir" "$upstream_dir"
ln -sfn "$rust_loader" "$rust_dir/libvulkan.so.1"
ln -sfn "$upstream_loader" "$upstream_dir/libvulkan.so.1"

cc -std=c11 -Wall -Wextra -Werror \
  "$repo_root/vk-loader/tests/pre_instance_contract.c" -lvulkan -o "$probe"

run_probe() {
  local loader_dir="$1"
  local stdout_file="$2"
  local stderr_file="$3"
  VK_DRIVER_FILES="$empty_dir/missing-driver.json" \
    VK_LAYER_PATH="$empty_dir" \
    LD_LIBRARY_PATH="$loader_dir${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" \
    "$probe" >"$stdout_file" 2>"$stderr_file"
}

set +e
run_probe "$upstream_dir" "$output_dir/upstream.stdout" "$output_dir/upstream.stderr"
upstream_status=$?
run_probe "$rust_dir" "$output_dir/rust.stdout" "$output_dir/rust.stderr"
rust_status=$?
set -e

if ! compare_pair_outputs "$output_dir" "$upstream_status" "$rust_status"; then
  echo "pre-instance public-contract behavior differs (results: $output_dir)" >&2
  exit 1
fi

discard_matching_pair_logs "$output_dir"
echo "pre-instance public-contract behavior matches upstream"
