#!/usr/bin/env bun
// tool/src/objects/dependent-code-inl.h.js — 覆盖 v8/src/objects/dependent-code-inl.h (单文件族)
// → v8-objects-maps/src/dependent_code_inl.rs
// 机械提取: DependentCode 模板内联入口签名骨架 (函数体留 todo! + /// v8: 锚)
// 对数 oracle: clang -E 预处理 v8/src/objects/dependent-code-inl.h
//   (3 个模板定义计数与签名文本取自编译器产物)

import { execFileSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { tmpdir } from "node:os";
import { readV8, writeSrc, banner, root } from "../../lib.js";

const inl = readV8("src/objects/dependent-code-inl.h");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};

// ── clang -E 对数基准 (原版侧 = 编译器预处理产物; 缺失头自动空 stub) ──────
const ppShim = join(tmpdir(), "v8rs-r3o4-pp-shim");
function clangE(rel) {
  const args = [
    "-E", "-P", "-x", "c++", "-std=c++20", "-I", "v8", "-I", "v8/include",
    "-I", ppShim, "-DV8_INTL_SUPPORT=1", "-DV8_ENABLE_WEBASSEMBLY=1", "-",
  ];
  for (let round = 0; round < 48; round++) {
    try {
      return execFileSync("clang", args, {
        input: `#include "${rel}"\n`,
        encoding: "utf8",
        cwd: root,
        maxBuffer: 512 * 1024 * 1024,
      });
    } catch (e) {
      const err = String(e.stderr ?? "");
      const m = err.match(/fatal error: '([^']+)' file not found/);
      if (!m) throw new Error(`clang -E ${rel} 失败: ${err.slice(0, 300)}`);
      const stub = join(ppShim, m[1]);
      mkdirSync(dirname(stub), { recursive: true });
      writeFileSync(stub, "#pragma once\n");
    }
  }
  throw new Error(`clang -E ${rel}: stub 轮数超限`);
}

// ── 产物 fmt 归一 (与 cargo +nightly fmt 同配置; 幂等) ────────────────────
function fmtNormalize(p) {
  const rustfmt = execFileSync("rustup", ["which", "--toolchain", "nightly", "rustfmt"],
    { encoding: "utf8", cwd: root }).trim();
  execFileSync(rustfmt, ["--edition", "2024", "--config-path", join(root, "rustfmt.toml"), p],
    { cwd: root });
}


const inlpp = clangE("src/objects/dependent-code-inl.h");

// pp 定义清单 (3 个模板; 剥 __attribute__ + 折叠缩进后匹配列首定义)
const defs = [...inlpp.replace(/__attribute__\(\(.*?\)\)/g, "")
  .split("\n").map((l) => l.replace(/^\s+/, "")).join("\n")
  .matchAll(/^(?:void|bool) DependentCode::(\w+)\(/gm)].map((m) => m[1]);
must(JSON.stringify(defs) === JSON.stringify([
  "DeoptimizeDependencyGroups", "DeoptimizeDependencyGroups", "MarkCodeForDeoptimization",
]), `pp 定义清单失配: ${JSON.stringify(defs)}`);

// pp 签名锚 (重载分辨: ObjectT 直传 vs Tagged<ObjectT>)
const flat = inlpp.replace(/\n\s+/g, " ");
must(flat.includes("void DependentCode::DeoptimizeDependencyGroups(Isolate* isolate, ObjectT object, DependencyGroups groups)"),
  "pp 缺 ObjectT 直传重载锚");
must(flat.includes("void DependentCode::DeoptimizeDependencyGroups(Isolate* isolate, Tagged<ObjectT> object, DependencyGroups groups)"),
  "pp 缺 Tagged 重载锚");
must(flat.includes("bool DependentCode::MarkCodeForDeoptimization(Isolate* isolate, Tagged<ObjectT> object, DependencyGroups groups)"),
  "pp 缺 MarkCodeForDeoptimization 锚");
// 共享/只读空间不失效断言 (语义锚; DCHECK 在 pp 中展开为空 → 取原文)
must(inl.includes("!HeapLayout::InAnySharedSpace(object)"), "原文缺共享空间断言锚");

// 原文行号锚
const lineOf = {};
inl.split("\n").forEach((l, i) => {
  const m = l.match(/void DependentCode::(\w+)\(|bool DependentCode::(\w+)\(/);
  if (m) {
    const name = m[1] ?? m[2];
    if (lineOf[name] === undefined) lineOf[name] = i + 1;
  }
});
must(lineOf.DeoptimizeDependencyGroups !== undefined && lineOf.MarkCodeForDeoptimization !== undefined,
  "原文行号锚缺失");

let rust = banner("src/objects/dependent-code-inl.h.js", "src/objects/dependent-code-inl.h",
  "DependentCode 内联入口骨架 (clang -E 对数; 函数体留 todo!)");
rust += `//! DependentCode 内联模板入口: 对象 → 依赖代码失效/标记。
//! 原版为 template <typename ObjectT> 双重载; Rust 侧以不透明句柄占位,
//! 语义面 (HeapLayout 共享/只读空间判定) 留 Developer 修订。
//! DependencyGroup 位标志表 (11 组) 归 v8-objects-rest 的 dependent-code.h 侧脚本。

/// v8: dependent-code-inl.h:${lineOf.DeoptimizeDependencyGroups} — template <typename ObjectT>
///   void DependentCode::DeoptimizeDependencyGroups(Isolate*, ObjectT, DependencyGroups)
///
/// 直传重载: 转发到 Tagged 重载 (static_assert kTaggedCanConvertToRawObjects)。
pub fn deoptimize_dependency_groups(_isolate: &str, _object: usize, _groups: u32) {
  todo!()
}

/// v8: dependent-code-inl.h:${lineOf.DeoptimizeDependencyGroups + 2} — template <typename ObjectT>
///   void DependentCode::DeoptimizeDependencyGroups(Isolate*, Tagged<ObjectT>, DependencyGroups)
///
/// 共享对象与只读对象设计上永不失效代码 (原版 DCHECK 双否定)。
pub fn deoptimize_dependency_groups_tagged(_isolate: &str, _object: usize, _groups: u32) {
  todo!()
}

/// v8: dependent-code-inl.h:${lineOf.MarkCodeForDeoptimization} — template <typename ObjectT>
///   bool DependentCode::MarkCodeForDeoptimization(Isolate*, Tagged<ObjectT>, DependencyGroups)
///
/// 返回是否命中任一依赖组。
pub fn mark_code_for_deoptimization(_isolate: &str, _object: usize, _groups: u32) -> bool {
  todo!()
}
`;

const p = writeSrc("v8-objects-maps", "dependent_code_inl.rs", rust);
fmtNormalize(p);
console.log(`dependent_code_inl.rs: 模板入口 ${defs.length} (pp 对数), 断言 ${checks} 项通过 → ${p}`);
