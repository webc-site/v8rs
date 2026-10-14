#!/usr/bin/env bun
// tool/src/objects/allocation-site.h.js — 覆盖 v8/src/objects/allocation-site.{h,inl.h,tq}
//        + allocation-site-scopes.{h,inl.h}
// AllocationSite: PretenureDecision 枚举 / pretransition 常量 / 位域布局 / 决策名表
// → v8-objects-rest/src/allocation_site.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/allocation-site.h");
const inlSrc = readV8("src/objects/allocation-site-inl.h");
const objectsCc = readV8("src/objects/objects.cc"); // PretenureDecisionName 实现落点
const globalsSrc = readV8("src/common/globals.h");

// 1. 常量
const maxBytes = Number(hSrc.match(/static const uint32_t kMaximumArrayBytesToPretransition = (\d+) \* (\d+);/)?.slice(1).reduce((a, b) => a * b));
if (maxBytes !== 8192) throw new Error(`kMaximumArrayBytesToPretransition 对数不符: ${maxBytes}`);

// 2. PretenureDecision 枚举 (含哨兵 kLastPretenureDecisionValue = kTenure)
const enumBody = hSrc.match(/enum PretenureDecision \{([\s\S]*?)\n  \};/);
if (!enumBody) throw new Error("未找到 PretenureDecision 枚举");
const entries = [];
for (const m of enumBody[1].matchAll(/k(\w+) = (\d+)/g)) entries.push({ name: m[1], repr: Number(m[2]) });
const last = enumBody[1].match(/kLastPretenureDecisionValue = k(\w+)/)?.[1];
if (!last) throw new Error("未找到 kLastPretenureDecisionValue 哨兵");
// 哨兵为别名 (kLast... = kTenure), 只取数值项
const decisions = entries;
if (decisions.length !== 4) throw new Error(`PretenureDecision 条数不符: 期望 4, 得 ${decisions.length}`);
if (decisions.find((e) => e.name === last).repr !== 3) throw new Error("哨兵指向项应为 3 (kTenure)");

// 3. 决策名表 (objects.cc 侧 switch)
const nameBody = objectsCc.match(/const char\* AllocationSite::PretenureDecisionName\(PretenureDecision decision\) \{([\s\S]*?)\n\}/);
if (!nameBody) throw new Error("未找到 PretenureDecisionName 实现");
const nameMap = new Map();
for (const m of nameBody[1].matchAll(/case k(\w+):\s*return "([^"]*)";/g)) nameMap.set(m[1], m[2]);
if (nameMap.size !== 4) throw new Error(`决策名表条数不符: 期望 4, 得 ${nameMap.size}`);

// 4. 位域: transition_info 与 pretenure_data 两组 (BitField<T, off, size> 显式形)
const bitSpecs = [...hSrc.matchAll(/using (\w+)(?:Bits|Bit) = base::BitField<[^,]+, (\d+), (\d+)>;/g)]
  .map((m) => ({ name: m[1], offset: Number(m[2]), size: Number(m[3]) }));
if (bitSpecs.length !== 5) throw new Error(`位域数不符: 期望 5, 得 ${bitSpecs.length}`);
const byName = Object.fromEntries(bitSpecs.map((b) => [b.name, b]));
if (byName.ElementsKind.offset !== 0 || byName.ElementsKind.size !== 6) throw new Error("ElementsKindBits 布局不符");
if (byName.PretenureDecision.offset !== 26 || byName.PretenureDecision.size !== 3) throw new Error("PretenureDecisionBits 布局不符");
if (byName.DeoptDependentCode.offset !== 29) throw new Error("DeoptDependentCodeBit 偏移不符");

// 5. AllocationSiteUpdateMode (定义于 globals.h)
const updBody = globalsSrc.match(/enum class AllocationSiteUpdateMode \{([^}]*)\};/);
if (!updBody) throw new Error("未在 globals.h 找到 AllocationSiteUpdateMode");
const updModes = [...updBody[1].matchAll(/k(\w+)/g)].map((m) => m[1]);
if (updModes.length !== 2) throw new Error(`AllocationSiteUpdateMode 条数不符: ${updModes.length}`);

// 6. inl 侧无比例常量 (MakePretenureDecision 比率逻辑在调用方), 不提取

console.log(`AllocationSite 决策 ${decisions.length}, 位域 ${bitSpecs.length}, 名表 ${nameMap.size}`);

const pascal = (s) => s.replace(/(^|_)(\w)/g, (_, a, b) => b.toUpperCase());

