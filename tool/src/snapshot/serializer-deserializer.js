#!/usr/bin/env bun
// tool/src/snapshot/serializer-deserializer.js
// 机械提取 v8/src/snapshot/serializer-deserializer.{h,cc} 的序列化字节码体系:
// Bytecode 枚举 / UNUSED_SERIALIZER_BYTE_CODES X-macro 展开 / 段编码常量

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/snapshot/serializer-deserializer.h");

// 1. SlotType 枚举
const slotM = hSrc.match(/enum class SlotType \{([^}]+)\};/);
if (!slotM) throw new Error("未找到 SlotType 枚举");
const slotTypes = slotM[1]
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);
if (slotTypes.length !== 2) {
  throw new Error(`SlotType 变体数不符: 期望 2, 实得 ${slotTypes.length}`);
}

// 2. UNUSED_SERIALIZER_BYTE_CODES X-macro 展开 → 空闲码表
const xmacroM = hSrc.match(
  /#define UNUSED_SERIALIZER_BYTE_CODES\(V\)([\s\S]*?)\/\/ clang-format on/,
);
if (!xmacroM) throw new Error("未找到 UNUSED_SERIALIZER_BYTE_CODES X-macro");
const unused = [...xmacroM[1].matchAll(/V\((0x[0-9a-fA-F]+)\)/g)].map((m) =>
  parseInt(m[1], 16),
);
if (unused.length !== 133) {
  throw new Error(`X-macro 空闲码数不符: 期望 133, 实得 ${unused.length}`);
}
// 空闲码必须严格递增且不重复
for (let i = 1; i < unused.length; i++) {
  if (unused[i] <= unused[i - 1]) throw new Error(`X-macro 空闲码乱序: @${i}`);
}

// 3. Bytecode 枚举 (显式值 + 顺序递增)
const bcM = hSrc.match(/enum Bytecode : uint8_t \{([\s\S]*?)\n  \};/);
if (!bcM) throw new Error("未找到 Bytecode 枚举");
const bytecodes = [];
let next = 0;
for (const rawLine of bcM[1].split("\n")) {
  const line = rawLine.replace(/\/\/.*$/, "").trim();
  if (!line) continue;
  const m = line.match(/^k(\w+)(?:\s*=\s*(0x[0-9a-fA-F]+|\d+))?,?$/);
  if (!m) continue;
  let repr = next;
  if (m[2] !== undefined) repr = parseInt(m[2], m[2].startsWith("0x") ? 16 : 10);
  bytecodes.push({ raw: `k${m[1]}`, rustName: m[1], repr });
  next = repr + 1;
}
if (bytecodes.length !== 36) {
  throw new Error(`Bytecode 条数不符: 期望 36, 实得 ${bytecodes.length}`);
}
const byName = Object.fromEntries(bytecodes.map((b) => [b.raw, b.repr]));
// 段锚点对数 (编译器侧基准见 clang -E)
if (byName.kNewObject !== 0x00) throw new Error("kNewObject != 0x00");
if (byName.kBackref !== 0x04) throw new Error("kBackref != 0x04");
if (byName.kProtectedPointerPrefix !== 0x22) throw new Error("kProtectedPointerPrefix != 0x22");
if (byName.kRootArrayConstants !== 0x40) throw new Error("kRootArrayConstants != 0x40");
if (byName.kFixedRawData !== 0x60) throw new Error("kFixedRawData != 0x60");
if (byName.kFixedRepeatRoot !== 0x80) throw new Error("kFixedRepeatRoot != 0x80");
if (byName.kHotObject !== 0x90) throw new Error("kHotObject != 0x90");

// 4. 段宽常量 (h 内 static const int)
function mustConst(name, expect) {
  const m = hSrc.match(new RegExp(`static const (?:int|uint32_t) ${name} = ([^;]+);`));
  if (!m) throw new Error(`未找到常量 ${name}`);
  const v = parseInt(m[1].trim(), m[1].includes("0x") ? 16 : 10);
  if (v !== expect) throw new Error(`常量 ${name}=${v} 与期望 ${expect} 不符`);
  return v;
}
const rootArrayConstantsCount = mustConst("kRootArrayConstantsCount", 0x20);
const fixedRawDataCount = mustConst("kFixedRawDataCount", 0x20);
const fixedRepeatRootCount = mustConst("kFixedRepeatRootCount", 0x10);
const hotObjectCount = mustConst("kHotObjectCount", 8);
const doubleAlignmentSentinel = mustConst("kDoubleAlignmentSentinel", 0);
const emptyBackingStoreRefSentinel = mustConst("kEmptyBackingStoreRefSentinel", 0);

// 5. 段编码范围 (kFirstEncodable* 推导)
const firstFixedRaw = mustConst("kFirstEncodableFixedRawDataSize", 1);
const lastFixedRaw = firstFixedRaw + fixedRawDataCount - 1;
const firstRepeatRoot = mustConst("kFirstEncodableRepeatRootCount", 2);
const lastFixedRepeat = firstRepeatRoot + fixedRepeatRootCount - 1;
const firstVariableRepeat = lastFixedRepeat + 1;
console.log(
  `SlotType ${slotTypes.length} + Bytecode ${bytecodes.length} + 空闲码 ${unused.length} + 常量 6`,
);

