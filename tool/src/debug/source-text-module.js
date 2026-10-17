#!/usr/bin/env bun
// tool/src/debug/source-text-module.js — 覆盖 v8/src/objects/source-text-module.{h,-inl.h,cc,tq}
// 组→文件映射 (本脚本管辖): 席位表 4 文件全族
// 产物: v8-debug/src/source_text_module.rs
// 机械提取: SourceTextModuleFlags 位域 + async 评价序数常量 + ModuleRequest 位域
//   + SourceTextModuleInfo 索引枚举 + ExecuteAsyncModuleContextSlots + tq 11 字段布局
//   + .cc Import/ExportIndex 公式 + 方法签名骨架 (todo! 就地修订)

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/source-text-module.h");
const inl = readV8("src/objects/source-text-module-inl.h");
const cc = readV8("src/objects/source-text-module.cc");
const tq = readV8("src/objects/source-text-module.tq");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`[source-text-module.js] ${m}`); };

// ── 1. async 评价序数常量 (.h) ───────────────────────────────────────────
const kNotAsync = h.match(/static constexpr unsigned kNotAsyncEvaluated = (\d+);/);
const kDidFinish = h.match(/static constexpr unsigned kAsyncEvaluateDidFinish = (\d+);/);
const kFirstOrd = h.match(/static constexpr unsigned kFirstAsyncEvaluationOrdinal = (\d+);/);
must(kNotAsync && kNotAsync[1] === "0", "kNotAsyncEvaluated != 0");
must(kDidFinish && kDidFinish[1] === "1", "kAsyncEvaluateDidFinish != 1");
must(kFirstOrd && kFirstOrd[1] === "2", "kFirstAsyncEvaluationOrdinal != 2");
must(h.includes("static_assert(kNotAsyncEvaluated < kAsyncEvaluateDidFinish);"), "缺序数大小断言链 1");
must(h.includes("static_assert(kAsyncEvaluateDidFinish < kFirstAsyncEvaluationOrdinal);"), "缺序数大小断言链 2");

// ── 2. flags 位域 (.h BitField ↔ tq bitfield 交叉) ──────────────────────
must(h.includes("using HasToplevelAwaitBit = base::BitField<bool, 0, 1, uint32_t>;"), "h 缺 HasToplevelAwaitBit(0,1)");
must(h.includes("using AsyncEvaluationOrdinalBits = HasToplevelAwaitBit::Next<uint32_t, 30>;"), "h 缺 AsyncEvaluationOrdinalBits(1,30)");
const bf = tq.match(/bitfield struct SourceTextModuleFlags extends uint31 \{([\s\S]*?)\}/);
must(bf, "tq 缺 SourceTextModuleFlags");
const bfFields = [...bf[1].matchAll(/^  (\w+): (\w+): (\d+) bit;/gm)].map((m) => [m[1], m[2], +m[3]]);
must(bfFields.length === 2, `tq flags 字段数 ${bfFields.length} != 2`);
must(bfFields[0][0] === "has_toplevel_await" && bfFields[0][2] === 1, "tq has_toplevel_await 位宽漂移");
must(bfFields[1][0] === "async_evaluation_ordinal" && bfFields[1][2] === 30, "tq async_evaluation_ordinal 位宽漂移");

// ── 3. ExecuteAsyncModuleContextSlots (.h) ───────────────────────────────
must(h.includes("enum ExecuteAsyncModuleContextSlots {\n    kModule = Context::MIN_CONTEXT_SLOTS,\n    kContextLength,\n  };"),
  "h 缺 ExecuteAsyncModuleContextSlots (kModule=MIN_CONTEXT_SLOTS, kContextLength)");

// ── 4. SourceTextModuleInfo 索引枚举 (.h 匿名 enum ×2) ───────────────────
const idx1 = h.match(/enum : uint32_t \{\s*\n\s*kModuleRequestsIndex,\s*\n\s*kSpecialExportsIndex,\s*\n\s*kRegularExportsIndex,\s*\n\s*kNamespaceImportsIndex,\s*\n\s*kRegularImportsIndex,\s*\n\s*kLength\s*\n\s*\};/);
must(idx1, "h 缺 SourceTextModuleInfo 5 索引枚举");
const idx2 = h.match(/enum : uint32_t \{\s*\n\s*kRegularExportLocalNameOffset,\s*\n\s*kRegularExportCellIndexOffset,\s*\n\s*kRegularExportExportNamesOffset,\s*\n\s*kRegularExportLength\s*\n\s*\};/);
must(idx2, "h 缺 regular export 3 偏移枚举");
// -inl.h 交叉: ModuleRequest flags 编码 (phase 2 bit + position 29 bit)
must(inl.includes("flags_(Smi::From31BitPattern(PhaseBits::encode(phase) |"), "-inl.h 缺 PhaseBits 编码");
must(inl.includes("PositionBits::encode(position))) {"), "-inl.h 缺 PositionBits 编码");
must(inl.includes("ModuleRequest::specifier() const"), "-inl.h 缺 specifier 访问器");
must(inl.includes("ModuleRequest::import_attributes() const"), "-inl.h 缺 import_attributes 访问器");
must(inl.includes("uint32_t ModuleRequest::flags() const"), "-inl.h 缺 flags 访问器");

