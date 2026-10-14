// v8-objects-js-async/src/js_generator.rs
// 对应原版 v8/src/objects/js-generator.h + js-generator-inl.h + js-generator.tq

//! JSGeneratorObject 族: JavaScript 生成器与异步生成器核心对象实现。
//! 管理挂起与恢复 continuation 状态、ResumeMode、操作数寄存器与异步执行队列。

use std::{
  mem,
  ops::{Deref, DerefMut},
  ptr,
  sync::atomic::{AtomicUsize, Ordering},
};

use v8_objects_arrays::{fixed_array::FixedArray, fixed_array_base::FixedArrayBase};
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
use v8_objects_js_func::{context::Context, js_function::JSFunction};
use v8_objects_maps::PropertyArray;

use crate::{
  error::{Error, Result},
  js_promise::JSPromise,
};

// ── 1. ResumeMode 枚举 ─────────────────────────────────────────────────────

/// 生成器恢复模式 (对应 JSGeneratorObject::ResumeMode)
#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ResumeMode {
  /// 正常继续执行下一步 (kNext = 0)
  Next = 0,
  /// 提前退出返回 (kReturn = 1)
  Return = 1,
  /// 抛出异常 (kThrow = 2)
  Throw = 2,
  /// 重新抛出异常 (kRethrow = 3)
  Rethrow = 3,
}

impl TryFrom<i32> for ResumeMode {
  type Error = Error;

  #[inline]
  fn try_from(v: i32) -> Result<Self> {
    match v {
      0 => Ok(Self::Next),
      1 => Ok(Self::Return),
      2 => Ok(Self::Throw),
      3 => Ok(Self::Rethrow),
      _ => Err(Error::InvalidResumeMode(v)),
    }
  }
}

impl ResumeMode {
  /// 获取模式字符串表示
  #[inline]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Next => "next",
      Self::Return => "return",
      Self::Throw => "throw",
      Self::Rethrow => "rethrow",
    }
  }
}

// ── 2. Continuation 状态哨兵常量 ──────────────────────────────────────────

/// 生成器正在执行中哨兵值 (kGeneratorExecuting = -2)
pub const K_GENERATOR_EXECUTING: i32 = -2;

/// 生成器已关闭哨兵值 (kGeneratorClosed = -1)
pub const K_GENERATOR_CLOSED: i32 = -1;

// ── 3. 槽位偏移与大小常量 ──────────────────────────────────────────────────

// JSGeneratorObject 槽位 (紧随 JSObject 之后)
pub const K_FUNCTION_OFFSET: usize = K_JS_OBJECT_HEADER_SIZE;
pub const K_CONTEXT_OFFSET: usize = K_FUNCTION_OFFSET + mem::size_of::<usize>();
pub const K_RECEIVER_OFFSET: usize = K_CONTEXT_OFFSET + mem::size_of::<usize>();
pub const K_INPUT_OR_DEBUG_POS_OFFSET: usize = K_RECEIVER_OFFSET + mem::size_of::<usize>();
pub const K_RESUME_MODE_OFFSET: usize = K_INPUT_OR_DEBUG_POS_OFFSET + mem::size_of::<usize>();
pub const K_CONTINUATION_OFFSET: usize = K_RESUME_MODE_OFFSET + mem::size_of::<usize>();
pub const K_PARAMETERS_AND_REGISTERS_OFFSET: usize =
  K_CONTINUATION_OFFSET + mem::size_of::<usize>();
pub const K_YIELDED_VALUE_OFFSET: usize =
  K_PARAMETERS_AND_REGISTERS_OFFSET + mem::size_of::<usize>();
pub const K_GENERATOR_HEADER_SIZE: usize = mem::size_of::<JSGeneratorObject>();

pub const K_MAX_GENERATOR_IN_OBJECT_PROPERTIES: usize =
  (K_MAX_INSTANCE_SIZE - K_GENERATOR_HEADER_SIZE) / mem::size_of::<usize>();

// JSAsyncGeneratorObject 槽位 (紧随 JSGeneratorObject 之后)
pub const K_ASYNC_GEN_QUEUE_OFFSET: usize = K_GENERATOR_HEADER_SIZE;
pub const K_ASYNC_GEN_IS_AWAITING_OFFSET: usize =
  K_ASYNC_GEN_QUEUE_OFFSET + mem::size_of::<usize>();
pub const K_ASYNC_GENERATOR_HEADER_SIZE: usize = mem::size_of::<JSAsyncGeneratorObject>();

pub const K_MAX_ASYNC_GENERATOR_IN_OBJECT_PROPERTIES: usize =
  (K_MAX_INSTANCE_SIZE - K_ASYNC_GENERATOR_HEADER_SIZE) / mem::size_of::<usize>();

// JSAsyncFunctionObject 槽位 (紧随 JSGeneratorObject 之后)
pub const K_ASYNC_FUNC_PROMISE_OFFSET: usize = K_GENERATOR_HEADER_SIZE;
pub const K_ASYNC_FUNC_AWAIT_RESOLVE_CLOSURE_OFFSET: usize =
  K_ASYNC_FUNC_PROMISE_OFFSET + mem::size_of::<usize>();
pub const K_ASYNC_FUNC_AWAIT_REJECT_CLOSURE_OFFSET: usize =
  K_ASYNC_FUNC_AWAIT_RESOLVE_CLOSURE_OFFSET + mem::size_of::<usize>();
pub const K_ASYNC_FUNCTION_HEADER_SIZE: usize = mem::size_of::<JSAsyncFunctionObject>();

pub const K_MAX_ASYNC_FUNCTION_IN_OBJECT_PROPERTIES: usize =
  (K_MAX_INSTANCE_SIZE - K_ASYNC_FUNCTION_HEADER_SIZE) / mem::size_of::<usize>();

// AsyncGeneratorRequest 槽位 (继承 Struct -> HeapObject 1 word)
pub const K_REQ_NEXT_OFFSET: usize = mem::size_of::<usize>();
pub const K_REQ_RESUME_MODE_OFFSET: usize = K_REQ_NEXT_OFFSET + mem::size_of::<usize>();
pub const K_REQ_VALUE_OFFSET: usize = K_REQ_RESUME_MODE_OFFSET + mem::size_of::<usize>();
pub const K_REQ_PROMISE_OFFSET: usize = K_REQ_VALUE_OFFSET + mem::size_of::<usize>();
pub const K_ASYNC_GENERATOR_REQUEST_HEADER_SIZE: usize = mem::size_of::<AsyncGeneratorRequest>();

// 编译期断言
const _: () = assert!(K_FUNCTION_OFFSET == 3 * mem::size_of::<usize>());
const _: () = assert!(K_CONTEXT_OFFSET == 4 * mem::size_of::<usize>());
const _: () = assert!(K_RECEIVER_OFFSET == 5 * mem::size_of::<usize>());
const _: () = assert!(K_INPUT_OR_DEBUG_POS_OFFSET == 6 * mem::size_of::<usize>());
const _: () = assert!(K_RESUME_MODE_OFFSET == 7 * mem::size_of::<usize>());
const _: () = assert!(K_CONTINUATION_OFFSET == 8 * mem::size_of::<usize>());
const _: () = assert!(K_PARAMETERS_AND_REGISTERS_OFFSET == 9 * mem::size_of::<usize>());
const _: () = assert!(K_YIELDED_VALUE_OFFSET == 10 * mem::size_of::<usize>());
const _: () = assert!(K_GENERATOR_HEADER_SIZE == 11 * mem::size_of::<usize>());
const _: () = assert!(K_MAX_GENERATOR_IN_OBJECT_PROPERTIES == 244);

const _: () = assert!(K_ASYNC_GEN_QUEUE_OFFSET == 11 * mem::size_of::<usize>());
const _: () = assert!(K_ASYNC_GEN_IS_AWAITING_OFFSET == 12 * mem::size_of::<usize>());
const _: () = assert!(K_ASYNC_GENERATOR_HEADER_SIZE == 13 * mem::size_of::<usize>());
const _: () = assert!(K_MAX_ASYNC_GENERATOR_IN_OBJECT_PROPERTIES == 242);

const _: () = assert!(K_ASYNC_FUNC_PROMISE_OFFSET == 11 * mem::size_of::<usize>());
const _: () = assert!(K_ASYNC_FUNC_AWAIT_RESOLVE_CLOSURE_OFFSET == 12 * mem::size_of::<usize>());
const _: () = assert!(K_ASYNC_FUNC_AWAIT_REJECT_CLOSURE_OFFSET == 13 * mem::size_of::<usize>());
const _: () = assert!(K_ASYNC_FUNCTION_HEADER_SIZE == 14 * mem::size_of::<usize>());
const _: () = assert!(K_MAX_ASYNC_FUNCTION_IN_OBJECT_PROPERTIES == 241);

