#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
upstream_loader="${VK_LOADER_PARITY_UPSTREAM_LIBRARY:-$upstream_build_dir/loader/libvulkan.so}"
output_dir="${VK_LOADER_CREATE_TERMINATOR_PARITY_DIR:-$repo_root/target/create-terminator-parity}"
filter='LoaderInstPhysDevExts.PhysDevProps2Simple:LoaderInstPhysDevExts.PhysDevFeats2Simple'

ensure_upstream_tests test_regression "$upstream_loader"
rust_loader="$(resolve_rust_loader "${VK_LOADER_PARITY_RUST_LIBRARY:-}" release)"
require_files "$upstream_loader" "$rust_loader"

mkdir -p "$output_dir"
run_case() {
  local loader="$1"
  local log="$2"
  VK_LOADER_TEST_LOADER_PATH="$loader" \
    "$upstream_build_dir/tests/test_regression" --gtest_color=no \
    "--gtest_filter=$filter" >"$log" 2>&1
}

set +e
run_case "$upstream_loader" "$output_dir/upstream.log"
upstream_status=$?
run_case "$rust_loader" "$output_dir/rust.log"
rust_status=$?
set -e
printf 'implementation\texit_status\nupstream\t%d\nrust\t%d\n' \
  "$upstream_status" "$rust_status" >"$output_dir/status.tsv"

if (( upstream_status != 0 || rust_status != 0 )); then
  echo "layer-modified create-terminator behavior differs (results: $output_dir)" >&2
  exit 1
fi

echo "layer-modified create-terminator behavior passes unchanged upstream tests with both loaders"
