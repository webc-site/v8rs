// v8-objects-maps/src/transitions.rs
// 对应原版 v8/src/objects/transitions.h + transitions-inl.h + transitions.cc

//! Transitions: 形状过渡树 (TransitionArray 与 TransitionsAccessor)。
//!
//! Map 的过渡信息保存在 `Map::transitions_or_prototype_info` 槽位中，支持三种形态:
//! 1. `kUninitialized`: 无过渡 (槽位为 null / Smi)
//! 2. `kWeakRef`: 单一弱引用过渡 (`Weak<Map>`)，键与属性详情直接从目标 Map 获取
//! 3. `kFullTransitionArray`: 多分支过渡数组 (`TransitionArray`)

use std::{
  collections::VecDeque,
  mem,
  ops::Deref,
  ptr,
  sync::atomic::{AtomicUsize, Ordering},
};

use v8_objects_arrays::{
  fixed_array::{TaggedArray, WeakFixedArray},
  fixed_array_base::FixedArrayBase,
};
use v8_objects_core::{
  instance_type::InstanceType,
  map::Map,
  name::Name,
  property_details::{PropertyAttributes, PropertyKind},
  smi::Smi,
  tagged::{MaybeObject, Object, Tagged, make_weak},
  tagged_field::TaggedSlot,
};

use crate::{
  descriptor_array::{DescriptorArray, DescriptorArrayExt},
  error::{Error, Result},
  name_ext::NameTaggedExt,
};

pub const K_MAX_NUMBER_OF_TRANSITIONS: usize = 1536;
pub const K_MAX_ELEMENTS_FOR_LINEAR_SEARCH: usize = 32;

pub const K_PROTOTYPE_TRANSITIONS_INDEX: usize = 0;
pub const K_SIDE_STEP_TRANSITIONS_INDEX: usize = 1;
pub const K_TRANSITION_LENGTH_INDEX: usize = 2;
pub const K_FIRST_INDEX: usize = 3;

pub const K_ENTRY_KEY_INDEX: usize = 0;
pub const K_ENTRY_TARGET_INDEX: usize = 1;
pub const K_ENTRY_SIZE: usize = 2;

/// 侧步过渡类别
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SideStepTransitionKind {
  CloneObject = 0,
  ObjectAssign = 1,
  ObjectAssignValidityCell = 2,
}

impl SideStepTransitionKind {
  /// v8 原版对位: transitions.h:43 — `static_cast<uint32_t>(kind)` (侧步过渡槽位下标)。
  #[inline]
  #[must_use]
  pub const fn index_of(self) -> u32 {
    self as u32
  }
}

pub const SIDE_STEP_TRANSITION_SIZE: usize = 3;
pub const SIDE_STEP_TRANSITION_EMPTY: Tagged<Smi> = Smi::from_int(0);
pub const SIDE_STEP_TRANSITION_UNREACHABLE: Tagged<Smi> = Smi::from_int(1);

// 侧步过渡数组下标界 (v8 原版对位: transitions.h:46-49 kFirstMapIdx/kLastMapIdx)。
pub const SIDE_STEP_FIRST_MAP_IDX: u32 = 0;
pub const SIDE_STEP_LAST_MAP_IDX: u32 = 1;

// 原型过渡缓存数组布局 (v8 原版对位: transitions.h:398-409)。
// 缓存格式: [0] 条目数 (Smi 指纹), [header + i] 目标 Map (弱引用)。
pub const K_PROTO_TRANSITION_HEADER_SIZE: usize = 1;
pub const K_PROTO_TRANSITION_NUMBER_OF_ENTRIES_OFFSET: usize = 0;
pub const K_MAX_CACHED_PROTOTYPE_TRANSITIONS: usize = 256;

/// 过渡槽位编码状态
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Encoding {
  PrototypeInfo = 0,
  Uninitialized = 1,
  MigrationTarget = 2,
  WeakRef = 3,
  FullTransitionArray = 4,
  PrototypeSharedClosureInfo = 5,
}

/// 内存分配抽象特征，支持 Zone、`Vec<u64>` 或外部内存池
///
/// 模拟地址契约 (W11MPR round1 固化): allocate 以 usize 裸地址交付对象内存, 后续经
/// `Tagged::from_raw_address` ↔ `address()` 往返访问, 槽位读写走 `Atomic*::from_ptr`。
/// Miri Stacked Borrows 下该往返仅在「暴露点携带可写裸 provenance」时合法: 来源须经
/// `Tagged::from_ptr(&raw mut x)` / `Box::leak(..) as *mut T` / 循环外一次性
/// `as_mut_ptr()` hoist 等形态派生; 若经 `Tagged::from_ref(&x)` 或共享 reborrow 暴露,
/// 后续原子写与跨元素旧 tag 会触发 `<wildcard>` retag 违规。测试 harness 已按此改造,
/// 台账/白名单见 log/v8-objects-maps.md 与 todo.md §13。
pub trait MemoryAllocator {
  fn allocate(&mut self, size: usize) -> Result<usize>;
}

