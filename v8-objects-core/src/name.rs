// 由 tool/src/objects/name.h.js 从 v8/src/objects/name.h + name-inl.h 生成; 修订就地改, 复跑覆盖
// 哈希字段位段族机械展开; String 慢路径/转发表推迟归 v8-objects-strings

//! Name: 可作属性名的一切对象 (String 与 Symbol) 的公共基类, 持有 32 位原始哈希字段。

use crate::{heap_object::HeapObject, map::Map, tagged::Tagged};

/// 哈希字段存储类型 (对应 C++ Name::HashFieldType, 判别值 1:1)。
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum HashFieldType {
  /// 已计算的属性哈希值 (0b10)。
  Hash = 0b10,
  /// 数组整数索引 (0b00)。
  IntegerIndex = 0b00,
  /// 字符串转发表行索引 (0b01)。
  ForwardingIndex = 0b01,
  /// 空状态, 哈希未计算 (0b11)。
  Empty = 0b11,
}

// ── 哈希字段位段常量 (name.h:175-239 BitField 链机械展开) ──────────────────
pub const HASH_FIELD_TYPE_SHIFT: u32 = 0;
pub const HASH_FIELD_TYPE_SIZE: u32 = 2;
pub const HASH_FIELD_TYPE_MASK: u32 = (1u32 << 2) - 1;
pub const HASH_SHIFT: u32 = 2;
pub const HASH_SIZE: u32 = 30;
pub const HASH_MASK: u32 = ((1u32 << 30) - 1) << 2;
pub const IS_INTERNALIZED_FORWARDING_INDEX_SHIFT: u32 = 2;
pub const IS_INTERNALIZED_FORWARDING_INDEX_SIZE: u32 = 1;
pub const IS_INTERNALIZED_FORWARDING_INDEX_MASK: u32 = ((1u32 << 1) - 1) << 2;
pub const IS_EXTERNAL_FORWARDING_INDEX_SHIFT: u32 = 3;
pub const IS_EXTERNAL_FORWARDING_INDEX_SIZE: u32 = 1;
pub const IS_EXTERNAL_FORWARDING_INDEX_MASK: u32 = ((1u32 << 1) - 1) << 3;
pub const FORWARDING_INDEX_VALUE_SHIFT: u32 = 4;
pub const FORWARDING_INDEX_VALUE_SIZE: u32 = 28;
pub const FORWARDING_INDEX_VALUE_MASK: u32 = ((1u32 << 28) - 1) << 4;
pub const ARRAY_INDEX_VALUE_SHIFT: u32 = 2;
pub const ARRAY_INDEX_VALUE_SIZE: u32 = 24;
pub const ARRAY_INDEX_VALUE_MASK: u32 = ((1u32 << 24) - 1) << 2;
pub const ARRAY_INDEX_LENGTH_SHIFT: u32 = 26;
pub const ARRAY_INDEX_LENGTH_SIZE: u32 = 6;
pub const ARRAY_INDEX_LENGTH_MASK: u32 = ((1u32 << 6) - 1) << 26;

/// 未计算哈希的判定位掩码 (最低位为 1 → 空状态或转发索引)。
pub const HASH_NOT_COMPUTED_MASK: u32 = 1;

/// 哈希字段初始空状态数值 (HashFieldTypeBits::encode(kEmpty) = 0b11)。
pub const EMPTY_HASH_FIELD: u32 = 0b11;

/// 含缓存数组索引时不可能置位的掩码 (原版 kDoesNotContainCachedArrayIndexMask):
/// 任一位置 1 → 哈希字段不含缓存数组索引。
pub const DOES_NOT_CONTAIN_CACHED_ARRAY_INDEX_MASK: u32 =
  (!7u32 << ARRAY_INDEX_LENGTH_SHIFT) | HASH_FIELD_TYPE_MASK;

/// 不含整数/转发索引的判定位 (原版 kDoesNotContainIntegerOrForwardingIndexMask)。
pub const DOES_NOT_CONTAIN_INTEGER_OR_FORWARDING_INDEX_MASK: u32 = 0b10;

// ── 数组索引常量 (name.h:198-234) ────────────────────────────────────────
/// 可把索引缓存在哈希字段中的数组索引字符串最大长度。
pub const MAX_CACHED_ARRAY_INDEX_LENGTH: u32 = 7;
/// 最大数组索引 (kMaxUInt32 - 1)。
pub const MAX_ARRAY_INDEX: u32 = u32::MAX - 1;
/// 尝试把字符串转为数组索引时考虑的最大字符数。
pub const MAX_ARRAY_INDEX_SIZE: u32 = 10;
/// 可能是规范 "整数索引" 的最大字符数 (size_t=8B: min(16, 10*2)=16)。
pub const MAX_INTEGER_INDEX_SIZE: u32 = 16;

