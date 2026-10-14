#!/usr/bin/env bun
// tool/src/diagnostics/gdb-jit.h.js
// 机械提取 v8/src/diagnostics/gdb-jit.h — GDB JIT 接口声明面。
// MaglevVariableInfo 结构 + maglev 调试文件名规则 + GDBJITInterface 函数签名。
// 注: gdb-jit.cc 侧数据结构已由 tool/src/diagnostics/gdb-jit.js 落 gdb_jit_data.rs,
//     本脚本只覆盖 .h 声明面, 落独立文件避冲突。
// 产物: v8-diagnostics/src/gdb_jit_h_generated.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/diagnostics/gdb-jit.h";
const h = readV8(H_REL);

const structM = h.match(/struct MaglevVariableInfo\s*\{([^}]+)\};/);
if (!structM) throw new Error("未找到 MaglevVariableInfo");
const fields = [...structM[1].matchAll(/int\s+(\w+);/g)].map((m) => m[1]);
if (fields.length !== 3) throw new Error(`MaglevVariableInfo 字段数不符: ${fields.length} != 3`);

const nameFnM = h.match(/inline std::string GetMaglevGraphFilename\(const std::string& func_name,\s*int opt_id\)\s*\{([\s\S]*?)\}/);
if (!nameFnM) throw new Error("未找到 GetMaglevGraphFilename");
if (!nameFnM[1].includes('"maglev-"')) throw new Error("文件名规则锚缺失");

const sigs = [
  ...h.matchAll(/^\s*V8_EXPORT_PRIVATE\s+(?:void|size_t)\s+(?:\n\s*)?(\w+\([^;]*\));/gm),
].map((m) => m[1].replace(/\s+/g, " ").trim());
if (sigs.length < 4) throw new Error(`GDBJITInterface 签名不足: ${sigs.length}`);

let rust = banner("src/diagnostics/gdb-jit.h.js", H_REL, "GDB JIT 接口声明面");
rust += `
// 注: gdb-jit.cc 数据结构由 tool/src/diagnostics/gdb-jit.js 落 gdb_jit_data.rs。
// ENABLE_GDB_JIT_INTERFACE 门控面, 语义实现登记跳过。

/// 原 maglev::MaglevVariableInfo
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaglevVariableInfo {
${fields.map((f) => `  pub ${f}: i32, // 原 ${f}`).join("\n")}
}

impl MaglevVariableInfo {
  /// 原 GetMaglevGraphFilename: \`maglev-<func_name>-<opt_id>.mgl\`
  #[must_use]
  pub fn graph_filename(func_name: &str, opt_id: i32) -> String {
    format!("maglev-{func_name}-{opt_id}.mgl")
  }
}

/// 原 namespace GDBJITInterface 导出函数签名骨架
${sigs.map((s) => `// 原签名: ${s}`).join("\n")}

// —— 提取不变式断言 ——
const _: () = {
  assert!(MaglevVariableInfo { node_id: 0, spill_slot_index: 0, start_pc: 0 }.start_pc == 0);
};
`;

const p = writeSrc("v8-diagnostics", "gdb_jit_h_generated.rs", rust);
console.log(`written ${p} (字段 ${fields.length}, 签名 ${sigs.length})`);
