#!/usr/bin/env python3
"""
Test262 Standard Conformance Runner for rs8 (with V8 baseline skip list).

Executes official Test262 language tests against the pure Rust rs8 engine.
Loads V8 failure YAML to skip tests that native V8 does not pass.
Aims for 100% pass rate on all tests supported by upstream V8.
"""

import argparse
import json
import os
from pathlib import Path
import sys

from common import (
    find_repo_root,
    find_v8_binary,
    find_rs8_binary,
    find_harness_dir,
    load_yaml_set,
    save_yaml_list,
    run_test_suite,
)

def main():
    root = find_repo_root()
    default_fail_yml = root / "tool" / "test262" / "v8_fail.yml"

    parser = argparse.ArgumentParser(description="Test262 Standard Conformance Runner for rs8")
    parser.add_argument("--path", nargs="+", default=[
        "v8/test/test262/test/language/expressions",
        "v8/test/test262/test/language/statements"
    ], help="Paths or directories of Test262 test files")
    parser.add_argument("--max", type=int, default=None, help="Maximum tests to run per directory")
    parser.add_argument("-j", "--concurrency", type=int, default=os.cpu_count() or 4, help="Concurrency workers")
    parser.add_argument("--timeout", type=float, default=5.0, help="Per-test timeout in seconds")
    parser.add_argument("--fail-yml", type=str, default=str(default_fail_yml), help="Path to V8 failure YAML skip list")
    parser.add_argument("--record-v8", action="store_true", help="Run V8 first, record failures to YAML, then run rs8")
    parser.add_argument("--engine", choices=["rs8", "v8"], default="rs8", help="Target JS execution engine")
    parser.add_argument("--report", type=str, default="test262_report.json", help="Path to output JSON report")
    parser.add_argument("--md-report", type=str, default="test262_report.md", help="Path to output Markdown report")
    parser.add_argument("--no-skip", action="store_true", help="Do not skip tests recorded in V8 failure YAML")
    args = parser.parse_args()

    harness_dir = find_harness_dir()
    print(f"Using harness directory: {harness_dir}")

    # Collect test files
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

    print(f"Collected {len(test_files)} test files across target suites.")
    if not test_files:
        print("No test files found.")
        sys.exit(0)

    # If --record-v8 or --engine v8
    if args.record_v8 or args.engine == "v8":
        v8_bin = find_v8_binary()
        print(f"\n[V8 Baseline] Using V8 binary: {v8_bin}")
        v8_summary = run_test_suite(
            v8_bin,
            test_files,
            harness_dir,
            skip_set=None,
            concurrency=args.concurrency,
            timeout=args.timeout,
            engine_name="V8"
        )
        failed_files = v8_summary['failed_files']
        save_yaml_list(args.fail_yml, failed_files, title="V8 Test262 Failures")
        print(f"Recorded {len(failed_files)} V8 failure(s) to YAML: {args.fail_yml}")
        print(f"V8 Pass Rate: {v8_summary['pass_rate_pct']:.2f}% ({v8_summary['passed']}/{v8_summary['executed']})")

        if args.engine == "v8" and not args.record_v8:
            return

    # Load V8 failure YAML to skip
    skip_set = set()
    if not args.no_skip:
        skip_set = load_yaml_set(args.fail_yml)
        if skip_set:
            print(f"Loaded {len(skip_set)} failure records from {args.fail_yml} (will be skipped).")
        else:
            print(f"Note: No skip records loaded from {args.fail_yml} (empty or not found).")

    # Run rs8
    rs8_bin = find_rs8_binary()
    print(f"\n[rs8 Conformance] Using rs8 binary: {rs8_bin}")

    summary = run_test_suite(
        rs8_bin,
        test_files,
        harness_dir,
        skip_set=skip_set,
        concurrency=args.concurrency,
        timeout=args.timeout,
        engine_name="rs8"
    )

    executed = summary['executed']
    passed = summary['passed']
    failed = summary['failed']
    skipped = summary['skipped']
    pass_rate = summary['pass_rate_pct']
    fail_reasons = summary['failure_categories']
    elapsed = summary['elapsed_sec']

    # Save JSON report
    report_data = {
        'engine': 'rs8',
        'total': len(test_files),
        'executed': executed,
        'passed': passed,
        'failed': failed,
        'skipped': skipped,
        'pass_rate_pct': pass_rate,
        'elapsed_sec': elapsed,
        'failure_categories': fail_reasons,
        'details': summary['details']
    }
    with open(args.report, 'w', encoding='utf-8') as f:
        json.dump(report_data, f, indent=2, ensure_ascii=False)
    print(f"Detailed JSON report written to: {args.report}")

    # Save Markdown report
    md_content = f"""# Test262 符合性测试报表 (rs8)

## 1. 执行总览

| 指标 | 数值 |
| :--- | :--- |
| **测试路径** | {", ".join(args.path)} |
| **总用例数** | {len(test_files)} |
| **实际执行** | {executed} |
| **通过用例** | {passed} |
| **失败用例** | {failed} |
| **跳过用例** | {skipped} |
| **通过率** | **{pass_rate:.2f}%** |
| **总耗时** | {elapsed:.2f}s |

## 2. 失败原因聚类

| 失败类型 | 出现频次 | 典型成因分析 |
| :--- | :--- | :--- |
"""
    for cat, cnt in sorted(fail_reasons.items(), key=lambda x: -x[1]):
        md_content += f"| {cat} | {cnt} | 语法特性或标准库未覆盖 |\n"

    md_content += f"""
## 3. 标准符合性结论
- V8 基准跳过记录: {len(skip_set)} 项
- rs8 在 V8 达标集上的通过率为 **{pass_rate:.2f}%**。
"""
    with open(args.md_report, 'w', encoding='utf-8') as f:
        f.write(md_content)
    print(f"Markdown report written to: {args.md_report}")

    if failed > 0:
        sys.exit(1)

if __name__ == "__main__":
    main()
