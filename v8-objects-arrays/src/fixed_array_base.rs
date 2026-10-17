// 由 tool/src/objects/fixed-array-base.h.js 从 v8/src/objects/fixed-array-base.h + fixed-array-base-inl.h + fixed-array-base.tq 生成; 修订就地改, 复跑覆盖
// FixedArrayBase/TrustedFixedArrayBase 头布局 + length 语义; cow/ElementsKind 依赖参数注入 (脚本头登记)

//! FixedArray 族公共头: FixedArrayBase / TrustedFixedArrayBase。
//!
//! 布局双轨制 (同 v8-objects-strings 先例):
//! - snapshot 偏移常量 (K_*): 压缩指针默认配置 (K_TAGGED_SIZE=4) 的堆布局,
//!   snapshot 兼容锚点;
//! - 运行时访问: Rust 结构体逻辑布局 (Tagged 槽 usize 宽), length/capacity 槽
//!   经结构体字段偏移读写, Tagged 槽压缩收窄后两者合一。
//!
//! 跳过登记 (脚本头): IsCowArray roots 依赖 / GetMaxLengthForNewSpaceAllocation
//! ElementsKind 依赖 / DECL_VERIFIER 族。

use std::{
  mem::{offset_of, size_of},
  sync::atomic::Ordering,
};

use v8_common::{K_MAX_REGULAR_HEAP_OBJECT_SIZE, K_TAGGED_SIZE, TAGGED_SIZE_8_BYTES};
use v8_objects_core::{
  heap_object::HeapObject, map::Map, objects::WriteBarrierMode, tagged::Tagged,
};

use crate::trusted::TrustedObject;

/// 全 fixed array 族统一容量上限 (fixed-array-base.h:22)。
///
/// 统一上限保证 非定容 elements kind 迁移 (如 Smi → Double) 不因容量上限差异失败;
/// 再大一档 (2 的幂) 会把 FixedDoubleArray 最大字节尺寸推出 int32 范围。
pub const K_MAX_FIXED_ARRAY_CAPACITY: u32 = 128 * 1024 * 1024;

// ── 布局常量 (snapshot; kLengthOffset = sizeof(HeapObject/TrustedObject)) ──
/// length_ 字段偏移 (两基类同值: sizeof(TrustedObject) == sizeof(HeapObject))。
pub const K_LENGTH_OFFSET: usize = K_TAGGED_SIZE;
/// optional_padding_ 偏移 (仅 TAGGED_SIZE_8_BYTES 配置存在)。
pub const K_PADDING_OFFSET: usize = K_LENGTH_OFFSET + 4;
/// 头大小 (8 字节 tagged 配置含 4 字节 padding)。
pub const K_HEADER_SIZE: usize = if TAGGED_SIZE_8_BYTES {
  K_PADDING_OFFSET + 4
} else {
  K_LENGTH_OFFSET + 4
};
/// 容量上限 (== kMaxFixedArrayCapacity)。
pub const K_MAX_LENGTH: u32 = K_MAX_FIXED_ARRAY_CAPACITY;
/// 常规页 (非大对象空间) 内的最大槽数。
pub const K_MAX_REGULAR_LENGTH: usize =
  (K_MAX_REGULAR_HEAP_OBJECT_SIZE - K_HEADER_SIZE) / K_TAGGED_SIZE;

// 编译期布局对数 (压缩默认锚点; 原版 static_assert(sizeof(FixedArray) == kHeaderSize))
const _: () = assert!(K_HEADER_SIZE == K_TAGGED_SIZE + 4);
const _: () = assert!(!TAGGED_SIZE_8_BYTES && K_HEADER_SIZE == 8);

/// FixedArray 族公共基类头: HeapObject + length_ (fixed-array-base.h:29)。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FixedArrayBase {
  /// HeapObject 头 (map 槽)。
  pub header: HeapObject,
  /// 元素数 (V8_TQ_CONST uint32_t length_)。
  pub length: u32,
}

impl FixedArrayBase {
  /// 构造头 (snapshot 布局下 8 字节 tagged 配置另有 optional_padding_)。
  #[inline]
  pub const fn new(map: Tagged<Map>, length: u32) -> Self {
    Self {
      header: HeapObject::new(map),
      length,
    }
  }
}

