#!/usr/bin/env bun
// tool/src/compiler/js-native-context-specialization.js
// 从 v8/src/compiler/js-native-context-specialization.h + js-native-context-specialization.cc
// 提取 Flag 位标志、Reduce*/Build* 方法面、ValueEffectControl 与原型链判定枚举
// 生成 v8-compiler-frontend/src/js_native_context_specialization.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/js-native-context-specialization.h");
const cc = readV8("src/compiler/js-native-context-specialization.cc");

// —— 1. 锚点断言 ——
const anchors = [
  ["class V8_EXPORT_PRIVATE JSNativeContextSpecialization final", "Reducer 类"],
  ["kBailoutOnUninitialized = 1u << 0,", "kBailoutOnUninitialized 1<<0"],
  ["static std::optional<size_t> GetMaxStringLength(JSHeapBroker* broker,", "GetMaxStringLength 静态工具"],
  ["class ValueEffectControl final {", "ValueEffectControl"],
  ["enum InferHasInPrototypeChainResult {", "原型链判定枚举"],
  ["ZoneUnorderedSet<IndirectHandle<String>, IndirectHandle<String>::hash,", "created_strings 成员"],
];
for (const [lit, name] of anchors) {
  if (!h.includes(lit)) throw new Error(`js-native-context-specialization.h 锚点缺失: ${name}`);
}
const ccAnchors = [
  ["Reduction JSNativeContextSpecialization::Reduce(Node* node) {", "Reduce 分派"],
  ["Reduction JSNativeContextSpecialization::ReduceElementAccess(", "ReduceElementAccess"],
  ["JSNativeContextSpecialization::BuildPropertyAccess(", "BuildPropertyAccess"],
  ["JSNativeContextSpecialization::BuildElementAccess(", "BuildElementAccess"],
  ["CanTreatHoleAsUndefined", "CanTreatHoleAsUndefined"],
];
for (const [lit, name] of ccAnchors) {
  if (!cc.includes(lit)) throw new Error(`js-native-context-specialization.cc 锚点缺失: ${name}`);
}

// —— 2. 真解析: Flag 位标志 ——
const flagSrc = h.match(/enum Flag \{([\s\S]*?)\};/);
const flags = [...flagSrc[1].matchAll(/(k\w+)\s*=\s*(1u << \d+|0u)/g)].map((m) => [m[1], m[2]]);
if (flags.length !== 2) throw new Error(`Flag 应 2 项: ${flags.length}`);

// —— 3. 真解析: InferHasInPrototypeChainResult 枚举值 ——
const protoEnum = h.match(/enum InferHasInPrototypeChainResult \{([^}]+)\}/);
const protoVals = protoEnum[1].split(",").map((s) => s.trim()).filter(Boolean);

