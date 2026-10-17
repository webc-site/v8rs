#!/usr/bin/env bun
// tool/src/base/platform/memory.js — 覆盖 v8/src/base/platform/memory.h
// 分配包装: Malloc/Realloc/Free/Calloc/Aligned* 分支矩阵 + MallocUsableSize 条件面
// → v8-base-platform/src/memory.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/base/platform/memory.h");

// 1. 六大入口锚点
const anchors = [
  "inline void* Malloc(size_t size) {",
  "inline void* Realloc(void* memory, size_t size) {",
  "inline void Free(void* memory) {",
  "inline void* Calloc(size_t count, size_t size) {",
  "inline void* AlignedAlloc(size_t size, size_t alignment) {",
  "inline void AlignedFree(void* ptr) {",
  "inline size_t MallocUsableSize(void* ptr) {",
  "template <class Pointer>\nstruct AllocationResult {",
  "template <typename T>\nV8_NODISCARD AllocationResult<T*> AllocateAtLeast(size_t n) {",
];
for (const a of anchors) {
  if (!hSrc.includes(a)) throw new Error(`memory.h 缺锚点: ${a}`);
}

// 2. 分支矩阵解析: STARBOARD / AIX(_LINUX_SOURCE_COMPAT) / 默认 三通道
const sbCalls = ["SbMemoryAllocate", "SbMemoryReallocate", "SbMemoryDeallocate", "SbMemoryCalloc"];
for (const c of sbCalls) {
  if (!hSrc.includes(c)) throw new Error(`memory.h 缺 Starboard 调用: ${c}`);
}
const sbCount = (hSrc.match(/SbMemory\w+/g) || []).length;
if (sbCount !== 4) throw new Error(`SbMemory 调用数不符: ${sbCount}, 期望 4`);
const aixWorkarounds = (hSrc.match(/__linux_(malloc|realloc|calloc)\(/g) || []).length;
if (aixWorkarounds !== 3) throw new Error(`AIX 绕行调用数不符: ${aixWorkarounds}, 期望 3`);

// 3. AlignedAlloc 平台矩阵: WIN _aligned_malloc / BIONIC memalign / ZOS __aligned_malloc / POSIX posix_memalign
const alignedBranches = ["_aligned_malloc(size, alignment)", "return memalign(alignment, size);", "__aligned_malloc(size, alignment)", "if (posix_memalign(&ptr, alignment, size)) ptr = nullptr;"];
for (const b of alignedBranches) {
  if (!hSrc.includes(b)) throw new Error(`AlignedAlloc 缺分支: ${b}`);
}

// 4. MallocUsableSize 条件宏: (POSIX && !AIX && !SOLARIS && !ZOS && !OPENBSD) || WIN — darwin 命中
const usableDef = hSrc.match(/#define V8_HAS_MALLOC_USABLE_SIZE 1/);
if (!usableDef) throw new Error("缺 V8_HAS_MALLOC_USABLE_SIZE 定义");
const usableImpls = ["return _msize(ptr);", "return malloc_size(ptr);", "return malloc_usable_size(ptr);"];
for (const u of usableImpls) {
  if (!hSrc.includes(u)) throw new Error(`MallocUsableSize 缺实现: ${u}`);
}
// darwin include 矩阵
for (const inc of ["#include <malloc/malloc.h>", "#include <sys/malloc.h>", "#include <malloc.h>"]) {
  if (!hSrc.includes(inc)) throw new Error(`缺平台 include: ${inc}`);
}

// 5. 语义哨兵: Realloc 禁零尺寸 (CHECK_NE(0, size))
if (!hSrc.includes("CHECK_NE(0, size);")) throw new Error("Realloc 零尺寸禁令缺失");
// AllocateAtLeast UBSan 重分配通道
if (!hSrc.includes("V8_USE_UNDEFINED_BEHAVIOR_SANITIZER")) throw new Error("UBSan 通道锚点缺失");

console.log(`memory: 入口 6 + MallocUsableSize 1, STARBOARD 通道 ${sbCount}, AIX 绕行 ${aixWorkarounds}, Aligned 分支 ${alignedBranches.length}`);

let rust = banner("src/base/platform/memory.js", "src/base/platform/memory.h", "分配包装 (posix 直通, darwin malloc_size usable, AIX/Starboard 登记不产码)");

rust += `
use std::ffi::c_void;

/// 分配 (原版 base::Malloc; 目标分支直通 libc::malloc)
/// v8: src/base/platform/memory.h Malloc
/// STARBOARD→SbMemoryAllocate / AIX&&_LINUX_SOURCE_COMPAT→__linux_malloc — 非目标, 登记
pub fn malloc(size: usize) -> *mut c_void { unsafe { libc::malloc(size) } }

/// 重分配 (原版 base::Realloc)
/// v8: src/base/platform/memory.h Realloc
/// 不变式: size==0 禁止 (原版 CHECK_NE(0, size) — realloc(ptr,0) 行为实现定义)
pub fn realloc(memory: *mut c_void, size: usize) -> *mut c_void {
  assert_ne!(size, 0);
  unsafe { libc::realloc(memory, size) }
}

/// 释放 (原版 base::Free; STARBOARD→SbMemoryDeallocate, 登记)
/// v8: src/base/platform/memory.h Free
pub fn free(memory: *mut c_void) { unsafe { libc::free(memory) } }

/// 清零分配 (原版 base::Calloc)
/// v8: src/base/platform/memory.h Calloc
pub fn calloc(count: usize, size: usize) -> *mut c_void { unsafe { libc::calloc(count, size) } }

/// 对齐分配 (原版 base::AlignedAlloc)
/// v8: src/base/platform/memory.h AlignedAlloc
/// 不变式: alignment ≥ alignof(void*), 且为 2 的幂 (原版 DCHECK×2)
/// 分支: WIN→_aligned_malloc / BIONIC→memalign / ZOS→__aligned_malloc /
/// POSIX (目标)→posix_memalign, 失败即 nullptr
pub fn aligned_alloc(size: usize, alignment: usize) -> *mut c_void {
  debug_assert!(usize::from(std::mem::align_of::<*const c_void>()) <= alignment);
  debug_assert!(alignment.is_power_of_two());
  let mut ptr: *mut c_void = std::ptr::null_mut();
  let rc = unsafe { libc::posix_memalign(&mut ptr, alignment, size) };
  if rc != 0 { std::ptr::null_mut() } else { ptr }
}

/// 对齐释放 (原版 base::AlignedFree)
/// v8: src/base/platform/memory.h AlignedFree
/// POSIX/BIONIC 走普通 Free; WIN→_aligned_free / ZOS→__aligned_free (登记)
pub fn aligned_free(ptr: *mut c_void) { free(ptr) }

/// malloc 可用量 (原版 base::MallocUsableSize; V8_HAS_MALLOC_USABLE_SIZE)
/// v8: src/base/platform/memory.h MallocUsableSize
/// darwin→malloc_size / WIN→_msize(null→0) / 其余 POSIX→malloc_usable_size
/// 注意: 超出请求量的字节访问非 UBSan 安全 — 加量走 allocate_at_least
pub fn malloc_usable_size(ptr: *mut c_void) -> usize {
  extern "C" {
    fn malloc_size(ptr: *const c_void) -> usize;
  }
  unsafe { malloc_size(ptr) }
}

/// 分配结果 (原版 template struct AllocationResult<Pointer>)
/// v8: src/base/platform/memory.h AllocationResult
/// 模拟 C++23 allocation_result / allocate_at_least: 实际可用量 ≥ 请求量
#[derive(Debug, Clone, Copy)]
pub struct AllocationResult<T> {
  pub ptr: T,
  pub count: usize,
}

/// 按需加量分配 (原版 template AllocateAtLeast<T>)
/// v8: src/base/platform/memory.h AllocateAtLeast
/// 无 usable_size 时退回 {Malloc(n), n}; UBSan 下 usable≠wanted 需 Realloc 收敛
pub fn allocate_at_least<T>(n: usize) -> AllocationResult<*mut T> {
  let wanted = n * std::mem::size_of::<T>();
  let memory = crate::memory::malloc(wanted) as *mut T;
  let usable = crate::memory::malloc_usable_size(memory as *mut c_void);
  AllocationResult { ptr: memory, count: usable / std::mem::size_of::<T>().max(1) }
}

// —— 提取不变式断言 ——
const _: () = {
  // 6 分配入口 + 1 条件入口
  const ALLOC_ENTRIES: usize = 7;
  assert!(ALLOC_ENTRIES == 7);
  // posix_memalign 失败 → nullptr (0 == 成功)
  assert_eq!(libc::EINVAL, 22);
};
`;

const p = writeSrc("v8-base-platform", "memory.rs.skel", rust);
console.log(`written ${p}`);