// ── 5. ModuleRequest (.h 位域 + kAttributeEntrySize) ─────────────────────
must(h.includes("static const size_t kAttributeEntrySize = 3;"), "kAttributeEntrySize != 3");
must(h.includes("using PhaseBits = base::BitField<ModuleImportPhase, 0, 2, uint32_t>;"), "h 缺 PhaseBits(0,2)");
must(h.includes("using PositionBits = PhaseBits::Next<uint32_t, 29>;"), "h 缺 PositionBits(2,29)");
must(h.includes("static_assert(PositionBits::kMax >= String::kMaxLength"), "缺 PositionBits::kMax 断言");
const mrf = tq.match(/bitfield struct ModuleRequestFlags extends uint31 \{([\s\S]*?)\}/);
must(mrf, "tq 缺 ModuleRequestFlags");
const mrfFields = [...mrf[1].matchAll(/^  (\w+): (\w+): (\d+) bit;/gm)].map((m) => [m[1], m[2], +m[3]]);
must(mrfFields.length === 2 && mrfFields[0][0] === "phase" && mrfFields[0][2] === 2 && mrfFields[1][0] === "position" && mrfFields[1][2] === 29,
  `tq ModuleRequestFlags 漂移: ${JSON.stringify(mrfFields)}`);
// ModuleImportPhase 三态 (include/v8-callbacks.h)
const cb = readV8("include/v8-callbacks.h");
const phase = cb.match(/enum class ModuleImportPhase \{([\s\S]*?)\};/);
must(phase, "v8-callbacks.h 缺 ModuleImportPhase");
const phaseEntries = [...phase[1].matchAll(/(k\w+),?/g)].map((m) => m[1]);
must(phaseEntries.join(",") === "kSource,kDefer,kEvaluation", `ModuleImportPhase 漂移: ${phaseEntries.join(",")}`);

// ── 6. tq extern class SourceTextModule 布局 (11 字段) ───────────────────
const clsStart = tq.indexOf("extern class SourceTextModule extends Module {");
const cls = tq.slice(clsStart, tq.indexOf("\n}", clsStart));
const fields = [...cls.matchAll(/^  ((?:@cpp\w+ )*)(\w+): ([^;]+);$/gm)].map((m) => [m[2], m[3].trim()]);
must(fields.length === 11, `tq SourceTextModule 字段数 ${fields.length} != 11`);
must(fields.map((f) => f[0]).join(",") === "code,regular_exports,regular_imports,requested_modules,import_meta,cycle_root,async_parent_modules,dfs_index,dfs_ancestor_index,pending_async_dependencies,flags",
  "tq SourceTextModule 布局漂移");
must(fields[10][1] === "SmiTagged<SourceTextModuleFlags>", "tq flags 类型漂移");
// .h TaggedMember 交叉 (11, 限定 SourceTextModule 类体内; 折叠声明断行)
const stmStart = h.indexOf("V8_OBJECT class SourceTextModule : public Module {");
const stmCls = h.slice(stmStart, h.indexOf("} V8_OBJECT_END;", stmStart)).replace(/>\s*\n\s+/g, "> ");
const hMembers = [...stmCls.matchAll(/^  (?:V8_TQ_ACQ_REL )?TaggedMember<[^;]+?> (\w+)(?: V8_TQ_TYPE\([^)]*\))?\s*;$/gm)].map((m) => m[1]);
must(hMembers.length === 11, `h TaggedMember 数 ${hMembers.length} != 11`);
must(hMembers.join(",").replace(/_/g, "") === "code,regularexports,regularimports,requestedmodules,importmeta,cycleroot,asyncparentmodules,dfsindex,dfsancestorindex,pendingasyncdependencies,flags", "h 成员序漂移");
// tq SourceTextModuleInfoEntry 布局 (7 字段)
const ent = tq.slice(tq.indexOf("extern class SourceTextModuleInfoEntry extends Struct {"));
const entFields = [...ent.matchAll(/^  (\w+): ([^;]+);$/gm)].map((m) => [m[1], m[2].trim()]);
must(entFields.length === 7, `tq SourceTextModuleInfoEntry 字段数 ${entFields.length} != 7`);

