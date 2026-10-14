// v8-objects-js-async/src/js_promise.rs
// 对应原版 v8/src/objects/js-promise.h + js-promise-inl.h + js-promise.tq

//! JSPromise: JavaScript Promise 对象实现，管理状态流转 (Pending/Fulfilled/Rejected)、
//! Reactions 链表或结算结果、Flags 位域（含 async_task_id、has_handler 等）。

use std::{
  mem,
  ops::{Deref, DerefMut},
  ptr,
  sync::atomic::{AtomicUsize, Ordering},
};

use v8_objects_arrays::fixed_array_base::FixedArrayBase;
use v8_objects_core::{
  map::Map,
  smi::Smi,
  tagged::{Object, Tagged},
};
use v8_objects_js_core::{
  field_index::FieldIndex,
  js_object::{
    JSObject, JSObjectExt, K_HEADER_SIZE as K_JS_OBJECT_HEADER_SIZE, K_MAX_INSTANCE_SIZE,
  },
  js_receiver::{JSReceiver, JSReceiverExt},
};
use v8_objects_maps::PropertyArray;

use crate::error::{Error, Result};

// ── 1. PromiseState 枚举 ───────────────────────────────────────────────────

/// Promise 状态枚举 (对应 v8::Promise::PromiseState)
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PromiseState {
  /// 等待状态
  Pending = 0,
  /// 已达成状态
  Fulfilled = 1,
  /// 已拒绝状态
  Rejected = 2,
}

impl TryFrom<u32> for PromiseState {
  type Error = Error;

  #[inline]
  fn try_from(v: u32) -> Result<Self> {
    match v {
      0 => Ok(Self::Pending),
      1 => Ok(Self::Fulfilled),
      2 => Ok(Self::Rejected),
      _ => Err(Error::InvalidPromiseState(v)),
    }
  }
}

impl PromiseState {
  /// 获取状态字符串表示 (对应 JSPromise::Status)
  #[inline]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Pending => "pending",
      Self::Fulfilled => "fulfilled",
      Self::Rejected => "rejected",
    }
  }
}

// ── 2. Flags 位域常量 ──────────────────────────────────────────────────────

pub const K_STATUS_SHIFT: u32 = 0;
pub const K_STATUS_BITS: u32 = 2;
pub const K_STATUS_MASK: u32 = (1 << K_STATUS_BITS) - 1; // 0x3

pub const K_IS_NATIVE_RESOLVER_INVOKED_SHIFT: u32 = 2;
pub const K_IS_NATIVE_RESOLVER_INVOKED_BITS: u32 = 1;
pub const K_IS_NATIVE_RESOLVER_INVOKED_MASK: u32 = 1 << K_IS_NATIVE_RESOLVER_INVOKED_SHIFT; // 0x4

pub const K_HAS_HANDLER_SHIFT: u32 = 3;
pub const K_HAS_HANDLER_BITS: u32 = 1;
pub const K_HAS_HANDLER_MASK: u32 = 1 << K_HAS_HANDLER_SHIFT; // 0x8

pub const K_IS_SILENT_SHIFT: u32 = 4;
pub const K_IS_SILENT_BITS: u32 = 1;
pub const K_IS_SILENT_MASK: u32 = 1 << K_IS_SILENT_SHIFT; // 0x10

pub const K_ASYNC_TASK_ID_SHIFT: u32 = 5;
pub const K_ASYNC_TASK_ID_BITS: u32 = 26;
pub const K_MAX_ASYNC_TASK_ID: u32 = (1 << K_ASYNC_TASK_ID_BITS) - 1; // 0x3FFFFFF = 67,108,863
pub const K_ASYNC_TASK_ID_MASK: u32 = K_MAX_ASYNC_TASK_ID << K_ASYNC_TASK_ID_SHIFT;

pub const K_INVALID_ASYNC_TASK_ID: u32 = 0;

