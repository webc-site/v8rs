//! 主分配器与分配策略族 (MainAllocator / AllocatorPolicy)。
//! 对应原版 v8/src/heap/main-allocator.h (367 行) + main-allocator.cc (983 行)。
//! (HC-2c 签名骨架面 — R5H 富余票开面: 仅落签名/字段/常量声明面, 全数不填体,
//! 函数体以 `todo!("就地修订: …")` 占位 (全仓骨架先例, 如 v8-objects-core slots.rs /
//! v8-wasm-objects wasm_objects_cc.rs); 填体归后续 HC-2c 接力票。)
//!
//! 票面差异申报 (以本仓源实态为准, 主控汇报):
//! 台账/票面所述 h552 行 23 方法 / cc1698 行、"Isolate 推迟仅 cc:1191-1359 族 8 处,
//! 1688-1697 已承接" 均为旧版叙事; 本仓 main-allocator 源实态为 h 367 行 / cc 983 行,
//! 定义面 52 处 (MainAllocator 30 + AllocatorPolicy 族 3 + 三策略类 17 +
//! LinearAreaOriginalData 2)。本文件行号锚全部按实态源标定。
//!
//! 承接形映射 (骨架申报):
//! - `AllocatorPolicy` 抽象类 (h:30-52, 纯虚 ×2 + virtual ×1 + protected 辅助 ×2) →
//!   object-safe trait; h:353 `std::unique_ptr<AllocatorPolicy>` → `Option<Box<dyn …>>`
//!   (C++ 可空, ctor 初始化列表 cc:51 恒 emplace, 空位仅移动前可见)。
//! - `SafeHeapObjectSize` (src/heap/allocation.h 的 StrongAlias 面, 未落地) → `usize`
//!   形参承接 (doc 注记; 填体票若需强类型再引 allocation.h 族)。
//! - `IsNewGeneration` (h:142 base::StrongAlias) → [`v8_base::strong_alias::StrongAlias`]
//!   (v8-base 已落地移植面)。
//! - C++ 默认参 (`allocation_info = nullptr` h:152 / `max_pages = uint32 max` h:86) →
//!   裸指针传 null / `Option<u32>` 承接, 签名处注记。
//! - `v8_flags.sticky_mark_bits` (cc:35 全局) → bool 参数注入 (free_list.rs /
//!   memory_allocator.rs 参数注入先例)。
//! - friend (h:359-361 ×3 + h:100 PagedNewSpaceAllocatorPolicy) → Rust 无友元, 全 pub
//!   承接 (memory_allocator 台账 16 先例)。
//! - `#if DEBUG Verify()` (h:230-232 + cc:394-415) → `#[cfg(debug_assertions)]`
//!   (memory_allocator DEBUG 登记面先例)。
//! - 实体他票: Isolate/Heap (crate::memory_allocator::Heap 前向承接复用)、LocalHeap、
//!   SpaceWithLinearArea/PagedNewSpace/PagedSpaceBase/SemiSpaceNewSpace (spaces.h 族,
//!   HC-4 队列)、YoungPendingAllocations (pending-allocations.h)、页/空间方法面 →
//!   本模块全部以 opaque 指针形参/字段承接, 从不解引用。
//! - main-allocator-inl.h (110 行: AllocateRaw/TryFreeLast/GetFillToAlign/
//!   AllocateFastAligned/AllocateFastUnaligned 的 inline 实现体) 属 -inl 面他票,
//!   本票仅按 h 声明面 (h:192-194/221/254-255/268-278) 落签名。

use std::ptr::{null, null_mut};

use v8_base::strong_alias::StrongAlias;
use v8_base_platform::Mutex;
use v8_common::{
  Address, AllocationAlignment, AllocationHint, AllocationSpace, K_DOUBLE_ALIGNMENT_MASK,
  K_DOUBLE_SIZE, K_NULL_ADDRESS, K_TAGGED_SIZE, KB, V8_COMPRESS_POINTERS_8GB,
};

use crate::{
  allocation::{AllocationOrigin, AllocationResult},
  allocation_observer::{AllocationCounter, ObserverRef},
  linear_allocation_area::LinearAllocationArea,
  memory_allocator::Heap,
};

// ══════════════════════════════════════════════════════════════════════════
// 前向声明承接 (原版 h:21-28 namespace 前置类)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 h:22 `class LocalHeap;` 的 opaque 承接 (实体 heap/local-heap.h 属他票)。
pub enum LocalHeap {}

/// 原版 h:27 `class SpaceWithLinearArea;` 的 opaque 承接 (spaces.h 族, HC-4 队列)。
/// cc:51 `space->CreateAllocatorPolicy(this)` / cc:446 `space->identity()` 等消费面
/// 归填体票。
pub enum SpaceWithLinearArea {}

/// 原版 h:24 `class PagedNewSpace;` 的 opaque 承接 (paged-spaces.h 族, HC-4 队列)。
pub enum PagedNewSpace {}

/// 原版 h:25 `class PagedSpaceBase;` 的 opaque 承接 (spaces.h 族, HC-4 队列)。
pub enum PagedSpaceBase {}

/// 原版 h:26 `class SemiSpaceNewSpace;` 的 opaque 承接 (new-spaces.h 族, HC-4 队列)。
pub enum SemiSpaceNewSpace {}

/// 原版 h:28 `class YoungPendingAllocations;` 的 opaque 承接 (pending-allocations.h)。
pub enum YoungPendingAllocations {}

// h:23 `class MainAllocator;` 由本文件落地直用; h:21 `class Heap;` 复用
// crate::memory_allocator::Heap 前向承接位 (同型指针, 不重复声明)。

// ══════════════════════════════════════════════════════════════════════════
// AllocatorPolicy 抽象基类 (原版 h:30-52)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 h:30-52 `class AllocatorPolicy` — 分配策略抽象基类。
/// h:40-43 纯虚双件套 → trait 无默认体签名; h:45 `SupportsExtendingLAB` 虚默认 false →
/// 默认体 todo 占位 (骨架票不填体); h:48-49 protected 辅助 (space_heap/isolate_heap) →
/// trait 默认方法承接 (C++ 友元/protected 语义以 pub 承接, 见模块头申报)。
pub trait AllocatorPolicy {
  /// 原版 h:40-42 纯虚 `EnsureAllocation(size_in_bytes, alignment, origin)` —
  /// 备好容纳 size 的 LAB; 空间不足返回 false 由调用方 GC 后重试。
  /// SafeHeapObjectSize 承接差异见模块头。
  fn ensure_allocation(
    &mut self,
    _size_in_bytes: i32,
    _alignment: AllocationAlignment,
    _origin: AllocationOrigin,
  ) -> bool;

  /// 原版 h:43 纯虚 `FreeLinearAllocationArea()`。
  fn free_linear_allocation_area(&mut self);

  /// 原版 h:45 `virtual bool SupportsExtendingLAB() const` (inline default return false;
  /// PagedNewSpaceAllocatorPolicy h:111 覆写 return true)。
  fn supports_extending_lab(&self) -> bool {
    // 原版 h:45 inline default `return false` (PagedNewSpaceAllocatorPolicy h:111 覆写 true)。
    false
  }

  /// 原版 h:51 `MainAllocator* const allocator_` 字段读出 (基类 protected 面承接;
  /// h:32 ctor 注入, cc:458-459 初始化)。
  fn allocator(&self) -> *mut MainAllocator;

  /// 原版 h:48 声明 + cc:461 `AllocatorPolicy::space_heap()` —
  /// `allocator_->space_heap()` 转发 (友元消费, pub 承接)。
  fn space_heap(&self) -> *mut Heap {
    todo!("就地修订: cc:461 return allocator_->space_heap()")
  }

