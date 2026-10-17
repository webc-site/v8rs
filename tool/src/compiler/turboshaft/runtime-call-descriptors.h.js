#!/usr/bin/env bun
// tool/src/compiler/turboshaft/runtime-call-descriptors.h.js
// 机械提取 v8/src/compiler/turboshaft/runtime-call-descriptors.h
// → v8-turboshaft/src/runtime_call_descriptors.rs.skel
//
// 提取面: runtime::Descriptor 逐条登记 (Runtime 函数名 / 参数表 / 返回
/// 类型 / 可 lazy deopt / Operator properties / 自定义 OpEffects)。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/runtime-call-descriptors.h");

// 1. 哨兵
for (const [re, name] of [
  [/struct runtime : CallDescriptorBuilder \{/, "runtime 命名空间"],
  [/using Void = V<Any>;/, "Void 哨兵"],
  [/using Never = V<Any>;/, "Never 哨兵"],
  [/static constexpr OpEffects kEffects = OpEffects\(\)\.CanCallAnything\(\);/, "默认 CanCallAnything"],
  [/#define ARG\(type, name\) DEFINE_TURBOSHAFT_CALL_DESCRIPTOR_ARG\(type, name\)/, "ARG 宏"],
  [/Linkage::GetRuntimeCallDescriptor\(/, "Linkage 装配"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 2. Descriptor 条目解析
const entries = [...h.matchAll(
  /struct (\w+)(?:\s*|\s*\n\s*): public Descriptor<\1> \{([\s\S]*?)\n  \};/g,
)].map((m) => {
  const b = m[2];
  const fn = b.match(/kFunction =\s*\n?\s*Runtime::k(\w+)/)?.[1] ?? throwM(m[1]);
  const args = [...b.matchAll(/ARG\(V<(\w+)>, (\w+)\)/g)].map((a) => `${a[2]}: V<${a[1]}>`);
  const noArgs = /using Arguments = NoArguments;/.test(b);
  const ret = b.match(/using returns_t = (?:V<(\w+)>|(\w+));/)?.slice(1).find(Boolean) ?? throwM(m[1] + " ret");
  const deopt = /kCanTriggerLazyDeopt = (\w+)/.exec(b)?.[1];
  if (deopt === undefined) throw new Error(`${m[1]} 缺 kCanTriggerLazyDeopt`);
  const props = b.match(/kProperties = (Operator::\w+)/)?.[1] ?? "";
  const effects = b.match(/kEffects =\s*\n?\s*OpEffects\(\)([\s\S]*?);/)?.[1]?.replace(/\s+/g, " ").trim();
  return { name: m[1], fn, args, noArgs, ret, deopt, props, effects };
});
if (entries.length !== 31) throw new Error(`Descriptor ${entries.length} != 31`);
for (const must of ["Abort", "StackGuard", "ToString", "MajorGCForCompilerTesting"]) {
  if (!entries.some((e) => e.name === must)) throw new Error(`Descriptor 缺 ${must}`);
}
const lazyTrue = entries.filter((e) => e.deopt === "true").length;

function throwM(w) { throw new Error(`解析失败: ${w}`); }

let rust = banner(
  "src/compiler/turboshaft/runtime-call-descriptors.h.js",
  "src/compiler/turboshaft/runtime-call-descriptors.h",
  "运行时调用描述符表",
);

rust += `
/// 原版 runtime::Descriptor — 每条对应一个 Runtime::k 函数的 Turboshaft
/// 调用描述符; Create() 经 Linkage::GetRuntimeCallDescriptor 装配,
/// kCanTriggerLazyDeopt && caller_can_deopt 时挂 kNeedsFrameState。
/// 默认 kEffects = CanCallAnything; Void/Never 以 V<Any> 哨兵表达。
/// v8: src/compiler/turboshaft/runtime-call-descriptors.h runtime
pub struct RuntimeCallDescriptor {
  /// 原版 kFunction (Runtime::kXxx)
  pub function: RuntimeFunctionId,
  /// 原版 Arguments (ARG 展开)
  pub args: &'static [(&'static str, Type)],
  /// 原版 returns_t
  pub returns: Type,
  /// 原版 kCanTriggerLazyDeopt
  pub can_trigger_lazy_deopt: bool,
  /// 原版 kProperties (Operator::kNoProperties / kNoWrite / kFoldable / ...)
  pub properties: OperatorProperties,
  /// 原版自定义 kEffects (None = 默认 CanCallAnything)
  pub effects: Option<OpEffects>,
}

pub const RUNTIME_CALL_DESCRIPTORS: &[RuntimeCallDescriptor] = &[
${entries.map((e) => `  /// ${e.name}: Runtime::k${e.fn}, 返回 V<${e.ret}>, lazy_deopt=${e.deopt}${e.props ? `, ${e.props}` : ""}${e.effects ? `, effects=${e.effects}` : ""}${e.noArgs ? ", 无参" : ""}
  RuntimeCallDescriptor { function: RuntimeFunctionId::${e.fn}, args: &[${e.args.map((a) => { const [n, t] = a.split(": V<"); return `("${n}", Type::${t.replace(">", "")})`; }).join(", ")}], returns: Type::${e.ret}, can_trigger_lazy_deopt: ${e.deopt}, properties: _PROPS, effects: ${e.effects ? "_EFFECTS" : "None"} },`).join("\n")}
];
`;

const p = writeSrc("v8-turboshaft", "runtime_call_descriptors.rs.skel", rust);
console.log(`written ${p}`);
console.log(`Descriptor=${entries.length} lazyDeopt=true:${lazyTrue} 自定义effects=${entries.filter((e) => e.effects).length}`);
