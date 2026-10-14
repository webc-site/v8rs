// v8-objects-js-func/src/context.rs
// 对应原版 v8/src/objects/contexts.h + contexts-inl.h

//! Context: JavaScript 执行上下文对象，采用类似 FixedArray 的变长槽位布局，
//! 负责保存词法作用域、变量绑定、外层上一级 Context 引用（链式溯源）与 NativeContext 根。

use std::{
  mem,
  ops::{Deref, DerefMut},
  ptr,
};

use v8_objects_core::{
  map::Map,
  smi::Smi,
  tagged::{Object, Tagged},
};

use crate::error::{Error, Result};

/// ScopeInfo 基础槽位编号
pub const K_SCOPE_INFO_INDEX: usize = 0;

/// Previous 槽位编号（上一层 Context 引用）
pub const K_PREVIOUS_INDEX: usize = 1;

/// Extension 槽位编号（扩展词法槽）
pub const K_EXTENSION_INDEX: usize = 2;

/// 最少上下文槽位数
pub const K_MIN_CONTEXT_SLOTS: usize = 2;

/// 包含扩展槽的最少上下文槽位数
pub const K_MIN_CONTEXT_EXTENDED_SLOTS: usize = 3;

/// NativeContext 全局对象槽位 (与 EXTENSION_INDEX 相同，指向 JSGlobalObject)
pub const K_GLOBAL_OBJECT_INDEX: usize = 2;

/// NativeContext 全局代理对象槽位
pub const K_GLOBAL_PROXY_INDEX: usize = 3;

/// NativeContext 嵌入器数据槽位
pub const K_EMBEDDER_DATA_INDEX: usize = 4;

/// 上下文头部固定大小 (map + length = 2 words = 16 字节)
pub const K_HEADER_SIZE: usize = mem::size_of::<ContextHeader>();

/// 最大溯源遍历深度（防止恶意自环死循环）
const K_MAX_ANCESTOR_DEPTH: usize = 1024;

/// 上下文固定头部
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ContextHeader {
  /// Map 对象指针 (8 字节)
  pub map: Tagged<Map>,
  /// 槽位总长度 (Tagged<Smi>, 8 字节)
  pub length: Tagged<Smi>,
}

const _: () = assert!(mem::size_of::<ContextHeader>() == 16);

/// Context 堆对象标记结构体
#[repr(C)]
#[derive(Debug)]
pub struct Context {
  /// 头部字段
  pub header: ContextHeader,
}

/// Context 访问与链式溯源扩展 trait
pub trait ContextExt {
  /// 获取上下文元素容量
  fn length(&self) -> usize;

  /// 获取指定槽位的值
  fn get(&self, index: usize) -> Result<Tagged<Object>>;

  /// 设置指定槽位的值
  fn set(&mut self, index: usize, value: Tagged<Object>) -> Result<()>;

  /// 获取 ScopeInfo 槽位对象
  #[inline]
  fn scope_info(&self) -> Result<Tagged<Object>> {
    self.get(K_SCOPE_INFO_INDEX)
  }

  /// 设置 ScopeInfo 槽位对象
  #[inline]
  fn set_scope_info(&mut self, info: Tagged<Object>) -> Result<()> {
    self.set(K_SCOPE_INFO_INDEX, info)
  }

  /// 获取上一级 Context（链式溯源）
  #[inline]
  fn previous(&self) -> Result<Option<Tagged<Context>>> {
    let raw = self.get(K_PREVIOUS_INDEX)?;
    if raw.is_null() || raw.is_smi() {
      Ok(None)
    } else {
      // SAFETY: raw 为合法的堆对象指针，指向父级 Context
      Ok(Some(Tagged::new(raw.ptr())))
    }
  }

  /// 设置上一级 Context
  #[inline]
  fn set_previous(&mut self, prev: Option<Tagged<Context>>) -> Result<()> {
    let val = match prev {
      Some(p) => Tagged::new(p.ptr()),
      None => Tagged::new(0),
    };
    self.set(K_PREVIOUS_INDEX, val)
  }

  /// 获取扩展槽位
  #[inline]
  fn extension(&self) -> Result<Tagged<Object>> {
    self.get(K_EXTENSION_INDEX)
  }

