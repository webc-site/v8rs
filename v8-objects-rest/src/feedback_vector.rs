// v8-objects-rest/src/feedback_vector.rs
// 对应原版 v8/src/objects/feedback-vector.h 与 feedback-vector-inl.h

use std::{
  mem::{offset_of, size_of},
  ops::Deref,
  ptr,
  sync::atomic::{AtomicI32, AtomicU8, AtomicUsize, Ordering},
};

use v8_objects_core::{
  heap_object::HeapObject,
  map::Map,
  tagged::{Object, Tagged},
  tagged_field::TaggedSlot,
};

use crate::{
  error::{Error, Result},
  feedback_cell::FeedbackCell,
};

/// 反馈槽类别 (对应 C++ FeedbackSlotKind)
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum FeedbackSlotKind {
  Invalid = 0,
  StoreGlobalSloppy = 1,
  SetNamedSloppy = 2,
  SetKeyedSloppy = 3,
  Call = 4,
  LoadProperty = 5,
  LoadGlobalNotInsideTypeof = 6,
  LoadGlobalInsideTypeof = 7,
  LoadKeyed = 8,
  HasKeyed = 9,
  StoreGlobalStrict = 10,
  SetNamedStrict = 11,
  DefineNamedOwn = 12,
  DefineKeyedOwn = 13,
  SetKeyedStrict = 14,
  StoreInArrayLiteral = 15,
  BinaryOp = 16,
  CompareOp = 17,
  DefineKeyedOwnPropertyInLiteral = 18,
  Literal = 19,
  ForIn = 20,
  InstanceOf = 21,
  TypeOf = 22,
  CloneObject = 23,
  StringAddAndInternalize = 24,
  JumpLoop = 25,
}

impl FeedbackSlotKind {
  /// 从 u8 数值解析 FeedbackSlotKind
  #[inline]
  pub fn from_u8(value: u8) -> Result<Self> {
    match value {
      0 => Ok(Self::Invalid),
      1 => Ok(Self::StoreGlobalSloppy),
      2 => Ok(Self::SetNamedSloppy),
      3 => Ok(Self::SetKeyedSloppy),
      4 => Ok(Self::Call),
      5 => Ok(Self::LoadProperty),
      6 => Ok(Self::LoadGlobalNotInsideTypeof),
      7 => Ok(Self::LoadGlobalInsideTypeof),
      8 => Ok(Self::LoadKeyed),
      9 => Ok(Self::HasKeyed),
      10 => Ok(Self::StoreGlobalStrict),
      11 => Ok(Self::SetNamedStrict),
      12 => Ok(Self::DefineNamedOwn),
      13 => Ok(Self::DefineKeyedOwn),
      14 => Ok(Self::SetKeyedStrict),
      15 => Ok(Self::StoreInArrayLiteral),
      16 => Ok(Self::BinaryOp),
      17 => Ok(Self::CompareOp),
      18 => Ok(Self::DefineKeyedOwnPropertyInLiteral),
      19 => Ok(Self::Literal),
      20 => Ok(Self::ForIn),
      21 => Ok(Self::InstanceOf),
      22 => Ok(Self::TypeOf),
      23 => Ok(Self::CloneObject),
      24 => Ok(Self::StringAddAndInternalize),
      25 => Ok(Self::JumpLoop),
      other => Err(Error::InvalidFeedbackSlotKind(other)),
    }
  }
}

/// 反馈槽索引包装
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FeedbackSlot(pub u32);

impl FeedbackSlot {
  #[inline]
  pub const fn invalid() -> Self {
    Self(u32::MAX)
  }

  #[inline]
  pub const fn is_valid(&self) -> bool {
    self.0 != u32::MAX
  }

  #[inline]
  pub const fn to_int(&self) -> usize {
    self.0 as usize
  }
}

/// 反馈向量头部 (FeedbackVector)
///
/// 存储函数运行时 IC 监控数据、OSR (栈上替换) 状态、分层编译进度与可变长度反馈槽位。
#[repr(C)]
#[derive(Debug)]
pub struct FeedbackVector {
  /// 继承自 HeapObject (Map 8B)
  pub header: HeapObject,
  /// 反馈槽位数 (4B, offset 8)
  pub length: i32,
  /// 调用次数原子计数器 (4B, offset 12)
  pub invocation_count: AtomicI32,
  /// 64 位对齐填充 (4B, offset 16)
  pub optional_padding: u32,
  /// 稳定前调用计数 (1B, offset 20)
  pub invocation_count_before_stable: AtomicU8,
  /// OSR 状态位域 (1B, offset 21)
  pub osr_state: u8,
  /// 分层与状态标志位域 (2B, offset 22)
  pub flags: u16,
  /// 关联的 SharedFunctionInfo (8B, offset 24)
  pub shared_function_info: Tagged<Object>,
  /// 闭包反馈单元格数组引用 (8B, offset 32)
  pub closure_feedback_cell_array: Tagged<Object>,
  /// 所属外层反馈单元格 (8B, offset 40)
  pub parent_feedback_cell: Tagged<FeedbackCell>,
}

