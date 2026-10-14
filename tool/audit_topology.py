#!/usr/bin/env python3
"""
V8RS Upstream/Downstream Call Topology & Grounding Audit Script
Inspired by reverify (grounded facts vs AI claims).
"""

import json
import os
import re
import subprocess
from pathlib import Path
from collections import defaultdict
import networkx as nx
from rich.console import Console
from rich.table import Table
from rich.panel import Panel

console = Console()

def get_workspace_metadata(root: Path):
    res = subprocess.run(
        ["cargo", "metadata", "--format-version", "1", "--no-deps"],
        cwd=root,
        capture_output=True,
        text=True,
        check=True
    )
    return json.loads(res.stdout)

def scan_crate_sources(src_dir: Path, all_crate_names: set, current_crate: str):
    if not src_dir.exists():
        return {}, {"todos": 0, "unimplemented": 0}

    crate_name_to_ident = {c: c.replace("-", "_") for c in all_crate_names}
    ident_to_crate = {ident: c for c, ident in crate_name_to_ident.items()}

    used_crates = defaultdict(int)
    hallucination_stats = {"todos": 0, "unimplemented": 0}
    valid_idents = {ident: c for ident, c in ident_to_crate.items() if c != current_crate}
    call_regex = re.compile(r"\buse\s+(?:::)?([a-zA-Z0-9_]+)\b|(?<!/)\b([a-zA-Z0-9_]+)::")

    for rs_file in src_dir.rglob("*.rs"):
        try:
            code = rs_file.read_text(encoding="utf-8")
        except Exception:
            continue

        # 过滤注释，仅统计真实生效的代码中的 todo! 与 unimplemented!
        clean_code = re.sub(r"/\*[\s\S]*?\*/", "", code)
        clean_code = re.sub(r"//.*", "", clean_code)

        hallucination_stats["todos"] += len(re.findall(r"\btodo!\s*\(", clean_code))
        hallucination_stats["unimplemented"] += len(re.findall(r"\bunimplemented!\s*\(", clean_code))

        for m in call_regex.finditer(code):
            ident = m.group(1) or m.group(2)
            if ident in valid_idents:
                used_crates[valid_idents[ident]] += 1

    return used_crates, hallucination_stats

