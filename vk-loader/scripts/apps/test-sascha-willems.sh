#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
wsi="${VK_LOADER_SASCHA_WSI:-}"
frames="${VK_LOADER_SASCHA_FRAMES:-120}"
duration_seconds="${VK_LOADER_SASCHA_DURATION:-30}"
timeout_seconds="${VK_LOADER_SASCHA_TIMEOUT:-60}"
validation="${VK_LOADER_SASCHA_VALIDATION:-0}"
jobs="${VK_LOADER_SASCHA_JOBS:-2}"
output_dir="${VK_LOADER_SASCHA_OUTPUT_DIR:-$repo_root/target/sascha-willems-results}"

require_tools cargo timeout
[[ "$jobs" =~ ^[1-9][0-9]*$ ]] || {
  echo "VK_LOADER_SASCHA_JOBS must be a positive integer" >&2
  exit 2
}

if [[ -z "$wsi" ]]; then
  if [[ -n "${WAYLAND_DISPLAY:-}" ]]; then
    wsi="wayland"
  else
    wsi="xcb"
  fi
fi

case "$wsi" in
  wayland)
    [[ -n "${WAYLAND_DISPLAY:-}" ]] || {
      echo "WAYLAND_DISPLAY is required for the Wayland examples" >&2
      exit 2
    }
    ;;
  xcb)
    [[ -n "${DISPLAY:-}" ]] || {
      echo "DISPLAY is required for the XCB examples" >&2
      exit 2
    }
    ;;
  *)
    echo "VK_LOADER_SASCHA_WSI must be 'wayland' or 'xcb', got: $wsi" >&2
    exit 2
    ;;
esac

build_dir="$repo_root/target/sascha-willems-vulkan-$wsi"
if [[ ! -x "$build_dir/bin/triangle" ]]; then
  if [[ "${VK_LOADER_SASCHA_NO_SETUP:-0}" == 1 ]]; then
    echo "Sascha Willems examples are not built for $wsi" >&2
    exit 77
  fi
  VK_LOADER_SASCHA_WSI="$wsi" \
    "$loader_scripts/apps/setup-sascha-willems-tests.sh"
fi

rust_loader="$(resolve_rust_loader "${VK_LOADER_LIBRARY:-}" release)"
require_files "$rust_loader"

mapfile -t samples < <(find "$build_dir/bin" -maxdepth 1 -type f -perm -u+x -printf '%f\n' | sort)
if ((${#samples[@]} == 0)); then
  echo "No Sascha Willems example executables found for $wsi" >&2
  exit 77
fi
benchmark_args=(-b -bw 0 -br "$duration_seconds" -bfs "$frames")
if [[ "$validation" == "1" ]]; then
  benchmark_args+=(-v)
fi

mkdir -p "$output_dir"
summary="$output_dir/status-$wsi-validation-$validation.tsv"
printf 'implementation\tsample\tstatus\n' >"$summary"

active_library_dir=""
cleanup_suite() {
  if [[ -n "$active_library_dir" ]]; then
    unlink "$active_library_dir/libvulkan.so.1" 2>/dev/null || true
    rmdir "$active_library_dir" 2>/dev/null || true
    active_library_dir=""
  fi
}
trap cleanup_suite EXIT

run_suite() {
  local label="$1"
  local loader="$2"
  local library_dir label_slug result_dir
  library_dir="$(mktemp -d)"
  active_library_dir="$library_dir"
  ln -s "$loader" "$library_dir/libvulkan.so.1"
  label_slug="${label,,}"
  label_slug="${label_slug// /-}"
  result_dir="$output_dir/results-$wsi-validation-$validation-$label_slug"
  mkdir -p "$result_dir" "$output_dir/logs" \
    "$output_dir/work/$wsi/validation-$validation/$label_slug"

  echo "$label ($wsi, $frames frames per example, $jobs workers)"
  local sample log work_dir
  local -a pids=()
  for sample in "${samples[@]}"; do
    echo "  $sample"
    log="$output_dir/logs/$wsi-validation-$validation-$label_slug-$sample.log"
    work_dir="$output_dir/work/$wsi/validation-$validation/$label_slug/$sample"
    mkdir -p "$work_dir"
    (
      if (
        cd "$work_dir"
        LD_LIBRARY_PATH="$library_dir${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" \
          timeout "${timeout_seconds}s" "$build_dir/bin/$sample" "${benchmark_args[@]}"
      ) >"$log" 2>&1; then
        printf '%s\t%s\tpass\n' "$label" "$sample" >"$result_dir/$sample.tsv"
        rm -f "$log"
      else
        printf '%s\t%s\tfail\n' "$label" "$sample" >"$result_dir/$sample.tsv"
      fi
    ) &
    pids+=("$!")
    if ((${#pids[@]} == jobs)); then
      wait "${pids[@]}"
      pids=()
    fi
  done
  ((${#pids[@]} == 0)) || wait "${pids[@]}"
  find "$result_dir" -maxdepth 1 -type f -name '*.tsv' -print0 \
    | sort -z \
    | xargs -0 cat >>"$summary"
  cleanup_suite
}


run_suite "Rust loader" "$rust_loader"

if [[ "${VK_LOADER_SASCHA_COMPARE_UPSTREAM:-0}" == "1" ]]; then

  upstream_loader="${VK_LOADER_SASCHA_UPSTREAM_LIBRARY:-$upstream_build_dir/loader/libvulkan.so.1.4.361}"
  [[ -f "$upstream_loader" ]] || {
    echo "upstream loader not found: $upstream_loader" >&2
    exit 2
  }
  run_suite "Upstream loader" "$upstream_loader"
fi

failures="$(awk -F '\t' '$3 == "fail" { count++ } END { print count + 0 }' "$summary")"
if (( failures != 0 )); then
  echo "$failures Sascha Willems behavioural tests failed (summary: $summary)" >&2
  exit 1
fi
echo "Sascha Willems behavioural tests passed"
echo "Summary: $summary"
