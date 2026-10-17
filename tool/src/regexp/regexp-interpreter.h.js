#!/usr/bin/env bun
// tool/src/regexp/regexp-interpreter.h.js — v8/src/regexp/regexp-interpreter.h → v8-regexp-exec/src/regexp_interpreter.rs
// 规则: 类面全集逐项 (77 行): include 守卫/AllStatic 基类/Result 枚举 5 值 (值以
//   regexp.h:150-155 实测对数交叉)/静态方法全集 4 个 (公有 3 + 私有 Match)/
//   前向声明/namespace 嵌套
// 产物: v8-regexp-exec/src/regexp_interpreter.rs.skel (签名骨架基线, 审计专用;
//   落地文件以就地修订为准) + 落地对数核对 (骨架票面: h 侧协议面须逐项命中)
// 推迟面 (cc 侧 handler 主体) 的对数由 regexp-interpreter.cc.js 承接
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { banner, readV8, root, writeSrc } from "../../lib.js";

const h = readV8("src/regexp/regexp-interpreter.h");

// ── 对数断言 (原版锚点 + 实测值; 任一不符 throw) ────────────────────────────
let assertCount = 0;
const fail = (label, got, want) => {
  throw new Error(`对数失配: ${label} got=${JSON.stringify(got)} want=${JSON.stringify(want)}`);
};
const count = (src, re) => (src.match(re) || []).length;

