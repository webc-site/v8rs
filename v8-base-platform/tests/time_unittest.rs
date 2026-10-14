//! time-unittest.cc 1:1 迁移。规约见 tests/SPEC.md。
use core::hint::spin_loop;
use std::ops::Sub;

use aok::{OK, Void};
use log::info;
use v8_base_platform::{ElapsedTimer, OS, ThreadTicks, Time, TimeConstants, TimeDelta, TimeTicks};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// EXPECT_DOUBLE_EQ 等价: gtest AlmostEquals = 4 ULP 容差。
fn double_eq(a: f64, b: f64) -> bool {
  a == b || (a - b).abs() <= 4.0 * f64::EPSILON * a.abs().max(b.abs())
}

// TEST(TimeDelta, ZeroMinMax) cc:27-42
#[test]
fn time_delta_zero_min_max() -> Void {
  info!("> time_delta_zero_min_max");
  let k_zero = TimeDelta::new(); // cc:28 constexpr TimeDelta kZero
  assert!(k_zero.is_zero()); // cc:29 static_assert
  let k_max = TimeDelta::max(); // cc:31
  assert!(k_max.is_max()); // cc:32
  assert_eq!(k_max, TimeDelta::max()); // cc:33
  assert!(k_max > TimeDelta::from_days(100 * 365)); // cc:34 EXPECT_GT
  assert!(k_max > k_zero); // cc:35
  let k_min = TimeDelta::min(); // cc:37
  assert!(k_min.is_min()); // cc:38
  assert_eq!(k_min, TimeDelta::min()); // cc:39
  assert!(k_min < TimeDelta::from_days(-100 * 365)); // cc:40 EXPECT_LT
  assert!(k_min < k_zero); // cc:41
  OK
}

// TEST(TimeDelta, MaxConversions) cc:44-54
#[test]
fn time_delta_max_conversions() -> Void {
  let k_max = TimeDelta::max();
  assert_eq!(k_max.in_days(), i32::MAX); // cc:47
  assert_eq!(k_max.in_hours(), i32::MAX); // cc:48
  assert_eq!(k_max.in_minutes(), i32::MAX); // cc:49
  assert!(k_max.in_seconds_f().is_infinite() && k_max.in_seconds_f().is_sign_positive()); // cc:50 == +inf
  assert_eq!(k_max.in_seconds(), i64::MAX); // cc:51
  assert!(k_max.in_milliseconds_f().is_infinite() && k_max.in_milliseconds_f().is_sign_positive()); // cc:52 == +inf
  assert_eq!(k_max.in_milliseconds(), i64::MAX); // cc:53
  assert_eq!(k_max.in_milliseconds_rounded_up(), i64::MAX); // cc:54
  // cc:57-92 原版已注释的 overflow TODO 不迁移
  OK
}

// TEST(TimeDelta, NumericOperators) cc:95-124
// Rust 侧 Mul/Div 仅有 i64（原版 i32/i64/字面量三组收敛为两组, 语义等价）。
#[test]
fn time_delta_numeric_operators() -> Void {
  const I: i64 = 2; // 原版 constexpr int i = 2 与 int64_t i64 = 2 在 Rust 同为 i64
  assert_eq!(
    TimeDelta::from_milliseconds(1000) * I,
    TimeDelta::from_milliseconds(2000)
  ); // cc:97-98
  assert_eq!(
    TimeDelta::from_milliseconds(1000) / I,
    TimeDelta::from_milliseconds(500)
  ); // cc:99-100
  assert_eq!(
    TimeDelta::from_milliseconds(1000) * 2,
    TimeDelta::from_milliseconds(2000)
  ); // cc:107-108 (i64 组)
  assert_eq!(
    TimeDelta::from_milliseconds(1000) / 2,
    TimeDelta::from_milliseconds(500)
  ); // cc:109-110
  assert_eq!(
    TimeDelta::from_milliseconds(2000),
    TimeDelta::from_milliseconds(1000) * 2
  ); // cc:116-117 字面量组
  assert_eq!(
    TimeDelta::from_milliseconds(500),
    TimeDelta::from_milliseconds(1000) / 2
  ); // cc:118-119
  OK
}

