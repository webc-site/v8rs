// 由 tool/src/objects/backing-store.cc.js 从 v8/src/objects/backing-store.cc 生成; 修订就地改, 复跑覆盖
// 席位表差集: .cc 分配状态/保留区公式 + 方法语义体 (布局归 backing-store.h)
//! BackingStore .cc 差集 (ArrayBuffer/WasmMemory 后备存储实现面)。
//!
//! 布局与类型常量归既有 backing-store.h 产物; 本文件登记分配结果枚举、
//! wasm 保留区公式与 .cc 方法语义体。
//!
//! 依赖 Isolate / PageAllocator / Wasm 引擎 / cppgc 的方法体走推迟台账
//! (见 readme/zh.md「推迟台账」节): allocate / try_allocate_* / set_allocator_from_isolate /
//! allocate_wasm_memory / copy_wasm_memory / grow_wasm_memory_in_place /
//! broadcast_shared_wasm_memory_grow / remove_shared_wasm_memory_objects /
//! update_shared_wasm_memory_objects / resize_in_place / grow_in_place /
//! get_v8_api_array_buffer_allocator / get_shared_wasm_memory_data。

use std::{
  ffi::c_void,
  ptr::NonNull,
  sync::atomic::{AtomicU16, AtomicU32, AtomicUsize, Ordering},
};

// ── AllocationStatus (匿名 ns, UMA wasm_memory_allocation_result 上报) ──
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AllocationStatus {
  /// kSuccess
  Success = 0,
  /// kSuccessAfterRetry
  SuccessAfterRetry = 1,
  /// kAddressSpaceLimitReachedFailure
  AddressSpaceLimitReachedFailure = 2,
  /// kOtherFailure
  OtherFailure = 3,
}

impl AllocationStatus {
  /// 状态总数
  pub const COUNT: usize = 4;
}

// ── WasmMemoryFlag (backing-store.h) ──
/// 原版 `enum class WasmMemoryFlag : uint8_t`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum WasmMemoryFlag {
  /// kNotWasm
  #[default]
  NotWasm = 0,
  /// kWasmMemory32
  WasmMemory32 = 1,
  /// kWasmMemory64
  WasmMemory64 = 2,
}

// ── wasm 内存保留区 (64 位 + V8_ENABLE_WEBASSEMBLY 分支) ──
/// 32 位 wasm 内存的完整 guard 区大小 (原版 kFullGuardSize32 = uint64{8} * GB)。
pub const K_FULL_GUARD_SIZE32: usize = 8 * 1024 * 1024 * 1024;

/// backing_store_id 计数器起始值 (原版 next_backing_store_id_{1})。
pub const K_NEXT_BACKING_STORE_ID_INIT: u32 = 1;

/// wasm 页大小 (kWasmPageSize = 0x10000)。
pub const K_WASM_PAGE_SIZE: usize = 0x10000;

/// 64 位实现支持的最大 wasm memory64 页数 (kV8MaxWasmMemory64Pages)。
pub const K_MAX_WASM_MEMORY64_PAGES: usize = 262_144; // 16 GiB

/// kMaxMemory64Size = 页数 × 页大小。
pub const K_MAX_MEMORY64_SIZE: usize = K_MAX_WASM_MEMORY64_PAGES * K_WASM_PAGE_SIZE;

// ── Flag 位标 (backing-store.h private enum Flag, 10 位, EnumSet<Flag,uint16_t>) ──
/// Flag 位标; 每成员编码 `1u16 << index`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum Flag {
  IsShared = 1 << 0,
  IsResizableByJs = 1 << 1,
  IsImmutable = 1 << 2,
  IsWasmMemory = 1 << 3,
  IsWasmMemory64 = 1 << 4,
  HoldsSharedPtrToAllocator = 1 << 5,
  HasGuardRegions = 1 << 6,
  GloballyRegistered = 1 << 7,
  CustomDeleter = 1 << 8,
  EmptyDeleter = 1 << 9,
}

