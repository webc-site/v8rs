#!/usr/bin/env bun
// tool/src/objects/megadom-handler.h.js — 覆盖 v8/src/objects/megadom-handler.{h,-inl.h,tq}
// → v8-objects-rest/src/megadom_handler.rs
// 机械提取: tq 布局字段表 + h 侧成员 1:1 对数

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/megadom-handler.h");
const tqSrc = readV8("src/objects/megadom-handler.tq");

// tq 布局
const tqRe = tqSrc.match(/extern class MegaDomHandler extends (\w+)\s*\{([\s\S]*?)\n\}/);
if (!tqRe) throw new Error("未找到 tq MegaDomHandler");
if (tqRe[1] !== "HeapObject") throw new Error(`基类失配: ${tqRe[1]}`);
const tqFields = [...tqRe[2].matchAll(/^\s{2}(\w+):\s*([^;]+);/gm)].map((m) => ({ name: m[1], type: m[2].trim() }));
if (tqFields.length !== 2) throw new Error(`tq 字段数 ${tqFields.length} != 2`);
for (const f of tqFields) if (f.type !== "MaybeObject") throw new Error(`字段类型失配: ${f.name}`);

// h 侧成员 (TaggedMember<MaybeObject> accessor_; context_;) 1:1
const hMembers = [...hSrc.matchAll(/^\s{2}TaggedMember<MaybeObject> (\w+);$/gm)].map((m) => m[1]);
if (hMembers.join(",") !== "accessor_,context_") throw new Error(`h 侧成员失配: ${hMembers.join(",")}`);
// tq 名 ↔ h 成员名对应 (accessor ↔ accessor_ 等去尾下划线)
for (let i = 0; i < 2; i++) {
  if (tqFields[i].name + "_" !== hMembers[i]) throw new Error(`tq/h 名失配 @${i}`);
}

let rust = banner("src/objects/megadom-handler.h.js", "src/objects/megadom-handler.{h,-inl.h,tq}", "MegaDomHandler 布局字段表");
rust += `//! MegaDomHandler: mega-dom 桩访问器对 (accessor + context, 均为弱引用位 MaybeObject)。

/// 布局字段表 (megadom-handler.tq, 顺序 1:1)。
pub const MEGADOM_HANDLER_FIELDS: [(&str, &str); ${tqFields.length}] = [
`;
for (const f of tqFields) rust += `  ("${f.name}", "${f.type}"),\n`;
rust += `];

/// 字段槽总数。
pub const FIELD_COUNT: usize = ${tqFields.length};

// —— 提取不变式断言 ——
const _: () = {
  assert!(FIELD_COUNT == 2);
  assert!(MEGADOM_HANDLER_FIELDS[0] == ("accessor", "MaybeObject"));
  assert!(MEGADOM_HANDLER_FIELDS[1] == ("context", "MaybeObject"));
};
`;

const p = writeSrc("v8-objects-rest", "megadom_handler.rs", rust);
console.log(`megadom_handler.rs: 2 字段布局表 (tq/h 1:1), 对数通过`);
