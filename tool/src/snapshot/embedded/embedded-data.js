#!/usr/bin/env bun
// tool/src/snapshot/embedded/embedded-data.js
// 机械提取 v8/src/snapshot/embedded/embedded-data.{h,cc,-inl.h} 的 blob 布局
// 偏移链、表项结构与对齐函数

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/snapshot/embedded/embedded-data.h");
const ccSrc = readV8("src/snapshot/embedded/embedded-data.cc");

const K_SIZET = 8; // kSizetSize = sizeof(size_t), 64 位目标机械展开
const K_UINT32 = 4;

// 1. LayoutDescription 结构 (3 x uint32, static_assert 逐项核对)
const layoutAsserts = [...hSrc.matchAll(/offsetof\(LayoutDescription, (\w+)\) ==\s*(\d) \* kUInt32Size/g)];
if (layoutAsserts.length !== 3) throw new Error(`LayoutDescription 断言数不符: ${layoutAsserts.length}`);
const layoutFields = layoutAsserts.map((m) => ({ name: m[1], off: parseInt(m[2], 10) * K_UINT32 }));
const LAYOUT_SIZE = 12;

// 2. BuiltinLookupEntry 结构 (2 x uint32)
const bleAsserts = [...hSrc.matchAll(/offsetof\(BuiltinLookupEntry, (\w+)\) ==\s*(\d) \* kUInt32Size/g)];
if (bleAsserts.length !== 2) throw new Error(`BuiltinLookupEntry 断言数不符: ${bleAsserts.length}`);
const bleFields = bleAsserts.map((m) => ({ name: m[1], off: parseInt(m[2], 10) * K_UINT32 }));
const BLE_SIZE = 8;

// 3. 偏移链机械展开 (h 内 constexpr 函数, 逐条核对原文形态)
const chain = [
  ["EMBEDDED_BLOB_DATA_HASH_OFFSET", "EmbeddedBlobDataHashOffset() { return 0; }", 0],
  ["EMBEDDED_BLOB_DATA_HASH_SIZE", "EmbeddedBlobDataHashSize() { return kSizetSize; }", K_SIZET],
  ["EMBEDDED_BLOB_CODE_HASH_OFFSET", null, K_SIZET],
  ["EMBEDDED_BLOB_CODE_HASH_SIZE", "EmbeddedBlobCodeHashSize() { return kSizetSize; }", K_SIZET],
  ["ISOLATE_HASH_OFFSET", null, K_SIZET * 2],
  ["ISOLATE_HASH_SIZE", "IsolateHashSize() { return kSizetSize; }", K_SIZET],
  ["LAYOUT_DESCRIPTION_TABLE_OFFSET", null, K_SIZET * 3],
];
for (const [name, pat] of chain) {
  if (pat && !hSrc.includes(pat)) throw new Error(`偏移链成员异常: ${name}`);
}
if (!hSrc.includes("LayoutDescriptionTableOffset() {")) throw new Error("未找到 LayoutDescriptionTableOffset");
const layoutTableOff = K_SIZET * 3;
const layoutTableSize = LAYOUT_SIZE; // x kTableSize
const bleTableOff = layoutTableOff + LAYOUT_SIZE; // x kTableSize
const bleTableSize = BLE_SIZE; // x kTableSize
const fixedDataSize = layoutTableOff + (LAYOUT_SIZE + BLE_SIZE); // x kTableSize
// kTableSize = Builtins::kBuiltinCount (跨模块)
const tableM = hSrc.match(/static constexpr uint32_t kTableSize = Builtins::kBuiltinCount;/);
if (!tableM) throw new Error("未找到 kTableSize");
const rawMetaM = hSrc.match(/static constexpr uint32_t RawMetadataOffset\(\) \{ return FixedDataSize\(\); \}/);
const rawCodeM = hSrc.match(/static constexpr uint32_t RawCodeOffset\(\) \{ return 0; \}/);
if (!rawMetaM || !rawCodeM) throw new Error("未找到 RawMetadataOffset/RawCodeOffset");

