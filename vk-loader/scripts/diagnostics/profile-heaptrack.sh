#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
output_dir="${VK_LOADER_HEAPTRACK_DIR:-$repo_root/target/heaptrack}"
label="${VK_LOADER_HEAPTRACK_LABEL:-rust-loader}"
filter="${1:-SettingsFile.DeviceConfigurationWithSameDriver}"

require_tools heaptrack heaptrack_print
ensure_upstream_tests

if [[ -n "${VK_LOADER_HEAPTRACK_LIBRARY:-}" ]]; then
  loader="$VK_LOADER_HEAPTRACK_LIBRARY"
else
  profile_target="$repo_root/target/vk-loader-heaptrack"
  CARGO_TARGET_DIR="$profile_target" \
    CARGO_PROFILE_RELEASE_DEBUG=1 \
    CARGO_PROFILE_RELEASE_STRIP=none \
    cargo build --quiet --manifest-path "$repo_root/Cargo.toml" -p vk-loader --release
  loader="$profile_target/release/libvulkan.so"
fi

mkdir -p "$output_dir"
output_pattern="$output_dir/$label.%p"
echo "heaptrack record-only: $filter"
VK_LOADER_TEST_LOADER_PATH="$loader" \
  heaptrack --record-only -o "$output_pattern" \
  "$upstream_build_dir/tests/test_regression" "--gtest_filter=$filter"

data="$({ find "$output_dir" -maxdepth 1 -type f -name "$label.*.zst" -printf '%T@ %p\n' || true; } | sort -nr | head -1 | cut -d' ' -f2-)"
if [[ -z "$data" ]]; then
  echo "heaptrack did not produce a trace under $output_dir" >&2
  exit 1
fi
report="${data%.zst}.txt"
heaptrack_print "$data" >"$report"
rg 'calls to allocation functions:|peak heap memory consumption:|total memory leaked:' "$report" || true
echo "Heaptrack data: $data"
echo "Heaptrack CLI report: $report"
