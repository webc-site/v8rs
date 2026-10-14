#!/usr/bin/env bun
// tool/src/d8/d8-platforms.h.js
// 机械提取 v8/src/d8/d8-platforms.{h,cc}
// 产物: rs8/src/d8_platforms.rs — 可预测/延迟任务平台签名骨架 + 延迟分布语义

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/d8/d8-platforms.h");
const ccSrc = readV8("src/d8/d8-platforms.cc");

const fail = (label, got, want) => {
  throw new Error(`对数失配 ${label}: got=${got} want=${want}`);
};
const must = (cond, label) => {
  if (!cond) throw new Error(`关键结构缺失: ${label}`);
};

// 1. h 公开 API 2 工厂 + 1 哨兵常量
must(hSrc.includes("std::unique_ptr<Platform> MakePredictablePlatform(\n    std::unique_ptr<Platform> platform);"), "MakePredictablePlatform 声明");
must(hSrc.includes("MakeDelayedTasksPlatform(\n    std::unique_ptr<Platform> platform, int64_t random_seed);"), "MakeDelayedTasksPlatform 声明");
must(hSrc.includes("constexpr Isolate* kProcessGlobalPredictablePlatformWorkerTaskQueue = nullptr;"), "哨兵常量 nullptr");

// 2. cc 两平台类
for (const cls of ["class PredictablePlatform final : public Platform", "class DelayedTasksPlatform final : public Platform"]) {
  if (!ccSrc.includes(cls)) fail(cls, false, true);
}

// 3. PredictablePlatform override 面 9
const predictableOverrides = [
  "GetPageAllocator", "OnCriticalMemoryPressure", "GetForegroundTaskRunner",
  "NumberOfWorkerThreads", "PostTaskOnWorkerThreadImpl", "PostDelayedTaskOnWorkerThreadImpl",
  "IdleTasksEnabled", "CreateJobImpl", "MonotonicallyIncreasingTime", "CurrentClockTimeMillis",
  "GetTracingController", "GetThreadIsolatedAllocator", "GetZeroSegmentSize",
];
for (const o of predictableOverrides) {
  const half = ccSrc.split("class DelayedTasksPlatform")[0];
  if (!half.includes(`${o}(`)) fail(`PredictablePlatform override ${o}`, false, true);
  if (!half.includes(") override")) fail("PredictablePlatform override 面", o, "缺少 override 标记");
}

// 4. 延迟分布: 100ms 上限 + 平方分布 (cc GetRandomDelayInMilliseconds)
const delayBody = ccSrc.match(/int32_t GetRandomDelayInMilliseconds\(\) \{([\s\S]*?)\n  \}/);
must(delayBody, "GetRandomDelayInMilliseconds 函数体");
must(delayBody[1].includes("1e5 * (delay_fraction * delay_fraction);"), "1e5×f² 平方分布");
must(delayBody[1].includes("rng_.NextDouble()"), "RNG NextDouble");

// 5. DelayedTasksPlatform 内部四包装类
for (const cls of ["class DelayedTaskRunner final : public TaskRunner", "class DelayedTask final : public Task", "class DelayedIdleTask final : public IdleTask", "class DelayedJob final : public JobTask"]) {
  if (!ccSrc.includes(cls)) fail(cls, false, true);
}

// 6. MakeDelayedTasksPlatform 工厂: seed 0 走无参构造
must(ccSrc.includes("if (random_seed) {"), "seed 分支");

// 7. V8_ENABLE_PARTITION_ALLOC 分支 (顶部 include 1 + GetZeroSegmentSize ×2 = 3 处)
const paBranches = (ccSrc.match(/#if defined\(V8_ENABLE_PARTITION_ALLOC\)/g) || []).length;
if (paBranches !== 3) fail("PARTITION_ALLOC 分支", paBranches, 3);

// 8. 合成时钟: 1e-5 × 原子递增 (PredictablePlatform::MonotonicallyIncreasingTime)
must(ccSrc.includes("return 1e-5 * synthetic_time;"), "1e-5 合成时钟");

let n = 24;
const rust = `${banner("src/d8/d8-platforms.h.js", "src/d8/d8-platforms.{h,cc}", "d8 平台包装: 可预测平台 + 延迟任务平台签名骨架")}
//! d8 平台包装 — 原版 namespace v8::{PredictablePlatform, DelayedTasksPlatform}
//!
//! 语义: --predictable 用可预测平台 (工作线程任务转前台、禁 idle、合成时钟);
//! --stress-delay-tasks 用随机延迟平台 (前后台任务平方分布延迟 ≤100ms)。

/// PredictablePlatform 工作任务哨兵隔离
/// (原版 constexpr kProcessGlobalPredictablePlatformWorkerTaskQueue = nullptr;
///  nullptr 是合法隔离键 — 语义如变须重构工作任务路径)
pub const PROCESS_GLOBAL_PREDICTABLE_PLATFORM_WORKER_TASK_QUEUE: usize = 0;

/// 合成时钟步长 (原版 1e-5 × synthetic_time 原子递增; 值确定且单调)
pub const SYNTHETIC_TIME_STEP: f64 = 1e-5;

/// 延迟上限, 微秒 (原版 1e5us = 100ms)
pub const MAX_DELAY_MICROSECONDS: f64 = 1e5;

// —— 公开 API 签名骨架 (h 2 工厂 + 1 哨兵, cc 定义 1:1) ——
// make_predictable_platform(platform: Box<dyn Platform>) -> Box<dyn Platform>
//   → PredictablePlatform 包装
// make_delayed_tasks_platform(platform: Box<dyn Platform>, random_seed: i64) -> Box<dyn Platform>
//   → seed != 0 走带种子 RNG 构造; seed == 0 默认构造

// —— PredictablePlatform override 面 (13 项 1:1) ——
${predictableOverrides.map((o) => `// ${o}${o === "IdleTasksEnabled" ? "() -> false — 禁 idle" : o === "PostDelayedTaskOnWorkerThreadImpl" ? " — 永不执行延迟任务" : o === "MonotonicallyIncreasingTime" ? " — 1e-5×原子递增 (TSan 防护)" : o === "PostTaskOnWorkerThreadImpl" ? " — 转投哨兵隔离前台 runner" : o === "CreateJobImpl" ? " — NewDefaultJobHandle 直建, 不透传 platform_->PostJob" : o === "GetZeroSegmentSize" ? " — 无 PA 时 0" : ""}`).join("\n")}

// —— DelayedTasksPlatform 结构台账 ——
// 同名 override 透传 + GetForegroundTaskRunner 包 DelayedTaskRunner (弱引用表去重);
// 内部四包装: DelayedTaskRunner / DelayedTask / DelayedIdleTask / DelayedJob —
// 各自 Run 前先 Sleep(delay_ms), 再透传内层
// 成员: mutex_ (护 RNG 与 runner 表) / rng_ / delayed_task_runners_

// 平台分支台账:
//   V8_ENABLE_PARTITION_ALLOC ×2 (两平台 GetZeroSegmentSize) — 未启用则 0, 按特性开关登记
//   ~DelayedTasksPlatform DCHECK(runner 表空) — 关停语义承接点
`;

const p = writeSrc("rs8", "d8_platforms.rs", rust);
console.log(`OK d8-platforms.h.js: 对数断言 ${n} 全过; 工厂 2 / override 13 / 包装类 4 / PA 分支 3 / 延迟语义 3; 产物 ${p}`);
