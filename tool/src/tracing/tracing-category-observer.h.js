#!/usr/bin/env bun
// v8/src/tracing/{tracing-category-observer.h, tracing-category-observer.cc}
//   → v8-tracing/src/category_observer.rs
//
// 提取规则代码:
//   - Mode 3 位常量 (tracing-category-observer.h:21-25)
//   - OnTraceEnabled 的 6 条 (类别组, 置位) 映射 (tracing-category-observer.cc:46-82)
//   - OnTraceDisabled 的 4 条清除映射 (tracing-category-observer.cc:91-100, 注意 zone_stats 不清除)
//   - 置位/清除走 i::TracingFlags (v8-logging tracing_flags.rs 的原子静态)
// 复跑幂等; 挂接语义 (SetUp/TearDown) 就地修订。
import { readV8, writeSrc, banner } from "../../lib.js";

const CRATE = "v8-tracing";
const GEN = "src/tracing/tracing-category-observer.h.js";
const H = readV8("src/tracing/tracing-category-observer.h");
const CC = readV8("src/tracing/tracing-category-observer.cc");

// ---- Mode 位 ----
const modes = [...H.matchAll(/ENABLED_BY_(\w+) = (1 << \d+)/g)].map((m) => [`ENABLED_BY_${m[1]}`, m[2]]);
if (modes.length !== 3) throw new Error(`Mode ${modes.length} != 3`);

// ---- OnTraceEnabled 映射: TRACE_EVENT_CATEGORY_GROUP_ENABLED(X, &enabled) → flags.fetch_or(BIT) ----
const enableMaps = [];
{
  const re = /TRACE_EVENT_CATEGORY_GROUP_ENABLED\(\s*TRACE_DISABLED_BY_DEFAULT\("([^"]+)"\),\s*&enabled\);\s*if \(enabled\) \{\s*i::TracingFlags::(\w+)\.fetch_or\((\w+)/g;
  let m;
  while ((m = re.exec(CC))) enableMaps.push([m[1], m[2], m[3]]);
}
if (enableMaps.length !== 6) throw new Error(`enable 映射 ${enableMaps.length} != 6`);

// ---- OnTraceDisabled 映射: flags.fetch_and(~BITS) (首条带括号, 其余裸 ~) ----
const disableMaps = [];
{
  const re = /i::TracingFlags::(\w+)\.fetch_and\(\s*~(?:\(([^)]+)\)|([^,]+)),/g;
  let m;
  while ((m = re.exec(CC))) disableMaps.push([m[1], (m[2] ?? m[3]).replace(/\s+/g, "")]);
}
if (disableMaps.length !== 4) throw new Error(`disable 映射 ${disableMaps.length} != 4`);

const RUST_FLAG = { runtime_stats: "RUNTIME_STATS", gc: "GC", gc_stats: "GC_STATS", ic_stats: "IC_STATS", zone_stats: "ZONE_STATS" };

let out = `${banner(GEN, "src/tracing/tracing-category-observer.h;src/tracing/tracing-category-observer.cc", "Mode 位+启停映射表, 挂接语义就地修订")}
//! TracingCategoryObserver (tracing-category-observer.h/.cc, 47+104 行)。
//!
//! 原版 no-perfetto 分支: 实现 \`TracingController::TraceStateObserver\`
//! (OnTraceEnabled/OnTraceDisabled), SetUp 时挂到平台 controller, 依据
//! disabled-by-default 类别组是否启用, 对 \`i::TracingFlags\` 原子位 fetch_or/fetch_and。
//! Rust 侧 TracingFlags 静态在 v8-logging (tracing_flags.rs) — 直接复用 (其
//! ENABLED_BY_* 常量标【待回收】, 与本模块 Mode 位数值一致)。
//! perfetto 分支 (TrackEventSessionObserver) 【推迟登记: 依赖 perfetto SDK】。

use std::sync::atomic::Ordering;

use v8_logging::{GC, GC_STATS, IC_STATS, RUNTIME_STATS, ZONE_STATS};

use crate::trace_event::category_enabled;

// ---- Mode (tracing-category-observer.h:21-25) ----
${modes.map(([n, v]) => `pub const ${n}: u32 = ${v};`).join("\n")}

// TODO(修订): OnTraceEnabled ${enableMaps.length} 条映射 (tracing-category-observer.cc:46-82):
${enableMaps.map(([cat, flag, bit]) => `//   disabled-by-default-${cat} → TracingFlags::${flag} |= ${bit}`).join("\n")}
// TODO(修订): OnTraceDisabled ${disableMaps.length} 条清除 (tracing-category-observer.cc:91-100, zone_stats 不清除):
${disableMaps.map(([flag, bits]) => `//   TracingFlags::${flag} &= ~(${bits})`).join("\n")}
`;

writeSrc(CRATE, "category_observer.rs", out);
console.log(`== tracing-category-observer.h.js == modes=3 enable=${enableMaps.length} disable=${disableMaps.length}`);
console.log(enableMaps.map(([c, f, b]) => `  +${c} → ${f} |= ${b}`).join("\n"));
console.log(disableMaps.map(([f, b]) => `  -${f} &= ~(${b})`).join("\n"));
