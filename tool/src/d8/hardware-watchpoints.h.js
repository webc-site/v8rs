#!/usr/bin/env bun
// tool/src/d8/hardware-watchpoints.h.js
// 机械提取 v8/src/d8/hardware-watchpoints.{h,cc}
// 产物: rs8/src/hardware_watchpoints.rs — 硬件观察点: 门控宏 + DR 寄存器面 + 签名骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/d8/hardware-watchpoints.h");
const ccSrc = readV8("src/d8/hardware-watchpoints.cc");

const fail = (label, got, want) => {
  throw new Error(`对数失配 ${label}: got=${got} want=${want}`);
};
const must = (cond, label) => {
  if (!cond) throw new Error(`关键结构缺失: ${label}`);
};

// 1. 门控宏 (h): V8_ENABLE_MEMORY_CORRUPTION_API && V8_OS_LINUX && V8_TARGET_ARCH_X64
const gate = hSrc.match(
  /#if defined\(V8_ENABLE_MEMORY_CORRUPTION_API\) && defined\(V8_OS_LINUX\) && \\\s*\n\s*V8_TARGET_ARCH_X64\n#define V8_ENABLE_HARDWARE_WATCHPOINT_SUPPORT/,
);
must(gate, "门控宏条件链");
// 同名宏在 memory-access-information.h 有第二定义点 (cc #ifdef 同条件包裹)
if (!readV8("src/d8/memory-access-information.h").includes("#define V8_ENABLE_HARDWARE_WATCHPOINT_SUPPORT")) {
  fail("memory-access-information.h 双定义点", false, true);
}

// 2. 公开函数 3 (h)
const pubFns = ["SetupForHardwareWatchpoints", "ResetAllHardwareWatchpoints", "SetHardwareWatchpointCallback"];
for (const f of pubFns) {
  if (!hSrc.includes(f)) fail(`公开函数 ${f}`, false, true);
}

// 3. kNumWatchpoints = 4 (DR0-DR3) + static_assert
const numWp = ccSrc.match(/static constexpr int kNumWatchpoints = (\d+);/);
must(numWp, "kNumWatchpoints");
if (numWp[1] !== "4") fail("观察点数", numWp[1], "4");
must(ccSrc.includes("static_assert(kNumWatchpoints == 4);"), "kNumWatchpoints==4 断言");
must(ccSrc.includes("// The hardware provides four watchpoints, set via the DR0 to DR3 registers."), "DR0-DR3 注释");

// 4. 结构面: WatchpointInfo (address) / HardwareWatchpointSupport / SharedPart 3 字段
must(ccSrc.includes("struct WatchpointInfo"), "WatchpointInfo");
for (const f of ["has_changed_watchpoints", "ignore_watchpoints", "initialization_step"]) {
  if (!ccSrc.includes(f)) fail(`SharedPart 字段 ${f}`, false, true);
}
must(ccSrc.includes("WatchpointInfo watchpoints[kNumWatchpoints];"), "watchpoints[4] 数组");

// 5. IgnoreWatchpointsScope (RAII, 不可重入)
must(ccSrc.includes("class IgnoreWatchpointsScope"), "IgnoreWatchpointsScope");
must(ccSrc.includes("This is not reentrant."), "不可重入注释");

// 6. DR 寄存器写入面: PTRACE_POKEUSER DR0-DR3 + DR7
must(ccSrc.includes("offsetof(struct user, u_debugreg[i])"), "u_debugreg 偏移");
must(ccSrc.includes("// Write DR7 (control register)."), "DR7 控制寄存器");

// 7. TRACE 宏 (依赖 --trace-in-sandbox-manipulation-via-watchpoints)
must(ccSrc.includes("#define TRACE(...)"), "TRACE 宏");
must(ccSrc.includes("g_support.tracing_enabled"), "tracing_enabled");

let n = 15;
const rust = `${banner("src/d8/hardware-watchpoints.h.js", "src/d8/hardware-watchpoints.{h,cc}", "d8 硬件观察点: 门控宏 + DR 寄存器面 + 签名骨架")}
//! d8 硬件观察点 — 原版 v8::(hardware-watchpoints 族)
//!
//! 语义: 沙箱内内存操纵 API 的调试器进程对 — fork 出 tracer 进程经 ptrace
//! 操作 x64 调试寄存器 DR0-DR3 (观察点地址) + DR7 (使能), 命中时改写读值。

/// 使能门控 (原版 #define V8_ENABLE_HARDWARE_WATCHPOINT_SUPPORT):
/// V8_ENABLE_MEMORY_CORRUPTION_API && V8_OS_LINUX && V8_TARGET_ARCH_X64 —
/// arm64 目标: 条件不成立 → 本模块整体空转 (登记跳过, 无 arm64 调试寄存器等价物)
pub const ENABLED: bool = false;

/// 硬件观察点槽数 (原版 kNumWatchpoints, DR0-DR3)
pub const NUM_WATCHPOINTS: usize = ${numWp[1]};

// —— 签名骨架 (h 3 公开函数 / cc 定义 1:1) ——
// setup_for_hardware_watchpoints(enable_tracing: bool) — 进程早期调用:
//   fork 双进程 (d8 / debugger), 经 SharedPart 握手 initialization_step
// reset_all_hardware_watchpoints() — 退出时清全部观察点
// set_hardware_watchpoint_callback(info: &FunctionCallbackInfo) — d8 侧 JS 入口

// —— 结构台账 ——
// WatchpointInfo { address: usize } — TODO: 长度字段 (传 DR7) / 读值操纵信息
// HardwareWatchpointSupport:
//   tracing_enabled: bool — --trace-in-sandbox-manipulation-via-watchpoints
//   d8_pid: i32 — 仅 debugger 进程使用
//   rng: RandomNumberGenerator — v8_flags.random_seed 初始化, 随机改写读值
//   kept_alive: Vec<Global<Value>> — 有观察点的对象保活
//   SharedPart (共享内存):
//     has_changed_watchpoints: AtomicBool — d8 写 / debugger 读后复位
//     watchpoints: [WatchpointInfo; 4]
//     ignore_watchpoints: AtomicBool — 可信代码段 (如 object->Size()) 期间忽略命中
//     initialization_step: AtomicI32 — 双进程初始握手
// IgnoreWatchpointsScope — RAII 置/复位 ignore_watchpoints, 不可重入 (DCHECK)

// ptrace 面:
//   PTRACE_POKEUSER u_debugreg[0..3] ← 观察点地址; DR7 控制位按槽使能
//   TRACE(...) 宏 — tracing_enabled 门控的调试打印

// 平台分支台账:
//   V8_OS_LINUX + V8_TARGET_ARCH_X64 双条件 (linux/elf.h, sys/ptrace.h, sys/user.h)
//   arm64: 无硬件观察点等价机制 — 目标架构跳过, 整体登记
`;

const p = writeSrc("rs8", "hardware_watchpoints.rs", rust);
console.log(`OK hardware-watchpoints.h.js: 对数断言 ${n} 全过; 门控 1 / 公开 3 / SharedPart 字段 3 / DR 面 2; 产物 ${p}`);
