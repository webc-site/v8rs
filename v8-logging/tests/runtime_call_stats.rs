//! `src/runtime_call_stats.rs` 测试。1:1 规约来源:
//! `v8/test/unittests/logging/runtime-call-stats-unittest.cc` 的
//! `RuntimeCallStatsTest` 中不依赖 NativeContext/RunJS 的 9 用例。
//!
//! 跳过负登记(需引擎闭环, M4+ 回收): `BasicJavaScript`/`FunctionLengthGetter`/
//! `CallbackFunction`/`ApiGetter`/`GarbageCollection`(RunJS/API hook)。

use std::{
  ptr,
  sync::atomic::{AtomicI64, Ordering},
};

use aok::{OK, Void};
use log::info;
use parking_lot::Mutex;
use v8_logging::{
  RuntimeCallStats, RuntimeCallTimer, ThreadType, TimeTicks, change_current_runtime_counter, now,
  rcs_scope, reset_now, set_now,
};

// 串行锁:NOW 时间源/RUNTIME_STATS 为进程级全局(对齐 gtest 二进制内串行执行)。
static TEST_LOCK: Mutex<()> = Mutex::new(());

/// 对应 `runtime_call_stats_test_time_`(runtime-call-stats-unittest.cc:25)。
static TEST_TIME_US: AtomicI64 = AtomicI64::new(0);

/// 对应 `RuntimeCallStatsTestNow`(runtime-call-stats-unittest.cc:27-29)。
fn test_now() -> TimeTicks {
  TimeTicks::from_internal_value(TEST_TIME_US.load(Ordering::Relaxed))
}

/// 对应 fixture `RuntimeCallStatsTest`:
/// ctor 置 `RUNTIME_STATS = ENABLED_BY_NATIVE` + `Sleep(1)` + `Reset()`;
/// dtor 置回 0(防其他用例/收尾打印)。Sleep 即推进注入时钟。
struct Fixture {
  stats: RuntimeCallStats,
  _guard: StatsGuard,
}

struct StatsGuard;

impl Drop for StatsGuard {
  fn drop(&mut self) {
    v8_logging::RUNTIME_STATS.store(0, Ordering::Relaxed);
    reset_now();
  }
}

/// 对应 `Sleep(microseconds)` 时钟推进(runtime-call-stats-unittest.cc:112-116)。
fn advance_clock(us: i64) {
  TEST_TIME_US.fetch_add(us, Ordering::Relaxed);
}

impl Fixture {
  fn new() -> Self {
    // gtest 二进制内用例串行;Rust 以互斥锁等价(全局 NOW/RUNTIME_STATS)。
    let _lock = TEST_LOCK.lock();
    v8_logging::RUNTIME_STATS.store(v8_logging::ENABLED_BY_NATIVE, Ordering::Relaxed);
    // 时钟需非零(0 会被 timer 视为未启动);对应 fixture `Sleep(1)`。
    advance_clock(1);
    let stats = RuntimeCallStats::new(ThreadType::MainIsolateThread);
    stats.reset();
    set_now(test_now);
    Self {
      stats,
      _guard: StatsGuard,
    }
  }

  /// 对应 `Sleep(microseconds)`:推进注入时钟(runtime-call-stats-unittest.cc:112-116)。
  fn sleep_us(&self, us: i64) {
    advance_clock(us);
  }

  fn counter_id(&self) -> usize {
    v8_logging::K_TEST_COUNTER1
  }
  fn counter_id2(&self) -> usize {
    v8_logging::K_TEST_COUNTER2
  }
  fn counter_id3(&self) -> usize {
    v8_logging::K_TEST_COUNTER3
  }
  fn counter(&self) -> &v8_logging::RuntimeCallCounter {
    self.stats.get_counter(self.counter_id())
  }
  fn counter2(&self) -> &v8_logging::RuntimeCallCounter {
    self.stats.get_counter(self.counter_id2())
  }
  fn counter3(&self) -> &v8_logging::RuntimeCallCounter {
    self.stats.get_counter(self.counter_id3())
  }
}

