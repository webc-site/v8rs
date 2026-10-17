// v8-objects-js-core/src/js_arguments.rs
// 对应原版 v8/src/objects/arguments.h + arguments-inl.h

//! JSArgumentsObject: 函数调用参数对象模型，支持宽松模式（Sloppy）上下文别名映射
//! 以及严格模式（Strict）不可变参数对象。

use std::{
  mem,
  ops::{Deref, DerefMut},
  ptr,
  sync::atomic::{AtomicUsize, Ordering},
};

use v8_objects_arrays::fixed_array_base::FixedArrayBase;
use v8_objects_core::{
  heap_object::HeapObject,
  map::Map,
  smi::Smi,
  tagged::{Object, Tagged},
};

use crate::{
  error::{Error, Result},
  js_object::{JSObject, K_HEADER_SIZE as K_JS_OBJECT_HEADER_SIZE},
};

/// JSArgumentsObject 头部大小 (等同于 JSObject)
pub const K_ARGUMENTS_HEADER_SIZE: usize = K_JS_OBJECT_HEADER_SIZE;

/// TheHole 哨兵（最低两位为 0b01 表示堆对象，代表已解绑或已删除槽位）
pub const K_THE_HOLE_SENTINEL: Tagged<Object> = Tagged::new(1);

/// 参数查找结果指示
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ArgumentLocation {
  /// 该形参映射到 Context 上下文槽位中（Smi 槽位序号）
  ContextSlot(i32),
  /// 该实参未被映射，存放于 arguments 数组指定下标处
  UnmappedArray(usize),
}

/// JSArgumentsObject 抽象基类头结构
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct JSArgumentsObject {
  /// JSObject 基类部分
  pub object: JSObject,
}

impl JSArgumentsObject {
  pub const K_HEADER_SIZE: usize = K_ARGUMENTS_HEADER_SIZE;
}

impl Deref for JSArgumentsObject {
  type Target = JSObject;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.object
  }
}

impl DerefMut for JSArgumentsObject {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.object
  }
}

/// 松散模式参数对象（JSSloppyArgumentsObject）形态规范
pub struct JSSloppyArgumentsObject;

impl JSSloppyArgumentsObject {
  pub const K_LENGTH_SLOT_INDEX: usize = 0;
  pub const K_CALLEE_SLOT_INDEX: usize = 1;
  pub const K_IN_OBJECT_PROPERTY_COUNT: usize = 2;

  pub const K_LENGTH_OFFSET: usize =
    K_ARGUMENTS_HEADER_SIZE + Self::K_LENGTH_SLOT_INDEX * mem::size_of::<usize>();
  pub const K_CALLEE_OFFSET: usize =
    K_ARGUMENTS_HEADER_SIZE + Self::K_CALLEE_SLOT_INDEX * mem::size_of::<usize>();
  pub const K_SIZE: usize =
    K_ARGUMENTS_HEADER_SIZE + Self::K_IN_OBJECT_PROPERTY_COUNT * mem::size_of::<usize>();

  /// 读取 length 槽位
  #[inline]
  pub fn length(object: Tagged<JSObject>) -> Tagged<Object> {
    let slot_ptr = (object.address() + Self::K_LENGTH_OFFSET) as *const AtomicUsize;
    // SAFETY: slot_ptr 位于合法分配的 JSSloppyArgumentsObject 对象内属性区间
    Tagged::new(unsafe { (*slot_ptr).load(Ordering::Relaxed) })
  }

  /// 写入 length 槽位
  #[inline]
  pub fn set_length(object: Tagged<JSObject>, value: Tagged<Object>) {
    let slot_ptr = (object.address() + Self::K_LENGTH_OFFSET) as *mut AtomicUsize;
    // SAFETY: slot_ptr 位于合法分配的 JSSloppyArgumentsObject 对象内属性区间
    unsafe { (*slot_ptr).store(value.ptr(), Ordering::Relaxed) };
  }

  /// 读取 callee 槽位
  #[inline]
  pub fn callee(object: Tagged<JSObject>) -> Tagged<Object> {
    let slot_ptr = (object.address() + Self::K_CALLEE_OFFSET) as *const AtomicUsize;
    // SAFETY: slot_ptr 位于合法分配的 JSSloppyArgumentsObject 对象内属性区间
    Tagged::new(unsafe { (*slot_ptr).load(Ordering::Relaxed) })
  }

  /// 写入 callee 槽位
  #[inline]
  pub fn set_callee(object: Tagged<JSObject>, value: Tagged<Object>) {
    let slot_ptr = (object.address() + Self::K_CALLEE_OFFSET) as *mut AtomicUsize;
    // SAFETY: slot_ptr 位于合法分配的 JSSloppyArgumentsObject 对象内属性区间
    unsafe { (*slot_ptr).store(value.ptr(), Ordering::Relaxed) };
  }
}

