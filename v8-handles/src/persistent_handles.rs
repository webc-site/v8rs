//! 由 tool/src/handles/persistent-handles.js 从 v8/src/handles/persistent-handles{.h,.cc} 生成
//! Isolate 依赖推迟: 链表注册经显式 list 引用 (原 isolate->persistent_handles_list());
//! LocalHeap 依赖推迟: Attach/Detach 保留为 owner 线程标记。
use std::{
  ptr::{NonNull, null_mut},
  thread::{ThreadId, current},
};

use parking_lot::Mutex;
use v8_common::Address;

use crate::handles::{Handle, K_HANDLE_BLOCK_SIZE, SlotPtr, Tagged, with_arena};
#[cfg(debug_assertions)]
use crate::handles::{K_PERSISTENT_HANDLE_ZAP_VALUE, zap_range};

/// 跨线程往返传递的句柄容器 (原 class PersistentHandles):
/// 分配/释放线程安全, 所有容器登记在链表上 (原类注释)。
/// Box move 后堆地址稳定, 链表侵入指针 (prev_/next_) 保持有效 —— 对应原版 unique_ptr 移交。
pub struct PersistentHandles {
  /// 所属链表 (原 isolate_->persistent_handles_list())
  list: *const PersistentHandlesList,
  /// 块集 (原 std::vector<Address*> blocks_)
  blocks: Vec<NonNull<Address>>,
  /// 下一空闲槽 (原 block_next_)
  block_next: SlotPtr,
  /// 当前块槽位上界 (原 block_limit_)
  block_limit: SlotPtr,
  /// 链表前驱 (原 prev_)
  prev: *mut PersistentHandles,
  /// 链表后继 (原 next_)
  next: *mut PersistentHandles,
  /// 归属线程 (原 DEBUG owner_): Attach/Detach 语义保留
  owner: Option<ThreadId>,
}

// SAFETY: 原版即跨线程传递 (unique_ptr 移交); 块指针所有权归容器, 无共享可变
unsafe impl Send for PersistentHandles {}

impl PersistentHandles {
  /// 构造并登记到链表 (原 PersistentHandles(Isolate*): 构造内 Add(this))
  pub fn new(list: &PersistentHandlesList) -> Box<Self> {
    let mut ph = Box::new(Self {
      list,
      blocks: Vec::new(),
      block_next: null_mut(),
      block_limit: null_mut(),
      prev: null_mut(),
      next: null_mut(),
      owner: None,
    });
    // SAFETY: list 为构造时传入的活指针, 调用方约定存活至本容器 Drop (对应原版 isolate 生命周期);
    // place 解引用进 autoref 不构成 unsafe 操作, 编译器已验证
    (*list).add(&mut ph);
    ph
  }

  /// 为对象分配持久句柄 (原 NewHandle(Tagged<T>))
  pub fn new_handle<T>(&mut self, obj: Tagged<T>) -> Handle<T> {
    self.check_owner_is_not_parked();
    Handle::from_location(self.get_handle(obj.ptr()))
  }

  /// 由既有句柄重分配 (原 NewHandle(IndirectHandle<T>) / NewHandle(DirectHandle<T>))
  pub fn new_handle_from<T>(&mut self, obj: Handle<T>) -> Handle<T> {
    self.new_handle(obj.value())
  }

  /// 归属标记 (原 DEBUG Attach(LocalHeap*))
  pub fn attach(&mut self) {
    debug_assert!(self.owner.is_none());
    self.owner = Some(current().id());
  }

  /// 解除归属 (原 DEBUG Detach())
  pub fn detach(&mut self) {
    debug_assert!(self.owner.is_some());
    self.owner = None;
  }

  /// 归属线程未驻留校验 (原 CheckOwnerIsNotParked): parked 属 LocalHeap (推迟), 保留调用面
  fn check_owner_is_not_parked(&self) {
    if let Some(owner) = self.owner {
      debug_assert_ne!(owner, current().id(), "owner 线程持有中");
    }
  }

  /// 槽位是否属于本容器 (原 DEBUG Contains): 末块上界特判 block_next
  pub fn contains(&self, location: SlotPtr) -> bool {
    let Some(&back) = self.blocks.last() else {
      return false;
    };
    for &block in &self.blocks {
      let lower = block.as_ptr();
      // SAFETY: 块首 + 容量 K_HANDLE_BLOCK_SIZE; 末块上界改用 block_next
      let upper = unsafe {
        if block != back {
          lower.add(K_HANDLE_BLOCK_SIZE)
        } else {
          self.block_next
        }
      };
      if lower <= location && location < upper {
        return true;
      }
    }
    false
  }