// 原版 NumericOperators 的 *=, /= 半边 cc:101-104,111-114,121-124（6 条断言）。
// i / i64 / 字面量三组在 Rust 侧同为 i64（登记偏差见 SPEC.md）。
#[test]
fn time_delta_numeric_operators_compound() -> Void {
  const I: i64 = 2; // constexpr int i = 2 / int64_t i64 = 2
  let mut d = TimeDelta::from_milliseconds(1000);
  d *= I;
  assert_eq!(TimeDelta::from_milliseconds(2000), d); // cc:101-102
  let mut d = TimeDelta::from_milliseconds(1000);
  d /= I;
  assert_eq!(TimeDelta::from_milliseconds(500), d); // cc:103-104
  let mut d = TimeDelta::from_milliseconds(1000);
  d *= I;
  assert_eq!(TimeDelta::from_milliseconds(2000), d); // cc:111-112 (int64_t 组)
  let mut d = TimeDelta::from_milliseconds(1000);
  d /= I;
  assert_eq!(TimeDelta::from_milliseconds(500), d); // cc:113-114
  let mut d = TimeDelta::from_milliseconds(1000);
  d *= 2;
  assert_eq!(TimeDelta::from_milliseconds(2000), d); // cc:121-122 (字面量组)
  let mut d = TimeDelta::from_milliseconds(1000);
  d /= 2;
  assert_eq!(TimeDelta::from_milliseconds(500), d); // cc:123-124
  OK
}

// TEST(TimeDelta, DISABLED_Overflows) cc:127-182
// [SKIP] 原版 DISABLED_ 前缀, gtest 默认不运行; 理由登记 SPEC.md。

// TEST(TimeDelta, FromAndIn) cc:184-200
#[test]
fn time_delta_from_and_in() -> Void {
  assert_eq!(TimeDelta::from_days(2), TimeDelta::from_hours(48)); // cc:185
  assert_eq!(TimeDelta::from_hours(3), TimeDelta::from_minutes(180)); // cc:186
  assert_eq!(TimeDelta::from_minutes(2), TimeDelta::from_seconds(120)); // cc:187
  assert_eq!(
    TimeDelta::from_seconds(2),
    TimeDelta::from_milliseconds(2000)
  ); // cc:188
  assert_eq!(
    TimeDelta::from_milliseconds(2),
    TimeDelta::from_microseconds(2000)
  ); // cc:189
  assert_eq!(13, TimeDelta::from_days(13).in_days()); // cc:190
  assert_eq!(13, TimeDelta::from_hours(13).in_hours()); // cc:191
  assert_eq!(13, TimeDelta::from_minutes(13).in_minutes()); // cc:192
  assert_eq!(13, TimeDelta::from_seconds(13).in_seconds()); // cc:193
  assert!(
    double_eq(13.0, TimeDelta::from_seconds(13).in_seconds_f()),
    "cc:194 EXPECT_DOUBLE_EQ"
  ); // cc:194
  assert_eq!(13, TimeDelta::from_milliseconds(13).in_milliseconds()); // cc:195-196
  assert!(
    double_eq(13.0, TimeDelta::from_milliseconds(13).in_milliseconds_f()),
    "cc:197"
  ); // cc:197
  assert_eq!(13, TimeDelta::from_microseconds(13).in_microseconds()); // cc:198-199
  OK
}

