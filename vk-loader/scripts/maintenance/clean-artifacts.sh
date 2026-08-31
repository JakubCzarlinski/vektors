#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
target_dir="$repo_root/target"

delete_generated_tree() {
  local path="$1"
  case "$path" in
    "$target_dir"/*) ;;
    *)
      echo "refusing to delete path outside target: $path" >&2
      exit 2
      ;;
  esac
  [[ -e "$path" ]] || return 0
  echo "clean: ${path#"$repo_root/"}"
  find "$path" -depth -delete
}

# Core images are purely diagnostic and dwarf the useful Valgrind XML logs.
if [[ -d "$target_dir/valgrind" ]]; then
  core_count="$(find "$target_dir/valgrind" -type f -name '*.core.*' | wc -l)"
  if (( core_count > 0 )); then
    find "$target_dir/valgrind" -type f -name '*.core.*' -delete
    echo "clean: removed $core_count Valgrind core images"
  fi
fi

if [[ "${1:-}" == "--deep" ]]; then
  # Reproducible compiler caches from cross-platform, instrumentation, and
  # code-generation audits. Keep ordinary debug/release artifacts usable, but
  # discard the especially large debug incremental cache.
  build_trees=(
    debug/incremental
    aarch64-apple-darwin
    aarch64-unknown-fuchsia
    armv7-unknown-linux-gnueabihf
    i686-pc-windows-gnu
    i686-unknown-linux-gnu
    loader-benchmarks
    vk-loader-apple-static
    vk-loader-apple-static-probe
    vk-loader-asan
    vk-loader-codegen-audit
    vk-loader-cross
    vk-loader-glibc217
    vk-loader-heaptrack
    vk-loader-qnx-cross
    vk-loader-valgrind
    vk-loader-valgrind-symbols
    vk-loader-valgrind-tests
    x86_64-pc-windows-gnu
    x86_64-unknown-freebsd
    x86_64-unknown-fuchsia
    x86_64-unknown-netbsd
  )
  for name in "${build_trees[@]}"; do
    delete_generated_tree "$target_dir/$name"
  done
elif (( $# > 0 )); then
  echo "usage: $0 [--deep]" >&2
  exit 2
fi

echo "clean: retained current build and test artifacts ($(du -sh "$target_dir" | cut -f1))"