/// 计算下一个合法的 async task id (静默回绕，跳过 0)
#[inline]
pub fn get_next_async_task_id(mut id: u32) -> u32 {
  loop {
    id = (id.wrapping_add(1)) & K_MAX_ASYNC_TASK_ID;
    if id != K_INVALID_ASYNC_TASK_ID {
      return id;
    }
  }
}

// ── 3. 内存槽位与大小常量 ──────────────────────────────────────────────────

/// reactions_or_result 字段偏移量
pub const K_REACTIONS_OR_RESULT_OFFSET: usize = K_JS_OBJECT_HEADER_SIZE;

/// flags 字段偏移量
pub const K_FLAGS_OFFSET: usize = K_REACTIONS_OR_RESULT_OFFSET + mem::size_of::<usize>();

/// JSPromise 头部大小 (5 words)
pub const K_HEADER_SIZE: usize = mem::size_of::<JSPromise>();

/// JSPromise 最大对象内属性数量
pub const K_MAX_IN_OBJECT_PROPERTIES: usize =
  (K_MAX_INSTANCE_SIZE - K_HEADER_SIZE) / mem::size_of::<usize>();

// 编译期断言
const _: () = assert!(K_REACTIONS_OR_RESULT_OFFSET == 3 * mem::size_of::<usize>());
const _: () = assert!(K_FLAGS_OFFSET == 4 * mem::size_of::<usize>());
const _: () = assert!(K_HEADER_SIZE == 5 * mem::size_of::<usize>());
const _: () = assert!(K_MAX_IN_OBJECT_PROPERTIES == 250);

// ── 4. JSPromise 结构体 ────────────────────────────────────────────────────

/// JSPromise 结构体
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct JSPromise {
  /// JSObject 基类部分 (包含 map, properties_or_hash, elements)
  pub object: JSObject,
  /// 反应链表（Pending 状态时）或结算结果（Fulfilled/Rejected 状态时）
  pub reactions_or_result: Tagged<Object>,
  /// 位域标志（Smi 存储）
  pub flags: Tagged<Smi>,
}

impl Deref for JSPromise {
  type Target = JSObject;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.object
  }
}

impl DerefMut for JSPromise {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.object
  }
}

impl JSPromise {
  pub const K_HEADER_SIZE: usize = K_HEADER_SIZE;
  pub const K_REACTIONS_OR_RESULT_OFFSET: usize = K_REACTIONS_OR_RESULT_OFFSET;
  pub const K_FLAGS_OFFSET: usize = K_FLAGS_OFFSET;
  pub const K_MAX_IN_OBJECT_PROPERTIES: usize = K_MAX_IN_OBJECT_PROPERTIES;

  /// 创建新的 JSPromise 实例
  #[inline]
  pub const fn new(
    map: Tagged<Map>,
    properties_or_hash: Tagged<Object>,
    elements: Tagged<FixedArrayBase>,
    reactions_or_result: Tagged<Object>,
    flags: i32,
  ) -> Self {
    Self {
      object: JSObject::new(map, properties_or_hash, elements),
      reactions_or_result,
      flags: Smi::from_31bit_pattern(flags),
    }
  }

  /// reactions_or_result 原子槽位指针
  #[inline]
  fn atomic_reactions_or_result(&self) -> &AtomicUsize {
    // SAFETY: Tagged<Object> 与 AtomicUsize 具有相同的字长和对齐
    unsafe { &*(&self.reactions_or_result as *const Tagged<Object> as *const AtomicUsize) }
  }

  /// flags 原子槽位指针
  #[inline]
  fn atomic_flags(&self) -> &AtomicUsize {
    // SAFETY: Tagged<Smi> 与 AtomicUsize 具有相同的字长和对齐
    unsafe { &*(&self.flags as *const Tagged<Smi> as *const AtomicUsize) }
  }
}

// ── 5. JSPromiseExt 操作扩展 Trait ─────────────────────────────────────────

/// JSPromise 操作扩展 trait
pub trait JSPromiseExt {
  /// 读取 reactions_or_result 原始槽位
  fn reactions_or_result(&self) -> Tagged<Object>;

