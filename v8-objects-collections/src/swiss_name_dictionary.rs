//! 瑞士表字典实现: SwissNameDictionary / SwissTable
//! 对应原版: v8/src/objects/swiss-name-dictionary.h + swiss-name-dictionary-inl.h + swiss-hash-table-helpers.h

use std::{
  mem, ptr,
  slice::{from_raw_parts, from_raw_parts_mut},
  sync::atomic::{AtomicUsize, Ordering},
};

use v8_objects_arrays::primitive_array::ByteArray;
use v8_objects_core::{
  map::Map,
  property_details::PropertyDetails,
  tagged::{Object, Tagged},
};

use crate::{
  allocator::{HeapStorage, MemoryAllocator},
  error::{Error, Result},
  internal_index::InternalIndex,
  ordered_hash_table::{THE_HOLE, compute_object_hash, keys_equal},
};

// ── 核心常量与控制字节 ───────────────────────────────────────────────────
pub const K_EMPTY: u8 = 0x80; // -128
pub const K_DELETED: u8 = 0xfe; // -2
pub const K_SENTINEL: u8 = 0xff; // -1

pub const K_H2_BITS: u32 = 7;
pub const K_H2_MASK: u32 = (1 << K_H2_BITS) - 1; // 0x7F
pub const K_NOT_FULL_MASK: u32 = 1 << K_H2_BITS; // 0x80

pub const K_GROUP_WIDTH: usize = 16;
pub const K_INITIAL_CAPACITY: usize = 4;
pub const TAGGED_SLOT_SIZE: usize = mem::size_of::<usize>();
pub const K_DATA_TABLE_ENTRY_COUNT: usize = 2;
pub const K_DATA_TABLE_KEY_ENTRY_INDEX: usize = 0;
pub const K_DATA_TABLE_VALUE_ENTRY_INDEX: usize = 1;

pub const K_META_TABLE_ELEMENT_COUNT_FIELD_INDEX: usize = 0;
pub const K_META_TABLE_DELETED_ELEMENT_COUNT_FIELD_INDEX: usize = 1;
pub const K_META_TABLE_ENUMERATION_DATA_START_INDEX: usize = 2;

pub const K_MAX_1_BYTE_META_TABLE_CAPACITY: usize = 256;
pub const K_MAX_2_BYTE_META_TABLE_CAPACITY: usize = 65536;

// ── 控制字节谓词 ─────────────────────────────────────────────────────────
#[inline]
pub const fn is_empty(c: u8) -> bool {
  c == K_EMPTY
}

#[inline]
pub const fn is_deleted(c: u8) -> bool {
  c == K_DELETED
}

#[inline]
pub const fn is_full(c: u8) -> bool {
  (c & 0x80) == 0
}

#[inline]
pub const fn is_empty_or_deleted(c: u8) -> bool {
  (c as i8) < -1
}

pub const K_SWISS_INITIAL_CAPACITY: usize = K_INITIAL_CAPACITY;

// ── H1/H2 哈希拆分 ───────────────────────────────────────────────────────
#[inline]
pub const fn h1(hash: u32) -> u32 {
  hash >> K_H2_BITS
}

#[inline]
pub const fn h2(hash: u32) -> u8 {
  (hash & K_H2_MASK) as u8
}

pub use h1 as h1_from_hash;
pub use h2 as h2_from_hash;

// ── 探测序列 ─────────────────────────────────────────────────────────────
pub struct ProbeSequence {
  mask: u32,
  offset: u32,
  index: u32,
}

impl ProbeSequence {
  #[inline]
  pub fn new(h1: u32, capacity: u32) -> Self {
    let non_zero_cap = if capacity == 0 { 1 } else { capacity };
    let mask = non_zero_cap - 1;
    let offset = h1 & mask;
    Self {
      mask,
      offset,
      index: 0,
    }
  }

  #[inline]
  pub fn offset(&self) -> u32 {
    self.offset
  }

  #[inline]
  pub fn offset_at(&self, i: usize) -> u32 {
    (self.offset + i as u32) & self.mask
  }

  #[inline]
  pub fn next(&mut self) {
    self.index += K_GROUP_WIDTH as u32;
    self.offset = (self.offset + self.index) & self.mask;
  }

  #[inline]
  pub fn index(&self) -> u32 {
    self.index
  }
}

// ── 组控制字节匹配 ───────────────────────────────────────────────────────
pub struct Group16<'a>(pub &'a [u8]);

impl<'a> Group16<'a> {
  /// # Safety
  /// ptr 必须指向至少 16 字节有效控制字节
  #[inline]
  pub unsafe fn load(ptr: *const u8) -> Self {
    // SAFETY: 由调用方保证 ptr 指向至少 16 字节有效且已初始化的控制字节
    unsafe { Self(from_raw_parts(ptr, K_GROUP_WIDTH)) }
  }

