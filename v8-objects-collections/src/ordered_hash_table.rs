//! 有序哈希表家族: OrderedHashTable / OrderedHashSet / OrderedHashMap
//! 对应原版: v8/src/objects/ordered-hash-table.h + ordered-hash-table-inl.h + ordered-hash-table.cc

use std::{
  mem,
  ptr::write,
  sync::atomic::{AtomicUsize, Ordering},
};

use v8_objects_arrays::fixed_array::{FixedArray, TaggedArray};
use v8_objects_core::{
  instance_type_checker as itc,
  map::Map,
  name::Name,
  objects::{instance_type_of, object_same_value_zero},
  property_details::PropertyDetails,
  smi::Smi,
  tagged::{Object, Tagged},
  tagged_constants::{HEAP_OBJECT_TAG, HEAP_OBJECT_TAG_MASK},
};
use v8_objects_strings::string::String as V8String;

use crate::{
  allocator::{HeapStorage, MemoryAllocator},
  error::Result,
  internal_index::{InternalIndex, InternalIndexRange},
};

// ── 核心常量 ─────────────────────────────────────────────────────────────
pub const K_NOT_FOUND: i32 = -1;
pub const K_INITIAL_CAPACITY: usize = 4;
pub const K_LOAD_FACTOR: usize = 2;
pub const K_CLEARED_TABLE_SENTINEL: i32 = -1;

/// 哈希表 Hole 哨兵 (非空、非 Smi、与任何实际堆对象地址不重叠)
pub const THE_HOLE: Tagged<Object> =
  Tagged::new((usize::MAX & !HEAP_OBJECT_TAG_MASK) | HEAP_OBJECT_TAG);

/// 判断是否为 Hole 哨兵
#[inline]
pub fn is_the_hole(obj: Tagged<Object>) -> bool {
  obj == THE_HOLE
}

/// 键相等性比较 (SameValueZero 语义，空穴不匹配)
#[inline]
pub fn keys_equal(a: Tagged<Object>, b: Tagged<Object>) -> bool {
  if a == b {
    return true;
  }
  if is_the_hole(a) || is_the_hole(b) {
    return false;
  }
  if let (Some(hoa), Some(hob)) = (a.to_heap_object(), b.to_heap_object())
    && instance_type_of(a).is_some_and(itc::is_string)
    && instance_type_of(b).is_some_and(itc::is_string)
  {
    // SAFETY: 前置断言已验证 hoa 与 hob 为 String 实例，address() 处存在合法的 V8String 结构
    let sa = unsafe { &*(hoa.address() as *const V8String) };
    let sb = unsafe { &*(hob.address() as *const V8String) };
    if sa.length != sb.length {
      return false;
    }
    if sa.name.has_hash_code() && sb.name.has_hash_code() && sa.name.hash() != sb.name.hash() {
      return false;
    }
  }
  object_same_value_zero(a, b).unwrap_or(false)
}

/// 针对 Tagged<Object> 计算哈希值
#[inline]
pub fn compute_object_hash(key: Tagged<Object>) -> u32 {
  if is_the_hole(key) {
    return 0;
  }
  if key.is_smi() {
    let val = Smi::value(key.cast::<Smi>());
    let mut h = val as u32;
    h = (!h).wrapping_add(h << 15);
    h ^= h >> 12;
    h = h.wrapping_add(h << 2);
    h ^= h >> 4;
    h = h.wrapping_mul(2057);
    h ^= h >> 16;
    h & 0x3fffffff
  } else if let Some(ho) = key.to_heap_object() {
    let addr = ho.address();
    if instance_type_of(key).is_some_and(itc::is_name) {
      // SAFETY: 已验证为 Name 实例类型
      let name = unsafe { &*(addr as *const Name) };
      if name.has_hash_code() {
        return name.hash();
      }
    }
    let mut h = (addr ^ (addr >> 32)) as u32;
    h = (!h).wrapping_add(h << 15);
    h ^= h >> 12;
    h = h.wrapping_add(h << 2);
    h ^= h >> 4;
    h = h.wrapping_mul(2057);
    h ^= h >> 16;
    h & 0x3fffffff
  } else {
    0
  }
}

// ── 布局与槽位换算 ───────────────────────────────────────────────────────
#[inline]
pub const fn prefix_index() -> usize {
  0
}

#[inline]
pub const fn number_of_elements_index(prefix_size: usize) -> usize {
  prefix_size
}

#[inline]
pub const fn number_of_deleted_elements_index(prefix_size: usize) -> usize {
  prefix_size + 1
}

#[inline]
pub const fn number_of_buckets_index(prefix_size: usize) -> usize {
  prefix_size + 2
}

#[inline]
pub const fn hash_table_start_index(prefix_size: usize) -> usize {
  prefix_size + 3
}

#[inline]
pub const fn calc_capacity(num_buckets: usize) -> usize {
  num_buckets * K_LOAD_FACTOR
}

#[inline]
pub const fn hash_to_bucket(hash: u32, num_buckets: usize) -> usize {
  debug_assert!(num_buckets > 0 && (num_buckets & (num_buckets - 1)) == 0);
  (hash as usize) & (num_buckets - 1)
}

#[inline]
pub const fn entry_to_index(
  prefix_size: usize,
  num_buckets: usize,
  entry: usize,
  entry_size: usize,
) -> usize {
  hash_table_start_index(prefix_size) + num_buckets + entry * entry_size
}

#[inline]
pub const fn total_slots(
  prefix_size: usize,
  num_buckets: usize,
  capacity: usize,
  entry_size: usize,
) -> usize {
  hash_table_start_index(prefix_size) + num_buckets + capacity * entry_size
}

// ── OrderedHashSet ────────────────────────────────────────────────────────
/// OrderedHashSet (entrysize = 1, kEntrySize = 2 [key, chain])
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct OrderedHashSet {
  pub base: FixedArray,
}

impl OrderedHashSet {
  pub const ENTRY_SIZE_WITHOUT_CHAIN: usize = 1;
  pub const ENTRY_SIZE: usize = 2;
  pub const CHAIN_OFFSET: usize = 1;
  pub const PREFIX_SIZE: usize = 0;

  #[inline]
  pub const fn size_for(capacity: usize) -> usize {
    let num_buckets = capacity / K_LOAD_FACTOR;
    let slots = total_slots(Self::PREFIX_SIZE, num_buckets, capacity, Self::ENTRY_SIZE);
    FixedArray::size_for(slots as u32)
  }

  /// 分配并初始化 OrderedHashSet
  pub fn allocate<A: MemoryAllocator>(
    allocator: &mut A,
    map: Tagged<Map>,
    capacity: usize,
  ) -> Result<Tagged<Self>> {
    let cap = capacity.max(K_INITIAL_CAPACITY).next_power_of_two();
    let num_buckets = cap / K_LOAD_FACTOR;
    let slots = total_slots(
      OrderedHashSet::PREFIX_SIZE,
      num_buckets,
      cap,
      OrderedHashSet::ENTRY_SIZE,
    );
    let size = FixedArray::size_for(slots as u32);
    let addr = allocator.allocate(size)?;

    let header = FixedArray::new(map, slots as u32);
    // SAFETY: addr 由分配器成功分配并对齐，写入 FixedArray 头部并在随后完成插槽初始化
    unsafe {
      write(addr as *mut FixedArray, header);
      let table: Tagged<Self> = Tagged::from_raw_address(addr);
      table.set_number_of_buckets(num_buckets);
      table.set_number_of_elements(0);
      table.set_number_of_deleted_elements(0);
      for b in 0..num_buckets {
        table.set_bucket(b, K_NOT_FOUND);
      }
      for entry in 0..cap {
        table.set_key_at(entry, THE_HOLE);
        table.set_chain_at(entry, K_NOT_FOUND);
      }
      Ok(table)
    }
  }
}

