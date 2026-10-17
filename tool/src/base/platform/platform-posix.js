#!/usr/bin/env bun
// tool/src/base/platform/platform-posix.js — 覆盖 v8/src/base/platform/platform-posix.{h,cc}
// POSIX 基座面: OS 53 方法清单 + 内存权限标志矩阵 + 时区缓存基类
// → v8-base-platform/src/platform_posix.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/base/platform/platform-posix.h");
const ccSrc = readV8("src/base/platform/platform-posix.cc");

// 1. 头锚点: PosixInitializeCommon / PosixTimezoneCache / GetProtectionFromMemoryPermission
const hAnchors = [
  "void PosixInitializeCommon(const char* const gc_fake_mmap);",
  "class PosixTimezoneCache : public TimezoneCache {",
  "double DaylightSavingsOffset(double time_ms) override;",
  "static const int msPerSecond = 1000;",
  "int GetProtectionFromMemoryPermission(OS::MemoryPermission access);",
];
for (const a of hAnchors) {
  if (!hSrc.includes(a)) throw new Error(`platform-posix.h 缺锚点: ${a}`);
}

// 2. cc 常量: kAppleArmPageSize = 1<<14, kMmapFdOffset = 0, kNoThread
const armPage = ccSrc.match(/kAppleArmPageSize = (\d+) << (\d+);/);
if (!armPage || Number(armPage[1] << Number(armPage[2])) !== 16384) throw new Error("kAppleArmPageSize 不符");
if (!ccSrc.includes("const int kMmapFdOffset = 0;")) throw new Error("kMmapFdOffset 缺失");
const noThreadForms = (ccSrc.match(/const pthread_t kNoThread/g) || []).length;
if (noThreadForms !== 2) throw new Error(`kNoThread 声明数不符: ${noThreadForms}, 期望 2 (z/OS struct 形态 + 整数形态)`);

