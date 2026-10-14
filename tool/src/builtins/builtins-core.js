#!/usr/bin/env bun
// tool/src/builtins/builtins-core.js — v8-builtins-core 基线生成
// 来源 25 文件: builtins.h/inl/cc/descriptors/definitions/constructor.h/
// iterator.h/inl/promise.h/utils.h/inl/superspread/torque-csa-includes/
// constants-table-builder.cc+h/setup-builtins-internal/generate-bytecodes-list/
// profile-data-reader.cc+h/base.tq/internal.tq/internal-coverage/torque-internal/
// frames.tq/frame-arguments/cast.tq/convert.tq
// → v8-builtins-core/src/{kind,descriptors,table,infra}.rs
import { anchors, ccBuiltins, fmt, head, hRules, readV8, tqDecls, writeSrc } from "./builtins-lib.js";
const GEN = "builtins-core.js";
const CRATE = "v8-builtins-core";
const h = readV8("src/builtins/builtins.h");
anchors(h, ["enum class Builtin : int32_t", "kLast = static_cast<Builtin>(kBuiltinCount - 1)"], "builtins.h");
const defs = readV8("src/builtins/builtins-definitions.h");
anchors(defs, ["BUILTIN_LIST_BASE_TIER0", "BUILTIN_LIST_BASE(CPP"], "builtins-definitions.h");
const desc = readV8("src/builtins/builtins-descriptors.h");
anchors(desc, ["Builtin_", "InterfaceDescriptor"], "builtins-descriptors.h");
anchors(readV8("src/builtins/base.tq"), ["type Tagged generates 'MaybeObject'", "type void;"], "base.tq");
anchors(readV8("src/builtins/setup-builtins-internal.cc"), ["SetupBuiltins"], "setup-builtins-internal.cc");
// kind.rs: X-macro 参数位 → BuiltinKind
{
  const kinds = ["CPP", "TFJ_TSA", "TFJ", "TFC_TSA", "TFC", "TFS", "TFH", "ASM", "BCH"];
  const doc = { CPP: "C++ 实现, 经 BUILTIN_EXIT 帧进入", TFJ_TSA: "Turboshaft JS linkage", TFJ: "Turbofan JS linkage", TFC_TSA: "Turboshaft CodeStub linkage + 自定义 descriptor", TFC: "Turbofan CodeStub linkage + 自定义 descriptor", TFS: "Turbofan CodeStub linkage", TFH: "Turbofan handler", ASM: "平台相关汇编实现", BCH: "字节码 handler" };
  const o = [head(GEN, ["builtins-definitions.h", "builtins.h"], "内建种类枚举 (X-macro 参数位对应)")];
  o.push('use strum::{Display, EnumIter, EnumString};\n');
  o.push('/// 内建实现种类 — BUILTIN_LIST_* 参数位 (builtins-definitions.h:75 起)');
  o.push('#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]');
  o.push('pub enum BuiltinKind {');
  for (const k of kinds) { const v = k.replace(/_/g, ""); o.push(`  /// ${doc[k]}\n  #[strum(serialize = "${k}")]\n  ${v},`); }
  o.push('}\n');
  for (const g of ["BUILTIN_LIST_BASE_TIER0", "BUILTIN_LIST_BASE_TIER1", "BUILTIN_LIST_TEMPORAL", "BUILTIN_LIST_INTL"]) {
    const n = defs.split("\n").filter((l) => l.includes(g + "(")).length;
    o.push(`/// ${g} 出现次数 (机械计数)\npub const ${g}_USE_COUNT: usize = ${n};`);
  }
  o.push(`
impl BuiltinKind {
  /// 是否为 C++ 实现内建
  #[inline]
  pub const fn is_cpp(self) -> bool {
    matches!(self, Self::CPP)
  }

  /// 是否具备 JS linkage
  #[inline]
  pub const fn has_js_linkage(self) -> bool {
    matches!(self, Self::TFJ | Self::TFJTSA)
  }

  /// 是否为 CodeStub linkage
  #[inline]
  pub const fn is_code_stub(self) -> bool {
    matches!(self, Self::TFC | Self::TFCTSA | Self::TFS)
  }

  /// 是否为平台相关汇编实现
  #[inline]
  pub const fn is_asm(self) -> bool {
    matches!(self, Self::ASM)
  }

  /// 是否为字节码处理器
  #[inline]
  pub const fn is_bytecode_handler(self) -> bool {
    matches!(self, Self::BCH)
  }
}
`);
  writeSrc(CRATE, "kind.rs", fmt(o.join("\n") + "\n"));
}
// descriptors.rs
{
  const cc = readV8("src/builtins/builtins.cc");
  const uniq = [...new Set(ccBuiltins(cc).map((b) => b.name))].sort();
  const o = [head(GEN, ["builtins.cc", "builtins-descriptors.h"], "内建名 → descriptor 名映射骨架")];
  o.push('/// 内建 descriptor 名 (Builtin_<Name>_InterfaceDescriptor)\npub fn descriptor_name(builtin: &str) -> String {\n  format!("Builtin_{builtin}_InterfaceDescriptor")\n}\n');
  o.push(`/// builtins.cc 登记内建数: ${uniq.length}\npub const CC_BUILTIN_COUNT: usize = ${uniq.length};`);
  writeSrc(CRATE, "descriptors.rs", fmt(o.join("\n") + "\n"));
}
// table.rs: BUILTIN_LIST 条目 (kind,name,行号)
{
  const lines = defs.split("\n");
  const rows = [];
  for (let i = 0; i < lines.length; i++) {
    const m = lines[i].match(/^\s*(CPP|TFJ_TSA|TFJ|TFC_TSA|TFC|TFS|TFH|ASM|BCH)\((\w+)/);
    if (m) rows.push({ kind: m[1], name: m[2], line: i + 1 });
  }
  const o = [head(GEN, ["builtins-definitions.h"], `BUILTIN_LIST 条目 ${rows.length} 行`)];
  o.push('/// (kind, name, definitions.h 行号)\npub const BUILTIN_TABLE: &[(&str, &str, u32)] = &[');
  for (const r of rows.slice(0, 2000)) o.push(`  ("${r.kind}", "${r.name}", ${r.line}),`);
  o.push('];');
  o.push(`pub const BUILTIN_TABLE_LEN: usize = ${rows.length};`);
  o.push(`
/// 获取指定索引的内建条目 (kind, name, line)
#[inline]
pub fn get_builtin(index: usize) -> Option<(&'static str, &'static str, u32)> {
  BUILTIN_TABLE.get(index).copied()
}

/// 按名称查找内建索引
pub fn find_builtin(name: &str) -> Option<usize> {
  BUILTIN_TABLE.iter().position(|(_, n, _)| *n == name)
}

/// 判断是否为 Tier0 核心内建 (前 7 项)
#[inline]
pub const fn is_tier0(index: usize) -> bool {
  index < 7
}

/// 统计指定 kind 的内建数量
pub fn count_by_kind(kind: &str) -> usize {
  BUILTIN_TABLE.iter().filter(|(k, _, _)| *k == kind).count()
}
`);
  writeSrc(CRATE, "table.rs", fmt(o.join("\n") + "\n"));
  console.log(`[${GEN}] table: ${rows.length} 条`);
}
// infra.rs: .tq 基建声明 + utils 头规则
{
  const tqFiles = ["base.tq", "internal.tq", "internal-coverage.tq", "torque-internal.tq", "frames.tq", "frame-arguments.tq", "cast.tq", "convert.tq"];
  const decls = [];
  for (const f of tqFiles) for (const d of tqDecls(readV8(`src/builtins/${f}`))) decls.push({ ...d, file: f });
  const gapFiles = ["builtins-inl.h", "builtins-utils-gen.h", "builtins-utils-inl.h", "superspread.h", "torque-csa-header-includes.h", "constants-table-builder.cc", "constants-table-builder.h", "generate-bytecodes-builtins-list.cc", "profile-data-reader.cc", "profile-data-reader.h"];
  for (const f of gapFiles) {
    const c = readV8("src/builtins/" + f);
    if (!c.length) throw new Error("空文件: " + f);
  }
  anchors(readV8("src/builtins/builtins-inl.h"), ["Builtins"], "builtins-inl.h");
  anchors(readV8("src/builtins/builtins-utils-gen.h"), ["TF_BUILTIN"], "builtins-utils-gen.h");
  anchors(readV8("src/builtins/superspread.h"), ["SuperSpreadArgs"], "superspread.h");
  anchors(readV8("src/builtins/constants-table-builder.h"), ["BuiltinsConstantsTableBuilder"], "constants-table-builder.h");
  anchors(readV8("src/builtins/profile-data-reader.h"), ["ProfileDataFromFile"], "profile-data-reader.h");
  const { enums, consts } = hRules(readV8("src/builtins/builtins-utils.h"));
  const o = [head(GEN, [...tqFiles, "builtins-utils.h", ...gapFiles], "基建 .tq 声明 + 工具头规则")];
  o.push(`/// .tq 基建声明 (${decls.length} 条)\npub const TQ_INFRA_DECLS: &[(&str, &str, &str)] = &[`);
  for (const d of decls.slice(0, 1500)) o.push(`  ("${d.kind}", "${d.name}", "${d.file}"),`);
  o.push('];');
  o.push(`// builtins-utils.h 枚举: ${enums.map((e) => e.name).join(", ") || "无"}`);
  for (const c of consts.slice(0, 200)) o.push(`// ${c.ty} ${c.name} = ${c.expr}`);
  writeSrc(CRATE, "infra.rs", fmt(o.join("\n") + "\n"));
  console.log(`[${GEN}] infra: tq ${decls.length}, 枚举 ${enums.length}, 常量 ${consts.length}`);
}
// lib.rs
{
  const o = [head(GEN, ["(上 25 文件)"], "模块索引")];
  o.push('#![forbid(unsafe_code)]\n');
  o.push('mod descriptors;\nmod infra;\nmod kind;\nmod table;\n');
  o.push('pub use descriptors::{CC_BUILTIN_COUNT, descriptor_name};');
  o.push('pub use infra::TQ_INFRA_DECLS;');
  o.push('pub use kind::{BuiltinKind, BUILTIN_LIST_BASE_TIER0_USE_COUNT, BUILTIN_LIST_BASE_TIER1_USE_COUNT, BUILTIN_LIST_INTL_USE_COUNT, BUILTIN_LIST_TEMPORAL_USE_COUNT};');
  o.push('pub use table::{BUILTIN_TABLE, BUILTIN_TABLE_LEN, count_by_kind, find_builtin, get_builtin, is_tier0};');
  o.push('mod error;\npub use error::{Error, Result};');
  writeSrc(CRATE, "lib.rs", fmt(o.join("\n") + "\n"));
}
console.log(`[${GEN}] 完成`);
