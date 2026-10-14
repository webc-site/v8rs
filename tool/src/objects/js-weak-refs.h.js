#!/usr/bin/env bun
// tool/src/objects/js-weak-refs.h.js — JSFinalizationRegistry/WeakCell/JSWeakRef 骨架 (3 类 + 1bit flag)
// 组→文件映射: js-weak-refs.h + js-weak-refs-inl.h + js-weak-refs.cc + js-weak-refs.tq
// 产物: v8-objects-js-core/src/js_weak_refs.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/js-weak-refs.h");
const inl = readV8("src/objects/js-weak-refs-inl.h");
const cc = readV8("src/objects/js-weak-refs.cc");
const tq = readV8("src/objects/js-weak-refs.tq");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`锚点失配: ${m}`); };
const eq = (a, e, m) => { checks++; if (a !== e) throw new Error(`锚点失配: ${m} (期望 ${e}, 实得 ${a})`); };
const fieldsOf = (src, from, to) => [...src.slice(src.indexOf(from), src.indexOf(to, src.indexOf(from)))
  .matchAll(/^\s{2}TaggedMember<(.+?)> (\w+)(?: V8_TQ_TYPE\([^)]*\))?;/gm)].map((m) => [m[2], m[1]]);

// ── 1. JSFinalizationRegistry: 7 字段 ──────────────────────────────────
must(h.includes("V8_OBJECT class JSFinalizationRegistry : public JSObject {"), "类签名");
const frFields = fieldsOf(h, "V8_OBJECT class JSFinalizationRegistry", "inline constexpr int JSFinalizationRegistry::kHeaderSize");
eq(frFields.length, 7, "JSFinalizationRegistry 字段数");
must(frFields.map((f) => f[0]).join(",") === "native_context_,cleanup_,active_cells_,cleared_cells_,key_map_,next_dirty_,flags_", `字段序 ${frFields.map((f) => f[0])}`);
must(frFields[2][1] === "UnionOf<WeakCell, Undefined>", `active_cells 类型 ${frFields[2][1]}`);
must(frFields[5][1] === "UnionOf<JSFinalizationRegistry, Undefined>", `next_dirty 类型 ${frFields[5][1]}`);
must(h.includes("using ScheduledForCleanupBit = base::BitField<bool, 0, 1, uint32_t>;"), "ScheduledForCleanupBit bit0");
must(h.includes("kScheduledForCleanup = ScheduledForCleanupBit::kMask,"), "kScheduledForCleanup = kMask");
must(h.includes("static constexpr int kFlagCount = 1;"), "kFlagCount = 1");
must(h.includes("enum RemoveUnregisterTokenMode {\n    kRemoveMatchedCellsFromRegistry,\n    kKeepMatchedCellsInRegistry\n  };"), "RemoveUnregisterTokenMode 枚举");
must(h.includes("DECL_BOOLEAN_ACCESSORS(scheduled_for_cleanup)"), "scheduled_for_cleanup 布尔访问器");
must(h.includes("inline bool NeedsCleanup() const;"), "NeedsCleanup 声明");

// ── 2. WeakCell: 8 字段 (HeapObject 直继) ──────────────────────────────
must(h.includes("V8_OBJECT class WeakCell : public HeapObject {"), "WeakCell 继承 HeapObject");
const wcFields = fieldsOf(h, "V8_OBJECT class WeakCell", "V8_OBJECT class JSWeakRef");
eq(wcFields.length, 8, "WeakCell 字段数");
must(wcFields.map((f) => f[0]).join(",") === "finalization_registry_,holdings_,target_,unregister_token_,prev_,next_,key_list_prev_,key_list_next_", `字段序 ${wcFields.map((f) => f[0])}`);
must(wcFields[2][1] === "UnionOf<Symbol, JSReceiver, Undefined>", `target 类型 ${wcFields[2][1]}`);
must(h.includes("inline Tagged<HeapObject> relaxed_target() const;"), "relaxed_target 声明");
must(h.includes("inline Tagged<HeapObject> relaxed_unregister_token() const;"), "relaxed_unregister_token 声明");
// target/unregister_token 无 setter 公开 (GC 内部私改)
must(h.includes("inline void set_target(Tagged<UnionOf<Symbol, JSReceiver, Undefined>> value,"), "set_target 私有");

// ── 3. JSWeakRef: 1 字段 ───────────────────────────────────────────────
must(h.includes("V8_OBJECT class JSWeakRef : public JSObject {"), "JSWeakRef 类签名");
const wrFields = fieldsOf(h, "V8_OBJECT class JSWeakRef", "}  // namespace internal");
eq(wrFields.length, 1, "JSWeakRef 字段数");
must(wrFields[0][0] === "target_", `字段 ${wrFields[0][0]}`);