  /// 追加新块 (原 private AddBlock)
  fn add_block(&mut self) {
    debug_assert_eq!(self.block_next, self.block_limit);
    let block: Box<[Address; K_HANDLE_BLOCK_SIZE]> = Box::new([0; K_HANDLE_BLOCK_SIZE]);
    // SAFETY: Box 指针非空
    let start = unsafe { NonNull::new_unchecked(Box::into_raw(block) as *mut Address) };
    self.blocks.push(start);
    self.block_next = start.as_ptr();
    // SAFETY: 块首 + 容量
    self.block_limit = unsafe { start.as_ptr().add(K_HANDLE_BLOCK_SIZE) };
  }

  /// 取槽写值 (原 private GetHandle)
  fn get_handle(&mut self, value: Address) -> SlotPtr {
    if self.block_next == self.block_limit {
      self.add_block();
    }
    debug_assert!(self.block_next < self.block_limit);
    // SAFETY: block_next < block_limit 且指向本容器持有块内空槽
    unsafe {
      *self.block_next = value;
      let result = self.block_next;
      self.block_next = self.block_next.add(1);
      result
    }
  }

  /// 遍历全部存活句柄槽区间 [start, end) (原 Iterate(RootVisitor*)):
  /// 非末块整块访问, 末块截至 block_next
  pub fn iterate(&self, mut visit: impl FnMut(*const Address, *const Address)) {
    let Some((&back, rest)) = self.blocks.split_last() else {
      return;
    };
    for &block in rest {
      let start = block.as_ptr();
      // SAFETY: 块首 + 容量
      let end = unsafe { start.add(K_HANDLE_BLOCK_SIZE) };
      visit(start, end);
    }
    // 末块截至 block_next
    visit(back.as_ptr(), self.block_next);
  }
}

impl Drop for PersistentHandles {
  fn drop(&mut self) {
    // SAFETY: list 在构造时传入且存活至本容器 Drop (调用方约定, 对应原版 isolate 生命周期)
    unsafe {
      (*self.list).remove(self);
    }
    // debug 构建回收块前填充 zap 值 (原 persistent-handles.cc:31
    // ZapRange(..., kPersistentHandleZapValue) —— 与句柄 zap 值刻意不同)
    #[cfg(debug_assertions)]
    for &block in &self.blocks {
      let start = block.as_ptr();
      // SAFETY: 块首 + 容量
      let end = unsafe { start.add(K_HANDLE_BLOCK_SIZE) };
      zap_range(start, end, K_PERSISTENT_HANDLE_ZAP_VALUE);
    }
    for &block in &self.blocks {
      // SAFETY: 块首来自 Box::into_raw, Drop 时容器唯一持有
      drop(unsafe { Box::from_raw(block.as_ptr() as *mut [Address; K_HANDLE_BLOCK_SIZE]) });
    }
  }
}

/// 容器链表 (原 class PersistentHandlesList): 侵入式双向链表 + 互斥锁
pub struct PersistentHandlesList {
  /// (原 base::Mutex persistent_handles_mutex_)
  mutex: Mutex<ListState>,
}

#[derive(Default)]
struct ListState {
  /// (原 persistent_handles_head_)
  head: *mut PersistentHandles,
}

impl Default for PersistentHandlesList {
  fn default() -> Self {
    Self::new()
  }
}

impl PersistentHandlesList {
  /// 原构造: head 置空
  pub const fn new() -> Self {
    Self {
      mutex: Mutex::new(ListState { head: null_mut() }),
    }
  }

  /// 头插 (原 private Add)
  fn add(&self, ph: &mut PersistentHandles) {
    let mut guard = self.mutex.lock();
    if !guard.head.is_null() {
      // SAFETY: head 为链表节点, 链表互斥锁内访问
      unsafe {
        (*guard.head).prev = ph as *mut PersistentHandles;
      }
    }
    ph.prev = null_mut();
    ph.next = guard.head;
    guard.head = ph;
  }

