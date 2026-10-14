#!/usr/bin/env bun
// v8/src/flags/feature-flags.h → v8-flags/src/features.rs (特性枚举 + JSON 输出基线)
// 提取规则:
//   - #define FOREACH_<STAGE>_FEATURE_FLAG(...) 宏名 → FeatureStage 变体 (EXPERIMENTAL..SHIPPED)
//   - JS_FEATURE / WASM_FEATURE / INTERNAL_FEATURE 三槽宏 → FeatureKind
//   - IF_INTL_ENABLED / IF_REVEC_ENABLED / IF_SPARKPLUG_PLUS_ENABLED 条件宏 → 规范配置核对
// 数据表由 flag-definitions.h.js 生成 features_gen.rs; 本脚本产公开枚举与 JSON 输出
import { readV8, writeSrc, banner } from "../../lib.js";

const CRATE = "v8-flags";
const GEN = "src/flags/feature-flags.h.js";
const src = readV8("src/flags/feature-flags.h");

// ---- 提取: 阶段宏 (定义序 = 阶段序) ----
const stages = [...src.matchAll(/#define FOREACH_([A-Z_]+)_FEATURE_FLAG\(/g)].map((m) => m[1]);
if (stages.length !== 4) throw new Error(`FOREACH 阶段宏数异常: ${stages.length} ${stages}`);

// ---- 提取: 特性三槽宏 ----
const kinds = ["JS", "WASM", "INTERNAL"];
for (const k of kinds) {
  if (!src.includes(`${k}_FEATURE(`)) throw new Error(`${k}_FEATURE 宏未找到`);
}

// 阶段宏名 → JSON 组名 (原版 PrintFeatureFlagsJSON: EXPERIMENTAL/PRE_STAGED → in-progress, STAGED → staged, SHIPPED → shipping)
const STAGE2GROUP = { EXPERIMENTAL: "Inprogress", PRE_STAGED: "Inprogress", STAGED: "Staged", SHIPPED: "Shipping" };
// 变体名: SCREAMING_SNAKE → CamelCase
const camel = (s) => s.toLowerCase().split("_").map((w) => w[0].toUpperCase() + w.slice(1)).join("");

const out = [
  banner(GEN, "src/flags/feature-flags.h", "特性层: 阶段/种类枚举 + PrintFeatureFlagsJSON 输出 (数据在 features_gen.rs)"),
  `use strum::Display;`,
  ``,
  `use crate::features_gen::{Feature, FEATURES};`,
  ``,
  `/// 特性阶段 (原版 FOREACH_<STAGE>_FEATURE_FLAG 宏族; 定义序即阶段序)`,
  `#[derive(Copy, Clone, Debug, Display, PartialEq, Eq)]`,
  `pub enum FeatureStage {`,
  ...stages.map((s) => `  #[strum(serialize = "${s}")]\n  ${camel(s)},`),
  `}`,
  ``,
  `/// 特性种类 (原版三槽宏 JS_FEATURE / WASM_FEATURE / INTERNAL_FEATURE)`,
  `#[derive(Copy, Clone, Debug, Display, PartialEq, Eq)]`,
  `pub enum FeatureKind {`,
  ...kinds.map((k) => `  ${k[0] + k.slice(1).toLowerCase()},`),
  `}`,
  ``,
  `impl Feature {`,
  `  /// 阶段枚举视图`,
  `  pub fn stage(&self) -> FeatureStage {`,
  `    match self.stage {`,
  ...stages.map((s) => `      "${s}" => FeatureStage::${camel(s)},`),
  `      _ => FeatureStage::${camel(stages[0])}, // 生成表内阶段恒在枚举内`,
  `    }`,
  `  }`,
  ``,
  `  /// 种类枚举视图`,
  `  pub fn kind(&self) -> FeatureKind {`,
  `    match self.kind {`,
  ...kinds.map((k) => `      "${k.toLowerCase()}" => FeatureKind::${k[0] + k.slice(1).toLowerCase()},`),
  `      _ => FeatureKind::Js,`,
  `    }`,
  `  }`,
  `}`,
  ``,
  `/// 特性 JSON (原版 FlagList::PrintFeatureFlagsJSON; 供脚本清理测试文件中的特性 flag)`,
  `/// 输出形状与原版一致: js / harmony (harmony_ 前缀拆组) / wasm 三大组,`,
  `/// 各组含 in-progress (EXPERIMENTAL+PRE_STAGED) / staged / shipping 数组; internal 不输出`,
  `pub fn feature_flags_json() -> String {`,
  `  let mut s = String::from("{\\n");`,
  `  s.push_str("  \\"js\\": ");`,
  `  feature_group(&mut s, FeatureKind::Js, false);`,
  `  s.push_str(",\\n");`,
  `  s.push_str("  \\"harmony\\": ");`,
  `  feature_group(&mut s, FeatureKind::Js, true);`,
  `  s.push_str(",\\n");`,
  `  s.push_str("  \\"wasm\\": ");`,
  `  feature_group(&mut s, FeatureKind::Wasm, false);`,
  `  s.push_str("\\n}\\n");`,
  `  s`,
  `}`,
  ``,
  `/// 单组输出; harmony 组仅收 harmony_ 前缀名, 非 harmony 组排除之 (原版 strncmp("harmony_",8) 拆分)`,
  `fn feature_group(out: &mut String, kind: FeatureKind, harmony: bool) {`,
  `  out.push_str("{\\n");`,
  `  out.push_str("    \\"in-progress\\": ");`,
  `  names_array(out, kind, harmony, &[FeatureStage::${camel(stages[0])}, FeatureStage::${camel(stages[1])}]);`,
  `  out.push_str(",\\n");`,
  `  out.push_str("    \\"staged\\": ");`,
  `  names_array(out, kind, harmony, &[FeatureStage::${camel(stages[2])}]);`,
  `  out.push_str(",\\n");`,
  `  out.push_str("    \\"shipping\\": ");`,
  `  names_array(out, kind, harmony, &[FeatureStage::${camel(stages[3])}]);`,
  `  out.push_str("\\n  }");`,
  `}`,
  ``,
  `fn names_array(out: &mut String, kind: FeatureKind, harmony: bool, stages: &[FeatureStage]) {`,
  `  let mut first = true;`,
  `  out.push('[');`,
  `  for f in FEATURES.iter().filter(|f| {`,
  `    // internal 特性不进 JSON (原版 IGNORE_FEATURE 槽); 调用方只传 Js/Wasm`,
  `    f.kind() == kind && stages.contains(&f.stage()) && f.name.starts_with("harmony_") == harmony`,
  `  }) {`,
  `    if !first {`,
  `      out.push(',');`,
  `    }`,
  `    out.push_str("\\n      \\"");`,
  `    out.push_str(f.name);`,
  `    out.push('"');`,
  `    first = false;`,
  `  }`,
  `  if !first {`,
  `    out.push_str("\\n    ");`,
  `  }`,
  `  out.push(']');`,
  `}`,
  ``,
  `/// 全部特性 (定义序)`,
  `pub fn features() -> &'static [Feature] {`,
  `  FEATURES`,
  `}`,
  ``,
];

writeSrc(CRATE, "features.rs", out.join("\n") + "\n");
console.log(`features.rs: 阶段 ${stages.join("/")} → ${[...new Set(stages.map((s) => STAGE2GROUP[s]))].join("/")}`);
