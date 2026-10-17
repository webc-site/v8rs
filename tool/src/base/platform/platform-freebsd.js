#!/usr/bin/env bun
// tool/src/base/platform/platform-freebsd.js — 覆盖 v8/src/base/platform/platform-freebsd.cc
// FreeBSD 面: sysctl 库枚举 + pthread 栈 (非目标登记)
// → v8-base-platform/src/platform_freebsd.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const ccSrc = readV8("src/base/platform/platform-freebsd.cc");

// 1. 函数面: OS 5 + Stack 1
const osFns = new Set([...ccSrc.matchAll(/OS::(\w+)\(/g)].map((m) => m[1]));
const stackFns = new Set([...ccSrc.matchAll(/Stack::(\w+)\(/g)].map((m) => m[1]));
if (osFns.size !== 5) throw new Error(`OS:: 唯一方法数不符: ${osFns.size}, 期望 5`);
if (stackFns.size !== 1) throw new Error(`Stack:: 唯一方法数不符: ${stackFns.size}, 期望 1`);

// 2. 语义锚: sysctl KERN_PROC_VMMAP / pthread 栈
const anchors = [
  "TimezoneCache* OS::CreateTimezoneCache() {",
  "KERN_PROC_VMMAP",
  "sysctl(",
  "mib[",
  "void OS::SignalCodeMovingGC() {}",
  "void OS::AdjustSchedulingParams() {}",
  "return std::nullopt;",
  "pthread_attr_init",
  "pthread_attr_get_np(pthread_self(), &attr)",
  "pthread_attr_getstack",
];
for (const a of anchors) {
  if (!ccSrc.includes(a)) throw new Error(`platform-freebsd.cc 缺锚点: ${a}`);
}

console.log(`platform-freebsd: OS:: ${osFns.size}, Stack:: ${stackFns.size}`);

let rust = banner("src/base/platform/platform-freebsd.js", "src/base/platform/platform-freebsd.cc", "FreeBSD 平台面 (非目标; sysctl vmmap + pthread 栈)");

rust += `
// —— OS 5 方法登记 ——
/// v8: src/base/platform/platform-freebsd.cc OS::CreateTimezoneCache
/// new PosixDefaultTimezoneCache (转发 posix-time 族)
/// v8: src/base/platform/platform-freebsd.cc OS::GetSharedLibraryAddresses
/// sysctl(KERN_PROC_VMMAP) mib 链取进程映射, 逐条换 SharedLibraryAddress
/// v8: src/base/platform/platform-freebsd.cc OS::SignalCodeMovingGC — 空体
/// v8: src/base/platform/platform-freebsd.cc OS::AdjustSchedulingParams — 空体
/// v8: src/base/platform/platform-freebsd.cc OS::GetFirstFreeMemoryRangeWithin — 恒 nullopt

// —— Stack 1 方法登记 ——
/// v8: src/base/platform/platform-freebsd.cc Stack::ObtainCurrentThreadStackStart
/// pthread_attr_init + pthread_attr_get_np(pthread_self()) + pthread_attr_getstack
/// 取栈基址 (保留下限未覆写 — 沿用基座缺省)

// —— 提取不变式断言 ——
const _: () = {
  // 5 OS 面 + 1 Stack 面; 无本文件特有常量
  const OS_SURFACE: usize = 5;
  const STACK_SURFACE: usize = 1;
  assert!(OS_SURFACE == 5 && STACK_SURFACE == 1);
};
`;

const p = writeSrc("v8-base-platform", "platform_freebsd.rs.skel", rust);
console.log(`written ${p}`);
