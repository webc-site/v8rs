// 由 tool/src/compiler/turboshaft/layered-hash-map.h.js 从 v8/src/compiler/turboshaft/layered-hash-map.h 生成; 已收编 (W11TS2), 复跑产物冲突以本体现态为准
// 分层哈希表: 支配树分层的 map/set 骨架

use core::mem::take;

use crate::fast_hash::FastHash;

/// 原版 LayeredHashMap::kNeedResizePercentage — 触发扩容的占用比
pub const NEED_RESIZE_PERCENTAGE: f64 = 0.75;
/// 原版 LayeredHashMap::kGrowthFactor — 扩容倍数
pub const GROWTH_FACTOR: usize = 2;

/// 原版 ctor 内最小容量 16 (std::max<uint32_t>(initial_capacity, 16))
const MIN_CAPACITY: usize = 16;

/// 原版 ctor 默认实参 initial_capacity = 64
const DEFAULT_INITIAL_CAPACITY: usize = 64;

/// 原版 nullptr 哨兵 — 层链/桶链尾
const NO_ENTRY: usize = usize::MAX;

/// 原版 LayeredHashMap::Entry — hash/key/value + depth_neighboring_entry 同层链。
/// Rust 侧以表内索引代替裸指针 (NO_ENTRY = nullptr)。
/// v8: src/compiler/turboshaft/layered-hash-map.h LayeredHashMap::Entry
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entry<K, V> {
  /// hash == 0 即空槽 (ComputeHash 保证非零)
  pub hash: u64,
  pub key: K,
  pub value: V,
  /// 原版 Entry* depth_neighboring_entry — 同层下一条目
  pub next_in_layer: usize,
}

impl<K: Default, V: Default> Default for Entry<K, V> {
  fn default() -> Self {
    Self {
      hash: 0,
      key: K::default(),
      value: V::default(),
      next_in_layer: NO_ENTRY,
    }
  }
}

/// 原版 LayeredHashMap — 元素按层分组, 丢弃最后一层全部插入极快
/// (StartLayer / DropLastLayer)。不支持重复键插入, 不支持原地更新:
/// 需先 DropLastLayer 到键消失再重插。
/// 实现: table 开放寻址 (hash==0 空槽), 每层一条 depth 链;
/// ResizeIfNeeded 按深度升序重灌, 保证 DropLastLayer 不产生探测空洞。
/// Rust 侧 Zone 推迟 (台账), 载体取 Vec/usize 索引。
/// v8: src/compiler/turboshaft/layered-hash-map.h LayeredHashMap
pub struct LayeredHashMap<K, V> {
  /// 原版 base::Vector<Entry> table_ (zone 分配)
  table: Vec<Entry<K, V>>,
  /// 原版 mask_ = capacity - 1 (capacity 为 2 的幂)
  mask: usize,
  /// 原版 entry_count_
  entry_count: usize,
  /// 原版 ZoneVector<Entry*> depths_heads_ — 每层层链头
  depths_heads: Vec<usize>,
}

impl<K: Copy + Default + PartialEq + FastHash, V: Copy + Default> LayeredHashMap<K, V> {
  /// 原版 ctor(zone, initial_capacity = 64) — 最小 16, 向上取 2 的幂
  /// v8: src/compiler/turboshaft/layered-hash-map.h LayeredHashMap::LayeredHashMap
  #[must_use]
  pub fn with_capacity(initial_capacity: usize) -> Self {
    // 原版 std::bit_ceil — Rust next_power_of_two 等价
    let capacity = initial_capacity.max(MIN_CAPACITY).next_power_of_two();
    Self {
      table: vec![Entry::default(); capacity],
      mask: capacity - 1,
      entry_count: 0,
      depths_heads: Vec::new(),
    }
  }

  /// 原版默认实参 initial_capacity = 64
  #[must_use]
  pub fn new() -> Self {
    Self::with_capacity(DEFAULT_INITIAL_CAPACITY)
  }

  /// 原版 void StartLayer() — 支配树新层 (层链头置 nullptr)
  /// v8: src/compiler/turboshaft/layered-hash-map.h LayeredHashMap::StartLayer
  pub fn start_layer(&mut self) {
    self.depths_heads.push(NO_ENTRY);
  }

  /// 原版 void DropLastLayer() — 回溯沿同层链逐条清空 (hash=0 即删除) 后弃层
  /// v8: src/compiler/turboshaft/layered-hash-map.h LayeredHashMap::DropLastLayer
  pub fn drop_last_layer(&mut self) {
    debug_assert!(
      !self.depths_heads.is_empty(),
      "DCHECK_GT(depths_heads_.size(), 0)"
    );
    let Some(mut entry) = self.depths_heads.pop() else {
      return;
    };
    while entry != NO_ENTRY {
      self.entry_count -= 1;
      let next = self.table[entry].next_in_layer;
      // 原版 *entry = Entry() — 置空槽 (hash=0)
      self.table[entry] = Entry::default();
      entry = next;
    }
  }

  /// 原版 size_t NextEntryIndex(index) = (index + 1) & mask_
  #[inline]
  const fn next_entry_index(&self, index: usize) -> usize {
    (index + 1) & self.mask
  }

