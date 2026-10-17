// 由 tool/src/heap/memory-reducer.cc.js 从 v8/src/heap/memory-reducer.{h,cc} 收编; 修订就地改, 复跑覆盖
// 窗口常量 + Step 状态机纯函数 1:1; 枚举判别值面在 memory_reducer_enums.rs (此处 re-export 合并)

//! MemoryReducer 状态机规则面 — Step 纯函数 1:1 转写。
//!
//! - 状态: UNINIT/DONE/WAIT/RUN; WAIT(n, x, t) = 已发起 n 次 GC, 下次最早 x, 上次 GC t
//! - 事件: TIMER / MarkCompact / PossibleGarbage
//! - flag 依赖参数化: long_delay_ms (memory_reducer_delay_ms) /
//!   start_delay_ms (gc_memory_reducer_start_delay_ms) / max_gcs (memory_reducer_gc_count)
//! - Heap/定时器耦合方法 (Notify*/TimerTask::RunInternal) 属收集器大盘, 见 readme 推迟台账

use crate::memory_reducer_enums::{EventType, Id};

/// 短延迟 ms (kShortDelayMs, cc:41) — kRun→kWait 回退窗口。
/// v8: src/heap/memory-reducer.cc:41 kShortDelayMs
pub const K_SHORT_DELAY_MS: i32 = 500;
/// 看门狗延迟 ms (kWatchdogDelayMs, cc:42)。
/// v8: src/heap/memory-reducer.cc:42 kWatchdogDelayMs
pub const K_WATCHDOG_DELAY_MS: i64 = 100000;
/// 触发增长因子 (kCommittedMemoryFactor, cc:43)。
/// v8: src/heap/memory-reducer.cc:43 kCommittedMemoryFactor
pub const K_COMMITTED_MEMORY_FACTOR: f64 = 1.1;
/// 触发增长增量 (kCommittedMemoryDelta = 10 * MB, cc:44)。
/// v8: src/heap/memory-reducer.cc:44 kCommittedMemoryDelta
pub const K_COMMITTED_MEMORY_DELTA: usize = 10 * 1024 * 1024;
/// 定时器调度余量 ms (kSlackMs, cc:247 ScheduleTimer 局部)。
/// v8: src/heap/memory-reducer.cc:247 kSlackMs
pub const K_SLACK_MS: f64 = 100.0;

/// 状态 (原版 class State 五字段数据面; 字段序 = 原版声明序)。
/// v8: src/heap/memory-reducer.h MemoryReducer::State
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct State {
  /// id_
  pub id: Id,
  /// started_gcs_ — 已发起 GC 数 (WAIT/RUN 有效)
  pub started_gcs: i32,
  /// next_gc_start_ms_ — 下次 GC 最早时刻 (WAIT 有效)
  pub next_gc_start_ms: f64,
  /// last_gc_time_ms_ — 上次全量 GC 时刻 (WAIT/DONE/UNINIT 有效)
  pub last_gc_time_ms: f64,
  /// committed_memory_at_last_run_ — 上轮结束时已提交内存 (UNINIT/DONE 有效)
  pub committed_memory_at_last_run: usize,
}

impl State {
  /// State::CreateUninitialized — {kUninit, 0, 0, 0, 0}。
  /// v8: src/heap/memory-reducer.h:109
  #[must_use]
  pub const fn create_uninitialized() -> Self {
    Self {
      id: Id::Uninit,
      started_gcs: 0,
      next_gc_start_ms: 0.0,
      last_gc_time_ms: 0.0,
      committed_memory_at_last_run: 0,
    }
  }

  /// State::CreateDone — {kDone, 0, 0, last_gc_time_ms, committed_memory}。
  /// v8: src/heap/memory-reducer.h:111
  #[must_use]
  pub const fn create_done(last_gc_time_ms: f64, committed_memory: usize) -> Self {
    Self {
      id: Id::Done,
      started_gcs: 0,
      next_gc_start_ms: 0.0,
      last_gc_time_ms,
      committed_memory_at_last_run: committed_memory,
    }
  }

