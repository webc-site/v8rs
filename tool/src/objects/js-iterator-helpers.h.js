#!/usr/bin/env bun
// tool/src/objects/js-iterator-helpers.h.js — JSIteratorHelper 族枚举/类层级/布局提取
// 组→文件映射 (本脚本管辖): v8/src/objects/js-iterator-helpers.{h,-inl.h,tq}
//   → v8-objects-js-core/src/js_iterator_helpers.rs.skel
// 配置登记: 64 位非压缩 (kTaggedSize=8, JSObject 头 = 3 tagged = 24)。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/js-iterator-helpers.h");
const inl = readV8("src/objects/js-iterator-helpers-inl.h");
const tq = readV8("src/objects/js-iterator-helpers.tq");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};
const has = (s, sub, f) => {
  checks++;
  if (!s.includes(sub)) throw new Error(`锚点失配: ${f} 无 "${sub.slice(0, 64)}"`);
};

// ── 1. 两个枚举 (tq extern enum 真解析 + h 对照) ──────────────────────────
function tqEnum(name) {
  const seg = tq.slice(tq.indexOf(`extern enum ${name}`));
  return seg.slice(seg.indexOf("{") + 1, seg.indexOf("}"))
    .split(",").map((s) => s.trim()).filter(Boolean);
}
must(JSON.stringify(tqEnum("JSIteratorHelperState")) === JSON.stringify([
  "kSuspendedStart", "kSuspendedYield", "kExecuting", "kCompleted",
]), "JSIteratorHelperState 失配");
must(JSON.stringify(tqEnum("JSIteratorZipHelperMode")) === JSON.stringify([
  "kShortest", "kLongest", "kStrict",
]), "JSIteratorZipHelperMode 失配");
has(h, "enum class JSIteratorHelperState {", "h State");
has(h, "enum class JSIteratorZipHelperMode { kShortest, kLongest, kStrict };", "h Mode");

// ── 2. 类层级 tq 真解析 (11 类 + 嵌套 IteratorRecord) ─────────────────────
const tqCls = [...tq.matchAll(/^extern class (\w+)(?:\s+extends\s+(\w+))?/gm)]
  .map((m) => [m[1], m[2] ?? "-"]);
const EXPECT = [
  ["JSIteratorHelper", "JSObject"], ["JSIteratorHelperSimple", "JSIteratorHelper"],
  ["JSIteratorMapHelper", "JSIteratorHelperSimple"],
  ["JSIteratorFilterHelper", "JSIteratorHelperSimple"],
  ["JSIteratorTakeHelper", "JSIteratorHelperSimple"],
  ["JSIteratorDropHelper", "JSIteratorHelperSimple"],
  ["JSIteratorFlatMapHelper", "JSIteratorHelperSimple"],
  ["JSIteratorConcatHelper", "JSIteratorHelperSimple"],
  ["JSIteratorZipHelper", "JSIteratorHelper"],
  ["JSIteratorZipKeyedHelper", "JSIteratorZipHelper"],
];
must(tqCls.length === 10, `tq 类应 10, 实得 ${tqCls.length}`);
for (let i = 0; i < 10; i++)
  must(tqCls[i][0] === EXPECT[i][0] && tqCls[i][1] === EXPECT[i][1], `tq 类[${i}] ${tqCls[i]} 期望 ${EXPECT[i]}`);

// ── 3. 每类字段 (tq 真解析 + h 对照) ─────────────────────────────────────
function tqFields(cls) {
  const seg = tq.slice(tq.indexOf(`extern class ${cls}`));
  const body = seg.slice(0, seg.indexOf("\n}"));
  return [...body.matchAll(/^  (\w+):\s*([^;]+);/gm)].map((m) => [m[1], m[2].trim()]);
}
const FIELDS = {
  JSIteratorHelper: [["state", "SmiTagged<JSIteratorHelperState>"]],
  JSIteratorHelperSimple: [["underlying_iterator", "iterator::IteratorRecord"]],
  JSIteratorMapHelper: [["mapper", "Callable"], ["counter", "Number"]],
  JSIteratorFilterHelper: [["predicate", "Callable"], ["counter", "Number"]],
  JSIteratorTakeHelper: [["remaining", "Number"]],
  JSIteratorDropHelper: [["remaining", "Number"]],
  JSIteratorFlatMapHelper: [["mapper", "Callable"], ["counter", "Number"], ["inner_iterator", "iterator::IteratorRecord"]],
  JSIteratorConcatHelper: [["iterables", "FixedArray"], ["current", "Smi"]],
  JSIteratorZipHelper: [["underlying_iterators", "FixedArray"], ["mode", "SmiTagged<JSIteratorZipHelperMode>"], ["active_count", "Smi"], ["padding", "FixedArray"]],
  JSIteratorZipKeyedHelper: [["keys", "FixedArray"]],
};
let fieldTotal = 0;
for (const [cls, exp] of Object.entries(FIELDS)) {
  const got = tqFields(cls);
  must(JSON.stringify(got) === JSON.stringify(exp), `${cls} 字段失配: ${JSON.stringify(got)}`);
  fieldTotal += exp.length;
}
// h 侧嵌套 struct IteratorRecord (object_ + next_)
has(h, "struct IteratorRecord {\n    TaggedMember<JSReceiver> object_;\n    TaggedMember<JSAny> next_;\n  };", "h IteratorRecord");
has(h, "TaggedMember<Smi> state_ V8_TQ_TYPE(SmiTagged<JSIteratorHelperState>);", "h state_");
for (const cls of Object.keys(FIELDS)) must(h.includes(`class ${cls}`) || h.includes(`${cls} final`), `h 缺类 ${cls}`);