const _: () = assert!(K_REQ_NEXT_OFFSET == mem::size_of::<usize>());
const _: () = assert!(K_REQ_RESUME_MODE_OFFSET == 2 * mem::size_of::<usize>());
const _: () = assert!(K_REQ_VALUE_OFFSET == 3 * mem::size_of::<usize>());
const _: () = assert!(K_REQ_PROMISE_OFFSET == 4 * mem::size_of::<usize>());
const _: () = assert!(K_ASYNC_GENERATOR_REQUEST_HEADER_SIZE == 5 * mem::size_of::<usize>());

// ── 4. GeneratorFields 状态配置结构体 ──────────────────────────────────────

/// 生成器可变运行时状态与上下文配置
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct GeneratorFields {
  /// 绑定的生成器函数
  pub function: Tagged<JSFunction>,
  /// 绑定的上下文环境
  pub context: Tagged<Context>,
  /// 生成器接收者 (this)
  pub receiver: Tagged<Object>,
  /// 最近输入值或挂起点字节码偏移 (input_or_debug_pos)
  pub input_or_debug_pos: Tagged<Object>,
  /// 恢复模式 (Smi 编码)
  pub resume_mode: i32,
  /// 挂起状态/接续计数 (continuation: >=0 挂起, -1 关闭, -2 执行中)
  pub continuation: i32,
  /// 解释器局部变量与参数暂存数组
  pub parameters_and_registers: Tagged<FixedArray>,
  /// 最近一次产出的值 (yielded_value)
  pub yielded_value: Tagged<Object>,
}

impl Default for GeneratorFields {
  #[inline]
  fn default() -> Self {
    Self {
      function: Tagged::NULL,
      context: Tagged::NULL,
      receiver: Tagged::NULL,
      input_or_debug_pos: Tagged::NULL,
      resume_mode: ResumeMode::Next as i32,
      continuation: 0,
      parameters_and_registers: Tagged::NULL,
      yielded_value: Tagged::NULL,
    }
  }
}

// ── 5. JSGeneratorObject 结构体 ────────────────────────────────────────────

/// JSGeneratorObject 结构体
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct JSGeneratorObject {
  /// JSObject 基类部分
  pub object: JSObject,
  /// 绑定的生成器函数
  pub function: Tagged<JSFunction>,
  /// 绑定的上下文环境
  pub context: Tagged<Context>,
  /// 生成器接收者 (this)
  pub receiver: Tagged<Object>,
  /// 最近输入值或挂起点字节码偏移 (input_or_debug_pos)
  pub input_or_debug_pos: Tagged<Object>,
  /// 恢复模式 (Smi 编码)
  pub resume_mode: Tagged<Smi>,
  /// 挂起状态/接续计数 (continuation: >=0 挂起, -1 关闭, -2 执行中)
  pub continuation: Tagged<Smi>,
  /// 解释器局部变量与参数暂存数组
  pub parameters_and_registers: Tagged<FixedArray>,
  /// 最近一次产出的值 (yielded_value)
  pub yielded_value: Tagged<Object>,
}

impl Deref for JSGeneratorObject {
  type Target = JSObject;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.object
  }
}

impl DerefMut for JSGeneratorObject {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.object
  }
}

impl JSGeneratorObject {
  pub const K_HEADER_SIZE: usize = K_GENERATOR_HEADER_SIZE;
  pub const K_FUNCTION_OFFSET: usize = K_FUNCTION_OFFSET;
  pub const K_CONTEXT_OFFSET: usize = K_CONTEXT_OFFSET;
  pub const K_RECEIVER_OFFSET: usize = K_RECEIVER_OFFSET;
  pub const K_INPUT_OR_DEBUG_POS_OFFSET: usize = K_INPUT_OR_DEBUG_POS_OFFSET;
  pub const K_RESUME_MODE_OFFSET: usize = K_RESUME_MODE_OFFSET;
  pub const K_CONTINUATION_OFFSET: usize = K_CONTINUATION_OFFSET;
  pub const K_PARAMETERS_AND_REGISTERS_OFFSET: usize = K_PARAMETERS_AND_REGISTERS_OFFSET;
  pub const K_YIELDED_VALUE_OFFSET: usize = K_YIELDED_VALUE_OFFSET;
  pub const K_MAX_IN_OBJECT_PROPERTIES: usize = K_MAX_GENERATOR_IN_OBJECT_PROPERTIES;

  /// 创建新的 JSGeneratorObject 实例
  #[inline]
  pub const fn new(object: JSObject, fields: GeneratorFields) -> Self {
    Self {
      object,
      function: fields.function,
      context: fields.context,
      receiver: fields.receiver,
      input_or_debug_pos: fields.input_or_debug_pos,
      resume_mode: Smi::from_int(fields.resume_mode),
      continuation: Smi::from_int(fields.continuation),
      parameters_and_registers: fields.parameters_and_registers,
      yielded_value: fields.yielded_value,
    }
  }

  /// 从各组成字段构造 JSGeneratorObject 实例
  #[inline]
  pub const fn from_parts(
    map: Tagged<Map>,
    properties_or_hash: Tagged<Object>,
    elements: Tagged<FixedArrayBase>,
    fields: GeneratorFields,
  ) -> Self {
    Self::new(JSObject::new(map, properties_or_hash, elements), fields)
  }

  #[inline]
  fn atomic_function(&self) -> &AtomicUsize {
    // SAFETY: Tagged<JSFunction> 与 AtomicUsize 具有相同的字长和对齐
    unsafe { &*(&self.function as *const Tagged<JSFunction> as *const AtomicUsize) }
  }

  #[inline]
  fn atomic_context(&self) -> &AtomicUsize {
    // SAFETY: Tagged<Context> 与 AtomicUsize 具有相同的字长和对齐
    unsafe { &*(&self.context as *const Tagged<Context> as *const AtomicUsize) }
  }

  #[inline]
  fn atomic_receiver(&self) -> &AtomicUsize {
    // SAFETY: Tagged<Object> 与 AtomicUsize 具有相同的字长和对齐
    unsafe { &*(&self.receiver as *const Tagged<Object> as *const AtomicUsize) }
  }

  #[inline]
  fn atomic_input_or_debug_pos(&self) -> &AtomicUsize {
    // SAFETY: Tagged<Object> 与 AtomicUsize 具有相同的字长和对齐
    unsafe { &*(&self.input_or_debug_pos as *const Tagged<Object> as *const AtomicUsize) }
  }

  #[inline]
  fn atomic_resume_mode(&self) -> &AtomicUsize {
    // SAFETY: Tagged<Smi> 与 AtomicUsize 具有相同的字长和对齐
    unsafe { &*(&self.resume_mode as *const Tagged<Smi> as *const AtomicUsize) }
  }

  #[inline]
  fn atomic_continuation(&self) -> &AtomicUsize {
    // SAFETY: Tagged<Smi> 与 AtomicUsize 具有相同的字长和对齐
    unsafe { &*(&self.continuation as *const Tagged<Smi> as *const AtomicUsize) }
  }

  #[inline]
  fn atomic_parameters_and_registers(&self) -> &AtomicUsize {
    // SAFETY: Tagged<FixedArray> 与 AtomicUsize 具有相同的字长和对齐
    unsafe { &*(&self.parameters_and_registers as *const Tagged<FixedArray> as *const AtomicUsize) }
  }

  #[inline]
  fn atomic_yielded_value(&self) -> &AtomicUsize {
    // SAFETY: Tagged<Object> 与 AtomicUsize 具有相同的字长和对齐
    unsafe { &*(&self.yielded_value as *const Tagged<Object> as *const AtomicUsize) }
  }
}

// ── 5. JSGeneratorExt 扩展 Trait ───────────────────────────────────────────

/// JSGeneratorObject 专属操作扩展 trait
pub trait JSGeneratorExt {
  /// 获取关联的 JSFunction
  fn function(&self) -> Tagged<JSFunction>;
  /// 设置关联的 JSFunction
  fn set_function(&mut self, function: Tagged<JSFunction>);

  /// 获取绑定的 Context
  fn context(&self) -> Tagged<Context>;
  /// 设置绑定的 Context
  fn set_context(&mut self, context: Tagged<Context>);