  /// State::CreateWait — {kWait, started_gcs, next_gc_time_ms, last_gc_time_ms, 0}。
  /// v8: src/heap/memory-reducer.h:115
  #[must_use]
  pub const fn create_wait(started_gcs: i32, next_gc_time_ms: f64, last_gc_time_ms: f64) -> Self {
    Self {
      id: Id::Wait,
      started_gcs,
      next_gc_start_ms: next_gc_time_ms,
      last_gc_time_ms,
      committed_memory_at_last_run: 0,
    }
  }

  /// State::CreateRun — {kRun, started_gcs, 0, 0, 0}。
  /// v8: src/heap/memory-reducer.h:120
  #[must_use]
  pub const fn create_run(started_gcs: i32) -> Self {
    Self {
      id: Id::Run,
      started_gcs,
      next_gc_start_ms: 0.0,
      last_gc_time_ms: 0.0,
      committed_memory_at_last_run: 0,
    }
  }
}

/// 事件 (原版 struct Event 六字段, 字段序 = 原版声明序)。
/// v8: src/heap/memory-reducer.h MemoryReducer::Event
#[derive(Debug, Clone, Copy)]
pub struct Event {
  /// type
  pub event_type: EventType,
  /// time_ms
  pub time_ms: f64,
  /// committed_memory
  pub committed_memory: usize,
  /// next_gc_likely_to_collect_more
  pub next_gc_likely_to_collect_more: bool,
  /// should_start_incremental_gc
  pub should_start_incremental_gc: bool,
  /// can_start_incremental_gc
  pub can_start_incremental_gc: bool,
}

/// 看门狗: 距上次 GC 超过 kWatchdogDelayMs 则强制可跑。
/// v8: src/heap/memory-reducer.cc:161 MemoryReducer::WatchdogGC
#[must_use]
pub const fn watchdog_gc(state: &State, event: &Event) -> bool {
  state.last_gc_time_ms != 0.0 && event.time_ms > state.last_gc_time_ms + K_WATCHDOG_DELAY_MS as f64
}

/// 状态机步进 — MemoryReducer::Step 的 1:1 转写 (纯函数)。
/// flag 依赖参数化: long_delay_ms / start_delay_ms / max_gcs = MaxNumberOfGCs()。
/// v8: src/heap/memory-reducer.cc:168 MemoryReducer::Step
#[must_use]
pub const fn step(
  state: &State,
  event: &Event,
  long_delay_ms: f64,
  start_delay_ms: f64,
  max_gcs: i32,
) -> State {
  match state.id {
    Id::Uninit | Id::Done => match event.event_type {
      EventType::Timer => *state,
      EventType::MarkCompact => {
        // 提交内存增幅需超 factor 与 delta 的较大者才重新进入 WAIT。
        // 原版: std::max(static_cast<size_t>(last * kCommittedMemoryFactor),
        // last + kCommittedMemoryDelta), 乘法积经 static_cast<size_t> 向零截断,
        // 比较在整数域进行 (memory-reducer.cc:181-188); 此处 1:1 复现截断与整数比较。
        let last = state.committed_memory_at_last_run;
        let by_factor = ((last as f64) * K_COMMITTED_MEMORY_FACTOR) as usize;
        let by_delta = last.saturating_add(K_COMMITTED_MEMORY_DELTA);
        let threshold = if by_factor > by_delta {
          by_factor
        } else {
          by_delta
        };
        if event.committed_memory < threshold {
          *state
        } else {
          State::create_wait(0, event.time_ms + long_delay_ms, event.time_ms)
        }
      }
      EventType::PossibleGarbage => {
        State::create_wait(0, event.time_ms + start_delay_ms, state.last_gc_time_ms)
      }
    },
    Id::Wait => {
      // 原版 CHECK_LE (memory-reducer.cc:215), release 亦生效
      assert!(state.started_gcs <= max_gcs);
      match event.event_type {
        EventType::PossibleGarbage => *state,
        EventType::Timer => {
          if state.started_gcs >= max_gcs {
            State::create_done(state.last_gc_time_ms, event.committed_memory)
          } else if event.can_start_incremental_gc
            && (event.should_start_incremental_gc || watchdog_gc(state, event))
          {
            if state.next_gc_start_ms <= event.time_ms {
              State::create_run(state.started_gcs + 1)
            } else {
              *state
            }
          } else {
            State::create_wait(
              state.started_gcs,
              event.time_ms + long_delay_ms,
              state.last_gc_time_ms,
            )
          }
        }
        EventType::MarkCompact => State::create_wait(
          state.started_gcs,
          event.time_ms + long_delay_ms,
          event.time_ms,
        ),
      }
    }
    Id::Run => {
      // 原版 CHECK_LE (memory-reducer.cc:235), release 亦生效
      assert!(state.started_gcs <= max_gcs);
      if matches!(event.event_type, EventType::MarkCompact) {
        if state.started_gcs < max_gcs
          && (event.next_gc_likely_to_collect_more || state.started_gcs == 1)
        {
          State::create_wait(
            state.started_gcs,
            event.time_ms + K_SHORT_DELAY_MS as f64,
            event.time_ms,
          )
        } else {
          State::create_done(event.time_ms, event.committed_memory)
        }
      } else {
        *state
      }
    }
  }
}

