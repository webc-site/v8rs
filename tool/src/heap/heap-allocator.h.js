#!/usr/bin/env bun
// tool/src/heap/heap-allocator.h.js
// 机械提取 v8/src/heap/heap-allocator.{h,cc,-inl.h}:
// HeapAllocator 方法台账 (AllocateRaw/AllocateRawWith/LAB 族/观察者族) +
// AllocationRetryMode 枚举 (kLightRetry/kRetryOrFail) + MaxRegularHeapObjectSize 规则。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/heap/heap-allocator.h");
const cc = readV8("src/heap/heap-allocator.cc");
const inl = readV8("src/heap/heap-allocator-inl.h");

// —— 枚举 (h 内嵌) ——
const retry = h.match(/enum AllocationRetryMode \{ ([^}]*) \};/);
if (!retry || retry[1] !== "kLightRetry, kRetryOrFail") throw new Error("AllocationRetryMode 漂移");

// —— 方法台账锚 ——
for (const a of [
  "class V8_EXPORT_PRIVATE HeapAllocator final {",
  "using CustomAllocationFunction = base::FunctionRef<bool()>;",
  "explicit HeapAllocator(LocalHeap*);",
  "void Setup();",
  "void SetReadOnlySpace(ReadOnlySpace*);",
  "V8_WARN_UNUSED_RESULT V8_INLINE AllocationResult",
  "template <AllocationRetryMode mode>",
  "bool TryResizeLargeObject(Tagged<HeapObject> object, size_t old_object_size,",
  "V8_INLINE bool CanAllocateInReadOnlySpace() const;",
  "V8_INLINE int MaxRegularHeapObjectSize(AllocationType allocation) const;",
  "void FreeLinearAllocationAreas();",
  "void MakeLinearAllocationAreasIterable();",
  "void MarkLinearAllocationAreasBlack();",
  "void MarkSharedLinearAllocationAreasBlack();",
  "void FreeLinearAllocationAreasAndResetFreeLists();",
  "void FreeSharedLinearAllocationAreasAndResetFreeLists();",
  "void PauseAllocationObservers();",
  "void ResumeAllocationObservers();",
  "void PublishPendingAllocations();",
  "void AddAllocationObserver(AllocationObserver* observer,",
  "void RemoveAllocationObserver(AllocationObserver* observer,",
  "bool RetryCustomAllocate(CustomAllocationFunction allocate,",
  "void RetryCustomAllocateOrFail(CustomAllocationFunction allocate,",
  "V8_INLINE Address pending_large_object() const;",
  "void ResetPendingLargeObject();",
  "std::optional<MainAllocator> new_space_allocator_;",
]) {
  if (!h.includes(a)) throw new Error(`heap-allocator.h 锚点缺失: ${a}`);
}
// LAB 五路分配器槽
for (const slot of ["new_space_allocator_", "old_space_allocator_", "trusted_space_allocator_",
  "code_space_allocator_", "shared_space_allocator_"]) {
  if ((h.match(new RegExp(`std::optional<MainAllocator> ${slot};`, "g")) || []).length !== 1) {
    throw new Error(`分配器槽 ${slot} 漂移`);
  }
}
// cc / inl
for (const a of [
  "void HeapAllocator::Setup() {",
  "void HeapAllocator::FreeLinearAllocationAreas() {",
  "void HeapAllocator::MarkLinearAllocationAreasBlack() {",
  "bool HeapAllocator::TryResizeLargeObject(Tagged<HeapObject> object,",
  "bool HeapAllocator::RetryCustomAllocate(CustomAllocationFunction allocate,",
]) {
  if (!cc.includes(a)) throw new Error(`heap-allocator.cc 锚点缺失: ${a}`);
}
for (const a of [
  "AllocationResult HeapAllocator::AllocateRaw(",
  "int HeapAllocator::MaxRegularHeapObjectSize(",
  "bool HeapAllocator::CanAllocateInReadOnlySpace() const {",
]) {
  if (!inl.includes(a)) throw new Error(`heap-allocator-inl.h 锚点缺失: ${a}`);
}
console.log("HeapAllocator: 枚举 1 + 方法 26 + 分配器槽 5");

let rust = banner(
  "src/heap/heap-allocator.h.js",
  "src/heap/heap-allocator.{h,cc,-inl.h}",
  "LocalHeap 分配器: AllocateRaw/LAB 管理台账",
);

rust += `
/// 分配重试模式 (对应 HeapAllocator::AllocationRetryMode)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AllocationRetryMode {
  /// kLightRetry: 轻重试 (GC 前有限次)
  LIGHT_RETRY = 0,
  /// kRetryOrFail: 重试到成功或致命失败
  RETRY_OR_FAIL = 1,
}

/// 分配器骨架 (对应 class HeapAllocator final, 5 路 optional MainAllocator)。
/// v8: src/heap/heap-allocator.h HeapAllocator
pub struct HeapAllocatorSkeleton {
  pub new_space_allocator: Option<()>,
  pub old_space_allocator: Option<()>,
  pub trusted_space_allocator: Option<()>,
  pub code_space_allocator: Option<()>,
  pub shared_space_allocator: Option<()>,
}

impl HeapAllocatorSkeleton {
  /// 对应 AllocateRaw<mode>(size, allocation, origin, alignment, hint)
  /// v8: src/heap/heap-allocator-inl.h HeapAllocator::AllocateRaw
  pub fn allocate_raw(&mut self, _size_in_bytes: i32, _mode: AllocationRetryMode) -> Result<(), ()> {
    unimplemented!("骨架: MainAllocator/大对象回落待空间族就位");
  }

  /// 对应 MaxRegularHeapObjectSize: 代码空间随 OS 页大小变化, 其余
  /// kMaxRegularHeapObjectSize 常量
  /// v8: src/heap/heap-allocator-inl.h HeapAllocator::MaxRegularHeapObjectSize
  #[must_use]
  pub fn max_regular_heap_object_size(is_code_allocation: bool, k_max_regular_heap_object_size: i32, code_allocatable: i32) -> i32 {
    if is_code_allocation {
      code_allocatable
    } else {
      k_max_regular_heap_object_size
    }
  }

  /// 对应 TryResizeLargeObject: 新大小可原地容纳则改账返回 true
  /// v8: src/heap/heap-allocator.cc HeapAllocator::TryResizeLargeObject
  pub fn try_resize_large_object(&mut self, _old_size: usize, _new_size: usize) -> bool {
    unimplemented!("骨架: OldLargeObjectSpace::ShrinkPageToObjectSize 待 large-spaces 就位");
  }

  /// 对应 FreeLinearAllocationAreas: 全 GC 时弃所有 LAB
  /// v8: src/heap/heap-allocator.cc HeapAllocator::FreeLinearAllocationAreas
  pub fn free_linear_allocation_areas(&mut self) {
    unimplemented!("骨架: MainAllocator::FreeLinearAllocationArea 待 LAB 族就位");
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert_eq!(AllocationRetryMode::LIGHT_RETRY as u8, 0);
  assert_eq!(AllocationRetryMode::RETRY_OR_FAIL as u8, 1);
  assert_eq!(HeapAllocatorSkeleton::max_regular_heap_object_size(false, 131072, 65536), 131072);
  assert_eq!(HeapAllocatorSkeleton::max_regular_heap_object_size(true, 131072, 65536), 65536);
};
`;

const p = writeSrc("v8-heap-core", "heap_allocator.rs.skel", rust);
console.log(`written ${p}`);
