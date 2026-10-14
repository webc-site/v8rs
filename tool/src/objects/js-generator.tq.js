#!/usr/bin/env bun
// tool/src/objects/js-generator.tq.js — JSGenerator 族 tq 布局 + continuation 哨兵提取
// 组→文件映射 (本脚本管辖): v8/src/objects/js-generator.tq
//   (js-generator.h 由既有席 js-generator.h.js 覆盖; 本脚本只做 .tq 差集)
//   → v8-objects-js-async/src/js_generator_tq.rs.skel
// 配置登记: 64 位非压缩 (kTaggedSize=8, JSObject 头 = 3 tagged = 24; Struct 头 = 1 tagged = 8)。
import { readV8, writeSrc, banner } from "../../lib.js";

const tq = readV8("src/objects/js-generator.tq");
const h = readV8("src/objects/js-generator.h"); // 只读锚点: 哨兵值 + ResumeMode

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};
const has = (s, sub, f) => {
  checks++;
  if (!s.includes(sub)) throw new Error(`锚点失配: ${f} 无 "${sub.slice(0, 64)}"`);
};

// ── 1. tq generates 常量 (4) + h 侧取值 ───────────────────────────────────
const gen = [...tq.matchAll(/^const (\w+):\s*constexpr (\w+) generates '([^']*)';/gm)]
  .map((m) => [m[1], m[2], m[3]]);
must(gen.length === 4, `tq generates 常量应 4, 实得 ${gen.length}`);
for (const [n, , sym] of gen) must(sym.startsWith("JSGeneratorObject::"), `generates 符号非常量引用: ${sym}`);
// h: enum ResumeMode { kNext, kReturn, kThrow, kRethrow }; → kNext=0, kThrow=2
// h: kGeneratorExecuting = -2; kGeneratorClosed = -1;
has(h, "enum ResumeMode { kNext, kReturn, kThrow, kRethrow };", "h ResumeMode");
has(h, "static const int kGeneratorExecuting = -2;", "h executing");
has(h, "static const int kGeneratorClosed = -1;", "h closed");
// tq 常量名 ↔ generates 目标对照
const tqMap = Object.fromEntries(gen.map(([n, , s]) => [n, s]));
must(tqMap.kGeneratorExecuting === "JSGeneratorObject::kGeneratorExecuting", "executing 映射失配");
must(tqMap.kGeneratorClosed === "JSGeneratorObject::kGeneratorClosed", "closed 映射失配");
must(tqMap.kGeneratorResumeModeNext === "JSGeneratorObject::kNext", "kNext 映射失配");
must(tqMap.kGeneratorResumeModeThrow === "JSGeneratorObject::kThrow", "kThrow 映射失配");

// ── 2. tq 类字段真解析 (4 类) ─────────────────────────────────────────────
function tqFields(cls) {
  const seg = tq.slice(tq.indexOf(`extern class ${cls}`));
  const body = seg.slice(0, seg.indexOf("\n}"));
  return [...body.matchAll(/^  (\w+):\s*([^;]+);/gm)].map((m) => [m[1], m[2].trim()]);
}
const FIELDS = {
  JSGeneratorObject: [["function", "JSFunction"], ["context", "Context"], ["receiver", "JSAny"],
    ["input_or_debug_pos", "Object"], ["resume_mode", "Smi"], ["continuation", "Smi"],
    ["parameters_and_registers", "FixedArray"], ["yielded_value", "JSAny"]],
  JSAsyncFunctionObject: [["promise", "JSPromise"], ["await_resolve_closure", "JSFunction|Undefined"],
    ["await_reject_closure", "JSFunction|Undefined"]],
  JSAsyncGeneratorObject: [["queue", "AsyncGeneratorRequest|Undefined"], ["is_awaiting", "Smi"]],
  AsyncGeneratorRequest: [["next", "AsyncGeneratorRequest|Undefined"], ["resume_mode", "Smi"],
    ["value", "Object"], ["promise", "JSPromise"]],
};
for (const [cls, exp] of Object.entries(FIELDS)) {
  const got = tqFields(cls);
  must(JSON.stringify(got) === JSON.stringify(exp), `${cls} 字段失配: ${JSON.stringify(got)}`);
}
// 注释锚: await closure 懒分配; continuation 正值=挂起
has(tq, "// Resolve/reject closures for await, reused across all await expressions.", "tq await 注释");
has(tq, "// A positive value indicates a suspended generator.", "tq continuation 注释");

