#!/usr/bin/env bun
// tool/src/compiler/turboshaft/builtin-call-descriptors.h.js
// 机械提取 v8/src/compiler/turboshaft/builtin-call-descriptors.h
// → v8-turboshaft/src/builtin_call_descriptors.rs.skel
//
// 提取面: Descriptor 子类表 (直接 struct 98 个 + GENERIC_BINOP_LIST 18 +
// GENERIC_UNOP_LIST 4 + DECLARE_NUMBER_TO_STRING 4 = 124 builtin 描述符)
// + 属性五元组 (kFunction/kCanTriggerLazyDeopt/kNeedsContext/kProperties/
// kEffects) 约定。语义实现禁编造。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/builtin-call-descriptors.h");
// GENERIC_*_LIST 宏定义在 operations.h (跨文件引用, 完整字面量路径)
const ops = readV8("src/compiler/turboshaft/operations.h");

// 1. 直接 struct Descriptor 子类 (保序)
const descs = [...h.matchAll(/struct (\w+)\s*: public Descriptor<\1>/g)].map((m) => m[1]);
if (descs.length !== 118) throw new Error(`直接 Descriptor 子类 ${descs.length} != 118`);

// 2. 宏展开族
function macroList(src, name, expect) {
  const defM = src.match(new RegExp(`#define ${name}\\(V\\)([\\s\\S]*?)#define`));
  if (!defM) throw new Error(`operations.h 未找到 ${name}`);
  const items = [...defM[1].matchAll(/V\((\w+)\)/g)].map((m) => m[1]);
  if (items.length !== expect) throw new Error(`${name} ${items.length} != ${expect}`);
  return items;
}
const binops = macroList(ops, "GENERIC_BINOP_LIST", 18);
const unops = macroList(ops, "GENERIC_UNOP_LIST", 4);

// DECLARE_NUMBER_TO_STRING 展开调用
const n2sCalls = [...h.matchAll(/^\s*DECLARE_NUMBER_TO_STRING\((\w+), (\w+)\)/gm)].map((m) => [m[1], m[2]]);
if (n2sCalls.length !== 4) throw new Error(`NUMBER_TO_STRING ${n2sCalls.length} != 4`);

// h 内确认三处宏调用锚
for (const [frag, label] of [
  ["GENERIC_BINOP_LIST(DECL_GENERIC_BINOP)", "GENERIC_BINOP_LIST 展开"],
  ["GENERIC_UNOP_LIST(DECL_GENERIC_UNOP)", "GENERIC_UNOP_LIST 展开"],
  ["DECLARE_NUMBER_TO_STRING(Int32, Word32)", "NUMBER_TO_STRING 首项"],
]) {
  if (!h.includes(frag)) throw new Error(`宏调用缺失: ${label}`);
}

// 3. Descriptor 基类与属性五元组约定
for (const [re, label] of [
  [/struct builtin : CallDescriptorBuilder/, "builtin 基类 (CallDescriptorBuilder)"],
  [/template <typename Derived>\s*\n\s*struct Descriptor\s*\{/, "Descriptor<Derived> CRTP 模板"],
  [/static constexpr auto kFunction = Builtin::k\w+;/, "kFunction 约定"],
  [/kCanTriggerLazyDeopt/, "kCanTriggerLazyDeopt"],
  [/kNeedsContext/, "kNeedsContext"],
  [/kNeedsFrameState/, "kNeedsFrameState"],
  [/static constexpr Operator::Properties kProperties/, "kProperties"],
  [/static constexpr OpEffects kEffects/, "kEffects"],
  [/OpEffects base_effects/, "base_effects"],
  [/CanCallAnything\(\)/, "base_effects.CanCallAnything"],
  [/struct Arguments : ArgumentsBase/, "Arguments : ArgumentsBase (来自 call-descriptors-util.h)"],
  [/WasmAllocateDescriptorStruct/, "wasm 分配描述符区"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${label}`);
}

// 4. 生成 Rust
const total = descs.length + binops.length + unops.length + n2sCalls.length;
let rust = banner(
  "src/compiler/turboshaft/builtin-call-descriptors.h.js",
  "src/compiler/turboshaft/builtin-call-descriptors.h",
  "builtin 调用描述符表: 124 描述符属性登记",
);
rust += `
/// v8: src/compiler/turboshaft/builtin-call-descriptors.h builtin::Descriptor
///
/// 属性五元组约定 (每描述符):
/// - kFunction: Builtin 枚举
/// - kCanTriggerLazyDeopt / kNeedsFrameState / kNeedsContext: bool
/// - kProperties: Operator::Properties
/// - kEffects: OpEffects (base_effects 组合)
///
/// 表构成: 直接 struct ${descs.length} (含 26 跨行书写) + GENERIC_BINOP ${binops.length} +
/// GENERIC_UNOP ${unops.length} + NUMBER_TO_STRING ${n2sCalls.length} = ${total}
pub const BUILTIN_DESCRIPTOR_NAMES: &[&str] = &[
${descs.map((d) => `  "${d}",`).join("\n")}
  // —— 宏展开族 ——
${binops.map((d) => `  "${d}",`).join("\n")}
${unops.map((d) => `  "${d}",`).join("\n")}
${n2sCalls.map(([n]) => `  "${n}ToString",`).join("\n")}
];

/// v8: src/compiler/turboshaft/builtin-call-descriptors.h Descriptor (CRTP)
pub struct BuiltinCallDescriptor {
  /// kFunction
  pub function: u16,
  /// kCanTriggerLazyDeopt
  pub can_trigger_lazy_deopt: bool,
  /// kNeedsFrameState
  pub needs_frame_state: bool,
  /// kNeedsContext
  pub needs_context: bool,
  /// kEffects (kProperties 由 effects 派生)
  pub effects: crate::operations::OpEffects,
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(BUILTIN_DESCRIPTOR_NAMES.len() == ${total});
};
`;

const p = writeSrc("v8-turboshaft", "builtin_call_descriptors.rs.skel", rust);
console.log(`written ${p}`);
console.log(`直接=${descs.length} + BINOP宏=${binops.length} + UNOP宏=${unops.length} + N2S=${n2sCalls.length} = ${total}`);