// 3. OS 方法面: 唯一名计数 53
const osFns = new Set([...ccSrc.matchAll(/OS::(\w+)\(/g)].map((m) => m[1]));
if (osFns.size !== 53) throw new Error(`OS:: 唯一方法数不符: ${osFns.size}, 期望 53`);
// 分区锚: 内存面 / 打印面 / 线程面 / 文件面
for (const f of ["Allocate", "Free", "SetPermissions", "DecommitPages", "SealPages", "Print", "VPrint", "SNPrintF", "GetCurrentThreadId", "FOpen", "OpenTemporaryFile", "Sleep", "Abort", "DebugBreak", "GetUserTime"]) {
  if (!osFns.has(f)) throw new Error(`platform-posix.cc 缺 OS::${f}`);
}

// 4. GetFlagsForMemoryPermission 矩阵 (mmap flags) 与 GetProtectionFromMemoryPermission (mprotect prot)
const flagsBody = ccSrc.match(/int GetFlagsForMemoryPermission\([\s\S]*?\) \{([\s\S]*?)\n\}/);
if (!flagsBody) throw new Error("未找到 GetFlagsForMemoryPermission");
// if 链语义: ANONYMOUS/FIXED/SHARED|PRIVATE/NORESERVE(AIX|FREEBSD|QNX 除外)/MAP_JIT(darwin)
for (const a of ["flags |= MAP_ANONYMOUS;", "flags |= MAP_FIXED;", "MAP_SHARED : MAP_PRIVATE", "flags |= MAP_NORESERVE;", "flags |= MAP_JIT;"]) {
  if (!flagsBody[1].includes(a)) throw new Error(`GetFlagsForMemoryPermission 缺标志: ${a}`);
}
const jitGate = flagsBody[1].match(/#if V8_OS_DARWIN\n([\s\S]*?)#endif/);
if (!jitGate || !jitGate[1].includes("kNoAccessWillJitLater")) throw new Error("MAP_JIT darwin 门缺失");
const protMap = [...ccSrc.match(/GetProtectionFromMemoryPermission\([\s\S]{0,200}?PROT_(\w+)/g) || []];
if (!ccSrc.includes("case OS::MemoryPermission::kNoAccessWillJitLater")) throw new Error("WillJitLater 分支缺失");
for (const p of ["PROT_NONE", "PROT_READ", "PROT_WRITE", "PROT_EXEC"]) {
  if (!ccSrc.includes(p)) throw new Error(`缺 ${p}`);
}

// 5. 语义锚点: Apple arm 大页 / 半私有 anonymous / 时区缓存 / mmap 随机地址
const ccAnchors = [
  "return kAppleArmPageSize;",
  "MAP_ANONYMOUS | MAP_PRIVATE",
  "void OS::Initialize(const char* const gc_fake_mmap) {",
  "PosixInitializeCommon(gc_fake_mmap);",
  "double PosixTimezoneCache::DaylightSavingsOffset(double time) {",
  "t->tm_isdst > 0 ? 3600 * msPerSecond : 0",
  "constexpr int KB = 1024;",
  "void OS::ExitProcess(int exit_code) {",
];
for (const a of ccAnchors) {
  if (!ccSrc.includes(a)) throw new Error(`platform-posix.cc 缺锚点: ${a}`);
}

console.log(`platform-posix: OS:: 唯一面 ${osFns.size}, kNoThread 形态 ${noThreadForms}, armPage 16384, 头锚 ${hAnchors.length}`);

const osList = [...osFns].sort().join(", ");

let rust = banner("src/base/platform/platform-posix.js", "src/base/platform/platform-posix.{h,cc}", "POSIX 基座 53 OS 方法 (darwin 目标的实现底座)");

rust += `
use crate::platform::MemoryPermission;

/// Apple arm64 分配页 (原版 constexpr int kAppleArmPageSize = 1 << 14)
/// v8: src/base/platform/platform-posix.cc kAppleArmPageSize
pub const APPLE_ARM_PAGE_SIZE: usize = ${Number(armPage[1]) << Number(armPage[2])};
/// mmap fd 偏移常量 (原版 const int kMmapFdOffset = 0)
/// v8: src/base/platform/platform-posix.cc kMmapFdOffset
pub const MMAP_FD_OFFSET: i32 = 0;

/// MemoryPermission → mmap/mprotect 保护位 (原版 GetProtectionFromMemoryPermission)
/// v8: src/base/platform/platform-posix.cc GetProtectionFromMemoryPermission
/// kNoAccess→PROT_NONE(0) / kRead→PROT_READ / kReadWrite→+PROT_WRITE /
/// kReadWriteExecute→+PROT_EXEC / kReadExecute→READ|EXEC;
/// kNoAccessWillJitLater 原版并入 kNoAccess 处理 (w^x 过渡, 见 platform.h 族)
pub fn protection_from_memory_permission(access: MemoryPermission) -> i32 {
  const PROT_NONE: i32 = 0;
  const PROT_READ: i32 = 1;
  const PROT_WRITE: i32 = 2;
  const PROT_EXEC: i32 = 4;
  match access {
    MemoryPermission::NoAccess | MemoryPermission::NoAccessWillJitLater => PROT_NONE,
    MemoryPermission::Read => PROT_READ,
    MemoryPermission::ReadWrite => PROT_READ | PROT_WRITE,
    MemoryPermission::ReadWriteExecute => PROT_READ | PROT_WRITE | PROT_EXEC,
    MemoryPermission::ReadExecute => PROT_READ | PROT_EXEC,
  }
}

/// mmap 标志组装 (原版 GetFlagsForMemoryPermission)
/// v8: src/base/platform/platform-posix.cc GetFlagsForMemoryPermission
/// if 链: 无句柄→MAP_ANONYMOUS; fixed→MAP_FIXED;
/// page_type==kShared→MAP_SHARED 否则 MAP_PRIVATE;
/// NoAccess/NoAccessWillJitLater→MAP_NORESERVE (AIX/FREEBSD/QNX 除外, QNX 用 MAP_LAZY);
/// darwin: WillJitLater 或 ReadWriteExecute→MAP_JIT (Apple 硅硬化运行时 JIT 必需)
pub fn flags_for_memory_permission(access: MemoryPermission, shared: bool, fixed: bool, anonymous: bool) -> i32 {
  const MAP_PRIVATE: i32 = 0x0002;
  const MAP_FIXED: i32 = 0x0010;
  const MAP_ANONYMOUS: i32 = 0x1000;
  const MAP_NORESERVE: i32 = 0x0040;
  const MAP_SHARED: i32 = 0x0001;
  const MAP_JIT: i32 = 0x0800;
  let flags: i32 = 0;
  if (anonymous) { flags |= MAP_ANONYMOUS; }
  if (fixed) { flags |= MAP_FIXED; }
  flags |= if shared { MAP_SHARED } else { MAP_PRIVATE };
  if matches!(access, MemoryPermission::NoAccess | MemoryPermission::NoAccessWillJitLater) {
    flags |= MAP_NORESERVE;
  }
  if matches!(access, MemoryPermission::NoAccessWillJitLater | MemoryPermission::ReadWriteExecute) {
    flags |= MAP_JIT; // darwin 目标专用
  }
  flags
}

/// POSIX 公共初始化 (原版 PosixInitializeCommon)
/// v8: src/base/platform/platform-posix.cc PosixInitializeCommon
/// 记 g_gc_fake_mmap 供 GC 假 mmap 文件名
pub fn posix_initialize_common(gc_fake_mmap: *const std::ffi::c_char) {
  // 原版: g_gc_fake_mmap = gc_fake_mmap (线程安全由调用方保证)
}

/// 时区缓存基类 (原版 class PosixTimezoneCache : TimezoneCache)
/// v8: src/base/platform/platform-posix.h PosixTimezoneCache
/// msPerSecond = 1000; DaylightSavingsOffset 由 localtime_r tm_isdst 推导
pub const MS_PER_SECOND: i32 = 1000;

// —— OS 53 方法面登记 (供 Developer 逐项补语义; 名单取自 cc 唯一解析) ——
/// v8: src/base/platform/platform-posix.cc 完整面 (53):
/// ${osList}
///
/// 分区:
///  内存: Allocate/AllocateShared/Free/FreeShared/Release/SetPermissions/
///        SetDataReadOnly/RecommitPages/DiscardSystemPages/DecommitPages/
///        SealPages/CreateAddressSpaceReservation/FreeAddressSpaceReservation/
///        CanReserveAddressSpace/SetMemoryRegionName/CreateSharedMemoryHandleForTesting/
///        DestroySharedMemoryHandle/HasLazyCommits/GetRandomMmapAddr/SetRandomMmapSeed
///  页尺寸: AllocatePageSize (darwin 走 kAppleArmPageSize)/CommitPageSize/
///        ActivationFrameAlignment
///  线程: GetCurrentThreadId(Internal)/GetCurrentProcessId/ExitProcess/
///        GetUserTime/GetPeakMemoryUsageKb/EnsureAlternativeSignalStackIsAvailableForCurrentThread
///  时序: Sleep/TimeCurrentMillis + PosixTimezoneCache::DaylightSavingsOffset
///  IO/打印: FOpen/Remove/OpenTemporaryFile/LogFileOpenMode("w+")/Print/Error 系
///        (Print/PrintError/VPrint/VPrintError/VFPrint/FPrint/SNPrintF/VSNPrintF)
///  杂项: Initialize/Abort/DebugBreak/GetLastError/GetGCFakeMMapFile/
///        IsHardwareEnforcedShadowStacksEnabled(恒 false)/ArmUsingHardFloat/
///        DirectorySeparator('/')/isDirectorySeparator/StrNCpy/MemoryMappedFile

// —— 提取不变式断言 ——
const _: () = {
  assert_eq!(APPLE_ARM_PAGE_SIZE, 16384);
  assert_eq!(MMAP_FD_OFFSET, 0);
  assert_eq!(MS_PER_SECOND, 1000);
  // PROT 位与 sys/mman.h 一致
  assert_eq!(protection_from_memory_permission(MemoryPermission::NoAccess), 0);
  assert_eq!(protection_from_memory_permission(MemoryPermission::ReadWrite), 3);
  assert_eq!(protection_from_memory_permission(MemoryPermission::ReadWriteExecute), 7);
};
`;

const p = writeSrc("v8-base-platform", "platform_posix.rs.skel", rust);
console.log(`written ${p}`);
