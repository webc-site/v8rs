#!/usr/bin/env bun
// tool/src/compiler/js-type-hint-lowering.js
// 从 v8/src/compiler/js-type-hint-lowering.h + js-type-hint-lowering.cc
// 提取 Flag 位标志 (注意 1<<1)、LoweringResult 三态、hint→NumberOperationHint 映射与 Reduce 面
// 生成 v8-compiler-frontend/src/js_type_hint_lowering.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/js-type-hint-lowering.h");
const cc = readV8("src/compiler/js-type-hint-lowering.cc");

// —— 1. 锚点断言 ——
const anchors = [
  ["class JSTypeHintLowering {", "JSTypeHintLowering 类"],
  ["enum Flag { kNoFlags = 0u, kBailoutOnUninitialized = 1u << 1 };", "Flag: 1u<<1 (注意异于 1<<0)"],
  ["class LoweringResult {", "LoweringResult 嵌套类"],
  ["bool Changed() const { return kind_ != LoweringResultKind::kNoChange; }", "Changed 判据"],
  ["bool IsExit() const { return kind_ == LoweringResultKind::kExit; }", "IsExit 判据"],
  ["enum class LoweringResultKind { kNoChange, kSideEffectFree, kExit };", "三态枚举"],
  ["static LoweringResult SideEffectFree(", "SideEffectFree 工厂"],
  ["static LoweringResult NoChange() {", "NoChange 工厂"],
  ["static LoweringResult Exit(Node* control) {", "Exit 工厂"],
  ["DCHECK(value->op()->HasProperty(Operator::kNoThrow));", "SideEffectFree 无抛前提"],
];
for (const [lit, name] of anchors) {
  if (!h.includes(lit)) throw new Error(`js-type-hint-lowering.h 锚点缺失: ${name}`);
}
const ccAnchors = [
  ["bool BinaryOperationHintToNumberOperationHint(", "hint→Number 映射"],
  ["bool BinaryOperationHintToBigIntOperationHint(", "hint→BigInt 映射"],
  ["BinaryOperationHint JSTypeHintLowering::GetBinaryOperationHint(", "GetBinaryOperationHint 实现"],
  ["CompareOperationHint JSTypeHintLowering::GetCompareOperationHint(", "GetCompareOperationHint 实现"],
];
for (const [lit, name] of ccAnchors) {
  if (!cc.includes(lit)) throw new Error(`js-type-hint-lowering.cc 锚点缺失: ${name}`);
}

// —— 2. 真解析: hint → NumberOperationHint 映射 ——
const numSrc = cc.slice(
  cc.indexOf("bool BinaryOperationHintToNumberOperationHint"),
  cc.indexOf("bool BinaryOperationHintToBigIntOperationHint"),
);
const numPairs = [...numSrc.matchAll(/case BinaryOperationHint::k(\w+):\s*\n\s*\*number_hint = NumberOperationHint::k(\w+);/g)]
  .map((m) => [m[1], m[2]]);
const numFlagged = [...numSrc.matchAll(/case BinaryOperationHint::k(\w+):\s*\n\s*\*number_hint = v8_flags\.(\w+)\s*\n\s*\?\s*NumberOperationHint::k(\w+)\s*\n\s*:\s*NumberOperationHint::k(\w+);/g)]
  .map((m) => [m[1], m[2], m[3], m[4]]);
if (numPairs.length + numFlagged.length !== 5) throw new Error(`Number 映射应 5 路: ${numPairs.length}+${numFlagged.length}`);