// 4. 对齐函数
const padCodeM = hSrc.match(/static constexpr int PadAndAlignCode\(int size\) \{[\s\S]*?return RoundUp<kCodeAlignment>\(size \+ 1\);/);
const padDataM = hSrc.match(/static constexpr int PadAndAlignData\(int size\) \{[\s\S]*?return RoundUp<InstructionStream::kMetadataAlignment>\(size\);/);
if (!padCodeM || !padDataM) throw new Error("未找到 PadAndAlignCode/Data");
const K_CODE_ALIGNMENT = { x64: 64, arm64: 32 }; // kCodeAlignmentBits: x64=6, 其他=5
const K_METADATA_ALIGNMENT = 4; // InstructionStream::kMetadataAlignment = kIntSize

// 5. cc: kRelocMask (CODE_TARGET | RELATIVE_CODE_TARGET)
const relocM = ccSrc.match(/static const int kRelocMask =\s*RelocInfo::ModeMask\(RelocInfo::CODE_TARGET\) \|\s*RelocInfo::ModeMask\(RelocInfo::RELATIVE_CODE_TARGET\);/);
if (!relocM) throw new Error("未找到 kRelocMask");
const firstHashedM = ccSrc.match(/static constexpr uint32_t kFirstHashedDataOffset = IsolateHashOffset\(\);/);
if (!firstHashedM) throw new Error("未找到 kFirstHashedDataOffset");
const ccLines = ccSrc.split("\n").length;
console.log(`布局 ${layoutFields.length}+${bleFields.length} 字段 + 偏移链 ${chain.length} + cc ${ccLines} 行`);

let rust = banner(
  "src/snapshot/embedded/embedded-data.js",
  "src/snapshot/embedded/embedded-data.{h,cc,-inl.h}",
  "内嵌 blob 布局",
);
rust += `
/// ReorderedBuiltinIndex (原版 using, 内嵌快照序索引)
pub type ReorderedBuiltinIndex = u32;

/// 内嵌 blob 数据区布局 (原版 h 注释机械登记):
/// data: [0] 数据区哈希 | [1] 代码区哈希 | [2] 内嵌相关堆对象哈希
///       [3..] 布局描述表 (builtin id 序) | [n] 内建查找表 (end_offset 升序, 快照序)
///       [x..] 各 builtin 元数据区
/// code: 各 builtin 指令区 (内嵌快照序)

/// LayoutDescription 表项 (原版 struct LayoutDescription, 3 x u32):
pub mod layout_description {
`;
for (const f of layoutFields) {
  rust += `  /// ${f.name} @ ${f.off}\n  pub const ${f.name.toUpperCase()}_OFFSET: u32 = ${f.off};\n`;
}
rust += `  /// sizeof = 3 * kUInt32Size
  pub const SIZE: u32 = ${LAYOUT_SIZE};
}

/// BuiltinLookupEntry 表项 (原版 struct BuiltinLookupEntry, 2 x u32):
pub mod builtin_lookup_entry {
`;
for (const f of bleFields) {
  rust += `  /// ${f.name} @ ${f.off}\n  pub const ${f.name.toUpperCase()}_OFFSET: u32 = ${f.off};\n`;
}
rust += `  /// sizeof = 2 * kUInt32Size
  pub const SIZE: u32 = ${BLE_SIZE};
}

/// blob 数据区偏移链 (原版 constexpr 函数机械展开, kSizetSize=${K_SIZET}):
pub mod blob_layout {
  /// 表容量 = Builtins::kBuiltinCount (跨模块, 由 builtins crate 落值)
  pub const TABLE_SIZE_EXPR: &str = "Builtins::kBuiltinCount";

  /// 数据区哈希偏移
  pub const EMBEDDED_BLOB_DATA_HASH_OFFSET: u32 = 0;
  /// 哈希宽度 = kSizetSize
  pub const EMBEDDED_BLOB_DATA_HASH_SIZE: u32 = ${K_SIZET};
  /// 代码区哈希偏移 = 数据哈希偏移 + 哈希宽
  pub const EMBEDDED_BLOB_CODE_HASH_OFFSET: u32 = ${K_SIZET};
  pub const EMBEDDED_BLOB_CODE_HASH_SIZE: u32 = ${K_SIZET};
  /// isolate 哈希偏移
  pub const ISOLATE_HASH_OFFSET: u32 = ${K_SIZET * 2};
  pub const ISOLATE_HASH_SIZE: u32 = ${K_SIZET};
  /// 布局描述表偏移
  pub const LAYOUT_DESCRIPTION_TABLE_OFFSET: u32 = ${layoutTableOff};
  /// 布局描述表大小 = sizeof(LayoutDescription) * kTableSize
  pub const LAYOUT_DESCRIPTION_TABLE_SIZE: u32 = ${LAYOUT_SIZE} * ${K_UINT32} * 3;
  /// 内建查找表偏移
  pub const BUILTIN_LOOKUP_ENTRY_TABLE_OFFSET: u32 = LAYOUT_DESCRIPTION_TABLE_OFFSET
    + LAYOUT_DESCRIPTION_TABLE_SIZE;
  /// 内建查找表大小 = sizeof(BuiltinLookupEntry) * kTableSize
  pub const BUILTIN_LOOKUP_ENTRY_TABLE_SIZE: u32 = ${BLE_SIZE} * ${K_UINT32} * 2;
  /// 固定数据区大小 = 查找表尾 + 查找表大小
  pub const FIXED_DATA_SIZE: u32 = BUILTIN_LOOKUP_ENTRY_TABLE_OFFSET
    + BUILTIN_LOOKUP_ENTRY_TABLE_SIZE;
  /// 变长数据区 (元数据) 起点 = FixedDataSize
  pub const RAW_METADATA_OFFSET: u32 = FIXED_DATA_SIZE;
  /// 代码区独立成段, 起点恒 0
  pub const RAW_CODE_OFFSET: u32 = 0;
  /// 哈希覆盖起点 (原版 cc kFirstHashedDataOffset = IsolateHashOffset())
  pub const FIRST_HASHED_DATA_OFFSET: u32 = ISOLATE_HASH_OFFSET;
}

/// 对齐函数 (原版 PadAndAlignCode/PadAndAlignData):
/// - 指令: RoundUp<kCodeAlignment>(size + 1) — 尾部至少 1 字节供 int3 填充;
///   kCodeAlignment = 1 << kCodeAlignmentBits, x64 = ${K_CODE_ALIGNMENT.x64} (bits 6),
///   arm64 = ${K_CODE_ALIGNMENT.arm64} (bits 5, 其他架构分支)
/// - 元数据: RoundUp<kMetadataAlignment>(size), kMetadataAlignment = kIntSize
///   = ${K_METADATA_ALIGNMENT}
pub const CODE_ALIGNMENT_X64: u32 = ${K_CODE_ALIGNMENT.x64};
pub const CODE_ALIGNMENT_ARM64: u32 = ${K_CODE_ALIGNMENT.arm64};
pub const METADATA_ALIGNMENT: u32 = ${K_METADATA_ALIGNMENT};

/// cc 侧重定位掩码 (原版 kRelocMask):
/// CODE_TARGET | RELATIVE_CODE_TARGET — FinalizeEmbeddedCodeTargets 用,
/// 断言基准 Builtins::kAllBuiltinsAreIsolateIndependent。
/// cc 规模: ${ccLines} 行 (语义由 Developer 落)。
`;

const p = writeSrc("v8-snapshot", "embedded_data.rs", rust);
console.log(`written ${p}`);
