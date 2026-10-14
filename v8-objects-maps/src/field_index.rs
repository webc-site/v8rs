// 由 tool/src/objects/field-index.h.js 从 v8/src/objects/field-index.h 生成; 修订就地改, 复跑覆盖
// FieldIndex 位段布局展开 (BitField64 链 → 显式常量) + 结构体编解码 + 只读访问器
//! FieldIndex: 字段索引包装, 位段布局 1:1 对应原版 base::BitField64 链。
//!
//! 结构体字段 (bit_field_: u64) 与编解码规则严格对齐 field-index.h:69-118;
//! 工厂入口 (ForInObjectOffset/ForPropertyIndex/…) 归 crate::field_index_inl。

use v8_common::{K_TAGGED_SIZE, K_TAGGED_SIZE_LOG2};

/// 目标配置依赖常量 (v8/src/common/globals.h 压缩分支)。
pub const TAGGED_SIZE_LOG2: u32 = K_TAGGED_SIZE_LOG2;
/// 原版 kDescriptorIndexBitCount (property-details.h)。
pub const DESCRIPTOR_INDEX_BIT_COUNT: u32 = 10;
/// 原版 kFirstInobjectPropertyOffsetBitCount (property-details.h)。
pub const FIRST_INOBJECT_PROPERTY_OFFSET_BIT_COUNT: u32 = 8;

/// 原版 kOffsetBitsSize = kDescriptorIndexBitCount + 1 + kTaggedSizeLog2。
pub const OFFSET_BITS_BITS: u32 = DESCRIPTOR_INDEX_BIT_COUNT + 1 + K_TAGGED_SIZE_LOG2;
/// 原版 kPropertyArrayDataStartBitCount = 2 + kTaggedSizeLog2。
pub const PROPERTY_ARRAY_DATA_START_BIT_COUNT: u32 = 2 + K_TAGGED_SIZE_LOG2;

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

// ── 位段表 (原版 using 链展开; 顺序 = 位序, 无空洞) ─────────────────────────
/// 原版 OffsetBits<i32, 0, kOffsetBitsSize> 段起始位。
pub const OFFSET_BITS_SHIFT: u32 = 0;
/// OffsetBits 掩码 (低 OFFSET_BITS_BITS 位)。
pub const OFFSET_BITS_MASK: u64 = ((1u64 << OFFSET_BITS_BITS) - 1) << OFFSET_BITS_SHIFT;
/// 原版 IsInObjectBits<bool, 13, 1> 段起始位。
pub const IS_IN_OBJECT_BITS_SHIFT: u32 = OFFSET_BITS_SHIFT + OFFSET_BITS_BITS;
/// IsInObjectBits 位宽。
pub const IS_IN_OBJECT_BITS_BITS: u32 = 1;
/// IsInObjectBits 掩码。
pub const IS_IN_OBJECT_BITS_MASK: u64 =
  ((1u64 << IS_IN_OBJECT_BITS_BITS) - 1) << IS_IN_OBJECT_BITS_SHIFT;
/// 原版 EncodingBits<Encoding, 14, 2> 段起始位。
pub const ENCODING_BITS_SHIFT: u32 = IS_IN_OBJECT_BITS_SHIFT + IS_IN_OBJECT_BITS_BITS;
/// EncodingBits 位宽。
pub const ENCODING_BITS_BITS: u32 = 2;
/// EncodingBits 掩码。
pub const ENCODING_BITS_MASK: u64 = ((1u64 << ENCODING_BITS_BITS) - 1) << ENCODING_BITS_SHIFT;
/// 原版 InObjectPropertyCountBits<i32, 16, kDescriptorIndexBitCount> 段起始位。
pub const IN_OBJECT_PROPERTY_COUNT_BITS_SHIFT: u32 = ENCODING_BITS_SHIFT + ENCODING_BITS_BITS;
/// InObjectPropertyCountBits 位宽。
pub const IN_OBJECT_PROPERTY_COUNT_BITS_BITS: u32 = DESCRIPTOR_INDEX_BIT_COUNT;
/// InObjectPropertyCountBits 掩码。
pub const IN_OBJECT_PROPERTY_COUNT_BITS_MASK: u64 =
  ((1u64 << IN_OBJECT_PROPERTY_COUNT_BITS_BITS) - 1) << IN_OBJECT_PROPERTY_COUNT_BITS_SHIFT;
/// 原版 FirstFieldOffsetInStorageBits 段起始位。
pub const FIRST_FIELD_OFFSET_IN_STORAGE_BITS_SHIFT: u32 =
  IN_OBJECT_PROPERTY_COUNT_BITS_SHIFT + IN_OBJECT_PROPERTY_COUNT_BITS_BITS;
