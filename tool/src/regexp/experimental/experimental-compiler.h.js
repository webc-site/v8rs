#!/usr/bin/env bun
// tool/src/regexp/experimental/experimental-compiler.h.js
//   — v8/src/regexp/experimental/experimental-compiler.h → v8-regexp-exec/src/experimental/compiler.rs (入口层)
// 规则: AllStatic 类壳 + static 声明全集机械解析 (返回类型/函数名/形参列表逐项对账) +
//   声明行号锚 (h:24 CanBeHandled / h:28 Compile) + 4 个 include + 头卫 + 命名空间三层嵌套 +
//   TODO(mbid,v8:10765) 未支持面注释 (h:22-23)
// 产物: v8-regexp-exec/src/experimental/compiler.rs.skel (入口签名映射骨架, 审计专用)
//   + 落地对数核对 (compiler.rs 入口逐项命中; Zone*/v8_flags/capture_count 三处签名映射
//   必须在模块头显式登记, 缺项 throw — 禁隐性偏离原版)
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { banner, readV8, root, writeSrc } from "../../../lib.js";

const h = readV8("src/regexp/experimental/experimental-compiler.h");

let assertCount = 0;
const must = (cond, msg) => {
  assertCount++;
  if (!cond) throw new Error(`对数失配: ${msg}`);
};
const count = (src, re) => (src.match(re) || []).length;
const lineOf = (src, needle) => {
  const i = src.split("\n").findIndex((l) => l.includes(needle));
  must(i >= 0, `原版锚点缺失: ${needle}`);
  return i + 1;
};

// ── 原版侧解析 ─────────────────────────────────────────────────────────────

