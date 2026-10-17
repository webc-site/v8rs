//! 紧凑字节宽度有序哈希表: SmallOrderedHashSet / SmallOrderedHashMap
//! 对应原版: v8/src/objects/ordered-hash-table.h + ordered-hash-table-inl.h + ordered-hash-table.cc

use std::{
  mem, ptr,
  sync::atomic::{AtomicUsize, Ordering},
};

use v8_objects_core::{
  map::Map,
  tagged::{Object, Tagged},
};

use crate::{
  allocator::{HeapStorage, MemoryAllocator},
  error::{Error, Result},
  internal_index::{InternalIndex, InternalIndexRange},
  ordered_hash_table::{K_LOAD_FACTOR, THE_HOLE, compute_object_hash, is_the_hole, keys_equal},
};

// ── 核心常量 ─────────────────────────────────────────────────────────────
pub const K_SMALL_MIN_CAPACITY: usize = 4;
pub const K_SMALL_MAX_CAPACITY: usize = 254;
pub const K_SMALL_NOT_FOUND: u8 = 0xFF;
pub const K_SMALL_LOAD_FACTOR: usize = K_LOAD_FACTOR;
pub const K_SMALL_GROWTH_HACK: usize = 256;
pub const TAGGED_SLOT_SIZE: usize = mem::size_of::<usize>();

// ── 头部与内存布局 ───────────────────────────────────────────────────────
/// SmallOrderedHashTable 头部结构体 (16 字节，天然 8 字节对齐)
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SmallOrderedHashTableHead {
  pub map: Tagged<Map>,
  pub number_of_elements: u8,
  pub number_of_deleted_elements: u8,
  pub number_of_buckets: u8,
  pub padding: [u8; 5],
}

const _: () = assert!(mem::size_of::<SmallOrderedHashTableHead>() == 16);

#[inline]
pub const fn data_table_start_offset() -> usize {
  mem::size_of::<SmallOrderedHashTableHead>() // 16
}

#[inline]
pub const fn data_table_size_for(capacity: usize, entry_size: usize) -> usize {
  capacity * entry_size * TAGGED_SLOT_SIZE
}

#[inline]
pub const fn buckets_start_offset(capacity: usize, entry_size: usize) -> usize {
  data_table_start_offset() + data_table_size_for(capacity, entry_size)
}

#[inline]
pub const fn chain_table_offset(capacity: usize, entry_size: usize, num_buckets: usize) -> usize {
  buckets_start_offset(capacity, entry_size) + num_buckets
}

#[inline]
pub const fn size_for_small_table(capacity: usize, entry_size: usize) -> usize {
  let num_buckets = capacity / K_SMALL_LOAD_FACTOR;
  let total = chain_table_offset(capacity, entry_size, num_buckets) + capacity;
  (total + (TAGGED_SLOT_SIZE - 1)) & !(TAGGED_SLOT_SIZE - 1)
}

// ── SmallOrderedHashSet ───────────────────────────────────────────────────
/// SmallOrderedHashSet (kEntrySize = 1 [key])
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SmallOrderedHashSet {
  pub head: SmallOrderedHashTableHead,
}

impl SmallOrderedHashSet {
  pub const KEY_INDEX: usize = 0;
  pub const ENTRY_SIZE: usize = 1;
  pub const PREFIX_SIZE: usize = 0;

  #[inline]
  pub const fn size_for(capacity: usize) -> usize {
    size_for_small_table(capacity, Self::ENTRY_SIZE)
  }

