#!/usr/bin/env bun
// tool/src/objects/js-collection.h.js — JSCollection/JSWeakCollection 族布局提取
// 组→文件映射 (本脚本管辖): v8/src/objects/js-collection.{h,-inl.h,tq}
//   → v8-objects-js-core/src/js_collection.rs.skel
// 配置登记: 64 位非压缩 (kTaggedSize=8, JSObject 头 = 3 tagged = 24)。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/js-collection.h");
const inl = readV8("src/objects/js-collection-inl.h");
const tq = readV8("src/objects/js-collection.tq");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};
const has = (s, sub, f) => {
  checks++;
  if (!s.includes(sub)) throw new Error(`锚点失配: ${f} 无 "${sub.slice(0, 64)}"`);
};

// ── 1. 类全集 (h: 13 类) ──────────────────────────────────────────────────
for (const c of ["JSCollection : public JSObject", "class JSSet final : public JSCollection",
  "class JSMap final : public JSCollection", "JSWeakCollection : public JSObject",
  "class JSWeakMap final : public JSWeakCollection", "class JSWeakSet final : public JSWeakCollection",
  "class JSMapIterator", "class JSSetIterator",
  "class JSMapKeyIterator : public JSMapIterator", "class JSMapKeyValueIterator : public JSMapIterator",
  "class JSMapValueIterator : public JSMapIterator",
  "class JSSetKeyValueIterator : public JSSetIterator", "class JSSetValueIterator : public JSSetIterator"])
  has(h, c, "js-collection.h");

// tq 侧类全集真解析
const tqClasses = [...tq.matchAll(/extern class (\w+)(?:\s+extends\s+(\w+))?/g)]
  .map((m) => [m[1], m[2] ?? "-"]);
const EXPECT = [
  ["JSCollection", "JSObject"], ["JSSet", "JSCollection"], ["JSMap", "JSCollection"],
  ["JSWeakCollection", "JSObject"], ["JSWeakSet", "JSWeakCollection"], ["JSWeakMap", "JSWeakCollection"],
  ["JSMapIterator", "JSCollectionIterator"], ["JSMapKeyIterator", "JSMapIterator"],
  ["JSMapKeyValueIterator", "JSMapIterator"], ["JSMapValueIterator", "JSMapIterator"],
  ["JSSetIterator", "JSCollectionIterator"], ["JSSetKeyValueIterator", "JSSetIterator"],
  ["JSSetValueIterator", "JSSetIterator"],
];
must(tqClasses.length === 13, `tq 类应 13, 实得 ${tqClasses.length}:\n${JSON.stringify(tqClasses)}`);
for (let i = 0; i < 13; i++)
  must(tqClasses[i][0] === EXPECT[i][0] && tqClasses[i][1] === EXPECT[i][1],
    `tq 类[${i}] ${tqClasses[i]} 期望 ${EXPECT[i]}`);

// ── 2. 字段与常量 ─────────────────────────────────────────────────────────
has(h, "TaggedMember<Object> table_;", "h table_ (JSCollection+JSWeakCollection 共用声明形态)");
must((h.match(/TaggedMember<Object> table_;/g) || []).length === 2, "table_ 声明应 2 处");
has(h, "static const int kAddFunctionDescriptorIndex = 3;", "h kAddFunctionDescriptorIndex");
must((h.match(/kAddFunctionDescriptorIndex = 3;/g) || []).length === 2, "kAddFunctionDescriptorIndex 应 2 处");
has(tq, "table: Object;", "tq table 字段");

// transient 类型 (4)
const trans = [...tq.matchAll(/^transient type (\w+)(?:\s+extends\s+(\w+))?/gm)].map((m) => m[1]);
for (const t of ["JSSetWithNoCustomIteration", "JSMapWithNoCustomIteration",
  "StableOrderedHashSet", "StableOrderedHashMap"])
  must(trans.includes(t), `tq transient 缺 ${t}`);
