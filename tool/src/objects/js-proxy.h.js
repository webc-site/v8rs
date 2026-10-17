#!/usr/bin/env bun
// tool/src/objects/js-proxy.h.js — JSProxy 族骨架 (槽位 + 位标志 + shape 类 + trap 方法对数)
// 组→文件映射 (本脚本管辖): js-proxy.h + js-proxy-inl.h + js-proxy.tq
// 产物: v8-objects-js-core/src/js_proxy.rs.skel (不挂 lib.rs, 差集锚点)
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/js-proxy.h");
const inl = readV8("src/objects/js-proxy-inl.h");
const tq = readV8("src/objects/js-proxy.tq");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`锚点失配: ${m}`); };
const eq = (a, e, m) => { checks++; if (a !== e) throw new Error(`锚点失配: ${m} (期望 ${e}, 实得 ${a})`); };
const count = (s, re) => (s.match(re) || []).length;

// ── 1. 类签名与字段槽位 ──────────────────────────────────────────────────
must(h.includes("V8_OBJECT class JSProxy : public JSReceiver {"), "JSProxy 继承 JSReceiver");
must(h.includes("TaggedMember<UnionOf<JSReceiver, Null>> target_;"), "h target_ 槽");
must(h.includes("TaggedMember<UnionOf<JSReceiver, Null>> handler_;"), "h handler_ 槽");
must(h.includes("int32_t flags_;"), "h flags_ int32 槽");
must(h.includes("#if TAGGED_SIZE_8_BYTES\n  uint32_t padding_;"), "h 8B padding");
// 槽位 (非压缩 8B): JSReceiver 头 2 槽 (map + properties_or_hash)
const K_TARGET = 16, K_HANDLER = 24, K_FLAGS_64 = 32, K_SIZE_64 = 40; // int32 + padding 对齐到 8
must(h.includes("static_assert(static_cast<int>(offsetof(JSObject, elements_)) ==\n              static_cast<int>(offsetof(JSProxy, target_)));"),
  "target_ 与 JSObject::elements_ 别名 static_assert");

// ── 2. 位标志与常量 ──────────────────────────────────────────────────────
must(h.includes("using IsRevocableBit = base::BitField<bool, 0, 1>;"), "IsRevocableBit");
must(h.includes("static constexpr int kIsRevocableBit = IsRevocableBit::encode(true);"), "kIsRevocableBit");
must(h.includes("static const int kMaxIterationLimit = 100 * 1024;"), "kMaxIterationLimit");
must(inl.includes("bool JSProxy::is_revocable() const { return IsRevocableBit::decode(flags()); }"), "inl is_revocable");
must(inl.includes("bool JSProxy::IsRevoked() const { return !IsJSReceiver(handler()); }"), "inl IsRevoked");

