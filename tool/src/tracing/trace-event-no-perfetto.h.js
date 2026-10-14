#!/usr/bin/env bun
// v8/src/tracing/trace-event-no-perfetto.h → v8-tracing/src/trace_event_macros.rs
//
// 本文件是 trace-event 宏体系的 X-macro 展开器 (todo.md 十一章: 脚本即宏展开器, 逐个登记):
//   1. 机械提取 4 组底层常量: TRACE_EVENT_PHASE_* (23) / TRACE_EVENT_FLAG_* (15+MASK) /
//      TRACE_VALUE_TYPE_* (9) / TRACE_EVENT_SCOPE_* (3) + SCOPE_NAME_* (3)
//   2. 解析全部 #define 用户面宏 (~127 个), 按 INTERNAL_TRACE_EVENT_* 目标族分类,
//      从宏体逐参提取 phase/flags/固定实参("value"/"step"/"sync_id"/...),
//      展开为显式 Rust 函数 — 每个函数即一条宏登记 (doc 注明原宏名与行号)
//   3. 不可展开项 (TRACE_EVENT_METADATA1 依赖的 INTERNAL_TRACE_EVENT_METADATA_ADD 上游未定义 /
//      typed stub INTERNAL_TRACE_IGNORE 族) 只登记不生成, 汇总打印
// 产物为基线, 语义修订就地改; 复跑覆盖 (仅限重生成基线)。
import { readV8, writeSrc, banner } from "../../lib.js";

const CRATE = "v8-tracing";
const GEN = "src/tracing/trace-event-no-perfetto.h.js";
const SRC = readV8("src/tracing/trace-event-no-perfetto.h");

