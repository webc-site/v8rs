#!/usr/bin/env bun
// tool/src/base/platform/platform-win32.js — 覆盖 v8/src/base/platform/platform-win32.{h,cc}
// win32 面: 时间换算常量 + PreciseSleepTimer + OS 58 方法登记 (非目标)
// → v8-base-platform/src/platform_win32.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/base/platform/platform-win32.h");
const ccSrc = readV8("src/base/platform/platform-win32.cc");

// 1. PreciseSleepTimer 契约 (Win10 1803+ 高精度睡)
const hAnchors = [
  "class V8_BASE_EXPORT PreciseSleepTimer {",
  "void TryInit();",
  "bool IsInitialized() const;",
  "void Sleep(TimeDelta interval) const;",
  "void Close();",
  "HANDLE timer_;",
];
for (const a of hAnchors) {
  if (!hSrc.includes(a)) throw new Error(`platform-win32.h 缺锚点: ${a}`);
}

// 2. cc 常量: Windows 时区/时间换算五件套
const epoc = ccSrc.match(/kTimeEpoc = (\d+)LL;/)?.[1];
const scaler = ccSrc.match(/kTimeScaler = (\d+);/)?.[1];
const msPerMin = ccSrc.match(/kMsPerMinute = (\d+);/)?.[1];
const hnsPerSec = ccSrc.match(/kHundredNanosecondsPerSecond = (\d+);/)?.[1];
if (!epoc || !scaler || !msPerMin || !hnsPerSec) throw new Error("win32 时间常量缺失");
if (epoc !== "116444736000000000" || scaler !== "10000" || msPerMin !== "60000" || hnsPerSec !== "10000000") {
  throw new Error(`win32 常量值不符: ${epoc}/${scaler}/${msPerMin}/${hnsPerSec}`);
}
// kMaxClockElapsedTime = 60*kHundredNanosecondsPerSecond (1 分钟)
if (!/kMaxClockElapsedTime =\s*\n?\s*60\s*\*\s*kHundredNanosecondsPerSecond;/.test(ccSrc)) throw new Error("kMaxClockElapsedTime 形态不符");
const tzName = Number(ccSrc.match(/kTzNameSize = (\d+);/)?.[1]);
const shortTz = ccSrc.match(/kShortTzNames = (\w+);/)?.[1];
if (tzName !== 128 || shortTz !== "false") throw new Error("kTzNameSize/kShortTzNames 不符");
// kTimeEpoc 与 time.cc 的 kTimeToEpochInMicroseconds 换算一致 (LL 单位 100ns)
if (BigInt(epoc) / 10n !== 11644473600000000n) throw new Error("epoch 十倍率校验失败");

