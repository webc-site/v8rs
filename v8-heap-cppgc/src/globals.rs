// 由 tool/src/heap/cppgc-internal/globals.js 从 v8/src/heap/cppgc-internal/globals.h 生成; 修订就地改, 复跑覆盖
// cppgc 内部全局常量; 目标配置 V8_HOST_ARCH_64_BIT + !CPPGC_POINTER_COMPRESSION + CPPGC_CAGED_HEAP

/// v8: src/heap/cppgc-internal/globals.h:17 Address (uint8_t*)
pub type Address = *mut u8;
/// v8: src/heap/cppgc-internal/globals.h:18 ConstAddress (const uint8_t*)
pub type ConstAddress = *const u8;

/// v8: src/heap/cppgc-internal/globals.h:20-22 kKB / kMB / kGB
pub const K_KB: usize = 1024;
/// v8: src/heap/cppgc-internal/globals.h:21 kMB
pub const K_MB: usize = K_KB * 1024;
/// v8: src/heap/cppgc-internal/globals.h:22 kGB
pub const K_GB: usize = K_MB * 1024;

/// v8: src/heap/cppgc-internal/globals.h:25 AccessMode
/// 原版模板参数 `template <AccessMode mode>` → Rust 运行期参数 (同 v8-heap-base::AccessMode 先例);
/// 判别值按原版 `enum class AccessMode : uint8_t { kNonAtomic, kAtomic }` 保真。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum AccessMode {
  /// 原版 kNonAtomic: 单线程/已同步语境。
  NonAtomic = 0,
  /// 原版 kAtomic: 并发读 (原子原语 + acquire/release)。
  Atomic = 1,
}

/// v8: src/heap/cppgc-internal/globals.h:35 kAllocationGranularity (V8_HOST_ARCH_64_BIT = alignof(double))
pub const K_ALLOCATION_GRANULARITY: usize = 8;
/// v8: src/heap/cppgc-internal/globals.h:39 kAllocationMask
pub const K_ALLOCATION_MASK: usize = K_ALLOCATION_GRANULARITY - 1;

/// v8: src/heap/cppgc-internal/globals.h:41 kPageSizeLog2
pub const K_PAGE_SIZE_LOG2: usize = 17;
/// v8: src/heap/cppgc-internal/globals.h:42 kPageSize
pub const K_PAGE_SIZE: usize = 1 << K_PAGE_SIZE_LOG2;
/// v8: src/heap/cppgc-internal/globals.h:43 kPageOffsetMask
pub const K_PAGE_OFFSET_MASK: usize = K_PAGE_SIZE - 1;
/// v8: src/heap/cppgc-internal/globals.h:44 kPageBaseMask
pub const K_PAGE_BASE_MASK: usize = !K_PAGE_OFFSET_MASK;

/// v8: src/heap/cppgc-internal/globals.h:46 kLargeObjectSizeThreshold
pub const K_LARGE_OBJECT_SIZE_THRESHOLD: usize = K_PAGE_SIZE / 2;

/// v8: src/heap/cppgc-internal/globals.h:48 kFreeListGCInfoIndex (GCInfoIndex = uint16_t)
pub const K_FREE_LIST_GC_INFO_INDEX: u16 = 0;
/// v8: src/heap/cppgc-internal/globals.h:49 kFreeListEntrySize = 2 * sizeof(uintptr_t)
pub const K_FREE_LIST_ENTRY_SIZE: usize = 2 * size_of::<usize>();

/// v8: src/heap/cppgc-internal/globals.h:54 kSlotSize (!CPPGC_POINTER_COMPRESSION = sizeof(uintptr_t))
pub const K_SLOT_SIZE: usize = size_of::<usize>();