// ── DeleterCallback (v8::BackingStore::DeleterCallback) ──
/// 原版签名 `void (*)(void* data, size_t length, void* deleter_data)`。
pub type DeleterCallback = unsafe extern "C" fn(*mut c_void, usize, *mut c_void);

// 全局 id 计数器 (原版 std::atomic<uint32_t> next_backing_store_id_{1})。
static NEXT_BACKING_STORE_ID: AtomicU32 = AtomicU32::new(K_NEXT_BACKING_STORE_ID_INIT);

/// 原版 BackingStore 值语义面 (无 heap-object 侧的沙箱/页分配器接线)。
///
/// buffer_start 空态用 `Option<NonNull<c_void>>::None` 承接 (原 nullptr)。
pub struct BackingStore {
  /// 原版 void* buffer_start_。
  buffer_start: Option<NonNull<c_void>>,
  /// 原版 std::atomic<size_t> byte_length_ (shared wasm 直读)。
  byte_length: AtomicUsize,
  /// 原版 size_t max_byte_length_。
  max_byte_length: usize,
  /// 原版 size_t byte_capacity_。
  byte_capacity: usize,
  /// 原版 const uint32_t id_ (fetch_add)。
  id: u32,
  /// 原版 std::atomic<base::EnumSet<Flag, uint16_t>> flags_ (relaxed 序)。
  flags: AtomicU16,
  /// 原版 TypeSpecificData 联合体 — 本态仅承接 wrap 路径的 deleter 面。
  deleter: Option<DeleterCallback>,
  deleter_data: Option<NonNull<c_void>>,
}

/// BackingStore 构造入参配置 (解耦过多形参, 消除 clippy::too_many_arguments)
#[derive(Debug, Clone, Copy, Default)]
pub struct BackingStoreInit {
  pub buffer_start: Option<NonNull<c_void>>,
  pub byte_length: usize,
  pub max_byte_length: usize,
  pub byte_capacity: usize,
  pub shared: bool,
  pub resizable: bool,
  pub immutable: bool,
  pub wasm_memory: WasmMemoryFlag,
  pub has_guard_regions: bool,
  pub custom_deleter: bool,
  pub empty_deleter: bool,
}

impl BackingStore {
  /// v8: src/objects/backing-store.cc BackingStore::BackingStore (private ctor)。
  ///
  /// 字段初始化 + id_ 计数 + flags 落位 (原版 EnumSet 语义 → u16 位掩码)。
  /// 原版 DCHECK/CHECK 用 debug_assert 承接。
  /// Rust 侧无 friend 机制, 该 ctor 对外可见; V8 C++ 通过 Allocate/Wrap/Empty 工厂
  /// 间接构造的语义面本态同样提供。
  pub fn new(init: BackingStoreInit) -> Self {
    let BackingStoreInit {
      buffer_start,
      byte_length,
      max_byte_length,
      byte_capacity,
      shared,
      resizable,
      immutable,
      wasm_memory,
      has_guard_regions,
      custom_deleter,
      empty_deleter,
    } = init;
    let is_wasm_memory = wasm_memory != WasmMemoryFlag::NotWasm;
    let is_wasm_memory64 = wasm_memory == WasmMemoryFlag::WasmMemory64;

    // DCHECK_IMPLIES(is_wasm_memory64, is_wasm_memory);
    debug_assert!(!is_wasm_memory64 || is_wasm_memory);
    // DCHECK_IMPLIES(has_guard_regions, is_wasm_memory);
    debug_assert!(!has_guard_regions || is_wasm_memory);
    // DCHECK_IMPLIES(is_wasm_memory, !resizable);
    debug_assert!(!(is_wasm_memory && resizable));
    // DCHECK_IMPLIES(resizable, !custom_deleter);
    debug_assert!(!(resizable && custom_deleter));
    // DCHECK_IMPLIES(!is_wasm_memory && !resizable, byte_length_ == max_byte_length_);
    debug_assert!(is_wasm_memory || resizable || byte_length == max_byte_length);
    // DCHECK_GE(max_byte_length, byte_length);
    debug_assert!(max_byte_length >= byte_length);
    // DCHECK_GE(byte_capacity, max_byte_length);
    debug_assert!(byte_capacity >= max_byte_length);
    // CHECK_IMPLIES(is_wasm_memory, byte_capacity != 0);
    debug_assert!(!is_wasm_memory || byte_capacity != 0);

    // flags 落位 (原版 EnumSet Add)。
    let mut bits: u16 = 0;
    if shared {
      bits |= Flag::IsShared as u16;
    }
    if resizable {
      bits |= Flag::IsResizableByJs as u16;
    }
    if immutable {
      bits |= Flag::IsImmutable as u16;
    }
    if is_wasm_memory {
      bits |= Flag::IsWasmMemory as u16;
    }
    if is_wasm_memory64 {
      bits |= Flag::IsWasmMemory64 as u16;
    }
    if has_guard_regions {
      bits |= Flag::HasGuardRegions as u16;
    }
    if custom_deleter {
      bits |= Flag::CustomDeleter as u16;
    }
    if empty_deleter {
      bits |= Flag::EmptyDeleter as u16;
    }

    // id_(next_backing_store_id_.fetch_add(1))
    let id = NEXT_BACKING_STORE_ID.fetch_add(1, Ordering::Relaxed);

    Self {
      buffer_start,
      byte_length: AtomicUsize::new(byte_length),
      max_byte_length,
      byte_capacity,
      id,
      flags: AtomicU16::new(bits),
      deleter: None,
      deleter_data: None,
    }
  }

