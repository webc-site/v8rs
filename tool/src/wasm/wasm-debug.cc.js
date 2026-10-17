#!/usr/bin/env bun
// tool/src/wasm/wasm-debug.cc.js
// 机械提取 v8/src/wasm/wasm-debug.cc 的断点 flooding 哨兵、调试代码 LRU 缓存上限
// 与大小打印锚。骨架在 v8-wasm-core/src/wasm_debug_consts.rs (wasm-debug.h.js)。

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/wasm/wasm-debug.cc");

// ---------- 1. flooding 断点哨兵: 偏移 0 表示泛洪 ----------
if (!src.includes("// 0 is an invalid offset used to indicate flooding.")) {
  throw new Error("flooding 哨兵注释缺失");
}
if (!src.includes("constexpr int kFloodingBreakpoints[] = {0};")) {
  throw new Error("kFloodingBreakpoints 缺失");
}

// ---------- 2. 调试代码 LRU 缓存上限 ----------
const cacheMax = src.match(/static constexpr size_t kMaxCachedDebuggingCode = (\d+);/);
if (!cacheMax || cacheMax[1] !== "3") throw new Error("调试代码缓存上限 != 3");
if (!src.includes("// This is a LRU cache (most recently used entries first).")) {
  throw new Error("LRU 语义注释缺失");
}

// ---------- 3. 缓存条目结构 (函数号/断点偏移/死断点/代码) ----------
if (!src.includes("struct CachedDebuggingCode {")) throw new Error("缓存条目结构缺失");
if (!src.includes("base::UniqueArray<const int> dead_breakpoints;")) {
  throw new Error("死断点字段缺失");
}

// ---------- 4. jitless 门控 (解释器不支持断点) ----------
if (!src.includes("// TODO(paolosev@microsoft.com) - Add support for breakpoints in Wasm")) {
  throw new Error("jitless 门控缺失");
}

// ---------- 5. flooding 要求 liftoff 代码 ----------
if (!src.includes("DCHECK(frame->wasm_code()->is_liftoff());")) {
  throw new Error("flooding liftoff 前提缺失");
}

// ---------- 6. 大小打印锚 ----------
if (!src.includes('PrintF("DebugInfo: %zu\\n", result);')) {
  throw new Error("DebugInfo 大小打印锚缺失");
}

// ---------- 7. 步进帧登记语义 ----------
if (!src.includes("stepping_frame = frame->id();")) {
  throw new Error("步进帧登记缺失");
}

console.log("wasm-debug: flooding哨兵=0 LRU上限=3");

// ---------- 生成 Rust ----------
const rust = `${banner(
  "src/wasm/wasm-debug.cc.js",
  "src/wasm/wasm-debug.cc",
  "断点 flooding 与调试代码缓存规则 (骨架在 wasm_debug_consts.rs)",
)}
//! 断点/步进的调试代码管理规则。

/// flooding 哨兵: 偏移 0 是无效偏移, 用于表示"泛洪"模式
/// v8: src/wasm/wasm-debug.cc FloodWithBreakpoints
pub const K_FLOODING_BREAKPOINTS: &[i32] = &[0];

/// 调试代码 LRU 缓存上限 (最近使用在前; 步进每暂停清一次、下一步重装)
/// v8: src/wasm/wasm-debug.cc kMaxCachedDebuggingCode
pub const K_MAX_CACHED_DEBUGGING_CODE: usize = ${cacheMax[1]};

/// 缓存条目字段 (LRU 淘汰)
/// v8: src/wasm/wasm-debug.cc CachedDebuggingCode
#[rustfmt::skip]
pub static CACHED_DEBUGGING_CODE_FIELDS: &[&str] = &[
    "func_index",
    "breakpoint_offsets",
    "dead_breakpoints",
    "code",
];

/// flooding 仅支持 liftoff 代码; jitless (解释器) 无断点
/// v8: src/wasm/wasm-debug.cc FloodWithBreakpoints
pub const FLOODING_REQUIRES_LIFTOFF: bool = true;
pub const JITLESS_NO_BREAKPOINTS: bool = true;

/// 步进以帧 id 登记 (stepping_frame), 断点重编译后更新返回地址
/// v8: src/wasm/wasm-debug.cc FloodWithBreakpoints / ReturnLocation
pub const STEPPING_FRAME_SEMANTICS: &str = "帧 id 登记 + 返回地址更新";

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(K_FLOODING_BREAKPOINTS, &[0]);
    assert_eq!(K_MAX_CACHED_DEBUGGING_CODE, 3);
    assert_eq!(CACHED_DEBUGGING_CODE_FIELDS.len(), 4);
};
`;

const p = writeSrc("v8-wasm-objects", "wasm_debug_cc.rs", rust);
console.log(`written ${p}`);
