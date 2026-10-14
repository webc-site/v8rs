// v8-objects-js-func/src/js_function.rs
// 对应原版 v8/src/objects/js-function.h + js-function-inl.h

//! JSFunction: JavaScript 函数对象，继承自 JSObject，
//! 包含执行代码派发句柄 (dispatch_handle)、共享函数信息 (SharedFunctionInfo)、
//! 上下文 (Context)、反馈元胞 (FeedbackCell) 及可选的原型或初始 Map 槽位。

use std::{
  mem,
  ops::{Deref, DerefMut},
  ptr,
};

use v8_objects_arrays::fixed_array_base::FixedArrayBase;
use v8_objects_core::{
  instance_type_checker::is_js_function_without_prototype,
  map::Map,
  tagged::{Object, Tagged},
};
use v8_objects_js_core::{
  field_index::FieldIndex,
  js_object::{
    JSObject, JSObjectExt, K_HEADER_SIZE as K_JS_OBJECT_HEADER_SIZE, K_MAX_IN_OBJECT_PROPERTIES,
  },
  js_receiver::{JSReceiver, JSReceiverExt},
};

use crate::{
  context::Context,
  error::{Error, Result},
  shared_function_info::SharedFunctionInfo,
};

/// dispatch_handle_ 字段偏移量
pub const K_DISPATCH_HANDLE_OFFSET: usize = K_JS_OBJECT_HEADER_SIZE;

/// shared_function_info_ 槽位偏移量
pub const K_SHARED_FUNCTION_INFO_OFFSET: usize = K_DISPATCH_HANDLE_OFFSET + 8;

/// context_ 槽位偏移量
pub const K_CONTEXT_OFFSET: usize = K_SHARED_FUNCTION_INFO_OFFSET + 8;

/// feedback_cell_ 槽位偏移量
pub const K_FEEDBACK_CELL_OFFSET: usize = K_CONTEXT_OFFSET + 8;

/// JSFunction 头部大小 (7 words = 56 字节)
pub const K_HEADER_SIZE: usize = mem::size_of::<JSFunction>();

/// prototype_or_initial_map_ 槽位偏移量
pub const K_PROTOTYPE_OR_INITIAL_MAP_OFFSET: usize = K_HEADER_SIZE;

/// 带 prototype 槽位的 JSFunction 头部大小 (8 words = 64 字节)
pub const K_WITH_PROTOTYPE_HEADER_SIZE: usize = mem::size_of::<JSFunctionWithPrototype>();

// 编译期布局断言
const _: () = assert!(K_DISPATCH_HANDLE_OFFSET == 24);
const _: () = assert!(K_SHARED_FUNCTION_INFO_OFFSET == 32);
const _: () = assert!(K_CONTEXT_OFFSET == 40);
const _: () = assert!(K_FEEDBACK_CELL_OFFSET == 48);
const _: () = assert!(K_HEADER_SIZE == 56);
const _: () = assert!(K_WITH_PROTOTYPE_HEADER_SIZE == 64);

/// JSFunction 结构体（无独立 prototype 槽位的基础函数）
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct JSFunction {
  /// JSObject 基类部分（包含 map, properties_or_hash, elements）
  pub object: JSObject,
  /// 派发句柄编号 (4 字节)
  pub dispatch_handle: u32,
  /// 64 位平台对齐填充 (4 字节)
  pub _padding: u32,
  /// SharedFunctionInfo 槽位 (8 字节)
  pub shared_function_info: Tagged<SharedFunctionInfo>,
  /// 上下文槽位 (8 字节)
  pub context: Tagged<Context>,
  /// 反馈单元槽位 (8 字节)
  pub feedback_cell: Tagged<Object>,
}

impl Deref for JSFunction {
  type Target = JSObject;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.object
  }
}

impl DerefMut for JSFunction {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.object
  }
}

impl JSFunction {
  /// 创建新的 JSFunction 实例
  #[inline]
  pub const fn new(
    map: Tagged<Map>,
    properties_or_hash: Tagged<Object>,
    elements: Tagged<FixedArrayBase>,
    dispatch_handle: u32,
    shared_function_info: Tagged<SharedFunctionInfo>,
    context: Tagged<Context>,
    feedback_cell: Tagged<Object>,
  ) -> Self {
    Self {
      object: JSObject::new(map, properties_or_hash, elements),
      dispatch_handle,
      _padding: 0,
      shared_function_info,
      context,
      feedback_cell,
    }
  }
}

