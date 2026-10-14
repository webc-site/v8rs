#!/usr/bin/env bun
// tool/src/sandbox/compactible-external-entity-table.h.js
// 机械提取 v8/src/sandbox/compactible-external-entity-table.{h,-inl.h} —
// 压缩迁移层: EvacuateMarkMode / CompactionOutcome 枚举 + 模板类/Space 方法面。
// 产物: v8-sandbox/src/compactible_external_entity_table.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/sandbox/compactible-external-entity-table.h";
const INL_REL = "src/sandbox/compactible-external-entity-table-inl.h";
const h = readV8(H_REL);
readV8(INL_REL); // 家族覆盖锚

// 1. EvacuateMarkMode 枚举 (自动编号)
const emmM = h.match(/enum class EvacuateMarkMode\s*\{([^}]+)\};/);
if (!emmM) throw new Error("未找到 EvacuateMarkMode 枚举");
const emm = emmM[1]
  .replace(/\/\/[^\n]*/g, "")
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);
if (emm.length !== 3) throw new Error(`EvacuateMarkMode 条数不符: ${emm.length} != 3`);

// 2. CompactionOutcome 枚举 (显式值)
const outM = h.match(/enum class ExternalEntityTableCompactionOutcome\s*\{([^}]+)\};/);
if (!outM) throw new Error("未找到 CompactionOutcome 枚举");
const outcomes = [...outM[1].matchAll(/k(\w+)\s*=\s*(\d+)/g)].map((m) => ({ name: m[1], value: Number(m[2]) }));
if (outcomes.length !== 2 || outcomes[0].value !== 0 || outcomes[1].value !== 2) {
  throw new Error(`CompactionOutcome 条目异常: ${JSON.stringify(outcomes)}`);
}

// 3. 模板类声明锚 + 方法面
if (!/template <typename Entry, size_t size>\s*\nclass V8_EXPORT_PRIVATE CompactibleExternalEntityTable\s*:\s*public ExternalEntityTable<Entry, size>/.test(h)) {
  throw new Error("模板类声明锚不符");
}
if (!h.includes("static constexpr bool kSupportsCompaction = true;")) throw new Error("缺 kSupportsCompaction 锚");
const classM = h.match(/class V8_EXPORT_PRIVATE CompactibleExternalEntityTable\s*:\s*public ExternalEntityTable<Entry, size>\s*\{([\s\S]*?)\n\};/);
if (!classM) throw new Error("未提取类体");
const methods = [...classM[1].matchAll(/^\s*(?:inline\s+)?[\w:<>,&*\s]+?\b(\w+)\(/gm)].map((m) => m[1]);
if (methods.length < 8) throw new Error(`方法面不足: ${methods.length}`);
const spaceM = classM[1].match(/struct Space : public Base::Space\s*\{([\s\S]*?)\n  \};/);
if (!spaceM) throw new Error("未提取 Space 结构体");
const spaceMethods = [...spaceM[1].matchAll(/^\s*(?:inline\s+)?[\w:<>,&*\s]+?\b(\w+)\(/gm)].map((m) => m[1]);
if (spaceMethods.length < 4) throw new Error(`Space 方法面不足: ${spaceMethods.length}`);

let rust = banner("src/sandbox/compactible-external-entity-table.h.js", "src/sandbox/compactible-external-entity-table.{h,-inl.h}", "CompactibleExternalEntityTable 枚举与骨架");
rust += `
/// 迁移时的标记位处理 (原 enum class EvacuateMarkMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EvacuateMarkMode {
${emm.map((n, i) => `  /// ${n}\n  ${n.replace(/^k/, "")} = ${i},`).join("\n")}
}

/// 表压缩结果 (供 ExternalPointerTableCompactionOutcome 直方图)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ExternalEntityTableCompactionOutcome {
  /// 压缩成功
  Success = 0,
  /// 中止 (freelist 过短); 值 1 部分成功已弃用
  Aborted = 2,
}

/// 原 template <typename Entry, size_t size> class CompactibleExternalEntityTable
/// : public ExternalEntityTable (基类由 tool/src/sandbox/external-entity-table.js 覆盖)
pub const SUPPORTS_COMPACTION: bool = true;

/// 原 CompactionResult
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompactionResult {
  pub start_of_evacuation_area: u32,
  pub success: bool,
}

/// 原 Space (表空间 + 压缩状态) 方法面 (${spaceMethods.length} 条):
${[...new Set(spaceMethods)].map((s) => `// - ${s}`).join("\n")}

/// 模板类方法面 (${[...new Set(methods)].length} 条):
${[...new Set(methods)].map((s) => `// - ${s}`).join("\n")}

pub struct CompactibleExternalEntityTable;

// TODO(dev): 依 ExternalEntityTable 基类补 Mark/Sweep/Compact 实现
// (inl 侧 ${readV8(INL_REL).split("\n").length} 行语义面登记跳过)。

// —— 提取不变式断言 ——
const _: () = {
  assert!(ExternalEntityTableCompactionOutcome::Success as u32 == 0);
  assert!(ExternalEntityTableCompactionOutcome::Aborted as u32 == 2);
};
`;

const p = writeSrc("v8-sandbox", "compactible_external_entity_table.rs", rust);
console.log(`written ${p} (EvacuateMarkMode 3, Outcome 2, 方法 ${[...new Set(methods)].length}+${[...new Set(spaceMethods)].length})`);
