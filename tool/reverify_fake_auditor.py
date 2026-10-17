#!/usr/bin/env python3
"""
v8rs 自动化排查 AI 假实现与伪对接工具 (基于 reverify 确定性哲学)
"""

import json
import re
import subprocess
import sys
from pathlib import Path
from collections import defaultdict

def run_cmd(cmd, cwd):
    res = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True)
    return res.returncode, res.stdout, res.stderr

def audit_workspace():
    root = Path(__file__).resolve().parent.parent
    code, out, _ = run_cmd(["cargo", "metadata", "--format-version", "1", "--no-deps"], root)
    if code != 0:
        print("错误: 无法获取 cargo metadata")
        sys.exit(1)
        
    metadata = json.loads(out)
    packages = metadata["packages"]
    
    findings = defaultdict(list)
    total_issues = 0

    # 1. 扫描 AST / 源码级别的假实现模式
    # 模式 A: 公开接口丢弃参数 (_arg) 且只返回常量/默认值
    # 模式 B: 裸硬编码假指针/假句柄 (0x1234, 0xdeadbeef, 0xcafe)
    # 模式 C: 虚假空测试 (测试体内无任何 assert 或只写 assert!(true))
    # 模式 D: 残留的 todo! / unimplemented! 桩
    # 模式 E: 大段 C++ 字符串数组伪造实现
    
    fake_ptr_regex = re.compile(r"\b(0x1234|0xdeadbeef|0xbeef|0xcafe_?[0-9a-fA-F]*)\b")
    stub_discard_regex = re.compile(r"pub\s+fn\s+[a-zA-Z0-9_]+\s*\([^)]*_[a-zA-Z0-9_]+:[^)]*\)\s*->[^{]+\{\s*(Ok\(\(\)\)|true|false|0|None|Default::default\(\))\s*\}")
    fake_test_regex = re.compile(r"#\[test\]\s*(?:async\s+)?fn\s+[a-zA-Z0-9_]+\s*\(\)\s*\{\s*(?:assert!\(true\);?)?\s*\}")
    cpp_string_array_regex = re.compile(r"(?:pub\s+)?(?:static|const)\s+[A-Z0-9_]+:\s*&\[&str\]\s*=\s*&\[[^\]]{100,}\];")

    for pkg in packages:
        cname = pkg["name"]
        pkg_dir = Path(pkg["manifest_path"]).parent
        
        for path in pkg_dir.rglob("*.rs"):
            # 跳过 target 目录
            if "target" in path.parts:
                continue
                
            try:
                text = path.read_text(encoding="utf-8")
            except Exception:
                continue
                
            # 去除注释
            clean_text = re.sub(r"/\*[\s\S]*?\*/", "", text)
            clean_text = re.sub(r"//.*", "", clean_text)
            
            rel_path = path.relative_to(root)
            
            # 检测 A: 丢弃参数的空桩
            for m in stub_discard_regex.finditer(clean_text):
                findings["discard_param_stub"].append(f"{rel_path}: {m.group(0)[:60]}...")
                total_issues += 1
                
            # 检测 B: 裸硬编码假指针
            for m in fake_ptr_regex.finditer(clean_text):
                findings["fake_pointer_literal"].append(f"{rel_path}: 发现假地址字面量 `{m.group(0)}`")
                total_issues += 1
                
            # 检测 C: 虚假空测试
            for m in fake_test_regex.finditer(clean_text):
                findings["empty_or_tautology_test"].append(f"{rel_path}: 发现无意义空测试或自证恒真测试")
                total_issues += 1
                
            # 检测 D: 残留桩
            todos = len(re.findall(r"\btodo!\s*\(", clean_text))
            unimpls = len(re.findall(r"\bunimplemented!\s*\(", clean_text))
            if "src/" in str(rel_path) and (todos > 0 or unimpls > 0):
                findings["residual_stubs"].append(f"{rel_path}: {todos} 处 todo!(), {unimpls} 处 unimplemented!()")
                total_issues += 1

            # 检测 E: 字符串假实现
            for m in cpp_string_array_regex.finditer(clean_text):
                # 排除合理的指令集/符号名称映射
                if "OPCODES" not in m.group(0) and "MNEMONICS" not in m.group(0):
                    findings["cpp_string_stub"].append(f"{rel_path}: 疑似使用 C++ 字符串常量数组伪造接口实现")
                    total_issues += 1

    # 2. 拓扑级假对接扫描：声明了 workspace 内部依赖但源码 0 引用
    all_workspace_crates = {p["name"] for p in packages}
    crate_ident_map = {c.replace("-", "_"): c for c in all_workspace_crates}
    
    for pkg in packages:
        cname = pkg["name"]
        pkg_dir = Path(pkg["manifest_path"]).parent
        src_dir = pkg_dir / "src"
        if not src_dir.exists():
            continue
            
        declared_internal = [
            d["name"] for d in pkg["dependencies"] 
            if d["name"] in all_workspace_crates and d.get("kind") is None
        ]
        
        all_code = ""
        for path in src_dir.rglob("*.rs"):
            try:
                all_code += path.read_text(encoding="utf-8") + "\n"
            except Exception:
                pass
                
        for dep in declared_internal:
            ident = dep.replace("-", "_")
            # 检查是否包含 use dep 或 dep::
            if not re.search(r"\buse\s+(?:::)?(" + ident + r")\b|(?<!/)\b(" + ident + r")::", all_code):
                findings["fake_topology_connection"].append(f"{cname} -> {dep} (Cargo.toml 声明依赖，但 src/ 内 0 调用)")
                total_issues += 1

    # 3. 输出审计报告
    print("=" * 80)
    print("           v8rs 基于 reverify 哲学的 AI 假实现排查报告")
    print("=" * 80)
    
    if total_issues == 0:
        print("[VERIFIED] 全仓未发现已知模式的 AI 假实现、假数据或伪对接！")
        print("状态: 0 缺陷通过")
    else:
        print(f"[REFUTED] 共排查出 {total_issues} 处疑似假实现与伪对接问题：\n")
        for cat, items in findings.items():
            print(f"▼ 发现类别: {cat} (共 {len(items)} 处)")
            for it in items[:10]:  # 最多显示10条
                print(f"  - {it}")
            if len(items) > 10:
                print(f"  ... 剩余 {len(items) - 10} 项省略")
            print()

    print("=" * 80)
    return total_issues

if __name__ == "__main__":
    issues = audit_workspace()
    sys.exit(1 if issues > 0 else 0)
