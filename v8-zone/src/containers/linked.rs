//! 原版 zone-containers.h 中 ZoneForwardList/ZoneLinkedList 转写
//!
//! 原版是 std::forward_list/std::list 配 ZoneAllocator 的薄包装。
//! Rust 无 allocator 概念, 此处以 zone 分配节点实现等价链表;
//! 节点析构与存储归还 (仅记账) 行为与 ZoneAllocator 一致。
//!
//! 偏差记录 (readme):
//! - 仅转写 V8 实际使用的子集 (push/pop/前插前删/迭代/clear),
//!   std::list 的 splice/insert(it)/erase(it) 中段接口未转写。
//! - forward_list 原版无 size(); Rust 版维护 length 计数方便使用。

use std::{marker::PhantomData, ptr};

use crate::{error::Result, zone::core::Zone};

/// ZoneForwardList 节点
struct FNode<T> {
  next: *mut FNode<T>,
  value: T,
}

/// 原版 ZoneForwardList<T>: std::forward_list 包装
pub struct ZoneForwardList<'z, 'a, T> {
  zone: &'z Zone<'a>,
  head: *mut FNode<T>,
  length: usize,
}

impl<'z, 'a, T> ZoneForwardList<'z, 'a, T> {
  /// 原版 explicit ZoneForwardList(Zone* zone)
  pub fn new(zone: &'z Zone<'a>) -> ZoneForwardList<'z, 'a, T> {
    ZoneForwardList {
      zone,
      head: ptr::null_mut(),
      length: 0,
    }
  }

  /// 原版 empty()
  pub fn is_empty(&self) -> bool {
    self.head.is_null()
  }

  /// 原版 size() (std::forward_list 无此接口, 便利扩展)
  pub fn len(&self) -> usize {
    self.length
  }

  /// 原版 push_front(T&&)
  pub fn push_front(&mut self, value: T) -> Result<()> {
    // zone.make 完成对齐检查 + 构造, 'z 引用随后转裸指针 (arena 内存不回收)
    let node = self.zone.make(FNode {
      next: self.head,
      value,
    })?;
    let ptr: *mut FNode<T> = node;
    self.head = ptr;
    self.length += 1;
    Ok(())
  }

  /// 原版 pop_front(): C++ UB 空形态收紧为 None
  pub fn pop_front(&mut self) -> Option<T> {
    if self.head.is_null() {
      return None;
    }
    // SAFETY: head 指向本列表此前经 zone.make 构造的有效节点
    unsafe {
      let node = &mut *self.head;
      let next = node.next;
      // move 取出 value, 节点槽位成为 gap (仅剩簿记壳)
      let value = ptr::read(ptr::addr_of_mut!(node.value));
      self.zone.delete(self.head, 1);
      self.head = next;
      self.length -= 1;
      Some(value)
    }
  }

  /// 原版 begin()/end() 前向迭代
  pub fn iter(&self) -> ZoneForwardListIter<'_, T> {
    ZoneForwardListIter {
      cur: self.head,
      _p: PhantomData,
    }
  }

  /// 原版 clear()
  pub fn clear(&mut self) {
    while self.pop_front().is_some() {}
  }
}

impl<T> Drop for ZoneForwardList<'_, '_, T> {
  /// 原版 ~std::forward_list: 析构全部元素并归还节点 (记账)
  fn drop(&mut self) {
    self.clear();
  }
}

/// ZoneForwardList 前向迭代器
pub struct ZoneForwardListIter<'d, T> {
  cur: *mut FNode<T>,
  _p: PhantomData<&'d T>,
}

impl<'d, T> Iterator for ZoneForwardListIter<'d, T> {
  type Item = &'d T;
  fn next(&mut self) -> Option<&'d T> {
    if self.cur.is_null() {
      return None;
    }
    // SAFETY: 节点存活期覆盖 'd (借用自列表)
    let node = unsafe { &*self.cur };
    self.cur = node.next;
    Some(&node.value)
  }
}

/// ZoneLinkedList 节点
struct DNode<T> {
  prev: *mut DNode<T>,
  next: *mut DNode<T>,
  value: T,
}

/// 原版 ZoneLinkedList<T>: std::list 包装
pub struct ZoneLinkedList<'z, 'a, T> {
  zone: &'z Zone<'a>,
  head: *mut DNode<T>,
  tail: *mut DNode<T>,
  length: usize,
}

