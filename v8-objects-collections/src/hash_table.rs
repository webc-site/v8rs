// 由 tool/src/objects/hash-table.h.js 从 v8/src/objects/hash-table.{h,-inl.h} + 跨文件锚 objects.cc/fixed-array-base.h 生成; 修订就地改, 复跑覆盖
// 开放寻址二次探测哈希表基座 (Shape 参数化规则面)

//! HashTable 家族规则面: HashTableBase / HashTable<Derived, Shape>。
//!
//! - 空槽 = undefined, 删除槽 = the_hole; 探测遇 undefined 停, 遇 the_hole 继续
//! - 容量恒 2 的幂 (bit_ceil + 50% 松弛, 下限 kMinCapacity)
//! - 大表哈希分散 (kDoHashSpreading): hash * (size >> kHashBits) & (size-1)
//!
//! 槽位语义 (Smi 计数字段): [0]=元素数 [1]=删除数 [2]=容量 [3..3+prefix)=前缀。

use std::{mem, ptr::write, sync::atomic::{AtomicUsize, Ordering}};

use v8_objects_arrays::fixed_array::{FixedArray, TaggedArray};
use v8_objects_core::{
  map::Map,
  smi::Smi,
  tagged::{Object, Tagged},
  tagged_constants::{HEAP_OBJECT_TAG_MASK, WEAK_HEAP_OBJECT_TAG},
};

use crate::{
  allocator::MemoryAllocator,
  error::Result,
  internal_index::InternalIndex,
  ordered_hash_table::{THE_HOLE, compute_object_hash, is_the_hole},
};

// ── HashTableBase 槽位索引 (hash-table.h:103-109) ─────────────────────
/// 元素计数槽。
pub const K_NUMBER_OF_ELEMENTS_INDEX: usize = 0;
/// 删除元素计数槽。
pub const K_NUMBER_OF_DELETED_ELEMENTS_INDEX: usize = 1;
/// 容量槽。
pub const K_CAPACITY_INDEX: usize = 2;
/// 前缀起始槽 (前缀归各派生 Shape 使用)。
pub const K_PREFIX_START_INDEX: usize = 3;
/// 新表最小容量 (kMinCapacity)。
pub const K_MIN_CAPACITY: u32 = 4;

// ── HashTable 容量常量 (hash-table.h:201-220) ────────────────────────
/// 键槽在 entry 内的下标。
pub const K_ENTRY_KEY_INDEX: usize = 0;
/// 不收缩到该容量以下 (kMinShrinkCapacity)。
pub const K_MIN_SHRINK_CAPACITY: u32 = 16;
/// 超过该容量转为老年代分配 (kMinCapacityForPretenure)。
pub const K_MIN_CAPACITY_FOR_PRETENURE: u32 = 256;
/// 普通对象区最大容量 = kMaxRegularHeapObjectSize / 32 (默认页配置 kPageSizeBits=18)。
pub const K_MAX_REGULAR_CAPACITY: u32 = 4096;
/// 底层 FixedArray 最大长度 (kMaxFixedArrayCapacity, 非 LOWER_LIMITS 配置)。
pub const K_FIXED_ARRAY_MAX_LENGTH: u32 = 134217728;

/// 计算至少容纳 at_least_space_for 的容量 (hash-table-inl.h:88 ComputeCapacity)。
///
/// 加 50% 松弛 + 2 的幂上取整, 下限 kMinCapacity;
/// 与 CodeStubAssembler::HashTableComputeCapacity 保持同步 (原版注释)。
#[inline]
#[must_use]
pub const fn compute_capacity(at_least_space_for: u32) -> u32 {
  let raw_cap = at_least_space_for + (at_least_space_for >> 1);
  let capacity = raw_cap.next_power_of_two();
  if capacity < K_MIN_CAPACITY { K_MIN_CAPACITY } else { capacity }
}

