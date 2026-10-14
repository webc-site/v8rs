#!/usr/bin/env bun
// tool/src/base/platform/platform-starboard.js — 覆盖 v8/src/base/platform/platform-starboard.cc
// Starboard 面: Sb* 直映射 41 方法 + 栈顶探测 (非目标登记)
// → v8-base-platform/src/platform_starboard.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const ccSrc = readV8("src/base/platform/platform-starboard.cc");

// 1. 函数面: OS 41 + Stack 2
const osFns = new Set([...ccSrc.matchAll(/OS::(\w+)\(/g)].map((m) => m[1]));
const stackFns = new Set([...ccSrc.matchAll(/Stack::(\w+)\(/g)].map((m) => m[1]));
if (osFns.size !== 41) throw new Error(`OS:: 唯一方法数不符: ${osFns.size}, 期望 41`);
if (stackFns.size !== 2) throw new Error(`Stack:: 唯一方法数不符: ${stackFns.size}, 期望 2`);

// 2. 常量: 探栈 1 字节 + 毫秒基数
const stackSize = Number(ccSrc.match(/const int kStackSize = (\d+);/)?.[1]);
if (stackSize !== 1) throw new Error(`kStackSize 不符: ${stackSize}`);
const msPerSecond = Number(ccSrc.match(/static const int msPerSecond = (\d+);/)?.[1]);
if (msPerSecond !== 1000) throw new Error(`msPerSecond 不符: ${msPerSecond}`);

// 3. 语义锚: Sb 内存/线程/时钟直映射 + 对齐分配
const anchors = [
  "size_t OS::AllocatePageSize() { return kSbMemoryPageSize; }",
  "size_t OS::CommitPageSize() { return kSbMemoryPageSize; }",
  "void OS::SetRandomMmapSeed(int64_t seed) { SB_NOTIMPLEMENTED(); }",
  "void* OS::GetRandomMmapAddr() { return nullptr; }",
  "void OS::Sleep(TimeDelta interval) { SbThreadSleep(interval.InMicroseconds()); }",
  "void OS::Abort() { SbSystemBreakIntoDebugger(); }",
  "char OS::DirectorySeparator() { return kSbFileSepChar; }",
  "request_size = RoundUp(request_size, OS::AllocatePageSize());",
];
for (const a of anchors) {
  if (!ccSrc.includes(a)) throw new Error(`platform-starboard.cc 缺锚点: ${a}`);
}

console.log(`platform-starboard: OS:: ${osFns.size}, Stack:: ${stackFns.size}, kStackSize ${stackSize}`);

let rust = banner("src/base/platform/platform-starboard.js", "src/base/platform/platform-starboard.cc", "Starboard 平台面 (非目标; Sb* 直映射)");

rust += `
/// 探栈尺寸 (原版 const int kStackSize = 1)
/// v8: src/base/platform/platform-starboard.cc kStackSize
/// 语义: 仅用于取最顶栈帧 — 分配 1 字节即触发栈信息读取
pub const STACK_SIZE: i32 = ${stackSize};
/// 毫秒基数 (原版 static const int msPerSecond)
/// v8: src/base/platform/platform-starboard.cc msPerSecond
pub const MS_PER_SECOND: i32 = ${msPerSecond};

// —— OS 41 方法登记 (Sb* 直映射) ——
/// v8: src/base/platform/platform-starboard.cc 内存面:
///   Allocate (SbMemoryAllocate + RoundUp 页尺寸)/Free (SbMemoryFree)/
///   Release/SetPermissions (SbMemorySetProtection)/RecommitPages/
///   AllocatePageSize = CommitPageSize = kSbMemoryPageSize/
///   SetRandomMmapSeed (SB_NOTIMPLEMENTED)/GetRandomMmapAddr (nullptr)
/// v8: src/base/platform/platform-starboard.cc 线程/进程面:
///   Sleep (SbThreadSleep(μs))/Abort+DebugBreak (SbSystemBreakIntoDebugger)/
///   GetCurrentProcessId/GetCurrentThreadIdInternal (SbThreadGetId)/
///   GetUserTime/Initialize (SbSystemSetLogPath)/AdjustSchedulingParams
/// v8: src/base/platform/platform-starboard.cc IO/打印面:
///   FOpen (SbFileOpen)/Remove/DirectorySeparator (kSbFileSepChar)/
///   isDirectorySeparator/OpenTemporaryFile/Print 族 (SbLog*)/SNPrintF/StrNCpy
/// v8: src/base/platform/platform-starboard.cc 时钟面:
///   TimeCurrentMillis (Time::Now().ToJsTime)/HasLazyCommits/
///   ActivationFrameAlignment (arm/s390x 8, 其余 16)/ArmUsingHardFloat
/// v8: src/base/platform/platform-starboard.cc 其余:
///   CreateTimezoneCache/GetSharedLibraryAddresses (空表)/SignalCodeMovingGC/
///   GetFirstFreeMemoryRangeWithin/GetLastError (SbSystemGetLastError)/
///   DiscardSystemPages

// —— Stack 2 方法登记 ——
/// v8: src/base/platform/platform-starboard.cc Stack::GetStackStart
/// 经 kStackSize=1 的探针分配取栈顶 (SbThreadGetStackInfo 形态)
/// v8: src/base/platform/platform-starboard.cc Stack::GetCurrentStackPosition
/// 取当前栈指针位置

// —— 提取不变式断言 ——
const _: () = {
  assert_eq!(STACK_SIZE, 1);
  assert_eq!(MS_PER_SECOND, 1000);
};
`;

const p = writeSrc("v8-base-platform", "platform_starboard.rs.skel", rust);
console.log(`written ${p}`);
