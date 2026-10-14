#!/usr/bin/env bun
// tool/src/maglev/opcodes.js — Maglev Opcode 全表 (图节点规则先行)
// 覆盖: src/maglev/maglev-ir.h (全部 *_NODE_LIST 宏 → Opcode 枚举)
// 产出: v8-maglev-ir/src/opcodes.rs.skel (规则代码: Opcode 枚举全表 + 分组清单)
// 对数基准: clang-AST 非脚本自解析 (todo.md 十一章红线); 本脚本断言行数锚,
//   Developer 修订时取 `clang -Xclang -ast-dump=json` 逐变体核对。
// 复跑幂等: 纯文本扫描 + 固定排序(原版宏顺序) + 确定性输出, 无时间戳。
import { banner, readV8, writeSrc } from "../../lib.js";

const GEN = "src/maglev/opcodes.js";
const SRC = "src/maglev/maglev-ir.h";
const txt = readV8(SRC);
const die = (m) => { throw new Error(`[opcodes.js] ${m}`); };

// —— 锚点: 子列表宏定义必须存在 ————————————————
const LISTS = [
  "GENERIC_OPERATIONS_NODE_LIST",
  "BIGINT_OPERATIONS_NODE_LIST",
  "INT32_OPERATIONS_NODE_LIST",
  "FLOAT64_OPERATIONS_NODE_LIST",
  "SMI_OPERATIONS_NODE_LIST",
  "CONSTANT_VALUE_NODE_LIST",
  "INLINE_BUILTIN_NODE_LIST",
  "TURBOLEV_VALUE_NODE_LIST",
  "TURBOLEV_NON_VALUE_NODE_LIST",
  "CONVERSION_NODE_LIST",
  "VALUE_NODE_LIST",
  "GAP_MOVE_NODE_LIST",
  "NON_VALUE_NODE_LIST",
  "NODE_LIST",
  "BRANCH_CONTROL_NODE_LIST",
  "CONDITIONAL_CONTROL_NODE_LIST",
  "UNCONDITIONAL_CONTROL_NODE_LIST",
  "TERMINAL_CONTROL_NODE_LIST",
  "CONTROL_NODE_LIST",
  "NODE_BASE_LIST",
];
for (const l of LISTS) {
  if (!txt.includes(`#define ${l}(V)`)) die(`缺宏 #define ${l}(V)`);
}
// Opcode 枚举 + 计数器锚
for (const needle of [
  "enum class Opcode : uint16_t { NODE_BASE_LIST(DEF_OPCODES) };",
  "static constexpr int kOpcodeCount = NODE_BASE_LIST(PLUS_ONE);",
  "static constexpr Opcode kFirstNodeOpcode",
  "static constexpr Opcode kLastNodeOpcode",
]) {
  if (!txt.includes(needle)) die(`缺针 ${needle}`);
}

// —— 抽取: 全部 `V(Name)` / `IF_*(V, Name` 行, 去重保序 (原版宏顺序即枚举序) ——
// 405 行去重 405 唯一 (python 实测); IF_UD/IF_NOT_UD 二选一, IF_INTL 按目标配置取舍,
// 修订时以 clang -E 预处理地面真值为准, 此处两分支都登记。
const names = [];
const seen = new Set();
for (const m of txt.matchAll(/^\s*(?:V|IF_\w+)\(V?,\s*(\w+)|^\s*V\((\w+)\)/gm)) {
  const n = m[1] ?? m[2];
  if (!seen.has(n)) { seen.add(n); names.push(n); }
}
// 分组规模锚 (python 实测 405 唯一; 原版漂移即停下修脚本)
if (names.length !== 405) die(`Opcode 总数=${names.length}, 期望 405`);
const has = (n) => { if (!seen.has(n)) die(`缺节点 ${n}`); };
// 各族代表针 (全表由脚本产出清单, 断言代表项防漂移)
for (const n of [
  "GenericAdd", "BigIntCompare", "Int32Add", "Float64Add",
  "CheckedSmiIncrement", "HeapConstant", "BuiltinStringFromCharCode",
  "CreateFastArrayElements", "TransitionAndStoreArrayElement",
  "ChangeInt32ToFloat64", "Identity", "Phi", "CheckSmi", "GapMove",
  "BranchIfSmi", "Switch", "Jump", "Return", "Deopt",
  "HoleyFloat64IsHole", "HoleyFloat64IsUndefinedOrHole", "StringLocaleCompareIntl",
]) has(n);

// —— 条件编译分支登记 (目标配置=Chromium 默认 x64/arm64 release, 见 todo.md 五) ——
// IF_UD/IF_NOT_UD (maglev-ir.h:72-76): 是否 undefined 判 hole 二选一, 修订取其一
// IF_INTL (StringLocaleCompareIntl 等 3 处): V8_INTL_SUPPORT 开默认进表
const condNames = ["HoleyFloat64IsUndefinedOrHole", "HoleyFloat64IsHole", "StringLocaleCompareIntl",
  "BranchIfFloat64IsUndefinedOrHole", "StringPrototypeToLowerCaseIntl", "StringPrototypeToUpperCaseIntl"];

// —— 产出 ————
const out = [];
out.push(banner(GEN, SRC, "Opcode 全表 405; 条件分支 6 处见下登记"));
out.push(`//! 图节点规则先行: 本表即 Opcode 真值来源; 类体见 nodes.js, 控制节点见 control.js。`);
out.push(`//! 对数基准: clang -Xclang -ast-dump=json 逐变体核对 (禁脚本自证)。`);
out.push(``);
out.push(`// ── 条件编译登记 (修订时按目标配置取舍, 非目标分支 feature 隔离默认关) ──`);
for (const n of condNames) out.push(`//   COND ${n}`);
out.push(``);
out.push(`// ── Opcode 全表 (${names.length} 变体, 顺序即 C++ 枚举序) ──`);
out.push(`//   pub enum Opcode : u16 — 修订阶段按此顺序展开, 变体名与 C++ 1:1`);
for (const n of names) out.push(`//   V ${n}`);
out.push(``);
out.push(`// 对数: 子列表宏 20 + 枚举计数锚 4 + 代表针 22 + 总数 405 全过`);

writeSrc("v8-maglev-ir", "opcodes.rs.skel", out.join("\n") + "\n");
console.log(`OK opcodes.js: ${names.length} opcode → v8-maglev-ir/src/opcodes.rs.skel`);
