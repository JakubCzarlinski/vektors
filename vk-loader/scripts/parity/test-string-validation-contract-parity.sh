#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
upstream_loader="${VK_LOADER_PARITY_UPSTREAM_LIBRARY:-$upstream_build_dir/loader/libvulkan.so.1.4.361}"
output_dir="${VK_LOADER_STRING_VALIDATION_PARITY_DIR:-$loader_test_root/parity/string-validation}"

require_tools cc mold
ensure_upstream_tests test_regression "$upstream_loader"
rust_loader="$(resolve_rust_loader "${VK_LOADER_PARITY_RUST_LIBRARY:-}" release)"
require_files "$rust_loader" "$upstream_loader"

mkdir -p "$output_dir"
probe="$output_dir/string-validation-contract"
rust_dir="$output_dir/rust-loader"
upstream_dir="$output_dir/upstream-loader"
mkdir -p "$rust_dir" "$upstream_dir"
ln -sfn "$rust_loader" "$rust_dir/libvulkan.so.1"
ln -sfn "$upstream_loader" "$upstream_dir/libvulkan.so.1"
cc -fuse-ld=mold -std=c11 -Wall -Wextra -Werror \
  "$repo_root/vk-loader/tests/string_validation_contract.c" -ldl -o "$probe"

run_probe() {
  local loader_dir="$1"
  local stdout_file="$2"
  local stderr_file="$3"
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
  echo "string-validation contract differs (results: $output_dir)" >&2
  exit 1
fi

discard_matching_pair_logs "$output_dir"
echo "string-validation contract matches upstream"