  /// 摘除 (原 private Remove)
  fn remove(&self, ph: &mut PersistentHandles) {
    let mut guard = self.mutex.lock();
    if !ph.next.is_null() {
      // SAFETY: next 为链表节点, 链表互斥锁内访问
      unsafe {
        (*ph.next).prev = ph.prev;
      }
    }
    if !ph.prev.is_null() {
      // SAFETY: prev 为链表节点, 链表互斥锁内访问
      unsafe {
        (*ph.prev).next = ph.next;
      }
    } else {
      guard.head = ph.next;
    }
  }

  /// 遍历全部容器 (原 Iterate(RootVisitor*, Isolate*)): 锁内逐容器回调
  pub fn iterate(&self, mut visit: impl FnMut(&PersistentHandles)) {
    let guard = self.mutex.lock();
    let mut current = guard.head;
    while !current.is_null() {
      // SAFETY: 持锁期间链表结构稳定, 节点存活至 Remove
      unsafe {
        visit(&*current);
        current = (*current).next;
      }
    }
  }

  /// 登记容器数
  pub fn len(&self) -> usize {
    let mut n = 0;
    let guard = self.mutex.lock();
    let mut current = guard.head;
    while !current.is_null() {
      // SAFETY: 同 iterate
      unsafe {
        n += 1;
        current = (*current).next;
      }
    }
    n
  }

  /// 是否无登记容器
  pub fn is_empty(&self) -> bool {
    self.mutex.lock().head.is_null()
  }
}

/// persistent 收集作用域 (原 class V8_NODISCARD PersistentHandlesScope):
/// 作用域内主线程句柄全部落入将移交他线程的 persistent 块 (原类注释)
#[must_use]
pub struct PersistentHandlesScope {
  /// persistent 首块 (原 first_block_)
  first_block: SlotPtr,
  /// (原 prev_limit_)
  prev_limit: SlotPtr,
  /// (原 prev_next_)
  prev_next: SlotPtr,
  /// 是否已 detach (原 DEBUG handles_detached_)
  detached: bool,
}

impl PersistentHandlesScope {
  /// 进入作用域 (原 PersistentHandlesScope(Isolate*)):
  /// 标记 persistent 起点, 推入新块并切换 data.next/limit
  pub fn new() -> Self {
    with_arena(|a| {
      a.begin_persistent_scope();
      let (new_next, new_limit) = a.push_persistent_block();
      a.data.level += 1;
      let scope = Self {
        first_block: new_next,
        prev_limit: a.data.limit,
        prev_next: a.data.next,
        detached: false,
      };
      a.data.next = new_next;
      a.data.limit = new_limit;
      scope
    })
  }

  /// 把作用域内全部块移入新 PersistentHandles 并返回 (原 Detach → unique_ptr)
  pub fn detach(mut self, list: &PersistentHandlesList) -> Box<PersistentHandles> {
    let ph = with_arena(|a| {
      let (blocks, block_next) = a.take_persistent_blocks(self.first_block);
      // 恢复进入前的 next/limit (原 data->next = prev_next_; data->limit = prev_limit_)
      a.data.next = self.prev_next;
      a.data.limit = self.prev_limit;
      let mut ph = PersistentHandles::new(list);
      ph.blocks = blocks;
      // SAFETY: 末块(= first_block 满块)首 + 容量
      let back = ph.blocks[ph.blocks.len() - 1].as_ptr();
      ph.block_limit = unsafe { back.add(K_HANDLE_BLOCK_SIZE) };
      ph.block_next = block_next;
      ph
    });
    self.detached = true;
    ph
  }

  /// 当前是否处于 persistent 作用域 (原 static IsActive)
  pub fn is_active() -> bool {
    with_arena(|a| a.has_persistent_scope())
  }
}

impl Default for PersistentHandlesScope {
  fn default() -> Self {
    Self::new()
  }
}

impl Drop for PersistentHandlesScope {
  fn drop(&mut self) {
    // 原版要求析构前必须 Detach (DCHECK(handles_detached_)):
    // 未 detach 时回滚块并恢复 next/limit, 防止泄漏 —— 安全降级
    if !self.detached {
      with_arena(|a| {
        let (blocks, _) = a.take_persistent_blocks(self.first_block);
        a.data.next = self.prev_next;
        a.data.limit = self.prev_limit;
        for block in blocks {
          // SAFETY: 块首来自 Box::into_raw, 此刻唯一持有
          drop(unsafe { Box::from_raw(block.as_ptr() as *mut [Address; K_HANDLE_BLOCK_SIZE]) });
        }
      });
    }
    with_arena(|a| a.data.level -= 1);
  }
}