// —— 3. 真解析: Reduce 公有面 ——
const reduceFns = [...h.matchAll(/LoweringResult (Reduce\w+)\(/g)].map((m) => m[1]);
const uniqueReduce = [...new Set(reduceFns)];

// —— 4. cc 实现计数 ——
const impls = new Set([...cc.matchAll(/^JSTypeHintLowering::LoweringResult\s*\n?JSTypeHintLowering::(Reduce\w+)\(|^JSTypeHintLowering::LoweringResult JSTypeHintLowering::(Reduce\w+)\(/gm)].map((m) => m[1] ?? m[2]));

const body = `${banner("src/compiler/js-type-hint-lowering.js", "src/compiler/js-type-hint-lowering.h + js-type-hint-lowering.cc", "类型提示早期降级: 建图期直发 speculative simplified 算子")}

bitflags! {
    /// v8: src/compiler/js-type-hint-lowering.h JSTypeHintLowering::Flag
    /// 注意: kBailoutOnUninitialized = 1u << 1 (此族独用 bit1)
    pub struct Flags: u32 {
        const NO_FLAGS = 0;
        const BAILOUT_ON_UNINITIALIZED = 1 << 1;
    }
}

/// 降级结果三态
/// v8: src/compiler/js-type-hint-lowering.h JSTypeHintLowering::LoweringResultKind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoweringResultKind {
    NoChange,
    SideEffectFree,
    Exit,
}

/// 降级结果 — SideEffectFree 带 value/effect/control; Exit 只带 control (连到 End)
/// v8: src/compiler/js-type-hint-lowering.h JSTypeHintLowering::LoweringResult
pub enum LoweringResult {
    /// 未降级, 调用方走 generic 版
    NoChange,
    /// 无副作用: value/effect/control 三元组 (op 须 kNoThrow)
    SideEffectFree { value: NodeRef, effect: NodeRef, control: NodeRef },
    /// 图出口 (deopt)
    Exit { control: NodeRef },
}

/// 早期降级器 — 非标准 reducer 接口, 建图前调用
/// v8: src/compiler/js-type-hint-lowering.h JSTypeHintLowering
pub struct JSTypeHintLowering { broker: JSHeapBrokerRef, jsgraph: JSGraphRef, flags: Flags, feedback_vector: FeedbackVectorRef }

impl JSTypeHintLowering {
    // —— Reduce 面 (${uniqueReduce.length} 个) ——
${uniqueReduce.map((n) => `    /// v8: src/compiler/js-type-hint-lowering.h JSTypeHintLowering::${n}\n    pub fn ${n.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase()}(&self /*, op/operands/effect/control/slot */) -> LoweringResult;`).join("\n")}

    // —— hint 查询 ——
    /// v8: src/compiler/js-type-hint-lowering.cc JSTypeHintLowering::GetBinaryOperationHint
    fn get_binary_operation_hint(&self, slot: FeedbackSlot) -> BinaryOperationHint;
    /// v8: src/compiler/js-type-hint-lowering.cc JSTypeHintLowering::GetCompareOperationHint
    fn get_compare_operation_hint(&self, slot: FeedbackSlot) -> CompareOperationHint;
}

/// BinaryOperationHint → NumberOperationHint (成功 ${numPairs.length + numFlagged.length} 路):
${numPairs.map(([k, v]) => `/// k${k} => k${v}`).join("\n")}
${numFlagged.map(([k, flag, t, f]) => `/// k${k} => 旗标 ${flag} ? k${t} : k${f}`).join("\n")}
/// 其余 (kAny/kNone/kString/kStringOrStringWrapper/kBigInt/kBigInt64) → false
/// v8: src/compiler/js-type-hint-lowering.cc BinaryOperationHintToNumberOperationHint
pub fn binary_operation_hint_to_number_operation_hint(hint: BinaryOperationHint) -> Option<NumberOperationHint>;

/// BigInt 变体: 只接 kBigInt/kBigInt64
/// v8: src/compiler/js-type-hint-lowering.cc BinaryOperationHintToBigIntOperationHint
pub fn binary_operation_hint_to_big_int_operation_hint(hint: BinaryOperationHint) -> Option<BigIntOperationHint>;

// —— 对数基准 ——
// Reduce ${uniqueReduce.length} (cc 实现 ${impls.size}) / Number 映射 ${numPairs.length} 直接 + ${numFlagged.length} 旗标路
`;

const p = writeSrc("v8-compiler-frontend", "js_type_hint_lowering.rs.skel", body);
console.log(`✓ js-type-hint-lowering: Reduce ${uniqueReduce.length}(实现 ${impls.size}) / Number 映射 ${numPairs.length}+${numFlagged.length}(旗标) / 锚点 ${anchors.length + ccAnchors.length}`);