pub trait OrderedHashSetExt: Copy {
  fn number_of_elements(&self) -> usize;
  fn set_number_of_elements(&self, n: usize);
  fn number_of_deleted_elements(&self) -> usize;
  fn set_number_of_deleted_elements(&self, n: usize);
  fn number_of_buckets(&self) -> usize;
  fn set_number_of_buckets(&self, n: usize);
  fn capacity(&self) -> usize;
  fn used_capacity(&self) -> usize;
  fn is_obsolete(&self) -> bool;
  fn get_bucket(&self, bucket: usize) -> i32;
  fn set_bucket(&self, bucket: usize, entry: i32);
  fn entry_index(&self, entry: usize) -> usize;
  fn key_at(&self, entry: InternalIndex) -> Tagged<Object>;
  fn set_key_at(&self, entry: usize, key: Tagged<Object>);
  fn chain_at(&self, entry: usize) -> i32;
  fn set_chain_at(&self, entry: usize, next_entry: i32);
  fn find_entry(&self, key: Tagged<Object>) -> InternalIndex;
  fn has_key(&self, key: Tagged<Object>) -> bool;
  fn add<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
    key: Tagged<Object>,
  ) -> Result<Tagged<OrderedHashSet>>;
  fn slot_addr(&self, index: usize) -> usize;
  fn read_slot(&self, index: usize) -> Tagged<Object>;
  fn write_slot(&self, index: usize, val: Tagged<Object>);
  fn delete(&self, key: Tagged<Object>) -> bool;
  fn shrink<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
  ) -> Result<Tagged<OrderedHashSet>>;
  fn clear<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
  ) -> Result<Tagged<OrderedHashSet>>;
  fn rehash<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
    new_capacity: usize,
  ) -> Result<Tagged<OrderedHashSet>>;
  fn next_table(&self) -> Option<Tagged<OrderedHashSet>>;
  fn set_removed_index_at(&self, index: usize, removed_index: usize);
  fn removed_index_at(&self, index: usize) -> usize;
  fn iter(&self) -> OrderedHashSetIterator;
}

impl OrderedHashSetExt for Tagged<OrderedHashSet> {
  #[inline]
  fn capacity(&self) -> usize {
    calc_capacity(self.number_of_buckets())
  }

  #[inline]
  fn used_capacity(&self) -> usize {
    let nod = self.number_of_deleted_elements();
    if nod == K_CLEARED_TABLE_SENTINEL as usize {
      0
    } else {
      self.number_of_elements() + nod
    }
  }

  #[inline]
  fn is_obsolete(&self) -> bool {
    let raw = self.read_slot(number_of_elements_index(OrderedHashSet::PREFIX_SIZE));
    !raw.is_smi()
  }

  #[inline]
  fn number_of_elements(&self) -> usize {
    let raw = self.read_slot(number_of_elements_index(OrderedHashSet::PREFIX_SIZE));
    if raw.is_smi() {
      Smi::value(raw.cast::<Smi>()).max(0) as usize
    } else {
      0
    }
  }

  #[inline]
  fn set_number_of_elements(&self, n: usize) {
    self.write_slot(
      number_of_elements_index(OrderedHashSet::PREFIX_SIZE),
      Smi::from_int(n as i32).cast(),
    );
  }

  #[inline]
  fn number_of_deleted_elements(&self) -> usize {
    let raw = self.read_slot(number_of_deleted_elements_index(
      OrderedHashSet::PREFIX_SIZE,
    ));
    if raw.is_smi() {
      Smi::value(raw.cast::<Smi>()) as isize as usize
    } else {
      0
    }
  }

  #[inline]
  fn set_number_of_deleted_elements(&self, n: usize) {
    self.write_slot(
      number_of_deleted_elements_index(OrderedHashSet::PREFIX_SIZE),
      Smi::from_int(n as i32).cast(),
    );
  }

  #[inline]
  fn number_of_buckets(&self) -> usize {
    let raw = self.read_slot(number_of_buckets_index(OrderedHashSet::PREFIX_SIZE));
    if raw.is_smi() {
      Smi::value(raw.cast::<Smi>()).max(0) as usize
    } else {
      0
    }
  }

  #[inline]
  fn set_number_of_buckets(&self, n: usize) {
    self.write_slot(
      number_of_buckets_index(OrderedHashSet::PREFIX_SIZE),
      Smi::from_int(n as i32).cast(),
    );
  }

  #[inline]
  fn get_bucket(&self, bucket: usize) -> i32 {
    let slot = hash_table_start_index(OrderedHashSet::PREFIX_SIZE) + bucket;
    let raw = self.read_slot(slot);
    if raw.is_smi() {
      Smi::value(raw.cast::<Smi>())
    } else {
      K_NOT_FOUND
    }
  }

  #[inline]
  fn set_bucket(&self, bucket: usize, entry: i32) {
    let slot = hash_table_start_index(OrderedHashSet::PREFIX_SIZE) + bucket;
    self.write_slot(slot, Smi::from_int(entry).cast());
  }

  #[inline]
  fn entry_index(&self, entry: usize) -> usize {
    entry_to_index(
      OrderedHashSet::PREFIX_SIZE,
      self.number_of_buckets(),
      entry,
      OrderedHashSet::ENTRY_SIZE,
    )
  }

  #[inline]
  fn key_at(&self, entry: InternalIndex) -> Tagged<Object> {
    debug_assert!(entry.as_usize() < self.used_capacity());
    self.read_slot(self.entry_index(entry.as_usize()))
  }

  #[inline]
  fn set_key_at(&self, entry: usize, key: Tagged<Object>) {
    self.write_slot(self.entry_index(entry), key);
  }

  #[inline]
  fn chain_at(&self, entry: usize) -> i32 {
    let raw = self.read_slot(self.entry_index(entry) + OrderedHashSet::CHAIN_OFFSET);
    if raw.is_smi() {
      Smi::value(raw.cast::<Smi>())
    } else {
      K_NOT_FOUND
    }
  }

  #[inline]
  fn set_chain_at(&self, entry: usize, next_entry: i32) {
    self.write_slot(
      self.entry_index(entry) + OrderedHashSet::CHAIN_OFFSET,
      Smi::from_int(next_entry).cast(),
    );
  }

  fn find_entry(&self, key: Tagged<Object>) -> InternalIndex {
    if self.number_of_elements() == 0 || self.capacity() == 0 {
      return InternalIndex::NOT_FOUND;
    }
    let hash = compute_object_hash(key);
    let bucket = hash_to_bucket(hash, self.number_of_buckets());
    let mut raw_entry = self.get_bucket(bucket);

    while raw_entry >= 0 && (raw_entry as usize) < self.used_capacity() {
      let entry_idx = raw_entry as usize;
      let candidate = self.key_at(InternalIndex::new(entry_idx));
      if keys_equal(candidate, key) {
        return InternalIndex::new(entry_idx);
      }
      raw_entry = self.chain_at(entry_idx);
    }
    InternalIndex::NOT_FOUND
  }

  #[inline]
  fn has_key(&self, key: Tagged<Object>) -> bool {
    self.find_entry(key).is_found()
  }