// TEST_F(RuntimeCallStatsTest, RuntimeCallTimer)(runtime-call-stats-unittest.cc:141-158)
#[test]
fn runtime_call_stats_test_runtime_call_timer() -> Void {
  let f = Fixture::new();
  let timer = RuntimeCallTimer::new();

  f.sleep_us(50);
  f.stats.enter(&timer, f.counter_id());
  assert!(ptr::eq(timer.counter().unwrap(), f.counter()));
  assert!(timer.parent().is_none());
  assert!(timer.is_started());
  assert!(ptr::eq(f.stats.current_timer().unwrap(), &timer));

  f.sleep_us(100);

  f.stats.leave(&timer);
  f.sleep_us(50);
  assert!(!timer.is_started());
  assert_eq!(f.counter().count(), 1);
  assert_eq!(f.counter().time().in_microseconds(), 100);
  OK
}

// TEST_F(RuntimeCallStatsTest, RuntimeCallTimerSubTimer)(:160-204)
#[test]
fn runtime_call_stats_test_runtime_call_timer_sub_timer() -> Void {
  let f = Fixture::new();
  let timer = RuntimeCallTimer::new();
  let timer2 = RuntimeCallTimer::new();

  f.stats.enter(&timer, f.counter_id());
  assert!(timer.is_started());
  assert!(!timer2.is_started());
  assert!(ptr::eq(timer.counter().unwrap(), f.counter()));
  assert!(timer.parent().is_none());
  assert!(ptr::eq(f.stats.current_timer().unwrap(), &timer));

  f.sleep_us(50);

  f.stats.enter(&timer2, f.counter_id2());
  // timer 1 is paused, while timer 2 is active.
  assert!(timer2.is_started());
  assert!(ptr::eq(timer.counter().unwrap(), f.counter()));
  assert!(ptr::eq(timer2.counter().unwrap(), f.counter2()));
  assert!(timer.parent().is_none());
  assert!(ptr::eq(timer2.parent().unwrap(), &timer));
  assert!(ptr::eq(f.stats.current_timer().unwrap(), &timer2));

  f.sleep_us(100);
  f.stats.leave(&timer2);

  // The subtimer subtracts its time from the parent timer.
  assert!(timer.is_started());
  assert!(!timer2.is_started());
  assert_eq!(f.counter().count(), 0);
  assert_eq!(f.counter2().count(), 1);
  assert_eq!(f.counter().time().in_microseconds(), 0);
  assert_eq!(f.counter2().time().in_microseconds(), 100);
  assert!(ptr::eq(f.stats.current_timer().unwrap(), &timer));

  f.sleep_us(100);

  f.stats.leave(&timer);
  assert!(!timer.is_started());
  assert_eq!(f.counter().count(), 1);
  assert_eq!(f.counter2().count(), 1);
  assert_eq!(f.counter().time().in_microseconds(), 150);
  assert_eq!(f.counter2().time().in_microseconds(), 100);
  assert!(f.stats.current_timer().is_none());
  OK
}

// TEST_F(RuntimeCallStatsTest, RuntimeCallTimerRecursive)(:205-238)
#[test]
fn runtime_call_stats_test_runtime_call_timer_recursive() -> Void {
  let f = Fixture::new();
  let timer = RuntimeCallTimer::new();
  let timer2 = RuntimeCallTimer::new();

  f.stats.enter(&timer, f.counter_id());
  assert!(ptr::eq(timer.counter().unwrap(), f.counter()));
  assert!(timer.parent().is_none());
  assert!(timer.is_started());
  assert!(ptr::eq(f.stats.current_timer().unwrap(), &timer));

  f.stats.enter(&timer2, f.counter_id());
  assert!(ptr::eq(timer2.counter().unwrap(), f.counter()));
  assert!(timer.parent().is_none());
  assert!(ptr::eq(timer2.parent().unwrap(), &timer));
  assert!(timer2.is_started());
  assert!(ptr::eq(f.stats.current_timer().unwrap(), &timer2));

  f.sleep_us(50);

  f.stats.leave(&timer2);
  assert!(timer.parent().is_none());
  assert!(!timer2.is_started());
  assert!(timer.is_started());
  assert_eq!(f.counter().count(), 1);
  assert_eq!(f.counter().time().in_microseconds(), 50);

  f.sleep_us(100);

  f.stats.leave(&timer);
  assert!(!timer.is_started());
  assert_eq!(f.counter().count(), 2);
  assert_eq!(f.counter().time().in_microseconds(), 150);
  OK
}

