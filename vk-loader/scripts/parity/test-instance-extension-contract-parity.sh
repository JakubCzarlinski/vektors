#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
upstream_loader="${VK_LOADER_PARITY_UPSTREAM_LIBRARY:-$upstream_build_dir/loader/libvulkan.so.1.4.361}"
output_dir="${VK_LOADER_EXTENSION_CONTRACT_PARITY_DIR:-$repo_root/target/instance-extension-contract-parity}"

require_tools cargo cc mold
ensure_upstream_tests test_regression "$upstream_loader"
rust_loader="$(resolve_rust_loader "${VK_LOADER_PARITY_RUST_LIBRARY:-}" release)"

fake_icd="$upstream_build_dir/tests/framework/icd/libtest_icd_version_2.so"
require_files "$rust_loader" "$upstream_loader" "$fake_icd"

mkdir -p "$output_dir"
probe="$output_dir/instance-extension-contract"
manifest="$output_dir/test-icd.json"
rust_dir="$output_dir/rust-loader"
upstream_dir="$output_dir/upstream-loader"
mkdir -p "$rust_dir" "$upstream_dir"
ln -sfn "$rust_loader" "$rust_dir/libvulkan.so.1"
ln -sfn "$upstream_loader" "$upstream_dir/libvulkan.so.1"

cc -fuse-ld=mold -std=c11 -Wall -Wextra -Werror \
  "$repo_root/vk-loader/tests/instance_extension_contract.c" -lvulkan -o "$probe"
printf '{"file_format_version":"1.0.0","ICD":{"library_path":"%s","api_version":"1.0.0"}}\n' \
  "$fake_icd" >"$manifest"

run_probe() {
  local loader_dir="$1"
  local stdout_file="$2"
  local stderr_file="$3"
  VK_DRIVER_FILES="$manifest" \
    VK_LAYER_PATH="$output_dir" \
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
  echo "instance-extension public-contract behavior differs (results: $output_dir)" >&2
  exit 1
fi

discard_matching_pair_logs "$output_dir"
echo "instance-extension public-contract behavior matches upstream"
