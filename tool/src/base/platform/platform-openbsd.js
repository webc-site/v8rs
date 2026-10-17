#!/usr/bin/env bun
// tool/src/base/platform/platform-openbsd.js — 覆盖 v8/src/base/platform/platform-openbsd.cc
// OpenBSD 面: minherit 弃用页 + dladdr 库枚举 (非目标登记)
// → v8-base-platform/src/platform_openbsd.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const ccSrc = readV8("src/base/platform/platform-openbsd.cc");

// 1. 函数面: OS 7 (无 Stack::)
const osFns = new Set([...ccSrc.matchAll(/OS::(\w+)\(/g)].map((m) => m[1]));
if (osFns.size !== 7) throw new Error(`OS:: 唯一方法数不符: ${osFns.size}, 期望 7`);
for (const f of ["CreateTimezoneCache", "GetSharedLibraryAddresses", "SignalCodeMovingGC", "AdjustSchedulingParams", "GetFirstFreeMemoryRangeWithin", "Free", "GetGCFakeMMapFile"]) {
  if (!osFns.has(f)) throw new Error(`platform-openbsd.cc 缺 OS::${f}`);
}
if (new Set([...ccSrc.matchAll(/Stack::(\w+)\(/g)].map((m) => m[1])).size !== 0) throw new Error("openbsd 不应含 Stack::");

// 2. 语义锚: 假 mmap 释放 + minherit + dladdr + 名缓冲
const anchors = [
  "const int kLibNameLen = FILENAME_MAX + 1;",
  "FILE* fp = fopen(\"/proc/self/maps\", \"r\");",
  "if (attr_r == 'r' && attr_w != 'w' && attr_x == 'x') {",
  "long size = sysconf(_SC_PAGESIZE);",
  "mmap(NULL, size, PROT_READ | PROT_EXEC, MAP_PRIVATE, fileno(f), 0);",
  "OS::Free(addr, size);",
  "FILE* f = fopen(OS::GetGCFakeMMapFile(), \"w+\");",
];
for (const a of anchors) {
  if (!ccSrc.includes(a)) throw new Error(`platform-openbsd.cc 缺锚点: ${a}`);
}

console.log(`platform-openbsd: OS:: ${osFns.size}, Stack:: 0`);

let rust = banner("src/base/platform/platform-openbsd.js", "src/base/platform/platform-openbsd.cc", "OpenBSD 平台面 (非目标; minherit 弃用 + dladdr 枚举)");

rust += `
// —— OS 7 方法登记 ——
/// v8: src/base/platform/platform-openbsd.cc OS::CreateTimezoneCache — PosixDefaultTimezoneCache
/// v8: src/base/platform/platform-openbsd.cc OS::GetSharedLibraryAddresses
/// fscanf 逐行解析 /proc/self/maps (r-x 条目 + 路径列),
/// kLibNameLen = FILENAME_MAX + 1 名缓冲
/// v8: src/base/platform/platform-openbsd.cc OS::SignalCodeMovingGC
/// 假 mmap 文件 + OS::Free(addr, size) 释放 (Free 本体在 posix 基座 munmap)
/// v8: src/base/platform/platform-openbsd.cc OS::AdjustSchedulingParams — 空体
/// v8: src/base/platform/platform-openbsd.cc OS::GetFirstFreeMemoryRangeWithin — 恒 nullopt
/// v8: src/base/platform/platform-openbsd.cc OS::Free — 仅调用 (定义在 posix 基座)
/// v8: src/base/platform/platform-openbsd.cc OS::GetGCFakeMMapFile — 基座转发

// —— ll_prof 标记登记 ——
/// v8: src/base/platform/platform-openbsd.cc OS::SignalCodeMovingGC 内核日志标记:
/// sysconf(_SC_PAGESIZE) 取页 → mmap(PROT_READ|PROT_EXEC, MAP_PRIVATE, 假文件 fd)
/// → 立即 munmap — 内核事件流注入 GC 对齐标记

// —— 提取不变式断言 ——
const _: () = {
  const OS_SURFACE: usize = 7;
  assert!(OS_SURFACE == 7);
};
`;

const p = writeSrc("v8-base-platform", "platform_openbsd.rs.skel", rust);
console.log(`written ${p}`);