must(trans.length === 4, `tq transient 应 4, 实得 ${trans.length}`);

// ── 3. 行为方法锚点 ───────────────────────────────────────────────────────
for (const m of ["static void Initialize(DirectHandle<JSSet> set", "static void Clear(Isolate* isolate, DirectHandle<JSSet> set)",
  "void Rehash(Isolate* isolate);", "static void Initialize(DirectHandle<JSMap> map",
  "static void Clear(Isolate* isolate, DirectHandle<JSMap> map)",
  "static void Set(DirectHandle<JSWeakCollection>", "static bool Delete(DirectHandle<JSWeakCollection>",
  "static DirectHandle<JSArray> GetEntries(DirectHandle<JSWeakCollection>",
  "inline Tagged<Object> CurrentValue();"])
  has(h, m, "js-collection.h");
has(inl, "Tagged<Object> JSCollection::table() const", "inl table");
has(inl, "Tagged<Object> JSMapIterator::CurrentValue() {", "inl CurrentValue");
has(inl, "int index = Smi::ToInt(this->index());", "inl index 解码");

// ── 4. 64 位布局 ──────────────────────────────────────────────────────────
const T = 8;
const K_JS_OBJECT_HEADER = 3 * T;
const K_COLLECTION_HEADER = K_JS_OBJECT_HEADER + T; // + table_
const K_ITER_HEADER = K_JS_OBJECT_HEADER + 2 * T; // + table_ + index_ (js-collection-iterator 席)
must(K_COLLECTION_HEADER === 32, "collection 布局失配");

const body = `${banner("src/objects/js-collection.h.js", "src/objects/js-collection.{h,-inl.h,tq}", "JSCollection/JSWeakCollection 族布局与迭代器类全集")}
//! 差集骨架: JSCollection (Set/Map) + JSWeakCollection (WeakSet/WeakMap) + 5 个具体迭代器视图。
//! 配置: 64 位非压缩 (kTaggedSize=8)。哈希表语义 (Rehash/Clear/Set/Delete) 留 Developer。

/// v8: src/objects/js-collection.h JSCollection: table_ offset=${K_JS_OBJECT_HEADER}
pub const K_JS_COLLECTION_HEADER_SIZE: usize = ${K_COLLECTION_HEADER};
/// v8: src/objects/js-collection.h JSWeakCollection: table_ offset=${K_JS_OBJECT_HEADER}
pub const K_JS_WEAK_COLLECTION_HEADER_SIZE: usize = ${K_COLLECTION_HEADER};
/// v8: src/objects/js-collection.h kAddFunctionDescriptorIndex (JSCollection 与 JSWeakCollection 各一处)
pub const K_ADD_FUNCTION_DESCRIPTOR_INDEX: i32 = 3;

/// v8: src/objects/js-collection-inl.h JSCollection::table / set_table
/// v8: src/objects/js-collection-inl.h JSMapIterator::CurrentValue — table->ValueAt(InternalIndex(Smi::ToInt(index())))
pub struct JSCollection { /* table_: Tagged<Object> */ }

/// v8: src/objects/js-collection.h JSSet::Initialize/Clear/Rehash; JSMap::Initialize/Clear/Rehash
/// v8: src/objects/js-collection.h JSWeakCollection::Initialize/Set/Delete/GetEntries
/// v8: src/objects/js-collection.tq transient 视图: ${trans.join(" / ")}
/// v8: src/objects/js-collection.h 迭代器 5 视图 (Map{Key,KeyValue,Value}Iterator / Set{KeyValue,Value}Iterator)
///   布局归 js-collection-iterator 席 (table_+index_ 两字段)
`;

const p = writeSrc("v8-objects-js-core", "js_collection.rs.skel", body);
console.log(
  `js-collection.h.js: ${p} — 类 ${tqClasses.length}/13 (tq+h 对齐)、transient ${trans.length}/4、` +
    `布局 ${K_COLLECTION_HEADER}、断言 ${checks} 项通过`,
);
