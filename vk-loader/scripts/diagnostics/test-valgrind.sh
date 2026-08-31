#!/usr/bin/env bash
set -euo pipefail

# The upstream tests intentionally exercise fatal child-process paths. Valgrind
# otherwise writes a full `vgcore.*` image for each one, which can consume many
# gigabytes without helping the memcheck result.
ulimit -c 0

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
log_dir="${VK_LOADER_VALGRIND_LOG_DIR:-$repo_root/target/valgrind}"
error_exitcode=99
container_image="${VK_LOADER_VALGRIND_IMAGE:-archlinux:latest}"
in_container="${VK_LOADER_VALGRIND_IN_CONTAINER:-0}"

if [[ "$in_container" != 1 ]]; then
  ensure_upstream_tests

  require_tools objcopy

  target="$(rustc -vV | sed -n 's/^host: //p')"
  target_env="$(tr '[:lower:]-' '[:upper:]_' <<<"$target")"
  valgrind_target="$repo_root/target/vk-loader-valgrind"
  # Valgrind 3.25 advertises AVX/AVX2 to the guest but not AVX-512, GFNI,
  # VAES or VPCLMULQDQ. Disabling AVX512F also disables its dependent
  # AVX-512 features in LLVM, while retaining the supported AVX2 code paths.
  rustflags='-Ctarget-cpu=native -Ctarget-feature=-avx512f,-gfni,-vaes,-vpclmulqdq -Cforce-frame-pointers=yes'
  if [[ "$target" != x86_64-* ]]; then
    rustflags='-Cforce-frame-pointers=yes'
  fi
  env RUSTC_BOOTSTRAP=1 \
    CARGO_TARGET_DIR="$valgrind_target" \
    CARGO_PROFILE_RELEASE_DEBUG=1 \
    CARGO_PROFILE_RELEASE_STRIP=none \
    "CARGO_TARGET_${target_env}_RUSTFLAGS=$rustflags" \
    cargo build --quiet --manifest-path "$repo_root/Cargo.toml" -p vk-loader \
      --release --target "$target" -Zbuild-std=std,panic_abort
  loader="$valgrind_target/$target/release/libvulkan.so"

  test_copy_dir="$repo_root/target/vk-loader-valgrind-tests"
  mkdir -p "$test_copy_dir"
  for suite in test_regression test_fuzzing test_threading; do
    cp --reflink=auto "$upstream_build_dir/tests/$suite" "$test_copy_dir/$suite"
    # CachyOS marks its startup objects as requiring x86-64-v4 even when the
    # test's actual instruction stream is baseline. Work on a disposable copy.
    if [[ "$target" == x86_64-* ]]; then
      objcopy --remove-section=.note.gnu.property "$test_copy_dir/$suite"
    fi
  done
else
  target="$(uname -m)-unknown-linux-gnu"
  [[ "$(uname -m)" == x86_64 ]] && target=x86_64-unknown-linux-gnu
  loader="$repo_root/target/vk-loader-valgrind/$target/release/libvulkan.so"
  test_copy_dir="$repo_root/target/vk-loader-valgrind-tests"
fi

require_tools valgrind

# Some Valgrind/host combinations cannot execute the dynamic linker's startup
# instructions. Detect that before the test and move to the baseline container.
mkdir -p "$log_dir"
preflight_log="$log_dir/preflight.log"
set +e
bash -c 'ulimit -c 0; valgrind --quiet --error-exitcode="$1" /bin/true; status=$?; :; exit "$status"' \
  valgrind-preflight "$error_exitcode" > /dev/null 2> "$preflight_log"
preflight_status=$?
set -e
if [[ "$preflight_status" -ne 0 ]]; then
  if [[ "$in_container" == 1 ]] || ! command -v docker >/dev/null; then
    echo "Valgrind cannot execute binaries on this host (status $preflight_status)" >&2
    tail -n 20 "$preflight_log" >&2
    exit 77
  fi
  echo "Host Valgrind cannot decode the system linker; using $container_image"
  exec docker run --rm \
    -v "$repo_root:$repo_root" -w "$repo_root" \
    -e VK_LOADER_VALGRIND_IN_CONTAINER=1 \
    -e VK_LOADER_VALGRIND_LOG_DIR="$log_dir" \
    -e VK_LOADER_VALGRIND_UID="$(id -u)" \
    -e VK_LOADER_VALGRIND_GID="$(id -g)" \
    "$container_image" bash -lc '
      set -e
      pacman -Sy --noconfirm --needed valgrind elfutils >/dev/null
      test_home=/tmp/vk-loader-valgrind-home
      mkdir -p "$test_home"
      chown "$VK_LOADER_VALGRIND_UID:$VK_LOADER_VALGRIND_GID" "$test_home"
      mkdir -p "$VK_LOADER_VALGRIND_LOG_DIR"
      chown -R "$VK_LOADER_VALGRIND_UID:$VK_LOADER_VALGRIND_GID" \
        "$VK_LOADER_VALGRIND_LOG_DIR"
      exec setpriv \
        --reuid="$VK_LOADER_VALGRIND_UID" \
        --regid="$VK_LOADER_VALGRIND_GID" \
        --clear-groups \
        env HOME="$test_home" bash -lc '\''
          set -e
          build_id=$(readelf -n /usr/lib/ld-linux-x86-64.so.2 | sed -n "s/.*Build ID: //p")
          DEBUGINFOD_URLS=https://debuginfod.archlinux.org \
            debuginfod-find debuginfo "$build_id" >/dev/null
          exec "$1" "${@:2}"
        '\'' vk-loader-valgrind-user "$1" "${@:2}"
    ' vk-loader-valgrind "$loader_scripts/diagnostics/test-valgrind.sh" "$@"
fi

valgrind_options=(
  --tool=memcheck
  --error-exitcode="$error_exitcode"
  --track-origins=yes
  --leak-check=full
  --show-leak-kinds=definite
  --errors-for-leak-kinds=definite
  --num-callers=40
  --child-silent-after-fork=yes
)

case "${1:-}" in
  --full)
    suites=(test_regression test_fuzzing test_threading)
    filter=()
    ;;
  --suite)
    case "${2:-}" in
      test_regression|test_fuzzing|test_threading) suites=("$2") ;;
      *)
        echo "usage: $0 [--full | --suite {test_regression|test_fuzzing|test_threading} | GTEST_FILTER]" >&2
        exit 2
        ;;
    esac
    filter=()
    ;;
  ?*)
    suites=(test_regression)
    filter=("--gtest_filter=$1")
    ;;
  *)
    suites=(test_regression)
    filter=(--gtest_filter='Allocation.*:CreateDevice.*:WsiTests.*')
    ;;
esac

for suite in "${suites[@]}"; do
  log="$log_dir/$suite.log"
  xml="$log_dir/$suite.xml"
  echo "valgrind: $suite (log: $log, XML: $xml)"
  VK_LOADER_TEST_LOADER_PATH="$loader" \
    valgrind "${valgrind_options[@]}" --log-file="$log" \
      --xml=yes --xml-file="$xml" \
    "$test_copy_dir/$suite" "${filter[@]}"
  if grep -q '<error>' "$xml"; then
    echo "Valgrind reported an error in $suite despite a zero exit status" >&2
    exit "$error_exitcode"
  fi
done

echo "Valgrind parity tests passed"
