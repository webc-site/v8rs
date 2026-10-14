// v8-objects-js-core/src/js_object.rs
// 对应原版 v8/src/objects/js-objects.h + js-objects-inl.h + js-objects.cc

//! JSObject: JavaScript 对象实现，包含 elements 槽位、快属性与字典属性寻址、
//! 对象内属性（in-object properties）与外部 PropertyArray 的统一调度。

use std::{
  mem,
  ops::{Deref, DerefMut},
  ptr,
  sync::atomic::{AtomicUsize, Ordering},
};

use v8_objects_arrays::fixed_array_base::FixedArrayBase;
use v8_objects_core::{
  map::Map,
  property_details::PropertyDetails,
  tagged::{Object, Tagged},
};
use v8_objects_maps::{PropertyArray, PropertyArrayExt};

use crate::{
  error::{Error, Result},
  field_index::FieldIndex,
  js_array::K_FAST_ELEMENTS_KIND_MAX,
  js_receiver::{JSReceiver, JSReceiverExt, K_HEADER_SIZE as K_JS_RECEIVER_HEADER_SIZE},
};

/// elements_ 槽位偏移量
pub const K_ELEMENTS_OFFSET: usize = K_JS_RECEIVER_HEADER_SIZE;

/// JSObject 头部大小 (HeapObject + properties_or_hash + elements = 3 words)
pub const K_HEADER_SIZE: usize = mem::size_of::<JSObject>();

/// 强引用字段结束偏移
pub const K_END_OF_STRONG_FIELDS_OFFSET: usize = K_HEADER_SIZE;

/// 最大元素容量
pub const K_MAX_ELEMENT_COUNT: u32 = u32::MAX;

/// 最大元素索引
pub const K_MAX_ELEMENT_INDEX: u32 = K_MAX_ELEMENT_COUNT - 1;

/// 允许引入的最大元素间隙
pub const K_MAX_GAP: u32 = 1024;

/// 允许免密度检查扩展的快元素数组最大长度
pub const K_MAX_UNCHECKED_FAST_ELEMENTS_LENGTH: usize = 5000;

/// 老生代快元素数组最大免检长度
pub const K_MAX_UNCHECKED_OLD_FAST_ELEMENTS_LENGTH: usize = 500;

/// 全局 Object 初始未使用属性字段数
pub const K_INITIAL_GLOBAL_OBJECT_UNUSED_PROPERTIES_COUNT: usize = 4;

/// 最大实例大小（以字节计，255 words）
pub const K_MAX_INSTANCE_SIZE: usize = 255 * mem::size_of::<usize>();

/// 共享数组元素类型 (原版 elements-kind.h SHARED_ARRAY_ELEMENTS = 12)
pub const K_SHARED_ARRAY_ELEMENTS_KIND: u8 = 12;

/// 字典元素类型 (原版 elements-kind.h DICTIONARY_ELEMENTS = 13)
pub const K_DICTIONARY_ELEMENTS_KIND: u8 = 13;

/// 属性存储扩容步长
pub const K_FIELDS_ADDED: usize = 3;

/// 最大对象内属性数量
pub const K_MAX_IN_OBJECT_PROPERTIES: usize =
  (K_MAX_INSTANCE_SIZE - K_HEADER_SIZE) / mem::size_of::<usize>();

// 布局对数断言
const _: () = assert!(K_ELEMENTS_OFFSET == 2 * mem::size_of::<usize>());
const _: () = assert!(K_HEADER_SIZE == 3 * mem::size_of::<usize>());
const _: () = assert!(K_END_OF_STRONG_FIELDS_OFFSET == 3 * mem::size_of::<usize>());
const _: () = assert!(K_MAX_IN_OBJECT_PROPERTIES == 252);

/// JSObject 结构体
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct JSObject {
  /// JSReceiver 基类部分（包含 map 与 properties_or_hash）
  pub receiver: JSReceiver,
  /// elements_ 槽位
  pub elements: Tagged<FixedArrayBase>,
}

impl Deref for JSObject {
  type Target = JSReceiver;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.receiver
  }
}

impl DerefMut for JSObject {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.receiver
  }
}

impl JSObject {
  pub const K_HEADER_SIZE: usize = K_HEADER_SIZE;
  pub const K_ELEMENTS_OFFSET: usize = K_ELEMENTS_OFFSET;