impl<'a> MemoryAllocator for v8_zone::Zone<'a> {
  #[inline]
  fn allocate(&mut self, size: usize) -> Result<usize> {
    let slice = self.allocate_bytes_ref(size)?;
    Ok(slice.as_mut_ptr() as usize)
  }
}

impl MemoryAllocator for Vec<u64> {
  #[inline]
  fn allocate(&mut self, size: usize) -> Result<usize> {
    let words = size.div_ceil(8);
    let old_len = self.len();
    if old_len + words > self.capacity() {
      // 严禁隐式动态重分配导致已分配对象地址悬垂
      return Err(Error::OutOfMemory);
    }
    self.resize(old_len + words, 0);
    let base = self.as_ptr() as usize + old_len * 8;
    Ok(base)
  }
}

impl<F: FnMut(usize) -> Result<usize>> MemoryAllocator for F {
  #[inline]
  fn allocate(&mut self, size: usize) -> Result<usize> {
    self(size)
  }
}

/// TransitionArray 内部继承 WeakFixedArray (带 map + length 的定长数组)
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TransitionArray {
  pub base: FixedArrayBase,
}

impl TransitionArray {
  pub const K_HEADER_SIZE: usize = mem::size_of::<Self>();
  pub const K_SLOT_SIZE: usize = mem::size_of::<TaggedSlot>();

  #[inline]
  pub const fn to_key_index(transition_number: usize) -> usize {
    K_FIRST_INDEX + transition_number * K_ENTRY_SIZE + K_ENTRY_KEY_INDEX
  }

  #[inline]
  pub const fn to_target_index(transition_number: usize) -> usize {
    K_FIRST_INDEX + transition_number * K_ENTRY_SIZE + K_ENTRY_TARGET_INDEX
  }

  #[inline]
  pub const fn size_for(capacity: usize) -> usize {
    Self::K_HEADER_SIZE + (K_FIRST_INDEX + capacity * K_ENTRY_SIZE) * Self::K_SLOT_SIZE
  }

  #[inline]
  pub const fn new(map: Tagged<Map>, capacity: usize) -> Self {
    let total_length = (K_FIRST_INDEX + capacity * K_ENTRY_SIZE) as u32;
    Self {
      base: FixedArrayBase::new(map, total_length),
    }
  }

  /// 在分配器中构造并初始化 TransitionArray
  pub fn allocate<A: MemoryAllocator>(
    allocator: &mut A,
    map: Tagged<Map>,
    capacity: usize,
  ) -> Result<Tagged<Self>> {
    let size = Self::size_for(capacity);
    let addr = allocator.allocate(size)?;
    let header = Self::new(map, capacity);
    // SAFETY: addr 来自内存分配器，尺寸和容量满足对象及条目存储要求
    unsafe {
      ptr::write(addr as *mut Self, header);
      let tagged = Tagged::from_raw_address(addr);
      // 初始化固定头部槽位与所有条目槽位
      let total_slots = K_FIRST_INDEX + capacity * K_ENTRY_SIZE;
      for i in 0..total_slots {
        tagged.set_raw_slot(i, Tagged::<Object>::NULL.cast());
      }
      let zero_smi: Tagged<MaybeObject> = Smi::from_int(0).cast();
      tagged.set_raw_slot(K_PROTOTYPE_TRANSITIONS_INDEX, zero_smi);
      tagged.set_raw_slot(K_SIDE_STEP_TRANSITIONS_INDEX, zero_smi);
      tagged.set_raw_slot(K_TRANSITION_LENGTH_INDEX, zero_smi);
      Ok(tagged)
    }
  }

  #[inline]
  pub fn target_details(
    key: Tagged<Name>,
    target: Tagged<MaybeObject>,
  ) -> (PropertyKind, PropertyAttributes) {
    if let Some(target_map) = target.get_heap_object_if_weak() {
      TransitionsAccessor::get_target_details(key, target_map.cast())
    } else {
      (PropertyKind::Data, PropertyAttributes::NONE)
    }
  }

  #[inline]
  pub fn compare_details(
    kind1: PropertyKind,
    attributes1: PropertyAttributes,
    kind2: PropertyKind,
    attributes2: PropertyAttributes,
  ) -> i32 {
    if kind1 != kind2 {
      return if (kind1 as u8) < (kind2 as u8) { -1 } else { 1 };
    }
    if attributes1 != attributes2 {
      return if attributes1.0 < attributes2.0 { -1 } else { 1 };
    }
    0
  }

  #[inline]
  pub fn compare_keys(
    key1: Tagged<Name>,
    kind1: PropertyKind,
    attributes1: PropertyAttributes,
    key2: Tagged<Name>,
    kind2: PropertyKind,
    attributes2: PropertyAttributes,
  ) -> i32 {
    if key1 != key2 {
      let h1 = key1.hash();
      let h2 = key2.hash();
      // 原版 transitions-inl.h:426-427: In case of hash collisions key1 is always "less" than key2.
      return if h1 <= h2 { -1 } else { 1 };
    }
    Self::compare_details(kind1, attributes1, kind2, attributes2)
  }

