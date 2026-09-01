#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
variant="${VK_LOADER_BENCH_VARIANT:-baseline}"
case "$variant" in
  baseline)
    default_output_dir="$repo_root/target/loader-benchmarks"
    rust_target_dir="$repo_root/target"
    rust_profile='opt-level=3,lto=false,codegen-units=1,strip=symbols,panic=abort'
    upstream_lto=none
    ;;
  fat-lto)
    default_output_dir="$repo_root/target/loader-benchmarks-fat-lto"
    rust_target_dir="$repo_root/target/vk-loader-benchmark-fat-lto"
    rust_profile='opt-level=3,lto=fat,codegen-units=1,strip=symbols,panic=abort'
    upstream_lto=full
    ;;
  *)
    echo "VK_LOADER_BENCH_VARIANT must be baseline or fat-lto" >&2
    exit 2
    ;;
esac
output_dir="${VK_LOADER_BENCH_OUTPUT_DIR:-$default_output_dir}"
rust_loader="${VK_LOADER_BENCH_RUST_LIBRARY:-$rust_target_dir/release/libvulkan.so}"
performance_build_dir="${VK_LOADER_BENCH_UPSTREAM_BUILD_DIR:-$upstream_dir/build-performance-clang-$variant}"
upstream_loader="${VK_LOADER_BENCH_UPSTREAM_LIBRARY:-$performance_build_dir/loader/libvulkan.so.1.4.361}"
repetitions="${VK_LOADER_BENCH_REPETITIONS:-9}"
bench_cpu="${VK_LOADER_BENCH_CPU:-2}"
collect_perf="${VK_LOADER_BENCH_PERF:-1}"
mode_filter="${VK_LOADER_BENCH_MODE_FILTER:-}"

require_tools clang cmake column mold ninja python3 sha256sum taskset
[[ "$repetitions" =~ ^[1-9][0-9]*$ ]] || {
  echo "VK_LOADER_BENCH_REPETITIONS must be a positive integer" >&2
  exit 2
}
[[ "$bench_cpu" =~ ^[0-9]+$ ]] || {
  echo "VK_LOADER_BENCH_CPU must be a non-negative integer" >&2
  exit 2
}

if [[ "${VK_LOADER_BENCH_NO_BUILD:-0}" != 1 ]]; then
  if [[ "$variant" == fat-lto ]]; then
    CARGO_TARGET_DIR="$rust_target_dir" CARGO_PROFILE_RELEASE_LTO=fat \
      cargo build --quiet --manifest-path "$repo_root/Cargo.toml" -p vk-loader --release
  else
    cargo build --quiet --manifest-path "$repo_root/Cargo.toml" -p vk-loader --release
  fi
  upstream_c_flags='-O3 -DNDEBUG'
  upstream_linker_flags='-fuse-ld=mold'
  if [[ "$upstream_lto" == full ]]; then
    upstream_c_flags+=' -flto=full'
    upstream_linker_flags+=' -flto=full'
  fi
  cmake -S "$upstream_dir" -B "$performance_build_dir" -G Ninja \
    -D CMAKE_BUILD_TYPE=Release \
    -D CMAKE_C_COMPILER=clang \
    -D CMAKE_INTERPROCEDURAL_OPTIMIZATION=OFF \
    -D CMAKE_C_FLAGS_RELEASE="$upstream_c_flags" \
    -D CMAKE_EXE_LINKER_FLAGS="$upstream_linker_flags" \
    -D CMAKE_MODULE_LINKER_FLAGS="$upstream_linker_flags" \
    -D CMAKE_SHARED_LINKER_FLAGS="$upstream_linker_flags" \
    -D UPDATE_DEPS=ON \
    -D BUILD_TESTS=OFF \
    -D BUILD_WERROR=OFF >/dev/null
  cmake --build "$performance_build_dir" --parallel >/dev/null
fi

require_files "$rust_loader" "$upstream_loader"
mkdir -p "$output_dir"
harness="$output_dir/loader-benchmark"
clang -fuse-ld=mold -O3 -DNDEBUG -std=c11 -Wall -Wextra -Werror \
  "$repo_root/vk-loader/tests/loader_benchmark.c" -ldl -lvulkan -o "$harness"

