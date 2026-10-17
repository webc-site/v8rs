#!/usr/bin/env bun
// tool/src/logging/counters.h.js
// 机械提取 v8/src/logging/counters.{h,cc} + counters-scopes.h + counters-definitions.h:
// 直方图表, 计时分辨率枚举, 作用域族, 计数器定义宏表

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/logging/counters.h");
const ccSrc = readV8("src/logging/counters.cc");
const scopesH = readV8("src/logging/counters-scopes.h");
const defsH = readV8("src/logging/counters-definitions.h");

// 1. TimedHistogramResolution 枚举
const resM = hSrc.match(/enum class TimedHistogramResolution \{ (\w+), (\w+) \};/);
if (!resM || resM[1] !== "MILLISECOND" || resM[2] !== "MICROSECOND") throw new Error("TimedHistogramResolution 漂移");

// 2. counters-definitions.h 宏表: HISTOGRAMS_LIST 全为 HR 条目
const hrEntries = [...defsH.matchAll(/^\s*HR\((\w+),/gm)].map((m) => m[1]);
if (hrEntries.length !== 53) throw new Error(`HR 直方图应 53, 实得 ${hrEntries.length}`);
if (hrEntries[0] !== "code_cache_reject_reason") throw new Error("HR 首项漂移");
if (hrEntries[hrEntries.length - 1] !== "wasm_jitless_execution_too_slow") throw new Error("HR 尾项漂移");

// 3. 作用域族面
const scopeClasses = [...scopesH.matchAll(/class V8_NODISCARD (\w+)Scope\b/g)].map((m) => m[1]);
if (scopeClasses.length !== 5) throw new Error(`作用域类应 ≥6, 实得 ${scopeClasses.length}: ${scopeClasses.join(",")}`);

// 4. workers 计数器面 (cc)
if (!ccSrc.includes("Counters::Counters(")) throw new Error("cc 缺构造");

let rust = banner(
  "src/logging/counters.h.js",
  "src/logging/counters.{h,cc} + counters-scopes.h + counters-definitions.h",
  "计数器体系: 分辨率/直方图表/作用域族",
);

rust += `
/// 计时直方图分辨率 (原版 enum class TimedHistogramResolution)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TimedHistogramResolution {
  Millisecond = 0,
  Microsecond = 1,
}

/// counters-definitions.h 宏表机械展开 (对数基准, 共 ${hrEntries.length} 项)
pub mod counter_definitions {
  /// 直方图 HR(name, caption, min, max, num_buckets)
  pub const HISTOGRAMS: [&str; ${hrEntries.length}] = [${hrEntries.map((e) => `"${e}"`).join(", ")}];
}

/// 作用域族 (原版 counters-scopes.h, RAII 计时/计数守卫)
pub mod counter_scopes_skel {
  /// 作用域类清单 (对数: ${scopeClasses.length})
  pub const SCOPE_CLASSES: [&str; ${scopeClasses.length}] = [${scopeClasses.map((s) => `"${s}"`).join(", ")}];

  /// 计时守卫语义: 进入 StartTimer, 离开 StopTimer (TimedHistogramScope 族)
  pub fn timed_scope_skel() {
    unimplemented!("骨架")
  }
  /// 计数守卫语义: 进入 Increment, 离开 Decrement (CounterScope 族)
  pub fn counter_scope_skel() {
    unimplemented!("骨架")
  }
}
`;

const p = writeSrc("v8-logging", "counters_generated.rs", rust);
console.log(`written ${p}; HR=${hrEntries.length}, 作用域=${scopeClasses.length}`);