  /// v8: transitions-inl.h:121 — TransitionArray::NumberOfPrototypeTransitions
  #[inline]
  pub fn number_of_prototype_transitions(proto_transitions: Tagged<WeakFixedArray>) -> usize {
    use v8_objects_arrays::fixed_array::TaggedArrayExt;
    let len = unsafe { proto_transitions.length() };
    if len == 0 {
      return 0;
    }
    let raw = unsafe { proto_transitions.get(K_PROTO_TRANSITION_NUMBER_OF_ENTRIES_OFFSET as u32) };
    raw.as_smi().map(|s| s.value().max(0) as usize).unwrap_or(0)
  }
}

impl TaggedArray for TransitionArray {
  type Element = MaybeObject;

  const LENGTH_OFFSET: usize = mem::offset_of!(TransitionArray, base.length);
  const CAPACITY_OFFSET: usize = Self::LENGTH_OFFSET;
  const OBJECTS_OFFSET: usize = mem::size_of::<TransitionArray>();
}

/// 对 `Tagged<TransitionArray>` 的扩展方法
pub trait TransitionArrayExt {
  fn raw_slot_addr(&self, index: usize) -> usize;
  fn get_raw_slot(&self, index: usize) -> Tagged<MaybeObject>;
  fn get_raw_slot_acquire(&self, index: usize) -> Tagged<MaybeObject>;
  fn set_raw_slot(&self, index: usize, val: Tagged<MaybeObject>);
  fn set_raw_slot_release(&self, index: usize, val: Tagged<MaybeObject>);

  fn capacity(&self) -> usize;
  fn number_of_transitions(&self) -> usize;
  fn set_number_of_transitions(&self, count: usize);

  fn get_key(&self, transition_number: usize) -> Tagged<Name>;
  fn set_key(&self, transition_number: usize, key: Tagged<Name>);
  fn get_raw_target(&self, transition_number: usize) -> Tagged<MaybeObject>;
  fn set_raw_target(&self, transition_number: usize, target: Tagged<MaybeObject>);
  fn get_target(&self, transition_number: usize) -> Option<Tagged<Map>>;
  fn set(&self, transition_number: usize, key: Tagged<Name>, target: Tagged<MaybeObject>);

  fn has_prototype_transitions(&self) -> bool;
  fn get_prototype_transitions(&self) -> Option<Tagged<WeakFixedArray>>;
  fn set_prototype_transitions(&self, transitions: Tagged<WeakFixedArray>);

  fn has_side_step_transitions(&self) -> bool;
  fn get_side_step_transitions(&self) -> Option<Tagged<WeakFixedArray>>;
  fn set_side_step_transitions(&self, transitions: Tagged<WeakFixedArray>);

  fn search(
    &self,
    kind: PropertyKind,
    name: Tagged<Name>,
    attributes: PropertyAttributes,
    out_insertion_index: &mut Option<usize>,
  ) -> Option<usize>;
  fn search_name(
    &self,
    name: Tagged<Name>,
    concurrent_search: bool,
    out_insertion_index: &mut Option<usize>,
  ) -> Option<usize>;
  fn linear_search_name(
    &self,
    name: Tagged<Name>,
    out_insertion_index: &mut Option<usize>,
  ) -> Option<usize>;
  fn binary_search_name(
    &self,
    name: Tagged<Name>,
    out_insertion_index: &mut Option<usize>,
  ) -> Option<usize>;
  fn search_and_get_target(
    &self,
    kind: PropertyKind,
    name: Tagged<Name>,
    attributes: PropertyAttributes,
  ) -> Option<Tagged<Map>>;
  fn sort(&self, force: bool);
}

impl TransitionArrayExt for Tagged<TransitionArray> {
  #[inline]
  fn raw_slot_addr(&self, index: usize) -> usize {
    self.address() + TransitionArray::K_HEADER_SIZE + index * TransitionArray::K_SLOT_SIZE
  }

  #[inline]
  fn get_raw_slot(&self, index: usize) -> Tagged<MaybeObject> {
    let addr = self.raw_slot_addr(index);
    // SAFETY: addr 位于有效的 TransitionArray 槽位范围内
    let val = unsafe { AtomicUsize::from_ptr(addr as *mut usize).load(Ordering::Relaxed) };
    Tagged::new(val)
  }

  #[inline]
  fn get_raw_slot_acquire(&self, index: usize) -> Tagged<MaybeObject> {
    let addr = self.raw_slot_addr(index);
    // SAFETY: addr 位于有效的 TransitionArray 槽位范围内
    let val = unsafe { AtomicUsize::from_ptr(addr as *mut usize).load(Ordering::Acquire) };
    Tagged::new(val)
  }

