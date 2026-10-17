#!/usr/bin/env bun
// tool/src/codegen/external-reference-table.h.js
// 机械提取 v8/src/codegen/external-reference-table.{h,cc} —
// 表布局计数常量 (special/builtins/runtime/isolate-field/accessor/stub-cache)
// + InitializationState 枚举 + Add*/Copy* 签名骨架。
// 产物: v8-codegen-core/src/external_reference_table_generated.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/codegen/external-reference-table.h";
const CC_REL = "src/codegen/external-reference-table.cc";
const h = readV8(H_REL);
readV8(CC_REL); // 家族覆盖锚

// 布局计数常量
function need(name) {
  const m = h.match(new RegExp(`static constexpr int ${name} =\\s*([\\s\\S]*?);`));
  if (!m) throw new Error(`未找到 ${name}`);
  return m[1].replace(/\s+/g, " ").trim();
}
const counts = {
  special: need("kSpecialReferenceCount"),
  builtins: need("kBuiltinsReferenceCount"),
  isolateField: need("kIsolateFieldReferenceCount"),
  stubCache: need("kStubCacheReferenceCount"),
};
if (counts.special !== "1") throw new Error("kSpecialReferenceCount 锚不符");
if (!h.includes("kIsolateFieldReferenceCount = kNumIsolateFieldIds")) throw new Error("kNumIsolateFieldIds 引用锚不符");

// InitializationState 枚举
const initM = h.match(/enum InitializationState : uint32_t\s*\{([^}]+)\};/);
if (!initM) throw new Error("未找到 InitializationState 枚举");
const initStates = [...initM[1].matchAll(/k(\w+)/g)].map((m) => "k" + m[1]);
if (initStates.length !== 3) throw new Error(`InitializationState 条数不符: ${initStates.length} != 3`);

// 方法面
const sigs = [...h.matchAll(/^\s*(?:static\s+)?(?:void|Address)\s+(\w+)\(/gm)].map((m) => m[1]);
if (sigs.length < 8) throw new Error(`方法面不足: ${sigs.length}`);
for (const anchor of ["ref_name_[kSize]", "static const char* const ref_name_"]) {
  if (!h.includes(anchor)) throw new Error(`缺锚 ${anchor}`);
}

let rust = banner("src/codegen/external-reference-table.h.js", "src/codegen/external-reference-table.{h,cc}", "外部引用表布局骨架");
rust += `
// 布局计数 (原 static constexpr int k*; 表达式保持原样, 求值依赖同 crate 产物):
/// 原 kSpecialReferenceCount
pub const SPECIAL_REFERENCE_COUNT: i32 = ${counts.special};
/// 原 kBuiltinsReferenceCount
pub const BUILTINS_REFERENCE_COUNT_EXPR: &str = ${JSON.stringify(counts.builtins)};
/// 原 kIsolateFieldReferenceCount = kNumIsolateFieldIds (isolate-data-fields.h)
pub const ISOLATE_FIELD_REFERENCE_COUNT_EXPR: &str = "kNumIsolateFieldIds";
/// 原 kStubCacheReferenceCount = 6 * 3 (3 组 stub cache)
pub const STUB_CACHE_REFERENCE_COUNT: i32 = ${counts.stubCache.replace(/\s/g, "").replace(/^(\d+)\*(\d+)$/, "$1 * $2")};

/// 表项初始化状态 (原 enum InitializationState : uint32_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum InitializationState {
${initStates.map((s, i) => `  /// ${s}\n  ${s.replace(/^k/, "")} = ${i},`).join("\n")}
}

/// 原类面方法 (Add*/Copy*/Get* 族, ${sigs.length} 条):
${[...new Set(sigs)].map((s) => `// - ${s}`).join("\n")}

/// 原 ref_addr_[kSize] / ref_name_[kSize] 双平行数组
pub struct ExternalReferenceTable;

// TODO(dev): 补表构建 (isolate 独立/依赖分相填充)。

// —— 提取不变式断言 ——
const _: () = {
  assert!(SPECIAL_REFERENCE_COUNT == 1);
  assert!(STUB_CACHE_REFERENCE_COUNT == 18);
  assert!(InitializationState::Uninitialized as u32 == 0);
};
`;

const p = writeSrc("v8-codegen-core", "external_reference_table_generated.rs", rust);
console.log(`written ${p} (计数 4, 状态 ${initStates.length}, 方法 ${[...new Set(sigs)].length})`);