  /// 原版 h:49 声明 + cc:463-465 `AllocatorPolicy::isolate_heap()` —
  /// `allocator_->isolate_heap()` 转发。
  fn isolate_heap(&self) -> *mut Heap {
    todo!("就地修订: cc:463-465 return allocator_->isolate_heap()")
  }
}

// ══════════════════════════════════════════════════════════════════════════
// SemiSpaceNewSpaceAllocatorPolicy (原版 h:54-70)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 h:54-70 `class SemiSpaceNewSpaceAllocatorPolicy final` — 半空间新生代策略。
pub struct SemiSpaceNewSpaceAllocatorPolicy {
  /// 原版 h:51 基类字段 `MainAllocator* const allocator_` (h:58 ctor 基类段注入)。
  pub allocator: *mut MainAllocator,
  /// 原版 h:69 `SemiSpaceNewSpace* const space_`。
  pub space: *mut SemiSpaceNewSpace,
}

impl SemiSpaceNewSpaceAllocatorPolicy {
  /// 原版 kLabSizeInGC (h:65 `static constexpr int kLabSizeInGC = 32 * KB`)。
  pub const K_LAB_SIZE_IN_GC: i32 = (32 * KB) as i32;

  /// 原版 h:56-58 inline ctor (初始化列表 AllocatorPolicy(allocator), space_(space))。
  /// v8: src/heap/main-allocator.h SemiSpaceNewSpaceAllocatorPolicy
  #[must_use]
  pub fn new(
    space: *mut SemiSpaceNewSpace,
    allocator: *mut MainAllocator,
  ) -> SemiSpaceNewSpaceAllocatorPolicy {
    // 原版 h:56-58 inline ctor: AllocatorPolicy(allocator), space_(space) 双字段直落。
    Self { allocator, space }
  }

  /// 原版 h:67 声明 + cc:536-547 `FreeLinearAllocationAreaUnsynchronized()` —
  /// cc:523-534 公有版持 VM 状态后转本无锁版。
  /// v8: src/heap/main-allocator.cc SemiSpaceNewSpaceAllocatorPolicy::FreeLinearAllocationAreaUnsynchronized
  pub fn free_linear_allocation_area_unsynchronized(&mut self) {
    todo!("就地修订: cc:536-547 (heap->CollectAll/VM 状态面属他票)")
  }
}

impl AllocatorPolicy for SemiSpaceNewSpaceAllocatorPolicy {
  /// 原版 h:61 + cc:467-521 `EnsureAllocation` final 覆写。
  /// v8: src/heap/main-allocator.cc SemiSpaceNewSpaceAllocatorPolicy::EnsureAllocation
  fn ensure_allocation(
    &mut self,
    _size_in_bytes: i32,
    _alignment: AllocationAlignment,
    _origin: AllocationOrigin,
  ) -> bool {
    todo!("就地修订: cc:467-521 (SemiSpaceNewSpace 页轮换面属 HC-4)")
  }

  /// 原版 h:62 + cc:523-534 `FreeLinearAllocationArea` final 覆写。
  /// v8: src/heap/main-allocator.cc SemiSpaceNewSpaceAllocatorPolicy::FreeLinearAllocationArea
  fn free_linear_allocation_area(&mut self) {
    todo!("就地修订: cc:523-534 (VM 状态作用域 + 转 Unsynchronized)")
  }

  /// 原版 h:51 基类字段读出。
  /// v8: src/heap/main-allocator.h AllocatorPolicy::allocator_
  fn allocator(&self) -> *mut MainAllocator {
    self.allocator
  }
}

// ══════════════════════════════════════════════════════════════════════════
// PagedSpaceAllocatorPolicy (原版 h:72-101)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 h:72-101 `class PagedSpaceAllocatorPolicy final` — 分页空间策略。
/// h:100 `friend class PagedNewSpaceAllocatorPolicy` → Rust 无友元, 全 pub 承接。
pub struct PagedSpaceAllocatorPolicy {
  /// 原版 h:51 基类字段 `MainAllocator* const allocator_`。
  pub allocator: *mut MainAllocator,
  /// 原版 h:98 `PagedSpaceBase* const space_`。
  pub space: *mut PagedSpaceBase,
}

impl PagedSpaceAllocatorPolicy {
  /// 原版 h:74-75 inline ctor。
  /// v8: src/heap/main-allocator.h PagedSpaceAllocatorPolicy
  #[must_use]
  pub fn new(
    space: *mut PagedSpaceBase,
    allocator: *mut MainAllocator,
  ) -> PagedSpaceAllocatorPolicy {
    // 原版 h:74-75 inline ctor: AllocatorPolicy(allocator), space_(space) 双字段直落。
    Self { allocator, space }
  }

  /// 原版 h:82 声明 + cc:683-787 `RefillLab(size_in_bytes, origin)` —
  /// 页取/队列重填/free-list 主流程 (依赖 NormalPage/free_list 运行态, HC-4 衔接)。
  /// v8: src/heap/main-allocator.cc PagedSpaceAllocatorPolicy::RefillLab
  pub fn refill_lab(&mut self, _size_in_bytes: usize, _origin: AllocationOrigin) -> bool {
    todo!("就地修订: cc:683-787 (NormalPage/CodeFlushManager 面属他票)")
  }

  /// 原版 h:85-86 声明 + cc:801-834 `ContributeToSweeping(max_pages)` —
  /// 默认参 `std::numeric_limits<uint32_t>::max()` (h:86) → Option<u32> 承接 (None 即默认)。
  /// v8: src/heap/main-allocator.cc PagedSpaceAllocatorPolicy::ContributeToSweeping
  pub fn contribute_to_sweeping(&mut self, _max_pages: Option<u32>) -> bool {
    todo!("就地修订: cc:801-834 (sweeper/concurrent_sweeping 面属他票)")
  }

  /// 原版 h:88 声明 + cc:849-905 `TryAllocationFromFreeList(size_in_bytes, origin)`。
  /// v8: src/heap/main-allocator.cc PagedSpaceAllocatorPolicy::TryAllocationFromFreeList
  pub fn try_allocation_from_free_list(
    &mut self,
    _size_in_bytes: usize,
    _origin: AllocationOrigin,
  ) -> bool {
    todo!("就地修订: cc:849-905 (free_list 落地面已备, 页方法面属 HC-4)")
  }

  /// 原版 h:90 声明 + cc:789-799 `TryExpandAndAllocate(size_in_bytes, origin)`。
  /// v8: src/heap/main-allocator.cc PagedSpaceAllocatorPolicy::TryExpandAndAllocate
  pub fn try_expand_and_allocate(
    &mut self,
    _size_in_bytes: usize,
    _origin: AllocationOrigin,
  ) -> bool {
    todo!("就地修订: cc:789-799 (space->ExpandAndAllocateFinished 面属 HC-4)")
  }

  /// 原版 h:92-93 `V8_WARN_UNUSED_RESULT bool TryExtendLAB(int size_in_bytes)` +
  /// cc:907-929。
  /// v8: src/heap/main-allocator.cc PagedSpaceAllocatorPolicy::TryExtendLAB
  #[must_use]
  pub fn try_extend_lab(&mut self, _size_in_bytes: i32) -> bool {
    todo!("就地修订: cc:907-929 (page->allocated_bytes/heap 面属 HC-4)")
  }

  /// 原版 h:94 声明 + cc:836-847 `SetLinearAllocationArea(top, limit, end)`。
  /// v8: src/heap/main-allocator.cc PagedSpaceAllocatorPolicy::SetLinearAllocationArea
  pub fn set_linear_allocation_area(&mut self, _top: Address, _limit: Address, _end: Address) {
    todo!("就地修订: cc:836-847 (allocator->ResetLab + 观测器 ComputeLimit)")
  }

