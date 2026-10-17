//! debug zap 语义测试: 回收块/回收槽区间在 debug 构建填充 zap 值
//! (原 ENABLE_LOCAL_HANDLE_ZAPPING → cfg(debug_assertions)), 常量与原版 globals.h 1:1
use std::mem::size_of;

use aok::{OK, Void};
// 封印句柄仅 debug 断言路径使用 (原 debug-only 实现语义)
#[cfg(debug_assertions)]
use v8_handles::SealHandleScope;
use v8_handles::{
  Handle, HandleScope, K_CHECK_HANDLE_THRESHOLD, K_HANDLE_BLOCK_SIZE, K_HANDLE_ZAP_VALUE,
  K_PERSISTENT_HANDLE_ZAP_VALUE, K_TAGGED_NULL_ADDRESS, Object, PersistentHandlesList,
  PersistentHandlesScope, Tagged,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 规约: debug 构建回收区填充值与原版 globals.h 字面量 1:1 (改错即暴露)
#[test]
fn zap_constants_match_original() -> Void {
  assert_eq!(K_HANDLE_ZAP_VALUE, 0x1baddead0baddeaf, "globals.h:1154");
  assert_eq!(
    K_PERSISTENT_HANDLE_ZAP_VALUE, 0x1baffed66baffedf,
    "globals.h:1156, 不得与句柄 zap 值混用"
  );
  assert_eq!(K_TAGGED_NULL_ADDRESS, 0x1, "handles.h:53");
  assert_eq!(
    K_HANDLE_BLOCK_SIZE, 1022,
    "handle-scope-implementer.h:128 KB-2"
  );
  assert_eq!(K_CHECK_HANDLE_THRESHOLD, 48 * 1024, "handles.h:315");
  OK
}

/// 规约: 顶层作用域关闭 → 两块均回收 (次块先释放, 首块 zap 后留 spare),
/// 新作用域首分配命中复用块 slot0, slot1.. 残留 zap 值
/// (zap 行为仅 debug 构建 —— 原版 ENABLE_LOCAL_HANDLE_ZAPPING 同样仅 debug 启用)
#[cfg(debug_assertions)]
#[test]
fn recycled_block_zapped_before_spare_reuse() -> Void {
  let block0;
  {
    let scope = HandleScope::new();
    let first = Handle::new(Tagged::<Object>::from_ptr(0x1001), &scope);
    block0 = first.location();
    let total = K_HANDLE_BLOCK_SIZE + 10;
    for i in 1..total {
      Handle::new(Tagged::<Object>::from_ptr(0x1000 + i * 8 + 1), &scope);
    }
    assert_eq!(HandleScope::number_of_handles(), total);
  }
  // 顶层关闭: delete_extensions 逐块 pop → 整块 zap → 首块留 spare
  {
    let scope = HandleScope::new();
    let h = Handle::new(Tagged::<Object>::from_ptr(0x2001), &scope);
    // 复用 spare 块: 首分配即落在原首块 slot0
    assert_eq!(h.location(), block0, "spare 复用应命中回收的首块");
    // 未覆写槽位残留 zap 值 (pop 分支 ZapRange(block_start, block_limit))
    // SAFETY: block0 为复用块首, 槽 1..=8 在块容量内且未被新分配覆写
    for i in 1..=8 {
      assert_eq!(
        unsafe { *block0.add(i) },
        K_HANDLE_ZAP_VALUE,
        "槽 {i} 应残留 zap 值"
      );
    }
    assert_eq!(h.value().ptr(), 0x2001);
  }
  OK
}

/// 规约: HandleScope::iterate_handles 遍历存活槽区间 —— 总数 = 已分配句柄数,
/// persistent 作用域开启时分界块截至分界点 (原 IterateThis 的 persistent 特判)
#[test]
fn iterate_handles_ranges_and_persistent_boundary() -> Void {
  let list = PersistentHandlesList::new();
  let scope = HandleScope::new();
  for i in 0..K_HANDLE_BLOCK_SIZE + 4 {
    Handle::new(Tagged::<Object>::from_ptr(0x6000 + i), &scope);
  }
  let total = HandleScope::number_of_handles();
  let slots = count_slots();
  assert_eq!(slots, total, "非 persistent 时全部存活句柄入扫");

  // persistent 作用域: 分界点 = 进入前 next (首块 slot K_HANDLE_BLOCK_SIZE-? 处)
  // 进入前已分配 total 个, 分界指针在末块内 → 分界块截至分界点, 末块(persistent 块)整块入扫
  let ps = PersistentHandlesScope::new();
  for i in 0..3 {
    Handle::new(Tagged::<Object>::from_ptr(0x7000 + i), &scope);
  }
  let after = count_slots();
  // 分界块(persistent 前的最后普通块)截断至分界点: 计数 = total(分界前存活) + 3(persistent 块内)
  assert_eq!(
    after,
    total + 3,
    "persistent 分界: 前段截至分界点 + persistent 块整块"
  );
  let _ph = ps.detach(&list);
  drop(scope);
  OK
}

fn count_slots() -> usize {
  let mut slots = 0usize;
  HandleScope::iterate_handles(|s, e| slots += (e as usize - s as usize) / size_of::<usize>());
  slots
}

/// 规约: persistent 分界指针恰为块尾 (进入前恰好填满一块) —— 分界块判定须含块尾边界
/// (原版 IterateThis 用闭区间 lhb <= &block[kHandleBlockSize] && lhb >= block,
///  块尾时访问 [block, lhb) 即整块; 半开区间漏判会误报 debug 断言)
#[test]
fn iterate_handles_persistent_boundary_at_block_end() -> Void {
  let list = PersistentHandlesList::new();
  let scope = HandleScope::new();
  // 恰好填满首块: next == 首块尾
  for i in 0..K_HANDLE_BLOCK_SIZE {
    Handle::new(Tagged::<Object>::from_ptr(0x8000 + i), &scope);
  }
  // persistent 起点 = 首块尾 (块尾边界)
  let ps = PersistentHandlesScope::new();
  for i in 0..2 {
    Handle::new(Tagged::<Object>::from_ptr(0x9000 + i), &scope);
  }
  let slots = count_slots();
  // 首块整块 (分界 = 块尾) + persistent 块 2 槽
  assert_eq!(slots, K_HANDLE_BLOCK_SIZE + 2, "分界在块尾: 首块整块入扫");
  let _ph = ps.detach(&list);
  drop(scope);
  OK
}

/// 规约: 块内作用域关闭 (未扩块, limit 未变) → 本作用域分配的槽位填 zap 值
/// (原版 CloseScope std::swap 后 limit = 关闭时 next, ZapRange(进入 next, 关闭 next);
///  无扩展时 DeleteExtensions 不介入, 仅靠此处 zap 覆盖块内已用槽)
#[cfg(debug_assertions)]
#[test]
fn inner_scope_close_zaps_in_block_slots() -> Void {
  let outer = HandleScope::new();
  let _h0 = Handle::new(Tagged::<Object>::from_ptr(0x1), &outer);
  let loc;
  {
    let inner = HandleScope::new();
    let h = Handle::new(Tagged::<Object>::from_ptr(0xdead), &inner);
    loc = h.location();
  }
  // 内层关闭: 进入 next = loc, 关闭 next = loc+1, limit 未变 → 原版 zap [loc, loc+1)
  // SAFETY: loc 在 arena 持有块内, 外层作用域存活
  assert_eq!(
    unsafe { *loc },
    K_HANDLE_ZAP_VALUE,
    "内层关闭后本作用域槽位应填 zap 值"
  );
  // 外层句柄不受影响
  assert_eq!(_h0.value().ptr(), 0x1);
  OK
}

/// 规约: SealHandleScope 使 prev_limit 落在末块内 —— DeleteExtensions break 分支
/// zap [prev_limit, 块尾), 内层已写值被覆盖 (固化缺陷: break 分支 zap 缺失)
#[cfg(debug_assertions)]
#[test]
fn seal_break_zaps_block_tail() -> Void {
  let scope = HandleScope::new();
  // 填满首块 + 次块前 2 槽
  let total = K_HANDLE_BLOCK_SIZE + 2;
  for i in 0..total {
    Handle::new(Tagged::<Object>::from_ptr(0x3000 + i), &scope);
  }
  // seal 点 = 当前 next (次块 slot2)
  // SAFETY: 由 number_of_handles 反推 next 槽位
  let seal_point = {
    let probe = Handle::new(Tagged::<Object>::from_ptr(0x7), &scope);
    let p = probe.location();
    // probe 落在 slot2, seal 点在其后一格
    // SAFETY: 块内偏移 +1 不越界
    unsafe { p.add(1) }
  };
  {
    let _seal = SealHandleScope::new();
    {
      let inner = HandleScope::new();
      let a = Handle::new(Tagged::<Object>::from_ptr(0xAA01), &inner);
      let _b = Handle::new(Tagged::<Object>::from_ptr(0xAA02), &inner);
      let _c = Handle::new(Tagged::<Object>::from_ptr(0xAA03), &inner);
      // 内层首分配命中 seal 点 (Extend 把 limit 拉回末块尾后继续用块内余量)
      assert_eq!(a.location(), seal_point);
    }
    // inner 关闭: limit 曾被 Extend 拉伸至末块尾 ≠ prev_limit(seal 点)
    // → delete_extensions: prev_limit 落末块内 → break 前 zap [seal 点, 块尾)
  }
  // seal 点后一格曾在内层写入 0xAA02, 现应为 zap 值
  // SAFETY: seal 点在块内, +1 不越界; 块仍被 arena 持有 (seal 未关, 作用域存活)
  assert_eq!(unsafe { *seal_point.add(1) }, K_HANDLE_ZAP_VALUE);
  // 外层句柄不受影响
  let after = Handle::new(Tagged::<Object>::from_ptr(0x99), &scope);
  assert_eq!(after.value().ptr(), 0x99);
  OK
}

/// 规约: 无先前句柄时开启 PersistentHandlesScope (last_handle 为 null)
/// iterate_handles 与 detach 均稳定通过, 不触发分界断言失败
#[test]
fn persistent_scope_without_prior_handles_iterate() -> Void {
  let list = PersistentHandlesList::new();
  let scope = HandleScope::new();
  // 此时无任何句柄分配, data.next 为 null
  let ps = PersistentHandlesScope::new();
  assert!(PersistentHandlesScope::is_active());
  // 作用域内分配一个持久句柄
  let h = Handle::new(Tagged::<Object>::from_ptr(0xbeef_0001), &scope);
  assert_eq!(h.value().ptr(), 0xbeef_0001);

  // 遍历存活句柄: 此时分界指针为 null, 仅末块 1 个句柄入扫
  let slots = count_slots();
  assert_eq!(slots, 1);

  let ph = ps.detach(&list);
  assert!(ph.contains(h.location()));
  drop(scope);
  OK
}