  /// 分配并初始化 SmallOrderedHashSet
  pub fn allocate<A: MemoryAllocator>(
    allocator: &mut A,
    map: Tagged<Map>,
    capacity: usize,
  ) -> Result<Tagged<Self>> {
    let cap = if capacity >= K_SMALL_MAX_CAPACITY || capacity > 128 {
      K_SMALL_MAX_CAPACITY
    } else {
      capacity.max(K_SMALL_MIN_CAPACITY).next_power_of_two()
    };
    let num_buckets = cap / K_SMALL_LOAD_FACTOR;
    let size = size_for_small_table(cap, SmallOrderedHashSet::ENTRY_SIZE);
    let addr = allocator.allocate(size)?;

    let head = SmallOrderedHashTableHead {
      map,
      number_of_elements: 0,
      number_of_deleted_elements: 0,
      number_of_buckets: num_buckets as u8,
      padding: [0; 5],
    };

    // SAFETY: addr 由分配器成功分配并对齐，大小满足 size_for_small_table 计算出的容量所需字节
    unsafe {
      ptr::write(addr as *mut SmallOrderedHashTableHead, head);
      let table: Tagged<Self> = Tagged::from_raw_address(addr);
      for b in 0..num_buckets {
        table.set_bucket(b, K_SMALL_NOT_FOUND);
      }
      for entry in 0..cap {
        table.set_key_at(entry, THE_HOLE);
        table.set_chain(entry, K_SMALL_NOT_FOUND);
      }
      Ok(table)
    }
  }
}

pub trait SmallOrderedHashSetExt: Copy {
  fn head(&self) -> &SmallOrderedHashTableHead;
  fn data_slot_addr(&self, entry: usize) -> usize;
  fn bucket_addr(&self, bucket: usize) -> usize;
  fn chain_addr(&self, entry: usize) -> usize;
  fn number_of_elements(&self) -> usize;
  fn set_number_of_elements(&self, n: usize);
  fn number_of_deleted_elements(&self) -> usize;
  fn set_number_of_deleted_elements(&self, n: usize);
  fn number_of_buckets(&self) -> usize;
  fn set_number_of_buckets(&self, n: usize);
  fn capacity(&self) -> usize;
  fn used_capacity(&self) -> usize;
  fn key_at(&self, entry: InternalIndex) -> Tagged<Object>;
  fn set_key_at(&self, entry: usize, key: Tagged<Object>);
  fn get_bucket(&self, bucket: usize) -> u8;
  fn set_bucket(&self, bucket: usize, entry: u8);
  fn get_chain(&self, entry: usize) -> u8;
  fn set_chain(&self, entry: usize, next_entry: u8);
  fn find_entry(&self, key: Tagged<Object>) -> InternalIndex;
  fn has_key(&self, key: Tagged<Object>) -> bool;
  fn grow<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
  ) -> Result<Tagged<SmallOrderedHashSet>>;
  fn add<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
    key: Tagged<Object>,
  ) -> Result<Tagged<SmallOrderedHashSet>>;
  fn delete(&self, key: Tagged<Object>) -> bool;
  fn shrink<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
  ) -> Result<Tagged<SmallOrderedHashSet>>;
  fn rehash<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
    new_capacity: usize,
  ) -> Result<Tagged<SmallOrderedHashSet>>;
  fn iter(&self) -> SmallOrderedHashSetIterator;
}

impl SmallOrderedHashSetExt for Tagged<SmallOrderedHashSet> {
  #[inline]
  fn number_of_elements(&self) -> usize {
    self.head().number_of_elements as usize
  }

  #[inline]
  fn set_number_of_elements(&self, n: usize) {
    let offset = mem::offset_of!(SmallOrderedHashTableHead, number_of_elements);
    // SAFETY: self.address() 指向有效且对齐的 SmallOrderedHashTableHead 结构体
    unsafe {
      *((self.address() + offset) as *mut u8) = n as u8;
    }
  }

  #[inline]
  fn number_of_deleted_elements(&self) -> usize {
    self.head().number_of_deleted_elements as usize
  }

  #[inline]
  fn set_number_of_deleted_elements(&self, n: usize) {
    let offset = mem::offset_of!(SmallOrderedHashTableHead, number_of_deleted_elements);
    // SAFETY: self.address() 指向有效且对齐的 SmallOrderedHashTableHead 结构体
    unsafe {
      *((self.address() + offset) as *mut u8) = n as u8;
    }
  }