def audit():
    root = Path(__file__).resolve().parent.parent
    metadata = get_workspace_metadata(root)

    packages = metadata["packages"]
    pkg_by_name = {p["name"]: p for p in packages}
    workspace_crates = sorted(pkg_by_name.keys())
    all_crates_set = set(workspace_crates)

    G = nx.DiGraph()
    declared_graph = nx.DiGraph()

    for c in workspace_crates:
        G.add_node(c)
        declared_graph.add_node(c)

    crate_stats = {}
    crate_declared_deps = {}
    crate_actual_calls = {}

    for c in workspace_crates:
        pkg = pkg_by_name[c]
        manifest_dir = Path(pkg["manifest_path"]).parent
        src_dir = manifest_dir / "src"

        declared = {}
        for dep in pkg["dependencies"]:
            dep_name = dep["name"]
            kind = dep.get("kind") or "normal"
            declared[dep_name] = kind

        actual_calls, hstats = scan_crate_sources(src_dir, all_crates_set, c)

        crate_stats[c] = hstats
        crate_declared_deps[c] = declared
        crate_actual_calls[c] = actual_calls

        for dep, kind in declared.items():
            if dep in all_crates_set and kind == "normal":
                declared_graph.add_edge(c, dep)

        for dep, call_count in actual_calls.items():
            if call_count > 0:
                G.add_edge(c, dep, weight=call_count)

    # 1. Pipeline Connectivity Analysis
    critical_pipelines = {
        "Pipeline 1: Parsing -> AST -> Interpreter -> Isolate": [
            ("v8-parsing", "v8-ast"),
            ("v8-interpreter", "v8-ast"),
            ("v8-interpreter", "v8-codegen-core"),
            ("v8-execution-isolate", "v8-interpreter"),
            ("rs8", "v8-execution-isolate"),
        ],
        "Pipeline 2: Objects -> Maps -> Arrays -> Heap Core -> GC": [
            ("v8-objects-maps", "v8-objects-core"),
            ("v8-objects-maps", "v8-objects-arrays"),
            ("v8-objects-maps", "v8-objects-strings"),
            ("v8-heap-core", "v8-objects-core"),
            ("v8-heap-gc", "v8-heap-core"),
            ("v8-execution-isolate", "v8-heap-core"),
            ("v8-execution-isolate", "v8-heap-gc"),
        ],
        "Pipeline 3: Codegen Core -> Arch Codegen -> Builtins": [
            ("v8-codegen-x64", "v8-codegen-core"),
            ("v8-codegen-arm64", "v8-codegen-core"),
            ("v8-builtins-core", "v8-codegen-core"),
            ("v8-builtins-x64", "v8-codegen-x64"),
            ("v8-builtins-arm64", "v8-codegen-arm64"),
        ],
        "Pipeline 4: Turboshaft & Compiler Backend": [
            ("v8-turboshaft", "v8-compiler-graph"),
            ("v8-turboshaft", "v8-compiler-ops"),
            ("v8-compiler-frontend", "v8-compiler-ops"),
            ("v8-compiler-passes", "v8-compiler-frontend"),
            ("v8-compiler-backend", "v8-turboshaft"),
        ],
        "Pipeline 5: Maglev Pipeline": [
            ("v8-maglev-frontend", "v8-maglev-ir"),
            ("v8-maglev-backend", "v8-maglev-frontend"),
        ],
        "Pipeline 6: Wasm Pipeline": [
            ("v8-wasm-objects", "v8-wasm-core"),
            ("v8-wasm-liftoff", "v8-wasm-core"),
            ("v8-wasm-turbofan", "v8-wasm-core"),
            ("v8-wasm-js", "v8-wasm-objects"),
        ],
        "Pipeline 7: Shell & C-API": [
            ("rs8", "v8-api"),
            ("rs8", "v8-libplatform"),
            ("rs8-capi", "v8-api"),
        ],
    }

    console.print(Panel.fit("[bold green]V8RS Pipeline Topology & Grounding Audit[/bold green]"))

    table = Table(title="Critical Execution Pipeline Linkage Verification")
    table.add_column("Pipeline / Edge", style="cyan")
    table.add_column("Declared in Cargo.toml", justify="center")
    table.add_column("Source Callsite Count (Actual)", justify="right")
    table.add_column("Linkage Status", justify="center")

    all_passed = True

    for pname, edges in critical_pipelines.items():
        table.add_row(f"[bold yellow]{pname}[/bold yellow]", "", "", "")
        for u, v in edges:
            declared = v in crate_declared_deps.get(u, {})
            calls = crate_actual_calls.get(u, {}).get(v, 0)
            if declared and calls > 0:
                status = "[green]VERIFIED (CONNECTED)[/green]"
            elif declared and calls == 0:
                status = "[yellow]DECLARED_ONLY (NO_SRC_CALL)[/yellow]"
                all_passed = False
            else:
                status = "[red]REFUTED (DISCONNECTED)[/red]"
                all_passed = False
            table.add_row(f"  {u} -> {v}", "Yes" if declared else "No", str(calls), status)

    console.print(table)

    # 2. Graph Metrics
    console.print(f"\n[bold]Total Crates:[/bold] {len(workspace_crates)}")
    console.print(f"[bold]Total Actual Cross-Crate Call Edges:[/bold] {G.number_of_edges()}")
    console.print(f"[bold]Total Declared Dependency Edges:[/bold] {declared_graph.number_of_edges()}")

    # 3. Orphan check
    in_degrees = dict(G.in_degree())
    out_degrees = dict(G.out_degree())

    leaf_crates = [c for c in workspace_crates if out_degrees[c] == 0]
    root_crates = [c for c in workspace_crates if in_degrees[c] == 0]

    console.print(f"[bold]Top Consumer Crates (in-degree=0):[/bold] {', '.join(root_crates)}")
    console.print(f"[bold]Foundation Leaf Crates (out-degree=0):[/bold] {', '.join(leaf_crates)}")

    # 4. Hallucination stats
    total_todos = sum(s["todos"] for s in crate_stats.values())
    total_unimplemented = sum(s["unimplemented"] for s in crate_stats.values())

    console.print(f"\n[bold]Total `todo!()` in src/ across workspace:[/bold] {total_todos}")
    console.print(f"[bold]Total `unimplemented!()` in src/ across workspace:[/bold] {total_unimplemented}")

    if total_todos > 0:
        todo_table = Table(title="Crates with Residual todo!() in src/")
        todo_table.add_column("Crate", style="red")
        todo_table.add_column("todo!() Count", justify="right")
        for c, s in crate_stats.items():
            if s["todos"] > 0:
                todo_table.add_row(c, str(s["todos"]))
        console.print(todo_table)

    return all_passed

if __name__ == "__main__":
    audit()
