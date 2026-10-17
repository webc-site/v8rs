#!/usr/bin/env bun
// tool/src/objects/js-objects.cc.js — JSObject 族 tq 布局骨架 (15 extern class) + cc 方法对数
// 组→文件映射 (本脚本管辖): js-objects.tq + js-objects.cc (席位表差集; .h/-inl.h 归既有 js-objects.h.js)
// 产物: v8-objects-js-core/src/js_objects.rs.skel (不挂 lib.rs; JSReceiver/JSObject 基类细节见 js_receiver.rs/js_object.rs)
import { readV8, writeSrc, banner } from "../../lib.js";

const tq = readV8("src/objects/js-objects.tq");
const cc = readV8("src/objects/js-objects.cc");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`锚点失配: ${m}`); };
const eq = (a, e, m) => { checks++; if (a !== e) throw new Error(`锚点失配: ${m} (期望 ${e}, 实得 ${a})`); };

// ── 1. tq extern class 全集解析 ────────────────────────────────────────
const classRe = /extern (class|shape) (\w+) extends (\w+)/g;
const classes = [...tq.matchAll(classRe)].map((m) => ({ kind: m[1], name: m[2], parent: m[3] }));
eq(classes.length, 15, "tq extern class/shape 总数");
const parents = new Map(classes.map((c) => [c.name, c.parent]));
for (const [name, parent] of [
  ["JSReceiver", "HeapObject"], ["JSObject", "JSReceiver"], ["JSExternalObject", "JSObject"],
  ["JSObjectWithEmbedderSlots", "JSObject"], ["JSAPIObjectWithEmbedderSlots", "JSObject"],
  ["JSCustomElementsObject", "JSObject"], ["JSSpecialObject", "JSCustomElementsObject"],
  ["JSGlobalProxy", "JSSpecialObject"], ["JSGlobalObject", "JSSpecialObject"],
  ["JSPrimitiveWrapper", "JSCustomElementsObject"], ["JSMessageObject", "JSObject"],
  ["JSDate", "JSObject"], ["JSAsyncFromSyncIterator", "JSObject"],
  ["JSStringIterator", "JSObject"], ["JSValidIteratorWrapper", "JSObject"],
]) must(parents.get(name) === parent, `tq ${name} extends 应为 ${parent}, 实得 ${parents.get(name)}`);
eq(classes.filter((c) => c.kind === "shape").length, 0, "tq 无 extern shape (本表)");

// ── 2. 关键字段槽锚 ────────────────────────────────────────────────────
must(tq.includes("properties_or_hash: SwissNameDictionary|FixedArrayBase|PropertyArray|Smi;"), "JSReceiver.properties_or_hash 联合类型");
must(tq.includes("elements: FixedArrayBase;"), "JSObject.elements");
must(tq.includes("value: ExternalPointer;"), "JSExternalObject.value");
must(tq.includes("cpp_heap_wrappable: CppHeapPointer;"), "cpp_heap_wrappable (两处: API/Special)");
eq((tq.match(/cpp_heap_wrappable: CppHeapPointer;/g) || []).length, 2, "cpp_heap_wrappable 槽计数");
must(tq.includes("global_proxy: JSGlobalProxy;"), "JSGlobalObject.global_proxy");
must(tq.includes("global_proxy_for_api: JSGlobalProxy;"), "JSGlobalObject.global_proxy_for_api");
must(tq.includes("value: JSAny;\n}\n\n@cppObjectLayoutDefinition\nextern class JSMessageObject"), "JSPrimitiveWrapper.value 邻接锚");
must(tq.includes("@apiExposedInstanceTypeValue(0x421)"), "JSObject 实例类型锚 0x421");
must(tq.includes("@highestInstanceTypeWithinParentClassRange"), "JSReceiver/JSObject 范围注解");