  #[inline]
  fn number_of_buckets(&self) -> usize {
    self.head().number_of_buckets as usize
  }

  #[inline]
  fn set_number_of_buckets(&self, n: usize) {
    let offset = mem::offset_of!(SmallOrderedHashTableHead, number_of_buckets);
    // SAFETY: self.address() 指向有效且对齐的 SmallOrderedHashTableHead 结构体
    unsafe {
      *((self.address() + offset) as *mut u8) = n as u8;
    }
  }

  #[inline]
  fn capacity(&self) -> usize {
    self.number_of_buckets() * K_SMALL_LOAD_FACTOR
  }

  #[inline]
  fn used_capacity(&self) -> usize {
    self.number_of_elements() + self.number_of_deleted_elements()
  }

  #[inline]
  fn key_at(&self, entry: InternalIndex) -> Tagged<Object> {
    debug_assert!(entry.as_usize() < self.used_capacity());
    // SAFETY: data_slot_addr 计算自紧跟头部的已对齐数据表槽位，在分配的 DataTable 内存范围内
    let raw = unsafe { AtomicUsize::from_ptr(self.data_slot_addr(entry.as_usize()) as *mut usize) }
      .load(Ordering::Relaxed);
    Tagged::new(raw)
  }

  #[inline]
  fn set_key_at(&self, entry: usize, key: Tagged<Object>) {
    // SAFETY: data_slot_addr 在有效的数据槽位范围内，且按 usize 边界对齐
    unsafe { AtomicUsize::from_ptr(self.data_slot_addr(entry) as *mut usize) }
      .store(key.ptr(), Ordering::Relaxed);
  }

  #[inline]
  fn get_bucket(&self, bucket: usize) -> u8 {
    // SAFETY: bucket_addr 位于 DataTable 之后的 HashTable 字节数组范围内
    unsafe { *(self.bucket_addr(bucket) as *const u8) }
  }

  #[inline]
  fn set_bucket(&self, bucket: usize, entry: u8) {
    // SAFETY: bucket_addr 位于 HashTable 字节数组范围内，具有可写内存权限
    unsafe { *(self.bucket_addr(bucket) as *mut u8) = entry };
  }

  #[inline]
  fn get_chain(&self, entry: usize) -> u8 {
    // SAFETY: chain_addr 位于 HashTable 之后的 Chains 字节数组范围内
    unsafe { *(self.chain_addr(entry) as *const u8) }
  }

  #[inline]
  fn set_chain(&self, entry: usize, next_entry: u8) {
    // SAFETY: chain_addr 位于 Chains 字节数组范围内，具有可写内存权限
    unsafe { *(self.chain_addr(entry) as *mut u8) = next_entry };
  }

  fn find_entry(&self, key: Tagged<Object>) -> InternalIndex {
    if self.number_of_elements() == 0 || self.capacity() == 0 {
      return InternalIndex::NOT_FOUND;
    }
    let hash = compute_object_hash(key);
    let bucket = (hash as usize) & (self.number_of_buckets() - 1);
    let mut raw_entry = self.get_bucket(bucket);

    while raw_entry != K_SMALL_NOT_FOUND && (raw_entry as usize) < self.used_capacity() {
      let entry_idx = raw_entry as usize;
      let candidate = self.key_at(InternalIndex::new(entry_idx));
      if keys_equal(candidate, key) {
        return InternalIndex::new(entry_idx);
      }
      raw_entry = self.get_chain(entry_idx);
    }
    InternalIndex::NOT_FOUND
  }

  #[inline]
  fn has_key(&self, key: Tagged<Object>) -> bool {
    self.find_entry(key).is_found()
  }