// ---------- 解析: 按续行聚合 #define 块 ----------
function parseDefines(src) {
  const out = [];
  const ls = src.split("\n");
  for (let i = 0; i < ls.length; i++) {
    // 函数式宏: '(' 紧跟宏名 (无空格)
    const m = ls[i].match(/^#define\s+([A-Z][A-Z0-9_]*)\((.+)?\)?/);
    if (!m || !/^[A-Z]/.test(m[1])) continue;
    let text = ls[i];
    let end = i;
    while (ls[end].trimEnd().endsWith("\\")) {
      end++;
      text += "\n" + ls[end];
    }
    out.push({
      name: m[1],
      line: i + 1,
      flat: text.replace(/\\\n/g, " ").replace(/\/\*.*?\*\//g, " ").replace(/\s+/g, " ").trim(),
    });
    i = end;
  }
  return out;
}

const defines = parseDefines(SRC);

// ---------- 提取: 底层常量 ----------
const phases = [...SRC.matchAll(/#define TRACE_EVENT_PHASE_(\w+)\s+\('(.)'\)/g)].map((m) => [m[1], m[2]]);
const flags = [...SRC.matchAll(
  /#define TRACE_EVENT_FLAG_(\w+)[\s\\]*\(static_cast<unsigned int>\(([^)]+)\)\)/g,
)].map((m) => [m[1], m[2]]);
const valueTypes = [...SRC.matchAll(
  /#define TRACE_VALUE_TYPE_(\w+)\s+\(static_cast<unsigned char>\(([^)]+)\)\)/g,
)].map((m) => [m[1], m[2]]);
const scopes = [...SRC.matchAll(
  /#define TRACE_EVENT_SCOPE_(\w+)\s+\(static_cast<unsigned char>\(([^)]+)\)\)/g,
)].map((m) => [m[1], m[2]]);
const scopeNames = [...SRC.matchAll(/#define TRACE_EVENT_SCOPE_NAME_(\w+)\s+\('(.)'\)/g)].map((m) => [m[1], m[2]]);

if (phases.length !== 23) throw new Error(`phase 常量数 ${phases.length} != 23`);
// 16 = NONE + 13 位标志 + JAVA_STRING_LITERALS + SCOPE_MASK (复合)
if (flags.length !== 16) throw new Error(`flag 常量数 ${flags.length} != 16`);
if (valueTypes.length !== 9) throw new Error(`value_type 常量数 ${valueTypes.length} != 9`);
if (scopes.length !== 3 || scopeNames.length !== 3) throw new Error("scope 常量数错误");

const cExpr = (e) => e.replace(/\\\s*/g, "").replace(/\s+/g, ""); // 去续行符 + 压空白

// ---------- 分类: INTERNAL 目标族 ----------
const FAMILY = [
  ["ADD_SCOPED_WITH_FLOW", /INTERNAL_TRACE_EVENT_ADD_SCOPED_WITH_FLOW\(/],
  ["ADD_SCOPED", /INTERNAL_TRACE_EVENT_ADD_SCOPED\(/],
  ["ADD_WITH_ID_TID_AND_TIMESTAMP", /INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP\(/],
  ["ADD_WITH_ID", /INTERNAL_TRACE_EVENT_ADD_WITH_ID\(/],
  ["ADD_WITH_TIMESTAMP", /INTERNAL_TRACE_EVENT_ADD_WITH_TIMESTAMP\(/],
  ["ADD", /INTERNAL_TRACE_EVENT_ADD\(/],
];

// 顶层逗号切分 (尊重括号嵌套, 如 static_cast<int>(x))
function splitArgs(s) {
  const out = [];
  let depth = 0, cur = "";
  for (const ch of s) {
    if (ch === "(") depth++;
    if (ch === ")") depth--;
    if (ch === "," && depth === 0) { out.push(cur.trim()); cur = ""; } else cur += ch;
  }
  if (cur.trim()) out.push(cur.trim());
  return out;
}

// 取 INTERNAL 调用实参表: [目标宏名, 实参...]
function internalCall(def) {
  for (const [name, re] of FAMILY) {
    const m = def.flat.match(re);
    if (!m) continue;
    const start = m.index + m[0].length;
    // 找匹配的右括号
    let depth = 1, i = start;
    for (; i < def.flat.length && depth > 0; i++) {
      if (def.flat[i] === "(") depth++;
      if (def.flat[i] === ")") depth--;
    }
    return [name, ...splitArgs(def.flat.slice(start, i - 1))];
  }
  return null;
}

// 实参语义化: 字符串字面量去引号, static_cast<int>(x) → {int:x}, 其余原样标识符
function sem(a) {
  let m;
  if ((m = a.match(/^"((?:[^"\\]|\\.)*)"$/))) return { k: "lit", v: m[1] };
  if ((m = a.match(/^static_cast<int>\((\w+)\)$/))) return { k: "int", v: m[1] };
  return { k: "id", v: a };
}

// ---------- Rust 参数映射 ----------
// 返回 [rustTy, rustExpr] 或 null (跳过该参数)
function paramTy(p) {
  if (/^(category_group|category)$/.test(p)) return "&'static str";
  if (p === "name") return "&'static str";
  if (p === "scope") return "u8";
  if (/^(scope_and_flags|flags|flag|flow_flags)$/.test(p)) return "u32";
  if (/^(timestamp|issue_ts|issue_end_ts)$/.test(p)) return "i64";
  if (/^(id|bind_id)$/.test(p)) return "impl Into<TraceId>";
  if (/^(value|value1_val|value2_val)$/.test(p)) return "i32"; // static_cast<int>
  if (/_name$/.test(p)) return "&'static str";
  return "impl Into<TraceValue>"; // arg*_val / step / snapshot / sync_id
}

const snake = (s) => s.toLowerCase();

// ---------- 生成宏展开函数 ----------
const fns = [];
const deferred = [];
const typedStubs = [];
const stats = {};

for (const def of defines) {
  const { name, line, flat } = def;
  if (name.startsWith("INTERNAL_")) continue; // 实现层宏, 非用户面登记
  // 形参
  const pm = flat.match(/^#define\s+[A-Z0-9_]+\(([^)]*)\)/);
  if (!pm) continue;
  const params = pm[1].split(",").map((s) => s.trim()).filter(Boolean);

  if (name === "TRACE_DISABLED_BY_DEFAULT") {
    fns.push({ name, line, params, kind: "prefix" });
    continue;
  }
  if (/^(TRACE_EVENT_PHASE|TRACE_EVENT_FLAG|TRACE_VALUE_TYPE|TRACE_EVENT_SCOPE)/.test(name)) continue; // 常量已单列

  if (/INTERNAL_TRACE_IGNORE/.test(flat)) {
    // 5 个 perfetto typed 宏 → 文件尾固定块展开为 no-op (见 typedStubBlock)
    typedStubs.push([name, line]);
    continue;
  }
  if (/INTERNAL_TRACE_EVENT_METADATA_ADD/.test(flat)) {
    deferred.push([name, line, "INTERNAL_TRACE_EVENT_METADATA_ADD 上游未定义, 无从展开"]);
    stats.metadata = (stats.metadata ?? 0) + 1;
    continue;
  }
  if (/TRACE_EVENT_API_GET_NUM_TRACES_RECORDED|lastRecordingNumber/.test(flat)) {
    deferred.push([name, line, "TRACE_EVENT_API_GET_NUM_TRACES_RECORDED = UNIMPLEMENTED() (trace-event.h:87)"]);
    stats.newtrace = (stats.newtrace ?? 0) + 1;
    continue;
  }

  // 时钟宏: TRACE_TIME_TICKS_NOW / TRACE_TIME_NOW → INTERNAL_TRACE_*_NOW (平台时钟)
  if (/INTERNAL_TRACE_TIME/.test(flat)) {
    fns.push({ name, line, params, kind: "clock" });
    continue;
  }

  const call = internalCall(def);
  if (!call) {
    // 类别启用查询族: 直接展开表达式, 无 INTERNAL 目标
    if (name === "TRACE_EVENT_CATEGORY_ENABLED") { fns.push({ name, line, params, kind: "enabled" }); continue; }
    if (name === "TRACE_EVENT_CATEGORY_GROUP_ENABLED") { fns.push({ name, line, params, kind: "enabled" }); continue; }
    deferred.push([name, line, "无 INTERNAL 目标, 未识别"]);
    continue;
  }
  const target = call[0];
  const as = call.slice(1).map(sem);

  // 按族定位字段
  let phase = null, flags = null, id = null, bindId = null, ts = null, rest = [];
  switch (target) {
    case "ADD":
      [phase, , , flags, ...rest] = as;
      break;
    case "ADD_SCOPED":
      rest = as.slice(2);
      break;
    case "ADD_SCOPED_WITH_FLOW":
      [, , bindId, flags, ...rest] = as;
      break;
    case "ADD_WITH_ID":
      [phase, , , id, flags, ...rest] = as;
      break;
    case "ADD_WITH_TIMESTAMP":
      [phase, , , ts, flags, ...rest] = as;
      break;
    case "ADD_WITH_ID_TID_AND_TIMESTAMP":
      [phase, , , id, , ts, flags, ...rest] = as; // thread_id 丢弃 (原版重定向注释)
      break;
  }
  if (phase && phase.k === "id") phase = { k: "phase", v: phase.v };
  if (flags && flags.k === "lit") throw new Error(`${name}: flags 误判为字面量`);

  fns.push({ name, line, params, kind: target, phase, flags, id, bindId, ts, rest });
}

// ---------- flags 表达式翻译 ----------
function flagsExpr(f) {
  if (!f) return "TRACE_EVENT_FLAG_NONE";
  const e = f.v.replace(/\s+/g, "");
  if (e === "TRACE_EVENT_FLAG_NONE") return "TRACE_EVENT_FLAG_NONE";
  if (e === "TRACE_EVENT_FLAG_COPY") return "TRACE_EVENT_FLAG_COPY";
  if (e === "TRACE_EVENT_FLAG_NONE|scope") return "TRACE_EVENT_FLAG_NONE | u32::from(scope)";
  if (e === "TRACE_EVENT_FLAG_COPY|scope") return "TRACE_EVENT_FLAG_COPY | u32::from(scope)";
  if (e === "TRACE_EVENT_FLAG_ASYNC_TTS|TRACE_EVENT_FLAG_COPY") return "TRACE_EVENT_FLAG_ASYNC_TTS | TRACE_EVENT_FLAG_COPY";
  if (/^(flags|flag|flow_flags|scope_and_flags)$/.test(e)) return e;
  throw new Error(`${e}: 未知 flags 表达式`);
}

// ---------- 参数重排: Rust 参数序 = 宏形参序, 但 tid 丢弃 ----------
function rustParams(fn) {
  const ps = [];
  for (let i = 0; i < fn.params.length; i++) {
    const p = fn.params[i];
    if (p === "thread_id") continue; // 原版 ADD_WITH_ID_TID_AND_TIMESTAMP 显式弃用
    const m = /^arg(\d+)_name$/.exec(p);
    if (m && fn.params[i + 1] === `arg${m[1]}_val`) {
      // arg 对打包: (name, val) 二元组, 消解原版 8 形参宏的 too_many_arguments
      ps.push(`arg${m[1]}: (&'static str, impl Into<TraceValue>)`);
      i++;
      continue;
    }
    ps.push(`${snake(p)}: ${paramTy(p)}`);
  }
  return ps;
}

// ---------- 函数体 ----------
function argList(fn) {
  // rest 成对 (argname, argval) → TraceEventArg 数组字面量
  const items = [];
  for (let i = 0; i + 1 < fn.rest.length; i += 2) {
    const [n, v] = [fn.rest[i], fn.rest[i + 1]];
    const pair = /^arg(\d+)_name$/.exec(n.k === "lit" ? "" : n.v);
    if (pair) items.push(`TraceEventArg::new(arg${pair[1]}.0, arg${pair[1]}.1)`);
    else if (n.k === "lit") items.push(`TraceEventArg::new("${n.v}", ${snake(v.v)})`);
    else items.push(`TraceEventArg::new(${snake(n.v)}, ${snake(v.v)})`);
  }
  return items;
}

function genFn(fn) {
  const doc = `/// \`${fn.name}(${fn.params.join(", ")})\` 展开 (trace-event-no-perfetto.h:${fn.line})。`;
  const params = rustParams(fn);
  let body;
  switch (fn.kind) {
    case "prefix":
      return `${doc}\npub fn trace_disabled_by_default(name: &str) -> String {\n  format!("{DISABLED_BY_DEFAULT_PREFIX}{name}")\n}\n`;
    case "clock":
      if (fn.name === "TRACE_TIME_TICKS_NOW")
        body = "ticks_now_ns()";
      else body = "time_now_ns()";
      return `${doc}\npub fn ${snake(fn.name)}() -> i64 {\n  ${body}\n}\n`;
    case "enabled":
      return `${doc}\npub fn ${snake(fn.name)}(category_group: &'static str) -> bool {\n  category_enabled(category_group)\n}\n`;
    case "ADD_SCOPED":
      return `${doc}\npub fn ${snake(fn.name)}(${params.join(", ")}) -> ScopedTracer {\n  scoped_add(category_group, name, vec![${argList(fn).join(", ")}])\n}\n`;
    case "ADD_SCOPED_WITH_FLOW":
      return `${doc}\npub fn ${snake(fn.name)}(${params.join(", ")}) -> ScopedTracer {\n  scoped_add_with_flow(category_group, name, bind_id.into(), flow_flags, vec![${argList(fn).join(", ")}])\n}\n`;
    default:
      break;
  }
  if (fn.kind === "ADD" || fn.kind === "ADD_WITH_ID" || fn.kind === "ADD_WITH_TIMESTAMP" || fn.kind === "ADD_WITH_ID_TID_AND_TIMESTAMP") {
    const args = `vec![${argList(fn).join(", ")}]`;
    const ph = phaseConst(fn.phase.v);
    const fl = fn.flags ? flagsExpr(fn.flags) : "TRACE_EVENT_FLAG_NONE";
    if (fn.kind === "ADD") body = `emit(${ph}, category_group, name, ${fl}, ${args})`;
    else if (fn.kind === "ADD_WITH_ID") body = `emit_id(${ph}, category_group, name, id.into(), ${fl}, ${args})`;
    else if (fn.kind === "ADD_WITH_TIMESTAMP") body = `emit_ts(${ph}, category_group, name, timestamp, ${fl}, ${args})`;
    else body = `emit_id_ts(${ph}, category_group, name, id.into(), timestamp, ${fl}, ${args})`;
    return `${doc}\npub fn ${snake(fn.name)}(${params.join(", ")}) {\n  ${body};\n}\n`;
  }
  return `${doc}\npub fn ${snake(fn.name)}(${params.join(", ")}) -> ScopedTracer {\n  ${body}\n}\n`;
}

const PHASE_CONST = {
  INSTANT: "TRACE_EVENT_PHASE_INSTANT", BEGIN: "TRACE_EVENT_PHASE_BEGIN", END: "TRACE_EVENT_PHASE_END",
  MARK: "TRACE_EVENT_PHASE_MARK", COUNTER: "TRACE_EVENT_PHASE_COUNTER", SAMPLE: "TRACE_EVENT_PHASE_SAMPLE",
  ASYNC_BEGIN: "TRACE_EVENT_PHASE_ASYNC_BEGIN", ASYNC_END: "TRACE_EVENT_PHASE_ASYNC_END",
  ASYNC_STEP_INTO: "TRACE_EVENT_PHASE_ASYNC_STEP_INTO", ASYNC_STEP_PAST: "TRACE_EVENT_PHASE_ASYNC_STEP_PAST",
  NESTABLE_ASYNC_BEGIN: "TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN",
  NESTABLE_ASYNC_END: "TRACE_EVENT_PHASE_NESTABLE_ASYNC_END",
  NESTABLE_ASYNC_INSTANT: "TRACE_EVENT_PHASE_NESTABLE_ASYNC_INSTANT",
  CLOCK_SYNC: "TRACE_EVENT_PHASE_CLOCK_SYNC", CREATE_OBJECT: "TRACE_EVENT_PHASE_CREATE_OBJECT",
  SNAPSHOT_OBJECT: "TRACE_EVENT_PHASE_SNAPSHOT_OBJECT", DELETE_OBJECT: "TRACE_EVENT_PHASE_DELETE_OBJECT",
};
function phaseConst(v) {
  if (v.startsWith("TRACE_EVENT_PHASE_")) return v; // 已是完整常量名
  if (!PHASE_CONST[v]) throw new Error(`未知 phase ${v}`);
  return PHASE_CONST[v];
}

// 特例: CLOCK_SYNC_RECEIVER/ISSUER 的固定 category/name ("__metadata"/"clock_sync")
function genClockSync(fn) {
  if (fn.name === "TRACE_EVENT_CLOCK_SYNC_RECEIVER")
    return `/// \`TRACE_EVENT_CLOCK_SYNC_RECEIVER(sync_id)\` 展开 (trace-event-no-perfetto.h:${fn.line})。\npub fn trace_event_clock_sync_receiver(sync_id: impl Into<TraceValue>) {\n  emit(TRACE_EVENT_PHASE_CLOCK_SYNC, "__metadata", "clock_sync", TRACE_EVENT_FLAG_NONE, vec![TraceEventArg::new("sync_id", sync_id)]);\n}\n`;
  return `/// \`TRACE_EVENT_CLOCK_SYNC_ISSUER(sync_id, issue_ts, issue_end_ts)\` 展开 (trace-event-no-perfetto.h:${fn.line})。\npub fn trace_event_clock_sync_issuer(sync_id: impl Into<TraceValue>, issue_ts: i64, issue_end_ts: i64) {\n  emit_ts(TRACE_EVENT_PHASE_CLOCK_SYNC, "__metadata", "clock_sync", issue_end_ts, TRACE_EVENT_FLAG_NONE, vec![TraceEventArg::new("sync_id", sync_id), TraceEventArg::new("issue_ts", issue_ts)]);\n}\n`;
}

// ---------- 组装 ----------
const constBlock = (title, items, ty, fmt) =>
  `${title}\n${items.map(([n, v]) => `pub const ${n}: ${ty} = ${fmt(v)};`).join("\n")}\n`;

let out = `${banner(GEN, "src/tracing/trace-event-no-perfetto.h", "X-macro 展开: TRACE_EVENT* 宏族 → 显式 Rust 函数, 逐个登记")}
//! trace-event 宏体系展开登记表 (trace-event-no-perfetto.h, 982 行)。
//!
//! 原版宏 \`TRACE_EVENT*\` 经 \`INTERNAL_TRACE_EVENT_*\` 两层转发到
//! \`v8::internal::tracing::AddTraceEvent[WithTimestamp]\` (trace-event.h);
//! 本模块把每个宏机械展开为同名 snake_case 函数, 参数/phase/flags 1:1。
//! 展开规则 (trace-event.h:181-297):
//! - \`ADD\`: 类别启用检查 → \`emit(phase, ...)\`
//! - \`ADD_WITH_ID\`: \`flags |= HAS_ID\` + TraceID → \`emit_id\`
//! - \`ADD_WITH_TIMESTAMP\` / \`ADD_WITH_ID_TID_AND_TIMESTAMP\`: 同上加 timestamp (tid 丢弃, 原版重定向)
//! - \`ADD_SCOPED\`: COMPLETE 相位 + \`ScopedTracer\` (drop 时补 end 时长)
//! - 计数器 value 一律 \`static_cast<int>\` (i32 截断, 原版行为)

use crate::trace_event::{
  ScopedTracer, TraceEventArg, TraceId, TraceValue, category_enabled, emit, emit_id, emit_id_ts, emit_ts,
  scoped_add, scoped_add_with_flow, ticks_now_ns, time_now_ns,
};

/// \`TRACE_DISABLED_BY_DEFAULT\` 前缀 ("disabled-by-default-")。
pub const DISABLED_BY_DEFAULT_PREFIX: &str = "disabled-by-default-";

${constBlock("// ---- TRACE_EVENT_PHASE_* (23) ----", phases.map(([n, c]) => [`TRACE_EVENT_PHASE_${n}`, c]), "u8", (c) => `b'${c}'`)}

${constBlock("// ---- TRACE_EVENT_FLAG_* 位标志 (14) ----", flags.filter(([n]) => n !== "NONE" && n !== "SCOPE_MASK").map(([n, e]) => [`TRACE_EVENT_FLAG_${n}`, e]), "u32", (e) => cExpr(e).replace("1<<", "1 << "))}

/// TRACE_EVENT_FLAG_NONE (unsigned int 字面 0)。
pub const TRACE_EVENT_FLAG_NONE: u32 = 0;

/// TRACE_EVENT_FLAG_SCOPE_MASK = SCOPE_OFFSET | SCOPE_EXTRA (复合, 原版逐字组合)。
pub const TRACE_EVENT_FLAG_SCOPE_MASK: u32 = TRACE_EVENT_FLAG_SCOPE_OFFSET | TRACE_EVENT_FLAG_SCOPE_EXTRA;

${constBlock("// ---- TRACE_VALUE_TYPE_* (9) ----", valueTypes.map(([n, e]) => [`TRACE_VALUE_TYPE_${n}`, e]), "u8", (e) => e)}

${constBlock("// ---- TRACE_EVENT_SCOPE_* (3, 须落在 SCOPE_MASK 内) ----", scopes.map(([n, e]) => [`TRACE_EVENT_SCOPE_${n}`, e]), "u8", (e) => cExpr(e).replace(/<</g, " << "))}

${constBlock("// ---- TRACE_EVENT_SCOPE_NAME_* (3) ----", scopeNames.map(([n, c]) => [`TRACE_EVENT_SCOPE_NAME_${n}`, c]), "u8", (c) => `b'${c}'`)}

`;

const byKind = {};
for (const fn of fns) {
  if (fn.name === "TRACE_EVENT_CLOCK_SYNC_RECEIVER" || fn.name === "TRACE_EVENT_CLOCK_SYNC_ISSUER") {
    out += genClockSync(fn) + "\n";
  } else {
    out += genFn(fn) + "\n";
  }
  byKind[fn.kind] = (byKind[fn.kind] ?? 0) + 1;
}

// ---- perfetto typed 宏族固定展开块 (trace-event-no-perfetto.h:26-32) ----
// 原版 5 个 typed 宏把实参整体擦除 → `trace_event_internal::Ignore(...)` (L14-18
// 空变参函数) — no-perfetto 构建下即 no-op。C 变参在 Rust 无匿名擦除形态,
// 机械展开为 (category[, name]) 两参 no-op, 其余实参由原版宏本体擦除。
const STUB_SIG = {
  TRACE_EVENT_BEGIN: ["category", "name"],
  TRACE_EVENT_END: ["category"],
  TRACE_EVENT: ["category", "name"],
  TRACE_EVENT_INSTANT: ["category", "name"],
  TRACE_COUNTER: ["category", "name"],
};
if (typedStubs.length !== 5) throw new Error(`typed stub ${typedStubs.length} != 5`);
out += `
// ---- perfetto typed 宏族展开 (trace-event-no-perfetto.h:26-32) ----
//
// 原版 5 个 typed 宏把实参整体擦除 → \`trace_event_internal::Ignore(...)\`
// (trace-event-no-perfetto.h:14-18, 空变参函数) — no-perfetto 构建下即 no-op。
// C 变参在 Rust 侧无可表达的匿名擦除形态, 机械展开为 (category, name) 两参
// no-op, 其余实参由原版宏本体擦除, 调用方 (L4+ 引擎代码) 适配时不传。
`;
for (const [name, line] of typedStubs) {
  const [a, b] = STUB_SIG[name];
  const params = b ? `${a}: &'static str, ${b}: &'static str` : `${a}: &'static str`;
  const body = b ? `(${a}, ${b})` : a;
  out += `/// \`${name}(...)\` 展开 (trace-event-no-perfetto.h:${line})。\n#[inline]\npub fn ${snake(name)}(${params}) {\n  let _ = ${body}; // Ignore(${a}${b ? `, ${b}` : ""})\n}\n\n`;
}

out += `// ---- 登记不展开 (2 项, 实测本 checkout 无可展开语义) ----
//
// 1. \`TRACE_EVENT_METADATA1(category_group, name, arg1_name, arg1_val)\`
//    (trace-event-no-perfetto.h:832-834) → \`INTERNAL_TRACE_EVENT_METADATA_ADD\`;
//    该 INTERNAL 宏在 include/ 与 src/ 全 checkout 无定义 (仅此一处使用),
//    no-perfetto 分支缺失, 被调用即编译失败 — 无可转写语义。
// 2. \`TRACE_EVENT_IS_NEW_TRACE(ret)\` (trace-event-no-perfetto.h:882-892) →
//    \`TRACE_EVENT_API_GET_NUM_TRACES_RECORDED\` = \`UNIMPLEMENTED()\`
//    (trace-event.h:86-87), no-perfetto 构建下调用即 abort — 无可转写语义。
//
// ---- perfetto stub 类型登记 (trace-event-no-perfetto.h:42-109) ----
//
// \`v8_stub::EventContext / StaticString / DynamicString / Track / ThreadTrack /
// NamedTrack / CounterTrack / Flow / TerminatingFlow\`: 全部方法体为空 (typed
// 宏的 C++ ABI 占位), 无可观察行为; typed 宏已展开为上述 no-op 函数,
// 空壳类型不生成 (零死代码), 此处登记备查。
`;

writeSrc(CRATE, "trace_event_macros.rs", out);

// ---------- 对数核对输出 ----------
console.log("== trace-event-no-perfetto.h.js 对数 ==");
console.log(`常量: phase=${phases.length} flag=${flags.length}(含 MASK) value_type=${valueTypes.length} scope=${scopes.length}+name=${scopeNames.length}`);
console.log(`宏展开函数: ${fns.length} + typed no-op ${typedStubs.length} = ${fns.length + typedStubs.length} 个`);
console.log(`登记不展开: ${deferred.length} 个`);
for (const [n, l, why] of deferred) console.log(`  - ${n} (L${l}): ${why}`);
console.log("复跑后需 cargo fmt -p v8-tracing 对齐格式");