// 1) 头卫 + include 全集 (h:5-11) 与命名空间三层 (h:13-15)
must(
  h.includes("#ifndef V8_REGEXP_EXPERIMENTAL_EXPERIMENTAL_COMPILER_H_"),
  "h:5 头卫 ifndef 锚点",
);
must(count(h, /#ifndef|#define|#endif/g) === 3, "头卫三件套 (ifndef/define/endif)");
const includes = [...h.matchAll(/^#include "([^"]+)"$/gm)].map((m) => m[1]);
must(includes.length === 4, `h: include 应 4, 实得 ${includes.length}: ${includes.join(",")}`);
for (const inc of [
  "src/regexp/experimental/experimental-bytecode.h",
  "src/regexp/regexp-ast.h",
  "src/regexp/regexp-flags.h",
  "src/zone/zone-list.h",
]) {
  must(includes.includes(inc), `h: include ${inc}`);
}
// zone-list.h 只服务 `ZoneList<Instruction> Compile` 返回类型 ⟹ Rust 侧无需对应物
must(count(h, /^namespace \w+ \{$/gm) === 3, "命名空间三层嵌套 (v8/internal/regexp) 开");
must(count(h, /^\}  \/\/ namespace \w+$/gm) === 3, "命名空间三层闭注释 ×3");
must(/namespace v8 \{\nnamespace internal \{\nnamespace regexp \{/.test(h), "h:13-15 嵌套顺序");

// 2) 类壳: AllStatic (不可实例化 ⟹ Rust 侧只能是无状态自由函数)
must(
  h.includes("class ExperimentalCompiler final : public AllStatic"),
  "h:17 ExperimentalCompiler : AllStatic",
);

// 3) static 声明全集 (恰 2 个, 逐项含返回类型与形参列表)
const decls = [...h.matchAll(/^  static ([\w:<>*& ]+?) (\w+)\(([^)]*)\);$/gm)];
must(decls.length === 2, `h: static 声明应 2, 实得 ${decls.length}`);
const DECL_EXPECT = [
  { name: "CanBeHandled", ret: "bool", params: "Tree* tree, Flags flags, int capture_count" },
  { name: "Compile", ret: "ZoneList<Instruction>", params: "Tree* tree, Flags flags, Zone* zone" },
];
for (const want of DECL_EXPECT) {
  const got = decls.find((d) => d[2] === want.name);
  must(got !== undefined, `h: 声明 ${want.name}`);
  must(got[1].trim() === want.ret, `h: ${want.name} 返回类型 ${got[1].trim()} != ${want.ret}`);
  must(got[3].trim() === want.params, `h: ${want.name} 形参 ${got[3].trim()} != ${want.params}`);
}
// 行号锚 (转写注释以 h:24 / h:28 为键)
must(lineOf(h, "static bool CanBeHandled(") === 24, "h:24 CanBeHandled 行号");
must(lineOf(h, "static ZoneList<Instruction> Compile(") === 28, "h:28 Compile 行号");

// 4) 未支持面 TODO (h:22-23) — 反引号 typo (`CanBeHandled`. 前缺闭引号) 原样不改语义
const todo = h.match(/TODO\(([^)]*)\)/g) || [];
must(todo.length === 1 && todo[0] === "TODO(mbid,v8:10765)", `h: TODO 恰 1 (实得 ${todo.join("|")})`);
must(h.includes("e.g. some\n  // quantifiers and unicode"), "h:22-23 未支持面文案 (quantifiers/unicode)");

// ── 产物: 入口签名映射骨架 ─────────────────────────────────────────────────
const skel = `${banner(
  "src/regexp/experimental/experimental-compiler.h.js",
  "src/regexp/experimental/experimental-compiler.h",
  "入口签名映射骨架 (审计专用): h 层只含 2 个 static, 实现体在 .cc (见 experimental-compiler.cc.js)",
)}
//! ExperimentalCompiler : AllStatic (h:17) ⟹ 无实例状态 → Rust 无状态自由函数 (模块级 pub)
//!
//! h:24  static bool CanBeHandled(Tree* tree, Flags flags, int capture_count)
//!       → pub fn can_be_handled(tree: &Tree, flags: RegExpFlags, capture_count: i32,
//!                              v8_flags: &V8Flags) -> bool
//!   - capture_count: 原版声明而 cc:30-35 体内从不读取 → 1:1 保留形参并登记
//!   - Flags (typedef uint32_t, regexp-flags.h) → RegExpFlags (syntax crate 冻结类型)
//!   - v8_flags 注入: cc:187 读全局 v8_flags.experimental_regexp_engine_capture_group_opt
//!
//! h:28  static ZoneList<Instruction> Compile(Tree* tree, Flags flags, Zone* zone)
//!       → pub fn compile(tree: &Tree, flags: RegExpFlags, v8_flags: &V8Flags)
//!                              -> Vec<Instruction>
//!   - Zone* 删除: zone 裸分配容器 (cc:257 code_ / cc:530 nodes_ / cc:612-614,1199-1200
//!     ZoneMap / cc:1197 ZoneLinkedList) → Rust 栈上所有权容器 Vec/VecDeque/RemapMap
//!   - 返回 ZoneList<Instruction> (cc:1219-1222) → Vec<Instruction>
//!   - 前置条件 "must be handlable; see CanBeHandled" (h:25-27) 由调用方保证 (原版同)
//!
//! 实现类全集 (cc 层, 由 experimental-compiler.cc.js 逐函数对账):
//!   CanBeHandledVisitor / Label / BytecodeAssembler / FilterGroupsCompileVisitor /
//!   CompileVisitor
`;
writeSrc("v8-regexp-exec", "experimental/compiler.rs.skel", skel);

// ── 落地对数核对 ───────────────────────────────────────────────────────────
const dir = join(root, "v8-regexp-exec", "src", "experimental");
const landed = readFileSync(join(dir, "compiler.rs"), "utf8");
const modrs = readFileSync(join(dir, "mod.rs"), "utf8");
const missing = [];
const land = (re, want, label) => {
  const got = count(landed, re);
  if (got !== want) missing.push(`${label}: got=${got} want=${want}`);
  assertCount++;
};

// 1) 两个入口以 pub 自由函数落地, 返回类型对齐 (逐形参对账, 禁隐性偏离)
land(
  /^pub fn can_be_handled\(\n {2}tree: &Tree,\n {2}flags: RegExpFlags,\n {2}capture_count: i32,\n {2}v8_flags: &V8Flags,\n\) -> bool \{$/m,
  1,
  "Rust can_be_handled 入口签名 (h:24 映射)",
);
land(
  /^pub fn compile\(tree: &Tree, flags: RegExpFlags, v8_flags: &V8Flags\) -> Vec<Instruction> \{$/m,
  1,
  "Rust compile 入口签名 (h:28 映射, Zone* 已登记删除)",
);
// 2) 行号锚入注释 (可追溯性硬要求)
land(/h:24/, 1, "注释锚 h:24");
land(/h:28/, 1, "注释锚 h:28");
land(/experimental-compiler\.h/, 1, "注释引用原文件 experimental-compiler.h");
// 3) 三处签名映射必须显式登记 (禁隐性偏离)
land(/`Zone\*` 形参删除/, 1, "登记: Zone* 形参删除");
land(/capture_count`? [^\n]*原版声明[^\n]*从不/, 1, "登记: capture_count 原版未读取");
land(/let _ = capture_count;/, 1, "capture_count 形参 1:1 保留");
land(/全局 `v8_flags\./, 1, "登记: 全局 v8_flags → 注入 &V8Flags");
land(/use v8_flags::Flags as V8Flags;/, 1, "V8Flags 注入类型 import");
// 4) 入口实现即原版两行转发 (cc:217-220 / cc:1219-1222)
land(/CanBeHandledVisitor::check\(/, 1, "can_be_handled → Check 转发 (cc:219)");
land(/CompileVisitor::compile\(/, 1, "compile → CompileVisitor::Compile 转发 (cc:1221)");
// 5) 模块挂载 (mod.rs 属他席, 只读核对, 不改)
must(/pub mod compiler;/.test(modrs), "mod.rs 应已声明 pub mod compiler (只读核对)");

if (missing.length) throw new Error(`compiler.rs 入口落地缺项 (票单):\n  ${missing.join("\n  ")}`);

console.log(
  `experimental-compiler.h.js: 对数断言 ${assertCount} 条全过; 原版 h 层 2 个 static 声明` +
    ` (h:24 CanBeHandled(Tree*,Flags,int) / h:28 Compile(Tree*,Flags,Zone*), AllStatic 类壳,` +
    ` include 4, 头卫 3, 命名空间 3 层, TODO(mbid,v8:10765) ×1) 逐项对账;` +
    ` 落地入口 2 + 行号锚 + Zone*/capture_count/v8_flags 三处签名映射登记全命中, 缺项 0;` +
    ` 产物 v8-regexp-exec/src/experimental/compiler.rs.skel`,
);
