#!/usr/bin/env bun
// tool/src/objects/oddball-predicates.h.js — 覆盖 v8/src/objects/oddball-predicates.{h,-inl.h}
// Is<Type> 谓词声明表 (ODDBALL_LIST/HOLE_LIST X-macro 展开)
// → v8-objects-core/src/oddball_predicates.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/oddball-predicates.h");
const inlSrc = readV8("src/objects/oddball-predicates-inl.h");
const listSrc = readV8("src/objects/object-list-macros.h");

// 1. ODDBALL_LIST 展开 (4 项)
const oddballBody = listSrc.match(/#define ODDBALL_LIST\(V\)([\s\S]*?)\n\n/);
if (!oddballBody) throw new Error("未找到 ODDBALL_LIST 宏体");
const oddballs = [...oddballBody[1].matchAll(/V\((\w+),\s*(\w+),\s*(\w+)\)/g)].map((m) => m[1]);
if (oddballs.length !== 4) throw new Error(`ODDBALL_LIST 条数不符: ${oddballs.length}, 期望 4`);
if (oddballs[0] !== "Undefined") throw new Error("ODDBALL_LIST 首项锚点不符");

// 2. HOLE_LIST 展开 (TDZ 关闭分支: DisabledTdzHole, 嵌套宏按位内联)
const holeBody = listSrc.match(/#define HOLE_LIST\(V\)([\s\S]*?)\n\n/);
if (!holeBody) throw new Error("未找到 HOLE_LIST 宏体");
const tdzPos = listSrc.indexOf('#define TDZ_HOLE_LIST', listSrc.indexOf('#else', listSrc.indexOf('#ifdef V8_ENABLE_TDZ_HOLE')));
if (tdzPos === -1) throw new Error("未找到 TDZ_HOLE_LIST #else 分支");
const tdzElse = listSrc.slice(tdzPos, tdzPos + 200).match(/V\((\w+), (\w+), (\w+)\)/);
const holeFlat = holeBody[1].replace("TDZ_HOLE_LIST(V)", `V(${tdzElse[1]}, ${tdzElse[2]}, ${tdzElse[3]})`);
let holeNames = [...holeFlat.replace(/\\/g, " ").matchAll(/V\((\w+),/g)].map((m) => m[1]);
if (holeNames.length !== 13) throw new Error(`HOLE_LIST 条数不符: ${holeNames.length}, 期望 13`);

// 3. 额外谓词 (h 侧直书项)
const extras = ["TdzHole", "UndefinedContextCell", "NullOrUndefined"];
for (const e of extras) {
  if (!hSrc.includes(`IS_TYPE_FUNCTION_DECL(${e})`)) throw new Error(`缺谓词声明 ${e}`);
}

// 4. inl 侧实现对数: 实现为 X-macro 生成 (Is##Type), 对数按结构锚点
//    两级块: 包装层 (HeapObject→Object 委托) + 主体层 (静态根比较/动态根比较)
const inlAnchors = [
  "ODDBALL_LIST(IS_TYPE_FUNCTION_DEF)",
  "HOLE_LIST(IS_TYPE_FUNCTION_DEF)",
  "IS_TYPE_FUNCTION_DEF(TdzHole)",
  "IS_TYPE_FUNCTION_DEF(UndefinedContextCell",
  "bool IsNullOrUndefined(Tagged<Object> obj)",
];
for (const a of inlAnchors) {
  if (!inlSrc.includes(a)) throw new Error(`inl 缺结构锚点: ${a}`);
}
// TDZ 关闭分支: TdzHole 与 UndefinedContextCell 参与生成, DisabledTdzHole 无独立谓词
const expected = [...oddballs, ...holeNames.filter((n) => n !== "DisabledTdzHole"), "TdzHole", "UndefinedContextCell", "NullOrUndefined"];

console.log(`oddball-predicates 谓词 ${expected.length} 个 (oddball ${oddballs.length} + hole ${holeNames.length - 1} + 直书 3)`);

let rust = banner(
  "src/objects/oddball-predicates.h.js",
  "src/objects/oddball-predicates.{h,-inl.h}",
  "Oddball/Hole 类型谓词表 (V8_ENABLE_TDZ_HOLE 关闭分支)",
);

rust += `
/// 谓词函数名全表 (原版 IS_TYPE_FUNCTION_DECL 展开序: ODDBALL_LIST → HOLE_LIST → 直书项)
/// 每项对应 inl 侧 bool Is<Type>(Tagged<Object>) 实现
pub const PREDICATES: [&str; ${expected.length}] = [
${expected.map((n) => `  "${n}",`).join("\n")}
];

/// Oddball 项 (ODDBALL_LIST)
pub const ODDBALL_PREDICATES: [&str; ${oddballs.length}] = [
${oddballs.map((n) => `  "${n}",`).join("\n")}
];

/// Hole 项 (HOLE_LIST, TDZ 关闭分支)
pub const HOLE_PREDICATES: [&str; ${holeNames.length}] = [
${holeNames.map((n) => `  "${n}",`).join("\n")}
];

// —— 签名骨架 (原版公开接口, 供 Developer 补语义) ——
// 每谓词两个重载: bool Is<Type>(Tagged<Object>, EarlyReadOnlyRoots) / bool Is<Type>(Tagged<Object>)
// inl 实现形态: 位比较 (oddball.kind()/roots 判等), 语义待 Developer 就地接根表
pub const PREDICATE_OVERLOADS: usize = 2;

// —— 提取不变式断言 ——
const _: () = {
  assert!(PREDICATES.len() == ${expected.length});
  assert!(PREDICATES[0] == "Undefined");
  assert!(ODDBALL_PREDICATES.len() == 4);
  assert!(HOLE_PREDICATES[0] == "TheHole");
  assert!(HOLE_PREDICATES[4] == "DisabledTdzHole");
};
`;

const p = writeSrc("v8-objects-core", "oddball_predicates.rs", rust);
console.log(`written ${p}`);
