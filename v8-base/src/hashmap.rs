//! v8/src/base/hashmap.h 的 1:1 转写 — V8 自研开放寻址 (linear probing) hash 表
//! 红线 (todo 10.3): 保留策略参数化设计 (Entry/MatchFun/AllocationPolicy 策略 + capacity/occupancy/map 三段布局)
//! C++ Entry* 裸指针 API → Rust 以槽位索引 usize + Option 显式失败 (禁 panic)

use std::{
  ffi::c_void,
  marker::PhantomData,
  mem::{size_of, take},
  ops::{Deref, DerefMut},
};

use crate::hashmap_entry::{K_HASH_VALUE_MASK, NoHashMapValue, TemplateHashMapEntry};

/// DefaultAllocationPolicy: malloc/free 数组 (Rust 以 Vec 承接同一分配器语义)
#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultAllocationPolicy;

/// 分配策略 trait (原版 AllocationPolicy 模板参数)
/// allocate_array 返回长度恰为 length 且已 zero-init 的缓冲
/// (对应原版 AllocateArray + memset 0 / Clear); delete_array 对应 DeleteArray
pub trait AllocationPolicy<E> {
  type Buf: Deref<Target = [E]> + DerefMut<Target = [E]>;
  fn allocate_array(&self, length: usize) -> Self::Buf;
  fn delete_array(&self, buf: Self::Buf);
}

impl<E: Default> AllocationPolicy<E> for DefaultAllocationPolicy {
  type Buf = Vec<E>;
  #[inline]
  fn allocate_array(&self, length: usize) -> Vec<E> {
    // memset 0 等价: 逐项 E::default()
    (0..length).map(|_| E::default()).collect()
  }
  #[inline]
  fn delete_array(&self, _buf: Vec<E>) {
    // Vec drop 即 free
  }
}

/// MatchFun 策略 (原版 bool(hash1, hash2, key1, key2))
/// 异质查找: LK 为 lookup key 类型 (原版 Heterogeneous LookupOrInsert)
pub trait MatchFn<LK, K> {
  fn match_fn(&self, hash1: u32, hash2: u32, lookup: &LK, entry: &K) -> bool;
}

/// KeyEqualityMatcher<Key>: 直接按 key 相等比较 (原版忽略 hash 参数)
#[derive(Debug, Clone, Copy, Default)]
pub struct KeyEqualityMatcher<K> {
  _marker: PhantomData<K>,
}

impl<K: PartialEq> MatchFn<K, K> for KeyEqualityMatcher<K> {
  #[inline]
  fn match_fn(&self, _hash1: u32, _hash2: u32, lookup: &K, entry: &K) -> bool {
    lookup == entry
  }
}

/// HashEqualityThenKeyMatcher: 先比 hash 再执行 (可能昂贵的) key 比较
#[derive(Debug, Clone, Copy)]
pub struct HashEqualityThenKeyMatcher<K, F> {
  match_: F,
  _marker: PhantomData<K>,
}

impl<K, F> HashEqualityThenKeyMatcher<K, F> {
  #[inline]
  pub fn new(match_: F) -> Self {
    Self {
      match_,
      _marker: PhantomData,
    }
  }
}

impl<K: Copy, F: Fn(K, K) -> bool> MatchFn<K, K> for HashEqualityThenKeyMatcher<K, F> {
  #[inline]
  fn match_fn(&self, hash1: u32, hash2: u32, key1: &K, key2: &K) -> bool {
    hash1 == hash2 && (self.match_)(*key1, *key2)
  }
}

/// kDefaultHashMapCapacity
pub const K_DEFAULT_HASH_MAP_CAPACITY: u32 = 8;

/// Probe 的 hash 掩码 (原版 hash &= 0x7FFFFFFF)
const K_PROBE_HASH_MASK: u32 = 0x7FFF_FFFF;

type Entry<K, V> = TemplateHashMapEntry<K, V>;

/// TemplateHashMapImpl<Key, Value, MatchFun, AllocationPolicy>
pub struct TemplateHashMapImpl<K, V, M, A: AllocationPolicy<Entry<K, V>>> {
  match_: M,
  allocator_: A,
  map_: Option<A::Buf>,
  capacity_: u32,
  occupancy_: u32,
}

