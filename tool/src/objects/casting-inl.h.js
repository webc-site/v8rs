#!/usr/bin/env bun
// tool/src/objects/casting-inl.h.js — GC 感知类型检查生成
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/casting-inl.h (+ casting.h 谓词/转换面登记) → v8-objects-core/src/casting_inl.rs
//   casting.h 本体不在本席位 (差集登记, 见产物 doc)
// 规则面: GCAwareObjectTypeCheck 的 4 种 GC 场景判定 (DEBUG-only 帮助函数)
import { readV8, writeSrc, banner } from "../../lib.js";

const inl = readV8("src/objects/casting-inl.h");
const h = readV8("src/objects/casting.h");

// —— 锚点断言 ——
if (!/template <typename T>\nbool GCAwareObjectTypeCheck\(Tagged<Object> object, const Heap\* heap\)/.test(inl))
  throw new Error("GCAwareObjectTypeCheck 模板签名缺失");
if (!/#ifdef DEBUG/.test(inl) || !/#endif  \/\/ DEBUG/.test(inl))
  throw new Error("GCAwareObjectTypeCheck 应整体位于 #ifdef DEBUG 内");
// 4 场景注释锚 (原版 1)..4) 编号)
for (const anchor of ["1) A scavenger is ongoing", "2) A conservative scavenge is ongoing", "3) A GC is ongoing", "4) If none of the above special cases apply"])
  if (!inl.includes(anchor)) throw new Error(`场景锚缺失: ${anchor}`);

// casting.h 侧规则面登记 (差集对数): Cast/UncheckedCast/TryCast/Is 家族数量
for (const need of ["struct CastTraits", "inline Tagged<To> UncheckedCast(Tagged<From> value)", "inline bool TryCast", "inline bool Is(Tagged<U> value)", "is_subtype_v"])
  if (!h.includes(need)) throw new Error(`casting.h 缺 ${need}`);

const body = `${banner("src/objects/casting-inl.h.js", "src/objects/casting-inl.h (+ casting.h 家族登记)", "GCAwareObjectTypeCheck 语义骨架")}
//! GC 感知对象类型检查 (原版 casting-inl.h)
//!
//! 原版仅一个 DEBUG-only 模板函数 \`GCAwareObjectTypeCheck<T>\`:
//! 在 GC 进行中 map 字可能被改为转发指针, 此时对象类型检查需按 GC 场景放宽.
//! 其余 casting 规则 (Cast/UncheckedCast/TryCast/Is) 定义于 casting.h,
//! 归属其他席位, 本模块只登记 GC 场景语义 (开发期断言用).

/// GC 场景判定枚举 (原版函数内 4 个编号分支, ${4} 种)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GcObjectTypeCheckCase {
  /// 1) Scavenge 进行中且对象为已提升的大对象页 (map 检查跳过)
  PromotedLargeObjectInScavenge,
  /// 2) 保守栈扫描下被钉住的年轻代对象, map 字已是自转发地址
  PinnedByConservativeStackScanning,
  /// 3) GC 进行中且对象已被疏散, 转发目标是 T
  EvacuatedToTargetOfT,
  /// 4) 常规情形: 退回 Is<T>(object)
  RegularIsCheck,
}

/// GC 感知类型检查 (原版 GCAwareObjectTypeCheck<T>, DEBUG 构建)
///
/// 场景 2 的前置条件 (原版逐条):
/// - gc_state == SCAVENGE 且 InYoungGeneration
/// - ConservativeStackScanningModeForMinorGC != kNone 或 ShouldUsePrecisePinningForMinorGC
/// - map_word.IsForwardingAddress() 且 HeapLayout::IsSelfForwarded
pub fn gc_aware_object_type_check<T>(/* object: Tagged<Object>, heap: &Heap */) -> bool {
  let _ = T;
  todo!("就地修订: 按四场景依次判定, 未命中走 Is::<T>")
}
`;

const p = writeSrc("v8-objects-core", "casting_inl.rs", body);
console.log(`casting-inl.h.js: 已生成 ${p} (GC 场景 4, casting.h 锚 5)`);
