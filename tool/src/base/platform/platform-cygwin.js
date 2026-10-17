#!/usr/bin/env bun
// tool/src/base/platform/platform-cygwin.js — 覆盖 v8/src/base/platform/platform-cygwin.cc
// Cygwin 面: Win32 API 的 POSIX 壳 (对齐分配三试 + dladdr 库枚举; 非目标登记)
// → v8-base-platform/src/platform_cygwin.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const ccSrc = readV8("src/base/platform/platform-cygwin.cc");

// 1. 函数面: OS 12 (无 Stack::)
const osFns = new Set([...ccSrc.matchAll(/OS::(\w+)\(/g)].map((m) => m[1]));
if (osFns.size !== 12) throw new Error(`OS:: 唯一方法数不符: ${osFns.size}, 期望 12`);
for (const f of ["Allocate", "Free", "Release", "SetPermissions", "RecommitPages", "DiscardSystemPages", "SealPages", "HasLazyCommits", "GetSharedLibraryAddresses", "SignalCodeMovingGC", "AdjustSchedulingParams", "GetFirstFreeMemoryRangeWithin"]) {
  if (!osFns.has(f)) throw new Error(`platform-cygwin.cc 缺 OS::${f}`);
}
if (new Set([...ccSrc.matchAll(/Stack::(\w+)\(/g)].map((m) => m[1])).size !== 0) throw new Error("cygwin 不应含 Stack::");

// 2. 常量: 对齐重试 3 + 库名缓冲
const maxAttempts = Number(ccSrc.match(/kMaxAttempts = (\d+);/)?.[1]);
if (maxAttempts !== 3) throw new Error(`kMaxAttempts 不符: ${maxAttempts}`);
if (!ccSrc.includes("const int kLibNameLen = FILENAME_MAX + 1;")) throw new Error("kLibNameLen 缺失");
if (!ccSrc.includes("size_t padded_size = size + (alignment - page_size);")) throw new Error("对齐填充语义锚缺失");

// 3. 语义锚: Win32 底座 + VirtualFree 裁剪
const anchors = [
  "RandomizedVirtualAlloc(padded_size, flags, protect, hint)",
  "FILE* fp = fopen(\"/proc/self/maps\", \"r\");",
  "if (attr_r == 'r' && attr_w != 'w' && attr_x == 'x') {",
  "GetModuleHandle(L\"Kernel32.dll\")",
  "return VirtualFree(address, size, MEM_DECOMMIT) != 0;",
  "return VirtualAlloc(address, size, MEM_COMMIT, protect) != nullptr;",
];
for (const a of anchors) {
  if (!ccSrc.includes(a)) throw new Error(`platform-cygwin.cc 缺锚点: ${a}`);
}

console.log(`platform-cygwin: OS:: ${osFns.size}, 重试 ${maxAttempts}, Stack:: 0`);

let rust = banner("src/base/platform/platform-cygwin.js", "src/base/platform/platform-cygwin.cc", "Cygwin 平台面 (非目标; Win32 API 的 POSIX 壳)");

rust += `
/// 对齐分配重试上限 (原版 const int kMaxAttempts)
/// v8: src/base/platform/platform-cygwin.cc kMaxAttempts
pub const MAX_ATTEMPTS: i32 = ${maxAttempts};

// —— OS 12 方法登记 ——
/// v8: src/base/platform/platform-cygwin.cc OS::Allocate
/// 填充分配法: padded = size + (alignment - page_size), RandomizedVirtualAlloc
/// 落点后裁剪出头尾 (VirtualFree), 最多 kMaxAttempts 轮
/// v8: src/base/platform/platform-cygwin.cc OS::Free / OS::Release
/// VirtualFree(MEM_DECOMMIT / MEM_RELEASE) 分档
/// v8: src/base/platform/platform-cygwin.cc OS::SetPermissions
/// kNoAccess → VirtualFree(MEM_DECOMMIT); 其余 → VirtualAlloc(MEM_COMMIT, protect)
/// v8: src/base/platform/platform-cygwin.cc OS::RecommitPages / DiscardSystemPages
/// v8: src/base/platform/platform-cygwin.cc OS::SealPages — 恒 false (不支持)
/// v8: src/base/platform/platform-cygwin.cc OS::HasLazyCommits
/// v8: src/base/platform/platform-cygwin.cc OS::GetSharedLibraryAddresses
/// fscanf 逐行解析 /proc/self/maps (r-x 条目 + 路径列),
/// kLibNameLen = FILENAME_MAX + 1 名缓冲; DiscardVirtualMemory 经
/// GetModuleHandle(Kernel32.dll) 动态解析
/// v8: src/base/platform/platform-cygwin.cc OS::SignalCodeMovingGC
/// v8: src/base/platform/platform-cygwin.cc OS::AdjustSchedulingParams — 空体
/// v8: src/base/platform/platform-cygwin.cc OS::GetFirstFreeMemoryRangeWithin

// —— 提取不变式断言 ——
const _: () = {
  assert_eq!(MAX_ATTEMPTS, 3);
};
`;

const p = writeSrc("v8-base-platform", "platform_cygwin.rs.skel", rust);
console.log(`written ${p}`);
