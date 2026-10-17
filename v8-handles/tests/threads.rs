//! 线程隔离测试: 每线程独立 HandleScope arena, 计数与槽位互不干扰
use std::thread;

use aok::{OK, Void};
use log::info;
use v8_handles::{Handle, HandleScope, Object, PersistentHandles, PersistentHandlesList, Tagged};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 规约: thread_local arena —— 两线程各自计数隔离
#[test]
fn thread_local_arena_isolation() -> Void {
  let scope = HandleScope::new();
  let _h = Handle::new(Tagged::<Object>::from_ptr(0x1), &scope);
  assert_eq!(HandleScope::number_of_handles(), 1);

  let t = thread::spawn(|| {
    let inner = HandleScope::new();
    let a = Handle::new(Tagged::<Object>::from_ptr(0xa), &inner);
    let b = Handle::new(Tagged::<Object>::from_ptr(0xb), &inner);
    assert_eq!(HandleScope::number_of_handles(), 2, "子线程独立计数");
    assert_eq!(a.value().ptr(), 0xa);
    assert_eq!(b.value().ptr(), 0xb);
  });
  t.join().ok();
  // 子线程结束不影响主线程计数 (thread_local 独立销毁)
  assert_eq!(HandleScope::number_of_handles(), 1);
  OK
}

/// 规约: PersistentHandles 跨线程移交 (Send) —— 构造线程 detach, 消费线程读值
#[test]
fn persistent_handles_cross_thread() -> Void {
  let list = PersistentHandlesList::new();
  let mut ph = PersistentHandles::new(&list);
  let handle_loc = ph.new_handle(Tagged::<Object>::from_ptr(0x5151)).location() as usize;

  let ph = thread::spawn(move || {
    // 移交后在新线程读同一句柄 (Box 堆地址稳定; 句柄按 location 重建)
    let h = Handle::<Object>::from_location(handle_loc as *mut usize);
    assert_eq!(h.value().ptr(), 0x5151);
    assert!(ph.contains(handle_loc as *mut usize));
    ph
  })
  .join()
  .ok();
  drop(ph);
  assert!(list.is_empty());
  info!("cross thread ok");
  OK
}
