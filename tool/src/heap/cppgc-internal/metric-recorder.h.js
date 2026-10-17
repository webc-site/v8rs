#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/metric-recorder.h.js — v8/src/heap/cppgc-internal/metric-recorder.h → v8-heap-cppgc/src/metric_recorder.rs.skel
// 职责: MetricRecorder 嵌套事件结构体族 (GCCycle/Phases/IncrementalPhases/Sizes/Mark/Sweep) + 3 虚 AddMainThreadEvent 签名
// 规则:
//   ① 解析 GCCycle.Type 枚举 (kMinor/kMajor) → Rust enum
//   ② 解析 5 个 struct 的字段表 (类型+默认值), -1 默认值是"未采样"哨兵 → Rust Option/哨兵常量注释
//   ③ 3 个虚方法重载按参数类型区分 → Rust trait 3 方法 (重载拆名)
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/metric-recorder.h";
const src = readV8(REL);
let n = 0;
const need = (needle, label) => {
  if (!src.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
};
const count = (re) => (src.match(re) || []).length;

need("class MetricRecorder {", "类声明");
need("enum class Type { kMinor, kMajor };", "GCCycle.Type 枚举");
need("struct IncrementalPhases {", "IncrementalPhases");
need("struct Phases : public IncrementalPhases {", "Phases 继承");
need("struct Sizes {", "Sizes");
need("struct MainThreadIncrementalMark {", "Mark 事件");
need("struct MainThreadIncrementalSweep {", "Sweep 事件");
if (count(/virtual void AddMainThreadEvent\(/g) !== 3)
  throw new Error(`虚方法计数: got=${count(/virtual void AddMainThreadEvent\(/g)} want=3`);
n++;

// GCCycle.Type 枚举
const typeEnum = src.match(/enum class Type \{ (k\w+), (k\w+) \};/);
if (!typeEnum) throw new Error("Type 枚举正则未命中");
n++;
const variants = [typeEnum[1], typeEnum[2]];

// int64_t 字段默认 -1 计数 (未采样哨兵): Phases 2 + IncrementalPhases 2 + Sizes 3 + Mark 1 + Sweep 1 = 9
const negOne = count(/int64_t \w+ = -1;/g);
if (negOne !== 9) throw new Error(`-1 哨兵字段计数: got=${negOne} want=9`);
n++;

// GCCycle 聚合字段 10 个 (type + 3 Phases + 1 IncrementalPhases + 2 Sizes + 3 double), 类体内缩进 4 空格
const gccycleRegion = src.slice(src.indexOf("struct GCCycle {"), src.indexOf("};", src.indexOf("Sizes memory;")));
const gccycleFields = [...gccycleRegion.matchAll(/^ {4}(Type|Phases|IncrementalPhases|Sizes|double) (\w+)(;| = )/gm)];
if (gccycleFields.length !== 10)
  throw new Error(`GCCycle 聚合字段数: got=${gccycleFields.length} want=10`);
n++;

const snake = (s) => s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();
const rustEnum = variants.map((v) => `    ${v.slice(1).replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase()},`).join("\n");

const body = `${banner("src/heap/cppgc-internal/metric-recorder.h.js", REL, "MetricRecorder 事件结构体族 + 嵌入器上报 trait 骨架")}
// ══ MetricRecorder (原版 metric-recorder.h:21-64) ══
// 嵌入器实现 trait 上报 GC 直方图; Rust 侧以 trait MetricRecorder 对应 3 个虚 AddMainThreadEvent (重载拆名)。

/// GCCycle.Type — 原版 metric-recorder.h:24
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GcCycleType {
${rustEnum}
}

/// GCCycle.IncrementalPhases — 原版 metric-recorder.h:25-28; -1 = 未采样哨兵
#[derive(Debug, Clone, Copy, Default)]
pub struct IncrementalPhases {
    /// int64_t mark_duration_us = -1
    pub mark_duration_us: Option<i64>,
    /// int64_t sweep_duration_us = -1
    pub sweep_duration_us: Option<i64>,
}

/// GCCycle.Phases : IncrementalPhases — 原版 metric-recorder.h:29-32 (继承扁平化)
#[derive(Debug, Clone, Copy, Default)]
pub struct Phases {
    pub incremental: IncrementalPhases,
    /// int64_t weak_duration_us = -1
    pub weak_duration_us: Option<i64>,
    /// int64_t compact_duration_us = -1
    pub compact_duration_us: Option<i64>,
}

/// GCCycle.Sizes — 原版 metric-recorder.h:33-37; -1 = 未采样哨兵
#[derive(Debug, Clone, Copy, Default)]
pub struct Sizes {
    pub before_bytes: Option<i64>,
    pub after_bytes: Option<i64>,
    pub freed_bytes: Option<i64>,
}

/// GCCycle — 原版 metric-recorder.h:23-49; 10 聚合字段 (type + 4 阶段 + 2 Sizes + 3 率)
#[derive(Debug, Clone, Copy, Default)]
pub struct GcCycle {
    pub type_: GcCycleType, // 默认 kMajor
    pub total: Phases,
    pub main_thread: Phases,
    pub main_thread_atomic: Phases,
    pub main_thread_incremental: IncrementalPhases,
    pub objects: Sizes,
    pub memory: Sizes,
    pub collection_rate_in_percent: f64,
    pub efficiency_in_bytes_per_us: f64,
    pub main_thread_efficiency_in_bytes_per_us: f64,
}

/// MainThreadIncrementalMark — 原版 metric-recorder.h:51-53
#[derive(Debug, Clone, Copy, Default)]
pub struct MainThreadIncrementalMark {
    pub duration_us: Option<i64>, // = -1 未采样
}

/// MainThreadIncrementalSweep — 原版 metric-recorder.h:55-57
#[derive(Debug, Clone, Copy, Default)]
pub struct MainThreadIncrementalSweep {
    pub duration_us: Option<i64>, // = -1 未采样
}

/// MetricRecorder — 原版 metric-recorder.h:21-64; 3 虚方法默认空实现 → Rust 默认空 trait
pub trait MetricRecorder {
    /// virtual void AddMainThreadEvent(const GCCycle&) — metric-recorder.h:61
    fn add_main_thread_event_gc_cycle(&mut self, _event: &GcCycle) {}
    /// virtual void AddMainThreadEvent(const MainThreadIncrementalMark&) — metric-recorder.h:62
    fn add_main_thread_event_incremental_mark(&mut self, _event: &MainThreadIncrementalMark) {}
    /// virtual void AddMainThreadEvent(const MainThreadIncrementalSweep&) — metric-recorder.h:63
    fn add_main_thread_event_incremental_sweep(&mut self, _event: &MainThreadIncrementalSweep) {}
}
`;
const out = writeSrc("v8-heap-cppgc", "metric_recorder.rs.skel", body);
console.log(`OK metric-recorder.h.js: 锚 ${n} 条 (类1+结构6+虚方法计数1+枚举正则1+哨兵9+聚合字段10); Type 2 变体 [${variants}]; 产物 ${out}`);