  fn add<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
    key: Tagged<Object>,
  ) -> Result<Tagged<OrderedHashSet>> {
    let entry = self.find_entry(key);
    if entry.is_found() {
      return Ok(*self);
    }

    let mut current = *self;
    if current.used_capacity() >= current.capacity() {
      let capacity = current.capacity();
      let nod = current.number_of_deleted_elements();
      let new_capacity = if capacity == 0 {
        K_INITIAL_CAPACITY
      } else if nod >= (capacity >> 1) {
        capacity
      } else {
        capacity << 1
      };
      current = current.rehash(allocator, map, new_capacity)?;
    }

    let nof = current.number_of_elements();
    let nod = current.number_of_deleted_elements();
    let new_entry = nof + nod;

    let hash = compute_object_hash(key);
    let bucket = hash_to_bucket(hash, current.number_of_buckets());
    let prev_entry = current.get_bucket(bucket);

    current.set_key_at(new_entry, key);
    current.set_chain_at(new_entry, prev_entry);
    current.set_bucket(bucket, new_entry as i32);
    current.set_number_of_elements(nof + 1);

    Ok(current)
  }

  fn delete(&self, key: Tagged<Object>) -> bool {
    let entry = self.find_entry(key);
    if entry.is_not_found() {
      return false;
    }
    let raw = entry.as_usize();
    self.set_key_at(raw, THE_HOLE);
    self.set_number_of_elements(self.number_of_elements() - 1);
    self.set_number_of_deleted_elements(self.number_of_deleted_elements() + 1);
    true
  }

  fn shrink<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
  ) -> Result<Tagged<OrderedHashSet>> {
    let nof = self.number_of_elements();
    let cap = self.capacity();
    if nof >= (cap >> 2) || (cap >> 1) < K_INITIAL_CAPACITY {
      return Ok(*self);
    }
    self.rehash(allocator, map, cap >> 1)
  }

  fn clear<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
  ) -> Result<Tagged<OrderedHashSet>> {
    let new_table = OrderedHashSet::allocate(allocator, map, K_INITIAL_CAPACITY)?;
    if self.number_of_buckets() > 0 {
      self.write_slot(
        number_of_elements_index(OrderedHashSet::PREFIX_SIZE),
        new_table.cast(),
      );
      self.set_number_of_deleted_elements(K_CLEARED_TABLE_SENTINEL as isize as usize);
    }
    Ok(new_table)
  }

  fn rehash<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
    new_capacity: usize,
  ) -> Result<Tagged<OrderedHashSet>> {
    let mut entries = Vec::with_capacity(self.number_of_elements());
    let old_has_buckets = self.number_of_buckets() > 0;
    let mut removed_holes_index = 0;

    for old_entry in 0..self.used_capacity() {
      let key = self.key_at(InternalIndex::new(old_entry));
      if is_the_hole(key) {
        if old_has_buckets {
          self.set_removed_index_at(removed_holes_index, old_entry);
        }
        removed_holes_index += 1;
      } else {
        entries.push(key);
      }
    }
    let old_nof = self.number_of_elements();

    let new_table = OrderedHashSet::allocate(allocator, map, new_capacity)?;
    for (new_entry, key) in entries.into_iter().enumerate() {
      let hash = compute_object_hash(key);
      let bucket = hash_to_bucket(hash, new_table.number_of_buckets());
      let chain = new_table.get_bucket(bucket);

      new_table.set_key_at(new_entry, key);
      new_table.set_chain_at(new_entry, chain);
      new_table.set_bucket(bucket, new_entry as i32);
    }

    new_table.set_number_of_elements(old_nof);
    if old_has_buckets {
      self.write_slot(
        number_of_elements_index(OrderedHashSet::PREFIX_SIZE),
        new_table.cast(),
      );
    }
    Ok(new_table)
  }

  #[inline]
  fn next_table(&self) -> Option<Tagged<OrderedHashSet>> {
    if self.is_obsolete() {
      let raw = self.read_slot(number_of_elements_index(OrderedHashSet::PREFIX_SIZE));
      Some(Tagged::new(raw.ptr()))
    } else {
      None
    }
  }

  #[inline]
  fn set_removed_index_at(&self, index: usize, removed_index: usize) {
    let slot = hash_table_start_index(OrderedHashSet::PREFIX_SIZE) + index;
    self.write_slot(slot, Smi::from_int(removed_index as i32).cast());
  }

  #[inline]
  fn removed_index_at(&self, index: usize) -> usize {
    let slot = hash_table_start_index(OrderedHashSet::PREFIX_SIZE) + index;
    let raw = self.read_slot(slot);
    if raw.is_smi() {
      Smi::value(raw.cast::<Smi>()).max(0) as usize
    } else {
      0
    }
  }

  #[inline]
  fn slot_addr(&self, index: usize) -> usize {
    self.address() + FixedArray::OBJECTS_OFFSET + index * mem::size_of::<usize>()
  }

  #[inline]
  fn read_slot(&self, index: usize) -> Tagged<Object> {
    // SAFETY: slot_addr 计算自 FixedArray 对象体，按 usize 边界自然对齐且在有效对象内存范围内
    let raw =
      unsafe { AtomicUsize::from_ptr(self.slot_addr(index) as *mut usize) }.load(Ordering::Relaxed);
    Tagged::new(raw)
  }

  #[inline]
  fn write_slot(&self, index: usize, val: Tagged<Object>) {
    // SAFETY: slot_addr 处于有效且对齐的 FixedArray 元素区，原子存储符合并发访问契约
    unsafe { AtomicUsize::from_ptr(self.slot_addr(index) as *mut usize) }
      .store(val.ptr(), Ordering::Relaxed);
  }

  #[inline]
  fn iter(&self) -> OrderedHashSetIterator {
    OrderedHashSetIterator {
      table: *self,
      range: InternalIndexRange::new(self.used_capacity()),
    }
  }
}

pub struct OrderedHashSetIterator {
  table: Tagged<OrderedHashSet>,
  range: InternalIndexRange,
}

impl Iterator for OrderedHashSetIterator {
  type Item = Tagged<Object>;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    for idx in self.range.by_ref() {
      let key = self.table.key_at(idx);
      if !is_the_hole(key) {
        return Some(key);
      }
    }
    None
  }
}

// ── OrderedHashMap ────────────────────────────────────────────────────────
/// OrderedHashMap (entrysize = 2, kEntrySize = 3 [key, value, chain])
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct OrderedHashMap {
  pub base: FixedArray,
}

impl OrderedHashMap {
  pub const ENTRY_SIZE_WITHOUT_CHAIN: usize = 2;
  pub const ENTRY_SIZE: usize = 3;
  pub const VALUE_OFFSET: usize = 1;
  pub const CHAIN_OFFSET: usize = 2;
  pub const PREFIX_SIZE: usize = 0;

  #[inline]
  pub const fn size_for(capacity: usize) -> usize {
    let num_buckets = capacity / K_LOAD_FACTOR;
    let slots = total_slots(Self::PREFIX_SIZE, num_buckets, capacity, Self::ENTRY_SIZE);
    FixedArray::size_for(slots as u32)
  }

  /// 分配并初始化 OrderedHashMap
  pub fn allocate<A: MemoryAllocator>(
    allocator: &mut A,
    map: Tagged<Map>,
    capacity: usize,
  ) -> Result<Tagged<Self>> {
    let cap = capacity.max(K_INITIAL_CAPACITY).next_power_of_two();
    let num_buckets = cap / K_LOAD_FACTOR;
    let slots = total_slots(
      OrderedHashMap::PREFIX_SIZE,
      num_buckets,
      cap,
      OrderedHashMap::ENTRY_SIZE,
    );
    let size = FixedArray::size_for(slots as u32);
    let addr = allocator.allocate(size)?;

    let header = FixedArray::new(map, slots as u32);
    // SAFETY: addr 由分配器成功分配并对齐，写入 FixedArray 头部并在随后完成插槽初始化
    unsafe {
      write(addr as *mut FixedArray, header);
      let table: Tagged<Self> = Tagged::from_raw_address(addr);
      table.set_number_of_buckets(num_buckets);
      table.set_number_of_elements(0);
      table.set_number_of_deleted_elements(0);
      for b in 0..num_buckets {
        table.set_bucket(b, K_NOT_FOUND);
      }
      for entry in 0..cap {
        table.set_entry(entry, THE_HOLE, THE_HOLE);
        table.set_chain_at(entry, K_NOT_FOUND);
      }
      Ok(table)
    }
  }
}