/// 下一探测位置: 二次探测 (hash-table.h:122 NextProbe)。
#[inline]
#[must_use]
pub const fn next_probe(last: u32, number: u32, size: u32) -> u32 {
  (last.wrapping_add(number)) & (size - 1)
}

/// 首探测位置 (hash-table.h:163 FirstProbe)。
///
/// 未启用哈希分散或 size ≤ 2^kHashBits 时直接取模;
/// 大表 + 哈希位不足时按 coefficient = size >> kHashBits 放大分散
/// (与 WeakCollectionsBuiltinsAssembler 生成的代码同步, 原版注释)。
#[inline]
#[must_use]
pub const fn first_probe(hash: u32, size: u32, do_hash_spreading: bool, hash_bits: u32) -> u32 {
  if !do_hash_spreading || size <= (1u32 << hash_bits) {
    return hash & (size - 1);
  }
  let coefficient = size >> hash_bits;
  (hash.wrapping_mul(coefficient)) & (size - 1)
}

/// entry → 槽下标 (hash-table.h:223 EntryToIndex)。
#[inline]
#[must_use]
pub const fn entry_to_index(entry: u32, entry_size: usize, elements_start_index: usize) -> usize {
  (entry as usize) * entry_size + elements_start_index
}

/// 槽下标 → entry (hash-table.h:228 IndexToEntry)。
#[inline]
#[must_use]
pub const fn index_to_entry(index: usize, entry_size: usize, elements_start_index: usize) -> u32 {
  ((index - elements_start_index) / entry_size) as u32
}

// ── Shape 规则面 (hash-table.h 各 Shape 展开) ────────────────────────
/// Shape 静态参数 (C++ Shape 类的关联常量集)。
#[derive(Copy, Clone, Debug)]
pub struct ShapeParams {
  /// 前缀槽数 (kPrefixSize)。
  pub prefix_size: usize,
  /// 每 entry 槽数 (kEntrySize)。
  pub entry_size: usize,
  /// IsMatch 是否需处理 the_hole 删除槽 (kMatchNeedsHoleCheck)。
  pub match_needs_hole_check: bool,
  /// 大表哈希分散开关 (kDoHashSpreading)。
  pub do_hash_spreading: bool,
  /// 哈希可用位数 (kHashBits; 0 = 不分散)。
  pub hash_bits: u32,
}

/// ObjectHashTableShapeBase (键 = 任意对象, 身份哈希)。
pub const OBJECT_HASH_TABLE_SHAPE_BASE: ShapeParams = ShapeParams {
  prefix_size: 0,
  entry_size: 2,
  match_needs_hole_check: false,
  do_hash_spreading: false,
  hash_bits: 0,
};
/// ObjectHashTableShape。
pub const OBJECT_HASH_TABLE_SHAPE: ShapeParams = OBJECT_HASH_TABLE_SHAPE_BASE;
/// EphemeronHashTableShape (WeakMap 大表分散探测; kHashBits=10 定义于 objects.cc)。
pub const EPHEMERON_HASH_TABLE_SHAPE: ShapeParams = ShapeParams {
  do_hash_spreading: true,
  hash_bits: 10,
  ..OBJECT_HASH_TABLE_SHAPE_BASE
};
/// ObjectHashSetShape (集合: 每 entry 1 槽)。
pub const OBJECT_HASH_SET_SHAPE: ShapeParams = ShapeParams {
  entry_size: 1,
  ..OBJECT_HASH_TABLE_SHAPE_BASE
};
/// NameToIndexShape (Name → 索引)。
pub const NAME_TO_INDEX_SHAPE: ShapeParams = OBJECT_HASH_TABLE_SHAPE_BASE;
/// RegisteredSymbolTableShape (描述符字符串 → Symbol)。
pub const REGISTERED_SYMBOL_TABLE_SHAPE: ShapeParams = OBJECT_HASH_TABLE_SHAPE_BASE;
/// ObjectMultiHashTableShape<N>: kEntrySize = 1 + N (每键 N 值, N>1)。
#[inline]
#[must_use]
pub const fn object_multi_hash_table_entry_size(n: usize) -> usize {
  1 + n
}
/// ObjectTwoHashTable (N = 2)。
pub const OBJECT_TWO_HASH_TABLE_ENTRY_SIZE: usize = object_multi_hash_table_entry_size(2);

