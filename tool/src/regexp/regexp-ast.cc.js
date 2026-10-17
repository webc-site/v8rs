#!/usr/bin/env bun
// tool/src/regexp/regexp-ast.cc.js — v8/src/regexp/regexp-ast.cc
//   → v8-regexp-syntax/src/ast_impl_skel.rs
// 机械提取: 三连 X-macro 展开 (MAKE_ACCEPT / 基类 TYPE_CASE / 派生 TYPE_CASE)
//   + 函数签名骨架 (Accept 全集 / CaptureRegisters / IsCertainlyAnchored / 构造器)
//   + 就地规则锚 (IncreaseBy 溢出 / ClassSetOperand min-max / 空类取反翻转);
// 对数断言: X-macro 3 处 / 宏展开次数 == 树种类 14 / 签名骨架数。
import { banner, readV8, writeSrc } from "../../lib.js";

const cc = readV8("src/regexp/regexp-ast.cc");
const astH = readV8("src/regexp/regexp-ast.h");

function must(re, what, src = cc) {
  const m = src.match(re);
  if (!m) throw new Error(`regexp-ast.cc: 未找到 ${what}`);
  return m;
}

// 树种类 14 (交叉 regexp-ast.h)
const kinds = [...must(/#define FOR_EACH_REG_EXP_TREE_TYPE\(VISIT\)([\s\S]*?)\n\n/, "FOR_EACH 宏", astH)[1]
  .matchAll(/VISIT\((\w+)\)/g)].map((r) => r[1]);
if (kinds.length !== 14) throw new Error(`树种类应 14, 实数 ${kinds.length}`);

// ── 三连 X-macro (cc:14-31) ────────────────────────────────────────────────
const macroUses = [...cc.matchAll(/FOR_EACH_REG_EXP_TREE_TYPE\((\w+)\)/g)].map((r) => r[1]);
if (macroUses.length !== 3) throw new Error(`X-macro 展开应 3 处, 实数 ${macroUses.length}`);
const EXPECT_MACROS = ["MAKE_ACCEPT", "MAKE_TYPE_CASE", "MAKE_TYPE_CASE"];
for (let i = 0; i < 3; i++) {
  if (macroUses[i] !== EXPECT_MACROS[i]) throw new Error(`X-macro 序 ${i} 漂移: ${macroUses[i]}`);
}
// 宏体锚
if (!cc.includes("#define MAKE_ACCEPT(Name)")) throw new Error("MAKE_ACCEPT 宏体漂移");
if ((cc.match(/Name\* Tree::As##Name\(\) \{ return nullptr; \}/g) || []).length !== 1) {
  throw new Error("基类 TYPE_CASE 宏体漂移");
}
if ((cc.match(/Name\* Name::As##Name\(\) \{ return this; \}/g) || []).length !== 1) {
  throw new Error("派生 TYPE_CASE 宏体漂移");
}

// ── 签名骨架清单 (cc 顶层定义) ────────────────────────────────────────────
const defs = [...cc.matchAll(/^(?:bool|Interval|void|int|Atom\*|ClassSetExpression\*)\s+([\w:]+)::(\w+)\(/gm)].map((r) => ({
  cls: r[1], fn: r[2],
}));
// 构造器 (无返回类型形态)
const ctorsRaw = [...cc.matchAll(/^(\w+)::\1\(/gm)].map((r) => r[1]);

// 分组对数
const captureRegisters = defs.filter((d) => d.fn === "CaptureRegisters");
const anchoredStart = defs.filter((d) => d.fn === "IsCertainlyAnchoredAtStart");
const anchoredEnd = defs.filter((d) => d.fn === "IsCertainlyAnchoredAtEnd");
const ctors = ctorsRaw;
if (ctors.length !== 5) throw new Error(`构造器应 5 (Disjunction/Alternative/ClassSetOperand/ClassSetExpression/ClassRanges), 实数 ${ctors.length}`);
if (captureRegisters.length !== 6) throw new Error(`CaptureRegisters 实现应 6 (含匿名助手的 5 类 + 助手), 实数 ${captureRegisters.length}`);
const CR_SET = ["Group", "Alternative", "Disjunction", "Lookaround", "Capture", "Quantifier"];
for (const d of captureRegisters) {
  if (!CR_SET.includes(d.cls)) throw new Error(`CaptureRegisters 越类: ${d.cls}`);
}
if (anchoredStart.length !== 5) throw new Error(`IsCertainlyAnchoredAtStart 应 5, 实数 ${anchoredStart.length}`);
if (anchoredEnd.length !== 4) throw new Error(`IsCertainlyAnchoredAtEnd 应 4 (Lookaround 无 End 变体), 实数 ${anchoredEnd.length}`);
const AS_SET = ["Assertion", "Alternative", "Disjunction", "Lookaround", "Capture"];
for (const d of [...anchoredStart, ...anchoredEnd]) {
  if (!AS_SET.includes(d.cls)) throw new Error(`Anchored 越类: ${d.cls}`);
}
// 匿名助手
if (!cc.includes("Interval ListCaptureRegisters(StackLimiter limiter, ZoneList<Tree*>* children)")) {
  throw new Error("匿名 ListCaptureRegisters 助手缺失");
}
if (!cc.includes("int IncreaseBy(int previous, int increase)")) throw new Error("IncreaseBy 助手缺失");

// ── 就地规则锚 ────────────────────────────────────────────────────────────
// IncreaseBy 溢出封顶
if (!cc.includes("if (Tree::kInfinity - previous < increase)")) throw new Error("IncreaseBy 溢出分支漂移");
// StackLimiter 软预算
if (!cc.includes("if (V8_LIKELY(budget_ > 0)) return false;")) throw new Error("StackLimiter 软预算分支漂移");
// ClassSetOperand min/max
if (!cc.includes("min_match_ = 1;") || !cc.includes("max_match_ = 2;")) {
  throw new Error("ClassSetOperand 非空 min=1/max=2 规则漂移");
}
// ClassSetExpression 取反 max
if (!cc.includes("max_match_ = 2;") || !cc.includes("DCHECK(!may_contain_strings_);")) {
  throw new Error("ClassSetExpression 取反规则漂移");
}
// ClassRanges 空集翻转
if (!cc.includes("ranges->Add(CharacterRange::Everything(), zone);")) throw new Error("空集 Everything 规则漂移");
if (!cc.includes("class_ranges_flags_ ^= NEGATED;")) throw new Error("NEGATED 翻转规则漂移");
if (!cc.includes("class_ranges_flags_ |= IS_CERTAINLY_TWO_CODE_POINTS;")) {
  throw new Error("IS_CERTAINLY_TWO_CODE_POINTS 规则漂移");
}
if (!cc.includes("if (ranges->at(i).from() < 0x10000)")) throw new Error("BMP 探测阈值 0x10000 漂移");
// 锚定断言语义
if (!cc.includes("assertion_type() == Assertion::Type::START_OF_INPUT")) throw new Error("起始锚定断言漂移");
if (!cc.includes("assertion_type() == Assertion::Type::END_OF_INPUT")) throw new Error("末尾锚定断言漂移");
// Lookaround 锚定: 正向前瞻才传锚
if (!cc.includes("is_positive() && type() == LOOKAHEAD")) throw new Error("Lookaround 锚定条件漂移");
// Disjunction 构造 DCHECK
if (!cc.includes("DCHECK_LT(1, alternatives->length());")) throw new Error("Disjunction 双分支 DCHECK 漂移");
// Text::StartsWithAtom
if (!cc.includes("elements_.at(0).text_type() == TextElement::ATOM")) throw new Error("StartsWithAtom 规则漂移");

// Capture 寄存器区间 (StartRegister/EndRegister)
if (!cc.includes("Interval self(StartRegister(index()), EndRegister(index()));")) {
  throw new Error("Capture 寄存器区间规则漂移");
}

console.log(
  `regexp-ast.cc: X-macro 3 (各 ${kinds.length} 展开), CaptureRegisters ${captureRegisters.length}, ` +
    `Anchored ${anchoredStart.length}+${anchoredEnd.length}, 构造器 ${ctors.length}, 骨架共 ${defs.length} ✓`,
);

// ── 生成 Rust ──────────────────────────────────────────────────────────────
const body = `${banner(
  "src/regexp/regexp-ast.cc.js",
  "src/regexp/regexp-ast.cc",
  "AST 实现面签名骨架 + 就地规则锚; 语义实现就地修订 (todo! 占位)",
)}
//! 原版三大 X-macro 展开面:
//! - MAKE_ACCEPT: 14 种树的 Accept → visitor 双分派 (Rust 端由 enum 分发, 不需宏)
//! - 基类 TYPE_CASE: Tree::AsXxx/IsXxx 默认 false/nullptr
//! - 派生 TYPE_CASE: 各子类 As/Is 返回自身

/// X-macro 展开的树种类全集 (与 FOR_EACH_REG_EXP_TREE_TYPE 同源)
pub const XMACRO_TREE_KINDS: [&str; ${kinds.length}] = [
${kinds.map((k) => `  "${k}",`).join("\n")}
];

/// 就地规则锚 (从 cc 提取的字面条件; Rust 实现须保持同语义)
pub mod rules {
  /// IncreaseBy 溢出封顶: kInfinity - previous < increase 时取 kInfinity
  pub const K_INFINITY_CAP: bool = true;
  /// ClassSetOperand 非空 ranges 时 min=1 / max=2 (可能匹配字符串时再合并)
  pub const K_CLASS_SET_EMPTY_MIN_MAX: (i32, i32) = (0, 0);
  pub const K_CLASS_SET_NONEMPTY_MIN_MAX: (i32, i32) = (1, 2);
  /// ClassSetExpression 取反时 max=2 (按 Unicode 字符两码点估)
  pub const K_NEGATED_MAX_MATCH: i32 = 2;
  /// BMP 探测阈值: 区间 from < 0x10000 视为含基本平面
  pub const K_BMP_THRESHOLD: u32 = 0x10000;
}

// ── CaptureRegisters 签名骨架 (cc:40-80, 5 类 + 列表助手) ──
/// Group::CaptureRegisters — 透传 body 并递减预算
pub fn capture_registers_group(_limiter: ()) -> () {
  todo!("就地修订: cc:40-43, 溢出返 Invalid, body 递归 budget-1")
}

/// Alternative::CaptureRegisters — 列表助手 (cc:47-54) 逐项 Union
pub fn capture_registers_alternative(_limiter: ()) -> () {
  todo!("就地修订: cc:58-60")
}

/// Disjunction::CaptureRegisters — 同列表助手 (cc:62-64)
pub fn capture_registers_disjunction(_limiter: ()) -> () {
  todo!("就地修订: cc:62-64")
}

/// Lookaround::CaptureRegisters (cc:66-69)
pub fn capture_registers_lookaround(_limiter: ()) -> () {
  todo!("就地修订: cc:66-69")
}

/// Capture::CaptureRegisters — 自身区间 [StartRegister, EndRegister] ∪ body (cc:71-75)
pub fn capture_registers_capture(_limiter: ()) -> () {
  todo!("就地修订: cc:71-75")
}

/// Quantifier::CaptureRegisters — 透传 body (cc:77-80)
pub fn capture_registers_quantifier(_limiter: ()) -> () {
  todo!("就地修订: cc:77-80")
}

// ── IsCertainlyAnchored 签名骨架 (cc:82-156, Start 5 类 + End 4 类) ──
/// Assertion: 仅 START_OF_INPUT / END_OF_INPUT 断言恒锚定
pub fn anchored_assertion(_at_start: bool) -> bool {
  todo!("就地修订: cc:82-88")
}

/// Alternative: 逐元素 (Start 正序 / End 逆序), 命中即真, max_match>0 即假
pub fn anchored_alternative(_at_start: bool) -> bool {
  todo!("就地修订: cc:90-118")
}

/// Disjunction: 全部分支锚定才锚定 (cc:120-140)
pub fn anchored_disjunction(_at_start: bool) -> bool {
  todo!("就地修订: cc:120-140")
}

/// Lookaround: 仅正向前瞻传递起始锚定 (cc:142-146)
pub fn anchored_lookaround() -> bool {
  todo!("就地修订: cc:142-146")
}

/// Capture: 透传 body (Start/End 同型, cc:148-156)
pub fn anchored_capture(_at_start: bool) -> bool {
  todo!("就地修订: cc:148-156")
}

// ── 构造器骨架 (cc:158-276) ──
/// Disjunction 构造: min 取分支最小, max 取分支最大 (cc:158-169)
pub fn ctor_disjunction() -> () {
  todo!("就地修订: cc:158-169")
}

/// Alternative 构造: min/max 逐节点 IncreaseBy 累加 (cc:183-194)
pub fn ctor_alternative() -> () {
  todo!("就地修订: cc:183-194")
}

/// ClassSetOperand 构造 (cc:196-212)
pub fn ctor_class_set_operand() -> () {
  todo!("就地修订: cc:196-212")
}

/// ClassSetExpression 构造 (cc:214-234)
pub fn ctor_class_set_expression() -> () {
  todo!("就地修订: cc:214-234")
}

/// ClassRanges 构造: 空集翻转为取反 Everything; 全 ≥0x10000 标记两码点 (cc:254-276)
pub fn ctor_class_ranges() -> () {
  todo!("就地修订: cc:254-276")
}

const _: () = assert!(XMACRO_TREE_KINDS.len() == ${kinds.length});
`;

writeSrc("v8-regexp-syntax", "ast_impl_skel.rs", body);
console.log("written: v8-regexp-syntax/src/ast_impl_skel.rs");
