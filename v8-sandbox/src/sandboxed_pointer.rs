// 由 tool/src/sandbox/sandboxed-pointer.js 从 v8/src/sandbox/sandboxed-pointer.h + src/sandbox/sandboxed-pointer-inl.h 生成; 修订就地改, 复跑覆盖
// 沙箱内指针: 40 位 cage 偏移 << 24 存储; 读 = cage_base + (raw >> 24)

//! SandboxedPointer 编解码; 对应原版 sandboxed-pointer{,-inl}.h。
//! 语义: 指针保证指向沙箱内, 以 `偏移 << K_SANDBOXED_POINTER_SHIFT` 存储 (右移 24 位后必小于 1TB)。

use crate::{
  Error, Result,
  internal::{Address, K_SANDBOX_SIZE, K_SANDBOXED_POINTER_SHIFT},
};

/// 原版 ReadSandboxedPointerField(field_address, cage_base)。
/// 字段是 64 位编码值; cage_base 即沙箱基址 (指针压缩 cage 位于沙箱起点, 语义一致)。
///
///
/// # Safety
///
/// field 须指向 8 字节可读内存 (对应原版 ReadUnalignedValue 的地址约束)。
pub unsafe fn read_sandboxed_pointer_field(field: *const u64, cage_base: Address) -> Address {
  // SAFETY: 调用方保证字段可读; 读后纯算术。
  let sandboxed_pointer = unsafe { field.read_unaligned() };
  let offset = (sandboxed_pointer >> K_SANDBOXED_POINTER_SHIFT) as Address;
  cage_base.wrapping_add(offset)
}

/// 原版 WriteSandboxedPointerField(field_address, cage_base, pointer)。
/// 指针必须在 [cage_base, cage_base + 沙箱大小) 内 (原版 CHECK, 这里返回错误不 panic)。
///
///
/// # Safety
///
/// field 须指向 8 字节可写内存。
pub unsafe fn write_sandboxed_pointer_field(
  field: *mut u64,
  cage_base: Address,
  pointer: Address,
) -> Result<()> {
  // 原版 CHECK(Sandbox::current()->Contains(pointer)); cage_base 对齐 4GB,
  // 偏移 < 1TB 即在沙箱内。
  let offset = pointer
    .checked_sub(cage_base)
    .ok_or(Error::SandboxViolation)?;
  if offset >= K_SANDBOX_SIZE {
    return Err(Error::SandboxViolation);
  }
  // SAFETY: 调用方保证字段可写。
  unsafe { field.write_unaligned((offset as u64) << K_SANDBOXED_POINTER_SHIFT) };
  Ok(())
}