  /// 获取 receiver
  fn receiver(&self) -> Tagged<Object>;
  /// 设置 receiver
  fn set_receiver(&mut self, receiver: Tagged<Object>);

  /// 获取 input_or_debug_pos
  fn input_or_debug_pos(&self) -> Tagged<Object>;
  /// 设置 input_or_debug_pos
  fn set_input_or_debug_pos(&mut self, value: Tagged<Object>);

  /// 获取原始 resume_mode 整数
  fn raw_resume_mode(&self) -> i32;
  /// 设置原始 resume_mode 整数
  fn set_raw_resume_mode(&mut self, mode: i32);

  /// 以 ResumeMode 解析恢复模式
  #[inline]
  fn resume_mode(&self) -> Result<ResumeMode> {
    ResumeMode::try_from(self.raw_resume_mode())
  }

  /// 设置 ResumeMode
  #[inline]
  fn set_resume_mode(&mut self, mode: ResumeMode) {
    self.set_raw_resume_mode(mode as i32);
  }

  /// 获取 continuation 状态数值
  fn continuation(&self) -> i32;
  /// 设置 continuation 状态数值
  fn set_continuation(&mut self, continuation: i32);

  /// 获取 parameters_and_registers
  fn parameters_and_registers(&self) -> Tagged<FixedArray>;
  /// 设置 parameters_and_registers
  fn set_parameters_and_registers(&mut self, array: Tagged<FixedArray>);

  /// 获取 yielded_value
  fn yielded_value(&self) -> Tagged<Object>;
  /// 设置 yielded_value
  fn set_yielded_value(&mut self, value: Tagged<Object>);

  /// 判断生成器是否处于挂起状态 (continuation >= 0)
  #[inline]
  fn is_suspended(&self) -> bool {
    self.continuation() >= 0
  }

  /// 判断生成器是否已关闭 (continuation == kGeneratorClosed)
  #[inline]
  fn is_closed(&self) -> bool {
    self.continuation() == K_GENERATOR_CLOSED
  }

  /// 判断生成器是否正在执行 (continuation == kGeneratorExecuting)
  #[inline]
  fn is_executing(&self) -> bool {
    self.continuation() == K_GENERATOR_EXECUTING
  }

  /// 将生成器标记为关闭
  #[inline]
  fn mark_closed(&mut self) {
    self.set_continuation(K_GENERATOR_CLOSED);
  }

  /// 将生成器标记为正在执行
  #[inline]
  fn mark_executing(&mut self) {
    self.set_continuation(K_GENERATOR_EXECUTING);
  }

  /// 将生成器挂起在指定偏移 (必须 >= 0)
  #[inline]
  fn suspend_at(&mut self, offset: i32) -> Result<()> {
    if offset < 0 {
      return Err(Error::Invariant("挂起位置必须为非负数"));
    }
    self.set_continuation(offset);
    Ok(())
  }
}

// ── 6. JSGeneratorObject Trait 实现 ────────────────────────────────────────

impl JSReceiverExt for JSGeneratorObject {
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

impl JSObjectExt for JSGeneratorObject {
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

impl JSGeneratorExt for JSGeneratorObject {
  #[inline]
  fn function(&self) -> Tagged<JSFunction> {
    Tagged::new(self.atomic_function().load(Ordering::Relaxed))
  }

  #[inline]
  fn set_function(&mut self, function: Tagged<JSFunction>) {
    self
      .atomic_function()
      .store(function.ptr(), Ordering::Relaxed);
  }

  #[inline]
  fn context(&self) -> Tagged<Context> {
    Tagged::new(self.atomic_context().load(Ordering::Relaxed))
  }

  #[inline]
  fn set_context(&mut self, context: Tagged<Context>) {
    self
      .atomic_context()
      .store(context.ptr(), Ordering::Relaxed);
  }

  #[inline]
  fn receiver(&self) -> Tagged<Object> {
    Tagged::new(self.atomic_receiver().load(Ordering::Relaxed))
  }

  #[inline]
  fn set_receiver(&mut self, receiver: Tagged<Object>) {
    self
      .atomic_receiver()
      .store(receiver.ptr(), Ordering::Relaxed);
  }

  #[inline]
  fn input_or_debug_pos(&self) -> Tagged<Object> {
    Tagged::new(self.atomic_input_or_debug_pos().load(Ordering::Relaxed))
  }

  #[inline]
  fn set_input_or_debug_pos(&mut self, value: Tagged<Object>) {
    self
      .atomic_input_or_debug_pos()
      .store(value.ptr(), Ordering::Relaxed);
  }

  #[inline]
  fn raw_resume_mode(&self) -> i32 {
    let raw = self.atomic_resume_mode().load(Ordering::Relaxed);
    Smi::value(Tagged::<Smi>::new(raw))
  }

  #[inline]
  fn set_raw_resume_mode(&mut self, mode: i32) {
    let smi = Smi::from_int(mode);
    self
      .atomic_resume_mode()
      .store(smi.ptr(), Ordering::Relaxed);
  }

  #[inline]
  fn continuation(&self) -> i32 {
    let raw = self.atomic_continuation().load(Ordering::Relaxed);
    Smi::value(Tagged::<Smi>::new(raw))
  }

  #[inline]
  fn set_continuation(&mut self, continuation: i32) {
    let smi = Smi::from_int(continuation);
    self
      .atomic_continuation()
      .store(smi.ptr(), Ordering::Relaxed);
  }

  #[inline]
  fn parameters_and_registers(&self) -> Tagged<FixedArray> {
    Tagged::new(
      self
        .atomic_parameters_and_registers()
        .load(Ordering::Relaxed),
    )
  }

  #[inline]
  fn set_parameters_and_registers(&mut self, array: Tagged<FixedArray>) {
    self
      .atomic_parameters_and_registers()
      .store(array.ptr(), Ordering::Relaxed);
  }

  #[inline]
  fn yielded_value(&self) -> Tagged<Object> {
    Tagged::new(self.atomic_yielded_value().load(Ordering::Relaxed))
  }

  #[inline]
  fn set_yielded_value(&mut self, value: Tagged<Object>) {
    self
      .atomic_yielded_value()
      .store(value.ptr(), Ordering::Relaxed);
  }
}

// ── 7. Tagged<JSGeneratorObject> Trait 实现 ────────────────────────────────

/// Tagged<JSGeneratorObject> 专属扩展 trait
pub trait TaggedJSGeneratorExt {
  /// 转换为 Tagged<JSObject>
  fn as_object(&self) -> Tagged<JSObject>;

  /// 转换为 Tagged<JSReceiver>
  fn as_receiver(&self) -> Tagged<JSReceiver>;

  /// 转换为 Tagged<JSGeneratorObject>
  fn as_generator(&self) -> Tagged<JSGeneratorObject>;
}

impl TaggedJSGeneratorExt for Tagged<JSGeneratorObject> {
  #[inline]
  fn as_object(&self) -> Tagged<JSObject> {
    Tagged::new(self.ptr())
  }

  #[inline]
  fn as_receiver(&self) -> Tagged<JSReceiver> {
    Tagged::new(self.ptr())
  }

  #[inline]
  fn as_generator(&self) -> Tagged<JSGeneratorObject> {
    *self
  }
}

impl JSGeneratorExt for Tagged<JSGeneratorObject> {
  #[inline]
  fn function(&self) -> Tagged<JSFunction> {
    // SAFETY: Tagged 包装合法 JSGeneratorObject 指针
    unsafe { (*self.as_ptr()).function() }
  }

  #[inline]
  fn set_function(&mut self, function: Tagged<JSFunction>) {
    // SAFETY: Tagged 包装合法可变 JSGeneratorObject 指针
    unsafe { (*self.as_mut_ptr()).set_function(function) }
  }

  #[inline]
  fn context(&self) -> Tagged<Context> {
    // SAFETY: Tagged 包装合法 JSGeneratorObject 指针
    unsafe { (*self.as_ptr()).context() }
  }

  #[inline]
  fn set_context(&mut self, context: Tagged<Context>) {
    // SAFETY: Tagged 包装合法可变 JSGeneratorObject 指针
    unsafe { (*self.as_mut_ptr()).set_context(context) }
  }

  #[inline]
  fn receiver(&self) -> Tagged<Object> {
    // SAFETY: Tagged 包装合法 JSGeneratorObject 指针
    unsafe { (*self.as_ptr()).receiver() }
  }

  #[inline]
  fn set_receiver(&mut self, receiver: Tagged<Object>) {
    // SAFETY: Tagged 包装合法可变 JSGeneratorObject 指针
    unsafe { (*self.as_mut_ptr()).set_receiver(receiver) }
  }