  fn grow<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
  ) -> Result<Tagged<SmallOrderedHashSet>> {
    let capacity = self.capacity();
    let nod = self.number_of_deleted_elements();
    let mut new_capacity = capacity;

    if nod < (capacity >> 1) {
      new_capacity = capacity << 1;
      if new_capacity == K_SMALL_GROWTH_HACK {
        new_capacity = K_SMALL_MAX_CAPACITY;
      }
      if new_capacity > K_SMALL_MAX_CAPACITY {
        return Err(Error::TableFull);
      }
    }
    self.rehash(allocator, map, new_capacity)
  }

  fn add<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
    key: Tagged<Object>,
  ) -> Result<Tagged<SmallOrderedHashSet>> {
    let entry = self.find_entry(key);
    if entry.is_found() {
      return Ok(*self);
    }

    let mut current = *self;
    if current.used_capacity() >= current.capacity() {
      current = current.grow(allocator, map)?;
    }

    let nof = current.number_of_elements();
    let nod = current.number_of_deleted_elements();
    let new_entry = nof + nod;

    let hash = compute_object_hash(key);
    let bucket = (hash as usize) & (current.number_of_buckets() - 1);
    let prev_entry = current.get_bucket(bucket);

    current.set_key_at(new_entry, key);
    current.set_chain(new_entry, prev_entry);
    current.set_bucket(bucket, new_entry as u8);
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
  ) -> Result<Tagged<SmallOrderedHashSet>> {
    let nof = self.number_of_elements();
    let cap = self.capacity();
    if nof >= (cap >> 2) || (cap >> 1) < K_SMALL_MIN_CAPACITY {
      return Ok(*self);
    }
    self.rehash(allocator, map, cap >> 1)
  }

  fn rehash<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
    new_capacity: usize,
  ) -> Result<Tagged<SmallOrderedHashSet>> {
    let mut entries = Vec::with_capacity(self.number_of_elements());
    for old_entry in 0..self.used_capacity() {
      let key = self.key_at(InternalIndex::new(old_entry));
      if !is_the_hole(key) {
        entries.push(key);
      }
    }
    let old_nof = self.number_of_elements();

    let new_table = SmallOrderedHashSet::allocate(allocator, map, new_capacity)?;
    for (new_entry, &key) in entries.iter().enumerate() {
      let hash = compute_object_hash(key);
      let bucket = (hash as usize) & (new_table.number_of_buckets() - 1);
      let chain = new_table.get_bucket(bucket);

      new_table.set_key_at(new_entry, key);
      new_table.set_chain(new_entry, chain);
      new_table.set_bucket(bucket, new_entry as u8);
    }

    new_table.set_number_of_elements(old_nof);
    Ok(new_table)
  }

  #[inline]
  fn head(&self) -> &SmallOrderedHashTableHead {
    // SAFETY: self.address() 指向有效且已初始化的 SmallOrderedHashTableHead 结构体
    unsafe { &*(self.address() as *const SmallOrderedHashTableHead) }
  }

  #[inline]
  fn data_slot_addr(&self, entry: usize) -> usize {
    self.address() + data_table_start_offset() + entry * TAGGED_SLOT_SIZE
  }

  #[inline]
  fn bucket_addr(&self, bucket: usize) -> usize {
    self.address() + buckets_start_offset(self.capacity(), SmallOrderedHashSet::ENTRY_SIZE) + bucket
  }

  #[inline]
  fn chain_addr(&self, entry: usize) -> usize {
    self.address()
      + chain_table_offset(
        self.capacity(),
        SmallOrderedHashSet::ENTRY_SIZE,
        self.number_of_buckets(),
      )
      + entry
  }

  #[inline]
  fn iter(&self) -> SmallOrderedHashSetIterator {
    SmallOrderedHashSetIterator {
      table: *self,
      range: InternalIndexRange::new(self.used_capacity()),
    }
  }
}

pub struct SmallOrderedHashSetIterator {
  table: Tagged<SmallOrderedHashSet>,
  range: InternalIndexRange,
}