  /// v8: src/objects/backing-store.cc BackingStore::GetWasmReservationSize。
  ///
  /// 64 位 + wasm 分支的完整公式 (非 64 位或未开 wasm 走 byte_capacity 早返)。
  /// ARM64 分支额外保留一分配页, 简化边界检查。
  pub fn get_wasm_reservation_size(
    has_guard_regions: bool,
    byte_capacity: usize,
    wasm_memory: WasmMemoryFlag,
    allocate_page_size: usize,
  ) -> usize {
    // #if V8_TARGET_ARCH_64_BIT && V8_ENABLE_WEBASSEMBLY
    let is_wasm_memory64 = wasm_memory == WasmMemoryFlag::WasmMemory64;
    if has_guard_regions {
      if is_wasm_memory64 {
        // DCHECK_LE(byte_capacity, wasm::kMaxMemory64Size);
        debug_assert!(byte_capacity <= K_MAX_MEMORY64_SIZE);
        // V8_TARGET_ARCH_ARM64: kMaxMemory64Size + AllocatePageSize()
        // 非 arm64: kMaxMemory64Size (无额外 guard 页)
        return K_MAX_MEMORY64_SIZE + allocate_page_size;
      } else {
        // static_assert(kFullGuardSize32 >= size_t{4} * GB);
        // DCHECK_LE(byte_capacity, size_t{4} * GB);
        debug_assert!(byte_capacity <= 4 * 1024 * 1024 * 1024);
        return K_FULL_GUARD_SIZE32;
      }
    }
    // #else DCHECK(!has_guard_regions);
    // #endif return byte_capacity
    byte_capacity
    // NOTE: 非 64 位或未开 wasm 时, has_guard_regions 应为 false (原版 DCHECK)。
  }

  /// v8: src/objects/backing-store.cc BackingStore::WrapAllocation。
  ///
  /// `deleter = None` 承接原版 `v8::BackingStore::EmptyDeleter` 语义位 (fn 指针
  /// 等式在 Rust 侧不保证唯一, 用 Option 显式表达空 deleter); `Some(cb)` 走
  /// 自定义路径, `custom_deleter` 位仍置 (与原版 WrapAllocation 常量传参一致)。
  pub fn wrap_allocation(
    allocation_base: Option<NonNull<c_void>>,
    allocation_length: usize,
    deleter: Option<DeleterCallback>,
    deleter_data: Option<NonNull<c_void>>,
    shared: bool,
  ) -> Self {
    let is_empty_deleter = deleter.is_none();

    let mut result = Self::new(BackingStoreInit {
      buffer_start: allocation_base,
      byte_length: allocation_length,
      max_byte_length: allocation_length,
      byte_capacity: allocation_length,
      shared,
      resizable: false,
      immutable: false,
      wasm_memory: WasmMemoryFlag::NotWasm,
      has_guard_regions: false,
      custom_deleter: true,
      empty_deleter: is_empty_deleter,
    });
    // type_specific_data_.deleter = {deleter, deleter_data};
    result.deleter = deleter;
    result.deleter_data = deleter_data;
    result
  }

