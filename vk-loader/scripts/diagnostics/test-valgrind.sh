#!/usr/bin/env bash
set -euo pipefail

# The upstream tests intentionally exercise fatal child-process paths. Valgrind
# otherwise writes a full `vgcore.*` image for each one, which can consume many
# gigabytes without helping the memcheck result.
ulimit -c 0

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
log_dir="${VK_LOADER_VALGRIND_LOG_DIR:-$loader_test_root/valgrind/logs}"
error_exitcode=99
container_image="${VK_LOADER_VALGRIND_IMAGE:-archlinux:latest}"
in_container="${VK_LOADER_VALGRIND_IN_CONTAINER:-0}"
rust_units=0
[[ "${1:-}" != --rust-units ]] || rust_units=1

if [[ "$in_container" != 1 ]]; then
  if (( rust_units == 0 )); then
    ensure_upstream_tests test_fuzzing_loader_neutral
  fi

  require_tools objcopy

  target="$(rustc -vV | sed -n 's/^host: //p')"
  target_env="$(tr '[:lower:]-' '[:upper:]_' <<<"$target")"
  valgrind_target="$loader_test_root/valgrind/build"
  # libtest uses unwinding; keep its rebuilt standard-library artifacts apart
  # from the production loader's panic-abort build.
  (( rust_units == 0 )) || valgrind_target="$valgrind_target/rust-units"
  # Valgrind 3.25 advertises AVX/AVX2 to the guest but not AVX-512, GFNI,
  # VAES or VPCLMULQDQ. Disabling AVX512F also disables its dependent
  # AVX-512 features in LLVM, while retaining the supported AVX2 code paths.
  rustflags='-Ctarget-cpu=native -Ctarget-feature=-avx512f,-gfni,-vaes,-vpclmulqdq -Cforce-frame-pointers=yes'
  if [[ "$target" != x86_64-* ]]; then
    rustflags='-Cforce-frame-pointers=yes'
  fi
  build_environment=(
    RUSTC_BOOTSTRAP=1
    "CARGO_TARGET_DIR=$valgrind_target"
    CARGO_PROFILE_RELEASE_DEBUG=1
    CARGO_PROFILE_RELEASE_STRIP=none
    "CARGO_TARGET_${target_env}_RUSTFLAGS=$rustflags"
  )
  if (( rust_units == 1 )); then
    require_tools python3
    build_environment+=(CARGO_PROFILE_RELEASE_PANIC=unwind)
    mkdir -p "$valgrind_target"
    unit_artifacts="$valgrind_target/rust-unit-artifacts.jsonl"
    # Rebuild the test harness and standard library too: the host libtest/libstd
    # can contain instructions that Valgrind cannot decode, even when loader
    # code itself was built with the restricted feature set above.
    env "${build_environment[@]}" \
      cargo test --quiet --manifest-path "$repo_root/Cargo.toml" -p vk-loader \
        --release --target "$target" --lib --no-run --message-format=json-render-diagnostics \
        -Zbuild-std=std,panic_unwind,test >"$unit_artifacts"
    unit_binary="$(python3 - "$unit_artifacts" <<'PY'
import json
import sys

with open(sys.argv[1]) as source:
    executables = [
        record["executable"]
        for line in source
        if (record := json.loads(line)).get("reason") == "compiler-artifact"
        and record.get("executable")
        and record.get("profile", {}).get("test")
    ]
if len(executables) != 1:
    raise SystemExit(f"expected one Rust unit-test executable, found {len(executables)}")
print(executables[0])
PY
)"
  else
    env "${build_environment[@]}" \
      cargo build --quiet --manifest-path "$repo_root/Cargo.toml" -p vk-loader \
        --release --target "$target" -Zbuild-std=std,panic_abort
  fi
  loader="$valgrind_target/$target/release/libvulkan.so"

  test_copy_dir="$loader_test_root/valgrind/tests"
  mkdir -p "$test_copy_dir"
  if (( rust_units == 1 )); then
    copy_suites=(test_rust_units)
  else
    copy_suites=(test_regression test_fuzzing test_threading test_fuzzing_loader_neutral)
  fi
  for suite in "${copy_suites[@]}"; do
    # `objcopy` rewrites the following file in place. Avoid copy-on-write
    # reflinks: some filesystems can SIGBUS while an ELF mapping is replaced.
    source_binary="$upstream_build_dir/tests/$suite"
    (( rust_units == 0 )) || source_binary="$unit_binary"
    cp --reflink=never "$source_binary" "$test_copy_dir/$suite"
    # CachyOS marks its startup objects as requiring x86-64-v4 even when the
    # test's actual instruction stream is baseline. Work on a disposable copy.
    if [[ "$target" == x86_64-* ]]; then
      objcopy --remove-section=.note.gnu.property "$test_copy_dir/$suite"
    fi
  done