const _: () = assert!(HASH_FIELD_TYPE_MASK == 0b11);
const _: () = assert!(ARRAY_INDEX_LENGTH_SHIFT + ARRAY_INDEX_LENGTH_SIZE == 32);
const _: () = assert!(10 < (1 << ARRAY_INDEX_LENGTH_SIZE));
const _: () = assert!(
  (HashFieldType::IntegerIndex as u32 & DOES_NOT_CONTAIN_INTEGER_OR_FORWARDING_INDEX_MASK) == 0
);
const _: () = assert!(
  (HashFieldType::ForwardingIndex as u32 & DOES_NOT_CONTAIN_INTEGER_OR_FORWARDING_INDEX_MASK) == 0
);

/// 属性名称基类 (Name)
///
/// 字符串 (String) 与符号 (Symbol) 的共同基类，内部持有 32 位原始哈希字段。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Name {
  /// 继承自 HeapObject
  pub header: HeapObject,
  /// 原始哈希字段 (包含标志位与哈希载荷)
  pub raw_hash_field: u32,
}

/// 哈希字段位判定与构造 (原版 Name 静态函数族, name-inl.h:113-169)。
impl Name {
  /// 哈希是否已计算 (bit0=0; 原版 IsHashFieldComputed)。
  #[inline]
  #[must_use]
  pub const fn is_hash_field_computed(raw_hash_field: u32) -> bool {
    (raw_hash_field & HASH_NOT_COMPUTED_MASK) == 0
  }

  /// 是否已计算哈希 (Hash 型; 原版 IsHash)。
  #[inline]
  #[must_use]
  pub const fn is_hash(raw_hash_field: u32) -> bool {
    (raw_hash_field & HASH_FIELD_TYPE_MASK) == HashFieldType::Hash as u32
  }

  /// 是否数组整数索引 (原版 IsIntegerIndex)。
  #[inline]
  #[must_use]
  pub const fn is_integer_index(raw_hash_field: u32) -> bool {
    (raw_hash_field & HASH_FIELD_TYPE_MASK) == HashFieldType::IntegerIndex as u32
  }

  /// 是否字符串转发表索引 (原版 IsForwardingIndex)。
  #[inline]
  #[must_use]
  pub const fn is_forwarding_index(raw_hash_field: u32) -> bool {
    (raw_hash_field & HASH_FIELD_TYPE_MASK) == HashFieldType::ForwardingIndex as u32
  }

  /// 是否内部化字符串转发索引 (原版 IsInternalizedForwardingIndex)。
  #[inline]
  #[must_use]
  pub const fn is_internalized_forwarding_index(raw_hash_field: u32) -> bool {
    Name::is_forwarding_index(raw_hash_field)
      && (raw_hash_field & IS_INTERNALIZED_FORWARDING_INDEX_MASK) != 0
  }

  /// 是否外部字符串转发索引 (原版 IsExternalForwardingIndex)。
  #[inline]
  #[must_use]
  pub const fn is_external_forwarding_index(raw_hash_field: u32) -> bool {
    Name::is_forwarding_index(raw_hash_field)
      && (raw_hash_field & IS_EXTERNAL_FORWARDING_INDEX_MASK) != 0
  }

  /// 构造哈希字段值 (原版 CreateHashFieldValue; 禁止 ForwardingIndex 型)。
  #[inline]
  #[must_use]
  pub const fn create_hash_field_value(hash: u32, ty: HashFieldType) -> u32 {
    debug_assert!(!matches!(ty, HashFieldType::ForwardingIndex));
    ((hash << HASH_SHIFT) & HASH_MASK) | (ty as u32 & HASH_FIELD_TYPE_MASK)
  }

  /// 构造内部化转发索引字段 (原版 CreateInternalizedForwardingIndex;
  /// 内部位=1, 外部位=0 (位清零即省略))。
  #[inline]
  #[must_use]
  pub const fn create_internalized_forwarding_index(index: u32) -> u32 {
    (index << FORWARDING_INDEX_VALUE_SHIFT)
      | IS_INTERNALIZED_FORWARDING_INDEX_MASK
      | (HashFieldType::ForwardingIndex as u32)
  }

