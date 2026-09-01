#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
upstream_loader="${VK_LOADER_PARITY_UPSTREAM_LIBRARY:-$upstream_build_dir/loader/libvulkan.so.1.4.361}"
output_dir="${VK_LOADER_DEBUG_REPORT_PARITY_DIR:-$repo_root/target/debug-report-forwarding-parity}"

require_tools cargo cc mold
ensure_upstream_tests test_regression "$upstream_loader"
rust_loader="$(resolve_rust_loader "${VK_LOADER_PARITY_RUST_LIBRARY:-}" release)"

mkdir -p "$output_dir/empty" "$output_dir/upstream-loader" "$output_dir/rust-loader"
icd="$output_dir/libdebug_report_icd.so"
probe="$output_dir/debug-report-forwarding"
manifest="$output_dir/debug-report-icd.json"
cc -fuse-ld=mold -std=c11 -Wall -Wextra -Werror -fPIC -shared \
  "$repo_root/vk-loader/tests/fixtures/debug_report_icd.c" -o "$icd"
cc -fuse-ld=mold -std=c11 -Wall -Wextra -Werror \
  "$repo_root/vk-loader/tests/debug_report_icd_forwarding.c" -ldl -lvulkan -o "$probe"
printf '{"file_format_version":"1.0.0","ICD":{"library_path":"%s","api_version":"1.0.0"}}\n' \
  "$icd" > "$manifest"
ln -sfn "$upstream_loader" "$output_dir/upstream-loader/libvulkan.so.1"
ln -sfn "$rust_loader" "$output_dir/rust-loader/libvulkan.so.1"

run_probe() {
  local loader_dir="$1"
  local stdout_file="$2"
  local stderr_file="$3"
  VK_DRIVER_FILES="$manifest" \
    VK_LAYER_PATH="$output_dir/empty" \
    VK_LOADER_FORWARDING_ICD="$icd" \
    LD_LIBRARY_PATH="$loader_dir${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" \
    "$probe" > "$stdout_file" 2> "$stderr_file"
}

set +e
run_probe "$output_dir/upstream-loader" "$output_dir/upstream.stdout" "$output_dir/upstream.stderr"
upstream_status=$?
run_probe "$output_dir/rust-loader" "$output_dir/rust.stdout" "$output_dir/rust.stderr"
rust_status=$?
set -e

if ! compare_pair_outputs "$output_dir" "$upstream_status" "$rust_status"; then
  echo "debug-report ICD forwarding differs (results: $output_dir)" >&2
  exit 1
fi

if ! grep -qx 'icd=2 report_from_utils=1 utils_from_report=1 order=BA alloc=1 free=1' "$output_dir/rust.stdout"; then
  echo "debug messages were not delivered and converted exactly once" >&2
  exit 1
fi
discard_matching_pair_logs "$output_dir"
echo "debug-report ICD forwarding matches upstream"