else
  target="$(uname -m)-unknown-linux-gnu"
  [[ "$(uname -m)" == x86_64 ]] && target=x86_64-unknown-linux-gnu
  loader="$loader_test_root/valgrind/build/$target/release/libvulkan.so"
  test_copy_dir="$loader_test_root/valgrind/tests"
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
  execution_mount=()
  if (( rust_units == 0 )); then
    private_execution_dir="$(mktemp -d "$loader_test_root/valgrind/execution.XXXXXX")"
    execution_mount=(-v "$private_execution_dir:$upstream_build_dir/tests/executing_tests")
  fi
  exec docker run --rm \
    -v "$repo_root:$repo_root" -w "$repo_root" \
    "${execution_mount[@]}" \
    -e VK_LOADER_VALGRIND_IN_CONTAINER=1 \
    -e VK_LOADER_VALGRIND_LOG_DIR="$log_dir" \
    -e VK_LOADER_VALGRIND_UID="$(id -u)" \
    -e VK_LOADER_VALGRIND_GID="$(id -g)" \
    -e VK_LOADER_VALGRIND_JOBS \
    -e VK_LOADER_VALGRIND_REGRESSION_JOBS \
    -e VK_LOADER_VALGRIND_FUZZING_JOBS \
    -e VK_LOADER_VALGRIND_THREADING_JOBS \
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
  --keep-debuginfo=yes
  --child-silent-after-fork=yes
)

if (( rust_units == 1 )); then
  unit_filter=()
  [[ -z "${2:-}" ]] || unit_filter=("$2")
  started="$(date +%s)"
  status=0
  valgrind "${valgrind_options[@]}" --log-file="$log_dir/rust-units.log" \
    --xml=yes --xml-file="$log_dir/rust-units.xml" \
    "$test_copy_dir/test_rust_units" "${unit_filter[@]}" --test-threads=1 \
    >"$log_dir/rust-units.output.log" 2>&1 || status=$?
  finished="$(date +%s)"
  errors="$(grep -c '<error>' "$log_dir/rust-units.xml" || true)"
  if ! grep -Eq '^running [1-9][0-9]* tests?$' "$log_dir/rust-units.output.log"; then
    echo "Rust unit-test selection did not execute any tests" >&2
    status="$error_exitcode"
  fi
  printf 'suite\tshard\tshard_count\texit_status\tmemcheck_errors\tduration_seconds\n' \
    >"$log_dir/rust-units-status.tsv"
  printf 'rust_units\t0\t1\t%d\t%d\t%d\n' "$status" "$errors" "$((finished - started))" \
    >>"$log_dir/rust-units-status.tsv"
  cat "$log_dir/rust-units.output.log"
  (( status == 0 && errors == 0 )) || exit "$error_exitcode"
  echo "Rust unit-test Memcheck passed ($log_dir/rust-units-status.tsv)"
  exit 0
fi

case "${1:-}" in
  --full)
    suites=(test_regression test_fuzzing test_threading test_fuzzing_loader_neutral)
    filter=()
    ;;
  --suite)
    case "${2:-}" in
      test_fuzzing) suites=(test_fuzzing test_fuzzing_loader_neutral) ;;
      test_regression|test_threading|test_fuzzing_loader_neutral) suites=("$2") ;;
      *)
        echo "usage: $0 [--full | --suite {test_regression|test_fuzzing|test_threading|test_fuzzing_loader_neutral} | GTEST_FILTER]" >&2
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