  #[inline]
  fn input_or_debug_pos(&self) -> Tagged<Object> {
    // SAFETY: Tagged 包装合法 JSGeneratorObject 指针
    unsafe { (*self.as_ptr()).input_or_debug_pos() }
  }

  #[inline]
  fn set_input_or_debug_pos(&mut self, value: Tagged<Object>) {
    // SAFETY: Tagged 包装合法可变 JSGeneratorObject 指针
    unsafe { (*self.as_mut_ptr()).set_input_or_debug_pos(value) }
  }

  #[inline]
  fn raw_resume_mode(&self) -> i32 {
    // SAFETY: Tagged 包装合法 JSGeneratorObject 指针
    unsafe { (*self.as_ptr()).raw_resume_mode() }
  }

  #[inline]
  fn set_raw_resume_mode(&mut self, mode: i32) {
    // SAFETY: Tagged 包装合法可变 JSGeneratorObject 指针
    unsafe { (*self.as_mut_ptr()).set_raw_resume_mode(mode) }
  }

  #[inline]
  fn continuation(&self) -> i32 {
    // SAFETY: Tagged 包装合法 JSGeneratorObject 指针
    unsafe { (*self.as_ptr()).continuation() }
  }

  #[inline]
  fn set_continuation(&mut self, continuation: i32) {
    // SAFETY: Tagged 包装合法可变 JSGeneratorObject 指针
    unsafe { (*self.as_mut_ptr()).set_continuation(continuation) }
  }

  #[inline]
  fn parameters_and_registers(&self) -> Tagged<FixedArray> {
    // SAFETY: Tagged 包装合法 JSGeneratorObject 指针
    unsafe { (*self.as_ptr()).parameters_and_registers() }
  }

  #[inline]
  fn set_parameters_and_registers(&mut self, array: Tagged<FixedArray>) {
    // SAFETY: Tagged 包装合法可变 JSGeneratorObject 指针
    unsafe { (*self.as_mut_ptr()).set_parameters_and_registers(array) }
  }

  #[inline]
  fn yielded_value(&self) -> Tagged<Object> {
    // SAFETY: Tagged 包装合法 JSGeneratorObject 指针
    unsafe { (*self.as_ptr()).yielded_value() }
  }

  #[inline]
  fn set_yielded_value(&mut self, value: Tagged<Object>) {
    // SAFETY: Tagged 包装合法可变 JSGeneratorObject 指针
    unsafe { (*self.as_mut_ptr()).set_yielded_value(value) }
  }
}

// ── 8. JSAsyncGeneratorObject 结构体 ───────────────────────────────────────

/// JSAsyncGeneratorObject 结构体 (对应 v8::internal::JSAsyncGeneratorObject)
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct JSAsyncGeneratorObject {
  /// JSGeneratorObject 基类部分
  pub generator: JSGeneratorObject,
  /// 异步生成器请求单向链表头节点 (AsyncGeneratorRequest 或 Undefined)
  pub queue: Tagged<Object>,
  /// 是否处于等待状态 (is_awaiting, Smi 0 或 1)
  pub is_awaiting: Tagged<Smi>,
}

impl Deref for JSAsyncGeneratorObject {
  type Target = JSGeneratorObject;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.generator
  }
}

impl DerefMut for JSAsyncGeneratorObject {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.generator
  }
}

impl JSAsyncGeneratorObject {
  pub const K_HEADER_SIZE: usize = K_ASYNC_GENERATOR_HEADER_SIZE;
  pub const K_QUEUE_OFFSET: usize = K_ASYNC_GEN_QUEUE_OFFSET;
  pub const K_IS_AWAITING_OFFSET: usize = K_ASYNC_GEN_IS_AWAITING_OFFSET;
  pub const K_MAX_IN_OBJECT_PROPERTIES: usize = K_MAX_ASYNC_GENERATOR_IN_OBJECT_PROPERTIES;

  /// 创建新的 JSAsyncGeneratorObject 实例
  #[inline]
  pub const fn new(generator: JSGeneratorObject, queue: Tagged<Object>, is_awaiting: bool) -> Self {
    Self {
      generator,
      queue,
      is_awaiting: Smi::from_int(if is_awaiting { 1 } else { 0 }),
    }
  }

  #[inline]
  fn atomic_queue(&self) -> &AtomicUsize {
    // SAFETY: Tagged<Object> 与 AtomicUsize 具有相同的字长和对齐
    unsafe { &*(&self.queue as *const Tagged<Object> as *const AtomicUsize) }
  }

  #[inline]
  fn atomic_is_awaiting(&self) -> &AtomicUsize {
    // SAFETY: Tagged<Smi> 与 AtomicUsize 具有相同的字长和对齐
    unsafe { &*(&self.is_awaiting as *const Tagged<Smi> as *const AtomicUsize) }
  }
}

/// JSAsyncGeneratorObject 专属操作扩展 trait
pub trait JSAsyncGeneratorExt: JSGeneratorExt {
  /// 获取异步请求队列头节点
  fn queue(&self) -> Tagged<Object>;
  /// 设置异步请求队列头节点
  fn set_queue(&mut self, queue: Tagged<Object>);

  /// 是否处于 awaiting 状态
  fn is_awaiting(&self) -> bool;
  /// 设置 awaiting 状态
  fn set_is_awaiting(&mut self, is_awaiting: bool);
}

impl JSReceiverExt for JSAsyncGeneratorObject {
  #[inline]
  fn map(&self) -> Tagged<Map> {
    self.generator.map()
  }

  #[inline]
  fn set_map(&mut self, map: Tagged<Map>) {
    self.generator.set_map(map);
  }

  #[inline]
  fn raw_properties_or_hash(&self) -> Tagged<Object> {
    self.generator.raw_properties_or_hash()
  }

  #[inline]
  fn set_raw_properties_or_hash(&mut self, value: Tagged<Object>) {
    self.generator.set_raw_properties_or_hash(value);
  }

  #[inline]
  fn has_fast_properties(&self) -> bool {
    self.generator.has_fast_properties()
  }

  #[inline]
  fn property_array(&self) -> Option<Tagged<PropertyArray>> {
    self.generator.property_array()
  }

  #[inline]
  fn get_prototype(&self) -> Tagged<Object> {
    self.generator.get_prototype()
  }

  #[inline]
  fn has_in_prototype_chain(&self, proto: Tagged<Object>) -> bool {
    self.generator.has_in_prototype_chain(proto)
  }

  #[inline]
  fn get_identity_hash(&self) -> Option<i32> {
    self.generator.get_identity_hash()
  }

  #[inline]
  fn set_identity_hash(&mut self, hash: i32) -> v8_objects_js_core::Result<()> {
    self.generator.set_identity_hash(hash)
  }
}

impl JSObjectExt for JSAsyncGeneratorObject {
  #[inline]
  fn elements(&self) -> Tagged<FixedArrayBase> {
    self.generator.elements()
  }

  #[inline]
  fn set_elements(&mut self, value: Tagged<FixedArrayBase>) {
    self.generator.set_elements(value);
  }

  #[inline]
  fn raw_fast_property_at(&self, index: FieldIndex) -> v8_objects_js_core::Result<Tagged<Object>> {
    self.generator.raw_fast_property_at(index)
  }

  #[inline]
  fn fast_property_at_put(
    &mut self,
    index: FieldIndex,
    value: Tagged<Object>,
  ) -> v8_objects_js_core::Result<()> {
    self.generator.fast_property_at_put(index, value)
  }
}

impl JSGeneratorExt for JSAsyncGeneratorObject {
  #[inline]
  fn function(&self) -> Tagged<JSFunction> {
    self.generator.function()
  }

  #[inline]
  fn set_function(&mut self, function: Tagged<JSFunction>) {
    self.generator.set_function(function);
  }

  #[inline]
  fn context(&self) -> Tagged<Context> {
    self.generator.context()
  }

  #[inline]
  fn set_context(&mut self, context: Tagged<Context>) {
    self.generator.set_context(context);
  }

  #[inline]
  fn receiver(&self) -> Tagged<Object> {
    self.generator.receiver()
  }

  #[inline]
  fn set_receiver(&mut self, receiver: Tagged<Object>) {
    self.generator.set_receiver(receiver);
  }

  #[inline]
  fn input_or_debug_pos(&self) -> Tagged<Object> {
    self.generator.input_or_debug_pos()
  }

