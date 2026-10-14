// 由 tool/src/sandbox/bounded-size.js 从 v8/src/sandbox/bounded-size.h + src/sandbox/bounded-size-inl.h 生成; 修订就地改, 复跑覆盖
// 有界大小: 值 << 29 存储, 读回必 <= 32GB-1 (kMaxSafeBufferSizeForSandbox)

//! BoundedSize 编解码; 对应原版 bounded-size{,-inl}.h。
//! 语义: size_t 左移 29 位存储; 读回右移 29 位, 高 35 位被攻击者篡改也不影响有界性。

use crate::internal::{K_BOUNDED_SIZE_SHIFT, K_MAX_SAFE_BUFFER_SIZE_FOR_SANDBOX};

/// 原版 ReadBoundedSizeField(field_address)。
///
///
/// # Safety
///
/// field 须指向 8 字节可读内存 (对应 ReadUnalignedValue<size_t>)。
pub unsafe fn read_bounded_size_field(field: *const usize) -> usize {
  // SAFETY: 调用方保证字段可读。
  let raw = unsafe { field.read_unaligned() };
  raw >> K_BOUNDED_SIZE_SHIFT
}

/// 原版 WriteBoundedSizeField(field_address, value)。
/// debug 断言 value <= kMaxSafeBufferSizeForSandbox (原版 DCHECK; 越界高位静默截断, 与原版一致)。
///
///
/// # Safety
///
/// field 须指向 8 字节可写内存。
pub unsafe fn write_bounded_size_field(field: *mut usize, value: usize) {
  debug_assert!(value <= K_MAX_SAFE_BUFFER_SIZE_FOR_SANDBOX);
  // SAFETY: 调用方保证字段可写。
  unsafe { field.write_unaligned(value << K_BOUNDED_SIZE_SHIFT) };
}