// 3. OS 面: 58 唯一方法 (win32 全量实现面, 登记态)
const osFns = new Set([...ccSrc.matchAll(/OS::(\w+)\(/g)].map((m) => m[1]));
if (osFns.size !== 58) throw new Error(`OS:: 唯一方法数不符: ${osFns.size}, 期望 58`);
for (const f of ["EnsureWin32MemoryAPILoaded", "ConvertUtf8StringToUtf16", "IsHardwareEnforcedShadowStacksEnabled"]) {
  if (!osFns.has(f)) throw new Error(`platform-win32.cc 缺 win32 特有方法 OS::${f}`);
}
// Stack 面 5 (win32 特有 Set/Get 界 + GetCurrentStackPosition)
const stackFns = new Set([...ccSrc.matchAll(/Stack::(\w+)\(/g)].map((m) => m[1]));
if (stackFns.size !== 5) throw new Error(`Stack:: 唯一方法数不符: ${stackFns.size}, 期望 5`);
for (const f of ["SetCurrentThreadStackBounds", "GetCommittedStackLimit", "GetCurrentStackPosition"]) {
  if (!stackFns.has(f)) throw new Error(`platform-win32.cc 缺 Stack::${f}`);
}
// ASR 面: 子预留/占位符 12
const asrFns = new Set([...ccSrc.matchAll(/AddressSpaceReservation::(\w+)\(/g)].map((m) => m[1]));
if (asrFns.size !== 12) throw new Error(`ASR:: 唯一方法数不符: ${asrFns.size}, 期望 12`);

// 4. 语义锚: VirtualAlloc 二段提交 / TEB / 时区注册表
const ccAnchors = [
  "return VirtualAlloc2(GetCurrentProcess(), address, size, flags, protect,",
  ": MEM_RESERVE | MEM_COMMIT;",
  "MEM_RESERVE | MEM_RESERVE_PLACEHOLDER, PAGE_NOACCESS);",
  "GetModuleHandle(L\"kernelbase.dll\"), \"VirtualAlloc2\");",
  "SetCurrentThreadStackBounds",
  "GetLastError()",
];
for (const a of ccAnchors) {
  if (!ccSrc.includes(a)) throw new Error(`platform-win32.cc 缺锚点: ${a}`);
}

console.log(`platform-win32: OS:: ${osFns.size}, Stack:: ${stackFns.size}, ASR:: ${asrFns.size}, 时间常量 5, tzName ${tzName}`);

let rust = banner("src/base/platform/platform-win32.js", "src/base/platform/platform-win32.{h,cc}", "win32 平台面 (非目标; 时间常量换算 + OS 58 方法登记)");

rust += `
// —— Windows 时间换算常量 (原版 WindowsTimezoneCache 私有面) ——
/// Windows epoch (1601) 相对 Unix epoch (1970) 的 100ns 计数
/// v8: src/base/platform/platform-win32.cc kTimeEpoc
pub const TIME_EPOC: i64 = ${epoc};
/// 100ns → μs 缩放 (1 μs = 10 × 100ns)
/// v8: src/base/platform/platform-win32.cc kTimeScaler
pub const TIME_SCALER: i64 = ${scaler};
/// 每分钟毫秒数 (原版 kMsPerMinute)
/// v8: src/base/platform/platform-win32.cc kMsPerMinute
pub const MS_PER_MINUTE: i64 = ${msPerMin};
/// 每秒 100ns 数 (原版 kHundredNanosecondsPerSecond)
/// v8: src/base/platform/platform-win32.cc kHundredNanosecondsPerSecond
pub const HUNDRED_NANOSECONDS_PER_SECOND: i64 = ${hnsPerSec};
/// 时钟重同步窗口 (原版 kMaxClockElapsedTime = 60×上式, 1 分钟)
/// v8: src/base/platform/platform-win32.cc kMaxClockElapsedTime
pub const MAX_CLOCK_ELAPSED_TIME: i64 = 60 * HUNDRED_NANOSECONDS_PER_SECOND;
/// 时区名缓冲 (原版 kTzNameSize)
/// v8: src/base/platform/platform-win32.cc kTzNameSize
pub const TZ_NAME_SIZE: usize = ${tzName};
/// 用长时区名 (原版 kShortTzNames = false)
/// v8: src/base/platform/platform-win32.cc kShortTzNames
pub const SHORT_TZ_NAMES: bool = ${shortTz};

/// 高精度睡 (原版 class PreciseSleepTimer)
/// v8: src/base/platform/platform-win32.h PreciseSleepTimer
/// CreateWaitableTimerExW(WAITABLETIMER_HIGH_RESOLUTION) — Win10 1803+;
/// TryInit 失败静默, IsInitialized 探测; Sleep 需先初始化且单线程使用
pub struct PreciseSleepTimer {
  timer: *mut core::ffi::c_void,
}

// —— OS 58 方法登记 (win32 非目标; 面清单取自 cc 唯一解析, 供映射审计) ——
/// v8: src/base/platform/platform-win32.cc 完整面 (58):
/// 内存: Allocate (VirtualAllocWrapper: 有 VirtualAlloc2 (kernelbase 动态解析) 走
///   带句柄形态, 否则 VirtualAlloc; 常规路径 MEM_RESERVE 或 MEM_RESERVE|MEM_COMMIT,
///   占位符路径 MEM_RESERVE|MEM_RESERVE_PLACEHOLDER + PAGE_NOACCESS)/
///   AllocateShared/Free/FreeShared/Release/SetPermissions (VirtualProtect)/
///   SetDataReadOnly/SetMemoryRegionName/RecommitPages/DiscardSystemPages/
///   DecommitPages/SealPages/CanReserveAddressSpace/CreateAddressSpaceReservation/
///   FreeAddressSpaceReservation/CreateSharedMemoryHandleForTesting/
///   DestroySharedMemoryHandle/HasLazyCommits/GetRandomMmapAddr/SetRandomMmapSeed/
///   AllocatePageSize/CommitPageSize/EnsureWin32MemoryAPILoaded
/// ASR 12 面: CreateSubReservation/FreeSubReservation/SplitPlaceholder/
///   MergePlaceholders/Allocate/Free/AllocateShared/FreeShared/SetPermissions/
///   RecommitPages/DiscardSystemPages/DecommitPages
/// 栈 5 面: ObtainCurrentThreadStackStart/ObtainCurrentThreadStackReservedLimit/
///   SetCurrentThreadStackBounds/GetCommittedStackLimit/GetCurrentStackPosition
/// 线程/进程: GetCurrentThreadId(Internal)/GetCurrentProcessId/ExitProcess/
///   GetUserTime/GetPeakMemoryUsageKb/EnsureAlternativeSignalStackIsAvailableForCurrentThread
/// 时间: TimeCurrentMillis/Sleep (+RolloverProtectedNow 见 time 族)
/// IO/打印: FOpen(UTF16 转换 ConvertUtf8StringToUtf16)/Remove/OpenTemporaryFile/
///   Print 族/SNPrintF/VSNPrintF/StrNCpy/DirectorySeparator('\\\\')/isDirectorySeparator
/// 杂项: Initialize/Abort/DebugBreak/GetLastError/AdjustSchedulingParams/
///   SignalCodeMovingGC/GetFirstFreeMemoryRangeWithin/CreateTimezoneCache(注册表)/
///   IsHardwareEnforcedShadowStacksEnabled

// —— 提取不变式断言 ——
const _: () = {
  assert_eq!(TIME_EPOC, 116444736000000000);
  assert_eq!(TIME_EPOC / 10, 11644473600000000); // 与 time.cc kTimeToEpochInMicroseconds 一致
  assert_eq!(TIME_SCALER, 10000);
  assert_eq!(MAX_CLOCK_ELAPSED_TIME, 600000000);
  assert_eq!(TZ_NAME_SIZE, 128);
  assert!(!SHORT_TZ_NAMES);
};
`;

const p = writeSrc("v8-base-platform", "platform_win32.rs.skel", rust);
console.log(`written ${p}`);