// 静态断言保证 64 位未压缩 tagged 布局精确匹配原版
const _: () = assert!(size_of::<FeedbackVector>() == 48);
const _: () = assert!(offset_of!(FeedbackVector, length) == 8);
const _: () = assert!(offset_of!(FeedbackVector, invocation_count) == 12);
const _: () = assert!(offset_of!(FeedbackVector, optional_padding) == 16);
const _: () = assert!(offset_of!(FeedbackVector, invocation_count_before_stable) == 20);
const _: () = assert!(offset_of!(FeedbackVector, osr_state) == 21);
const _: () = assert!(offset_of!(FeedbackVector, flags) == 22);
const _: () = assert!(offset_of!(FeedbackVector, shared_function_info) == 24);
const _: () = assert!(offset_of!(FeedbackVector, closure_feedback_cell_array) == 32);
const _: () = assert!(offset_of!(FeedbackVector, parent_feedback_cell) == 40);

impl FeedbackVector {
  pub const K_HEADER_SIZE: usize = size_of::<Self>();
  pub const K_SLOT_SIZE: usize = size_of::<TaggedSlot>();
  pub const K_MAX_SLOTS: usize = (i32::MAX as usize - Self::K_HEADER_SIZE) / Self::K_SLOT_SIZE;
  pub const K_MAX_OSR_URGENCY: u32 = 6;

  // OSR 位域常量
  pub const OSR_URGENCY_MASK: u8 = 0b111;
  pub const MAYBE_HAS_TURBOFAN_OSR_CODE_MASK: u8 = 1 << 3;
  pub const MAYBE_HAS_MAGLEV_OSR_CODE_MASK: u8 = 1 << 4;

  // Flags 位域常量
  pub const TIERING_IN_PROGRESS_MASK: u16 = 1 << 0;
  pub const OSR_TIERING_IN_PROGRESS_MASK: u16 = 1 << 1;
  pub const INTERRUPT_BUDGET_RESET_BY_IC_CHANGE_MASK: u16 = 1 << 2;

  /// 计算指定槽位数所需总字节数
  #[inline]
  #[must_use]
  pub const fn size_for(slot_count: usize) -> usize {
    Self::K_HEADER_SIZE + slot_count * Self::K_SLOT_SIZE
  }

  /// 构造 FeedbackVector 初始头部
  #[inline]
  pub fn new(map: Tagged<Map>, length: i32, parent_feedback_cell: Tagged<FeedbackCell>) -> Self {
    Self {
      header: HeapObject::new(map),
      length,
      invocation_count: AtomicI32::new(0),
      optional_padding: 0,
      invocation_count_before_stable: AtomicU8::new(0),
      osr_state: 0,
      flags: 0,
      shared_function_info: Tagged::NULL,
      closure_feedback_cell_array: Tagged::NULL,
      parent_feedback_cell,
    }
  }
}