  /// 写入 reactions_or_result 原始槽位
  fn set_reactions_or_result(&mut self, value: Tagged<Object>);

  /// 读取原始 flags 数值
  fn flags(&self) -> i32;

  /// 写入 flags 数值
  fn set_flags(&mut self, flags: i32);

  /// 读取 Promise 状态
  #[inline]
  fn status(&self) -> PromiseState {
    let raw = ((self.flags() as u32) & K_STATUS_MASK) >> K_STATUS_SHIFT;
    match raw {
      1 => PromiseState::Fulfilled,
      2 => PromiseState::Rejected,
      _ => PromiseState::Pending,
    }
  }

  /// 更新 Promise 状态（仅允许从 Pending 转为 Fulfilled 或 Rejected）
  fn set_status(&mut self, status: PromiseState) -> Result<()> {
    if self.status() != PromiseState::Pending {
      return Err(Error::PromiseNotPending);
    }
    if status == PromiseState::Pending {
      return Err(Error::Invariant("不能将已结算的 Promise 重置为 Pending"));
    }
    let curr = (self.flags() as u32) & 0x7FFF_FFFF;
    let updated = (curr & !K_STATUS_MASK) | ((status as u32) << K_STATUS_SHIFT);
    self.set_flags(updated as i32);
    Ok(())
  }

  /// 获取结算结果（校验必须处于非 Pending 状态）
  #[inline]
  fn result(&self) -> Result<Tagged<Object>> {
    if self.status() == PromiseState::Pending {
      return Err(Error::PromiseStillPending);
    }
    Ok(self.reactions_or_result())
  }

  /// 获取反应链表（校验必须处于 Pending 状态）
  #[inline]
  fn reactions(&self) -> Result<Tagged<Object>> {
    if self.status() != PromiseState::Pending {
      return Err(Error::PromiseNotPending);
    }
    Ok(self.reactions_or_result())
  }

  /// 是否登记了拒绝回调处理函数 (has_handler)
  #[inline]
  fn has_handler(&self) -> bool {
    ((self.flags() as u32) & K_HAS_HANDLER_MASK) != 0
  }

  /// 设置 has_handler
  #[inline]
  fn set_has_handler(&mut self, val: bool) {
    let curr = (self.flags() as u32) & 0x7FFF_FFFF;
    let updated = if val {
      curr | K_HAS_HANDLER_MASK
    } else {
      curr & !K_HAS_HANDLER_MASK
    };
    self.set_flags(updated as i32);
  }

  /// 是否静默（拒绝时不触发调试器暂停）
  #[inline]
  fn is_silent(&self) -> bool {
    ((self.flags() as u32) & K_IS_SILENT_MASK) != 0
  }

  /// 设置 is_silent
  #[inline]
  fn set_is_silent(&mut self, val: bool) {
    let curr = (self.flags() as u32) & 0x7FFF_FFFF;
    let updated = if val {
      curr | K_IS_SILENT_MASK
    } else {
      curr & !K_IS_SILENT_MASK
    };
    self.set_flags(updated as i32);
  }

  /// 是否已调用原生解析器 (is_native_resolver_invoked)
  #[inline]
  fn is_native_resolver_invoked(&self) -> bool {
    ((self.flags() as u32) & K_IS_NATIVE_RESOLVER_INVOKED_MASK) != 0
  }

  /// 设置 is_native_resolver_invoked
  #[inline]
  fn set_is_native_resolver_invoked(&mut self, val: bool) {
    let curr = (self.flags() as u32) & 0x7FFF_FFFF;
    let updated = if val {
      curr | K_IS_NATIVE_RESOLVER_INVOKED_MASK
    } else {
      curr & !K_IS_NATIVE_RESOLVER_INVOKED_MASK
    };
    self.set_flags(updated as i32);
  }

  /// 是否具备异步任务 ID
  #[inline]
  fn has_async_task_id(&self) -> bool {
    self.async_task_id() != K_INVALID_ASYNC_TASK_ID
  }

