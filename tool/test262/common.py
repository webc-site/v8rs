#!/usr/bin/env python3
"""
Test262 Runner Common Utilities for V8 and rs8.
"""

from concurrent.futures import ThreadPoolExecutor, as_completed
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import time

FRONTMATTER_RE = re.compile(r'/\*---\s*(.*?)\s*---\*/', re.DOTALL)

def find_repo_root():
    # tool/test262/common.py -> tool/test262 -> tool -> repo_root
    return Path(__file__).resolve().parent.parent.parent

def find_v8_binary():
    env_v8 = os.environ.get("V8_BIN") or os.environ.get("D8_PATH")
    if env_v8:
        p = Path(env_v8)
        if p.is_dir():
            p = p / "d8"
        if p.exists() and os.access(p, os.X_OK):
            return str(p)

    home = Path.home()
    candidates = [
        home / ".jsvu" / "engines" / "v8" / "d8",
        home / ".jsvu" / "bin" / "v8",
        home / ".jsvu" / "bin" / "d8",
    ]

    root = find_repo_root()
    # out directory candidates
    for platform in ["arm64", "x64"]:
        for mode in ["release", "optdebug", "debug"]:
            candidates.append(root / "v8" / "out" / f"{platform}.{mode}" / "d8")

    which_d8 = subprocess.run(["which", "d8"], capture_output=True, text=True)
    if which_d8.returncode == 0 and which_d8.stdout.strip():
        candidates.append(Path(which_d8.stdout.strip()))

    which_v8 = subprocess.run(["which", "v8"], capture_output=True, text=True)
    if which_v8.returncode == 0 and which_v8.stdout.strip():
        candidates.append(Path(which_v8.stdout.strip()))

    for c in candidates:
        if c.exists() and os.access(c, os.X_OK):
            return str(c)

    raise FileNotFoundError("Could not find V8/d8 binary. Set V8_BIN or install via jsvu.")

def find_rs8_binary():
    root = find_repo_root()
    candidates = [
        root / "target" / "release" / "rs8",
        root / "target" / "debug" / "rs8",
        Path("/tmp/rust/release/rs8"),
        Path("/tmp/rust/debug/rs8"),
    ]
    try:
        meta_res = subprocess.run(
            ["cargo", "metadata", "--format-version", "1", "--no-deps"],
            cwd=root,
            capture_output=True,
            text=True
        )
        if meta_res.returncode == 0:
            target_dir = Path(json.loads(meta_res.stdout)["target_directory"])
            candidates.insert(0, target_dir / "release" / "rs8")
            candidates.insert(1, target_dir / "debug" / "rs8")
    except Exception:
        pass

    for c in candidates:
        if c.exists() and os.access(c, os.X_OK):
            return str(c)

    print("Building rs8 binary...", file=sys.stderr)
    subprocess.run(["cargo", "build", "--release", "-p", "rs8"], cwd=root, check=True)
    for c in candidates:
        if c.exists() and os.access(c, os.X_OK):
            return str(c)
    raise FileNotFoundError("Could not find or build rs8 binary")

def find_harness_dir():
    root = find_repo_root()
    harness_candidates = [
        root / "v8" / "test" / "test262" / "harness",
        root / "v8" / "test" / "test262" / "data" / "harness",
    ]
    for cand in harness_candidates:
        if cand.exists() and (cand / "sta.js").exists():
            return cand
    raise FileNotFoundError("test262 harness directory not found")