/// kElementsStartIndex = kPrefixStartIndex + Shape::kPrefixSize (hash-table.h:201)。
#[inline]
#[must_use]
pub const fn elements_start_index(prefix_size: usize) -> usize {
  K_PREFIX_START_INDEX + prefix_size
}

/// 最大容量 = bit_floor((FixedArray::kMaxLength - kElementsStartIndex) / kEntrySize)
/// (hash-table.h:211; 保证 EntryToIndex 不溢出)。
#[inline]
#[must_use]
pub const fn max_capacity(prefix_size: usize, entry_size: usize) -> u32 {
  ((K_FIXED_ARRAY_MAX_LENGTH - elements_start_index(prefix_size) as u32) / entry_size as u32)
    .next_power_of_two()
    >> 1
}

// ── 堆访问面: 仿真堆上的类型化泛型实现 (就地修订完成) ──────────────────
/// 空槽哨兵 (对应 roots.undefined_value; 与 THE_HOLE 删除槽哨兵区分)。
pub const UNDEFINED: Tagged<Object> =
  Tagged::new((usize::MAX & !HEAP_OBJECT_TAG_MASK) | WEAK_HEAP_OBJECT_TAG);

/// 是否空槽 (undefined)。
#[inline]
#[must_use]
pub const fn is_undefined(obj: Tagged<Object>) -> bool {
  obj.ptr() == UNDEFINED.ptr()
}

/// 槽内键有效性 (hash-table-inl.h HashTable::IsKey: 非 undefined 且非 the_hole)。
#[inline]
#[must_use]
pub fn is_key(k: Tagged<Object>) -> bool {
  !is_undefined(k) && !is_the_hole(k)
}

/// HashTable 堆对象视图 (FixedArray 基座; 派生类 EphemeronHashTable 等共用布局)。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct HashTable {
  pub base: FixedArray,
}

impl HashTable {
  /// 底层数组总槽数 = 元素起始 + 容量 × entry 槽数 (hash-table-inl.h NewInternal 的 EntryToIndex(capacity))。
  #[inline]
  #[must_use]
  pub const fn length(shape: ShapeParams, capacity: u32) -> usize {
    elements_start_index(shape.prefix_size) + capacity as usize * shape.entry_size
  }

  /// 分配新表: 全槽填 undefined, 计数清零, 写容量 (空槽填充物与 NewFixedArrayWithMap 一致)。
  ///
  /// v8: src/objects/hash-table-inl.h HashTable::NewInternal
  pub fn new<A: MemoryAllocator>(
    allocator: &mut A,
    map: Tagged<Map>,
    shape: ShapeParams,
    capacity: u32,
  ) -> Result<Tagged<Self>> {
    // SetCapacity 契约: 容量须为正 2 的幂 (探测掩码依赖)
    debug_assert!(capacity > 0 && capacity.is_power_of_two());
    debug_assert!(capacity <= max_capacity(shape.prefix_size, shape.entry_size));
    let slots = Self::length(shape, capacity);
    let size = FixedArray::size_for(slots as u32);
    let addr = allocator.allocate(size)?;
    let header = FixedArray::new(map, slots as u32);
    // SAFETY: addr 由分配器成功分配且按 usize 对齐, size 字节已保留;
    // 先写 FixedArray 头部, 再逐槽初始化对象体, 之后内存即持有有效 HashTable 对象
    unsafe {
      write(addr as *mut FixedArray, header);
      let table: Tagged<Self> = Tagged::from_raw_address(addr);
      for slot in 0..slots {
        table.write_slot(slot, UNDEFINED);
      }
      table.set_number_of_elements(0);
      table.set_number_of_deleted_elements(0);
      table.set_capacity(capacity);
      Ok(table)
    }
  }

