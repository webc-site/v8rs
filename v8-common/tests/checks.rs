//! 原版 v8/src/common/checks.h DCHECK_TAG_ALIGNED 族校验 (上游无专属 unittest)
//!
//! 断言规约清单 (先规约后断言, 强度守恒):
//! - dcheck_tag_aligned_accepts_aligned: 2 字节 tag 对齐地址/尺寸通过
//!   (checks.h:44-48, `DCHECK_TAG_ALIGNED` 语义 = address & kHeapObjectTagMask == 0)
//! - dcheck_tag_aligned_rejects_untagged: debug 构建下未对齐即失败 —
//!   原 DCHECK abort → Rust debug_assert panic, catch_unwind 断言其终止性
//!   (release 构建原版为 no-op, 断言随之豁免)
//! - slow_dcheck_condition_not_evaluated: SLOW_DCHECK 禁用分支条件不求值
//!   (checks.h:21-22 `((void)0)`, round2-F1 惰性闭包回归)
//! - dcheck_with_sandbox_access_laziness: DCHECK_WITH_SANDBOX_ACCESS 族
//!   release 空操作分支条件不求值 / debug 分支必求值 (checks.h:38-42,
//!   round2-F1 惰性闭包回归)

use core::cell::Cell;
use std::panic;

use aok::{OK, Void};
use v8_common::{
  dcheck_size_tag_aligned, dcheck_tag_aligned, dcheck_with_sandbox_access,
  dcheck_with_sandbox_access_and_msg, slow_dcheck, slow_dcheck_implies,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

fn quiet_catch_unwind<F: FnOnce() + panic::UnwindSafe>(f: F) -> bool {
  let prev = panic::take_hook();
  panic::set_hook(Box::new(|_| {}));
  let res = panic::catch_unwind(f);
  panic::set_hook(prev);
  res.is_err()
}

/// 对齐输入不触发断言
#[test]
fn checks_dcheck_tag_aligned_accepts_aligned() -> Void {
  dcheck_tag_aligned(0);
  dcheck_tag_aligned(0x1000);
  dcheck_size_tag_aligned(0);
  dcheck_size_tag_aligned(0x100);
  OK
}

/// 未对齐输入在 debug 构建必须失败 (等价原版 DCHECK abort)
#[test]
fn checks_dcheck_tag_aligned_rejects_untagged() -> Void {
  if cfg!(debug_assertions) {
    assert!(quiet_catch_unwind(|| dcheck_tag_aligned(1)));
    assert!(quiet_catch_unwind(|| dcheck_size_tag_aligned(1)));
  } else {
    // release: 原版宏为空操作, Rust debug_assert 亦不编译
    dcheck_tag_aligned(1);
    dcheck_size_tag_aligned(1);
  }
  OK
}

/// [回归 round2-F1] SLOW_DCHECK 禁用分支条件不求值 (checks.h:21-22
/// `((void)0)`) — 闭包入参在空操作路径不得被调用 (副作用/开销不泄漏)
#[test]
fn checks_slow_dcheck_condition_not_evaluated() -> Void {
  let calls = Cell::new(0u32);
  let count = || {
    calls.set(calls.get() + 1);
    true
  };
  slow_dcheck(count);
  slow_dcheck_implies(count, count);
  assert_eq!(calls.get(), 0, "空操作路径条件不得求值");
  OK
}

/// [回归 round2-F1] DCHECK_WITH_SANDBOX_ACCESS 族惰性闭包: release 分支
/// (checks.h:38-42 `((void)0)`) 条件不求值; debug 分支 (= DCHECK) 条件必求值
#[test]
fn checks_dcheck_with_sandbox_access_laziness() -> Void {
  let calls = Cell::new(0u32);
  let count = || {
    calls.set(calls.get() + 1);
    true
  };
  dcheck_with_sandbox_access(count);
  dcheck_with_sandbox_access_and_msg(count);
  if cfg!(debug_assertions) {
    assert_eq!(calls.get(), 2, "debug 分支条件必须求值 (原 DCHECK)");
  } else {
    assert_eq!(calls.get(), 0, "release 空操作分支条件不得求值");
  }
  OK
}