impl Iterator for SmallOrderedHashSetIterator {
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

// ── SmallOrderedHashMap ───────────────────────────────────────────────────
/// SmallOrderedHashMap (kEntrySize = 2 [key, value])
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SmallOrderedHashMap {
  pub head: SmallOrderedHashTableHead,
}

impl SmallOrderedHashMap {
  pub const KEY_INDEX: usize = 0;
  pub const VALUE_INDEX: usize = 1;
  pub const ENTRY_SIZE: usize = 2;
  pub const PREFIX_SIZE: usize = 0;

  #[inline]
  pub const fn size_for(capacity: usize) -> usize {
    size_for_small_table(capacity, Self::ENTRY_SIZE)
  }

  /// 分配并初始化 SmallOrderedHashMap
  pub fn allocate<A: MemoryAllocator>(
    allocator: &mut A,
    map: Tagged<Map>,
    capacity: usize,
  ) -> Result<Tagged<Self>> {
    let cap = if capacity >= K_SMALL_MAX_CAPACITY || capacity > 128 {
      K_SMALL_MAX_CAPACITY
    } else {
      capacity.max(K_SMALL_MIN_CAPACITY).next_power_of_two()
    };
    let num_buckets = cap / K_SMALL_LOAD_FACTOR;
    let size = size_for_small_table(cap, SmallOrderedHashMap::ENTRY_SIZE);
    let addr = allocator.allocate(size)?;

    let head = SmallOrderedHashTableHead {
      map,
      number_of_elements: 0,
      number_of_deleted_elements: 0,
      number_of_buckets: num_buckets as u8,
      padding: [0; 5],
    };

    // SAFETY: addr 由分配器成功分配并对齐，大小满足 size_for_small_table 计算出的容量所需字节
    unsafe {
      ptr::write(addr as *mut SmallOrderedHashTableHead, head);
      let table: Tagged<Self> = Tagged::from_raw_address(addr);
      for b in 0..num_buckets {
        table.set_bucket(b, K_SMALL_NOT_FOUND);
      }
      for entry in 0..cap {
        table.set_entry(entry, THE_HOLE, THE_HOLE);
        table.set_chain(entry, K_SMALL_NOT_FOUND);
      }
      Ok(table)
    }
  }
}

pub trait SmallOrderedHashMapExt: Copy {
  fn head(&self) -> &SmallOrderedHashTableHead;
  fn data_slot_addr(&self, entry: usize, offset: usize) -> usize;
  fn bucket_addr(&self, bucket: usize) -> usize;
  fn chain_addr(&self, entry: usize) -> usize;
  fn number_of_elements(&self) -> usize;
  fn set_number_of_elements(&self, n: usize);
  fn number_of_deleted_elements(&self) -> usize;
  fn set_number_of_deleted_elements(&self, n: usize);
  fn number_of_buckets(&self) -> usize;
  fn set_number_of_buckets(&self, n: usize);
  fn capacity(&self) -> usize;
  fn used_capacity(&self) -> usize;
  fn key_at(&self, entry: InternalIndex) -> Tagged<Object>;
  fn value_at(&self, entry: InternalIndex) -> Tagged<Object>;
  fn set_entry(&self, entry: usize, key: Tagged<Object>, value: Tagged<Object>);
  fn get_bucket(&self, bucket: usize) -> u8;
  fn set_bucket(&self, bucket: usize, entry: u8);
  fn get_chain(&self, entry: usize) -> u8;
  fn set_chain(&self, entry: usize, next_entry: u8);
  fn find_entry(&self, key: Tagged<Object>) -> InternalIndex;
  fn has_key(&self, key: Tagged<Object>) -> bool;
  fn get(&self, key: Tagged<Object>) -> Option<Tagged<Object>>;
  fn grow<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
  ) -> Result<Tagged<SmallOrderedHashMap>>;
  fn add<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
    key: Tagged<Object>,
    value: Tagged<Object>,
  ) -> Result<Tagged<SmallOrderedHashMap>>;
  fn delete(&self, key: Tagged<Object>) -> bool;
  fn shrink<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
  ) -> Result<Tagged<SmallOrderedHashMap>>;
  fn rehash<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
    new_capacity: usize,
  ) -> Result<Tagged<SmallOrderedHashMap>>;
  fn iter(&self) -> SmallOrderedHashMapIterator;
}