pub trait OrderedHashMapExt: Copy {
  fn number_of_elements(&self) -> usize;
  fn set_number_of_elements(&self, n: usize);
  fn number_of_deleted_elements(&self) -> usize;
  fn set_number_of_deleted_elements(&self, n: usize);
  fn number_of_buckets(&self) -> usize;
  fn set_number_of_buckets(&self, n: usize);
  fn capacity(&self) -> usize;
  fn used_capacity(&self) -> usize;
  fn is_obsolete(&self) -> bool;
  fn get_bucket(&self, bucket: usize) -> i32;
  fn set_bucket(&self, bucket: usize, entry: i32);
  fn entry_index(&self, entry: usize) -> usize;
  fn key_at(&self, entry: InternalIndex) -> Tagged<Object>;
  fn value_at(&self, entry: InternalIndex) -> Tagged<Object>;
  fn set_entry(&self, entry: usize, key: Tagged<Object>, value: Tagged<Object>);
  fn chain_at(&self, entry: usize) -> i32;
  fn set_chain_at(&self, entry: usize, next_entry: i32);
  fn find_entry(&self, key: Tagged<Object>) -> InternalIndex;
  fn has_key(&self, key: Tagged<Object>) -> bool;
  fn get(&self, key: Tagged<Object>) -> Option<Tagged<Object>>;
  fn add<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
    key: Tagged<Object>,
    value: Tagged<Object>,
  ) -> Result<Tagged<OrderedHashMap>>;
  fn slot_addr(&self, index: usize) -> usize;
  fn read_slot(&self, index: usize) -> Tagged<Object>;
  fn write_slot(&self, index: usize, val: Tagged<Object>);
  fn delete(&self, key: Tagged<Object>) -> bool;
  fn shrink<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
  ) -> Result<Tagged<OrderedHashMap>>;
  fn clear<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
  ) -> Result<Tagged<OrderedHashMap>>;
  fn rehash<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
    new_capacity: usize,
  ) -> Result<Tagged<OrderedHashMap>>;
  fn next_table(&self) -> Option<Tagged<OrderedHashMap>>;
  fn set_removed_index_at(&self, index: usize, removed_index: usize);
  fn removed_index_at(&self, index: usize) -> usize;
  fn iter(&self) -> OrderedHashMapIterator;
}

impl OrderedHashMapExt for Tagged<OrderedHashMap> {
  #[inline]
  fn capacity(&self) -> usize {
    calc_capacity(self.number_of_buckets())
  }

  #[inline]
  fn used_capacity(&self) -> usize {
    let nod = self.number_of_deleted_elements();
    if nod == K_CLEARED_TABLE_SENTINEL as usize {
      0
    } else {
      self.number_of_elements() + nod
    }
  }

  #[inline]
  fn is_obsolete(&self) -> bool {
    let raw = self.read_slot(number_of_elements_index(OrderedHashMap::PREFIX_SIZE));
    !raw.is_smi()
  }

  #[inline]
  fn number_of_elements(&self) -> usize {
    let raw = self.read_slot(number_of_elements_index(OrderedHashMap::PREFIX_SIZE));
    if raw.is_smi() {
      Smi::value(raw.cast::<Smi>()).max(0) as usize
    } else {
      0
    }
  }

  #[inline]
  fn set_number_of_elements(&self, n: usize) {
    self.write_slot(
      number_of_elements_index(OrderedHashMap::PREFIX_SIZE),
      Smi::from_int(n as i32).cast(),
    );
  }

  #[inline]
  fn number_of_deleted_elements(&self) -> usize {
    let raw = self.read_slot(number_of_deleted_elements_index(
      OrderedHashMap::PREFIX_SIZE,
    ));
    if raw.is_smi() {
      Smi::value(raw.cast::<Smi>()) as isize as usize
    } else {
      0
    }
  }

  #[inline]
  fn set_number_of_deleted_elements(&self, n: usize) {
    self.write_slot(
      number_of_deleted_elements_index(OrderedHashMap::PREFIX_SIZE),
      Smi::from_int(n as i32).cast(),
    );
  }

  #[inline]
  fn number_of_buckets(&self) -> usize {
    let raw = self.read_slot(number_of_buckets_index(OrderedHashMap::PREFIX_SIZE));
    if raw.is_smi() {
      Smi::value(raw.cast::<Smi>()).max(0) as usize
    } else {
      0
    }
  }

  #[inline]
  fn set_number_of_buckets(&self, n: usize) {
    self.write_slot(
      number_of_buckets_index(OrderedHashMap::PREFIX_SIZE),
      Smi::from_int(n as i32).cast(),
    );
  }

  #[inline]
  fn get_bucket(&self, bucket: usize) -> i32 {
    let slot = hash_table_start_index(OrderedHashMap::PREFIX_SIZE) + bucket;
    let raw = self.read_slot(slot);
    if raw.is_smi() {
      Smi::value(raw.cast::<Smi>())
    } else {
      K_NOT_FOUND
    }
  }

  #[inline]
  fn set_bucket(&self, bucket: usize, entry: i32) {
    let slot = hash_table_start_index(OrderedHashMap::PREFIX_SIZE) + bucket;
    self.write_slot(slot, Smi::from_int(entry).cast());
  }

  #[inline]
  fn entry_index(&self, entry: usize) -> usize {
    entry_to_index(
      OrderedHashMap::PREFIX_SIZE,
      self.number_of_buckets(),
      entry,
      OrderedHashMap::ENTRY_SIZE,
    )
  }

  #[inline]
  fn key_at(&self, entry: InternalIndex) -> Tagged<Object> {
    debug_assert!(entry.as_usize() < self.used_capacity());
    self.read_slot(self.entry_index(entry.as_usize()))
  }

  #[inline]
  fn value_at(&self, entry: InternalIndex) -> Tagged<Object> {
    debug_assert!(entry.as_usize() < self.used_capacity());
    self.read_slot(self.entry_index(entry.as_usize()) + OrderedHashMap::VALUE_OFFSET)
  }

  #[inline]
  fn set_entry(&self, entry: usize, key: Tagged<Object>, value: Tagged<Object>) {
    let idx = self.entry_index(entry);
    self.write_slot(idx, key);
    self.write_slot(idx + OrderedHashMap::VALUE_OFFSET, value);
  }

  #[inline]
  fn chain_at(&self, entry: usize) -> i32 {
    let raw = self.read_slot(self.entry_index(entry) + OrderedHashMap::CHAIN_OFFSET);
    if raw.is_smi() {
      Smi::value(raw.cast::<Smi>())
    } else {
      K_NOT_FOUND
    }
  }

  #[inline]
  fn set_chain_at(&self, entry: usize, next_entry: i32) {
    self.write_slot(
      self.entry_index(entry) + OrderedHashMap::CHAIN_OFFSET,
      Smi::from_int(next_entry).cast(),
    );
  }

  fn find_entry(&self, key: Tagged<Object>) -> InternalIndex {
    if self.number_of_elements() == 0 || self.capacity() == 0 {
      return InternalIndex::NOT_FOUND;
    }
    let hash = compute_object_hash(key);
    let bucket = hash_to_bucket(hash, self.number_of_buckets());
    let mut raw_entry = self.get_bucket(bucket);

    while raw_entry >= 0 && (raw_entry as usize) < self.used_capacity() {
      let entry_idx = raw_entry as usize;
      let candidate = self.key_at(InternalIndex::new(entry_idx));
      if keys_equal(candidate, key) {
        return InternalIndex::new(entry_idx);
      }
      raw_entry = self.chain_at(entry_idx);
    }
    InternalIndex::NOT_FOUND
  }

  #[inline]
  fn has_key(&self, key: Tagged<Object>) -> bool {
    self.find_entry(key).is_found()
  }

  #[inline]
  fn get(&self, key: Tagged<Object>) -> Option<Tagged<Object>> {
    let entry = self.find_entry(key);
    if entry.is_found() {
      Some(self.value_at(entry))
    } else {
      None
    }
  }