  /// 至少容纳 at_least_space_for 分配新表 (ComputeCapacity 向上取容)。
  ///
  /// v8: src/objects/hash-table-inl.h HashTable::New
  pub fn new_with_space_for<A: MemoryAllocator>(
    allocator: &mut A,
    map: Tagged<Map>,
    shape: ShapeParams,
    at_least_space_for: u32,
  ) -> Result<Tagged<Self>> {
    Self::new(allocator, map, shape, compute_capacity(at_least_space_for))
  }
}

/// Tagged<HashTable> 扩展面: 槽访问 / 探测 / 原地重哈希 (CRTP 成员 → trait 方法)。
pub trait HashTableExt: Copy {
  fn slot_addr(&self, index: usize) -> usize;
  fn read_slot(&self, index: usize) -> Tagged<Object>;
  fn write_slot(&self, index: usize, val: Tagged<Object>);
  fn number_of_elements(&self) -> i32;
  fn set_number_of_elements(&self, n: i32);
  fn number_of_deleted_elements(&self) -> i32;
  fn set_number_of_deleted_elements(&self, n: i32);
  fn capacity(&self) -> u32;
  fn set_capacity(&self, capacity: u32);
  fn element_added(&self);
  fn element_removed(&self);
  fn entry_index(&self, entry: InternalIndex, shape: ShapeParams) -> usize;
  fn key_at(&self, entry: InternalIndex, shape: ShapeParams) -> Tagged<Object>;
  fn first_probe(&self, hash: u32, shape: ShapeParams) -> InternalIndex;
  fn find_entry(
    &self,
    key: Tagged<Object>,
    hash: u32,
    shape: ShapeParams,
    is_match: impl Fn(Tagged<Object>, Tagged<Object>) -> bool,
  ) -> InternalIndex;
  fn find_insertion_entry(&self, hash: u32, shape: ShapeParams) -> InternalIndex;
  fn has_sufficient_capacity_to_add(&self, n: i32) -> bool;
  fn entry_for_probe(
    &self,
    k: Tagged<Object>,
    probe: u32,
    expected: InternalIndex,
    shape: ShapeParams,
    hash_of: impl Fn(Tagged<Object>) -> u32,
  ) -> InternalIndex;
  fn swap(&self, entry1: InternalIndex, entry2: InternalIndex, shape: ShapeParams);
  fn rehash_in_place(&self, shape: ShapeParams, hash_of: impl Fn(Tagged<Object>) -> u32);
}

impl HashTableExt for Tagged<HashTable> {
  #[inline]
  fn slot_addr(&self, index: usize) -> usize {
    self.address() + FixedArray::OBJECTS_OFFSET + index * mem::size_of::<usize>()
  }

  #[inline]
  fn read_slot(&self, index: usize) -> Tagged<Object> {
    // SAFETY: slot_addr 计算自 FixedArray 对象体, 按 usize 自然对齐且在有效内存范围内
    let raw =
      unsafe { AtomicUsize::from_ptr(self.slot_addr(index) as *mut usize) }.load(Ordering::Relaxed);
    Tagged::new(raw)
  }

  #[inline]
  fn write_slot(&self, index: usize, val: Tagged<Object>) {
    // SAFETY: slot_addr 处于有效且对齐的 FixedArray 元素区, 原子存储符合并发访问契约
    unsafe { AtomicUsize::from_ptr(self.slot_addr(index) as *mut usize) }
      .store(val.ptr(), Ordering::Relaxed);
  }

  #[inline]
  fn number_of_elements(&self) -> i32 {
    let raw = self.read_slot(K_NUMBER_OF_ELEMENTS_INDEX);
    if raw.is_smi() {
      Smi::value(raw.cast::<Smi>())
    } else {
      0
    }
  }

  #[inline]
  fn set_number_of_elements(&self, n: i32) {
    self.write_slot(K_NUMBER_OF_ELEMENTS_INDEX, Smi::from_int(n).cast());
  }