impl SmallOrderedHashMapExt for Tagged<SmallOrderedHashMap> {
  #[inline]
  fn number_of_elements(&self) -> usize {
    self.head().number_of_elements as usize
  }

  #[inline]
  fn set_number_of_elements(&self, n: usize) {
    let offset = mem::offset_of!(SmallOrderedHashTableHead, number_of_elements);
    // SAFETY: self.address() 指向有效且对齐的 SmallOrderedHashTableHead 结构体
    unsafe {
      *((self.address() + offset) as *mut u8) = n as u8;
    }
  }

  #[inline]
  fn number_of_deleted_elements(&self) -> usize {
    self.head().number_of_deleted_elements as usize
  }

  #[inline]
  fn set_number_of_deleted_elements(&self, n: usize) {
    let offset = mem::offset_of!(SmallOrderedHashTableHead, number_of_deleted_elements);
    // SAFETY: self.address() 指向有效且对齐的 SmallOrderedHashTableHead 结构体
    unsafe {
      *((self.address() + offset) as *mut u8) = n as u8;
    }
  }

  #[inline]
  fn number_of_buckets(&self) -> usize {
    self.head().number_of_buckets as usize
  }

  #[inline]
  fn set_number_of_buckets(&self, n: usize) {
    let offset = mem::offset_of!(SmallOrderedHashTableHead, number_of_buckets);
    // SAFETY: self.address() 指向有效且对齐的 SmallOrderedHashTableHead 结构体
    unsafe {
      *((self.address() + offset) as *mut u8) = n as u8;
    }
  }

  #[inline]
  fn capacity(&self) -> usize {
    self.number_of_buckets() * K_SMALL_LOAD_FACTOR
  }

  #[inline]
  fn used_capacity(&self) -> usize {
    self.number_of_elements() + self.number_of_deleted_elements()
  }

  #[inline]
  fn key_at(&self, entry: InternalIndex) -> Tagged<Object> {
    debug_assert!(entry.as_usize() < self.used_capacity());
    // SAFETY: data_slot_addr 计算自 DataTable 键槽位，按 usize 对齐且在有效对象内存中
    let raw = unsafe {
      AtomicUsize::from_ptr(
        self.data_slot_addr(entry.as_usize(), SmallOrderedHashMap::KEY_INDEX) as *mut usize,
      )
    }
    .load(Ordering::Relaxed);
    Tagged::new(raw)
  }

  #[inline]
  fn value_at(&self, entry: InternalIndex) -> Tagged<Object> {
    debug_assert!(entry.as_usize() < self.used_capacity());
    // SAFETY: data_slot_addr 计算自 DataTable 值槽位，按 usize 对齐且在有效对象内存中
    let raw = unsafe {
      AtomicUsize::from_ptr(
        self.data_slot_addr(entry.as_usize(), SmallOrderedHashMap::VALUE_INDEX) as *mut usize,
      )
    }
    .load(Ordering::Relaxed);
    Tagged::new(raw)
  }

  #[inline]
  fn set_entry(&self, entry: usize, key: Tagged<Object>, value: Tagged<Object>) {
    // SAFETY: data_slot_addr 位于有效且对齐的 DataTable 槽位，原子更新保证内存安全性
    unsafe {
      AtomicUsize::from_ptr(
        self.data_slot_addr(entry, SmallOrderedHashMap::KEY_INDEX) as *mut usize
      )
      .store(key.ptr(), Ordering::Relaxed);
      AtomicUsize::from_ptr(
        self.data_slot_addr(entry, SmallOrderedHashMap::VALUE_INDEX) as *mut usize
      )
      .store(value.ptr(), Ordering::Relaxed);
    }
  }

  #[inline]
  fn get_bucket(&self, bucket: usize) -> u8 {
    // SAFETY: bucket_addr 位于 HashTable 字节数组范围内
    unsafe { *(self.bucket_addr(bucket) as *const u8) }
  }