// TEST(TimeDelta, MachTimespec) cc:202-211 (V8_OS_DARWIN 生效)
// 原版: null / 42ms / 42d 三条 FromMachTimespec(ToMachTimespec()) 往返相等。
#[cfg(target_os = "macos")]
#[test]
fn time_delta_mach_timespec() -> Void {
  let null = TimeDelta::new(); // cc:204 TimeDelta null = TimeDelta()
  assert_eq!(null, TimeDelta::from_mach_timespec(null.to_mach_timespec())); // cc:205
  let delta1 = TimeDelta::from_milliseconds(42); // cc:206
  assert_eq!(
    delta1,
    TimeDelta::from_mach_timespec(delta1.to_mach_timespec())
  ); // cc:207
  let delta2 = TimeDelta::from_days(42); // cc:208
  assert_eq!(
    delta2,
    TimeDelta::from_mach_timespec(delta2.to_mach_timespec())
  ); // cc:209
  OK
}

// TEST(Time, Max) cc:213-219
#[test]
fn time_max() -> Void {
  let max = Time::max();
  assert!(max.is_max()); // cc:215
  assert_eq!(max, Time::max()); // cc:216
  assert!(max > Time::now()); // cc:217
  assert!(max > Time::new()); // cc:218
  OK
}

// TEST(Time, MaxConversions) cc:221-253（OS_POSIX 分支生效, OS_WIN 分支跳过）
#[test]
fn time_max_conversions() -> Void {
  let t = Time::max();
  assert_eq!(i64::MAX, t.to_internal_value()); // cc:223
  // cc:225-229 原版注释掉的 FromJsTime(inf) TODO 不迁移
  // cc:231-241 POSIX timeval 块
  let tval_sec = i64::MAX; // std::numeric_limits<time_t>::max()
  let tval_usec = TimeConstants::MICROSECONDS_PER_SECOND - 1; // cc:234
  let t = Time::from_timeval(tval_sec, tval_usec);
  assert!(t.is_max()); // cc:236
  let (sec, usec) = t.to_timeval(); // cc:237
  assert_eq!(i64::MAX, sec); // cc:238
  assert_eq!(TimeConstants::MICROSECONDS_PER_SECOND - 1, usec); // cc:239
  OK
}

// TEST(Time, JsTime) cc:255-258
#[test]
fn time_js_time() -> Void {
  let t = Time::from_js_time(700000.3); // cc:256
  assert!(
    double_eq(700000.3, t.to_js_time()),
    "cc:257 EXPECT_DOUBLE_EQ(700000.3, ToJsTime())"
  );
  OK
}

// TEST(Time, Timespec) cc:262-275 (V8_OS_POSIX 生效)
#[test]
fn time_timespec() -> Void {
  let null = Time::new();
  assert!(null.is_null()); // cc:264
  let (s, n) = null.to_timespec();
  assert_eq!(null, Time::from_timespec(s, n)); // cc:265
  let now = Time::now(); // cc:266
  let (s, n) = now.to_timespec();
  assert_eq!(now, Time::from_timespec(s, n)); // cc:267
  let now_sys = Time::now_from_system_time(); // cc:268
  let (s, n) = now_sys.to_timespec();
  assert_eq!(now_sys, Time::from_timespec(s, n)); // cc:269
  let unix_epoch = Time::unix_epoch(); // cc:270
  let (s, n) = unix_epoch.to_timespec();
  assert_eq!(unix_epoch, Time::from_timespec(s, n)); // cc:271
  let max = Time::max(); // cc:272
  assert!(max.is_max()); // cc:273
  let (s, n) = max.to_timespec();
  assert_eq!(max, Time::from_timespec(s, n)); // cc:274
  OK
}

