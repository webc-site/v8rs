// 由 tool/src/objects/string-set.h.js 从 v8/src/objects/string-set.{h,-inl.h} 生成; 修订就地改, 复跑覆盖
// Shape 常量与判定契约 1:1; HashTable 基建/New/Add/Has 推迟 (登记见脚本头)

//! StringSet 形状 (Shape): 堆上字符串集合哈希表的键形状契约。
//!
//! 原版 StringSet 是 HashTable<StringSet, StringSetShape> 的特化;
//! HashTable 基建归 v8-objects-collections, 本模块落地形状常量与
//! 判定契约 (登记见脚本头)。

/// 字符串集合条目契约 (String 的 EnsureHash/Equals 段;
/// String 对象绑定归 OS-1, 集成时为 String 实现)。
pub trait StringSetEntry {
  /// 确保哈希已计算并返回 (原版 EnsureHash)。
  fn ensure_hash(&mut self) -> u32;

  /// 内容等值 (原版 String::Equals)。
  fn equals(&self, other: &Self) -> bool;
}

/// 字符串集合形状 (string-set.h:16 StringSetShape)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringSetShape;

impl StringSetShape {
  /// 前缀槽数 (kPrefixSize)。
  pub const PREFIX_SIZE: usize = 0;
  /// 每条目槽数 (kEntrySize)。
  pub const ENTRY_SIZE: usize = 1;
  /// 匹配需查洞 (kMatchNeedsHoleCheck; 删除槽语义随 HashTable 基建)。
  pub const MATCH_NEEDS_HOLE_CHECK: bool = true;
  /// 不做哈希扩散 (kDoHashSpreading)。
  pub const DO_HASH_SPREADING: bool = false;
  /// 哈希位数 (kHashBits = 0 → 全宽)。
  pub const HASH_BITS: u32 = 0;

  /// 键与槽值匹配 (string-set-inl.h:19 IsMatch: key->Equals(value))。
  #[inline]
  #[must_use]
  pub fn is_match<E: StringSetEntry>(key: &E, value: &E) -> bool {
    key.equals(value)
  }

  /// 键哈希 (string-set-inl.h:24 Hash: EnsureHash)。
  #[inline]
  pub fn hash<E: StringSetEntry>(key: &mut E) -> u32 {
    key.ensure_hash()
  }

  /// 槽值哈希 (string-set-inl.h:28 HashForObject: EnsureHash)。
  #[inline]
  pub fn hash_for_object<E: StringSetEntry>(object: &mut E) -> u32 {
    object.ensure_hash()
  }
}
