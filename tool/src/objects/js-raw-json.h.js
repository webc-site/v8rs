#!/usr/bin/env bun
// tool/src/objects/js-raw-json.h.js — JSRawJson 骨架 (零自有字段 shape 类)
// 组→文件映射: js-raw-json.h + js-raw-json-inl.h + js-raw-json.cc + js-raw-json.tq
// 产物: v8-objects-js-core/src/js_raw_json.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/js-raw-json.h");
const inl = readV8("src/objects/js-raw-json-inl.h");
const cc = readV8("src/objects/js-raw-json.cc");
const tq = readV8("src/objects/js-raw-json.tq");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`锚点失配: ${m}`); };
const eq = (a, e, m) => { checks++; if (a !== e) throw new Error(`锚点失配: ${m} (期望 ${e}, 实得 ${a})`); };

// ── 1. 类签名: 零自有字段, 尺寸等于 JSObject ────────────────────────────
must(h.includes("V8_OBJECT class JSRawJson : public JSObject {"), "JSRawJson 继承 JSObject");
const rawFields = [...h.slice(h.indexOf("V8_OBJECT class JSRawJson"), h.indexOf("} V8_OBJECT_END;"))
  .matchAll(/TaggedMember<[^>]+> (\w+);/g)].map((m) => m[1]);
eq(rawFields.length, 0, "JSRawJson 自有 tagged 字段数 (应为 0)");
must(tq.includes("extern class JSRawJson extends JSObject {}"), "tq 空类体");
must(h.includes("static MaybeDirectHandle<JSRawJson> Create("), "h Create 声明");

// ── 2. inl + cc 关键锚点 ────────────────────────────────────────────────
must(inl.includes("bool JSRawJson::HasInitialLayout(Isolate* isolate) const {"), "inl HasInitialLayout");
must(inl.includes("return map() == *isolate->js_raw_json_map();"), "inl 初始 map 判定");
must(cc.includes("MaybeDirectHandle<JSRawJson> JSRawJson::Create(Isolate* isolate,"), "cc Create 定义");
must(cc.includes("result->InObjectPropertyPutAtOffset(sizeof(JSRawJson), *flat);"), "cc rawJSON 存 sizeof(JSRawJson) 起始 in-object 槽");
must(cc.includes("JSReceiver::SetIntegrityLevel(isolate, result, FROZEN, kThrowOnError).Check();"), "cc FROZEN 冻结");
must(cc.includes("JsonParser<uint8_t>::CheckRawJson") && cc.includes("JsonParser<uint16_t>::CheckRawJson"), "cc 双宽度 CheckRawJson");

// ── 3. 骨架产物 ──────────────────────────────────────────────────────────
const body = `${banner("src/objects/js-raw-json.h.js", "src/objects/js-raw-json.h + -inl.h + .cc + .tq", "JSRawJson 零字段 shape 类差集骨架")}
//! JSRawJson: 无自有 C++ 存储; "rawJSON" 住 JSObject in-object 槽 0 (偏移 = sizeof(JSRawJson))。

/// v8: src/objects/js-raw-json.h JSRawJson (kHeaderSize = sizeof = JSObject::kHeaderSize)
pub const K_HEADER_SIZE: usize = 24; // 非压缩 8B: JSObject 头 3 槽
/// v8: src/objects/js-raw-json.cc JSRawJson::Create — rawJSON in-object 槽偏移
pub const K_RAW_JSON_VALUE_OFFSET: usize = K_HEADER_SIZE;

// Create 流程 (cc): ToString → Flatten → CheckRawJson(1/2 字节宽) → NewJSObjectFromMap(js_raw_json_map)
//   → InObjectPropertyPutAtOffset(sizeof(JSRawJson), flat) → SetIntegrityLevel(FROZEN)
// HasInitialLayout (inl): map() == isolate->js_raw_json_map()
`;

const p = writeSrc("v8-objects-js-core", "js_raw_json.rs.skel", body);
console.log(
  `js-raw-json.h.js: ${p} — 自有字段 ${rawFields.length}/0、Create/HasInitialLayout 锚 6/6、断言 ${checks} 项全部通过`,
);
