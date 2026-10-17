//! 分配结果与分配统计。
//! 对应原版 v8/src/heap/allocation-result.h (76 行) + allocation-stats.h (120 行)。
//!
//! 落地范围 (与 tool/src/heap/allocation-result.h.js + allocation-stats.h.js 锚逐项对齐):
//! - [`AllocationOrigin`] 6 项枚举 (原版 allocation-result.h:15-22), 逐名逐值逐行承接。
//! - [`AllocationResult`] 10 方法 + 字段 + sizeof 静态断言 (原版 h:26-71)。
//! - [`AllocationStats`] 11 方法 + 4 字段 + DEBUG 账目 (原版 allocation-stats.h:24-115)。
//!
//! 票面差异申报 (以本仓源实态为准, 见主控汇报):
//! 本仓 allocation-result.h 为 76 行旧版, `AllocationResult` **仅持 `object_` 字段**,
//! 无 `origin`/`value`/`success` 成员 (新版 V8 才引入); `AllocationOrigin` 为独立枚举,
//! 未挂载到 AllocationResult。票面所述 "value/success/origin 面" 在源中不存在, 故不落地。

use core::{
  mem::size_of,
  sync::atomic::{AtomicUsize, Ordering},
};
#[cfg(debug_assertions)]
use std::collections::HashMap;

use v8_base::logging::check_with_msg;
use v8_common::{
  Address, K_OBJECT_ALIGNMENT_8GB_HEAP, K_SYSTEM_POINTER_SIZE, V8_COMPRESS_POINTERS_8GB,
};
use v8_objects_core::{heap_object::HeapObject, tagged::Tagged};

/// 分配来源。原版 allocation-result.h:15-22 `enum class AllocationOrigin`。
///
/// C++ `enum class` 默认底层类型 `int` → `#[repr(i32)]`; 枚举值即 ABI, 逐值承接。
/// `K_FIRST/K_LAST/K_NUMBER_OF` 三项在原版是别名/派生常量 (非独立状态), Rust 以关联常量承接
/// (Rust 枚举不允许重复判别式, 但常量别名语义等价)。变体去 k 前缀 PascalCase 承接
/// (同 v8-common LanguageMode 先例), 值不变 ABI 等价。
#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AllocationOrigin {
  /// 由生成代码分配 (原版 h:16 kGeneratedCode, 值 0)
  GeneratedCode = 0,
  /// 由运行时分配 (原版 h:17 kRuntime, 值 1)
  Runtime = 1,
  /// 由 GC 分配 (原版 h:18 kGC, 值 2)
  Gc = 2,
}

impl AllocationOrigin {
  /// 首个来源 = kGeneratedCode (原版 h:19)
  pub const K_FIRST_ALLOCATION_ORIGIN: Self = Self::GeneratedCode;
  /// 末个来源 = kGC (原版 h:20)
  pub const K_LAST_ALLOCATION_ORIGIN: Self = Self::Gc;
  /// 来源总数 = kLastAllocationOrigin + 1 (原版 h:21, 值 3)
  pub const K_NUMBER_OF_ALLOCATION_ORIGINS: usize = Self::K_LAST_ALLOCATION_ORIGIN as usize + 1;
}

/// 一次分配尝试的结果: 要么是成功分配 (可转为对象), 要么是失败。
/// 原版 allocation-result.h:26-69 `class AllocationResult final`。
///
/// 单字段 `object_: Tagged<HeapObject>` (原版 h:68); 布局静态断言 sizeof==kSystemPointerSize
/// (原版 h:71), 由 [`const@SIZE_OK`] 编译期恒检。
#[derive(Copy, Clone, Debug)]
pub struct AllocationResult {
  /// 分配出的对象 (原版 h:68 `Tagged<HeapObject> object_`)。null 表示失败 (h:38)。
  object_: Tagged<HeapObject>,
}

