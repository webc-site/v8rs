#!/usr/bin/env bun
// tool/src/base/platform/platform-aix.js — 覆盖 v8/src/base/platform/platform-aix.cc
// AIX 面: /proc/self/maps 枚举 + pthread_getthrds_np 栈界 + mprotect 弃用页 (非目标登记)
// → v8-base-platform/src/platform_aix.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const ccSrc = readV8("src/base/platform/platform-aix.cc");

// 1. 函数面: OS 6 + Stack 2
const osFns = new Set([...ccSrc.matchAll(/OS::(\w+)\(/g)].map((m) => m[1]));
const stackFns = new Set([...ccSrc.matchAll(/Stack::(\w+)\(/g)].map((m) => m[1]));
if (osFns.size !== 6) throw new Error(`OS:: 唯一方法数不符: ${osFns.size}, 期望 6`);
if (stackFns.size !== 2) throw new Error(`Stack:: 唯一方法数不符: ${stackFns.size}, 期望 2`);
for (const f of ["CreateTimezoneCache", "GetSharedLibraryAddresses", "SignalCodeMovingGC", "AdjustSchedulingParams", "GetFirstFreeMemoryRangeWithin", "DecommitPages"]) {
  if (!osFns.has(f)) throw new Error(`platform-aix.cc 缺 OS::${f}`);
}

// 2. 常量与关键语义锚
const mapLen = Number(ccSrc.match(/MAP_LENGTH = (\d+);/)?.[1]);
if (mapLen !== 1024) throw new Error(`MAP_LENGTH 不符: ${mapLen}`);
const anchors = [
  "class AIXTimezoneCache",
  "open(\"/proc/self/maps\", O_RDONLY)",
  "pthread_getthrds_np",
  "__pi_stackaddr",
  "__pi_stacksize",
  "__pi_stackend",
  "if (mprotect(address, size, PROT_NONE) != 0) return false;",
];
for (const a of anchors) {
  if (!ccSrc.includes(a)) throw new Error(`platform-aix.cc 缺锚点: ${a}`);
}

console.log(`platform-aix: OS:: ${osFns.size}, Stack:: ${stackFns.size}, MAP_LENGTH ${mapLen}`);

let rust = banner("src/base/platform/platform-aix.js", "src/base/platform/platform-aix.cc", "AIX 平台面 (非目标; /proc 枚举 + getthrds 栈界 + mprotect 弃用页)");

rust += `
/// maps 行缓冲 (原版 static const int MAP_LENGTH)
/// v8: src/base/platform/platform-aix.cc MAP_LENGTH
pub const MAP_LENGTH: usize = ${mapLen};

// —— AIXTimezoneCache (原版 class AIXTimezoneCache : public PosixTimezoneCache) ——
/// v8: src/base/platform/platform-aix.cc AIXTimezoneCache
/// DaylightSavingsOffset 走 tzset() 重建环境 (AIX 无 tm_gmtoff 语义时的替代)

// —— OS 6 方法登记 ——
/// v8: src/base/platform/platform-aix.cc OS::CreateTimezoneCache — new AIXTimezoneCache
/// v8: src/base/platform/platform-aix.cc OS::GetSharedLibraryAddresses
/// open("/proc/self/maps") 逐行解析 (loadquery 替代路径)
/// v8: src/base/platform/platform-aix.cc OS::SignalCodeMovingGC — 空体
/// v8: src/base/platform/platform-aix.cc OS::AdjustSchedulingParams — 空体
/// v8: src/base/platform/platform-aix.cc OS::GetFirstFreeMemoryRangeWithin — 恒 nullopt
/// v8: src/base/platform/platform-aix.cc OS::DecommitPages
/// AIX 特例: 预定地址 mmap(MAP_FIXED) 默认失败 (SPEC1170 才允许),
/// 弃用页改走 mprotect(PROT_NONE) 而非 posix 基座的 mmap 重盖

// —— Stack 2 方法登记 ——
/// v8: src/base/platform/platform-aix.cc Stack::ObtainCurrentThreadStackStart
/// pthread_getthrds_np 三值: __pi_stackend 为栈基 (高址端)
/// v8: src/base/platform/platform-aix.cc Stack::ObtainCurrentThreadStackReservedLimit
/// __pi_stackaddr - __pi_stacksize 形态 (低址界)

// —— 提取不变式断言 ——
const _: () = {
  assert_eq!(MAP_LENGTH, 1024);
};
`;

const p = writeSrc("v8-base-platform", "platform_aix.rs.skel", rust);
console.log(`written ${p}`);