// TEST(Time, Timeval) cc:278-291 (V8_OS_POSIX 生效)
#[test]
fn time_timeval() -> Void {
  let null = Time::new();
  assert!(null.is_null()); // cc:280
  let (s, u) = null.to_timeval();
  assert_eq!(null, Time::from_timeval(s, u)); // cc:281
  let now = Time::now(); // cc:282
  let (s, u) = now.to_timeval();
  assert_eq!(now, Time::from_timeval(s, u)); // cc:283
  let now_sys = Time::now_from_system_time(); // cc:284
  let (s, u) = now_sys.to_timeval();
  assert_eq!(now_sys, Time::from_timeval(s, u)); // cc:285
  let unix_epoch = Time::unix_epoch(); // cc:286
  let (s, u) = unix_epoch.to_timeval();
  assert_eq!(unix_epoch, Time::from_timeval(s, u)); // cc:287
  let max = Time::max(); // cc:288
  assert!(max.is_max()); // cc:289
  let (s, u) = max.to_timeval();
  assert_eq!(max, Time::from_timeval(s, u)); // cc:290
  OK
}

// TEST(Time, Filetime) cc:295-309 [SKIP] V8_OS_WIN 专属, 本机构建不编译。

// ResolutionTest<T> cc:315-340
fn resolution_test<T>(now: fn() -> T, target_granularity: TimeDelta) -> Void
where
  T: Copy + Ord + Sub<Output = TimeDelta>,
{
  const K_EXPIRATION_TIMEOUT: TimeDelta = TimeDelta::from_seconds(1); // cc:323
  let mut timer = ElapsedTimer::new();
  timer.start(); // cc:325
  let mut delta;
  loop {
    let start = now(); // cc:328
    let mut current;
    loop {
      // cc:330-336 自旋直到时钟变化
      current = now();
      delta = current - start;
      if current > start {
        break;
      }
    }
    assert_ne!(
      0,
      delta.in_microseconds(),
      "cc:337 EXPECT_NE(0, delta.InMicroseconds())"
    );
    // cc:338 while (delta > target && !timer.HasExpired(kExpirationTimeout))
    if !(delta > target_granularity && !timer.has_expired(K_EXPIRATION_TIMEOUT)) {
      break;
    }
  }
  assert!(
    delta <= target_granularity,
    "cc:339 EXPECT_LE(delta, target_granularity)（1s 预算内未观测到目标粒度）"
  );
  OK
}

// TEST(Time, NowResolution) cc:345-349
#[test]
fn time_now_resolution() -> Void {
  const K_TARGET_GRANULARITY: TimeDelta = TimeDelta::from_milliseconds(16); // cc:347
  resolution_test::<Time>(Time::now, K_TARGET_GRANULARITY)
}

// TEST(TimeTicks, NowResolution) cc:352-360
#[test]
fn time_ticks_now_resolution() -> Void {
  let k_target_granularity = if TimeTicks::is_high_resolution() {
    TimeDelta::from_microseconds(1) // cc:357
  } else {
    TimeDelta::from_milliseconds(16) // cc:358
  };
  resolution_test::<TimeTicks>(TimeTicks::now, k_target_granularity)
}

// TEST(TimeTicks, IsMonotonic) cc:362-372
#[test]
fn time_ticks_is_monotonic() -> Void {
  let mut previous_ticks = TimeTicks::new();
  let mut timer = ElapsedTimer::new();
  timer.start();
  while !timer.has_expired(TimeDelta::from_milliseconds(100)) {
    let ticks = TimeTicks::now();
    assert!(ticks >= previous_ticks); // cc:368 EXPECT_GE(ticks, previous)
    assert!(ticks.minus(previous_ticks).in_microseconds() >= 0); // cc:369 EXPECT_GE(..., 0)
    previous_ticks = ticks;
  }
  OK
}

// 原版匿名 Sleep cc:374-382
fn sleep_by(wait_time: TimeDelta) {
  let mut waiter = ElapsedTimer::new();
  waiter.start();
  while !waiter.has_expired(wait_time) {
    OS::sleep(TimeDelta::from_milliseconds(1));
  }
}

