#!/usr/bin/env bun
// tool/src/objects/js-array.tq.js — JSArray/JSArrayIterator/FastJSArrayWitness tq 布局提取
// 组→文件映射 (本脚本管辖): v8/src/objects/js-array.tq (单文件族)
//   → v8-objects-js-core/src/js_array_tq.rs.skel
// 配置登记: 64 位非压缩 (kTaggedSize=8, JSObject 头 = 3 tagged = 24)。
import { readV8, writeSrc, banner } from "../../lib.js";

const tq = readV8("src/objects/js-array.tq");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};
const has = (s, sub, f) => {
  checks++;
  if (!s.includes(sub)) throw new Error(`锚点失配: ${f} 无 "${sub.slice(0, 64)}"`);
};

// ── 1. IterationKind 枚举 (kKeys=0, kValues=1, kEntries=2) ───────────────
const ik = tq.match(/extern enum IterationKind extends uint31 \{([^}]*)\}/);
must(ik !== null, "IterationKind 枚举缺失");
const ikVals = ik[1].split(",").map((s) => s.trim().split(/\s+/)[0]).filter(Boolean);
must(JSON.stringify(ikVals) === JSON.stringify(["kKeys", "kValues", "kEntries"]),
  `IterationKind 值序失配: ${JSON.stringify(ikVals)}`);

// ── 2. tq 类字段真解析 ────────────────────────────────────────────────────
function tqFields(cls) {
  const seg = tq.slice(tq.indexOf(`extern class ${cls} `));
  const body = seg.slice(0, seg.indexOf("\n}"));
  return [...body.matchAll(/^  (\w+): ([^;]+);/gm)].map((m) => [m[1], m[2].trim()]);
}
const iterFields = tqFields("JSArrayIterator");
must(JSON.stringify(iterFields) === JSON.stringify([
  ["iterated_object", "JSReceiver"], ["next_index", "Number"], ["kind", "SmiTagged<IterationKind>"],
]), `JSArrayIterator 字段失配: ${JSON.stringify(iterFields)}`);
const jsArrayFields = tqFields("JSArray");
must(JSON.stringify(jsArrayFields) === JSON.stringify([["length", "Number"]]),
  `JSArray 字段失配: ${JSON.stringify(jsArrayFields)}`);
const tlFields = tqFields("TemplateLiteralObject");
must(JSON.stringify(tlFields) === JSON.stringify([
  ["raw", "JSArray"], ["function_literal_id", "Smi"], ["slot_id", "Smi"],
]), `TemplateLiteralObject 字段失配: ${JSON.stringify(tlFields)}`);

has(tq, "extern class JSArrayConstructor extends JSFunctionWithPrototype\n    generates 'JSFunction';",
  "JSArrayConstructor");
has(tq, "@hasSameInstanceTypeAsParent", "TemplateLiteralObject 注解");

// ── 3. transient 类型全集 (7) ──────────────────────────────────────────────
const transients = [...tq.matchAll(/^transient type (\w+)(?:\s+extends\s+(\w+))?/gm)]
  .map((m) => [m[1], m[2] ?? "-"]);
const EXPECT_TRANSIENT = [
  ["FastJSArray", "JSArray"], ["FastJSArrayForRead", "JSArray"],
  ["FastJSArrayForCopy", "FastJSArray"], ["FastJSArrayForConcat", "FastJSArrayForCopy"],
  ["FastJSArrayWithNoCustomIteration", "FastJSArray"],
  ["FastJSArrayForReadWithNoCustomIteration", "FastJSArrayForRead"],
  ["JSArrayForFastToString", "JSArray"],
];
must(transients.length === 7, `transient 应 7 个, 实得 ${transients.length}`);
for (let i = 0; i < 7; i++)
  must(transients[i][0] === EXPECT_TRANSIENT[i][0] && transients[i][1] === EXPECT_TRANSIENT[i][1],
    `transient[${i}] 失配: ${transients[i]} 期望 ${EXPECT_TRANSIENT[i]}`);

// ── 4. Witness struct 字段与 macro 对数 ───────────────────────────────────
function structFields(name) {
  const seg = tq.slice(tq.indexOf(`struct ${name} {`));
  const body = seg.slice(0, seg.indexOf("\n}"));
  return [...body.matchAll(/^  (?:@if\([^)]*\) )?(?:const )?(\w+): ([^;]+);/gm)]
    .map((m) => m[1]).filter((n) => n !== "macro");
}
const wFields = structFields("FastJSArrayWitness");
// hasDoubleHoles 仅 V8_ENABLE_UNDEFINED_DOUBLE 分支存在
must(wFields.includes("stable") && wFields.includes("unstable") && wFields.includes("map") &&
  wFields.includes("hasDoubles") && wFields.includes("hasSmis") && wFields.includes("arrayIsPushable"),
  `FastJSArrayWitness 字段失配: ${JSON.stringify(wFields)}`);
const rFields = structFields("FastJSArrayForReadWitness");
must(JSON.stringify(rFields.slice(0, 4)) === JSON.stringify(["stable", "unstable", "map", "hasDoubles"]),
  `FastJSArrayForReadWitness 字段失配: ${JSON.stringify(rFields)}`);
