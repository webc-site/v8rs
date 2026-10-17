#!/usr/bin/env bun
// tool/src/wasm/interpreter/wasm-interpreter-simd.cc.js
// 机械提取 v8/src/wasm/interpreter/wasm-interpreter-simd.cc 的 SIMD 保留码表与立即数区间

import { readV8, writeSrc, banner } from "../../../lib.js";

const src = readV8("src/wasm/interpreter/wasm-interpreter-simd.cc");

// 1. 真解析 256 项保留表 (按位置序; 声明 [256] 但显式 255 项, 尾项隐式 false)
const tableMatch = src.match(/kIsReservedSimdOpcode\[256\] = \{([\s\S]*?)\n  \};/);
if (!tableMatch) throw new Error("未找到 kIsReservedSimdOpcode");
const items = [...tableMatch[1].matchAll(/\b(true|false)\b/g)].map((m) => m[1] === "true");
if (items.length !== 255) throw new Error(`显式表项 ${items.length} != 255`);
items.push(false); // 尾项隐式 false
if (items.length !== 256) throw new Error("总表项 != 256");
const reservedCount = items.filter(Boolean).length;

// 2. 表按位与取模索引
if (!src.includes("kIsReservedSimdOpcode[opcode & 0xff]")) {
  throw new Error("未找到 opcode & 0xff 索引");
}

// 3. relaxed SIMD 上界 (0xfd100..0xfd113, 20 个)
const relaxedMatch = src.match(/return opcode <= 0xfd(\d+);/);
if (!relaxedMatch) throw new Error("未找到 relaxed 上界");
const relaxedMax = parseInt(`0xfd${relaxedMatch[1]}`, 16);
if (!src.includes("opcode >= 0xfd200 ||")) throw new Error("未找到 0xfd200 拒绝门");

// 4. memarg 偏移 48 位门槛
if (!src.includes("if (mem_imm.offset >= ((uint64_t)1 << 48)) {")) {
  throw new Error("未找到 48 位偏移门槛");
}

// 5. lane 立即数区间 (extract/replace lane)
if (!src.includes("(opcode >= kExprI8x16ExtractLaneS) &&")) {
  throw new Error("未找到 lane 区间起点");
}
if (!src.includes("(opcode <= kExprF64x2ReplaceLane))")) throw new Error("未找到 lane 区间终点");

// 6. load/store lane 区间
if (!src.includes("(opcode >= kExprS128Load8Lane) &&")) throw new Error("未找到 lane load 起点");
if (!src.includes("(opcode <= kExprS128Store64Lane))")) throw new Error("未找到 lane store 终点");

// 7. v128.const 16 字节立即数
if (!src.includes("*len += 16;")) throw new Error("未找到 v128.const 16 字节");

// 8. RMW 判定复用
if (!src.includes("bool is_rmw = WasmOpcodes::IsAtomicRmwOpcode(opcode);")) {
  throw new Error("未找到 IsAtomicRmwOpcode");
}

// 9. NoValidation 常量
if (!src.includes("constexpr Decoder::NoValidationTag kNoValidate;")) {
  throw new Error("未找到 kNoValidate");
}

// 10. 未知码 FATAL
if (!src.includes('FATAL("Unknown or unimplemented opcode #%d:%s"')) {
  throw new Error("未找到 FATAL");
}

console.log(
  `wasm-interpreter-simd: 表项=${items.length} 保留=${reservedCount} relaxedMax=0x${relaxedMax.toString(16)}`,
);

// 11. 生成 Rust (表分行 16 列)
const rows = [];
for (let i = 0; i < 256; i += 16) {
  rows.push("    " + items.slice(i, i + 16).map((v) => (v ? "true " : "false")).join(", ") + ", // 0x" + i.toString(16));
}
let rust = banner(
  "src/wasm/interpreter/wasm-interpreter-simd.cc.js",
  "src/wasm/interpreter/wasm-interpreter-simd.cc",
  "SIMD 解码: 保留码表与立即数消费区间",
);

rust += `
//! SIMD 操作码到解释器字节码的译码辅助。
/// v8: src/wasm/interpreter/wasm-interpreter-simd.cc WasmBytecodeGenerator::DecodeSimdOp

/// 保留 (未实现) SIMD 操作码表, 按位置序索引 [opcode & 0xff]
/// 注意: 原版声明 [256] 但显式初始化 255 项 (0xee 注释缺失, 后续注释错位一格),
/// 尾项隐式 false — 本表按 C++ 聚合初始化语义忠实展开。
/// v8: src/wasm/interpreter/wasm-interpreter-simd.cc kIsReservedSimdOpcode
pub static IS_RESERVED_SIMD_OPCODE: [bool; 256] = [
${rows.join("\n")}
];

/// relaxed SIMD 接受区间: [0xfd100, 0xfd113] 共 20 个
/// v8: DecodeSimdOp "return opcode <= 0xfd113"
pub const RELAXED_SIMD_MIN: u32 = 0xfd100;
pub const RELAXED_SIMD_MAX: u32 = ${`0x${relaxedMax.toString(16)}`};
pub const RELAXED_SIMD_COUNT: u32 = RELAXED_SIMD_MAX - RELAXED_SIMD_MIN + 1;

/// 0xfd200 及以上直接拒绝
/// v8: "opcode >= 0xfd200 ||"
pub const SIMD_PREFIX_REJECT_MIN: u32 = 0xfd200;

/// memarg 偏移立即数 48 位上限 (超出即译码失败)
/// v8: "mem_imm.offset >= ((uint64_t)1 << 48)"
pub const MEMARG_OFFSET_MAX: u64 = 1 << 48;

/// v128.const 立即数宽 (16 字节入 simd_immediates_)
/// v8: "*len += 16"
pub const V128_CONST_BYTES: usize = 16;

/// lane 立即数宽 (extract/replace lane 与 load/store lane)
/// v8: "*len += 1" (SimdLaneImmediate)
pub const LANE_IMM_BYTES: usize = 1;

/// v8: 译码不做验证 (模块已验证)
pub const NO_VALIDATE: bool = true;

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(IS_RESERVED_SIMD_OPCODE.len(), 256);
    assert!(!IS_RESERVED_SIMD_OPCODE[0x00]); // v128.load 已实现
    assert!(IS_RESERVED_SIMD_OPCODE[0x9a]); // (reserved)
    assert_eq!(RELAXED_SIMD_COUNT, 20);
    assert_eq!(MEMARG_OFFSET_MAX, 1 << 48);
};
`;

const p = writeSrc("v8-wasm-interp", "wasm_interpreter_simd.rs.skel", rust);
console.log(`written ${p}`);
