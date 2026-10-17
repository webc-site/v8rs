#!/usr/bin/env bun
// tool/src/objects/js-shadow-realm.h.js — JSShadowRealm 骨架 (1 tagged 槽)
// 组→文件映射: js-shadow-realm.h + js-shadow-realm-inl.h + js-shadow-realm.tq
// 产物: v8-objects-js-core/src/js_shadow_realm.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/js-shadow-realm.h");
const inl = readV8("src/objects/js-shadow-realm-inl.h");
const tq = readV8("src/objects/js-shadow-realm.tq");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`锚点失配: ${m}`); };
const eq = (a, e, m) => { checks++; if (a !== e) throw new Error(`锚点失配: ${m} (期望 ${e}, 实得 ${a})`); };

// ── 1. 类签名 + 字段槽 ──────────────────────────────────────────────────
must(h.includes("V8_OBJECT class JSShadowRealm : public JSObject {"), "JSShadowRealm 继承 JSObject");
must(h.includes("TaggedMember<NativeContext> native_context_;"), "h native_context_ 槽");
must(h.includes("EXPORT_DECL_VERIFIER(JSShadowRealm)"), "EXPORT_DECL_VERIFIER");
eq((h.match(/DECL_PRINTER\(JSShadowRealm\)/g) || []).length, 1, "DECL_PRINTER 计数");
must(h.includes("class BodyDescriptor;"), "BodyDescriptor 前向声明");
// 槽位: JSObject 头 3 槽 (map/properties_or_hash/elements), 后 1 tagged 槽
const K_NATIVE_CONTEXT = 24, K_SIZE = 32;

// ── 2. inl 访问器 + tq 交叉 ─────────────────────────────────────────────
must(inl.includes("inline Tagged<NativeContext> JSShadowRealm::native_context() const {"), "inl getter");
must(inl.includes("inline void JSShadowRealm::set_native_context(Tagged<NativeContext> value,"), "inl setter");
must(tq.includes("extern class JSShadowRealm extends JSObject {"), "tq 继承");
must(tq.includes("native_context: NativeContext;"), "tq native_context 字段");
const tqFields = [...tq.slice(tq.indexOf("extern class JSShadowRealm")).matchAll(/^  (\w+): (\w+);$/gm)].map((m) => `${m[1]}:${m[2]}`);
eq(tqFields.length, 1, "tq 字段数");

// ── 3. 骨架产物 ──────────────────────────────────────────────────────────
const body = `${banner("src/objects/js-shadow-realm.h.js", "src/objects/js-shadow-realm.h + js-shadow-realm-inl.h + js-shadow-realm.tq", "JSShadowRealm 单槽布局差集骨架")}
//! JSShadowRealm: JSObject + native_context (ShadowRealm 提案; 仅 1 个 tagged 槽)。

/// v8: src/objects/js-shadow-realm.h JSShadowRealm::native_context_
pub const K_NATIVE_CONTEXT_OFFSET: usize = ${K_NATIVE_CONTEXT};
/// v8: src/objects/js-shadow-realm.h JSShadowRealm (隐式 kHeaderSize = sizeof)
pub const K_HEADER_SIZE: usize = ${K_SIZE};

// 访问器 (inl): native_context()/set_native_context() → Tagged<NativeContext> load/store + 写屏障
// 推迟: DECL_PRINTER / EXPORT_DECL_VERIFIER (L4 诊断), BodyDescriptor (heap 层)
`;

const p = writeSrc("v8-objects-js-core", "js_shadow_realm.rs.skel", body);
console.log(
  `js-shadow-realm.h.js: ${p} — 槽 1/1 (偏移 ${K_NATIVE_CONTEXT}, size ${K_SIZE})、tq 字段 ${tqFields.length}/1、断言 ${checks} 项全部通过`,
);