const wMacros = [...tq.matchAll(/macro (\w+)\(/g)].map((m) => m[1]);
for (const m of ["Get", "Recheck", "LoadElementNoHole", "StoreHole", "LoadElementOrUndefined",
  "EnsureArrayPushable", "ChangeLength", "Push", "MoveElements", "CreateArrayIterator",
  "NewJSArray", "IsEmpty"])
  must(wMacros.includes(m), `tq 缺 macro ${m}`);
has(tq, "macro NewFastJSArrayWitness(array: FastJSArray): FastJSArrayWitness", "NewFastJSArrayWitness");
has(tq, "macro NewFastJSArrayForReadWitness(array: FastJSArrayForRead):", "NewFastJSArrayForReadWitness");
has(tq, "macro TorqueMoveElementsSmi(", "TorqueMoveElementsSmi");
must((tq.match(/macro TorqueMoveElements\(/g) || []).length === 2, "TorqueMoveElements 重载应 2 个");
must((tq.match(/macro TorqueCopyElements\(/g) || []).length === 2, "TorqueCopyElements 重载应 2 个");

// ── 5. 64 位布局推导 ──────────────────────────────────────────────────────
const T = 8;
const K_JS_OBJECT_HEADER = 3 * T; // 24
const K_ARRAY_ITERATOR_HEADER = K_JS_OBJECT_HEADER + 3 * T; // 48
const K_JS_ARRAY_HEADER = K_JS_OBJECT_HEADER + T; // 32
const K_TEMPLATE_LITERAL_HEADER = K_JS_ARRAY_HEADER + 2 * T; // 48
must(K_ARRAY_ITERATOR_HEADER === 48 && K_JS_ARRAY_HEADER === 32 && K_TEMPLATE_LITERAL_HEADER === 48,
  "布局推导失配");

const body = `${banner("src/objects/js-array.tq.js", "src/objects/js-array.tq", "JSArray 族 tq 布局/迭代器枚举/Witness 骨架")}
//! 差集骨架: JSArray / JSArrayIterator / TemplateLiteralObject / FastJSArrayWitness (Torque 侧)。
//! 配置: 64 位非压缩 (kTaggedSize=8)。

/// v8: src/objects/js-array.tq extern enum IterationKind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IterationKind { Keys = 0, Values = 1, Entries = 2 }

/// v8: src/objects/js-array.tq JSArrayIterator 迭代终止哨兵: JSArray → kMaxUInt32;
/// JSTypedArray → Smi::kMaxValue; 其他 JSReceiver → kMaxSafeInteger (语义面留 Developer)

/// v8: src/objects/js-array.tq JSArrayIterator: iterated_object offset=${K_JS_OBJECT_HEADER},
///   next_index offset=${K_JS_OBJECT_HEADER + T}, kind offset=${K_JS_OBJECT_HEADER + 2 * T}
pub const K_ARRAY_ITERATOR_HEADER_SIZE: usize = ${K_ARRAY_ITERATOR_HEADER};
/// v8: src/objects/js-array.tq JSArray: length offset=${K_JS_OBJECT_HEADER}
pub const K_JS_ARRAY_HEADER_SIZE: usize = ${K_JS_ARRAY_HEADER};
/// v8: src/objects/js-array.tq TemplateLiteralObject: raw=${K_JS_ARRAY_HEADER},
///   function_literal_id=${K_JS_ARRAY_HEADER + T}, slot_id=${K_JS_ARRAY_HEADER + 2 * T}
pub const K_TEMPLATE_LITERAL_OBJECT_HEADER_SIZE: usize = ${K_TEMPLATE_LITERAL_HEADER};

/// v8: src/objects/js-array.tq transient 类型 (protector 守卫的快速路径视图)
pub const FAST_ARRAY_TRANSIENTS: &[(&str, &str)] = &[
${transients.map(([a, b]) => `    ("${a}", "${b}"),`).join("\n")}
];

/// v8: src/objects/js-array.tq struct FastJSArrayWitness (字段 ${wFields.join(", ")};
///   hasDoubleHoles 仅 V8_ENABLE_UNDEFINED_DOUBLE) — 语义面 (Recheck/LoadElementNoHole/
///   StoreHole/EnsureArrayPushable/ChangeLength/Push/MoveElements) 留 Developer 修订
pub struct FastJSArrayWitness<'a> { /* stable/unstable/map/hasDoubles/hasSmis/arrayIsPushable */ }

/// v8: src/objects/js-array.tq struct FastJSArrayForReadWitness (Get/Recheck/LoadElementNoHole)
pub struct FastJSArrayForReadWitness<'a> { /* stable/unstable/map/hasDoubles */ }

/// v8: src/objects/js-array.tq TorqueMoveElements/TorqueMoveElementsSmi/TorqueCopyElements
/// — elements 搬移入口 (HOLEY_{SMI,DOUBLE,}_ELEMENTS 三通道), 语义面留 Developer
`;

const p = writeSrc("v8-objects-js-core", "js_array_tq.rs.skel", body);
console.log(
  `js-array.tq.js: ${p} — 枚举 3 值、类字段 ${iterFields.length}+${jsArrayFields.length}+${tlFields.length}、` +
    `transient ${transients.length}/7、witness 字段 ${wFields.length}/${rFields.length}、断言 ${checks} 项通过`,
);
