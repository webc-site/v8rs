#!/usr/bin/env bun
// tool/src/wasm/wasm-tracing.h.js
// 机械提取 v8/src/wasm/wasm-tracing.{h,cc} 的追踪记录布局与打印格式

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/wasm-tracing.h");
const ccSrc = readV8("src/wasm/wasm-tracing.cc");

// 1. h: 生成代码侧的低阶结构
for (const anchor of [
  "struct MemoryTracingInfo {",
  "uintptr_t offset;",
  "uint32_t mem_index;",
  "uint8_t is_store;  // 0 or 1",
  "uint8_t mem_rep;",
  "struct GlobalTracingInfo {",
  "uint32_t global_index;",
]) {
  if (!hSrc.includes(anchor)) throw new Error(`h 结构锚缺失: ${anchor}`);
}

// 2. h: value_bytes 宽度
const vb = (hSrc.match(/uint8_t value_bytes\[16\];/g) || []).length;
if (vb !== 2) throw new Error(`value_bytes[16] 出现 ${vb} 次 != 2`);

// 3. h: GlobalTraceEntry / MemoryTraceEntry 字段
for (const f of [
  "int function_index;",
  "int frame_position;",
  "wasm::ExecutionTier tier;",
  "wasm::ValueKind kind;",
  "MachineRepresentation representation;",
]) {
  if (!hSrc.includes(f)) throw new Error(`Entry 字段缺失: ${f}`);
}

// 4. h: 引用值不比较语义
if (!hSrc.includes("For references, value_bytes holds the address, which doesn't make sense")) {
  throw new Error("未找到引用不比较注释");
}

// 5. cc: memory 打印格式串
const memFmt = ccSrc.match(/"%-8s func %5d:0x%-4x mem %d %-12s %s %016x val: "/);
if (!memFmt) throw new Error("未找到 memory 打印格式串");
const globalFmt = ccSrc.match(/"%-8s func %5d:0x%-4x %s %d val: %s:"/);
if (!globalFmt) throw new Error("未找到 global 打印格式串");

// 6. cc: 表示 -> type_str 映射
const repMap = [
  ["kWord8", "i8", "uint8_t"],
  ["kWord16", "i16", "uint16_t"],
  ["kWord32", "i32", "uint32_t"],
  ["kWord64", "i64", "uint64_t"],
  ["kFloat32", "f32", "float, uint32_t"],
  ["kFloat64", "f64", "double, uint64_t"],
  ["kSimd128", null, null],
];
for (const [rep, tag] of repMap) {
  if (!ccSrc.includes(`MachineRepresentation::${rep}`)) {
    throw new Error(`表示分支缺失: ${rep}`);
  }
  if (tag && !ccSrc.includes(`"${tag}"`)) throw new Error(`type_str 缺失: ${tag}`);
}

// 7. cc: s128 四道 u32 打印
if (!ccSrc.includes('"s128:%d %d %d %d / %08x %08x %08x %08x"')) {
  throw new Error("未找到 s128 格式串");
}

// 8. cc: 方向字符串
if (!ccSrc.includes('trace_entry.is_store ? "to  " : "from"')) {
  throw new Error("未找到 to/from 方向串");
}

// 9. cc: u8 特殊格式 (%u 而非 %v)
if (!ccSrc.includes('"%4s:%u / 0x%x"')) throw new Error("未找到 u8 打印格式");

console.log(`wasm-tracing: 结构=4 valueBytes16=${vb} 表示分支=${repMap.length}`);

// 10. 生成 Rust
let rust = banner(
  "src/wasm/wasm-tracing.h.js",
  "src/wasm/wasm-tracing.{h,cc}",
  "内存/全局变量追踪记录布局与打印格式",
);

rust += `
//! --trace-wasm 生成的内存访问/全局变量读写记录。
/// v8: src/wasm/wasm-tracing.h MemoryTraceEntry

/// 生成代码中构造的低阶信息 (布局必须与机器码写出的一致)
/// v8: src/wasm/wasm-tracing.h MemoryTracingInfo
#[repr(C)]
pub struct MemoryTracingInfo {
    /// v8: uintptr_t offset
    pub offset: usize,
    /// v8: uint32_t mem_index
    pub mem_index: u32,
    /// v8: uint8_t is_store (0 or 1)
    pub is_store: u8,
    /// v8: uint8_t mem_rep (MachineRepresentation 的 u8 底层)
    pub mem_rep: u8,
}

/// v8: src/wasm/wasm-tracing.h GlobalTracingInfo
#[repr(C)]
pub struct GlobalTracingInfo {
    /// v8: uint32_t global_index
    pub global_index: u32,
    /// v8: uint8_t is_store (0 or 1)
    pub is_store: u8,
}

/// 值字节缓冲宽度 (所有 kind 统一 16 字节)
/// v8: src/wasm/wasm-tracing.h value_bytes[16]
pub const VALUE_BYTES_LEN: usize = 16;

/// MachineRepresentation -> 打印标签
/// v8: src/wasm/wasm-tracing.cc PrintMemoryTraceString
pub fn rep_tag(rep: MachineRepresentation) -> Option<&'static str> {
    Some(match rep {
        MachineRepresentation::Word8 => "i8",
        MachineRepresentation::Word16 => "i16",
        MachineRepresentation::Word32 => "i32",
        MachineRepresentation::Word64 => "i64",
        MachineRepresentation::Float32 => "f32",
        MachineRepresentation::Float64 => "f64",
        MachineRepresentation::Simd128 => "s128",
        _ => return None,
    })
}

/// 内存访问行格式 (tier func_idx:frame_pos mem_idx opcode dir offset)
/// v8: src/wasm/wasm-tracing.cc PrintMemoryTraceString
pub const MEMORY_TRACE_FORMAT: &str = "%-8s func %5d:0x%-4x mem %d %-12s %s %016x val: ";

/// 全局变量行格式
/// v8: src/wasm/wasm-tracing.cc PrintGlobalTraceString
pub const GLOBAL_TRACE_FORMAT: &str = "%-8s func %5d:0x%-4x %s %d val: %s:";

/// store/load 方向串
/// v8: src/wasm/wasm-tracing.cc "to  " / "from"
pub const DIRECTION_STORE: &str = "to  ";
pub const DIRECTION_LOAD: &str = "from";

/// 值打印: 有符号十进制 / 十六进制位表示双写; s128 四道 u32
/// v8: src/wasm/wasm-tracing.cc PrintMemoryTraceValue
pub const VALUE_FORMAT: &str = "%4s:%v / 0x%x";
pub const VALUE_FORMAT_U8: &str = "%4s:%u / 0x%x";
pub const SIMD128_FORMAT: &str = "s128:%d %d %d %d / %08x %08x %08x %08x";

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(VALUE_BYTES_LEN, 16);
    assert_eq!(DIRECTION_STORE.len(), 4);
};
`;

const p = writeSrc("v8-wasm-core", "wasm_tracing.rs.skel", rust);
console.log(`written ${p}`);
