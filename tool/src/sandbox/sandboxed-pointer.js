#!/usr/bin/env bun
// tool/src/sandbox/sandboxed-pointer.js — 沙箱内指针编码生成
// 组→文件映射 (本脚本管辖):
//   v8/src/sandbox/sandboxed-pointer.h + sandboxed-pointer-inl.h → v8-sandbox/src/sandboxed_pointer.rs
// 规则:
//   - 提取 Read/Write 两个函数的移位表达式, 断言与 internal.rs 常量一致
//   - 编码位运算 1:1 直出 (逻辑仅 2 行, 无需骨架)
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/sandbox/sandboxed-pointer.h");
const inl = readV8("src/sandbox/sandboxed-pointer-inl.h");

// ---- 锚点断言 ----
for (const sym of ["ReadSandboxedPointerField", "WriteSandboxedPointerField"]) {
  if (!h.includes(sym) || !inl.includes(sym)) throw new Error(`sandboxed-pointer: ${sym} 缺失`);
}
// 读: sandboxed_pointer >> kSandboxedPointerShift; 写: offset << shift (对数核对)
const readShift = inl.match(/Address offset = sandboxed_pointer >> (\w+);/);
if (!readShift || readShift[1] !== "kSandboxedPointerShift")
  throw new Error(`sandboxed-pointer: 读路径应为 >> kSandboxedPointerShift, 实得 ${readShift}`);
const writeShift = inl.match(/SandboxedPointer_t sandboxed_pointer = offset << (\w+);/);
if (!writeShift || writeShift[1] !== "kSandboxedPointerShift")
  throw new Error(`sandboxed-pointer: 写路径应为 << kSandboxedPointerShift, 实得 ${writeShift}`);
// 写路径含沙箱包含性检查 (CHECK Contains)
if (!/CHECK\(Sandbox::current\(\)->Contains\(pointer\)\);/.test(inl))
  throw new Error("sandboxed-pointer: 写路径缺少 Contains 检查");
// 读路径 = cage_base.address() + offset
if (!/Address pointer = cage_base\.address\(\) \+ offset;/.test(inl))
  throw new Error("sandboxed-pointer: 读路径应为 cage_base + offset");

const body = `${banner("src/sandbox/sandboxed-pointer.js", "src/sandbox/sandboxed-pointer.h + src/sandbox/sandboxed-pointer-inl.h", "沙箱内指针: 40 位 cage 偏移 << 24 存储; 读 = cage_base + (raw >> 24)")}
//! SandboxedPointer 编解码; 对应原版 sandboxed-pointer{,-inl}.h。
//! 语义: 指针保证指向沙箱内, 以 \`偏移 << K_SANDBOXED_POINTER_SHIFT\` 存储 (右移 24 位后必小于 1TB)。

use crate::internal::{Address, K_SANDBOXED_POINTER_SHIFT};
use crate::{Error, Result};

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
  if offset >= crate::internal::K_SANDBOX_SIZE {
    return Err(Error::SandboxViolation);
  }
  // SAFETY: 调用方保证字段可写。
  unsafe { field.write_unaligned((offset as u64) << K_SANDBOXED_POINTER_SHIFT) };
  Ok(())
}
`;

const p = writeSrc("v8-sandbox", "sandboxed_pointer.rs", body);
console.log(
  `sandboxed-pointer.js: ${p} (读写移位方向核对通过: >> ${readShift[1]} / << ${writeShift[1]})`,
);