// ── 3. trap/静态方法全集对数 (ES6 9.5.x) ────────────────────────────────
const methods = [...h.matchAll(/V8_WARN_UNUSED_RESULT static (?:MaybeDirectHandle<\w+>|Maybe<\w+>|MaybeHandle<\w+>) (\w+)\(/g)].map((m) => m[1]);
eq(methods.length, 14, "V8_WARN_UNUSED_RESULT 静态方法数 (New + 13)");
const expectTraps = ["GetPrototype", "SetPrototype", "IsExtensible", "IsArray", "PreventExtensions",
  "GetOwnPropertyDescriptor", "DefineOwnProperty", "HasProperty", "CheckHasTrap", "CheckDeleteTrap",
  "GetProperty", "CheckGetSetTrapResult", "SetProperty", "DeletePropertyOrElement", "SetPrivateSymbol"];
for (const t of expectTraps) must(new RegExp(`\\b${t}\\s*\\(`).test(h), `trap 方法 ${t} 缺失`);
eq(methods.filter((m) => m !== "New").length, 13, "V8_WARN_UNUSED_RESULT 方法中 trap 计数");
must(h.includes("static Maybe<PropertyAttributes> GetPropertyAttributes("), "GetPropertyAttributes");
must(h.includes("static MaybeDirectHandle<JSProxy> New("), "New");
must(h.includes("static void Revoke(DirectHandle<JSProxy> proxy);"), "Revoke");
must(h.includes("enum AccessKind { kGet, kSet };"), "AccessKind 枚举");
eq(count(h, /DECL_PRINTER\(JSProxy\)/g), 1, "DECL_PRINTER 计数");
eq(count(h, /DECL_VERIFIER\(JSProxy\)/g), 1, "DECL_VERIFIER 计数");

// ── 4. JSProxyRevocableResult shape 类 ──────────────────────────────────
must(h.includes("class JSProxyRevocableResult : public JSObject {"), "RevocableResult 继承");
must(h.includes("V8_IT_REUSE_PARENT;"), "V8_IT_REUSE_PARENT (无自有存储)");
must(h.includes("static constexpr int kProxySlotIndex = 0;"), "kProxySlotIndex");
must(h.includes("static constexpr int kRevokeSlotIndex = 1;"), "kRevokeSlotIndex");
must(h.includes("static constexpr int kInObjectPropertyCount = 2;"), "kInObjectPropertyCount");
const K_REVOKE_HEAD = 24; // JSObject::kHeaderSize (非压缩)
must(h.includes("static constexpr int kProxyOffset =\n      JSObject::kHeaderSize + kProxySlotIndex * kTaggedSize;"), "kProxyOffset 公式");
must(tq.includes("extern shape JSProxyRevocableResult extends JSObject {"), "tq shape 声明");
const shapeFields = [...tq.slice(tq.indexOf("extern shape JSProxyRevocableResult")).matchAll(/^  (\w+): (\w+);$/gm)].map((m) => `${m[1]}:${m[2]}`);
must(shapeFields.join(",") === "proxy:JSAny,revoke:JSAny", `tq shape 字段 ${shapeFields}`);

// ── 5. tq 交叉核对 ──────────────────────────────────────────────────────
must(tq.includes("extern class JSProxy extends JSReceiver {"), "tq JSProxy");
must(tq.includes("target: JSReceiver|Null;"), "tq target");
must(tq.includes("handler: JSReceiver|Null;"), "tq handler");
must(tq.includes("flags: int32;"), "tq flags int32");
must(tq.includes("@if(TAGGED_SIZE_8_BYTES) padding: uint32;"), "tq padding 条件");

// ── 6. 骨架产物 ──────────────────────────────────────────────────────────
const body = `${banner("src/objects/js-proxy.h.js", "src/objects/js-proxy.h + js-proxy-inl.h + js-proxy.tq", "JSProxy 槽位/位标志/revocable shape 差集骨架")}
//! JSProxy: target/handler/flags; target_ 偏移与 JSObject::elements_ 别名 (static_assert)。

/// v8: src/objects/js-proxy.h JSProxy::target_
pub const K_TARGET_OFFSET: usize = ${K_TARGET};
/// v8: src/objects/js-proxy.h JSProxy::handler_
pub const K_HANDLER_OFFSET: usize = ${K_HANDLER};
/// v8: src/objects/js-proxy.h JSProxy::flags_ (int32; 8B 布局下后随 padding_)
pub const K_FLAGS_OFFSET: usize = ${K_FLAGS_64};
/// v8: src/objects/js-proxy.h JSProxy::kHeaderSize
pub const K_HEADER_SIZE_64: usize = ${K_SIZE_64};

/// v8: src/objects/js-proxy.h JSProxy::IsRevocableBit (bit 0)
pub const K_IS_REVOCABLE_SHIFT: u32 = 0;
pub const K_IS_REVOCABLE_MASK: u32 = 1;
/// v8: src/objects/js-proxy.h JSProxy::kMaxIterationLimit
pub const K_MAX_ITERATION_LIMIT: i32 = 100 * 1024;

// JSProxyRevocableResult (shape, 无自有存储; 字段住 JSObject in-object 槽):
/// v8: src/objects/js-proxy.h JSProxyRevocableResult::kProxyOffset
pub const K_PROXY_OFFSET: usize = ${K_REVOKE_HEAD}; // JSObject::kHeaderSize + 0*kTaggedSize
/// v8: src/objects/js-proxy.h JSProxyRevocableResult::kRevokeOffset
pub const K_REVOKE_OFFSET: usize = ${K_REVOKE_HEAD + 8};
/// v8: src/objects/js-proxy.h JSProxyRevocableResult::kSize
pub const K_REVOCABLE_RESULT_SIZE: usize = ${K_REVOKE_HEAD + 16};

// trap 方法映射 (15 个 ES6 9.5.x 操作 + New/Revoke/GetPropertyAttributes, 全推迟到 trap 执行层):
//   GetPrototype/SetPrototype/IsExtensible/IsArray/PreventExtensions/GetOwnPropertyDescriptor/
//   DefineOwnProperty/HasProperty/GetProperty/SetProperty/DeletePropertyOrElement/
//   CheckHasTrap/CheckDeleteTrap/CheckGetSetTrapResult/SetPrivateSymbol
// inl 语义: is_revocable = (flags >> 0) & 1; IsRevoked = !IsJSReceiver(handler)
`;

const p = writeSrc("v8-objects-js-core", "js_proxy.rs.skel", body);
console.log(
  `js-proxy.h.js: ${p} — 槽 4、V8_WARN_UNUSED_RESULT 方法 14/14 (New+13)、trap 全集 ${expectTraps.length}/15、shape 槽 2/2、断言 ${checks} 项全部通过`,
);