// TEST(ElapsedTimer, StartStop) cc:384-423
#[test]
fn elapsed_timer_start_stop() -> Void {
  const WAIT_TIME: TimeDelta = TimeDelta::from_milliseconds(100);
  const NOISE: TimeDelta = TimeDelta::from_milliseconds(200);
  let mut timer = ElapsedTimer::new();
  assert!(!timer.is_started()); // cc:388 DCHECK(!IsStarted)

  timer.start();
  assert!(timer.is_started()); // cc:391

  sleep_by(WAIT_TIME);
  let mut delta = timer.elapsed();
  assert!(timer.is_started()); // cc:395
  assert!(delta >= WAIT_TIME); // cc:396 EXPECT_GE
  assert!(delta < WAIT_TIME + NOISE); // cc:397 EXPECT_LT

  #[cfg(debug_assertions)]
  assert!(!timer.is_paused()); // cc:399 DCHECK(!IsPaused)
  timer.pause();
  #[cfg(debug_assertions)]
  assert!(timer.is_paused()); // cc:401
  sleep_by(WAIT_TIME);

  timer.resume();
  assert!(timer.is_started()); // cc:405
  delta = timer.elapsed(); // cc:406
  #[cfg(debug_assertions)]
  assert!(!timer.is_paused()); // cc:407
  timer.pause(); // cc:408
  #[cfg(debug_assertions)]
  assert!(timer.is_paused()); // cc:409
  assert!(delta >= WAIT_TIME); // cc:410
  assert!(delta < WAIT_TIME + NOISE); // cc:411

  sleep_by(WAIT_TIME);
  timer.resume(); // cc:414
  #[cfg(debug_assertions)]
  {
    assert!(!timer.is_paused()); // cc:415
    assert!(timer.is_started()); // cc:416
  }
  delta = timer.elapsed(); // cc:417
  assert!(delta >= WAIT_TIME); // cc:418
  assert!(delta < WAIT_TIME + NOISE); // cc:419

  timer.stop(); // cc:421
  assert!(!timer.is_started()); // cc:422
  OK
}

// TEST(ElapsedTimer, StartStopArgs) cc:425-482
#[test]
fn elapsed_timer_start_stop_args() -> Void {
  const WAIT_TIME: TimeDelta = TimeDelta::from_milliseconds(100);
  let mut timer1 = ElapsedTimer::new();
  let mut timer2 = ElapsedTimer::new();
  assert!(!timer1.is_started()); // cc:429
  assert!(!timer2.is_started()); // cc:430

  let now = TimeTicks::now(); // cc:432
  timer1.start_at(now);
  timer2.start_at(now);
  assert!(timer1.is_started()); // cc:435
  assert!(timer2.is_started()); // cc:436

  sleep_by(WAIT_TIME);
  let now = TimeTicks::now(); // cc:439 (复用 now 变量名, 对应原版赋值)
  let delta1 = timer1.elapsed_at(now); // cc:440
  sleep_by(WAIT_TIME);
  let delta2 = timer2.elapsed_at(now); // cc:442
  assert!(timer1.is_started()); // cc:443
  assert!(timer2.is_started()); // cc:444
  assert!(delta1 >= delta2); // cc:445 EXPECT_GE
  sleep_by(WAIT_TIME);
  assert_ne!(delta1, timer2.elapsed()); // cc:447 EXPECT_NE

  let now2 = TimeTicks::now(); // cc:449
  assert_ne!(timer1.elapsed_at(now), timer1.elapsed_at(now2)); // cc:450
  assert_ne!(delta1, timer1.elapsed_at(now2)); // cc:451
  assert_ne!(delta2, timer2.elapsed_at(now2)); // cc:452
  assert!(timer1.elapsed_at(now2) >= timer2.elapsed_at(now2)); // cc:453 EXPECT_GE

  let now = TimeTicks::now(); // cc:455
  timer1.pause_at(now);
  timer2.pause_at(now);
  #[cfg(debug_assertions)]
  {
    assert!(timer1.is_paused()); // cc:458
    assert!(timer2.is_paused()); // cc:459
  }
  sleep_by(WAIT_TIME);

  let now = TimeTicks::now(); // cc:462
  timer1.resume_at(now); // cc:463
  #[cfg(debug_assertions)]
  {
    assert!(!timer1.is_paused()); // cc:464
    assert!(timer2.is_paused()); // cc:465
  }
  sleep_by(WAIT_TIME);
  timer2.resume_at(now); // cc:467
  #[cfg(debug_assertions)]
  {
    assert!(!timer1.is_paused()); // cc:468
    assert!(!timer2.is_paused()); // cc:469
    assert!(timer1.is_started()); // cc:470
    assert!(timer2.is_started()); // cc:471
  }

  let delta1 = timer1.elapsed_at(now); // cc:473
  sleep_by(WAIT_TIME);
  let delta2 = timer2.elapsed_at(now); // cc:475
  assert!(delta1 >= delta2); // cc:476 EXPECT_GE

  timer1.stop();
  timer2.stop(); // cc:478-479
  assert!(!timer1.is_started()); // cc:480
  assert!(!timer2.is_started()); // cc:481
  OK
}