impl<'z, 'a, T> ZoneLinkedList<'z, 'a, T> {
  /// 原版 explicit ZoneLinkedList(Zone* zone)
  pub fn new(zone: &'z Zone<'a>) -> ZoneLinkedList<'z, 'a, T> {
    ZoneLinkedList {
      zone,
      head: ptr::null_mut(),
      tail: ptr::null_mut(),
      length: 0,
    }
  }

  /// 原版 empty()/size()
  pub fn is_empty(&self) -> bool {
    self.length == 0
  }

  /// 原版 size()
  pub fn len(&self) -> usize {
    self.length
  }

  /// 原版 push_back(T&&)
  pub fn push_back(&mut self, value: T) -> Result<()> {
    let node = self.zone.make(DNode {
      prev: self.tail,
      next: ptr::null_mut(),
      value,
    })?;
    let ptr: *mut DNode<T> = node;
    if self.tail.is_null() {
      self.head = ptr;
    } else {
      // SAFETY: tail 为有效节点
      unsafe { (*self.tail).next = ptr };
    }
    self.tail = ptr;
    self.length += 1;
    Ok(())
  }

  /// 原版 push_front(T&&)
  pub fn push_front(&mut self, value: T) -> Result<()> {
    let node = self.zone.make(DNode {
      prev: ptr::null_mut(),
      next: self.head,
      value,
    })?;
    let ptr: *mut DNode<T> = node;
    if self.head.is_null() {
      self.tail = ptr;
    } else {
      // SAFETY: head 为有效节点
      unsafe { (*self.head).prev = ptr };
    }
    self.head = ptr;
    self.length += 1;
    Ok(())
  }

  /// 原版 pop_front()
  pub fn pop_front(&mut self) -> Option<T> {
    if self.head.is_null() {
      return None;
    }
    // SAFETY: head 为有效节点
    unsafe {
      let node = &mut *self.head;
      let next = node.next;
      let value = ptr::read(ptr::addr_of_mut!(node.value));
      self.zone.delete(self.head, 1);
      self.head = next;
      if next.is_null() {
        self.tail = ptr::null_mut();
      } else {
        (*next).prev = ptr::null_mut();
      }
      self.length -= 1;
      Some(value)
    }
  }

  /// 原版 pop_back()
  pub fn pop_back(&mut self) -> Option<T> {
    if self.tail.is_null() {
      return None;
    }
    // SAFETY: tail 为有效节点
    unsafe {
      let node = &mut *self.tail;
      let prev = node.prev;
      let value = ptr::read(ptr::addr_of_mut!(node.value));
      self.zone.delete(self.tail, 1);
      self.tail = prev;
      if prev.is_null() {
        self.head = ptr::null_mut();
      } else {
        (*prev).next = ptr::null_mut();
      }
      self.length -= 1;
      Some(value)
    }
  }

  /// 原版 front()/back(): C++ UB 空形态收紧为 Option
  pub fn front(&self) -> Option<&T> {
    if self.head.is_null() {
      None
    } else {
      Some(&unsafe { &*self.head }.value)
    }
  }

  /// 原版 back()
  pub fn back(&self) -> Option<&T> {
    if self.tail.is_null() {
      None
    } else {
      Some(&unsafe { &*self.tail }.value)
    }
  }

  /// 原版 begin()/end() 前向迭代
  pub fn iter(&self) -> ZoneLinkedListIter<'_, T> {
    ZoneLinkedListIter {
      cur: self.head,
      _p: PhantomData,
    }
  }

  /// 原版 clear()
  pub fn clear(&mut self) {
    while self.pop_front().is_some() {}
  }
}

impl<T> Drop for ZoneLinkedList<'_, '_, T> {
  /// 原版 ~std::list: 析构全部元素并归还节点 (记账)
  fn drop(&mut self) {
    self.clear();
  }
}

/// ZoneLinkedList 前向迭代器
pub struct ZoneLinkedListIter<'d, T> {
  cur: *mut DNode<T>,
  _p: PhantomData<&'d T>,
}

impl<'d, T> Iterator for ZoneLinkedListIter<'d, T> {
  type Item = &'d T;
  fn next(&mut self) -> Option<&'d T> {
    if self.cur.is_null() {
      return None;
    }
    // SAFETY: 节点存活期覆盖 'd (借用自列表)
    let node = unsafe { &*self.cur };
    self.cur = node.next;
    Some(&node.value)
  }
}