  /// 创建新的 JSObject 实例
  #[inline]
  pub const fn new(
    map: Tagged<Map>,
    properties_or_hash: Tagged<Object>,
    elements: Tagged<FixedArrayBase>,
  ) -> Self {
    Self {
      receiver: JSReceiver::new(map, properties_or_hash),
      elements,
    }
  }

  /// elements 原子槽位指针
  #[inline]
  fn atomic_elements(&self) -> &AtomicUsize {
    // SAFETY: Tagged<FixedArrayBase> 与 AtomicUsize 在当前平台具有相同的大小和对齐
    unsafe { &*(&self.elements as *const Tagged<FixedArrayBase> as *const AtomicUsize) }
  }
}

/// JSObject 访问与操作扩展 trait
pub trait JSObjectExt: JSReceiverExt {
  /// 获取 elements
  fn elements(&self) -> Tagged<FixedArrayBase>;

  /// 设置 elements
  fn set_elements(&mut self, value: Tagged<FixedArrayBase>);

  /// 是否为字典属性模式
  fn has_dictionary_properties(&self) -> bool {
    !self.has_fast_properties()
  }

  /// 是否为快速元素模式 (PACKED_SMI ..= HOLEY_DOUBLE)
  fn has_fast_elements(&self) -> bool {
    // SAFETY: self.map() 必定指向合法的 Map 对象
    let map = unsafe { self.map().as_ref() };
    map.elements_kind() <= K_FAST_ELEMENTS_KIND_MAX
  }

  /// 是否为紧凑快速元素模式 (PACKED_SMI, PACKED_ELEMENTS, PACKED_DOUBLE)
  fn has_fast_packed_elements(&self) -> bool {
    // SAFETY: self.map() 必定指向合法的 Map 对象
    let map = unsafe { self.map().as_ref() };
    let kind = map.elements_kind();
    kind <= K_FAST_ELEMENTS_KIND_MAX && (kind % 2 == 0)
  }

  /// 是否为含空洞的元素模式 (HOLEY_*)
  fn has_holey_elements(&self) -> bool {
    // SAFETY: self.map() 必定指向合法的 Map 对象
    let map = unsafe { self.map().as_ref() };
    let kind = map.elements_kind();
    matches!(kind, 1 | 3 | 5 | 7 | 9 | 11)
  }

  /// 是否为字典元素模式 (原版 elements-kind.h IsDictionaryElementsKind)
  fn has_dictionary_elements(&self) -> bool {
    // SAFETY: self.map() 必定指向合法的 Map 对象
    let map = unsafe { self.map().as_ref() };
    map.elements_kind() == K_DICTIONARY_ELEMENTS_KIND
  }

  /// 快速属性读取（统一寻址对象内与外部 PropertyArray）
  fn raw_fast_property_at(&self, index: FieldIndex) -> Result<Tagged<Object>>;

  /// 快速属性写入（统一寻址对象内与外部 PropertyArray）
  fn fast_property_at_put(&mut self, index: FieldIndex, value: Tagged<Object>) -> Result<()>;

  /// 根据 PropertyDetails 读取属性值
  fn get_property_by_details(&self, details: PropertyDetails) -> Result<Tagged<Object>> {
    let index = FieldIndex::for_details(self.map(), details);
    self.raw_fast_property_at(index)
  }

  /// 根据 PropertyDetails 写入属性值
  fn set_property_by_details(
    &mut self,
    details: PropertyDetails,
    value: Tagged<Object>,
  ) -> Result<()> {
    let index = FieldIndex::for_details(self.map(), details);
    self.fast_property_at_put(index, value)
  }
}

impl JSReceiverExt for JSObject {
  #[inline]
  fn map(&self) -> Tagged<Map> {
    self.receiver.map()
  }

  #[inline]
  fn set_map(&mut self, map: Tagged<Map>) {
    self.receiver.set_map(map);
  }

  #[inline]
  fn raw_properties_or_hash(&self) -> Tagged<Object> {
    self.receiver.raw_properties_or_hash()
  }

  #[inline]
  fn set_raw_properties_or_hash(&mut self, value: Tagged<Object>) {
    self.receiver.set_raw_properties_or_hash(value);
  }

  #[inline]
  fn has_fast_properties(&self) -> bool {
    self.receiver.has_fast_properties()
  }

  #[inline]
  fn property_array(&self) -> Option<Tagged<PropertyArray>> {
    self.receiver.property_array()
  }

  #[inline]
  fn get_prototype(&self) -> Tagged<Object> {
    self.receiver.get_prototype()
  }