/// 严格模式参数对象（JSStrictArgumentsObject）形态规范
pub struct JSStrictArgumentsObject;

impl JSStrictArgumentsObject {
  pub const K_LENGTH_SLOT_INDEX: usize = 0;
  pub const K_IN_OBJECT_PROPERTY_COUNT: usize = 1;

  pub const K_LENGTH_OFFSET: usize =
    K_ARGUMENTS_HEADER_SIZE + Self::K_LENGTH_SLOT_INDEX * mem::size_of::<usize>();
  pub const K_SIZE: usize =
    K_ARGUMENTS_HEADER_SIZE + Self::K_IN_OBJECT_PROPERTY_COUNT * mem::size_of::<usize>();

  /// 读取 length 槽位
  #[inline]
  pub fn length(object: Tagged<JSObject>) -> Tagged<Object> {
    let slot_ptr = (object.address() + Self::K_LENGTH_OFFSET) as *const AtomicUsize;
    // SAFETY: slot_ptr 位于合法分配的 JSStrictArgumentsObject 对象内属性区间
    Tagged::new(unsafe { (*slot_ptr).load(Ordering::Relaxed) })
  }

  /// 写入 length 槽位
  #[inline]
  pub fn set_length(object: Tagged<JSObject>, value: Tagged<Object>) {
    let slot_ptr = (object.address() + Self::K_LENGTH_OFFSET) as *mut AtomicUsize;
    // SAFETY: slot_ptr 位于合法分配的 JSStrictArgumentsObject 对象内属性区间
    unsafe { (*slot_ptr).store(value.ptr(), Ordering::Relaxed) };
  }
}

/// 慢速别名映射条目（AliasedArgumentsEntry）
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AliasedArgumentsEntry {
  pub header: HeapObject,
  pub aliased_context_slot: Tagged<Smi>,
}

impl AliasedArgumentsEntry {
  pub const K_HEADER_SIZE: usize = mem::size_of::<Self>();

  #[inline]
  pub const fn new(map: Tagged<Map>, slot: i32) -> Self {
    Self {
      header: HeapObject::new(map),
      aliased_context_slot: Smi::from_int(slot),
    }
  }

  #[inline]
  pub fn aliased_context_slot(&self) -> i32 {
    self.aliased_context_slot.value()
  }
}

const _: () = assert!(AliasedArgumentsEntry::K_HEADER_SIZE == 2 * mem::size_of::<usize>());

/// 松散模式参数元素存储结构（SloppyArgumentsElements）
///
/// 内存布局:
/// - FixedArrayBase header (map + length)
/// - context_ (Tagged<Context>)
/// - arguments_ (Tagged<FixedArray>)
/// - mapped_entries[0..length] (Smi 或 TheHole)
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SloppyArgumentsElements {
  pub base: FixedArrayBase,
  pub context: Tagged<Object>,
  pub arguments: Tagged<FixedArrayBase>,
}

impl SloppyArgumentsElements {
  pub const K_CONTEXT_OFFSET: usize = mem::size_of::<FixedArrayBase>();
  pub const K_ARGUMENTS_OFFSET: usize = Self::K_CONTEXT_OFFSET + mem::size_of::<usize>();
  pub const K_MAPPED_ENTRIES_OFFSET: usize = Self::K_ARGUMENTS_OFFSET + mem::size_of::<usize>();
  pub const K_HEADER_SIZE: usize = mem::size_of::<Self>();

  /// 计算指定 mapped 条目数所需的字节大小
  #[inline]
  pub const fn size_for(mapped_count: usize) -> usize {
    Self::K_HEADER_SIZE + mapped_count * mem::size_of::<usize>()
  }
}

/// 松散参数元素操作扩展 trait
pub trait SloppyArgumentsElementsExt {
  /// 获取映射形参个数
  fn mapped_count(&self) -> usize;

  /// 读取第 index 个形参的映射条目
  fn mapped_entry(&self, index: usize) -> Result<Tagged<Object>>;

  /// 写入第 index 个形参的映射条目
  fn set_mapped_entry(&mut self, index: usize, value: Tagged<Object>) -> Result<()>;

  /// 依据原版 V8 逻辑解析实参查找位置
  fn lookup_argument(&self, key: usize) -> Result<ArgumentLocation>;
}

impl SloppyArgumentsElementsExt for SloppyArgumentsElements {
  #[inline]
  fn mapped_count(&self) -> usize {
    self.base.length as usize
  }

