#!/usr/bin/env bun
// tool/src/wasm/wasm-deopt-data.h.js
// 机械提取 v8/src/wasm/wasm-deopt-data.{h,cc} 的 deopt 数据序列化布局

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/wasm-deopt-data.h");
const ccSrc = readV8("src/wasm/wasm-deopt-data.cc");

// 1. h: WasmDeoptData 头字段
const headerFields = [
  "uint32_t entry_count = 0;",
  "uint32_t translation_array_size = 0;",
  "uint32_t num_deopt_literals = 0;",
  "int deopt_exit_start_offset = 0;",
  "int eager_deopt_count = 0;",
];
for (const f of headerFields) {
  if (!hSrc.includes(f)) throw new Error(`WasmDeoptData 字段缺失: ${f}`);
}

// 2. h: WasmDeoptEntry 字段
for (const f of [
  "BytecodeOffset bytecode_offset = BytecodeOffset::None();",
  "int translation_index = -1;",
]) {
  if (!hSrc.includes(f)) throw new Error(`WasmDeoptEntry 字段缺失: ${f}`);
}

// 3. h: CET 条件字段
if (!hSrc.includes("#ifdef V8_ENABLE_CET_SHADOW_STACK")) throw new Error("未找到 CET 门");
if (!hSrc.includes("uint32_t adapt_shadow_stack_pc_offset = 0;")) {
  throw new Error("未找到 adapt_shadow_stack_pc_offset");
}

// 4. h: LiftoffFrameDescriptionForDeopt 字段
for (const f of [
  "uint32_t wire_bytes_offset = 0;",
  "uint32_t pc_offset = 0;",
  "std::vector<LiftoffVarState> var_state = {};",
  "Register trusted_instance = no_reg;",
  "int total_frame_size = 0;",
]) {
  if (!hSrc.includes(f)) throw new Error(`LiftoffFrameDescription 字段缺失: ${f}`);
}

// 5. cc: 序列化布局 = header + translations + entries + literals
for (const a of [
  "size_t byte_size = sizeof(data) + translation_array_byte_size +",
  "deopt_entries_byte_size + deopt_literals_byte_size;",
  "std::memcpy(remaining_buffer.begin(), &data, sizeof(data));",
  "buffer += DeoptimizationLiteral::Read(buffer, deopt_literals.data() + i);",
]) {
  if (!ccSrc.includes(a)) throw new Error(`序列化锚缺失: ${a}`);
}

// 6. cc: 读侧三段游标
for (const a of [
  "deopt_data_ + sizeof(base_data_) + base_data_.translation_array_size +",
  "sizeof(WasmDeoptEntry) * base_data_.entry_count",
  "CHECK_NE(literal.kind(), DeoptimizationLiteralKind::kObject);",
]) {
  if (!ccSrc.includes(a)) throw new Error(`读取锚缺失: ${a}`);
}

// 7. cc: 一致性检查
if (!ccSrc.includes("CHECK_EQ(deopt_entries.size(), static_cast<size_t>(eager_deopt_count));")) {
  throw new Error("未找到 entry 数一致性检查");
}

console.log(`wasm-deopt-data: 头字段=${headerFields.length} 序列化段=4`);

// 8. 生成 Rust
let rust = banner(
  "src/wasm/wasm-deopt-data.h.js",
  "src/wasm/wasm-deopt-data.{h,cc}",
  "wasm deopt 数据序列化布局与 Liftoff 帧描述",
);

rust += `
//! 优化 wasm 函数的 deopt 点数据: 头 + 翻译数组 + 条目 + 字面量 四段连续布局。
/// v8: src/wasm/wasm-deopt-data.h WasmDeoptData

/// deopt 数据头 (trivially copyable, 直接 memcpy)
/// v8: src/wasm/wasm-deopt-data.h struct WasmDeoptData
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct WasmDeoptData {
    /// v8: uint32_t entry_count — deopt 点计数
    pub entry_count: u32,
    /// v8: uint32_t translation_array_size
    pub translation_array_size: u32,
    /// v8: uint32_t num_deopt_literals
    pub num_deopt_literals: u32,
    /// v8: int deopt_exit_start_offset — 首个 deopt builtin 调用的代码偏移
    pub deopt_exit_start_offset: i32,
    /// v8: int eager_deopt_count
    pub eager_deopt_count: i32,
}

/// 单个 deopt 点
/// v8: src/wasm/wasm-deopt-data.h struct WasmDeoptEntry
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WasmDeoptEntry {
    /// v8: BytecodeOffset bytecode_offset (None 初始)
    pub bytecode_offset: u32,
    /// v8: int translation_index = -1
    pub translation_index: i32,
}

/// v8: translation_index 哨兵初值
pub const NO_TRANSLATION_INDEX: i32 = -1;

/// 字节流四段布局的偏移计算
/// v8: src/wasm/wasm-deopt-data.cc Serialize / WasmDeoptView
pub struct DeoptDataLayout {
    pub header_size: usize,
    pub translation_array_size: usize,
    pub deopt_entry_size: usize,
    pub deopt_entry_count: usize,
}

impl DeoptDataLayout {
    /// literals 段起始偏移: sizeof(header) + translations + entries
    /// v8: src/wasm/wasm-deopt-data.cc BuildDeoptimizationLiteralArray
    pub const fn literals_offset(&self) -> usize {
        self.header_size + self.translation_array_size + self.deopt_entry_size * self.deopt_entry_count
    }
    /// 序列化总长 (字面量字节宽由调用方追加)
    /// v8: src/wasm/wasm-deopt-data.cc Serialize
    pub const fn total_with_literals(&self, literal_bytes: usize) -> usize {
        self.literals_offset() + literal_bytes
    }
}

/// Liftoff 帧还原描述 (CET 开启时多一个 shadow stack 适配偏移)
/// v8: src/wasm/wasm-deopt-data.h struct LiftoffFrameDescriptionForDeopt
pub struct LiftoffFrameDescriptionForDeopt {
    /// v8: uint32_t wire_bytes_offset
    pub wire_bytes_offset: u32,
    /// v8: uint32_t pc_offset
    pub pc_offset: u32,
    /// v8: uint32_t adapt_shadow_stack_pc_offset (V8_ENABLE_CET_SHADOW_STACK)
    pub adapt_shadow_stack_pc_offset: Option<u32>,
    /// v8: std::vector<LiftoffVarState> var_state
    pub var_state: Vec<LiftoffVarState>,
    /// v8: Register trusted_instance (寄存器缓存的实例, 需同步更新)
    pub trusted_instance: Option<Register>,
    /// v8: int total_frame_size
    pub total_frame_size: i32,
}

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(NO_TRANSLATION_INDEX, -1);
    let l = DeoptDataLayout { header_size: 20, translation_array_size: 8, deopt_entry_size: 8, deopt_entry_count: 3 };
    assert_eq!(l.literals_offset(), 20 + 8 + 24);
};
`;

const p = writeSrc("v8-wasm-core", "wasm_deopt_data.rs.skel", rust);
console.log(`written ${p}`);
