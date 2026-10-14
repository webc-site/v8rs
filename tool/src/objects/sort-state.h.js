#!/usr/bin/env bun
// tool/src/objects/sort-state.h.js — 覆盖 v8/src/objects/sort-state.{h,-inl.h}
// → v8-objects-rest/src/sort_state.rs
// 机械提取: SortState 布局字段表 (13 个 TaggedMember, 全由 Torque 侧访问)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/sort-state.h");

// 类体成员 (TaggedMember<T> name_; 顺序 1:1)
const classRe = hSrc.match(/V8_OBJECT class SortState : public (\w+)\s*\{([\s\S]*?)\} V8_OBJECT_END;/);
if (!classRe) throw new Error("未找到 SortState 类体");
if (classRe[1] !== "HeapObject") throw new Error(`SortState 基类失配: ${classRe[1]}`);
const fields = [...classRe[2].matchAll(/^\s{2}(?:const )?TaggedMember<(.+?)> (\w+);$/gm)].map((m) => ({ type: m[1], name: m[2] }));
if (fields.length !== 13) throw new Error(`SortState 字段数 ${fields.length} != 13`);
// 顺序锚 (首尾)
if (fields[0].name !== "receiver_" || fields[fields.length - 1].name !== "number_of_undefined_") {
  throw new Error("SortState 字段顺序锚失配");
}
// SizeFor 声明存在 (sizeof 语义, 值不可静态落)
if (!hSrc.includes("static constexpr int SizeFor()")) throw new Error("未找到 SizeFor 声明");

let rust = banner("src/objects/sort-state.h.js", "src/objects/sort-state.{h,-inl.h}", "SortState 布局字段表 (13 槽)");
rust += `//! SortState: Array.prototype.sort 的 Torque 侧状态对象。
//! 字段全部由 Torque 生成代码访问 (原版无 C++ 访问器); 偏移由布局引擎计算,
//! 此处登记字段清单与类型 1:1。

/// SortState 布局字段表 (sort-state.h 成员声明顺序, 共 ${fields.length} 槽)。
pub const SORT_STATE_FIELDS: [(&str, &str); ${fields.length}] = [
`;
for (const f of fields) rust += `  ("${f.name}", "${f.type}"),\n`;
rust += `];

/// 字段槽总数。
pub const FIELD_COUNT: usize = ${fields.length};

// —— 提取不变式断言 ——
const _: () = {
  assert!(FIELD_COUNT == 13);
  assert!(SORT_STATE_FIELDS[0] == ("receiver_", "JSReceiver"));
  assert!(SORT_STATE_FIELDS[FIELD_COUNT - 1].0 == "number_of_undefined_");
  assert!(SORT_STATE_FIELDS.iter().all(|(_, t)| !t.is_empty()));
};
`;

const p = writeSrc("v8-objects-rest", "sort_state.rs", rust);
console.log(`sort_state.rs: 13 字段表, 对数通过`);