  #[inline]
  fn number_of_deleted_elements(&self) -> i32 {
    let raw = self.read_slot(K_NUMBER_OF_DELETED_ELEMENTS_INDEX);
    if raw.is_smi() {
      Smi::value(raw.cast::<Smi>())
    } else {
      0
    }
  }

  #[inline]
  fn set_number_of_deleted_elements(&self, n: i32) {
    self.write_slot(K_NUMBER_OF_DELETED_ELEMENTS_INDEX, Smi::from_int(n).cast());
  }

  #[inline]
  fn capacity(&self) -> u32 {
    let raw = self.read_slot(K_CAPACITY_INDEX);
    if raw.is_smi() {
      Smi::value(raw.cast::<Smi>()).max(0) as u32
    } else {
      0
    }
  }

  #[inline]
  fn set_capacity(&self, capacity: u32) {
    // v8: hash-table-inl.h SetCapacity — 容量须正且 ≤ kMaxCapacity (探测用与掩码)
    debug_assert!(capacity > 0);
    self.write_slot(K_CAPACITY_INDEX, Smi::from_int(capacity as i32).cast());
  }

  #[inline]
  fn element_added(&self) {
    self.set_number_of_elements(self.number_of_elements() + 1);
  }

  #[inline]
  fn element_removed(&self) {
    self.set_number_of_elements(self.number_of_elements() - 1);
    self.set_number_of_deleted_elements(self.number_of_deleted_elements() + 1);
  }

  #[inline]
  fn entry_index(&self, entry: InternalIndex, shape: ShapeParams) -> usize {
    entry_to_index(entry.as_usize() as u32, shape.entry_size, elements_start_index(shape.prefix_size))
  }

  #[inline]
  fn key_at(&self, entry: InternalIndex, shape: ShapeParams) -> Tagged<Object> {
    self.read_slot(self.entry_index(entry, shape) + K_ENTRY_KEY_INDEX)
  }

  #[inline]
  fn first_probe(&self, hash: u32, shape: ShapeParams) -> InternalIndex {
    InternalIndex::new(
      first_probe(hash, self.capacity(), shape.do_hash_spreading, shape.hash_bits) as usize,
    )
  }

  /// 二次探测查键; undefined 空槽即停, the_hole 视 kMatchNeedsHoleCheck 跳过。
  ///
  /// v8: src/objects/hash-table-inl.h HashTable::FindEntry
  fn find_entry(
    &self,
    key: Tagged<Object>,
    hash: u32,
    shape: ShapeParams,
    is_match: impl Fn(Tagged<Object>, Tagged<Object>) -> bool,
  ) -> InternalIndex {
    let capacity = self.capacity();
    if capacity == 0 {
      // 防御: 0 容量空表直接未命中 (原版 EnsureCapacity 不变式保证不会出现)
      return InternalIndex::NOT_FOUND;
    }
    let mut count = 1u32;
    let mut entry =
      first_probe(hash, capacity, shape.do_hash_spreading, shape.hash_bits);
    loop {
      let element = self.key_at(InternalIndex::new(entry as usize), shape);
      // 空槽: 探测终止 (EnsureCapacity 保证表永不满)
      if is_undefined(element) {
        return InternalIndex::NOT_FOUND;
      }
      if shape.match_needs_hole_check && is_the_hole(element) {
        entry = next_probe(entry, count, capacity);
        count += 1;
        continue;
      }
      if is_match(key, element) {
        return InternalIndex::new(entry as usize);
      }
      entry = next_probe(entry, count, capacity);
      count += 1;
    }
  }

  /// 查可插入槽: 首个非键槽 (undefined 空槽或 the_hole 删除槽)。
  ///
  /// v8: src/objects/objects.cc HashTable::FindInsertionEntry
  fn find_insertion_entry(&self, hash: u32, shape: ShapeParams) -> InternalIndex {
    let capacity = self.capacity();
    let mut count = 1u32;
    let mut entry = first_probe(hash, capacity, shape.do_hash_spreading, shape.hash_bits);
    loop {
      let element = self.key_at(InternalIndex::new(entry as usize), shape);
      if !is_key(element) {
        return InternalIndex::new(entry as usize);
      }
      entry = next_probe(entry, count, capacity);
      count += 1;
    }
  }

