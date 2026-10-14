// 由 tool/src/heap/base/cached-unordered-map.h.js 从 v8/src/heap/base/cached-unordered-map.h 生成; 修订就地改, 复跑覆盖
// LRU 单键缓存哈希表 (absl flat_hash_map → gxhash)

//! CachedUnorderedMap: 以 LRU 方式加速 operator[] 的哈希表。
//! 原版缓存 last_key_/last_mapped_ 裸指针 (对数核对通过);
//! Rust 侧以 Option<(K, *mut V)> 等价承载, 任何插入/删除/清空立即失效缓存
//! (gxhash 扩容会迁移条目, 保留悬垂指针违背健全性 — 语义收严)。

use std::{hash::Hash, mem};

use gxhash::HashMap;

/// 原 CachedUnorderedMap (cached-unordered-map.h:16)。
pub struct CachedUnorderedMap<K: Eq + Hash, V> {
  map: HashMap<K, V>,
  /// SAFETY 约束: 指针仅在上一次 map 变更为纯查找时有效;
  /// 插入/erase/clear/take 后必须置 None。
  last: Option<(K, *mut V)>,
}

// SAFETY: last 原始指针仅在 &mut self (独占访问) 期间解引用,
// 且任何结构性变更 (插入/删除/清空/取走) 均先失效缓存;
// 跨线程移动后接收方独占持有, 与移动前无共享 — 等价原版可移动语义。
unsafe impl<K: Eq + Hash + Send, V: Send> Send for CachedUnorderedMap<K, V> {}

impl<K: Eq + Hash + Clone, V: Default> CachedUnorderedMap<K, V> {
  pub fn new() -> Self {
    // gxhash::HashMap 为 std HashMap + GxBuildHasher 别名, 构造走 Default。
    Self {
      map: HashMap::default(),
      last: None,
    }
  }

  /// 原 operator[] (h:24-43): 命中缓存免查找; 缺省值插入。
  pub fn get_or_insert(&mut self, key: &K) -> &mut V {
    let Self { map, last } = self;
    if let Some((last_key, last_mapped)) = last
      && last_key == key
    {
      // SAFETY: last 仅在上次 map 变更为纯查找时记录;
      // 其后任何插入/删除/清空/取走都会置 None, 指针仍指向 map 内有效槽。
      return unsafe { &mut **last_mapped };
    }
    let value = map.entry(key.clone()).or_default();
    *last = Some((key.clone(), value as *mut V));
    value
  }

  /// 原 erase (h:45-51): 缓存一律失效
  /// (原版仅擦缓存键时失效; gxhash 删除即 swap 搬移, 收严为整体失效, README 已登记)。
  pub fn erase(&mut self, key: &K) -> bool {
    self.last = None;
    self.map.remove(key).is_some()
  }

  /// 原 find (h:55): 无迭代器缓存, 每次真实查找。
  pub fn find(&mut self, key: &K) -> Option<&V> {
    self.map.get(key)
  }

  /// 原 contains (h:62)。
  pub fn contains(&self, key: &K) -> bool {
    self.map.contains_key(key)
  }

  /// 原 clear (h:64-68)。
  pub fn clear(&mut self) {
    self.last = None;
    self.map.clear();
  }

  /// 原 empty (h:70)。
  pub fn is_empty(&self) -> bool {
    self.map.is_empty()
  }

  /// 原 Take (h:72-79): 取走整个 map, 自身清空。
  pub fn take(&mut self) -> HashMap<K, V> {
    self.last = None;
    mem::take(&mut self.map)
  }

  /// 原 begin/end → iter。
  pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
    self.map.iter()
  }
}

impl<K: Eq + Hash + Clone, V: Default> Default for CachedUnorderedMap<K, V> {
  fn default() -> Self {
    Self::new()
  }
}
