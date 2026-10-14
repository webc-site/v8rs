//! PersistentHandles 语义测试: 跨作用域存活, 块扩展, 链表登记, scope detach, move 语义
use std::{
  mem::size_of,
  ptr::{eq, null_mut},
};

use aok::{OK, Void};
use log::info;
use v8_handles::{
  Handle, HandleScope, HeapObject, K_HANDLE_BLOCK_SIZE, Object, PersistentHandles,
  PersistentHandlesList, PersistentHandlesScope, Tagged,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 规约: 持久句柄跨 HandleScope 存活 —— scope 关闭后仍可解引用 (独立分块)
#[test]
fn persistent_handle_outlives_scope() -> Void {
  let list = PersistentHandlesList::new();
  let mut ph = PersistentHandles::new(&list);
  let handle;
  {
    let scope = HandleScope::new();
    let local = Handle::new(Tagged::<Object>::from_ptr(0x31), &scope);
    assert_eq!(local.value().ptr(), 0x31);
    // 持久分配独立于 HandleScope
    handle = ph.new_handle(Tagged::<HeapObject>::from_ptr(0x3001));
    assert!(ph.contains(handle.location()));
  }
  // scope 已关闭, 持久句柄仍有效
  assert_eq!(handle.value().ptr(), 0x3001);
  assert!(ph.contains(handle.location()));
  assert!(!list.is_empty());
  OK
}

/// 规约: new_handle_from 重分配 (原 NewHandle(Handle<T>))
#[test]
fn persistent_handle_reallocate() -> Void {
  let list = PersistentHandlesList::new();
  let mut ph = PersistentHandles::new(&list);
  let scope = HandleScope::new();
  let h = Handle::new(Tagged::<HeapObject>::from_ptr(0x3002), &scope);
  let p = ph.new_handle_from(h);
  assert_ne!(p.location(), h.location(), "重分配新槽");
  assert_eq!(p.value().ptr(), 0x3002);
  OK
}

/// 规约: 块扩展 —— 超 K_HANDLE_BLOCK_SIZE 后新块, contains 与值保持
#[test]
fn persistent_block_extension() -> Void {
  let list = PersistentHandlesList::new();
  let mut ph = PersistentHandles::new(&list);
  let total = K_HANDLE_BLOCK_SIZE + 256;
  let mut handles = Vec::with_capacity(total);
  for i in 0..total {
    handles.push(ph.new_handle(Tagged::<HeapObject>::from_ptr(0x4000 + i * 8 + 1)));
  }
  for (i, h) in handles.iter().enumerate() {
    assert_eq!(h.value().ptr(), 0x4000 + i * 8 + 1);
    assert!(ph.contains(h.location()), "句柄 #{i} 应属容器");
  }
  assert!(!ph.contains(null_mut()));
  OK
}

/// 规约: iterate 遍历槽区间 —— 非末块整块, 末块截至 block_next
#[test]
fn persistent_iterate_ranges() -> Void {
  let list = PersistentHandlesList::new();
  let mut ph = PersistentHandles::new(&list);
  let n = 5;
  for i in 0..n {
    ph.new_handle(Tagged::<HeapObject>::from_ptr(0x5000 + i));
  }
  let mut slots = 0usize;
  ph.iterate(|start, end| {
    slots += (end as usize - start as usize) / size_of::<usize>();
  });
  assert_eq!(slots, n);
  OK
}

/// 规约: 链表 Add/Remove —— 构造登记, Drop 摘除, iterate 逐容器
#[test]
fn persistent_list_registration() -> Void {
  let list = PersistentHandlesList::new();
  assert!(list.is_empty());
  let ph1 = PersistentHandles::new(&list);
  assert_eq!(list.len(), 1);
  let ph2 = PersistentHandles::new(&list);
  assert_eq!(list.len(), 2);

  let mut visited = 0;
  list.iterate(|_| visited += 1);
  assert_eq!(visited, 2);

  drop(ph1);
  assert_eq!(list.len(), 1);
  let mut seen = 0;
  list.iterate(|ph| {
    seen += 1;
    assert!(eq(ph, ph2.as_ref()), "仅剩 ph2 在链");
  });
  assert_eq!(seen, 1);
  drop(ph2);
  assert!(list.is_empty());
  OK
}

/// 规约: move 语义 —— Box 转移 (unique_ptr 移交) 后链表有效, 句柄可用
#[test]
fn persistent_move_semantics() -> Void {
  let list = PersistentHandlesList::new();
  let mut ph = PersistentHandles::new(&list);
  let handle = ph.new_handle(Tagged::<HeapObject>::from_ptr(0x6001));

  // move 转移 (原 unique_ptr 移交): 堆地址稳定, 链表侵入指针不失效
  let ph = move_container(ph);
  assert_eq!(list.len(), 1);
  assert_eq!(handle.value().ptr(), 0x6001);
  assert!(ph.contains(handle.location()));
  drop(ph);
  assert!(list.is_empty(), "move 后 Drop 仍正常摘除");
  OK
}

fn move_container(ph: Box<PersistentHandles>) -> Box<PersistentHandles> {
  ph
}

/// 规约: Attach/Detach 归属标记 (owner 线程语义)
#[test]
fn persistent_attach_detach_owner() -> Void {
  let list = PersistentHandlesList::new();
  let mut ph = PersistentHandles::new(&list);
  ph.attach();
  ph.detach();
  ph.attach();
  drop(ph);
  OK
}

/// 规约: PersistentHandlesScope —— 作用域内句柄落 persistent 块, detach 移交容器
#[test]
fn persistent_scope_detach() -> Void {
  let list = PersistentHandlesList::new();
  let scope = HandleScope::new();
  assert!(!PersistentHandlesScope::is_active());

  let ps = PersistentHandlesScope::new();
  assert!(PersistentHandlesScope::is_active());
  let h1 = Handle::new(Tagged::<Object>::from_ptr(0x7001), &scope);
  let h2 = Handle::new(Tagged::<Object>::from_ptr(0x7002), &scope);

  let ph = ps.detach(&list);
  assert!(!PersistentHandlesScope::is_active());
  // 移交后句柄仍存活
  assert_eq!(h1.value().ptr(), 0x7001);
  assert_eq!(h2.value().ptr(), 0x7002);
  assert!(ph.contains(h1.location()));
  assert!(ph.contains(h2.location()));
  // scope 在 detach 后仍可正常关闭
  drop(scope);
  assert_eq!(h1.value().ptr(), 0x7001);
  info!("persistent scope detach ok");
  OK
}

/// 规约: PersistentHandlesScope::detach 后句柄数计入容器 iterate
#[test]
fn persistent_scope_detach_counts() -> Void {
  let list = PersistentHandlesList::new();
  let scope = HandleScope::new();
  let ps = PersistentHandlesScope::new();
  let mut handles = Vec::new();
  for i in 0..3 {
    handles.push(Handle::new(Tagged::<Object>::from_ptr(0x7100 + i), &scope));
  }
  let ph = ps.detach(&list);
  let mut slots = 0;
  ph.iterate(|s, e| slots += (e as usize - s as usize) / size_of::<usize>());
  assert_eq!(slots, 3);
  drop(scope);
  let _ = handles;
  OK
}