  #[inline]
  fn set_bucket(&self, bucket: usize, entry: u8) {
    // SAFETY: bucket_addr 位于 HashTable 字节数组范围内，具有可写内存权限
    unsafe { *(self.bucket_addr(bucket) as *mut u8) = entry };
  }

  #[inline]
  fn get_chain(&self, entry: usize) -> u8 {
    // SAFETY: chain_addr 位于 Chains 字节数组范围内
    unsafe { *(self.chain_addr(entry) as *const u8) }
  }

  #[inline]
  fn set_chain(&self, entry: usize, next_entry: u8) {
    // SAFETY: chain_addr 位于 Chains 字节数组范围内，具有可写内存权限
    unsafe { *(self.chain_addr(entry) as *mut u8) = next_entry };
  }

  fn find_entry(&self, key: Tagged<Object>) -> InternalIndex {
    if self.number_of_elements() == 0 || self.capacity() == 0 {
      return InternalIndex::NOT_FOUND;
    }
    let hash = compute_object_hash(key);
    let bucket = (hash as usize) & (self.number_of_buckets() - 1);
    let mut raw_entry = self.get_bucket(bucket);

    while raw_entry != K_SMALL_NOT_FOUND && (raw_entry as usize) < self.used_capacity() {
      let entry_idx = raw_entry as usize;
      let candidate = self.key_at(InternalIndex::new(entry_idx));
      if keys_equal(candidate, key) {
        return InternalIndex::new(entry_idx);
      }
      raw_entry = self.get_chain(entry_idx);
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

  fn grow<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
  ) -> Result<Tagged<SmallOrderedHashMap>> {
    let capacity = self.capacity();
    let nod = self.number_of_deleted_elements();
    let mut new_capacity = capacity;

    if nod < (capacity >> 1) {
      new_capacity = capacity << 1;
      if new_capacity == K_SMALL_GROWTH_HACK {
        new_capacity = K_SMALL_MAX_CAPACITY;
      }
      if new_capacity > K_SMALL_MAX_CAPACITY {
        return Err(Error::TableFull);
      }
    }
    self.rehash(allocator, map, new_capacity)
  }

  fn add<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
    key: Tagged<Object>,
    value: Tagged<Object>,
  ) -> Result<Tagged<SmallOrderedHashMap>> {
    let entry = self.find_entry(key);
    if entry.is_found() {
      self.set_entry(entry.as_usize(), key, value);
      return Ok(*self);
    }

    let mut current = *self;
    if current.used_capacity() >= current.capacity() {
      current = current.grow(allocator, map)?;
    }

    let nof = current.number_of_elements();
    let nod = current.number_of_deleted_elements();
    let new_entry = nof + nod;

    let hash = compute_object_hash(key);
    let bucket = (hash as usize) & (current.number_of_buckets() - 1);
    let prev_entry = current.get_bucket(bucket);

    current.set_entry(new_entry, key, value);
    current.set_chain(new_entry, prev_entry);
    current.set_bucket(bucket, new_entry as u8);
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
  ) -> Result<Tagged<SmallOrderedHashMap>> {
    let nof = self.number_of_elements();
    let cap = self.capacity();
    if nof >= (cap >> 2) || (cap >> 1) < K_SMALL_MIN_CAPACITY {
      return Ok(*self);
    }
    self.rehash(allocator, map, cap >> 1)
  }