// 原版 h:71 `static_assert(sizeof(AllocationResult) == kSystemPointerSize)`
const _: () = assert!(size_of::<AllocationResult>() == K_SYSTEM_POINTER_SIZE);

impl AllocationResult {
  /// 原版 h:28 `static AllocationResult Failure() { return AllocationResult(); }`
  /// 空 ctor 即失败结果。
  #[inline]
  pub fn failure() -> Self {
    Self::default()
  }

  /// 原版 h:30-32 `static AllocationResult FromObject(Tagged<HeapObject> heap_object)`
  #[inline]
  pub fn from_object(heap_object: Tagged<HeapObject>) -> Self {
    Self {
      object_: heap_object,
    }
  }

  /// 原版 h:36 `AllocationResult() = default` — 空 ctor 造失败结果 (object_ 为 null)。
  /// 调用点据请求决定触发哪个 GC。
  #[inline]
  fn null() -> Self {
    Self {
      object_: Tagged::NULL,
    }
  }

  /// 原版 h:38 `bool IsFailure() const { return object_.is_null(); }`
  #[inline]
  #[must_use]
  pub fn is_failure(&self) -> bool {
    self.object_.is_null()
  }

  /// 原版 h:40-45 `template <typename T> bool To(Tagged<T>* obj) const`
  /// 失败则不写 obj 返回 false; 成功写入 Cast<T>(object_) 返回 true。
  /// C++ `Cast<T>` (调试类型校验) 以 [`Tagged::cast`] 零开销承接 (object_ 已确保非空)。
  #[inline]
  pub fn to<T>(&self, obj: &mut Tagged<T>) -> bool {
    if self.is_failure() {
      return false;
    }
    *obj = self.object_.cast::<T>();
    true
  }

  /// 原版 h:47-50 `Tagged<HeapObject> ToObjectChecked() const` — CHECK(!IsFailure())。
  /// 失败即 v8_fatal 终止 (非 panic, 与原版 CHECK 一致)。
  #[must_use]
  pub fn to_object_checked(&self) -> Tagged<HeapObject> {
    check_with_msg(!self.is_failure(), "!IsFailure()");
    self.object_.cast::<HeapObject>()
  }

  /// 原版 h:52-55 `Tagged<HeapObject> ToObject() const` — DCHECK(!IsFailure())。
  #[inline]
  #[must_use]
  pub fn to_object(&self) -> Tagged<HeapObject> {
    debug_assert!(
      !self.is_failure(),
      "ToObject 须在成功结果上调用 (原版 h:53 DCHECK)"
    );
    self.object_.cast::<HeapObject>()
  }

  /// 原版 h:57-60 `Address ToAddress() const` — DCHECK(!IsFailure()), 返回 object_.address()。
  #[inline]
  #[must_use]
  pub fn to_address(&self) -> Address {
    debug_assert!(
      !self.is_failure(),
      "ToAddress 须在成功结果上调用 (原版 h:58 DCHECK)"
    );
    self.object_.cast::<HeapObject>().address()
  }

  /// 原版 h:62 `explicit operator bool() const { return !IsFailure(); }`
  /// Rust 无 operator bool → 提供同名方法 (等价语义, 由调用点显式调用)。
  #[inline]
  #[must_use]
  pub fn ok(&self) -> bool {
    !self.is_failure()
  }
}

impl Default for AllocationResult {
  /// 原版 h:36 空 ctor (默认即失败)。
  fn default() -> Self {
    Self::null()
  }
}

// ═══════════════════════════════════════════════════════════════════════════
// AllocationStats (原版 allocation-stats.h:24-115)
// ═══════════════════════════════════════════════════════════════════════════

/// 页基空间 (BasePage) 的前向声明 (原版 allocation-stats.h:13 include heap/base-page.h)。
///
/// heap-base/base-page 属他票未转写, 本 crate 仅在其指针形参上做账目, 从不解引用 →
/// 与 memory_chunk.rs 的同名 opaque 承接 (原版 memory-chunk.h:34 前向声明) 共用同一类型,
/// 保证跨模块 BasePage 指针形参类型一致 (等价 C++ 单一 `class BasePage;` 声明)。
pub use crate::memory_chunk::BasePage;