  #[inline]
  fn set_input_or_debug_pos(&mut self, value: Tagged<Object>) {
    self.generator.set_input_or_debug_pos(value);
  }

  #[inline]
  fn raw_resume_mode(&self) -> i32 {
    self.generator.raw_resume_mode()
  }

  #[inline]
  fn set_raw_resume_mode(&mut self, mode: i32) {
    self.generator.set_raw_resume_mode(mode);
  }

  #[inline]
  fn continuation(&self) -> i32 {
    self.generator.continuation()
  }

  #[inline]
  fn set_continuation(&mut self, continuation: i32) {
    self.generator.set_continuation(continuation);
  }

  #[inline]
  fn parameters_and_registers(&self) -> Tagged<FixedArray> {
    self.generator.parameters_and_registers()
  }

  #[inline]
  fn set_parameters_and_registers(&mut self, array: Tagged<FixedArray>) {
    self.generator.set_parameters_and_registers(array);
  }

  #[inline]
  fn yielded_value(&self) -> Tagged<Object> {
    self.generator.yielded_value()
  }

  #[inline]
  fn set_yielded_value(&mut self, value: Tagged<Object>) {
    self.generator.set_yielded_value(value);
  }
}

impl JSAsyncGeneratorExt for JSAsyncGeneratorObject {
  #[inline]
  fn queue(&self) -> Tagged<Object> {
    Tagged::new(self.atomic_queue().load(Ordering::Relaxed))
  }

  #[inline]
  fn set_queue(&mut self, queue: Tagged<Object>) {
    self.atomic_queue().store(queue.ptr(), Ordering::Relaxed);
  }

  #[inline]
  fn is_awaiting(&self) -> bool {
    let raw = self.atomic_is_awaiting().load(Ordering::Relaxed);
    Smi::value(Tagged::<Smi>::new(raw)) != 0
  }

  #[inline]
  fn set_is_awaiting(&mut self, is_awaiting: bool) {
    let smi = Smi::from_int(if is_awaiting { 1 } else { 0 });
    self
      .atomic_is_awaiting()
      .store(smi.ptr(), Ordering::Relaxed);
  }
}

impl TaggedJSGeneratorExt for Tagged<JSAsyncGeneratorObject> {
  #[inline]
  fn as_object(&self) -> Tagged<JSObject> {
    Tagged::new(self.ptr())
  }

  #[inline]
  fn as_receiver(&self) -> Tagged<JSReceiver> {
    Tagged::new(self.ptr())
  }

  #[inline]
  fn as_generator(&self) -> Tagged<JSGeneratorObject> {
    Tagged::new(self.ptr())
  }
}

impl JSGeneratorExt for Tagged<JSAsyncGeneratorObject> {
  #[inline]
  fn function(&self) -> Tagged<JSFunction> {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_ptr()).function() }
  }

  #[inline]
  fn set_function(&mut self, function: Tagged<JSFunction>) {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_mut_ptr()).set_function(function) }
  }

  #[inline]
  fn context(&self) -> Tagged<Context> {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_ptr()).context() }
  }

  #[inline]
  fn set_context(&mut self, context: Tagged<Context>) {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_mut_ptr()).set_context(context) }
  }

  #[inline]
  fn receiver(&self) -> Tagged<Object> {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_ptr()).receiver() }
  }

  #[inline]
  fn set_receiver(&mut self, receiver: Tagged<Object>) {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_mut_ptr()).set_receiver(receiver) }
  }

  #[inline]
  fn input_or_debug_pos(&self) -> Tagged<Object> {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_ptr()).input_or_debug_pos() }
  }

  #[inline]
  fn set_input_or_debug_pos(&mut self, value: Tagged<Object>) {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_mut_ptr()).set_input_or_debug_pos(value) }
  }

  #[inline]
  fn raw_resume_mode(&self) -> i32 {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_ptr()).raw_resume_mode() }
  }

  #[inline]
  fn set_raw_resume_mode(&mut self, mode: i32) {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_mut_ptr()).set_raw_resume_mode(mode) }
  }

  #[inline]
  fn continuation(&self) -> i32 {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_ptr()).continuation() }
  }

  #[inline]
  fn set_continuation(&mut self, continuation: i32) {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_mut_ptr()).set_continuation(continuation) }
  }

  #[inline]
  fn parameters_and_registers(&self) -> Tagged<FixedArray> {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_ptr()).parameters_and_registers() }
  }

  #[inline]
  fn set_parameters_and_registers(&mut self, array: Tagged<FixedArray>) {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_mut_ptr()).set_parameters_and_registers(array) }
  }

  #[inline]
  fn yielded_value(&self) -> Tagged<Object> {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_ptr()).yielded_value() }
  }

  #[inline]
  fn set_yielded_value(&mut self, value: Tagged<Object>) {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_mut_ptr()).set_yielded_value(value) }
  }
}

impl JSAsyncGeneratorExt for Tagged<JSAsyncGeneratorObject> {
  #[inline]
  fn queue(&self) -> Tagged<Object> {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_ptr()).queue() }
  }

  #[inline]
  fn set_queue(&mut self, queue: Tagged<Object>) {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_mut_ptr()).set_queue(queue) }
  }

  #[inline]
  fn is_awaiting(&self) -> bool {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_ptr()).is_awaiting() }
  }

  #[inline]
  fn set_is_awaiting(&mut self, is_awaiting: bool) {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_mut_ptr()).set_is_awaiting(is_awaiting) }
  }
}

// ── 9. JSAsyncFunctionObject 结构体 ────────────────────────────────────────

/// JSAsyncFunctionObject 结构体 (对应 v8::internal::JSAsyncFunctionObject)
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct JSAsyncFunctionObject {
  /// JSGeneratorObject 基类部分
  pub generator: JSGeneratorObject,
  /// 关联的 JSPromise 实例
  pub promise: Tagged<JSPromise>,
  /// await 履约闭包 (JSFunction 或 Undefined)
  pub await_resolve_closure: Tagged<Object>,
  /// await 拒绝闭包 (JSFunction 或 Undefined)
  pub await_reject_closure: Tagged<Object>,
}

impl Deref for JSAsyncFunctionObject {
  type Target = JSGeneratorObject;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.generator
  }
}

impl DerefMut for JSAsyncFunctionObject {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.generator
  }
}

impl JSAsyncFunctionObject {
  pub const K_HEADER_SIZE: usize = K_ASYNC_FUNCTION_HEADER_SIZE;
  pub const K_PROMISE_OFFSET: usize = K_ASYNC_FUNC_PROMISE_OFFSET;
  pub const K_AWAIT_RESOLVE_CLOSURE_OFFSET: usize = K_ASYNC_FUNC_AWAIT_RESOLVE_CLOSURE_OFFSET;
  pub const K_AWAIT_REJECT_CLOSURE_OFFSET: usize = K_ASYNC_FUNC_AWAIT_REJECT_CLOSURE_OFFSET;

  /// 创建新的 JSAsyncFunctionObject 实例
  #[inline]
  pub const fn new(
    generator: JSGeneratorObject,
    promise: Tagged<JSPromise>,
    await_resolve_closure: Tagged<Object>,
    await_reject_closure: Tagged<Object>,
  ) -> Self {
    Self {
      generator,
      promise,
      await_resolve_closure,
      await_reject_closure,
    }
  }

  #[inline]
  fn atomic_promise(&self) -> &AtomicUsize {
    // SAFETY: Tagged<JSPromise> 与 AtomicUsize 具有相同的字长和对齐
    unsafe { &*(&self.promise as *const Tagged<JSPromise> as *const AtomicUsize) }
  }

  #[inline]
  fn atomic_await_resolve_closure(&self) -> &AtomicUsize {
    // SAFETY: Tagged<Object> 与 AtomicUsize 具有相同的字长和对齐
    unsafe { &*(&self.await_resolve_closure as *const Tagged<Object> as *const AtomicUsize) }
  }

  #[inline]
  fn atomic_await_reject_closure(&self) -> &AtomicUsize {
    // SAFETY: Tagged<Object> 与 AtomicUsize 具有相同的字长和对齐
    unsafe { &*(&self.await_reject_closure as *const Tagged<Object> as *const AtomicUsize) }
  }
}

impl JSReceiverExt for JSAsyncFunctionObject {
  #[inline]
  fn map(&self) -> Tagged<Map> {
    self.generator.map()
  }

  #[inline]
  fn set_map(&mut self, map: Tagged<Map>) {
    self.generator.set_map(map);
  }

  #[inline]
  fn raw_properties_or_hash(&self) -> Tagged<Object> {
    self.generator.raw_properties_or_hash()
  }

