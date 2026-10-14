// 由 tool/src/objects/backing-store.h.js 从 v8/src/objects/backing-store.{h,cc} 生成; 修订就地改, 复跑覆盖
// BackingStore 枚举与私有标志位
//! BackingStore: ArrayBuffer/WasmMemory 后备存储描述。
//! .cc 语义实现面 (分配/GC 注册) 登记跳过。

/// Wasm 内存类别 (原版 enum class WasmMemoryFlag : uint8_t)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum WasmMemoryFlag {
  /// kNotWasm
  Notwasm = 0,
  /// kWasmMemory32
  Wasmmemory32 = 1,
  /// kWasmMemory64
  Wasmmemory64 = 2,
}

/// 扩容结果 (原版 enum ResizeOrGrowResult)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ResizeOrGrowResult {
  /// kSuccess
  Success = 0,
  /// kFailure
  Failure = 1,
  /// kRace
  Race = 2,
}

/// BackingStore::kTypeID 锚值 (ManagedTypeId::kBackingStore)。
pub const TYPE_ID: &str = "kBackingStore";

/// 私有标志位 (原版 enum Flag, 顺序位; 存于 atomic flags_ 字段)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum Flag {
  /// kIsShared
  Isshared = 1 << 0,
  /// kIsResizableByJs
  Isresizablebyjs = 1 << 1,
  /// kIsImmutable
  Isimmutable = 1 << 2,
  /// kIsWasmMemory
  Iswasmmemory = 1 << 3,
  /// kIsWasmMemory64
  Iswasmmemory64 = 1 << 4,
  /// kHoldsSharedPtrToAllocater
  Holdssharedptrtoallocater = 1 << 5,
  /// kHasGuardRegions
  Hasguardregions = 1 << 6,
  /// kGloballyRegistered
  Globallyregistered = 1 << 7,
  /// kCustomDeleter
  Customdeleter = 1 << 8,
  /// kEmptyDeleter
  Emptydeleter = 1 << 9,
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(WasmMemoryFlag::WasmMemory64 as u8 == 2);
  assert!(ResizeOrGrowResult::Race as u8 == 2);
  assert!(Flag::EmptyDeleter as u32 == 1 << 9);
};
