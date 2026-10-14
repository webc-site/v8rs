#!/usr/bin/env bun
// tool/src/base/atomicops.h.js — 族: src/base/atomicops.h
// 覆盖: Atomic8/16/32/64/Word 类型别名 + 14 个 atomic_ref 模板操作 + Relaxed_Memcpy/Memmove/Memcmp
// → v8-base/src/atomicops.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/base/atomicops.h");

// —— 锚点断言 ——
for (const t of ["Atomic8", "Atomic16", "Atomic32", "Atomic64", "AtomicWord"]) {
  if (!src.includes(`using ${t} =`)) throw new Error(`缺类型别名 ${t}`);
}
if (!src.includes("using Atomic8 = char;")) throw new Error("Atomic8 非 char 分支");
if (!src.includes("concept AtomicTypeForTrivialOperations")) throw new Error("缺 concept 约束");
if (!src.includes("inline void SeqCst_MemoryFence()")) throw new Error("缺 SeqCst_MemoryFence");

// —— 真解析: 全部模板自由函数, 对数 ——
const fns = [...src.matchAll(/^template <AtomicTypeForTrivialOperations T>\ninline (?:[\w:<>, ]+ )?(\w+)\(/gm)].map((m) => m[1]);
const expectedFns = [
  "Relaxed_CompareAndSwap", "AcquireRelease_CompareAndSwap", "Release_CompareAndSwap", "SeqCst_CompareAndSwap",
  "Relaxed_AtomicExchange", "SeqCst_AtomicExchange", "Relaxed_FetchOr", "Relaxed_AtomicIncrement",
  "Relaxed_Store", "Release_Store", "SeqCst_Store", "Relaxed_Load", "Acquire_Load", "SeqCst_Load",
];
for (const e of expectedFns) if (!fns.includes(e)) throw new Error(`缺模板函数 ${e}`);
if (fns.length !== expectedFns.length) throw new Error(`模板函数总数 ${fns.length} != ${expectedFns.length}`);

// relaxed 内存三件套 + helper
for (const f of ["Relaxed_Memcpy", "Relaxed_Memmove", "Relaxed_Memcmp", "MemcmpNotEqualFundamental"]) {
  if (!src.includes(f)) throw new Error(`缺 ${f}`);
}
// MemcmpNotEqualFundamental 双重载 (Atomic8 / AtomicWord)
const fundamentalOverloads = (src.match(/inline int MemcmpNotEqualFundamental\(/g) || []).length;
if (fundamentalOverloads !== 2) throw new Error(`MemcmpNotEqualFundamental 重载 ${fundamentalOverloads} != 2`);
console.log(`atomicops: 类型别名 5, 模板函数 ${fns.length}, Relaxed_Mem* 3, helper 2 重载, MemoryFence 1`);

let rust = banner(
  "src/base/atomicops.h.js",
  "src/base/atomicops.h",
  "原子原语层 (v8 旧原子 API, 全部委托 std::atomic_ref)",
);

rust += `
//! v8: src/base/atomicops.h
//! Rust 对应: std::sync::atomic::{AtomicI8..AtomicIsize, Ordering}; 此处登记原版 API 面

// v8: using Atomic8 = char; Atomic16 = int16_t; Atomic32 = int32_t;
//     Atomic64 = intptr_t (非 ILP32 的 64 位); AtomicWord = Atomic64 (64 位宿主)
// Rust: i8/i16/i32/i64/isize, 登记跳过 (类型别名无独立语义)

/// v8: src/base/atomicops.h SeqCst_MemoryFence
pub fn seq_cst_memory_fence();

// v8: 以下 14 个模板函数均为 std::atomic_ref + 对应 Ordering 的薄包装
//     CAS 系列用 compare_exchange_strong, 返回操作前的旧值 (C++ 版返回 old_value 形参)

/// v8: atomicops.h Relaxed_CompareAndSwap — relaxed 成功序 + relaxed 失败序
pub fn relaxed_cas<T>(ptr: &T, old: T, new: T) -> T;
/// v8: atomicops.h AcquireRelease_CompareAndSwap — acq_rel 成功 + acquire 失败
pub fn acq_rel_cas<T>(ptr: &T, old: T, new: T) -> T;
/// v8: atomicops.h Release_CompareAndSwap — release 成功 + relaxed 失败
pub fn release_cas<T>(ptr: &T, old: T, new: T) -> T;
/// v8: atomicops.h SeqCst_CompareAndSwap — seq_cst 成功 + seq_cst 失败
pub fn seq_cst_cas<T>(ptr: &T, old: T, new: T) -> T;
/// v8: atomicops.h Relaxed_AtomicExchange
pub fn relaxed_exchange<T>(ptr: &T, new: T) -> T;
/// v8: atomicops.h SeqCst_AtomicExchange
pub fn seq_cst_exchange<T>(ptr: &T, new: T) -> T;
/// v8: atomicops.h Relaxed_FetchOr
pub fn relaxed_fetch_or<T>(ptr: &T, bits: T) -> T;
/// v8: atomicops.h Relaxed_AtomicIncrement — 返回 increment + fetch_add 结果
pub fn relaxed_increment<T>(ptr: &T, increment: T) -> T;
/// v8: atomicops.h Relaxed_Store / Release_Store / SeqCst_Store
pub fn relaxed_store<T>(ptr: &T, value: T);
pub fn release_store<T>(ptr: &T, value: T);
pub fn seq_cst_store<T>(ptr: &T, value: T);
/// v8: atomicops.h Relaxed_Load / Acquire_Load / SeqCst_Load
pub fn relaxed_load<T>(ptr: &T) -> T;
pub fn acquire_load<T>(ptr: &T) -> T;
pub fn seq_cst_load<T>(ptr: &T) -> T;

/// v8: src/base/atomicops.h Relaxed_Memcpy
/// 逐字节对齐前导 → 字对齐主体 (AtomicWord 宽) → 尾部逐字节; 全程 Relaxed
pub fn relaxed_memcpy(dst: &mut [u8], src: &[u8]);

/// v8: src/base/atomicops.h Relaxed_Memmove
/// dst-src >= bytes (无重叠或 dst 在前) 走 memcpy; 否则反向复制
pub fn relaxed_memmove(dst: &mut [u8], src: &[u8]);

/// v8: src/base/atomicops.h helper::MemcmpNotEqualFundamental
/// Atomic8 版: u1 < u2 ? -1 : 1; AtomicWord 版: 小端逐字节比较
/// v8: src/base/atomicops.h Relaxed_Memcmp — 对齐前导 + 字对齐主体 + 尾部; 相等返回 0
pub fn relaxed_memcmp(s1: &[u8], s2: &[u8]) -> std::cmp::Ordering;
`;

writeSrc("v8-base", "atomicops.rs.skel", rust);
console.log("written v8-base/src/atomicops.rs.skel");