  fn mapped_entry(&self, index: usize) -> Result<Tagged<Object>> {
    let count = self.mapped_count();
    if index >= count {
      return Err(Error::IndexOutOfBounds { index, len: count });
    }
    let slot_addr =
      (self as *const Self as usize) + Self::K_HEADER_SIZE + index * mem::size_of::<usize>();
    let slot_ptr = slot_addr as *const AtomicUsize;
    // SAFETY: slot_addr 处于 mapped_entries 数组边界内且已验证 index < mapped_count
    Ok(Tagged::new(unsafe { (*slot_ptr).load(Ordering::Relaxed) }))
  }

  fn set_mapped_entry(&mut self, index: usize, value: Tagged<Object>) -> Result<()> {
    let count = self.mapped_count();
    if index >= count {
      return Err(Error::IndexOutOfBounds { index, len: count });
    }
    let slot_addr =
      (self as *mut Self as usize) + Self::K_HEADER_SIZE + index * mem::size_of::<usize>();
    let slot_ptr = slot_addr as *mut AtomicUsize;
    // SAFETY: slot_addr 处于 mapped_entries 数组边界内且已验证 index < mapped_count
    unsafe { (*slot_ptr).store(value.ptr(), Ordering::Relaxed) };
    Ok(())
  }

  fn lookup_argument(&self, key: usize) -> Result<ArgumentLocation> {
    if key >= self.mapped_count() {
      return Ok(ArgumentLocation::UnmappedArray(key));
    }
    let mapped = self.mapped_entry(key)?;
    if mapped.is_smi() {
      let smi = Tagged::<Smi>::new(mapped.ptr());
      Ok(ArgumentLocation::ContextSlot(smi.value()))
    } else {
      Ok(ArgumentLocation::UnmappedArray(key))
    }
  }
}

impl SloppyArgumentsElementsExt for Tagged<SloppyArgumentsElements> {
  #[inline]
  fn mapped_count(&self) -> usize {
    // SAFETY: Tagged 包装合法 SloppyArgumentsElements 指针
    unsafe { (*self.as_ptr()).mapped_count() }
  }

  #[inline]
  fn mapped_entry(&self, index: usize) -> Result<Tagged<Object>> {
    // SAFETY: Tagged 包装合法 SloppyArgumentsElements 指针
    unsafe { (*self.as_ptr()).mapped_entry(index) }
  }

  #[inline]
  fn set_mapped_entry(&mut self, index: usize, value: Tagged<Object>) -> Result<()> {
    // SAFETY: Tagged 包装合法可变 SloppyArgumentsElements 指针
    unsafe { (*self.as_mut_ptr()).set_mapped_entry(index, value) }
  }

  #[inline]
  fn lookup_argument(&self, key: usize) -> Result<ArgumentLocation> {
    // SAFETY: Tagged 包装合法 SloppyArgumentsElements 指针
    unsafe { (*self.as_ptr()).lookup_argument(key) }
  }
}

/// 拥有独立内存块的 SloppyArgumentsElements 容器
pub struct SloppyArgumentsElementsBuf {
  _storage: Vec<u64>,
  tagged: Tagged<SloppyArgumentsElements>,
}

impl SloppyArgumentsElementsBuf {
  /// 创建新的 SloppyArgumentsElementsBuf
  pub fn new(
    map: Tagged<Map>,
    context: Tagged<Object>,
    arguments: Tagged<FixedArrayBase>,
    mapped_count: usize,
  ) -> Result<Self> {
    let bytes = SloppyArgumentsElements::size_for(mapped_count);
    let words = bytes.div_ceil(8);
    let mut storage = vec![0u64; words];
    let base = storage.as_mut_ptr() as usize;

    let header = SloppyArgumentsElements {
      base: FixedArrayBase::new(map, mapped_count as u32),
      context,
      arguments,
    };

    // SAFETY: base 为新分配且容量足额的连续 8 字节对齐缓冲区
    unsafe {
      ptr::write(base as *mut SloppyArgumentsElements, header);
    }

    Ok(Self {
      _storage: storage,
      // SAFETY: base 包装为 Tagged<SloppyArgumentsElements>
      tagged: unsafe { Tagged::from_raw_address(base) },
    })
  }

  /// 获取 Tagged 引用
  #[inline]
  pub fn as_tagged(&self) -> Tagged<SloppyArgumentsElements> {
    self.tagged
  }
}

impl Deref for SloppyArgumentsElementsBuf {
  type Target = Tagged<SloppyArgumentsElements>;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.tagged
  }
}

impl DerefMut for SloppyArgumentsElementsBuf {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.tagged
  }
}