  /// v8: src/objects/backing-store.cc BackingStore::EmptyBackingStore。
  #[inline]
  pub fn empty_backing_store(shared: bool) -> Self {
    Self::new(BackingStoreInit {
      shared,
      ..Default::default()
    })
  }

  /// v8: src/objects/backing-store.cc BackingStore::MakeWasmMemoryResizableByJS。
  ///
  /// 只对非 shared wasm 内存生效; 位标翻转。
  pub fn make_wasm_memory_resizable_by_js(&mut self, resizable: bool) {
    // DCHECK(is_wasm_memory());
    debug_assert!(self.is_wasm_memory());
    // DCHECK(!is_shared());
    debug_assert!(!self.is_shared());
    if resizable {
      self.set_flag(Flag::IsResizableByJs);
    } else {
      self.clear_flag(Flag::IsResizableByJs);
    }
  }

  /// v8: src/objects/backing-store.h BackingStore::{has_flag,set_flag,clear_flag}
  ///
  /// EnumSet<uint16_t> 语义承接 (原版 relaxed 序原子 CAS 环)。
  #[inline]
  fn has_flag(&self, flag: Flag) -> bool {
    let bits = self.flags.load(Ordering::Relaxed);
    bits & (flag as u16) != 0
  }

  #[inline]
  fn set_flag(&self, flag: Flag) {
    // 单线程语义等价: fetch_or。
    self.flags.fetch_or(flag as u16, Ordering::Relaxed);
  }

  #[inline]
  fn clear_flag(&self, flag: Flag) {
    self.flags.fetch_and(!(flag as u16), Ordering::Relaxed);
  }

  // ── 访问器 (与 backing-store.h inline 面 1:1) ──
  #[inline]
  #[must_use]
  pub fn is_shared(&self) -> bool {
    self.has_flag(Flag::IsShared)
  }

  #[inline]
  #[must_use]
  pub fn is_resizable_by_js(&self) -> bool {
    self.has_flag(Flag::IsResizableByJs)
  }

  #[inline]
  #[must_use]
  pub fn is_immutable(&self) -> bool {
    self.has_flag(Flag::IsImmutable)
  }

  #[inline]
  #[must_use]
  pub fn is_wasm_memory(&self) -> bool {
    self.has_flag(Flag::IsWasmMemory)
  }

  #[inline]
  #[must_use]
  pub fn is_wasm_memory64(&self) -> bool {
    self.has_flag(Flag::IsWasmMemory64)
  }

  #[inline]
  #[must_use]
  pub fn has_guard_regions(&self) -> bool {
    self.has_flag(Flag::HasGuardRegions)
  }

  #[inline]
  #[must_use]
  pub fn wasm_memory_flag(&self) -> WasmMemoryFlag {
    if self.is_wasm_memory64() {
      WasmMemoryFlag::WasmMemory64
    } else if self.is_wasm_memory() {
      WasmMemoryFlag::WasmMemory32
    } else {
      WasmMemoryFlag::NotWasm
    }
  }

  #[inline]
  #[must_use]
  pub fn byte_length(&self) -> usize {
    self.byte_length.load(Ordering::Relaxed)
  }

  #[inline]
  #[must_use]
  pub fn max_byte_length(&self) -> usize {
    self.max_byte_length
  }

