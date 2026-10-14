//! ScopedModification 作用域替换与回滚校验

use aok::{OK, Void};
use v8_common::ScopedModification;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// old_value 可读、Drop 恢复旧值 (scoped-modification.h:15-29);
/// 存续期独占 &mut, 新值经 Deref 不可取 — 仅断言回滚结果
#[test]
fn scoped_modification_revert() -> Void {
  let mut slot = 7i64;
  {
    let m = ScopedModification::new(&mut slot, 42);
    assert_eq!(*m.old_value(), 7);
  }
  assert_eq!(slot, 7);

  // 无 Default/Copy 的类型同样可回滚 (String)
  let mut s = String::from("old");
  {
    let m = ScopedModification::new(&mut s, String::from("new"));
    assert_eq!(m.old_value(), "old");
  }
  assert_eq!(s, "old");
  OK
}