/// 定时器投递延迟换算 — (delay_ms + kSlackMs) / 1000.0 秒。
/// v8: src/heap/memory-reducer.cc:243 MemoryReducer::ScheduleTimer
#[must_use]
pub const fn schedule_delay_seconds(delay_ms: f64) -> f64 {
  (delay_ms + K_SLACK_MS) / 1000.0
}

/// MaxNumberOfGCs — 返回 v8_flags.memory_reducer_gc_count。
/// v8: src/heap/memory-reducer.cc:254 MemoryReducer::MaxNumberOfGCs
#[must_use]
pub const fn max_number_of_gcs(memory_reducer_gc_count: i32) -> i32 {
  memory_reducer_gc_count
}

// —— 编译期不变式 ——
const _: () = {
  assert!(K_SHORT_DELAY_MS == 500);
  assert!(K_WATCHDOG_DELAY_MS == 100_000);
  assert!(K_COMMITTED_MEMORY_DELTA == 10 * 1024 * 1024);
};

/// 状态机场景不变式 (对齐 v8/test/unittests/heap/memory-reducer-unittest.cc 8 组用例)。
/// 使用运行时断言而非 const 上下文以支持 f64 比较。
#[cfg(test)]
mod state_machine_invariants {
  use super::*;

  fn timer(t: f64) -> Event {
    Event {
      event_type: EventType::Timer,
      time_ms: t,
      committed_memory: 0,
      next_gc_likely_to_collect_more: false,
      should_start_incremental_gc: false,
      can_start_incremental_gc: false,
    }
  }
  fn mc(t: f64, mem: usize) -> Event {
    Event {
      event_type: EventType::MarkCompact,
      time_ms: t,
      committed_memory: mem,
      next_gc_likely_to_collect_more: false,
      should_start_incremental_gc: false,
      can_start_incremental_gc: false,
    }
  }
  fn pg(t: f64) -> Event {
    Event {
      event_type: EventType::PossibleGarbage,
      time_ms: t,
      committed_memory: 0,
      next_gc_likely_to_collect_more: false,
      should_start_incremental_gc: false,
      can_start_incremental_gc: false,
    }
  }

  const LONG: f64 = 2000.0;
  const START: f64 = 8000.0;
  const MAX: i32 = 4;

  #[test]
  fn schedule_delay_seconds_matches_slack() {
    assert_eq!(schedule_delay_seconds(500.0), 0.6);
  }