// —— 4. 真解析: Reduce*/Build* 方法面 ——
const reduceFns = [...new Set([...h.matchAll(/Reduction (ReduceJS\w+)\(Node\* node\)/g)].map((m) => m[1]))];
const buildFns = [...new Set([...h.matchAll(/(?:std::optional<)?ValueEffectControl>?\s+(Build\w+)\(/g)].map((m) => m[1]))];
const inlineFns = [...new Set([...h.matchAll(/(?:Node\*|void) (Inline\w+)\(/g)].map((m) => m[1]))];

// —— 5. cc 实现计数 ——
const implReduce = new Set([...cc.matchAll(/^Reduction JSNativeContextSpecialization::(Reduce\w+)\(/gm)].map((m) => m[1]));
const implBuild = new Set([...cc.matchAll(/ValueEffectControl>?\s*\n\s*JSNativeContextSpecialization::(Build\w+)\(/g)].map((m) => m[1]));
if (reduceFns.length < 25 || buildFns.length < 5) throw new Error(`方法面异常: Reduce ${reduceFns.length} / Build ${buildFns.length}`);

const body = `${banner("src/compiler/js-native-context-specialization.js", "src/compiler/js-native-context-specialization.h + js-native-context-specialization.cc", "原生上下文特化: 全局/命名/键控属性访问按反馈特化")}
//! 最大特化 pass: LoadGlobal/StoreGlobal 折叠, LoadNamed/键控访问按类型反馈特化,
//! 元素访问子图构建 (含 TypedArray/RAB-GSAB), 访问器内联, MegaDOM。

bitflags! {
    /// v8: src/compiler/js-native-context-specialization.h JSNativeContextSpecialization::Flag
    pub struct Flags: u32 {
        const NO_FLAGS = 0;
${flags.filter((f) => f[1] !== "0u").map(([k, v]) => `        const ${k.replace(/^k/, "").toUpperCase()} = 1 << ${v.match(/\d+/)[0]};`).join("\n")}
    }
}

/// 原型链包含性判定三态
/// v8: src/compiler/js-native-context-specialization.h JSNativeContextSpecialization::InferHasInPrototypeChainResult
pub enum InferHasInPrototypeChainResult {
    IsInPrototypeChain,
    IsNotInPrototypeChain,
    MayBeInPrototypeChain,
}

/// 续接三元组
/// v8: src/compiler/js-native-context-specialization.h JSNativeContextSpecialization::ValueEffectControl
pub struct ValueEffectControl {
    pub value: NodeRef,
    pub effect: NodeRef,
    pub control: NodeRef,
}

pub struct JSNativeContextSpecialization { /* jsgraph/broker/flags/global_object/global_proxy/zone/shared_zone/type_cache/created_strings */ }

impl JSNativeContextSpecialization {
    /// 字符串常量拼接折叠上限 (公用, 单测可达)
    /// v8: src/compiler/js-native-context-specialization.h JSNativeContextSpecialization::GetMaxStringLength
    pub fn get_max_string_length(broker: &JSHeapBroker, node: &Node) -> Option<usize>;

    // —— Reduce 面 ${reduceFns.length} 个 (cc 实现 ${implReduce.size}) ——
${reduceFns.slice(0, 14).map((n) => `    /// v8: src/compiler/js-native-context-specialization.h JSNativeContextSpecialization::${n}\n    fn ${n.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase()}(&mut self, node: &Node) -> Reduction;`).join("\n")}
    // ... 其余 ${reduceFns.length - 14} 个同面 (含 ReduceElementAccess/ReducePropertyAccess/ReduceNamedAccess/ReduceGlobalAccess)

    // —— Build 面 ${buildFns.length} 个 (cc 实现 ${implBuild.size}) ——
${buildFns.map((n) => `    /// v8: src/compiler/js-native-context-specialization.h JSNativeContextSpecialization::${n}\n    fn ${n.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase()}(&mut self /*, args */) -> Option<ValueEffectControl>;`).join("\n")}

    // —— 访问器内联 (${inlineFns.length}) ——
${inlineFns.map((n) => `    /// v8: src/compiler/js-native-context-specialization.h JSNativeContextSpecialization::${n}\n    fn ${n.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase()}(&mut self /*, args */);`).join("\n")}

    /// 原型链三态判定
    /// v8: src/compiler/js-native-context-specialization.cc JSNativeContextSpecialization::InferHasInPrototypeChain
    fn infer_has_in_prototype_chain(&mut self, receiver: NodeRef, effect: Effect, prototype: HeapObjectRef) -> InferHasInPrototypeChainResult;
    /// hole→undefined 转换许可 (数组 protector 依赖)
    /// v8: src/compiler/js-native-context-specialization.cc JSNativeContextSpecialization::CanTreatHoleAsUndefined
    fn can_treat_hole_as_undefined(&mut self, receiver_maps: &[MapRef]) -> bool;
}

// —— 对数基准 ——
// Flag ${flags.length} / 原型三态 ${protoVals.length} (${protoVals.join(",")})
// Reduce ${reduceFns.length}(实现 ${implReduce.size}) / Build ${buildFns.length}(实现 ${implBuild.size}) / Inline ${inlineFns.length}
`;

const p = writeSrc("v8-compiler-frontend", "js_native_context_specialization.rs.skel", body);
console.log(`✓ js-native-context-specialization: Flag ${flags.length} / Reduce ${reduceFns.length}(${implReduce.size}) / Build ${buildFns.length}(${implBuild.size}) / Inline ${inlineFns.length} / 三态 ${protoVals.length} / 锚点 ${anchors.length + ccAnchors.length}`);
