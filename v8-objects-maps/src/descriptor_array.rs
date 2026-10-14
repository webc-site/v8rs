// v8-objects-maps/src/descriptor_array.rs
// 对应原版 v8/src/objects/descriptor-array.h + descriptor-array-inl.h

//! DescriptorArray: 存储对象所有自有属性描述符元数据的数组。
//!
//! 每个描述符条目占用 3 个相邻槽位:
//! - 槽位 0: Key (属性名，内部化字符串或符号)
//! - 槽位 1: Details (PropertyDetails 编码，作为 Smi 存储)
//! - 槽位 2: Value (字段类型、常量值或 `Weak<Map>`)

use std::{
  mem,
  ops::Deref,
  ptr,
  sync::atomic::{AtomicUsize, Ordering},
};

use v8_common::K_TAGGED_SIZE_LOG2;
use v8_objects_arrays::fixed_array::FixedArray;
use v8_objects_core::{
  heap_object::HeapObject,
  map::Map,
  name::Name,
  property_details::PropertyDetails,
  tagged::{MaybeObject, Object, Tagged},
  tagged_field::TaggedSlot,
};

use crate::{
  error::{Error, Result},
  name_ext::NameTaggedExt,
};

pub const K_ENTRY_KEY_INDEX: usize = 0;
pub const K_ENTRY_DETAILS_INDEX: usize = 1;
pub const K_ENTRY_VALUE_INDEX: usize = 2;
pub const K_ENTRY_SIZE: usize = 3;

pub const K_MAX_ELEMENTS_FOR_LINEAR_SEARCH: usize = 32;
pub const K_NOT_FOUND: i32 = -1;

/// JSON 快速迭代状态
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub enum FastIterableState {
  #[default]
  JsonFast = 0b00,
  JsonSlow = 0b01,
  Unknown = 0b11,
}

impl FastIterableState {
  #[inline]
  #[must_use]
  pub const fn from_u32(val: u32) -> Self {
    match val & 0b11 {
      0b00 => Self::JsonFast,
      0b01 => Self::JsonSlow,
      _ => Self::Unknown,
    }
  }
}

/// 枚举缓存结构体
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct EnumCache {
  pub header: HeapObject,
  pub keys: Tagged<FixedArray>,
  pub indices: Tagged<FixedArray>,
}

impl EnumCache {
  #[inline]
  pub const fn new(
    map: Tagged<Map>,
    keys: Tagged<FixedArray>,
    indices: Tagged<FixedArray>,
  ) -> Self {
    Self {
      header: HeapObject::new(map),
      keys,
      indices,
    }
  }
}

/// 描述符条目逻辑值类型
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DescriptorEntry {
  pub key: Tagged<Name>,
  pub details: PropertyDetails,
  pub value: Tagged<MaybeObject>,
}

impl DescriptorEntry {
  #[inline]
  pub const fn new(
    key: Tagged<Name>,
    details: PropertyDetails,
    value: Tagged<MaybeObject>,
  ) -> Self {
    Self {
      key,
      details,
      value,
    }
  }
}

/// 描述符查找缓存 (64 项固定大小哈希表)
pub struct DescriptorLookupCache {
  keys: [(Tagged<Map>, Tagged<Name>); Self::K_LENGTH],
  results: [i32; Self::K_LENGTH],
}

impl Default for DescriptorLookupCache {
  fn default() -> Self {
    Self::new()
  }
}

impl DescriptorLookupCache {
  pub const K_LENGTH: usize = 64;
  pub const K_ABSENT: i32 = -2;

  #[inline]
  pub fn new() -> Self {
    Self {
      keys: [(Tagged::NULL, Tagged::NULL); Self::K_LENGTH],
      results: [Self::K_ABSENT; Self::K_LENGTH],
    }
  }

  #[inline]
  pub fn clear(&mut self) {
    self.keys.fill((Tagged::NULL, Tagged::NULL));
    self.results.fill(Self::K_ABSENT);
  }

