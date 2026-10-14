#!/usr/bin/env bun
// tool/src/base/atomic-utils.h.js — 族: src/base/atomic-utils.h
// 覆盖: AtomicValue + AsAtomicImpl (16 原子操作) + 4 类型别名 + AtomicTypeFromByteWidth
//        + AsAtomicPointerImpl + CheckedIncrement/Decrement + AsAtomicPtr
// → v8-base/src/atomic_utils.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/base/atomic-utils.h");

// —— 锚点断言 ——
if (!src.includes("class AtomicValue {")) throw new Error("缺 AtomicValue");
if (!src.includes("class AsAtomicImpl {")) throw new Error("缺 AsAtomicImpl");

// —— 真解析: AsAtomicImpl 全部静态模板方法, 对数 ——
const expected = [
  "SeqCst_Load", "Acquire_Load", "Relaxed_Load",
  "SeqCst_Store", "Release_Store", "Relaxed_Store",
  "SeqCst_Swap", "Relaxed_Swap",
  "Release_CompareAndSwap", "Relaxed_CompareAndSwap", "AcquireRelease_CompareAndSwap", "SeqCst_CompareAndSwap",
  "Relaxed_FetchOr",
  "Release_SetBits", "Relaxed_SetBits",
];
for (const e of expected) {
  // 签名形如 static T|void|bool <Op>(T* addr, ...)
  if (!new RegExp(`static (?:T|void|bool) ${e}\\(`).test(src)) throw new Error(`缺操作 ${e}`);
}
const opCount = expected.filter((e) => src.includes(e)).length;
if (opCount !== expected.length) throw new Error(`操作数 ${opCount} != ${expected.length}`);

// SetBits 双重载 (三参带 bits / 两参仅 mask)
const setBitsOverloads = [...src.matchAll(/static bool Relaxed_SetBits\(T\* addr(?:, T bits)?, T mask\)/g)];
if (setBitsOverloads.length !== 2) throw new Error(`Relaxed_SetBits 重载 ${setBitsOverloads.length} != 2`);

// 类型别名
const aliases = ["AsAtomic8", "AsAtomic16", "AsAtomic32", "AsAtomicWord", "AsAtomicPointer"];
for (const a of aliases) if (!src.includes(`using ${a} =`)) throw new Error(`缺别名 ${a}`);

// AtomicTypeFromByteWidth 特化
const specs = [...src.matchAll(/struct AtomicTypeFromByteWidth<(\d)>/g)].map((m) => m[1]);
if (specs.join(",") !== "1,2,4,8") throw new Error(`AtomicTypeFromByteWidth 特化 ${specs}`);

// 尾部自由函数
for (const f of ["CheckedIncrement", "CheckedDecrement", "AsAtomicPtr"]) {
  if (!src.includes(f)) throw new Error(`缺 ${f}`);
}
console.log(`atomic-utils: AsAtomicImpl 操作 ${opCount} (SetBits 2 重载), 别名 ${aliases.length}, 宽度特化 ${specs.length}, 自由函数 3`);

let rust = banner(
  "src/base/atomic-utils.h.js",
  "src/base/atomic-utils.h",
  "原子访问封装 (std::atomic_ref 之前的旧 API 面)",
);