  #[test]
  fn uninit_timer_stays_uninit() {
    let s = step(
      &State::create_uninitialized(),
      &timer(100.0),
      LONG,
      START,
      MAX,
    );
    assert!(matches!(s.id, Id::Uninit));
  }

  #[test]
  fn uninit_possible_garbage_uses_start_delay() {
    let s = step(&State::create_uninitialized(), &pg(100.0), LONG, START, MAX);
    assert!(matches!(s.id, Id::Wait));
    assert_eq!(s.started_gcs, 0);
    assert_eq!(s.next_gc_start_ms, 100.0 + 8000.0);
    assert_eq!(s.last_gc_time_ms, 0.0);
  }

  #[test]
  fn done_mark_compact_below_threshold_stays_done() {
    // kCommittedMemoryFactor = 1.1, kCommittedMemoryDelta = 10MB
    // last = 100MB, threshold = max(110MB, 110MB) = 110MB
    let done = State::create_done(50.0, 100 * 1024 * 1024);
    // 111MB > threshold, upstream `if (memory < threshold) keep else wait` → 超过则进 WAIT
    let s = step(&done, &mc(100.0, 111 * 1024 * 1024), LONG, START, MAX);
    assert!(matches!(s.id, Id::Wait));
    // 105MB < threshold 110MB → 保持 DONE
    let s2 = step(&done, &mc(100.0, 105 * 1024 * 1024), LONG, START, MAX);
    assert!(matches!(s2.id, Id::Done));
  }

  #[test]
  fn wait_timer_run_when_ready() {
    let wait = State::create_wait(2, 5000.0, 1.0);
    let mut e = timer(6000.0);
    e.can_start_incremental_gc = true;
    e.should_start_incremental_gc = true;
    let s = step(&wait, &e, LONG, START, MAX);
    assert!(matches!(s.id, Id::Run));
    assert_eq!(s.started_gcs, 3);
  }

  #[test]
  fn wait_timer_watchdog_forces_run() {
    let wait = State::create_wait(2, 5000.0, 1.0);
    let mut e = timer(100_002.0);
    e.can_start_incremental_gc = true;
    e.should_start_incremental_gc = false;
    assert!(watchdog_gc(&wait, &e));
    let s = step(&wait, &e, LONG, START, MAX);
    assert!(matches!(s.id, Id::Run));
  }

  #[test]
  fn wait_timer_exhausted_gcs_transitions_done() {
    let wait4 = State::create_wait(4, 5000.0, 42.0);
    let mut e = timer(6000.0);
    e.committed_memory = 77;
    let s = step(&wait4, &e, LONG, START, MAX);
    assert!(matches!(s.id, Id::Done));
    assert_eq!(s.last_gc_time_ms, 42.0);
    assert_eq!(s.committed_memory_at_last_run, 77);
  }

  #[test]
  fn run_first_round_mark_compact_back_to_wait_short_delay() {
    let run1 = State::create_run(1);
    let s = step(&run1, &mc(100.0, 0), LONG, START, MAX);
    assert!(matches!(s.id, Id::Wait));
    assert_eq!(s.started_gcs, 1);
    assert_eq!(s.next_gc_start_ms, 100.0 + K_SHORT_DELAY_MS as f64);
    assert_eq!(s.last_gc_time_ms, 100.0);
  }

  #[test]
  fn run_second_round_mark_compact_no_more_garbage_transitions_done() {
    let run2 = State::create_run(2);
    let mut e = mc(100.0, 55);
    e.next_gc_likely_to_collect_more = false;
    let s = step(&run2, &e, LONG, START, MAX);
    assert!(matches!(s.id, Id::Done));
    assert_eq!(s.last_gc_time_ms, 100.0);
    assert_eq!(s.committed_memory_at_last_run, 55);
  }

  #[test]
  fn run_at_max_transitions_done_on_mark_compact() {
    let run4 = State::create_run(4);
    let s = step(&run4, &mc(100.0, 10), LONG, START, MAX);
    assert!(matches!(s.id, Id::Done));
  }
}
