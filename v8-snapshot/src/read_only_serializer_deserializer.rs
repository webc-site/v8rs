// 由 tool/src/snapshot/read-only-serializer-deserializer.js 从 v8/src/snapshot/read-only-serializer-deserializer.h 生成; 修订就地改, 复跑覆盖
// RO 序列化/反序列化公共字节码与位域

/// v8: src/snapshot/read-only-serializer-deserializer.h ro::Bytecode
/// RO 序列化字节码 (原版 v8::internal::ro::Bytecode, 顺序赋值)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Bytecode {
  // kAllocatePage
  AllocatePage = 0,
  // kAllocatePageAt
  AllocatePageAt = 1,
  // kSegment
  Segment = 2,
  // kRelocateSegment
  RelocateSegment = 3,
  // kReadOnlyRootsTable
  ReadOnlyRootsTable = 4,
  // kFinalizeReadOnlySpace
  FinalizeReadOnlySpace = 5,
}

/// 字节码总数 (原版 kNumberOfBytecodes = kFinalizeReadOnlySpace + 1)
pub const NUMBER_OF_BYTECODES: usize = 6;

/// v8: src/snapshot/read-only-serializer-deserializer.h ro::BitSet
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

/// v8: src/snapshot/read-only-serializer-deserializer.h ro::EncodedTagged
/// EncodedTagged 位域 (原版 ro::EncodedTagged, 打包 u32;
/// V8_STATIC_ROOTS 关闭时反序列化后需重定位):
/// - 低 18 位 offset (按 kTaggedSizeLog2 移位)
/// - 高 14 位 page_index (决定 RO 页数上限)
pub mod encoded_tagged {
  /// 原版 kOffsetBits = kPageSizeBits (目标配置 256k 页展开)
  pub const OFFSET_BITS: u32 = 18;
  /// 原版 kSize = kUInt32Size
  pub const SIZE: u32 = 4;
  /// 原版 kPageIndexBits = kSize * 8 - kOffsetBits
  pub const PAGE_INDEX_BITS: u32 = 14;
  pub const PAGE_INDEX_MASK: u32 = ((1 << PAGE_INDEX_BITS) - 1) << OFFSET_BITS;
  pub const OFFSET_MASK: u32 = (1 << OFFSET_BITS) - 1;
}

/// v8: src/snapshot/read-only-serializer-deserializer.h ro::EncodedExternalReference
/// EncodedExternalReference 位域 (原版 ro::EncodedExternalReference, 打包 u32):
/// tag(8) + is_api_reference(1) + index(23)
pub mod encoded_external_reference {
  pub const TAG_BITS: u32 = 8;
  pub const IS_API_REFERENCE_BITS: u32 = 1;
  pub const INDEX_BITS: u32 = 23;
  pub const SIZE: u32 = 4;
  pub const TAG_MASK: u32 = (1 << TAG_BITS) - 1;
  pub const IS_API_REFERENCE_MASK: u32 = 1 << TAG_BITS;
  pub const INDEX_MASK: u32 = ((1 << INDEX_BITS) - 1) << (TAG_BITS + IS_API_REFERENCE_BITS);
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(NUMBER_OF_BYTECODES == 6);
  assert!(Bytecode::AllocatePage as u8 == 0);
  assert!(Bytecode::FinalizeReadOnlySpace as u8 == 5);
  assert!(encoded_tagged::OFFSET_BITS + encoded_tagged::PAGE_INDEX_BITS == 32);
  assert!(
    encoded_external_reference::TAG_BITS
      + encoded_external_reference::IS_API_REFERENCE_BITS
      + encoded_external_reference::INDEX_BITS
      == 32
  );
};