  #[inline]
  fn has_in_prototype_chain(&self, proto: Tagged<Object>) -> bool {
    self.receiver.has_in_prototype_chain(proto)
  }

  #[inline]
  fn get_identity_hash(&self) -> Option<i32> {
    self.receiver.get_identity_hash()
  }

  #[inline]
  fn set_identity_hash(&mut self, hash: i32) -> Result<()> {
    self.receiver.set_identity_hash(hash)
  }
}

impl JSObjectExt for JSObject {
  #[inline]
  fn elements(&self) -> Tagged<FixedArrayBase> {
    Tagged::new(self.atomic_elements().load(Ordering::Relaxed))
  }

  #[inline]
  fn set_elements(&mut self, value: Tagged<FixedArrayBase>) {
    self.atomic_elements().store(value.ptr(), Ordering::Relaxed);
  }

  fn raw_fast_property_at(&self, index: FieldIndex) -> Result<Tagged<Object>> {
    if index.is_inobject() {
      let offset = index.offset();
      // SAFETY: self.map() 必定指向合法的 Map 对象
      let map = unsafe { self.map().as_ref() };
      let instance_size = (map.instance_size_in_words as usize) * mem::size_of::<usize>();
      if instance_size == 0 || offset + mem::size_of::<usize>() > instance_size {
        return Err(Error::IndexOutOfBounds {
          index: offset,
          len: instance_size,
        });
      }
      let base_addr = self as *const Self as usize;
      let slot_ptr = (base_addr + offset) as *const AtomicUsize;
      // SAFETY: offset 已通过 instance_size 严格边界检查，内存安全且对齐
      let val = unsafe { (*slot_ptr).load(Ordering::Relaxed) };
      Ok(Tagged::new(val))
    } else {
      let prop_arr = self.property_array().ok_or(Error::NotFastObject)?;
      let val = prop_arr.get(index.outobject_array_index())?;
      Ok(val)
    }
  }

  fn fast_property_at_put(&mut self, index: FieldIndex, value: Tagged<Object>) -> Result<()> {
    if index.is_inobject() {
      let offset = index.offset();
      // SAFETY: self.map() 必定指向合法的 Map 对象
      let map = unsafe { self.map().as_ref() };
      let instance_size = (map.instance_size_in_words as usize) * mem::size_of::<usize>();
      if instance_size == 0 || offset + mem::size_of::<usize>() > instance_size {
        return Err(Error::IndexOutOfBounds {
          index: offset,
          len: instance_size,
        });
      }
      let base_addr = self as *mut Self as usize;
      let slot_ptr = (base_addr + offset) as *mut AtomicUsize;
      // SAFETY: offset 已通过 instance_size 严格边界检查，内存安全且对齐
      unsafe { (*slot_ptr).store(value.ptr(), Ordering::Relaxed) };
      Ok(())
    } else {
      let prop_arr = self.property_array().ok_or(Error::NotFastObject)?;
      prop_arr.set(index.outobject_array_index(), value)?;
      Ok(())
    }
  }
}

impl JSReceiverExt for Tagged<JSObject> {
  #[inline]
  fn map(&self) -> Tagged<Map> {
    // SAFETY: Tagged 包装合法 JSObject 指针
    unsafe { (*self.as_ptr()).map() }
  }

  #[inline]
  fn set_map(&mut self, map: Tagged<Map>) {
    // SAFETY: Tagged 包装合法可变 JSObject 指针
    unsafe { (*self.as_mut_ptr()).set_map(map) }
  }

  #[inline]
  fn raw_properties_or_hash(&self) -> Tagged<Object> {
    // SAFETY: Tagged 包装合法 JSObject 指针
    unsafe { (*self.as_ptr()).raw_properties_or_hash() }
  }

  #[inline]
  fn set_raw_properties_or_hash(&mut self, value: Tagged<Object>) {
    // SAFETY: Tagged 包装合法可变 JSObject 指针
    unsafe { (*self.as_mut_ptr()).set_raw_properties_or_hash(value) }
  }

  #[inline]
  fn has_fast_properties(&self) -> bool {
    // SAFETY: Tagged 包装合法 JSObject 指针
    unsafe { (*self.as_ptr()).has_fast_properties() }
  }

  #[inline]
  fn property_array(&self) -> Option<Tagged<PropertyArray>> {
    // SAFETY: Tagged 包装合法 JSObject 指针
    unsafe { (*self.as_ptr()).property_array() }
  }