  fn add<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
    key: Tagged<Object>,
    value: Tagged<Object>,
  ) -> Result<Tagged<OrderedHashMap>> {
    let entry = self.find_entry(key);
    if entry.is_found() {
      self.set_entry(entry.as_usize(), key, value);
      return Ok(*self);
    }

    let mut current = *self;
    if current.used_capacity() >= current.capacity() {
      let capacity = current.capacity();
      let nod = current.number_of_deleted_elements();
      let new_capacity = if capacity == 0 {
        K_INITIAL_CAPACITY
      } else if nod >= (capacity >> 1) {
        capacity
      } else {
        capacity << 1
      };
      current = current.rehash(allocator, map, new_capacity)?;
    }

    let nof = current.number_of_elements();
    let nod = current.number_of_deleted_elements();
    let new_entry = nof + nod;

    let hash = compute_object_hash(key);
    let bucket = hash_to_bucket(hash, current.number_of_buckets());
    let prev_entry = current.get_bucket(bucket);

    current.set_entry(new_entry, key, value);
    current.set_chain_at(new_entry, prev_entry);
    current.set_bucket(bucket, new_entry as i32);
    current.set_number_of_elements(nof + 1);

    Ok(current)
  }

  fn delete(&self, key: Tagged<Object>) -> bool {
    let entry = self.find_entry(key);
    if entry.is_not_found() {
      return false;
    }
    let raw = entry.as_usize();
    self.set_entry(raw, THE_HOLE, THE_HOLE);
    self.set_number_of_elements(self.number_of_elements() - 1);
    self.set_number_of_deleted_elements(self.number_of_deleted_elements() + 1);
    true
  }

  fn shrink<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
  ) -> Result<Tagged<OrderedHashMap>> {
    let nof = self.number_of_elements();
    let cap = self.capacity();
    if nof >= (cap >> 2) || (cap >> 1) < K_INITIAL_CAPACITY {
      return Ok(*self);
    }
    self.rehash(allocator, map, cap >> 1)
  }

  fn clear<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
  ) -> Result<Tagged<OrderedHashMap>> {
    let new_table = OrderedHashMap::allocate(allocator, map, K_INITIAL_CAPACITY)?;
    if self.number_of_buckets() > 0 {
      self.write_slot(
        number_of_elements_index(OrderedHashMap::PREFIX_SIZE),
        new_table.cast(),
      );
      self.set_number_of_deleted_elements(K_CLEARED_TABLE_SENTINEL as isize as usize);
    }
    Ok(new_table)
  }

  fn rehash<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
    new_capacity: usize,
  ) -> Result<Tagged<OrderedHashMap>> {
    let mut entries = Vec::with_capacity(self.number_of_elements());
    let old_has_buckets = self.number_of_buckets() > 0;
    let mut removed_holes_index = 0;

    for old_entry in 0..self.used_capacity() {
      let key = self.key_at(InternalIndex::new(old_entry));
      if is_the_hole(key) {
        if old_has_buckets {
          self.set_removed_index_at(removed_holes_index, old_entry);
        }
        removed_holes_index += 1;
      } else {
        let value = self.value_at(InternalIndex::new(old_entry));
        entries.push((key, value));
      }
    }
    let old_nof = self.number_of_elements();

    let new_table = OrderedHashMap::allocate(allocator, map, new_capacity)?;
    for (new_entry, (key, value)) in entries.into_iter().enumerate() {
      let hash = compute_object_hash(key);
      let bucket = hash_to_bucket(hash, new_table.number_of_buckets());
      let chain = new_table.get_bucket(bucket);

      new_table.set_entry(new_entry, key, value);
      new_table.set_chain_at(new_entry, chain);
      new_table.set_bucket(bucket, new_entry as i32);
    }

    new_table.set_number_of_elements(old_nof);
    if old_has_buckets {
      self.write_slot(
        number_of_elements_index(OrderedHashMap::PREFIX_SIZE),
        new_table.cast(),
      );
    }
    Ok(new_table)
  }

  #[inline]
  fn next_table(&self) -> Option<Tagged<OrderedHashMap>> {
    if self.is_obsolete() {
      let raw = self.read_slot(number_of_elements_index(OrderedHashMap::PREFIX_SIZE));
      Some(Tagged::new(raw.ptr()))
    } else {
      None
    }
  }

  #[inline]
  fn set_removed_index_at(&self, index: usize, removed_index: usize) {
    let slot = hash_table_start_index(OrderedHashMap::PREFIX_SIZE) + index;
    self.write_slot(slot, Smi::from_int(removed_index as i32).cast());
  }

  #[inline]
  fn removed_index_at(&self, index: usize) -> usize {
    let slot = hash_table_start_index(OrderedHashMap::PREFIX_SIZE) + index;
    let raw = self.read_slot(slot);
    if raw.is_smi() {
      Smi::value(raw.cast::<Smi>()).max(0) as usize
    } else {
      0
    }
  }

  #[inline]
  fn slot_addr(&self, index: usize) -> usize {
    self.address() + FixedArray::OBJECTS_OFFSET + index * mem::size_of::<usize>()
  }

  #[inline]
  fn read_slot(&self, index: usize) -> Tagged<Object> {
    // SAFETY: slot_addr 计算自 FixedArray 对象体，按 usize 边界自然对齐且在有效对象内存范围内
    let raw =
      unsafe { AtomicUsize::from_ptr(self.slot_addr(index) as *mut usize) }.load(Ordering::Relaxed);
    Tagged::new(raw)
  }

  #[inline]
  fn write_slot(&self, index: usize, val: Tagged<Object>) {
    // SAFETY: slot_addr 处于有效且对齐的 FixedArray 元素区，原子存储符合并发访问契约
    unsafe { AtomicUsize::from_ptr(self.slot_addr(index) as *mut usize) }
      .store(val.ptr(), Ordering::Relaxed);
  }

  #[inline]
  fn iter(&self) -> OrderedHashMapIterator {
    OrderedHashMapIterator {
      table: *self,
      range: InternalIndexRange::new(self.used_capacity()),
    }
  }
}

pub struct OrderedHashMapIterator {
  table: Tagged<OrderedHashMap>,
  range: InternalIndexRange,
}

impl Iterator for OrderedHashMapIterator {
  type Item = (Tagged<Object>, Tagged<Object>);

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    for idx in self.range.by_ref() {
      let key = self.table.key_at(idx);
      if !is_the_hole(key) {
        let val = self.table.value_at(idx);
        return Some((key, val));
      }
    }
    None
  }
}

// ── 安全封装类型 ──────────────────────────────────────────────────────────
/// 自持内存缓冲区的安全 OrderedHashSet
pub struct OrderedHashSetTable {
  storage: HeapStorage,
  map: Tagged<Map>,
  table: Tagged<OrderedHashSet>,
}

impl OrderedHashSetTable {
  pub fn new(map: Tagged<Map>) -> Result<Self> {
    let mut storage = HeapStorage::new();
    let table = OrderedHashSet::allocate(&mut storage, map, K_INITIAL_CAPACITY)?;
    Ok(Self {
      storage,
      map,
      table,
    })
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.table.number_of_elements()
  }

  #[inline]
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  #[inline]
  pub fn capacity(&self) -> usize {
    self.table.capacity()
  }

  #[inline]
  pub fn contains(&self, key: Tagged<Object>) -> bool {
    self.table.has_key(key)
  }

  pub fn insert(&mut self, key: Tagged<Object>) -> Result<bool> {
    if self.table.has_key(key) {
      return Ok(false);
    }
    let new_table = self.table.add(&mut self.storage, self.map, key)?;
    self.table = new_table;
    Ok(true)
  }

  pub fn remove(&mut self, key: Tagged<Object>) -> bool {
    let removed = self.table.delete(key);
    if removed && let Ok(new_table) = self.table.shrink(&mut self.storage, self.map) {
      self.table = new_table;
    }
    removed
  }

  pub fn clear(&mut self) -> Result<()> {
    let new_table = self.table.clear(&mut self.storage, self.map)?;
    self.table = new_table;
    Ok(())
  }

  #[inline]
  pub fn iter(&self) -> OrderedHashSetIterator {
    self.table.iter()
  }

  #[inline]
  pub fn tagged(&self) -> Tagged<OrderedHashSet> {
    self.table
  }
}