/// Trusted 空间镜像基类头: TrustedObject + length_ (fixed-array-base.h:67)。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TrustedFixedArrayBase {
  /// TrustedObject 头 (无额外字段, 布局与 HeapObject 同)。
  pub header: TrustedObject,
  /// 元素数。
  pub length: u32,
}

impl TrustedFixedArrayBase {
  /// 构造头。
  #[inline]
  pub const fn new(map: Tagged<Map>, length: u32) -> Self {
    Self {
      header: TrustedObject::new(map),
      length,
    }
  }
}

// 运行时 length 槽偏移对数: 两基类字段布局一致 (sizeof(TrustedObject)==sizeof(HeapObject))
const _: () = assert!(size_of::<TrustedFixedArrayBase>() == size_of::<FixedArrayBase>());

/// 写屏障桩 (heap 层 WriteBarrier::ForValue 落地前为 no-op)。
///
/// TODO(L3/heap): 接真实 remembered-set 记录; trusted space 需
/// ForProtectedPointer 专用路径 (TRUSTED_TO_SHARED_TRUSTED 条目)。
#[inline]
pub(crate) fn write_barrier_stub(_mode: WriteBarrierMode) {}

/// u32 槽读写助手 (宿主地址 + 偏移; C++ 普通读写 → Relaxed 原子)。
pub(crate) mod u32_slot {
  use std::sync::atomic::{AtomicU32, Ordering};

  /// # Safety
  /// addr 须指向对象内有效 4 字节对齐 u32 槽。
  #[inline]
  pub unsafe fn load(addr: usize) -> u32 {
    // SAFETY: 调用方契约
    unsafe { AtomicU32::from_ptr(addr as *mut u32) }.load(Ordering::Relaxed)
  }

  /// # Safety
  /// 同 [load]。
  #[inline]
  pub unsafe fn load_order(addr: usize, order: Ordering) -> u32 {
    // SAFETY: 调用方契约
    unsafe { AtomicU32::from_ptr(addr as *mut u32) }.load(order)
  }

  /// # Safety
  /// 同 [load]; 无并发别名写。
  #[inline]
  pub unsafe fn store(addr: usize, value: u32) {
    // SAFETY: 调用方契约
    unsafe { AtomicU32::from_ptr(addr as *mut u32) }.store(value, Ordering::Relaxed);
  }

  /// # Safety
  /// 同 [load]。
  #[inline]
  pub unsafe fn store_order(addr: usize, value: u32, order: Ordering) {
    // SAFETY: 调用方契约
    unsafe { AtomicU32::from_ptr(addr as *mut u32) }.store(value, order);
  }
}

/// Tagged<FixedArrayBase> 成员语义 (fixed-array-base-inl.h:19-37)。
pub trait FixedArrayBaseExt: Copy {
  /// length() (SafeHeapObjectSize(length_))。
  ///
  /// # Safety
  /// 指针须指向有效 FixedArrayBase 内存。
  #[inline]
  unsafe fn length(&self) -> u32 {
    // SAFETY: 调用方契约; 槽位恒在对象内
    unsafe { u32_slot::load(self.slot_addr()) }
  }

  /// ulength() (无符号视图, 与 length 同)。
  ///
  /// # Safety
  /// 同 [Self::length]。
  #[inline]
  unsafe fn ulength(&self) -> u32 {
    // SAFETY: 同 length 契约
    unsafe { self.length() }
  }

  /// length(kAcquireLoad)。
  ///
  /// # Safety
  /// 同 [Self::length]。
  #[inline]
  unsafe fn length_acquire(&self) -> u32 {
    // SAFETY: 调用方契约
    unsafe { u32_slot::load_order(self.slot_addr(), Ordering::Acquire) }
  }

  /// length(kRelaxedLoad)。
  ///
  /// # Safety
  /// 同 [Self::length]。
  #[inline]
  unsafe fn length_relaxed(&self) -> u32 {
    // SAFETY: 调用方契约
    unsafe { u32_slot::load_order(self.slot_addr(), Ordering::Relaxed) }
  }

  /// set_length(value)。
  ///
  /// # Safety
  /// 同 [Self::length]; 无并发别名。
  #[inline]
  unsafe fn set_length(&self, value: u32) {
    // SAFETY: 调用方契约
    unsafe { u32_slot::store(self.slot_addr(), value) }
  }