  #[inline]
  fn set_raw_slot(&self, index: usize, val: Tagged<MaybeObject>) {
    let addr = self.raw_slot_addr(index);
    // SAFETY: addr 位于有效的 TransitionArray 槽位范围内
    unsafe {
      AtomicUsize::from_ptr(addr as *mut usize).store(val.ptr(), Ordering::Relaxed);
    }
  }

  #[inline]
  fn set_raw_slot_release(&self, index: usize, val: Tagged<MaybeObject>) {
    let addr = self.raw_slot_addr(index);
    // SAFETY: addr 位于有效的 TransitionArray 槽位范围内
    unsafe {
      AtomicUsize::from_ptr(addr as *mut usize).store(val.ptr(), Ordering::Release);
    }
  }

  #[inline]
  fn capacity(&self) -> usize {
    // SAFETY: self 指向有效的 TransitionArray 内存
    let len = unsafe { (*self.as_ptr()).base.length as usize };
    if len <= K_FIRST_INDEX {
      0
    } else {
      (len - K_FIRST_INDEX) / K_ENTRY_SIZE
    }
  }

  #[inline]
  fn number_of_transitions(&self) -> usize {
    let len = unsafe { (*self.as_ptr()).base.length as usize };
    if len < K_FIRST_INDEX {
      return 0;
    }
    let smi = self.get_raw_slot(K_TRANSITION_LENGTH_INDEX);
    smi.as_smi().map(|s| s.value().max(0) as usize).unwrap_or(0)
  }

  #[inline]
  fn set_number_of_transitions(&self, count: usize) {
    debug_assert!(count <= self.capacity());
    let smi = Smi::from_int(count as i32);
    self.set_raw_slot(K_TRANSITION_LENGTH_INDEX, smi.cast());
  }

  #[inline]
  fn get_key(&self, transition_number: usize) -> Tagged<Name> {
    let idx = TransitionArray::to_key_index(transition_number);
    self.get_raw_slot(idx).cast()
  }

  #[inline]
  fn set_key(&self, transition_number: usize, key: Tagged<Name>) {
    let idx = TransitionArray::to_key_index(transition_number);
    self.set_raw_slot(idx, key.cast());
  }

  #[inline]
  fn get_raw_target(&self, transition_number: usize) -> Tagged<MaybeObject> {
    let idx = TransitionArray::to_target_index(transition_number);
    self.get_raw_slot(idx)
  }

  #[inline]
  fn set_raw_target(&self, transition_number: usize, target: Tagged<MaybeObject>) {
    let idx = TransitionArray::to_target_index(transition_number);
    self.set_raw_slot(idx, target);
  }

  #[inline]
  fn get_target(&self, transition_number: usize) -> Option<Tagged<Map>> {
    let raw = self.get_raw_target(transition_number);
    raw.get_heap_object_if_weak().map(|h| h.cast())
  }

  #[inline]
  fn set(&self, transition_number: usize, key: Tagged<Name>, target: Tagged<MaybeObject>) {
    self.set_key(transition_number, key);
    self.set_raw_target(transition_number, target);
  }

  #[inline]
  fn has_prototype_transitions(&self) -> bool {
    // 原版 transitions-inl.h:47: get(kPrototypeTransitionsIndex, kAcquireLoad) != Smi::zero()
    let slot = self.get_raw_slot_acquire(K_PROTOTYPE_TRANSITIONS_INDEX);
    !slot.is_null() && slot != Smi::from_int(0).cast()
  }

  #[inline]
  fn get_prototype_transitions(&self) -> Option<Tagged<WeakFixedArray>> {
    if self.has_prototype_transitions() {
      Some(
        self
          .get_raw_slot_acquire(K_PROTOTYPE_TRANSITIONS_INDEX)
          .cast(),
      )
    } else {
      None
    }
  }

  #[inline]
  fn set_prototype_transitions(&self, transitions: Tagged<WeakFixedArray>) {
    // 原版 transitions-inl.h:118: set(kPrototypeTransitionsIndex, transitions, kReleaseStore)
    self.set_raw_slot_release(K_PROTOTYPE_TRANSITIONS_INDEX, transitions.cast());
  }

  #[inline]
  fn has_side_step_transitions(&self) -> bool {
    let slot = self.get_raw_slot(K_SIDE_STEP_TRANSITIONS_INDEX);
    !slot.is_null() && slot != Smi::from_int(0).cast()
  }

  #[inline]
  fn get_side_step_transitions(&self) -> Option<Tagged<WeakFixedArray>> {
    if self.has_side_step_transitions() {
      Some(self.get_raw_slot(K_SIDE_STEP_TRANSITIONS_INDEX).cast())
    } else {
      None
    }
  }

  #[inline]
  fn set_side_step_transitions(&self, transitions: Tagged<WeakFixedArray>) {
    self.set_raw_slot(K_SIDE_STEP_TRANSITIONS_INDEX, transitions.cast());
  }

