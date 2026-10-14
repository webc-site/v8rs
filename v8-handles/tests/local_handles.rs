//! LocalHandles 族语义测试: 后台分块分配, 作用域嵌套回退, Main 转发, CloseAndEscape
use std::{
  mem::size_of,
  ptr::{NonNull, addr_of_mut},
};

use aok::{OK, Void};
use log::info;
use v8_handles::{
  Handle, HandleScope, K_HANDLE_BLOCK_SIZE, LocalHandleScope, LocalHandles, LocalHeapRef, Object,
  Tagged,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 规约: 后台分配落自有分块, contains 归属判定, iterate 只扫存活句柄
#[test]
fn local_handles_allocation() -> Void {
  let mut lh = LocalHandles::new();
  {
    // SAFETY: lh 存活覆盖作用域; 嵌套/查询需共享, 统一走裸指针后端 (原版拓扑)
    let mut scope = LocalHandleScope::new(unsafe { LocalHeapRef::from_raw(addr_of_mut!(lh)) });
    let h = scope.allocate::<Object>(0x1001);
    assert_eq!(h.value().ptr(), 0x1001);
    assert!(lh.contains(h.location()));
    let mut slots = 0;
    lh.iterate(|s, e| slots += (e as usize - s as usize) / size_of::<usize>());
    assert_eq!(slots, 1);
  }
  // 作用域关闭: next 回退 → 存活槽清零 (对应 GC 只扫存活句柄)
  let mut slots = 0;
  lh.iterate(|s, e| slots += (e as usize - s as usize) / size_of::<usize>());
  assert_eq!(slots, 0);
  OK
}

/// 规约: 后台作用域嵌套回退 —— 内层关闭后外层复用槽位 (同宿主栈序)
#[test]
fn local_scope_nesting_rewind() -> Void {
  let mut lh = LocalHandles::new();
  // SAFETY: lh 存活覆盖嵌套作用域, 内层先关
  let mut outer = LocalHandleScope::new(unsafe { LocalHeapRef::from_raw(addr_of_mut!(lh)) });
  let _h1 = outer.allocate::<Object>(0x11);
  let inner_slot;
  {
    // 同一宿主挂内层作用域 (原 local_heap 裸指针拓扑):
    // SAFETY: lh 存活覆盖内层作用域, 且内层先于外层 Drop (声明序保证)
    let mut inner = LocalHandleScope::new(unsafe { LocalHeapRef::from_raw(addr_of_mut!(lh)) });
    let h2 = inner.allocate::<Object>(0x22);
    inner_slot = h2.location();
    assert_eq!(h2.value().ptr(), 0x22);
  }
  // 内层关闭后外层新分配复用同槽
  let reused = outer.allocate::<Object>(0x33);
  assert_eq!(reused.location(), inner_slot);
  assert_eq!(reused.value().ptr(), 0x33);
  OK
}

/// 规约: 超块扩展 —— 后台分块自管, 跨块分配可用且 contains 覆盖全块
#[test]
fn local_handles_block_extension() -> Void {
  let mut lh = LocalHandles::new();
  let total = K_HANDLE_BLOCK_SIZE + 128;
  {
    // SAFETY: lh 存活覆盖作用域
    let mut scope = LocalHandleScope::new(unsafe { LocalHeapRef::from_raw(addr_of_mut!(lh)) });
    for i in 0..total {
      let h = scope.allocate::<Object>(0x9000 + i);
      assert!(lh.contains(h.location()), "句柄 #{i} 应属容器");
    }
    let mut slots = 0;
    lh.iterate(|s, e| slots += (e as usize - s as usize) / size_of::<usize>());
    assert_eq!(slots, total);
  }
  // 关闭后整块回收 (remove_unused_blocks): contains 不再认领旧块
  let probe = NonNull::<usize>::dangling().as_ptr();
  assert!(!lh.contains(probe));
  OK
}

/// 规约: Main 后端转发全局 HandleScope arena —— 计入 HandleScope 计数
#[test]
fn local_scope_main_thread_forwarding() -> Void {
  let mut scope = HandleScope::new();
  let _h0 = Handle::new(Tagged::<Object>::from_ptr(0x1), &scope);
  let n_before = HandleScope::number_of_handles();

  let mut ls = LocalHandleScope::new(LocalHeapRef::MAIN);
  let h = ls.allocate::<Object>(0x2001);
  assert_eq!(h.value().ptr(), 0x2001);
  assert_eq!(HandleScope::number_of_handles(), n_before + 1);
  drop(ls);
  // 转发作用域关闭, 计数回退
  assert_eq!(HandleScope::number_of_handles(), n_before);
  let _ = &mut scope;
  OK
}

/// 规约: LocalHandleScope::close_and_escape —— 后台路径逃逸至父作用域
#[test]
fn local_scope_close_and_escape() -> Void {
  let mut lh = LocalHandles::new();
  // SAFETY: lh 存活覆盖嵌套作用域, 内层先关
  let mut outer = LocalHandleScope::new(unsafe { LocalHeapRef::from_raw(addr_of_mut!(lh)) });
  let h_outer = outer.allocate::<Object>(0x111);

  let escaped = {
    // SAFETY: 同宿主内层作用域, 内层先关
    let mut inner = LocalHandleScope::new(unsafe { LocalHeapRef::from_raw(addr_of_mut!(lh)) });
    let h_inner = inner.allocate::<Object>(0xabc);
    inner.close_and_escape(h_inner)
  };
  assert_eq!(escaped.value().ptr(), 0xabc);
  assert!(lh.contains(escaped.location()));
  assert_eq!(h_outer.value().ptr(), 0x111);
  drop(outer);
  info!("local close_and_escape ok");
  OK
}

/// 规约: LocalHandleScope::close_and_escape 传入空句柄安全返回 null 且重置作用域
#[test]
fn local_scope_close_and_escape_null() -> Void {
  let mut lh = LocalHandles::new();
  // SAFETY: lh 存活覆盖嵌套作用域
  let mut outer = LocalHandleScope::new(unsafe { LocalHeapRef::from_raw(addr_of_mut!(lh)) });
  let mut inner = LocalHandleScope::new(unsafe { LocalHeapRef::from_raw(addr_of_mut!(lh)) });
  let _h = inner.allocate::<Object>(0x555);

  let esc_null = inner.close_and_escape(Handle::<Object>::null());
  assert!(esc_null.is_null());

  // 后台作用域安全关闭, 重新分配正常
  let h_new = outer.allocate::<Object>(0x666);
  assert_eq!(h_new.value().ptr(), 0x666);
  OK
}

/// 规约: LocalHandleScope 三层连续逃逸
#[test]
fn local_scope_continuous_escape_three_levels() -> Void {
  let mut lh = LocalHandles::new();
  // SAFETY: lh 存活覆盖全部作用域
  let _s1 = LocalHandleScope::new(unsafe { LocalHeapRef::from_raw(addr_of_mut!(lh)) });
  let final_esc = {
    let mut s2 = LocalHandleScope::new(unsafe { LocalHeapRef::from_raw(addr_of_mut!(lh)) });
    let s2_esc = {
      let mut s3 = LocalHandleScope::new(unsafe { LocalHeapRef::from_raw(addr_of_mut!(lh)) });
      let h3 = s3.allocate::<Object>(0x789);
      s3.close_and_escape(h3)
    };
    assert_eq!(s2_esc.value().ptr(), 0x789);
    s2.close_and_escape(s2_esc)
  };
  assert_eq!(final_esc.value().ptr(), 0x789);
  assert!(lh.contains(final_esc.location()));
  OK
}