  /// set_length(value, kReleaseStore)。
  ///
  /// # Safety
  /// 同 [Self::length]。
  #[inline]
  unsafe fn set_length_release(&self, value: u32) {
    // SAFETY: 调用方契约
    unsafe { u32_slot::store_order(self.slot_addr(), value, Ordering::Release) }
  }

  /// length 槽地址。
  ///
  /// # Safety
  /// 同 [Self::length]。
  unsafe fn slot_addr(&self) -> usize;
}

impl FixedArrayBaseExt for Tagged<FixedArrayBase> {
  #[inline]
  unsafe fn slot_addr(&self) -> usize {
    // 运行时槽位 = 结构体字段偏移 (Tagged usize 宽); snapshot 锚点 K_LENGTH_OFFSET
    self.address() + offset_of!(FixedArrayBase, length)
  }
}

/// Tagged<TrustedFixedArrayBase> 成员语义 (fixed-array-base-inl.h:39-64)。
pub trait TrustedFixedArrayBaseExt: Copy {
  /// length()。
  ///
  /// # Safety
  /// 指针须指向有效 TrustedFixedArrayBase 内存。
  #[inline]
  unsafe fn length(&self) -> u32 {
    // SAFETY: 调用方契约; 槽位恒在对象内
    unsafe { u32_slot::load(self.slot_addr()) }
  }

  /// ulength()。
  ///
  /// # Safety
  /// 同 [Self::length]。
  #[inline]
  unsafe fn ulength(&self) -> u32 {
    // SAFETY: 同 length 契约
    unsafe { self.length() }
  }

  /// length(kAcquireLoad)。
  ///
  /// # Safety
  /// 同 [Self::length]。
  #[inline]
  unsafe fn length_acquire(&self) -> u32 {
    // SAFETY: 调用方契约
    unsafe { u32_slot::load_order(self.slot_addr(), Ordering::Acquire) }
  }

  /// length(kRelaxedLoad)。
  ///
  /// # Safety
  /// 同 [Self::length]。
  #[inline]
  unsafe fn length_relaxed(&self) -> u32 {
    // SAFETY: 调用方契约
    unsafe { u32_slot::load_order(self.slot_addr(), Ordering::Relaxed) }
  }

  /// set_length(value)。
  ///
  /// # Safety
  /// 同 [Self::length]; 无并发别名。
  #[inline]
  unsafe fn set_length(&self, value: u32) {
    // SAFETY: 调用方契约
    unsafe { u32_slot::store(self.slot_addr(), value) }
  }

  /// set_length(value, kReleaseStore)。
  ///
  /// # Safety
  /// 同 [Self::length]。
  #[inline]
  unsafe fn set_length_release(&self, value: u32) {
    // SAFETY: 调用方契约
    unsafe { u32_slot::store_order(self.slot_addr(), value, Ordering::Release) }
  }

  /// length 槽地址。
  ///
  /// # Safety
  /// 同 [Self::length]。
  unsafe fn slot_addr(&self) -> usize;
}

impl TrustedFixedArrayBaseExt for Tagged<TrustedFixedArrayBase> {
  #[inline]
  unsafe fn slot_addr(&self) -> usize {
    // 运行时槽位 = 结构体字段偏移; snapshot 锚点 K_LENGTH_OFFSET
    self.address() + offset_of!(TrustedFixedArrayBase, length)
  }
}

impl FixedArrayBase {
  /// GetMaxLengthForNewSpaceAllocation (fixed-array-base-inl.h:67)。
  ///
  /// ElementsKindToShiftSize 归 elements crate (脚本头登记), 调用方传入
  /// elements kind 的 shift size (tagged 槽 = K_TAGGED_SIZE_LOG2)。
  #[inline]
  #[must_use]
  pub const fn max_length_for_new_space_allocation(shift_size: u32) -> usize {
    (K_MAX_REGULAR_HEAP_OBJECT_SIZE - K_HEADER_SIZE) >> shift_size
  }
}

// capacity+length 族 snapshot 常量再导出 (定义于 fixed_array.rs, 头层统一入口)
pub use crate::fixed_array::{
  K_ARRAY_LIST_CAPACITY_OFFSET, K_ARRAY_LIST_HEADER_SIZE, K_ARRAY_LIST_LENGTH_OFFSET,
  K_WEAK_ARRAY_LIST_CAPACITY_OFFSET, K_WEAK_ARRAY_LIST_HEADER_SIZE,
  K_WEAK_ARRAY_LIST_LENGTH_OFFSET,
};
