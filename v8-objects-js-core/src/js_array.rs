// v8-objects-js-core/src/js_array.rs
// 对应原版 v8/src/objects/js-array.h + js-array-inl.h

//! JSArray: JavaScript 数组对象实现，管理 length 字段、快慢数组判定及元素访问。

use std::{
  mem,
  ops::{Deref, DerefMut},
  ptr,
  sync::atomic::{AtomicUsize, Ordering},
};

use v8_objects_arrays::fixed_array_base::FixedArrayBase;
use v8_objects_core::{
  heap_number::HeapNumber,
  map::Map,
  smi::Smi,
  tagged::{Object, Tagged},
};
use v8_objects_maps::PropertyArray;

use crate::{
  error::{Error, Result},
  field_index::FieldIndex,
  js_object::{
    JSObject, JSObjectExt, K_HEADER_SIZE as K_JS_OBJECT_HEADER_SIZE, K_MAX_INSTANCE_SIZE,
  },
  js_receiver::JSReceiverExt,
};

/// length 字段偏移量
pub const K_LENGTH_OFFSET: usize = K_JS_OBJECT_HEADER_SIZE;

/// JSArray 头部大小
pub const K_HEADER_SIZE: usize = mem::size_of::<JSArray>();

/// JSArray 最大对象内属性数量 (最大实例 255 words 减去 4 words 头部 = 251)
pub const K_MAX_IN_OBJECT_PROPERTIES: usize =
  (K_MAX_INSTANCE_SIZE - K_HEADER_SIZE) / mem::size_of::<usize>();

/// 空数组预分配元素个数
pub const K_PREALLOCATED_ARRAY_ELEMENTS: usize = 4;

/// length 属性描述符索引
pub const K_LENGTH_DESCRIPTOR_INDEX: usize = 0;

/// 数组内置函数复制元素阈值
pub const K_MAX_COPY_ELEMENTS: usize = 100;

/// 允许内联排序的最大数组长度
pub const K_MAX_INLINE_SORT_LENGTH: usize = 16;

/// 数组最大长度
pub const K_MAX_ARRAY_LENGTH: u32 = u32::MAX;

/// 数组最大元素下标
pub const K_MAX_ARRAY_INDEX: u32 = K_MAX_ARRAY_LENGTH - 1;

/// 快数组最大长度阈值（超过则需慢速化/字典化）
pub const K_MAX_FAST_ARRAY_LENGTH: u32 = 32 * 1024 * 1024;

/// 快速元素类型上限 (PACKED_SMI = 0, ..., HOLEY_DOUBLE = 5)
pub const K_FAST_ELEMENTS_KIND_MAX: u8 = 5;

/// join 检测环的最小栈深度
pub const K_MIN_JOIN_STACK_SIZE: u32 = 2;

// 布局对数断言
const _: () = assert!(K_LENGTH_OFFSET == 3 * mem::size_of::<usize>());
const _: () = assert!(K_HEADER_SIZE == 4 * mem::size_of::<usize>());
const _: () = assert!(K_MAX_IN_OBJECT_PROPERTIES == 251);

/// JSArray 头结构体
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct JSArray {
  /// JSObject 基类部分
  pub object: JSObject,
  /// 数组长度字段 (length_)，存放 Smi 或 HeapNumber
  pub length: Tagged<Object>,
}

impl Deref for JSArray {
  type Target = JSObject;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.object
  }
}

impl DerefMut for JSArray {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.object
  }
}

impl JSArray {
  pub const K_HEADER_SIZE: usize = K_HEADER_SIZE;
  pub const K_LENGTH_OFFSET: usize = K_LENGTH_OFFSET;
  pub const K_MAX_IN_OBJECT_PROPERTIES: usize = K_MAX_IN_OBJECT_PROPERTIES;

  /// 创建新的 JSArray 实例
  #[inline]
  pub const fn new(
    map: Tagged<Map>,
    properties_or_hash: Tagged<Object>,
    elements: Tagged<FixedArrayBase>,
    length: Tagged<Object>,
  ) -> Self {
    Self {
      object: JSObject::new(map, properties_or_hash, elements),
      length,
    }
  }

  /// length 原子槽位指针
  #[inline]
  fn atomic_length(&self) -> &AtomicUsize {
    // SAFETY: Tagged<Object> 与 AtomicUsize 在当前平台具有相同的字长和对齐
    unsafe { &*(&self.length as *const Tagged<Object> as *const AtomicUsize) }
  }
}

/// JSArray 专属操作扩展 trait
pub trait JSArrayExt: JSObjectExt {
  /// 读取原始 length 槽位
  fn length(&self) -> Tagged<Object>;

