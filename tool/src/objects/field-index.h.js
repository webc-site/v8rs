#!/usr/bin/env bun
// tool/src/objects/field-index.h.js — 覆盖 v8/src/objects/field-index.{h,-inl.h}
// → v8-objects-maps/src/field_index.rs
// 机械提取: FieldIndex::Encoding 枚举 + BitField64 位段表 (X-macro 式展开为显式常量)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/field-index.h");
const globalsSrc = readV8("src/common/globals.h");
const pdSrc = readV8("src/objects/property-details.h");

// 依赖常量 (原版侧提取, 带守卫)
function grab(src, name, label) {
  const m = src.match(new RegExp(`${name}\\s*=\\s*(\\d+)`));
  if (!m) throw new Error(`未找到 ${label}`);
  return parseInt(m[1], 10);
}

// kTaggedSizeLog2: V8_COMPRESS_POINTERS 分支 (目标配置 x64+arm64 压缩开)
const compressedBranch = globalsSrc.match(
  /#ifdef V8_COMPRESS_POINTERS[\s\S]*?constexpr int kTaggedSizeLog2 = (\d+);/,
);
if (!compressedBranch) throw new Error("未找到压缩分支 kTaggedSizeLog2");
const taggedSizeLog2 = parseInt(compressedBranch[1], 10);
if (taggedSizeLog2 !== 2) throw new Error(`kTaggedSizeLog2=${taggedSizeLog2} != 2 (压缩配置锚)`);

const descriptorBits = grab(pdSrc, "kDescriptorIndexBitCount", "kDescriptorIndexBitCount");
const inObjOffsetBits = grab(pdSrc, "kFirstInobjectPropertyOffsetBitCount", "kFirstInobjectPropertyOffsetBitCount");

// Encoding 枚举
const encRe = hSrc.match(/enum Encoding\s*\{\s*(kTagged),\s*(kDouble),\s*(kWord32)\s*\}/);
if (!encRe) throw new Error("未找到 FieldIndex::Encoding 枚举");

// 位段尺寸 (field-index.h 显式表达式)
const offsetBitsSizeRe = hSrc.match(
  /kOffsetBitsSize =\s*\n\s*\(kDescriptorIndexBitCount \+ 1 \+ kTaggedSizeLog2\)/,
);
if (!offsetBitsSizeRe) throw new Error("未找到 kOffsetBitsSize 表达式");
const propArrDataStartRe = hSrc.match(
  /kPropertyArrayDataStartBitCount = 2 \+ kTaggedSizeLog2/,
);
if (!propArrDataStartRe) throw new Error("未找到 kPropertyArrayDataStartBitCount 表达式");

// 机械展开: kOffsetBitsSize = 10+1+2 = 13; kPropertyArrayDataStartBitCount = 2+2 = 4
const kOffsetBitsSize = descriptorBits + 1 + taggedSizeLog2;
const kPropArrStart = 2 + taggedSizeLog2;

// BitField64 链展开 (原版 using 声明顺序 = 位序, 无空洞):
//   OffsetBits<int, 0, 13> | IsInObjectBits<bool,1> | EncodingBits<Encoding,2>
//   | InObjectPropertyCountBits<int,10> | FirstFieldOffsetInStorageBits<int,max(4,8)>
const firstFieldBits = Math.max(kPropArrStart, inObjOffsetBits);
const bits = [
  { name: "OffsetBits", type: "i32", start: 0, len: kOffsetBitsSize },
  { name: "IsInObjectBits", type: "bool", start: kOffsetBitsSize, len: 1 },
  { name: "EncodingBits", type: "u8", start: kOffsetBitsSize + 1, len: 2 },
  { name: "InObjectPropertyCountBits", type: "i32", start: kOffsetBitsSize + 3, len: descriptorBits },
  { name: "FirstFieldOffsetInStorageBits", type: "i32", start: kOffsetBitsSize + 3 + descriptorBits, len: firstFieldBits },
];
const lastBit = bits[bits.length - 1].start + bits[bits.length - 1].len;
// 原版 static_assert(FirstFieldOffsetInStorageBits::kLastUsedBit < 64)
if (lastBit >= 64) throw new Error(`位段越界: kLastUsedBit=${lastBit} >= 64`);

