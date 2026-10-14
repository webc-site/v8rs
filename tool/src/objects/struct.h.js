#!/usr/bin/env bun
// tool/src/objects/struct.h.js — Struct 族签名骨架生成
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/struct.h → v8-objects-arrays/src/struct_.rs (struct 为 Rust 关键字, 模块名 struct_)
//   v8/src/objects/struct.tq → 继承/字段交叉核对 (同头族, 差集并入本脚本)
//   -inl 语义见 tool/src/objects/struct-inl.h.js (合并指引, 就地修订)
// 产物: v8-objects-arrays/src/struct_.rs.skel (差集骨架锚点; 落地版 struct_.rs 就地修订)
// 布局登记: 非压缩 8 字节 tagged (objects-core TaggedSlot=usize) → HeapObject 头 8,
//   Tagged 槽 8 字节; Struct 各子类字段按槽位 8/16/24/32 排布 (Struct 直接继承
//   HeapObject, 不经 FixedArrayBase, 不涉 AA-1 数组符号)。
// AA-1 状态更新: fixed_array::FixedArray 已落地 → ForInEnumeratorHolder.named_keys_
//   以 FixedArray 标记类型表达 (替换此前 "HeapObject 暂代" 登记)。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/struct.h");
const tq = readV8("src/objects/struct.tq");

let checks = 0;
const must = (cond, msg) => {
  checks++;
  if (!cond) throw new Error(`锚点失配: ${msg}`);
};
const has = (s, sub, where) => {
  if (!s.includes(sub)) throw new Error(`锚点失配: ${where} 无 "${sub.slice(0, 64)}"`);
  checks++;
};
const eq = (actual, expected, msg) => {
  checks++;
  if (actual !== expected) throw new Error(`锚点失配: ${msg} (期望 ${expected}, 实得 ${actual})`);
};
const countOf = (s, re) => (s.match(re) || []).length;

// ── 1. 类全集 (5) + 继承关系 + 宏对数 ────────────────────────────────────
const classDefs = [...h.matchAll(/class (\w+) : public (\w+)/g)].map((m) => [m[1], m[2]]);
eq(classDefs.length, 5, "类定义 (X : public Y) 总数");
const byName = new Map(classDefs);
for (const [name, parent] of [
  ["Struct", "HeapObject"], ["Tuple2", "Struct"], ["AccessorPair", "Struct"],
  ["ClassPositions", "Struct"], ["ForInEnumeratorHolder", "Struct"],
]) {
  must(byName.get(name) === parent, `class ${name} 父类应为 ${parent}, 实得 ${byName.get(name)}`);
}
// 前向声明 StructBodyDescriptor 1 项 (定义在 heap 层, 本族仅 using BodyDescriptor 引用)
eq(countOf(h, /class StructBodyDescriptor;/g), 1, "StructBodyDescriptor 前向声明");
eq(countOf(h, /V8_OBJECT_END;/g), 5, "V8_OBJECT_END 收尾计数");
eq(countOf(h, /V8_IT_ABSTRACT/g), 1, "V8_IT_ABSTRACT (Struct) 计数");
eq(countOf(h, /V8_IT_NO_AUTO_DISPATCH/g), 3, "V8_IT_NO_AUTO_DISPATCH 计数 (3 子类)");

// ── 2. static_assert: sizeof(Struct)==sizeof(HeapObject) ─────────────────
has(h, "static_assert(sizeof(Struct) == sizeof(HeapObject));", "struct.h:32");

// ── 3. struct.tq 交叉核对: extern class 全集 + ForIn 字段表 ──────────────
const tqClasses = [...tq.matchAll(/class (\w+) extends (\w+)/g)].map((m) => [m[1], m[2]]);
eq(tqClasses.length, 5, "tq extern class 总数");
for (const [name, parent] of tqClasses)
  must(byName.get(name) === parent, `tq/h 继承不一致: ${name} extends ${parent}`);
const fhTq = [...tq.matchAll(/extern class ForInEnumeratorHolder extends Struct \{([\s\S]*?)\}/g)][0][1];
const tqFields = [...fhTq.matchAll(/(\w+): (\w+);/g)].map((m) => `${m[1]}:${m[2]}`);
eq(tqFields.join(","), "enum_cache_map:Map,named_keys:FixedArray,elements_length:Smi,cache_length:Smi",
  "tq ForInEnumeratorHolder 字段表");