/// FeedbackVector 语义扩展 trait (挂载于 Tagged&lt;FeedbackVector&gt;)
pub trait FeedbackVectorExt {
  fn length(&self) -> usize;
  fn invocation_count(&self) -> i32;
  fn set_invocation_count(&self, value: i32);
  fn clear_invocation_count(&self);
  fn invocation_count_before_stable(&self) -> u8;
  fn set_invocation_count_before_stable(&self, value: u8);
  fn osr_state(&self) -> u8;
  fn set_osr_state(&self, state: u8);
  fn osr_urgency(&self) -> u32;
  fn set_osr_urgency(&self, urgency: u32);
  fn reset_osr_urgency(&self);
  fn request_osr_at_next_opportunity(&self);
  fn maybe_has_turbofan_osr_code(&self) -> bool;
  fn set_maybe_has_turbofan_osr_code(&self, val: bool);
  fn maybe_has_maglev_osr_code(&self) -> bool;
  fn set_maybe_has_maglev_osr_code(&self, val: bool);
  fn maybe_has_optimized_osr_code(&self) -> bool;
  fn flags(&self) -> u16;
  fn set_flags(&self, flags: u16);
  fn tiering_in_progress(&self) -> bool;
  fn set_tiering_in_progress(&self, val: bool);
  fn osr_tiering_in_progress(&self) -> bool;
  fn set_osr_tiering_in_progress(&self, val: bool);
  fn interrupt_budget_reset_by_ic_change(&self) -> bool;
  fn set_interrupt_budget_reset_by_ic_change(&self, val: bool);
  fn shared_function_info(&self) -> Tagged<Object>;
  fn set_shared_function_info(&self, sfi: Tagged<Object>);
  fn closure_feedback_cell_array(&self) -> Tagged<Object>;
  fn set_closure_feedback_cell_array(&self, arr: Tagged<Object>);
  fn parent_feedback_cell(&self) -> Tagged<FeedbackCell>;
  fn set_parent_feedback_cell(&self, cell: Tagged<FeedbackCell>);

  /// 读取槽位值 (Relaxed 序)
  fn get(&self, slot: usize) -> Result<Tagged<Object>>;
  /// 写入槽位值 (Relaxed 序)
  fn set(&self, slot: usize, value: Tagged<Object>) -> Result<()>;
  /// 读取槽位值 (Acquire 序)
  fn synchronized_get(&self, slot: usize) -> Result<Tagged<Object>>;
  /// 写入槽位值 (Release 序)
  fn synchronized_set(&self, slot: usize, value: Tagged<Object>) -> Result<()>;
}

impl FeedbackVectorExt for Tagged<FeedbackVector> {
  #[inline]
  fn length(&self) -> usize {
    // SAFETY: 指针有效且长度非负
    let len = unsafe { (*self.as_ptr()).length };
    if len < 0 { 0 } else { len as usize }
  }

  #[inline]
  fn invocation_count(&self) -> i32 {
    // SAFETY: 对象内存有效
    unsafe { (*self.as_ptr()).invocation_count.load(Ordering::Relaxed) }
  }

  #[inline]
  fn set_invocation_count(&self, value: i32) {
    // SAFETY: 原子写入
    unsafe {
      (*self.as_ptr())
        .invocation_count
        .store(value, Ordering::Relaxed);
    }
  }

  #[inline]
  fn clear_invocation_count(&self) {
    self.set_invocation_count(0);
  }

  #[inline]
  fn invocation_count_before_stable(&self) -> u8 {
    // SAFETY: 对象内存有效
    unsafe {
      (*self.as_ptr())
        .invocation_count_before_stable
        .load(Ordering::Relaxed)
    }
  }

  #[inline]
  fn set_invocation_count_before_stable(&self, value: u8) {
    // SAFETY: 原子写入
    unsafe {
      (*self.as_ptr())
        .invocation_count_before_stable
        .store(value, Ordering::Relaxed);
    }
  }

  #[inline]
  fn osr_state(&self) -> u8 {
    // SAFETY: 对象内存有效
    unsafe { (*self.as_ptr()).osr_state }
  }

  #[inline]
  fn set_osr_state(&self, state: u8) {
    // SAFETY: 独占写入
    unsafe {
      (*self.as_mut_ptr()).osr_state = state;
    }
  }

  #[inline]
  fn osr_urgency(&self) -> u32 {
    (self.osr_state() & FeedbackVector::OSR_URGENCY_MASK) as u32
  }

  #[inline]
  fn set_osr_urgency(&self, urgency: u32) {
    let cur = self.osr_state() & !FeedbackVector::OSR_URGENCY_MASK;
    let next = cur | ((urgency as u8) & FeedbackVector::OSR_URGENCY_MASK);
    self.set_osr_state(next);
  }

  #[inline]
  fn reset_osr_urgency(&self) {
    self.set_osr_urgency(0);
  }

  #[inline]
  fn request_osr_at_next_opportunity(&self) {
    self.set_osr_urgency(FeedbackVector::K_MAX_OSR_URGENCY);
  }

  #[inline]
  fn maybe_has_turbofan_osr_code(&self) -> bool {
    (self.osr_state() & FeedbackVector::MAYBE_HAS_TURBOFAN_OSR_CODE_MASK) != 0
  }

