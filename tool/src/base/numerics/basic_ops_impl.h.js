#!/usr/bin/env bun
// tool/src/base/numerics/basic_ops_impl.h.js — 覆盖 v8/src/base/numerics/basic_ops_impl.h
// 解析 MathType 别名与 SwapBytes/FromLittleEndian/ToLittleEndian 的 signed/unsigned 重载对
// → v8-base/src/numerics/basic_ops_impl.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/base/numerics/basic_ops_impl.h");

for (const anchor of [
  "V8_BASE_NUMERICS_BASIC_OPS_IMPL_H_",
  "using MathType",
  "SwapBytes",
  "FromLittleEndian",
  "ToLittleEndian",
  "__builtin_bswap",
]) {
  if (!h.includes(anchor)) throw new Error(`anchor missing: ${anchor}`);
}

// 真解析: MathType 条件别名
const mathType = h.match(/using MathType = std::conditional_t<\s*sizeof\(T\) >= sizeof\(int\), T,\s*std::conditional_t<std::is_signed_v<T>, int, unsigned int>>;/s);
if (!mathType) throw new Error("MathType alias not found");

// 真解析: 三个函数名的 signed/unsigned 重载各按精确签名计数
const sigCounts = {
  SwapBytes: (h.match(/inline constexpr T SwapBytes\(T value\) \{/g) || []).length,
  FromLittleEndian: (h.match(/inline constexpr T FromLittleEndian\(std::span<const uint8_t, sizeof\(T\)> bytes\) \{/g) || []).length,
  ToLittleEndian: (h.match(/inline constexpr std::array<uint8_t, sizeof\(T\)> ToLittleEndian\(T val\) \{/g) || []).length,
};
const names = Object.keys(sigCounts);
const fns = names.map((n) => ({ name: n, count: sigCounts[n] }));
if (fns.length !== 3 || fns.some((f) => f.count !== 2)) {
  throw new Error(`expect 3 fns x 2 overloads, got ${JSON.stringify(sigCounts)}`);
}

// bswap 宽度分派 (constexpr 路径): sizeof 1/2/4/8
const widths = [...h.matchAll(/sizeof\(T\) == (\d+)u/g)].map((m) => m[1]);
const widthSet = [...new Set(widths)].sort();

const body = `${banner("src/base/numerics/basic_ops_impl.h.js", "src/base/numerics/basic_ops_impl.h", "字节序基础原语: MathType/SwapBytes/LE 读写")}
//! v8 侧: 别名 1 个 (MathType), 函数 ${names.length} 个 × signed/unsigned 重载 = ${names.length * 2} 签名
//! 重载计数: ${names.map((n) => `${n}=${sigCounts[n]}`).join(", ")}
//! SwapBytes 宽度分派: ${widthSet.join("/")} 字节 (constexpr 路径纯移位实现)

/// v8: src/base/numerics/basic_ops_impl.h MathType
/// T 位宽 >= int 直接用 T; 否则提升为 int/unsigned (避免有符号提升 UB)
/// Rust: 数值字面量与运算不提升, 该别名在 Rust 侧退化为类型恒等; 保留文档锚
pub type MathType<T> = T;

/// 字节序反转; signed 版转无符号处理后转回
/// v8: src/base/numerics/basic_ops_impl.h SwapBytes (unsigned + signed 重载)
/// Rust: u16::swap_bytes/u32::swap_bytes/u64::swap_bytes 即 constexpr 等价
#[inline]
pub const fn swap_bytes<T>(value: T) -> T {
    // 语义: 按 sizeof(T) 分派 swap_bytes; 1 字节原样 — 就地修订特化
    /// v8: widths = ${widthSet.join(", ")}
    todo!()
}

/// 字节数组 → 整数 (小端)
/// v8: src/base/numerics/basic_ops_impl.h FromLittleEndian (unsigned + signed 重载)
/// v8 语义: 常量求值走逐字节 val |= byte << (8*i); 运行时走 memcpy
#[inline]
pub const fn from_little_endian<const N: usize>(bytes: [u8; N]) -> u64 {
    // 语义: u64::from_le_bytes — 就地修订泛型化
    todo!()
}

/// 整数 → 字节数组 (小端)
/// v8: src/base/numerics/basic_ops_impl.h ToLittleEndian (unsigned + signed 重载)
/// v8 语义: 常量求值走逐字节 val & 0xff 后 >> 8; 运行时走 memcpy
#[inline]
pub const fn to_little_endian<const N: usize>(val: u64) -> [u8; N] {
    // 语义: val.to_le_bytes() — 就地修订泛型化
    todo!()
}
`;

writeSrc("v8-base", "numerics/basic_ops_impl.rs.skel", body);
console.log(`basic_ops_impl.h.js: aliases=1 fns=${names.length} (${names.join(",")}) overloads=${names.length * 2} widths=${widthSet.join("/")}`);
