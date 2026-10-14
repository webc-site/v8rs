#!/usr/bin/env bun
// tool/src/objects/hash-seed-wrapper.h.js — 覆盖 v8/src/objects/hash-seed-wrapper.{h,-inl.h}
// → v8-objects-rest/src/hash_seed_wrapper.rs
// 机械提取: 布局字段 (UnalignedValueMember<HashSeed::Data>)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/hash-seed-wrapper.h");

const classRe = hSrc.match(/V8_OBJECT class HashSeedWrapper : public (\w+)\s*\{([\s\S]*?)\} V8_OBJECT_END;/);
if (!classRe) throw new Error("未找到 HashSeedWrapper 类体");
if (classRe[1] !== "HeapObject") throw new Error(`基类失配: ${classRe[1]}`);
const member = classRe[2].match(/^\s{2}UnalignedValueMember<([^>]+)> (\w+);$/m);
if (!member || member[1] !== "HashSeed::Data" || member[2] !== "data_") {
  throw new Error(`布局成员失配: ${member?.slice(1).join(",")}`);
}
// 注解锚
if (!hSrc.includes("V8_IT_NO_AUTO_DISPATCH")) throw new Error("缺 V8_IT_NO_AUTO_DISPATCH 注解");

let rust = banner("src/objects/hash-seed-wrapper.h.js", "src/objects/hash-seed-wrapper.{h,-inl.h}", "HashSeedWrapper 布局登记");
rust += `//! HashSeedWrapper: 只读堆中的哈希种子包装 (单字段非对齐载荷)。

/// 布局字段 (h 侧唯一成员; 非对齐值成员)。
pub const HASH_SEED_WRAPPER_FIELDS: [(&str, &str); 1] = [("data_", "HashSeed::Data")];

// —— 提取不变式断言 ——
const _: () = {
  assert!(HASH_SEED_WRAPPER_FIELDS.len() == 1);
  assert!(HASH_SEED_WRAPPER_FIELDS[0].0 == "data_");
};
`;

const p = writeSrc("v8-objects-rest", "hash_seed_wrapper.rs", rust);
console.log(`hash_seed_wrapper.rs: 1 字段布局登记, 对数通过`);
