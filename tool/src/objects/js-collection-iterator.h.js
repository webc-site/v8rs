#!/usr/bin/env bun
// tool/src/objects/js-collection-iterator.h.js — JSCollectionIterator/OrderedHashTableIterator 提取
// 组→文件映射 (本脚本管辖): v8/src/objects/js-collection-iterator.{h,-inl.h,tq}
//   → v8-objects-js-core/src/js_collection_iterator.rs.skel
// 配置登记: 64 位非压缩 (kTaggedSize=8, JSObject 头 = 3 tagged = 24)。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/js-collection-iterator.h");
const inl = readV8("src/objects/js-collection-iterator-inl.h");
const tq = readV8("src/objects/js-collection-iterator.tq");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};
const has = (s, sub, f) => {
  checks++;
  if (!s.includes(sub)) throw new Error(`锚点失配: ${f} 无 "${sub.slice(0, 64)}"`);
};

// ── 1. JSCollectionIterator 字段 (tq: table + index) ──────────────────────
has(h, "V8_OBJECT class JSCollectionIterator : public JSObject {", "h 类头");
const seg = tq.slice(tq.indexOf("extern class JSCollectionIterator"));
const tqF = [...seg.slice(0, seg.indexOf("\n}")).matchAll(/^  (\w+):\s*([^;]+);/gm)]
  .map((m) => [m[1], m[2].replace(/\s+/g, " ").trim()]);
must(JSON.stringify(tqF) === JSON.stringify([["table", "Object"], ["index", "Object"]]),
  `tq 字段失配: ${JSON.stringify(tqF)}`);
has(h, "TaggedMember<Object> table_;", "h table_");
has(h, "TaggedMember<Object> index_;", "h index_");

// ── 2. OrderedHashTableIterator 模板方法 (4) ─────────────────────────────
for (const m of ["bool HasMore();", "void MoveNext() { set_index(Smi::FromInt(Smi::ToInt(index()) + 1)); }",
  "inline Tagged<Object> CurrentKey();", "void Transition();"])
  has(h, m, "OrderedHashTableIterator");
has(h, "template <class Derived, class TableType>", "模板头");

// ── 3. inl accessor 对数 (4 个自由函数) ───────────────────────────────────
has(inl, "Tagged<Object> JSCollectionIterator::table() const", "inl table");
has(inl, "void JSCollectionIterator::set_table(", "inl set_table");
has(inl, "Tagged<Object> JSCollectionIterator::index() const", "inl index");
has(inl, "void JSCollectionIterator::set_index(", "inl set_index");

// ── 4. 64 位布局 ──────────────────────────────────────────────────────────
const T = 8;
const K_HEADER = 3 * T + 2 * T; // 24 + table_ + index_
must(K_HEADER === 40, `布局推导失配 ${K_HEADER}`);

const body = `${banner("src/objects/js-collection-iterator.h.js", "src/objects/js-collection-iterator.{h,-inl.h,tq}", "JSCollectionIterator 布局与 OrderedHashTableIterator 模板骨架")}
//! 差集骨架: JSCollectionIterator (collection 迭代器公共基类)。
//! 配置: 64 位非压缩 (kTaggedSize=8)。

/// v8: src/objects/js-collection-iterator.h JSCollectionIterator: table_ offset=24, index_ offset=32
pub const K_JS_COLLECTION_ITERATOR_HEADER_SIZE: usize = ${K_HEADER};

/// v8: src/objects/js-collection-iterator-inl.h table/index accessor 族
pub struct JSCollectionIterator { /* table_: Tagged<Object>, index_: Tagged<Object> */ }

/// v8: src/objects/js-collection-iterator.h OrderedHashTableIterator<Derived, TableType>
///   trait OrderedHashTableIterator { fn has_more(); fn move_next(); fn current_key(); }
///   MoveNext 规则: index = Smi(Smi::ToInt(index) + 1)
///   Transition 规则: 迭代器跳到非 obsolete 后备表 (语义面留 Developer)
pub trait OrderedHashTableIterator { /* HasMore / MoveNext / CurrentKey / Transition */ }
`;

const p = writeSrc("v8-objects-js-core", "js_collection_iterator.rs.skel", body);
console.log(`js-collection-iterator.h.js: ${p} — 字段 ${tqF.length}/2、模板方法 4、inl accessor 4、布局 ${K_HEADER}、断言 ${checks} 项通过`);
