#!/usr/bin/env bash
set -euo pipefail

args=()
for arg in "$@"; do
  # rustc passes this deprecated no-op to GNU-like linkers. LLD warns about it,
  # so omit it rather than hiding linker diagnostics globally.
  if [[ "$arg" == "-Wl,-O1" ]]; then
    continue
  fi
  args+=("$arg")
done

exec zig cc -target x86_64-linux-gnu.2.17 "${args[@]}"