impl<K, V, M, A> TemplateHashMapImpl<K, V, M, A>
where
  A: AllocationPolicy<Entry<K, V>>,
  Entry<K, V>: Default,
{
  /// 构造 (原版 ctor: Initialize(initial_capacity))
  #[inline]
  pub fn new(capacity: u32, match_: M, allocator: A) -> Self {
    let mut this = Self {
      match_,
      allocator_: allocator,
      map_: None,
      capacity_: 0,
      occupancy_: 0,
    };
    this.initialize(capacity);
    this
  }

  #[inline]
  fn map(&self) -> &[Entry<K, V>] {
    // invalidate 后 (None) 返回空切片, 与 probe/start 的 None 语义配套
    self.map_.as_deref().unwrap_or(&[])
  }

  #[inline]
  fn map_mut(&mut self) -> &mut [Entry<K, V>] {
    debug_assert!(self.map_.is_some());
    self.map_.as_deref_mut().unwrap_or(&mut [])
  }

  /// occupancy() — 非空条目数
  #[inline]
  pub fn occupancy(&self) -> u32 {
    self.occupancy_
  }

  /// capacity() — occupancy 恒 <= 80% capacity
  #[inline]
  pub fn capacity(&self) -> u32 {
    self.capacity_
  }

  /// allocator()
  #[inline]
  pub fn allocator(&self) -> &A {
    &self.allocator_
  }

  /// impl_.match()
  #[inline]
  pub fn match_fn(&self) -> &M {
    &self.match_
  }

  /// base::bits::IsPowerOfTwo 的本地等价 (仅用于断言, 避免为 debug_assert 引入跨模块依赖)
  #[inline]
  fn is_power_of_two(value: u32) -> bool {
    value != 0 && value & (value - 1) == 0
  }

  /// Initialize(capacity): 分配并 Clear
  pub fn initialize(&mut self, capacity: u32) {
    debug_assert!(Self::is_power_of_two(capacity));
    // 原版 OOM 走 FATAL; Rust 分配失败即 abort, 此处不引入 panic 分支。
    let buf = self.allocator_.allocate_array(capacity as usize);
    self.map_ = Some(buf);
    self.capacity_ = capacity;
    self.occupancy_ = 0;
  }

  /// Clear(): memset 0 — 全部标记为空
  pub fn clear(&mut self) {
    let map = self.map_mut();
    for e in map.iter_mut() {
      *e = Entry::default();
    }
    self.occupancy_ = 0;
  }

  /// Invalidate(): 释放并置为不可用 (原版 impl_ = Impl(match, AllocationPolicy()))
  pub fn invalidate(&mut self) {
    debug_assert!(self.map_.is_some());
    let buf = take(&mut self.map_);
    self.capacity_ = 0;
    self.occupancy_ = 0;
    if let Some(buf) = buf {
      self.allocator_.delete_array(buf);
    }
  }

  /// Probe(key, hash) → 命中槽位或空槽 (原版 Entry*)
  /// 表不可用 (capacity == 0) 时返回 None, 对应原版对 Invalidate 后使用的 DCHECK 语义
  pub fn probe<LK>(&self, key: &LK, hash: u32) -> Option<usize>
  where
    M: MatchFn<LK, K>,
  {
    let hash = hash & K_PROBE_HASH_MASK;
    // 表不可用 (capacity == 0) 时先早退, 再做不变量断言 (0 非 2 的幂)
    if self.capacity_ == 0 || self.map().is_empty() {
      return None;
    }
    debug_assert!(Self::is_power_of_two(self.capacity_));
    let mask = self.capacity_ as usize - 1;
    debug_assert!(self.occupancy_ < self.capacity_); // Guarantees loop termination.
    let mut i = hash as usize & mask;
    let map = self.map();
    while map[i].exists() && !self.match_.match_fn(hash, map[i].hash(), key, &map[i].key) {
      i = (i + 1) & mask;
    }
    Some(i)
  }

  /// Lookup(key, hash) → Some(槽位) / None (原版 nullptr)
  pub fn lookup<LK>(&self, key: &LK, hash: u32) -> Option<usize>
  where
    M: MatchFn<LK, K>,
  {
    let i = self.probe(key, hash)?;
    if self.map().get(i).is_some_and(|e| e.exists()) {
      Some(i)
    } else {
      None
    }
  }

  /// FillEmptyEntry(entry, key, value, hash) → 实际槽位 (扩容后重探测)
  /// 槽位不可用返回 None (原版对空 map 写入为 FATAL)
  fn fill_empty_entry(&mut self, entry: usize, key: K, value: V, hash: u32) -> Option<usize>
  where
    M: MatchFn<K, K>,
    K: Clone,
    V: Default,
  {
    debug_assert!(!self.map().get(entry).is_some_and(|e| e.exists()));

    // 条目仅存 31 位 hash; 与 Probe 掩码一致 (原版注释)
    let hash = hash & K_HASH_VALUE_MASK;
    {
      let map = self.map_mut();
      *map.get_mut(entry)? = Entry::new(key.clone(), value, hash);
    }
    self.occupancy_ += 1;

    // Grow the map if we reached >= 80% occupancy.
    if self.occupancy_ + self.occupancy_ / 4 >= self.capacity_ {
      self.resize();
      return self.probe(&key, hash);
    }
    Some(entry)
  }

  /// Resize(): 容量翻倍并重布所有条目
  fn resize(&mut self)
  where
    M: MatchFn<K, K>,
    K: Clone,
    V: Default,
  {
    let mut old_map = take(&mut self.map_);
    let old_capacity = self.capacity_;
    let mut remaining = self.occupancy_;

    // Allocate larger map.
    self.initialize(self.capacity_.wrapping_mul(2));

    // Rehash all current entries.
    // 新容量是旧容量两倍, 重布期间 occupancy 达不到 80% 阈值,
    // 故 fill_empty_value_at 与原版 FillEmptyEntry 在此等价且不会二次扩容。
    if let Some(old_map) = old_map.as_mut() {
      for idx in 0..old_capacity as usize {
        if remaining == 0 {
          break;
        }
        if old_map[idx].exists() {
          let hash = old_map[idx].hash();
          if let Some(i) = self.probe(&old_map[idx].key, hash) {
            let key = old_map[idx].key.clone();
            let value = take(&mut old_map[idx].value);
            self.fill_empty_value_at(i, key, value, hash);
            remaining -= 1;
          }
        }
      }
    }

    // Delete old map.
    if let Some(old_map) = old_map {
      self.allocator_.delete_array(old_map);
    }
  }

  /// resize 内部填槽 (跳过扩容检查, 见 resize 注释)
  fn fill_empty_value_at(&mut self, entry: usize, key: K, value: V, hash: u32) {
    let map = self.map_mut();
    if let Some(slot) = map.get_mut(entry) {
      *slot = Entry::new(key, value, hash);
      self.occupancy_ += 1;
    }
  }

  /// LookupOrInsert(key, hash) — 默认值初始化 value (原版 []() { return Value(); })
  pub fn lookup_or_insert(&mut self, key: K, hash: u32) -> Option<usize>
  where
    M: MatchFn<K, K>,
    V: Default,
    K: Clone,
  {
    self.lookup_or_insert_with(key, hash, V::default)
  }

  /// LookupOrInsert(key, hash, value_func)
  pub fn lookup_or_insert_with(
    &mut self,
    key: K,
    hash: u32,
    value_func: impl FnOnce() -> V,
  ) -> Option<usize>
  where
    M: MatchFn<K, K>,
    K: Clone,
    V: Default,
  {
    let i = self.probe(&key, hash)?;
    if self.map().get(i).is_some_and(|e| e.exists()) {
      return Some(i);
    }
    let value = value_func();
    self.fill_empty_entry(i, key, value, hash)
  }

  /// 异质 LookupOrInsert(lookup_key, hash, key_func, value_func)
  pub fn lookup_or_insert_heterogeneous<LK>(
    &mut self,
    lookup_key: &LK,
    hash: u32,
    key_func: impl FnOnce() -> K,
    value_func: impl FnOnce() -> V,
  ) -> Option<usize>
  where
    M: MatchFn<LK, K> + MatchFn<K, K>,
    K: Clone,
    V: Default,
  {
    let i = self.probe(lookup_key, hash)?;
    if self.map().get(i).is_some_and(|e| e.exists()) {
      return Some(i);
    }
    let key = key_func();
    let value = value_func();
    self.fill_empty_entry(i, key, value, hash)
  }

  /// InsertNew(key, hash): 原版不查重, 直接 FillEmptyEntry
  pub fn insert_new(&mut self, key: K, hash: u32) -> Option<usize>
  where
    M: MatchFn<K, K>,
    V: Default,
    K: Clone,
  {
    let i = self.probe(&key, hash)?;
    self.fill_empty_entry(i, key, V::default(), hash)
  }

  /// Remove(key, hash): 未找到返回 None (原版 return nullptr)
  pub fn remove(&mut self, key: &K, hash: u32) -> Option<V>
  where
    M: MatchFn<K, K>,
    K: Clone + Default,
    V: Default,
  {
    let i = self.probe(key, hash)?;
    if !self.map().get(i).is_some_and(|e| e.exists()) {
      return None;
    }
    self.remove_at(i)
  }

  /// Remove(Entry* p): 开放寻址删除 + 前移回填, 返回被删条目 value
  /// 算法来自 http://en.wikipedia.org/wiki/Open_addressing (原版注释)
  pub fn remove_at(&mut self, p_start: usize) -> Option<V>
  where
    K: Default,
    V: Default,
  {
    debug_assert!(Self::is_power_of_two(self.capacity_));
    // This guarantees loop termination as there is at least one empty entry.
    debug_assert!(self.occupancy_ < self.capacity_);
    if self.capacity_ == 0 || !self.map().get(p_start).is_some_and(|e| e.exists()) {
      return None;
    }
    let mask = self.capacity_ as usize - 1;

    let map = self.map_mut();
    let value = take(&mut map[p_start].value);

    // p is the candidate entry to clear. q is used to scan forwards.
    let mut p = p_start;
    let mut q = p;
    loop {
      q = (q + 1) & mask;
      if !map[q].exists() {
        break;
      }
      // Find the initial position for the entry at position q.
      let r = map[q].hash() as usize & mask;
      // All entries between p and q have their initial position between p and q.
      let move_needed = (q > p && (r <= p || r > q)) || (q < p && (r <= p && r > q));
      if move_needed {
        let moved = take(&mut map[q]);
        map[p] = moved;
        p = q;
      }
    }

    // Clear the entry which is allowed to be emptied.
    map[p].clear();
    self.occupancy_ -= 1;
    Some(value)
  }

  /// Start(): 首个存在条目槽位 (原版 Next(map_ - 1))
  pub fn start(&self) -> Option<usize> {
    self.next_from(0)
  }

  /// Next(entry): 下一个存在条目
  #[inline]
  pub fn next(&self, entry: usize) -> Option<usize> {
    self.next_from(entry + 1)
  }

  fn next_from(&self, from: usize) -> Option<usize> {
    let map = self.map();
    let mut i = from;
    while i < map.len() {
      if map[i].exists() {
        return Some(i);
      }
      i += 1;
    }
    None
  }

  /// 条目只读访问 (原版 Entry* 解引用面)
  #[inline]
  pub fn entry(&self, index: usize) -> Option<&Entry<K, V>> {
    self.map().get(index).filter(|e| e.exists())
  }

  /// 条目可变访问
  #[inline]
  pub fn entry_mut(&mut self, index: usize) -> Option<&mut Entry<K, V>> {
    let map = self.map_mut();
    map.get_mut(index).filter(|e| e.exists())
  }

  /// 拷贝构造 (原版 TemplateHashMapImpl(const TemplateHashMapImpl* original, ...))
  /// memcpy 语义 → 条目按位可复制 (Entry: Copy)
  pub fn clone_from_map(&self, match_: M, allocator: A) -> Self
  where
    K: Copy,
    V: Copy,
  {
    let capacity = self.capacity_;
    let mut buf = allocator.allocate_array(capacity as usize);
    for (slot, e) in buf.iter_mut().zip(self.map().iter()) {
      *slot = *e;
    }
    Self {
      match_,
      allocator_: allocator,
      map_: Some(buf),
      capacity_: capacity,
      occupancy_: self.occupancy_,
    }
  }
}

