#!/usr/bin/env bun
// tool/src/heap/main-allocator.js
// 机械提取 v8/src/heap/main-allocator{.cc,-inl.h} 族 (族 .h 归他席, 读取仅作上下文):
// AllocatorPolicy 族 + MainAllocator::BlackAllocation 枚举 + ComputeBlackAllocation

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/main-allocator.h"); // 上下文 (本席表外)
const ccSrc = readV8("src/heap/main-allocator.cc");
const inlSrc = readV8("src/heap/main-allocator-inl.h");

const anchors = [
  [hSrc, "class AllocatorPolicy {"],
  [hSrc, "class SemiSpaceNewSpaceAllocatorPolicy final : public AllocatorPolicy {"],
  [hSrc, "class PagedSpaceAllocatorPolicy final : public AllocatorPolicy {"],
  [hSrc, "class PagedNewSpaceAllocatorPolicy final : public AllocatorPolicy {"],
  [hSrc, "static constexpr int kLabSizeInGC = 32 * KB;"],
  [hSrc, "enum class BlackAllocation {"],
  [hSrc, "static constexpr BlackAllocation ComputeBlackAllocation(IsNewGeneration);"],
  [ccSrc, "if (is_new_generation == kNewGeneration) {"],
  [ccSrc, "return BlackAllocation::kAlwaysDisabled;"],
  [ccSrc, "if (v8_flags.sticky_mark_bits) {"],
  [ccSrc, "return BlackAllocation::kAlwaysEnabled;"],
  [ccSrc, "return BlackAllocation::kEnabledOnMarking;"],
  [inlSrc, "MainAllocator::"],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

// 枚举真提取
const bm = hSrc.match(/enum class BlackAllocation \{([^}]*)\};/);
if (!bm) throw new Error("BlackAllocation 未找到");
const black = bm[1].split(",").map((s) => s.trim()).filter(Boolean);
if (black.join() !== "kAlwaysEnabled,kAlwaysDisabled,kEnabledOnMarking") throw new Error(`BlackAllocation 漂移: ${black}`);
// 常量
const lab = parseInt(hSrc.match(/kLabSizeInGC = (\d+) \* KB;/)[1], 10);
if (lab !== 32) throw new Error("kLabSizeInGC 漂移");
console.log(`main-allocator: 枚举 1 (${black.length}), 常量 kLabSizeInGC=${lab}KB, 策略类 4, ComputeBlackAllocation 三分支`);

const snake = (s) => s.replace(/^k/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();
let rust = banner(
  "src/heap/main-allocator.js",
  "src/heap/main-allocator.cc, src/heap/main-allocator-inl.h (族 .h: src/heap/main-allocator.h)",
  "主分配器: 分配策略族与黑分配开关",
);

rust += `/// v8: src/heap/main-allocator.h MainAllocator::BlackAllocation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BlackAllocation {
`;
black.forEach((k, i) => (rust += `  /// ${k}\n  ${snake(k)} = ${i},\n`));
rust += `}

/// v8: src/heap/main-allocator.cc MainAllocator::ComputeBlackAllocation
/// 入参 IsNewGeneration = StrongAlias<bool> (kNewGeneration{true}/kOldGeneration{false})
/// 新生代 → AlwaysDisabled; sticky_mark_bits → AlwaysEnabled; 否则 EnabledOnMarking
pub fn compute_black_allocation(is_new_generation: bool) -> BlackAllocation {
  if is_new_generation {
    return BlackAllocation::AlwaysDisabled;
  }
  if v8_flags().sticky_mark_bits {
    return BlackAllocation::AlwaysEnabled;
  }
  BlackAllocation::EnabledOnMarking
}

/// v8: src/heap/main-allocator.h AllocatorPolicy — 基类虚接口
/// EnsureAllocation(size, alignment, origin) / FreeLinearAllocationArea /
/// SupportsExtendingLAB (默认 false)
pub trait AllocatorPolicy {
  fn ensure_allocation(&mut self, size_in_bytes: i32, alignment: AllocationAlignment, origin: AllocationOrigin) -> bool;
  fn free_linear_allocation_area(&mut self);
  fn supports_extending_lab(&self) -> bool { false }
}

/// v8: src/heap/main-allocator.h SemiSpaceNewSpaceAllocatorPolicy
pub struct SemiSpaceNewSpaceAllocatorPolicy<'a> {
  /// v8: space_
  pub space: &'a mut SemiSpaceNewSpace,
}
impl SemiSpaceNewSpaceAllocatorPolicy<'_> {
  /// v8: kLabSizeInGC = 32 * KB — GC 中 LAB 固定 32KB
  pub const K_LAB_SIZE_IN_GC: i32 = ${lab} * 1024;
  /// v8: FreeLinearAllocationAreaUnsynchronized
  /// v8: EnsureAllocation (cc)
}

/// v8: src/heap/main-allocator.h PagedSpaceAllocatorPolicy
/// RefillLab / ContributeToSweeping (kMaxPagesToSweep=1 见 cc) /
/// TryAllocationFromFreeList / TryExpandAndAllocate / TryExtendLAB /
/// SetLinearAllocationArea / FreeLinearAllocationAreaUnsynchronized
/// v8: src/heap/main-allocator.cc PagedSpaceAllocatorPolicy 各实现

/// v8: src/heap/main-allocator.h PagedNewSpaceAllocatorPolicy
/// SupportsExtendingLAB() = true; TryAllocatePage / WaitForSweepingForAllocation
/// v8: src/heap/main-allocator.cc PagedNewSpaceAllocatorPolicy 各实现

/// v8: src/heap/main-allocator-inl.h MainAllocator 内联面
/// (AllocateRaw/fast path/lab 管理 — 语义体留 Developer)
/// v8: src/heap/main-allocator.cc MainAllocator::MainAllocator / 析构

// —— 提取不变式断言 ——
const _: () = {
  assert!(BlackAllocation::AlwaysDisabled == 1);
  assert!(BlackAllocation::EnabledOnMarking == 2);
};
`;

const p = writeSrc("v8-heap-core", "main_allocator.rs.skel", rust);
console.log(`written ${p}`);
