#!/usr/bin/env bun
// tool/src/sandbox/bounded-size.js — 有界大小编码生成
// 组→文件映射 (本脚本管辖):
//   v8/src/sandbox/bounded-size.h + bounded-size-inl.h → v8-sandbox/src/bounded_size.rs
// 规则:
//   - 提取 Read/Write 移位与 DCHECK 边界, 断言与 internal.rs 常量一致
//   - 编码位运算 1:1 直出
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/sandbox/bounded-size.h");
const inl = readV8("src/sandbox/bounded-size-inl.h");

// ---- 锚点断言 ----
for (const sym of ["ReadBoundedSizeField", "WriteBoundedSizeField"]) {
  if (!h.includes(sym) || !inl.includes(sym)) throw new Error(`bounded-size: ${sym} 缺失`);
}
const readShift = inl.match(/return raw_value >> (\w+);/);
if (!readShift || readShift[1] !== "kBoundedSizeShift")
  throw new Error(`bounded-size: 读路径应为 >> kBoundedSizeShift, 实得 ${readShift}`);
const writeShift = inl.match(/size_t raw_value = value << (\w+);/);
if (!writeShift || writeShift[1] !== "kBoundedSizeShift")
  throw new Error(`bounded-size: 写路径应为 << kBoundedSizeShift, 实得 ${writeShift}`);
if (!/DCHECK_LE\(value, kMaxSafeBufferSizeForSandbox\);/.test(inl))
  throw new Error("bounded-size: 写路径缺少 <= kMaxSafeBufferSizeForSandbox 检查");

const body = `${banner("src/sandbox/bounded-size.js", "src/sandbox/bounded-size.h + src/sandbox/bounded-size-inl.h", "有界大小: 值 << 29 存储, 读回必 <= 32GB-1 (kMaxSafeBufferSizeForSandbox)")}
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
`;

const p = writeSrc("v8-sandbox", "bounded_size.rs", body);
console.log(
  `bounded-size.js: ${p} (移位核对通过: >> ${readShift[1]} / << ${writeShift[1]}, 边界 DCHECK 在位)`,
);
