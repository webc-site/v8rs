#!/usr/bin/env bun
// tool/src/wasm/wasm-features.cc.js
// 机械提取 v8/src/wasm/wasm-features.cc: FromFlags/FromContext 的特性→flag 映射表与骨架。
// 特性枚举本体已由 tool/src/wasm/wasm-features.js 产出 (wasm_features.rs), 本脚本做 .cc 侧规则。

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/wasm-features.h");
const ccSrc = readV8("src/wasm/wasm-features.cc");
// 已有产物用于对数交叉核对 (只读, 不改)
const featuresRs = readV8("../v8-wasm-core/src/wasm_features.rs");

// 叶子宏定义于 src/flags/feature-flags.h (wasm-features.h 只做组合)
const flagsH = readV8("src/flags/feature-flags.h");

// 展开 FOREACH_*_FEATURE_FLAG 叶子宏, 还原 FOREACH_WASM_FEATURE_FLAG 顺序
function macroBody(src, name) {
  const lines = src.split("\n");
  for (let i = 0; i < lines.length; i++) {
    const head = lines[i].match(new RegExp(`^#define\\s+${name}\\(`));
    if (!head) continue;
    // 参数表可能跨行: 先累积到圆括号配平, 再合并续行
    let acc = lines[i];
    const bal = (t) => {
      let n = 0;
      for (const ch of t) {
        if (ch === "(") n++;
        else if (ch === ")") n--;
      }
      return n;
    };
    let depth = bal(acc);
    while (depth > 0 && i + 1 < lines.length) {
      acc += " " + lines[++i];
      depth = bal(acc);
    }
    // acc 现含完整参数表; 之后的续行是宏体
    let body = "";
    while (acc.endsWith("\\")) {
      body += " " + (lines[++i] ?? "").trim();
      acc = lines[i] ?? "";
    }
    // 最后一行已在循环内追加, 不再重复
    return body.replace(/\/\*[\s\S]*?\*\//g, " ");
  }
  throw new Error(`未找到宏 ${name}`);
}

const FLAG_LEAVES = [
  "FOREACH_EXPERIMENTAL_FEATURE_FLAG",
  "FOREACH_PRE_STAGED_FEATURE_FLAG",
  "FOREACH_STAGED_FEATURE_FLAG",
  "FOREACH_SHIPPED_FEATURE_FLAG",
];

// 每个叶子宏形如 #define FOREACH_X(V1, V2, V3) ... 含 V(feat) 条目
const feats = [];
for (const leaf of FLAG_LEAVES) {
  const body = macroBody(flagsH, leaf);
  // 叶子宏体以 WASM_FEATURE(feat, "desc") 形式书写 (组合时 WASM_FEATURE 绑定为 V)
  const leafFeats = [...body.matchAll(/WASM_FEATURE\(\s*(\w+)\s*,\s*"[^"]*"\s*\)/g)].map((m) => m[1]);
  if (leafFeats.length === 0) throw new Error(`${leaf} 展开为空`);
  feats.push(...leafFeats);
}
if (feats.length < 10) throw new Error(`FOREACH_WASM_FEATURE_FLAG 展开 ${feats.length} 条, 异常偏少`);
{
  const s = new Set(feats);
  if (s.size !== feats.length) throw new Error("特性名重复");
  if (!feats.includes("custom_descriptors") || !feats.includes("legacy_eh")) {
    throw new Error("特性锚点缺失: custom_descriptors / legacy_eh");
  }
}

// 交叉核对: 已有 wasm_features.rs 的 WasmEnabledFeature 变体数应一致
const enumBlock = featuresRs.match(/pub enum WasmEnabledFeature\s*\{([\s\S]*?)\n\}/);
if (!enumBlock) throw new Error("wasm_features.rs 中未找到 WasmEnabledFeature (对数基准缺失)");
const rsVariants = [...enumBlock[1].matchAll(/^\s*([A-Z]\w+)\s*[=,]/gm)].map((m) => m[1]);
if (rsVariants.length !== feats.length) {
  throw new Error(`对数不符: 头侧宏 ${feats.length} vs 既有枚举 ${rsVariants.length}`);
}
console.log(`FOREACH_WASM_FEATURE_FLAG 展开 ${feats.length} 条, 与 wasm_features.rs 一致`);

// .cc 锚点
for (const a of ["WasmEnabledFeatures::FromFlags", "WasmEnabledFeatures::FromIsolate", "WasmEnabledFeatures::FromContext", "wasm_jitless", "custom_descriptors"]) {
  if (!ccSrc.includes(a)) throw new Error(`.cc 锚点缺失: ${a}`);
}

let rust = banner(
  "src/wasm/wasm-features.cc.js",
  "src/wasm/wasm-features.{h,cc}",
  "特性启用集 FromFlags/FromContext 规则表 + 骨架 (枚举本体在 wasm_features.rs)",
);

rust += `
use crate::wasm_features::{WasmEnabledFeature, WasmEnabledFeatures};

/// 特性 → v8 flag 名映射 (对应 CHECK_FEATURE_FLAG 展开中的 v8_flags.wasm_##feat)
/// 顺序 = FOREACH_WASM_FEATURE_FLAG 展开序, 共 ${feats.length} 条
pub static FEATURE_FLAG_NAMES: &[(WasmEnabledFeature, &'static str)] = &[
`;

for (const f of feats) {
  rust += `  (WasmEnabledFeature::${f.split("_").map((w) => w.charAt(0).toUpperCase() + w.slice(1)).join("")}, "wasm_${f}"),\n`;
}

rust += `];

/// jitless (DrumBrake) 模式下强制启用的特性 (对应 FromFlags 中 V8_ENABLE_DRUMBRAKE 分支)
pub static JITLESS_FORCED_FEATURES: &[WasmEnabledFeature] = &[
  WasmEnabledFeature::LegacyEh,
];

/// 上下文侧额外启用 (对应 FromContext: IsWasmCustomDescriptorsEnabled)
pub static CONTEXT_GATED_FEATURES: &[WasmEnabledFeature] = &[
  WasmEnabledFeature::CustomDescriptors,
];

/// 从 v8_flags 构建 (对应 WasmEnabledFeatures::FromFlags, 骨架)
pub fn from_flags(_flag_enabled: impl Fn(&str) -> bool, jitless: bool) -> WasmEnabledFeatures {
  let mut features = WasmEnabledFeatures::none();
  if jitless {
    for f in JITLESS_FORCED_FEATURES {
      features.add(*f);
    }
  }
  for (feat, flag) in FEATURE_FLAG_NAMES {
    // 原版: if (!v8_flags.wasm_jitless && v8_flags.wasm_##feat) features.Add(feat)
    let _ = (feat, flag);
    unimplemented!("骨架: 逐 flag 读取待 Developer 修订");
  }
  features
}

/// 从 Isolate 构建 (对应 WasmEnabledFeatures::FromIsolate, 骨架)
pub fn from_isolate(_isolate: &core::ffi::c_void) -> WasmEnabledFeatures {
  unimplemented!("骨架: 参照原版 FromIsolate→FromContext 修订")
}

/// 从 Context 构建 (对应 WasmEnabledFeatures::FromContext, 骨架)
pub fn from_context(_isolate: &core::ffi::c_void, _context: &core::ffi::c_void) -> WasmEnabledFeatures {
  unimplemented!("骨架: 参照原版 FromContext (custom_descriptors 上下文门控) 修订")
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(FEATURE_FLAG_NAMES.len() == ${feats.length});
  assert!(JITLESS_FORCED_FEATURES.len() == 1);
  assert!(CONTEXT_GATED_FEATURES.len() == 1);
};
`;

const p = writeSrc("v8-wasm-core", "wasm_features_cc.rs", rust);
console.log(`written ${p}`);