let rust = banner(
  "src/snapshot/serializer-deserializer.js",
  "src/snapshot/serializer-deserializer.{h,cc}",
  "序列化/反序列化公共字节码体系",
);
rust += `
/// 槽位类型 (原版 enum class SlotType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SlotType {
  /// ${slotTypes[0]}
  AnySlot = 0,
  /// ${slotTypes[1]}
  MapSlot = 1,
}

/// 序列化字节码 (原版 enum Bytecode : uint8_t, 共 ${bytecodes.length} 个命名入口)。
/// 段布局:
/// - 0x00..0x22  逐条字节码区 (kNewObject/kBackref 各占 4 码按空间编码)
/// - 0x40..0x5f  kRootArrayConstants 段 (按 RootIndex 编码)
/// - 0x60..0x7f  kFixedRawData 段 (按原始数据长度编码)
/// - 0x80..0x8f  kFixedRepeatRoot 段 (按重复数编码)
/// - 0x90..0x97  kHotObject 段 (热对象环)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Bytecode {
`;
for (const b of bytecodes) {
  rust += `  /// ${b.raw}${b.repr >= 0x40 ? " (段基址)" : ""}\n  ${b.rustName} = 0x${b.repr.toString(16)},\n`;
}
rust += `}

impl Bytecode {
  /// 命名入口总数
  pub const COUNT: usize = ${bytecodes.length};

  /// 从数值反查 (段内偏移码不在枚举内, 返回 None — 用段辅助函数解码)
  #[inline]
  #[must_use]
  pub const fn from_repr(val: u8) -> Option<Self> {
    match val {
`;
for (const b of bytecodes) {
  rust += `      0x${b.repr.toString(16)} => Some(Self::${b.rustName}),\n`;
}
rust += `      _ => None,
    }
  }
}

/// 空闲/预留字节码 (原版 UNUSED_SERIALIZER_BYTE_CODES X-macro 机械展开,
/// 共 ${unused.length} 个; 段内未占码位另行空闲, 不在此表)#[rustfmt::skip]
pub static UNUSED_SERIALIZER_BYTE_CODES: [u8; ${unused.length}] = [
`;
for (let i = 0; i < unused.length; i += 12) {
  const row = unused
    .slice(i, i + 12)
    .map((v) => `0x${v.toString(16).padStart(2, "0")}`)
    .join(", ");
  rust += `  ${row},\n`;
}
rust += `];

/// 段宽常量 (原版 static const int)
/// 首 ${rootArrayConstantsCount} 个根数组条目直接编入字节码
pub const ROOT_ARRAY_CONSTANTS_COUNT: usize = ${rootArrayConstantsCount};
/// ${fixedRawDataCount} 个常用原始数据长度
pub const FIXED_RAW_DATA_COUNT: usize = ${fixedRawDataCount};
/// ${fixedRepeatRootCount} 个常用重复根数
pub const FIXED_REPEAT_ROOT_COUNT: usize = ${fixedRepeatRootCount};
/// ${hotObjectCount} 个热对象槽
pub const HOT_OBJECT_COUNT: usize = ${hotObjectCount};
/// 新对象后的双对齐哨兵 (原版 kDoubleAlignmentSentinel)
pub const DOUBLE_ALIGNMENT_SENTINEL: i32 = ${doubleAlignmentSentinel};
/// 空后备存储引用哨兵 (原版 kEmptyBackingStoreRefSentinel)
pub const EMPTY_BACKING_STORE_REF_SENTINEL: u32 = ${emptyBackingStoreRefSentinel};

/// 段编码范围 (原版 kFirstEncodable* / kLastEncodable*, 机械展开)
pub const FIRST_ENCODABLE_FIXED_RAW_DATA_SIZE: i32 = ${firstFixedRaw};
pub const LAST_ENCODABLE_FIXED_RAW_DATA_SIZE: i32 = ${lastFixedRaw};
pub const FIRST_ENCODABLE_REPEAT_ROOT_COUNT: i32 = ${firstRepeatRoot};
pub const LAST_ENCODABLE_FIXED_REPEAT_ROOT_COUNT: i32 = ${lastFixedRepeat};
pub const FIRST_ENCODABLE_VARIABLE_REPEAT_ROOT_COUNT: i32 = ${firstVariableRepeat};

/// SpaceEncoder / BytecodeValueEncoder 机械展开:
/// 段基址 + (value - min) 即码位。快照空间共 4 个 (NUMBER_OF_SNAPSHOT_SPACES),
/// 故 kNewObject/kBackref 各占 4 码。
pub mod encode {
  use super::Bytecode;

  /// 快照空间数 (原版 kNumberOfSnapshotSpaces, 对齐 references.rs)
  pub const NUMBER_OF_SNAPSHOT_SPACES: usize = 4;

  /// 原版 SpaceEncoder<kNewObject>::Encode(space): 0x00..0x03
  #[inline]
  #[must_use]
  pub const fn encode_new_object(space: u8) -> u8 {
    Bytecode::NewObject as u8 + space
  }

  /// 原版 SpaceEncoder<kNewObject>::Decode
  #[inline]
  #[must_use]
  pub const fn decode_new_object(bc: u8) -> u8 {
    bc - Bytecode::NewObject as u8
  }

  /// 原版 SpaceEncoder<kBackref>::Encode(space): 0x04..0x07
  #[inline]
  #[must_use]
  pub const fn encode_backref(space: u8) -> u8 {
    Bytecode::Backref as u8 + space
  }

  /// 原版 FixedRawDataWithSize::Encode(size): 0x60..0x7f
  #[inline]
  #[must_use]
  pub const fn encode_fixed_raw_data(size: i32) -> u8 {
    (Bytecode::FixedRawData as u8 as i32 + size - super::FIRST_ENCODABLE_FIXED_RAW_DATA_SIZE) as u8
  }

  /// 原版 FixedRawDataWithSize::Decode
  #[inline]
  #[must_use]
  pub const fn decode_fixed_raw_data(bc: u8) -> i32 {
    bc as i32 - Bytecode::FixedRawData as u8 as i32 + super::FIRST_ENCODABLE_FIXED_RAW_DATA_SIZE
  }

  /// 原版 FixedRepeatRootWithCount::Encode(count): 0x80..0x8f
  #[inline]
  #[must_use]
  pub const fn encode_fixed_repeat_root(count: i32) -> u8 {
    (Bytecode::FixedRepeatRoot as u8 as i32
      + count
      - super::FIRST_ENCODABLE_REPEAT_ROOT_COUNT) as u8
  }

  /// 原版 FixedRepeatRootWithCount::Decode
  #[inline]
  #[must_use]
  pub const fn decode_fixed_repeat_root(bc: u8) -> i32 {
    bc as i32 - Bytecode::FixedRepeatRoot as u8 as i32
      + super::FIRST_ENCODABLE_REPEAT_ROOT_COUNT
  }

  /// 原版 RootArrayConstant::Encode(root_index): 0x40..0x5f
  #[inline]
  #[must_use]
  pub const fn encode_root_array_constant(root_index: i32) -> u8 {
    (Bytecode::RootArrayConstants as u8 as i32 + root_index) as u8
  }

  /// 原版 HotObject::Encode(index): 0x90..0x97
  #[inline]
  #[must_use]
  pub const fn encode_hot_object(index: i32) -> u8 {
    (Bytecode::HotObject as u8 as i32 + index) as u8
  }

  /// 原版 VariableRepeatRootCount::Encode(count) (变长重复, 减去可编码下限)
  #[inline]
  #[must_use]
  pub const fn encode_variable_repeat_root(count: i32) -> i32 {
    count - super::FIRST_ENCODABLE_VARIABLE_REPEAT_ROOT_COUNT
  }

  /// 原版 VariableRepeatRootCount::Decode
  #[inline]
  #[must_use]
  pub const fn decode_variable_repeat_root(value: i32) -> i32 {
    value + super::FIRST_ENCODABLE_VARIABLE_REPEAT_ROOT_COUNT
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(Bytecode::COUNT == 36);
  assert!(Bytecode::NewObject as u8 == 0x00);
  assert!(Bytecode::Backref as u8 == 0x04);
  assert!(Bytecode::ProtectedPointerPrefix as u8 == 0x22);
  assert!(Bytecode::RootArrayConstants as u8 == 0x40);
  assert!(Bytecode::FixedRawData as u8 == 0x60);
  assert!(Bytecode::FixedRepeatRoot as u8 == 0x80);
  assert!(Bytecode::HotObject as u8 == 0x90);
  assert!(UNUSED_SERIALIZER_BYTE_CODES[0] == 0x23);
  assert!(UNUSED_SERIALIZER_BYTE_CODES[${unused.length} - 1] == 0xff);
  assert!(ROOT_ARRAY_CONSTANTS_COUNT == 0x20);
  assert!(FIXED_RAW_DATA_COUNT == 0x20);
  assert!(FIXED_REPEAT_ROOT_COUNT == 0x10);
  assert!(HOT_OBJECT_COUNT == 8);
  assert!(encode::encode_new_object(3) == 0x03);
  assert!(encode::encode_backref(3) == 0x07);
  assert!(encode::decode_fixed_raw_data(0x60) == 1);
  assert!(encode::encode_fixed_raw_data(0x20) == 0x7f);
  assert!(encode::decode_fixed_repeat_root(0x80) == 2);
  assert!(encode::encode_hot_object(7) == 0x97);
};
`;

const p = writeSrc("v8-snapshot", "serializer_deserializer.rs", rust);
console.log(`written ${p}`);
