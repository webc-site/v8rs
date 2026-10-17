#!/usr/bin/env bun
// tool/src/objects/cell.h.js — 覆盖 v8/src/objects/cell.{h,-inl.h,tq}
// Cell 单值槽对象签名骨架
// → v8-objects-rest/src/cell.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/cell.h");
const inlSrc = readV8("src/objects/cell-inl.h");
const tqSrc = readV8("src/objects/cell.tq");

// 结构对数: h 侧类声明与字段, inl 侧访问器, tq 侧布局
const anchors = [
  ["h", hSrc, "V8_OBJECT class Cell : public HeapObject"],
  ["h", hSrc, "TaggedMember<MaybeObject> maybe_value_;"],
  ["inl", inlSrc, "Tagged<MaybeObject> Cell::maybe_value()"],
  ["inl", inlSrc, "void Cell::set_value("],
  ["tq", tqSrc, "extern class Cell extends HeapObject"],
];
for (const [tag, src, a] of anchors) {
  if (!src.includes(a)) throw new Error(`cell.${tag} 缺锚点: ${a}`);
}
// 访问器族计数 (inl 侧 Cell:: 方法)
const methods = [...inlSrc.matchAll(/^[\s\S]*?Cell::(\w+)\(/gm)].map((m) => m[1]);
const uniq = [...new Set(methods)];
if (uniq.length < 4) throw new Error(`inl 访问器数异常: ${uniq.length}`);

console.log(`Cell 锚点 ${anchors.length}, inl 访问器 ${uniq.length}`);

let rust = banner("src/objects/cell.h.js", "src/objects/cell.{h,-inl.h,tq}", "Cell 单值槽对象骨架");

rust += `
/// 单值槽 (原版 V8_OBJECT class Cell : public HeapObject)
/// 布局 (tq): map + maybe_value (MaybeObject, 可为弱引用/Smi 清空值)
pub struct Cell;

impl Cell {
  /// 唯一字段偏移提示: maybe_value 紧随 map (TaggedMember<MaybeObject>)
  /// 数值常量由布局求解器统一定版, 此处不臆测
  pub const FIELD: &str = "maybe_value";
}

// —— 签名骨架 (原版公开接口, 供 Developer 补语义) ——
// explicit Cell(const AllocationWitness&);
// Cell(const AllocationWitness&, Tagged<Smi>);
// Cell(const AllocationWitness&, Tagged<ClearedWeakValue>);
// Tagged<MaybeObject> maybe_value() const; (+RelaxedLoadTag 重载)
// void set_maybe_value(Tagged<MaybeObject>, WriteBarrierMode);
// Tagged<Object> value() const; (+RelaxedLoadTag 重载)
// void set_value(Tagged<Object>, WriteBarrierMode);
// DECL_PRINTER(Cell) / DECL_VERIFIER(Cell)
pub const CELL_SIGNATURES: [&str; 8] = [
  "ctor_witness",
  "ctor_smi",
  "ctor_cleared_weak",
  "maybe_value",
  "set_maybe_value",
  "value",
  "set_value",
  "verify_print",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(CELL_SIGNATURES.len() == 8);
  assert!(Cell::FIELD == "maybe_value");
};
`;

const p = writeSrc("v8-objects-rest", "cell.rs", rust);
console.log(`written ${p}`);
