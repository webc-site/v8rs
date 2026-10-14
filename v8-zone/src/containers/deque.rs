//! 原版 zone-containers.h 中 ZoneDeque/ZoneQueue/ZoneStack 转写
//!
//! 原版为 std::deque/std::queue/std::stack 的 zone 分配器包装
//! (std::deque<T, RecyclingZoneAllocator<T>> 等), 无自有 V8 API。
//! Rust 无对应 STL 家族, 此处实现行为等价的双栈式双端队列:
//! front 逆序存队头, back 正序存队尾。
//!
//! 偏差记录 (readme):
//! - std::deque 保证元素引用在增删后稳定; 本实现不保证 (Rust 以索引/
//!   值访问代替)。
//! - 双端交替弹出在两侧全量搬运时可能摊化为 O(n) 单次; 原版 V8 代码
//!   仅将 ZoneDeque 用作 queue/stack 单端用途, 影响面为零。
//! - insert/erase 中段操作原版无实际使用, 未转写。

use super::vector::ZoneVector;
use crate::{error::Result, zone::core::Zone};

/// 原版 ZoneDeque<T>: 双端队列
pub struct ZoneDeque<'z, 'a, T> {
  /// 队头侧, 逆序存放 (front.back() 即队头)
  front: ZoneVector<'z, 'a, T>,
  /// 队尾侧, 正序存放 (back.at(0) 即紧邻 front 的元素)
  back: ZoneVector<'z, 'a, T>,
}

impl<'z, 'a, T> ZoneDeque<'z, 'a, T> {
  /// 原版 explicit ZoneDeque(Zone* zone)
  pub fn new(zone: &'z Zone<'a>) -> ZoneDeque<'z, 'a, T> {
    ZoneDeque {
      front: ZoneVector::new(zone),
      back: ZoneVector::new(zone),
    }
  }

  /// 原版 size()
  pub fn size(&self) -> usize {
    self.front.size() + self.back.size()
  }

  /// 原版 empty()
  pub fn is_empty(&self) -> bool {
    self.size() == 0
  }

  /// 原版 push_back(T&&)
  pub fn push_back(&mut self, value: T) -> Result<()> {
    self.back.push_back(value)
  }

  /// 原版 push_front(const T&)
  pub fn push_front(&mut self, value: T) -> Result<()> {
    self.front.push_back(value)
  }

  /// front 侧为空时, 将 back 整体逆序搬入 (摊还搬运)
  fn move_back_to_front(&mut self) -> Result<()> {
    while let Some(value) = self.back.pop_back() {
      self.front.push_back(value)?;
    }
    Ok(())
  }

  /// back 侧为空时, 将 front 整体逆序搬回
  fn move_front_to_back(&mut self) -> Result<()> {
    while let Some(value) = self.front.pop_back() {
      self.back.push_back(value)?;
    }
    Ok(())
  }

  /// 原版 pop_front(): C++ UB 形态 (空队列) 收紧为 None, 搬运失败返回 Err
  pub fn pop_front(&mut self) -> Result<Option<T>> {
    if self.front.is_empty() {
      self.move_back_to_front()?;
    }
    Ok(self.front.pop_back())
  }

  /// 原版 pop_back()
  pub fn pop_back(&mut self) -> Result<Option<T>> {
    if self.back.is_empty() {
      self.move_front_to_back()?;
    }
    Ok(self.back.pop_back())
  }

  /// 原版 front() (空返回 None; C++ UB 形态收紧)
  pub fn front(&self) -> Option<&T> {
    if !self.front.is_empty() {
      self.front.back()
    } else {
      self.back.front()
    }
  }

  /// 原版 back()
  pub fn back(&self) -> Option<&T> {
    if !self.back.is_empty() {
      self.back.back()
    } else {
      self.front.front()
    }
  }

  /// 原版 operator[](size_t)
  pub fn at(&self, index: usize) -> &T {
    let f = self.front.size();
    if index < f {
      self.front.at(f - 1 - index)
    } else {
      self.back.at(index - f)
    }
  }

  /// 原版 begin()/end() 顺序迭代
  pub fn iter(&self) -> impl Iterator<Item = &T> {
    self.front.iter_rev().chain(self.back.iter())
  }

  /// 原版 clear()
  pub fn clear(&mut self) {
    self.front.clear();
    self.back.clear();
  }
}

/// 原版 ZoneQueue<T>: std::queue 包装
pub struct ZoneQueue<'z, 'a, T> {
  storage: ZoneDeque<'z, 'a, T>,
}

impl<'z, 'a, T> ZoneQueue<'z, 'a, T> {
  /// 原版 explicit ZoneQueue(Zone* zone)
  pub fn new(zone: &'z Zone<'a>) -> ZoneQueue<'z, 'a, T> {
    ZoneQueue {
      storage: ZoneDeque::new(zone),
    }
  }

  /// 原版 push(T&&)
  pub fn push(&mut self, value: T) -> Result<()> {
    self.storage.push_back(value)
  }

  /// 原版 pop(): C++ 返回 void, Rust 交还被弹出值
  pub fn pop(&mut self) -> Result<Option<T>> {
    self.storage.pop_front()
  }

  /// 原版 front()
  pub fn front(&self) -> Option<&T> {
    self.storage.front()
  }

  /// 原版 empty()/size()
  pub fn is_empty(&self) -> bool {
    self.storage.is_empty()
  }

  /// 原版 size()
  pub fn size(&self) -> usize {
    self.storage.size()
  }
}

/// 原版 ZoneStack<T>: std::stack 包装
pub struct ZoneStack<'z, 'a, T> {
  storage: ZoneDeque<'z, 'a, T>,
}

impl<'z, 'a, T> ZoneStack<'z, 'a, T> {
  /// 原版 explicit ZoneStack(Zone* zone)
  pub fn new(zone: &'z Zone<'a>) -> ZoneStack<'z, 'a, T> {
    ZoneStack {
      storage: ZoneDeque::new(zone),
    }
  }

  /// 原版 push(T&&)
  pub fn push(&mut self, value: T) -> Result<()> {
    self.storage.push_back(value)
  }

  /// 原版 pop()
  pub fn pop(&mut self) -> Result<Option<T>> {
    self.storage.pop_back()
  }

  /// 原版 top()
  pub fn top(&self) -> Option<&T> {
    self.storage.back()
  }

  /// 原版 empty()/size()
  pub fn is_empty(&self) -> bool {
    self.storage.is_empty()
  }

  /// 原版 size()
  pub fn size(&self) -> usize {
    self.storage.size()
  }
}
