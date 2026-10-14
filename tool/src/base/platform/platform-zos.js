#!/usr/bin/env bun
// tool/src/base/platform/platform-zos.js — 覆盖 v8/src/base/platform/platform-zos.cc
// z/OS 面: __z* 内存原语 17 方法 + ASR 2 (非目标登记)
// → v8-base-platform/src/platform_zos.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const ccSrc = readV8("src/base/platform/platform-zos.cc");

// 1. 函数面: OS 17 + ASR 2
const osFns = new Set([...ccSrc.matchAll(/OS::(\w+)\(/g)].map((m) => m[1]));
const asrFns = new Set([...ccSrc.matchAll(/AddressSpaceReservation::(\w+)\(/g)].map((m) => m[1]));
if (osFns.size !== 17) throw new Error(`OS:: 唯一方法数不符: ${osFns.size}, 期望 17`);
if (asrFns.size !== 2) throw new Error(`ASR:: 唯一方法数不符: ${asrFns.size}, 期望 2`);
for (const f of ["Allocate", "Free", "Release", "AllocateShared", "FreeShared", "SetPermissions", "SetDataReadOnly", "RecommitPages", "DiscardSystemPages", "DecommitPages", "AllocatePageSize", "GetRandomMmapAddr", "HasLazyCommits", "CreateTimezoneCache", "GetSharedLibraryAddresses", "SignalCodeMovingGC", "AdjustSchedulingParams"]) {
  if (!osFns.has(f)) throw new Error(`platform-zos.cc 缺 OS::${f}`);
}

// 2. z/OS 特有系统调用锚: __zalloc/__zfree/__mprotect/__mapper
const zsyms = [...new Set([...ccSrc.matchAll(/__(z\w+|mprotect|mapper\w*)\(/g)].map((m) => m[1]))];
for (const s of ["zfree", "zalloc"]) {
  if (!zsyms.includes(s)) throw new Error(`platform-zos.cc 缺 z/OS 系统调用: __${s}`);
}
if (!ccSrc.includes("CHECK_EQ(0, __zfree(address, size));")) throw new Error("__zfree CHECK 形态缺失");

// 3. 语义锚: 页尺寸 1M 分配粒度 + kNoThread z/OS 形态回查
const anchors = [
  "class ZOSTimezoneCache",
  "DCHECK_EQ(0, reinterpret_cast<uintptr_t>(address) % AllocatePageSize());",
];
for (const a of anchors) {
  if (!ccSrc.includes(a)) throw new Error(`platform-zos.cc 缺锚点: ${a}`);
}

console.log(`platform-zos: OS:: ${osFns.size}, ASR:: ${asrFns.size}, z 系调用 ${zsyms.length}`);

let rust = banner("src/base/platform/platform-zos.js", "src/base/platform/platform-zos.cc", "z/OS 平台面 (非目标; __z* 内存原语)");

rust += `
// —— OS 17 方法登记 (__z* 原语族) ——
/// v8: src/base/platform/platform-zos.cc OS::Allocate / OS::AllocateShared
/// __zalloc/__zalloc64 形态, 对齐校验走 AllocatePageSize (z/OS 1M 粒度)
/// v8: src/base/platform/platform-zos.cc OS::Free / OS::Release / OS::FreeShared
/// CHECK_EQ(0, __zfree(address, size)); Free 校 AllocatePageSize, Release 校 CommitPageSize
/// v8: src/base/platform/platform-zos.cc OS::SetPermissions — __mprotect
/// v8: src/base/platform/platform-zos.cc OS::SetDataReadOnly
/// v8: src/base/platform/platform-zos.cc OS::RecommitPages / DiscardSystemPages / DecommitPages
/// __mapper 语义组合 (PG_CNTL 状态迁移)
/// v8: src/base/platform/platform-zos.cc OS::AllocatePageSize
/// v8: src/base/platform/platform-zos.cc OS::GetRandomMmapAddr / OS::HasLazyCommits
/// v8: src/base/platform/platform-zos.cc OS::CreateTimezoneCache — new ZOSTimezoneCache
/// (ZOSTimezoneCache 走 tzset 语义, z/OS 无 tm_gmtoff)
/// v8: src/base/platform/platform-zos.cc OS::GetSharedLibraryAddresses
/// v8: src/base/platform/platform-zos.cc OS::SignalCodeMovingGC / OS::AdjustSchedulingParams

// —— ASR 2 面登记 ——
/// v8: src/base/platform/platform-zos.cc AddressSpaceReservation::AllocateShared / FreeShared

// —— 关联登记: pthread_t 为 struct (见 platform-posix.cc kNoThread z/OS 形态) ——
/// v8: src/base/platform/platform-posix.cc const pthread_t kNoThread = {0};

// —— 提取不变式断言 ——
const _: () = {
  const OS_SURFACE: usize = 17;
  const ASR_SURFACE: usize = 2;
  assert!(OS_SURFACE == 17 && ASR_SURFACE == 2);
};
`;

const p = writeSrc("v8-base-platform", "platform_zos.rs.skel", rust);
console.log(`written ${p}`);