summary="$log_dir/status.tsv"
printf 'suite\tshard\tshard_count\texit_status\tmemcheck_errors\tduration_seconds\n' >"$summary"
for suite in "${suites[@]}"; do
  case "$suite" in
    test_regression) suite_jobs="${VK_LOADER_VALGRIND_REGRESSION_JOBS:-8}" ;;
    test_fuzzing|test_fuzzing_loader_neutral) suite_jobs="${VK_LOADER_VALGRIND_FUZZING_JOBS:-4}" ;;
    test_threading) suite_jobs="${VK_LOADER_VALGRIND_THREADING_JOBS:-3}" ;;
  esac
  jobs="${VK_LOADER_VALGRIND_JOBS:-$suite_jobs}"
  [[ "$jobs" =~ ^[1-9][0-9]*$ ]] || {
    echo "VK_LOADER_VALGRIND_JOBS must be a positive integer" >&2
    exit 2
  }
  (( jobs > 8 )) && jobs=8
  count="$(gtest_case_count "$suite")"
  (( jobs > count )) && jobs="$count"
  ((${#filter[@]} != 0)) && jobs=1
  suite_filter=("${filter[@]}")
  isolated_filter=()
  if [[ "$suite" == test_regression ]] && ((${#filter[@]} == 0)) && (( jobs > 1 )); then
    # These tests deliberately share literal paths such as /tmp/carol. Keep
    # them out of the sharded pass because the container cannot create an
    # unprivileged mount namespace for every shard.
    suite_filter=(--gtest_filter=-EnvVarICDOverrideSetup.*)
    isolated_filter=(--gtest_filter=EnvVarICDOverrideSetup.*)
  fi

  echo "valgrind: $suite ($jobs shards)"
  pids=()
  outputs=()
  xmls=()
  for ((shard = 0; shard < jobs; shard++)); do
    log="$log_dir/$suite.shard-$shard.log"
    xml="$log_dir/$suite.shard-$shard.xml"
    output="$log_dir/$suite.shard-$shard.output.log"
    shard_status="$log_dir/$suite.shard-$shard.status"
    started="$log_dir/$suite.shard-$shard.started"
    date +%s >"$started"
    outputs+=("$output")
    xmls+=("$xml")
    env \
      GTEST_TOTAL_SHARDS="$jobs" \
      GTEST_SHARD_INDEX="$shard" \
      GTEST_SHARD_STATUS_FILE="$shard_status" \
      VK_LOADER_TEST_LOADER_PATH="$loader" \
      valgrind "${valgrind_options[@]}" --log-file="$log" \
        --xml=yes --xml-file="$xml" \
        "$test_copy_dir/$suite" --gtest_brief=1 "${suite_filter[@]}" >"$output" 2>&1 &
    pids+=("$!")
  done

  suite_failed=0
  for shard in "${!pids[@]}"; do
    status=0
    wait "${pids[$shard]}" || status=$?
    errors=0
    finished="$(date +%s)"
    started="$(<"$log_dir/$suite.shard-$shard.started")"
    rm -f "$log_dir/$suite.shard-$shard.started"
    if [[ -f "${xmls[$shard]}" ]]; then
      errors="$(grep -c '<error>' "${xmls[$shard]}" || true)"
    fi
    printf '%s\t%d\t%d\t%d\t%d\t%d\n' "$suite" "$shard" "$jobs" "$status" "$errors" \
      "$((finished - started))" >>"$summary"
    if (( status != 0 || errors != 0 )); then
      suite_failed=1
      tail -n 200 "${outputs[$shard]}" >&2
    else
      rm -f "${outputs[$shard]}"
    fi
  done
  (( suite_failed == 0 )) || exit "$error_exitcode"

  if ((${#isolated_filter[@]} != 0)); then
    log="$log_dir/$suite.isolated.log"
    xml="$log_dir/$suite.isolated.xml"
    output="$log_dir/$suite.isolated.output.log"
    started="$(date +%s)"
    status=0
    env VK_LOADER_TEST_LOADER_PATH="$loader" \
      valgrind "${valgrind_options[@]}" --log-file="$log" \
        --xml=yes --xml-file="$xml" \
        "$test_copy_dir/$suite" --gtest_brief=1 "${isolated_filter[@]}" \
        >"$output" 2>&1 || status=$?
    finished="$(date +%s)"
    errors=0
    [[ ! -f "$xml" ]] || errors="$(grep -c '<error>' "$xml" || true)"
    printf '%s\t%d\t%d\t%d\t%d\t%d\n' "${suite}_isolated" 0 1 "$status" "$errors" \
      "$((finished - started))" >>"$summary"
    if (( status != 0 || errors != 0 )); then
      tail -n 200 "$output" >&2
      exit "$error_exitcode"
    fi
    rm -f "$output"
  fi
done

echo "Valgrind parity tests passed"
echo "Summary: $summary"