  #[inline]
  fn set_maybe_has_turbofan_osr_code(&self, val: bool) {
    let mut state = self.osr_state();
    if val {
      state |= FeedbackVector::MAYBE_HAS_TURBOFAN_OSR_CODE_MASK;
    } else {
      state &= !FeedbackVector::MAYBE_HAS_TURBOFAN_OSR_CODE_MASK;
    }
    self.set_osr_state(state);
  }

  #[inline]
  fn maybe_has_maglev_osr_code(&self) -> bool {
    (self.osr_state() & FeedbackVector::MAYBE_HAS_MAGLEV_OSR_CODE_MASK) != 0
  }

  #[inline]
  fn set_maybe_has_maglev_osr_code(&self, val: bool) {
    let mut state = self.osr_state();
    if val {
      state |= FeedbackVector::MAYBE_HAS_MAGLEV_OSR_CODE_MASK;
    } else {
      state &= !FeedbackVector::MAYBE_HAS_MAGLEV_OSR_CODE_MASK;
    }
    self.set_osr_state(state);
  }

  #[inline]
  fn maybe_has_optimized_osr_code(&self) -> bool {
    self.maybe_has_maglev_osr_code() || self.maybe_has_turbofan_osr_code()
  }

  #[inline]
  fn flags(&self) -> u16 {
    // SAFETY: 对象内存有效
    unsafe { (*self.as_ptr()).flags }
  }

  #[inline]
  fn set_flags(&self, flags: u16) {
    // SAFETY: 独占写入
    unsafe {
      (*self.as_mut_ptr()).flags = flags;
    }
  }

  #[inline]
  fn tiering_in_progress(&self) -> bool {
    (self.flags() & FeedbackVector::TIERING_IN_PROGRESS_MASK) != 0
  }

  #[inline]
  fn set_tiering_in_progress(&self, val: bool) {
    let mut flags = self.flags();
    if val {
      flags |= FeedbackVector::TIERING_IN_PROGRESS_MASK;
    } else {
      flags &= !FeedbackVector::TIERING_IN_PROGRESS_MASK;
    }
    self.set_flags(flags);
  }

  #[inline]
  fn osr_tiering_in_progress(&self) -> bool {
    (self.flags() & FeedbackVector::OSR_TIERING_IN_PROGRESS_MASK) != 0
  }

  #[inline]
  fn set_osr_tiering_in_progress(&self, val: bool) {
    let mut flags = self.flags();
    if val {
      flags |= FeedbackVector::OSR_TIERING_IN_PROGRESS_MASK;
    } else {
      flags &= !FeedbackVector::OSR_TIERING_IN_PROGRESS_MASK;
    }
    self.set_flags(flags);
  }

  #[inline]
  fn interrupt_budget_reset_by_ic_change(&self) -> bool {
    (self.flags() & FeedbackVector::INTERRUPT_BUDGET_RESET_BY_IC_CHANGE_MASK) != 0
  }

  #[inline]
  fn set_interrupt_budget_reset_by_ic_change(&self, val: bool) {
    let mut flags = self.flags();
    if val {
      flags |= FeedbackVector::INTERRUPT_BUDGET_RESET_BY_IC_CHANGE_MASK;
    } else {
      flags &= !FeedbackVector::INTERRUPT_BUDGET_RESET_BY_IC_CHANGE_MASK;
    }
    self.set_flags(flags);
  }

  #[inline]
  fn shared_function_info(&self) -> Tagged<Object> {
    // SAFETY: 对象内存有效
    unsafe { (*self.as_ptr()).shared_function_info }
  }

  #[inline]
  fn set_shared_function_info(&self, sfi: Tagged<Object>) {
    // SAFETY: 独占写入
    unsafe {
      (*self.as_mut_ptr()).shared_function_info = sfi;
    }
  }

  #[inline]
  fn closure_feedback_cell_array(&self) -> Tagged<Object> {
    // SAFETY: 对象内存有效
    unsafe { (*self.as_ptr()).closure_feedback_cell_array }
  }

  #[inline]
  fn set_closure_feedback_cell_array(&self, arr: Tagged<Object>) {
    // SAFETY: 独占写入
    unsafe {
      (*self.as_mut_ptr()).closure_feedback_cell_array = arr;
    }
  }

  #[inline]
  fn parent_feedback_cell(&self) -> Tagged<FeedbackCell> {
    // SAFETY: 对象内存有效
    unsafe { (*self.as_ptr()).parent_feedback_cell }
  }

