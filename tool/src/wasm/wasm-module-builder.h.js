#!/usr/bin/env bun
// tool/src/wasm/wasm-module-builder.h.js
// 机械提取 v8/src/wasm/wasm-module-builder.{h,cc} 的 ZoneBuffer 协议与构建器面

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/wasm-module-builder.h");
const ccSrc = readV8("src/wasm/wasm-module-builder.cc");

// 1. h: ZoneBuffer 常量与扩容
if (!hSrc.includes("static constexpr size_t kInitialSize = 1024;")) {
  throw new Error("未找到 kInitialSize=1024");
}
if (!hSrc.includes("size_t new_size = size + (end_ - buffer_) * 2;")) {
  throw new Error("未找到扩容公式");
}

// 2. h: 写入原语清单
const writeFns = [...hSrc.matchAll(/void (write_\w+)\(/g)].map((m) => m[1]);
const uniqWrites = [...new Set(writeFns)];
if (uniqWrites.length < 10) throw new Error(`写入原语 ${uniqWrites.length} < 10`);
for (const w of ["write_u8", "write_u32", "write_u64", "write_u32v", "write_i32v", "write_u64v", "write_i64v", "write_f32", "write_f64", "write_string"]) {
  if (!uniqWrites.includes(w)) throw new Error(`写入原语缺失: ${w}`);
}

// 3. h: 预留/修补协议
if (!hSrc.includes("size_t reserve_u32v() {")) throw new Error("未找到 reserve_u32v");
if (!hSrc.includes("for (size_t pos = 0; pos != kPaddedVarInt32Size; ++pos)")) {
  throw new Error("未找到 padded 修补循环");
}

// 4. h: WasmElemSegment 枚举
if (!hSrc.includes("kStatusActive,      // copied automatically during instantiation.")) {
  throw new Error("未找到 kStatusActive");
}
const entryKinds = ["kGlobalGetEntry", "kRefFuncEntry", "kRefNullEntry"];
for (const k of entryKinds) {
  if (!hSrc.includes(k)) throw new Error(`Entry kind 缺失: ${k}`);
}
// 偏移仅四种合法 init kind
const validOffsets = ["WasmInitExpr::kI32Const", "WasmInitExpr::kI64Const", "WasmInitExpr::kGlobalGet", "WasmInitExpr::kRefNullConst"];
for (const v of validOffsets) {
  if (!hSrc.includes(v)) throw new Error(`合法偏移缺: ${v}`);
}

// 5. h: 递归组协议
if (!hSrc.includes("void StartRecursiveTypeGroup() {")) throw new Error("未找到 StartRecursiveTypeGroup");
if (!hSrc.includes("void EndRecursiveTypeGroup() {")) throw new Error("未找到 EndRecursiveTypeGroup");
if (!hSrc.includes("int current_recursive_group_start_")) throw new Error("未找到组起点哨兵");

// 6. h: Add* 构建面计数
const addFns = [...new Set([...hSrc.matchAll(/(uint32_t|void|ModuleTypeIndex) (Add\w+)\(/g)].map((m) => m[2]))];
if (addFns.length < 15) throw new Error(`Add 面 ${addFns.length} < 15`);

// 7. cc: 写出协议 (magic/version 前导 + 段序)
if (!ccSrc.includes("kWasmMagic") && !ccSrc.includes("kWasmVersion")) {
  if (!ccSrc.includes("buffer->write_u32(kWasmMagic)")) throw new Error("未找到 magic 前导");
}
const sectionCalls = [...new Set([...ccSrc.matchAll(/k(\w+)SectionCode/g)].map((m) => `k${m[1]}SectionCode`))];
if (sectionCalls.length < 10) throw new Error(`段码引用 ${sectionCalls.length} < 10`);

// 8. cc: 直接调用索引回填
if (!ccSrc.includes("direct_calls_") && !hSrc.includes("ZoneVector<DirectCallIndex> direct_calls_;")) {
  throw new Error("未找到直接调用回填");
}

console.log(
  `wasm-module-builder: 写入原语=${uniqWrites.length} Add面=${addFns.length} 段码=${sectionCalls.length}`,
);

// 9. 生成 Rust
let rust = banner(
  "src/wasm/wasm-module-builder.h.js",
  "src/wasm/wasm-module-builder.{h,cc}",
  "wasm 模块字节码构建器 (ZoneBuffer 协议与段写出)",
);

rust += `
//! fuzzer 用的模块构建器: ZoneBuffer 顺序写出 + 预留修补。
/// v8: src/wasm/wasm-module-builder.h WasmModuleBuilder

/// ZoneBuffer 参数
/// v8: src/wasm/wasm-module-builder.h ZoneBuffer
pub mod zone_buffer {
    /// v8: kInitialSize
    pub const INITIAL_SIZE: usize = 1024;
    /// v8: EnsureSpace: new_size = need + old_capacity*2
    pub fn grown_size(need: usize, old_capacity: usize) -> usize {
        need + old_capacity * 2
    }
}

/// 写入原语 (宽度约定)
/// v8: src/wasm/wasm-module-builder.h ZoneBuffer::write_*
pub const WRITE_PRIMITIVES: &[&str] = &[
${uniqWrites.map((w) => `    "${w}",`).join("\n")}
];

/// reserve_u32v 预留 kMaxVarInt32Size 后 patch_u32v 以定长 padded LEB 回填
/// v8: src/wasm/wasm-module-builder.h reserve_u32v / patch_u32v
pub const PADDED_PATCH_WIDTH: usize = 5;

/// 元素段状态
/// v8: src/wasm/wasm-module-builder.h WasmElemSegment::Status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElemSegmentStatus {
    /// v8: kStatusActive — 实例化时自动拷贝
    Active,
    /// v8: kStatusPassive — 实例化后显式拷贝
    Passive,
    /// v8: kStatusDeclarative — 仅声明, 永不拷贝
    Declarative,
}

/// 元素段条目来源
/// v8: src/wasm/wasm-module-builder.h WasmElemSegment::Entry::Kind
pub const ENTRY_KINDS: &[&str] = &["kGlobalGetEntry", "kRefFuncEntry", "kRefNullEntry"];

/// 元素段偏移仅允许的四种初始化表达式
/// v8: src/wasm/wasm-module-builder.h IsValidOffsetKind
pub const VALID_OFFSET_KINDS: &[&str] = &[
    "kI32Const", "kI64Const", "kGlobalGet", "kRefNullConst",
];

/// 递归类型组: Start 记录起点, End 登记 (start, size) 对
/// v8: src/wasm/wasm-module-builder.h StartRecursiveTypeGroup / EndRecursiveTypeGroup
#[derive(Debug, Clone, Copy)]
pub struct RecGroup {
    /// v8: uint32_t start_index
    pub start_index: u32,
    /// v8: uint32_t size
    pub size: u32,
}

/// v8: 组未开启时的哨兵
pub const NO_RECURSIVE_GROUP: i32 = -1;

/// Add* 构建面
/// v8: src/wasm/wasm-module-builder.h WasmModuleBuilder
pub const ADD_API: &[&str] = &[
${addFns.map((a) => `    "${a}",`).join("\n")}
];

/// 写出时引用的 section 码 (WriteTo 段序)
/// v8: src/wasm/wasm-module-builder.cc WriteTo
pub const SECTION_CODES_REFERENCED: &[&str] = &[
${sectionCalls.map((s) => `    "${s}",`).join("\n")}
];

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(zone_buffer::INITIAL_SIZE, 1024);
    assert_eq!(zone_buffer::grown_size(1, 1024), 2049);
    assert_eq!(PADDED_PATCH_WIDTH, 5);
    assert_eq!(ENTRY_KINDS.len(), 3);
    assert_eq!(VALID_OFFSET_KINDS.len(), 4);
    assert_eq!(NO_RECURSIVE_GROUP, -1);
};
`;

const p = writeSrc("v8-wasm-core", "wasm_module_builder.rs.skel", rust);
console.log(`written ${p}`);