// TEST_F(RuntimeCallStatsTest, RuntimeCallTimerScope)(:239-254)
#[test]
fn runtime_call_stats_test_runtime_call_timer_scope() -> Void {
  let f = Fixture::new();
  {
    let _scope = rcs_scope(&f.stats, f.counter_id());
    f.sleep_us(50);
  }
  f.sleep_us(100);
  assert_eq!(f.counter().count(), 1);
  assert_eq!(f.counter().time().in_microseconds(), 50);
  {
    let _scope = rcs_scope(&f.stats, f.counter_id());
    f.sleep_us(50);
  }
  assert_eq!(f.counter().count(), 2);
  assert_eq!(f.counter().time().in_microseconds(), 100);
  OK
}

// TEST_F(RuntimeCallStatsTest, RuntimeCallTimerScopeRecursive)(:255-271)
#[test]
fn runtime_call_stats_test_runtime_call_timer_scope_recursive() -> Void {
  let f = Fixture::new();
  {
    let _scope = rcs_scope(&f.stats, f.counter_id());
    f.sleep_us(50);
    assert_eq!(f.counter().count(), 0);
    assert_eq!(f.counter().time().in_microseconds(), 0);
    {
      let _inner = rcs_scope(&f.stats, f.counter_id());
      f.sleep_us(50);
    }
    assert_eq!(f.counter().count(), 1);
    assert_eq!(f.counter().time().in_microseconds(), 50);
  }
  assert_eq!(f.counter().count(), 2);
  assert_eq!(f.counter().time().in_microseconds(), 100);
  OK
}

// TEST_F(RuntimeCallStatsTest, RenameTimer)(:272-296)
#[test]
fn runtime_call_stats_test_rename_timer() -> Void {
  let f = Fixture::new();
  {
    let _scope = rcs_scope(&f.stats, f.counter_id());
    f.sleep_us(50);
    assert_eq!(f.counter().count(), 0);
    assert_eq!(f.counter2().count(), 0);
    assert_eq!(f.counter().time().in_microseconds(), 0);
    assert_eq!(f.counter2().time().in_microseconds(), 0);
    {
      let _inner = rcs_scope(&f.stats, f.counter_id());
      f.sleep_us(100);
    }
    // CHANGE_CURRENT_RUNTIME_COUNTER(stats(), kTestCounter2)
    change_current_runtime_counter(Some(&f.stats), f.counter_id2());
    assert_eq!(f.counter().count(), 1);
    assert_eq!(f.counter2().count(), 0);
    assert_eq!(f.counter().time().in_microseconds(), 100);
    assert_eq!(f.counter2().time().in_microseconds(), 0);
  }
  assert_eq!(f.counter().count(), 1);
  assert_eq!(f.counter2().count(), 1);
  assert_eq!(f.counter().time().in_microseconds(), 100);
  assert_eq!(f.counter2().time().in_microseconds(), 50);
  OK
}

// TEST_F(RuntimeCallStatsTest, BasicPrintAndSnapshot)(:297-320)
#[test]
fn runtime_call_stats_test_basic_print_and_snapshot() -> Void {
  let f = Fixture::new();
  let mut out: Vec<u8> = Vec::new();
  f.stats.print(&mut out).unwrap();
  assert_eq!(f.counter().count(), 0);
  assert_eq!(f.counter2().count(), 0);
  assert_eq!(f.counter3().count(), 0);
  assert_eq!(f.counter().time().in_microseconds(), 0);
  assert_eq!(f.counter2().time().in_microseconds(), 0);
  assert_eq!(f.counter3().time().in_microseconds(), 0);

  {
    let _scope = rcs_scope(&f.stats, f.counter_id());
    f.sleep_us(50);
    f.stats.print(&mut out).unwrap();
  }
  f.stats.print(&mut out).unwrap();
  assert_eq!(f.counter().count(), 1);
  assert_eq!(f.counter2().count(), 0);
  assert_eq!(f.counter3().count(), 0);
  assert_eq!(f.counter().time().in_microseconds(), 50);
  assert_eq!(f.counter2().time().in_microseconds(), 0);
  assert_eq!(f.counter3().time().in_microseconds(), 0);
  // Print 输出确有表头与数据行(ostringstream 等价物非空)。
  let text = String::from_utf8(out).unwrap();
  assert!(text.contains("Runtime Function/C++ Builtin"));
  assert!(text.contains("TestCounter1"));
  OK
}