  #[inline]
  pub fn match_byte(&self, byte: u8) -> MatchMask {
    MatchMask(self.match_h2(byte))
  }

  #[inline]
  pub fn match_h2(&self, hash: u8) -> u16 {
    #[cfg(target_arch = "x86_64")]
    {
      use core::arch::x86_64::*;
      // SAFETY: self.0 包含至少 16 字节控制字节，_mm_loadu_si128 支持未对齐安全加载
      unsafe {
        let ctrl = _mm_loadu_si128(self.0.as_ptr() as *const __m128i);
        let match_vec = _mm_set1_epi8(hash as i8);
        let cmp = _mm_cmpeq_epi8(ctrl, match_vec);
        _mm_movemask_epi8(cmp) as u16
      }
    }

    #[cfg(target_arch = "aarch64")]
    {
      use core::arch::aarch64::*;
      // SAFETY: self.0 包含至少 16 字节控制字节，vld1q_u8 支持未对齐安全加载
      unsafe {
        let ctrl = vld1q_u8(self.0.as_ptr());
        let match_vec = vdupq_n_u8(hash);
        let cmp = vceqq_u8(ctrl, match_vec);
        const BIT_MASKS: [u8; 16] = [1, 2, 4, 8, 16, 32, 64, 128, 1, 2, 4, 8, 16, 32, 64, 128];
        let mask_vec = vld1q_u8(BIT_MASKS.as_ptr());
        let masked = vandq_u8(cmp, mask_vec);
        let low = vget_low_u8(masked);
        let high = vget_high_u8(masked);
        (vaddv_u8(low) as u16) | ((vaddv_u8(high) as u16) << 8)
      }
    }

    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
      let mut mask = 0u16;
      for (i, &b) in self.0.iter().take(K_GROUP_WIDTH).enumerate() {
        if b == hash {
          mask |= 1 << i;
        }
      }
      mask
    }
  }

  #[inline]
  pub fn match_empty(&self) -> MatchMask {
    self.match_byte(K_EMPTY)
  }

  #[inline]
  pub fn match_empty_or_deleted(&self) -> MatchMask {
    #[cfg(target_arch = "x86_64")]
    {
      use core::arch::x86_64::*;
      // SAFETY: self.0 包含至少 16 字节有效控制字节
      unsafe {
        let ctrl = _mm_loadu_si128(self.0.as_ptr() as *const __m128i);
        let sentinel = _mm_set1_epi8(K_SENTINEL as i8);
        // kEmpty (-128) 和 kDeleted (-2) 在有符号数下均小于 kSentinel (-1)
        let cmp = _mm_cmplt_epi8(ctrl, sentinel);
        MatchMask(_mm_movemask_epi8(cmp) as u16)
      }
    }

    #[cfg(target_arch = "aarch64")]
    {
      use core::arch::aarch64::*;
      // SAFETY: self.0 包含至少 16 字节有效控制字节
      unsafe {
        let ctrl = vld1q_s8(self.0.as_ptr() as *const i8);
        let sentinel = vdupq_n_s8(K_SENTINEL as i8);
        let cmp = vcltq_s8(ctrl, sentinel);
        const BIT_MASKS: [u8; 16] = [1, 2, 4, 8, 16, 32, 64, 128, 1, 2, 4, 8, 16, 32, 64, 128];
        let mask_vec = vld1q_u8(BIT_MASKS.as_ptr());
        let masked = vandq_u8(cmp, mask_vec);
        let low = vget_low_u8(masked);
        let high = vget_high_u8(masked);
        MatchMask((vaddv_u8(low) as u16) | ((vaddv_u8(high) as u16) << 8))
      }
    }

    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
      let mut mask = 0u16;
      for (i, &b) in self.0.iter().take(K_GROUP_WIDTH).enumerate() {
        if is_empty_or_deleted(b) {
          mask |= 1 << i;
        }
      }
      MatchMask(mask)
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MatchMask(pub u16);

impl MatchMask {
  #[inline]
  pub fn count(&self) -> usize {
    self.0.count_ones() as usize
  }

  #[inline]
  pub fn trailing_zeros(&self) -> u32 {
    self.0.trailing_zeros()
  }
}

impl PartialEq<u16> for MatchMask {
  #[inline]
  fn eq(&self, other: &u16) -> bool {
    self.0 == *other
  }
}

impl PartialEq<MatchMask> for u16 {
  #[inline]
  fn eq(&self, other: &MatchMask) -> bool {
    *self == other.0
  }
}

impl IntoIterator for MatchMask {
  type Item = usize;
  type IntoIter = MatchMaskIterator;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    MatchMaskIterator(self.0)
  }
}