/// FirstFieldOffsetInStorageBits 位宽 = max(kPropertyArrayDataStartBitCount, kFirstInobjectPropertyOffsetBitCount)。
pub const FIRST_FIELD_OFFSET_IN_STORAGE_BITS_BITS: u32 = {
  let a = PROPERTY_ARRAY_DATA_START_BIT_COUNT;
  let b = FIRST_INOBJECT_PROPERTY_OFFSET_BIT_COUNT;
  if a > b { a } else { b }
};
/// FirstFieldOffsetInStorageBits 掩码。
pub const FIRST_FIELD_OFFSET_IN_STORAGE_BITS_MASK: u64 =
  ((1u64 << FIRST_FIELD_OFFSET_IN_STORAGE_BITS_BITS) - 1)
    << FIRST_FIELD_OFFSET_IN_STORAGE_BITS_SHIFT;

/// 位段总宽 (原版 FirstFieldOffsetInStorageBits::kLastUsedBit + 1)。
pub const BITFIELD_USED_BITS: u32 =
  FIRST_FIELD_OFFSET_IN_STORAGE_BITS_SHIFT + FIRST_FIELD_OFFSET_IN_STORAGE_BITS_BITS;

/// 段编码 (对齐 base::BitField64::encode: 截取本段位宽)。
#[inline]
const fn encode_field(value: i64, shift: u32, mask: u64) -> u64 {
  debug_assert!(value >= 0);
  debug_assert!((value as u64) <= (mask >> shift));
  ((value as u64) << shift) & mask
}

/// 段无符号解码 (对齐 base::BitField::decode 无符号分支)。
#[inline]
const fn decode_unsigned(bit_field: u64, shift: u32, mask: u64) -> u32 {
  ((bit_field & mask) >> shift) as u32
}

/// FieldIndex: 字段索引包装, 与原版 `class FieldIndex final` 1:1。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct FieldIndex {
  /// 原版 uint64_t bit_field_。
  pub bit_field: u64,
}

impl FieldIndex {
  /// 默认构造 (原版 FieldIndex() : bit_field_(0))。
  #[inline]
  #[must_use]
  pub const fn zeroed() -> Self {
    Self { bit_field: 0 }
  }

  /// 私有构造器 (原版 FieldIndex(bool, int, Encoding, int, int))。
  #[inline]
  #[must_use]
  const fn from_parts(
    is_inobject: bool,
    offset: i32,
    encoding: Encoding,
    inobject_property_count: i32,
    first_field_offset_in_storage: i32,
  ) -> Self {
    debug_assert!(first_field_offset_in_storage % K_TAGGED_SIZE as i32 == 0); // 原版 DCHECK(IsAligned(first_field_offset_in_storage, kTaggedSize))
    let bit_field = encode_field(
      is_inobject as i64,
      IS_IN_OBJECT_BITS_SHIFT,
      IS_IN_OBJECT_BITS_MASK,
    ) | encode_field(encoding as i64, ENCODING_BITS_SHIFT, ENCODING_BITS_MASK)
      | encode_field(
        first_field_offset_in_storage as i64,
        FIRST_FIELD_OFFSET_IN_STORAGE_BITS_SHIFT,
        FIRST_FIELD_OFFSET_IN_STORAGE_BITS_MASK,
      )
      | encode_field(offset as i64, OFFSET_BITS_SHIFT, OFFSET_BITS_MASK)
      | encode_field(
        inobject_property_count as i64,
        IN_OBJECT_PROPERTY_COUNT_BITS_SHIFT,
        IN_OBJECT_PROPERTY_COUNT_BITS_MASK,
      );
    Self { bit_field }
  }

  /// 原版 is_inobject()。
  #[inline]
  #[must_use]
  pub const fn is_inobject(&self) -> bool {
    decode_unsigned(
      self.bit_field,
      IS_IN_OBJECT_BITS_SHIFT,
      IS_IN_OBJECT_BITS_MASK,
    ) != 0
  }

  /// 原版 is_double()。
  #[inline]
  #[must_use]
  pub const fn is_double(&self) -> bool {
    decode_unsigned(self.bit_field, ENCODING_BITS_SHIFT, ENCODING_BITS_MASK)
      == Encoding::Double as u32
  }

  /// 原版 encoding (EncodingBits::decode)。
  #[inline]
  #[must_use]
  pub const fn encoding(&self) -> Encoding {
    decode_encoding(self.bit_field)
  }

  /// 原版 offset(): 存储对象起始的字节偏移 (对齐 base::BitField::decode 恒非负)。
  #[inline]
  #[must_use]
  pub const fn offset(&self) -> i32 {
    decode_unsigned(self.bit_field, OFFSET_BITS_SHIFT, OFFSET_BITS_MASK) as i32
  }