// ---------------------------------------------------------------------------
// 便捷别名 (对应原版类族)
// ---------------------------------------------------------------------------

/// PointerTemplateHashMapImpl: 指针键值按地址比较
pub type PointerTemplateHashMapImpl<A> =
  TemplateHashMapImpl<*mut c_void, *mut c_void, KeyEqualityMatcher<*mut c_void>, A>;

/// using HashMap = PointerTemplateHashMapImpl<DefaultAllocationPolicy>;
pub type HashMap = PointerTemplateHashMapImpl<DefaultAllocationPolicy>;

impl HashMap {
  /// 原版默认容量构造
  #[inline]
  pub fn new_default() -> Self {
    Self::new(
      K_DEFAULT_HASH_MAP_CAPACITY,
      KeyEqualityMatcher::default(),
      DefaultAllocationPolicy,
    )
  }
}

/// CustomMatcherTemplateHashMapImpl: void* 键值 + 自定义比较函数指针
pub type CustomMatchFn = fn(*mut c_void, *mut c_void) -> bool;
pub type CustomMatcherTemplateHashMapImpl<A> = TemplateHashMapImpl<
  *mut c_void,
  *mut c_void,
  HashEqualityThenKeyMatcher<*mut c_void, CustomMatchFn>,
  A,
>;