pub struct MatchMaskIterator(pub u16);

impl Iterator for MatchMaskIterator {
  type Item = usize;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    if self.0 == 0 {
      None
    } else {
      let trailing = self.0.trailing_zeros() as usize;
      self.0 &= self.0 - 1;
      Some(trailing)
    }
  }
}

// ── 容量与内存布局计算 ───────────────────────────────────────────────────
#[inline]
pub const fn max_usable_capacity(capacity: usize) -> usize {
  capacity - capacity / 8
}

#[inline]
pub fn capacity_for(at_least_space_for: usize) -> usize {
  if at_least_space_for <= 4 {
    if at_least_space_for == 0 { 0 } else { 4 }
  } else {
    let non_normalized = at_least_space_for + at_least_space_for / 7;
    non_normalized.next_power_of_two()
  }
}

#[inline]
pub const fn meta_table_size_per_entry(capacity: usize) -> usize {
  if capacity <= K_MAX_1_BYTE_META_TABLE_CAPACITY {
    1
  } else if capacity <= K_MAX_2_BYTE_META_TABLE_CAPACITY {
    2
  } else {
    4
  }
}

#[inline]
pub const fn meta_table_data_size(capacity: usize) -> usize {
  meta_table_size_per_entry(capacity) * (max_usable_capacity(capacity) + 2)
}

#[inline]
pub const fn data_table_start_offset() -> usize {
  24 // Prefix (8: map, 4: hash, 4: cap, 8: meta_table)
}

#[inline]
pub const fn data_table_size(capacity: usize) -> usize {
  capacity * K_DATA_TABLE_ENTRY_COUNT * TAGGED_SLOT_SIZE
}

#[inline]
pub const fn ctrl_table_start_offset(capacity: usize) -> usize {
  data_table_start_offset() + data_table_size(capacity)
}

#[inline]
pub const fn ctrl_table_size(capacity: usize) -> usize {
  capacity + K_GROUP_WIDTH
}

#[inline]
pub const fn property_details_table_start_offset(capacity: usize) -> usize {
  ctrl_table_start_offset(capacity) + ctrl_table_size(capacity)
}

#[inline]
pub const fn total_dict_size(capacity: usize) -> usize {
  let raw = property_details_table_start_offset(capacity) + capacity;
  (raw + (TAGGED_SLOT_SIZE - 1)) & !(TAGGED_SLOT_SIZE - 1)
}

// ── SwissNameDictionary 头部结构体 ────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SwissNameDictionaryHead {
  pub map: Tagged<Map>,
  pub identity_hash: i32,
  pub capacity: u32,
  pub meta_table: Tagged<ByteArray>,
}

const _: () = assert!(mem::size_of::<SwissNameDictionaryHead>() == 24);

/// SwissNameDictionary
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SwissNameDictionary {
  pub head: SwissNameDictionaryHead,
}

impl SwissNameDictionary {
  #[inline]
  pub const fn size_for(capacity: usize) -> usize {
    total_dict_size(capacity)
  }

  /// 分配并初始化 SwissNameDictionary 与其 MetaTable
  pub fn allocate<A: MemoryAllocator>(
    allocator: &mut A,
    map: Tagged<Map>,
    byte_array_map: Tagged<Map>,
    capacity: usize,
  ) -> Result<Tagged<Self>> {
    let cap = capacity.max(K_INITIAL_CAPACITY).next_power_of_two();
    let meta_len = meta_table_data_size(cap);
    let meta_size = ByteArray::size_for(meta_len as u32);
    let meta_addr = allocator.allocate(meta_size)?;

    let meta_head = ByteArray::new(byte_array_map, meta_len as u32);
    // SAFETY: meta_addr 由分配器成功分配 meta_size 字节并已对齐
    unsafe {
      ptr::write(meta_addr as *mut ByteArray, meta_head);
      let meta_slice =
        from_raw_parts_mut((meta_addr + ByteArray::DATA_OFFSET) as *mut u8, meta_len);
      meta_slice.fill(0);
    }
    // SAFETY: meta_addr 满足对齐和堆对象指针约束
    let meta_table: Tagged<ByteArray> = unsafe { Tagged::from_raw_address(meta_addr) };

    let dict_size = total_dict_size(cap);
    let dict_addr = allocator.allocate(dict_size)?;

    let head = SwissNameDictionaryHead {
      map,
      identity_hash: 0,
      capacity: cap as u32,
      meta_table,
    };

    // SAFETY: dict_addr 由分配器成功分配 dict_size 字节并对齐，写入头部并初始化各表数据
    unsafe {
      ptr::write(dict_addr as *mut SwissNameDictionaryHead, head);
      let dict: Tagged<Self> = Tagged::from_raw_address(dict_addr);

      // 初始化 CtrlTable 为 K_EMPTY
      let ctrl_slice = from_raw_parts_mut(dict.ctrl_table_ptr(), ctrl_table_size(cap));
      ctrl_slice.fill(K_EMPTY);

      // 初始化 DataTable
      for i in 0..cap {
        dict.set_key_at(i, THE_HOLE);
        dict.set_value_at(i, THE_HOLE);
        dict.set_details_at(i, PropertyDetails::default());
      }

      dict.set_number_of_elements(0);
      dict.set_number_of_deleted_elements(0);

      Ok(dict)
    }
  }
}