  /// 原版 h:96 声明 + cc:938-969 `FreeLinearAllocationAreaUnsynchronized()`。
  /// v8: src/heap/main-allocator.cc PagedSpaceAllocatorPolicy::FreeLinearAllocationAreaUnsynchronized
  pub fn free_linear_allocation_area_unsynchronized(&mut self) {
    todo!("就地修订: cc:938-969 (page/owner/free-list 归还面属 HC-4)")
  }
}

impl AllocatorPolicy for PagedSpaceAllocatorPolicy {
  /// 原版 h:77 + cc:657-681 `EnsureAllocation` final 覆写。
  /// v8: src/heap/main-allocator.cc PagedSpaceAllocatorPolicy::EnsureAllocation
  fn ensure_allocation(
    &mut self,
    _size_in_bytes: i32,
    _alignment: AllocationAlignment,
    _origin: AllocationOrigin,
  ) -> bool {
    todo!("就地修订: cc:657-681 (LAB 有效性 + RefillLab 分发)")
  }

  /// 原版 h:79 + cc:931-936 `FreeLinearAllocationArea` final 覆写。
  /// v8: src/heap/main-allocator.cc PagedSpaceAllocatorPolicy::FreeLinearAllocationArea
  fn free_linear_allocation_area(&mut self) {
    todo!("就地修订: cc:931-936 (VM 状态作用域 + 转 Unsynchronized)")
  }

  /// 原版 h:51 基类字段读出。
  /// v8: src/heap/main-allocator.h AllocatorPolicy::allocator_
  fn allocator(&self) -> *mut MainAllocator {
    self.allocator
  }
}

// ══════════════════════════════════════════════════════════════════════════
// PagedNewSpaceAllocatorPolicy (原版 h:103-119)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 h:103-119 `class PagedNewSpaceAllocatorPolicy final` — 新生代分页策略。
pub struct PagedNewSpaceAllocatorPolicy {
  /// 原版 h:51 基类字段 `MainAllocator* const allocator_`。
  pub allocator: *mut MainAllocator,
  /// 原版 h:117 `PagedNewSpace* const space_`。
  pub space: *mut PagedNewSpace,
  /// 原版 h:118 `std::unique_ptr<PagedSpaceAllocatorPolicy> paged_space_allocator_policy_`
  /// (cc:549-554 ctor make_unique, h:100 友元授权消费)。
  pub paged_space_allocator_policy: Option<Box<PagedSpaceAllocatorPolicy>>,
}

impl PagedNewSpaceAllocatorPolicy {
  /// 原版 h:105 声明 + cc:549-554 ctor (基类段 + space_ + make_unique 委托件)。
  /// v8: src/heap/main-allocator.cc PagedNewSpaceAllocatorPolicy::PagedNewSpaceAllocatorPolicy
  #[must_use]
  pub fn new(
    space: *mut PagedNewSpace,
    allocator: *mut MainAllocator,
  ) -> PagedNewSpaceAllocatorPolicy {
    // 原版 cc:549-554 ctor: AllocatorPolicy(allocator) + space_ +
    // make_unique<PagedSpaceAllocatorPolicy>(space, allocator) 委托件 (PagedNewSpace
    // 系 PagedSpaceBase 派生, 指针按基类承接)。
    let paged_space = Box::new(PagedSpaceAllocatorPolicy::new(
      space as *mut PagedSpaceBase,
      allocator,
    ));
    Self {
      allocator,
      space,
      paged_space_allocator_policy: Some(paged_space),
    }
  }

  /// 原版 h:114 声明 + cc:638-648 `TryAllocatePage(size_in_bytes, origin)`。
  /// v8: src/heap/main-allocator.cc PagedNewSpaceAllocatorPolicy::TryAllocatePage
  pub fn try_allocate_page(&mut self, _size_in_bytes: i32, _origin: AllocationOrigin) -> bool {
    todo!("就地修订: cc:638-648 (space->AllocatePage 面属 HC-4)")
  }

  /// 原版 h:115 声明 + cc:589-636 `WaitForSweepingForAllocation(size_in_bytes, origin)` —
  /// concurrent sweeping 等待/让步 (TaskScheduler/Isolate 运行态)。
  /// v8: src/heap/main-allocator.cc PagedNewSpaceAllocatorPolicy::WaitForSweepingForAllocation
  pub fn wait_for_sweeping_for_allocation(
    &mut self,
    _size_in_bytes: i32,
    _origin: AllocationOrigin,
  ) -> bool {
    todo!("就地修订: cc:589-636 (sweeping 等待面属他票)")
  }
}

impl AllocatorPolicy for PagedNewSpaceAllocatorPolicy {
  /// 原版 h:107 + cc:556-587 `EnsureAllocation` final 覆写。
  /// v8: src/heap/main-allocator.cc PagedNewSpaceAllocatorPolicy::EnsureAllocation
  fn ensure_allocation(
    &mut self,
    _size_in_bytes: i32,
    _alignment: AllocationAlignment,
    _origin: AllocationOrigin,
  ) -> bool {
    todo!("就地修订: cc:556-587 (LAB 快路 + sweeping 等待 + 页取分发)")
  }

  /// 原版 h:109 + cc:650-655 `FreeLinearAllocationArea` final 覆写。
  /// v8: src/heap/main-allocator.cc PagedNewSpaceAllocatorPolicy::FreeLinearAllocationArea
  fn free_linear_allocation_area(&mut self) {
    todo!("就地修订: cc:650-655 (转委托件 FreeLinearAllocationArea)")
  }

  /// 原版 h:111 `bool SupportsExtendingLAB() const final` (inline return true)。
  /// v8: src/heap/main-allocator.h PagedNewSpaceAllocatorPolicy::SupportsExtendingLAB
  fn supports_extending_lab(&self) -> bool {
    // 原版 h:111 inline final `return true` (唯一支持 LAB 扩容的策略)。
    true
  }

  /// 原版 h:51 基类字段读出。
  /// v8: src/heap/main-allocator.h AllocatorPolicy::allocator_
  fn allocator(&self) -> *mut MainAllocator {
    self.allocator
  }
}

// ══════════════════════════════════════════════════════════════════════════
// LinearAreaOriginalData (原版 h:121-135)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 h:121-135 `class LinearAreaOriginalData` — 后台任务可读的 LAB 原始 top/limit
/// 登记 (mutex_ 保护, h:128-133 注释照录)。
#[derive(Debug)]
pub struct LinearAreaOriginalData {
  /// 原版 h:130 `Address original_top_ = 0`。
  pub original_top: Address,
  /// 原版 h:131 `Address original_limit_ = 0`。
  pub original_limit: Address,
  /// 原版 h:133-134 `mutable base::Mutex mutex_` (protects original_top_/limit_)。
  pub mutex: Mutex,
}

impl Default for LinearAreaOriginalData {
  /// 原版 h:130/131 字段初值 0。
  fn default() -> Self {
    Self {
      original_top: 0,
      original_limit: 0,
      mutex: Mutex::new(),
    }
  }
}

impl LinearAreaOriginalData {
  /// 原版 h:123 声明 + cc:971-974 `std::pair<Address, Address> GetTopAndLimit() const`。
  /// v8: src/heap/main-allocator.cc LinearAreaOriginalData::GetTopAndLimit
  #[must_use]
  pub fn get_top_and_limit(&self) -> (Address, Address) {
    // 原版 cc:971-974: MutexGuard 持锁读 original_top_/original_limit_ 双字段。
    let _guard = self.mutex.lock_guard();
    (self.original_top, self.original_limit)
  }

