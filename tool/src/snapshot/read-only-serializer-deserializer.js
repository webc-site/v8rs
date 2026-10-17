#!/usr/bin/env bun
// tool/src/snapshot/read-only-serializer-deserializer.js
// 机械提取 v8/src/snapshot/read-only-serializer-deserializer.h 的 ro::Bytecode
// 枚举、BitSet 布局与 EncodedTagged/EncodedExternalReference 位域

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/snapshot/read-only-serializer-deserializer.h");

// 1. ro::Bytecode 枚举 (顺序赋值)
const bcM = src.match(/enum Bytecode \{([^}]+)\};/);
if (!bcM) throw new Error("未找到 ro::Bytecode 枚举");
const rawBc = bcM[1]
  .replace(/\/\/.*$/gm, "")
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);
if (rawBc.length !== 6) {
  throw new Error(`ro::Bytecode 条数不符: 期望 6, 实得 ${rawBc.length}`);
}
const bytecodes = rawBc.map((t, idx) => {
  const nm = t.match(/^k(\w+)$/);
  if (!nm) throw new Error(`ro::Bytecode 条目异常: ${t}`);
  return { raw: `k${nm[1]}`, rustName: nm[1], repr: idx };
});

// 2. kNumberOfBytecodes = kFinalizeReadOnlySpace + 1
const numM = src.match(/static constexpr int kNumberOfBytecodes =\s*static_cast<int>\(kFinalizeReadOnlySpace\) \+ (\d+);/);
if (!numM || numM[1] !== "1") throw new Error("kNumberOfBytecodes 表达式异常");

// 3. BitSet: kBitsPerChunk = kUInt8Size * kBitsPerByte = 8
const bitsPerChunkM = src.match(/static constexpr int kBitsPerChunk = kUInt8Size \* kBitsPerByte;/);
if (!bitsPerChunkM) throw new Error("未找到 kBitsPerChunk");

// 4. EncodedTagged 位域: kOffsetBits = kPageSizeBits, kSize = kUInt32Size,
//    kPageIndexBits = kSize*8 - kOffsetBits
const etM = src.match(/struct EncodedTagged \{([\s\S]*?)\n\};/);
if (!etM) throw new Error("未找到 EncodedTagged");
if (!etM[1].includes("kOffsetBits = kPageSizeBits")) throw new Error("EncodedTagged::kOffsetBits 异常");
const K_PAGE_SIZE_BITS = 18; // v8 目标配置 (x64/arm64, 256k 页): kPageSizeBits 机械展开
const etSize = 4;
const etOffsetBits = K_PAGE_SIZE_BITS;
const etPageIndexBits = etSize * 8 - etOffsetBits;

// 5. EncodedExternalReference 位域: tag 8 + is_api_reference 1 + index 23
const eerM = src.match(/struct EncodedExternalReference \{([\s\S]*?)\n\};/);
if (!eerM) throw new Error("未找到 EncodedExternalReference");
const eerTag = parseInt((eerM[1].match(/kTagBits = (\d+);/) || [])[1], 10);
const eerApi = parseInt((eerM[1].match(/kIsApiReferenceBits = (\d+);/) || [])[1], 10);
const eerIndex = parseInt((eerM[1].match(/kIndexBits = (\d+);/) || [])[1], 10);
const eerSize = parseInt((eerM[1].match(/kSize = kUInt32Size/) ? 4 : 0), 10);
if (eerTag + eerApi + eerIndex !== eerSize * 8) {
  throw new Error(`EncodedExternalReference 位宽不齐: ${eerTag}+${eerApi}+${eerIndex}`);
}
console.log(`ro::Bytecode ${bytecodes.length} + BitSet + 位域 (${etPageIndexBits}+${etOffsetBits}) / (${eerTag}+${eerApi}+${eerIndex})`);