pub trait SwissNameDictionaryExt: Copy {
  fn head(&self) -> &SwissNameDictionaryHead;
  fn meta_data_ptr(&self) -> *mut u8;
  fn ctrl_table_ptr(&self) -> *mut u8;
  fn data_slot_addr(&self, entry: usize, slot: usize) -> usize;
  fn details_table_ptr(&self) -> *mut u8;
  fn capacity(&self) -> usize;
  fn identity_hash(&self) -> i32;
  fn set_identity_hash(&self, h: i32);
  fn meta_table(&self) -> Tagged<ByteArray>;
  fn get_meta_field(&self, field_index: usize) -> u32;
  fn set_meta_field(&self, field_index: usize, val: u32);
  fn number_of_elements(&self) -> usize;
  fn set_number_of_elements(&self, n: usize);
  fn number_of_deleted_elements(&self) -> usize;
  fn set_number_of_deleted_elements(&self, n: usize);
  fn used_capacity(&self) -> usize;
  fn entry_for_enumeration_index(&self, enum_idx: usize) -> usize;
  fn set_entry_for_enumeration_index(&self, enum_idx: usize, entry: usize);
  fn get_ctrl(&self, entry: usize) -> u8;
  fn set_ctrl(&self, entry: usize, h: u8);
  fn key_at(&self, entry: InternalIndex) -> Tagged<Object>;
  fn set_key_at(&self, entry: usize, key: Tagged<Object>);
  fn value_at(&self, entry: InternalIndex) -> Tagged<Object>;
  fn set_value_at(&self, entry: usize, val: Tagged<Object>);
  fn details_at(&self, entry: InternalIndex) -> PropertyDetails;
  fn set_details_at(&self, entry: usize, details: PropertyDetails);
  fn find_entry(&self, key: Tagged<Object>) -> InternalIndex;
  fn has_key(&self, key: Tagged<Object>) -> bool;
  fn find_first_empty(&self, hash: u32) -> usize;
  fn add<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
    byte_array_map: Tagged<Map>,
    key: Tagged<Object>,
    value: Tagged<Object>,
    details: PropertyDetails,
  ) -> Result<Tagged<SwissNameDictionary>>;
  fn delete_entry(&self, entry: InternalIndex) -> bool;
  fn shrink<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
    byte_array_map: Tagged<Map>,
  ) -> Result<Tagged<SwissNameDictionary>>;
  fn rehash<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
    byte_array_map: Tagged<Map>,
    new_capacity: usize,
  ) -> Result<Tagged<SwissNameDictionary>>;
  fn rehash_in_place(&self);
  fn iter_entries_ordered(&self) -> SwissNameDictionaryIterator;
}

impl SwissNameDictionaryExt for Tagged<SwissNameDictionary> {
  #[inline]
  fn capacity(&self) -> usize {
    self.head().capacity as usize
  }

  #[inline]
  fn identity_hash(&self) -> i32 {
    self.head().identity_hash
  }

  #[inline]
  fn set_identity_hash(&self, h: i32) {
    let offset = mem::offset_of!(SwissNameDictionaryHead, identity_hash);
    // SAFETY: self.address() 指向有效且对齐的 SwissNameDictionaryHead 结构体
    unsafe {
      *((self.address() + offset) as *mut i32) = h;
    }
  }

  #[inline]
  fn meta_table(&self) -> Tagged<ByteArray> {
    self.head().meta_table
  }

  #[inline]
  fn get_meta_field(&self, field_index: usize) -> u32 {
    let cap = self.capacity();
    let ptr = self.meta_data_ptr();
    // SAFETY: field_index 严格处于分配的 MetaTable 数组界限内，类型换算依据容量自适应 8/16/32 位
    if cap <= K_MAX_1_BYTE_META_TABLE_CAPACITY {
      unsafe { *ptr.add(field_index) as u32 }
    } else if cap <= K_MAX_2_BYTE_META_TABLE_CAPACITY {
      unsafe { *(ptr as *const u16).add(field_index) as u32 }
    } else {
      unsafe { *(ptr as *const u32).add(field_index) }
    }
  }