  #[inline]
  fn set_parent_feedback_cell(&self, cell: Tagged<FeedbackCell>) {
    // SAFETY: 独占写入
    unsafe {
      (*self.as_mut_ptr()).parent_feedback_cell = cell;
    }
  }

  #[inline]
  fn get(&self, slot: usize) -> Result<Tagged<Object>> {
    let len = self.length();
    if slot >= len {
      return Err(Error::OutOfBounds {
        index: slot,
        length: len,
      });
    }
    let slot_addr =
      self.address() + FeedbackVector::K_HEADER_SIZE + slot * FeedbackVector::K_SLOT_SIZE;
    // SAFETY: slot_addr 位于有效槽位边界内
    let ptr_val = unsafe { AtomicUsize::from_ptr(slot_addr as *mut usize).load(Ordering::Relaxed) };
    Ok(Tagged::new(ptr_val))
  }

  #[inline]
  fn set(&self, slot: usize, value: Tagged<Object>) -> Result<()> {
    let len = self.length();
    if slot >= len {
      return Err(Error::OutOfBounds {
        index: slot,
        length: len,
      });
    }
    let slot_addr =
      self.address() + FeedbackVector::K_HEADER_SIZE + slot * FeedbackVector::K_SLOT_SIZE;
    // SAFETY: slot_addr 位于有效槽位边界内
    unsafe {
      AtomicUsize::from_ptr(slot_addr as *mut usize).store(value.ptr(), Ordering::Relaxed);
    }
    Ok(())
  }

  #[inline]
  fn synchronized_get(&self, slot: usize) -> Result<Tagged<Object>> {
    let len = self.length();
    if slot >= len {
      return Err(Error::OutOfBounds {
        index: slot,
        length: len,
      });
    }
    let slot_addr =
      self.address() + FeedbackVector::K_HEADER_SIZE + slot * FeedbackVector::K_SLOT_SIZE;
    // SAFETY: slot_addr 位于有效槽位边界内
    let ptr_val = unsafe { AtomicUsize::from_ptr(slot_addr as *mut usize).load(Ordering::Acquire) };
    Ok(Tagged::new(ptr_val))
  }

  #[inline]
  fn synchronized_set(&self, slot: usize, value: Tagged<Object>) -> Result<()> {
    let len = self.length();
    if slot >= len {
      return Err(Error::OutOfBounds {
        index: slot,
        length: len,
      });
    }
    let slot_addr =
      self.address() + FeedbackVector::K_HEADER_SIZE + slot * FeedbackVector::K_SLOT_SIZE;
    // SAFETY: slot_addr 位于有效槽位边界内
    unsafe {
      AtomicUsize::from_ptr(slot_addr as *mut usize).store(value.ptr(), Ordering::Release);
    }
    Ok(())
  }
}

/// 拥有独立内存块的 FeedbackVector 缓冲区容器（方便脱离堆环境的构造与测试）
pub struct FeedbackVectorBuf {
  _storage: Vec<u64>,
  tagged: Tagged<FeedbackVector>,
}

impl FeedbackVectorBuf {
  /// 分配并初始化包含 `slot_count` 个反馈槽的 FeedbackVector
  pub fn new(
    map: Tagged<Map>,
    slot_count: usize,
    parent_feedback_cell: Tagged<FeedbackCell>,
  ) -> Result<Self> {
    if slot_count > FeedbackVector::K_MAX_SLOTS {
      return Err(Error::CapacityExceeded {
        requested: slot_count,
        capacity: FeedbackVector::K_MAX_SLOTS,
      });
    }
    let bytes = FeedbackVector::size_for(slot_count);
    let words = bytes.div_ceil(8);
    let mut storage = vec![0u64; words];
    let base = storage.as_mut_ptr() as usize;
    let header = FeedbackVector::new(map, slot_count as i32, parent_feedback_cell);
    // SAFETY: base 指向已分配的 storage 向量首地址，满足 u64 对齐与尺寸需求
    unsafe {
      ptr::write(base as *mut FeedbackVector, header);
    }
    Ok(Self {
      _storage: storage,
      // SAFETY: base 指向有效的 FeedbackVector 内存
      tagged: unsafe { Tagged::from_raw_address(base) },
    })
  }

  /// 获取指向 Tagged&lt;FeedbackVector&gt; 的句柄
  #[inline]
  pub const fn tagged(&self) -> Tagged<FeedbackVector> {
    self.tagged
  }
}

impl Deref for FeedbackVectorBuf {
  type Target = Tagged<FeedbackVector>;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.tagged
  }
}