  #[inline]
  fn set_raw_properties_or_hash(&mut self, value: Tagged<Object>) {
    self.generator.set_raw_properties_or_hash(value);
  }

  #[inline]
  fn has_fast_properties(&self) -> bool {
    self.generator.has_fast_properties()
  }

  #[inline]
  fn property_array(&self) -> Option<Tagged<PropertyArray>> {
    self.generator.property_array()
  }

  #[inline]
  fn get_prototype(&self) -> Tagged<Object> {
    self.generator.get_prototype()
  }

  #[inline]
  fn has_in_prototype_chain(&self, proto: Tagged<Object>) -> bool {
    self.generator.has_in_prototype_chain(proto)
  }

  #[inline]
  fn get_identity_hash(&self) -> Option<i32> {
    self.generator.get_identity_hash()
  }

  #[inline]
  fn set_identity_hash(&mut self, hash: i32) -> v8_objects_js_core::Result<()> {
    self.generator.set_identity_hash(hash)
  }
}

impl JSObjectExt for JSAsyncFunctionObject {
  #[inline]
  fn elements(&self) -> Tagged<FixedArrayBase> {
    self.generator.elements()
  }

  #[inline]
  fn set_elements(&mut self, value: Tagged<FixedArrayBase>) {
    self.generator.set_elements(value);
  }

  #[inline]
  fn raw_fast_property_at(&self, index: FieldIndex) -> v8_objects_js_core::Result<Tagged<Object>> {
    self.generator.raw_fast_property_at(index)
  }

  #[inline]
  fn fast_property_at_put(
    &mut self,
    index: FieldIndex,
    value: Tagged<Object>,
  ) -> v8_objects_js_core::Result<()> {
    self.generator.fast_property_at_put(index, value)
  }
}

impl JSGeneratorExt for JSAsyncFunctionObject {
  #[inline]
  fn function(&self) -> Tagged<JSFunction> {
    self.generator.function()
  }

  #[inline]
  fn set_function(&mut self, function: Tagged<JSFunction>) {
    self.generator.set_function(function);
  }

  #[inline]
  fn context(&self) -> Tagged<Context> {
    self.generator.context()
  }

  #[inline]
  fn set_context(&mut self, context: Tagged<Context>) {
    self.generator.set_context(context);
  }

  #[inline]
  fn receiver(&self) -> Tagged<Object> {
    self.generator.receiver()
  }

  #[inline]
  fn set_receiver(&mut self, receiver: Tagged<Object>) {
    self.generator.set_receiver(receiver);
  }

  #[inline]
  fn input_or_debug_pos(&self) -> Tagged<Object> {
    self.generator.input_or_debug_pos()
  }

  #[inline]
  fn set_input_or_debug_pos(&mut self, value: Tagged<Object>) {
    self.generator.set_input_or_debug_pos(value);
  }

  #[inline]
  fn raw_resume_mode(&self) -> i32 {
    self.generator.raw_resume_mode()
  }

  #[inline]
  fn set_raw_resume_mode(&mut self, mode: i32) {
    self.generator.set_raw_resume_mode(mode);
  }

  #[inline]
  fn continuation(&self) -> i32 {
    self.generator.continuation()
  }

  #[inline]
  fn set_continuation(&mut self, continuation: i32) {
    self.generator.set_continuation(continuation);
  }

  #[inline]
  fn parameters_and_registers(&self) -> Tagged<FixedArray> {
    self.generator.parameters_and_registers()
  }

  #[inline]
  fn set_parameters_and_registers(&mut self, array: Tagged<FixedArray>) {
    self.generator.set_parameters_and_registers(array);
  }

  #[inline]
  fn yielded_value(&self) -> Tagged<Object> {
    self.generator.yielded_value()
  }

  #[inline]
  fn set_yielded_value(&mut self, value: Tagged<Object>) {
    self.generator.set_yielded_value(value);
  }
}

/// JSAsyncFunctionObject 专属操作扩展 trait
pub trait JSAsyncFunctionExt: JSGeneratorExt {
  /// 获取关联的 JSPromise
  fn promise(&self) -> Tagged<JSPromise>;
  /// 设置关联的 JSPromise
  fn set_promise(&mut self, promise: Tagged<JSPromise>);

  /// 获取 await_resolve_closure
  fn await_resolve_closure(&self) -> Tagged<Object>;
  /// 设置 await_resolve_closure
  fn set_await_resolve_closure(&mut self, closure: Tagged<Object>);

  /// 获取 await_reject_closure
  fn await_reject_closure(&self) -> Tagged<Object>;
  /// 设置 await_reject_closure
  fn set_await_reject_closure(&mut self, closure: Tagged<Object>);
}

impl JSAsyncFunctionExt for JSAsyncFunctionObject {
  #[inline]
  fn promise(&self) -> Tagged<JSPromise> {
    Tagged::new(self.atomic_promise().load(Ordering::Relaxed))
  }

  #[inline]
  fn set_promise(&mut self, promise: Tagged<JSPromise>) {
    self
      .atomic_promise()
      .store(promise.ptr(), Ordering::Relaxed);
  }

  #[inline]
  fn await_resolve_closure(&self) -> Tagged<Object> {
    Tagged::new(self.atomic_await_resolve_closure().load(Ordering::Relaxed))
  }

  #[inline]
  fn set_await_resolve_closure(&mut self, closure: Tagged<Object>) {
    self
      .atomic_await_resolve_closure()
      .store(closure.ptr(), Ordering::Relaxed);
  }

  #[inline]
  fn await_reject_closure(&self) -> Tagged<Object> {
    Tagged::new(self.atomic_await_reject_closure().load(Ordering::Relaxed))
  }

  #[inline]
  fn set_await_reject_closure(&mut self, closure: Tagged<Object>) {
    self
      .atomic_await_reject_closure()
      .store(closure.ptr(), Ordering::Relaxed);
  }
}

impl TaggedJSGeneratorExt for Tagged<JSAsyncFunctionObject> {
  #[inline]
  fn as_object(&self) -> Tagged<JSObject> {
    Tagged::new(self.ptr())
  }

  #[inline]
  fn as_receiver(&self) -> Tagged<JSReceiver> {
    Tagged::new(self.ptr())
  }

  #[inline]
  fn as_generator(&self) -> Tagged<JSGeneratorObject> {
    Tagged::new(self.ptr())
  }
}

impl JSGeneratorExt for Tagged<JSAsyncFunctionObject> {
  #[inline]
  fn function(&self) -> Tagged<JSFunction> {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_ptr()).function() }
  }

  #[inline]
  fn set_function(&mut self, function: Tagged<JSFunction>) {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_mut_ptr()).set_function(function) }
  }

  #[inline]
  fn context(&self) -> Tagged<Context> {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_ptr()).context() }
  }

  #[inline]
  fn set_context(&mut self, context: Tagged<Context>) {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_mut_ptr()).set_context(context) }
  }

  #[inline]
  fn receiver(&self) -> Tagged<Object> {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_ptr()).receiver() }
  }

  #[inline]
  fn set_receiver(&mut self, receiver: Tagged<Object>) {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_mut_ptr()).set_receiver(receiver) }
  }

  #[inline]
  fn input_or_debug_pos(&self) -> Tagged<Object> {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_ptr()).input_or_debug_pos() }
  }

  #[inline]
  fn set_input_or_debug_pos(&mut self, value: Tagged<Object>) {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_mut_ptr()).set_input_or_debug_pos(value) }
  }

  #[inline]
  fn raw_resume_mode(&self) -> i32 {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_ptr()).raw_resume_mode() }
  }

  #[inline]
  fn set_raw_resume_mode(&mut self, mode: i32) {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_mut_ptr()).set_raw_resume_mode(mode) }
  }

  #[inline]
  fn continuation(&self) -> i32 {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_ptr()).continuation() }
  }

  #[inline]
  fn set_continuation(&mut self, continuation: i32) {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_mut_ptr()).set_continuation(continuation) }
  }

  #[inline]
  fn parameters_and_registers(&self) -> Tagged<FixedArray> {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_ptr()).parameters_and_registers() }
  }

  #[inline]
  fn set_parameters_and_registers(&mut self, array: Tagged<FixedArray>) {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_mut_ptr()).set_parameters_and_registers(array) }
  }

  #[inline]
  fn yielded_value(&self) -> Tagged<Object> {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_ptr()).yielded_value() }
  }

  #[inline]
  fn set_yielded_value(&mut self, value: Tagged<Object>) {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_mut_ptr()).set_yielded_value(value) }
  }
}