  #[inline]
  fn hash(source: Tagged<Map>, name: Tagged<Name>) -> usize {
    let source_hash = (source.ptr() as u32) >> K_TAGGED_SIZE_LOG2;
    let name_hash = name.hash();
    ((source_hash ^ name_hash) as usize) % Self::K_LENGTH
  }

  #[inline]
  pub fn lookup(&self, source: Tagged<Map>, name: Tagged<Name>) -> i32 {
    let index = Self::hash(source, name);
    let (k_source, k_name) = self.keys[index];
    if k_source == source && k_name == name {
      self.results[index]
    } else {
      Self::K_ABSENT
    }
  }

  #[inline]
  pub fn update(&mut self, source: Tagged<Map>, name: Tagged<Name>, result: i32) {
    let index = Self::hash(source, name);
    self.keys[index] = (source, name);
    self.results[index] = result;
  }
}

/// 描述符数组头
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DescriptorArray {
  pub header: HeapObject,
  pub number_of_all_descriptors: u16,
  pub number_of_descriptors: u16,
  pub flags: u32,
  pub enum_cache: Tagged<EnumCache>,
}

impl DescriptorArray {
  pub const K_HEADER_SIZE: usize = mem::size_of::<Self>();
  pub const K_SLOT_SIZE: usize = mem::size_of::<TaggedSlot>();

  /// 计算指定容量所需字节数
  #[inline]
  #[must_use]
  pub const fn size_for(capacity: usize) -> usize {
    Self::K_HEADER_SIZE + capacity * K_ENTRY_SIZE * Self::K_SLOT_SIZE
  }

  /// 计算描述符下标对应的字节偏移量
  #[inline]
  #[must_use]
  pub const fn offset_of_descriptor_at(descriptor: usize) -> usize {
    Self::size_for(descriptor)
  }

  #[inline]
  pub const fn new(map: Tagged<Map>, capacity: u16) -> Self {
    Self {
      header: HeapObject::new(map),
      number_of_all_descriptors: capacity,
      number_of_descriptors: 0,
      flags: FastIterableState::Unknown as u32,
      enum_cache: Tagged::NULL,
    }
  }

  /// 在 Zone 中分配 DescriptorArray
  pub fn allocate_in_zone(
    zone: &v8_zone::Zone,
    map: Tagged<Map>,
    capacity: u16,
  ) -> Result<Tagged<Self>> {
    let size = Self::size_for(capacity as usize);
    let slice = zone.allocate_bytes_ref(size)?;
    slice.fill(mem::MaybeUninit::new(0));
    let ptr = slice.as_mut_ptr();
    let header = Self::new(map, capacity);
    // SAFETY: zone 分配的内存非空、已对齐且尺寸与容量满足要求
    unsafe {
      ptr::write(ptr as *mut Self, header);
      Ok(Tagged::from_raw_address(ptr as usize))
    }
  }
}

/// 对 `Tagged<DescriptorArray>` 的扩展方法
pub trait DescriptorArrayExt {
  fn number_of_descriptors(&self) -> u16;
  fn set_number_of_descriptors(&self, value: u16);
  fn number_of_all_descriptors(&self) -> u16;
  fn set_number_of_all_descriptors(&self, value: u16);
  fn number_of_slack_descriptors(&self) -> u16;

  fn flags(&self) -> u32;
  fn set_flags(&self, flags: u32);
  fn fast_iterable(&self) -> FastIterableState;
  fn set_fast_iterable(&self, state: FastIterableState);

  fn enum_cache(&self) -> Tagged<EnumCache>;
  fn set_enum_cache(&self, cache: Tagged<EnumCache>);

  fn get_key(&self, descriptor_number: usize) -> Result<Tagged<Name>>;
  fn get_details(&self, descriptor_number: usize) -> Result<PropertyDetails>;
  fn get_value(&self, descriptor_number: usize) -> Result<Tagged<MaybeObject>>;
  fn get_strong_value(&self, descriptor_number: usize) -> Result<Tagged<Object>>;

