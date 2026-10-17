//! HandleScope 语义测试: 嵌套 Enter/Leave, 作用域隔离, 槽位复用, 块扩展, CloseAndEscape, 封印
use aok::{OK, Void};
use log::info;
use v8_handles::{Handle, HandleScope, K_HANDLE_BLOCK_SIZE, Object, SealHandleScope, Tagged};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 规约: 构造即 Enter, Drop 即 Leave; 嵌套关闭后外层计数回退
#[test]
fn nested_scopes_isolated() -> Void {
  let outer = HandleScope::new();
  let h1 = Handle::new(Tagged::<Object>::from_ptr(0x1001), &outer);
  let n_outer = HandleScope::number_of_handles();
  info!("外层句柄数 {n_outer}");

  let inner_location;
  {
    let inner = HandleScope::new();
    let h2 = Handle::new(Tagged::<Object>::from_ptr(0x2002), &inner);
    inner_location = h2.location();
    assert_eq!(HandleScope::number_of_handles(), n_outer + 1);
    assert_eq!(h2.value().ptr(), 0x2002);
  }
  // 内层关闭: 计数回退, h2 随作用域失效 (规约, 不再读)
  assert_eq!(HandleScope::number_of_handles(), n_outer);
  // 外层句柄不受内层开关影响
  assert_eq!(h1.value().ptr(), 0x1001);
  // 槽位复用: 内层回退后的槽被外层下一分配命中 (链式 next/limit 推进语义)
  let h3 = Handle::new(Tagged::<Object>::from_ptr(0x3003), &outer);
  assert_eq!(h3.location(), inner_location);
  assert_eq!(h3.value().ptr(), 0x3003);
  OK
}

/// 规约: 句柄数计数 = (块数-1)*块大小 + 末块已用槽
#[test]
fn number_of_handles_counts() -> Void {
  let scope = HandleScope::new();
  assert_eq!(HandleScope::number_of_handles(), 0);
  let _h1 = Handle::new(Tagged::<Object>::from_ptr(0x11), &scope);
  let _h2 = Handle::new(Tagged::<Object>::from_ptr(0x22), &scope);
  assert_eq!(HandleScope::number_of_handles(), 2);
  OK
}

/// 规约: 超出单块容量时 Extend 分配新块, 计数与值连续正确
#[test]
fn block_extension_beyond_one_block() -> Void {
  let scope = HandleScope::new();
  let total = K_HANDLE_BLOCK_SIZE + 512;
  let mut handles = Vec::with_capacity(total);
  for i in 0..total {
    let ptr = 0x1000 + i * 8 + 1;
    handles.push(Handle::new(Tagged::<Object>::from_ptr(ptr), &scope));
  }
  assert_eq!(HandleScope::number_of_handles(), total);
  // 跨块值抽检: 首块尾, 次块头, 末尾
  for &i in &[0, K_HANDLE_BLOCK_SIZE - 1, K_HANDLE_BLOCK_SIZE, total - 1] {
    let ptr = 0x1000 + i * 8 + 1;
    assert_eq!(handles[i].value().ptr(), ptr);
  }
  // 新块必然分配 (跨过 K_HANDLE_BLOCK_SIZE)
  assert!(
    handles[K_HANDLE_BLOCK_SIZE].location() != handles[0].location(),
    "跨块句柄应位于不同槽"
  );
  OK
}

/// 规约: CloseAndEscape 关闭当前作用域并在父作用域重分配, 逃逸句柄在内层关闭后存活
#[test]
fn close_and_escape_survives_scope() -> Void {
  let outer = HandleScope::new();
  let h0 = Handle::new(Tagged::<Object>::from_ptr(0x777), &outer);
  let mut inner = HandleScope::new();
  let h_inner = Handle::new(Tagged::<Object>::from_ptr(0xabc), &inner);
  let n_before = HandleScope::number_of_handles();

  let escaped = inner.close_and_escape(h_inner);
  drop(inner);
  // 逃逸句柄在外层存活, 值保持
  assert_eq!(escaped.value().ptr(), 0xabc);
  assert_eq!(h0.value().ptr(), 0x777);
  // 内层非逃逸句柄被抛弃, 净变化: -1 (h_inner) +1 (escaped) + 外层 h0
  assert_eq!(HandleScope::number_of_handles(), n_before);
  OK
}