  #[inline]
  fn set_meta_field(&self, field_index: usize, val: u32) {
    let cap = self.capacity();
    let ptr = self.meta_data_ptr();
    // SAFETY: field_index 严格处于分配的 MetaTable 数组界限内，具有可写内存权限
    if cap <= K_MAX_1_BYTE_META_TABLE_CAPACITY {
      unsafe { *ptr.add(field_index) = val as u8 };
    } else if cap <= K_MAX_2_BYTE_META_TABLE_CAPACITY {
      unsafe { *(ptr as *mut u16).add(field_index) = val as u16 };
    } else {
      unsafe { *(ptr as *mut u32).add(field_index) = val };
    }
  }

  #[inline]
  fn number_of_elements(&self) -> usize {
    self.get_meta_field(K_META_TABLE_ELEMENT_COUNT_FIELD_INDEX) as usize
  }

  #[inline]
  fn set_number_of_elements(&self, n: usize) {
    self.set_meta_field(K_META_TABLE_ELEMENT_COUNT_FIELD_INDEX, n as u32);
  }

  #[inline]
  fn number_of_deleted_elements(&self) -> usize {
    self.get_meta_field(K_META_TABLE_DELETED_ELEMENT_COUNT_FIELD_INDEX) as usize
  }

  #[inline]
  fn set_number_of_deleted_elements(&self, n: usize) {
    self.set_meta_field(K_META_TABLE_DELETED_ELEMENT_COUNT_FIELD_INDEX, n as u32);
  }

  #[inline]
  fn used_capacity(&self) -> usize {
    self.number_of_elements() + self.number_of_deleted_elements()
  }

  #[inline]
  fn entry_for_enumeration_index(&self, enum_idx: usize) -> usize {
    debug_assert!(enum_idx < self.used_capacity());
    self.get_meta_field(K_META_TABLE_ENUMERATION_DATA_START_INDEX + enum_idx) as usize
  }

  #[inline]
  fn set_entry_for_enumeration_index(&self, enum_idx: usize, entry: usize) {
    self.set_meta_field(
      K_META_TABLE_ENUMERATION_DATA_START_INDEX + enum_idx,
      entry as u32,
    );
  }

  #[inline]
  fn get_ctrl(&self, entry: usize) -> u8 {
    debug_assert!(entry < self.capacity() + K_GROUP_WIDTH);
    // SAFETY: entry 处于控制表分配区域内 (包含末尾 K_GROUP_WIDTH 镜像缓冲区)
    unsafe { *self.ctrl_table_ptr().add(entry) }
  }

  #[inline]
  fn set_ctrl(&self, entry: usize, h: u8) {
    let capacity = self.capacity();
    debug_assert!(entry < capacity);
    let ctrl = self.ctrl_table_ptr();
    // SAFETY: entry < capacity，且环形首尾镜像拷贝下标 < capacity + K_GROUP_WIDTH，完全在有效控制表边界内
    unsafe {
      *ctrl.add(entry) = h;
      let copy_entry = if entry < K_GROUP_WIDTH {
        capacity + entry
      } else {
        entry
      };
      *ctrl.add(copy_entry) = h;
    }
  }

  #[inline]
  fn key_at(&self, entry: InternalIndex) -> Tagged<Object> {
    debug_assert!(entry.as_usize() < self.capacity());
    // SAFETY: entry 在容量界限内，data_slot_addr 计算出的地址按字对齐且处于 DataTable 范围内
    let raw = unsafe {
      AtomicUsize::from_ptr(
        self.data_slot_addr(entry.as_usize(), K_DATA_TABLE_KEY_ENTRY_INDEX) as *mut usize,
      )
    }
    .load(Ordering::Relaxed);
    Tagged::new(raw)
  }

  #[inline]
  fn set_key_at(&self, entry: usize, key: Tagged<Object>) {
    // SAFETY: entry 在容量界限内，data_slot_addr 处于有效数据槽位并按字自然对齐
    unsafe {
      AtomicUsize::from_ptr(self.data_slot_addr(entry, K_DATA_TABLE_KEY_ENTRY_INDEX) as *mut usize)
        .store(key.ptr(), Ordering::Relaxed);
    }
  }

  #[inline]
  fn value_at(&self, entry: InternalIndex) -> Tagged<Object> {
    debug_assert!(entry.as_usize() < self.capacity());
    // SAFETY: entry 在容量界限内，data_slot_addr 计算出的地址按字对齐且处于 DataTable 范围内
    let raw = unsafe {
      AtomicUsize::from_ptr(
        self.data_slot_addr(entry.as_usize(), K_DATA_TABLE_VALUE_ENTRY_INDEX) as *mut usize,
      )
    }
    .load(Ordering::Relaxed);
    Tagged::new(raw)
  }