  /// 设置扩展槽位
  #[inline]
  fn set_extension(&mut self, ext: Tagged<Object>) -> Result<()> {
    self.set(K_EXTENSION_INDEX, ext)
  }

  /// 是否为原生/根上下文（无 previous 父节点）
  #[inline]
  fn is_native_context(&self) -> Result<bool> {
    Ok(self.previous()?.is_none())
  }

  /// 沿着 previous 引用向上溯源，查找 NativeContext 根节点
  fn native_context(&self) -> Result<Tagged<Context>>;
}

impl ContextExt for Tagged<Context> {
  #[inline]
  fn length(&self) -> usize {
    // SAFETY: Tagged 包装合法 ContextHeader 指针
    let header = unsafe { &*self.as_ptr() };
    Smi::value(header.header.length) as usize
  }

  fn get(&self, index: usize) -> Result<Tagged<Object>> {
    let len = self.length();
    if index >= len {
      return Err(Error::IndexOutOfBounds { index, len });
    }
    let slot_addr = self.address() + K_HEADER_SIZE + index * mem::size_of::<usize>();
    let slot_ptr = slot_addr as *const Tagged<Object>;
    // SAFETY: index < len 保证 slot_addr 位于有效分配的 Context 槽位范围内
    unsafe { Ok(ptr::read(slot_ptr)) }
  }

  fn set(&mut self, index: usize, value: Tagged<Object>) -> Result<()> {
    let len = self.length();
    if index >= len {
      return Err(Error::IndexOutOfBounds { index, len });
    }
    let slot_addr = self.address() + K_HEADER_SIZE + index * mem::size_of::<usize>();
    let slot_ptr = slot_addr as *mut Tagged<Object>;
    // SAFETY: index < len 保证 slot_addr 位于有效分配的 Context 槽位范围内
    unsafe { ptr::write(slot_ptr, value) };
    Ok(())
  }

  fn native_context(&self) -> Result<Tagged<Context>> {
    let mut current = *self;
    let mut depth = 0;
    while let Some(parent) = current.previous()? {
      current = parent;
      depth += 1;
      if depth >= K_MAX_ANCESTOR_DEPTH {
        return Err(Error::ContextNotFound);
      }
    }
    Ok(current)
  }
}

/// 拥有独立连续内存的 Context 容器
pub struct ContextBuf {
  _storage: Vec<u64>,
  tagged: Tagged<Context>,
  length: usize,
}

impl ContextBuf {
  /// 创建指定槽位容量的 Context
  pub fn new(map: Tagged<Map>, length: usize) -> Result<Self> {
    if length < K_MIN_CONTEXT_SLOTS {
      return Err(Error::InvalidCapacity(length));
    }
    let total_bytes = K_HEADER_SIZE + length * mem::size_of::<usize>();
    let words = total_bytes.div_ceil(8);
    let mut storage = vec![0u64; words];
    let base = storage.as_mut_ptr() as usize;

    let header = ContextHeader {
      map,
      length: Smi::from_int(length as i32),
    };
    // SAFETY: base 指向有效分配且 8 字节对齐的内存首地址
    unsafe {
      ptr::write(base as *mut ContextHeader, header);
    }

    Ok(Self {
      _storage: storage,
      // SAFETY: base 为有效分配的 Context 内存首地址
      tagged: unsafe { Tagged::from_raw_address(base) },
      length,
    })
  }

  /// 获取 Tagged 引用
  #[inline]
  pub const fn as_tagged(&self) -> Tagged<Context> {
    self.tagged
  }
}

impl Deref for ContextBuf {
  type Target = Tagged<Context>;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.tagged
  }
}

impl DerefMut for ContextBuf {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.tagged
  }
}

impl ContextExt for ContextBuf {
  #[inline]
  fn length(&self) -> usize {
    self.length
  }

  #[inline]
  fn get(&self, index: usize) -> Result<Tagged<Object>> {
    self.tagged.get(index)
  }

  #[inline]
  fn set(&mut self, index: usize, value: Tagged<Object>) -> Result<()> {
    self.tagged.set(index, value)
  }

  #[inline]
  fn native_context(&self) -> Result<Tagged<Context>> {
    self.tagged.native_context()
  }
}