// ── 4. 字段声明全集对数 + 槽位偏移机械推导 (8/16/24/32) ─────────────────
const HEAD = 8; // HeapObject 头 = map TaggedSlot (非压缩)
const SLOT = 8; // TaggedMember 槽宽 = sizeof(TaggedSlot)
const slot = (cls, re) => {
  const start = h.indexOf(`class ${cls}`);
  const end = h.indexOf("V8_OBJECT_END;", start);
  return [...h.slice(start, end).matchAll(re)];
};
const fieldsWithOffset = (fields) => fields.map((f, i) => ({ ...f, offset: HEAD + i * SLOT }));
const t2 = fieldsWithOffset(slot("Tuple2", /TaggedMember<(\w+)> (\w+);/g).map((m) => ({ type: m[1], name: m[2] })));
eq(t2.length, 2, "Tuple2 字段数");
must(t2.map((f) => f.name).join(",") === "value1_,value2_", `Tuple2 字段名序 ${t2.map((f) => f.name)}`);
const ap = fieldsWithOffset(slot("AccessorPair", /TaggedMember<(\w+)> (\w+);/g).map((m) => ({ type: m[1], name: m[2] })));
eq(ap.length, 2, "AccessorPair 字段数");
must(ap.map((f) => f.name).join(",") === "getter_,setter_", `AccessorPair 字段名序 ${ap.map((f) => f.name)}`);
const cp = fieldsWithOffset(slot("ClassPositions", /const TaggedMember<Smi> (\w+);/g).map((m) => ({ type: "Smi", name: m[1] })));
eq(cp.length, 2, "ClassPositions 字段数 (const Smi)");
must(cp.map((f) => f.name).join(",") === "start_,end_", `ClassPositions 字段名序 ${cp.map((f) => f.name)}`);
const fh = fieldsWithOffset(slot("ForInEnumeratorHolder", /const TaggedMember<(\w+)> (\w+);/g).map((m) => ({ type: m[1], name: m[2] })));
eq(fh.length, 4, "ForInEnumeratorHolder 字段数");
must(fh.map((f) => f.name).join(",") === "enum_cache_map_,named_keys_,elements_length_,cache_length_", "ForIn 字段序失配");
must(fh.map((f) => f.type).join(",") === "Map,FixedArray,Smi,Smi", `ForIn 字段类型序 ${fh.map((f) => f.type)}`);
// 全族字段总数对数: 2+2+2+4 = 10
eq(t2.length + ap.length + cp.length + fh.length, 10, "Struct 族字段声明总数");

// ── 5. ObjectTraits offsetof 常量锚点 (struct.h:179-189) ─────────────────
const traits = [...h.slice(h.indexOf("struct ObjectTraits<ForInEnumeratorHolder>"))
  .matchAll(/static constexpr int (k\w+Offset) =\s*offsetof\(ForInEnumeratorHolder, (\w+)\);/g)]
  .slice(0, 4);
eq(traits.length, 4, "ObjectTraits offsetof 常量数");
for (let i = 0; i < 4; i++) {
  const [name, field] = [traits[i][1], traits[i][2]];
  // offsetof 目标须与私有字段序第 i 项一致, 偏移按 HEAD+i*SLOT 机械推导
  must(field === fh[i].name, `ObjectTraits ${name} 目标字段应为 ${fh[i].name}, 实得 ${field}`);
  must(HEAD + i * SLOT === fh[i].offset, `槽 ${i} (${field}) 偏移推导 ${HEAD + i * SLOT} != ${fh[i].offset}`);
}
const offsets = fh.map((f) => f.offset); // [8,16,24,32]

// ── 6. DECL_VERIFIER / DECL_PRINTER 对数 (4+4, printer 全集含无 verifier 类) ──
eq(countOf(h, /DECL_VERIFIER\((\w+)\)/g), 4, "DECL_VERIFIER 总数");
eq(countOf(h, /DECL_PRINTER\((\w+)\)/g), 4, "DECL_PRINTER 总数");
for (const cls of ["Tuple2", "AccessorPair", "ClassPositions", "ForInEnumeratorHolder"])
  must(h.includes(`DECL_VERIFIER(${cls})`) && h.includes(`DECL_PRINTER(${cls})`), `${cls} verifier/printer 缺失`);

