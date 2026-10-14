#!/usr/bin/env bun
// tool/src/base/platform/platform-qnx.js — 覆盖 v8/src/base/platform/platform-qnx.cc
// QNX 面: /proc pid as + devctl 库枚举 + kNoThread 哨兵 (非目标登记)
// → v8-base-platform/src/platform_qnx.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const ccSrc = readV8("src/base/platform/platform-qnx.cc");

// 1. 函数面: OS 6
const osFns = new Set([...ccSrc.matchAll(/OS::(\w+)\(/g)].map((m) => m[1]));
if (osFns.size !== 6) throw new Error(`OS:: 唯一方法数不符: ${osFns.size}, 期望 6`);
for (const f of ["ArmUsingHardFloat", "CreateTimezoneCache", "GetSharedLibraryAddresses", "SignalCodeMovingGC", "AdjustSchedulingParams", "GetFirstFreeMemoryRangeWithin"]) {
  if (!osFns.has(f)) throw new Error(`platform-qnx.cc 缺 OS::${f}`);
}

// 2. 语义锚: kNoThread 哨兵 + /proc/<pid>/as + procfs_debuginfo
const anchors = [
  "static const pthread_t kNoThread = (pthread_t) 0;",
  "// 0 is never a valid thread id on Qnx since tids and pids share a",
  "snprintf(buf, PATH_MAX + 1, \"/proc/%d/as\", getpid());",
  "procfs_debuginfo info;",
];
for (const a of anchors) {
  if (!ccSrc.includes(a)) throw new Error(`platform-qnx.cc 缺锚点: ${a}`);
}

console.log(`platform-qnx: OS:: ${osFns.size}, 哨兵 1`);

let rust = banner("src/base/platform/platform-qnx.js", "src/base/platform/platform-qnx.cc", "QNX 平台面 (非目标; /proc/<pid>/as + procfs 枚举)");

rust += `
/// 无线程哨兵 (原版 static const pthread_t kNoThread = (pthread_t) 0)
/// v8: src/base/platform/platform-qnx.cc kNoThread
/// 语义: QNX 的 tid 与 pid 共享名字空间, pid 0 保留 (man 2 kill),
/// 故 0 永不是合法线程号 (本文件内仅作哨兵登记, 不直接比对)
pub const NO_THREAD: libc::pthread_t = 0;

// —— OS 6 方法登记 ——
/// v8: src/base/platform/platform-qnx.cc OS::ArmUsingHardFloat
/// v8: src/base/platform/platform-qnx.cc OS::CreateTimezoneCache — PosixDefaultTimezoneCache
/// v8: src/base/platform/platform-qnx.cc OS::GetSharedLibraryAddresses
/// open("/proc/<pid>/as") + devctl(DCMD_PROC_MAPDEBUG) 逐条 procfs_debuginfo
/// v8: src/base/platform/platform-qnx.cc OS::SignalCodeMovingGC — 空体
/// v8: src/base/platform/platform-qnx.cc OS::AdjustSchedulingParams — 空体
/// v8: src/base/platform/platform-qnx.cc OS::GetFirstFreeMemoryRangeWithin — 恒 nullopt

// —— 提取不变式断言 ——
const _: () = {
  assert_eq!(NO_THREAD, 0);
};
`;

const p = writeSrc("v8-base-platform", "platform_qnx.rs.skel", rust);
console.log(`written ${p}`);
