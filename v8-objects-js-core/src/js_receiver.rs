// v8-objects-js-core/src/js_receiver.rs
// 对应原版 v8/src/objects/js-objects.h + js-objects-inl.h

//! JSReceiver: JS 对象与 JS 代理的公共基类，支持属性定义与原型链查询。

use std::{
  mem,
  sync::atomic::{AtomicUsize, Ordering},
};

use v8_objects_core::{
  heap_object::HeapObject,
  map::Map,
  smi::Smi,
  tagged::{Object, Tagged},
};
use v8_objects_maps::{
  Error as V8ObjectsMapsError, PropertyArray, PropertyArrayExt, property_array::K_MAX_HASH,
};

use crate::error::{Error, Result};

/// `properties_or_hash` 槽位偏移量（以当前平台字节为准）
pub const K_PROPERTIES_OR_HASH_OFFSET: usize = mem::size_of::<HeapObject>();

/// JSReceiver 头部大小
pub const K_HEADER_SIZE: usize = mem::size_of::<JSReceiver>();

/// 哈希码掩码（对齐 PropertyArray::HashField::kMask）
pub const K_HASH_MASK: u32 = K_MAX_HASH;

// 布局对数断言
const _: () = assert!(K_PROPERTIES_OR_HASH_OFFSET == mem::size_of::<usize>());
const _: () = assert!(K_HEADER_SIZE == 2 * mem::size_of::<usize>());

/// JSReceiver 头结构体
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct JSReceiver {
  /// HeapObject 头部（Map 槽位）
  pub header: HeapObject,
  /// 属性数组、字典或哈希码槽位 (properties_or_hash_)
  pub properties_or_hash: Tagged<Object>,
}

impl JSReceiver {
  pub const K_HEADER_SIZE: usize = K_HEADER_SIZE;
  pub const K_PROPERTIES_OR_HASH_OFFSET: usize = K_PROPERTIES_OR_HASH_OFFSET;

  /// 创建新的 JSReceiver 实例
  #[inline]
  pub const fn new(map: Tagged<Map>, properties_or_hash: Tagged<Object>) -> Self {
    Self {
      header: HeapObject::new(map),
      properties_or_hash,
    }
  }

  /// 获取槽位内存原子指针
  #[inline]
  fn atomic_properties_or_hash(&self) -> &AtomicUsize {
    // SAFETY: Tagged<Object> 与 AtomicUsize 在当前平台均具相同的字长和对齐约束，指针强转安全
    unsafe { &*(&self.properties_or_hash as *const Tagged<Object> as *const AtomicUsize) }
  }
}

/// JSReceiver 操作扩展 trait
pub trait JSReceiverExt {
  /// 获取 Map
  fn map(&self) -> Tagged<Map>;

  /// 设置 Map
  fn set_map(&mut self, map: Tagged<Map>);

  /// 读取原始 `properties_or_hash`
  fn raw_properties_or_hash(&self) -> Tagged<Object>;

  /// 写入原始 `properties_or_hash`
  fn set_raw_properties_or_hash(&mut self, value: Tagged<Object>);

  /// 是否为快属性模式
  fn has_fast_properties(&self) -> bool;

  /// 获取快属性存储 PropertyArray
  fn property_array(&self) -> Option<Tagged<PropertyArray>>;

  /// 获取自身原型对象
  fn get_prototype(&self) -> Tagged<Object>;

  /// 检查目标对象是否在原型链中
  fn has_in_prototype_chain(&self, proto: Tagged<Object>) -> bool;

  /// 读取对象的身份哈希（IdentityHash）
  fn get_identity_hash(&self) -> Option<i32>;

  /// 设置对象的身份哈希（IdentityHash）
  fn set_identity_hash(&mut self, hash: i32) -> Result<()>;
}

impl JSReceiverExt for JSReceiver {
  #[inline]
  fn map(&self) -> Tagged<Map> {
    self.header.map()
  }

  #[inline]
  fn set_map(&mut self, map: Tagged<Map>) {
    self.header.set_map(map);
  }

  #[inline]
  fn raw_properties_or_hash(&self) -> Tagged<Object> {
    Tagged::new(self.atomic_properties_or_hash().load(Ordering::Relaxed))
  }

  #[inline]
  fn set_raw_properties_or_hash(&mut self, value: Tagged<Object>) {
    self
      .atomic_properties_or_hash()
      .store(value.ptr(), Ordering::Relaxed);
  }

  #[inline]
  fn has_fast_properties(&self) -> bool {
    // SAFETY: self.map() 必定指向合法的 Map 对象
    let map = unsafe { self.map().as_ref() };
    !map.is_dictionary_map()
  }

  #[inline]
  fn property_array(&self) -> Option<Tagged<PropertyArray>> {
    if !self.has_fast_properties() {
      return None;
    }
    let raw = self.raw_properties_or_hash();
    if raw.is_smi() {
      return None;
    }
    // 非 Smi 且为快模式时转换为 PropertyArray
    Some(Tagged::new(raw.ptr()))
  }