  #[inline]
  fn set_value_at(&self, entry: usize, val: Tagged<Object>) {
    // SAFETY: entry 在容量界限内，data_slot_addr 处于有效数据槽位并按字自然对齐
    unsafe {
      AtomicUsize::from_ptr(
        self.data_slot_addr(entry, K_DATA_TABLE_VALUE_ENTRY_INDEX) as *mut usize
      )
      .store(val.ptr(), Ordering::Relaxed);
    }
  }

  #[inline]
  fn details_at(&self, entry: InternalIndex) -> PropertyDetails {
    debug_assert!(entry.as_usize() < self.capacity());
    // SAFETY: entry 在容量界限内，details_table_ptr() 区域容纳 capacity 字节
    let byte_val = unsafe { *self.details_table_ptr().add(entry.as_usize()) };
    PropertyDetails::from_byte(byte_val)
  }

  #[inline]
  fn set_details_at(&self, entry: usize, details: PropertyDetails) {
    // SAFETY: entry 在容量界限内，details_table_ptr 区域具有可写内存权限
    unsafe {
      *self.details_table_ptr().add(entry) = details.to_byte();
    }
  }

  fn find_entry(&self, key: Tagged<Object>) -> InternalIndex {
    let cap = self.capacity();
    if cap == 0 || self.number_of_elements() == 0 {
      return InternalIndex::NOT_FOUND;
    }
    let hash = compute_object_hash(key);
    let target_h2 = h2(hash);
    let mut seq = ProbeSequence::new(h1(hash), cap as u32);
    let ctrl_ptr = self.ctrl_table_ptr();

    loop {
      let offset = seq.offset() as usize;
      // SAFETY: 控制表分配了 capacity + K_GROUP_WIDTH 字节，offset < capacity，故 offset..offset + K_GROUP_WIDTH 处于有效内存中
      let group = unsafe { Group16::load(ctrl_ptr.add(offset)) };

      for i in group.match_byte(target_h2) {
        let candidate_entry = seq.offset_at(i) as usize;
        let candidate_key = self.key_at(InternalIndex::new(candidate_entry));
        if keys_equal(candidate_key, key) {
          return InternalIndex::new(candidate_entry);
        }
      }

      if group.match_empty() != 0 {
        return InternalIndex::NOT_FOUND;
      }

      seq.next();
      if seq.index() >= cap as u32 {
        return InternalIndex::NOT_FOUND;
      }
    }
  }

  #[inline]
  fn has_key(&self, key: Tagged<Object>) -> bool {
    self.find_entry(key).is_found()
  }

  fn find_first_empty(&self, hash: u32) -> usize {
    let cap = self.capacity();
    let mut seq = ProbeSequence::new(h1(hash), cap as u32);
    let ctrl_ptr = self.ctrl_table_ptr();

    loop {
      let offset = seq.offset() as usize;
      // SAFETY: 控制表具有 capacity + K_GROUP_WIDTH 字节，offset < capacity，故从 offset 连续读取 16 字节完全在界内
      let group = unsafe { Group16::load(ctrl_ptr.add(offset)) };
      let empty_mask = group.match_empty();
      if empty_mask != 0 {
        let i = empty_mask.trailing_zeros() as usize;
        return seq.offset_at(i) as usize;
      }
      seq.next();
      if seq.index() >= cap as u32 {
        break;
      }
    }
    usize::MAX
  }

