#!/usr/bin/env bun
// tool/src/heap/memory-reducer.cc.js
// 机械提取 v8/src/heap/memory-reducer.{h,cc}: 窗口常量 + State/Event 数据面 +
// Step 状态机纯函数 1:1 转写 + Heap 耦合面签名骨架
// 枚举判别值面已在 memory_reducer_enums.rs (memory-reducer.h.js), 此处为 cc 规则面

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/memory-reducer.h");
const ccSrc = readV8("src/heap/memory-reducer.cc");

// —— 窗口常量 (cc:41-44) ——
const c = {};
for (const [, , k, v] of ccSrc.matchAll(/const (int|double|size_t) MemoryReducer::k(\w+) = ([^;]+);/g)) {
  c[k] = v.trim();
}
if (c.ShortDelayMs !== "500") throw new Error(`kShortDelayMs 漂移: ${c.ShortDelayMs}`);
if (c.WatchdogDelayMs !== "100000") throw new Error(`kWatchdogDelayMs 漂移: ${c.WatchdogDelayMs}`);
if (c.CommittedMemoryFactor !== "1.1") throw new Error(`kCommittedMemoryFactor 漂移: ${c.CommittedMemoryFactor}`);
if (c.CommittedMemoryDelta.replace(/\s+/g, "") !== "10*MB") throw new Error(`kCommittedMemoryDelta 漂移: ${c.CommittedMemoryDelta}`);

// —— kSlackMs (cc:247, ScheduleTimer 内局部常量) ——
if (!/\[\[maybe_unused\]\]\s+const double kSlackMs = 100;|const double kSlackMs = 100;/.test(ccSrc)) {
  throw new Error("kSlackMs 缺失或漂移");
}

// —— .h 数据面锚 ——
const idm = hSrc.match(/enum Id \{([^}]*)\};/);
if (!idm) throw new Error("enum Id 缺失");
const ids = idm[1].split(",").map((s) => s.trim()).filter(Boolean);
if (ids.join() !== "kUninit,kDone,kWait,kRun") throw new Error(`Id 序漂移: ${ids.join()}`);
const etm = hSrc.match(/enum EventType \{([^}]*)\};/);
if (!etm) throw new Error("enum EventType 缺失");
const ets = etm[1].split(",").map((s) => s.trim()).filter(Boolean);
if (ets.join() !== "kTimer,kMarkCompact,kPossibleGarbage") throw new Error(`EventType 序漂移: ${ets.join()}`);

// State 工厂初始化式 (h:109-122) — 五字段序 [id, started_gcs, next_gc_start_ms, last_gc_time_ms, committed_memory_at_last_run]
const factories = {
  CreateUninitialized: [0, 0, 0, 0, 0],
  CreateDone: [3, 0, 0, 1, 1],
  CreateWait: [2, 1, 1, 1, 0],
  CreateRun: [3, 1, 0, 0, 0],
};
const factSrc = {
  CreateUninitialized: hSrc.match(/State CreateUninitialized\(\)\s*\{\s*return \{([^}]*)\};/),
  CreateDone: hSrc.match(/State CreateDone\(double last_gc_time_ms, size_t committed_memory\)\s*\{\s*return \{([^}]*)\};/),
  CreateWait: hSrc.match(/State CreateWait\(int started_gcs, double next_gc_time_ms,\s*double last_gc_time_ms\)\s*\{\s*return \{([^}]*)\};/),
  CreateRun: hSrc.match(/State CreateRun\(int started_gcs\)\s*\{\s*return \{([^}]*)\};/),
};
for (const [k, re] of Object.entries(factSrc)) {
  if (!re) throw new Error(`State 工厂 ${k} 缺失`);
  const args = re[1].split(",").map((s) => s.trim());
  if (args.length !== 5) throw new Error(`${k} 初始化字段数 ${args.length}, 期望 5`);
}

