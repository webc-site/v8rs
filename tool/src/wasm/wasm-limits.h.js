#!/usr/bin/env bun
// v8/src/wasm/wasm-limits.h → v8-wasm-core/src/wasm_limits.rs
// 解析全部 constexpr 常量: 数字含 ' 分隔 / 三元 (kSystemPointerSize==4) 取 64 位平台分支 /
// 常量引用与算术表达式求值 / static_assert 直出 const assert。
// 对数: 运行时逐条打印 name = value 与条目计数, 供人工与原版核对。
import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/wasm/wasm-limits.h");

// wasm-constants.h 依赖 (kWasmPageSize), 供 kMaxMemory64Size 求值
const WASM_PAGE_SIZE = 0x10000;

// 归一化: 去 // 注释与 '\n' 连接, 压空白
const flat = src
  .split("\n")
  .map((l) => l.replace(/\/\/.*$/, ""))
  .join(" ")
  .replace(/'/g, "")
  .replace(/\s+/g, " ");

// 解析 constexpr 定义 (到分号为止)
const defs = [];
{
  const re = /constexpr (size_t|uint32_t|uint64_t) (\w+) = ([^;]+);/g;
  let m;
  while ((m = re.exec(flat)) !== null) defs.push({ type: m[1], name: m[2], expr: m[3].trim() });
}

// 表达式求值: 先文本替换已知模式与已定义常量, 再数字算术求值
function evaluate(expr, table) {
  let e = expr
    // 64 位平台分支: cond ? A : B → B (kSystemPointerSize==8)
    .replace(/\w+\s*==\s*4\s*\?[^:]*:([^?]*)/, "$1")
    .replace(/static_cast<uint64_t>\(\s*std::numeric_limits<uint32_t>::max\(\)\s*\)/g, "4294967295")
    .replace(/std::numeric_limits<uint32_t>::max\(\)/g, "4294967295")
    .replace(/\b(?:size_t|uint64_t)\{([^}]*)\}/g, "($1)")
    .replace(/kWasmPageSize/g, String(WASM_PAGE_SIZE));
  for (let pass = 0; pass < 4; pass++) {
    for (const [k, v] of table) e = e.replace(new RegExp(`\\b${k}\\b`, "g"), `(${v})`);
  }
  if (!/^[\d()+\-*/ <>]*$/.test(e)) throw new Error(`无法求值: ${expr} → ${e}`);
  return Function(`"use strict"; return (${e});`)();
}

// 求值并登记
const table = new Map();
const rows = [];
for (const d of defs) {
  const v = evaluate(d.expr, table);
  if (typeof v !== "number" || !Number.isInteger(v) || v < 0) {
    throw new Error(`${d.name} 求值异常: ${v}`);
  }
  table.set(d.name, v);
  rows.push({ ...d, v });
}

// Rust 常量名: kV8MaxWasmTypes → K_V8_MAX_WASM_TYPES
const NAME = (n) => "K_" + n.replace(/^k(?=[A-Z])/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();
// 数字下划线分组 (每 3 位)
const num = (v) => v.toLocaleString("en-US").replace(/,/g, "_");
const rustType = { size_t: "usize", uint32_t: "u32", uint64_t: "u64" };

// 对数输出
console.log(`wasm-limits.h: 提取 ${rows.length} 个 constexpr 常量`);
for (const r of rows) console.log(`  ${r.name} (${r.type}) = ${r.v}`);

let out = `//! v8/src/wasm/wasm-limits.h 的 1:1 转写
//! 登记差异:
//! - 三元 (kSystemPointerSize == 4 ? .. : ..) 统一取 64 位平台分支。
//! - max_mem32_pages()/max_mem64_pages()/max_table_size()/max_module_size()
//!   及 max_mem32_bytes()/max_mem64_bytes() 定义于 wasm-engine.cc, 依赖
//!   v8_flags → 推迟至 flags/engine 梯队 (见 crate README)。
`;
out += banner("src/wasm/wasm-limits.h.js", "src/wasm/wasm-limits.h", "纯常量表; 平台三元统一取 64 位分支 (kSystemPointerSize==8)");
out += `use crate::wasm_constants::WASM_PAGE_SIZE;

/// 规格内存页上限 (声明上限; 运行时仅可增长至此)
/// https://webassembly.github.io/spec/js-api/index.html#limits
pub const K_SPEC_MAX_MEMORY32_PAGES: usize = ${num(table.get("kSpecMaxMemory32Pages"))}; // 4GB
pub const K_SPEC_MAX_MEMORY64_PAGES: usize = ${num(table.get("kSpecMaxMemory64Pages"))}; // 16GB

// --- V8 施加的限制 (与其他引擎协商一致) ---
`;
const skip = new Set(["kSpecMaxMemory32Pages", "kSpecMaxMemory64Pages"]);
for (const r of rows) {
  if (skip.has(r.name)) continue;
  out += `pub const ${NAME(r.name)}: ${rustType[r.type]} = ${num(r.v)};\n`;
}
out += `
// static_assert (wasm-limits.h:81-89, 108): 64 位平台支持完整规格内存上限
const _: () = assert!(K_V8_MAX_WASM_TABLE_SIZE <= 4_294_967_295, "v8 should not exceed WebAssembly's non-web embedding limits");
const _: () = assert!(K_V8_MAX_WASM_TABLE_INIT_ENTRIES <= K_V8_MAX_WASM_TABLE_SIZE, "JS-API should not exceed v8's limit");
const _: () = assert!(K_SPEC_MAX_MEMORY32_PAGES == 65_536 && K_SPEC_MAX_MEMORY64_PAGES == 262_144);
const _: () = assert!(K_V8_MAX_WASM_MEMORY64_PAGES <= u32::MAX as usize);
const _: () = assert!(K_MAX_MEMORY64_SIZE == K_V8_MAX_WASM_MEMORY64_PAGES * WASM_PAGE_SIZE);
`;

const p = writeSrc("v8-wasm-core", "wasm_limits.rs", out);
console.log(`→ ${p}`);