/// 自持内存缓冲区的安全 OrderedHashMap
pub struct OrderedHashMapTable {
  storage: HeapStorage,
  map: Tagged<Map>,
  table: Tagged<OrderedHashMap>,
}

impl OrderedHashMapTable {
  pub fn new(map: Tagged<Map>) -> Result<Self> {
    let mut storage = HeapStorage::new();
    let table = OrderedHashMap::allocate(&mut storage, map, K_INITIAL_CAPACITY)?;
    Ok(Self {
      storage,
      map,
      table,
    })
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.table.number_of_elements()
  }

  #[inline]
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  #[inline]
  pub fn capacity(&self) -> usize {
    self.table.capacity()
  }

  #[inline]
  pub fn contains_key(&self, key: Tagged<Object>) -> bool {
    self.table.has_key(key)
  }

  #[inline]
  pub fn get(&self, key: Tagged<Object>) -> Option<Tagged<Object>> {
    self.table.get(key)
  }

  pub fn insert(
    &mut self,
    key: Tagged<Object>,
    value: Tagged<Object>,
  ) -> Result<Option<Tagged<Object>>> {
    let entry = self.table.find_entry(key);
    if entry.is_found() {
      let prev = self.table.value_at(entry);
      self.table.set_entry(entry.as_usize(), key, value);
      return Ok(Some(prev));
    }
    let new_table = self.table.add(&mut self.storage, self.map, key, value)?;
    self.table = new_table;
    Ok(None)
  }

  pub fn remove(&mut self, key: Tagged<Object>) -> bool {
    let removed = self.table.delete(key);
    if removed && let Ok(new_table) = self.table.shrink(&mut self.storage, self.map) {
      self.table = new_table;
    }
    removed
  }

  pub fn clear(&mut self) -> Result<()> {
    let new_table = self.table.clear(&mut self.storage, self.map)?;
    self.table = new_table;
    Ok(())
  }

  #[inline]
  pub fn iter(&self) -> OrderedHashMapIterator {
    self.table.iter()
  }

  #[inline]
  pub fn tagged(&self) -> Tagged<OrderedHashMap> {
    self.table
  }
}

// ── OrderedNameDictionary ─────────────────────────────────────────────────
/// OrderedNameDictionary (entrysize = 3, kEntrySize = 4 [key, value, details, chain])
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct OrderedNameDictionary {
  pub base: FixedArray,
}

impl OrderedNameDictionary {
  pub const ENTRY_SIZE_WITHOUT_CHAIN: usize = 3;
  pub const ENTRY_SIZE: usize = 4;
  pub const VALUE_OFFSET: usize = 1;
  pub const DETAILS_OFFSET: usize = 2;
  pub const CHAIN_OFFSET: usize = 3;
  pub const PREFIX_SIZE: usize = 1;
  pub const HASH_INDEX: usize = 0;

  #[inline]
  pub const fn size_for(capacity: usize) -> usize {
    let num_buckets = capacity / K_LOAD_FACTOR;
    let slots = total_slots(Self::PREFIX_SIZE, num_buckets, capacity, Self::ENTRY_SIZE);
    FixedArray::size_for(slots as u32)
  }

  /// 分配并初始化 OrderedNameDictionary
  pub fn allocate<A: MemoryAllocator>(
    allocator: &mut A,
    map: Tagged<Map>,
    capacity: usize,
  ) -> Result<Tagged<Self>> {
    let cap = capacity.max(K_INITIAL_CAPACITY).next_power_of_two();
    let num_buckets = cap / K_LOAD_FACTOR;
    let slots = total_slots(
      OrderedNameDictionary::PREFIX_SIZE,
      num_buckets,
      cap,
      OrderedNameDictionary::ENTRY_SIZE,
    );
    let size = FixedArray::size_for(slots as u32);
    let addr = allocator.allocate(size)?;

    let header = FixedArray::new(map, slots as u32);
    // SAFETY: addr 由分配器成功分配并对齐，写入 FixedArray 头部并在随后完成插槽初始化
    unsafe {
      write(addr as *mut FixedArray, header);
      let table: Tagged<Self> = Tagged::from_raw_address(addr);
      table.set_hash(0);
      table.set_number_of_buckets(num_buckets);
      table.set_number_of_elements(0);
      table.set_number_of_deleted_elements(0);
      for b in 0..num_buckets {
        table.set_bucket(b, K_NOT_FOUND);
      }
      for entry in 0..cap {
        table.set_entry(entry, THE_HOLE, THE_HOLE, PropertyDetails::empty());
        table.set_chain_at(entry, K_NOT_FOUND);
      }
      Ok(table)
    }
  }
}

pub trait OrderedNameDictionaryExt: Copy {
  fn hash(&self) -> i32;
  fn set_hash(&self, h: i32);
  fn number_of_elements(&self) -> usize;
  fn set_number_of_elements(&self, n: usize);
  fn number_of_deleted_elements(&self) -> usize;
  fn set_number_of_deleted_elements(&self, n: usize);
  fn number_of_buckets(&self) -> usize;
  fn set_number_of_buckets(&self, n: usize);
  fn capacity(&self) -> usize;
  fn used_capacity(&self) -> usize;
  fn is_obsolete(&self) -> bool;
  fn next_table(&self) -> Option<Tagged<OrderedNameDictionary>>;
  fn set_removed_index_at(&self, index: usize, removed_index: usize);
  fn removed_index_at(&self, index: usize) -> usize;
  fn get_bucket(&self, bucket: usize) -> i32;
  fn set_bucket(&self, bucket: usize, entry: i32);
  fn entry_index(&self, entry: usize) -> usize;
  fn key_at(&self, entry: InternalIndex) -> Tagged<Object>;
  fn value_at(&self, entry: InternalIndex) -> Tagged<Object>;
  fn details_at(&self, entry: InternalIndex) -> PropertyDetails;
  fn set_entry(
    &self,
    entry: usize,
    key: Tagged<Object>,
    value: Tagged<Object>,
    details: PropertyDetails,
  );
  fn chain_at(&self, entry: usize) -> i32;
  fn set_chain_at(&self, entry: usize, next_entry: i32);
  fn find_entry(&self, key: Tagged<Object>) -> InternalIndex;
  fn has_key(&self, key: Tagged<Object>) -> bool;
  fn get(&self, key: Tagged<Object>) -> Option<(Tagged<Object>, PropertyDetails)>;
  fn add<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
    key: Tagged<Object>,
    value: Tagged<Object>,
    details: PropertyDetails,
  ) -> Result<Tagged<OrderedNameDictionary>>;
  fn delete_entry(&self, entry: InternalIndex) -> bool;
  fn delete(&self, key: Tagged<Object>) -> bool;
  fn shrink<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
  ) -> Result<Tagged<OrderedNameDictionary>>;
  fn clear<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
  ) -> Result<Tagged<OrderedNameDictionary>>;
  fn rehash<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
    new_capacity: usize,
  ) -> Result<Tagged<OrderedNameDictionary>>;
  fn slot_addr(&self, index: usize) -> usize;
  fn read_slot(&self, index: usize) -> Tagged<Object>;
  fn write_slot(&self, index: usize, val: Tagged<Object>);
  fn iter(&self) -> OrderedNameDictionaryIterator;
}

impl OrderedNameDictionaryExt for Tagged<OrderedNameDictionary> {
  #[inline]
  fn hash(&self) -> i32 {
    let raw = self.read_slot(OrderedNameDictionary::HASH_INDEX);
    if raw.is_smi() {
      Smi::value(raw.cast::<Smi>())
    } else {
      0
    }
  }

  #[inline]
  fn set_hash(&self, h: i32) {
    self.write_slot(OrderedNameDictionary::HASH_INDEX, Smi::from_int(h).cast());
  }

  #[inline]
  fn number_of_elements(&self) -> usize {
    let raw = self.read_slot(number_of_elements_index(OrderedNameDictionary::PREFIX_SIZE));
    if raw.is_smi() {
      Smi::value(raw.cast::<Smi>()).max(0) as usize
    } else {
      0
    }
  }

