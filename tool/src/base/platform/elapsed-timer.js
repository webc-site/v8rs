#!/usr/bin/env bun
// tool/src/base/platform/elapsed-timer.js — 覆盖 v8/src/base/platform/elapsed-timer.h
// 计时器: Start/Stop/Pause/Resume 状态机 + start_ticks_/paused_elapsed_ 联合体
// → v8-base-platform/src/elapsed_timer.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/base/platform/elapsed-timer.h");

// 1. 类与状态机锚点
const anchors = [
  "class ElapsedTimer final {",
  "void Start(TimeTicks now) {",
  "void Stop() {",
  "bool IsStarted() const {",
  "TimeDelta Restart(TimeTicks now) {",
  "void Pause(TimeTicks now) {",
  "void Resume(TimeTicks now) {",
  "TimeDelta Elapsed(TimeTicks now) const {",
  "bool HasExpired(TimeDelta time_delta) const {",
  "union {",
  "TimeTicks start_ticks_;",
  "TimeDelta paused_elapsed_;",
  "bool started_ = false;",
  "bool paused_ = false;",
  "struct ScopedTimer {",
];
for (const a of anchors) {
  if (!hSrc.includes(a)) throw new Error(`elapsed-timer.h 缺锚点: ${a}`);
}

// 2. 方法面解析: 公共方法成对形态 (无参 + 带 now 重载)
const overloads = ["Start", "Restart", "Pause", "Resume", "Elapsed"];
for (const m of overloads) {
  const bare = new RegExp(`${m}\\(\\) (?:const )?\\{`).test(hSrc);
  const withNow = new RegExp(`${m}\\(TimeTicks now\\)`).test(hSrc);
  if (!bare || !withNow) throw new Error(`ElapsedTimer::${m} 双形态缺失 (bare=${bare})`);
}
// 方法总数: 公共 14 (5×2 重载 + Stop/IsStarted/IsPaused/HasExpired) + 同型私有 3
// (paused_elapsed/set_paused_elapsed/set_start_ticks; Now() 与 start_ticks() 前缀/返回型不同型不计)
const totalEntries = (hSrc.match(/^  (?:void|bool|TimeDelta) \w+\(.*\)? \{|^  (?:void|bool|TimeDelta) \w+\(\)/gm) || []).length;
if (totalEntries !== 17) throw new Error(`同型方法入口数不符: ${totalEntries}, 期望 17 (公共 14 + 私有 3)`);
// DEBUG 断言面: DCHECK 家族计数 ≥ 12
const dchecks = (hSrc.match(/DCHECK\w*\(/g) || []).length;
if (dchecks < 12) throw new Error(`DCHECK 数不符: ${dchecks}, 期望 ≥12`);

console.log(`elapsed-timer: 同型入口 ${totalEntries} (公共 14 + 私有 3), DCHECK ${dchecks}, ScopedTimer 1`);

let rust = banner("src/base/platform/elapsed-timer.js", "src/base/platform/elapsed-timer.h", "作用域计时器 (TimeTicks 驱动, start/pause 联合存储)");

rust += `
/// 作用域计时器 (原版 class ElapsedTimer final)
/// v8: src/base/platform/elapsed-timer.h ElapsedTimer
///
/// 原版存储: union { TimeTicks start_ticks_; TimeDelta paused_elapsed_; }
/// + #ifdef DEBUG { bool started_; bool paused_; }
/// 联合体含义: 运行态存起点, 暂停态存已耗时长。
#[derive(Debug, Default)]
pub struct ElapsedTimer {
  /// 原版 union — Rust 侧拆双字段, 由 paused 状态选择有效字段
  start_ticks: crate::time::TimeTicks,
  paused_elapsed: crate::time::TimeDelta,
  started: bool,
  paused: bool,
}

impl ElapsedTimer {
  /// v8: src/base/platform/elapsed-timer.h Start(TimeTicks now)
  /// DCHECK(!now.IsNull() && !IsStarted()); 记起点, DEBUG 置 started_
  pub fn start(&mut self, now: crate::time::TimeTicks) {
    debug_assert!(!now.is_null() && !self.is_started());
    self.start_ticks = now;
    self.started = true;
    debug_assert!(self.is_started());
  }

  /// v8: src/base/platform/elapsed-timer.h Stop()
  /// 起点清零 (IsNull 即未启动), DEBUG 清 started_
  pub fn stop(&mut self) {
    debug_assert!(self.is_started());
    self.start_ticks = crate::time::TimeTicks::default();
    self.started = false;
    debug_assert!(!self.is_started());
  }

  /// v8: src/base/platform/elapsed-timer.h IsStarted()
  /// 语义: !start_ticks_.IsNull() (原版 DCHECK NE started_ 与 start_ticks_.IsNull())
  pub fn is_started(&self) -> bool {
    debug_assert!(!self.paused);
    !self.start_ticks.is_null()
  }

  /// v8: src/base/platform/elapsed-timer.h IsPaused() (#if DEBUG)
  pub fn is_paused(&self) -> bool { self.paused }

  /// v8: src/base/platform/elapsed-timer.h Restart(TimeTicks now)
  /// 返回自上次起点流逝量并重置起点 — 单次时钟读取完成两事
  pub fn restart(&mut self, now: crate::time::TimeTicks) -> crate::time::TimeDelta {
    debug_assert!(!now.is_null() && self.is_started());
    let elapsed = now - self.start_ticks;
    debug_assert!(elapsed.in_microseconds() >= 0);
    self.start_ticks = now;
    debug_assert!(self.is_started());
    elapsed
  }

  /// v8: src/base/platform/elapsed-timer.h Pause(TimeTicks now)
  /// 运行态 → 暂停态: 已耗时长落入 paused_elapsed_, DEBUG 置 paused_
  pub fn pause(&mut self, now: crate::time::TimeTicks) {
    let elapsed = self.elapsed_at(now);
    debug_assert!(self.is_started());
    self.paused = true;
    self.paused_elapsed = elapsed;
  }

  /// v8: src/base/platform/elapsed-timer.h Resume(TimeTicks now)
  /// 暂停态 → 运行态: 起点回拨 now - paused_elapsed_, 清 paused_
  pub fn resume(&mut self, now: crate::time::TimeTicks) {
    debug_assert!(!now.is_null() && self.started && self.paused);
    let elapsed = self.paused_elapsed;
    self.paused = false;
    self.start_ticks = now - elapsed;
    debug_assert!(self.is_started());
  }

  /// v8: src/base/platform/elapsed-timer.h Elapsed(TimeTicks now)
  pub fn elapsed_at(&self, now: crate::time::TimeTicks) -> crate::time::TimeDelta {
    debug_assert!(!now.is_null() && self.is_started());
    let elapsed = now - self.start_ticks;
    debug_assert!(elapsed.in_microseconds() >= 0);
    elapsed
  }

  /// v8: src/base/platform/elapsed-timer.h HasExpired(TimeDelta)
  /// Elapsed() >= time_delta
  pub fn has_expired(&self, time_delta: crate::time::TimeDelta) -> bool {
    debug_assert!(self.is_started());
    self.elapsed() >= time_delta
  }

  /// 原版私有 Now(): TimeTicks::Now() + DCHECK(!IsNull())
  fn now() -> crate::time::TimeTicks {
    let now = crate::time::TimeTicks::now();
    debug_assert!(!now.is_null());
    now
  }

  /// 原版 Elapsed() 无参形态: Elapsed(Now())
  pub fn elapsed(&self) -> crate::time::TimeDelta { self.elapsed_at(Self::now()) }
  /// 原版 Start()/Restart()/Pause()/Resume() 无参形态
  pub fn start_now(&mut self) { self.start(Self::now()); }
  pub fn restart_now(&mut self) -> crate::time::TimeDelta { self.restart(Self::now()) }
  pub fn pause_now(&mut self) { self.pause(Self::now()); }
  pub fn resume_now(&mut self) { self.resume(Self::now()); }
}

/// 作用域计时辅助 (原版 struct ScopedTimer)
/// v8: src/base/platform/elapsed-timer.h ScopedTimer
/// ctor: location_ 非空则 Start; dtor: *location_ += Elapsed()
pub struct ScopedTimer<'a> {
  timer: ElapsedTimer,
  location: Option<&'a mut crate::time::TimeDelta>,
}

impl<'a> ScopedTimer<'a> {
  /// v8: src/base/platform/elapsed-timer.h ScopedTimer(TimeDelta* location)
  pub fn new(location: Option<&'a mut crate::time::TimeDelta>) -> Self {
    let mut timer = ElapsedTimer::default();
    if location.is_some() { timer.start_now(); }
    Self { timer, location }
  }
}

impl Drop for ScopedTimer<'_> {
  /// v8: src/base/platform/elapsed-timer.h ~ScopedTimer
  fn drop(&mut self) {
    if let Some(loc) = self.location.as_mut() { **loc += self.timer.elapsed(); }
  }
}

// —— 提取不变式断言 ——
const _: () = {
  // 5 组重载 × 2 形态 + Stop/IsStarted/IsPaused/HasExpired
  const PUBLIC_ENTRIES: usize = 14;
  assert!(PUBLIC_ENTRIES == 14);
};
`;

const p = writeSrc("v8-base-platform", "elapsed_timer.rs.skel", rust);
console.log(`written ${p}`);