// ── 3. 64 位布局 (与 js-generator.h.js 席数值互证) ────────────────────────
const T = 8;
const K_GEN = 3 * T + 8 * T; // 88
const K_ASYNC_FUNC = K_GEN + 3 * T; // 112
const K_ASYNC_GEN = K_GEN + 2 * T; // 104
const K_REQ = 1 * T + 4 * T; // Struct(8) + 4 字段 = 40
must(K_GEN === 88 && K_ASYNC_FUNC === 112 && K_ASYNC_GEN === 104 && K_REQ === 40, "布局推导失配");

const body = `${banner("src/objects/js-generator.tq.js", "src/objects/js-generator.tq (+js-generator.h 只读哨兵锚点)", "JSGenerator 族 tq 布局与 continuation 哨兵")}
//! 差集骨架: JSGeneratorObject / JSAsyncFunctionObject / JSAsyncGeneratorObject / AsyncGeneratorRequest。
//! 配置: 64 位非压缩 (kTaggedSize=8)。resume/await 语义面留 Developer (js_generator.rs 已有人工实现)。

// ── continuation 哨兵 (tq generates → h 取值) ──
/// v8: src/objects/js-generator.tq kGeneratorExecuting → js-generator.h JSGeneratorObject::kGeneratorExecuting
pub const K_GENERATOR_EXECUTING: i32 = -2;
/// v8: src/objects/js-generator.tq kGeneratorClosed → js-generator.h JSGeneratorObject::kGeneratorClosed
pub const K_GENERATOR_CLOSED: i32 = -1;
/// v8: src/objects/js-generator.tq kGeneratorResumeModeNext → JSGeneratorObject::kNext (ResumeMode 枚举)
pub const K_RESUME_MODE_NEXT: i32 = 0;
/// v8: src/objects/js-generator.tq kGeneratorResumeModeThrow → JSGeneratorObject::kThrow
pub const K_RESUME_MODE_THROW: i32 = 2;
/// v8: src/objects/js-generator.h enum ResumeMode { kNext=0, kReturn=1, kThrow=2, kRethrow=3 }

// ── 64 位布局 ──
/// v8: src/objects/js-generator.tq JSGeneratorObject: function 24, context 32, receiver 40,
///   input_or_debug_pos 48, resume_mode 56, continuation 64, parameters_and_registers 72,
///   yielded_value 80
pub const K_JS_GENERATOR_OBJECT_HEADER_SIZE: usize = ${K_GEN};
/// v8: src/objects/js-generator.tq JSAsyncFunctionObject: promise 88, await_resolve_closure 96,
///   await_reject_closure 104 (懒分配, 初值 undefined)
pub const K_JS_ASYNC_FUNCTION_OBJECT_HEADER_SIZE: usize = ${K_ASYNC_FUNC};
/// v8: src/objects/js-generator.tq JSAsyncGeneratorObject: queue 88, is_awaiting 96
pub const K_JS_ASYNC_GENERATOR_OBJECT_HEADER_SIZE: usize = ${K_ASYNC_GEN};
/// v8: src/objects/js-generator.tq AsyncGeneratorRequest (Struct 头 8): next 8, resume_mode 16,
///   value 24, promise 32
pub const K_ASYNC_GENERATOR_REQUEST_HEADER_SIZE: usize = ${K_REQ};
`;

const p = writeSrc("v8-objects-js-async", "js_generator_tq.rs.skel", body);
console.log(
  `js-generator.tq.js: ${p} — generates 常量 4/4、类 4 (字段 ${Object.values(FIELDS).flat().length})、` +
    `布局 ${K_GEN}/${K_ASYNC_FUNC}/${K_ASYNC_GEN}/${K_REQ}、断言 ${checks} 项通过`,
);
