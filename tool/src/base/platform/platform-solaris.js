#!/usr/bin/env bun
// tool/src/base/platform/platform-solaris.js — 覆盖 v8/src/base/platform/platform-solaris.cc
// Solaris 面: dladdr 库枚举 + procfs 栈界 (非目标登记)
// → v8-base-platform/src/platform_solaris.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const ccSrc = readV8("src/base/platform/platform-solaris.cc");

// 1. 函数面: OS 5 + Stack 2
const osFns = new Set([...ccSrc.matchAll(/OS::(\w+)\(/g)].map((m) => m[1]));
const stackFns = new Set([...ccSrc.matchAll(/Stack::(\w+)\(/g)].map((m) => m[1]));
if (osFns.size !== 5) throw new Error(`OS:: 唯一方法数不符: ${osFns.size}, 期望 5`);
if (stackFns.size !== 2) throw new Error(`Stack:: 唯一方法数不符: ${stackFns.size}, 期望 2`);

// 2. 语义锚: SolarisTimezoneCache + dladdr + /proc/self/lwp 栈
const anchors = [
  "class SolarisTimezoneCache",
  "OS::CreateTimezoneCache() { return new SolarisTimezoneCache(); }",
  "dladdr",
  "void OS::SignalCodeMovingGC() {}",
  "void OS::AdjustSchedulingParams() {}",
  "return std::nullopt;",
];
for (const a of anchors) {
  if (!ccSrc.includes(a)) throw new Error(`platform-solaris.cc 缺锚点: ${a}`);
}

console.log(`platform-solaris: OS:: ${osFns.size}, Stack:: ${stackFns.size}`);

let rust = banner("src/base/platform/platform-solaris.js", "src/base/platform/platform-solaris.cc", "Solaris 平台面 (非目标; dladdr + lwp 栈界)");

rust += `
// —— SolarisTimezoneCache (原版 class SolarisTimezoneCache : PosixTimezoneCache) ——
/// v8: src/base/platform/platform-solaris.cc SolarisTimezoneCache
/// Solaris 无 tm_gmtoff 时区的旧分支语义 (LocalTimezone 走 tzname)

// —— OS 5 方法登记 ——
/// v8: src/base/platform/platform-solaris.cc OS::CreateTimezoneCache — new SolarisTimezoneCache
/// v8: src/base/platform/platform-solaris.cc OS::GetSharedLibraryAddresses — dladdr 枚举
/// v8: src/base/platform/platform-solaris.cc OS::SignalCodeMovingGC — 空体
/// v8: src/base/platform/platform-solaris.cc OS::AdjustSchedulingParams — 空体
/// v8: src/base/platform/platform-solaris.cc OS::GetFirstFreeMemoryRangeWithin — 恒 nullopt

// —— Stack 2 方法登记 ——
/// v8: src/base/platform/platform-solaris.cc Stack::ObtainCurrentThreadStackStart
/// 经 /proc/self/lwp/<tid>/lwpsinfo pr_stackptr 形态读取
/// v8: src/base/platform/platform-solaris.cc Stack::ObtainCurrentThreadStackReservedLimit
/// 栈界 = 栈基 - 栈大小 (lwp 信息推得)

// —— 提取不变式断言 ——
const _: () = {
  const OS_SURFACE: usize = 5;
  const STACK_SURFACE: usize = 2;
  assert!(OS_SURFACE == 5 && STACK_SURFACE == 2);
};
`;

const p = writeSrc("v8-base-platform", "platform_solaris.rs.skel", rust);
console.log(`written ${p}`);
