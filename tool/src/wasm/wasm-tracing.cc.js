#!/usr/bin/env bun
// tool/src/wasm/wasm-tracing.cc.js
// 机械提取 v8/src/wasm/wasm-tracing.cc 的追踪打印格式串与表示->标签映射。
// 记录布局常量在 v8-wasm-core/src/wasm_tracing.rs.skel (wasm-tracing.h.js)。

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/wasm/wasm-tracing.cc");

// ---------- 1. 内存追踪头格式 ----------
const memFmt = src.match(/"%(-8s func %5d:0x%-4x mem %d %-12s %s %016x val: )"/);
if (!memFmt) throw new Error("内存追踪头格式缺失");
if (!src.includes('trace_entry.is_store ? "to  " : "from"')) {
  throw new Error("存/取方向标签缺失");
}
// 方向标签是定宽 4 字符 ("to  " / "from")
if (!src.includes('"to  "')) throw new Error("to 标签需 4 字符定宽");

// ---------- 2. 表示 -> 标签映射 (7 项: i8/i16/i32/i64/f32/f64/s128) ----------
const reps = [...src.matchAll(/PrintMemoryTraceValue<(uint8_t|uint16_t|uint32_t|uint64_t)>\(trace_entry\.value_bytes, "(\w+)", outs\)/g)].map((m) => [m[1], m[2]]);
const floatReps = [...src.matchAll(/PrintMemoryTraceValue<(float|double), (uint32_t|uint64_t)>\(trace_entry\.value_bytes, "(\w+)",\s*outs\)/g)].map((m) => [m[1], m[2], m[3]]);
if (reps.length !== 4) throw new Error(`整型表示 ${reps.length} != 4`);
if (reps.map((r) => r[1]).join(",") !== "i8,i16,i32,i64") throw new Error("整型标签序异常");
if (floatReps.length !== 2) throw new Error(`浮点表示 ${floatReps.length} != 2`);
if (floatReps.map((r) => r[2]).join(",") !== "f32,f64") throw new Error("浮点标签序异常");
// SIMD
if (!/MachineRepresentation::kSimd128/.test(src)) throw new Error("s128 表示缺失");

// ---------- 3. 数值打印格式 ----------
if (!src.includes('absl::StreamFormat("%4s:%u / 0x%x", type_str, value, hex_value)')) {
  throw new Error("uint8 打印格式缺失");
}
if (!src.includes('absl::StreamFormat("%4s:%v / 0x%x", type_str, value, hex_value)')) {
  throw new Error("通用打印格式缺失");
}
// SIMD: 4 个 u32 十进制 + 4 个 %08x 十六进制
if (!src.includes('"s128:%d %d %d %d / %08x %08x %08x %08x"')) {
  throw new Error("s128 打印格式缺失");
}

// ---------- 4. 全局追踪格式 ----------
const globFmt = src.match(/"%(-8s func %5d:0x%-4x %s %d val: %s:)"/);
if (!globFmt) throw new Error("全局追踪头格式缺失");
if (!src.includes('type.is_numeric()')) throw new Error("数值/引用分派缺失");
if (!src.includes('absl::StreamFormat("0x%x", addr)')) throw new Error("引用值格式缺失");

// ---------- 5. 前缀操作码解析 (prefix 指令需读后续字节) ----------
if (!src.includes("WasmOpcodes::IsPrefixOpcode(opcode)")) {
  throw new Error("prefix 操作码解析缺失");
}

console.log(`wasm-tracing: 整型表示=${reps.length} 浮点表示=${floatReps.length}`);

// ---------- 生成 Rust ----------
const rust = `${banner(
  "src/wasm/wasm-tracing.cc.js",
  "src/wasm/wasm-tracing.cc",
  "内存/全局追踪打印格式 (记录布局在 wasm_tracing.rs.skel)",
)}
//! --trace-wasm 输出格式。

/// 内存追踪头格式 (tier 左对齐 8, 函数号 5, 帧位 0x 左对齐 4, 内存号, 操作码名左对齐 12, 方向, 偏移 016x)
/// v8: src/wasm/wasm-tracing.cc PrintMemoryTraceString
pub const MEMORY_TRACE_HEADER_FORMAT: &str = "%-8s func %5d:0x%-4x mem %d %-12s %s %016x val: ";

/// 存/取方向标签 (4 字符定宽)
/// v8: src/wasm/wasm-tracing.cc PrintMemoryTraceString
pub const DIRECTION_STORE: &str = "to  ";
pub const DIRECTION_LOAD: &str = "from";

/// 数值打印格式: 标签右对齐 4 + 十进制 / 十六进制
/// v8: src/wasm/wasm-tracing.cc PrintMemoryTraceValue
pub const VALUE_FORMAT_U8: &str = "%4s:%u / 0x%x";
pub const VALUE_FORMAT_GENERIC: &str = "%4s:%v / 0x%x";

/// SIMD128 打印格式: 4 个 u32 十进制 + 4 个 %08x
/// v8: src/wasm/wasm-tracing.cc PrintMemoryTraceString (kSimd128)
pub const VALUE_FORMAT_S128: &str = "s128:%d %d %d %d / %08x %08x %08x %08x";

/// MachineRepresentation -> 打印标签 (7 项)
/// v8: src/wasm/wasm-tracing.cc PrintMemoryTraceString
#[rustfmt::skip]
pub static REP_TAGS: &[(&str, &str)] = &[
    ("kWord8", "i8"),
    ("kWord16", "i16"),
    ("kWord32", "i32"),
    ("kWord64", "i64"),
    ("kFloat32", "f32"),
    ("kFloat64", "f64"),
    ("kSimd128", "s128"),
];

/// 浮点位表示: 打印用浮点解释, 十六进制位用等宽整数解释
/// v8: src/wasm/wasm-tracing.cc PrintMemoryTraceValue<float, uint32_t> 等
pub const FLOAT_BIT_REPS: &[(&str, &str)] = &[("f32", "uint32_t"), ("f64", "uint64_t")];

/// 全局追踪头格式 (无内存号与方向, 含全局号与类型名)
/// v8: src/wasm/wasm-tracing.cc PrintGlobalTraceString
pub const GLOBAL_TRACE_HEADER_FORMAT: &str = "%-8s func %5d:0x%-4x %s %d val: %s:";

/// 全局值分派: 数值类型走 WasmValue.to_string, 引用类型打 "0x%x"
/// v8: src/wasm/wasm-tracing.cc PrintGlobalTraceString
pub const GLOBAL_REF_VALUE_FORMAT: &str = "0x%x";

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(REP_TAGS.len(), 7);
    assert_eq!(DIRECTION_STORE.len(), 4);
    assert_eq!(DIRECTION_LOAD.len(), 4);
    assert_eq!(FLOAT_BIT_REPS.len(), 2);
};
`;

const p = writeSrc("v8-wasm-objects", "wasm_tracing_cc.rs", rust);
console.log(`written ${p}`);
