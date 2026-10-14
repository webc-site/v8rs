#!/usr/bin/env bun
// tool/src/maglev/repr.js — ValueRepresentation / UseRepresentation + NodeType 全表
// 覆盖: src/maglev/maglev-ir.h (ValueRepresentation/UseRepresentation 区) +
//       src/maglev/maglev-node-type.h (LEAF/COMBINED_NODE_TYPE_LIST → NodeType)
// 产出: v8-maglev-ir/src/repr.rs.skel (规则代码: 两表示枚举 + NodeType 位表)
// 复跑幂等: 纯文本扫描 + 固定顺序 + 确定性输出。
import { banner, readV8, writeSrc } from "../../lib.js";

const GEN = "src/maglev/repr.js";
const IR = "src/maglev/maglev-ir.h";
const NT = "src/maglev/maglev-node-type.h";
const NTC = "src/maglev/maglev-node-type.cc";
const ir = readV8(IR);
const nt = readV8(NT);
readV8(NTC);
const die = (m) => { throw new Error(`[repr.js] ${m}`); };

// —— ValueRepresentation: 8 变体 ————
for (const needle of [
  "enum class ValueRepresentation : uint8_t {",
  "kHoleyFloat64,",
  "kIntPtr,",
  "kRawPtr,",
  "kNone,",
  "IsDoubleRepresentation(ValueRepresentation repr)",
  "typedef base::EnumSet<ValueRepresentation, int8_t> ValueRepresentationSet;",
]) {
  if (!ir.includes(needle)) die(`ir 缺针 ${needle}`);
}
const VREPR = ["kTagged", "kInt32", "kUint32", "kFloat64", "kHoleyFloat64", "kIntPtr", "kRawPtr", "kNone"];
for (const v of VREPR) if (!ir.includes(v)) die(`缺 ValueRepresentation::${v}`);

// —— UseRepresentation: 8 变体 ————
for (const needle of [
  "enum class UseRepresentation : uint8_t {",
  "kTaggedForNumberToString,",
  "kTruncatedInt32,",
  "kNonTruncated,",
  "kLast = kNonTruncated",
  "typedef base::EnumSet<UseRepresentation, int8_t> UseRepresentationSet;",
]) {
  if (!ir.includes(needle)) die(`ir 缺针 ${needle}`);
}
const UREPR = ["kTagged", "kTaggedForNumberToString", "kInt32", "kTruncatedInt32",
  "kUint32", "kFloat64", "kHoleyFloat64", "kNonTruncated"];
for (const v of UREPR) if (!ir.includes(v)) die(`缺 UseRepresentation::${v}`);

// —— NodeType: LEAF 18 + COMBINED 20 = 38 ————
for (const needle of [
  "#define LEAF_NODE_TYPE_LIST(V)",
  "#define COMBINED_NODE_TYPE_LIST(V)",
  "#define NODE_TYPE_LIST(V)",
  "enum class NodeType : uint32_t {",
  "enum class NodeTypeIsVariant {",
  "static constexpr int kNumberOfLeafNodeTypes = 0 LEAF_NODE_TYPE_LIST(COUNT);",
  "NODE_TYPE_LIST(DEFINE_NODE_TYPE_CHECK)",
  "StaticTypeForMap(compiler::MapRef map,",
  "StaticTypeForConstant(compiler::JSHeapBroker* broker,",
]) {
  if (!nt.includes(needle)) die(`node-type 缺针 ${needle}`);
}
const LEAF = ["Smi", "HeapNumber", "Null", "Undefined", "Boolean", "Symbol",
  "InternalizedString", "OtherString", "Context", "StringWrapper", "JSArray",
  "JSFunction", "OtherCallable", "JSDataView", "JSGeneratorObject",
  "OtherHeapObject", "OtherJSReceiver", "BigInt"];
for (const v of LEAF) if (!nt.includes(`V(${v}, (1 <<`)) die(`缺 LEAF ${v}`);
if (LEAF.length !== 18) die(`LEAF 数=${LEAF.length}, 期望 18`);
const COMB = ["Unknown", "None", "Callable", "NullOrUndefined", "Oddball", "Number",
  "Numeric", "NumberOrBoolean", "NumberOrUndefined", "NumberOrOddball", "String",
  "StringOrStringWrapper", "StringOrOddball", "Name", "JSPrimitive", "JSReceiver",
  "JSReceiverOrNull", "JSReceiverOrNullOrUndefined", "NotUndetectable", "AnyHeapObject"];
for (const v of COMB) if (!nt.includes(`V(${v},`)) die(`缺 COMBINED ${v}`);
if (COMB.length !== 20) die(`COMBINED 数=${COMB.length}, 期望 20`);

const out = [];
out.push(banner(GEN, `${IR}, ${NT}, ${NTC}`, "表示双枚举 + NodeType 位表 18+20"));
out.push(`//! 表示规则: Phi 解包看 UseRepresentation (TruncatedInt32 允许 Float64 解包, Int32 不行);`);
out.push(`//! NodeType 位运算语义见 NodeTypeIs/CanBe/UnionType/RemoveType, 修订阶段 1:1 转写。`);
out.push(``);
out.push(`// ── ValueRepresentation (u8, 8 变体; HoleyFloat64 目标配置二选一见 opcodes.js) ──`);
for (const v of VREPR) out.push(`//   VREPR ${v}`);
out.push(``);
out.push(`// ── UseRepresentation (u8, 8 变体; kLast=kNonTruncated; hint-only 修订保留语义) ──`);
for (const v of UREPR) out.push(`//   UREPR ${v}`);
out.push(``);
out.push(`// ── NodeType LEAF (18, 1<<0..1<<17; kNumberOfLeafNodeTypes=18) ──`);
LEAF.forEach((v, i) => out.push(`//   LEAF ${v} = 1<<${i}`));
out.push(``);
out.push(`// ── NodeType COMBINED (20; Unknown=全叶位, None=0) ──`);
for (const v of COMB) out.push(`//   COMB ${v}`);
out.push(``);
out.push(`// 对数: 表示针 12 + LEAF 18 + COMBINED 20 全过`);

writeSrc("v8-maglev-ir", "repr.rs.skel", out.join("\n") + "\n");
console.log(`OK repr.js: VREPR 8 + UREPR 8 + LEAF 18 + COMBINED 20 → v8-maglev-ir/src/repr.rs.skel`);
