#!/usr/bin/env bun
// tool/src/objects/js-function.cc.js — JSFunction/JSBoundFunction/JSWrappedFunction tq 布局 + cc 函数清单
// 组→文件映射 (本脚本管辖): v8/src/objects/js-function.{cc,tq}
//   (js-function.h 由既有席 js-function.h.js 覆盖; 本脚本只做 .cc/.tq 差集)
//   → v8-objects-js-func/src/js_function_cc.rs.skel
// 配置登记: 64 位非压缩 (kTaggedSize=8, JSObject 头 = 3 tagged = 24; dispatch_handle+padding 4+4)。
import { readV8, writeSrc, banner } from "../../lib.js";

const cc = readV8("src/objects/js-function.cc");
const tq = readV8("src/objects/js-function.tq");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};
const has = (s, sub, f) => {
  checks++;
  if (!s.includes(sub)) throw new Error(`锚点失配: ${f} 无 "${sub.slice(0, 64)}"`);
};

// ── 1. tq 类层级与字段真解析 ──────────────────────────────────────────────
const tqCls = [...tq.matchAll(/^extern class (\w+)(?:\s+extends\s+([\w\s]+?))?\s*(?:generates|\{)/gm)]
  .map((m) => [m[1], (m[2] ?? "").replace(/\s+/g, " ").trim()]);
const EXPECT_CLS = [
  ["JSFunctionOrBoundFunctionOrWrappedFunction", "JSObject"],
  ["JSBoundFunction", "JSFunctionOrBoundFunctionOrWrappedFunction"],
  ["JSWrappedFunction", "JSFunctionOrBoundFunctionOrWrappedFunction"],
  ["JSFunction", "JSFunctionOrBoundFunctionOrWrappedFunction"],
  ["JSFunctionWithoutPrototype", "JSFunction"],
  ["JSFunctionWithPrototype", "JSFunction"],
  ["JSClassConstructor", "JSFunctionWithPrototype"],
];
must(tqCls.length === 7, `tq 类应 7, 实得 ${tqCls.length}:\n${JSON.stringify(tqCls)}`);
for (let i = 0; i < 7; i++)
  must(tqCls[i][0] === EXPECT_CLS[i][0] && tqCls[i][1] === EXPECT_CLS[i][1], `tq 类[${i}] ${tqCls[i]} 期望 ${EXPECT_CLS[i]}`);

function tqFields(cls) {
  const at = tq.search(new RegExp(`^extern class ${cls}\\s`, "m"));
  must(at >= 0, `tq 缺类 ${cls}`);
  const body = tq.slice(at);
  return [...body.slice(0, body.indexOf("\n}")).matchAll(/^  (?:@if\(TAGGED_SIZE_8_BYTES\) )?(\w+):\s*([^;]+);/gm)]
    .map((m) => [m[1], m[2].trim()]);
}
const FIELDS = {
  JSBoundFunction: [["bound_target_function", "Callable"], ["bound_this", "JSAny|SourceTextModule"], ["bound_arguments", "FixedArray"]],
  JSWrappedFunction: [["wrapped_target_function", "Callable"], ["context", "NativeContext"]],
  JSFunction: [["dispatch_handle", "int32"], ["padding", "uint32"], ["shared_function_info", "SharedFunctionInfo"], ["context", "Context"], ["feedback_cell", "FeedbackCell"]],
  JSFunctionWithPrototype: [["prototype_or_initial_map", "JSReceiver|Map|Tuple2|TheHole"]],
};
for (const [cls, exp] of Object.entries(FIELDS)) {
  const got = tqFields(cls);
  must(JSON.stringify(got) === JSON.stringify(exp), `${cls} 字段失配: ${JSON.stringify(got)}`);
}
has(tq, "extern operator '.prototype_or_initial_map' macro", "tq operator load");
has(tq, "extern operator '.prototype_or_initial_map=' macro", "tq operator store");
has(tq, "@highestInstanceTypeWithinParentClassRange", "tq 注解");

// ── 2. cc 函数清单真解析 (定义处: 返回类型开头行) ─────────────────────────
const defs = [...cc.matchAll(/^(?:[A-Za-z_][\w:<>,*& ]* )?(JSFunction|JSBoundFunction|JSWrappedFunction|JSFunctionOrBoundFunctionOrWrappedFunction|Map)::(\w+)\(/gm)]
  .map((m) => `${m[1]}::${m[2]}`);
const unique = [...new Set(defs)];
must(unique.length >= 40, `cc 方法定义应 ≥40, 实得 ${unique.length}`);
for (const key of ["JSFunction::GetAttachedCodeKinds", "JSFunction::SetInterruptBudget",
  "JSBoundFunction::GetName", "JSBoundFunction::GetLength", "JSBoundFunction::ToString",
  "JSWrappedFunction::GetName", "JSWrappedFunction::Create", "JSFunction::GetDerivedMap",
  "JSFunction::SetInitialMap", "JSFunction::SetPrototype", "JSFunction::CalculateInstanceSizeHelper",
  "JSFunction::ComputeInstanceSizeWithMinSlack", "JSFunction::SetName", "JSFunction::GetDebugName",
  "JSFunction::ToString", "JSFunction::CalculateExpectedNofProperties",
  "JSFunctionOrBoundFunctionOrWrappedFunction::CopyNameAndLength",
  "JSFunction::EnsureHasInitialMap", "JSFunction::EnsureFeedbackVector"])
  must(unique.includes(key) || defs.includes(key), `cc 缺方法 ${key}`);
// 自由函数 (无类限定)
for (const f of ["bool CanSubclassHaveInobjectProperties(", "bool FastInitializeDerivedMap(",
  "int TypedArrayElementsKindToConstructorIndex(", "int TypedArrayElementsKindToRabGsabCtorIndex(",
  "bool UseFastFunctionNameLookup("])
  has(cc, f, "cc 自由函数");

// ── 3. cc 关键规则锚点 ────────────────────────────────────────────────────
has(cc, "const int kind_as_int_flag = static_cast<int>(CodeKindToCodeKindFlag(kind));", "cc tier flag");
has(cc, "const int mask = kind_as_int_flag | (kind_as_int_flag - 1);", "cc 含以下 tier 掩码");
has(cc, "const int highest_tier_log2 =", "cc highest_tier_log2");
has(cc, "nof_bound_arguments = Smi::kMaxValue;", "cc bound arguments 上限");

// ── 4. 64 位布局 ──────────────────────────────────────────────────────────
const T = 8;
const HO = 3 * T; // JSObject 24
const K_BOUND = HO + 3 * T; // 48
const K_WRAPPED = HO + 2 * T; // 40
const K_FUNC = HO + (4 + 4) + 3 * T; // dispatch_handle+padding 8 + 3 tagged → 56
const K_FUNC_WITH_PROTO = K_FUNC + T; // 64
must(K_BOUND === 48 && K_WRAPPED === 40 && K_FUNC === 56 && K_FUNC_WITH_PROTO === 64, "布局推导失配");

const body = `${banner("src/objects/js-function.cc.js", "src/objects/js-function.{cc,tq}", "JSFunction 族 tq 布局与 cc 方法清单骨架 (h 差集归 js-function.h.js 席)")}
//! 差集骨架: JSFunction / JSBoundFunction / JSWrappedFunction / JSClassConstructor。
//! 配置: 64 位非压缩 (kTaggedSize=8, JSFunction.dispatch_handle(4)+padding(4))。
//! tier 管理 / 初始 map / bound name&length 语义面留 Developer (js_function.rs 已有人工实现)。

// ── 64 位布局 ──
/// v8: src/objects/js-function.tq JSBoundFunction: bound_target_function offset=24,
///   bound_this 32 (JSAny|SourceTextModule), bound_arguments 40 (FixedArray)
pub const K_JS_BOUND_FUNCTION_HEADER_SIZE: usize = ${K_BOUND};
/// v8: src/objects/js-function.tq JSWrappedFunction: wrapped_target_function offset=24, context 32
pub const K_JS_WRAPPED_FUNCTION_HEADER_SIZE: usize = ${K_WRAPPED};
/// v8: src/objects/js-function.tq JSFunction: dispatch_handle offset=24 (int32) +
///   padding 28 (uint32, TAGGED_SIZE_8_BYTES), shared_function_info 32, context 40, feedback_cell 48
pub const K_JS_FUNCTION_HEADER_SIZE: usize = ${K_FUNC};
/// v8: src/objects/js-function.tq JSFunctionWithPrototype: prototype_or_initial_map offset=56
///   (TheHole | JSReceiver | Map | Tuple2 四态)
pub const K_JS_FUNCTION_WITH_PROTOTYPE_HEADER_SIZE: usize = ${K_FUNC_WITH_PROTO};

/// v8: src/objects/js-function.tq operator '.prototype_or_initial_map' / '.prototype_or_initial_map='
///   — LoadJSFunctionPrototypeOrInitialMap / StoreJSFunctionPrototypeOrInitialMap

// ── cc 方法清单 (${unique.length} 个类方法 + 5 自由函数) ──
/// v8: src/objects/js-function.cc
pub const JS_FUNCTION_CC_METHODS: &[&str] = &[
${unique.map((d) => `    "${d}",`).join("\n")}
];

/// v8: src/objects/js-function.cc GetAttachedCodeKinds/GetAvailableCodeKinds:
///   CodeKinds 位集 = kind_flag | (kind_flag - 1) 取连续低 tier; highest_tier_log2 语义面留锚
/// v8: src/objects/js-function.cc JSBoundFunction::GetLength — nof_bound_arguments 上限 Smi::kMaxValue
`;

const p = writeSrc("v8-objects-js-func", "js_function_cc.rs.skel", body);
console.log(
  `js-function.cc.js: ${p} — tq 类 ${tqCls.length}/7、字段 ${Object.values(FIELDS).flat().length}、` +
    `cc 方法 ${unique.length}+5 自由、布局 ${K_BOUND}/${K_WRAPPED}/${K_FUNC}/${K_FUNC_WITH_PROTO}、断言 ${checks} 项通过`,
);