  #[inline]
  fn set_number_of_elements(&self, n: usize) {
    self.write_slot(
      number_of_elements_index(OrderedNameDictionary::PREFIX_SIZE),
      Smi::from_int(n as i32).cast(),
    );
  }

  #[inline]
  fn number_of_deleted_elements(&self) -> usize {
    let raw = self.read_slot(number_of_deleted_elements_index(
      OrderedNameDictionary::PREFIX_SIZE,
    ));
    if raw.is_smi() {
      Smi::value(raw.cast::<Smi>()) as isize as usize
    } else {
      0
    }
  }

  #[inline]
  fn set_number_of_deleted_elements(&self, n: usize) {
    self.write_slot(
      number_of_deleted_elements_index(OrderedNameDictionary::PREFIX_SIZE),
      Smi::from_int(n as i32).cast(),
    );
  }

  #[inline]
  fn number_of_buckets(&self) -> usize {
    let raw = self.read_slot(number_of_buckets_index(OrderedNameDictionary::PREFIX_SIZE));
    if raw.is_smi() {
      Smi::value(raw.cast::<Smi>()).max(0) as usize
    } else {
      0
    }
  }

  #[inline]
  fn set_number_of_buckets(&self, n: usize) {
    self.write_slot(
      number_of_buckets_index(OrderedNameDictionary::PREFIX_SIZE),
      Smi::from_int(n as i32).cast(),
    );
  }

  #[inline]
  fn capacity(&self) -> usize {
    calc_capacity(self.number_of_buckets())
  }

  #[inline]
  fn used_capacity(&self) -> usize {
    let nod = self.number_of_deleted_elements();
    if nod == K_CLEARED_TABLE_SENTINEL as isize as usize {
      0
    } else {
      self.number_of_elements() + nod
    }
  }

  #[inline]
  fn is_obsolete(&self) -> bool {
    let raw = self.read_slot(number_of_elements_index(OrderedNameDictionary::PREFIX_SIZE));
    !raw.is_smi()
  }

  #[inline]
  fn next_table(&self) -> Option<Tagged<OrderedNameDictionary>> {
    if self.is_obsolete() {
      let raw = self.read_slot(number_of_elements_index(OrderedNameDictionary::PREFIX_SIZE));
      if raw.is_smi() || is_the_hole(raw) {
        None
      } else {
        Some(Tagged::new(raw.ptr()))
      }
    } else {
      None
    }
  }

  #[inline]
  fn set_removed_index_at(&self, index: usize, removed_index: usize) {
    let slot = hash_table_start_index(OrderedNameDictionary::PREFIX_SIZE) + index;
    self.write_slot(slot, Smi::from_int(removed_index as i32).cast());
  }

  #[inline]
  fn removed_index_at(&self, index: usize) -> usize {
    let slot = hash_table_start_index(OrderedNameDictionary::PREFIX_SIZE) + index;
    let raw = self.read_slot(slot);
    if raw.is_smi() {
      Smi::value(raw.cast::<Smi>()).max(0) as usize
    } else {
      0
    }
  }

  #[inline]
  fn get_bucket(&self, bucket: usize) -> i32 {
    debug_assert!(bucket < self.number_of_buckets());
    let raw = self.read_slot(hash_table_start_index(OrderedNameDictionary::PREFIX_SIZE) + bucket);
    if raw.is_smi() {
      Smi::value(raw.cast::<Smi>())
    } else {
      K_NOT_FOUND
    }
  }

  #[inline]
  fn set_bucket(&self, bucket: usize, entry: i32) {
    debug_assert!(bucket < self.number_of_buckets());
    self.write_slot(
      hash_table_start_index(OrderedNameDictionary::PREFIX_SIZE) + bucket,
      Smi::from_int(entry).cast(),
    );
  }

  #[inline]
  fn entry_index(&self, entry: usize) -> usize {
    entry_to_index(
      OrderedNameDictionary::PREFIX_SIZE,
      self.number_of_buckets(),
      entry,
      OrderedNameDictionary::ENTRY_SIZE,
    )
  }

  #[inline]
  fn key_at(&self, entry: InternalIndex) -> Tagged<Object> {
    debug_assert!(entry.as_usize() < self.used_capacity());
    self.read_slot(self.entry_index(entry.as_usize()))
  }

  #[inline]
  fn value_at(&self, entry: InternalIndex) -> Tagged<Object> {
    debug_assert!(entry.as_usize() < self.used_capacity());
    self.read_slot(self.entry_index(entry.as_usize()) + OrderedNameDictionary::VALUE_OFFSET)
  }

  #[inline]
  fn details_at(&self, entry: InternalIndex) -> PropertyDetails {
    debug_assert!(entry.as_usize() < self.used_capacity());
    let raw =
      self.read_slot(self.entry_index(entry.as_usize()) + OrderedNameDictionary::DETAILS_OFFSET);
    if raw.is_smi() {
      PropertyDetails::from_smi(raw.cast::<Smi>())
    } else {
      PropertyDetails::empty()
    }
  }

  #[inline]
  fn set_entry(
    &self,
    entry: usize,
    key: Tagged<Object>,
    value: Tagged<Object>,
    details: PropertyDetails,
  ) {
    let idx = self.entry_index(entry);
    self.write_slot(idx, key);
    self.write_slot(idx + OrderedNameDictionary::VALUE_OFFSET, value);
    self.write_slot(
      idx + OrderedNameDictionary::DETAILS_OFFSET,
      details.as_smi().cast(),
    );
  }

  #[inline]
  fn chain_at(&self, entry: usize) -> i32 {
    let raw = self.read_slot(self.entry_index(entry) + OrderedNameDictionary::CHAIN_OFFSET);
    if raw.is_smi() {
      Smi::value(raw.cast::<Smi>())
    } else {
      K_NOT_FOUND
    }
  }

  #[inline]
  fn set_chain_at(&self, entry: usize, next_entry: i32) {
    self.write_slot(
      self.entry_index(entry) + OrderedNameDictionary::CHAIN_OFFSET,
      Smi::from_int(next_entry).cast(),
    );
  }

  fn find_entry(&self, key: Tagged<Object>) -> InternalIndex {
    if self.number_of_elements() == 0 || self.capacity() == 0 {
      return InternalIndex::NOT_FOUND;
    }
    let hash = compute_object_hash(key);
    let bucket = hash_to_bucket(hash, self.number_of_buckets());
    let mut raw_entry = self.get_bucket(bucket);

    while raw_entry >= 0 && (raw_entry as usize) < self.used_capacity() {
      let entry_idx = raw_entry as usize;
      let candidate = self.key_at(InternalIndex::new(entry_idx));
      if keys_equal(candidate, key) {
        return InternalIndex::new(entry_idx);
      }
      raw_entry = self.chain_at(entry_idx);
    }
    InternalIndex::NOT_FOUND
  }

  #[inline]
  fn has_key(&self, key: Tagged<Object>) -> bool {
    self.find_entry(key).is_found()
  }

  #[inline]
  fn get(&self, key: Tagged<Object>) -> Option<(Tagged<Object>, PropertyDetails)> {
    let entry = self.find_entry(key);
    if entry.is_found() {
      Some((self.value_at(entry), self.details_at(entry)))
    } else {
      None
    }
  }

