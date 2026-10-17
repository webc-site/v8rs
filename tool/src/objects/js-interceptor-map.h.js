#!/usr/bin/env bun
// tool/src/objects/js-interceptor-map.h.js — JSInterceptorMap 布局与位标志提取
// 组→文件映射 (本脚本管辖): v8/src/objects/js-interceptor-map.{h,-inl.h,tq}
//   → v8-objects-js-core/src/js_interceptor_map.rs.skel
// 配置登记: 64 位非压缩 (kTaggedSize=8; ExtendedMap 头 + flags(1) + padding(7) + 3 tagged)。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/js-interceptor-map.h");
const inl = readV8("src/objects/js-interceptor-map-inl.h");
const tq = readV8("src/objects/js-interceptor-map.tq");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};
const has = (s, sub, f) => {
  checks++;
  if (!s.includes(sub)) throw new Error(`锚点失配: ${f} 无 "${sub.slice(0, 64)}"`);
};

// ── 1. 类与位标志 ─────────────────────────────────────────────────────────
has(h, "V8_OBJECT class JSInterceptorMap : public ExtendedMap {", "h 类头");
has(h, "V8_IT_REUSE_PARENT;", "h V8_IT_REUSE_PARENT");
has(h, "using SupportsFastIterableToListBit = base::BitField<bool, 0, 1, uint8_t>;", "h 位标志");
has(h, "uint8_t flags_;", "h flags_");
has(h, "uint8_t extended_padding_[kTaggedSize - 2];", "h padding");
has(h, "TaggedMember<InterceptorInfo> named_interceptor_;", "h named");
has(h, "TaggedMember<InterceptorInfo> indexed_interceptor_;", "h indexed");
has(h, "TaggedMember<Cell> fast_case_validity_cell_;", "h validity cell");

// tq 侧字段与条件 padding (@if/@ifnot 两条各计一)
const seg = tq.slice(tq.indexOf("extern class JSInterceptorMap"));
const tqF = [...seg.slice(0, seg.indexOf("\n}")).matchAll(/^  (@if\(\w+\)|@ifnot\(\w+\))? ?(\w+)(?:\[(\d+)\])?:\s*([^;]+);/gm)]
  .map((m) => [m[2], m[3] ? `${m[3]}×` : "", m[4].trim(), m[1] ?? ""]);
must(tqF.length === 6, `tq 字段应 6 (两条条件 padding 各计一), 实得 ${tqF.length}`);
must(tqF[0][0] === "flags" && tqF[0][2] === "uint8" && tqF[0][3] === "", "tq flags 失配");
const pads = tqF.filter((f) => f[0] === "extended_padding");
must(pads.length === 2, `tq padding 应 2 条, 实得 ${pads.length}`);
const padIf = pads.find((f) => f[3] === "@if(TAGGED_SIZE_8_BYTES)");
const padIfnot = pads.find((f) => f[3] === "@ifnot(TAGGED_SIZE_8_BYTES)");
must(padIf && padIf[1] === "6×", "tq padding[6] (@if TAGGED_SIZE_8_BYTES) 失配");
must(padIfnot && padIfnot[1] === "2×", "tq padding[2] (@ifnot TAGGED_SIZE_8_BYTES) 失配");
must(tqF.map((f) => f[0]).join(",") === "flags,extended_padding,extended_padding,named_interceptor,indexed_interceptor,fast_case_validity_cell",
  `tq 字段序失配: ${tqF.map((f) => f[0]).join(",")}`);

// ── 2. inl 行为锚点 ───────────────────────────────────────────────────────
has(inl, "Tagged<InterceptorInfo> JSInterceptorMap::named_interceptor() const", "inl named");
has(inl, "DCHECK(interceptor_info->is_named());", "inl named DCHECK");
has(inl, "DCHECK(!interceptor_info->is_named());", "inl indexed DCHECK");
has(inl, "SupportsFastIterableToListBit::decode(flags_)", "inl decode");
has(inl, "SupportsFastIterableToListBit::update(flags_, value)", "inl update");
has(inl, "void JSInterceptorMap::init_flags_and_clear_extended_padding() {", "inl init");
has(inl, "Tagged<InterceptorInfo> Map::GetNamedInterceptor() const {", "inl Map::GetNamedInterceptor");
has(inl, "Tagged<InterceptorInfo> Map::GetIndexedInterceptor() const {", "inl Map::GetIndexedInterceptor");

// ── 3. 64 位布局 (相对 ExtendedMap 头) ────────────────────────────────────
// flags_(1) + extended_padding_(6) = 7 → 隐式填充 1 对齐 tagged → 8; 3 tagged 字段 24
const T = 8;
const K_SELF_SIZE = Math.ceil((1 + (T - 2)) / T) * T + 3 * T; // 8 + 24 = 32
must(K_SELF_SIZE === 32, `自有字段段失配 ${K_SELF_SIZE}`);

const body = `${banner("src/objects/js-interceptor-map.h.js", "src/objects/js-interceptor-map.{h,-inl.h,tq}", "JSInterceptorMap 位标志与拦截器缓存布局")}
//! 差集骨架: JSInterceptorMap (ExtendedMap 的拦截器扩展视图, hasSameInstanceTypeAsParent)。
//! 配置: 64 位非压缩 (kTaggedSize=8 → extended_padding_[7], tq @if 分支)。

/// v8: src/objects/js-interceptor-map.h JSInterceptorMap::SupportsFastIterableToListBit
pub const K_SUPPORTS_FAST_ITERABLE_TO_LIST_MASK: u8 = 0x1;

/// v8: src/objects/js-interceptor-map.h JSInterceptorMap 自有字段段 (相对 ExtendedMap 头):
///   flags_ offset=0 (u8, 1 字节) + extended_padding_[7] + named_interceptor_ offset=8 +
///   indexed_interceptor_ offset=16 + fast_case_validity_cell_ offset=24
pub const K_JS_INTERCEPTOR_MAP_SELF_SIZE: usize = ${K_SELF_SIZE};

/// v8: src/objects/js-interceptor-map-inl.h named_interceptor DCHECK(is_named) / indexed DCHECK(!is_named)
pub struct JSInterceptorMap { /* flags_: u8, padding_[7], named/indexed_interceptor_, fast_case_validity_cell_ */ }

/// v8: src/objects/js-interceptor-map-inl.h Map::GetNamedInterceptor / GetIndexedInterceptor —
///   Cast<JSInterceptorMap>(map) 后取缓存; 与 FunctionTemplateInfo 的 handler 一致性 DCHECK 语义面留锚
/// v8: src/objects/js-interceptor-map-inl.h init_flags_and_clear_extended_padding — flags_=0 + memset padding
`;

const p = writeSrc("v8-objects-js-core", "js_interceptor_map.rs.skel", body);
console.log(`js-interceptor-map.h.js: ${p} — 字段 ${tqF.length}/5 (含条件 padding 6/7)、位标志 1、布局自段 ${K_SELF_SIZE}、断言 ${checks} 项通过`);