  #[inline]
  fn has_sufficient_capacity_to_add(&self, n: i32) -> bool {
    has_sufficient_capacity_to_add(
      self.capacity() as i32,
      self.number_of_elements(),
      self.number_of_deleted_elements(),
      n,
    )
  }

  /// 第 probe 轮探测下 current 元素的归位槽。
  ///
  /// v8: src/objects/objects.cc HashTable::EntryForProbe
  fn entry_for_probe(
    &self,
    k: Tagged<Object>,
    probe: u32,
    expected: InternalIndex,
    shape: ShapeParams,
    hash_of: impl Fn(Tagged<Object>) -> u32,
  ) -> InternalIndex {
    let hash = hash_of(k);
    let capacity = self.capacity();
    let mut entry = self.first_probe(hash, shape);
    for i in 1..probe {
      if entry == expected {
        return expected;
      }
      entry =
        InternalIndex::new(next_probe(entry.as_usize() as u32, i, capacity) as usize);
    }
    entry
  }

  /// 整 entry 交换 (entry_size 槽逐一)。
  ///
  /// v8: src/objects/objects.cc HashTable::Swap
  fn swap(&self, entry1: InternalIndex, entry2: InternalIndex, shape: ShapeParams) {
    let index1 = self.entry_index(entry1, shape);
    let index2 = self.entry_index(entry2, shape);
    for j in 0..shape.entry_size {
      let v1 = self.read_slot(index1 + j);
      let v2 = self.read_slot(index2 + j);
      self.write_slot(index1 + j, v2);
      self.write_slot(index2 + j, v1);
    }
  }

  /// 同容量原地重哈希: 逐 probe 轮归位 + 删除槽抹除。
  ///
  /// v8: src/objects/objects.cc HashTable::Rehash()
  fn rehash_in_place(&self, shape: ShapeParams, hash_of: impl Fn(Tagged<Object>) -> u32) {
    let capacity = self.capacity();
    let mut done = false;
    let mut probe = 1u32;
    while !done {
      done = true;
      let mut current = InternalIndex::new(0);
      while current.as_usize() < capacity as usize {
        let current_key = self.key_at(current, shape);
        if !is_key(current_key) {
          current = current.adjust_up(1);
          continue;
        }
        let target = self.entry_for_probe(current_key, probe, current, shape, &hash_of);
        if current == target {
          current = current.adjust_up(1);
          continue;
        }
        let target_key = self.key_at(target, shape);
        if !is_key(target_key)
          || self.entry_for_probe(target_key, probe, target, shape, &hash_of) != target
        {
          // 把 current 元素换到正确位置; 对方元素留待下轮处理, 此处不前进 current
          self.swap(current, target, shape);
        } else {
          // 归位槽被占, 留待下一 probe 轮
          done = false;
          current = current.adjust_up(1);
        }
      }
      probe += 1;
    }
    // 抹除删除槽: the_hole 键槽恢复 undefined
    for i in 0..capacity {
      let entry = InternalIndex::new(i as usize);
      if self.key_at(entry, shape) == THE_HOLE {
        self.write_slot(self.entry_index(entry, shape) + K_ENTRY_KEY_INDEX, UNDEFINED);
      }
    }
    self.set_number_of_deleted_elements(0);
  }
}