// Event 字段 (h:164-171)
const evm = hSrc.match(/struct Event \{([\s\S]*?)\n  \};/);
if (!evm) throw new Error("struct Event 缺失");
const evFields = [...evm[1].matchAll(/^\s*(\w+) (\w+);/gm)].map((m) => `${m[1]} ${m[2]}`);
if (evFields.join("|") !== "EventType type|double time_ms|size_t committed_memory|bool next_gc_likely_to_collect_more|bool should_start_incremental_gc|bool can_start_incremental_gc") {
  throw new Error(`Event 字段面漂移: ${evFields.join("|")}`);
}

// —— Step 状态机结构锚 (cc:168-241) ——
const stepStart = ccSrc.indexOf("MemoryReducer::State MemoryReducer::Step(");
if (stepStart < 0) throw new Error("Step 定义缺失");
const stepEnd = ccSrc.indexOf("\n}", stepStart);
const stepBody = ccSrc.slice(stepStart, stepEnd);
const stepCases = [...stepBody.matchAll(/case (k\w+):/g)].map((m) => m[1]);
if (stepCases.join() !== "kUninit,kDone,kWait,kPossibleGarbage,kTimer,kMarkCompact,kRun") {
  throw new Error(`Step case 序漂移: ${stepCases.join()}`);
}
if (!stepBody.includes("CHECK_LE(state.started_gcs(), MaxNumberOfGCs())")) throw new Error("Step CHECK_LE 锚缺失");
if (!stepBody.includes("WatchdogGC(state, event)")) throw new Error("WatchdogGC 调用锚缺失");
if (!/state\.committed_memory_at_last_run\(\) \*\s*kCommittedMemoryFactor/.test(stepBody)) throw new Error("Factor 项锚缺失");
if (!/state\.committed_memory_at_last_run\(\) \+ kCommittedMemoryDelta/.test(stepBody)) throw new Error("Delta 项锚缺失");
if (!/gc_memory_reducer_start_delay_ms/.test(stepBody)) throw new Error("start_delay flag 锚缺失");
if (!/v8_flags\.memory_reducer_delay_ms/.test(stepBody)) throw new Error("long_delay flag 锚缺失");
if (!/state\.started_gcs\(\) == 1/.test(stepBody)) throw new Error("kRun 首轮特判锚缺失");