// TEST(ThreadTicks, MAYBE_ThreadNow) cc:485-533 (MAYBE_ThreadNow=ThreadNow 非 android)
// 原版 darwin 上 ThreadTicks::IsSupported()==true 并 WaitUntilInitialized() 后执行主体。
#[test]
fn thread_ticks_maybe_thread_now() -> Void {
  // 原版 cc:487 `if (IsSupported())`：平台不支持时跳过测试主体。
  if !ThreadTicks::is_supported() {
    return OK;
  }
  // 以下对应原版主体 (cc:491-532)。
  ThreadTicks::wait_until_initialized(); // cc:489 WaitUntilInitialized()
  let begin = TimeTicks::now();
  let begin_thread = ThreadTicks::now();
  assert!(
    begin_thread > ThreadTicks::new(),
    "cc:496 EXPECT_GT(begin_thread, ThreadTicks())"
  );
  #[cfg(all(target_os = "windows", target_arch = "aarch64"))]
  const LIMIT_MS: i64 = 100; // cc:501 (win-arm64 分支)
  #[cfg(not(all(target_os = "windows", target_arch = "aarch64")))]
  const LIMIT_MS: i64 = 10; // cc:505
  let limit_us: i64 = LIMIT_MS * 1000;
  while ThreadTicks::now() == begin_thread
    && (TimeTicks::now() - begin).in_microseconds() < limit_us
  {
    spin_loop();
  }
  assert!(ThreadTicks::now() > begin_thread, "cc:515 EXPECT_GT");
  let mut iterations_count = 0;
  let end;
  let end_thread;
  loop {
    OS::sleep(TimeDelta::from_milliseconds(LIMIT_MS)); // cc:519
    let end_now_thread = ThreadTicks::now(); // cc:520（原版每轮重赋值, 仅末次值会被读取）
    let end_now = TimeTicks::now(); // cc:521
    let delta = end_now - begin;
    iterations_count += 1;
    assert!(iterations_count <= 2, "cc:523 EXPECT_LE(iterations, 2)");
    if delta.in_microseconds() >= limit_us {
      end_thread = end_now_thread;
      end = end_now;
      break;
    }
  }
  let delta = end - begin;
  let delta_thread = end_thread - begin_thread;
  assert!(
    delta_thread.in_microseconds() > 0,
    "cc:528 EXPECT_GT(delta_thread, 0)"
  );
  let difference = delta - delta_thread;
  assert!(
    difference.in_microseconds() >= limit_us * 9 / 10,
    "cc:531 EXPECT_GE(difference, 9ms)"
  );
  OK
}

// TEST(TimeTicks, TimerPerformance) cc:536-578
// [SKIP] V8_OS_WIN 专属, 本机构建不编译; 且其唯一断言已被原版注释掉(仅剩 printf)。