  /// 原版 first_field_offset_in_storage()。
  #[inline]
  #[must_use]
  pub const fn first_field_offset_in_storage(&self) -> i32 {
    decode_unsigned(
      self.bit_field,
      FIRST_FIELD_OFFSET_IN_STORAGE_BITS_SHIFT,
      FIRST_FIELD_OFFSET_IN_STORAGE_BITS_MASK,
    ) as i32
  }

  /// 原版 inobject_property_count (InObjectPropertyCountBits::decode)。
  #[inline]
  #[must_use]
  pub const fn inobject_property_count(&self) -> i32 {
    decode_unsigned(
      self.bit_field,
      IN_OBJECT_PROPERTY_COUNT_BITS_SHIFT,
      IN_OBJECT_PROPERTY_COUNT_BITS_MASK,
    ) as i32
  }

  /// 原版 offset_in_words(): offset/kTaggedSize (0 基自存储对象起始)。
  #[inline]
  #[must_use]
  pub const fn offset_in_words(&self) -> i32 {
    debug_assert!(self.offset() % K_TAGGED_SIZE as i32 == 0);
    self.offset() / K_TAGGED_SIZE as i32
  }

  /// 原版 outobject_array_index()。
  #[inline]
  #[must_use]
  pub const fn outobject_array_index(&self) -> i32 {
    debug_assert!(!self.is_inobject());
    debug_assert!(
      self.first_field_offset_in_storage()
        == PROPERTY_ARRAY_DATA_START_IN_WORDS * (K_TAGGED_SIZE as i32)
    );
    self.offset_in_words() - PROPERTY_ARRAY_DATA_START_IN_WORDS
  }

  /// 原版 property_index() (0 基自首个 inobject 属性, 越界折入 outobject)。
  #[inline]
  #[must_use]
  pub const fn property_index(&self) -> i32 {
    let mut index = (self.offset() - self.first_field_offset_in_storage()) / K_TAGGED_SIZE as i32;
    if !self.is_inobject() {
      index += self.inobject_property_count();
    }
    index
  }

  /// 原版 GetFieldAccessStubKey(): IsInObject|Encoding|Offset 三段掩码。
  #[inline]
  #[must_use]
  pub const fn get_field_access_stub_key(&self) -> u64 {
    self.bit_field & (IS_IN_OBJECT_BITS_MASK | ENCODING_BITS_MASK | OFFSET_BITS_MASK)
  }

  /// 构造入口 (供 field_index_inl 工厂调用, 复刻原版 private 构造)。
  #[inline]
  #[must_use]
  pub const fn new(
    is_inobject: bool,
    offset: i32,
    encoding: Encoding,
    inobject_property_count: i32,
    first_field_offset_in_storage: i32,
  ) -> Self {
    Self::from_parts(
      is_inobject,
      offset,
      encoding,
      inobject_property_count,
      first_field_offset_in_storage,
    )
  }
}

/// Encoding 段取值 (原版 EncodingBits::decode 结果即 Encoding 枚举值)。
#[inline]
#[must_use]
pub const fn decode_encoding(bit_field: u64) -> Encoding {
  match decode_unsigned(bit_field, ENCODING_BITS_SHIFT, ENCODING_BITS_MASK) {
    0 => Encoding::Tagged,
    1 => Encoding::Double,
    _ => Encoding::Word32,
  }
}

/// 原版 GetFieldAccessStubKey 静态版本: 保留 IsInObject|Encoding|Offset 三段。
#[inline]
#[must_use]
pub const fn get_field_access_stub_key(bit_field: u64) -> u64 {
  bit_field & (IS_IN_OBJECT_BITS_MASK | ENCODING_BITS_MASK | OFFSET_BITS_MASK)
}

// ── 存储对象起点 (词单位) ──
/// 原版 JSObject::kHeaderSize / kTaggedSize (压缩配置 map+properties+elements = 3 词)。
pub const JS_OBJECT_HEADER_SIZE_IN_WORDS: i32 = 3;
/// 原版 FieldStorageLocation::kFirstOutOfObjectOffsetInWords (= OFFSET_OF_DATA_START(PropertyArray)/kTaggedSize, property-details.h:270)。
pub const PROPERTY_ARRAY_DATA_START_IN_WORDS: i32 = 2;

// —— 提取不变式断言 (对齐原版 static_assert 与 field-index.h 位序) ——
const _: () = {
  assert!(OFFSET_BITS_BITS == 13);
  assert!(FIRST_FIELD_OFFSET_IN_STORAGE_BITS_BITS == 8);
  assert!(ENCODING_BITS_MASK == 0xc000);
  assert!(IS_IN_OBJECT_BITS_MASK == 0x2000);
  assert!(IN_OBJECT_PROPERTY_COUNT_BITS_MASK == 0x3ff0000);
  assert!(FIRST_FIELD_OFFSET_IN_STORAGE_BITS_MASK == 0x3fc000000);
  assert!(BITFIELD_USED_BITS == 34 && BITFIELD_USED_BITS < 64);
};