// WatchdogGC (cc:161-164)
const wd = ccSrc.match(/bool MemoryReducer::WatchdogGC\(const State& state, const Event& event\) \{\s*return ([^;]+);/s);
if (!wd) throw new Error("WatchdogGC 缺失");
if (!wd[1].includes("state.last_gc_time_ms() != 0") || !wd[1].includes("kWatchdogDelayMs")) {
  throw new Error(`WatchdogGC 规则漂移: ${wd[1].replace(/\s+/g, " ")}`);
}

// MaxNumberOfGCs (cc:254-257): v8_flags.memory_reducer_gc_count
if (!ccSrc.includes("return v8_flags.memory_reducer_gc_count;")) throw new Error("MaxNumberOfGCs 来源锚缺失");

// —— Heap 耦合面签名骨架清单: [原版签名, 行号, rust 骨架名] ——
const heapFns = [
  ["size_t GetCommitedSize(Heap* heap)", 20, "get_commited_size"],
  ["MemoryReducerBase::MemoryReducerBase(Heap* heap)", 26, "base_new"],
  ["std::unique_ptr<MemoryReducerBase> MemoryReducerBase::Create(Heap* heap)", 33, "base_create"],
  ["void MemoryReducer::TimerTask::RunInternal()", 56, "reducer_timer_task_run_internal"],
  ["void MemoryReducer::NotifyTimer(const Event& event)", 92, "reducer_notify_timer"],
  ["void MemoryReducer::NotifyMarkCompact(size_t committed_memory_before)", 117, "reducer_notify_mark_compact"],
  ["void MemoryReducer::NotifyPossibleGarbage()", 145, "reducer_notify_possible_garbage"],
  ["void MemoryReducer::ScheduleTimer(double delay_ms)", 243, "reducer_schedule_timer"],
  ["LimitBasedMemoryReducer::LimitBasedMemoryReducer(Heap* heap)", 259, "limit_based_new"],
  ["void LimitBasedMemoryReducer::NotifyMarkCompact(size_t committed_memory_before)", 262, "limit_based_notify_mark_compact"],
  ["void LimitBasedMemoryReducer::NotifyPossibleGarbage()", 268, "limit_based_notify_possible_garbage"],
  ["void LimitBasedMemoryReducer::ScheduleTimer(base::TimeDelta delay)", 277, "limit_based_schedule_timer"],
  ["void LimitBasedMemoryReducer::TimerTask::RunInternal()", 294, "limit_based_timer_task_run_internal"],
];
const ccFlat = ccSrc.replace(/\s+/g, " ").replace(/\(\s+/g, "(").replace(/\s+\)/g, ")");
for (const [sig] of heapFns) {
  if (!ccFlat.includes(sig)) throw new Error(`耦合面签名缺失: ${sig}`);
}

console.log(`memory-reducer: 常量 4 + slack, Id ${ids.length} + EventType ${ets.length}, 工厂 4×5 字段, Event ${evFields.length} 字段, case ${stepCases.length}`);

let rust = banner(
  "src/heap/memory-reducer.cc.js",
  "src/heap/memory-reducer.{h,cc}",
  "窗口常量 + Step 状态机纯函数; 枚举判别值面在 memory_reducer_enums.rs",
);

rust += `
//! MemoryReducer 状态机规则面 — Step 纯函数 1:1 转写。
//!
//! - 状态: UNINIT/DONE/WAIT/RUN; WAIT(n, x, t) = 已发起 n 次 GC, 下次最早 x, 上次 GC t
//! - 事件: TIMER / MARK_COMPACT / POSSIBLE_GARBAGE
//! - flag 依赖参数化: long_delay_ms (memory_reducer_delay_ms) /
//!   start_delay_ms (gc_memory_reducer_start_delay_ms) / max_gcs (memory_reducer_gc_count)
//! - 枚举判别值与 memory_reducer_enums.rs 同源, 挂接时由 Developer 合并 re-export

/// 短延迟 ms (kShortDelayMs, cc:41) — kRun→kWait 回退窗口
pub const K_SHORT_DELAY_MS: i32 = 500;
/// 看门狗延迟 ms (kWatchdogDelayMs, cc:42) — WAIT 期强制转 RUN 的时限
pub const K_WATCHDOG_DELAY_MS: i64 = 100000;
/// 触发增长因子 (kCommittedMemoryFactor, cc:43)
pub const K_COMMITTED_MEMORY_FACTOR: f64 = 1.1;
/// 触发增长增量 (kCommittedMemoryDelta = 10 * MB, cc:44)
pub const K_COMMITTED_MEMORY_DELTA: usize = 10 * 1024 * 1024;
/// 定时器调度余量 ms (kSlackMs, cc:247 ScheduleTimer 局部)
pub const K_SLACK_MS: f64 = 100.0;

/// 状态机状态 (enum Id; 判别值面见 memory_reducer_enums.rs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Id {
  /// kUninit
  UNINIT = 0,
  /// kDone
  DONE = 1,
  /// kWait
  WAIT = 2,
  /// kRun
  RUN = 3,
}

/// 触发事件 (enum EventType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EventType {
  /// kTimer
  TIMER = 0,
  /// kMarkCompact
  MARK_COMPACT = 1,
  /// kPossibleGarbage
  POSSIBLE_GARBAGE = 2,
}

/// 状态 (原版 class State 的五字段数据面; 字段序 = 原版声明序)
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
  /// State::CreateUninitialized — {kUninit, 0, 0, 0, 0}
  /// v8: src/heap/memory-reducer.h:109
  #[must_use]
  pub const fn create_uninitialized() -> Self {
    Self { id: Id::UNINIT, started_gcs: 0, next_gc_start_ms: 0.0, last_gc_time_ms: 0.0, committed_memory_at_last_run: 0 }
  }

  /// State::CreateDone — {kDone, 0, 0, last_gc_time_ms, committed_memory}
  /// v8: src/heap/memory-reducer.h:111
  #[must_use]
  pub const fn create_done(last_gc_time_ms: f64, committed_memory: usize) -> Self {
    Self { id: Id::DONE, started_gcs: 0, next_gc_start_ms: 0.0, last_gc_time_ms, committed_memory_at_last_run: committed_memory }
  }

  /// State::CreateWait — {kWait, started_gcs, next_gc_time_ms, last_gc_time_ms, 0}
  /// v8: src/heap/memory-reducer.h:115
  #[must_use]
  pub const fn create_wait(started_gcs: i32, next_gc_time_ms: f64, last_gc_time_ms: f64) -> Self {
    Self { id: Id::WAIT, started_gcs, next_gc_start_ms: next_gc_time_ms, last_gc_time_ms, committed_memory_at_last_run: 0 }
  }

  /// State::CreateRun — {kRun, started_gcs, 0, 0, 0}
  /// v8: src/heap/memory-reducer.h:120
  #[must_use]
  pub const fn create_run(started_gcs: i32) -> Self {
    Self { id: Id::RUN, started_gcs, next_gc_start_ms: 0.0, last_gc_time_ms: 0.0, committed_memory_at_last_run: 0 }
  }
}

/// 事件 (原版 struct Event 六字段, 字段序 = 原版声明序)
#[derive(Debug, Clone, Copy)]
pub struct Event {
  /// type
  pub event_type: EventType,
  /// time_ms
  pub time_ms: f64,
  /// committed_memory
  pub committed_memory: usize,
  /// next_gc_likely_to_collect_more — 上轮 GC 后仍有可收垃圾
  pub next_gc_likely_to_collect_more: bool,
  /// should_start_incremental_gc — 分配率低/后台模式
  pub should_start_incremental_gc: bool,
  /// can_start_incremental_gc — 增量标记可启动
  pub can_start_incremental_gc: bool,
}

/// 看门狗: 距上次 GC 超过 kWatchdogDelayMs 则强制可跑
/// v8: src/heap/memory-reducer.cc:161 MemoryReducer::WatchdogGC
#[must_use]
pub const fn watchdog_gc(state: &State, event: &Event) -> bool {
  state.last_gc_time_ms != 0.0 && event.time_ms > state.last_gc_time_ms + K_WATCHDOG_DELAY_MS as f64
}

/// 状态机步进 — MemoryReducer::Step 的 1:1 转写 (纯函数)
///
/// flag 依赖参数化: long_delay_ms = v8_flags.memory_reducer_delay_ms;
/// start_delay_ms = v8_flags.gc_memory_reducer_start_delay_ms (仅 POSSIBLE_GARBAGE 支);
/// max_gcs = MaxNumberOfGCs() = v8_flags.memory_reducer_gc_count
/// v8: src/heap/memory-reducer.cc:168 MemoryReducer::Step
#[must_use]
pub const fn step(state: &State, event: &Event, long_delay_ms: f64, start_delay_ms: f64, max_gcs: i32) -> State {
  match state.id {
    Id::UNINIT | Id::DONE => match event.event_type {
      EventType::TIMER => *state,
      EventType::MARK_COMPACT => {
        // 提交内存增幅需超 factor 与 delta 的较大者才重新进入 WAIT
        let last = state.committed_memory_at_last_run;
        let by_factor = (last as f64) * K_COMMITTED_MEMORY_FACTOR;
        let by_delta = last.saturating_add(K_COMMITTED_MEMORY_DELTA) as f64;
        let threshold = if by_delta > by_factor { by_delta } else { by_factor };
        if (event.committed_memory as f64) < threshold {
          *state
        } else {
          State::create_wait(0, event.time_ms + long_delay_ms, event.time_ms)
        }
      }
      EventType::POSSIBLE_GARBAGE => {
        State::create_wait(0, event.time_ms + start_delay_ms, state.last_gc_time_ms)
      }
    },
    Id::WAIT => {
      // 原版 CHECK_LE, release 亦生效
      assert!(state.started_gcs <= max_gcs);
      match event.event_type {
        EventType::POSSIBLE_GARBAGE => *state,
        EventType::TIMER => {
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
            State::create_wait(state.started_gcs, event.time_ms + long_delay_ms, state.last_gc_time_ms)
          }
        }
        EventType::MARK_COMPACT => {
          State::create_wait(state.started_gcs, event.time_ms + long_delay_ms, event.time_ms)
        }
      }
    }
    Id::RUN => {
      // 原版 CHECK_LE, release 亦生效
      assert!(state.started_gcs <= max_gcs);
      if matches!(event.event_type, EventType::MARK_COMPACT) {
        if state.started_gcs < max_gcs
          && (event.next_gc_likely_to_collect_more || state.started_gcs == 1)
        {
          State::create_wait(state.started_gcs, event.time_ms + K_SHORT_DELAY_MS as f64, event.time_ms)
        } else {
          State::create_done(event.time_ms, event.committed_memory)
        }
      } else {
        *state
      }
    }
  }
}

/// 定时器投递延迟换算 — (delay_ms + kSlackMs) / 1000.0 秒
/// v8: src/heap/memory-reducer.cc:243 MemoryReducer::ScheduleTimer
#[must_use]
pub const fn schedule_delay_seconds(delay_ms: f64) -> f64 {
  (delay_ms + K_SLACK_MS) / 1000.0
}
`;

// —— Heap 耦合面签名骨架 ——
rust += `
// —— Heap 耦合面签名骨架 (语义实现由 Developer 就地修订) ——
`;
for (const [sig, line, nm] of heapFns) {
  rust += `/// 原版签名: ${sig}
/// v8: src/heap/memory-reducer.cc:${line}
pub fn ${nm}() {
  todo!("就地修订")
}
`;
}
rust += `/// 原版签名: int MemoryReducer::MaxNumberOfGCs() — 返回 v8_flags.memory_reducer_gc_count
/// v8: src/heap/memory-reducer.cc:254
#[must_use]
pub fn max_number_of_gcs(memory_reducer_gc_count: i32) -> i32 {
  memory_reducer_gc_count
}
`;

rust += `
// —— 提取不变式断言 ——
const _: () = {
  assert!(K_SHORT_DELAY_MS == 500);
  assert!(K_WATCHDOG_DELAY_MS == 100_000);
  assert!(K_COMMITTED_MEMORY_FACTOR == 1.1);
  assert!(K_COMMITTED_MEMORY_DELTA == 10 * 1024 * 1024);
  assert!(K_SLACK_MS == 100.0);
  assert!(schedule_delay_seconds(500.0) == 0.6);

  let timer = Event { event_type: EventType::TIMER, time_ms: 100.0, committed_memory: 0, next_gc_likely_to_collect_more: false, should_start_incremental_gc: false, can_start_incremental_gc: false };
  let mc = Event { event_type: EventType::MARK_COMPACT, time_ms: 100.0, committed_memory: 0, next_gc_likely_to_collect_more: false, should_start_incremental_gc: false, can_start_incremental_gc: false };
  let pg = Event { event_type: EventType::POSSIBLE_GARBAGE, time_ms: 100.0, committed_memory: 0, next_gc_likely_to_collect_more: false, should_start_incremental_gc: false, can_start_incremental_gc: false };

  // UNINIT/DONE × TIMER → 原状态
  assert!(matches!(step(&State::create_uninitialized(), &timer, 2000.0, 8000.0, 4).id, Id::UNINIT));
  // UNINIT × POSSIBLE_GARBAGE → WAIT(0, t+start_delay, 0) — 用 start_delay 而非 long_delay
  let s = step(&State::create_uninitialized(), &pg, 2000.0, 8000.0, 4);
  assert!(matches!(s.id, Id::WAIT) && s.started_gcs == 0 && s.next_gc_start_ms == 8100.0 && s.last_gc_time_ms == 0.0);
  // DONE × MARK_COMPACT: 增幅 (100MB→110MB 临界, == factor 与 == delta 同点) 不小于阈值 → 原状态
  let done = State::create_done(50.0, 100 * 1024 * 1024);
  let mut e = mc;
  e.committed_memory = 100 * 1024 * 1024 + 10 * 1024 * 1024; // == 100*1.1 == 100+10MB
  assert!(step(&done, &e, 2000.0, 8000.0, 4).committed_memory_at_last_run == done.committed_memory_at_last_run);
  // DONE × MARK_COMPACT: 增幅超阈值 → WAIT(0, t+long_delay, t)
  e.committed_memory += 1;
  let s = step(&done, &e, 2000.0, 8000.0, 4);
  assert!(matches!(s.id, Id::WAIT) && s.started_gcs == 0 && s.next_gc_start_ms == 2100.0 && s.last_gc_time_ms == 100.0);
  // WAIT × TIMER 到达且可启动 → RUN(n+1)
  let wait = State::create_wait(2, 5000.0, 1.0);
  let mut e = timer;
  e.time_ms = 6000.0;
  e.can_start_incremental_gc = true;
  e.should_start_incremental_gc = true;
  assert!(matches!(step(&wait, &e, 2000.0, 8000.0, 4).id, Id::RUN));
  // WAIT × TIMER 看门狗强制 (last=1.0, t=100002 > 1+100000) → RUN
  let mut e = timer;
  e.time_ms = 100_002.0;
  e.can_start_incremental_gc = true;
  e.should_start_incremental_gc = false;
  assert!(matches!(step(&wait, &e, 2000.0, 8000.0, 4).id, Id::RUN));
  assert!(watchdog_gc(&wait, &e));
  // WAIT × TIMER 未到 next_gc_start_ms → 原状态
  let mut e = timer;
  e.time_ms = 4999.0;
  e.can_start_incremental_gc = true;
  e.should_start_incremental_gc = true;
  assert!(step(&wait, &e, 2000.0, 8000.0, 4).next_gc_start_ms == 5000.0);
  // WAIT × TIMER 次数耗尽 → DONE(last, committed)
  let wait4 = State::create_wait(4, 5000.0, 42.0);
  let mut e = timer;
  e.committed_memory = 77;
  let s = step(&wait4, &e, 2000.0, 8000.0, 4);
  assert!(matches!(s.id, Id::DONE) && s.last_gc_time_ms == 42.0 && s.committed_memory_at_last_run == 77);
  // RUN(1) × MARK_COMPACT 首轮 → WAIT(1, t+short, t)
  let run1 = State::create_run(1);
  let mut e = mc;
  e.next_gc_likely_to_collect_more = false;
  let s = step(&run1, &e, 2000.0, 8000.0, 4);
  assert!(matches!(s.id, Id::WAIT) && s.started_gcs == 1 && s.next_gc_start_ms == 600.0 && s.last_gc_time_ms == 100.0);
  // RUN(2) × MARK_COMPACT 无更多可收 → DONE(t, committed)
  let run2 = State::create_run(2);
  let mut e = mc;
  e.committed_memory = 55;
  let s = step(&run2, &e, 2000.0, 8000.0, 4);
  assert!(matches!(s.id, Id::DONE) && s.last_gc_time_ms == 100.0 && s.committed_memory_at_last_run == 55);
  // RUN(4) × MARK_COMPACT 到达上限 → DONE
  assert!(matches!(step(&State::create_run(4), &e, 2000.0, 8000.0, 4).id, Id::DONE));
};
`;

const p = writeSrc("v8-heap-gc", "memory_reducer_cc.rs.skel", rust);
console.log(`written ${p}`);
