#!/usr/bin/env bun
// tool/src/objects/simd.h.js — 覆盖 v8/src/objects/simd.{h,cc}
// → v8-objects-js-async/src/simd.rs
// 机械提取: Array.indexOf / Uint8Array↔hex 工具族签名 + NibbleToHex/ByteToHex 编码锚

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/simd.h");
const cc = readV8("src/objects/simd.cc");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`[simd.h.js] ${m}`); };

// ── 1. .h 声明面 (4 函数) ────────────────────────────────────────────────
must(h.includes("uintptr_t ArrayIndexOfIncludesSmiOrObject(Address array_start,"), "h 缺 ArrayIndexOfIncludesSmiOrObject");
must(h.includes("uintptr_t ArrayIndexOfIncludesDouble(Address array_start,"), "h 缺 ArrayIndexOfIncludesDouble");
must(h.includes("Tagged<Object> Uint8ArrayToHex(const char* bytes, size_t length,"), "h 缺 Uint8ArrayToHex");
must(h.includes("bool ArrayBufferFromHex(const base::Vector<T>& input_vector,"), "h 缺 ArrayBufferFromHex 模板");
// 未找到哨兵: cc 返回 -1 (uintptr_t 全 1 位模式)
must(cc.includes("  return -1;"), "cc 缺 indexOf 未找到返回 -1 锚");

// ── 2. .cc 实现锚 ────────────────────────────────────────────────────────
must(cc.includes("char NibbleToHex(uint8_t nibble) {"), "cc 缺 NibbleToHex");
must(cc.includes("inline uint16_t ByteToHex(uint8_t byte) {"), "cc 缺 ByteToHex");
must(cc.includes("void Uint8ArrayToHexSlow("), "cc 缺 Uint8ArrayToHexSlow");
must(cc.includes("void AtomicUint8ArrayToHexSlow("), "cc 缺 AtomicUint8ArrayToHexSlow");
must(cc.includes("void Uint8ArrayToHexFastWithSSE("), "cc 缺 Uint8ArrayToHexFastWithSSE");
must(cc.includes("ArrayIndexOfIncludesSmiOrObject(Address array_start"), "cc 缺 SmiOrObject 实现");
must(cc.includes("ArrayIndexOfIncludesDouble(Address array_start"), "cc 缺 Double 实现");
// NibbleToHex 编码表: '0'..'9' + 'a'..'f' 小写
const nib = cc.match(/char NibbleToHex\(uint8_t nibble\) \{([\s\S]*?)\n\}/);
must(nib, "cc NibbleToHex 体缺失");
must(nib[1].includes("'0'") && nib[1].includes("'a'"), "NibbleToHex 编码表应含 '0' 与 'a' 基准");

let rust = banner("src/objects/simd.h.js", "src/objects/simd.{h,cc}",
  "SIMD 加速工具: Array.indexOf 扫描 + Uint8Array↔hex 转换");
rust += `//! simd: 数组线性查找 (Smi/Object/Double) 与 Uint8Array 十六进制互转。
//!
//! indexOf 返回槽下标, 未命中返回 kNotFound (usize::MAX);
//! hex 编码为小写 ('0'..'9' + 'a'..'f'), SSE 快路径按平台分派。

/// 未找到哨兵 (原版 cc 返回 -1, uintptr_t 全 1 位模式 = usize::MAX)。
pub const K_NOT_FOUND: usize = usize::MAX;

// ── .h/.cc 签名骨架 (语义就地修订) ──
/// 原版 ArrayIndexOfIncludesSmiOrObject (h/cc: 标签槽扫描)。
pub fn array_index_of_includes_smi_or_object() {
  todo!("就地修订: simd.cc ArrayIndexOfIncludesSmiOrObject")
}

/// 原版 ArrayIndexOfIncludesDouble (h/cc: double 位模式扫描)。
pub fn array_index_of_includes_double() {
  todo!("就地修订: simd.cc ArrayIndexOfIncludesDouble")
}

/// 原版 Uint8ArrayToHex (h/cc: 慢路径 + SSE 快路径分派, 小写输出)。
pub fn uint8_array_to_hex() {
  todo!("就地修订: simd.cc Uint8ArrayToHex")
}

/// 原版 ArrayBufferFromHex<T> (h 模板声明: hex → 字节缓冲)。
pub fn array_buffer_from_hex() {
  todo!("就地修订: simd.h ArrayBufferFromHex 模板")
}

// ── 提取不变式断言 ──
const _: () = {
  assert!(K_NOT_FOUND == usize::MAX);
  assert!(usize::MAX.wrapping_add(1) == 0);
};
`;

const p = writeSrc("v8-objects-js-async", "simd.rs", rust);
console.log(`simd.h.js: ${p} — 4 函数签名 + kNotFound 哨兵 + hex 编码锚, 断言 ${checks} 项全过`);
