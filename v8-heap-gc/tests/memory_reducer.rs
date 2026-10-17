// 1:1 转写 v8/test/unittests/heap/memory-reducer-unittest.cc (8 组 MemoryReducer 用例)
// flag 依赖以默认值参数化: long = memory_reducer_delay_ms(8000) / start = gc_memory_reducer_start_delay_ms(30000)
// / max_gcs = memory_reducer_gc_count(2); 用例内增量事件 (if !incremental_marking) 默认开启, 全跑。
use aok::{OK, Void};
use v8_heap_gc::{
  K_COMMITTED_MEMORY_DELTA, K_COMMITTED_MEMORY_FACTOR, K_SHORT_DELAY_MS, K_WATCHDOG_DELAY_MS,
  MemoryReducerEvent as Event, MemoryReducerEventType as EventType, MemoryReducerId as Id,
  MemoryReducerState as State, memory_reducer_step as step,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

const MB: usize = 1024 * 1024;
// 默认 flag 值
const LONG: f64 = 8000.0;
const START: f64 = 30000.0;
const MAX_GCS: i32 = 2;

fn mark_compact_event(time_ms: f64, more: bool, committed: usize) -> Event {
  Event {
    event_type: EventType::MarkCompact,
    time_ms,
    committed_memory: committed,
    next_gc_likely_to_collect_more: more,
    should_start_incremental_gc: false,
    can_start_incremental_gc: false,
  }
}
fn mc_garbage_left(time_ms: f64, committed: usize) -> Event {
  mark_compact_event(time_ms, true, committed)
}
fn mc_no_garbage_left(time_ms: f64, committed: usize) -> Event {
  mark_compact_event(time_ms, false, committed)
}
fn timer_event(time_ms: f64, should: bool, can: bool) -> Event {
  Event {
    event_type: EventType::Timer,
    time_ms,
    committed_memory: 0,
    next_gc_likely_to_collect_more: false,
    should_start_incremental_gc: should,
    can_start_incremental_gc: can,
  }
}
fn timer_low_alloc(time_ms: f64) -> Event {
  timer_event(time_ms, true, true)
}
fn timer_high_alloc(time_ms: f64) -> Event {
  timer_event(time_ms, false, true)
}
fn timer_pending(time_ms: f64) -> Event {
  timer_event(time_ms, true, false)
}
fn possible_garbage(time_ms: f64) -> Event {
  Event {
    event_type: EventType::PossibleGarbage,
    time_ms,
    committed_memory: 0,
    next_gc_likely_to_collect_more: false,
    should_start_incremental_gc: false,
    can_start_incremental_gc: false,
  }
}

// TEST(MemoryReducer, FromDoneToDone)
#[test]
fn from_done_to_done() -> Void {
  let state0 = State::create_done(1.0, 0);
  let run = |e: Event| step(&state0, &e, LONG, START, MAX_GCS);
  assert_eq!(run(timer_low_alloc(0.0)).id, Id::Done);
  assert_eq!(run(timer_high_alloc(0.0)).id, Id::Done);
  assert_eq!(run(timer_pending(0.0)).id, Id::Done);
  assert_eq!(
    run(mc_garbage_left(0.0, K_COMMITTED_MEMORY_DELTA - 1)).id,
    Id::Done
  );

  let state0 = State::create_done(1.0, 1000 * MB);
  let below = ((K_COMMITTED_MEMORY_FACTOR * (1000 * MB) as f64) as usize) - 1;
  assert_eq!(
    step(&state0, &mc_garbage_left(0.0, below), LONG, START, MAX_GCS).id,
    Id::Done
  );
  OK
}

// TEST(MemoryReducer, FromDoneToWait)
#[test]
fn from_done_to_wait() -> Void {
  let state0 = State::create_done(1.0, 0);

  let s = step(
    &state0,
    &mc_garbage_left(2.0, K_COMMITTED_MEMORY_DELTA),
    LONG,
    START,
    MAX_GCS,
  );
  assert_eq!(s.id, Id::Wait);
  assert_eq!(s.next_gc_start_ms, LONG + 2.0);
  assert_eq!(s.started_gcs, 0);
  assert_eq!(s.last_gc_time_ms, 2.0);

  let s = step(
    &state0,
    &mc_no_garbage_left(2.0, K_COMMITTED_MEMORY_DELTA),
    LONG,
    START,
    MAX_GCS,
  );
  assert_eq!(s.id, Id::Wait);
  assert_eq!(s.next_gc_start_ms, LONG + 2.0);
  assert_eq!(s.last_gc_time_ms, 2.0);

  let s = step(&state0, &possible_garbage(0.0), LONG, START, MAX_GCS);
  assert_eq!(s.id, Id::Wait);
  assert_eq!(s.next_gc_start_ms, START);
  assert_eq!(s.started_gcs, 0);
  assert_eq!(s.last_gc_time_ms, state0.last_gc_time_ms);

  let state0 = State::create_done(1.0, 1000 * MB);
  let at_factor = (K_COMMITTED_MEMORY_FACTOR * (1000 * MB) as f64) as usize;
  let s = step(
    &state0,
    &mc_garbage_left(2.0, at_factor),
    LONG,
    START,
    MAX_GCS,
  );
  assert_eq!(s.id, Id::Wait);
  assert_eq!(s.next_gc_start_ms, LONG + 2.0);
  assert_eq!(s.last_gc_time_ms, 2.0);
  OK
}

// TEST(MemoryReducer, FromWaitToWait)
#[test]
fn from_wait_to_wait() -> Void {
  let state0 = State::create_wait(MAX_GCS - 1, 1000.0, 1.0);

  let s = step(&state0, &possible_garbage(2000.0), LONG, START, MAX_GCS);
  assert_eq!(s.id, Id::Wait);
  assert_eq!(s.next_gc_start_ms, state0.next_gc_start_ms);
  assert_eq!(s.started_gcs, state0.started_gcs);

  let s = step(
    &state0,
    &timer_low_alloc(state0.next_gc_start_ms - 1.0),
    LONG,
    START,
    MAX_GCS,
  );
  assert_eq!(s.id, Id::Wait);
  assert_eq!(s.next_gc_start_ms, state0.next_gc_start_ms);
  assert_eq!(s.started_gcs, state0.started_gcs);

  let s = step(&state0, &timer_high_alloc(2000.0), LONG, START, MAX_GCS);
  assert_eq!(s.id, Id::Wait);
  assert_eq!(s.next_gc_start_ms, 2000.0 + LONG);
  assert_eq!(s.started_gcs, state0.started_gcs);

  let s = step(&state0, &timer_pending(2000.0), LONG, START, MAX_GCS);
  assert_eq!(s.id, Id::Wait);
  assert_eq!(s.next_gc_start_ms, 2000.0 + LONG);
  assert_eq!(s.started_gcs, state0.started_gcs);

  let s = step(&state0, &mc_garbage_left(2000.0, 0), LONG, START, MAX_GCS);
  assert_eq!(s.id, Id::Wait);
  assert_eq!(s.next_gc_start_ms, 2000.0 + LONG);
  assert_eq!(s.started_gcs, state0.started_gcs);
  assert_eq!(s.last_gc_time_ms, 2000.0);

  // W11HGR2 补迁: 原版 FromWaitToWait 第 6 步 MarkCompactEventNoGarbageLeft(2000, 0)
  let s = step(
    &state0,
    &mc_no_garbage_left(2000.0, 0),
    LONG,
    START,
    MAX_GCS,
  );
  assert_eq!(s.id, Id::Wait);
  assert_eq!(s.next_gc_start_ms, 2000.0 + LONG);
  assert_eq!(s.started_gcs, state0.started_gcs);
  assert_eq!(s.last_gc_time_ms, 2000.0);

  let state0 = State::create_wait(MAX_GCS - 1, 1000.0, 0.0);
  let s = step(
    &state0,
    &timer_high_alloc(K_WATCHDOG_DELAY_MS as f64 + 1.0),
    LONG,
    START,
    MAX_GCS,
  );
  assert_eq!(s.id, Id::Wait);
  assert_eq!(s.next_gc_start_ms, K_WATCHDOG_DELAY_MS as f64 + 1.0 + LONG);
  assert_eq!(s.started_gcs, state0.started_gcs);
  assert_eq!(s.last_gc_time_ms, state0.last_gc_time_ms);

  let state0 = State::create_wait(MAX_GCS - 1, 1000.0, 1.0);
  let s = step(&state0, &timer_high_alloc(2000.0), LONG, START, MAX_GCS);
  assert_eq!(s.id, Id::Wait);
  // W11HGR2 补全: 原版该步同断 next_gc_start_ms 与 started_gcs
  assert_eq!(s.next_gc_start_ms, 2000.0 + LONG);
  assert_eq!(s.started_gcs, state0.started_gcs);
  assert_eq!(s.last_gc_time_ms, state0.last_gc_time_ms);
  OK
}

// TEST(MemoryReducer, FromWaitToRun)
#[test]
fn from_wait_to_run() -> Void {
  let state0 = State::create_wait(0, 1000.0, 1.0);

  let s = step(
    &state0,
    &timer_low_alloc(state0.next_gc_start_ms + 1.0),
    LONG,
    START,
    MAX_GCS,
  );
  assert_eq!(s.id, Id::Run);
  assert_eq!(s.started_gcs, state0.started_gcs + 1);

  let s = step(
    &state0,
    &timer_high_alloc(K_WATCHDOG_DELAY_MS as f64 + 2.0),
    LONG,
    START,
    MAX_GCS,
  );
  assert_eq!(s.id, Id::Run);
  assert_eq!(s.started_gcs, state0.started_gcs + 1);
  OK
}

// TEST(MemoryReducer, FromWaitToDone)
#[test]
fn from_wait_to_done() -> Void {
  let state0 = State::create_wait(MAX_GCS, 0.0, 1.0);
  for event in [
    timer_low_alloc(2000.0),
    timer_high_alloc(2000.0),
    timer_pending(2000.0),
  ] {
    let s = step(&state0, &event, LONG, START, MAX_GCS);
    assert_eq!(s.id, Id::Done);
    assert_eq!(s.last_gc_time_ms, state0.last_gc_time_ms);
  }
  OK
}

// TEST(MemoryReducer, FromRunToRun)
#[test]
fn from_run_to_run() -> Void {
  let state0 = State::create_run(1);
  for event in [
    timer_low_alloc(2000.0),
    timer_high_alloc(2000.0),
    timer_pending(2000.0),
    possible_garbage(2000.0),
  ] {
    let s = step(&state0, &event, LONG, START, MAX_GCS);
    assert_eq!(s.id, Id::Run);
    assert_eq!(s.started_gcs, state0.started_gcs);
  }
  OK
}

// TEST(MemoryReducer, FromRunToDone)
#[test]
fn from_run_to_done() -> Void {
  let started = if MAX_GCS > 1 { 2 } else { 1 };
  let state0 = State::create_run(started);
  let s = step(
    &state0,
    &mc_no_garbage_left(2000.0, 0),
    LONG,
    START,
    MAX_GCS,
  );
  assert_eq!(s.id, Id::Done);
  assert_eq!(s.last_gc_time_ms, 2000.0);

  let state0 = State::create_run(MAX_GCS);
  let s = step(&state0, &mc_garbage_left(2000.0, 0), LONG, START, MAX_GCS);
  assert_eq!(s.id, Id::Done);
  OK
}

// TEST(MemoryReducer, FromRunToWait)
#[test]
fn from_run_to_wait() -> Void {
  if MAX_GCS > 2 {
    let state0 = State::create_run(2);
    let s = step(&state0, &mc_garbage_left(2000.0, 0), LONG, START, MAX_GCS);
    assert_eq!(s.id, Id::Wait);
    assert_eq!(s.next_gc_start_ms, 2000.0 + K_SHORT_DELAY_MS as f64);
    assert_eq!(s.started_gcs, state0.started_gcs);
    assert_eq!(s.last_gc_time_ms, 2000.0);
  }

  let state0 = State::create_run(1);
  let s = step(
    &state0,
    &mc_no_garbage_left(2000.0, 0),
    LONG,
    START,
    MAX_GCS,
  );
  assert_eq!(s.id, Id::Wait);
  assert_eq!(s.next_gc_start_ms, 2000.0 + K_SHORT_DELAY_MS as f64);
  assert_eq!(s.started_gcs, state0.started_gcs);
  assert_eq!(s.last_gc_time_ms, 2000.0);
  OK
}
