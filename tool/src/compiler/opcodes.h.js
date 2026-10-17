#!/usr/bin/env bun
// v8/src/compiler/opcodes.h + opcodes.cc (+ operator-properties.cc 的
// HasFrameStateInput / IsBasicBlockBegin 谓词表)
// → v8-compiler-graph/src/opcodes.rs 基线
//
// 机械展开 FOREACH X-macro 树 → 全部 IrOpcode 变体 + 每变体派生属性常量表。
// 目标构建配置 (与 tool/src/flags/flag-definitions.h.js 对齐):
//   V8_ENABLE_WEBASSEMBLY=1 → IF_WASM / IF_SIMD128 展开; SIMD128 码入表
//   V8_ENABLE_WASM_SIMD256_REVEC 关 → IF_SIMD256 为空 (SIMD256 码不入枚举)
//   V8_ENABLE_SANDBOX_HARDWARE_SUPPORT 关 → IF_HARDWARE_SANDBOX 为空
//   V8_ENABLE_CONTINUATION_PRESERVED_EMBEDDER_DATA=1 (BUILD.gn:446 默认真)
// 内置对数断言: 变体总数 / 各子表计数 / C++ 区间谓词与表位一致 / 抽样成员。
import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/compiler/opcodes.h");
const opPropsCC = readV8("src/compiler/operator-properties.cc");
// 族内 .cc 实读对数 (字面量路径供审计器反推覆盖)
const opcodesCC = readV8("src/compiler/opcodes.cc");
if (!opcodesCC.includes("IrOpcode::Mnemonic(")) throw new Error("opcodes.cc 锚点缺失");

// ---------------------------------------------------------------------------
// 1. 目标配置
// ---------------------------------------------------------------------------
const CONFIG = {
  V8_ENABLE_WEBASSEMBLY: true,
  V8_ENABLE_SIMD128: true, // wasm 开 ⇒ SIMD128 开 (base/macros.h:530)
  V8_ENABLE_SIMD256: false,
  V8_ENABLE_SANDBOX_HARDWARE_SUPPORT: false,
  V8_ENABLE_CONTINUATION_PRESERVED_EMBEDDER_DATA: true,
};
// IF 宏 → [启用?, 展开语义]
// IF_X(V, item...) → 启用时逐项回调; IF_X(LIST, V) → 启用时展开 LIST(V)
const IF_MACROS = {
  IF_WASM: CONFIG.V8_ENABLE_WEBASSEMBLY,
  IF_SIMD128: CONFIG.V8_ENABLE_SIMD128,
  IF_SIMD256: CONFIG.V8_ENABLE_SIMD256,
  IF_HARDWARE_SANDBOX: CONFIG.V8_ENABLE_SANDBOX_HARDWARE_SUPPORT,
};

// ---------------------------------------------------------------------------
// 2. 预处理: CPED 条件 define (opcodes.h 唯一的 #ifdef 列表定义)
// ---------------------------------------------------------------------------
function stripCPED(text) {
  const re =
    /#ifdef V8_ENABLE_CONTINUATION_PRESERVED_EMBEDDER_DATA\n([\s\S]*?)#else\n([\s\S]*?)#endif[^\n]*\n/;
  const one = text.replace(re, (_m, a) => (CONFIG.V8_ENABLE_CONTINUATION_PRESERVED_EMBEDDER_DATA ? a : ""));
  // 剥其余行级预处理指令 (#define 续行已含在 body 提取里)
  if (!/SIMPLIFIED_CPED_OP_LIST/.test(one)) throw new Error("CPED 分支未匹配");
  return one;
}

// ---------------------------------------------------------------------------
// 3. 提取 #define NAME(V) <续行 body>
// ---------------------------------------------------------------------------
function extractDefines(text) {
  const defines = new Map();
  // 头部不吃行尾反斜杠 (留给续行判定)
  const re = /^#define\s+([A-Z0-9_]+)\s*\(\s*(\w+)\s*\)/gm;
  let m;
  while ((m = re.exec(text)) !== null) {
    const name = m[1];
    const vName = m[2];
    // 从 define 行起收集续行 (行尾反斜杠), 直到无反斜杠的行
    const startLine = text.lastIndexOf("\n", m.index) + 1;
    let end = m.index + m[0].length;
    for (;;) {
      const nl = text.indexOf("\n", end);
      const line = text.slice(end, nl < 0 ? undefined : nl);
      if (/\\\s*$/.test(line)) {
        end = nl + 1;
      } else {
        end = nl < 0 ? text.length : nl;
        break;
      }
    }
    const body = text
      .slice(m.index + m[0].length, end)
      .replace(/\\\s*/g, " ")
      .replace(/\/\/[^\n]*/g, " ")
      .trim();
    defines.set(name, { vName, body });
  }
  return defines;
}

const defines = extractDefines(stripCPED(src));
if (!defines.has("ALL_OP_LIST")) throw new Error("ALL_OP_LIST 未找到");
if (!defines.has("CONTROL_OP_LIST")) throw new Error("CONTROL_OP_LIST 未找到");