  fn search_name(
    &self,
    name: Tagged<Name>,
    concurrent_search: bool,
    out_insertion_index: &mut Option<usize>,
  ) -> Option<usize> {
    let count = self.number_of_transitions();
    if count == 0 {
      *out_insertion_index = Some(0);
      return None;
    }
    if count <= K_MAX_ELEMENTS_FOR_LINEAR_SEARCH || concurrent_search {
      self.linear_search_name(name, out_insertion_index)
    } else {
      self.binary_search_name(name, out_insertion_index)
    }
  }

  fn linear_search_name(
    &self,
    name: Tagged<Name>,
    out_insertion_index: &mut Option<usize>,
  ) -> Option<usize> {
    let len = self.number_of_transitions();
    for i in 0..len {
      if self.get_key(i) == name {
        return Some(i);
      }
    }
    *out_insertion_index = Some(len);
    None
  }

  fn binary_search_name(
    &self,
    name: Tagged<Name>,
    out_insertion_index: &mut Option<usize>,
  ) -> Option<usize> {
    let end = self.number_of_transitions();
    let hash = name.hash();
    let mut low = 0;
    let mut high = end;
    while low < high {
      let mid = low + (high - low) / 2;
      if self.get_key(mid).hash() < hash {
        low = mid + 1;
      } else {
        high = mid;
      }
    }
    let mut i = low;
    while i < end {
      let entry = self.get_key(i);
      if entry == name {
        return Some(i);
      }
      let entry_hash = entry.hash();
      if entry_hash != hash {
        *out_insertion_index = Some(i + if entry_hash > hash { 0 } else { 1 });
        return None;
      }
      i += 1;
    }
    *out_insertion_index = Some(end);
    None
  }

  fn search(
    &self,
    kind: PropertyKind,
    name: Tagged<Name>,
    attributes: PropertyAttributes,
    out_insertion_index: &mut Option<usize>,
  ) -> Option<usize> {
    let transition = self.search_name(name, false, out_insertion_index)?;
    let count = self.number_of_transitions();
    let mut i = transition;
    while i < count && self.get_key(i) == name {
      if let Some(target) = self.get_target(i) {
        let (target_kind, target_attributes) =
          TransitionsAccessor::get_target_details(name, target);
        let cmp =
          TransitionArray::compare_details(kind, attributes, target_kind, target_attributes);
        if cmp == 0 {
          return Some(i);
        } else if cmp < 0 {
          break;
        }
      }
      i += 1;
    }
    *out_insertion_index = Some(i);
    None
  }

  fn search_and_get_target(
    &self,
    kind: PropertyKind,
    name: Tagged<Name>,
    attributes: PropertyAttributes,
  ) -> Option<Tagged<Map>> {
    let mut insertion = None;
    let idx = self.search(kind, name, attributes, &mut insertion)?;
    self.get_target(idx)
  }

  fn sort(&self, force: bool) {
    let length = self.number_of_transitions();
    if !force && length <= K_MAX_ELEMENTS_FOR_LINEAR_SEARCH {
      return;
    }
    for i in 1..length {
      let key = self.get_key(i);
      let target = self.get_raw_target(i);
      let (kind, attributes) = TransitionArray::target_details(key, target);
      let mut j = i;
      while j > 0 {
        let temp_key = self.get_key(j - 1);
        let temp_target = self.get_raw_target(j - 1);
        let (temp_kind, temp_attributes) = TransitionArray::target_details(temp_key, temp_target);
        let cmp = TransitionArray::compare_keys(
          temp_key,
          temp_kind,
          temp_attributes,
          key,
          kind,
          attributes,
        );
        if cmp > 0 {
          self.set(j, temp_key, temp_target);
          j -= 1;
        } else {
          break;
        }
      }
      self.set(j, key, target);
    }
  }
}

/// TransitionsAccessor: 统一访问与修改 Map 过渡结构的门面工具
pub struct TransitionsAccessor {
  map: Tagged<Map>,
  raw_transitions: Tagged<MaybeObject>,
  encoding: Encoding,
  concurrent_access: bool,
}

impl TransitionsAccessor {
  pub fn new(map: Tagged<Map>) -> Self {
    Self::new_with_concurrent(map, false)
  }

  pub fn new_with_concurrent(map: Tagged<Map>, concurrent_access: bool) -> Self {
    // SAFETY: map 指针有效，指向已分配的 Map 堆对象; 原版 transitions-inl.h:337 map->raw_transitions(kAcquireLoad)
    let raw = unsafe {
      let slot_addr =
        (map.address() + mem::offset_of!(Map, transitions_or_prototype_info)) as *mut usize;
      let val = AtomicUsize::from_ptr(slot_addr).load(Ordering::Acquire);
      Tagged::<MaybeObject>::new(val)
    };
    let encoding = Self::get_encoding(raw);
    Self {
      map,
      raw_transitions: raw,
      encoding,
      concurrent_access,
    }
  }