// 1) 文件结构全集 (77 行版式)
for (const [label, re, want] of [
  ["include 守卫三连 (ifndef/define/endif)", /V8_REGEXP_REGEXP_INTERPRETER_H_/g, 3],
  ["AllStatic 基类", /class V8_EXPORT_PRIVATE IrregexpInterpreter : public AllStatic/, 1],
  ["TrustedByteArray 前向声明", /^class TrustedByteArray;$/gm, 1],
  ["namespace v8", /^namespace v8 \{$/gm, 1],
  ["namespace internal", /^namespace internal \{$/gm, 1],
  ["namespace regexp", /^namespace regexp \{$/gm, 1],
  ["区段 public:", /public:/g, 1],
  ["区段 private:", /private:/g, 1],
] ) {
  if (count(h, re) !== want) fail(label, count(h, re), want);
  assertCount++;
}

// 2) Result 枚举 5 值 (h:21-27) 与 RegExp::kInternalRegExp* 名对 + regexp.h 实测值
const RESULT_TABLE = [
  ["FAILURE", "kInternalRegExpFailure"],
  ["SUCCESS", "kInternalRegExpSuccess"],
  ["EXCEPTION", "kInternalRegExpException"],
  ["RETRY", "kInternalRegExpRetry"],
  ["FALLBACK_TO_EXPERIMENTAL", "kInternalRegExpFallbackToExperimental"],
];
if (count(h, /static constexpr int kInternalRegExp\w+ = -?\d+;/g) !== 0)
  fail("h 内直接量应为 0 (全部经 RegExp:: 别名)", count(h, /static constexpr int kInternalRegExp\w+ = -?\d+;/g), 0);
assertCount++;
for (const [name, alias] of RESULT_TABLE) {
  if (!new RegExp(`${name} = RegExp::${alias},`).test(h)) fail(`Result.${name} 别名锚`, true, false);
  assertCount++;
}
// 值实测: regexp.h:150-154 (含 SmallestResult:155 = -3)
const regexpH = readV8("src/regexp/regexp.h");
const VALUE_TABLE = [
  ["kInternalRegExpFailure", 0],
  ["kInternalRegExpSuccess", 1],
  ["kInternalRegExpException", -1],
  ["kInternalRegExpRetry", -2],
  ["kInternalRegExpFallbackToExperimental", -3],
  ["kInternalRegExpSmallestResult", -3],
];
const actualValues = {};
for (const [k, want] of VALUE_TABLE) {
  const m = regexpH.match(new RegExp(`${k} = (-?\\d+);`));
  if (!m) fail(`regexp.h 常量缺失 ${k}`, true, false);
  if (Number(m[1]) !== want) fail(`regexp.h ${k} 实测值`, Number(m[1]), want);
  actualValues[k] = want;
  assertCount++;
}

// 3) 静态方法全集 4 (公有 h:31/51/58 + 私有 h:67)
const METHOD_TABLE = {
  MatchForCallFromRuntime: /static int MatchForCallFromRuntime\(Isolate\* isolate,/,
  MatchForCallFromJs: /static int MatchForCallFromJs\(Address subject, int32_t start_position,/,
  MatchInternal: /static Result MatchInternal\(Isolate\* isolate,/,
  Match: /static int Match\(Isolate\* isolate, Tagged<IrRegExpData> regexp_data,/,
};
for (const [name, re] of Object.entries(METHOD_TABLE)) {
  if (!re.test(h)) fail(`方法锚 ${name}`, true, false);
  assertCount++;
}
if (count(h, /static (int|Result) \w+\(/g) !== 4)
  fail("静态方法声明总数", count(h, /static (int|Result) \w+\(/g), 4);
assertCount++;
// 签名依赖: CallOrigin (regexp.h:111-114 实测 kFromRuntime=0/kFromJs=1)
if (count(h, /RegExp::CallOrigin/g) !== 3) fail("CallOrigin 形参出现数", count(h, /RegExp::CallOrigin/g), 3);
assertCount++;
const co = regexpH.match(/kFromRuntime = (\d+),\s*kFromJs = (\d+)/);
if (!co || co[1] !== "0" || co[2] !== "1") fail("CallOrigin 实测值", co && [co[1], co[2]], ["0", "1"]);
assertCount++;
// h:38-50 结果协议注释区 (EXCEPTION/RETRY/outputs 语义) 存在性
if (count(h, /RETRY is returned if a retry through the runtime/g) !== 1)
  fail("RETRY 协议注释锚", 0, 1);
assertCount++;

// ── 产物: 签名骨架 ─────────────────────────────────────────────────────────
const skel = `${banner(
  "src/regexp/regexp-interpreter.h.js",
  "src/regexp/regexp-interpreter.h",
  "签名骨架基线 (审计专用): 类面 77 行逐项 1:1; 落地以 regexp_interpreter.rs 就地修订为准",
)}
//! regexp-interpreter.h → regexp_interpreter.rs 处置表 (骨架票 REG-1b1):
//! - IrregexpInterpreter (AllStatic) → 模块级项 (类即命名空间)
//! - Result 枚举 5 值 (h:21-27) → pub enum Result {Failure=0,Success=1,Exception=-1,Retry=-2,FallbackToExperimental=-3}
//! - RegExp::CallOrigin (regexp.h:112-113) → pub enum CallOrigin (h 签名依赖, 双引用)
//! - MatchForCallFromRuntime (h:31) / MatchForCallFromJs (h:51) / MatchInternal (h:58) /
//!   Match (h:67 私有) → 推迟 [P-1 入口批] (Isolate/DirectHandle/FlatContent 硬耦合, 不生成桩体)
//! - h:29-30/38-50 结果协议注释 → Result 文档区承接 (StackOverflow/RETRY/outputs 语义)

// pub enum Result (5 值 + SMALLEST 界)
// pub enum CallOrigin (2 值)
// 落地核心语义面 (非 isolate, 详见 cc.js 对数): Bytecode 枚举 61 项 /
//   BacktrackStack / InterpreterRegisters / 寻址纯函数 / 字符辅助函数
`;

writeSrc("v8-regexp-exec", "regexp_interpreter.rs.skel", skel);

// ── 落地对数核对 (regexp_interpreter.rs 就地修订版逐项命中; 缺项 throw 列票单) ─
const landed = readFileSync(join(root, "v8-regexp-exec", "src", "regexp_interpreter.rs"), "utf8");
const LAND_TABLE = [
  [/pub enum Result \{/g, 1, "Result 枚举"],
  [/Failure = 0,/g, 1, "FAILURE=regexp.h:150"],
  [/Success = 1,/g, 1, "SUCCESS=regexp.h:151"],
  [/Exception = -1,/g, 1, "EXCEPTION=regexp.h:152"],
  [/Retry = -2,/g, 1, "RETRY=regexp.h:153"],
  [/FallbackToExperimental = -3,/g, 1, "FALLBACK=regexp.h:154"],
  [/pub const SMALLEST: i32 = -3;/g, 1, "kInternalRegExpSmallestResult=regexp.h:155"],
  [/pub enum CallOrigin \{/g, 1, "CallOrigin 枚举"],
  [/FromRuntime = 0,/g, 1, "kFromRuntime=regexp.h:112"],
  [/FromJs = 1,/g, 1, "kFromJs=regexp.h:113"],
  [/h:21-27/g, 3, "Result 双引用 h 行"],
  [/regexp\.h:150-154/g, 3, "值源双引用"],
  [/pub struct BacktrackStack/g, 1, "BacktrackStack (cc:129 落地)"],
  [/pub struct InterpreterRegisters/g, 1, "InterpreterRegisters (cc:172 落地)"],
  [/pub enum Bytecode/g, 1, "Bytecode 枚举 (bytecodes.h:336 落地)"],
];
const missing = [];
for (const [re, want, label] of LAND_TABLE) {
  const got = count(landed, new RegExp(re.source, "g"));
  if (got !== want) missing.push(`${label}: got=${got} want=${want}`);
  assertCount++;
}
// 推迟登记: h 侧 4 方法在模块头清单逐名在案, 且均无桩体
for (const n of Object.keys(METHOD_TABLE)) {
  if (!new RegExp(`\\b${n}\\b`).test(landed)) missing.push(`推迟登记名 ${n} 缺失`);
  assertCount++;
  // 桩体禁入: 不生成 `fn match_internal...` 同名小写函数
  const snake = n.replace(/([A-Z])/g, (c, p, i) => (i ? "_" : "") + c.toLowerCase());
  if (new RegExp(`fn ${snake}\\s*\\(`).test(landed))
    missing.push(`推迟项 ${n} 不应有桩体`);
  assertCount++;
}
if (missing.length) throw new Error(`regexp_interpreter.rs 落地对数缺项 (票单):\n  ${missing.join("\n  ")}`);

console.log(
  `regexp-interpreter.h.js: 对数断言 ${assertCount} 条全过; 原版类面全集 (守卫/AllStatic/` +
    `Result 5 值经 regexp.h 实测 0/1/-1/-2/-3 交叉 + Smallest=-3/静态方法 4/CallOrigin 0,1); ` +
    `落地 h 侧协议面逐项命中, 推迟 4 方法登记在案且 0 桩体; 产物 v8-regexp-exec/src/regexp_interpreter.rs.skel`,
);
