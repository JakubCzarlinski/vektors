#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
output_dir="${VK_LOADER_HEAPTRACK_DIR:-$repo_root/target/heaptrack}"
label="${VK_LOADER_HEAPTRACK_LABEL:-rust-loader}"
filter="${1:-SettingsFile.DeviceConfigurationWithSameDriver}"
benchmark_mode="${VK_LOADER_HEAPTRACK_BENCHMARK_MODE:-}"
benchmark_iterations="${VK_LOADER_HEAPTRACK_BENCHMARK_ITERATIONS:-10}"

require_tools clang heaptrack heaptrack_print mold
if [[ -z "$benchmark_mode" ]]; then
  ensure_upstream_tests
fi

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
if [[ -n "$benchmark_mode" ]]; then
  harness="$output_dir/loader-benchmark"
  library_dir="$(mktemp -d)"
  trap 'rm -f "$library_dir/libvulkan.so.1"; rmdir "$library_dir"' EXIT
  ln -s "$loader" "$library_dir/libvulkan.so.1"
  clang -fuse-ld=mold -O3 -g -DNDEBUG -std=c11 -Wall -Wextra -Werror \
    "$repo_root/vk-loader/tests/loader_benchmark.c" -ldl -lvulkan -o "$harness"
  echo "heaptrack record-only: $benchmark_mode ($benchmark_iterations iterations)"
  LD_LIBRARY_PATH="$library_dir${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" \
    heaptrack --record-only -o "$output_pattern" \
    "$harness" "$benchmark_mode" "$benchmark_iterations"
else
  echo "heaptrack record-only: $filter"
  VK_LOADER_TEST_LOADER_PATH="$loader" \
    heaptrack --record-only -o "$output_pattern" \
    "$upstream_build_dir/tests/test_regression" "--gtest_filter=$filter"
fi

data="$({ find "$output_dir" -maxdepth 1 -type f -name "$label.*.zst" -printf '%T@ %p\n' || true; } | sort -nr | head -1 | cut -d' ' -f2-)"
if [[ -z "$data" ]]; then
  echo "heaptrack did not produce a trace under $output_dir" >&2
  exit 1
fi
report="${data%.zst}.txt"
heaptrack_print --print-peaks=false --print-temporary=true --peak-limit=30 "$data" >"$report"
rg 'calls to allocation functions:|peak heap memory consumption:|total memory leaked:' "$report" || true
echo "Heaptrack data: $data"
echo "Heaptrack CLI report: $report"