  #[inline]
  pub fn concurrent_access(&self) -> bool {
    self.concurrent_access
  }

  #[inline]
  pub fn map(&self) -> Tagged<Map> {
    self.map
  }

  #[inline]
  pub fn encoding(&self) -> Encoding {
    self.encoding
  }

  #[inline]
  pub fn get_encoding(raw: Tagged<MaybeObject>) -> Encoding {
    if raw.is_null() || raw.is_smi() || raw.is_cleared() {
      Encoding::Uninitialized
    } else if raw.is_weak_live() {
      Encoding::WeakRef
    } else if let Some(heap_obj) = raw.get_heap_object_if_strong() {
      // 强引用堆对象: 判定是 TransitionArray, PrototypeInfo, PrototypeSharedClosureInfo 还是 Map (MigrationTarget)
      let map = unsafe { (*heap_obj.as_ptr()).map };
      if map.is_heap_object() {
        let it = unsafe { (*map.as_ptr()).instance_type };
        match it {
          InstanceType::TransitionArray => Encoding::FullTransitionArray,
          InstanceType::PrototypeInfo => Encoding::PrototypeInfo,
          InstanceType::PrototypeSharedClosureInfo => Encoding::PrototypeSharedClosureInfo,
          InstanceType::Map => Encoding::MigrationTarget,
          _ => Encoding::FullTransitionArray,
        }
      } else {
        Encoding::FullTransitionArray
      }
    } else {
      Encoding::Uninitialized
    }
  }

  #[inline]
  pub fn transitions(&self) -> Option<Tagged<TransitionArray>> {
    if self.encoding == Encoding::FullTransitionArray {
      Some(self.raw_transitions.cast())
    } else {
      None
    }
  }

  #[inline]
  pub fn get_target_from_weak_ref(&self) -> Option<Tagged<Map>> {
    self
      .raw_transitions
      .get_heap_object_if_weak()
      .map(|h| h.cast())
  }

  #[inline]
  pub fn has_prototype_transitions(&self) -> bool {
    self
      .transitions()
      .is_some_and(|t| t.has_prototype_transitions())
  }

  #[inline]
  pub fn get_prototype_transitions(&self) -> Option<Tagged<WeakFixedArray>> {
    self
      .transitions()
      .and_then(|t| t.get_prototype_transitions())
  }

  #[inline]
  pub fn has_side_step_transitions(&self) -> bool {
    self
      .transitions()
      .is_some_and(|t| t.has_side_step_transitions())
  }

  #[inline]
  pub fn get_side_step_transitions(&self) -> Option<Tagged<WeakFixedArray>> {
    self
      .transitions()
      .and_then(|t| t.get_side_step_transitions())
  }

  pub fn search_transition(
    &self,
    name: Tagged<Name>,
    kind: PropertyKind,
    attributes: PropertyAttributes,
  ) -> Option<Tagged<Map>> {
    match self.encoding {
      Encoding::Uninitialized
      | Encoding::PrototypeInfo
      | Encoding::MigrationTarget
      | Encoding::PrototypeSharedClosureInfo => None,
      Encoding::WeakRef => {
        let target = self.get_target_from_weak_ref()?;
        if Self::is_matching_map(target, name, kind, attributes) {
          Some(target)
        } else {
          None
        }
      }
      Encoding::FullTransitionArray => {
        let array = self.transitions()?;
        array.search_and_get_target(kind, name, attributes)
      }
    }
  }

  pub fn search_special(&self, name: Tagged<Name>) -> Option<Tagged<Map>> {
    match self.encoding {
      Encoding::FullTransitionArray => {
        let array = self.transitions()?;
        let mut insertion = None;
        let idx = array.search_name(name, self.concurrent_access, &mut insertion)?;
        array.get_target(idx)
      }
      Encoding::WeakRef => {
        let target = self.get_target_from_weak_ref()?;
        if Self::get_simple_transition_key(target) == name {
          Some(target)
        } else {
          None
        }
      }
      _ => None,
    }
  }

  /// 遍历当前 Map 的所有直接子过渡目标 Map
  pub fn for_each_transition<F: FnMut(Tagged<Map>)>(&self, mut f: F) {
    match self.encoding {
      Encoding::WeakRef => {
        if let Some(target) = self.get_target_from_weak_ref() {
          f(target);
        }
      }
      Encoding::FullTransitionArray => {
        if let Some(array) = self.transitions() {
          // 原版 transitions.cc:661-676: 先遍历原型过渡分支
          if array.has_prototype_transitions()
            && let Some(proto_trans) = array.get_prototype_transitions()
          {
            use v8_objects_arrays::fixed_array::TaggedArrayExt;
            let proto_count = TransitionArray::number_of_prototype_transitions(proto_trans);
            for i in 0..proto_count {
              let idx = (K_PROTO_TRANSITION_HEADER_SIZE + i) as u32;
              let target = unsafe { proto_trans.get(idx) };
              if let Some(map) = target.get_heap_object_if_weak() {
                f(map.cast());
              }
            }
          }
          let count = array.number_of_transitions();
          for i in 0..count {
            if let Some(target) = array.get_target(i) {
              f(target);
            }
          }
        }
      }
      _ => {}
    }
  }