  fn rehash<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
    new_capacity: usize,
  ) -> Result<Tagged<SmallOrderedHashMap>> {
    let mut entries = Vec::with_capacity(self.number_of_elements());
    for old_entry in 0..self.used_capacity() {
      let key = self.key_at(InternalIndex::new(old_entry));
      if !is_the_hole(key) {
        let value = self.value_at(InternalIndex::new(old_entry));
        entries.push((key, value));
      }
    }
    let old_nof = self.number_of_elements();

    let new_table = SmallOrderedHashMap::allocate(allocator, map, new_capacity)?;
    for (new_entry, (key, value)) in entries.into_iter().enumerate() {
      let hash = compute_object_hash(key);
      let bucket = (hash as usize) & (new_table.number_of_buckets() - 1);
      let chain = new_table.get_bucket(bucket);

      new_table.set_entry(new_entry, key, value);
      new_table.set_chain(new_entry, chain);
      new_table.set_bucket(bucket, new_entry as u8);
    }

    new_table.set_number_of_elements(old_nof);
    Ok(new_table)
  }

  #[inline]
  fn head(&self) -> &SmallOrderedHashTableHead {
    // SAFETY: self.address() 指向有效且已初始化的 SmallOrderedHashTableHead 结构体
    unsafe { &*(self.address() as *const SmallOrderedHashTableHead) }
  }

  #[inline]
  fn data_slot_addr(&self, entry: usize, offset: usize) -> usize {
    self.address()
      + data_table_start_offset()
      + (entry * SmallOrderedHashMap::ENTRY_SIZE + offset) * TAGGED_SLOT_SIZE
  }

  #[inline]
  fn bucket_addr(&self, bucket: usize) -> usize {
    self.address() + buckets_start_offset(self.capacity(), SmallOrderedHashMap::ENTRY_SIZE) + bucket
  }

  #[inline]
  fn chain_addr(&self, entry: usize) -> usize {
    self.address()
      + chain_table_offset(
        self.capacity(),
        SmallOrderedHashMap::ENTRY_SIZE,
        self.number_of_buckets(),
      )
      + entry
  }

  #[inline]
  fn iter(&self) -> SmallOrderedHashMapIterator {
    SmallOrderedHashMapIterator {
      table: *self,
      range: InternalIndexRange::new(self.used_capacity()),
    }
  }
}

pub struct SmallOrderedHashMapIterator {
  table: Tagged<SmallOrderedHashMap>,
  range: InternalIndexRange,
}

impl Iterator for SmallOrderedHashMapIterator {
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
/// 自持内存缓冲区的安全 SmallOrderedHashSet
pub struct SmallOrderedHashSetTable {
  storage: HeapStorage,
  map: Tagged<Map>,
  table: Tagged<SmallOrderedHashSet>,
}

impl SmallOrderedHashSetTable {
  pub fn new(map: Tagged<Map>) -> Result<Self> {
    let mut storage = HeapStorage::new();
    let table = SmallOrderedHashSet::allocate(&mut storage, map, K_SMALL_MIN_CAPACITY)?;
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
    let new_table =
      SmallOrderedHashSet::allocate(&mut self.storage, self.map, K_SMALL_MIN_CAPACITY)?;
    self.table = new_table;
    Ok(())
  }

  #[inline]
  pub fn iter(&self) -> SmallOrderedHashSetIterator {
    self.table.iter()
  }

  #[inline]
  pub fn tagged(&self) -> Tagged<SmallOrderedHashSet> {
    self.table
  }
}

/// 自持内存缓冲区的安全 SmallOrderedHashMap
pub struct SmallOrderedHashMapTable {
  storage: HeapStorage,
  map: Tagged<Map>,
  table: Tagged<SmallOrderedHashMap>,
}

impl SmallOrderedHashMapTable {
  pub fn new(map: Tagged<Map>) -> Result<Self> {
    let mut storage = HeapStorage::new();
    let table = SmallOrderedHashMap::allocate(&mut storage, map, K_SMALL_MIN_CAPACITY)?;
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
    let new_table =
      SmallOrderedHashMap::allocate(&mut self.storage, self.map, K_SMALL_MIN_CAPACITY)?;
    self.table = new_table;
    Ok(())
  }

  #[inline]
  pub fn iter(&self) -> SmallOrderedHashMapIterator {
    self.table.iter()
  }

  #[inline]
  pub fn tagged(&self) -> Tagged<SmallOrderedHashMap> {
    self.table
  }
}