def parse_frontmatter(content):
    match = FRONTMATTER_RE.search(content)
    if not match:
        return {}
    raw_yaml = match.group(1)
    meta = {}

    inc_m = re.search(r'includes:\s*\[(.*?)\]', raw_yaml, re.DOTALL)
    if inc_m:
        items = [x.strip().strip("'\"") for x in inc_m.group(1).split(',') if x.strip()]
        meta['includes'] = items

    flags_m = re.search(r'flags:\s*\[(.*?)\]', raw_yaml, re.DOTALL)
    if flags_m:
        flags = [x.strip().strip("'\"") for x in flags_m.group(1).split(',') if x.strip()]
        meta['flags'] = flags

    neg_m = re.search(r'negative:\s*\n((?:\s+.*\n?)+)', raw_yaml)
    if neg_m:
        neg_block = neg_m.group(1)
        phase_m = re.search(r'phase:\s*(\w+)', neg_block)
        type_m = re.search(r'type:\s*(\w+)', neg_block)
        meta['negative'] = {
            'phase': phase_m.group(1) if phase_m else 'runtime',
            'type': type_m.group(1) if type_m else 'Test262Error'
        }

    feat_m = re.search(r'features:\s*\[(.*?)\]', raw_yaml, re.DOTALL)
    if feat_m:
        feats = [x.strip().strip("'\"") for x in feat_m.group(1).split(',') if x.strip()]
        meta['features'] = feats

    return meta

def load_yaml_set(file_path):
    p = Path(file_path)
    if not p.exists():
        return set()
    try:
        import yaml
        with open(p, 'r', encoding='utf-8') as f:
            data = yaml.safe_load(f)
            if isinstance(data, list):
                return set(data)
            elif isinstance(data, dict):
                failures = data.get("failures", [])
                return set(failures)
            return set()
    except Exception:
        # Fallback pure-python line parser
        res = set()
        with open(p, 'r', encoding='utf-8') as f:
            for line in f:
                line = line.strip()
                if line.startswith("- "):
                    val = line[2:].strip().strip("'\"")
                    if val:
                        res.add(val)
        return res

def save_yaml_list(file_path, items, title="Test Failures"):
    p = Path(file_path)
    p.parent.mkdir(parents=True, exist_ok=True)
    sorted_items = sorted(items)
    try:
        import yaml
        with open(p, 'w', encoding='utf-8') as f:
            yaml.safe_dump(sorted_items, f, default_flow_style=False, allow_unicode=True)
        return
    except Exception:
        pass

    with open(p, 'w', encoding='utf-8') as f:
        f.write(f"# {title}\n")
        f.write(f"# Total: {len(sorted_items)}\n")
        for it in sorted_items:
            f.write(f"- {it}\n")

def run_single_test(engine_bin, harness_dir, test_path, skip_set=None, timeout=5.0):
    root = find_repo_root()
    try:
        rel_path = str(test_path.relative_to(root))
    except ValueError:
        rel_path = str(test_path)

    if skip_set and (rel_path in skip_set or str(test_path) in skip_set or test_path.name in skip_set):
        return {
            'file': rel_path,
            'status': 'SKIP',
            'reason': 'Skipped (recorded in failure YAML)'
        }

    try:
        with open(test_path, 'r', encoding='utf-8', errors='replace') as f:
            content = f.read()
    except Exception as e:
        return {
            'file': rel_path,
            'status': 'SKIP',
            'reason': f"Read error: {e}"
        }

    meta = parse_frontmatter(content)
    flags = meta.get('flags', [])

    if 'module' in flags:
        return {'file': rel_path, 'status': 'SKIP', 'reason': 'ES module format unsupported'}
    if 'async' in flags:
        return {'file': rel_path, 'status': 'SKIP', 'reason': 'async harness unsupported'}
    if 'raw' in flags:
        return {'file': rel_path, 'status': 'SKIP', 'reason': 'raw flag unsupported'}

    sta_js = harness_dir / "sta.js"
    assert_js = harness_dir / "assert.js"
    cmd = [engine_bin, str(sta_js), str(assert_js)]

    includes = meta.get('includes', [])
    for inc in includes:
        inc_file = harness_dir / inc
        if inc_file.exists():
            cmd.append(str(inc_file))
        else:
            return {'file': rel_path, 'status': 'SKIP', 'reason': f"Missing include: {inc}"}

    cmd.append(str(test_path))

    negative = meta.get('negative')
    try:
        proc = subprocess.run(
            cmd,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=timeout,
            text=True
        )
    except subprocess.TimeoutExpired:
        return {'file': rel_path, 'status': 'FAIL', 'reason': f"Timeout (>{timeout}s)"}
    except Exception as e:
        return {'file': rel_path, 'status': 'FAIL', 'reason': f"Process error: {e}"}

    if negative:
        expected_type = negative.get('type', 'Error')
        if proc.returncode != 0:
            return {'file': rel_path, 'status': 'PASS', 'negative': True}
        else:
            return {
                'file': rel_path,
                'status': 'FAIL',
                'reason': f"Expected error {expected_type}, but exited cleanly"
            }
    else:
        if proc.returncode == 0:
            return {'file': rel_path, 'status': 'PASS', 'negative': False}
        else:
            err = proc.stderr.strip() or proc.stdout.strip()
            err_line = err.splitlines()[0] if err else f"Exit code {proc.returncode}"
            return {
                'file': rel_path,
                'status': 'FAIL',
                'reason': err_line
            }

