//! v8/src/base/hashmap-entry.h 的 1:1 转写
//! TemplateHashMapEntry<Key, Value>: (key, value, hash|exists) 三元组
//! HashField = BitField<uint32_t,0,31>, ExistsField = Next<bool,1> → 位常量直接展开
//! NoHashMapValue 特化 (key 与 value 共 union) → HashSetEntry<K>

/// 无值 map (hashset) 的标记类型 (原版 struct NoHashMapValue {})
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NoHashMapValue {}

/// HashField<uint32_t,0,31>: kMax = 2^31 - 1
pub const K_HASH_VALUE_MASK: u32 = (1 << 31) - 1;

/// 通用 HashMap 条目 (含 value)
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TemplateHashMapEntry<K, V> {
  pub key: K,
  pub value: V,
  /// HashField::encode(hash) | ExistsField::encode(true) 的组合槽位
  hash_and_exists: u32,
}

impl<K, V> TemplateHashMapEntry<K, V> {
  /// TemplateHashMapEntry(Key, Value, uint32_t hash)
  #[inline]
  pub fn new(key: K, value: V, hash: u32) -> Self {
    Self {
      key,
      value,
      hash_and_exists: (hash & K_HASH_VALUE_MASK) | (1 << 31),
    }
  }

  /// hash(): HashField::decode
  #[inline]
  pub fn hash(&self) -> u32 {
    self.hash_and_exists & K_HASH_VALUE_MASK
  }

  /// exists(): ExistsField::decode
  #[inline]
  pub fn exists(&self) -> bool {
    (self.hash_and_exists >> 31) != 0
  }

  /// clear(): ExistsField::update(.., false)
  #[inline]
  pub fn clear(&mut self) {
    self.hash_and_exists &= !(1 << 31);
  }
}

/// 默认空槽 (memset 0 语义: exists=false, hash=0)
impl<K: Default, V: Default> Default for TemplateHashMapEntry<K, V> {
  #[inline]
  fn default() -> Self {
    Self {
      key: K::default(),
      value: V::default(),
      hash_and_exists: 0,
    }
  }
}

/// NoHashMapValue 特化: 不为 value 分配空间 (原版 union { Key key; NoHashMapValue } )
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct HashSetEntry<K> {
  pub key: K,
  hash_and_exists: u32,
}

impl<K> HashSetEntry<K> {
  #[inline]
  pub fn new(key: K, hash: u32) -> Self {
    Self {
      key,
      hash_and_exists: (hash & K_HASH_VALUE_MASK) | (1 << 31),
    }
  }

  #[inline]
  pub fn hash(&self) -> u32 {
    self.hash_and_exists & K_HASH_VALUE_MASK
  }

  #[inline]
  pub fn exists(&self) -> bool {
    (self.hash_and_exists >> 31) != 0
  }

  #[inline]
  pub fn clear(&mut self) {
    self.hash_and_exists &= !(1 << 31);
  }
}

impl<K: Default> Default for HashSetEntry<K> {
  #[inline]
  fn default() -> Self {
    Self {
      key: K::default(),
      hash_and_exists: 0,
    }
  }
}
