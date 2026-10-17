// 由 tool/src/utils/consts.js 从 v8/src/utils/locked-queue.h;src/utils/locked-queue-inl.h 生成; 修订就地改, 复跑覆盖

//! 锁保护的无界队列 (多生产者/多消费者), 移植自 v8/src/utils/locked-queue。
//!
//! Scott & Michael 阻塞队列算法: 哑头节点单链表, head/tail 各持一把锁
//! (入队出队可并行), 节点链接走原子指针 (上游 base::AtomicValue<Node*>),
//! size 走原子计数。节点为 Malloced → `Box::into_raw`。
//!
//! 不变量: head 恒指向哑头或最老数据节点; tail 恒指向最后一个节点 (可为哑头);
//! 队列非空时 head != tail (tail.next 恒空)。

use std::{
  cell::UnsafeCell,
  marker::PhantomData,
  ptr::null_mut,
  sync::atomic::{AtomicPtr, AtomicUsize, Ordering},
};

use parking_lot::Mutex;

/// 链表节点 (LockedQueue::Node)
struct Node<T> {
  value: Option<T>,
  /// next: 上游 base::AtomicValue<Node*>, head/tail 两锁交叉读写须原子
  next: AtomicPtr<Node<T>>,
}

/// 锁保护队列 (LockedQueue<Record>)
pub struct LockedQueue<T> {
  /// head 锁: 保护出队/判空/窥视
  head_mutex: Mutex<()>,
  /// tail 锁: 保护入队
  tail_mutex: Mutex<()>,
  head: UnsafeCell<*mut Node<T>>,
  tail: UnsafeCell<*mut Node<T>>,
  size: AtomicUsize,
  _pd: PhantomData<Node<T>>,
}

impl<T> LockedQueue<T> {
  /// 构造: 哑头节点 + tail 同指 (LockedQueue())
  pub fn new() -> Self {
    let dummy = Box::into_raw(Box::new(Node {
      value: None,
      next: AtomicPtr::new(null_mut()),
    }));
    LockedQueue {
      head_mutex: Mutex::new(()),
      tail_mutex: Mutex::new(()),
      head: UnsafeCell::new(dummy),
      tail: UnsafeCell::new(dummy),
      size: AtomicUsize::new(0),
      _pd: PhantomData,
    }
  }

  /// Enqueue: 尾插
  pub fn enqueue(&self, record: T) {
    let n = Box::into_raw(Box::new(Node {
      value: Some(record),
      next: AtomicPtr::new(null_mut()),
    }));
    let _guard = self.tail_mutex.lock();
    // SAFETY: tail 持锁写 (仅 enqueue 改 tail); next 原子发布 (Release)
    unsafe {
      // size 先计数再发布 next (上游 Enqueue 同序, locked-queue-inl.h:51-52):
      // 出队者经 Acquire 读到新节点时必已看到 size 自增, fetch_sub 不下溢
      self.size.fetch_add(1, Ordering::Release);
      (**self.tail.get()).next.store(n, Ordering::Release);
      *self.tail.get() = n;
    }
  }

  /// Dequeue: 头出; 空返回 None
  pub fn dequeue(&self) -> Option<T> {
    let _guard = self.head_mutex.lock();
    // SAFETY: head 持锁; next 原子读取 (Acquire 配对发布); 队列非空时 head != tail,
    // 释放的旧头不可能是 tail 正指向的节点
    unsafe {
      let head = *self.head.get();
      let next = (*head).next.load(Ordering::Acquire);
      if next.is_null() {
        return None; // 空: 仅剩哑头
      }
      let old = Box::from_raw(head);
      debug_assert!(old.value.is_none());
      *self.head.get() = next;
      let value = (*next).value.take();
      self.size.fetch_sub(1, Ordering::AcqRel);
      value
    }
  }

  /// IsEmpty: 哑头无后继即空
  pub fn is_empty(&self) -> bool {
    let _guard = self.head_mutex.lock();
    // SAFETY: head 持锁
    unsafe { (**self.head.get()).next.load(Ordering::Acquire).is_null() }
  }

  /// Peek: 窥视队首值 (克隆), 空返回 None
  pub fn peek(&self) -> Option<T>
  where
    T: Clone,
  {
    let _guard = self.head_mutex.lock();
    // SAFETY: head 持锁; next 已发布且仍处 head 锁内
    unsafe {
      let next = (**self.head.get()).next.load(Ordering::Acquire);
      if next.is_null() {
        return None;
      }
      (*next).value.clone()
    }
  }

  /// size
  pub fn size(&self) -> usize {
    self.size.load(Ordering::Acquire)
  }
}

impl<T> Default for LockedQueue<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T> Drop for LockedQueue<T> {
  fn drop(&mut self) {
    // ~LockedQueue: 沿链销毁全部剩余节点与值
    // SAFETY: 独占, 链终止于 null
    unsafe {
      let mut cur = *self.head.get();
      while !cur.is_null() {
        let next = (*cur).next.load(Ordering::Relaxed);
        drop(Box::from_raw(cur));
        cur = next;
      }
    }
  }
}

// SAFETY: 裸指针仅在各自锁内解引用, next 链接原子; T 跨线程收发即可共享
unsafe impl<T: Send> Send for LockedQueue<T> {}
unsafe impl<T: Send> Sync for LockedQueue<T> {}
