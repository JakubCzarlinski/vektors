#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"

output_dir="${VK_LOADER_E2E_MEMORY_DIR:-$repo_root/target/loader-e2e-memory}"
repetitions="${VK_LOADER_E2E_MEMORY_REPETITIONS:-7}"
rust_loader="$(resolve_rust_loader "${VK_LOADER_LIBRARY:-}" release)"
upstream_loader="${VK_LOADER_E2E_UPSTREAM_LIBRARY:-$upstream_dir/build-performance-clang-baseline/loader/libvulkan.so.1.4.361}"
collect_heaptrack="${VK_LOADER_E2E_HEAPTRACK:-0}"
IFS=, read -r -a layers <<<"${VK_LOADER_E2E_LAYERS:-none,VK_LAYER_KHRONOS_validation}"
if (( $# == 0 )); then
  command=(vulkaninfo --summary)
else
  command=("$@")
fi

require_tools "${command[0]}"
if [[ "$collect_heaptrack" == 1 ]]; then
  require_tools heaptrack heaptrack_print
fi
require_files /usr/bin/time "$rust_loader" "$upstream_loader"
[[ "$repetitions" =~ ^[1-9][0-9]*$ ]] || {
  echo "VK_LOADER_E2E_MEMORY_REPETITIONS must be a positive integer" >&2
  exit 2
}

mkdir -p "$output_dir"
samples="$output_dir/samples.csv"
printf 'loader,layer,sample,pair_order,elapsed_seconds,max_rss_kib,minor_faults,major_faults,voluntary_context_switches,involuntary_context_switches,exit_status\n' >"$samples"

rust_library_dir="$(mktemp -d)"
upstream_library_dir="$(mktemp -d)"
cleanup() {
  unlink "$rust_library_dir/libvulkan.so.1" "$upstream_library_dir/libvulkan.so.1" 2>/dev/null || true
  rmdir "$rust_library_dir" "$upstream_library_dir" 2>/dev/null || true
}
trap cleanup EXIT
ln -s "$rust_loader" "$rust_library_dir/libvulkan.so.1"
ln -s "$upstream_loader" "$upstream_library_dir/libvulkan.so.1"

run_sample() {
  local loader_name="$1" layer="$2" sample="$3" pair_order="$4" library_dir metrics status
  if [[ "$loader_name" == rust ]]; then
    library_dir="$rust_library_dir"
  else
    library_dir="$upstream_library_dir"
  fi
  metrics="$output_dir/time-$loader_name-$layer-$sample.txt"
  set +e
  if [[ "$layer" == none ]]; then
    LD_LIBRARY_PATH="$library_dir${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" \
      /usr/bin/time -q -f '%e,%M,%R,%F,%w,%c,%x' -o "$metrics" \
      "${command[@]}" >/dev/null 2>&1
  else
    LD_LIBRARY_PATH="$library_dir${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" \
      VK_INSTANCE_LAYERS="$layer" \
      /usr/bin/time -q -f '%e,%M,%R,%F,%w,%c,%x' -o "$metrics" \
      "${command[@]}" >/dev/null 2>&1
  fi
  status=$?
  set -e
  printf '%s,%s,%s,%s,%s\n' "$loader_name" "$layer" "$sample" "$pair_order" \
    "$(<"$metrics")" >>"$samples"
  if (( status != 0 )); then
    echo "$loader_name sample $sample failed with status $status" >&2
    exit "$status"
  fi
}

for layer in "${layers[@]}"; do
  for ((sample = 0; sample < repetitions; sample++)); do
    case $((sample % 4)) in
      0|3) order=(rust upstream) ;;
      1|2) order=(upstream rust) ;;
    esac
    run_sample "${order[0]}" "$layer" "$sample" 0
    run_sample "${order[1]}" "$layer" "$sample" 1
  done
done

if [[ "$collect_heaptrack" == 1 ]]; then
  for layer in "${layers[@]}"; do
    for loader_name in rust upstream; do
      if [[ "$loader_name" == rust ]]; then
        library_dir="$rust_library_dir"
      else
        library_dir="$upstream_library_dir"
      fi
      pattern="$output_dir/heaptrack-$loader_name-$layer.%p"
      if [[ "$layer" == none ]]; then
        LD_LIBRARY_PATH="$library_dir${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" \
          heaptrack --record-only -o "$pattern" "${command[@]}" >/dev/null
      else
        LD_LIBRARY_PATH="$library_dir${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" \
          VK_INSTANCE_LAYERS="$layer" \
          heaptrack --record-only -o "$pattern" "${command[@]}" >/dev/null
      fi
      data="$({ find "$output_dir" -maxdepth 1 -type f -name "heaptrack-$loader_name-$layer.*.zst" -printf '%T@ %p\n' || true; } | sort -nr | head -1 | cut -d' ' -f2-)"
      [[ -n "$data" ]] || {
        echo "Heaptrack did not produce a $loader_name/$layer trace" >&2
        exit 1
      }
      report="${data%.zst}.txt"
      heaptrack_print --print-peaks=false --print-temporary=true --peak-limit=30 \
        "$data" >"$report"
      echo "$loader_name/$layer Heaptrack: $data"
      rg 'calls to allocation functions:|peak heap memory consumption:|total memory leaked:' \
        "$report" || true
    done
  done
fi

echo "E2E command: ${command[*]}"
echo "Samples: $samples"
for layer in "${layers[@]}"; do
  for loader_name in rust upstream; do
    awk -F, -v loader="$loader_name" -v layer="$layer" '
    BEGIN { count = 0 }
    $1 == loader && $2 == layer {
      rss[count] = $6;
      elapsed[count] = $5;
      minor += $7;
      major += $8;
      count++;
    }
    END {
      for (i = 0; i < count; i++) {
        for (j = i + 1; j < count; j++) {
          if (rss[j] < rss[i]) { value = rss[i]; rss[i] = rss[j]; rss[j] = value }
          if (elapsed[j] < elapsed[i]) { value = elapsed[i]; elapsed[i] = elapsed[j]; elapsed[j] = value }
        }
      }
      middle = int(count / 2);
      printf "%s/%s: median elapsed %.3fs, median max RSS %d KiB, mean minor faults %.1f, mean major faults %.1f\n", loader, layer, elapsed[middle], rss[middle], minor / count, major / count;
    }
  ' "$samples"
  done
done
