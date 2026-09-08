#!/usr/bin/env bash

if [[ -n "${VK_LOADER_SCRIPT_COMMON_LOADED:-}" ]]; then
  return 0
fi
VK_LOADER_SCRIPT_COMMON_LOADED=1

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
loader_scripts="$repo_root/vk-loader/scripts"
loader_test_root="$repo_root/target/test"
loader_test_build_dir="$loader_test_root/build/rust"
windows_upstream_build_dir="${VK_LOADER_WINDOWS_UPSTREAM_BUILD_DIR:-$loader_test_root/platform/windows/upstream}"
upstream_dir="$repo_root/.upstream/vulkan-loader"
upstream_build_dir="${VK_LOADER_UPSTREAM_BUILD_DIR:-$loader_test_root/parity/upstream}"

test_scratch_dir() {
  local category="$1"
  [[ "$category" =~ ^[a-z0-9][a-z0-9/-]*$ ]] || {
    echo "invalid test category: $category" >&2
    return 2
  }
  mkdir -p "$loader_test_root/$category"
  mktemp -d "$loader_test_root/$category/work.XXXXXX"
}

require_tools() {
  local tool
  for tool in "$@"; do
    command -v "$tool" >/dev/null || {
      echo "required tool not found: $tool" >&2
      return 2
    }
  done
}

require_files() {
  local path
  for path in "$@"; do
    [[ -f "$path" ]] || {
      echo "required file not found: $path" >&2
      return 2
    }
  done
}

resolve_coverage_source() {
  local reported="$1"
  if [[ "$reported" == /* && -f "$reported" ]]; then
    printf '%s\n' "$reported"
  elif [[ -f "$repo_root/$reported" ]]; then
    printf '%s\n' "$repo_root/$reported"
  elif [[ -f "/$reported" ]]; then
    # llvm-cov can omit the leading slash from absolute paths in text reports.
    printf '/%s\n' "$reported"
  else
    return 1
  fi
}

ensure_upstream_tests() {
  local suite="${1:-test_regression}"
  local loader="${2:-$upstream_build_dir/loader/libvulkan.so}"
  if [[ ! -x "$upstream_build_dir/tests/$suite" || ! -f "$loader" ]]; then
    "$loader_scripts/parity/setup-upstream-tests.sh"
  fi
}

gtest_case_count() {
  local suite="$1"
  "$upstream_build_dir/tests/$suite" --gtest_list_tests |
    awk '
      /^[^[:space:]].*\.$/ { suite = $1; next }
      /^  [^[:space:]]/ {
        test = $1
        if (suite !~ /(^|\/)DISABLED_/ && test !~ /^DISABLED_/) count++
      }
      END { print count + 0 }
    '
}

loader_test_jobs() {
  if [[ -n "${VK_LOADER_TEST_JOBS:-}" ]]; then
    [[ "$VK_LOADER_TEST_JOBS" =~ ^[1-9][0-9]*$ ]] || {
      echo "VK_LOADER_TEST_JOBS must be a positive integer" >&2
      return 2
    }
    printf '%s\n' "$VK_LOADER_TEST_JOBS"
    return
  fi
  local jobs
  jobs="$(nproc 2>/dev/null || getconf _NPROCESSORS_ONLN 2>/dev/null || printf '1')"
  (( jobs > 8 )) && jobs=8
  printf '%s\n' "$jobs"
}

run_gtest_shards() {
  local loader="$1"
  local suite="$2"
  local output="$3"
  shift 3

  local jobs count
  jobs="$(loader_test_jobs)" || return
  count="$(gtest_case_count "$suite")"
  (( jobs > count )) && jobs="$count"
  local argument
  for argument in "$@"; do
    if [[ "$argument" == --gtest_filter=* ]]; then
      jobs=1
      break
    fi
  done

  local -a pids=() logs=() status_files=()
  local shard log status_file
  for ((shard = 0; shard < jobs; shard++)); do
    log="$output.shard-$shard.log"
    status_file="$output.shard-$shard.status"
    logs+=("$log")
    status_files+=("$status_file")
    env \
      GTEST_TOTAL_SHARDS="$jobs" \
      GTEST_SHARD_INDEX="$shard" \
      GTEST_SHARD_STATUS_FILE="$status_file" \
      VK_LOADER_TEST_LOADER_PATH="$loader" \
      "$upstream_build_dir/tests/$suite" "$@" >"$log" 2>&1 &
    pids+=("$!")
  done

  local result=0 pid
  for pid in "${pids[@]}"; do
    wait "$pid" || result=1
  done
  : >"$output"
  for log in "${logs[@]}"; do
    cat "$log" >>"$output"
  done
  if (( result == 0 )); then
    rm -f "${logs[@]}" "${status_files[@]}"
  else
    tail -n 200 "$output" >&2
  fi
  return "$result"
}

build_rust_loader() {
  local profile="${1:-release}"
  local target_dir="${2:-$loader_test_build_dir}"
  local profile_args=()
  [[ "$profile" == release ]] && profile_args+=(--release)
  CARGO_TARGET_DIR="$target_dir" \
    cargo build --quiet --manifest-path "$repo_root/Cargo.toml" -p vk-loader \
      "${profile_args[@]}"
}

rust_loader_library() {
  local profile="${1:-release}"
  local target_dir="${2:-$loader_test_build_dir}"
  local directory="$target_dir/$profile"
  case "$(uname -s)" in
    Darwin) printf '%s/libvulkan.dylib\n' "$directory" ;;
    Linux|FreeBSD|OpenBSD|DragonFly) printf '%s/libvulkan.so\n' "$directory" ;;
    MINGW*|MSYS*|CYGWIN*) printf '%s/vulkan.dll\n' "$directory" ;;
    *) echo "unsupported loader platform: $(uname -s)" >&2; return 2 ;;
  esac
}

resolve_rust_loader() {
  local override="${1:-}"
  local profile="${2:-release}"
  local target_dir="${3:-$loader_test_build_dir}"
  if [[ -n "$override" ]]; then
    printf '%s\n' "$override"
    return
  fi
  build_rust_loader "$profile" "$target_dir" >&2 || return $?
  rust_loader_library "$profile" "$target_dir"
}

compare_pair_outputs() {
  local output_dir="$1"
  local upstream_status="$2"
  local rust_status="$3"
  printf 'implementation\texit_status\nupstream\t%d\nrust\t%d\n' \
    "$upstream_status" "$rust_status" >"$output_dir/status.tsv"

  local matches=0
  diff -u "$output_dir/upstream.stdout" "$output_dir/rust.stdout" \
    >"$output_dir/stdout.diff" || matches=1
  diff -u "$output_dir/upstream.stderr" "$output_dir/rust.stderr" \
    >"$output_dir/stderr.diff" || matches=1
  (( upstream_status == 0 && rust_status == 0 && matches == 0 ))
}

discard_matching_pair_logs() {
  local output_dir="$1"
  [[ "${VK_LOADER_KEEP_MATCH_LOGS:-0}" == 1 ]] && return
  rm -f "$output_dir"/{upstream,rust}.{stdout,stderr} \
    "$output_dir"/{stdout,stderr}.diff
}
