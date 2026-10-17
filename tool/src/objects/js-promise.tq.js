#!/usr/bin/env bun
// tool/src/objects/js-promise.tq.js — JSPromise tq 差集骨架 (bitfield 布局 + 字段槽位)
// 组→文件映射 (本脚本管辖): js-promise.tq (席位表差集; .h/-inl.h 归既有 js-promise.h.js)
// 产物: v8-objects-js-async/src/js_promise.rs.skel (落地版 js_promise.rs 就地修订, 本 skel 仅差集锚)
import { readV8, writeSrc, banner } from "../../lib.js";

const tq = readV8("src/objects/js-promise.tq");
const h = readV8("src/objects/js-promise.h");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`锚点失配: ${m}`); };
const eq = (a, e, m) => { checks++; if (a !== e) throw new Error(`锚点失配: ${m} (期望 ${e}, 实得 ${a})`); };

// ── 1. tq bitfield struct JSPromiseFlags 解析 ────────────────────────────
must(tq.includes("@cppScope('JSPromise')"), "tq 缺 @cppScope('JSPromise')");
const bfBody = tq.slice(tq.indexOf("bitfield struct JSPromiseFlags extends uint31 {"));
const bfFields = [...bfBody.matchAll(/^  (\w+): (\w+): (\d+) bit;/gm)].map((m) => [m[1], m[2], +m[3]]);
eq(bfFields.length, 5, "tq JSPromiseFlags 位域字段数");
const expectBf = [
  ["status", "PromiseState", 2],
  ["is_native_resolver_invoked", "bool", 1],
  ["has_handler", "bool", 1],
  ["is_silent", "bool", 1],
  ["async_task_id", "uint32", 26],
];
let shift = 0;
const layout = [];
for (let i = 0; i < 5; i++) {
  must(bfFields[i][0] === expectBf[i][0] && bfFields[i][1] === expectBf[i][1] && bfFields[i][2] === expectBf[i][2],
    `tq flags 字段 ${i} 应为 ${expectBf[i]}, 实得 ${bfFields[i]}`);
  layout.push({ name: bfFields[i][0], shift, bits: bfFields[i][2] });
  shift += bfFields[i][2];
}
eq(shift, 31, "uint31 位域总宽");

// ── 2. 与 .h BitField 链交叉核对 ────────────────────────────────────────
must(h.includes("using StatusBits = base::BitField<Promise::PromiseState, 0, 2, uint32_t>;"), "h StatusBits");
must(h.includes("using IsNativeResolverInvokedBit = StatusBits::Next<bool, 1>;"), "h IsNativeResolverInvokedBit");
must(h.includes("using HasHandlerBit = IsNativeResolverInvokedBit::Next<bool, 1>;"), "h HasHandlerBit");
must(h.includes("using IsSilentBit = HasHandlerBit::Next<bool, 1>;"), "h IsSilentBit");
must(h.includes("using AsyncTaskIdBits = IsSilentBit::Next<uint32_t, 26>;"), "h AsyncTaskIdBits");
must(h.includes("TaggedMember<UnionOf<PromiseReaction, JSAny>> reactions_or_result_;"), "h reactions_or_result_");
must(h.includes("TaggedMember<Smi> flags_ V8_TQ_TYPE(SmiTagged<JSPromiseFlags>);"), "h flags_ 槽");

// ── 3. tq extern class 字段 + macro 对数 ────────────────────────────────
must(tq.includes("extern class JSPromise extends JSObjectWithEmbedderSlots {"), "tq JSPromise 继承");
const clsBody = tq.slice(tq.indexOf("extern class JSPromise extends"), tq.indexOf("extern class JSPromiseConstructor"));
const tqFields = [...clsBody.matchAll(/^  (\w+): ([^;]+);$/gm)].map((m) => [m[1], m[2].trim()]);
eq(tqFields.length, 2, "tq JSPromise 数据字段数 (macro 之外)");
must(tqFields[0][0] === "reactions_or_result" && tqFields[0][1] === "PromiseReaction|JSAny", `tq 字段0 ${tqFields[0]}`);
must(tqFields[1][0] === "flags" && tqFields[1][1] === "SmiTagged<JSPromiseFlags>", `tq 字段1 ${tqFields[1]}`);
const macros = [...clsBody.matchAll(/^  macro (\w+)\(/gm)].map((m) => m[1]);
eq(macros.length, 4, "tq macro 数");
must(macros.join(",") === "Status,SetStatus,HasHandler,SetHasHandler", `tq macro 名序 ${macros}`);
must(tq.includes("extern class JSPromiseConstructor extends JSFunctionWithPrototype"), "tq JSPromiseConstructor");

// ── 4. 槽位偏移机械推导 (非压缩 8B tagged; JSObject 头 3 槽) ────────────
const HEAD = 24; // map + properties_or_hash + elements
const K_REACTIONS_OR_RESULT = HEAD;
const K_FLAGS = HEAD + 8;
const K_SIZE = K_FLAGS + 8;
must(K_SIZE === 40, "JSPromise 头 5 槽");

// ── 5. 骨架产物 ──────────────────────────────────────────────────────────
const flagDefs = layout.map((f) =>
  `pub const K_${f.name.toUpperCase()}_SHIFT: u32 = ${f.shift}; pub const K_${f.name.toUpperCase()}_BITS: u32 = ${f.bits};`
).join("\n");
const body = `${banner("src/objects/js-promise.tq.js", "src/objects/js-promise.tq + js-promise.h", "JSPromise tq 差集: flags 位域与槽位锚")}
//! 差集骨架: js-promise.tq 视角 (bitfield struct + extern class + macro);
//! .h/-inl.h 布局常量归 tool/src/objects/js-promise.h.js, 语义以落地版 js_promise.rs 为准。

// JSPromiseFlags (uint31, tq js-promise.tq:6-12; 与 h BitField 链 1:1)
${flagDefs}
pub const K_MAX_ASYNC_TASK_ID: u32 = (1 << K_ASYNC_TASK_ID_BITS) - 1; // 0x3FF_FFFF

// 槽位 (非压缩 8B): JSObject 头 3 槽后 2 槽
/// v8: src/objects/js-promise.tq JSPromise.reactions_or_result
pub const K_REACTIONS_OR_RESULT_OFFSET: usize = ${K_REACTIONS_OR_RESULT};
/// v8: src/objects/js-promise.tq JSPromise.flags
pub const K_FLAGS_OFFSET: usize = ${K_FLAGS};
/// v8: src/objects/js-promise.h JSPromise::kHeaderSize (sizeof, 5 words)
pub const K_HEADER_SIZE: usize = ${K_SIZE};

// tq macro (语义在 Torque/builtins 层, Rust 侧为 Tagged 访问器):
//   Status() / SetStatus(kPending→kFulfilled|kRejected) / HasHandler() / SetHasHandler()
//   dcheck 约束: SetStatus 仅可自 kPending 迁出且目标不可为 kPending
`;

const p = writeSrc("v8-objects-js-async", "js_promise.rs.skel", body);
console.log(
  `js-promise.tq.js: ${p} — tq 位域字段 ${bfFields.length}/5 (总宽 ${shift} bit)、tq 数据字段 ${tqFields.length}/2、macro ${macros.length}/4、偏移 ${K_REACTIONS_OR_RESULT}/${K_FLAGS}/${K_SIZE}、断言 ${checks} 项全部通过`,
);