/// 页基空间记账统计的抽象。原版 allocation-stats.h:24-115 `class AllocationStats`。
///
/// capacity_ 与 size_ 为原子字段 (原版 h:103/109,  evacuation 期多线程读 capacity 校验硬限);
/// 原子序采用 `Ordering::SeqCst` 承接 C++ `std::atomic` 默认序 (与原版一致)。
/// DEBUG-only 页账目 `allocated_on_page_` 以 `#[cfg(debug_assertions)]` 隔离 (原版 h:32/47/65-72/78-81/111-114)。
#[derive(Default)]
pub struct AllocationStats {
  /// 对象区当前字节数 (不含页簿记)。原版 h:103 `std::atomic<size_t> capacity_`。
  capacity_: AtomicUsize,
  /// 观测到的历史最大容量。原版 h:106 `size_t max_capacity_`。
  max_capacity_: usize,
  /// 已分配字节数。原版 h:109 `std::atomic<size_t> size_`。
  size_: AtomicUsize,
  /// DEBUG-only 每页分配账目, 键为 BasePage 指针地址。原版 h:112 `std::unordered_map<...>`。
  #[cfg(debug_assertions)]
  allocated_on_page_: HashMap<usize, usize>,
}

impl AllocationStats {
  /// 原版 h:26 `AllocationStats() { Clear(); }` — 默认即清零 (与 Default 一致)。
  #[inline]
  #[must_use]
  pub fn new() -> Self {
    Self::default()
  }

  /// 原版 h:28-36 `operator=(const AllocationStats& stats)` — 逐项拷贝 (assign 法承接)。
  /// 原子字段以 load/store (SeqCst) 拷贝; DEBUG 页账目 h:32-34 一并拷贝。
  pub fn assign(&mut self, stats: &Self) {
    self
      .capacity_
      .store(stats.capacity_.load(Ordering::SeqCst), Ordering::SeqCst);
    self.max_capacity_ = stats.max_capacity_;
    self
      .size_
      .store(stats.size_.load(Ordering::SeqCst), Ordering::SeqCst);
    #[cfg(debug_assertions)]
    {
      self.allocated_on_page_ = stats.allocated_on_page_.clone();
    }
  }

  /// 原版 h:39-43 `Clear()` — 清零所有统计 (含容量)。
  pub fn clear(&mut self) {
    self.capacity_.store(0, Ordering::SeqCst);
    self.max_capacity_ = 0;
    self.clear_size();
  }

  /// 原版 h:45-50 `ClearSize()` — 清 size (DEBUG 清页账目)。
  pub fn clear_size(&mut self) {
    self.size_.store(0, Ordering::SeqCst);
    #[cfg(debug_assertions)]
    self.allocated_on_page_.clear();
  }

  /// 原版 h:53 `size_t Capacity() const`。
  #[inline]
  #[must_use]
  pub fn capacity(&self) -> usize {
    self.capacity_.load(Ordering::SeqCst)
  }

  /// 原版 h:54 `size_t MaxCapacity() const`。
  #[inline]
  #[must_use]
  pub fn max_capacity(&self) -> usize {
    self.max_capacity_
  }

  /// 原版 h:55 `size_t Size() const`。
  #[inline]
  #[must_use]
  pub fn size(&self) -> usize {
    self.size_.load(Ordering::SeqCst)
  }

  /// 原版 h:57-59 `size_t AllocatedOnPage(const BasePage* page) const` — DEBUG-only。
  /// C++ `unordered_map::at` 缺键抛异常; 承接为 `Option` (本仓禁 panic), None 即原版缺账。
  #[cfg(debug_assertions)]
  #[must_use]
  pub fn allocated_on_page(&self, page: *const BasePage) -> Option<usize> {
    self.allocated_on_page_.get(&(page as usize)).copied()
  }