/// 带有 prototype 槽位的 JSFunction 结构体
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct JSFunctionWithPrototype {
  /// JSFunction 基础部分
  pub function: JSFunction,
  /// 原型或 initial map 槽位
  pub prototype_or_initial_map: Tagged<Object>,
}

impl Deref for JSFunctionWithPrototype {
  type Target = JSFunction;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.function
  }
}

impl DerefMut for JSFunctionWithPrototype {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.function
  }
}

/// JSFunction 扩展访问 trait
pub trait JSFunctionExt {
  /// 获取派发句柄
  fn dispatch_handle(&self) -> u32;

  /// 设置派发句柄
  fn set_dispatch_handle(&mut self, handle: u32);

  /// 获取 SharedFunctionInfo
  fn shared(&self) -> Tagged<SharedFunctionInfo>;

  /// 设置 SharedFunctionInfo
  fn set_shared(&mut self, shared: Tagged<SharedFunctionInfo>);

  /// 获取 Context
  fn context(&self) -> Tagged<Context>;

  /// 设置 Context
  fn set_context(&mut self, context: Tagged<Context>);

  /// 获取 FeedbackCell
  fn feedback_cell(&self) -> Tagged<Object>;

  /// 设置 FeedbackCell
  fn set_feedback_cell(&mut self, cell: Tagged<Object>);

  /// 是否包含 prototype 槽位
  fn has_prototype_slot(&self) -> bool;

  /// 获取 prototype 或 initial map
  fn prototype_or_initial_map(&self) -> Result<Tagged<Object>>;

  /// 设置 prototype 或 initial map
  fn set_prototype_or_initial_map(&mut self, val: Tagged<Object>) -> Result<()>;

  /// 判断该函数是否已完成编译
  #[inline]
  fn is_compiled(&self) -> bool {
    self.dispatch_handle() != 0
  }
}

