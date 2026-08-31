#!/usr/bin/env bash

if [[ -n "${VK_LOADER_SCRIPT_COMMON_LOADED:-}" ]]; then
  return 0
fi
VK_LOADER_SCRIPT_COMMON_LOADED=1

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
loader_scripts="$repo_root/vk-loader/scripts"
upstream_dir="$repo_root/.upstream/vulkan-loader"
upstream_build_dir="$upstream_dir/build-rust-parity"

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

ensure_upstream_tests() {
  local suite="${1:-test_regression}"
  local loader="${2:-$upstream_build_dir/loader/libvulkan.so}"
  if [[ ! -x "$upstream_build_dir/tests/$suite" || ! -f "$loader" ]]; then
    "$loader_scripts/parity/setup-upstream-tests.sh"
  fi
}

build_rust_loader() {
  local profile="${1:-release}"
  local target_dir="${2:-$repo_root/target}"
  local profile_args=()
  [[ "$profile" == release ]] && profile_args+=(--release)
  CARGO_TARGET_DIR="$target_dir" \
    cargo build --quiet --manifest-path "$repo_root/Cargo.toml" -p vk-loader \
      "${profile_args[@]}"
}

rust_loader_library() {
  local profile="${1:-release}"
  local target_dir="${2:-$repo_root/target}"
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
  local target_dir="${3:-$repo_root/target}"
  if [[ -n "$override" ]]; then
    printf '%s\n' "$override"
    return
  fi
  build_rust_loader "$profile" "$target_dir" >&2
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