// EncodingBits 段内编码值 = Encoding 枚举值
let rust = banner("src/objects/field-index.h.js", "src/objects/field-index.{h,-inl.h}", "FieldIndex 位段布局展开 (BitField64 链 → 显式常量)");
rust += `//! FieldIndex: 字段索引包装, 位段布局 1:1 对应原版 base::BitField64 链。

/// 目标配置依赖常量 (v8/src/common/globals.h 压缩分支)。
pub const TAGGED_SIZE_LOG2: u32 = ${taggedSizeLog2};
/// 原版 kDescriptorIndexBitCount (property-details.h)。
pub const DESCRIPTOR_INDEX_BIT_COUNT: u32 = ${descriptorBits};
/// 原版 kFirstInobjectPropertyOffsetBitCount (property-details.h)。
pub const FIRST_INOBJECT_PROPERTY_OFFSET_BIT_COUNT: u32 = ${inObjOffsetBits};

/// 存储编码 (原版 enum Encoding)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Encoding {
  /// kTagged
  Tagged = 0,
  /// kDouble
  Double = 1,
  /// kWord32
  Word32 = 2,
}

/// 位段总宽 (原版 OffsetBits + IsInObject + Encoding + InObjectPropertyCount + FirstFieldOffset)。
pub const BITFIELD_USED_BITS: u32 = ${lastBit};

// ── 位段表 (原版 using 链展开; 顺序 = 位序, 无空洞) ─────────────────────────
`;
for (const b of bits) {
  const mask = (b.len >= 64 ? 0xffffffffffffffffn : (1n << BigInt(b.len)) - 1n) << BigInt(b.start);
  rust += `/// 原版 ${b.name}<${b.type}, ${b.start}, ${b.len}>。
pub const ${b.name.toUpperCase()}_SHIFT: u32 = ${b.start};
/// ${b.name} 位宽。
pub const ${b.name.toUpperCase()}_BITS: u32 = ${b.len};
/// ${b.name} 掩码 (原版 kMask)。
pub const ${b.name.toUpperCase()}_MASK: u64 = 0x${mask.toString(16)};
`;
}

rust += `
/// Encoding 段取值 (原版 EncodingBits::decode 结果即 Encoding 枚举值)。
#[inline]
#[must_use]
pub const fn decode_encoding(bit_field: u64) -> Encoding {
  match ((bit_field & ENCODING_BITS_MASK) >> ENCODING_BITS_SHIFT) as u8 {
    0 => Encoding::Tagged,
    1 => Encoding::Double,
    _ => Encoding::Word32,
  }
}

/// 原版 GetFieldAccessStubKey: 保留 IsInObject|Encoding|Offset 三段。
#[inline]
#[must_use]
pub const fn get_field_access_stub_key(bit_field: u64) -> u64 {
  bit_field & (IS_IN_OBJECT_BITS_MASK | ENCODING_BITS_MASK | OFFSET_BITS_MASK)
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(BITFIELD_USED_BITS == ${lastBit} && BITFIELD_USED_BITS < 64);
  assert!(OFFSET_BITS_BITS == ${kOffsetBitsSize});
  assert!(FIRST_FIELD_OFFSET_IN_STORAGE_BITS_BITS == ${firstFieldBits});
  assert!(ENCODING_BITS_MASK == 0x${((3n << BigInt(kOffsetBitsSize + 1))).toString(16)});
};
`;

const p = writeSrc("v8-objects-maps", "field_index.rs", rust);
console.log(`field_index.rs: Encoding 3 变体 + 5 位段 (共 ${lastBit} 位) 展开, 对数通过`);
