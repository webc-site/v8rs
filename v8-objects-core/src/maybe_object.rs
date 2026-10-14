// 由 tool/src/objects/maybe-object.h.js 从 v8/src/objects/maybe-object.h + maybe-object-inl.h 生成; 修订就地改, 复跑覆盖
// 钉死配置: V8_COMPRESS_POINTERS 开 (compressed-slots-inl.h 同基线)

//! MaybeObject 支撑面
//!
//! ClearedWeakValue 是弱引用槽的"已清除"哨兵: 上 32 位未置位的形式
//! (kClearedWeakValue, 3) 不属于任何压缩 cage;
//! 运行时值必须带上所属 cage 的高位 (ClearedValue / ClearedTrustedValue)。

use v8_common::{TrustedSpaceCompressionScheme, V8HeapCompressionScheme, has_smi_tag};

#[cfg(debug_assertions)]
use crate::tagged_constants::WEAK_HEAP_OBJECT_TAG;
use crate::{
  compressed_slots::CompressedHeapObjectSlot,
  heap_object::HeapObject,
  tagged::{ClearedWeakValue, Tagged},
  tagged_constants::{HEAP_OBJECT_TAG, HEAP_OBJECT_TAG_MASK},
};

/// 已清除弱值的低 32 位
pub const K_CLEARED_WEAK_HEAP_OBJECT_LOWER32: u32 = 3;

/// 静态清除哨兵值
pub const K_CLEARED_WEAK_VALUE: usize = 3;

/// 弱引用位掩码
const K_WEAK_HEAP_OBJECT_MASK: usize = 1 << 1;

/// 主 cage 清除值
#[inline]
pub fn cleared_value() -> Tagged<ClearedWeakValue> {
  Tagged::new(V8HeapCompressionScheme::decompress_tagged(
    K_CLEARED_WEAK_HEAP_OBJECT_LOWER32,
  ))
}

/// 信任 cage 清除值
#[inline]
pub fn cleared_trusted_value() -> Tagged<ClearedWeakValue> {
  Tagged::new(TrustedSpaceCompressionScheme::decompress_tagged(
    K_CLEARED_WEAK_HEAP_OBJECT_LOWER32,
  ))
}

/// 更新堆对象引用槽，保持标记位不变
///
/// # Safety
/// 槽地址须有效对齐且内容为堆对象引用 (非 Smi)。
#[inline]
pub unsafe fn update_heap_object_reference_slot(
  slot: CompressedHeapObjectSlot,
  value: Tagged<HeapObject>,
) {
  // SAFETY: 契约由调用方承接 (maybe-object-inl.h:45 直读槽)。
  let old_value = unsafe { slot.load() };
  debug_assert!(!has_smi_tag(old_value.ptr()), "旧值不得为 Smi");
  debug_assert!(
    (value.ptr() & HEAP_OBJECT_TAG_MASK) == HEAP_OBJECT_TAG,
    "新值须带堆对象标记位"
  );

  #[cfg(debug_assertions)]
  let weak_before = (old_value.ptr() & HEAP_OBJECT_TAG_MASK) == WEAK_HEAP_OBJECT_TAG;

  let new_value = value.ptr() | (old_value.ptr() & K_WEAK_HEAP_OBJECT_MASK);
  // SAFETY: 同槽位宽 store (调用方契约覆盖)。
  unsafe { slot.store(Tagged::new(new_value)) };

  #[cfg(debug_assertions)]
  {
    // 原版 weak_before == weak_after 不变式 (debug 双读)。
    let weak_after = (unsafe { slot.load() }.ptr() & HEAP_OBJECT_TAG_MASK) == WEAK_HEAP_OBJECT_TAG;
    debug_assert_eq!(weak_before, weak_after);
  }
}
