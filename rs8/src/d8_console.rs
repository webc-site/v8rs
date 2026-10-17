// 由 tool/src/d8/d8-console.h.js 从 v8/src/d8/d8-console.{h,cc} 生成; 修订就地改, 复跑覆盖
// d8 Console: 13 方法全集 + 计时器语义 + 签名骨架

//! d8 控制台 — 原版 namespace v8::D8Console (debug::ConsoleDelegate 实现)
//!
//! 语义: console.* 落 stdout/stderr; 计时器表以 "default" 为无名标签;
//! cpu-profiler 经 console.profile/profileEnd 驱动。

/// CPU profile 默认输出文件 (原版 kCpuProfileOutputFilename)
pub const CPU_PROFILE_OUTPUT_FILENAME: &str = "v8.prof";

/// 无名计时器标签 (原版 GetTimerLabel — 0 参时返回 "default")
pub const DEFAULT_TIMER_LABEL: &str = "default";

/// console 方法 → 输出流/前缀映射 (原版 WriteToFile 调用点, 1:1)
pub const OUTPUT_ROUTES: [(&str, &str, &str); 6] = [
  // (方法, 流, 前缀; 空串 = 无前缀)
  ("Log", "stdout", ""),
  ("Assert", "stdout", "console.assert"),
  ("Error", "stderr", "console.error"),
  ("Warn", "stdout", "console.warn"),
  ("Info", "stdout", "console.info"),
  ("Debug", "stdout", "console.debug"),
];

/// 根据 console 方法名查找输出流与前缀
pub fn get_output_route(method: &str) -> Option<(&'static str, &'static str)> {
  OUTPUT_ROUTES
    .iter()
    .find(|(m, ..)| *m == method)
    .map(|(_, stream, prefix)| (*stream, *prefix))
}

/// console 方法全集 (原版 override 声明序, 13 条 1:1)
pub const CONSOLE_METHODS: [&str; 13] = [
  "Assert",
  "Log",
  "Error",
  "Warn",
  "Info",
  "Debug",
  "Profile",
  "ProfileEnd",
  "Time",
  "TimeLog",
  "TimeEnd",
  "TimeStamp",
  "Trace",
];

// —— 签名骨架 (h 13 override + DisposeProfiler + profiler() + ctor/dtor; cc 16 定义 1:1) ——
// D8Console::new(isolate) — origin_ = TimeTicks::Now()
// drop() — CHECK(!profiler_)  (pre-finalizer DisposeProfiler 先行)
// dispose_profiler() — 停活跃 profile → Dispose → 置空
// profiler() -> *const CpuProfiler
// Assert(args, context) — args[0] 真值直返; 否则 WriteToFile + ThrowError("console.assert failed")
// Log(args, context)
// Error(args, context)
// Warn(args, context)
// Info(args, context)
// Debug(args, context)
// Profile(args, context) — SandboxTrapFuzzer 启用时直返; 惰性 CpuProfiler::New + StartProfiling
// ProfileEnd(args, context) — StopProfiling → OnProfileEndListener 或落 v8.prof
// Time(args, context) — timers_.try_emplace(label, Now()); 重名打印告警
// TimeLog(args, context) — 查表打印相对 ms; 无名标签报不存在
// TimeEnd(args, context) — 查表打印并 erase
// TimeStamp(args, context) — 以 origin_ 为基准打印相对 ms
// Trace(args, context) — i_isolate->PrintStack(stderr, kPrintStackConcise)

// 旗标台账:
//   Time/TimeLog/TimeEnd/TimeStamp/Trace ×5 处 correctness_fuzzer_suppressions 早退守卫