let rust = banner(
  "src/objects/allocation-site.h.js",
  "src/objects/allocation-site.{h,inl.h,tq} + allocation-site-scopes.{h,inl.h}",
  "AllocationSite 预留决策位域与枚举 (AllocationMemento/遍历上下文骨架见尾注)",
);

rust += `
/// 预留决策 (原版 enum PretenureDecision)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum PretenureDecision {
`;
for (const e of decisions) rust += `  /// k${e.name}\n  ${pascal(e.name)} = ${e.repr},\n`;
rust += `}

impl PretenureDecision {
  /// 原版 kLastPretenureDecisionValue
  pub const LAST: Self = Self::${pascal(last)};

  /// 原版 PretenureDecisionName (objects.cc 侧 switch)
  #[inline]
  #[must_use]
  pub const fn to_str(self) -> &'static str {
    match self {
`;
for (const e of decisions) rust += `      Self::${pascal(e.name)} => "${nameMap.get(e.name)}",\n`;
rust += `    }
  }
}

/// 触发 pretransition 的数组最大字节数 (原版 kMaximumArrayBytesToPretransition)
pub const MAXIMUM_ARRAY_BYTES_TO_PRETRANSITION: u32 = ${maxBytes};

/// 更新模式 (原版 enum class AllocationSiteUpdateMode, 定义于 globals.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AllocationSiteUpdateMode {
  /// kUpdate
  Update = 0,
  /// kCheckOnly
  CheckOnly = 1,
}

/// transition_info 位域 (原版 ElementsKindBits / SpeculationDisabledBit)
pub struct TransitionInfoBits;
#[allow(dead_code)]
impl TransitionInfoBits {
`;
for (const n of ["ElementsKind", "SpeculationDisabled"]) {
  const b = byName[n];
  rust += `  /// ${n}: 位 ${b.offset}, 宽 ${b.size}\n  pub const ${pascal(n)}_SHIFT: u32 = ${b.offset};\n  pub const ${pascal(n)}_MASK: i32 = 0x${(((1 << b.size) - 1) << b.offset).toString(16)};\n`;
}
rust += `}

/// pretenure_data 位域 (原版 MementoFoundCountBits / PretenureDecisionBits / DeoptDependentCodeBit)
pub struct PretenureDataBits;
#[allow(dead_code)]
impl PretenureDataBits {
`;
for (const n of ["MementoFoundCount", "PretenureDecision", "DeoptDependentCode"]) {
  const b = byName[n];
  rust += `  /// ${n}Bits: 位 ${b.offset}, 宽 ${b.size}\n  pub const ${pascal(n)}_SHIFT: u32 = ${b.offset};\n  pub const ${pascal(n)}_MASK: i32 = 0x${((((1 << b.size) - 1) << b.offset) >>> 0).toString(16)};\n`;
}
rust += `}

/// memento 计数与决策迁移语义见原版 allocation-site-inl.h (比率逻辑在调用方)

/// 遍历上下文常量 (原版 AllocationSiteUsageContext::kCopying)
pub const ALLOCATION_SITE_USAGE_CONTEXT_COPYING: bool = true;

// —— 签名骨架 (原版公开接口, 供 Developer 补语义) ——
// class AllocationSiteContext { top(); current(); ShouldCreateMemento(); InitializeTraversal(); }
// class AllocationSiteUsageContext : AllocationSiteContext { EnterNewScope(); ExitScope(); }
// AllocationSite: Initialize / HasWeakNext / IsNested / GetAllocationType /
//   ResetPretenureDecision / MakePretenureDecision / DigestPretenuringFeedback /
//   DigestTransitionFeedback / ShouldTrack / CanTrack / IncrementMementoFoundCount
// AllocationMemento : Struct { allocation_site: AllocationSite } (tq)
// HasWeakNext 形态: AllocationSiteWithWeakNext 附加 weak_next 字段 (h 侧, 布局待接)

// —— 提取不变式断言 ——
const _: () = {
  assert!(MAXIMUM_ARRAY_BYTES_TO_PRETRANSITION == 8192);
  assert!(PretenureDecision::LAST as u8 == 3);
  assert!(PretenureDecision::Tenure.to_str() == "tenure");
  assert!(TransitionInfoBits::ELEMENTS_KIND_SHIFT == 0);
  assert!(PretenureDataBits::PRETENURE_DECISION_SHIFT == 26);
  assert!(PretenureDataBits::DEOPT_DEPENDENT_CODE_MASK == 0x20000000);
  assert!(ALLOCATION_SITE_USAGE_CONTEXT_COPYING);
};
`;

const p = writeSrc("v8-objects-rest", "allocation_site.rs", rust);
console.log(`written ${p}`);