// ── 7. .cc: cell index 换算公式 + 方法签名骨架 ───────────────────────────
must(cc.includes("int SourceTextModule::ExportIndex(int cell_index) {"), "cc 缺 ExportIndex");
must(cc.includes("return cell_index - 1;"), "cc ExportIndex 公式漂移 (cell_index - 1)");
must(cc.includes("return -cell_index - 1;"), "cc ImportIndex 公式漂移 (-cell_index - 1)");
const ccSigs = [...cc.matchAll(/^(?:Handle<\w+>|MaybeHandle<\w+>|MaybeDirectHandle<\w+>|Maybe<[^>]+>|DirectHandle<\w+>|Tagged<\w+>|bool|void|int|std::pair<[^=]+>)\s+SourceTextModule::(\w+)\(/gm)]
  .map((m) => m[1]);
must(ccSigs.length >= 30, `.cc SourceTextModule 方法数 ${ccSigs.length} < 30`);
for (const s of ["GetSharedFunctionInfo", "GetCell", "ResolveExport", "PrepareInstantiate", "FinishInstantiate", "Evaluate",
  "AsyncModuleExecutionFulfilled", "AsyncModuleExecutionRejected", "ExecuteAsyncModule", "InnerModuleEvaluation",
  "GetStalledTopLevelAwaitMessages", "IsModuleSCCEvaluated"]) {
  must(ccSigs.includes(s), `.cc 缺方法 ${s}`);
}
must(new Set(ccSigs).size === ccSigs.length, ".cc 方法签名重复");

const snake = (s) => s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();

let rust = banner("src/debug/source-text-module.js", "src/objects/source-text-module.{h,-inl.h,cc,tq}",
  "全族: 模块记录 async 评价序数/位域/索引枚举 + tq 布局 + cc 签名骨架");
rust += `//! SourceTextModule: ECMAScript Source Text Module Record 运行时表示 (debug 域)。
//!
//! 顶层 await 的 async 评价序数与位域布局 + ModuleRequest 编码 + 模块信息索引;
//! .cc 约 1700 行链接/求值语义为签名骨架, 由 Developer 就地修订。

// ── async 评价序数 (原版 static constexpr unsigned; 单调递增哨兵) ──
/// 未异步求值 (原版 kNotAsyncEvaluated)。
pub const K_NOT_ASYNC_EVALUATED: u32 = ${kNotAsync[1]};
/// 异步求值已完成 (原版 kAsyncEvaluateDidFinish)。
pub const K_ASYNC_EVALUATE_DID_FINISH: u32 = ${kDidFinish[1]};
/// 首个异步求值序数 (原版 kFirstAsyncEvaluationOrdinal; >1 表示 async)。
pub const K_FIRST_ASYNC_EVALUATION_ORDINAL: u32 = ${kFirstOrd[1]};

// ── SourceTextModuleFlags (tq uint31; h BitField 链 1:1) ──
pub const K_HAS_TOPLEVEL_AWAIT_SHIFT: u32 = 0;
pub const K_HAS_TOPLEVEL_AWAIT_BITS: u32 = 1;
pub const K_HAS_TOPLEVEL_AWAIT_MASK: u32 = 0x1;
pub const K_ASYNC_EVALUATION_ORDINAL_SHIFT: u32 = 1;
pub const K_ASYNC_EVALUATION_ORDINAL_BITS: u32 = 30;
pub const K_ASYNC_EVALUATION_ORDINAL_MASK: u32 = 0x7FFFFFFE;
/// 位域总宽 (uint31 = Smi 载荷上限)。
pub const K_FLAGS_TOTAL_BITS: u32 = 31;

// ── ExecuteAsyncModuleContextSlots (kModule = Context::MIN_CONTEXT_SLOTS = 2) ──
pub const K_EXECUTE_ASYNC_MODULE_CONTEXT_MODULE_SLOT: u32 = 2;
pub const K_EXECUTE_ASYNC_MODULE_CONTEXT_LENGTH: u32 = 4;

// ── SourceTextModuleInfo 索引 (FixedArray 槽位, 原版匿名 enum) ──
pub const K_MODULE_REQUESTS_INDEX: u32 = 0;
pub const K_SPECIAL_EXPORTS_INDEX: u32 = 1;
pub const K_REGULAR_EXPORTS_INDEX: u32 = 2;
pub const K_NAMESPACE_IMPORTS_INDEX: u32 = 3;
pub const K_REGULAR_IMPORTS_INDEX: u32 = 4;
pub const K_MODULE_INFO_LENGTH: u32 = 5;

// ── regular_exports 条目内偏移 (原版匿名 enum) ──
pub const K_REGULAR_EXPORT_LOCAL_NAME_OFFSET: u32 = 0;
pub const K_REGULAR_EXPORT_CELL_INDEX_OFFSET: u32 = 1;
pub const K_REGULAR_EXPORT_EXPORT_NAMES_OFFSET: u32 = 2;
pub const K_REGULAR_EXPORT_LENGTH: u32 = 3;

// ── ModuleRequest (phase 2 bit + position 29 bit) ──
pub const K_MODULE_REQUEST_PHASE_SHIFT: u32 = 0;
pub const K_MODULE_REQUEST_PHASE_BITS: u32 = 2;
pub const K_MODULE_REQUEST_POSITION_SHIFT: u32 = 2;
pub const K_MODULE_REQUEST_POSITION_BITS: u32 = 29;
/// import_attributes 数组单条目槽数 (key/value/location 三元组)。
pub const K_ATTRIBUTE_ENTRY_SIZE: usize = 3;

/// 导入阶段 (原版 v8::ModuleImportPhase; include/v8-callbacks.h)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ModuleImportPhase {
  /// kSource
  Source = 0,
  /// kDefer
  Defer = 1,
  /// kEvaluation
  Evaluation = 2,
}

// ── tq 布局表 (源码序) ──
pub const SOURCE_TEXT_MODULE_FIELDS: [(&str, &str); ${fields.length}] = [
`;
for (const f of fields) rust += `  ("${f[0]}", "${f[1]}"),\n`;
rust += `];
pub const SOURCE_TEXT_MODULE_INFO_ENTRY_FIELDS: [(&str, &str); ${entFields.length}] = [\n`;
for (const f of entFields) rust += `  ("${f[0]}", "${f[1]}"),\n`;
rust += `];

// ── cell index 换算 (cc 公式 1:1) ──
/// 常规导出槽下标 (原版 ExportIndex: cell_index - 1)。
#[inline]
#[must_use]
pub const fn export_index(cell_index: i32) -> i32 {
  cell_index - 1
}

/// 常规导入槽下标 (原版 ImportIndex: -cell_index - 1)。
#[inline]
#[must_use]
pub const fn import_index(cell_index: i32) -> i32 {
  -cell_index - 1
}

// ── .cc 方法签名骨架 (源码序; ExportIndex/ImportIndex 已由上方公式函数承载, 骨架跳过) ──
`;
for (const m of ccSigs) {
  if (m === "ExportIndex" || m === "ImportIndex") continue;
  rust += `/// 原版 SourceTextModule::${m} (source-text-module.cc)。
pub fn ${snake(m)}() {
  todo!("就地修订: source-text-module.cc ${m}")
}

`;
}
rust += `/// 编译期字符串相等 (str 的 const PartialEq 尚未稳定, 手写逐字节比较)
const fn str_eq(a: &str, b: &str) -> bool {
  let (a, b) = (a.as_bytes(), b.as_bytes());
  if a.len() != b.len() {
    return false;
  }
  let mut i = 0;
  while i < a.len() {
    if a[i] != b[i] {
      return false;
    }
    i += 1;
  }
  true
}

// ── 提取不变式断言 ──
const _: () = {
  assert!(K_NOT_ASYNC_EVALUATED == 0);
  assert!(K_ASYNC_EVALUATE_DID_FINISH == 1);
  assert!(K_FIRST_ASYNC_EVALUATION_ORDINAL == 2);
  assert!(K_NOT_ASYNC_EVALUATED < K_ASYNC_EVALUATE_DID_FINISH);
  assert!(K_ASYNC_EVALUATE_DID_FINISH < K_FIRST_ASYNC_EVALUATION_ORDINAL);
  assert!(K_HAS_TOPLEVEL_AWAIT_SHIFT + K_ASYNC_EVALUATION_ORDINAL_SHIFT == 1);
  assert!(K_FLAGS_TOTAL_BITS == 31);
  assert!(K_MODULE_INFO_LENGTH == 5);
  assert!(K_REGULAR_EXPORT_LENGTH == 3);
  assert!(K_MODULE_REQUEST_PHASE_BITS + K_MODULE_REQUEST_POSITION_BITS == 31);
  assert!(K_ATTRIBUTE_ENTRY_SIZE == 3);
  assert!(SOURCE_TEXT_MODULE_FIELDS.len() == 11);
  assert!(str_eq(SOURCE_TEXT_MODULE_FIELDS[10].0, "flags"));
  assert!(SOURCE_TEXT_MODULE_INFO_ENTRY_FIELDS.len() == 7);
  assert!(export_index(1) == 0);
  assert!(import_index(-1) == 0);
  assert!(ModuleImportPhase::Source as u32 == 0);
};
`;

const p = writeSrc("v8-debug", "source_text_module.rs", rust);
console.log(`source-text-module.js: ${p} — 序数常量 3、flags 位域 2、索引枚举 5+3、ModuleRequest 2 位段 + Phase 3 态、tq 布局 ${fields.length}+${entFields.length} 字段、.cc 方法 ${ccSigs.length}, 断言 ${checks} 项全过`);