  /// 广度优先遍历以此 Map 为根的整棵过渡子树
  pub fn traverse_transition_tree<F: FnMut(Tagged<Map>)>(root: Tagged<Map>, mut f: F) {
    let mut queue = VecDeque::new();
    queue.push_back(root);
    while let Some(current) = queue.pop_front() {
      f(current);
      let accessor = Self::new(current);
      accessor.for_each_transition(|child| {
        queue.push_back(child);
      });
    }
  }

  /// 插入新过渡
  pub fn insert<A: MemoryAllocator>(
    map: Tagged<Map>,
    name: Tagged<Name>,
    target: Tagged<Map>,
    allocator: &mut A,
  ) -> Result<()> {
    // 原版 transitions.cc:51 (*target)->SetBackPointer(*map)
    unsafe {
      (*target.as_mut_ptr()).constructor_or_back_pointer_or_native_context = map.cast();
    }

    let accessor = Self::new(map);
    match accessor.encoding {
      Encoding::Uninitialized | Encoding::MigrationTarget => {
        // 直接变为单一弱引用过渡
        // SAFETY: map 独占写入过渡槽位, Release 发布保证写入可见
        unsafe {
          let slot_addr =
            (map.address() + mem::offset_of!(Map, transitions_or_prototype_info)) as *mut usize;
          let val = make_weak(target).cast::<Object>().ptr();
          AtomicUsize::from_ptr(slot_addr).store(val, Ordering::Release);
        }
        Ok(())
      }
      Encoding::WeakRef => {
        let old_target = accessor.get_target_from_weak_ref().ok_or(Error::NotFound)?;
        let old_key = Self::get_simple_transition_key(old_target);
        let (old_kind, old_attributes) = Self::get_target_details(old_key, old_target);
        let (new_kind, new_attributes) = Self::get_target_details(name, target);
        if old_key == name && old_kind == new_kind && old_attributes == new_attributes {
          // 相同名称与属性的过渡已存在于单一弱引用中，原地更新 target
          // SAFETY: map 独占写入过渡槽位, Release 发布
          unsafe {
            let slot_addr =
              (map.address() + mem::offset_of!(Map, transitions_or_prototype_info)) as *mut usize;
            let val = make_weak(target).cast::<Object>().ptr();
            AtomicUsize::from_ptr(slot_addr).store(val, Ordering::Release);
          }
          return Ok(());
        }

        // 已有单一过渡且键名或详情不同，裂变为 TransitionArray (容量 4)
        let array = TransitionArray::allocate(allocator, map, 4)?;
        array.set(0, old_key, make_weak(old_target).cast());
        array.set(1, name, make_weak(target).cast());
        array.set_number_of_transitions(2);
        array.sort(true);

        // SAFETY: map 独占写入过渡槽位, Release 发布
        unsafe {
          let slot_addr =
            (map.address() + mem::offset_of!(Map, transitions_or_prototype_info)) as *mut usize;
          let val = array.cast::<Object>().ptr();
          AtomicUsize::from_ptr(slot_addr).store(val, Ordering::Release);
        }
        Ok(())
      }
      Encoding::FullTransitionArray => {
        let array = accessor.transitions().ok_or(Error::NotFound)?;
        let count = array.number_of_transitions();
        let (new_kind, new_attributes) = Self::get_target_details(name, target);
        let mut insertion = None;
        if let Some(existing_idx) = array.search(new_kind, name, new_attributes, &mut insertion) {
          // 相同名称和详情的过渡已存在，更新 target
          array.set_raw_target(existing_idx, make_weak(target).cast());
          return Ok(());
        }

        let insert_idx = insertion.unwrap_or(count);
        let new_nof = count + 1;
        // 原版 transitions.cc:162 CHECK_LE(new_nof, kMaxNumberOfTransitions)
        if new_nof > K_MAX_NUMBER_OF_TRANSITIONS {
          return Err(Error::InvalidCapacity(new_nof));
        }
        if count < array.capacity() {
          // 容量充足，后移并插入
          for i in (insert_idx..count).rev() {
            let k = array.get_key(i);
            let t = array.get_raw_target(i);
            array.set(i + 1, k, t);
          }
          array.set(insert_idx, name, make_weak(target).cast());
          array.set_number_of_transitions(new_nof);
          if new_nof == K_MAX_ELEMENTS_FOR_LINEAR_SEARCH + 1 {
            array.sort(true);
          }
        } else {
          // 容量不足，翻倍扩容
          let old_proto = array.get_prototype_transitions();
          let old_side = array.get_side_step_transitions();
          let new_capacity = (array.capacity() * 2).max(4);
          let new_array = TransitionArray::allocate(allocator, map, new_capacity)?;
          if let Some(proto) = old_proto {
            new_array.set_prototype_transitions(proto);
          }
          if let Some(side) = old_side {
            new_array.set_side_step_transitions(side);
          }
          for i in 0..insert_idx {
            new_array.set(i, array.get_key(i), array.get_raw_target(i));
          }
          new_array.set(insert_idx, name, make_weak(target).cast());
          for i in insert_idx..count {
            new_array.set(i + 1, array.get_key(i), array.get_raw_target(i));
          }
          new_array.set_number_of_transitions(new_nof);
          if new_nof == K_MAX_ELEMENTS_FOR_LINEAR_SEARCH + 1 {
            new_array.sort(true);
          }

          // SAFETY: map 独占写入过渡槽位, Release 发布
          unsafe {
            let slot_addr =
              (map.address() + mem::offset_of!(Map, transitions_or_prototype_info)) as *mut usize;
            let val = new_array.cast::<Object>().ptr();
            AtomicUsize::from_ptr(slot_addr).store(val, Ordering::Release);
          }
        }
        Ok(())
      }
      _ => Err(Error::UnsupportedTransition),
    }
  }