  #[inline]
  #[must_use]
  pub fn byte_capacity(&self) -> usize {
    self.byte_capacity
  }

  #[inline]
  #[must_use]
  pub fn id(&self) -> u32 {
    self.id
  }

  #[inline]
  #[must_use]
  pub fn is_empty(&self) -> bool {
    // DCHECK_GE(byte_capacity_, byte_length_)
    debug_assert!(self.byte_capacity >= self.byte_length());
    self.byte_capacity == 0
  }

  /// v8: src/objects/backing-store.h BackingStore::PerIsolateAccountingLength。
  ///
  /// 非 shared/非 wasm/非 empty-deleter 时按 byte_length 计入外部内存压力。
  #[inline]
  #[must_use]
  pub fn per_isolate_accounting_length(&self) -> usize {
    if self.has_flag(Flag::IsShared) {
      return 0;
    }
    if self.is_wasm_memory() {
      return 0;
    }
    if self.has_flag(Flag::EmptyDeleter) {
      return 0;
    }
    self.byte_length()
  }

  /// v8: include/v8-array-buffer.h v8::BackingStore::EmptyDeleter。
  ///
  /// 空 deleter 桩 (no-op), 由 WrapAllocation 通过 fn 指针等式识别。
  ///
  /// # Safety
  /// 与 `DeleterCallback` ABI 一致 (unsafe extern "C"), 空实现不解引用参数。
  pub unsafe extern "C" fn empty_deleter(
    _data: *mut c_void,
    _length: usize,
    _deleter_data: *mut c_void,
  ) {
  }

  // 保留 buffer_start 读面 (原 nullptr → EmptyBackingStoreBuffer, 由 sandbox 承接, 此处透出原始值)
  #[inline]
  #[must_use]
  pub fn raw_buffer_start(&self) -> Option<NonNull<c_void>> {
    self.buffer_start
  }
}

// ── 深水面推迟 (依赖 Isolate/PageAllocator/Wasm 引擎/cppgc) ──
// 语义锚保留原函数名, 台账见 readme/zh.md「推迟台账」节。

/// v8: src/objects/backing-store.cc BackingStore::Allocate。
pub fn allocate() {
  todo!(
    "BackingStore::Allocate: 依赖 Isolate (array_buffer_allocator/counters/AllocateExternalBackingStore) + Sandbox 边界校验; 解锁 v8-execution-isolate + v8-sandbox + v8-heap-core"
  )
}

/// v8: src/objects/backing-store.cc BackingStore::SetAllocatorFromIsolate。
pub fn set_allocator_from_isolate() {
  todo!(
    "BackingStore::SetAllocatorFromIsolate: 依赖 Isolate::array_buffer_allocator_shared + v8::ArrayBuffer::Allocator + 联合体 placement new; 解锁 v8-execution-isolate + v8-api"
  )
}

/// v8: src/objects/backing-store.cc BackingStore::TryAllocateAndPartiallyCommitMemory。
pub fn try_allocate_and_partially_commit_memory() {
  todo!(
    "BackingStore::TryAllocateAndPartiallyCommitMemory: 依赖 Isolate + PageAllocator + Heap::allocator::RetryCustomAllocate + Sandbox + GC 触发; 解锁 v8-execution-isolate + v8-heap-core + v8-sandbox"
  )
}

/// v8: src/objects/backing-store.cc BackingStore::AllocateWasmMemory。
pub fn allocate_wasm_memory() {
  todo!(
    "BackingStore::AllocateWasmMemory: 依赖 wasm engine + Isolate + 共享内存对象表; 解锁 v8-wasm-core + v8-execution-isolate"
  )
}

/// v8: src/objects/backing-store.cc BackingStore::CopyWasmMemory。
pub fn copy_wasm_memory() {
  todo!("BackingStore::CopyWasmMemory: 依赖 AllocateWasmMemory 递归; 解锁 v8-wasm-core")
}