  fn set_key(&self, descriptor_number: usize, key: Tagged<Name>) -> Result<()>;
  fn set_details(&self, descriptor_number: usize, details: PropertyDetails) -> Result<()>;
  fn set_value(&self, descriptor_number: usize, value: Tagged<MaybeObject>) -> Result<()>;
  fn set(
    &self,
    descriptor_number: usize,
    key: Tagged<Name>,
    value: Tagged<MaybeObject>,
    details: PropertyDetails,
  ) -> Result<()>;

  fn append(
    &self,
    key: Tagged<Name>,
    value: Tagged<MaybeObject>,
    details: PropertyDetails,
  ) -> Result<()>;

  fn get_sorted_key_index(&self, descriptor_number: usize) -> usize;
  fn get_sorted_key(&self, descriptor_number: usize) -> Tagged<Name>;
  fn set_sorted_key(&self, descriptor_number: usize, pointer: usize);
  fn swap_sorted_keys(&self, first: usize, second: usize);
  fn sort(&self);

  fn search(&self, name: Tagged<Name>, valid_descriptors: usize) -> Option<usize>;
  fn linear_search(&self, name: Tagged<Name>, valid_descriptors: usize) -> Option<usize>;
  fn binary_search(&self, name: Tagged<Name>, valid_descriptors: usize) -> Option<usize>;
  fn search_with_cache(
    &self,
    cache: &mut DescriptorLookupCache,
    name: Tagged<Name>,
    map: Tagged<Map>,
  ) -> Option<usize>;
}

impl DescriptorArrayExt for Tagged<DescriptorArray> {
  #[inline]
  fn number_of_descriptors(&self) -> u16 {
    // SAFETY: 对象内存有效
    unsafe { (*self.as_ptr()).number_of_descriptors }
  }

  #[inline]
  fn set_number_of_descriptors(&self, value: u16) {
    // SAFETY: 独占写入
    unsafe {
      (*self.as_mut_ptr()).number_of_descriptors = value;
    }
  }

  #[inline]
  fn number_of_all_descriptors(&self) -> u16 {
    // SAFETY: 对象内存有效
    unsafe { (*self.as_ptr()).number_of_all_descriptors }
  }

  #[inline]
  fn set_number_of_all_descriptors(&self, value: u16) {
    // SAFETY: 独占写入
    unsafe {
      (*self.as_mut_ptr()).number_of_all_descriptors = value;
    }
  }

  #[inline]
  fn number_of_slack_descriptors(&self) -> u16 {
    self
      .number_of_all_descriptors()
      .saturating_sub(self.number_of_descriptors())
  }

  #[inline]
  fn flags(&self) -> u32 {
    // SAFETY: 对象内存有效
    unsafe { (*self.as_ptr()).flags }
  }

  #[inline]
  fn set_flags(&self, flags: u32) {
    // SAFETY: 独占写入
    unsafe {
      (*self.as_mut_ptr()).flags = flags;
    }
  }

  #[inline]
  fn fast_iterable(&self) -> FastIterableState {
    FastIterableState::from_u32(self.flags())
  }

  #[inline]
  fn set_fast_iterable(&self, state: FastIterableState) {
    let f = (self.flags() & !0b11) | (state as u32);
    self.set_flags(f);
  }

  #[inline]
  fn enum_cache(&self) -> Tagged<EnumCache> {
    // SAFETY: 对象内存有效
    unsafe { (*self.as_ptr()).enum_cache }
  }

  #[inline]
  fn set_enum_cache(&self, cache: Tagged<EnumCache>) {
    // SAFETY: 独占写入
    unsafe {
      (*self.as_mut_ptr()).enum_cache = cache;
    }
  }

  #[inline]
  fn get_key(&self, descriptor_number: usize) -> Result<Tagged<Name>> {
    if descriptor_number >= self.number_of_descriptors() as usize {
      return Err(Error::OutOfBounds {
        index: descriptor_number,
        length: self.number_of_descriptors() as usize,
      });
    }
    let slot_addr = self.address()
      + DescriptorArray::K_HEADER_SIZE
      + (descriptor_number * K_ENTRY_SIZE + K_ENTRY_KEY_INDEX) * DescriptorArray::K_SLOT_SIZE;
    // SAFETY: slot_addr 位于有效槽位边界内
    let ptr_val = unsafe { AtomicUsize::from_ptr(slot_addr as *mut usize).load(Ordering::Relaxed) };
    Ok(Tagged::new(ptr_val))
  }