  #[inline]
  pub fn get_simple_transition_key(target: Tagged<Map>) -> Tagged<Name> {
    // SAFETY: target 指针有效，指向已分配的 Map 堆对象
    let own_descriptors = unsafe { (*target.as_ptr()).number_of_own_descriptors() as usize };
    if own_descriptors == 0 {
      return Tagged::NULL;
    }
    // SAFETY: target 指针有效
    let desc_ptr = unsafe { (*target.as_ptr()).instance_descriptors };
    if desc_ptr.is_null() {
      return Tagged::NULL;
    }
    let last = own_descriptors - 1;
    let descriptors: Tagged<DescriptorArray> = desc_ptr.cast();
    descriptors.get_key(last).unwrap_or(Tagged::NULL)
  }

  #[inline]
  pub fn get_target_details(
    key: Tagged<Name>,
    target: Tagged<Map>,
  ) -> (PropertyKind, PropertyAttributes) {
    // SAFETY: target 指针有效，指向已分配的 Map 堆对象
    let own_descriptors = unsafe { (*target.as_ptr()).number_of_own_descriptors() as usize };
    if own_descriptors == 0 {
      return (PropertyKind::Data, PropertyAttributes::NONE);
    }
    // SAFETY: target 指针有效
    let desc_ptr = unsafe { (*target.as_ptr()).instance_descriptors };
    if desc_ptr.is_null() {
      return (PropertyKind::Data, PropertyAttributes::NONE);
    }
    let last = own_descriptors - 1;
    let descriptors: Tagged<DescriptorArray> = desc_ptr.cast();
    if let Ok(desc_key) = descriptors.get_key(last)
      && desc_key == key
      && let Ok(details) = descriptors.get_details(last)
    {
      return (details.kind(), details.attributes());
    }
    (PropertyKind::Data, PropertyAttributes::NONE)
  }

  #[inline]
  pub fn is_matching_map(
    target: Tagged<Map>,
    name: Tagged<Name>,
    kind: PropertyKind,
    attributes: PropertyAttributes,
  ) -> bool {
    let (target_kind, target_attributes) = Self::get_target_details(name, target);
    target_kind == kind && target_attributes == attributes
  }
}

/// 拥有独立内存块的 TransitionArray 容器
pub struct TransitionArrayBuf {
  _storage: Vec<u64>,
  tagged: Tagged<TransitionArray>,
}

impl TransitionArrayBuf {
  pub fn new(map: Tagged<Map>, capacity: usize) -> Self {
    let bytes = TransitionArray::size_for(capacity);
    let words = bytes.div_ceil(8);
    let mut storage = vec![0u64; words];
    let base = storage.as_mut_ptr() as usize;
    let header = TransitionArray::new(map, capacity);
    // SAFETY: base 指向已分配并按 u64 对齐的有效 storage 向量首地址
    unsafe {
      ptr::write(base as *mut TransitionArray, header);
      let tagged = Tagged::from_raw_address(base);
      let zero_smi: Tagged<MaybeObject> = Smi::from_int(0).cast();
      tagged.set_raw_slot(K_PROTOTYPE_TRANSITIONS_INDEX, zero_smi);
      tagged.set_raw_slot(K_SIDE_STEP_TRANSITIONS_INDEX, zero_smi);
      tagged.set_raw_slot(K_TRANSITION_LENGTH_INDEX, zero_smi);
      Self {
        _storage: storage,
        tagged,
      }
    }
  }

  #[inline]
  pub fn as_tagged(&self) -> Tagged<TransitionArray> {
    self.tagged
  }
}

impl Deref for TransitionArrayBuf {
  type Target = Tagged<TransitionArray>;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.tagged
  }
}