/// using CustomMatcherHashMap = CustomMatcherTemplateHashMapImpl<DefaultAllocationPolicy>;
pub type CustomMatcherHashMap = CustomMatcherTemplateHashMapImpl<DefaultAllocationPolicy>;

impl CustomMatcherHashMap {
  /// 原版 ctor(MatchFun match, capacity = kDefaultHashMapCapacity, ...)
  #[inline]
  pub fn with_matcher(match_: CustomMatchFn) -> Self {
    Self::new(
      K_DEFAULT_HASH_MAP_CAPACITY,
      HashEqualityThenKeyMatcher::new(match_),
      DefaultAllocationPolicy,
    )
  }
}

/// TemplateHashMap<Key, Value, MatchFun, AllocationPolicy>
/// 原版为 private 继承 Base<void*, void*, HashEqualityThenKeyMatcher<...>>
/// + reinterpret_cast 的 value_type {Key* first; Value* second} 视图。
pub struct TemplateHashMap<K, V, M, A: AllocationPolicy<Entry<*mut c_void, *mut c_void>>> {
  base: TemplateHashMapImpl<*mut c_void, *mut c_void, M, A>,
  _marker: PhantomData<(K, V)>,
}

// static_assert 等价 (原版 sizeof(Key*) == sizeof(void*)) 见 impl 内 _SIZE_ASSERT