// ── 7. 方法全集对数 (各类访问器族) ───────────────────────────────────────
const methodsOf = (cls) => {
  const start = h.indexOf(`class ${cls}`);
  const end = h.indexOf("V8_OBJECT_END;", start);
  const body = h.slice(start, end);
  return [...new Set([...body.matchAll(/\b(?:inline |static |template)?[\w:<>*&, ]*?\b(\w+)\s*\(/g)].map((m) => m[1]))];
};
const expectMethods = {
  Struct: ["Struct", "BriefPrintDetails"],
  Tuple2: ["Tuple2", "BriefPrintDetails", "value1", "set_value1", "value2", "set_value2"],
  AccessorPair: ["AccessorPair", "Copy", "get", "set", "getter", "set_getter", "setter", "set_setter",
    "GetComponent", "SetComponents", "Equals"],
  ClassPositions: ["ClassPositions", "start", "end", "BriefPrintDetails"],
  ForInEnumeratorHolder: ["ForInEnumeratorHolder", "enum_cache_map", "named_keys", "elements_length", "cache_length"],
};
let methodTotal = 0;
for (const [cls, ms] of Object.entries(expectMethods)) {
  const got = methodsOf(cls);
  for (const m of ms) must(got.includes(m), `${cls} 方法 ${m} 缺失`);
  methodTotal += ms.length;
}
eq(methodTotal, 28, "方法全集总数");

// ── 8. 骨架产物 (落地版 struct_.rs 的差集锚点; 语义以 Developer 修订为准) ──
const fieldLines = (cls, fields) =>
  fields.map((f) => `  pub ${f.name.replace(/_$/, "")}: Tagged<${f.type}>, // 偏移 ${f.offset}`).join("\n");
const body = `${banner("src/objects/struct.h.js", "src/objects/struct.h + struct.tq", "Struct 族骨架; 语义以 Developer 修订后的 struct_.rs 为准")}
//! 差集骨架: Struct 标记类 + Tuple2/AccessorPair/ClassPositions/ForInEnumeratorHolder
//!
//! 继承建模: Struct repr(transparent) 包 HeapObject (无新增字段, 尺寸编译期对数);
//! 子类 repr(C) 包 Struct, 字段自偏移 ${HEAD} 起按 Tagged 槽 (${SLOT} 字节) 顺序排布;
//! 堆语义经 Tagged<X> 裸槽原子访问 (Relaxed/Acquire/Release 序 + 写屏障桩)。

/// Struct: 简单结构类的抽象标记基类 (struct.h:22-31)
#[repr(transparent)]
pub struct Struct { pub header: HeapObject }
const _: () = assert!(size_of::<Struct>() == size_of::<HeapObject>());

/// Tuple2 (struct.h:34-70): 2 Object 槽, 偏移 ${t2.map((f) => f.offset).join("/")}
#[repr(C)]
pub struct Tuple2 { pub struct_: Struct, ${fieldLines("Tuple2", t2).trim().replace(/\n/g, "\n  ")} }

/// AccessorPair (struct.h:78-127): getter_/setter_ 偏移 ${ap.map((f) => f.offset).join("/")};
/// Copy/GetComponent 依赖 Isolate/NativeContext → 推迟 (票单); AccessorComponent 复用 core
#[repr(C)]
pub struct AccessorPair { pub struct_: Struct, ${fieldLines("AccessorPair", ap).trim().replace(/\n/g, "\n  ")} }

/// ClassPositions (struct.h:129-149): const Smi 槽, 偏移 ${cp.map((f) => f.offset).join("/")}
#[repr(C)]
pub struct ClassPositions { pub struct_: Struct, ${fieldLines("ClassPositions", cp).trim().replace(/\n/g, "\n  ")} }

/// ForInEnumeratorHolder (struct.h:151-177): 4 槽, 偏移 ${offsets.join("/")}
#[repr(C)]
pub struct ForInEnumeratorHolder { pub struct_: Struct, ${fieldLines("ForIn", fh).trim().replace(/\n/g, "\n  ")} }

// ObjectTraits<ForInEnumeratorHolder> offsetof 常量 (struct.h:179-189)
pub const K_ENUM_CACHE_MAP_OFFSET: usize = ${offsets[0]};
pub const K_NAMED_KEYS_OFFSET: usize = ${offsets[1]};
pub const K_ELEMENTS_LENGTH_OFFSET: usize = ${offsets[2]};
pub const K_CACHE_LENGTH_OFFSET: usize = ${offsets[3]};

// 方法映射 (struct.h 声明 → Tagged<X> inherent impl, 语义细节见 struct-inl.h.js 指引):
//   Tuple2: value1/value2 + set 族 (load/store/relaxed/acquire/release)
//   AccessorPair: get/set(component)/getter/setter(+acquire/release)/set_components/equals
//   ClassPositions: start()/end() (Smi 解包)
//   ForInEnumeratorHolder: 4 只读访问器
//   推迟: BriefPrintDetails/DECL_VERIFIER/DECL_PRINTER (L4 诊断), StructBodyDescriptor (heap 层)
`;

const p = writeSrc("v8-objects-arrays", "struct_.rs.skel", body);
console.log(
  `struct.h.js: 骨架 ${p} — 类 5/5 (h/tq 交叉)、字段 ${t2.length + ap.length + cp.length + fh.length}/10、` +
    `槽偏移 ${offsets.join("/")}、ObjectTraits 4/4、verifier+printer 8/8、方法全集 ${methodTotal}/28、断言 ${checks} 项全部通过`,
);