  #[inline]
  fn get_details(&self, descriptor_number: usize) -> Result<PropertyDetails> {
    if descriptor_number >= self.number_of_descriptors() as usize {
      return Err(Error::OutOfBounds {
        index: descriptor_number,
        length: self.number_of_descriptors() as usize,
      });
    }
    let slot_addr = self.address()
      + DescriptorArray::K_HEADER_SIZE
      + (descriptor_number * K_ENTRY_SIZE + K_ENTRY_DETAILS_INDEX) * DescriptorArray::K_SLOT_SIZE;
    // SAFETY: slot_addr 位于有效槽位边界内
    let ptr_val = unsafe { AtomicUsize::from_ptr(slot_addr as *mut usize).load(Ordering::Relaxed) };
    Ok(PropertyDetails::from_smi(Tagged::new(ptr_val)))
  }

  #[inline]
  fn get_value(&self, descriptor_number: usize) -> Result<Tagged<MaybeObject>> {
    if descriptor_number >= self.number_of_descriptors() as usize {
      return Err(Error::OutOfBounds {
        index: descriptor_number,
        length: self.number_of_descriptors() as usize,
      });
    }
    let slot_addr = self.address()
      + DescriptorArray::K_HEADER_SIZE
      + (descriptor_number * K_ENTRY_SIZE + K_ENTRY_VALUE_INDEX) * DescriptorArray::K_SLOT_SIZE;
    // SAFETY: slot_addr 位于有效槽位边界内
    let ptr_val = unsafe { AtomicUsize::from_ptr(slot_addr as *mut usize).load(Ordering::Relaxed) };
    Ok(Tagged::new(ptr_val))
  }

  #[inline]
  fn get_strong_value(&self, descriptor_number: usize) -> Result<Tagged<Object>> {
    let val = self.get_value(descriptor_number)?;
    Ok(val.get_heap_object_or_smi())
  }

  #[inline]
  fn set_key(&self, descriptor_number: usize, key: Tagged<Name>) -> Result<()> {
    if descriptor_number >= self.number_of_all_descriptors() as usize {
      return Err(Error::OutOfBounds {
        index: descriptor_number,
        length: self.number_of_all_descriptors() as usize,
      });
    }
    let slot_addr = self.address()
      + DescriptorArray::K_HEADER_SIZE
      + (descriptor_number * K_ENTRY_SIZE + K_ENTRY_KEY_INDEX) * DescriptorArray::K_SLOT_SIZE;
    // SAFETY: slot_addr 位于有效容量边界内
    unsafe {
      AtomicUsize::from_ptr(slot_addr as *mut usize).store(key.ptr(), Ordering::Relaxed);
    }
    if self.fast_iterable() == FastIterableState::JsonFast {
      self.set_fast_iterable(FastIterableState::Unknown);
    }
    Ok(())
  }

  #[inline]
  fn set_details(&self, descriptor_number: usize, details: PropertyDetails) -> Result<()> {
    if descriptor_number >= self.number_of_all_descriptors() as usize {
      return Err(Error::OutOfBounds {
        index: descriptor_number,
        length: self.number_of_all_descriptors() as usize,
      });
    }
    let slot_addr = self.address()
      + DescriptorArray::K_HEADER_SIZE
      + (descriptor_number * K_ENTRY_SIZE + K_ENTRY_DETAILS_INDEX) * DescriptorArray::K_SLOT_SIZE;
    // SAFETY: slot_addr 位于有效容量边界内
    unsafe {
      AtomicUsize::from_ptr(slot_addr as *mut usize)
        .store(details.as_smi().ptr(), Ordering::Relaxed);
    }
    Ok(())
  }