impl<K, V, M, A> TemplateHashMap<K, V, M, A>
where
  M: MatchFn<*mut c_void, *mut c_void>,
  A: AllocationPolicy<Entry<*mut c_void, *mut c_void>>,
  Entry<*mut c_void, *mut c_void>: Default,
{
  /// static_assert(sizeof(Key*) == sizeof(void*));
  /// static_assert(sizeof(Value*) == sizeof(void*));
  const _SIZE_ASSERT: () = {
    assert!(size_of::<*mut K>() == size_of::<*mut c_void>());
    assert!(size_of::<*mut V>() == size_of::<*mut c_void>());
  };

  /// explicit TemplateHashMap(MatchFun match, ...) — 以已构造 base 承接
  #[inline]
  pub fn new(base: TemplateHashMapImpl<*mut c_void, *mut c_void, M, A>) -> Self {
    let () = Self::_SIZE_ASSERT;
    Self {
      base,
      _marker: PhantomData,
    }
  }

  /// begin() → 首条目槽位 (原版 entry_ = Start())
  #[inline]
  pub fn begin(&self) -> Option<usize> {
    self.base.start()
  }

  /// Iterator::operator++
  #[inline]
  pub fn next(&self, entry: usize) -> Option<usize> {
    self.base.next(entry)
  }

  /// find(Key* key, bool insert) → 槽位 / None (原版 end(); 原版 hash = key->Hash())
  #[inline]
  pub fn find(&mut self, key: *mut K, hash: u32, insert: bool) -> Option<usize> {
    let raw = key.cast::<c_void>();
    if insert {
      // 原版 LookupOrInsert(key, key->Hash()): 空 value 即 nullptr
      self.base.lookup_or_insert(raw, hash)
    } else {
      self.base.lookup(&raw, hash)
    }
  }

  /// value_type* operator-> 的类型化视图
  pub fn entry_pair(&self, index: usize) -> Option<(*mut K, *mut V)> {
    let e = self.base.entry(index)?;
    // SAFETY: 条目为 #[repr(C)] { key: *mut c_void, value: *mut c_void, .. },
    // 与原版 value_type { Key* first; Value* second; } 前两字段布局一致;
    // 存在性由 entry() 保证, K/V 类型由调用方与插入时一致保证。
    Some((e.key.cast::<K>(), e.value.cast::<V>()))
  }

  /// 只读内部 impl (等价 private 继承的可见性)
  #[inline]
  pub fn inner(&self) -> &TemplateHashMapImpl<*mut c_void, *mut c_void, M, A> {
    &self.base
  }

  /// 可变内部 impl
  #[inline]
  pub fn inner_mut(&mut self) -> &mut TemplateHashMapImpl<*mut c_void, *mut c_void, M, A> {
    &mut self.base
  }
}

/// NoHashMapValue 集合标记 (原版 struct NoHashMapValue {})
pub type NoValue = NoHashMapValue;
