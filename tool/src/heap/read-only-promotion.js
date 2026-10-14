#!/usr/bin/env bun
// tool/src/heap/read-only-promotion.js
// 机械提取 v8/src/heap/read-only-promotion{.h,.cc} 族:
// ReadOnlyPromotion::Promote — 只读堆提升 (全静态)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/read-only-promotion.h");
const ccSrc = readV8("src/heap/read-only-promotion.cc");

const anchors = [
  [hSrc, "class ReadOnlyPromotion final : public AllStatic {"],
  [hSrc, "V8_EXPORT_PRIVATE static void Promote(Isolate* isolate,"],
  [hSrc, "const DisallowGarbageCollection& no_gc);"],
  [ccSrc, "void ReadOnlyPromotion::Promote("],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

console.log(`read-only-promotion: 静态入口 1 (Promote), AllStatic 无状态`);

let rust = banner(
  "src/heap/read-only-promotion.js",
  "src/heap/read-only-promotion.h, src/heap/read-only-promotion.cc",
  "启动时把老生代部分页提升为只读页",
);

rust += `/// v8: src/heap/read-only-promotion.h ReadOnlyPromotion (AllStatic)
pub struct ReadOnlyPromotion;

impl ReadOnlyPromotion {
  /// v8: ReadOnlyPromotion::Promote(isolate, safepoint_scope, no_gc)
  /// 语义: 遍历 ReadOnlyRoots 引用闭包, 迁移对象到只读空间,
  ///       去写屏障, 页转 ReadOnlyPage; 需在 safepoint 下运行。
  pub fn promote(isolate: &Isolate, safepoint_scope: &SafepointScope, no_gc: &DisallowGarbageCollection) { todo!() }
}
`;

const p = writeSrc("v8-heap-core", "read_only_promotion.rs.skel", rust);
console.log(`written ${p}`);