  /// 原版 h:125 声明 + cc:976-981 `SetTopAndLimit(top, limit)`。
  /// v8: src/heap/main-allocator.cc LinearAreaOriginalData::SetTopAndLimit
  pub fn set_top_and_limit(&mut self, top: Address, limit: Address) {
    // 原版 cc:976-981: MutexGuard 持锁写 original_top_/original_limit_ 双字段。
    let _guard = self.mutex.lock_guard();
    self.original_top = top;
    self.original_limit = limit;
  }
}

// ══════════════════════════════════════════════════════════════════════════
// MainAllocator (原版 h:137-362)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 h:139 `struct InGCTag {}` + h:140 `static constexpr InGCTag kInGC{}` —
/// GC LAB/分配用途 ctor 的标签型参。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InGCTag;

/// 原版 h:140 `kInGC` 常量位。
pub const K_IN_GC: InGCTag = InGCTag;

/// 原版 h:142 tag 结构 `struct IsNewGenerationTag` (StrongAlias 幻影参)。
pub struct IsNewGenerationTag;

/// 原版 h:142 `using IsNewGeneration = base::StrongAlias<IsNewGenerationTag, bool>`。
pub type IsNewGeneration = StrongAlias<IsNewGenerationTag, bool>;

/// 原版 h:143 `static constexpr IsNewGeneration kNewGeneration{true}`。
pub const K_NEW_GENERATION: IsNewGeneration = IsNewGeneration::new(true);

/// 原版 h:144 `static constexpr IsNewGeneration kOldGeneration{false}`。
pub const K_OLD_GENERATION: IsNewGeneration = IsNewGeneration::new(false);

/// 原版 h:258-262 `enum class BlackAllocation` (private; 全 pub 承接, 模块头申报)。
/// C++ enum class 默认底层 int → `#[repr(i32)]`; 变体去 k 前缀 PascalCase 值不变
/// (allocation.rs AllocationOrigin 先例)。
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlackAllocation {
  /// h:259 kAlwaysEnabled, 值 0。
  AlwaysEnabled = 0,
  /// h:260 kAlwaysDisabled, 值 1。
  AlwaysDisabled = 1,
  /// h:261 kEnabledOnMarking, 值 2。
  EnabledOnMarking = 2,
}

/// 原版 h:137-362 `class MainAllocator` — 主线程/后台线程线性分配 (LAB) 分配器,
/// 组合 MemoryAllocator (HC-2b 已落基座) 之上的空间分配策略与观测器基础设施。
///
/// 字段命名承接差异: C++ 字段尾下划线在 Rust 保留于与访问器同名者
/// (local_heap_/isolate_heap_/…_ 全列), 与 h:339-357 逐项对照;
/// std::optional<T> → Option<T>, std::unique_ptr<T> → Option<Box<T>>。
pub struct MainAllocator {
  /// 原版 h:338-339 注释 + 字段 `LocalHeap* const local_heap_` — 当前主/后台线程的
  /// LocalHeap, GC 线程为 nullptr (h:314 in_gc 判据源)。
  pub local_heap_: *mut LocalHeap,
  /// 原版 h:340 `Heap* const isolate_heap_`。
  pub isolate_heap_: *mut Heap,
  /// 原版 h:341 `SpaceWithLinearArea* const space_`。
  pub space_: *mut SpaceWithLinearArea,
  /// 原版 h:343 `std::optional<AllocationCounter> allocation_counter_` —
  /// 主线程才有 (cc:57-58 emplace 条件)。
  pub allocation_counter_: Option<AllocationCounter>,
  /// 原版 h:344 `LinearAllocationArea* const allocation_info_` — 外部注入或指
  /// owned_allocation_info_ (cc:48-49 三目)。
  pub allocation_info_: *mut LinearAllocationArea,
  /// 原版 h:345-346 注释 + 字段 `LinearAllocationArea owned_allocation_info_` —
  /// 未传引用时的自持位。
  pub owned_allocation_info_: LinearAllocationArea,
  /// 原版 h:347-350 注释 + 字段 `Address extended_limit_ = kNullAddress` — LAB 扩容位。
  pub extended_limit_: Address,
  /// 原版 h:352 `std::optional<LinearAreaOriginalData> linear_area_original_data_` —
  /// 主线程非 NEW_SPACE 才有 (cc:60-62 emplace 条件)。
  pub linear_area_original_data_: Option<LinearAreaOriginalData>,
  /// 原版 h:353 `std::unique_ptr<AllocatorPolicy> allocator_policy_` —
  /// cc:51 `space->CreateAllocatorPolicy(this)` 生产 (空间实体属 HC-4)。
  pub allocator_policy_: Option<Box<dyn AllocatorPolicy>>,
  /// 原版 h:355 `const bool supports_extending_lab_` — cc:52 取策略读出。
  pub supports_extending_lab_: bool,
  /// 原版 h:356 `const BlackAllocation black_allocation_` — cc:53 ComputeBlackAllocation。
  pub black_allocation_: BlackAllocation,
  /// 原版 h:357 `YoungPendingAllocations* const young_pending_allocations_` —
  /// cc:54-56 仅 NEW_SPACE 非空。
  pub young_pending_allocations_: *mut YoungPendingAllocations,
}

impl MainAllocator {
  // ── 构造 (h:146-156 + cc:42-77) ──

  /// 原版 h:149-152 声明 + cc:42-64 主 ctor (h:146-148 注释: 主/后台线程用,
  /// allocation_info 供生成代码分配支持)。
  /// 默认参 `LinearAllocationArea* allocation_info = nullptr` (h:152) → 裸指针传 null
  /// 承接 (cc:48-49 三目落 owned 位)。
  /// v8: src/heap/main-allocator.cc MainAllocator::MainAllocator(LocalHeap*,...)
  #[must_use]
  pub fn new(
    _local_heap: *mut LocalHeap,
    _space: *mut SpaceWithLinearArea,
    _is_new_generation: IsNewGeneration,
    _allocation_info: *mut LinearAllocationArea,
  ) -> MainAllocator {
    todo!("就地修订: cc:42-64 (local_heap->heap()/CreateAllocatorPolicy/emplace 条件面)")
  }

  /// 原版 h:155-156 声明 + cc:66-77 GC ctor (h:154 注释: GC LABs/分配用)。
  /// v8: src/heap/main-allocator.cc MainAllocator::MainAllocator(Heap*,...)
  #[must_use]
  pub fn new_in_gc(
    _heap: *mut Heap,
    _space: *mut SpaceWithLinearArea,
    _tag: InGCTag,
  ) -> MainAllocator {
    todo!("就地修订: cc:66-77 (local_heap_ 置空 + 双 optional 空 DCHECK)")
  }

  // ── LAB 三元组访问器 (h:158-171, 经 allocation_info_ 转发) ──

  /// 原版 h:159 `Address start() const` — `allocation_info_->start()`。
  /// (h:158 注释: Returns the allocation pointer in this space 领起 start/top/limit 三行。)
  /// v8: src/heap/main-allocator.h MainAllocator::start
  #[must_use]
  pub fn start(&self) -> Address {
    // 原版 h:159 `allocation_info_->start()` — allocation_info_ 转发到已落地 LAA。
    // SAFETY: ctor 恒置 allocation_info_ 非空且指向有效 LinearAllocationArea
    // (cc:48-49 三目落 owned 位), 与 C++ `allocation_info_->start()` 逐字对位。
    unsafe { (*self.allocation_info_).start() }
  }

  /// 原版 h:160 `Address top() const` — `allocation_info_->top()`。
  /// v8: src/heap/main-allocator.h MainAllocator::top
  #[must_use]
  pub fn top(&self) -> Address {
    // 原版 h:160 `allocation_info_->top()`。
    // SAFETY: 同 start — allocation_info_ 由 ctor 保证指向有效 LAA。
    unsafe { (*self.allocation_info_).top() }
  }

