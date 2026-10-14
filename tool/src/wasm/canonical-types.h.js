#!/usr/bin/env bun
// tool/src/wasm/canonical-types.h.js
// 机械提取 v8/src/wasm/canonical-types.h: 预定义规范类型索引表与容量常量

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/canonical-types.h");

const maxMatch = hSrc.match(/static constexpr size_t kMaxCanonicalTypes = 2 \* kV8MaxWasmTypes;/);
if (!maxMatch) throw new Error("kMaxCanonicalTypes 定义漂移");

// 预定义类型索引: static constexpr CanonicalTypeIndex kPredefinedXxx{N};
const predefined = [...hSrc.matchAll(
  /static constexpr CanonicalTypeIndex (kPredefined\w+)\{(\d+)\};/g,
)].map((m) => ({ name: m[1], index: Number(m[2]) }));

const countMatch = hSrc.match(/static constexpr uint32_t kNumberOfPredefinedTypes = (\d+);/);
if (!countMatch) throw new Error("未找到 kNumberOfPredefinedTypes");
const numberOfPredefined = Number(countMatch[1]);
if (predefined.length !== numberOfPredefined) {
  throw new Error(`预定义索引 ${predefined.length} 条 != kNumberOfPredefinedTypes ${numberOfPredefined}`);
}
// 连续性对数: 索引必须 0..N-1 连续
predefined.forEach((p, i) => {
  if (p.index !== i) throw new Error(`预定义索引断裂 @${i}: ${p.name}=${p.index}`);
});
if (predefined[0].name !== "kPredefinedArrayI8Index") throw new Error("首条锚点漂移");

console.log(`预定义类型 ${predefined.length} 条 (0..${numberOfPredefined - 1} 连续)`);

let rust = banner(
  "src/wasm/canonical-types.h.js",
  "src/wasm/canonical-types.h",
  "TypeCanonicalizer 预定义规范类型索引表 + 容量常量",
);

rust += `
use crate::value_type::CanonicalTypeIndex;

/// 规范类型容量上限 (对应 kMaxCanonicalTypes = 2 * kV8MaxWasmTypes)
pub const MAX_CANONICAL_TYPES: usize = 2 * crate::wasm_limits::K_V8_MAX_WASM_TYPES;

/// 预定义规范类型索引 (对应 TypeCanonicalizer 内 static constexpr, 连续 0..30)
#[rustfmt::skip]
pub static PREDEFINED_TYPE_INDICES: &[(&str, CanonicalTypeIndex)] = &[
`;

for (const p of predefined) {
  rust += `  ("${p.name}", CanonicalTypeIndex::new(${p.index})),\n`;
}

rust += `];

/// 预定义类型数量 (对应 kNumberOfPredefinedTypes)
pub const NUMBER_OF_PREDEFINED_TYPES: u32 = ${numberOfPredefined};

/// 具名常量直出 (便于按名引用, 与原版同名)
pub mod predefined {
  use crate::value_type::CanonicalTypeIndex;
`;

for (const p of predefined) {
  rust += `  /// ${p.name}\n  pub const ${p.name.replace(/^kPredefined/, "")}: CanonicalTypeIndex = CanonicalTypeIndex::new(${p.index});\n`;
}

rust += `}

// —— 提取不变式断言 ——
const _: () = {
  assert!(PREDEFINED_TYPE_INDICES.len() == ${numberOfPredefined});
  assert!(NUMBER_OF_PREDEFINED_TYPES == ${numberOfPredefined});
  assert!(predefined::ARRAY_I8_INDEX.index == 0);
  assert!(predefined::ARRAY_I8_SHARED_INDEX.index == 1);
  assert!(predefined::ARRAY_I16_INDEX.index == 2);
  assert!(predefined::ARRAY_EXTERN_REF_INDEX.index == 4);
  assert!(predefined::SIG_INDEX_E_I.index == 6);
  assert!(predefined::CONFIGURE_ALL.index == 30);
  assert!(predefined::CONFIGURE_ALL.index + 1 == NUMBER_OF_PREDEFINED_TYPES);
};
`;

const p = writeSrc("v8-wasm-core", "canonical_types.rs", rust);
console.log(`written ${p}`);
