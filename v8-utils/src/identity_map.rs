// 由 tool/src/utils/consts.js 从 v8/src/utils/identity-map.h;src/utils/identity-map.cc 生成; 修订就地改, 复跑覆盖

//! 身份映射 (地址 → 值), 移植自 v8/src/utils/identity-map。
//!
//! 上游键为堆对象地址, not_mapped 哨兵取 ReadOnlyRoots::not_mapped_symbol 地址;
//! Rust 版独立运行, 哨兵取 `NOT_MAPPED` (0 地址, 真实指针恒非 0)。
//! GC 失效 (invalidated) / 惰性 rehash / 80% 负载扩容 / 删除回填语义 1:1 保留。

use std::mem::replace;

use crate::utils::hash64;

/// 初始桶数 (kInitialIdentityMapSize)
const INITIAL_SIZE: usize = 4;
/// 扩缩因子 (kResizeFactor)
const RESIZE_FACTOR: usize = 2;
/// not_mapped 哨兵: 上游取 not_mapped_symbol 堆地址; Rust 版取 u64::MAX —
/// 真实堆指针不可达 (x64 用户态 < 2^47), 且不与 Smi(0) 编码 (全 0) 冲突。
const NOT_MAPPED: u64 = u64::MAX;

/// `IdentityMapFindResult` (identity-map.h): entry 指针 → 可变引用。
pub struct FindResult<'a, V> {
  /// 是否已存在 (false = 新插入槽)。
  pub already_exists: bool,
  /// 值存储位。
  pub entry: &'a mut V,
}

/// 迭代作用域 (`IterateScope`, identity-map.h): 借用期禁插入/删除
/// (原版 EnableIteration/DisableIteration 的 Rust 等价)。
pub struct IterScope<'a, V> {
  map: &'a mut IdentityMap<V>,
}

/// `IdentityMap` (identity-map.h:85): 开放寻址 (线性探测) 身份映射。
#[derive(Debug)]
pub struct IdentityMap<V> {
  keys: Vec<u64>,
  values: Vec<Option<V>>,
  size: usize,
  invalidated: bool,
}

impl<V> Default for IdentityMap<V> {
  fn default() -> Self {
    Self::new()
  }
}

impl<V> IdentityMap<V> {
  /// 默认构造: 零桶零元素。
  pub const fn new() -> Self {
    Self {
      keys: Vec::new(),
      values: Vec::new(),
      size: 0,
      invalidated: false,
    }
  }

  /// 桶数 (capacity_)。
  #[must_use]
  pub fn capacity(&self) -> usize {
    self.keys.len()
  }

  /// 元素数 (size_)。
  #[must_use]
  pub fn len(&self) -> usize {
    self.size
  }

  /// 空表 (上游 empty() 判 size_ == 0)。
  #[must_use]
  pub fn empty(&self) -> bool {
    self.size == 0
  }