  fn add<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
    byte_array_map: Tagged<Map>,
    key: Tagged<Object>,
    value: Tagged<Object>,
    details: PropertyDetails,
  ) -> Result<Tagged<SwissNameDictionary>> {
    let entry = self.find_entry(key);
    if entry.is_found() {
      self.set_value_at(entry.as_usize(), value);
      self.set_details_at(entry.as_usize(), details);
      return Ok(*self);
    }

    let mut current = *self;
    if current.used_capacity() >= max_usable_capacity(current.capacity()) {
      let new_capacity = if current.capacity() == 0 {
        K_INITIAL_CAPACITY
      } else {
        current.capacity() << 1
      };
      current = current.rehash(allocator, map, byte_array_map, new_capacity)?;
    }

    let hash = compute_object_hash(key);
    let target = current.find_first_empty(hash);
    if target == usize::MAX {
      return Err(Error::TableFull);
    }

    current.set_ctrl(target, h2(hash));
    current.set_key_at(target, key);
    current.set_value_at(target, value);
    current.set_details_at(target, details);

    let nof = current.number_of_elements();
    let nod = current.number_of_deleted_elements();
    let new_enum_idx = nof + nod;

    current.set_entry_for_enumeration_index(new_enum_idx, target);
    current.set_number_of_elements(nof + 1);

    Ok(current)
  }

  fn delete_entry(&self, entry: InternalIndex) -> bool {
    if entry.is_not_found() || entry.as_usize() >= self.capacity() {
      return false;
    }
    let raw = entry.as_usize();
    if !is_full(self.get_ctrl(raw)) {
      return false;
    }

    self.set_ctrl(raw, K_DELETED);
    self.set_key_at(raw, THE_HOLE);
    self.set_value_at(raw, THE_HOLE);

    self.set_number_of_elements(self.number_of_elements() - 1);
    self.set_number_of_deleted_elements(self.number_of_deleted_elements() + 1);
    true
  }

  fn shrink<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
    byte_array_map: Tagged<Map>,
  ) -> Result<Tagged<SwissNameDictionary>> {
    let nof = self.number_of_elements();
    let cap = self.capacity();
    if nof >= (cap >> 2) || (cap >> 1) < K_INITIAL_CAPACITY {
      return Ok(*self);
    }
    self.rehash(allocator, map, byte_array_map, cap >> 1)
  }

  fn rehash<A: MemoryAllocator>(
    &self,
    allocator: &mut A,
    map: Tagged<Map>,
    byte_array_map: Tagged<Map>,
    new_capacity: usize,
  ) -> Result<Tagged<SwissNameDictionary>> {
    // 提前快照旧表数据，防备分配器扩容搬迁导致指针悬垂
    let mut entries = Vec::with_capacity(self.number_of_elements());
    for idx in self.iter_entries_ordered() {
      let key = self.key_at(idx);
      let value = self.value_at(idx);
      let details = self.details_at(idx);
      let hash = compute_object_hash(key);
      entries.push((key, value, details, hash));
    }
    let old_nof = self.number_of_elements();
    let old_id_hash = self.identity_hash();

    let new_dict = SwissNameDictionary::allocate(allocator, map, byte_array_map, new_capacity)?;
    new_dict.set_identity_hash(old_id_hash);

    for (new_enum_idx, (key, value, details, hash)) in entries.into_iter().enumerate() {
      let target = new_dict.find_first_empty(hash);
      debug_assert!(target != usize::MAX);

      new_dict.set_ctrl(target, h2(hash));
      new_dict.set_key_at(target, key);
      new_dict.set_value_at(target, value);
      new_dict.set_details_at(target, details);

      new_dict.set_entry_for_enumeration_index(new_enum_idx, target);
    }

    new_dict.set_number_of_elements(old_nof);
    Ok(new_dict)
  }

  fn rehash_in_place(&self) {
    let cap = self.capacity();
    if cap == 0 {
      return;
    }
    // 收集所有有效条目（按枚举序）
    let mut entries = Vec::with_capacity(self.number_of_elements());
    for idx in self.iter_entries_ordered() {
      let key = self.key_at(idx);
      let value = self.value_at(idx);
      let details = self.details_at(idx);
      let hash = compute_object_hash(key);
      entries.push((key, value, details, hash));
    }

    // 重置 CtrlTable 为 K_EMPTY (含尾部镜像区)
    let ctrl_size = ctrl_table_size(cap);
    // SAFETY: self.ctrl_table_ptr() 指向容量为 ctrl_size 的控制字节区
    unsafe {
      let ctrl_slice = from_raw_parts_mut(self.ctrl_table_ptr(), ctrl_size);
      ctrl_slice.fill(K_EMPTY);
    }

    // 重置 DataTable 为 THE_HOLE
    for i in 0..cap {
      self.set_key_at(i, THE_HOLE);
      self.set_value_at(i, THE_HOLE);
      self.set_details_at(i, PropertyDetails::default());
    }

    self.set_number_of_elements(entries.len());
    self.set_number_of_deleted_elements(0);

    for (new_enum_idx, (key, value, details, hash)) in entries.into_iter().enumerate() {
      let target = self.find_first_empty(hash);
      debug_assert!(target != usize::MAX);

      self.set_ctrl(target, h2(hash));
      self.set_key_at(target, key);
      self.set_value_at(target, value);
      self.set_details_at(target, details);

      self.set_entry_for_enumeration_index(new_enum_idx, target);
    }
  }

  #[inline]
  fn head(&self) -> &SwissNameDictionaryHead {
    // SAFETY: self.address() 指向有效且已初始化的 SwissNameDictionaryHead 结构体
    unsafe { &*(self.address() as *const SwissNameDictionaryHead) }
  }

  #[inline]
  fn meta_data_ptr(&self) -> *mut u8 {
    let meta = self.meta_table();
    (meta.address() + ByteArray::DATA_OFFSET) as *mut u8
  }

  #[inline]
  fn ctrl_table_ptr(&self) -> *mut u8 {
    (self.address() + ctrl_table_start_offset(self.capacity())) as *mut u8
  }

  #[inline]
  fn data_slot_addr(&self, entry: usize, slot: usize) -> usize {
    self.address()
      + data_table_start_offset()
      + (entry * K_DATA_TABLE_ENTRY_COUNT + slot) * TAGGED_SLOT_SIZE
  }

  #[inline]
  fn details_table_ptr(&self) -> *mut u8 {
    (self.address() + property_details_table_start_offset(self.capacity())) as *mut u8
  }

  #[inline]
  fn iter_entries_ordered(&self) -> SwissNameDictionaryIterator {
    SwissNameDictionaryIterator {
      dict: *self,
      enum_index: 0,
      used_capacity: self.used_capacity(),
    }
  }
}