  /// 原版 h:161 `Address limit() const` — `allocation_info_->limit()`。
  /// v8: src/heap/main-allocator.h MainAllocator::limit
  #[must_use]
  pub fn limit(&self) -> Address {
    // 原版 h:161 `allocation_info_->limit()`。
    // SAFETY: 同 start — allocation_info_ 由 ctor 保证指向有效 LAA。
    unsafe { (*self.allocation_info_).limit() }
  }

  /// 原版 h:163-166 `Address* allocation_top_address() const` (h:163 注释照录)。
  /// v8: src/heap/main-allocator.h MainAllocator::allocation_top_address
  #[must_use]
  pub fn allocation_top_address(&self) -> *mut Address {
    // 原版 h:165 `allocation_info_->top_address()` — 取 LAA top_ 成员裸地址, 纯指针
    // 算术 (不形成可变引用), 经已落地 LinearAllocationArea::top_offset 偏移承接。
    unsafe {
      (self.allocation_info_ as *mut u8).add(LinearAllocationArea::top_offset() as usize)
        as *mut Address
    }
  }

  /// 原版 h:168-171 `Address* allocation_limit_address() const` (h:168 注释照录)。
  /// v8: src/heap/main-allocator.h MainAllocator::allocation_limit_address
  #[must_use]
  pub fn allocation_limit_address(&self) -> *mut Address {
    // 原版 h:170 `allocation_info_->limit_address()` — 取 LAA limit_ 成员裸地址,
    // 纯指针算术, 经 LinearAllocationArea::limit_offset 偏移承接。
    unsafe {
      (self.allocation_info_ as *mut u8).add(LinearAllocationArea::limit_offset() as usize)
        as *mut Address
    }
  }

  // ── LAB 生命周期面 (h:173-176 + h:239-241) ──

  /// 原版 h:173 声明 + cc:270-277 `MoveOriginalTopForward()`。
  /// v8: src/heap/main-allocator.cc MainAllocator::MoveOriginalTopForward
  pub fn move_original_top_forward(&mut self) {
    todo!("就地修订: cc:270-277 (SupportsPendingAllocation 判 + SetTopAndLimit)")
  }

  /// 原版 h:174-175 声明 + cc:279-304 `ResetLab(start, end, extended_end)`。
  /// v8: src/heap/main-allocator.cc MainAllocator::ResetLab
  pub fn reset_lab(&mut self, _start: Address, _end: Address, _extended_end: Address) {
    todo!("就地修订: cc:279-304 (策略 free + ComputeLimit + MarkLabStartInitialized)")
  }

  /// 原版 h:176 声明 + cc:306-310 `IsPendingAllocation(Address)` —
  /// 原始 top/limit 区间判 (后台扫描用)。
  /// v8: src/heap/main-allocator.cc MainAllocator::IsPendingAllocation
  #[must_use]
  pub fn is_pending_allocation(&self, object_address: Address) -> bool {
    // 原版 cc:306-310: DCHECK(SupportsPendingAllocation) 后取原始 top/limit 区间判
    // `top && top <= object_address && object_address < limit`。
    debug_assert!(self.supports_pending_allocation());
    match self.linear_area_original_data_.as_ref() {
      Some(data) => {
        let (top, limit) = data.get_top_and_limit();
        top != K_NULL_ADDRESS && top <= object_address && object_address < limit
      }
      None => false,
    }
  }

  /// 原版 h:178-182 双载 `LinearAllocationArea& allocation_info()` / const 版 —
  /// Rust 无 const 重载: `allocation_info()` 承 const 载 (h:180-182),
  /// `allocation_info_mut()` 承非 const 载 (h:178)。
  /// v8: src/heap/main-allocator.h MainAllocator::allocation_info
  #[must_use]
  pub fn allocation_info(&self) -> *const LinearAllocationArea {
    // 原版 h:181 `return *allocation_info_` (const 载) — 裸指针承载, 从不解引用。
    self.allocation_info_ as *const LinearAllocationArea
  }

  /// 原版 h:178 非 const 载 (见上条双载注)。
  /// v8: src/heap/main-allocator.h MainAllocator::allocation_info
  #[must_use]
  pub fn allocation_info_mut(&mut self) -> *mut LinearAllocationArea {
    // 原版 h:178 `return *allocation_info_` (非 const 载) — 裸指针承载。
    self.allocation_info_
  }

  /// 原版 h:184-186 `AllocationCounter& allocation_counter()` (非 const 载,
  /// `allocation_counter_.value()`)。
  /// v8: src/heap/main-allocator.h MainAllocator::allocation_counter
  #[must_use]
  pub fn allocation_counter_mut(&mut self) -> *mut AllocationCounter {
    // 原版 h:185 `allocation_counter_.value()` — Option 有值取内部裸指针, 空则 null
    // (承接 value() 前置条件, 从不解引用)。
    self
      .allocation_counter_
      .as_mut()
      .map_or(null_mut(), |counter| counter as *mut AllocationCounter)
  }

  /// 原版 h:188-190 const 载。
  /// v8: src/heap/main-allocator.h MainAllocator::allocation_counter
  #[must_use]
  pub fn allocation_counter(&self) -> *const AllocationCounter {
    // 原版 h:189 `allocation_counter_.value()` (const 载)。
    self
      .allocation_counter_
      .as_ref()
      .map_or(null(), |counter| counter as *const AllocationCounter)
  }

  // ── 分配主路径 (h:192-199 + private h:268-287) ──

  /// 原版 h:192-194 `V8_WARN_UNUSED_RESULT V8_INLINE AllocationResult AllocateRaw(
  /// SafeHeapObjectSize, AllocationAlignment, AllocationOrigin, AllocationHint)` —
  /// inline 实现体在 main-allocator-inl.h (模块头申报: -inl 面他票, 本票仅签名)。
  /// SafeHeapObjectSize → usize 承接。
  /// v8: src/heap/main-allocator.h MainAllocator::AllocateRaw
  #[must_use]
  pub fn allocate_raw(
    &mut self,
    _size_in_bytes: usize,
    _alignment: AllocationAlignment,
    _origin: AllocationOrigin,
    _hint: AllocationHint,
  ) -> AllocationResult {
    todo!("就地修订: main-allocator-inl.h AllocateRaw (快路 + EnsureAllocation 重试慢路)")
  }

  /// 原版 h:196-199 `AllocationResult AllocateRawForceAlignmentForTesting(
  /// int size_in_bytes, AllocationAlignment, AllocationOrigin)` + cc:93-104。
  /// v8: src/heap/main-allocator.cc MainAllocator::AllocateRawForceAlignmentForTesting
  #[must_use]
  pub fn allocate_raw_force_alignment_for_testing(
    &mut self,
    _size_in_bytes: i32,
    _alignment: AllocationAlignment,
    _origin: AllocationOrigin,
  ) -> AllocationResult {
    todo!("就地修订: cc:93-104 (ALIGN_TO_ALLOCATION_ALIGNMENT + 快慢路)")
  }

  /// 原版 h:268-269 private `AllocateFastUnaligned(SafeHeapObjectSize, origin)` —
  /// -inl.h 实现体他票。
  /// v8: src/heap/main-allocator.h MainAllocator::AllocateFastUnaligned
  #[must_use]
  pub fn allocate_fast_unaligned(
    &mut self,
    _size_in_bytes: usize,
    _origin: AllocationOrigin,
  ) -> AllocationResult {
    todo!("就地修订: -inl.h AllocateFastUnaligned (LAB increment_top 快路)")
  }