  /// 获取异步任务 ID
  #[inline]
  fn async_task_id(&self) -> u32 {
    ((self.flags() as u32) & K_ASYNC_TASK_ID_MASK) >> K_ASYNC_TASK_ID_SHIFT
  }

  /// 设置异步任务 ID
  #[inline]
  fn set_async_task_id(&mut self, id: u32) {
    let clean = id & K_MAX_ASYNC_TASK_ID;
    let curr = (self.flags() as u32) & 0x7FFF_FFFF;
    let updated = (curr & !K_ASYNC_TASK_ID_MASK) | (clean << K_ASYNC_TASK_ID_SHIFT);
    self.set_flags(updated as i32);
  }

  /// 达成 Promise (Fulfill)
  #[inline]
  fn fulfill(&mut self, value: Tagged<Object>) -> Result<()> {
    self.set_status(PromiseState::Fulfilled)?;
    self.set_reactions_or_result(value);
    Ok(())
  }

  /// 拒绝 Promise (Reject)
  #[inline]
  fn reject(&mut self, reason: Tagged<Object>) -> Result<()> {
    self.set_status(PromiseState::Rejected)?;
    self.set_reactions_or_result(reason);
    Ok(())
  }
}

// ── 6. JSPromise Trait 实现 ────────────────────────────────────────────────

impl JSReceiverExt for JSPromise {
  #[inline]
  fn map(&self) -> Tagged<Map> {
    self.object.map()
  }

  #[inline]
  fn set_map(&mut self, map: Tagged<Map>) {
    self.object.set_map(map);
  }

  #[inline]
  fn raw_properties_or_hash(&self) -> Tagged<Object> {
    self.object.raw_properties_or_hash()
  }

  #[inline]
  fn set_raw_properties_or_hash(&mut self, value: Tagged<Object>) {
    self.object.set_raw_properties_or_hash(value);
  }

  #[inline]
  fn has_fast_properties(&self) -> bool {
    self.object.has_fast_properties()
  }

  #[inline]
  fn property_array(&self) -> Option<Tagged<PropertyArray>> {
    self.object.property_array()
  }

  #[inline]
  fn get_prototype(&self) -> Tagged<Object> {
    self.object.get_prototype()
  }

  #[inline]
  fn has_in_prototype_chain(&self, proto: Tagged<Object>) -> bool {
    self.object.has_in_prototype_chain(proto)
  }

  #[inline]
  fn get_identity_hash(&self) -> Option<i32> {
    self.object.get_identity_hash()
  }

  #[inline]
  fn set_identity_hash(&mut self, hash: i32) -> v8_objects_js_core::Result<()> {
    self.object.set_identity_hash(hash)
  }
}

impl JSObjectExt for JSPromise {
  #[inline]
  fn elements(&self) -> Tagged<FixedArrayBase> {
    self.object.elements()
  }

  #[inline]
  fn set_elements(&mut self, value: Tagged<FixedArrayBase>) {
    self.object.set_elements(value);
  }

  #[inline]
  fn raw_fast_property_at(&self, index: FieldIndex) -> v8_objects_js_core::Result<Tagged<Object>> {
    self.object.raw_fast_property_at(index)
  }

  #[inline]
  fn fast_property_at_put(
    &mut self,
    index: FieldIndex,
    value: Tagged<Object>,
  ) -> v8_objects_js_core::Result<()> {
    self.object.fast_property_at_put(index, value)
  }
}

impl JSPromiseExt for JSPromise {
  #[inline]
  fn reactions_or_result(&self) -> Tagged<Object> {
    Tagged::new(self.atomic_reactions_or_result().load(Ordering::Relaxed))
  }

  #[inline]
  fn set_reactions_or_result(&mut self, value: Tagged<Object>) {
    self
      .atomic_reactions_or_result()
      .store(value.ptr(), Ordering::Relaxed);
  }

  #[inline]
  fn flags(&self) -> i32 {
    let raw = self.atomic_flags().load(Ordering::Relaxed);
    Smi::value(Tagged::<Smi>::new(raw))
  }

