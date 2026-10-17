#!/usr/bin/env bun
// tool/src/deoptimizer/materialized-object-store.cc.js
// 机械提取 v8/src/deoptimizer/materialized-object-store.cc — 骨架。
// 产物: v8-deoptimizer/src/materialized_object_store.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const CC_REL = "src/deoptimizer/materialized-object-store.cc";
const cc = readV8(CC_REL);

const sigs = [...cc.matchAll(/^[\w:<>,&*\s]+?MaterializedObjectStore::(\w+)\(/gm)].map((m) => m[1]);
if (sigs.length < 6) throw new Error(`方法面不足: ${sigs.length}`);
if (!cc.includes("StackIdToIndex")) throw new Error("缺 StackIdToIndex 锚");

let rust = banner("src/deoptimizer/materialized-object-store.cc.js", CC_REL, "物化对象存储骨架");
rust += `
// 原 class MaterializedObjectStore — 以 stack_id 索引的物化对象栈。
// 方法注册表 (${[...new Set(sigs)].length} 条):
${[...new Set(sigs)].map((s) => `// - ${s}`).join("\n")}
// 字段面: stack_id_ 向量 (每项为一帧的物化对象组)

pub struct MaterializedObjectStore;

// TODO(dev): 补存储与移除实现。

// —— 提取不变式断言 ——
const _: () = {};
`;

const p = writeSrc("v8-deoptimizer", "materialized_object_store.rs", rust);
console.log(`written ${p} (方法 ${[...new Set(sigs)].length})`);