def run_test_suite(engine_bin, test_files, harness_dir, skip_set=None, concurrency=None, timeout=5.0, engine_name="Engine"):
    concurrency = concurrency or (os.cpu_count() or 4)
    start_time = time.time()
    results = []
    passed = 0
    failed = 0
    skipped = 0
    fail_reasons = {}
    failed_files = []

    print(f"Executing {len(test_files)} tests with {engine_name} ({concurrency} workers)...")

    with ThreadPoolExecutor(max_workers=concurrency) as executor:
        futures = {
            executor.submit(run_single_test, engine_bin, harness_dir, tf, skip_set, timeout): tf
            for tf in test_files
        }
        for future in as_completed(futures):
            res = future.result()
            results.append(res)
            st = res['status']
            if st == 'PASS':
                passed += 1
            elif st == 'FAIL':
                failed += 1
                failed_files.append(res['file'])
                reason = res.get('reason', 'Unknown error')
                cat = "Other"
                if "语法解析错误" in reason or "ParseError" in reason or "Expected" in reason:
                    cat = "Syntax/Parse Error"
                elif "Test262Error" in reason:
                    cat = "Test262 Assertion Failed"
                elif "not defined" in reason:
                    cat = "ReferenceError / Undefined Symbol"
                elif "Timeout" in reason:
                    cat = "Timeout"
                elif "TypeError" in reason or "not a function" in reason:
                    cat = "TypeError"
                fail_reasons[cat] = fail_reasons.get(cat, 0) + 1
            elif st == 'SKIP':
                skipped += 1

            total_done = passed + failed + skipped
            if total_done % 20 == 0 or total_done == len(test_files):
                pct = (passed / (passed + failed) * 100) if (passed + failed) > 0 else 0.0
                print(f"[{engine_name}] [{total_done}/{len(test_files)}] | Passed: {passed} | Failed: {failed} | Skipped: {skipped} | Pass Rate: {pct:.1f}%", end="\r", flush=True)

    elapsed = time.time() - start_time
    print()

    executed = passed + failed
    pass_rate = (passed / executed * 100.0) if executed > 0 else 0.0

    print("\n" + "=" * 60)
    print(f"{engine_name} Test262 Conformance Summary")
    print("=" * 60)
    print(f"Total Test Cases   : {len(test_files)}")
    print(f"Executed           : {executed}")
    print(f"Passed             : {passed} ({pass_rate:.2f}%)")
    print(f"Failed             : {failed}")
    print(f"Skipped            : {skipped}")
    print(f"Execution Time     : {elapsed:.2f}s ({(len(test_files)/elapsed if elapsed > 0 else 0):.1f} tests/sec)")
    if fail_reasons:
        print("-" * 60)
        print("Failure Categories:")
        for cat, cnt in sorted(fail_reasons.items(), key=lambda x: -x[1]):
            print(f"  - {cat:<35}: {cnt}")
    print("=" * 60)

    return {
        'total': len(test_files),
        'executed': executed,
        'passed': passed,
        'failed': failed,
        'skipped': skipped,
        'pass_rate_pct': pass_rate,
        'elapsed_sec': elapsed,
        'failure_categories': fail_reasons,
        'failed_files': failed_files,
        'details': results
    }
