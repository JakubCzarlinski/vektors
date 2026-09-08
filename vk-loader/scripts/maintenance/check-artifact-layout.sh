#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
require_tools rg bash

# Keep ad-hoc target paths from reappearing; test scripts use shared roots.
if rg -n '\$repo_root/target' "$loader_scripts" -g '*.sh' \
    -g '!common.sh' -g '!clean-artifacts.sh' -g '!check-artifact-layout.sh'; then
  echo "test paths must derive from loader_test_root or loader_test_build_dir" >&2
  exit 1
fi
if rg -n 'mktemp -d\)' "$loader_scripts" -g '*.sh' -g '!check-artifact-layout.sh'; then
  echo "test scratch directories must use a category under target/test" >&2
  exit 1
fi
while IFS= read -r script; do
  bash -n "$script"
done < <(rg --files "$loader_scripts" -g '*.sh')

scratch="$(test_scratch_dir layout-check)"
trap 'rmdir "$scratch"' EXIT
[[ "$scratch" == "$repo_root/target/test/layout-check/"* ]]
[[ "$(rust_loader_library debug)" == "$loader_test_build_dir/debug/"* ]]
echo "Test artifact paths and shell syntax checks passed"