samples="$output_dir/microbenchmarks.csv"
summary="$output_dir/summary.csv"
perf_results="$output_dir/perf-counters.csv"
metadata="$output_dir/environment.txt"
rm -f "$perf_results"
printf 'loader,layer,sample,pair_order,mode,iteration_count,total_ns,ns_per_operation,sink,allocation_calls,allocated_bytes,free_calls\n' >"$samples"

modes=(
  'enumerate-extensions-cold|1|'
  'enumerate-extensions-warm|200|'
  'enumerate-extensions-fill|200|'
  'instance-cycle|10|'
  'device-cycle|10|'
  'instance-gpa-known|5000000|'
  'instance-gpa-missing|1000000|'
  'device-gpa-known|5000000|'
  'device-gpa-missing|1000000|'
  'physical-device-properties|1000000|'
)
instance_commands=(
  vkDestroyInstance
  vkEnumeratePhysicalDevices
  vkGetPhysicalDeviceFeatures
  vkGetPhysicalDeviceProperties
  vkGetPhysicalDeviceQueueFamilyProperties
  vkEnumerateDeviceExtensionProperties
  vkCreateDevice
  vkGetPhysicalDeviceFeatures2
)
device_commands=(
  vkDestroyDevice
  vkGetDeviceQueue
  vkQueueSubmit
  vkDeviceWaitIdle
  vkAllocateMemory
  vkCreateBuffer
  vkCreateImage
  vkCmdDraw
)
if [[ "${VK_LOADER_BENCH_COMMAND_MATRIX:-0}" == 1 ]]; then
  for command in "${instance_commands[@]}"; do
    modes+=("instance-gpa-known|2000000|$command")
  done
  for command in "${device_commands[@]}"; do
    modes+=("device-gpa-known|2000000|$command")
  done
fi
IFS=, read -r -a layers <<<"${VK_LOADER_BENCH_LAYERS:-none,VK_LAYER_KHRONOS_validation}"

rust_library_dir="$(mktemp -d)"
upstream_library_dir="$(mktemp -d)"
cleanup() {
  unlink "$rust_library_dir/libvulkan.so.1" 2>/dev/null || true
  unlink "$upstream_library_dir/libvulkan.so.1" 2>/dev/null || true
  rmdir "$rust_library_dir" "$upstream_library_dir" 2>/dev/null || true
}
trap cleanup EXIT
ln -s "$rust_loader" "$rust_library_dir/libvulkan.so.1"
ln -s "$upstream_loader" "$upstream_library_dir/libvulkan.so.1"

run_sample() {
  local loader_name="$1" layer="$2" mode="$3" iterations="$4"
  local command="$5" repetition="$6" pair_order="$7" library_dir result
  if [[ "$loader_name" == rust ]]; then
    library_dir="$rust_library_dir"
  else
    library_dir="$upstream_library_dir"
  fi
  local -a command_argument=()
  [[ -z "$command" ]] || command_argument+=("$command")
  if [[ "$layer" == none ]]; then
    result="$(LD_LIBRARY_PATH="$library_dir${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" \
      taskset -c "$bench_cpu" "$harness" "$mode" "$iterations" "${command_argument[@]}")"
  else
    result="$(LD_LIBRARY_PATH="$library_dir${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" \
      VK_LOADER_BENCH_LAYER="$layer" taskset -c "$bench_cpu" \
      "$harness" "$mode" "$iterations" "${command_argument[@]}")"
  fi
  printf '%s,%s,%d,%d,%s\n' "$loader_name" "$layer" "$repetition" "$pair_order" "$result" \
    >>"$samples"
}

for layer in "${layers[@]}"; do
  for mode_record in "${modes[@]}"; do
    IFS='|' read -r mode iterations command <<<"$mode_record"
    [[ -z "$mode_filter" || "$mode${command:+-$command}" == "$mode_filter" ]] || continue
    if [[ "$layer" != none && "$mode" == device-gpa-known && -z "$command" ]]; then
      iterations=1000000
    fi
    for ((repetition = 0; repetition < repetitions; repetition++)); do
      case $((repetition % 4)) in
        0|3) order=(rust upstream) ;;
        1|2) order=(upstream rust) ;;
      esac
      run_sample "${order[0]}" "$layer" "$mode" "$iterations" "$command" "$repetition" 0
      run_sample "${order[1]}" "$layer" "$mode" "$iterations" "$command" "$repetition" 1
    done
    echo "benchmark: $layer $mode${command:+ $command}"
  done