  #[inline]
  fn get_prototype(&self) -> Tagged<Object> {
    // SAFETY: self.map() 必定指向合法的 Map 对象
    let map = unsafe { self.map().as_ref() };
    map.prototype
  }

  fn has_in_prototype_chain(&self, proto: Tagged<Object>) -> bool {
    const MAX_PROTOTYPE_DEPTH: usize = 1024;
    let mut current = self.get_prototype();
    let mut depth = 0;
    while !current.is_null() && depth < MAX_PROTOTYPE_DEPTH {
      if current == proto {
        return true;
      }
      if current.is_smi() {
        break;
      }
      // SAFETY: current 经判定非 null 且非 Smi，作为堆对象安全解引用
      let heap_obj = unsafe { Tagged::<HeapObject>::new(current.ptr()).as_ref() };
      // SAFETY: HeapObject::map 必定指向有效的 Map 对象
      let map = unsafe { heap_obj.map().as_ref() };
      current = map.prototype;
      depth += 1;
    }
    false
  }

  fn get_identity_hash(&self) -> Option<i32> {
    let raw = self.raw_properties_or_hash();
    if raw.is_smi() {
      let smi = Tagged::<Smi>::new(raw.ptr());
      let val = smi.value();
      if val == 0 { None } else { Some(val) }
    } else if let Some(prop_arr) = self.property_array() {
      let h = prop_arr.hash();
      if h == 0 { None } else { Some(h) }
    } else {
      None
    }
  }

  fn set_identity_hash(&mut self, hash: i32) -> Result<()> {
    // 原版规定 DCHECK_NE(PropertyArray::kNoHashSentinel, hash)，0 代表哨兵值非有效哈希
    if hash <= 0 || (hash as u32) > K_HASH_MASK {
      return Err(Error::ObjectsMaps(V8ObjectsMapsError::HashOverflow(
        hash as u32,
      )));
    }
    let raw = self.raw_properties_or_hash();
    if raw.is_smi() {
      self.set_raw_properties_or_hash(Tagged::new(Smi::from_int(hash).ptr()));
      Ok(())
    } else if let Some(prop_arr) = self.property_array() {
      prop_arr.set_hash(hash)?;
      Ok(())
    } else {
      // 字典或其他模式暂存为 Smi
      self.set_raw_properties_or_hash(Tagged::new(Smi::from_int(hash).ptr()));
      Ok(())
    }
  }
}

impl JSReceiverExt for Tagged<JSReceiver> {
  #[inline]
  fn map(&self) -> Tagged<Map> {
    // SAFETY: Tagged 包装合法 JSReceiver 指针
    unsafe { (*self.as_ptr()).map() }
  }

  #[inline]
  fn set_map(&mut self, map: Tagged<Map>) {
    // SAFETY: Tagged 包装合法可变 JSReceiver 指针
    unsafe { (*self.as_mut_ptr()).set_map(map) }
  }

  #[inline]
  fn raw_properties_or_hash(&self) -> Tagged<Object> {
    // SAFETY: Tagged 包装合法 JSReceiver 指针
    unsafe { (*self.as_ptr()).raw_properties_or_hash() }
  }

  #[inline]
  fn set_raw_properties_or_hash(&mut self, value: Tagged<Object>) {
    // SAFETY: Tagged 包装合法可变 JSReceiver 指针
    unsafe { (*self.as_mut_ptr()).set_raw_properties_or_hash(value) }
  }

  #[inline]
  fn has_fast_properties(&self) -> bool {
    // SAFETY: Tagged 包装合法 JSReceiver 指针
    unsafe { (*self.as_ptr()).has_fast_properties() }
  }

  #[inline]
  fn property_array(&self) -> Option<Tagged<PropertyArray>> {
    // SAFETY: Tagged 包装合法 JSReceiver 指针
    unsafe { (*self.as_ptr()).property_array() }
  }

  #[inline]
  fn get_prototype(&self) -> Tagged<Object> {
    // SAFETY: Tagged 包装合法 JSReceiver 指针
    unsafe { (*self.as_ptr()).get_prototype() }
  }

  #[inline]
  fn has_in_prototype_chain(&self, proto: Tagged<Object>) -> bool {
    // SAFETY: Tagged 包装合法 JSReceiver 指针
    unsafe { (*self.as_ptr()).has_in_prototype_chain(proto) }
  }

  #[inline]
  fn get_identity_hash(&self) -> Option<i32> {
    // SAFETY: Tagged 包装合法 JSReceiver 指针
    unsafe { (*self.as_ptr()).get_identity_hash() }
  }

  #[inline]
  fn set_identity_hash(&mut self, hash: i32) -> Result<()> {
    // SAFETY: Tagged 包装合法可变 JSReceiver 指针
    unsafe { (*self.as_mut_ptr()).set_identity_hash(hash) }
  }
}