// TEST_F(RuntimeCallStatsTest, PrintAndSnapshot)(:321-369)
#[test]
fn runtime_call_stats_test_print_and_snapshot() -> Void {
  let f = Fixture::new();
  {
    let _scope = rcs_scope(&f.stats, f.counter_id());
    f.sleep_us(100);
    assert_eq!(f.counter().count(), 0);
    assert_eq!(f.counter().time().in_microseconds(), 0);
    {
      let _inner = rcs_scope(&f.stats, f.counter_id2());
      assert_eq!(f.counter2().count(), 0);
      assert_eq!(f.counter2().time().in_microseconds(), 0);
      f.sleep_us(50);

      // Snapshot 同步提交整条 timer 栈(:338-342)。
      let mut out: Vec<u8> = Vec::new();
      f.stats.print(&mut out).unwrap();
      assert_eq!(f.counter().count(), 0);
      assert_eq!(f.counter2().count(), 0);
      assert_eq!(f.counter().time().in_microseconds(), 100);
      assert_eq!(f.counter2().time().in_microseconds(), 50);
      // 多次 Print 不重复计时(:343-347)。
      f.stats.print(&mut out).unwrap();
      assert_eq!(f.counter().count(), 0);
      assert_eq!(f.counter2().count(), 0);
      assert_eq!(f.counter().time().in_microseconds(), 100);
      assert_eq!(f.counter2().time().in_microseconds(), 50);

      f.sleep_us(50);
      f.stats.print(&mut out).unwrap();
      assert_eq!(f.counter().count(), 0);
      assert_eq!(f.counter2().count(), 0);
      assert_eq!(f.counter().time().in_microseconds(), 100);
      assert_eq!(f.counter2().time().in_microseconds(), 100);
      f.sleep_us(50);
    }
    f.sleep_us(50);
    assert_eq!(f.counter().count(), 0);
    assert_eq!(f.counter2().count(), 1);
    assert_eq!(f.counter().time().in_microseconds(), 100);
    assert_eq!(f.counter2().time().in_microseconds(), 150);
    f.sleep_us(50);
  }
  assert_eq!(f.counter().count(), 1);
  assert_eq!(f.counter2().count(), 1);
  assert_eq!(f.counter().time().in_microseconds(), 200);
  assert_eq!(f.counter2().time().in_microseconds(), 150);
  OK
}

// TEST_F(RuntimeCallStatsTest, NestedScopes)(:370-402)
#[test]
fn runtime_call_stats_test_nested_scopes() -> Void {
  let f = Fixture::new();
  {
    let _s1 = rcs_scope(&f.stats, f.counter_id());
    f.sleep_us(100);
    {
      let _s2 = rcs_scope(&f.stats, f.counter_id2());
      f.sleep_us(100);
      {
        let _s3 = rcs_scope(&f.stats, f.counter_id3());
        f.sleep_us(50);
      }
      f.sleep_us(50);
      {
        let _s3 = rcs_scope(&f.stats, f.counter_id3());
        f.sleep_us(50);
      }
      f.sleep_us(50);
    }
    f.sleep_us(100);
    {
      let _s2 = rcs_scope(&f.stats, f.counter_id2());
      f.sleep_us(100);
    }
    f.sleep_us(50);
  }
  assert_eq!(f.counter().count(), 1);
  assert_eq!(f.counter2().count(), 2);
  assert_eq!(f.counter3().count(), 2);
  assert_eq!(f.counter().time().in_microseconds(), 250);
  assert_eq!(f.counter2().time().in_microseconds(), 300);
  assert_eq!(f.counter3().time().in_microseconds(), 100);
  // now() 默认时间源在守卫 Drop 后恢复(对应 TearDownTestSuite)。
  let _ = now();
  info!("nested scopes ok");
  OK
}

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}