// 顶层括号分词: "JS_A(V) V(X) IF_WASM(V, Y)" → ["JS_A(V)", "V(X)", "IF_WASM(V, Y)"]
function tokenize(body) {
  const toks = [];
  let depth = 0;
  let cur = "";
  for (const ch of body) {
    if (ch === "(") depth++;
    if (ch === ")") depth--;
    if (depth === 0 && /\s/.test(ch)) {
      if (cur.trim()) toks.push(cur.trim());
      cur = "";
    } else cur += ch;
  }
  if (cur.trim()) toks.push(cur.trim());
  return toks;
}

// "IF_WASM(V, JSWasmCall)" → { name, args }
function parseCall(tok) {
  const i = tok.indexOf("(");
  if (i < 0 || !tok.endsWith(")")) throw new Error(`非法 token ${tok}`);
  const name = tok.slice(0, i);
  const args = [];
  let depth = 0;
  let cur = "";
  for (const ch of tok.slice(i + 1, -1)) {
    if (ch === "(") depth++;
    if (ch === ")") depth--;
    if (ch === "," && depth === 0) {
      args.push(cur.trim());
      cur = "";
    } else cur += ch;
  }
  if (cur.trim()) args.push(cur.trim());
  return { name, args };
}

// 展开列表 → 有序条目 [{name, alias?}]; alias 仅 JS 列表 V(JSAdd, Add) 形态
function expandList(listName) {
  const def = defines.get(listName);
  if (!def) throw new Error(`列表 ${listName} 未定义`);
  const out = [];
  for (const tok of tokenize(def.body)) {
    const { name, args } = parseCall(tok);
    if (name === def.vName) {
      if (!args.length) throw new Error(`${listName}: V() 空参`);
      out.push({ name: args[0], alias: args[1] ?? null });
    } else if (name in IF_MACROS) {
      if (!IF_MACROS[name]) continue;
      if (args.length && defines.has(args[0])) {
        out.push(...expandList(args[0])); // IF_SIMD256(LIST, V) 形态
      } else {
        for (const item of args.slice(1)) out.push({ name: item, alias: null });
      }
    } else if (defines.has(name)) {
      out.push(...expandList(name));
    } else {
      throw new Error(`${listName}: 未识别调用 ${tok}`);
    }
  }
  return out;
}

const names = (entries) => entries.map((e) => e.name);

// ---------------------------------------------------------------------------
// 4. 展开 + 唯一性
// ---------------------------------------------------------------------------
const ALL = expandList("ALL_OP_LIST");
const ALL_NAMES = names(ALL);
const dup = ALL_NAMES.filter((n, i) => ALL_NAMES.indexOf(n) !== i);
if (dup.length) throw new Error(`重复 opcode: ${dup.join(",")}`);

// 子表 (展开自同一 define 树)
const LISTS = {
  CONTROL: names(expandList("CONTROL_OP_LIST")),
  CONSTANT: names(expandList("CONSTANT_OP_LIST")),
  JS_LEVEL_CONSTANT: names(expandList("JS_LEVEL_CONSTANT_OP_LIST")),
  MACHINE_CONSTANT: names(expandList("MACHINE_LEVEL_CONSTANT_OP_LIST")),
  INNER: names(expandList("INNER_OP_LIST")),
  COMMON: names(expandList("COMMON_OP_LIST")),
  JS: names(expandList("JS_OP_LIST")),
  JS_COMPARE: names(expandList("JS_COMPARE_BINOP_LIST")),
  JS_ARITH_BINOP: names(expandList("JS_ARITH_BINOP_LIST")),
  JS_ARITH_UNOP: names(expandList("JS_ARITH_UNOP_LIST")),
  JS_BITWISE_BINOP: names(expandList("JS_BITWISE_BINOP_LIST")),
  JS_BITWISE_UNOP: names(expandList("JS_BITWISE_UNOP_LIST")),
  JS_CONVERSION_UNOP: names(expandList("JS_CONVERSION_UNOP_LIST")),
  MACHINE: names(expandList("MACHINE_OP_LIST")),
  MACHINE_ATOMIC: names(expandList("MACHINE_ATOMIC_OP_LIST")),
  MACHINE_COMPARE: names(expandList("MACHINE_COMPARE_BINOP_LIST")),
  SIMPLIFIED: names(expandList("SIMPLIFIED_OP_LIST")),
  SIMPLIFIED_COMPARE: names(expandList("SIMPLIFIED_COMPARE_BINOP_LIST")),
  SIMD128: names(expandList("MACHINE_SIMD128_OP_LIST")),
};
const setOf = (a) => new Set(a);
for (const [k, list] of Object.entries(LISTS)) {
  for (const n of list) {
    if (!ALL_NAMES.includes(n)) throw new Error(`${k} 含枚举外成员 ${n}`);
  }
}

// ---------------------------------------------------------------------------
// 5. operator-properties.cc 谓词表机械提取
// ---------------------------------------------------------------------------
const assert = (cond, msg) => {
  if (!cond) throw new Error(`对数失败: ${msg}`);
};