impl JSAsyncFunctionExt for Tagged<JSAsyncFunctionObject> {
  #[inline]
  fn promise(&self) -> Tagged<JSPromise> {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_ptr()).promise() }
  }

  #[inline]
  fn set_promise(&mut self, promise: Tagged<JSPromise>) {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_mut_ptr()).set_promise(promise) }
  }

  #[inline]
  fn await_resolve_closure(&self) -> Tagged<Object> {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_ptr()).await_resolve_closure() }
  }

  #[inline]
  fn set_await_resolve_closure(&mut self, closure: Tagged<Object>) {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_mut_ptr()).set_await_resolve_closure(closure) }
  }

  #[inline]
  fn await_reject_closure(&self) -> Tagged<Object> {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_ptr()).await_reject_closure() }
  }

  #[inline]
  fn set_await_reject_closure(&mut self, closure: Tagged<Object>) {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_mut_ptr()).set_await_reject_closure(closure) }
  }
}

// ── 10. AsyncGeneratorRequest 结构体 ───────────────────────────────────────

/// AsyncGeneratorRequest 结构体 (对应 v8::internal::AsyncGeneratorRequest)
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AsyncGeneratorRequest {
  /// Struct 基类 (HeapObject map 槽位)
  pub map: Tagged<Map>,
  /// 下一个请求 (AsyncGeneratorRequest 或 Undefined)
  pub next: Tagged<Object>,
  /// 恢复模式 (Smi 编码)
  pub resume_mode: Tagged<Smi>,
  /// 传入的值或参数 (Object)
  pub value: Tagged<Object>,
  /// 关联的返回 Promise (JSPromise)
  pub promise: Tagged<JSPromise>,
}

impl AsyncGeneratorRequest {
  pub const K_HEADER_SIZE: usize = K_ASYNC_GENERATOR_REQUEST_HEADER_SIZE;
  pub const K_NEXT_OFFSET: usize = K_REQ_NEXT_OFFSET;
  pub const K_RESUME_MODE_OFFSET: usize = K_REQ_RESUME_MODE_OFFSET;
  pub const K_VALUE_OFFSET: usize = K_REQ_VALUE_OFFSET;
  pub const K_PROMISE_OFFSET: usize = K_REQ_PROMISE_OFFSET;

  /// 创建新的 AsyncGeneratorRequest 实例
  #[inline]
  pub const fn new(
    map: Tagged<Map>,
    next: Tagged<Object>,
    resume_mode: i32,
    value: Tagged<Object>,
    promise: Tagged<JSPromise>,
  ) -> Self {
    Self {
      map,
      next,
      resume_mode: Smi::from_int(resume_mode),
      value,
      promise,
    }
  }

  #[inline]
  fn atomic_map(&self) -> &AtomicUsize {
    // SAFETY: Tagged<Map> 与 AtomicUsize 具有相同的字长和对齐
    unsafe { &*(&self.map as *const Tagged<Map> as *const AtomicUsize) }
  }

  #[inline]
  fn atomic_next(&self) -> &AtomicUsize {
    // SAFETY: Tagged<Object> 与 AtomicUsize 具有相同的字长和对齐
    unsafe { &*(&self.next as *const Tagged<Object> as *const AtomicUsize) }
  }

  #[inline]
  fn atomic_resume_mode(&self) -> &AtomicUsize {
    // SAFETY: Tagged<Smi> 与 AtomicUsize 具有相同的字长和对齐
    unsafe { &*(&self.resume_mode as *const Tagged<Smi> as *const AtomicUsize) }
  }

  #[inline]
  fn atomic_value(&self) -> &AtomicUsize {
    // SAFETY: Tagged<Object> 与 AtomicUsize 具有相同的字长和对齐
    unsafe { &*(&self.value as *const Tagged<Object> as *const AtomicUsize) }
  }

  #[inline]
  fn atomic_promise(&self) -> &AtomicUsize {
    // SAFETY: Tagged<JSPromise> 与 AtomicUsize 具有相同的字长和对齐
    unsafe { &*(&self.promise as *const Tagged<JSPromise> as *const AtomicUsize) }
  }

  /// 获取 map
  #[inline]
  pub fn map(&self) -> Tagged<Map> {
    Tagged::new(self.atomic_map().load(Ordering::Relaxed))
  }

  /// 设置 map
  #[inline]
  pub fn set_map(&mut self, map: Tagged<Map>) {
    self.atomic_map().store(map.ptr(), Ordering::Relaxed);
  }

  /// 获取 next
  #[inline]
  pub fn next(&self) -> Tagged<Object> {
    Tagged::new(self.atomic_next().load(Ordering::Relaxed))
  }

  /// 设置 next
  #[inline]
  pub fn set_next(&mut self, next: Tagged<Object>) {
    self.atomic_next().store(next.ptr(), Ordering::Relaxed);
  }

  /// 获取原始 resume_mode 整数
  #[inline]
  pub fn raw_resume_mode(&self) -> i32 {
    let raw = self.atomic_resume_mode().load(Ordering::Relaxed);
    Smi::value(Tagged::<Smi>::new(raw))
  }

  /// 设置原始 resume_mode 整数
  #[inline]
  pub fn set_raw_resume_mode(&mut self, mode: i32) {
    let smi = Smi::from_int(mode);
    self
      .atomic_resume_mode()
      .store(smi.ptr(), Ordering::Relaxed);
  }

  /// 获取 resume_mode
  #[inline]
  pub fn resume_mode(&self) -> Result<ResumeMode> {
    ResumeMode::try_from(self.raw_resume_mode())
  }

  /// 设置 resume_mode
  #[inline]
  pub fn set_resume_mode(&mut self, mode: ResumeMode) {
    self.set_raw_resume_mode(mode as i32);
  }

  /// 获取 value
  #[inline]
  pub fn value(&self) -> Tagged<Object> {
    Tagged::new(self.atomic_value().load(Ordering::Relaxed))
  }

  /// 设置 value
  #[inline]
  pub fn set_value(&mut self, value: Tagged<Object>) {
    self.atomic_value().store(value.ptr(), Ordering::Relaxed);
  }

  /// 获取 promise
  #[inline]
  pub fn promise(&self) -> Tagged<JSPromise> {
    Tagged::new(self.atomic_promise().load(Ordering::Relaxed))
  }

  /// 设置 promise
  #[inline]
  pub fn set_promise(&mut self, promise: Tagged<JSPromise>) {
    self
      .atomic_promise()
      .store(promise.ptr(), Ordering::Relaxed);
  }
}

/// AsyncGeneratorRequest 专属操作扩展 trait
pub trait AsyncGeneratorRequestExt {
  /// 获取 map
  fn map(&self) -> Tagged<Map>;
  /// 设置 map
  fn set_map(&mut self, map: Tagged<Map>);
  /// 获取 next
  fn next(&self) -> Tagged<Object>;
  /// 设置 next
  fn set_next(&mut self, next: Tagged<Object>);
  /// 获取原始 resume_mode 整数
  fn raw_resume_mode(&self) -> i32;
  /// 设置原始 resume_mode 整数
  fn set_raw_resume_mode(&mut self, mode: i32);
  /// 获取 resume_mode
  fn resume_mode(&self) -> Result<ResumeMode>;
  /// 设置 resume_mode
  fn set_resume_mode(&mut self, mode: ResumeMode);
  /// 获取 value
  fn value(&self) -> Tagged<Object>;
  /// 设置 value
  fn set_value(&mut self, value: Tagged<Object>);
  /// 获取 promise
  fn promise(&self) -> Tagged<JSPromise>;
  /// 设置 promise
  fn set_promise(&mut self, promise: Tagged<JSPromise>);
}

impl AsyncGeneratorRequestExt for AsyncGeneratorRequest {
  #[inline]
  fn map(&self) -> Tagged<Map> {
    self.map()
  }

  #[inline]
  fn set_map(&mut self, map: Tagged<Map>) {
    self.set_map(map);
  }

  #[inline]
  fn next(&self) -> Tagged<Object> {
    self.next()
  }

  #[inline]
  fn set_next(&mut self, next: Tagged<Object>) {
    self.set_next(next);
  }

  #[inline]
  fn raw_resume_mode(&self) -> i32 {
    self.raw_resume_mode()
  }

  #[inline]
  fn set_raw_resume_mode(&mut self, mode: i32) {
    self.set_raw_resume_mode(mode);
  }

  #[inline]
  fn resume_mode(&self) -> Result<ResumeMode> {
    self.resume_mode()
  }

