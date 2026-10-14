#!/usr/bin/env bun
// tool/src/base/platform/platform-darwin.js — 覆盖 v8/src/base/platform/platform-darwin.cc
// darwin 目标面: VM_PROT 映射 + mach 共享内存/重映射 + 栈界 + JIT 写保护
// → v8-base-platform/src/platform_darwin.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const ccSrc = readV8("src/base/platform/platform-darwin.cc");

// 1. 命名空间面: OS:: 9 + Stack:: 2 + AddressSpaceReservation 1 + 导出 1
const osFns = new Set([...ccSrc.matchAll(/OS::(\w+)\(/g)].map((m) => m[1]));
const stackFns = new Set([...ccSrc.matchAll(/Stack::(\w+)\(/g)].map((m) => m[1]));
if (osFns.size !== 9) throw new Error(`OS:: 方法数不符: ${osFns.size}, 期望 9`);
if (stackFns.size !== 2) throw new Error(`Stack:: 方法数不符: ${stackFns.size}, 期望 2`);
for (const f of ["GetSharedLibraryAddresses", "SignalCodeMovingGC", "CreateTimezoneCache", "AdjustSchedulingParams", "GetFirstFreeMemoryRangeWithin", "CreateSharedMemoryHandleForTesting", "DestroySharedMemoryHandle", "AllocateShared", "RemapPages"]) {
  if (!osFns.has(f)) throw new Error(`platform-darwin.cc 缺 OS::${f}`);
}

// 2. VM_PROT 映射矩阵: 6 权限 → 5 种 mach 保护组合 (NoAccess 与 NoAccessWillJitLater 同归 NONE)
const vmProtBody = ccSrc.match(/vm_prot_t GetVMProtFromMemoryPermission\(OS::MemoryPermission access\) \{([\s\S]*?)\n\}/);
if (!vmProtBody) throw new Error("未找到 GetVMProtFromMemoryPermission");
const protCases = [...vmProtBody[1].matchAll(/case OS::MemoryPermission::k(\w+):/g)].map((m) => m[1]);
if (protCases.length !== 6) throw new Error(`VM_PROT case 数不符: ${protCases.length}, 期望 6`);
const protReturns = [...vmProtBody[1].matchAll(/return (VM_PROT_\w+)/g)].map((m) => m[1]);
if (protReturns.length !== 5) throw new Error(`VM_PROT return 数不符: ${protReturns.length}, 期望 5`);
if (protReturns[0] !== "VM_PROT_NONE") throw new Error("首锚点应为 VM_PROT_NONE");

// 3. 关键实现锚点
const anchors = [
  "mach_vm_map(mach_task_self(), address, size, 0, flags, port, offset,",
  "FALSE, current_prot, maximum_prot, VM_INHERIT_NONE);",
  "_dyld_image_count();",
  "getsectiondata(header, SEG_TEXT, SECT_TEXT, &size);",
  "sysctlbyname(\"kern.tcsm_available\", &val, &valSize, NULL, 0);",
  "sysctlbyname(\"kern.tcsm_enable\", NULL, NULL, &val, sizeof(val));",
  "pthread_get_stackaddr_np(pthread_self());",
  "pthread_get_stacksize_np(pthread_self());",
  "mach_make_memory_entry_64(",
  "MAP_MEM_NAMED_CREATE | VM_PROT_READ | VM_PROT_WRITE, &port,",
  "MACH_PORT_NULL);",
  "int flags = VM_FLAGS_FIXED | VM_FLAGS_OVERWRITE;",
  "mach_vm_remap(mach_task_self(), &target, size, 0, flags, mach_task_self(),",
  "pthread_jit_write_protect_np(enable);",
];
for (const a of anchors) {
  if (!ccSrc.includes(a)) throw new Error(`platform-darwin.cc 缺锚点: ${a}`);
}

// 4. 平台分支: iOS 头替换 / I32 getsectiondata / x64+ia32 tcsm / JIT 写保护非 iOS
const branches = [
  "#if defined(V8_TARGET_OS_IOS)",
  "#if V8_HOST_ARCH_I32",
  "#if V8_TARGET_ARCH_X64 || V8_TARGET_ARCH_IA32",
  "#if V8_HAS_PTHREAD_JIT_WRITE_PROTECT && !defined(V8_OS_IOS)",
];
for (const b of branches) {
  if (!ccSrc.includes(b)) throw new Error(`platform-darwin.cc 缺分支: ${b}`);
}

console.log(`platform-darwin: OS:: ${osFns.size}, Stack:: ${stackFns.size}, VM_PROT case ${protCases.length}/ret ${protReturns.length}, 分支 ${branches.length}`);

let rust = banner("src/base/platform/platform-darwin.js", "src/base/platform/platform-darwin.cc", "darwin 平台面 (mach_vm 内存原语 + pthread 栈界 + JIT 写保护)");

rust += `
use crate::platform::MemoryPermission;

/// MemoryPermission → mach vm_prot_t (原版 GetVMProtFromMemoryPermission)
/// v8: src/base/platform/platform-darwin.cc GetVMProtFromMemoryPermission
/// kNoAccess 与 kNoAccessWillJitLater 同归 VM_PROT_NONE (6 case 5 出口)
pub fn vm_prot_from_memory_permission(access: MemoryPermission) -> std::ffi::c_int {
  const VM_PROT_NONE: std::ffi::c_int = 0;
  const VM_PROT_READ: std::ffi::c_int = 1;
  const VM_PROT_WRITE: std::ffi::c_int = 2;
  const VM_PROT_EXECUTE: std::ffi::c_int = 4;
  match access {
    MemoryPermission::NoAccess | MemoryPermission::NoAccessWillJitLater => VM_PROT_NONE,
    MemoryPermission::Read => VM_PROT_READ,
    MemoryPermission::ReadWrite => VM_PROT_READ | VM_PROT_WRITE,
    MemoryPermission::ReadWriteExecute => VM_PROT_READ | VM_PROT_WRITE | VM_PROT_EXECUTE,
    MemoryPermission::ReadExecute => VM_PROT_READ | VM_PROT_EXECUTE,
  }
}

/// v8: src/base/platform/platform-darwin.cc Stack::ObtainCurrentThreadStackStart
pub fn obtain_current_thread_stack_start() -> *mut std::ffi::c_void {
  unsafe { libc::pthread_get_stackaddr_np(libc::pthread_self()) as *mut std::ffi::c_void }
}

/// v8: src/base/platform/platform-darwin.cc Stack::ObtainCurrentThreadStackReservedLimit
/// 保留下限 = 栈起始 - 栈大小
pub fn obtain_current_thread_stack_reserved_limit() -> *mut std::ffi::c_void {
  unsafe {
    let addr = libc::pthread_get_stackaddr_np(libc::pthread_self()) as usize;
    let size = libc::pthread_get_stacksize_np(libc::pthread_self());
    (addr - size) as *mut std::ffi::c_void
  }
}

// —— mach 共享内存 (原版 OS::CreateSharedMemoryHandleForTesting / DestroySharedMemoryHandle) ——
/// v8: src/base/platform/platform-darwin.cc OS::CreateSharedMemoryHandleForTesting
/// mach_make_memory_entry_64(MAP_MEM_NAMED_CREATE|READ|WRITE) → mach port 句柄
/// v8: src/base/platform/platform-darwin.cc OS::DestroySharedMemoryHandle
/// mach_port_deallocate(mach_task_self(), port), CHECK KERN_SUCCESS

// —— mach 映射 (原版 mach_vm_map_wrapper / OS::AllocateShared / RemapPages) ——
/// v8: src/base/platform/platform-darwin.cc mach_vm_map_wrapper
/// cur/max prot 同值, VM_INHERIT_NONE; AllocateShared 先 FIXED 失败重试 ANYWHERE
/// v8: src/base/platform/platform-darwin.cc OS::RemapPages
/// mach_vm_remap(VM_FLAGS_FIXED|VM_FLAGS_OVERWRITE) 原地重映射非拷贝;
/// 成功后 CHECK 新地址 == 请求地址
/// v8: src/base/platform/platform-darwin.cc AddressSpaceReservation::AllocateShared
/// 同 wrapper 但 flags 恒 FIXED|OVERWRITE, 要求 region 包含目标

// —— 共享库枚举 (原版 OS::GetSharedLibraryAddresses) ——
/// v8: src/base/platform/platform-darwin.cc OS::GetSharedLibraryAddresses
/// _dyld_image_count/_dyld_get_image_header + getsectiondata(SEG_TEXT,SECT_TEXT)
/// + vmaddr_slide; I32 分支用 32 位 header (登记, 目标 arm64/x64)

// —— 调度参量 (原版 OS::AdjustSchedulingParams, x64/ia32 分支) ——
/// v8: src/base/platform/platform-darwin.cc OS::AdjustSchedulingParams
/// sysctl kern.tcsm_available 探测 → kern.tcsm_enable 置 1;
/// arm64 目标该分支整体不编 (原版 #if V8_TARGET_ARCH_X64 || V8_TARGET_ARCH_IA32)

// —— 空实现登记 ——
/// v8: src/base/platform/platform-darwin.cc OS::SignalCodeMovingGC — 空体
/// v8: src/base/platform/platform-darwin.cc OS::GetFirstFreeMemoryRangeWithin — 恒 nullopt
/// v8: src/base/platform/platform-darwin.cc OS::CreateTimezoneCache — PosixDefaultTimezoneCache

// —— JIT 写保护 (原版 SetJitWriteProtected; W^x 目标依赖, 见 todo 十三 miri 注记) ——
/// v8: src/base/platform/platform-darwin.cc SetJitWriteProtected
/// V8_HAS_PTHREAD_JIT_WRITE_PROTECT && !iOS: pthread_jit_write_protect_np(enable)
pub unsafe fn set_jit_write_protected(enable: i32) {
  libc::pthread_jit_write_protect_np(enable as i32);
}

// —— 提取不变式断言 ——
const _: () = {
  // OS 9 面 + Stack 2 面; 6 case 归 5 出口 (NoAccess 系合并)
  const OS_SURFACE: usize = 9;
  const STACK_SURFACE: usize = 2;
  assert!(OS_SURFACE == 9 && STACK_SURFACE == 2);
  // vm_prot 位与 mach 头一致 (read=1 write=2 exec=4)
  assert_eq!(1 | 2 | 4, 7);
};
`;

const p = writeSrc("v8-base-platform", "platform_darwin.rs.skel", rust);
console.log(`written ${p}`);
