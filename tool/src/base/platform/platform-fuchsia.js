#!/usr/bin/env bun
// tool/src/base/platform/platform-fuchsia.js — 覆盖 v8/src/base/platform/platform-fuchsia.cc
// Fuchsia 面: Zircon VMO 全量内存面 + ASR 子预留 10 (非目标登记)
// → v8-base-platform/src/platform_fuchsia.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const ccSrc = readV8("src/base/platform/platform-fuchsia.cc");

// 1. 函数面: OS 27 + ASR 10
const osFns = new Set([...ccSrc.matchAll(/OS::(\w+)\(/g)].map((m) => m[1]));
const asrFns = new Set([...ccSrc.matchAll(/AddressSpaceReservation::(\w+)\(/g)].map((m) => m[1]));
if (osFns.size !== 27) throw new Error(`OS:: 唯一方法数不符: ${osFns.size}, 期望 27`);
if (asrFns.size !== 10) throw new Error(`ASR:: 唯一方法数不符: ${asrFns.size}, 期望 10`);
for (const f of ["Initialize", "Allocate", "Free", "AllocateShared", "FreeShared", "Release", "SetPermissions", "SealPages", "CanReserveAddressSpace", "CreateAddressSpaceReservation", "GetUserTime", "AdjustSchedulingParams"]) {
  if (!osFns.has(f)) throw new Error(`platform-fuchsia.cc 缺 OS::${f}`);
}

// 2. 语义锚: VMO 创建命名 + root VMAR + vmex 资源
const anchors = [
  "static const char kVirtualMemoryName[] = \"v8-virtualmem\";",
  "vmo.set_property(ZX_PROP_NAME, kVirtualMemoryName,",
  "zx_vmo_replace_as_executable",
  "PosixInitializeCommon(gc_fake_mmap);",
  "zx::vmar::root_self()->get_info(",
  "ZX_INFO_VMAR",
  "g_root_vmar_base",
  "SetVmexResource();",
  "PlacementMode::kUseHint",
  "PlacementMode::kAnywhere",
];
for (const a of anchors) {
  if (!ccSrc.includes(a)) throw new Error(`platform-fuchsia.cc 缺锚点: ${a}`);
}

// 3. 空实现/哨兵语义
if (!ccSrc.includes("bool OS::SealPages(void* address, size_t size) { return false; }")) throw new Error("SealPages 恒 false 锚缺失");
if (!ccSrc.includes("bool OS::HasLazyCommits() { return true; }")) throw new Error("HasLazyCommits 恒 true 锚缺失");

console.log(`platform-fuchsia: OS:: ${osFns.size}, ASR:: ${asrFns.size}, VMO 命名 1`);

let rust = banner("src/base/platform/platform-fuchsia.js", "src/base/platform/platform-fuchsia.cc", "Fuchsia 平台面 (非目标; Zircon VMO/VMAR 内存面)");

rust += `
/// VMO 命名 (原版 static const char kVirtualMemoryName[])
/// v8: src/base/platform/platform-fuchsia.cc kVirtualMemoryName
pub const VIRTUAL_MEMORY_NAME: &str = "v8-virtualmem";

// —— 内存模型登记: 一切映射皆 VMO ——
/// v8: src/base/platform/platform-fuchsia.cc OS::Initialize
/// 取 root VMAR 基址 (ZX_INFO_VMAR) 存 g_root_vmar_base; SetVmexResource 取执行权句柄
/// v8: src/base/platform/platform-fuchsia.cc CreateAndMapVmo
/// zx::vmo::create → set_property(ZX_PROP_NAME) →
/// 需执行时 zx_vmo_replace_as_executable; PlacementMode kUseHint/kAnywhere
/// v8: src/base/platform/platform-fuchsia.cc OS::Allocate / OS::Free
/// 文件句柄不支持 (DCHECK(!handle)); Free = UnmapVmo(root_vmar, ...)
/// v8: src/base/platform/platform-fuchsia.cc OS::AllocateShared / OS::FreeShared
/// SharedMemoryHandle 即 zx::vmo
/// v8: src/base/platform/platform-fuchsia.cc OS::Release — 直通 Free
/// v8: src/base/platform/platform-fuchsia.cc OS::SetPermissions — zx_vmar_protect
/// v8: src/base/platform/platform-fuchsia.cc OS::SetDataReadOnly — SetPermissions(kRead)
/// v8: src/base/platform/platform-fuchsia.cc OS::SetMemoryRegionName — ZX_PROP_NAME
/// v8: src/base/platform/platform-fuchsia.cc OS::RecommitPages / DiscardSystemPages /
///   DecommitPages — decommit 语义经 ZX_VMO_OP / protect 组合
/// v8: src/base/platform/platform-fuchsia.cc OS::SealPages — 恒 false
/// v8: src/base/platform/platform-fuchsia.cc OS::CanReserveAddressSpace — 恒 true
/// v8: src/base/platform/platform-fuchsia.cc OS::HasLazyCommits — 恒 true
/// v8: src/base/platform/platform-fuchsia.cc OS::CreateAddressSpaceReservation /
///   FreeAddressSpaceReservation — 子 VMAR
/// v8: src/base/platform/platform-fuchsia.cc OS::CreateSharedMemoryHandleForTesting /
///   DestroySharedMemoryHandle
/// v8: src/base/platform/platform-fuchsia.cc OS::GetUserTime — zx_info_task_stats
/// v8: src/base/platform/platform-fuchsia.cc OS::GetSharedLibraryAddresses — 空表
/// v8: src/base/platform/platform-fuchsia.cc OS::SignalCodeMovingGC — 空体
/// v8: src/base/platform/platform-fuchsia.cc OS::GetFirstFreeMemoryRangeWithin — 恒 nullopt
/// v8: src/base/platform/platform-fuchsia.cc OS::AdjustSchedulingParams — 空体
/// v8: src/base/platform/platform-fuchsia.cc OS::CreateTimezoneCache — PosixDefaultTimezoneCache

// —— ASR 10 面登记 ——
/// v8: src/base/platform/platform-fuchsia.cc AddressSpaceReservation
/// CreateSubReservation/FreeSubReservation/Allocate/Free/AllocateShared/FreeShared/
/// SetPermissions/RecommitPages/DiscardSystemPages/DecommitPages — 皆子 VMAR 操作

// —— 提取不变式断言 ——
const _: () = {
  assert_eq!(VIRTUAL_MEMORY_NAME, "v8-virtualmem");
};
`;

const p = writeSrc("v8-base-platform", "platform_fuchsia.rs.skel", rust);
console.log(`written ${p}`);