  /// 原版 Entry* FindEntryForKey(key, hash) — 命中返回条目, 否则首个空槽
  /// v8: src/compiler/turboshaft/layered-hash-map.h LayeredHashMap::FindEntryForKey
  fn find_entry_for_key(&self, key: &K, hash: u64) -> usize {
    let mut i = hash as usize & self.mask;
    loop {
      let e = &self.table[i];
      if e.hash == 0 {
        return i;
      }
      if e.hash == hash && e.key == *key {
        return i;
      }
      i = self.next_entry_index(i);
    }
  }

  /// 原版 size_t ComputeHash(key) — fast_hash, 0 修正为 1 (0 是空槽哨兵)
  /// v8: src/compiler/turboshaft/layered-hash-map.h LayeredHashMap::ComputeHash
  fn compute_hash(key: &K) -> u64 {
    let hash = key.fast_hash();
    if hash == 0 { 1 } else { hash }
  }

  /// 原版 void InsertNewKey(key, value) — 前置: 键不存在且已有当前层
  /// v8: src/compiler/turboshaft/layered-hash-map.h LayeredHashMap::InsertNewKey
  pub fn insert_new_key(&mut self, key: K, value: V) {
    // 原版 depths_heads_.back() 在无当前层时为 C++ UB; 此处按前置断言显式化
    debug_assert!(
      !self.depths_heads.is_empty(),
      "DCHECK: InsertNewKey 需先 StartLayer"
    );
    self.resize_if_needed();
    let hash = Self::compute_hash(&key);
    let destination = self.find_entry_for_key(&key, hash);
    debug_assert_eq!(
      self.table[destination].hash, 0,
      "DCHECK_EQ(destination->hash, 0)"
    );
    // 原版 depths_heads_.back(): 未 StartLayer 时 C++ 为 UB; Rust 侧按无层链头处理
    let head = self.depths_heads.last().copied().unwrap_or(NO_ENTRY);
    self.table[destination] = Entry {
      hash,
      key,
      value,
      next_in_layer: head,
    };
    if let Some(last) = self.depths_heads.last_mut() {
      *last = destination;
    }
    self.entry_count += 1;
  }

  /// 原版 std::optional<Value> Get(key)
  /// v8: src/compiler/turboshaft/layered-hash-map.h LayeredHashMap::Get
  pub fn get(&self, key: &K) -> Option<V> {
    let destination = self.find_entry_for_key(key, Self::compute_hash(key));
    let e = &self.table[destination];
    (e.hash != 0).then_some(e.value)
  }

  /// 原版 bool Contains(key) = Get(key).has_value()
  /// v8: src/compiler/turboshaft/layered-hash-map.h LayeredHashMap::Contains
  pub fn contains(&self, key: &K) -> bool {
    self.get(key).is_some()
  }

  /// 原版 void ResizeIfNeeded() — 占用比达 0.75 时 2× 扩容并按深度升序重灌。
  /// 重灌序保证: 同 hash 条目深度升序落位, DropLastLayer 不产生探测空洞
  /// (原版注释: 否则 a1/a2 会被 null 隔断而查不到)。
  /// v8: src/compiler/turboshaft/layered-hash-map.h LayeredHashMap::ResizeIfNeeded
  pub fn resize_if_needed(&mut self) {
    // 原版 table_.size() * 0.75 > entry_count_ → return; 整数精确等价式
    if self.table.len() * 3 > self.entry_count * 4 {
      return;
    }
    // 原版 CHECK_LE(table_.size(), max / kGrowthFactor) — debug 面显式化前置,
    // release 面本行等价「Vec 分配前必先 OOM」故 2× 溢出不可达; 若日后引入
    // 大表压力测试或 GROWTH_FACTOR 改动, 此断言将先于乘法暴露越界。
    debug_assert!(
      self.table.len() <= usize::MAX / GROWTH_FACTOR,
      "原版 ResizeIfNeeded CHECK_LE(size, max/kGrowthFactor) — 2× 溢出前置"
    );
    let old = take(&mut self.table);
    let new_len = old.len() * GROWTH_FACTOR;
    self.table = vec![Entry::default(); new_len];
    self.mask = new_len - 1;
    let mut heads = take(&mut self.depths_heads);
    for head in heads.iter_mut() {
      let mut entry = *head;
      *head = NO_ENTRY;
      while entry != NO_ENTRY {
        let e = old[entry];
        let new_entry_loc = self.find_entry_for_key(&e.key, e.hash);
        // 链头前插, 维持同 hash 深度升序
        self.table[new_entry_loc] = Entry {
          hash: e.hash,
          key: e.key,
          value: e.value,
          next_in_layer: *head,
        };
        *head = new_entry_loc;
        entry = old[entry].next_in_layer;
      }
    }
    self.depths_heads = heads;
  }
}

/// 原版默认实参面 (=64) 的 Rust Default 对应
impl<K: Copy + Default + PartialEq + FastHash, V: Copy + Default> Default for LayeredHashMap<K, V> {
  fn default() -> Self {
    Self::new()
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(NEED_RESIZE_PERCENTAGE == 0.75);
  assert!(GROWTH_FACTOR == 2);
  assert!(MIN_CAPACITY == 16);
};