  #[inline]
  fn get_prototype(&self) -> Tagged<Object> {
    // SAFETY: Tagged 包装合法 JSObject 指针
    unsafe { (*self.as_ptr()).get_prototype() }
  }

  #[inline]
  fn has_in_prototype_chain(&self, proto: Tagged<Object>) -> bool {
    // SAFETY: Tagged 包装合法 JSObject 指针
    unsafe { (*self.as_ptr()).has_in_prototype_chain(proto) }
  }

  #[inline]
  fn get_identity_hash(&self) -> Option<i32> {
    // SAFETY: Tagged 包装合法 JSObject 指针
    unsafe { (*self.as_ptr()).get_identity_hash() }
  }

  #[inline]
  fn set_identity_hash(&mut self, hash: i32) -> Result<()> {
    // SAFETY: Tagged 包装合法可变 JSObject 指针
    unsafe { (*self.as_mut_ptr()).set_identity_hash(hash) }
  }
}

impl JSObjectExt for Tagged<JSObject> {
  #[inline]
  fn elements(&self) -> Tagged<FixedArrayBase> {
    // SAFETY: Tagged 包装合法 JSObject 指针
    unsafe { (*self.as_ptr()).elements() }
  }

  #[inline]
  fn set_elements(&mut self, value: Tagged<FixedArrayBase>) {
    // SAFETY: Tagged 包装合法可变 JSObject 指针
    unsafe { (*self.as_mut_ptr()).set_elements(value) }
  }

  #[inline]
  fn has_fast_elements(&self) -> bool {
    // SAFETY: Tagged 包装合法 JSObject 指针
    unsafe { (*self.as_ptr()).has_fast_elements() }
  }

  #[inline]
  fn has_fast_packed_elements(&self) -> bool {
    // SAFETY: Tagged 包装合法 JSObject 指针
    unsafe { (*self.as_ptr()).has_fast_packed_elements() }
  }

  #[inline]
  fn has_holey_elements(&self) -> bool {
    // SAFETY: Tagged 包装合法 JSObject 指针
    unsafe { (*self.as_ptr()).has_holey_elements() }
  }

  #[inline]
  fn has_dictionary_elements(&self) -> bool {
    // SAFETY: Tagged 包装合法 JSObject 指针
    unsafe { (*self.as_ptr()).has_dictionary_elements() }
  }

  #[inline]
  fn raw_fast_property_at(&self, index: FieldIndex) -> Result<Tagged<Object>> {
    // SAFETY: Tagged 包装合法 JSObject 指针
    unsafe { (*self.as_ptr()).raw_fast_property_at(index) }
  }

  #[inline]
  fn fast_property_at_put(&mut self, index: FieldIndex, value: Tagged<Object>) -> Result<()> {
    // SAFETY: Tagged 包装合法可变 JSObject 指针
    unsafe { (*self.as_mut_ptr()).fast_property_at_put(index, value) }
  }
}

/// 拥有独立连续内存的 JSObject 实例容器（支持对象内属性）
pub struct JSObjectBuf {
  _storage: Vec<u64>,
  tagged: Tagged<JSObject>,
  in_object_count: usize,
}

impl JSObjectBuf {
  /// 创建包含指定数量对象内属性槽位的 JSObjectBuf
  pub fn new(
    map: Tagged<Map>,
    properties_or_hash: Tagged<Object>,
    elements: Tagged<FixedArrayBase>,
    in_object_count: usize,
  ) -> Result<Self> {
    if in_object_count > K_MAX_IN_OBJECT_PROPERTIES {
      return Err(Error::InvalidCapacity(in_object_count));
    }
    let total_bytes = K_HEADER_SIZE + in_object_count * mem::size_of::<usize>();
    let words = total_bytes.div_ceil(8);
    let mut storage = vec![0u64; words];
    let base = storage.as_mut_ptr() as usize;

    let header = JSObject::new(map, properties_or_hash, elements);
    // SAFETY: base 指向刚刚分配并具备充足容量及 8 字节对齐的 Vec<u64> 首地址
    unsafe {
      ptr::write(base as *mut JSObject, header);
    }

    Ok(Self {
      _storage: storage,
      // SAFETY: base 为有效分配的 JSObject 内存块首地址
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
  pub fn as_tagged(&self) -> Tagged<JSObject> {
    self.tagged
  }
}

impl Deref for JSObjectBuf {
  type Target = Tagged<JSObject>;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.tagged
  }
}

impl DerefMut for JSObjectBuf {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.tagged
  }
}