  #[inline]
  fn set_value(&self, descriptor_number: usize, value: Tagged<MaybeObject>) -> Result<()> {
    if descriptor_number >= self.number_of_all_descriptors() as usize {
      return Err(Error::OutOfBounds {
        index: descriptor_number,
        length: self.number_of_all_descriptors() as usize,
      });
    }
    let slot_addr = self.address()
      + DescriptorArray::K_HEADER_SIZE
      + (descriptor_number * K_ENTRY_SIZE + K_ENTRY_VALUE_INDEX) * DescriptorArray::K_SLOT_SIZE;
    // SAFETY: slot_addr 位于有效容量边界内
    unsafe {
      AtomicUsize::from_ptr(slot_addr as *mut usize).store(value.ptr(), Ordering::Relaxed);
    }
    Ok(())
  }

  #[inline]
  fn set(
    &self,
    descriptor_number: usize,
    key: Tagged<Name>,
    value: Tagged<MaybeObject>,
    details: PropertyDetails,
  ) -> Result<()> {
    self.set_key(descriptor_number, key)?;
    self.set_details(descriptor_number, details)?;
    self.set_value(descriptor_number, value)?;
    Ok(())
  }

  fn append(
    &self,
    key: Tagged<Name>,
    value: Tagged<MaybeObject>,
    details: PropertyDetails,
  ) -> Result<()> {
    let desc_number = self.number_of_descriptors() as usize;
    let new_count = desc_number + 1;
    if new_count > self.number_of_all_descriptors() as usize {
      return Err(Error::InvalidCapacity(new_count));
    }
    self.set_number_of_descriptors(new_count as u16);
    // 默认 pointer 设为自身下标
    let details_with_pointer = details.set_pointer(desc_number as u32);
    self.set(desc_number, key, value, details_with_pointer)?;

    if new_count <= K_MAX_ELEMENTS_FOR_LINEAR_SEARCH {
      // 线性阶段保持无重复检查
      return Ok(());
    } else if new_count == K_MAX_ELEMENTS_FOR_LINEAR_SEARCH + 1 {
      // 刚跨过线性阈值，全量排序
      self.sort();
      return Ok(());
    }

    // 二分插入法保持有序: 在 sorted 序列中逆序寻找插入点
    let desc_hash = key.hash();
    let mut insertion = desc_number;
    while insertion > 0 {
      let prev_key = self.get_sorted_key(insertion - 1);
      if prev_key.hash() <= desc_hash {
        break;
      }
      self.set_sorted_key(insertion, self.get_sorted_key_index(insertion - 1));
      insertion -= 1;
    }
    self.set_sorted_key(insertion, desc_number);
    Ok(())
  }

  #[inline]
  fn get_sorted_key_index(&self, descriptor_number: usize) -> usize {
    self
      .get_details(descriptor_number)
      .map(|d| d.pointer() as usize)
      .unwrap_or(descriptor_number)
  }

  #[inline]
  fn get_sorted_key(&self, descriptor_number: usize) -> Tagged<Name> {
    let sorted_index = self.get_sorted_key_index(descriptor_number);
    self.get_key(sorted_index).unwrap_or(Tagged::NULL)
  }

  #[inline]
  fn set_sorted_key(&self, descriptor_number: usize, pointer: usize) {
    if let Ok(details) = self.get_details(descriptor_number) {
      let _ = self.set_details(descriptor_number, details.set_pointer(pointer as u32));
    }
  }

  #[inline]
  fn swap_sorted_keys(&self, first: usize, second: usize) {
    let first_key = self.get_sorted_key_index(first);
    self.set_sorted_key(first, self.get_sorted_key_index(second));
    self.set_sorted_key(second, first_key);
  }

  fn sort(&self) {
    let len = self.number_of_descriptors() as usize;
    if len <= 1 {
      return;
    }
    // 1. 初始化 pointer: pointer[i] = i
    for i in 0..len {
      self.set_sorted_key(i, i);
    }

    // 堆下沉辅助闭包
    let sift_down = |mut parent: usize, heap_size: usize| {
      while 2 * parent + 1 < heap_size {
        let mut child = 2 * parent + 1;
        let mut child_hash = self.get_sorted_key(child).hash();
        if child + 1 < heap_size {
          let right_child_hash = self.get_sorted_key(child + 1).hash();
          if right_child_hash > child_hash {
            child += 1;
            child_hash = right_child_hash;
          }
        }
        let parent_hash = self.get_sorted_key(parent).hash();
        if child_hash <= parent_hash {
          break;
        }
        self.swap_sorted_keys(parent, child);
        parent = child;
      }
    };

    // 2. 自底向上建大顶堆
    for i in (0..(len / 2)).rev() {
      sift_down(i, len);
    }

    // 3. 逐个提取堆顶并下沉，实现升序
    for i in (1..len).rev() {
      self.swap_sorted_keys(0, i);
      sift_down(0, i);
    }
  }