  /// 原版 h:275-278 private `AllocateFastAligned(size, result_aligned_size*, align, origin)`
  /// (h:271-274 注释照录: 装不下返 nullptr, 否则对象指针 + 对齐填充尺寸回写) —
  /// -inl.h 实现体他票。
  /// v8: src/heap/main-allocator.h MainAllocator::AllocateFastAligned
  #[must_use]
  pub fn allocate_fast_aligned(
    &mut self,
    _size_in_bytes: usize,
    _result_aligned_size_in_bytes: *mut usize,
    _alignment: AllocationAlignment,
    _origin: AllocationOrigin,
  ) -> AllocationResult {
    todo!("就地修订: -inl.h AllocateFastAligned (GetFillToAlign + 快路)")
  }

  /// 原版 h:281-284 private `V8_NOINLINE V8_PRESERVE_MOST AllocationResult
  /// AllocateRawSlow(size, alignment, origin)` + cc:206-228。
  /// v8: src/heap/main-allocator.cc MainAllocator::AllocateRawSlow
  #[must_use]
  pub fn allocate_raw_slow(
    &mut self,
    _size_in_bytes: usize,
    _alignment: AllocationAlignment,
    _origin: AllocationOrigin,
  ) -> AllocationResult {
    todo!("就地修订: cc:206-228 (TryFreeLast 回收 + 观测器 + 慢路分配)")
  }

  /// 原版 h:286-287 private `EnsureAllocation(SafeHeapObjectSize, alignment, origin)` +
  /// cc:312-330 — 策略分发 + in_gc 判 + GC 后重试环。
  /// v8: src/heap/main-allocator.cc MainAllocator::EnsureAllocation
  #[must_use]
  pub fn ensure_allocation(
    &mut self,
    _size_in_bytes: usize,
    _alignment: AllocationAlignment,
    _origin: AllocationOrigin,
  ) -> bool {
    todo!("就地修订: cc:312-330 (allocator_policy_->EnsureAllocation + 重试)")
  }

  /// 原版 h:221 `V8_INLINE bool TryFreeLast(Address object_address, int object_size)` —
  /// -inl.h 实现体他票 (decrement_top_if_adjacent 承接位已落 LAA)。
  /// v8: src/heap/main-allocator.h MainAllocator::TryFreeLast
  #[must_use]
  pub fn try_free_last(&mut self, _object_address: Address, _object_size: i32) -> bool {
    todo!("就地修订: -inl.h TryFreeLast (pending allocation 判 + LAB 回退)")
  }

  // ── 观测器面 (h:201-210 + 私有 h:289) ──

  /// 原版 h:201 声明 + cc:113-119 `AddAllocationObserver(AllocationObserver*)` —
  /// SupportsAllocationObserver DCHECK 后转 counter (ObserverRef 落地面承接裸指针参)。
  /// v8: src/heap/main-allocator.cc MainAllocator::AddAllocationObserver
  pub fn add_allocation_observer(&mut self, _observer: ObserverRef) {
    todo!("就地修订: cc:113-119")
  }

  /// 原版 h:202 声明 + cc:121-127 `RemoveAllocationObserver(AllocationObserver*)`。
  /// v8: src/heap/main-allocator.cc MainAllocator::RemoveAllocationObserver
  pub fn remove_allocation_observer(&mut self, _observer: ObserverRef) {
    todo!("就地修订: cc:121-127")
  }

  /// 原版 h:203 声明 + cc:129 `PauseAllocationObservers()` (DCHECK(!IsLabValid()))。
  /// v8: src/heap/main-allocator.cc MainAllocator::PauseAllocationObservers
  pub fn pause_allocation_observers(&mut self) {
    // 原版 cc:129: 体仅 `DCHECK(!IsLabValid())` (pause 实际作用位在 PauseScope, 本方法
    // 承接该断言面; release 配置 DCHECK 剥除, 与 memory_allocator.rs DEBUG 先例同法)。
    debug_assert!(!self.is_lab_valid(), "cc:129 DCHECK(!IsLabValid())");
  }

  /// 原版 h:204 声明 + cc:131 `ResumeAllocationObservers()` (DCHECK(!IsLabValid()))。
  /// v8: src/heap/main-allocator.cc MainAllocator::ResumeAllocationObservers
  pub fn resume_allocation_observers(&mut self) {
    // 原版 cc:131: 体仅 `DCHECK(!IsLabValid())`。
    debug_assert!(!self.is_lab_valid(), "cc:131 DCHECK(!IsLabValid())");
  }

  /// 原版 h:206 声明 + cc:133-142 `AdvanceAllocationObservers()`。
  /// v8: src/heap/main-allocator.cc MainAllocator::AdvanceAllocationObservers
  pub fn advance_allocation_observers(&mut self) {
    todo!("就地修订: cc:133-142 (IsLabValid 时按 top-start 推进)")
  }

  /// 原版 h:207-210 声明 + cc:159-204 `InvokeAllocationObservers(soon_object,
  /// size_in_bytes, aligned_size_in_bytes, allocation_size)` —
  /// 观测器步长拆分与 re-pause 主流程 (AllocationResult failure 返回承接面)。
  /// v8: src/heap/main-allocator.cc MainAllocator::InvokeAllocationObservers
  pub fn invoke_allocation_observers(
    &mut self,
    _soon_object: Address,
    _size_in_bytes: usize,
    _aligned_size_in_bytes: usize,
    _allocation_size: usize,
  ) {
    todo!("就地修订: cc:159-204 (counter InvokeAllocationObservers + LAB 重设)")
  }

  /// 原版 h:289 声明 + cc:144-157 `MarkLabStartInitialized()` — 黑分配时给 LAB 起点
  /// 初始化标记 (heap->MarkInitializedObjects 面属他票)。
  /// v8: src/heap/main-allocator.cc MainAllocator::MarkLabStartInitialized
  pub fn mark_lab_start_initialized(&mut self) {
    todo!("就地修订: cc:144-157")
  }

  // ── LAB 位图/标记面 (h:212-216) ──

  /// 原版 h:212 声明 + cc:230-243 `MakeLinearAllocationAreaIterable()`。
  /// v8: src/heap/main-allocator.cc MainAllocator::MakeLinearAllocationAreaIterable
  pub fn make_linear_allocation_area_iterable(&mut self) {
    todo!("就地修订: cc:230-243 (页标记位 + in_gc_for_space 分发)")
  }

  /// 原版 h:214 声明 + cc:245-253 `MarkLinearAllocationAreaBlack()`。
  /// v8: src/heap/main-allocator.cc MainAllocator::MarkLinearAllocationAreaBlack
  pub fn mark_linear_allocation_area_black(&mut self) {
    todo!("就地修订: cc:245-253")
  }

  /// 原版 h:215 声明 + cc:255-262 `UnmarkLinearAllocationArea()`。
  /// v8: src/heap/main-allocator.cc MainAllocator::UnmarkLinearAllocationArea
  pub fn unmark_linear_allocation_area(&mut self) {
    todo!("就地修订: cc:255-262")
  }

  /// 原版 h:216 声明 + cc:264-268 `FreeLinearAllocationAreaAndResetFreeList()`。
  /// v8: src/heap/main-allocator.cc MainAllocator::FreeLinearAllocationAreaAndResetFreeList
  pub fn free_linear_allocation_area_and_reset_free_list(&mut self) {
    todo!("就地修订: cc:264-268 (策略 free + space ResetFreeList)")
  }

  // ── 测试钩子 (h:218/243-245/196-199 后两类) ──