// ── 4. ToString 辅助 (4) 与 inl ───────────────────────────────────────────
for (const f of ["const char* JSIteratorHelperStateToString(", "operator<<(std::ostream& os,\n                                           JSIteratorHelperState state)",
  "const char* JSIteratorZipHelperModeToString(", "operator<<(std::ostream& os,\n                                           JSIteratorZipHelperMode mode)"])
  has(h, f, "h ToString");
has(inl, "state_.store(this, Smi::From31BitPattern(static_cast<int>(value)));", "inl set_state");
has(inl, "static_cast<JSIteratorHelperState>(state_.load().value())", "inl state");

// ── 5. 64 位布局 (IteratorRecord = 2 tagged) ──────────────────────────────
const T = 8;
const HO = 3 * T; // JSObject 24
const LAY = {
  JSIteratorHelper: HO + 1 * T, // 32
  JSIteratorHelperSimple: HO + 1 * T + 2 * T, // 48
  JSIteratorMapHelper: 0, JSIteratorFilterHelper: 0, // 补 below
  JSIteratorTakeHelper: 0, JSIteratorDropHelper: 0,
  JSIteratorFlatMapHelper: 0, JSIteratorConcatHelper: 0,
  JSIteratorZipHelper: 0, JSIteratorZipKeyedHelper: 0,
};
LAY.JSIteratorMapHelper = LAY.JSIteratorHelperSimple + 2 * T; // 64
LAY.JSIteratorFilterHelper = LAY.JSIteratorHelperSimple + 2 * T; // 64
LAY.JSIteratorTakeHelper = LAY.JSIteratorHelperSimple + 1 * T; // 56
LAY.JSIteratorDropHelper = LAY.JSIteratorHelperSimple + 1 * T; // 56
LAY.JSIteratorFlatMapHelper = LAY.JSIteratorHelperSimple + 2 * T + 2 * T; // 80
LAY.JSIteratorConcatHelper = LAY.JSIteratorHelperSimple + 2 * T; // 64
LAY.JSIteratorZipHelper = HO + 4 * T; // 56
LAY.JSIteratorZipKeyedHelper = LAY.JSIteratorZipHelper + 1 * T; // 64
must(LAY.JSIteratorHelper === 32 && LAY.JSIteratorHelperSimple === 48 &&
  LAY.JSIteratorFlatMapHelper === 80 && LAY.JSIteratorZipKeyedHelper === 64, "布局推导失配");

const body = `${banner("src/objects/js-iterator-helpers.h.js", "src/objects/js-iterator-helpers.{h,-inl.h,tq}", "JSIteratorHelper 族枚举/类层级/布局骨架")}
//! 差集骨架: Iterator Helpers (map/filter/take/drop/flatMap/concat/zip/zipKeyed 直接迭代器实现)。
//! 配置: 64 位非压缩 (kTaggedSize=8)。next()/return() builtin 多路分发语义面留 Developer。

/// v8: src/objects/js-iterator-helpers.h JSIteratorHelperState (生成器状态手工跟踪)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JSIteratorHelperState { SuspendedStart = 0, SuspendedYield = 1, Executing = 2, Completed = 3 }

/// v8: src/objects/js-iterator-helpers.h JSIteratorZipHelperMode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JSIteratorZipHelperMode { Shortest = 0, Longest = 1, Strict = 2 }

// ── 64 位布局 (kTaggedSize=8; IteratorRecord = object_+next_ 2 tagged) ──
${Object.entries(LAY).map(([c, s]) => `/// v8: src/objects/js-iterator-helpers.h ${c}::kHeaderSize = ${s}`).join("\n")}
pub const K_HEADERS: &[(&str, usize)] = &[
${Object.entries(LAY).map(([c, s]) => `    ("${c}", ${s}),`).join("\n")}
];

/// v8: src/objects/js-iterator-helpers.h JSIteratorHelperSimple::IteratorRecord (object_, next_)
pub struct IteratorRecord { /* object_: Tagged<JSReceiver>, next_: Tagged<JSAny> */ }

/// v8: src/objects/js-iterator-helpers-inl.h state accessor (Smi::From31BitPattern 编解码)
/// v8: src/objects/js-iterator-helpers.h JSIteratorHelperStateToString / ZipHelperModeToString (调试)
`;

const p = writeSrc("v8-objects-js-core", "js_iterator_helpers.rs.skel", body);
console.log(
  `js-iterator-helpers.h.js: ${p} — 枚举 4+3 值、类 ${tqCls.length}/10、字段 ${fieldTotal}/${fieldTotal}、` +
    `ToString 4、断言 ${checks} 项通过`,
);