  /// 原版 h:62-73 `IncreaseAllocatedBytes(bytes, page)`。
  /// 对齐 DCHECK_IMPLIES (h:63-64) + 溢出 DCHECK_GE (h:65-68) + fetch_add + DEBUG 页账目 (h:70-72)。
  pub fn increase_allocated_bytes(&mut self, bytes: usize, page: *const BasePage) {
    let _ = page;
    // 原版 h:63-64 DCHECK_IMPLIES(V8_COMPRESS_POINTERS_8GB_BOOL, IsAligned(bytes, kObjectAlignment8GbHeap))
    debug_assert!(
      !V8_COMPRESS_POINTERS_8GB || bytes.is_multiple_of(K_OBJECT_ALIGNMENT_8GB_HEAP),
      "8GB 堆下增字节须 8GB 对齐 (原版 h:63-64)"
    );
    #[cfg(debug_assertions)]
    {
      // 原版 h:65-68 DCHECK_GE(size + bytes, size) — 无符号溢出
      let size = self.size();
      debug_assert!(
        size + bytes >= size,
        "size 增字节溢出 (原版 h:67 DCHECK_GE)"
      );
      // 原版 h:70-72 allocated_on_page_[page] += bytes
      *self.allocated_on_page_.entry(page as usize).or_insert(0) += bytes;
    }
    self.size_.fetch_add(bytes, Ordering::SeqCst);
  }

  /// 原版 h:75-82 `DecreaseAllocatedBytes(bytes, page)`。
  pub fn decrease_allocated_bytes(&mut self, bytes: usize, page: *const BasePage) {
    let _ = page;
    // 原版 h:76 DCHECK_GE(size_, bytes)
    debug_assert!(
      self.size() >= bytes,
      "减字节超出已分配 (原版 h:76 DCHECK_GE)"
    );
    #[cfg(debug_assertions)]
    {
      // 原版 h:79-80 DCHECK_GE(allocated_on_page_[page], bytes); allocated_on_page_[page] -= bytes
      let on_page = self.allocated_on_page_.entry(page as usize).or_insert(0);
      debug_assert!(*on_page >= bytes, "该页账目不足以减 (原版 h:79 DCHECK_GE)");
      *on_page -= bytes;
    }
    self.size_.fetch_sub(bytes, Ordering::SeqCst);
  }

  /// 原版 h:84-88 `DecreaseCapacity(bytes)`。
  pub fn decrease_capacity(&mut self, bytes: usize) {
    let capacity = self.capacity();
    // 原版 h:85-86 DCHECK_GE(capacity_, bytes); DCHECK_GE(capacity_ - bytes, size_)
    debug_assert!(
      capacity >= bytes,
      "容量减超出当前容量 (原版 h:85 DCHECK_GE)"
    );
    debug_assert!(
      capacity - bytes >= self.size(),
      "减容量后仍须 >= size (原版 h:86 DCHECK_GE)"
    );
    self.capacity_.store(capacity - bytes, Ordering::SeqCst);
  }

  /// 原版 h:90-96 `IncreaseCapacity(bytes)` — 增容量并刷新 max_capacity_。
  pub fn increase_capacity(&mut self, bytes: usize) {
    let capacity = self.capacity();
    // 原版 h:91 DCHECK_GE(capacity_ + bytes, capacity_)
    debug_assert!(
      capacity + bytes >= capacity,
      "容量增溢出 (原版 h:91 DCHECK_GE)"
    );
    let new_capacity = capacity + bytes;
    self.capacity_.store(new_capacity, Ordering::SeqCst);
    // 原版 h:93-95 if (capacity_ > max_capacity_) max_capacity_ = capacity_
    if new_capacity > self.max_capacity_ {
      self.max_capacity_ = new_capacity;
    }
  }
}