/// 规约: SealHandleScope 封印后新句柄需新作用域, 解封后恢复
#[test]
fn seal_handle_scope_restriction() -> Void {
  let outer = HandleScope::new();
  let h1 = Handle::new(Tagged::<Object>::from_ptr(0x51), &outer);
  {
    let _seal = SealHandleScope::new();
    // 封印期: 新作用域中分配合法 (level > sealed_level)
    let nested = HandleScope::new();
    let h2 = Handle::new(Tagged::<Object>::from_ptr(0x52), &nested);
    assert_eq!(h2.value().ptr(), 0x52);
  }
  // 解封后原作用域继续可用
  let h3 = Handle::new(Tagged::<Object>::from_ptr(0x53), &outer);
  assert_eq!(h1.value().ptr(), 0x51);
  assert_eq!(h3.value().ptr(), 0x53);
  OK
}

/// 规约: Drop (Leave) 后句柄失效 —— 内层句柄槽位被外层复用覆盖
#[test]
fn scope_close_invalidates_handles() -> Void {
  let outer = HandleScope::new();
  let inner_location;
  {
    let inner = HandleScope::new();
    let h = Handle::new(Tagged::<Object>::from_ptr(0xdead), &inner);
    inner_location = h.location();
  }
  // 内层关闭后, 同槽位被外层复用写入新值 —— 原句柄语义失效
  let reused = Handle::new(Tagged::<Object>::from_ptr(0xbeef), &outer);
  assert_eq!(reused.location(), inner_location);
  assert_eq!(reused.value().ptr(), 0xbeef);
  OK
}

/// 规约: 三层连续 CloseAndEscape (S3 -> S2 -> S1) 逃逸句柄最终在最外层稳固存活
#[test]
fn continuous_close_and_escape_three_levels() -> Void {
  let outer = HandleScope::new();
  let h_outer = Handle::new(Tagged::<Object>::from_ptr(0x1000), &outer);

  let final_escaped = {
    let mut mid = HandleScope::new();
    let mid_discarded = Handle::new(Tagged::<Object>::from_ptr(0x2001), &mid);

    let mid_escaped = {
      let mut inner = HandleScope::new();
      let inner_discarded = Handle::new(Tagged::<Object>::from_ptr(0x3001), &inner);
      let inner_target = Handle::new(Tagged::<Object>::from_ptr(0x3002), &inner);
      assert_eq!(inner_discarded.value().ptr(), 0x3001);
      inner.close_and_escape(inner_target)
    };
    // inner 已关闭, mid_escaped 现归属 mid
    assert_eq!(mid_escaped.value().ptr(), 0x3002);
    assert_eq!(mid_discarded.value().ptr(), 0x2001);

    mid.close_and_escape(mid_escaped)
  };
  // mid 也已关闭, final_escaped 现归属 outer
  assert_eq!(final_escaped.value().ptr(), 0x3002);
  assert_eq!(h_outer.value().ptr(), 0x1000);
  assert_eq!(HandleScope::number_of_handles(), 2);
  OK
}

/// 规约: CloseAndEscape 传入空句柄时安全关闭当前作用域, 返回空句柄, 免疫 panic/UB
#[test]
fn close_and_escape_null_handle() -> Void {
  let outer = HandleScope::new();
  let mut inner = HandleScope::new();
  let _h_inner = Handle::new(Tagged::<Object>::from_ptr(0x999), &inner);

  let escaped_null = inner.close_and_escape(Handle::<Object>::null());
  assert!(escaped_null.is_null());
  // 作用域安全关闭, 计数回退
  assert_eq!(HandleScope::number_of_handles(), 0);
  let h_new = Handle::new(Tagged::<Object>::from_ptr(0x123), &outer);
  assert_eq!(h_new.value().ptr(), 0x123);
  OK
}

/// 规约: CloseAndEscape 后作用域重新初始化, 可继续分配并在同一作用域二次逃逸
#[test]
fn scope_reuse_after_close_and_escape() -> Void {
  let _outer = HandleScope::new();
  let mut inner = HandleScope::new();

  let h1 = Handle::new(Tagged::<Object>::from_ptr(0x111), &inner);
  let esc1 = inner.close_and_escape(h1);
  assert_eq!(esc1.value().ptr(), 0x111);

  // 重新初始化后继续在 inner 分配
  let h2 = Handle::new(Tagged::<Object>::from_ptr(0x222), &inner);
  let esc2 = inner.close_and_escape(h2);
  assert_eq!(esc2.value().ptr(), 0x222);

  drop(inner);
  // 两个逃逸句柄在外层均正常存活
  assert_eq!(esc1.value().ptr(), 0x111);
  assert_eq!(esc2.value().ptr(), 0x222);
  assert_eq!(HandleScope::number_of_handles(), 2);
  OK
}