// ── 4. inl + cc + tq 交叉 ──────────────────────────────────────────────
must(inl.includes("int JSFinalizationRegistry::flags() const { return flags_.load().value(); }"), "inl flags");
must(inl.includes("BIT_FIELD_ACCESSORS(JSFinalizationRegistry, flags, scheduled_for_cleanup,"), "inl BIT_FIELD_ACCESSORS");
must(tq.includes("bitfield struct FinalizationRegistryFlags extends uint31 {"), "tq bitfield");
must(tq.includes("scheduled_for_cleanup: bool: 1 bit;"), "tq 1bit");
const tqFr = [...tq.slice(tq.indexOf("extern class JSFinalizationRegistry"), tq.indexOf("\n}", tq.indexOf("extern class JSFinalizationRegistry")))
  .matchAll(/^  (\w+): /gm)].map((m) => m[1]);
eq(tqFr.length, 7, "tq JSFinalizationRegistry 字段数");
const tqWc = [...tq.slice(tq.indexOf("extern class WeakCell"), tq.indexOf("\n}", tq.indexOf("extern class WeakCell")))
  .matchAll(/^  (\w+): /gm)].map((m) => m[1]);
eq(tqWc.length, 8, "tq WeakCell 字段数");
for (const sig of ["JSFinalizationRegistry::Cleanup(", "JSFinalizationRegistry::PopClearedCell(",
  "JSFinalizationRegistry::ShrinkKeyMap(", "JSFinalizationRegistry::RemoveCellFromUnregisterTokenMap("])
  must(cc.includes(sig), `cc 方法 ${sig}`);
must(h.includes("Tagged<WeakCell> JSFinalizationRegistry::PopClearedCell(") ||
  h.includes("V8_EXPORT_PRIVATE Tagged<WeakCell> PopClearedCell("), "PopClearedCell 导出");

// ── 5. 槽位推导 (非压缩 8B; JSObject 头 3 槽 / HeapObject 头 1 槽) ────
const FR0 = 24, WC0 = 8, WR0 = 24;
const frOff = frFields.map((_, i) => FR0 + i * 8);
const wcOff = wcFields.map((_, i) => WC0 + i * 8);

// ── 6. 骨架产物 ──────────────────────────────────────────────────────────
const body = `${banner("src/objects/js-weak-refs.h.js", "src/objects/js-weak-refs.h + -inl.h + .cc + .tq", "Weak Refs 三类槽位与 GC 链表差集骨架")}
//! JSFinalizationRegistry: active/cleared 双链 + key_map; WeakCell 同时挂
//! registry 链与 key 链 (key_list_prev/next); target/unregister_token 为弱引用。

// JSFinalizationRegistry (7 槽 @${FR0}..${frOff[6]}):
${frFields.map((f, i) => `/// v8: src/objects/js-weak-refs.h JSFinalizationRegistry::${f[0]}\npub const K_${f[0].replace(/_$/, "").toUpperCase()}_OFFSET: usize = ${frOff[i]};`).join("\n")}
pub const K_FR_HEADER_SIZE: usize = ${FR0 + 7 * 8};
/// v8: src/objects/js-weak-refs.h JSFinalizationRegistry::ScheduledForCleanupBit
pub const K_SCHEDULED_FOR_CLEANUP_SHIFT: u32 = 0;
pub const K_SCHEDULED_FOR_CLEANUP_MASK: u32 = 1;
pub const K_FLAG_COUNT: usize = 1;

// WeakCell (8 槽 @${WC0}..${wcOff[7]}, HeapObject 直继):
${wcFields.map((f, i) => `/// v8: src/objects/js-weak-refs.h WeakCell::${f[0]}\npub const K_WC_${f[0].replace(/_$/, "").toUpperCase()}_OFFSET: usize = ${wcOff[i]};`).join("\n")}
pub const K_WC_HEADER_SIZE: usize = ${WC0 + 8 * 8};

// JSWeakRef (1 槽):
/// v8: src/objects/js-weak-refs.h JSWeakRef::target_
pub const K_WR_TARGET_OFFSET: usize = ${WR0};
pub const K_WR_HEADER_SIZE: usize = ${WR0 + 8};

// 方法映射 (GC 协作, 语义推迟): RegisterWeakCellWithUnregisterToken / Unregister /
//   RemoveUnregisterToken (kRemoveMatchedCellsFromRegistry|kKeepMatchedCellsInRegistry) /
//   NeedsCleanup / PopClearedCell / ShrinkKeyMap / Cleanup / RemoveCellFromUnregisterTokenMap;
//   WeakCell::Nullify (GC 期改槽, gc_notify_updated_slot 回调) / relaxed_target / relaxed_unregister_token
`;

const p = writeSrc("v8-objects-js-core", "js_weak_refs.rs.skel", body);
console.log(
  `js-weak-refs.h.js: ${p} — FR 字段 ${frFields.length}/7 (size ${FR0 + 56})、WeakCell ${wcFields.length}/8 (size ${WC0 + 64})、JSWeakRef 1/1、tq 交叉 7/8、cc 方法 4/4、断言 ${checks} 项全部通过`,
);