// JSMessageObject 9 槽 (5 tagged + 4 raw-Smi)
const msgBody = tq.slice(tq.indexOf("extern class JSMessageObject"), tq.indexOf("extern class JSDate"));
const msgFields = [...msgBody.matchAll(/^  (\w+): ([^;\n]+)/gm)].map((m) => [m[1], m[2].trim()]);
eq(msgFields.length, 9, "JSMessageObject 字段总数");
must(msgFields[0][0] === "message_type" && msgFields[0][1] === "Smi", `message_type ${msgFields[0]}`);
must(msgFields[3][0] === "stack_trace" && msgFields[3][1] === "StackTraceInfo|TheHole", `stack_trace ${msgFields[3]}`);
must(msgFields[8][0] === "error_level", `error_level ${msgFields[8]}`);
// JSDate 9 槽 (value f64 + 8 缓存)
const dateBody = tq.slice(tq.indexOf("extern class JSDate"), tq.indexOf("extern class JSAsyncFromSyncIterator"));
const dateFields = [...dateBody.matchAll(/^  (\w+): ([^;\n]+)/gm)].map((m) => [m[1], m[2].trim()]);
eq(dateFields.length, 9, "JSDate 字段总数");
must(dateFields[0][0] === "value" && dateFields[0][1] === "float64", `JSDate.value ${dateFields[0]}`);
must(dateFields[8][0] === "cache_stamp", `cache_stamp ${dateFields[8]}`);
// 其余 2 槽类
const twoSlot = (cls, f0, f1) => {
  const start = tq.indexOf(`extern class ${cls}`);
  const b = tq.slice(start, tq.indexOf("\n}", start));
  const fs = [...b.matchAll(/^  (\w+): ([^;\n]+)/gm)];
  eq(fs.length, 2, `${cls} 字段数`);
  must(fs[0][1] === f0 && fs[1][1] === f1, `${cls} 字段序 ${fs.map((f) => f[1])}`);
};
twoSlot("JSAsyncFromSyncIterator", "sync_iterator", "next");
twoSlot("JSStringIterator", "string", "index");
must(tq.includes("underlying: iterator::IteratorRecord;"), "JSValidIteratorWrapper.underlying");

