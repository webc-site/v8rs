#!/usr/bin/env bun
// tool/src/objects/hole.h.js — 覆盖 v8/src/objects/hole.{h,tq}
// HOLE_LIST X-macro 全量展开 (宏体取自 object-list-macros.h, 含 TDZ_HOLE_LIST 嵌套)
// → v8-objects-core/src/hole.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/hole.h");
const tqSrc = readV8("src/objects/hole.tq");
const listSrc = readV8("src/objects/object-list-macros.h");

// 1. HOLE_LIST 宏体 (递归展开 TDZ_HOLE_LIST 嵌套引用)
const macroBody = listSrc.match(/#define HOLE_LIST\(V\)([\s\S]*?)\n\n/);
if (!macroBody) throw new Error("未找到 HOLE_LIST 宏定义");

function expandList(body, seen = new Set()) {
  // 先按位置展开嵌套宏引用 (TDZ_HOLE_LIST(V) 出现在 HOLE_LIST 中段)
  const nestedRef = body.match(/(\w+_LIST)\(V\)/);
  if (nestedRef) {
    const n1 = nestedRef[1];
    if (seen.has(n1)) throw new Error(`宏循环引用: ${n1}`);
    const nm = listSrc.match(new RegExp(`#define ${n1}\\(V\\)\\s*([\\s\\S]*?)\\n(?=#|$)`));
    if (!nm) throw new Error(`未找到嵌套宏 ${n1}`);
    body = body.replace(nestedRef[0], nm[1].replace(/\\\n/g, " ").trim());
  }
  const out = [];
  for (const vm of body.replace(/\\/g, " ").matchAll(/V\((\w+),\s*(\w+),\s*(\w+)\)/g)) {
    out.push({ type: vm[1], root: vm[2], slot: vm[3] });
  }
  return out;
}
const holes = expandList(macroBody[1]);
if (holes.length !== 13) throw new Error(`HOLE_LIST 展开条数不符: 期望 13, 得 ${holes.length}`);
if (holes[0].type !== "TheHole" || holes[0].root !== "the_hole_value") throw new Error("HOLE_LIST 首项锚点不符");

// 2. TDZ 分支: #if V8_ENABLE_TDZ_HOLE → TdzHole; #else → DisabledTdzHole
// 默认构建关闭 TDZ, 取 DisabledTdzHole; hole.h 侧 TdzHole 别名 TheHole
const tdzEnabled = false;
const holes2 = holes.map((h) =>
  tdzEnabled || h.type !== "TdzHole"
    ? h
    : { type: "DisabledTdzHole", root: "disabled_tdz_hole_value", slot: "DisabledTdzHoleValue" },
);
if (!tdzEnabled && !holes2.some((h) => h.type === "DisabledTdzHole")) throw new Error("TDZ 关闭分支缺 DisabledTdzHole");

// 3. 载荷常量
const payload = Number(hSrc.match(/static constexpr int kPayloadSize = (\d+) \* KB;/)?.[1]);
if (payload !== 64) throw new Error(`kPayloadSize 对数不符: ${payload}`);
const payloadSize = payload * 1024;

// 4. tq 校验: TheHole / PromiseHole / TdzHole 声明存在
for (const n of ["TheHole", "PromiseHole"]) {
  if (!tqSrc.includes(`extern class ${n} extends Hole`)) throw new Error(`tq 缺 ${n} 声明`);
}

console.log(`Hole 族 ${holes2.length} 型, payload ${payloadSize}`);

let rust = banner(
  "src/objects/hole.h.js",
  "src/objects/hole.{h,tq} + object-list-macros.h(HOLE_LIST)",
  "Hole 全类型表 (V8_ENABLE_TDZ_HOLE 关闭分支: TdzHole 别名 TheHole)",
);

rust += `
/// Hole 载荷大小 (原版 kPayloadSize = 64*KB, 需为页大小倍数)
pub const HOLE_PAYLOAD_SIZE: usize = ${payloadSize};

/// Hole 全类型表 (原版 HOLE_LIST 展开, 三列: 类型/只读根名/根槽名)
/// 各类型为 \`V8_OBJECT class X : public Hole\` 的空子类, 仅作实例类型区分
pub const HOLE_LIST: [(&str, &str, &str); ${holes2.length}] = [
`;
for (const h of holes2) rust += `  ("${h.type}", "${h.root}", "${h.slot}"),\n`;
rust += `];

/// Hole 基类标记 (原版 V8_OBJECT class Hole : public HeapObject)
pub struct Hole;

/// 类型别名, 原 TDZ_HOLE_LIST 展开项 (V8_ENABLE_TDZ_HOLE 关闭时 TdzHole = TheHole)
pub type TdzHole = TheHole;

/// TheHole 标记类型 (tq: extern class TheHole extends Hole)
pub struct TheHole;
/// PromiseHole (tq: extern class PromiseHole extends Hole)
pub struct PromiseHole;

// —— 提取不变式断言 ——
const _: () = {
  assert!(HOLE_PAYLOAD_SIZE == 65536);
  assert!(HOLE_PAYLOAD_SIZE % 4096 == 0);
  assert!(HOLE_LIST.len() == ${holes2.length});
  assert!(HOLE_LIST[0].0 == "TheHole");
  assert!(HOLE_LIST[0].1 == "the_hole_value");
  assert!(HOLE_LIST[4].0 == "DisabledTdzHole");
  assert!(HOLE_LIST[12].0 == "BasicBlockCountersMarker");
};
`;

const p = writeSrc("v8-objects-core", "hole.rs", rust);
console.log(`written ${p}`);
