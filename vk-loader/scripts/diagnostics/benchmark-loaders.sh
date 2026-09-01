#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
output_dir="${VK_LOADER_BENCH_OUTPUT_DIR:-$repo_root/target/loader-benchmarks}"
rust_loader="${VK_LOADER_BENCH_RUST_LIBRARY:-$repo_root/target/release/libvulkan.so}"
upstream_loader="${VK_LOADER_BENCH_UPSTREAM_LIBRARY:-$repo_root/.upstream/vulkan-loader/build-performance/loader/libvulkan.so.1.4.361}"
repetitions="${VK_LOADER_BENCH_REPETITIONS:-15}"
bench_cpu="${VK_LOADER_BENCH_CPU:-2}"

require_tools cc mold taskset
for loader in "$rust_loader" "$upstream_loader"; do
  [[ -f "$loader" ]] || {
    echo "loader not found: $loader" >&2
    exit 2
  }
done

mkdir -p "$output_dir"
harness="$output_dir/loader-benchmark"
cc -fuse-ld=mold -O3 -std=c11 -Wall -Wextra -Werror "$repo_root/vk-loader/tests/loader_benchmark.c" -lvulkan -o "$harness"

printf 'loader,layer,mode,iteration_count,total_ns,ns_per_operation,sink\n' >"$output_dir/microbenchmarks.csv"

modes=(
  enumerate-extensions:200
  instance-cycle:10
  device-cycle:10
  instance-gpa-known:10000000
  instance-gpa-missing:1000000
  device-gpa-known:10000000
  device-gpa-missing:1000000
  physical-device-properties:1000000
)
layers=(none VK_LAYER_KHRONOS_validation)

active_library_dir=""
cleanup() {
  if [[ -n "$active_library_dir" ]]; then
    unlink "$active_library_dir/libvulkan.so.1" 2>/dev/null || true
    rmdir "$active_library_dir" 2>/dev/null || true
  fi
}
trap cleanup EXIT

run_loader() {
  local loader_name="$1"
  local loader="$2"
  local library_dir
  library_dir="$(mktemp -d)"
  active_library_dir="$library_dir"
  ln -s "$loader" "$library_dir/libvulkan.so.1"

  local layer mode_record mode iterations repetition result
  for layer in "${layers[@]}"; do
    for mode_record in "${modes[@]}"; do
      mode="${mode_record%%:*}"
      iterations="${mode_record##*:}"
      if [[ "$layer" != "none" && "$mode" == "device-gpa-known" ]]; then
        iterations=1000000
      fi
      for ((repetition = 0; repetition < repetitions; ++repetition)); do
        if [[ "$layer" == "none" ]]; then
          result="$(LD_LIBRARY_PATH="$library_dir${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" taskset -c "$bench_cpu" "$harness" "$mode" "$iterations")"
        else
          result="$(LD_LIBRARY_PATH="$library_dir${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" VK_LOADER_BENCH_LAYER="$layer" taskset -c "$bench_cpu" "$harness" "$mode" "$iterations")"
        fi
        printf '%s,%s,%s\n' "$loader_name" "$layer" "$result" >>"$output_dir/microbenchmarks.csv"
      done
      echo "$loader_name $layer $mode"
    done
  done

  unlink "$library_dir/libvulkan.so.1"
  rmdir "$library_dir"
  active_library_dir=""
}

run_loader rust "$rust_loader"
run_loader upstream "$upstream_loader"

echo "microbenchmark results: $output_dir/microbenchmarks.csv"
