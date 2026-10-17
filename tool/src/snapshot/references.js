#!/usr/bin/env bun
// tool/src/snapshot/references.js
// 机械提取 v8/src/snapshot/references.h 的 SnapshotSpace 枚举、
// SerializerReference::SpecialValueType 与 TypeBits/ValueBits 位域布局

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/snapshot/references.h");

// 1. SnapshotSpace 枚举 (显式赋值 0..3)
const spaceM = src.match(/enum class SnapshotSpace : uint8_t \{([^}]+)\};/);
if (!spaceM) throw new Error("未找到 SnapshotSpace 枚举");
const rawSpaces = spaceM[1]
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);
if (rawSpaces.length !== 4) {
  throw new Error(`SnapshotSpace 变体数不符: 期望 4, 实得 ${rawSpaces.length}`);
}
const spaces = rawSpaces.map((t, idx) => {
  const nm = t.match(/^k(\w+)\s*=\s*(\d+)$/);
  if (!nm) throw new Error(`SnapshotSpace 变体不合期望格式: ${t}`);
  if (parseInt(nm[2], 10) !== idx) throw new Error(`SnapshotSpace 序号断裂: ${t}`);
  return { raw: `k${nm[1]}`, rustName: nm[1], repr: idx };
});

// 2. kNumberOfSnapshotSpaces
const numM = src.match(/static constexpr int kNumberOfSnapshotSpaces = (\d+);/);
if (!numM) throw new Error("未找到 kNumberOfSnapshotSpaces");
if (parseInt(numM[1], 10) !== spaces.length) {
  throw new Error(`kNumberOfSnapshotSpaces(${numM[1]}) 与枚举数 ${spaces.length} 不一致`);
}

// 3. SpecialValueType 内嵌枚举 (顺序赋值 0..3)
const svtM = src.match(/enum SpecialValueType \{([^}]+)\};/);
if (!svtM) throw new Error("未找到 SpecialValueType 枚举");
const rawSvt = svtM[1]
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);
if (rawSvt.length !== 4) {
  throw new Error(`SpecialValueType 变体数不符: 期望 4, 实得 ${rawSvt.length}`);
}
const svt = rawSvt.map((t, idx) => ({
  raw: t,
  rustName: t.replace(/^k/, "").charAt(0).toUpperCase() + t.replace(/^k/, "").slice(1),
  repr: idx,
}));

// 4. 位域布局: TypeBits(value,0,2) / ValueBits(value, 2, 30)
const typeBitsM = src.match(/using TypeBits = base::BitField<SpecialValueType, (\d+), (\d+)>;/);
const valueBitsM = src.match(/using ValueBits = TypeBits::Next<uint32_t, (\d+) - TypeBits::kSize>;/);
if (!typeBitsM || !valueBitsM) throw new Error("未找到 TypeBits/ValueBits 位域定义");
const typeOff = parseInt(typeBitsM[1], 10);
const typeW = parseInt(typeBitsM[2], 10);
// 原版 ValueBits 宽度写作 "32 - TypeBits::kSize", 机械展开:
const valueW = 32 - typeW;
if (typeOff !== 0 || typeW !== 2 || valueW !== 30) {
  throw new Error(`位域布局异常: off=${typeOff} w=${typeW} valueW=${valueW}`);
}
console.log(`SnapshotSpace ${spaces.length} + SpecialValueType ${svt.length} + 位域 (2+30)`);