rust += `
//! v8: src/base/atomic-utils.h
//! 原版头注明 Deprecated: 新代码用 std::atomic<T>; Rust 侧对应 AtomicU8..AtomicU64 + Ordering

/// v8: src/base/atomic-utils.h AtomicValue<T>
/// 原版: 存 AtomicWord, cast_helper 处理标量/指针/StrongAlias 三形; Rust 用泛型原子封装
pub struct AtomicValue<T>(std::sync::atomic::AtomicUsize, std::marker::PhantomData<T>);

impl<T> AtomicValue<T> {
    /// v8: AtomicValue::Value — Acquire_Load
    pub fn value(&self) -> T;
    /// v8: AtomicValue::SetValue — Release_Store
    pub fn set_value(&self, new_value: T);
}

/// v8: src/base/atomic-utils.h AsAtomicImpl<TAtomicStorageType>
/// 16 个操作 → Rust std::sync::atomic::Ordering 组合; 方法面登记如下
pub trait AsAtomic<T> {
    /// v8: AsAtomicImpl::SeqCst_Load
    fn seq_cst_load(&self) -> T;
    /// v8: AsAtomicImpl::Acquire_Load
    fn acquire_load(&self) -> T;
    /// v8: AsAtomicImpl::Relaxed_Load
    fn relaxed_load(&self) -> T;
    /// v8: AsAtomicImpl::SeqCst_Store
    fn seq_cst_store(&self, v: T);
    /// v8: AsAtomicImpl::Release_Store
    fn release_store(&self, v: T);
    /// v8: AsAtomicImpl::Relaxed_Store
    fn relaxed_store(&self, v: T);
    /// v8: AsAtomicImpl::SeqCst_Swap
    fn seq_cst_swap(&self, v: T) -> T;
    /// v8: AsAtomicImpl::Relaxed_Swap
    fn relaxed_swap(&self, v: T) -> T;
    /// v8: AsAtomicImpl::Release_CompareAndSwap — 返回旧值
    fn release_cas(&self, old: T, new: T) -> T;
    /// v8: AsAtomicImpl::Relaxed_CompareAndSwap
    fn relaxed_cas(&self, old: T, new: T) -> T;
    /// v8: AsAtomicImpl::AcquireRelease_CompareAndSwap
    fn acq_rel_cas(&self, old: T, new: T) -> T;
    /// v8: AsAtomicImpl::SeqCst_CompareAndSwap
    fn seq_cst_cas(&self, old: T, new: T) -> T;
    /// v8: AsAtomicImpl::Relaxed_FetchOr
    fn relaxed_fetch_or(&self, bits: T) -> T;
    /// v8: AsAtomicImpl::Release_SetBits(addr, bits, mask) — CAS 循环; 已达目标返回 false
    fn release_set_bits(&self, bits: T, mask: T) -> bool;
    /// v8: AsAtomicImpl::Relaxed_SetBits(addr, bits, mask) — CAS 循环版
    fn relaxed_set_bits(&self, bits: T, mask: T) -> bool;
    /// v8: AsAtomicImpl::Relaxed_SetBits(addr, mask) — FetchOr 版, 置 1
    fn relaxed_set_mask(&self, mask: T) -> bool;
}

// v8: using AsAtomic8/16/32/Word = AsAtomicImpl<base::Atomic8/16/32/Word>
// v8: using AsAtomicPointer = AsAtomicPointerImpl<AtomicWord> (SetBits 三参版 = delete)
// Rust 侧由具体 AtomicUxx 实现表达, 别名登记跳过

// v8: AtomicTypeFromByteWidth<1/2/4/8> → Atomic8/16/32/64
// Rust: 类型级选择由泛型常量/关联类型表达, 登记跳过 (std::sync::atomic 直接覆盖)

/// v8: src/base/atomic-utils.h CheckedIncrement — fetch_add 后 DCHECK 不回绕 (unsigned)
pub fn checked_increment(number: &std::sync::atomic::AtomicUsize, amount: usize, order: std::sync::atomic::Ordering);

/// v8: src/base/atomic-utils.h CheckedDecrement — fetch_sub 后 DCHECK 不下溢 (unsigned)
pub fn checked_decrement(number: &std::sync::atomic::AtomicUsize, amount: usize, order: std::sync::atomic::Ordering);

/// v8: src/base/atomic-utils.h AsAtomicPtr — T* → std::atomic<T>*; Rust 侧即 &AtomicT, 登记跳过
`;

writeSrc("v8-base", "atomic_utils.rs.skel", rust);
console.log("written v8-base/src/atomic_utils.rs.skel");
