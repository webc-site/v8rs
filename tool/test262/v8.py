#!/usr/bin/env python3
"""
Test262 Conformance Runner for V8 (d8).

Runs Test262 test suite with native V8 d8, prints pass rate,
and records failed test cases into a YAML file for rs8 baseline comparison.
"""

import argparse
import os
from pathlib import Path
import sys

from common import (
    find_repo_root,
    find_v8_binary,
    find_harness_dir,
    run_test_suite,
    save_yaml_list,
)

def main():
    root = find_repo_root()
    default_yml = root / "tool" / "test262" / "v8_fail.yml"

    parser = argparse.ArgumentParser(description="Test262 Runner for V8 (d8)")
    parser.add_argument("--path", nargs="+", default=[
        "v8/test/test262/test/language/expressions",
        "v8/test/test262/test/language/statements"
    ], help="Paths or directories of Test262 test files")
    parser.add_argument("--max", type=int, default=None, help="Maximum tests to run per directory")
    parser.add_argument("-j", "--concurrency", type=int, default=os.cpu_count() or 4, help="Concurrency workers")
    parser.add_argument("--timeout", type=float, default=5.0, help="Per-test timeout in seconds")
    parser.add_argument("--yml", type=str, default=str(default_yml), help="Path to output YAML file for failed tests")
    args = parser.parse_args()

    v8_bin = find_v8_binary()
    print(f"Using V8 binary: {v8_bin}")

    harness_dir = find_harness_dir()
    print(f"Using harness directory: {harness_dir}")

    test_files = []
    for p_str in args.path:
        p = Path(p_str)
        if not p.is_absolute():
            p = root / p
        if p.is_file() and p.suffix == ".js":
            test_files.append(p)
        elif p.is_dir():
            found = sorted(p.rglob("*.js"))
            if args.max:
                found = found[:args.max]
            test_files.extend(found)

    print(f"Collected {len(test_files)} test files.")
    if not test_files:
        print("No test files found.")
        sys.exit(0)

    summary = run_test_suite(
        v8_bin,
        test_files,
        harness_dir,
        skip_set=None,
        concurrency=args.concurrency,
        timeout=args.timeout,
        engine_name="V8"
    )

    failed_files = summary['failed_files']
    save_yaml_list(args.yml, failed_files, title="V8 Test262 Failures")
    print(f"\nRecorded {len(failed_files)} V8 failure(s) to YAML: {args.yml}")
    print(f"V8 Pass Rate: {summary['pass_rate_pct']:.2f}% ({summary['passed']}/{summary['executed']})")

if __name__ == "__main__":
    main()