let rust = banner("src/snapshot/references.js", "src/snapshot/references.h", "序列化引用与快照空间");
rust += `
/// SnapshotSpace: 快照分区空间。值必须连续且从 0 起 — 直接用作数组下标
/// (原版 enum class SnapshotSpace : uint8_t)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SnapshotSpace {
`;
for (const s of spaces) {
  rust += `  /// ${s.raw}\n  ${s.rustName} = ${s.repr},\n`;
}
rust += `}

/// 快照空间总数 (原版 kNumberOfSnapshotSpaces, 与 static_assert 4 == 一致)
pub const NUMBER_OF_SNAPSHOT_SPACES: usize = ${numM[1]};

impl SnapshotSpace {
  /// 全空间数组序 (原版按 0..kNumberOfSnapshotSpaces 遍历)
  pub const ALL: [SnapshotSpace; NUMBER_OF_SNAPSHOT_SPACES] = [
    ${spaces.map((s) => `SnapshotSpace::${s.rustName}`).join(",\n    ")},
  ];

  /// 从数值反查 (原版以空间值直接索引)
  #[inline]
  #[must_use]
  pub const fn from_repr(val: u8) -> Option<Self> {
    match val {
`;
for (const s of spaces) rust += `      ${s.repr} => Some(Self::${s.rustName}),\n`;
rust += `      _ => None,
    }
  }
}

/// SerializerReference 内嵌特殊值类型 (原版 SerializerReference::SpecialValueType,
/// private 枚举, 顺序赋值)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SpecialValueType {
`;
for (const s of svt) {
  rust += `  /// ${s.raw}\n  ${s.rustName} = ${s.repr},\n`;
}
rust += `}

/// SerializerReference 位域布局 (原版 TypeBits / ValueBits, 打包进单个 u32):
/// - bit 0..2   TypeBits   : SpecialValueType, 宽 ${typeW}
/// - bit 2..32  ValueBits  : u32 索引, 宽 ${valueW}
pub mod serializer_reference_bits {
  /// TypeBits 起始位
  pub const TYPE_OFFSET: u32 = ${typeOff};
  /// TypeBits 宽度
  pub const TYPE_BITS: u32 = ${typeW};
  /// ValueBits 起始位 (TypeBits::kSize)
  pub const VALUE_OFFSET: u32 = ${typeOff + typeW};
  /// ValueBits 宽度 (32 - TypeBits::kSize)
  pub const VALUE_BITS: u32 = ${valueW};

  /// TypeBits 掩码
  pub const TYPE_MASK: u32 = ((1 << TYPE_BITS) - 1) << TYPE_OFFSET;
  /// ValueBits 掩码
  pub const VALUE_MASK: u32 = ((1 << VALUE_BITS) - 1) << VALUE_OFFSET;

  /// 原版 TypeBits::encode
  #[inline]
  #[must_use]
  pub const fn encode_type(t: u8) -> u32 {
    ((t as u32) & ((1 << TYPE_BITS) - 1)) << TYPE_OFFSET
  }

  /// 原版 TypeBits::decode
  #[inline]
  #[must_use]
  pub const fn decode_type(bits: u32) -> u8 {
    ((bits & TYPE_MASK) >> TYPE_OFFSET) as u8
  }

  /// 原版 ValueBits::encode
  #[inline]
  #[must_use]
  pub const fn encode_value(v: u32) -> u32 {
    v << VALUE_OFFSET
  }

  /// 原版 ValueBits::decode
  #[inline]
  #[must_use]
  pub const fn decode_value(bits: u32) -> u32 {
    (bits & VALUE_MASK) >> VALUE_OFFSET
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(NUMBER_OF_SNAPSHOT_SPACES == 4);
  assert!(SnapshotSpace::ReadOnlyHeap as u8 == 0);
  assert!(SnapshotSpace::Old as u8 == 1);
  assert!(SnapshotSpace::Code as u8 == 2);
  assert!(SnapshotSpace::Trusted as u8 == 3);
  assert!(SpecialValueType::BackReference as u8 == 0);
  assert!(SpecialValueType::AttachedReference as u8 == 1);
  assert!(SpecialValueType::OffHeapBackingStore as u8 == 2);
  assert!(SpecialValueType::BuiltinReference as u8 == 3);
  assert!(serializer_reference_bits::TYPE_BITS == 2);
  assert!(serializer_reference_bits::VALUE_BITS == 30);
};
`;

const p = writeSrc("v8-snapshot", "references.rs", rust);
console.log(`written ${p}`);
