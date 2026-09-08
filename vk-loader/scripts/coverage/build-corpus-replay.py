#!/usr/bin/env python3
"""Build a loader-neutral corpus runner against an existing upstream test build."""

import argparse
from pathlib import Path
import shlex
import subprocess


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--upstream-build", required=True, type=Path)
    parser.add_argument("--corpus", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    build = args.upstream_build.resolve()
    corpus = args.corpus.resolve(strict=True)
    output = args.output.resolve()
    output.parent.mkdir(parents=True, exist_ok=True)
    commands = subprocess.check_output(
        ["ninja", "-C", str(build), "-t", "commands", "test_fuzzing_loader_neutral"], text=True)
    command = next(line for line in commands.splitlines()
                   if " -c " in line and line.endswith("/loader_fuzz_tests.cpp"))
    arguments = shlex.split(command)
    compiler = arguments[0]
    flags = []
    tokens = iter(arguments[1:])
    for token in tokens:
        if token in ("-o", "-c", "-MT", "-MF"):
            next(tokens)
        elif token != "-MD":
            flags.append(token)
    source = Path(__file__).resolve().parents[2] / "tests/corpus_replay.cpp"
    obj = output.with_suffix(".o")
    subprocess.run([compiler, *flags, f'-DCORPUS_DIRECTORY="{corpus}"',
                    "-c", str(source), "-o", str(obj)], cwd=build, check=True)
    dependencies = [
        "tests/CMakeFiles/test_fuzzing_loader_neutral.dir/loader_testing_main.cpp.o",
        "tests/framework/libtesting_dependencies.a",
        "tests/framework/shim/libshim-library.a",
        "tests/framework/shim/libshim-common.a",
        "tests/framework/util/libtesting_framework_util.a",
        "lib/libgtest.a",
    ]
    subprocess.run([compiler, "-fuse-ld=mold", str(obj), *dependencies,
                    "-ldl", "-o", str(output)], cwd=build, check=True)
    print(output)


if __name__ == "__main__":
    main()