  #[inline]
  fn set_flags(&mut self, flags: i32) {
    let smi = Smi::from_31bit_pattern(flags);
    self.atomic_flags().store(smi.ptr(), Ordering::Relaxed);
  }
}

// ── 7. Tagged<JSPromise> Trait 实现 ────────────────────────────────────────

/// Tagged<JSPromise> 专属扩展 trait
pub trait TaggedJSPromiseExt {
  /// 转换为 Tagged<JSObject>
  fn as_object(&self) -> Tagged<JSObject>;

  /// 转换为 Tagged<JSReceiver>
  fn as_receiver(&self) -> Tagged<JSReceiver>;
}

impl TaggedJSPromiseExt for Tagged<JSPromise> {
  #[inline]
  fn as_object(&self) -> Tagged<JSObject> {
    Tagged::new(self.ptr())
  }

  #[inline]
  fn as_receiver(&self) -> Tagged<JSReceiver> {
    Tagged::new(self.ptr())
  }
}

impl JSPromiseExt for Tagged<JSPromise> {
  #[inline]
  fn reactions_or_result(&self) -> Tagged<Object> {
    // SAFETY: Tagged 包装合法 JSPromise 指针
    unsafe { (*self.as_ptr()).reactions_or_result() }
  }

  #[inline]
  fn set_reactions_or_result(&mut self, value: Tagged<Object>) {
    // SAFETY: Tagged 包装合法可变 JSPromise 指针
    unsafe { (*self.as_mut_ptr()).set_reactions_or_result(value) }
  }

  #[inline]
  fn flags(&self) -> i32 {
    // SAFETY: Tagged 包装合法 JSPromise 指针
    unsafe { (*self.as_ptr()).flags() }
  }

  #[inline]
  fn set_flags(&mut self, flags: i32) {
    // SAFETY: Tagged 包装合法可变 JSPromise 指针
    unsafe { (*self.as_mut_ptr()).set_flags(flags) }
  }
}

// ── 8. JSPromiseBuf 连续内存容器 ───────────────────────────────────────────

/// 拥有独立连续内存的 JSPromise 实例容器
pub struct JSPromiseBuf {
  _storage: Vec<u64>,
  tagged: Tagged<JSPromise>,
  in_object_count: usize,
}

impl JSPromiseBuf {
  /// 创建新的 JSPromiseBuf 实例
  pub fn new(
    map: Tagged<Map>,
    properties_or_hash: Tagged<Object>,
    elements: Tagged<FixedArrayBase>,
    reactions_or_result: Tagged<Object>,
    flags: i32,
    in_object_count: usize,
  ) -> Result<Self> {
    if in_object_count > K_MAX_IN_OBJECT_PROPERTIES {
      return Err(Error::InvalidCapacity(in_object_count));
    }
    let total_bytes = K_HEADER_SIZE + in_object_count * mem::size_of::<usize>();
    let words = total_bytes.div_ceil(8);
    let mut storage = vec![0u64; words];
    let base = storage.as_mut_ptr() as usize;

    let header = JSPromise::new(
      map,
      properties_or_hash,
      elements,
      reactions_or_result,
      flags,
    );
    // SAFETY: base 指向足额分配且 8 字节对齐的连续堆内存首地址
    unsafe {
      ptr::write(base as *mut JSPromise, header);
    }

    Ok(Self {
      _storage: storage,
      // SAFETY: base 包装为 Tagged<JSPromise>
      tagged: unsafe { Tagged::from_raw_address(base) },
      in_object_count,
    })
  }

  /// 获取对象内属性个数
  #[inline]
  pub const fn in_object_count(&self) -> usize {
    self.in_object_count
  }

  /// 获取 Tagged 引用
  #[inline]
  pub fn as_tagged(&self) -> Tagged<JSPromise> {
    self.tagged
  }
}

impl Deref for JSPromiseBuf {
  type Target = Tagged<JSPromise>;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.tagged
  }
}

impl DerefMut for JSPromiseBuf {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.tagged
  }
}
