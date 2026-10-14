// 由 tool/src/d8/d8-platforms.h.js 从 v8/src/d8/d8-platforms.{h,cc} 生成; 修订就地改, 复跑覆盖
// d8 平台包装: 可预测平台 + 延迟任务平台签名骨架

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
// GetPageAllocator
// OnCriticalMemoryPressure
// GetForegroundTaskRunner
// NumberOfWorkerThreads
// PostTaskOnWorkerThreadImpl — 转投哨兵隔离前台 runner
// PostDelayedTaskOnWorkerThreadImpl — 永不执行延迟任务
// IdleTasksEnabled() -> false — 禁 idle
// CreateJobImpl — NewDefaultJobHandle 直建, 不透传 platform_->PostJob
// MonotonicallyIncreasingTime — 1e-5×原子递增 (TSan 防护)
// CurrentClockTimeMillis
// GetTracingController
// GetThreadIsolatedAllocator
// GetZeroSegmentSize — 无 PA 时 0

// —— DelayedTasksPlatform 结构台账 ——
// 同名 override 透传 + GetForegroundTaskRunner 包 DelayedTaskRunner (弱引用表去重);
// 内部四包装: DelayedTaskRunner / DelayedTask / DelayedIdleTask / DelayedJob —
// 各自 Run 前先 Sleep(delay_ms), 再透传内层
// 成员: mutex_ (护 RNG 与 runner 表) / rng_ / delayed_task_runners_

// 平台分支台账:
//   V8_ENABLE_PARTITION_ALLOC ×2 (两平台 GetZeroSegmentSize) — 未启用则 0, 按特性开关登记
//   ~DelayedTasksPlatform DCHECK(runner 表空) — 关停语义承接点