/// v8: src/objects/backing-store.cc BackingStore::GrowWasmMemoryInPlace。
pub fn grow_wasm_memory_in_place() {
  todo!(
    "BackingStore::GrowWasmMemoryInPlace: 依赖 PageAllocator + Isolate + WasmEngine + 原子 CAS + shared memory grow; 解锁 v8-wasm-core + v8-execution-isolate + v8-base-platform"
  )
}

/// v8: src/objects/backing-store.cc BackingStore::BroadcastSharedWasmMemoryGrow。
pub fn broadcast_shared_wasm_memory_grow() {
  todo!(
    "BackingStore::BroadcastSharedWasmMemoryGrow: 依赖 Isolate + SharedWasmMemoryData::isolates_ + WasmMemoryObject Handle; 解锁 v8-wasm-core + v8-execution-isolate + v8-handles"
  )
}

/// v8: src/objects/backing-store.cc BackingStore::RemoveSharedWasmMemoryObjects。
pub fn remove_shared_wasm_memory_objects() {
  todo!(
    "BackingStore::RemoveSharedWasmMemoryObjects: 依赖 Isolate + GlobalBackingStoreRegistry (mutex + unordered_map); 解锁 v8-wasm-core + v8-execution-isolate"
  )
}

/// v8: src/objects/backing-store.cc BackingStore::UpdateSharedWasmMemoryObjects。
pub fn update_shared_wasm_memory_objects() {
  todo!(
    "BackingStore::UpdateSharedWasmMemoryObjects: 依赖 Isolate + GlobalBackingStoreRegistry + Handle 遍历; 解锁 v8-wasm-core + v8-execution-isolate + v8-handles"
  )
}

/// v8: src/objects/backing-store.cc BackingStore::ResizeInPlace。
pub fn resize_in_place() {
  todo!(
    "BackingStore::ResizeInPlace: 依赖 PageAllocator + JSArrayBuffer::kMaxByteLength + memset + RoundUpToPageSize; 解锁 v8-objects-js-core + v8-base-platform + v8-execution-isolate"
  )
}

/// v8: src/objects/backing-store.cc BackingStore::GrowInPlace。
pub fn grow_in_place() {
  todo!(
    "BackingStore::GrowInPlace: 依赖 PageAllocator + 原子 CAS + memset + JSArrayBuffer::kMaxByteLength; 解锁 v8-objects-js-core + v8-base-platform + v8-execution-isolate"
  )
}

/// v8: src/objects/backing-store.cc BackingStore::get_v8_api_array_buffer_allocator。
pub fn get_v8_api_array_buffer_allocator() {
  todo!(
    "BackingStore::get_v8_api_array_buffer_allocator: 依赖 v8::ArrayBuffer::Allocator + TypeSpecificData 联合体 shared_ptr 分支; 解锁 v8-api + v8-execution-isolate"
  )
}

/// v8: src/objects/backing-store.cc BackingStore::get_shared_wasm_memory_data。
pub fn get_shared_wasm_memory_data() {
  todo!(
    "BackingStore::get_shared_wasm_memory_data: 依赖 SharedWasmMemoryData + Isolate 集 (vector<Isolate*>); 解锁 v8-wasm-core + v8-execution-isolate"
  )
}

// ── 提取不变式断言 ──
const _: () = {
  assert!(AllocationStatus::COUNT == 4);
  assert!(AllocationStatus::Success as u8 == 0);
  assert!(AllocationStatus::OtherFailure as u8 == 3);
  assert!(K_FULL_GUARD_SIZE32 >= 4 * 1024 * 1024 * 1024);
  assert!(K_NEXT_BACKING_STORE_ID_INIT == 1);
  assert!(WasmMemoryFlag::NotWasm as u8 == 0);
  assert!(WasmMemoryFlag::WasmMemory64 as u8 == 2);
  assert!(Flag::IsShared as u16 == 1);
  assert!(Flag::EmptyDeleter as u16 == 1 << 9);
  // kMaxMemory64Size = 262144 * 65536 = 2^34 = 16 GiB
  assert!(K_MAX_MEMORY64_SIZE == 17_179_869_184);
};