  /// 写入原始 length 槽位
  fn set_length(&mut self, value: Tagged<Object>);

  /// 以 u32 解析数组长度
  fn length_u32(&self) -> Result<u32>;

  /// 以 u32 设置数组长度（若符合 Smi 范围以 Smi 存储）
  fn set_length_u32(&mut self, len: u32) -> Result<()>;

  /// 判断是否使用快速元素模式
  fn is_fast_elements(&self) -> bool {
    self.has_fast_elements()
  }

  /// 判断扩容至新长度是否会导致规范化（转为字典模式，原版第一行判定 !HasFastElements 时直接返回 false）
  fn set_length_would_normalize(&self, new_length: u32) -> bool {
    if !self.has_fast_elements() {
      return false;
    }
    new_length > K_MAX_FAST_ARRAY_LENGTH
  }
}

impl JSReceiverExt for JSArray {
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
  fn set_identity_hash(&mut self, hash: i32) -> Result<()> {
    self.object.set_identity_hash(hash)
  }
}

impl JSObjectExt for JSArray {
  #[inline]
  fn elements(&self) -> Tagged<FixedArrayBase> {
    self.object.elements()
  }

  #[inline]
  fn set_elements(&mut self, value: Tagged<FixedArrayBase>) {
    self.object.set_elements(value);
  }

  #[inline]
  fn raw_fast_property_at(&self, index: FieldIndex) -> Result<Tagged<Object>> {
    self.object.raw_fast_property_at(index)
  }

  #[inline]
  fn fast_property_at_put(&mut self, index: FieldIndex, value: Tagged<Object>) -> Result<()> {
    self.object.fast_property_at_put(index, value)
  }
}

impl JSArrayExt for JSArray {
  #[inline]
  fn length(&self) -> Tagged<Object> {
    Tagged::new(self.atomic_length().load(Ordering::Relaxed))
  }

  #[inline]
  fn set_length(&mut self, value: Tagged<Object>) {
    self.atomic_length().store(value.ptr(), Ordering::Relaxed);
  }

  fn length_u32(&self) -> Result<u32> {
    let raw = self.length();
    if raw.is_smi() {
      let smi = Tagged::<Smi>::new(raw.ptr());
      let val = smi.value();
      if val >= 0 {
        Ok(val as u32)
      } else {
        Err(Error::InvalidCapacity(val as usize))
      }
    } else {
      // SAFETY: raw 为堆对象指针，指向有效 HeapNumber
      let hn = unsafe { Tagged::<HeapNumber>::new(raw.ptr()).as_ref() };
      let val = hn.value();
      if val >= 0.0 && val <= u32::MAX as f64 {
        Ok(val as u32)
      } else {
        Err(Error::InvalidCapacity(val as usize))
      }
    }
  }

  fn set_length_u32(&mut self, len: u32) -> Result<()> {
    if Smi::is_valid(len as i64) {
      self.set_length(Tagged::new(Smi::from_int(len as i32).ptr()));
      Ok(())
    } else {
      // 超出 Smi 范围暂不支持无 Heap 分配器的裸写入
      Err(Error::InvalidCapacity(len as usize))
    }
  }

  #[inline]
  fn is_fast_elements(&self) -> bool {
    self.has_fast_elements()
  }
}

impl JSReceiverExt for Tagged<JSArray> {
  #[inline]
  fn map(&self) -> Tagged<Map> {
    // SAFETY: Tagged 包装合法 JSArray 指针
    unsafe { (*self.as_ptr()).map() }
  }

  #[inline]
  fn set_map(&mut self, map: Tagged<Map>) {
    // SAFETY: Tagged 包装合法可变 JSArray 指针
    unsafe { (*self.as_mut_ptr()).set_map(map) }
  }

  #[inline]
  fn raw_properties_or_hash(&self) -> Tagged<Object> {
    // SAFETY: Tagged 包装合法 JSArray 指针
    unsafe { (*self.as_ptr()).raw_properties_or_hash() }
  }

  #[inline]
  fn set_raw_properties_or_hash(&mut self, value: Tagged<Object>) {
    // SAFETY: Tagged 包装合法可变 JSArray 指针
    unsafe { (*self.as_mut_ptr()).set_raw_properties_or_hash(value) }
  }

  #[inline]
  fn has_fast_properties(&self) -> bool {
    // SAFETY: Tagged 包装合法 JSArray 指针
    unsafe { (*self.as_ptr()).has_fast_properties() }
  }

  #[inline]
  fn property_array(&self) -> Option<Tagged<PropertyArray>> {
    // SAFETY: Tagged 包装合法 JSArray 指针
    unsafe { (*self.as_ptr()).property_array() }
  }