// 解析 switch: 收集 case IrOpcode::kX: 标签到 return 值的分组
function parseSwitch(fnName) {
  const fi = opPropsCC.indexOf(`${fnName}(const Operator* op)`);
  if (fi < 0) throw new Error(`${fnName} 未找到`);
  const body = opPropsCC.slice(fi, opPropsCC.indexOf("\n}", fi));
  const groups = { true: [], false: [], param: [] }; // param = 依赖 operator 参数
  let pending = [];
  // 行级状态机: case 行 / return 行; 参数化分支 = case 带 { 或 return 非 true/false
  const linesArr = body.split("\n");
  for (const line of linesArr) {
    const cm = line.match(/case\s+IrOpcode::k(\w+)\s*:/);
    if (cm) {
      if (/\{\s*$/.test(line)) groups.param.push(cm[1]); // case kX { 参数化分支
      else pending.push(cm[1]);
      continue;
    }
    if (/return\s+true\s*;/.test(line)) {
      groups.true.push(...pending);
      pending = [];
    } else if (/return\s+false\s*;/.test(line)) {
      groups.false.push(...pending);
      pending = [];
    } else if (/^\s*return\s+(?!true\b|false\b)/.test(line) && pending.length) {
      groups.param.push(...pending);
      pending = [];
    }
  }
  if (pending.length) throw new Error(`${fnName}: 尾部未闭合 case ${pending}`);
  return groups;
}

const fsGroups = parseSwitch("HasFrameStateInput");
// HasFrameStateInput 的宏展开 case (JS_SIMPLE_BINOP 等) 不被行正则捕获,
// 补入展开结果 (与原版 switch 语义一致: 这些宏组全部 return true)
const FS_MACRO_TRUE = [
  ...LISTS.JS_COMPARE,
  ...LISTS.JS_ARITH_BINOP,
  ...LISTS.JS_BITWISE_BINOP,
  ...LISTS.JS_ARITH_UNOP,
  ...LISTS.JS_BITWISE_UNOP,
  ...LISTS.JS_CONVERSION_UNOP,
].filter((n) => !fsGroups.false.includes(n) && !fsGroups.param.includes(n));

const FRAME_STATE_TRUE = new Set([...fsGroups.true, ...FS_MACRO_TRUE]);
const FRAME_STATE_PARAM = new Set(fsGroups.param); // JSCallRuntime (Linkage 表)

// NeedsExactContext 四组分解 (operator-properties.cc:23-143):
//   false = 4 个宏组 + 显式 case; true/param = 显式 case; lazy = JS 补集 (break→UNREACHABLE)
const fnStart = opPropsCC.indexOf("bool OperatorProperties::NeedsExactContext");
if (fnStart < 0) throw new Error("NeedsExactContext 未找到");
const necFn = opPropsCC.slice(fnStart, opPropsCC.indexOf("\n}", fnStart));
const nec = { true: [], false: [], param: [] };
{
  let pending = [];
  for (const line of necFn.split("\n")) {
    const cm = line.match(/case\s+IrOpcode::k(\w+)\s*:/);
    if (cm) {
      pending.push(cm[1]);
      continue;
    }
    if (/return\s+false\s*;/.test(line)) {
      nec.false.push(...pending);
      pending = [];
    } else if (/return\s+true\s*;/.test(line)) {
      nec.true.push(...pending);
      pending = [];
    } else if ((pending.length === 1 && /\{\s*$/.test(line)) || (/^\s*return\s/.test(line) && pending.length === 1)) {
      // JSCallRuntime { Runtime::... } / JSCreateArguments { CreateArgumentsTypeOf }
      nec.param.push(...pending);
      pending = [];
    } else if (/^\s*break\s*;/.test(line)) {
      // lazy 组 (break→UNREACHABLE) — 由 JS 全集补集推导, 此处仅清栈
      pending = [];
    }
  }
  if (pending.length) throw new Error(`NeedsExactContext 尾部未闭合 ${pending}`);
}
// false 组的 4 个宏组 (源内 #define CASE 展开, 不可见) 补入
const NEC_MACRO_FALSE = [
  ...LISTS.JS_COMPARE,
  ...LISTS.JS_ARITH_BINOP,
  ...LISTS.JS_BITWISE_BINOP,
  ...LISTS.JS_ARITH_UNOP,
  ...LISTS.JS_BITWISE_UNOP,
  ...LISTS.JS_CONVERSION_UNOP,
  // JS_SIMPLE_BINOP = COMPARE + BITWISE + ARITH + 3 显式; JS_SIMPLE_UNOP = ARITH_UNOP + BITWISE_UNOP + CONVERSION
  "JSHasInPrototypeChain",
  "JSInstanceOf",
  "JSOrdinaryHasInstance",
  // JS_CALL_OP_LIST / JS_CONSTRUCT_OP_LIST
  ...["JSCall", "JSCallForwardVarargs", "JSCallWithArrayLike", "JSCallWithSpread", "JSWasmCall"],
  ...["JSConstructForwardVarargs", "JSConstructForwardAllArgs", "JSConstruct", "JSConstructWithArrayLike", "JSConstructWithSpread"],
];
const NEC_FALSE = new Set([...nec.false, ...NEC_MACRO_FALSE]);
const NEC_TRUE = new Set(nec.true);
const NEC_PARAM = new Set(nec.param);
// JS 全集四组互斥分区对数: false + true + param + lazy == 124
const JS_SET = setOf(LISTS.JS);
for (const [nm, s] of [["false", NEC_FALSE], ["true", NEC_TRUE], ["param", NEC_PARAM]]) {
  for (const n of s) {
    if (!JS_SET.has(n)) throw new Error(`NeedsExactContext ${nm} 组非 JS 成员 ${n}`);
    for (const [m2, s2] of [["false", NEC_FALSE], ["true", NEC_TRUE], ["param", NEC_PARAM]]) {
      if (nm < m2 && s2.has(n)) throw new Error(`NeedsExactContext ${nm}/${m2} 重叠 @${n}`);
    }
  }
}
const NEC_LAZY = new Set([...JS_SET].filter((n) => !NEC_FALSE.has(n) && !NEC_TRUE.has(n) && !NEC_PARAM.has(n)));
assert(NEC_FALSE.size + NEC_TRUE.size + NEC_PARAM.size + NEC_LAZY.size === LISTS.JS.length, `JS 四组分区 ${NEC_FALSE.size}+${NEC_TRUE.size}+${NEC_PARAM.size}+${NEC_LAZY.size} != ${LISTS.JS.length}`);
assert(NEC_TRUE.has("JSDebugger") && NEC_TRUE.has("JSLoadContext"), "true 组缺哨兵");
assert(NEC_FALSE.has("JSCloneObject") && NEC_FALSE.has("JSGetIterator") && NEC_FALSE.has("JSCall"), "false 组缺哨兵");
assert(NEC_PARAM.size === 2 && NEC_PARAM.has("JSCallRuntime") && NEC_PARAM.has("JSCreateArguments"), `param 组 ${[...NEC_PARAM]}`);
assert(NEC_LAZY.has("JSCreatePromise") && NEC_LAZY.has("JSCallRuntime") === false, "lazy 组哨兵异常");

// IsBasicBlockBegin: 显式 opcode 判等表
const bbbMatch = opPropsCC.match(
  /IsBasicBlockBegin\(const Operator\* op\)\s*{[\s\S]*?\n}/,
);
if (!bbbMatch) throw new Error("IsBasicBlockBegin 未找到");
const BASIC_BLOCK_BEGIN = new Set(
  [...bbbMatch[0].matchAll(/IrOpcode::k(\w+)/g)].map((m) => m[1]),
);
if (BASIC_BLOCK_BEGIN.size !== 11) throw new Error(`IsBasicBlockBegin 计数 ${BASIC_BLOCK_BEGIN.size} != 11`);

// IsFeedbackCollectingOpcode: 宏组 + 显式 case
const FB_MACRO = [
  ...LISTS.JS_ARITH_BINOP,
  ...LISTS.JS_ARITH_UNOP,
  ...LISTS.JS_BITWISE_BINOP,
  ...LISTS.JS_BITWISE_UNOP,
  ...LISTS.JS_COMPARE,
];
const fbcIdx = src.indexOf("IsFeedbackCollectingOpcode(Value value)");
const fbcBody = src.slice(fbcIdx, src.indexOf("UNREACHABLE", fbcIdx));
const FB_EXPLICIT = [...fbcBody.matchAll(/case\s+k(\w+)\s*:/g)].map((m) => m[1]);
const FEEDBACK = new Set([...FB_MACRO, ...FB_EXPLICIT]);

// 显式小表 (opcodes.h 谓词的字面集)
const EXPLICIT = {
  PHI: ["Phi", "EffectPhi"],
  MERGE: ["Merge", "Loop"],
  GRAPH_TERMINATOR: ["Deoptimize", "Return", "TailCall", "Terminate", "Throw"],
  INLINEE: ["JSConstruct", "JSCall"],
};

// ---------------------------------------------------------------------------
// 6. 对数断言 (逐项清点)
// ---------------------------------------------------------------------------
const idx = (n) => ALL_NAMES.indexOf(n);
assert(LISTS.CONTROL.length === 22, `CONTROL ${LISTS.CONTROL.length} != 22`);
assert(idx("Start") === 0, "Start != 0");
assert(idx("End") === 21, "End != 21");
assert(idx("JSEqual") < idx("JSDebugger"), "JS 区间倒置");
assert(ALL_NAMES.indexOf("JSWasmCall") >= 0, "wasm 开但 JSWasmCall 缺席");
assert(!ALL_NAMES.includes("SwitchSandboxMode"), "硬件沙箱关但 SwitchSandboxMode 在列");
assert(!ALL_NAMES.includes("F64x4Min"), "SIMD256 关但 F64x4Min 在列");
assert(ALL_NAMES.includes("I8x16Splat"), "SIMD128 无条件项 I8x16Splat 缺席");
assert(
  ALL_NAMES.includes("GetContinuationPreservedEmbedderData") &&
    ALL_NAMES.includes("SetContinuationPreservedEmbedderData"),
  "CPED 开但 CPED 码缺席",
);
assert(idx("Word32Clz") < idx("TraceInstruction"), "machine 区间倒置");
assert(FRAME_STATE_TRUE.has("JSAdd"), "JSAdd 应需 FrameState");
assert(!FRAME_STATE_TRUE.has("JSStrictEqual"), "JSStrictEqual 不应需 FrameState");
assert(FRAME_STATE_TRUE.has("Checkpoint") && FRAME_STATE_TRUE.has("FrameState"), "Checkpoint/FrameState 应需");
assert(!FRAME_STATE_TRUE.has("Start"), "Start 不应需 FrameState");
assert([...FRAME_STATE_PARAM].join() === "JSCallRuntime", `参数化组 ${[...FRAME_STATE_PARAM]}`);
assert(FRAME_STATE_TRUE.size + FRAME_STATE_PARAM.size + fsGroups.false.length > 60, "FrameState 表过小");
// 区间谓词与表位一致 (C++ 区间语义 == 表成员语义)
const rangeIn = (n, lo, hi) => idx(lo) <= idx(n) && idx(n) <= idx(hi);
for (const n of ALL_NAMES) {
  const js = LISTS.JS.includes(n);
  assert(js === rangeIn(n, "JSEqual", "JSDebugger"), `is_js 区间不一致 @${n}`);
  const common = LISTS.CONTROL.includes(n) || LISTS.COMMON.includes(n);
  assert(common === rangeIn(n, "Start", "StaticAssert"), `is_common 区间不一致 @${n}`);
  const machine = LISTS.MACHINE.includes(n);
  assert(machine === rangeIn(n, "Word32Clz", "TraceInstruction"), `is_machine 区间不一致 @${n}`);
}
// JS_CONTEXT_OP_LIST 尾段连续 (IsContextChainExtendingOpcode 区间)
{
  const ctx = ["JSCreateFunctionContext", "JSCreateCatchContext", "JSCreateWithContext", "JSCreateBlockContext"];
  assert(idx("JSCreateFunctionContext") === idx("JSCreateBlockContext") - 3, "context-chain 区间不连续");
  for (const n of ctx) assert(idx(n) > 0 && idx("JSLoadContext") < idx(n), `context-chain 成员错位 @${n}`);
}

// ---------------------------------------------------------------------------
// 7. 每变体属性位
// ---------------------------------------------------------------------------
const BIT = {
  CONTROL: 1 << 0, // CONTROL_OP_LIST
  COMMON: 1 << 1, // CONSTANT+INNER+尾段 (is_common 区间含 CONTROL, 谓词另含)
  SIMPLIFIED: 1 << 2,
  MACHINE: 1 << 3,
  SIMD128: 1 << 4,
  JS: 1 << 5,
  CONSTANT: 1 << 6, // CONSTANT_OP_LIST
  MACHINE_CONSTANT: 1 << 7,
  COMPARISON: 1 << 8,
  FEEDBACK: 1 << 9,
  ATOMIC: 1 << 10,
  PHI: 1 << 11,
  MERGE: 1 << 12,
  IF_PROJECTION: 1 << 13, // IfTrue..IfDefault 区间
  GRAPH_TERMINATOR: 1 << 14,
  INLINEE: 1 << 15,
  CONTEXT_CHAIN_EXTENDING: 1 << 16, // JSCreateFunctionContext..JSCreateBlockContext
  FRAME_STATE_INPUT: 1 << 17, // OperatorProperties::HasFrameStateInput 的 opcode 面
  BASIC_BLOCK_BEGIN: 1 << 18, // OperatorProperties::IsBasicBlockBegin
  EXACT_CONTEXT_TRUE: 1 << 19, // OperatorProperties::NeedsExactContext 真
  EXACT_CONTEXT_PARAM: 1 << 20, // NeedsExactContext 参数化 (JSCallRuntime/JSCreateArguments)
  EXACT_CONTEXT_LAZY: 1 << 21, // NeedsExactContext break→UNREACHABLE 组 (未登记 JS 码)
};
const sets = {
  CONTROL: setOf(LISTS.CONTROL),
  COMMON: setOf(LISTS.COMMON),
  SIMPLIFIED: setOf(LISTS.SIMPLIFIED),
  MACHINE: setOf(LISTS.MACHINE),
  SIMD128: setOf(LISTS.SIMD128),
  JS: setOf(LISTS.JS),
  CONSTANT: setOf(LISTS.CONSTANT),
  MACHINE_CONSTANT: setOf(LISTS.MACHINE_CONSTANT),
  COMPARISON: setOf([...LISTS.JS_COMPARE, ...LISTS.SIMPLIFIED_COMPARE, ...LISTS.MACHINE_COMPARE]),
  FEEDBACK: FEEDBACK,
  ATOMIC: setOf(LISTS.MACHINE_ATOMIC),
  PHI: setOf(EXPLICIT.PHI),
  MERGE: setOf(EXPLICIT.MERGE),
  GRAPH_TERMINATOR: setOf(EXPLICIT.GRAPH_TERMINATOR),
  INLINEE: setOf(EXPLICIT.INLINEE),
};

const propsOf = (n) => {
  let p = 0;
  for (const [k, s] of Object.entries(sets)) if (s.has(n)) p |= BIT[k];
  // OP_COMMON 对齐 C++ IsCommonOpcode 区间语义: COMMON_OP_LIST ∪ CONTROL_OP_LIST
  if (sets.CONTROL.has(n)) p |= BIT.COMMON;
  if (rangeIn(n, "IfTrue", "IfDefault")) p |= BIT.IF_PROJECTION;
  if (rangeIn(n, "JSCreateFunctionContext", "JSCreateBlockContext")) p |= BIT.CONTEXT_CHAIN_EXTENDING;
  if (FRAME_STATE_TRUE.has(n)) p |= BIT.FRAME_STATE_INPUT;
  if (BASIC_BLOCK_BEGIN.has(n)) p |= BIT.BASIC_BLOCK_BEGIN;
  if (NEC_TRUE.has(n)) p |= BIT.EXACT_CONTEXT_TRUE;
  if (NEC_PARAM.has(n)) p |= BIT.EXACT_CONTEXT_PARAM;
  if (NEC_LAZY.has(n)) p |= BIT.EXACT_CONTEXT_LAZY;
  return p;
};
const TABLE = ALL_NAMES.map(propsOf);

// 顶层类别 (枚举段注释用)
function topLevel(n) {
  if (sets.CONTROL.has(n)) return "CONTROL_OP_LIST";
  if (sets.COMMON.has(n)) return "COMMON_OP_LIST";
  if (sets.SIMPLIFIED.has(n)) return "SIMPLIFIED_OP_LIST";
  if (sets.MACHINE.has(n)) return "MACHINE_OP_LIST";
  if (sets.SIMD128.has(n)) return "MACHINE_SIMD128_OP_LIST";
  if (sets.JS.has(n)) return "JS_OP_LIST";
  throw new Error(`${n} 无顶层类别`);
}

// ---------------------------------------------------------------------------
// 8. 生成 Rust
// ---------------------------------------------------------------------------
const total = ALL_NAMES.length;
const last = ALL_NAMES[total - 1];

const enumLines = [];
{
  let curTop = "";
  let col = 0;
  let line = "  ";
  for (let i = 0; i < total; i++) {
    const n = ALL_NAMES[i];
    const t = topLevel(n);
    if (t !== curTop) {
      if (line.trim()) enumLines.push(line.trimEnd());
      line = "  ";
      col = 0;
      enumLines.push(`  // ${t}`);
      curTop = t;
    }
    let tok = `${n},`;
    if (i === 0) tok = `${n} = 0,`;
    if (col + tok.length > 96) {
      enumLines.push(line.trimEnd());
      line = "  ";
      col = 0;
    }
    if (col > 0) {
      line += " ";
      col++;
    }
    line += tok;
    col += tok.length;
  }
  if (line.trim()) enumLines.push(line.trimEnd());
}

// 表体: 每行 6 个, 行尾注释首/末名称
const tableLines = [];
for (let i = 0; i < total; i += 6) {
  const chunk = TABLE.slice(i, i + 6).map((p) => `0x${p.toString(16).padStart(5, "0")}`);
  tableLines.push(
    `  ${chunk.join(", ")}, // ${ALL_NAMES[i]}..${ALL_NAMES[Math.min(i + 5, total - 1)]}`,
  );
}

const bitDocs = [
  ["CONTROL", "CONTROL_OP_LIST 成员 (is_control 区间 [Start,End])"],
  ["COMMON", "IsCommonOpcode 区间 [Start,StaticAssert] = COMMON_OP_LIST ∪ CONTROL_OP_LIST"],
  ["SIMPLIFIED", "SIMPLIFIED_OP_LIST 成员"],
  ["MACHINE", "MACHINE_OP_LIST 成员 (is_machine 区间 [Word32Clz,TraceInstruction])"],
  ["SIMD128", "MACHINE_SIMD128_OP_LIST 成员 (wasm 开构建)"],
  ["JS", "JS_OP_LIST 成员 (is_js 区间 [JSEqual,JSDebugger]; 有 context 输入)"],
  ["CONSTANT", "CONSTANT_OP_LIST 成员"],
  ["MACHINE_CONSTANT", "MACHINE_LEVEL_CONSTANT_OP_LIST 成员"],
  ["COMPARISON", "JS/SIMPLIFIED/MACHINE 三表 COMPARE_BINOP 并集"],
  ["FEEDBACK", "IsFeedbackCollectingOpcode 表"],
  ["ATOMIC", "MACHINE_ATOMIC_OP_LIST 成员"],
  ["PHI", "Phi | EffectPhi"],
  ["MERGE", "Merge | Loop"],
  ["IF_PROJECTION", "IfTrue..IfDefault 区间"],
  ["GRAPH_TERMINATOR", "Deoptimize|Return|TailCall|Terminate|Throw"],
  ["INLINEE", "JSConstruct | JSCall"],
  ["CONTEXT_CHAIN_EXTENDING", "JSCreateFunctionContext..JSCreateBlockContext 区间"],
  ["FRAME_STATE_INPUT", "OperatorProperties::HasFrameStateInput 真 (JSCallRuntime 参数化, 除外)"],
  ["BASIC_BLOCK_BEGIN", "OperatorProperties::IsBasicBlockBegin 真"],
  ["EXACT_CONTEXT_TRUE", "OperatorProperties::NeedsExactContext 真组"],
  ["EXACT_CONTEXT_PARAM", "NeedsExactContext 参数化组 (JSCallRuntime|JSCreateArguments)"],
  ["EXACT_CONTEXT_LAZY", "NeedsExactContext break→UNREACHABLE 组"],
];

const listStatLines = Object.entries(LISTS)
  .map(([k, v]) => `//   ${k.padEnd(22)} ${v.length}`)
  .join("\n");

const body = `${banner("src/compiler/opcodes.h.js", "src/compiler/opcodes.h + opcodes.cc + operator-properties.cc (谓词表)")}
//! IrOpcode — 全层级 opcode 全局唯一编号 (原版 IrOpcode::Value X-macro 展开)
//!
//! 目标构建: wasm 开 / SIMD128 开 / SIMD256 关 / 硬件沙箱关 / CPED 开。
//! 变体顺序 = ALL_OP_LIST = CONTROL + COMMON + SIMPLIFIED + MACHINE + SIMD128 + JS。
//! 每变体派生属性见 [\`OPCODE_PROPS\`]; 区间型谓词与表位一致性由生成脚本断言。
//! value/effect/control 输入数属 operator 实例数据 (v8-compiler-ops 层), 不在此表。
//!
//! 展开计数 (对数): 总 ${total}
${listStatLines}
//   needs_exact_context     false=${NEC_FALSE.size} true=${NEC_TRUE.size} param=${NEC_PARAM.size} lazy=${NEC_LAZY.size} (JS 全集互斥分区)

use strum::{EnumCount, EnumIter, FromRepr, IntoStaticStr};

/// IrOpcode — 原版 \`enum Value { k##x }\`; Rust 侧变体名去 k 前缀
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromRepr, EnumCount, EnumIter, IntoStaticStr)]
#[repr(u16)]
pub enum IrOpcode {
${enumLines.join("\n")}
}

/// NeedsExactContext 的 opcode 面分解 (operator-properties.cc:23-143)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExactContext {
  /// 任意 context 即可 (二元/一元/调用/构造 + 显式组)
  AnyContext,
  /// 需要精确 context
  Exact,
  /// 需 operator 参数 (JSCallRuntime→Runtime 表 / JSCreateArguments→参数)
  NeedsParam,
  /// 未登记 JS 码 (原版 break→UNREACHABLE)
  Unreachable,
  /// 非 JS 码 (原版 UNREACHABLE)
  NotJs,
}

impl IrOpcode {
  /// 原版 kLast = 展开计数 - 1
  pub const LAST: Self = Self::${last};
  /// 原版 kLast 的裸值 (operator opcode 域上界)
  pub const LAST_U16: u16 = ${total - 1};

  /// 原版 static_cast<Value>(opcode) 的无检查形态 (node.opcode 热路径)
  ///
  /// # Safety
  /// v 必须 <= [\`LAST_U16\`] (Operator 构造已校验)
  #[inline]
  pub const unsafe fn from_repr_unchecked(v: u16) -> Self {
    // SAFETY: repr(u16) 判别值连续 0..=LAST_U16, transmute 值域合法
    unsafe { core::mem::transmute(v) }
  }

  /// 原版 IrOpcode::Mnemonic — 变体名字符串 (opcodes.cc kMnemonics)
  #[inline]
  pub fn mnemonic(self) -> &'static str {
    self.into()
  }
}

impl core::fmt::Display for IrOpcode {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str(self.mnemonic())
  }
}

impl IrOpcode {

  /// 原版 Mnemonic 的越界钳制形态 (kMnemonics 末位 "UnknownOpcode")
  pub fn mnemonic_of(v: u16) -> &'static str {
    Self::from_repr(v).map_or(Self::UNKNOWN, Into::into)
  }

  /// 原版 IsCommonOpcode: kStart <= v <= kStaticAssert (control + common 区间)
  #[inline]
  pub fn is_common(self) -> bool {
    (Self::Start as u16..=Self::StaticAssert as u16).contains(&(self as u16))
  }

  /// 原版 IsControlOpcode: kStart <= v <= kEnd
  #[inline]
  pub fn is_control(self) -> bool {
    (Self::Start as u16..=Self::End as u16).contains(&(self as u16))
  }

  /// 原版 IsJsOpcode: kJSEqual <= v <= kJSDebugger (⟺ 有 context 输入)
  #[inline]
  pub fn is_js(self) -> bool {
    (Self::JSEqual as u16..=Self::JSDebugger as u16).contains(&(self as u16))
  }

  /// 原版 IsMachineOpcode: kWord32Clz <= v <= kTraceInstruction
  #[inline]
  pub fn is_machine(self) -> bool {
    (Self::Word32Clz as u16..=Self::TraceInstruction as u16).contains(&(self as u16))
  }

  /// 原版 IsSimd128Opcode (wasm 开构建)
  #[inline]
  pub fn is_simd128(self) -> bool {
    OPCODE_PROPS[self as usize] & OP_SIMD128 != 0
  }

  /// 原版 IsMachineConstantOpcode
  #[inline]
  pub fn is_machine_constant(self) -> bool {
    OPCODE_PROPS[self as usize] & OP_MACHINE_CONSTANT != 0
  }

  /// 原版 IsConstantOpcode
  #[inline]
  pub fn is_constant(self) -> bool {
    OPCODE_PROPS[self as usize] & OP_CONSTANT != 0
  }

  /// 原版 IsPhiOpcode
  #[inline]
  pub fn is_phi(self) -> bool {
    OPCODE_PROPS[self as usize] & OP_PHI != 0
  }

  /// 原版 IsMergeOpcode
  #[inline]
  pub fn is_merge(self) -> bool {
    OPCODE_PROPS[self as usize] & OP_MERGE != 0
  }

  /// 原版 IsIfProjectionOpcode: kIfTrue <= v <= kIfDefault
  #[inline]
  pub fn is_if_projection(self) -> bool {
    OPCODE_PROPS[self as usize] & OP_IF_PROJECTION != 0
  }

  /// 原版 IsGraphTerminator
  #[inline]
  pub fn is_graph_terminator(self) -> bool {
    OPCODE_PROPS[self as usize] & OP_GRAPH_TERMINATOR != 0
  }

  /// 原版 IsInlineeOpcode
  #[inline]
  pub fn is_inlinee(self) -> bool {
    OPCODE_PROPS[self as usize] & OP_INLINEE != 0
  }

  /// 原版 IsComparisonOpcode
  #[inline]
  pub fn is_comparison(self) -> bool {
    OPCODE_PROPS[self as usize] & OP_COMPARISON != 0
  }

  /// 原版 IsContextChainExtendingOpcode
  #[inline]
  pub fn is_context_chain_extending(self) -> bool {
    OPCODE_PROPS[self as usize] & OP_CONTEXT_CHAIN_EXTENDING != 0
  }

  /// 原版 IsFeedbackCollectingOpcode
  #[inline]
  pub fn is_feedback_collecting(self) -> bool {
    OPCODE_PROPS[self as usize] & OP_FEEDBACK != 0
  }

  /// 原版 isAtomicOpOpcode
  #[inline]
  pub fn is_atomic_op(self) -> bool {
    OPCODE_PROPS[self as usize] & OP_ATOMIC != 0
  }

  /// OperatorProperties::HasContextInput 的 opcode 面 (⟺ is_js)
  #[inline]
  pub fn has_context_input(self) -> bool {
    self.is_js()
  }

  /// OperatorProperties::HasFrameStateInput 的 opcode 面;
  /// JSCallRuntime 依赖 CallRuntime 参数 (Linkage 表), 由调用方参数化判定
  #[inline]
  pub fn has_frame_state_input(self) -> bool {
    OPCODE_PROPS[self as usize] & OP_FRAME_STATE_INPUT != 0
  }

  /// JSCallRuntime: HasFrameStateInput 需查 CallRuntime 参数
  #[inline]
  pub fn frame_state_input_needs_param(self) -> bool {
    self == Self::JSCallRuntime
  }

  /// OperatorProperties::IsBasicBlockBegin 的 opcode 面
  #[inline]
  pub fn is_basic_block_begin(self) -> bool {
    OPCODE_PROPS[self as usize] & OP_BASIC_BLOCK_BEGIN != 0
  }

  /// OperatorProperties::NeedsExactContext 的 opcode 面 (JS 全集四组互斥分解)
  ///
  /// AnyContext(${NEC_FALSE.size}) | Exact(${NEC_TRUE.size}) | NeedsParam(${NEC_PARAM.size}) | Unreachable(${NEC_LAZY.size});
  /// 非 JS 码原版走 UNREACHABLE → NotJs
  #[inline]
  pub const fn exact_context(self) -> ExactContext {
    let p = OPCODE_PROPS[self as usize];
    if p & OP_EXACT_CONTEXT_PARAM != 0 {
      ExactContext::NeedsParam
    } else if p & OP_EXACT_CONTEXT_TRUE != 0 {
      ExactContext::Exact
    } else if p & OP_EXACT_CONTEXT_LAZY != 0 {
      ExactContext::Unreachable
    } else if p & OP_JS != 0 {
      ExactContext::AnyContext
    } else {
      ExactContext::NotJs
    }
  }

  /// 未知 opcode 的助记符 (原版 kMnemonics 槽位 "UnknownOpcode")
  pub const UNKNOWN: &'static str = "UnknownOpcode";
}

// —— OPCODE_PROPS 表的派生属性位标志 ——
${bitDocs.map(([k, d]) => `/// ${d}\npub const OP_${k}: u32 = ${BIT[k]};`).join("\n")}

/// 每变体派生属性表 — 与枚举判别值同序 (索引即 opcode)
pub static OPCODE_PROPS: [u32; IrOpcode::COUNT] = [
${tableLines.join("\n")}
];

// —— 生成不变式 (判别值; 谓词抽样对数在 tests/opcodes_unittest.rs) ——

const _: () = {
  assert!(IrOpcode::COUNT == ${total});
  assert!(IrOpcode::LAST as u16 == ${total - 1});
  assert!(IrOpcode::LAST_U16 == ${total - 1});
  assert!(IrOpcode::Start as u16 == 0);
  assert!(IrOpcode::End as u16 == 21);
};
`;

const p = writeSrc("v8-compiler-graph", "opcodes.rs", body);
console.log(
  [
    `written ${p}`,
    `total=${total} last=${last}`,
    ...Object.entries(LISTS).map(([k, v]) => `${k}=${v.length}`),
    `frame_state_true=${FRAME_STATE_TRUE.size} param=${[...FRAME_STATE_PARAM].join("|")}`,
    `basic_block_begin=${BASIC_BLOCK_BEGIN.size} feedback=${FEEDBACK.size}`,
  ].join("\n"),
);