done

"$loader_scripts/diagnostics/summarize-benchmarks.py" "$samples" "$summary"

allocation_counter="$output_dir/liballocation-counter.so"
allocation_results="$output_dir/allocations.csv"
clang -shared -fPIC -O2 -std=c11 -Wall -Wextra -Werror \
  "$repo_root/vk-loader/tests/allocation_counter.c" -o "$allocation_counter"
printf 'loader,layer,mode,iteration_count,allocation_calls,allocated_bytes,free_calls\n' \
  >"$allocation_results"
allocation_modes=(
  'enumerate-extensions-cold|1'
  'enumerate-extensions-warm|10'
  'enumerate-extensions-fill|10'
  'instance-cycle|1'
  'device-cycle|1'
  'instance-gpa-known|10000'
  'instance-gpa-missing|10000'
  'device-gpa-known|10000'
  'device-gpa-missing|10000'
  'physical-device-properties|10000'
)
for layer in "${layers[@]}"; do
  for mode_record in "${allocation_modes[@]}"; do
    IFS='|' read -r mode iterations <<<"$mode_record"
    [[ -z "$mode_filter" || "$mode" == "$mode_filter" ]] || continue
    for loader_name in rust upstream; do
      if [[ "$loader_name" == rust ]]; then
        library_dir="$rust_library_dir"
      else
        library_dir="$upstream_library_dir"
      fi
      if [[ "$layer" == none ]]; then
        result="$(LD_PRELOAD="$allocation_counter" \
          LD_LIBRARY_PATH="$library_dir${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" \
          taskset -c "$bench_cpu" "$harness" "$mode" "$iterations")"
      else
        result="$(LD_PRELOAD="$allocation_counter" \
          LD_LIBRARY_PATH="$library_dir${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" \
          VK_LOADER_BENCH_LAYER="$layer" taskset -c "$bench_cpu" \
          "$harness" "$mode" "$iterations")"
      fi
      IFS=, read -r measured_mode measured_iterations _ _ _ allocations bytes frees <<<"$result"
      printf '%s,%s,%s,%s,%s,%s,%s\n' "$loader_name" "$layer" "$measured_mode" \
        "$measured_iterations" "$allocations" "$bytes" "$frees" >>"$allocation_results"
    done
  done
done

{
  echo "date=$(date --iso-8601=seconds)"
  echo "kernel=$(uname -srvmo)"
  echo "cpu=$bench_cpu"
  echo "repetitions=$repetitions"
  echo "variant=$variant"
  echo "rustc=$(rustc --version)"
  echo "clang=$(clang --version | head -n 1)"
  echo "rust_loader=$rust_loader"
  echo "upstream_loader=$upstream_loader"
  echo "rust_sha256=$(sha256sum "$rust_loader" | awk '{print $1}')"
  echo "upstream_sha256=$(sha256sum "$upstream_loader" | awk '{print $1}')"
  echo "rust_profile=$rust_profile"
  echo "upstream_profile=-O3,-DNDEBUG,lto=$upstream_lto,-fvisibility=hidden,-fuse-ld=mold"
  [[ ! -r "/sys/devices/system/cpu/cpu$bench_cpu/cpufreq/scaling_governor" ]] || \
    echo "governor=$(<"/sys/devices/system/cpu/cpu$bench_cpu/cpufreq/scaling_governor")"
  [[ ! -r "/sys/devices/system/cpu/cpu$bench_cpu/cpufreq/scaling_cur_freq" ]] || \
    echo "frequency_khz=$(<"/sys/devices/system/cpu/cpu$bench_cpu/cpufreq/scaling_cur_freq")"
  lscpu 2>/dev/null || true
} >"$metadata"

if [[ "$collect_perf" == 1 ]] && command -v perf >/dev/null && \
  perf stat -x, -e cycles,instructions true >/dev/null 2>&1; then
  perf_summary="$output_dir/perf-summary.csv"
  printf 'loader,layer,mode,iteration_count,group,sample,pair_order,event,value,time_enabled_ns,runtime_percent\n' >"$perf_results"
  perf_modes=(
    'enumerate-extensions-warm|1000'
    'instance-cycle|3'
    'device-cycle|2'
    'instance-gpa-known|5000000'
    'device-gpa-known|5000000'
    'physical-device-properties|2000000'
  )
  perf_groups=(
    'core|7|cycles,instructions,branches,branch-misses'
    'data-cache|5|cache-references,cache-misses,L1-dcache-loads,L1-dcache-load-misses'
    'instruction-cache|5|L1-icache-loads,L1-icache-load-misses,stalled-cycles-frontend'
  )
  if [[ "${VK_LOADER_BENCH_PERF_LEVEL:-standard}" == full ]]; then
    perf_modes+=(
      'enumerate-extensions-fill|1000'
      'instance-gpa-missing|1000000'
      'device-gpa-missing|1000000'
    )
    perf_groups+=(
      'tlb|10|dTLB-loads,dTLB-load-misses,iTLB-loads,iTLB-load-misses'
      'process|7|task-clock,context-switches,cpu-migrations,page-faults,minor-faults,major-faults'
    )
  fi
  if [[ -n "${VK_LOADER_BENCH_PERF_EVENTS:-}" ]]; then
    perf_groups=("custom|${VK_LOADER_BENCH_PERF_REPETITIONS:-10}|$VK_LOADER_BENCH_PERF_EVENTS")
  fi
  for layer in "${layers[@]}"; do
    for mode_record in "${perf_modes[@]}"; do
      IFS='|' read -r mode iterations <<<"$mode_record"
      [[ -z "$mode_filter" || "$mode" == "$mode_filter" ]] || continue
      for group_record in "${perf_groups[@]}"; do
        IFS='|' read -r group group_repetitions candidate_event_list <<<"$group_record"
        group_repetitions="${VK_LOADER_BENCH_PERF_REPETITIONS:-$group_repetitions}"
        IFS=, read -r -a candidate_perf_events <<<"$candidate_event_list"
        perf_events=()
        for event in "${candidate_perf_events[@]}"; do
          if perf stat -e "$event" true >/dev/null 2>&1; then
            perf_events+=("$event")
          fi
        done
        ((${#perf_events[@]} != 0)) || continue
        perf_event_list="$(IFS=,; echo "${perf_events[*]}")"
        for ((sample = 0; sample < group_repetitions; sample++)); do
          case $((sample % 4)) in
            0|3) order=(rust upstream) ;;
            1|2) order=(upstream rust) ;;
          esac
          for pair_order in 0 1; do
            loader_name="${order[$pair_order]}"
            if [[ "$loader_name" == rust ]]; then
              library_dir="$rust_library_dir"
            else
              library_dir="$upstream_library_dir"
            fi
            perf_output="$output_dir/perf-$loader_name-$layer-$mode-$group-$sample.csv"
            if [[ "$layer" == none ]]; then
              LD_LIBRARY_PATH="$library_dir${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" \
                taskset -c "$bench_cpu" perf stat -x, -o "$perf_output" -e "$perf_event_list" \
                "$harness" "$mode" "$iterations" >/dev/null
            else
              LD_LIBRARY_PATH="$library_dir${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" \
                VK_LOADER_BENCH_LAYER="$layer" taskset -c "$bench_cpu" \
                perf stat -x, -o "$perf_output" -e "$perf_event_list" \
                "$harness" "$mode" "$iterations" >/dev/null
            fi
            while IFS=, read -r value _ event time_enabled runtime_percent _; do
              value="${value// /}"
              time_enabled="${time_enabled// /}"
              runtime_percent="${runtime_percent// /}"
              [[ "$value" =~ ^[0-9]+([.][0-9]+)?$ ]] || continue
              printf '%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s\n' \
                "$loader_name" "$layer" "$mode" "$iterations" "$group" "$sample" \
                "$pair_order" "$event" "$value" "${time_enabled:-0}" \
                "${runtime_percent:-100}" >>"$perf_results"
            done <"$perf_output"
          done
        done
      done
    done
  done
  "$loader_scripts/diagnostics/summarize-perf.py" "$perf_results" "$perf_summary"
elif [[ "$collect_perf" == 1 ]]; then
  echo "perf counters unavailable; timing benchmarks still completed" >&2
fi

column -t -s, "$summary"
echo "raw samples: $samples"
echo "summary: $summary"
echo "allocations: $allocation_results"
echo "environment: $metadata"
[[ ! -f "$perf_results" ]] || echo "hardware counters: $perf_results"
[[ ! -f "${perf_summary:-}" ]] || echo "hardware counter summary: $perf_summary"