  /// 原版 h:218 声明 + cc:79-91 `AlignTopForTesting(alignment, offset)`。
  /// v8: src/heap/main-allocator.cc MainAllocator::AlignTopForTesting
  #[must_use]
  pub fn align_top_for_testing(
    &mut self,
    _alignment: AllocationAlignment,
    _offset: i32,
  ) -> Address {
    todo!("就地修订: cc:79-91 (CreateFillerObjectAt + IncrementTop)")
  }

  /// 原版 h:243-245 声明 + cc:417-421 `EnsureAllocationForTesting(size, alignment, origin)`。
  /// v8: src/heap/main-allocator.cc MainAllocator::EnsureAllocationForTesting
  #[must_use]
  pub fn ensure_allocation_for_testing(
    &mut self,
    _size_in_bytes: i32,
    _alignment: AllocationAlignment,
    _origin: AllocationOrigin,
  ) -> bool {
    todo!("就地修订: cc:417-421 (转 private EnsureAllocation)")
  }

  // ── LAB 有效性与释放 (h:228/235-237/239-241) ──

  /// 原版 h:223-227 注释 + h:228 声明 + cc:352-392 `ComputeLimit(start, end, min_size)` —
  /// 观测器提前中断位 (Isolate::Current/heap 运行态)。
  /// v8: src/heap/main-allocator.cc MainAllocator::ComputeLimit
  #[must_use]
  pub fn compute_limit(&self, _start: Address, _end: Address, _min_size: usize) -> Address {
    todo!("就地修订: cc:352-392 (观测器 next_bytes 反推限)")
  }

  /// 原版 h:230-232 `#if DEBUG void Verify() const` + cc:394-415 (Rust
  /// #[cfg(debug_assertions)] 承接, memory_allocator DEBUG 先例)。
  /// v8: src/heap/main-allocator.cc MainAllocator::Verify
  #[cfg(debug_assertions)]
  pub fn verify(&self) {
    todo!("就地修订: cc:394-415 (LAB 三元组不变量 + counter 一致性)")
  }

  /// 原版 h:234-237 `V8_INLINE bool IsLabValid() const` (h:234 注释: LAB 在用则真)。
  /// v8: src/heap/main-allocator.h MainAllocator::IsLabValid
  #[must_use]
  pub fn is_lab_valid(&self) -> bool {
    // 原版 h:236 `allocation_info_->top() != kNullAddress` — 转已填 top() 转发。
    self.top() != K_NULL_ADDRESS
  }

  /// 原版 h:239 声明 + cc:332-344 `FreeLinearAllocationArea()` —
  /// 顶对齐 kObjectAlignment + 策略 free + extended_limit 复位。
  /// v8: src/heap/main-allocator.cc MainAllocator::FreeLinearAllocationArea
  pub fn free_linear_allocation_area(&mut self) {
    todo!("就地修订: cc:332-344")
  }

  /// 原版 h:241 声明 + cc:346-350 `ExtendLAB(Address limit)` — DCHECK(supports_extending_lab_)。
  /// v8: src/heap/main-allocator.cc MainAllocator::ExtendLAB
  pub fn extend_lab(&mut self, limit: Address) {
    // 原版 cc:346-347 DCHECK(supports_extending_lab) + DCHECK_LE(limit, extended_limit()),
    // cc:349 `allocation_info().SetLimit(limit)` (转已落地 LAA set_limit)。
    debug_assert!(
      self.supports_extending_lab(),
      "cc:346 DCHECK(supports_extending_lab())"
    );
    debug_assert!(
      limit <= self.extended_limit(),
      "cc:347 DCHECK_LE(limit, extended_limit())"
    );
    // SAFETY: allocation_info_ 由 ctor 保证指向有效 LAA (同 start/top 面)。
    unsafe { (*self.allocation_info_).set_limit(limit) };
  }

  // ── 对齐计算 (h:247-255) ──

  /// 原版 h:247-250 注释 + h:249-250 声明 + cc:433-444 `static int
  /// GetMaximumFillToAlign(alignment)` (最大填充量)。
  /// v8: src/heap/main-allocator.cc MainAllocator::GetMaximumFillToAlign
  #[must_use]
  pub fn get_maximum_fill_to_align(alignment: AllocationAlignment) -> i32 {
    // 原版 cc:433-444: 8GB 压缩堆恒 0; kTaggedAligned 0; 双对齐/双非对齐支
    // kDoubleSize - kTaggedSize。Rust AllocationAlignment 仅三变体, match 穷尽 (无 default
    // UNREACHABLE)。
    if V8_COMPRESS_POINTERS_8GB {
      return 0;
    }
    match alignment {
      AllocationAlignment::TaggedAligned => 0,
      AllocationAlignment::DoubleAligned | AllocationAlignment::DoubleUnaligned => {
        (K_DOUBLE_SIZE - K_TAGGED_SIZE) as i32
      }
    }
  }

  /// 原版 h:252-253 注释 + h:254-255 `V8_INLINE static int GetFillToAlign(
  /// Address, AllocationAlignment)` — 实现体在 main-allocator-inl.h (-inl 面他票)。
  /// v8: src/heap/main-allocator.h MainAllocator::GetFillToAlign
  #[must_use]
  pub fn get_fill_to_align(address: Address, alignment: AllocationAlignment) -> i32 {
    // 原版 -inl.h:95-105: 8GB 压缩堆恒 0; 双对齐且地址带余 → kTaggedSize;
    // 双非对齐且地址对齐 → kDoubleSize - kTaggedSize; 否则 0。纯位运算, 无外部依赖。
    if V8_COMPRESS_POINTERS_8GB {
      return 0;
    }
    if alignment == AllocationAlignment::DoubleAligned && (address & K_DOUBLE_ALIGNMENT_MASK) != 0 {
      return K_TAGGED_SIZE as i32;
    }
    if alignment == AllocationAlignment::DoubleUnaligned && (address & K_DOUBLE_ALIGNMENT_MASK) == 0
    {
      return (K_DOUBLE_SIZE - K_TAGGED_SIZE) as i32;
    }
    0
  }

  // ── 私有判据/访问器 (h:264-336) ──

  /// 原版 h:264 `static constexpr BlackAllocation ComputeBlackAllocation(IsNewGeneration)`
  /// 与 cc:30-40 定义 — h:259-261 三支 (新生代恒关; sticky_mark_bits 旗标恒开; 否则标记期开)。
  /// 差异申报 (模块头): cc:35 `v8_flags.sticky_mark_bits` 全局 → bool 参数注入
  /// (free_list.rs/memory_allocator.rs 先例)。
  /// v8: src/heap/main-allocator.cc MainAllocator::ComputeBlackAllocation
  #[must_use]
  pub fn compute_black_allocation(
    is_new_generation: IsNewGeneration,
    sticky_mark_bits: bool,
  ) -> BlackAllocation {
    // 原版 cc:30-40 三支: 新生代恒关 (kAlwaysDisabled); sticky_mark_bits 全局恒开
    // (kAlwaysEnabled, 差异申报为 bool 参数注入); 否则标记期开 (kEnabledOnMarking)。
    // StrongAlias 内层 bool 读出比较 (IsNewGenerationTag 幻影参无数值, 原版 `== kNewGeneration`)。
    if is_new_generation.into_value() {
      return BlackAllocation::AlwaysDisabled;
    }
    if sticky_mark_bits {
      return BlackAllocation::AlwaysEnabled;
    }
    BlackAllocation::EnabledOnMarking
  }

  /// 原版 h:291 声明 + cc:106-111 `bool IsBlackAllocationEnabled() const` —
  /// kEnabledOnMarking 支走 heap->incremental_marking (他票运行态)。
  /// v8: src/heap/main-allocator.cc MainAllocator::IsBlackAllocationEnabled
  #[must_use]
  pub fn is_black_allocation_enabled(&self) -> bool {
    todo!("就地修订: cc:106-111")
  }