  #[inline]
  fn get_prototype(&self) -> Tagged<Object> {
    // SAFETY: Tagged 包装合法 JSArray 指针
    unsafe { (*self.as_ptr()).get_prototype() }
  }

  #[inline]
  fn has_in_prototype_chain(&self, proto: Tagged<Object>) -> bool {
    // SAFETY: Tagged 包装合法 JSArray 指针
    unsafe { (*self.as_ptr()).has_in_prototype_chain(proto) }
  }

  #[inline]
  fn get_identity_hash(&self) -> Option<i32> {
    // SAFETY: Tagged 包装合法 JSArray 指针
    unsafe { (*self.as_ptr()).get_identity_hash() }
  }

  #[inline]
  fn set_identity_hash(&mut self, hash: i32) -> Result<()> {
    // SAFETY: Tagged 包装合法可变 JSArray 指针
    unsafe { (*self.as_mut_ptr()).set_identity_hash(hash) }
  }
}

impl JSObjectExt for Tagged<JSArray> {
  #[inline]
  fn elements(&self) -> Tagged<FixedArrayBase> {
    // SAFETY: Tagged 包装合法 JSArray 指针
    unsafe { (*self.as_ptr()).elements() }
  }

  #[inline]
  fn set_elements(&mut self, value: Tagged<FixedArrayBase>) {
    // SAFETY: Tagged 包装合法可变 JSArray 指针
    unsafe { (*self.as_mut_ptr()).set_elements(value) }
  }

  #[inline]
  fn raw_fast_property_at(&self, index: FieldIndex) -> Result<Tagged<Object>> {
    // SAFETY: Tagged 包装合法 JSArray 指针
    unsafe { (*self.as_ptr()).raw_fast_property_at(index) }
  }

  #[inline]
  fn fast_property_at_put(&mut self, index: FieldIndex, value: Tagged<Object>) -> Result<()> {
    // SAFETY: Tagged 包装合法可变 JSArray 指针
    unsafe { (*self.as_mut_ptr()).fast_property_at_put(index, value) }
  }
}

impl JSArrayExt for Tagged<JSArray> {
  #[inline]
  fn length(&self) -> Tagged<Object> {
    // SAFETY: Tagged 包装合法 JSArray 指针
    unsafe { (*self.as_ptr()).length() }
  }

  #[inline]
  fn set_length(&mut self, value: Tagged<Object>) {
    // SAFETY: Tagged 包装合法可变 JSArray 指针
    unsafe { (*self.as_mut_ptr()).set_length(value) }
  }

  #[inline]
  fn length_u32(&self) -> Result<u32> {
    // SAFETY: Tagged 包装合法 JSArray 指针
    unsafe { (*self.as_ptr()).length_u32() }
  }

  #[inline]
  fn set_length_u32(&mut self, len: u32) -> Result<()> {
    // SAFETY: Tagged 包装合法可变 JSArray 指针
    unsafe { (*self.as_mut_ptr()).set_length_u32(len) }
  }

  #[inline]
  fn is_fast_elements(&self) -> bool {
    // SAFETY: Tagged 包装合法 JSArray 指针
    unsafe { (*self.as_ptr()).is_fast_elements() }
  }
}

/// 拥有独立连续内存的 JSArray 实例容器
pub struct JSArrayBuf {
  _storage: Vec<u64>,
  tagged: Tagged<JSArray>,
  in_object_count: usize,
}

impl JSArrayBuf {
  /// 创建包含指定数量对象内属性的 JSArrayBuf
  pub fn new(
    map: Tagged<Map>,
    properties_or_hash: Tagged<Object>,
    elements: Tagged<FixedArrayBase>,
    length: Tagged<Object>,
    in_object_count: usize,
  ) -> Result<Self> {
    if in_object_count > K_MAX_IN_OBJECT_PROPERTIES {
      return Err(Error::InvalidCapacity(in_object_count));
    }
    let total_bytes = K_HEADER_SIZE + in_object_count * mem::size_of::<usize>();
    let words = total_bytes.div_ceil(8);
    let mut storage = vec![0u64; words];
    let base = storage.as_mut_ptr() as usize;

    let header = JSArray::new(map, properties_or_hash, elements, length);
    // SAFETY: base 指向足额分配且 8 字节对齐的连续堆内存首地址
    unsafe {
      ptr::write(base as *mut JSArray, header);
    }

    Ok(Self {
      _storage: storage,
      // SAFETY: base 包装为 Tagged<JSArray>
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
  pub fn as_tagged(&self) -> Tagged<JSArray> {
    self.tagged
  }
}

impl Deref for JSArrayBuf {
  type Target = Tagged<JSArray>;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.tagged
  }
}

impl DerefMut for JSArrayBuf {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.tagged
  }
}