  #[inline]
  fn search(&self, name: Tagged<Name>, valid_descriptors: usize) -> Option<usize> {
    if valid_descriptors == 0 {
      return None;
    }
    if valid_descriptors <= K_MAX_ELEMENTS_FOR_LINEAR_SEARCH {
      self.linear_search(name, valid_descriptors)
    } else {
      self.binary_search(name, valid_descriptors)
    }
  }

  #[inline]
  fn linear_search(&self, name: Tagged<Name>, valid_descriptors: usize) -> Option<usize> {
    let len = (self.number_of_descriptors() as usize).min(valid_descriptors);
    for i in 0..len {
      if let Ok(k) = self.get_key(i)
        && k == name
      {
        return Some(i);
      }
    }
    None
  }

  fn binary_search(&self, name: Tagged<Name>, valid_descriptors: usize) -> Option<usize> {
    let end = self.number_of_descriptors() as usize;
    if end == 0 {
      return None;
    }
    let hash = name.hash();
    let mut low = 0;
    let mut high = end;
    while low < high {
      let mid = low + (high - low) / 2;
      if self.get_sorted_key(mid).hash() < hash {
        low = mid + 1;
      } else {
        high = mid;
      }
    }
    let mut number = low;
    while number < end {
      let sorted_index = self.get_sorted_key_index(number);
      let entry_key = self.get_key(sorted_index).ok()?;
      if entry_key == name {
        if sorted_index >= valid_descriptors {
          return None;
        }
        return Some(sorted_index);
      }
      if entry_key.hash() != hash {
        return None;
      }
      number += 1;
    }
    None
  }

  fn search_with_cache(
    &self,
    cache: &mut DescriptorLookupCache,
    name: Tagged<Name>,
    map: Tagged<Map>,
  ) -> Option<usize> {
    let cached = cache.lookup(map, name);
    if cached != DescriptorLookupCache::K_ABSENT {
      return if cached >= 0 {
        Some(cached as usize)
      } else {
        None
      };
    }
    // SAFETY: map 指向有效 Map
    let valid_descriptors = unsafe { (*map.as_ptr()).number_of_own_descriptors() as usize };
    let result = self.search(name, valid_descriptors);
    let res_int = match result {
      Some(idx) => idx as i32,
      None => K_NOT_FOUND,
    };
    cache.update(map, name, res_int);
    result
  }
}

/// 拥有独立内存块的 DescriptorArray 容器（方便测试与脱离堆环境的构造）
pub struct DescriptorArrayBuf {
  _storage: Vec<u64>,
  tagged: Tagged<DescriptorArray>,
}

impl DescriptorArrayBuf {
  pub fn new(map: Tagged<Map>, capacity: u16) -> Self {
    let bytes = DescriptorArray::size_for(capacity as usize);
    let words = bytes.div_ceil(8);
    let mut storage = vec![0u64; words];
    let base = storage.as_mut_ptr() as usize;
    let header = DescriptorArray::new(map, capacity);
    // SAFETY: base 指向已分配的 storage 向量首地址，满足 u64 对齐与尺寸需求
    unsafe {
      ptr::write(base as *mut DescriptorArray, header);
    }
    Self {
      _storage: storage,
      // SAFETY: base 指向有效的 DescriptorArray 内存
      tagged: unsafe { Tagged::from_raw_address(base) },
    }
  }

  #[inline]
  pub fn as_tagged(&self) -> Tagged<DescriptorArray> {
    self.tagged
  }
}

impl Deref for DescriptorArrayBuf {
  type Target = Tagged<DescriptorArray>;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.tagged
  }
}