  /// 原版 h:293-295 双载 `LinearAreaOriginalData& linear_area_original_data()` —
  /// const 载承接 (非 const 载同名规则同 allocation_info 对)。
  /// v8: src/heap/main-allocator.h MainAllocator::linear_area_original_data
  #[must_use]
  pub fn linear_area_original_data(&self) -> *const LinearAreaOriginalData {
    // 原版 h:298 `linear_area_original_data_.value()` (const 载) — Option 有值取内部
    // 裸指针, 空则 null (从不解引用)。
    self
      .linear_area_original_data_
      .as_ref()
      .map_or(null(), |data| data as *const LinearAreaOriginalData)
  }

  /// 原版 h:293-294 非 const 载。
  /// v8: src/heap/main-allocator.h MainAllocator::linear_area_original_data
  #[must_use]
  pub fn linear_area_original_data_mut(&mut self) -> *mut LinearAreaOriginalData {
    // 原版 h:294 `linear_area_original_data_.value()` (非 const 载)。
    self
      .linear_area_original_data_
      .as_mut()
      .map_or(null_mut(), |data| data as *mut LinearAreaOriginalData)
  }

  /// 原版 h:301 声明 + cc:423-431 `int ObjectAlignment() const` —
  /// kTrustedSpace/kSharedSpace/kNewSpace/kCode 支表 (identity 消费)。
  /// v8: src/heap/main-allocator.cc MainAllocator::ObjectAlignment
  #[must_use]
  pub fn object_alignment(&self) -> i32 {
    todo!("就地修订: cc:423-431")
  }

  /// 原版 h:303 声明 + cc:446 `AllocationSpace identity() const` — `space_->identity()`。
  /// v8: src/heap/main-allocator.cc MainAllocator::identity
  #[must_use]
  pub fn identity(&self) -> AllocationSpace {
    todo!("就地修订: cc:446 return space_->identity()")
  }

  /// 原版 h:305-307 `bool SupportsAllocationObserver() const` — counter 有值判。
  /// v8: src/heap/main-allocator.h MainAllocator::SupportsAllocationObserver
  #[must_use]
  pub fn supports_allocation_observer(&self) -> bool {
    // 原版 h:306 `allocation_counter_.has_value()`。
    self.allocation_counter_.is_some()
  }

  /// 原版 h:309-311 `bool SupportsPendingAllocation() const` — 原始数据有值判。
  /// v8: src/heap/main-allocator.h MainAllocator::SupportsPendingAllocation
  #[must_use]
  pub fn supports_pending_allocation(&self) -> bool {
    // 原版 h:310 `linear_area_original_data_.has_value()`。
    self.linear_area_original_data_.is_some()
  }

  /// 原版 h:313-314 `bool in_gc() const` (h:313 注释: GC 期间用则真) — local_heap_ 空判。
  /// v8: src/heap/main-allocator.h MainAllocator::in_gc
  #[must_use]
  pub fn in_gc(&self) -> bool {
    // 原版 h:314 `local_heap_ == nullptr`。
    self.local_heap_.is_null()
  }

  /// 原版 h:316-320 注释 + h:320 声明 + cc:452-454 `bool in_gc_for_space() const` —
  /// in_gc && space_heap 正被回收 (Heap::IsMarking 等运行态)。
  /// v8: src/heap/main-allocator.cc MainAllocator::in_gc_for_space
  #[must_use]
  pub fn in_gc_for_space(&self) -> bool {
    todo!("就地修订: cc:452-454")
  }

  /// 原版 h:322 `Address extended_limit() const` — 字段读出。
  /// v8: src/heap/main-allocator.h MainAllocator::extended_limit
  #[must_use]
  pub fn extended_limit(&self) -> Address {
    // 原版 h:322 `return extended_limit_`。
    self.extended_limit_
  }

  /// 原版 h:324 `bool supports_extending_lab() const` — 字段读出。
  /// v8: src/heap/main-allocator.h MainAllocator::supports_extending_lab
  #[must_use]
  pub fn supports_extending_lab(&self) -> bool {
    // 原版 h:324 `return supports_extending_lab_` (MainAllocator 字段读出)。
    self.supports_extending_lab_
  }

  /// 原版 h:326 声明 + cc:448-450 `bool is_main_thread() const` —
  /// `local_heap_->is_main_thread()` (cc:449, LocalHeap 运行态)。
  /// v8: src/heap/main-allocator.cc MainAllocator::is_main_thread
  #[must_use]
  pub fn is_main_thread(&self) -> bool {
    todo!("就地修订: cc:448-450")
  }

  /// 原版 h:328 `LocalHeap* local_heap() const`。
  /// v8: src/heap/main-allocator.h MainAllocator::local_heap
  #[must_use]
  pub fn local_heap(&self) -> *mut LocalHeap {
    // 原版 h:328 `return local_heap_`。
    self.local_heap_
  }

  /// 原版 h:330-332 注释 + `Heap* isolate_heap() const`。
  /// v8: src/heap/main-allocator.h MainAllocator::isolate_heap
  #[must_use]
  pub fn isolate_heap(&self) -> *mut Heap {
    // 原版 h:332 `return isolate_heap_`。
    self.isolate_heap_
  }

  /// 原版 h:334-336 注释 (h:334-335: 共享空间在工作 isolate 可与 isolate_heap 不同) +
  /// h:336 声明 + cc:456 `Heap* space_heap() const` — `space_->heap()`。
  /// v8: src/heap/main-allocator.cc MainAllocator::space_heap
  #[must_use]
  pub fn space_heap(&self) -> *mut Heap {
    todo!("就地修订: cc:456 return space_->heap()")
  }
}

// ══════════════════════════════════════════════════════════════════════════
// HC-2c 推迟台账 (填体票消费; 双行号 + 原因)
// ══════════════════════════════════════════════════════════════════════════
// 1. -inl 实现面 — main-allocator-inl.h (110 行): AllocateRaw (h:192-194 声明)、
//    TryFreeLast (h:221)、GetFillToAlign (h:254-255)、AllocateFastUnaligned
//    (h:268-269)、AllocateFastAligned (h:275-278) 的 inline 体。原因: -inl 面按票
//    口径 (本票仅 h/.cc 签名骨架) 归 HC-2c 填体票; 本文件已按 h 声明面落签名。
// 2. 空间实体族 — LocalHeap/SpaceWithLinearArea/PagedNewSpace/PagedSpaceBase/
//    SemiSpaceNewSpace/YoungPendingAllocations 方法面 (spaces.h/new-spaces.h/
//    paged-spaces.h/pending-allocations.h) 未落地 → CreateAllocatorPolicy (cc:51)、
//    space->identity() (cc:55/446)、页取还 (cc:594-648 RefillLab 族) 等消费位全部
//    留 todo; HC-4 spaces 族落地后衔接。
// 3. 运行态族 — Isolate::Current/Heap::deserialization/VM 状态作用域/
//    CodeFlushManager/ConcurrentSweeping/TaskScheduler (cc:159-204/589-636/
//    801-834 等) 属他票 → InvokeAllocationObservers/WaitForSweepingForAllocation/
//    ContributeToSweeping 填体依赖登记。
// 4. gc-tracer/cancelable-task include (h:14/16) — 本票签名面零消费
//    (AllocationResult/GC_PROTO_TYPE 等未入签名), 不落空依赖面 (memory_allocator
//    h:19-22 未落地 include 同型申报)。
// 5. TimeDelta 依赖 — cc:332-344 FreeLinearAllocationArea 的 post task 延迟位若引
//    入填体票再消费 (本文件 import 位见 use; 若填体票不用则移除)。