// ── 3. tq macro 对数 ────────────────────────────────────────────────────
const macros = [...tq.matchAll(/^macro (\w+)\(/gm)].map((m) => m[1]);
eq(macros.length, 4, "tq macro 数");
must(macros.join(",") === "NewJSObject,GetDerivedMap,GetDerivedRabGsabTypedArrayMap,AllocateFastOrSlowJSObjectFromMap",
  `tq macro 名序 ${macros}`);
eq((tq.match(/^extern macro AllocateJSObjectFromMap\(/gm) || []).length, 3, "extern macro AllocateJSObjectFromMap 重载数");
must(tq.includes("type Constructor extends JSReceiver;"), "Constructor 类型别名");

// ── 4. cc 方法定义对数 ──────────────────────────────────────────────────
const defs = [...cc.matchAll(/^[A-Za-z_][A-Za-z0-9_:<>, &*]* (JSReceiver|JSObject|JSExternalObject|JSGlobalObject|JSPrimitiveWrapper|JSMessageObject|JSDate|JSAsyncFromSyncIterator|JSStringIterator|JSValidIteratorWrapper)::[A-Za-z_]+\(/gm)];
eq(defs.length, 140, "cc 方法定义总数 (限定 JSObject 族)");
must(cc.includes("Tagged<Object> JSObject::SlowReverseLookup(Tagged<Object> value) {"), "cc SlowReverseLookup 锚");
must(cc.includes("JSObject::UpdateAllocationSite<AllocationSiteUpdateMode::kCheckOnly>("), "cc UpdateAllocationSite 模板锚");
must(cc.includes("Tagged<Object> JSObject::RawFastPropertyAtCompareAndSwap("), "cc RawFastPropertyAtCompareAndSwap (CAS) 锚");
must(cc.includes("static const char* NonAPIInstanceTypeToString(InstanceType instance_type) {"), "cc NonAPIInstanceTypeToString 锚");

// ── 5. 槽位偏移 (非压缩 8B tagged; JSReceiver 头 2 槽, JSObject 头 3 槽) ──
const HEAD_RECEIVER = 16, HEAD_OBJECT = 24;
const off = (head, i) => head + i * 8;

// ── 6. 骨架产物 ──────────────────────────────────────────────────────────
const body = `${banner("src/objects/js-objects.cc.js", "src/objects/js-objects.tq + js-objects.cc", "JSObject 族 tq 布局 (15 类) + cc 方法对数差集骨架")}
//! tq 差集: JSReceiver/JSObject 直系 15 类布局; .h/-inl.h 常量归
//! tool/src/objects/js-objects.h.js (JSReceiver::kHeaderSize=16, JSObject::kHeaderSize=24 非压缩)。

// 继承骨架 (tq @abstract: JSReceiver/JSSpecialObject/JSCustomElementsObject/JSAPIObjectWithEmbedderSlots):
//   HeapObject → JSReceiver → JSObject → {JSExternalObject, JSObjectWithEmbedderSlots,
//   JSAPIObjectWithEmbedderSlots(+cpp_heap_wrappable), JSCustomElementsObject → JSSpecialObject
//   (+cpp_heap_wrappable) → {JSGlobalProxy, JSGlobalObject}, JSPrimitiveWrapper, JSMessageObject,
//   JSDate, JSAsyncFromSyncIterator, JSStringIterator, JSValidIteratorWrapper}
/// v8: src/objects/js-objects.tq JSObject @apiExposedInstanceTypeValue(0x421)
pub const K_JS_OBJECT_API_INSTANCE_TYPE: u16 = 0x421;

// JSGlobalObject: 2 tagged 槽
/// v8: src/objects/js-objects.tq JSGlobalObject.global_proxy
pub const K_GLOBAL_PROXY_OFFSET: usize = ${off(HEAD_OBJECT, 0)};
/// v8: src/objects/js-objects.tq JSGlobalObject.global_proxy_for_api
pub const K_GLOBAL_PROXY_FOR_API_OFFSET: usize = ${off(HEAD_OBJECT, 1)};
// JSPrimitiveWrapper: value@${off(HEAD_OBJECT, 0)}
// JSMessageObject: 9 槽 (message_type..error_level) @${off(HEAD_OBJECT, 0)}..${off(HEAD_OBJECT, 8)}
// JSDate: value f64 @${off(HEAD_OBJECT, 0)} + year..cache_stamp 8 Smi|NaN 槽 @${off(HEAD_OBJECT, 1)}..${off(HEAD_OBJECT, 8)}
//   (NaN 缓存语义: date_cache_stamp 失配即失效, js-objects.cc:5936)
// JSAsyncFromSyncIterator: sync_iterator@${off(HEAD_OBJECT, 0)}, next@${off(HEAD_OBJECT, 1)}
// JSStringIterator: string@${off(HEAD_OBJECT, 0)}, index@${off(HEAD_OBJECT, 1)}
// JSValidIteratorWrapper: underlying (iterator::IteratorRecord)@${off(HEAD_OBJECT, 0)}

// cc 方法映射: 140 个定义 (SlowReverseLookup/UpdateAllocationSite/RawFastPropertyAtCompareAndSwap/
//   NonAPIInstanceTypeToString/ApplyAttributesToDictionary/... ) — 语义推迟到 objects 层实现
// tq macro: NewJSObject/GetDerivedMap/GetDerivedRabGsabTypedArrayMap/AllocateFastOrSlowJSObjectFromMap +
//   extern AllocateJSObjectFromMap x3 重载
`;

const p = writeSrc("v8-objects-js-core", "js_objects.rs.skel", body);
console.log(
  `js-objects.cc.js: ${p} — tq 类 ${classes.length}/15 (MessageObject 9 槽/JSDate 9 槽/2 槽类 2)、macro ${macros.length}/4 + extern 重载 3、cc 方法定义 ${defs.length}/140、断言 ${checks} 项全部通过`,
);
