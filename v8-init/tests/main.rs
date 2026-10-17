use aok::{OK, Void};
use v8_init::{V8StartupState, is_read_only_or_shared_sentinel};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_v8_startup_state_transitions() -> Void {
  // 验证状态机单步按序推进全周期 (Idle -> PlatformDisposed -> None)
  let mut cur = V8StartupState::Idle;
  let mut count = 1;

  while let Some(next_state) = cur.next() {
    assert!(cur.can_transition_to(next_state));
    // 跨步推进应被禁止
    if let Some(future_state) = next_state.next() {
      assert!(!cur.can_transition_to(future_state));
    }
    // 倒退推进应被禁止
    assert!(!next_state.can_transition_to(cur));

    cur = next_state;
    count += 1;
  }

  // 终态检查
  assert_eq!(cur, V8StartupState::PlatformDisposed);
  assert_eq!(count, V8StartupState::COUNT);
  assert!(cur.next().is_none());

  OK
}

#[test]
fn test_isolate_group_sentinel() -> Void {
  // 验证 RO/共享页全局哨兵识别与普通地址隔离
  assert!(is_read_only_or_shared_sentinel(u64::MAX));
  assert!(!is_read_only_or_shared_sentinel(0));
  assert!(!is_read_only_or_shared_sentinel(0x1000));
  assert!(!is_read_only_or_shared_sentinel(0x7fff_ffff_ffff));

  OK
}
