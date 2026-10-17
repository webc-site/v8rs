// 由 tool/src/builtins/builtins-array.js 生成; 来源 v8/src/builtins 下 3 文件
// 内建名表 + 动态定长数组 GrowableFixedArray 实现
// 来源: growable-fixed-array-gen.cc, growable-fixed-array-gen.h, growable-fixed-array.tq

use strum::{Display, EnumIter, EnumString};

use crate::error::{Error, Result};

/// 内建名表 (来源: growable-fixed-array-gen.cc, growable-fixed-array-gen.h, growable-fixed-array.tq)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
#[strum(serialize_all = "PascalCase")]
pub enum Builtin {}

/// .tq 声明 2 条 (macro/struct/type/extern)
pub const TQ_DECLS: &[(&str, &str)] = &[
  ("struct", "GrowableFixedArray"),
  ("macro", "NewGrowableFixedArray"),
];

/// 初始最小增长步长 (对应原版 GrowableFixedArray::NewCapacity: + 16)
pub const DEFAULT_MIN_GROWTH: usize = 16;
/// 固定数组最大元素容量限制 (512M elements)
pub const MAX_FIXED_ARRAY_CAPACITY: usize = 1024 * 1024 * 512;

/// 100% Safe Rust 动态可增长定长数组模型 (对应原版 V8 GrowableFixedArray)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrowableFixedArray<T = u64> {
  data: Vec<T>,
  capacity: usize,
}

impl<T> Default for GrowableFixedArray<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T> GrowableFixedArray<T> {
  /// 创建空的可动态增长定长数组
  pub fn new() -> Self {
    Self {
      data: Vec::new(),
      capacity: 0,
    }
  }

  /// 创建具备预设容量的实例
  pub fn with_capacity(capacity: usize) -> Result<Self> {
    let mut arr = Self::new();
    arr.reserve(capacity)?;
    Ok(arr)
  }

  /// 计算下一级扩容容量 (对应原版 GrowableFixedArray::NewCapacity)
  /// 公式: new_capacity = current_capacity + (current_capacity >> 1) + 16
  #[inline]
  pub fn next_capacity(current_capacity: usize) -> usize {
    current_capacity
      .saturating_add(current_capacity >> 1)
      .saturating_add(DEFAULT_MIN_GROWTH)
  }

  /// 获取当前已存储元素个数
  #[inline]
  pub fn length(&self) -> usize {
    self.data.len()
  }

  /// 数组是否为空
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.data.is_empty()
  }

  /// 获取当前已分配容量
  #[inline]
  pub fn capacity(&self) -> usize {
    self.capacity
  }

  /// 预留指定容量 (对应原版 GrowableFixedArray::Reserve)
  pub fn reserve(&mut self, required_capacity: usize) -> Result<()> {
    if self.capacity >= required_capacity {
      return Ok(());
    }
    if required_capacity > MAX_FIXED_ARRAY_CAPACITY {
      return Err(Error::CapacityOverflow {
        requested: required_capacity,
        max: MAX_FIXED_ARRAY_CAPACITY,
      });
    }

    let mut new_capacity = self.capacity;
    while new_capacity < required_capacity {
      new_capacity = Self::next_capacity(new_capacity);
      if new_capacity > MAX_FIXED_ARRAY_CAPACITY {
        new_capacity = MAX_FIXED_ARRAY_CAPACITY;
        break;
      }
    }
    self.capacity = new_capacity;
    self.data.reserve(self.capacity - self.data.len());
    Ok(())
  }

  /// 追加元素 (对应原版 GrowableFixedArray::Push)
  pub fn push(&mut self, value: T) -> Result<()> {
    if self.data.len() == self.capacity {
      let new_capacity = Self::next_capacity(self.capacity);
      self.reserve(new_capacity)?;
    }
    self.data.push(value);
    Ok(())
  }

  /// 获取指定索引的元素引用
  #[inline]
  pub fn get(&self, index: usize) -> Option<&T> {
    self.data.get(index)
  }

  /// 获取指定索引的元素可变引用
  #[inline]
  pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
    self.data.get_mut(index)
  }

  /// 收缩容量以精确匹配当前长度 (对应原版 ToFixedArray / ToJSArray 紧凑化)
  pub fn shrink_to_fit(&mut self) {
    self.data.shrink_to_fit();
    self.capacity = self.data.len();
  }

  /// 获取底层切片视图
  #[inline]
  pub fn as_slice(&self) -> &[T] {
    &self.data
  }

  /// 清空所有元素
  pub fn clear(&mut self) {
    self.data.clear();
  }
}
