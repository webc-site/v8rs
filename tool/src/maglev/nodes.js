#!/usr/bin/env bun
// tool/src/maglev/nodes.js — 节点类层级 + 控制节点 (图节点规则先行)
// 覆盖: src/maglev/maglev-ir.h (Node/ValueNode/mixin 基类 + 全部节点类体) +
//       src/maglev/maglev-ir.cc + src/maglev/maglev-ir-inl.h
// 产出: v8-maglev-ir/src/nodes.rs.skel (规则代码: 基类表 + 输入数表 + 控制节点表)
// 复跑幂等: 纯文本扫描 + 确定性输出。
import { banner, readV8, writeSrc } from "../../lib.js";

const GEN = "src/maglev/nodes.js";
const H = "src/maglev/maglev-ir.h";
const CC = "src/maglev/maglev-ir.cc";
const INL = "src/maglev/maglev-ir-inl.h";
const h = readV8(H);
readV8(CC); readV8(INL);
const die = (m) => { throw new Error(`[nodes.js] ${m}`); };

// —— 基类锚 ————
for (const needle of [
  "class alignas(8) Node : public NodeBase {",
  "class ValueNode : public Node {",
  "class NodeTMixin : public BaseNode {",
  "class FixedInputNodeTMixin : public NodeTMixin<BaseT, Derived> {",
  "class VarargsNodeTMixin : public NodeTMixin<BaseT, Derived> {",
  "class Identity : public FixedInputValueNodeT<1, Identity> {",
  "class DeadValue : public FixedInputValueNodeT<0, DeadValue> {",
  "class Phi : public",
  "class RegisterInput : public FixedInputValueNodeT<0, RegisterInput> {",
  "class InitialValue : public FixedInputValueNodeT<0, InitialValue> {",
]) {
  if (!h.includes(needle)) die(`缺针 ${needle}`);
}
// 节点类总数锚 (grep 实测 173 个 `^class … : public`; 含 `class alignas(8) Node`)
const classCount = (h.match(/^class\s.+\s:\spublic/gm) ?? []).length;
if (classCount !== 173) die(`节点类数=${classCount}, 期望 173`);

// —— 控制节点类锚 (CONTROL_NODE_LIST 全族代表) ————
for (const needle of [
  "BRANCH_CONTROL_NODE_LIST(V)",
  "CONDITIONAL_CONTROL_NODE_LIST(V)",
  "UNCONDITIONAL_CONTROL_NODE_LIST(V)",
  "TERMINAL_CONTROL_NODE_LIST(V)",
  "CONTROL_NODE_LIST(V)",
  "NODE_BASE_LIST(V)",
  "class UnconditionalControlNode",
  "class ConditionalControlNode",
]) {
  if (!h.includes(needle)) die(`缺针 ${needle}`);
}
// 分支条件表示锚: ConditionFor / ConditionForNaN + Int32/Float64 比较分支
for (const needle of ["constexpr Condition ConditionFor(Operation cond);",
  "constexpr Condition ConditionForNaN();",
  "BranchIfInt32Compare", "BranchIfFloat64Compare", "BranchIfSmi"]) {
  if (!h.includes(needle)) die(`缺针 ${needle}`);
}

// —— 输入数表抽取: `FixedInput*NodeT<N, Name>` / `VarargsNodeT<M, Name>` ——
// 修订阶段转 Rust: 固定输入 → [ValueNode; N], 变参 → 切片; 输入数错则 regalloc 越界
// 注: 约 50 节点经模板父类 (Int32BinaryNode/Float64BinaryNode/CheckMaps…族) 间接定输入,
//   直接 FixedInput*NodeT< 仅 122; 两类相加≈总数, 此处断言直接数 + 模板父类存在性。
const fixed = new Map();
for (const m of h.matchAll(/class (\w+) : public FixedInput(?:Value)?NodeT<(\d+),\s*\1>/g)) {
  fixed.set(m[1], Number(m[2]));
}
const varargs = new Map();
for (const m of h.matchAll(/class (\w+) : public VarargsNodeT<(\d+),\s*\1>/g)) {
  varargs.set(m[1], Number(m[2]));
}
if (fixed.size !== 122) die(`固定输入节点=${fixed.size}, 期望 122`);
for (const needle of ["class Int32BinaryNode : public FixedInputValueNodeT<2, Derived> {",
  "class Float64BinaryNode : public FixedInputValueNodeT<2, Derived> {",
  "class CheckMaps : public FixedInputNodeT<1, CheckMaps> {",
  "class Switch : public FixedInputNodeTMixin<1, ConditionalControlNode, Switch> {"]) {
  if (!h.includes(needle)) die(`缺模板/控制父类针 ${needle}`);
}
// 抽样核对代表项输入数 (直接 FixedInput*NodeT< 类; 模板派生类查父类, 另有针)
const expectFixed = { Identity: 1, DeadValue: 0, RegisterInput: 0, CheckSmi: 1, MigrateMapIfNeeded: 2, Phi: -1, Call: -1, Switch: -1 };
for (const [k, v] of Object.entries(expectFixed)) {
  if (v < 0) { if (!h.includes(`class ${k} `)) die(`缺类 ${k}`); continue; }
  if (fixed.get(k) !== v) die(`输入数失配 ${k}=${fixed.get(k)}, 期望 ${v}`);
}

const out = [];
out.push(banner(GEN, `${H}, ${CC}, ${INL}`, "节点类层级 173 + 输入数表 + 控制节点"));
out.push(`//! 层级: NodeBase → Node → ValueNode → FixedInputValueNodeT<N>/VarargsNodeT<M>;`);
out.push(`//!   非值节点 → FixedInputNodeT<N>; 控制节点独立层级 (Unconditional/Conditional/Terminal)。`);
out.push(`//! 修订: 固定输入转 [ValueNode; N] 定长数组, 变参转切片; N 以本表为准。`);
out.push(``);
out.push(`// ── 基类 (5; 对齐 C++ 继承链, Rust 转组合 + trait) ──`);
out.push(`//   BASE NodeBase → Node(align 8) → ValueNode`);
out.push(`//   BASE NodeTMixin → FixedInputNodeTMixin / VarargsNodeTMixin`);
out.push(``);
out.push(`// ── 固定输入节点 (${fixed.size}; 类名=输入数, 修订转定长数组) ──`);
for (const [k, v] of [...fixed.entries()].sort()) out.push(`//   FIXED ${k} ${v}`);
out.push(``);
out.push(`// ── 变参节点 (${varargs.size}; 类名=最少输入, 修订转切片) ──`);
for (const [k, v] of [...varargs.entries()].sort()) out.push(`//   VARARGS ${k} ${v}`);
out.push(``);
out.push(`// ── 控制节点 (分支条件经 ConditionFor(Operation)/ConditionForNaN 映射) ──`);
out.push(`//   CTRL UnconditionalControlNode: Jump/CheckpointedJump/JumpLoop`);
out.push(`//   CTRL ConditionalControlNode: Switch + BranchIf* 全族 (见 opcodes.js)`);
out.push(`//   CTRL TerminalControlNode: Abort/Return/Deopt/Throw`);
out.push(``);
out.push(`// 对数: 基类针 10 + 类总数 173 + 控制针 8 + 条件针 5 + 输入抽样 6 全过`);

writeSrc("v8-maglev-ir", "nodes.rs.skel", out.join("\n") + "\n");
console.log(`OK nodes.js: 类 ${classCount}, 固定输入 ${fixed.size}, 变参 ${varargs.size} → v8-maglev-ir/src/nodes.rs.skel`);