  fn add<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
    key: Tagged<Object>,
    value: Tagged<Object>,
    details: PropertyDetails,
  ) -> Result<Tagged<OrderedNameDictionary>> {
    let entry = self.find_entry(key);
    if entry.is_found() {
      self.set_entry(entry.as_usize(), key, value, details);
      return Ok(*self);
    }

    let mut current = *self;
    if current.used_capacity() >= current.capacity() {
      let capacity = current.capacity();
      let nod = current.number_of_deleted_elements();
      let new_capacity = if capacity == 0 {
        K_INITIAL_CAPACITY
      } else if nod >= (capacity >> 1) {
        capacity
      } else {
        capacity << 1
      };
      current = current.rehash(allocator, map, new_capacity)?;
    }

    let nof = current.number_of_elements();
    let nod = current.number_of_deleted_elements();
    let new_entry = nof + nod;

    let hash = compute_object_hash(key);
    let bucket = hash_to_bucket(hash, current.number_of_buckets());
    let prev_entry = current.get_bucket(bucket);

    current.set_entry(new_entry, key, value, details);
    current.set_chain_at(new_entry, prev_entry);
    current.set_bucket(bucket, new_entry as i32);
    current.set_number_of_elements(nof + 1);

    Ok(current)
  }

  fn delete_entry(&self, entry: InternalIndex) -> bool {
    if entry.is_not_found() || entry.as_usize() >= self.used_capacity() {
      return false;
    }
    let raw = entry.as_usize();
    let current_key = self.key_at(entry);
    if is_the_hole(current_key) {
      return false;
    }
    self.set_entry(raw, THE_HOLE, THE_HOLE, PropertyDetails::empty());
    self.set_number_of_elements(self.number_of_elements() - 1);
    self.set_number_of_deleted_elements(self.number_of_deleted_elements() + 1);
    true
  }

  #[inline]
  fn delete(&self, key: Tagged<Object>) -> bool {
    let entry = self.find_entry(key);
    self.delete_entry(entry)
  }

  fn shrink<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
  ) -> Result<Tagged<OrderedNameDictionary>> {
    let nof = self.number_of_elements();
    let cap = self.capacity();
    if nof >= (cap >> 2) || (cap >> 1) < K_INITIAL_CAPACITY {
      return Ok(*self);
    }
    self.rehash(allocator, map, cap >> 1)
  }

  fn clear<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
  ) -> Result<Tagged<OrderedNameDictionary>> {
    let new_table = OrderedNameDictionary::allocate(allocator, map, K_INITIAL_CAPACITY)?;
    if self.number_of_buckets() > 0 {
      self.write_slot(
        number_of_elements_index(OrderedNameDictionary::PREFIX_SIZE),
        new_table.cast(),
      );
      self.set_number_of_deleted_elements(K_CLEARED_TABLE_SENTINEL as isize as usize);
    }
    Ok(new_table)
  }

  fn rehash<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
    new_capacity: usize,
  ) -> Result<Tagged<OrderedNameDictionary>> {
    let mut entries = Vec::with_capacity(self.number_of_elements());
    let old_has_buckets = self.number_of_buckets() > 0;
    let mut removed_holes_index = 0;

    for old_entry in 0..self.used_capacity() {
      let key = self.key_at(InternalIndex::new(old_entry));
      if is_the_hole(key) {
        if old_has_buckets {
          self.set_removed_index_at(removed_holes_index, old_entry);
        }
        removed_holes_index += 1;
      } else {
        let value = self.value_at(InternalIndex::new(old_entry));
        let details = self.details_at(InternalIndex::new(old_entry));
        entries.push((key, value, details));
      }
    }
    let old_nof = self.number_of_elements();
    let old_hash = self.hash();

    let new_table = OrderedNameDictionary::allocate(allocator, map, new_capacity)?;
    new_table.set_hash(old_hash);

    for (new_entry, (key, value, details)) in entries.into_iter().enumerate() {
      let hash = compute_object_hash(key);
      let bucket = hash_to_bucket(hash, new_table.number_of_buckets());
      let chain = new_table.get_bucket(bucket);

      new_table.set_entry(new_entry, key, value, details);
      new_table.set_chain_at(new_entry, chain);
      new_table.set_bucket(bucket, new_entry as i32);
    }

    new_table.set_number_of_elements(old_nof);
    if old_has_buckets {
      self.write_slot(
        number_of_elements_index(OrderedNameDictionary::PREFIX_SIZE),
        new_table.cast(),
      );
    }
    Ok(new_table)
  }

  #[inline]
  fn slot_addr(&self, index: usize) -> usize {
    self.address() + FixedArray::OBJECTS_OFFSET + index * mem::size_of::<usize>()
  }

  #[inline]
  fn read_slot(&self, index: usize) -> Tagged<Object> {
    // SAFETY: slot_addr 处于有效且对齐的 FixedArray 元素区，原子加载符合并发访问契约
    let ptr =
      unsafe { AtomicUsize::from_ptr(self.slot_addr(index) as *mut usize) }.load(Ordering::Relaxed);
    Tagged::new(ptr)
  }

  #[inline]
  fn write_slot(&self, index: usize, val: Tagged<Object>) {
    // SAFETY: slot_addr 处于有效且对齐的 FixedArray 元素区，原子存储符合并发访问契约
    unsafe { AtomicUsize::from_ptr(self.slot_addr(index) as *mut usize) }
      .store(val.ptr(), Ordering::Relaxed);
  }

  #[inline]
  fn iter(&self) -> OrderedNameDictionaryIterator {
    OrderedNameDictionaryIterator {
      table: *self,
      range: InternalIndexRange::new(self.used_capacity()),
    }
  }
}

pub struct OrderedNameDictionaryIterator {
  table: Tagged<OrderedNameDictionary>,
  range: InternalIndexRange,
}

impl Iterator for OrderedNameDictionaryIterator {
  type Item = (Tagged<Object>, Tagged<Object>, PropertyDetails);

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    for idx in self.range.by_ref() {
      let key = self.table.key_at(idx);
      if !is_the_hole(key) {
        let value = self.table.value_at(idx);
        let details = self.table.details_at(idx);
        return Some((key, value, details));
      }
    }
    None
  }
}

/// 自持内存缓冲区的安全 OrderedNameDictionary
pub struct OrderedNameDictionaryTable {
  storage: HeapStorage,
  map: Tagged<Map>,
  table: Tagged<OrderedNameDictionary>,
}

impl OrderedNameDictionaryTable {
  pub fn new(map: Tagged<Map>) -> Result<Self> {
    let mut storage = HeapStorage::new();
    let table = OrderedNameDictionary::allocate(&mut storage, map, K_INITIAL_CAPACITY)?;
    Ok(Self {
      storage,
      map,
      table,
    })
  }

  #[inline]
  pub fn hash(&self) -> i32 {
    self.table.hash()
  }

  #[inline]
  pub fn set_hash(&mut self, h: i32) {
    self.table.set_hash(h);
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.table.number_of_elements()
  }

  #[inline]
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  #[inline]
  pub fn capacity(&self) -> usize {
    self.table.capacity()
  }

  #[inline]
  pub fn contains_key(&self, key: Tagged<Object>) -> bool {
    self.table.has_key(key)
  }

  #[inline]
  pub fn get(&self, key: Tagged<Object>) -> Option<(Tagged<Object>, PropertyDetails)> {
    self.table.get(key)
  }

  pub fn insert(
    &mut self,
    key: Tagged<Object>,
    value: Tagged<Object>,
    details: PropertyDetails,
  ) -> Result<Option<Tagged<Object>>> {
    let entry = self.table.find_entry(key);
    if entry.is_found() {
      let prev = self.table.value_at(entry);
      self.table.set_entry(entry.as_usize(), key, value, details);
      return Ok(Some(prev));
    }
    let new_table = self
      .table
      .add(&mut self.storage, self.map, key, value, details)?;
    self.table = new_table;
    Ok(None)
  }

  pub fn remove(&mut self, key: Tagged<Object>) -> bool {
    let removed = self.table.delete(key);
    if removed && let Ok(new_table) = self.table.shrink(&mut self.storage, self.map) {
      self.table = new_table;
    }
    removed
  }

  pub fn clear(&mut self) -> Result<()> {
    let new_table = self.table.clear(&mut self.storage, self.map)?;
    self.table = new_table;
    Ok(())
  }

  #[inline]
  pub fn iter(
    &self,
  ) -> impl Iterator<Item = (Tagged<Object>, Tagged<Object>, PropertyDetails)> + '_ {
    self.table.iter()
  }

  #[inline]
  pub fn tagged(&self) -> Tagged<OrderedNameDictionary> {
    self.table
  }
}
