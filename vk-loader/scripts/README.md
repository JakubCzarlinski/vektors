# Loader test artifacts

Generated test outputs live under `target/test/<category>/`:

| Category | Contents |
| --- | --- |
| `suite` | Orchestration logs, status and timings |
| `parity` | Upstream build, probes, output comparisons and ABI checks |
| `coverage` | Instrumented builds, profiles and reports |
| `sanitizers` | ASan builds and reports |
| `valgrind` | Build, test copies and Memcheck reports |
| `allocations` | Source inventory, audit-tool build and Heaptrack traces |
| `apps` | Example builds, working directories and results |
| `platform` | Cross-platform builds and Windows tests |
| `codegen` | Reproducibility scratch data and generated-code audits |
| `benchmarks`, `memory` | Benchmark builds, counters and memory reports |
| `build` | Shared ordinary Rust builds used by test scripts |

Explicit output/build overrides remain supported. Historical caches and reports
are not automatically deleted or silently reused. CMake builds must be configured
at their new location, not moved with stale absolute paths. Source checkouts
remain under `.upstream/`.

Scratch directories also live under the relevant category.

`diagnostics/test-valgrind.sh --rust-units [FILTER]` checks Rust unit tests with
a source-built standard library and test harness compatible with Memcheck.
For example, `--rust-units sync::` exercises the native pthread backend as well
as lazy registry locking. Omitting the filter runs all Rust unit tests. Results
are retained in `target/test/valgrind/logs/rust-units-status.tsv` and its companion
XML and output logs. This is separate from `--full`, which runs upstream suites.

`maintenance/clean-artifacts.sh --deep` removes the canonical test tree and
historical build caches; do not run it while tests or profilers are active.
