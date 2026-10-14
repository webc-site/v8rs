#!/usr/bin/env bun
// tool/src/objects/maybe-object.h.js — 清弱值与引用槽更新生成
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/maybe-object.h + maybe-object-inl.h → v8-objects-core/src/maybe_object.rs
// 规则面: kClearedWeakValue 常量 / ClearedValue 家族语义 / UpdateHeapObjectReferenceSlot 位运算
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/maybe-object.h");
const inl = readV8("src/objects/maybe-object-inl.h");
const globals = readV8("src/common/globals.h");

// —— kClearedWeakHeapObjectLower32 原始值 (v8-internal.h) ——
const m = globals.match(/kClearedWeakHeapObjectLower32 = (\d+);/);
if (!m) throw new Error("globals.h 缺 kClearedWeakHeapObjectLower32");
const clearedLower32 = m[1];
if (clearedLower32 !== "3") throw new Error(`kClearedWeakHeapObjectLower32 应 3 (弱 tag 0b11, 上 30 位清零), 实得 ${clearedLower32}`);

// —— 锚点断言 ——
if (!/static constexpr Tagged<ClearedWeakValue> kClearedWeakValue =\s*\n\s*Tagged<ClearedWeakValue>\(kClearedWeakHeapObjectLower32\);/.test(h))
  throw new Error("kClearedWeakValue 定义漂移");
for (const fn of ["ClearedValue", "ClearedTrustedValue", "UpdateHeapObjectReferenceSlot"])
  if (!new RegExp(`inline .*${fn}`).test(h)) throw new Error(`maybe-object.h 缺 ${fn}`);

// —— UpdateHeapObjectReferenceSlot 位运算面 (inl) ——
if (!/slot\.store\(Cast<HeapObjectReference>\(\s*\n?\s*Tagged<MaybeObject>\(new_value \| \(old_value & kWeakHeapObjectMask\)\)\)/.test(inl))
  throw new Error("UpdateHeapObjectReferenceSlot 弱位保持位运算缺失");
if (!/static_assert\(std::is_same_v<THeapObjectSlot, FullHeapObjectSlot> \|\|\s*std::is_same_v<THeapObjectSlot, HeapObjectSlot>/.test(inl))
  throw new Error("THeapObjectSlot 槽型约束缺失");

// ClearedValue 压缩分支: 主 cage / trusted cage 双方案
const decompressSchemes = [...inl.matchAll(/(\w+CompressionScheme)::DecompressTagged\(\s*kClearedWeakHeapObjectLower32\)/g)].map((x) => x[1]);
if (decompressSchemes.join() !== "V8HeapCompressionScheme,TrustedSpaceCompressionScheme")
  throw new Error(`Cleared 值解压方案应 V8Heap/TrustedSpace, 实得 ${decompressSchemes}`);

const body = `${banner("src/objects/maybe-object.h.js", "src/objects/maybe-object.h + maybe-object-inl.h", "kClearedWeakHeapObjectLower32=" + clearedLower32 + " (globals.h)")}
//! MaybeObject 支撑面 (原版 maybe-object.{h,-inl.h})
//!
//! ClearedWeakValue 是弱引用槽的"已清除"哨兵: 上 32 位未置位的形式
//! (kClearedWeakValue, ${clearedLower32}) 不属于任何压缩 cage;
//! 运行时值必须带上所属 cage 的高位 (ClearedValue / ClearedTrustedValue).

/// 已清除弱值的低 32 位 (原版 v8-internal.h kClearedWeakHeapObjectLower32)
pub const K_CLEARED_WEAK_HEAP_OBJECT_LOWER32: u32 = ${clearedLower32};

/// 静态哨兵: 无 cage 归属的清除值 (原版 kClearedWeakValue, 比较用)
pub const K_CLEARED_WEAK_VALUE: usize = ${clearedLower32};

/// 主 cage 清除值 (原版 ClearedValue: 压缩开时 DecompressTagged(kClearedWeakHeapObjectLower32) 补主 cage 高位)
pub fn cleared_value(/* cage_base */) /* -> Tagged<ClearedWeakValue> */ {
  todo!("就地修订: 非压缩恒等返回 K_CLEARED_WEAK_VALUE; 压缩开补 V8HeapCompressionScheme 高位")
}

/// trusted cage 清除值 (原版 ClearedTrustedValue: 同上但走 TrustedSpaceCompressionScheme)
pub fn cleared_trusted_value(/* cage_base */) /* -> Tagged<ClearedWeakValue> */ {
  todo!("就地修订: 压缩开补 TrustedSpaceCompressionScheme 高位")
}

/// 更新堆对象引用槽, 保持弱/强标记位不变
///
/// 位运算 (原版 1:1): \`new_value | (old_value & kWeakHeapObjectMask)\`
/// 槽型约束: 仅 FullHeapObjectSlot / HeapObjectSlot.
pub fn update_heap_object_reference_slot<THeapObjectSlot>(/* slot: THeapObjectSlot, value: Tagged<HeapObject> */) {
  let _ = ::std::marker::PhantomData::<THeapObjectSlot>;
  todo!("就地修订: 读旧值断言非 Smi, 或运算保弱位后 store")
}
`;

const p = writeSrc("v8-objects-core", "maybe_object.rs", body);
console.log(`maybe-object.h.js: 已生成 ${p} (哨兵常量 ${clearedLower32}, 解压方案 ${decompressSchemes.length})`);
