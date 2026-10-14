#!/usr/bin/env bun
// tool/src/logging/runtime-call-stats.h.js
// 机械提取 v8/src/logging/runtime-call-stats.{h,cc} + runtime-call-stats-scope.h:
// RCS 枚举结构/线程模式/作用域协议 (完整计数器名表见手写 rcs_names.rs, 不重复)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/logging/runtime-call-stats.h");
const ccSrc = readV8("src/logging/runtime-call-stats.cc");
const scopeH = readV8("src/logging/runtime-call-stats-scope.h");

// 1. 枚举结构: 五段 X-macro (GC/MANUAL/INTRINSIC/BUILTIN_C/API/HANDLER/THREAD_SPECIFIC)
const sections = ["FOR_EACH_GC_COUNTER", "FOR_EACH_MANUAL_COUNTER", "FOR_EACH_INTRINSIC", "BUILTIN_LIST_C", "FOR_EACH_API_COUNTER", "FOR_EACH_HANDLER_COUNTER", "FOR_EACH_THREAD_SPECIFIC_COUNTER"];
for (const s of sections) {
  if (!hSrc.includes(s)) throw new Error(`RuntimeCallCounterId 缺段 ${s}`);
}
if (!hSrc.includes("kNumberOfCounters,")) throw new Error("缺 kNumberOfCounters 哨兵");

// 2. ThreadType / CounterMode
const threadM = hSrc.match(/enum ThreadType \{ (kMainIsolateThread), (kWorkerThread) \};/);
if (!threadM) throw new Error("ThreadType 漂移");
const modeM = hSrc.match(/enum CounterMode \{ (kExact), (kThreadSpecific) \};/);
if (!modeM) throw new Error("CounterMode 漂移");

// 3. 作用域协议
for (const k of ["RuntimeCallTimerScope"]) {
  if (!scopeH.includes(k)) throw new Error(`scope 头缺 ${k}`);
}
// cc: kNames 表
if (!ccSrc.includes("kNames")) throw new Error("cc 缺 kNames 名称表");

// 4. V8_RUNTIME_CALL_STATS 开关
if (!hSrc.includes("#ifdef V8_RUNTIME_CALL_STATS")) throw new Error("缺 RCS 开关");

let rust = banner(
  "src/logging/runtime-call-stats.h.js",
  "src/logging/runtime-call-stats.{h,cc} + runtime-call-stats-scope.h",
  "运行时调用统计: 枚举结构与作用域协议",
);

rust += `
/// RCS 线程类别 (原版 enum ThreadType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ThreadType {
  MainIsolateThread = 0,
  WorkerThread = 1,
}

/// 计数器模式 (原版 enum CounterMode): 精确单计数器 vs 主/工作线程分立
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CounterMode {
  Exact = 0,
  ThreadSpecific = 1,
}

/// RuntimeCallCounterId 枚举段序 (原版展开次序; 完整名表见 rcs_names.rs)
pub mod rcs_enum_structure {
  /// 五段 X-macro 来源 (对数: ${sections.length})
  pub const SECTIONS: [&str; ${sections.length}] = [${sections.map((s) => `"${s}"`).join(", ")}];
  /// 尾部哨兵 (原版 kNumberOfCounters)
  pub const NUMBER_OF_COUNTERS_SENTINEL: &str = "kNumberOfCounters";
  /// 开关: V8_RUNTIME_CALL_STATS (x64/arm64 常规构建关闭)
  pub const ENABLED: bool = false;
}

/// RCS 作用域协议骨架 (语义见手写 runtime_call_stats.rs)
pub mod rcs_scope_skel {
  /// 进入计数 (原版 RuntimeCallScope: 挂到当前线程栈顶, 暂停前任)
  pub fn enter(_counter_id: usize) {
    unimplemented!("骨架")
  }
  /// 离开计数 (原版 ~RuntimeCallScope: 累加用时, 恢复前任)
  pub fn leave() {
    unimplemented!("骨架")
  }
  /// 计时器栈深度 (原版 current_timer 栈)
  pub fn current_timer() -> usize {
    unimplemented!("骨架")
  }
}

/// RuntimeCallCounter 记录单元骨架 (原版 name_/count_/time_)
#[derive(Debug, Clone, Copy, Default)]
pub struct RuntimeCallCounterSkel {
  pub count: i64,
  /// 微秒累计 (原版 time_ 以微秒存)
  pub time_us: i64,
}
`;

const p = writeSrc("v8-logging", "rcs_generated.rs", rust);
console.log(`written ${p}; 段=${sections.length}, 作用域=3`);
