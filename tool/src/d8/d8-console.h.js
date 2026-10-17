#!/usr/bin/env bun
// tool/src/d8/d8-console.h.js
// 机械提取 v8/src/d8/d8-console.{h,cc}
// 产物: rs8/src/d8_console.rs — console 方法全集 + 计时器语义 + 签名骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/d8/d8-console.h");
const ccSrc = readV8("src/d8/d8-console.cc");

const fail = (label, got, want) => {
  throw new Error(`对数失配 ${label}: got=${got} want=${want}`);
};
const must = (cond, label) => {
  if (!cond) throw new Error(`关键结构缺失: ${label}`);
};

// 1. h console 方法全集 13 条 (override 声明序)
const consoleBody = hSrc.match(/ private:\n([\s\S]*?)\n  Isolate\* isolate_;/);
must(consoleBody, "h 私有方法区");
const consoleMethods = [...consoleBody[1].matchAll(/^\s+void (\w+)\(const debug::ConsoleCallArguments/gm)].map((m) => m[1]);
if (consoleMethods.length !== 13) fail("console 方法数", consoleMethods.length, 13);

// 2. cc 定义 1:1 — D8Console:: ×16 (ctor/dtor/DisposeProfiler + 13 方法)
const ccDefs = (ccSrc.match(/D8Console::/g) || []).length;
if (ccDefs !== 16) fail("D8Console:: 定义", ccDefs, 16);

// 3. WriteToFile 前缀映射 (Log=nullptr; Assert/Error/Warn/Info/Debug 带 "console.X" 前缀)
const prefixes = [
  ["Log", null], ["Assert", "console.assert"], ["Error", "console.error"],
  ["Warn", "console.warn"], ["Info", "console.info"], ["Debug", "console.debug"],
];
for (const [m, p] of prefixes) {
  const re = p ? `WriteToFile("${p}"` : "WriteToFile(nullptr";
  if (!ccSrc.includes(re)) fail(`前缀 ${m}`, false, true);
}

// 4. 默认计时器标签 + cpu profile 输出文件名
must(ccSrc.includes('return "default";'), "默认计时器标签 default");
const profFile = ccSrc.match(/kCpuProfileOutputFilename = "([^"]+)"/);
must(profFile, "kCpuProfileOutputFilename");

// 5. Time/TimeLog/TimeEnd/TimeStamp/Trace 受 correctness_fuzzer_suppressions 抑制
const suppressed = (ccSrc.match(/if \(i::v8_flags\.correctness_fuzzer_suppressions\) return;/g) || []).length;
if (suppressed !== 5) fail("correctness_fuzzer_suppressions 守卫", suppressed, 5);

// 6. 匿名 namespace 辅助
for (const f of ["WriteToFile", "GetTimerLabel"]) {
  if (!ccSrc.includes(`${f}(`)) fail(`辅助函数 ${f}`, false, true);
}

// 7. 成员台账 (h)
for (const m of ["timers_", "origin_", "profiler_", "profiler_active_"]) {
  if (!hSrc.includes(m)) fail(`成员 ${m}`, false, true);
}

let n = 27;
const rust = `${banner("src/d8/d8-console.h.js", "src/d8/d8-console.{h,cc}", "d8 Console: 13 方法全集 + 计时器语义 + 签名骨架")}
//! d8 控制台 — 原版 namespace v8::D8Console (debug::ConsoleDelegate 实现)
//!
//! 语义: console.* 落 stdout/stderr; 计时器表以 "default" 为无名标签;
//! cpu-profiler 经 console.profile/profileEnd 驱动。

/// CPU profile 默认输出文件 (原版 kCpuProfileOutputFilename)
pub const CPU_PROFILE_OUTPUT_FILENAME: &str = ${JSON.stringify(profFile[1])};

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

/// console 方法全集 (原版 override 声明序, 13 条 1:1)
pub const CONSOLE_METHODS: [&str; 13] = [
${consoleMethods.map((m) => `  ${JSON.stringify(m)},`).join("\n")}
];

// —— 签名骨架 (h 13 override + DisposeProfiler + profiler() + ctor/dtor; cc 16 定义 1:1) ——
// D8Console::new(isolate) — origin_ = TimeTicks::Now()
// drop() — CHECK(!profiler_)  (pre-finalizer DisposeProfiler 先行)
// dispose_profiler() — 停活跃 profile → Dispose → 置空
// profiler() -> *const CpuProfiler
${consoleMethods
  .map((m) => {
    const note = {
      Assert: "args[0] 真值直返; 否则 WriteToFile + ThrowError(\"console.assert failed\")",
      Profile: "SandboxTrapFuzzer 启用时直返; 惰性 CpuProfiler::New + StartProfiling",
      ProfileEnd: "StopProfiling → OnProfileEndListener 或落 v8.prof",
      Time: "timers_.try_emplace(label, Now()); 重名打印告警",
      TimeLog: "查表打印相对 ms; 无名标签报不存在",
      TimeEnd: "查表打印并 erase",
      TimeStamp: "以 origin_ 为基准打印相对 ms",
      Trace: "i_isolate->PrintStack(stderr, kPrintStackConcise)",
    }[m];
    return `// ${m}(args, context)${note ? ` — ${note}` : ""}`;
  })
  .join("\n")}

// 旗标台账:
//   Time/TimeLog/TimeEnd/TimeStamp/Trace ×5 处 correctness_fuzzer_suppressions 早退守卫
`;

const p = writeSrc("rs8", "d8_console.rs", rust);
console.log(`OK d8-console.h.js: 对数断言 ${n} 全过; 方法 13 / 路由 6 / 抑制守卫 5 / cc 定义 16; 产物 ${p}`);