let rust = banner(
  "src/snapshot/read-only-serializer-deserializer.js",
  "src/snapshot/read-only-serializer-deserializer.h",
  "RO 序列化/反序列化公共字节码与位域",
);
rust += `
/// RO 序列化字节码 (原版 v8::internal::ro::Bytecode, 顺序赋值)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Bytecode {
`;
for (const b of bytecodes) {
  rust += `  /// ${b.raw}\n  ${b.rustName} = ${b.repr},\n`;
}
rust += `}

/// 字节码总数 (原版 kNumberOfBytecodes = kFinalizeReadOnlySpace + 1)
pub const NUMBER_OF_BYTECODES: usize = ${bytecodes.length};

/// BitSet (原版 ro::BitSet): 已知底层编码的位向量, uint8 chunk
pub mod bitset {
  /// 每 chunk 位数 = kUInt8Size * kBitsPerByte
  pub const BITS_PER_CHUNK: usize = 8;

  /// 原版 chunk_index(i)
  #[inline]
  #[must_use]
  pub const fn chunk_index(i: usize) -> usize {
    i / BITS_PER_CHUNK
  }

  /// 原版 bit_index(i)
  #[inline]
  #[must_use]
  pub const fn bit_index(i: usize) -> usize {
    i % BITS_PER_CHUNK
  }

  /// 原版 bit_mask(i)
  #[inline]
  #[must_use]
  pub const fn bit_mask(i: usize) -> u32 {
    1 << bit_index(i)
  }
}

/// EncodedTagged 位域 (原版 ro::EncodedTagged, 打包 u32;
/// V8_STATIC_ROOTS 关闭时反序列化后需重定位):
/// - 低 ${etOffsetBits} 位 offset (按 kTaggedSizeLog2 移位)
/// - 高 ${etPageIndexBits} 位 page_index (决定 RO 页数上限)
pub mod encoded_tagged {
  /// 原版 kOffsetBits = kPageSizeBits (目标配置 256k 页展开)
  pub const OFFSET_BITS: u32 = ${etOffsetBits};
  /// 原版 kSize = kUInt32Size
  pub const SIZE: u32 = ${etSize};
  /// 原版 kPageIndexBits = kSize * 8 - kOffsetBits
  pub const PAGE_INDEX_BITS: u32 = ${etPageIndexBits};
  pub const PAGE_INDEX_MASK: u32 = ((1 << PAGE_INDEX_BITS) - 1) << OFFSET_BITS;
  pub const OFFSET_MASK: u32 = (1 << OFFSET_BITS) - 1;
}

/// EncodedExternalReference 位域 (原版 ro::EncodedExternalReference, 打包 u32):
/// tag(8) + is_api_reference(1) + index(23)
pub mod encoded_external_reference {
  pub const TAG_BITS: u32 = ${eerTag};
  pub const IS_API_REFERENCE_BITS: u32 = ${eerApi};
  pub const INDEX_BITS: u32 = ${eerIndex};
  pub const SIZE: u32 = ${eerSize};
  pub const TAG_MASK: u32 = ((1 << TAG_BITS) - 1);
  pub const IS_API_REFERENCE_MASK: u32 = 1 << TAG_BITS;
  pub const INDEX_MASK: u32 = ((1 << INDEX_BITS) - 1) << (TAG_BITS + IS_API_REFERENCE_BITS);
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(NUMBER_OF_BYTECODES == 6);
  assert!(Bytecode::AllocatePage as u8 == 0);
  assert!(Bytecode::FinalizeReadOnlySpace as u8 == 5);
  assert!(encoded_tagged::OFFSET_BITS + encoded_tagged::PAGE_INDEX_BITS == 32);
  assert!(encoded_external_reference::TAG_BITS
    + encoded_external_reference::IS_API_REFERENCE_BITS
    + encoded_external_reference::INDEX_BITS == 32);
};
`;

const p = writeSrc("v8-snapshot", "read_only_serializer_deserializer.rs", rust);
console.log(`written ${p}`);