  /// 构造外部转发索引字段 (原版 CreateExternalForwardingIndex;
  /// 外部位=1, 内部位=0 (位清零即省略))。
  #[inline]
  #[must_use]
  pub const fn create_external_forwarding_index(index: u32) -> u32 {
    (index << FORWARDING_INDEX_VALUE_SHIFT)
      | IS_EXTERNAL_FORWARDING_INDEX_MASK
      | (HashFieldType::ForwardingIndex as u32)
  }

  /// 哈希字段是否含缓存数组索引 (原版 ContainsCachedArrayIndex)。
  #[inline]
  #[must_use]
  pub const fn contains_cached_array_index(raw_hash_field: u32) -> bool {
    (raw_hash_field & DOES_NOT_CONTAIN_CACHED_ARRAY_INDEX_MASK) == 0
  }
}

impl Name {
  /// 以指定 Map 构造未计算哈希的 Name
  #[inline]
  pub const fn new(map: Tagged<Map>) -> Self {
    Self {
      header: HeapObject::new(map),
      raw_hash_field: EMPTY_HASH_FIELD,
    }
  }

  /// 以指定 Map 和原始哈希字段构造 Name
  #[inline]
  pub const fn with_hash_field(map: Tagged<Map>, raw_hash_field: u32) -> Self {
    Self {
      header: HeapObject::new(map),
      raw_hash_field,
    }
  }

  /// 获取原始哈希字段数值
  #[inline]
  pub const fn raw_hash_field(&self) -> u32 {
    self.raw_hash_field
  }

  /// 设置原始哈希字段
  #[inline]
  pub fn set_raw_hash_field(&mut self, hash_field: u32) {
    self.raw_hash_field = hash_field;
  }

  /// 解析哈希字段的类型
  #[inline]
  pub const fn hash_field_type(&self) -> HashFieldType {
    Self::hash_field_type_of(self.raw_hash_field)
  }

  /// 解析任意哈希字段值的类型。
  #[inline]
  #[must_use]
  pub const fn hash_field_type_of(raw_hash_field: u32) -> HashFieldType {
    match raw_hash_field & HASH_FIELD_TYPE_MASK {
      0b00 => HashFieldType::IntegerIndex,
      0b01 => HashFieldType::ForwardingIndex,
      0b10 => HashFieldType::Hash,
      _ => HashFieldType::Empty,
    }
  }

  /// 哈希码是否已计算 (原版 HasHashCode, name-inl.h:171):
  /// 已计算 (Hash/IntegerIndex) 或转发索引均算 — 原版语义, 转发索引可换算出哈希。
  #[inline]
  pub const fn has_hash_code(&self) -> bool {
    Self::is_hash_field_computed(self.raw_hash_field)
      || Self::is_forwarding_index(self.raw_hash_field)
  }

  /// 哈希是否已含转发表索引 (原版 HasForwardingIndex)。
  #[inline]
  pub const fn has_forwarding_index(&self) -> bool {
    Self::is_forwarding_index(self.raw_hash_field)
  }

  /// 是否内部化字符串转发索引 (原版 HasInternalizedForwardingIndex)。
  #[inline]
  pub const fn has_internalized_forwarding_index(&self) -> bool {
    Self::is_internalized_forwarding_index(self.raw_hash_field)
  }

  /// 是否外部字符串转发索引 (原版 HasExternalForwardingIndex)。
  #[inline]
  pub const fn has_external_forwarding_index(&self) -> bool {
    Self::is_external_forwarding_index(self.raw_hash_field)
  }

  /// 获取 30 位哈希载荷 (HashBits::decode = raw >> 2)。
  ///
  /// 前置: 哈希已计算 (原版 hash() 对转发索引走转发表换算 — 推迟,
  /// 见模块头登记; 调用方先用 has_hash_code() 判定)。
  #[inline]
  pub const fn hash(&self) -> u32 {
    (self.raw_hash_field & HASH_MASK) >> HASH_SHIFT
  }

  /// 安全尝试获取已计算的哈希值 (原版 TryGetHash)。
  ///
  /// Hash/IntegerIndex → Some(哈希载荷); 转发索引的转发表换算推迟
  /// (归 v8-objects-strings), 与空状态一并返回 None。
  #[inline]
  pub const fn try_get_hash(&self) -> Option<u32> {
    if Self::is_hash_field_computed(self.raw_hash_field) {
      Some(self.hash())
    } else {
      None
    }
  }

  /// 设置普通属性哈希值 (Hash 型便捷写; 经 create_hash_field_value 语义)。
  #[inline]
  pub fn set_hash(&mut self, hash: u32) {
    self.raw_hash_field = Self::create_hash_field_value(hash, HashFieldType::Hash);
  }
}
