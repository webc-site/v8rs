//! tests/memory_balancer_unittest.rs — v8-heap-core/src/memory_balancer.rs 等价覆盖 (W10HCS 整改)。
//!
//! 原版无 memory-balancer 专名 unittest (v8/test/unittests 全树 grep 零命中) → 等价覆盖口径
//! 同 HC-23R/HC-2b 申报: 对已填两件纯函数体 (cc:74-84 / cc:63-72) 与 EWMA 核
//! (h:38-46) 逐断言注释原版 file:line 算例。心跳面 (cc:86-100) / 限额面 (cc:17-25)
//! 系 todo! 推迟体不入本文件。

use std::ptr::null_mut;

use aok::{OK, Void};
use log::info;
use v8_base_platform::{TimeDelta, TimeTicks};
use v8_heap_core::{
  memory_allocator::Heap,
  memory_balancer::{MemoryBalancer, SmoothedBytesAndDuration},
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// f64 容差比较 (EWMA 双乘加为二进制不精确小数, 原版 C++ 同为 double, 取 1e-9 相对界)。
fn approx_eq(a: f64, b: f64) -> bool {
  (a - b).abs() <= 1e-9 * a.abs().max(b.abs()).max(1.0)
}

/// ctor cc:14-15 `MemoryBalancer(heap, startup_time) : heap_(heap), last_measured_at_(startup_time)`
/// + h:63/:74/:77/:78/:82/:84 缺省初值面。
#[test]
fn memory_balancer_ctor_defaults() -> Void {
  info!("> memory_balancer_ctor_defaults");
  let startup = TimeTicks::from_ms_ticks_for_testing(7);
  let mb = MemoryBalancer::new(null_mut::<Heap>(), startup);
  // cc:15 last_measured_at_(startup_time)。
  assert_eq!(
    mb.last_measured_at.to_internal_value(),
    startup.to_internal_value()
  );
  // h:63/:74/:82/:84 缺省 0/0/0/false; h:77/:78 optional 空。
  assert_eq!(mb.live_memory_after_gc, 0);
  assert_eq!(mb.embedder_allocation_limit, 0);
  assert_eq!(mb.last_measured_memory, 0);
  assert!(!mb.heartbeat_task_started);
  assert!(mb.major_allocation_rate.is_none());
  assert!(mb.major_gc_speed.is_none());
  OK
}

/// EWMA 核 h:38 构造 (bytes_ = static_cast<double>(bytes)) + h:46 rate() = bytes_/duration_。
#[test]
fn smoothed_bytes_and_duration_new_rate() -> Void {
  info!("> smoothed_bytes_and_duration_new_rate");
  // h:38: 100 字节 / 10 ms → rate 10 B/ms。
  let s = SmoothedBytesAndDuration::new(100, 10.0);
  assert!(approx_eq(s.rate(), 10.0));
  OK
}

/// EWMA 核 h:40-44 `Update`: x' = x*decay + new*(1-decay) 双量同率。
/// 算例: (100,10) --update(200,10,0.5)--> bytes 150 / duration 10 → rate 15。
#[test]
fn smoothed_bytes_and_duration_update_ewma() -> Void {
  info!("> smoothed_bytes_and_duration_update_ewma");
  let mut s = SmoothedBytesAndDuration::new(100, 10.0);
  // h:41-42 bytes_ = 100*0.5 + 200*0.5 = 150; h:43-44 duration_ = 10*0.5 + 10*0.5 = 10。
  s.update(200, 10.0, 0.5);
  assert!(approx_eq(s.rate(), 15.0));
  OK
}

/// cc:74-78 `UpdateAllocationRate` 首次支: `!major_allocation_rate_` → 直构
/// {bytes, InMillisecondsF()}。decay 常量面 h:53 = 0.95。
#[test]
fn update_allocation_rate_first_insert() -> Void {
  info!("> update_allocation_rate_first_insert");
  assert_eq!(MemoryBalancer::K_MAJOR_ALLOCATION_DECAY_RATE, 0.95);
  let mut mb = MemoryBalancer::new(null_mut::<Heap>(), TimeTicks::new());
  // cc:77-78: 1000 B / 2 ms → rate 500 B/ms。
  mb.update_allocation_rate(1000, TimeDelta::from_milliseconds_d(2.0));
  let rate = mb.major_allocation_rate.unwrap();
  assert!(approx_eq(rate.rate(), 500.0));
  OK
}

/// cc:80-82 `UpdateAllocationRate` EWMA 支 (kMajorAllocationDecayRate=0.95, h:53)。
/// 算例: (1000,2) --(2000,4)--> bytes 1050 / duration 2.1 → rate 500。
#[test]
fn update_allocation_rate_ewma() -> Void {
  info!("> update_allocation_rate_ewma");
  let mut mb = MemoryBalancer::new(null_mut::<Heap>(), TimeTicks::new());
  mb.update_allocation_rate(1000, TimeDelta::from_milliseconds_d(2.0));
  // h:41-42: 1000*0.95 + 2000*0.05 = 1050; h:43-44: 2*0.95 + 4*0.05 = 2.1。
  mb.update_allocation_rate(2000, TimeDelta::from_milliseconds_d(4.0));
  let rate = mb.major_allocation_rate.unwrap();
  assert!(approx_eq(rate.rate(), 1050.0 / 2.1));
  // 差分锚: rate 值恰 500 (1050/2.1)。
  assert!(approx_eq(rate.rate(), 500.0));
  // cc:82 消费位不影响 gc_speed (h:77/:78 双 optional 独立)。
  assert!(mb.major_gc_speed.is_none());
  OK
}

/// cc:63-67 `UpdateGCSpeed` 首次支: 直构 {bytes, InMillisecondsF()}。h:54 decay=0.5。
#[test]
fn update_gc_speed_first_insert() -> Void {
  info!("> update_gc_speed_first_insert");
  assert_eq!(MemoryBalancer::K_MAJOR_GC_DECAY_RATE, 0.5);
  let mut mb = MemoryBalancer::new(null_mut::<Heap>(), TimeTicks::new());
  // cc:66-67: 800 B / 16 ms → rate 50 B/ms。
  mb.update_gc_speed(800, TimeDelta::from_milliseconds_d(16.0));
  let speed = mb.major_gc_speed.unwrap();
  assert!(approx_eq(speed.rate(), 50.0));
  // 双 optional 独立对偶面 (h:77/:78)。
  assert!(mb.major_allocation_rate.is_none());
  OK
}

/// cc:69-70 `UpdateGCSpeed` EWMA 支 (kMajorGCDecayRate=0.5, h:54)。
/// 算例: (800,16) --(1600,16)--> bytes 1200 / duration 16 → rate 75。
#[test]
fn update_gc_speed_ewma() -> Void {
  info!("> update_gc_speed_ewma");
  let mut mb = MemoryBalancer::new(null_mut::<Heap>(), TimeTicks::new());
  mb.update_gc_speed(800, TimeDelta::from_milliseconds_d(16.0));
  // h:41-42: 800*0.5 + 1600*0.5 = 1200; h:43-44: 16*0.5 + 16*0.5 = 16。
  mb.update_gc_speed(1600, TimeDelta::from_milliseconds_d(16.0));
  let speed = mb.major_gc_speed.unwrap();
  assert!(approx_eq(speed.rate(), 75.0));
  OK
}

/// TimeDelta→ms 转换锚 (cc:78/:81/:67/:69 InMillisecondsF 消费位, time.rs:175)。
/// from_milliseconds_d(2.5) 回读 2.5 精确 (整数微秒域)。
#[test]
fn update_rates_use_milliseconds_f() -> Void {
  info!("> update_rates_use_milliseconds_f");
  let mut mb = MemoryBalancer::new(null_mut::<Heap>(), TimeTicks::new());
  mb.update_allocation_rate(125, TimeDelta::from_milliseconds_d(2.5));
  // 125 B / 2.5 ms = 50 B/ms (原版 InMillisecondsF 双精度域)。
  assert!(approx_eq(mb.major_allocation_rate.unwrap().rate(), 50.0));
  OK
}
