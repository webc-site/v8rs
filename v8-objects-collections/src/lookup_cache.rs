// 由 tool/src/objects/lookup-cache.cc.js 从 v8/src/objects/lookup-cache.{h,-inl.h,cc} 生成; 修订就地改, 复跑覆盖
// (map, name) → 描述符索引 缓存 (含负结果)

//! DescriptorLookupCache: (map, 属性名) → 描述符下标的直接映射缓存。
//!
//! - 槽位 64, 正负结果皆缓存; kAbsent 表示未命中
//! - 哈希 = (map地址>>kTaggedSizeLog2) ^ name哈希, 对槽数取模
//! - 指针可能过期 → 读侧用 SafeEquals 比对; GC 前整体清空

use v8_objects_core::name::Name;
use v8_objects_core::tagged_constants::HEAP_OBJECT_TAG_MASK;

/// 未命中哨兵 (kAbsent; 注意与查找失败的 -1 区分)。
pub const K_ABSENT: i32 = -2;
/// 缓存槽数 (kLength)。
pub const K_LENGTH: usize = 64;
/// TaggedSize 位移 (指针压缩配置 kTaggedSizeLog2 = 2)。
pub const K_TAGGED_SIZE_LOG2: u32 = 2;

/// 缓存键 (map 指针 + name 指针)。
#[derive(Clone, Copy, Default)]
pub struct Key {
  /// map 的 Tagged 地址 (0 = 空槽; Clear 置 0)。
  pub source: usize,
  /// name 的 Tagged 地址。
  pub name: usize,
}

/// (map, name) → 描述符索引缓存 (lookup-cache.h:19 DescriptorLookupCache)。
pub struct DescriptorLookupCache {
  keys: [Key; K_LENGTH],
  results: [i32; K_LENGTH],
}

impl Default for DescriptorLookupCache {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

impl DescriptorLookupCache {
  /// 构造: 全槽清空 (原版私有无参构造; name 置空指针, 结果置 kAbsent)。
  #[inline]
  #[must_use]
  pub const fn new() -> Self {
    Self {
      keys: [Key { source: 0, name: 0 }; K_LENGTH],
      results: [K_ABSENT; K_LENGTH],
    }
  }

  /// 槽位哈希 (lookup-cache-inl.h:18 Hash)。
  ///
  /// 指针仅取低 32 位场景 (原版注释); dcheck 契约: name 须为 UniqueName。
  #[inline]
  #[must_use]
  pub const fn hash(source: usize, name_hash: u32) -> usize {
    let source_hash = ((source as u64) as u32) >> K_TAGGED_SIZE_LOG2;
    ((source_hash ^ name_hash) as usize) % K_LENGTH
  }

  /// 查描述符索引 (lookup-cache-inl.h:26 Lookup); 未命中返回 kAbsent。
  ///
  /// 槽内指针可能过期 (GC 移动对象) → 仅按当前指针相等比对 (SafeEquals 语义)。
  #[inline]
  #[must_use]
  pub fn lookup(&self, source: usize, name: usize) -> i32 {
    let index = Self::hash(source, Self::name_hash_of(name));
    let key = &self.keys[index];
    if key.source == source && key.name == name {
      return self.results[index];
    }
    K_ABSENT
  }

  /// 更新槽位 (lookup-cache-inl.h:36 Update)。
  ///
  /// 契约: result ≠ kAbsent (原版 DCHECK_NE)。
  #[inline]
  pub fn update(&mut self, source: usize, name: usize, result: i32) {
    debug_assert_ne!(result, K_ABSENT);
    let index = Self::hash(source, Self::name_hash_of(name));
    self.keys[index] = Key { source, name };
    self.results[index] = result;
  }

  /// 清空缓存 (lookup-cache.cc:10 Clear; 仅清 source 即视为空槽)。
  #[inline]
  pub const fn clear(&mut self) {
    let mut i = 0;
    while i < K_LENGTH {
      self.keys[i].source = 0;
      i += 1;
    }
  }

  /// name 哈希读取 (原版 name->hash(); 剥 Tag 位后读 Name.raw_hash_field 30 位哈希载荷)。
  ///
  /// v8: src/objects/name-inl.h Name::hash
  fn name_hash_of(name: usize) -> u32 {
    // 堆对象地址: 剥离低 2 位 Tag (4 字节对齐保证无损还原)
    let addr = name & !HEAP_OBJECT_TAG_MASK;
    // SAFETY: name 为 UniqueName 的有效 Tagged 指针 (lookup/update 调用方契约);
    // Name 头部 = HeapObject(8) + raw_hash_field(4), 地址处存在合法结构体布局
    unsafe { &*(addr as *const Name) }.hash()
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_ABSENT == -2 && K_LENGTH == 64 && K_TAGGED_SIZE_LOG2 == 2);
  assert!(DescriptorLookupCache::hash(0, 0) == 0);
  // (source>>2) ^ name_hash 对 64 取模
  assert!(DescriptorLookupCache::hash(1 << 2, 0b1) == 1);
  assert!(DescriptorLookupCache::hash(64, 5) == (1 ^ 5));
  let mut c = DescriptorLookupCache::new();
  c.keys[0] = Key { source: 11, name: 22 };
  c.results[0] = 7;
  assert!(c.results[0] == 7);
  c.clear();
  assert!(c.keys[0].source == 0);
};