impl JSReceiverExt for JSFunction {
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
  fn property_array(&self) -> Option<Tagged<v8_objects_maps::PropertyArray>> {
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

impl JSObjectExt for JSFunction {
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

impl JSFunctionExt for JSFunction {
  #[inline]
  fn dispatch_handle(&self) -> u32 {
    self.dispatch_handle
  }

  #[inline]
  fn set_dispatch_handle(&mut self, handle: u32) {
    self.dispatch_handle = handle;
  }

  #[inline]
  fn shared(&self) -> Tagged<SharedFunctionInfo> {
    self.shared_function_info
  }

  #[inline]
  fn set_shared(&mut self, shared: Tagged<SharedFunctionInfo>) {
    self.shared_function_info = shared;
  }

  #[inline]
  fn context(&self) -> Tagged<Context> {
    self.context
  }

  #[inline]
  fn set_context(&mut self, context: Tagged<Context>) {
    self.context = context;
  }

  #[inline]
  fn feedback_cell(&self) -> Tagged<Object> {
    self.feedback_cell
  }

  #[inline]
  fn set_feedback_cell(&mut self, cell: Tagged<Object>) {
    self.feedback_cell = cell;
  }

  #[inline]
  fn has_prototype_slot(&self) -> bool {
    false
  }

  #[inline]
  fn prototype_or_initial_map(&self) -> Result<Tagged<Object>> {
    Err(Error::NoPrototypeSlot)
  }

  #[inline]
  fn set_prototype_or_initial_map(&mut self, _val: Tagged<Object>) -> Result<()> {
    Err(Error::NoPrototypeSlot)
  }
}

impl JSReceiverExt for JSFunctionWithPrototype {
  #[inline]
  fn map(&self) -> Tagged<Map> {
    self.function.map()
  }

  #[inline]
  fn set_map(&mut self, map: Tagged<Map>) {
    self.function.set_map(map);
  }

  #[inline]
  fn raw_properties_or_hash(&self) -> Tagged<Object> {
    self.function.raw_properties_or_hash()
  }

  #[inline]
  fn set_raw_properties_or_hash(&mut self, value: Tagged<Object>) {
    self.function.set_raw_properties_or_hash(value);
  }

  #[inline]
  fn has_fast_properties(&self) -> bool {
    self.function.has_fast_properties()
  }

  #[inline]
  fn property_array(&self) -> Option<Tagged<v8_objects_maps::PropertyArray>> {
    self.function.property_array()
  }

  #[inline]
  fn get_prototype(&self) -> Tagged<Object> {
    self.function.get_prototype()
  }

  #[inline]
  fn has_in_prototype_chain(&self, proto: Tagged<Object>) -> bool {
    self.function.has_in_prototype_chain(proto)
  }

  #[inline]
  fn get_identity_hash(&self) -> Option<i32> {
    self.function.get_identity_hash()
  }

  #[inline]
  fn set_identity_hash(&mut self, hash: i32) -> v8_objects_js_core::Result<()> {
    self.function.set_identity_hash(hash)
  }
}

impl JSObjectExt for JSFunctionWithPrototype {
  #[inline]
  fn elements(&self) -> Tagged<FixedArrayBase> {
    self.function.elements()
  }

  #[inline]
  fn set_elements(&mut self, value: Tagged<FixedArrayBase>) {
    self.function.set_elements(value);
  }

  #[inline]
  fn raw_fast_property_at(&self, index: FieldIndex) -> v8_objects_js_core::Result<Tagged<Object>> {
    self.function.raw_fast_property_at(index)
  }

  #[inline]
  fn fast_property_at_put(
    &mut self,
    index: FieldIndex,
    value: Tagged<Object>,
  ) -> v8_objects_js_core::Result<()> {
    self.function.fast_property_at_put(index, value)
  }
}

impl JSFunctionExt for JSFunctionWithPrototype {
  #[inline]
  fn dispatch_handle(&self) -> u32 {
    self.function.dispatch_handle()
  }

  #[inline]
  fn set_dispatch_handle(&mut self, handle: u32) {
    self.function.set_dispatch_handle(handle);
  }

  #[inline]
  fn shared(&self) -> Tagged<SharedFunctionInfo> {
    self.function.shared()
  }

  #[inline]
  fn set_shared(&mut self, shared: Tagged<SharedFunctionInfo>) {
    self.function.set_shared(shared);
  }

  #[inline]
  fn context(&self) -> Tagged<Context> {
    self.function.context()
  }

  #[inline]
  fn set_context(&mut self, context: Tagged<Context>) {
    self.function.set_context(context);
  }

  #[inline]
  fn feedback_cell(&self) -> Tagged<Object> {
    self.function.feedback_cell()
  }

  #[inline]
  fn set_feedback_cell(&mut self, cell: Tagged<Object>) {
    self.function.set_feedback_cell(cell);
  }

  #[inline]
  fn has_prototype_slot(&self) -> bool {
    true
  }

  #[inline]
  fn prototype_or_initial_map(&self) -> Result<Tagged<Object>> {
    Ok(self.prototype_or_initial_map)
  }

  #[inline]
  fn set_prototype_or_initial_map(&mut self, val: Tagged<Object>) -> Result<()> {
    self.prototype_or_initial_map = val;
    Ok(())
  }
}

/// Tagged<JSFunction> 扩展 trait
pub trait TaggedJSFunctionExt {
  /// 转换为 Tagged<JSObject>
  fn as_object(&self) -> Tagged<JSObject>;

  /// 转换为 Tagged<JSReceiver>
  fn as_receiver(&self) -> Tagged<JSReceiver>;
}

impl TaggedJSFunctionExt for Tagged<JSFunction> {
  #[inline]
  fn as_object(&self) -> Tagged<JSObject> {
    Tagged::new(self.ptr())
  }

  #[inline]
  fn as_receiver(&self) -> Tagged<JSReceiver> {
    Tagged::new(self.ptr())
  }
}

impl JSFunctionExt for Tagged<JSFunction> {
  #[inline]
  fn dispatch_handle(&self) -> u32 {
    // SAFETY: Tagged 包装合法 JSFunction 堆对象指针
    unsafe { (*self.as_ptr()).dispatch_handle() }
  }

  #[inline]
  fn set_dispatch_handle(&mut self, handle: u32) {
    // SAFETY: Tagged 包装合法可变 JSFunction 堆对象指针
    unsafe { (*self.as_mut_ptr()).set_dispatch_handle(handle) }
  }

  #[inline]
  fn shared(&self) -> Tagged<SharedFunctionInfo> {
    // SAFETY: Tagged 包装合法 JSFunction 堆对象指针
    unsafe { (*self.as_ptr()).shared() }
  }

  #[inline]
  fn set_shared(&mut self, shared: Tagged<SharedFunctionInfo>) {
    // SAFETY: Tagged 包装合法可变 JSFunction 堆对象指针
    unsafe { (*self.as_mut_ptr()).set_shared(shared) }
  }

  #[inline]
  fn context(&self) -> Tagged<Context> {
    // SAFETY: Tagged 包装合法 JSFunction 堆对象指针
    unsafe { (*self.as_ptr()).context() }
  }

  #[inline]
  fn set_context(&mut self, context: Tagged<Context>) {
    // SAFETY: Tagged 包装合法可变 JSFunction 堆对象指针
    unsafe { (*self.as_mut_ptr()).set_context(context) }
  }

  #[inline]
  fn feedback_cell(&self) -> Tagged<Object> {
    // SAFETY: Tagged 包装合法 JSFunction 堆对象指针
    unsafe { (*self.as_ptr()).feedback_cell() }
  }

  #[inline]
  fn set_feedback_cell(&mut self, cell: Tagged<Object>) {
    // SAFETY: Tagged 包装合法可变 JSFunction 堆对象指针
    unsafe { (*self.as_mut_ptr()).set_feedback_cell(cell) }
  }

  #[inline]
  fn has_prototype_slot(&self) -> bool {
    // SAFETY: Tagged 包装合法 JSFunction 堆对象指针
    let map = unsafe { (*self.as_ptr()).map() };
    if map.is_null() {
      return false;
    }
    // SAFETY: map 经 is_null 校验且为有效 Map 对象
    let map_ref = unsafe { &*map.as_ptr() };
    !is_js_function_without_prototype(map_ref.instance_type)
  }

  fn prototype_or_initial_map(&self) -> Result<Tagged<Object>> {
    if !self.has_prototype_slot() {
      return Err(Error::NoPrototypeSlot);
    }
    let slot_addr = self.address() + K_PROTOTYPE_OR_INITIAL_MAP_OFFSET;
    let slot_ptr = slot_addr as *const Tagged<Object>;
    // SAFETY: has_prototype_slot 校验确保 slot_addr 位于有效分配的原型槽位
    unsafe { Ok(ptr::read(slot_ptr)) }
  }

  fn set_prototype_or_initial_map(&mut self, val: Tagged<Object>) -> Result<()> {
    if !self.has_prototype_slot() {
      return Err(Error::NoPrototypeSlot);
    }
    let slot_addr = self.address() + K_PROTOTYPE_OR_INITIAL_MAP_OFFSET;
    let slot_ptr = slot_addr as *mut Tagged<Object>;
    // SAFETY: has_prototype_slot 校验确保 slot_addr 位于有效分配的原型槽位
    unsafe { ptr::write(slot_ptr, val) };
    Ok(())
  }
}

/// 拥有独立连续内存的 JSFunction 实例容器（支持 prototype 槽位与对象内属性）
pub struct JSFunctionBuf {
  _storage: Vec<u64>,
  tagged: Tagged<JSFunction>,
  has_prototype_slot: bool,
  in_object_count: usize,
}

impl JSFunctionBuf {
  /// 创建新的 JSFunctionBuf 实例
  pub fn new(
    function: JSFunction,
    has_prototype_slot: bool,
    in_object_count: usize,
  ) -> Result<Self> {
    if in_object_count > K_MAX_IN_OBJECT_PROPERTIES {
      return Err(Error::InvalidCapacity(in_object_count));
    }
    let base_header_size = if has_prototype_slot {
      K_WITH_PROTOTYPE_HEADER_SIZE
    } else {
      K_HEADER_SIZE
    };
    let total_bytes = base_header_size + in_object_count * mem::size_of::<usize>();
    let words = total_bytes.div_ceil(8);
    let mut storage = vec![0u64; words];
    let base = storage.as_mut_ptr() as usize;

    // SAFETY: base 指向有效分配且 8 字节对齐的内存首地址
    unsafe {
      ptr::write(base as *mut JSFunction, function);
    }

    Ok(Self {
      _storage: storage,
      // SAFETY: base 为有效分配的 JSFunction 内存首地址
      tagged: unsafe { Tagged::from_raw_address(base) },
      has_prototype_slot,
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
  pub const fn as_tagged(&self) -> Tagged<JSFunction> {
    self.tagged
  }
}

impl Deref for JSFunctionBuf {
  type Target = Tagged<JSFunction>;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.tagged
  }
}

impl DerefMut for JSFunctionBuf {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.tagged
  }
}

impl JSReceiverExt for JSFunctionBuf {
  #[inline]
  fn map(&self) -> Tagged<Map> {
    self.tagged.as_receiver().map()
  }

  #[inline]
  fn set_map(&mut self, map: Tagged<Map>) {
    self.tagged.as_receiver().set_map(map);
  }

  #[inline]
  fn raw_properties_or_hash(&self) -> Tagged<Object> {
    self.tagged.as_receiver().raw_properties_or_hash()
  }

  #[inline]
  fn set_raw_properties_or_hash(&mut self, value: Tagged<Object>) {
    self.tagged.as_receiver().set_raw_properties_or_hash(value);
  }

  #[inline]
  fn has_fast_properties(&self) -> bool {
    self.tagged.as_receiver().has_fast_properties()
  }

  #[inline]
  fn property_array(&self) -> Option<Tagged<v8_objects_maps::PropertyArray>> {
    self.tagged.as_receiver().property_array()
  }

  #[inline]
  fn get_prototype(&self) -> Tagged<Object> {
    self.tagged.as_receiver().get_prototype()
  }

  #[inline]
  fn has_in_prototype_chain(&self, proto: Tagged<Object>) -> bool {
    self.tagged.as_receiver().has_in_prototype_chain(proto)
  }

  #[inline]
  fn get_identity_hash(&self) -> Option<i32> {
    self.tagged.as_receiver().get_identity_hash()
  }

  #[inline]
  fn set_identity_hash(&mut self, hash: i32) -> v8_objects_js_core::Result<()> {
    self.tagged.as_receiver().set_identity_hash(hash)
  }
}

impl JSObjectExt for JSFunctionBuf {
  #[inline]
  fn elements(&self) -> Tagged<FixedArrayBase> {
    self.tagged.as_object().elements()
  }

  #[inline]
  fn set_elements(&mut self, value: Tagged<FixedArrayBase>) {
    self.tagged.as_object().set_elements(value);
  }

  #[inline]
  fn raw_fast_property_at(&self, index: FieldIndex) -> v8_objects_js_core::Result<Tagged<Object>> {
    self.tagged.as_object().raw_fast_property_at(index)
  }

  #[inline]
  fn fast_property_at_put(
    &mut self,
    index: FieldIndex,
    value: Tagged<Object>,
  ) -> v8_objects_js_core::Result<()> {
    self.tagged.as_object().fast_property_at_put(index, value)
  }
}

impl JSFunctionExt for JSFunctionBuf {
  #[inline]
  fn dispatch_handle(&self) -> u32 {
    self.tagged.dispatch_handle()
  }

  #[inline]
  fn set_dispatch_handle(&mut self, handle: u32) {
    self.tagged.set_dispatch_handle(handle);
  }

  #[inline]
  fn shared(&self) -> Tagged<SharedFunctionInfo> {
    self.tagged.shared()
  }

  #[inline]
  fn set_shared(&mut self, shared: Tagged<SharedFunctionInfo>) {
    self.tagged.set_shared(shared);
  }

  #[inline]
  fn context(&self) -> Tagged<Context> {
    self.tagged.context()
  }

  #[inline]
  fn set_context(&mut self, context: Tagged<Context>) {
    self.tagged.set_context(context);
  }

  #[inline]
  fn feedback_cell(&self) -> Tagged<Object> {
    self.tagged.feedback_cell()
  }

  #[inline]
  fn set_feedback_cell(&mut self, cell: Tagged<Object>) {
    self.tagged.set_feedback_cell(cell);
  }

  #[inline]
  fn has_prototype_slot(&self) -> bool {
    self.has_prototype_slot
  }

  fn prototype_or_initial_map(&self) -> Result<Tagged<Object>> {
    if !self.has_prototype_slot {
      return Err(Error::NoPrototypeSlot);
    }
    let slot_addr = self.tagged.address() + K_PROTOTYPE_OR_INITIAL_MAP_OFFSET;
    let slot_ptr = slot_addr as *const Tagged<Object>;
    // SAFETY: has_prototype_slot 校验确保 slot_addr 位于有效分配的原型槽位
    unsafe { Ok(ptr::read(slot_ptr)) }
  }

  fn set_prototype_or_initial_map(&mut self, val: Tagged<Object>) -> Result<()> {
    if !self.has_prototype_slot {
      return Err(Error::NoPrototypeSlot);
    }
    let slot_addr = self.tagged.address() + K_PROTOTYPE_OR_INITIAL_MAP_OFFSET;
    let slot_ptr = slot_addr as *mut Tagged<Object>;
    // SAFETY: has_prototype_slot 校验确保 slot_addr 位于有效分配的原型槽位
    unsafe { ptr::write(slot_ptr, val) };
    Ok(())
  }
}