/// 实例容量判定 (四参静态形态, 与 ComputeCapacity 同步; hash-table-inl.h)。
///
/// v8: src/objects/objects.cc HashTable::HasSufficientCapacityToAdd
#[inline]
#[must_use]
pub const fn has_sufficient_capacity_to_add(
  capacity: i32,
  number_of_elements: i32,
  number_of_deleted_elements: i32,
  number_of_additional_elements: i32,
) -> bool {
  let nof = number_of_elements + number_of_additional_elements;
  // 加入后仍余 ≥50% 空位, 且至多 50% 的空位是删除槽
  if nof < capacity && number_of_deleted_elements <= (capacity - nof) / 2 {
    let needed_free = nof / 2;
    return nof + needed_free <= capacity;
  }
  false
}

/// 整表重哈希到已分配的新表 (前缀 + 键值 entry 逐一迁移)。
///
/// v8: src/objects/objects.cc HashTable::Rehash(Tagged<Derived> new_table)
pub fn rehash(
  table: Tagged<HashTable>,
  new_table: Tagged<HashTable>,
  shape: ShapeParams,
  hash_of: impl Fn(Tagged<Object>) -> u32,
) {
  debug_assert!((table.number_of_elements() as u32) < new_table.capacity());
  // 拷贝形态前缀槽 (计数之外归派生 Shape 的前缀 [3..3+prefix))
  let elements_start = elements_start_index(shape.prefix_size);
  for slot in K_PREFIX_START_INDEX..elements_start {
    new_table.write_slot(slot, table.read_slot(slot));
  }
  // 逐 entry 迁移
  for i in 0..table.capacity() {
    let from_index = table.entry_index(InternalIndex::new(i as usize), shape);
    let k = table.read_slot(from_index + K_ENTRY_KEY_INDEX);
    if !is_key(k) {
      continue;
    }
    let hash = hash_of(k);
    let insertion = new_table.find_insertion_entry(hash, shape);
    let to_index = new_table.entry_index(insertion, shape);
    for j in 0..shape.entry_size {
      let v = table.read_slot(from_index + j);
      new_table.write_slot(to_index + j, v);
    }
  }
  new_table.set_number_of_elements(table.number_of_elements());
  new_table.set_number_of_deleted_elements(0);
}

/// 容量不足时扩容: New(nof+n) + Rehash + 计数迁移 (老年代预Tenure 判定依赖分代, 仿真堆省略)。
///
/// v8: src/objects/objects.cc HashTable::EnsureCapacity
pub fn ensure_capacity<A: MemoryAllocator>(
  allocator: &mut A,
  map: Tagged<Map>,
  table: Tagged<HashTable>,
  n: i32,
  shape: ShapeParams,
) -> Result<Tagged<HashTable>> {
  if table.has_sufficient_capacity_to_add(n) {
    return Ok(table);
  }
  let new_nof = (table.number_of_elements() + n).max(0) as u32;
  let new_table = HashTable::new_with_space_for(allocator, map, shape, new_nof)?;
  rehash(table, new_table, shape, compute_object_hash);
  Ok(new_table)
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_NUMBER_OF_ELEMENTS_INDEX == 0 && K_CAPACITY_INDEX == 2 && K_PREFIX_START_INDEX == 3);
  assert!(K_MIN_CAPACITY == 4 && K_MIN_SHRINK_CAPACITY == 16 && K_MIN_CAPACITY_FOR_PRETENURE == 256);
  assert!(K_MAX_REGULAR_CAPACITY == 4096);
  assert!(compute_capacity(1) == 4 && compute_capacity(3) == 8 && compute_capacity(100) == 256);
  assert!(next_probe(0, 1, 8) == 1 && next_probe(7, 3, 8) == 2);
  assert!(first_probe(7, 8, false, 0) == 7);
  // 分散分支: size > 2^10 且 kHashBits=10 → coefficient = size >> 10
  assert!(first_probe(7, 2048, true, 10) == (7 * 2) & 2047);
  assert!(OBJECT_HASH_SET_SHAPE.entry_size == 1 && EPHEMERON_HASH_TABLE_SHAPE.hash_bits == 10);
  assert!(OBJECT_TWO_HASH_TABLE_ENTRY_SIZE == 3);
  assert!(elements_start_index(0) == 3 && elements_start_index(3) == 6);
};