  /// `empty` 的 Rust 惯用名。
  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.empty()
  }

  /// `Hash(address)` (identity-map.cc:172): base::hash<uintptr_t> → hash64。
  fn hash(address: u64) -> u32 {
    debug_assert_ne!(address, NOT_MAPPED);
    hash64(address) as u32
  }

  fn mask(&self) -> usize {
    self.keys.len() - 1
  }

  /// `ShouldGrow` (identity-map.cc:122): 负载 >= 80%。
  fn should_grow(&self) -> bool {
    self.size + self.size / 4 >= self.keys.len()
  }

  /// `ScanKeysFor` (identity-map.cc:102): 环形线性扫描,
  /// 命中返回 (idx, true), 空槽返回 (idx, false) (可作插入位)。
  fn scan_keys_for(&self, address: u64, hash: u32) -> (usize, bool) {
    let start = (hash as usize) & self.mask();
    for index in (start..self.keys.len()).chain(0..start) {
      if self.keys[index] == address {
        return (index, true);
      }
      if self.keys[index] == NOT_MAPPED {
        return (index, false);
      }
    }
    // size < capacity 保证必有空槽, 不可达
    (self.keys.len(), false)
  }

  /// `InsertKey` (identity-map.cc:130): 按需扩容后线性探测插入。
  fn insert_key(&mut self, address: u64, hash: u32) -> (usize, bool) {
    if self.should_grow() {
      self.resize(self.keys.len() * RESIZE_FACTOR);
    }
    let mut index = (hash as usize) & self.mask();
    loop {
      if self.keys[index] == address {
        return (index, true);
      }
      if self.keys[index] == NOT_MAPPED {
        self.size += 1;
        self.keys[index] = address;
        self.values[index] = None;
        return (index, false);
      }
      index = (index + 1) & self.mask();
    }
  }

  /// `Lookup` (identity-map.cc:191): 查找; invalidated 时惰性 rehash 后重查。
  fn lookup(&mut self, key: u64) -> Option<usize> {
    let hash = Self::hash(key);
    let (mut index, mut found) = self.scan_keys_for(key, hash);
    if !found && self.invalidated {
      self.rehash();
      (index, found) = self.scan_keys_for(key, hash);
    }
    found.then_some(index)
  }

  /// `LookupOrInsert` (identity-map.cc:205): 乐观查找; miss 时 rehash + 插入。
  fn lookup_or_insert(&mut self, key: u64) -> (usize, bool) {
    let hash = Self::hash(key);
    let (mut index, mut already_exists) = self.scan_keys_for(key, hash);
    if !already_exists {
      if self.invalidated {
        self.rehash();
        index = usize::MAX; // 原版 index = -1: 强制走 InsertKey
      }
      if index == usize::MAX || self.should_grow() {
        (index, already_exists) = self.insert_key(key, hash);
      } else {
        // 已找到合适空槽: 免二次扫描 (identity-map.cc:225)
        self.size += 1;
        debug_assert_eq!(self.keys[index], NOT_MAPPED);
        self.keys[index] = key;
        self.values[index] = None;
      }
    }
    (index, already_exists)
  }

  /// `InsertEntry` (identity-map.cc:258): 首插初始化桶数组后插入。
  fn insert_entry(&mut self, key: u64) -> usize {
    if self.keys.is_empty() {
      self.keys = vec![NOT_MAPPED; INITIAL_SIZE];
      self.values = (0..INITIAL_SIZE).map(|_| None).collect();
      self.invalidated = false;
    } else if self.invalidated {
      self.rehash();
    }
    let (index, already) = self.insert_key(key, Self::hash(key));
    debug_assert!(!already);
    index
  }

  /// `FindEntry` (identity-map.cc:249)。
  pub fn find(&mut self, key: u64) -> Option<&V> {
    if self.size == 0 {
      return None;
    }
    let index = self.lookup(key)?;
    self.values[index].as_ref()
  }

  /// `FindOrInsertEntry` (identity-map.cc:237)。
  ///
  /// 新插槽先置 `V::default()` 再借用 (原版返回未初始化槽由调用方写,
  /// Rust 无未初始化合法引用形态; 测试从不在写前读, 可观察行为一致)。
  pub fn find_or_insert(&mut self, key: u64) -> FindResult<'_, V>
  where
    V: Default,
  {
    if self.keys.is_empty() {
      let index = self.insert_entry(key);
      self.values[index] = Some(V::default());
      return FindResult {
        already_exists: false,
        entry: self.values[index].as_mut().expect("刚置默认值, 必为 Some"),
      };
    }
    let (index, already_exists) = self.lookup_or_insert(key);
    if self.values[index].is_none() {
      self.values[index] = Some(V::default());
    }
    FindResult {
      already_exists,
      entry: self.values[index].as_mut().expect("占用槽位值存在"),
    }
  }

  /// 便捷插入 (上游测试用 Insert(key, value))。
  pub fn insert(&mut self, key: u64, val: V)
  where
    V: Default,
  {
    let r = self.find_or_insert(key);
    *r.entry = val;
  }

  /// `DeleteEntry` (identity-map.cc:272): 命中删除, 输出旧值。
  pub fn delete(&mut self, key: u64, deleted_value: Option<&mut V>) -> bool {
    if self.size == 0 {
      return false;
    }
    let Some(index) = self.lookup(key) else {
      return false;
    };
    // DeleteIndex (identity-map.cc:144)
    if let Some(out) = deleted_value {
      *out = self.values[index].take().expect("命中槽位值存在");
    } else {
      self.values[index] = None;
    }
    self.keys[index] = NOT_MAPPED;
    self.size -= 1;

    if self.keys.len() > INITIAL_SIZE && self.size * RESIZE_FACTOR < self.keys.len() / RESIZE_FACTOR
    {
      self.resize(self.keys.len() / RESIZE_FACTOR);
      return true; // resize 已重排, 免回填
    }

    // 碰撞回填 (identity-map.cc:163-181)
    let mut index = index;
    let mut next_index = index;
    loop {
      next_index = (next_index + 1) & self.mask();
      let key = self.keys[next_index];
      if key == NOT_MAPPED {
        break;
      }
      let expected_index = (Self::hash(key) as usize) & self.mask();
      if index < next_index {
        if index < expected_index && expected_index <= next_index {
          continue;
        }
      } else if index < expected_index || expected_index <= next_index {
        continue;
      }
      self.keys.swap(index, next_index);
      self.values.swap(index, next_index);
      index = next_index;
    }
    true
  }

  /// `IterateScope`: 迭代作用域 (identity-map.h)。
  pub fn iter_scope(&mut self) -> IterScope<'_, V> {
    IterScope { map: self }
  }

  /// 测试钩子: SimulateGCByIncrementingSmisBy (identity-map-unittest.cc) —
  /// Smi 键 (低位 0) 高 32 位加 shift 后标记 invalidated。
  pub fn simulate_gc_by_incrementing_smis_by(&mut self, shift: i64) {
    let delta = u64::try_from(shift).unwrap_or(0).wrapping_shl(32);
    for key in &mut self.keys {
      if *key & 1 == 0 {
        // Smi 键: HasHeapObjectTag 为假 (低 32 位全 0)
        *key = key.wrapping_add(delta);
      }
    }
    self.invalidated = true;
  }

  /// 测试钩子: 显式 rehash。
  pub fn rehash_for_testing(&mut self) {
    self.rehash();
  }

  /// 测试钩子: 显式 resize。
  pub fn resize_for_testing(&mut self, new_capacity: usize) {
    self.resize(new_capacity);
  }

  /// `Rehash` (identity-map.cc:301): 疏散错位键后重插。
  fn rehash(&mut self) {
    self.invalidated = false;
    let mut reinsert: Vec<(u64, V)> = Vec::new();
    let mut last_empty: isize = -1;
    for i in 0..self.keys.len() {
      if self.keys[i] == NOT_MAPPED {
        last_empty = i as isize;
      } else {
        let pos = (Self::hash(self.keys[i]) as usize) & self.mask();
        if pos as isize <= last_empty || pos > i {
          let value = self.values[i].take().expect("占用槽位值存在");
          reinsert.push((self.keys[i], value));
          self.keys[i] = NOT_MAPPED;
          self.size -= 1;
          last_empty = i as isize;
        }
      }
    }
    for (key, value) in reinsert {
      let (index, _) = self.insert_key(key, Self::hash(key));
      self.values[index] = Some(value);
    }
  }

  /// `Resize` (identity-map.cc:335): 换桶重插全部键值。
  fn resize(&mut self, new_capacity: usize) {
    debug_assert!(new_capacity > self.size);
    let old_keys = replace(&mut self.keys, vec![NOT_MAPPED; new_capacity]);
    let mut old_values = replace(&mut self.values, (0..new_capacity).map(|_| None).collect());
    self.size = 0;
    self.invalidated = false;
    for (i, &key) in old_keys.iter().enumerate() {
      if key == NOT_MAPPED {
        continue;
      }
      let (index, _) = self.insert_key(key, Self::hash(key));
      self.values[index] = old_values[i].take();
    }
  }
}

impl<V> IterScope<'_, V> {
  /// 遍历全部占用槽 (IdentityMapBase::Iterate + 测试侧迭代)。
  pub fn for_each(self, mut f: impl FnMut(u64, &mut V)) {
    let map = self.map;
    for i in 0..map.keys.len() {
      if map.keys[i] != NOT_MAPPED
        && let Some(v) = map.values[i].as_mut()
      {
        f(map.keys[i], v);
      }
    }
  }
}