pub struct SwissNameDictionaryIterator {
  dict: Tagged<SwissNameDictionary>,
  enum_index: usize,
  used_capacity: usize,
}

impl Iterator for SwissNameDictionaryIterator {
  type Item = InternalIndex;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    while self.enum_index < self.used_capacity {
      let bucket = self.dict.entry_for_enumeration_index(self.enum_index);
      self.enum_index += 1;
      if is_full(self.dict.get_ctrl(bucket)) {
        return Some(InternalIndex::new(bucket));
      }
    }
    None
  }
}

// ── 安全封装类型 ──────────────────────────────────────────────────────────
/// 自持内存缓冲区的安全 SwissNameDictionary
pub struct SwissNameDictionaryTable {
  storage: HeapStorage,
  map: Tagged<Map>,
  byte_array_map: Tagged<Map>,
  dict: Tagged<SwissNameDictionary>,
}

impl SwissNameDictionaryTable {
  pub fn new(map: Tagged<Map>, byte_array_map: Tagged<Map>) -> Result<Self> {
    let mut storage = HeapStorage::new();
    let dict =
      SwissNameDictionary::allocate(&mut storage, map, byte_array_map, K_INITIAL_CAPACITY)?;
    Ok(Self {
      storage,
      map,
      byte_array_map,
      dict,
    })
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.dict.number_of_elements()
  }

  #[inline]
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  #[inline]
  pub fn capacity(&self) -> usize {
    self.dict.capacity()
  }

  #[inline]
  pub fn contains_key(&self, key: Tagged<Object>) -> bool {
    self.dict.has_key(key)
  }

  #[inline]
  pub fn get(&self, key: Tagged<Object>) -> Option<Tagged<Object>> {
    let entry = self.dict.find_entry(key);
    if entry.is_found() {
      Some(self.dict.value_at(entry))
    } else {
      None
    }
  }

  #[inline]
  pub fn get_details(&self, key: Tagged<Object>) -> Option<PropertyDetails> {
    let entry = self.dict.find_entry(key);
    if entry.is_found() {
      Some(self.dict.details_at(entry))
    } else {
      None
    }
  }

  pub fn insert(
    &mut self,
    key: Tagged<Object>,
    value: Tagged<Object>,
    details: PropertyDetails,
  ) -> Result<Option<Tagged<Object>>> {
    let entry = self.dict.find_entry(key);
    if entry.is_found() {
      let prev = self.dict.value_at(entry);
      self.dict.set_value_at(entry.as_usize(), value);
      self.dict.set_details_at(entry.as_usize(), details);
      return Ok(Some(prev));
    }
    let new_dict = self.dict.add(
      &mut self.storage,
      self.map,
      self.byte_array_map,
      key,
      value,
      details,
    )?;
    self.dict = new_dict;
    Ok(None)
  }

  pub fn remove(&mut self, key: Tagged<Object>) -> bool {
    let entry = self.dict.find_entry(key);
    if entry.is_not_found() {
      return false;
    }
    let removed = self.dict.delete_entry(entry);
    if removed
      && let Ok(new_dict) = self
        .dict
        .shrink(&mut self.storage, self.map, self.byte_array_map)
    {
      self.dict = new_dict;
    }
    removed
  }

  #[inline]
  pub fn iter(
    &self,
  ) -> impl Iterator<Item = (Tagged<Object>, Tagged<Object>, PropertyDetails)> + '_ {
    self.dict.iter_entries_ordered().map(move |idx| {
      (
        self.dict.key_at(idx),
        self.dict.value_at(idx),
        self.dict.details_at(idx),
      )
    })
  }

  #[inline]
  pub fn rehash_in_place(&mut self) {
    self.dict.rehash_in_place();
  }

  #[inline]
  pub fn tagged(&self) -> Tagged<SwissNameDictionary> {
    self.dict
  }
}