  #[inline]
  fn set_resume_mode(&mut self, mode: ResumeMode) {
    self.set_resume_mode(mode);
  }

  #[inline]
  fn value(&self) -> Tagged<Object> {
    self.value()
  }

  #[inline]
  fn set_value(&mut self, value: Tagged<Object>) {
    self.set_value(value);
  }

  #[inline]
  fn promise(&self) -> Tagged<JSPromise> {
    self.promise()
  }

  #[inline]
  fn set_promise(&mut self, promise: Tagged<JSPromise>) {
    self.set_promise(promise);
  }
}

impl AsyncGeneratorRequestExt for Tagged<AsyncGeneratorRequest> {
  #[inline]
  fn map(&self) -> Tagged<Map> {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_ptr()).map() }
  }

  #[inline]
  fn set_map(&mut self, map: Tagged<Map>) {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_mut_ptr()).set_map(map) }
  }

  #[inline]
  fn next(&self) -> Tagged<Object> {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_ptr()).next() }
  }

  #[inline]
  fn set_next(&mut self, next: Tagged<Object>) {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_mut_ptr()).set_next(next) }
  }

  #[inline]
  fn raw_resume_mode(&self) -> i32 {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_ptr()).raw_resume_mode() }
  }

  #[inline]
  fn set_raw_resume_mode(&mut self, mode: i32) {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_mut_ptr()).set_raw_resume_mode(mode) }
  }

  #[inline]
  fn resume_mode(&self) -> Result<ResumeMode> {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_ptr()).resume_mode() }
  }

  #[inline]
  fn set_resume_mode(&mut self, mode: ResumeMode) {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_mut_ptr()).set_resume_mode(mode) }
  }

  #[inline]
  fn value(&self) -> Tagged<Object> {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_ptr()).value() }
  }

  #[inline]
  fn set_value(&mut self, value: Tagged<Object>) {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_mut_ptr()).set_value(value) }
  }

  #[inline]
  fn promise(&self) -> Tagged<JSPromise> {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_ptr()).promise() }
  }

  #[inline]
  fn set_promise(&mut self, promise: Tagged<JSPromise>) {
    // SAFETY: Tagged 包装合法指针
    unsafe { (*self.as_mut_ptr()).set_promise(promise) }
  }
}

// ── 11. 连续内存容器 (Buf) ─────────────────────────────────────────────────

/// 拥有独立连续内存的 JSGeneratorObject 实例容器
pub struct JSGeneratorObjectBuf {
  _storage: Vec<u64>,
  tagged: Tagged<JSGeneratorObject>,
  in_object_count: usize,
}

impl JSGeneratorObjectBuf {
  /// 创建新的 JSGeneratorObjectBuf 实例
  pub fn new(generator: JSGeneratorObject, in_object_count: usize) -> Result<Self> {
    if in_object_count > K_MAX_GENERATOR_IN_OBJECT_PROPERTIES {
      return Err(Error::InvalidCapacity(in_object_count));
    }
    let total_bytes = K_GENERATOR_HEADER_SIZE + in_object_count * mem::size_of::<usize>();
    let words = total_bytes.div_ceil(8);
    let mut storage = vec![0u64; words];
    let base = storage.as_mut_ptr() as usize;

    // SAFETY: base 指向足额分配且 8 字节对齐的连续堆内存首地址
    unsafe {
      ptr::write(base as *mut JSGeneratorObject, generator);
    }

    Ok(Self {
      _storage: storage,
      // SAFETY: base 包装为 Tagged<JSGeneratorObject>
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
  pub fn as_tagged(&self) -> Tagged<JSGeneratorObject> {
    self.tagged
  }
}

impl Deref for JSGeneratorObjectBuf {
  type Target = Tagged<JSGeneratorObject>;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.tagged
  }
}

impl DerefMut for JSGeneratorObjectBuf {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.tagged
  }
}

/// 拥有独立连续内存的 JSAsyncGeneratorObject 实例容器
pub struct JSAsyncGeneratorObjectBuf {
  _storage: Vec<u64>,
  tagged: Tagged<JSAsyncGeneratorObject>,
  in_object_count: usize,
}

impl JSAsyncGeneratorObjectBuf {
  /// 创建新的 JSAsyncGeneratorObjectBuf 实例
  pub fn new(obj: JSAsyncGeneratorObject, in_object_count: usize) -> Result<Self> {
    if in_object_count > K_MAX_ASYNC_GENERATOR_IN_OBJECT_PROPERTIES {
      return Err(Error::InvalidCapacity(in_object_count));
    }
    let total_bytes = K_ASYNC_GENERATOR_HEADER_SIZE + in_object_count * mem::size_of::<usize>();
    let words = total_bytes.div_ceil(8);
    let mut storage = vec![0u64; words];
    let base = storage.as_mut_ptr() as usize;

    // SAFETY: base 指向足额分配且 8 字节对齐的连续堆内存首地址
    unsafe {
      ptr::write(base as *mut JSAsyncGeneratorObject, obj);
    }

    Ok(Self {
      _storage: storage,
      // SAFETY: base 包装为 Tagged<JSAsyncGeneratorObject>
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
  pub fn as_tagged(&self) -> Tagged<JSAsyncGeneratorObject> {
    self.tagged
  }
}

impl Deref for JSAsyncGeneratorObjectBuf {
  type Target = Tagged<JSAsyncGeneratorObject>;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.tagged
  }
}

impl DerefMut for JSAsyncGeneratorObjectBuf {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.tagged
  }
}

/// 拥有独立连续内存的 AsyncGeneratorRequest 实例容器
pub struct AsyncGeneratorRequestBuf {
  _storage: Vec<u64>,
  tagged: Tagged<AsyncGeneratorRequest>,
}

impl AsyncGeneratorRequestBuf {
  /// 创建新的 AsyncGeneratorRequestBuf 实例
  pub fn new(req: AsyncGeneratorRequest) -> Self {
    let total_bytes = K_ASYNC_GENERATOR_REQUEST_HEADER_SIZE;
    let words = total_bytes.div_ceil(8);
    let mut storage = vec![0u64; words];
    let base = storage.as_mut_ptr() as usize;

    // SAFETY: base 指向足额分配且 8 字节对齐的连续堆内存首地址
    unsafe {
      ptr::write(base as *mut AsyncGeneratorRequest, req);
    }

    Self {
      _storage: storage,
      // SAFETY: base 包装为 Tagged<AsyncGeneratorRequest>
      tagged: unsafe { Tagged::from_raw_address(base) },
    }
  }

  /// 获取 Tagged 引用
  #[inline]
  pub fn as_tagged(&self) -> Tagged<AsyncGeneratorRequest> {
    self.tagged
  }
}

impl Deref for AsyncGeneratorRequestBuf {
  type Target = Tagged<AsyncGeneratorRequest>;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.tagged
  }
}

impl DerefMut for AsyncGeneratorRequestBuf {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.tagged
  }
}

/// 拥有独立连续内存的 JSAsyncFunctionObject 实例容器
pub struct JSAsyncFunctionObjectBuf {
  _storage: Vec<u64>,
  tagged: Tagged<JSAsyncFunctionObject>,
  in_object_count: usize,
}

impl JSAsyncFunctionObjectBuf {
  /// 创建新的 JSAsyncFunctionObjectBuf 实例
  pub fn new(obj: JSAsyncFunctionObject, in_object_count: usize) -> Result<Self> {
    if in_object_count > K_MAX_ASYNC_FUNCTION_IN_OBJECT_PROPERTIES {
      return Err(Error::InvalidCapacity(in_object_count));
    }
    let total_bytes = K_ASYNC_FUNCTION_HEADER_SIZE + in_object_count * mem::size_of::<usize>();
    let words = total_bytes.div_ceil(8);
    let mut storage = vec![0u64; words];
    let base = storage.as_mut_ptr() as usize;

    // SAFETY: base 指向足额分配且 8 字节对齐的连续堆内存首地址
    unsafe {
      ptr::write(base as *mut JSAsyncFunctionObject, obj);
    }

    Ok(Self {
      _storage: storage,
      // SAFETY: base 包装为 Tagged<JSAsyncFunctionObject>
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
  pub fn as_tagged(&self) -> Tagged<JSAsyncFunctionObject> {
    self.tagged
  }
}

impl Deref for JSAsyncFunctionObjectBuf {
  type Target = Tagged<JSAsyncFunctionObject>;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.tagged
  }
}

impl DerefMut for JSAsyncFunctionObjectBuf {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.tagged
  }
}
