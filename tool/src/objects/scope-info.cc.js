#!/usr/bin/env bun
// tool/src/objects/scope-info.cc.js — 覆盖 v8/src/objects/scope-info.{cc,-inl.h}
// (差集: h/tq 归既有 scope-info.h.js 席; 其声明覆盖 cc 但登记跳过 >2k 行语义面)
// → v8-objects-js-func/src/scope_info_cc.rs
// 机械提取: 变长条件字段偏移链 (12 环, 全参数化 const fn) + DataSlotIndex 换算 +
//   -inl/cc 方法清单 + 关键语义锚。
// 配置登记: 64 位非压缩 (kTaggedSize=8 → FunctionVariableInfo=16, ModuleVariableInfo=24;
//   内联名阈值 kScopeInfoMaxInlinedLocalNamesSize=75 取自 globals.h)。

import { readV8, writeSrc, banner } from "../../lib.js";

const cc = readV8("src/objects/scope-info.cc");
const inl = readV8("src/objects/scope-info-inl.h");
const h = readV8("src/objects/scope-info.h");
const globals = readV8("src/common/globals.h");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};

// ── 1. 常量与结构尺寸锚 ────────────────────────────────────────────────
const maxInlined = globals.match(/constexpr int kScopeInfoMaxInlinedLocalNamesSize = (\d+);/);
if (!maxInlined) throw new Error("未找到 kScopeInfoMaxInlinedLocalNamesSize");
const K_MAX_INLINED = parseInt(maxInlined[1], 10);
must(K_MAX_INLINED === 75, `内联名阈值应 75, 实得 ${K_MAX_INLINED}`);
// 结构尺寸 (h 侧 LINT.IfChange 段)
must(h.includes("struct FunctionVariableInfo {") &&
  h.includes("TaggedMember<Object> name;") &&
  h.includes("TaggedMember<Smi> context_or_stack_slot_index;"),
  "FunctionVariableInfo 结构锚缺失");
must(h.includes("struct ModuleVariableInfo {") &&
  h.includes("TaggedMember<String> name;") &&
  h.includes("TaggedMember<Smi> index;") &&
  h.includes("TaggedMember<Smi> properties;"),
  "ModuleVariableInfo 结构锚缺失");
const T = 8;
const SZ_FUNCTION_VARIABLE_INFO = 2 * T; // 16
const SZ_MODULE_VARIABLE_INFO = 3 * T; // 24
must(SZ_FUNCTION_VARIABLE_INFO === 16 && SZ_MODULE_VARIABLE_INFO === 24, "结构尺寸推导失配");

// ── 2. -inl 偏移链 12 环 (每环锚定公式) ────────────────────────────────
must(inl.includes("int ScopeInfo::ModuleVariableCountOffset() const {"), "ModuleVariableCountOffset 锚缺失");
must(inl.includes("return OFFSET_OF_DATA_START(ScopeInfo);"), "ModuleVariableCountOffset 恒等式锚缺失");
must(inl.includes("return ModuleVariableCountOffset() + (is_module ? kTaggedSize : 0);"),
  "ContextLocalNamesOffset is_module 步进锚缺失");
must(inl.includes("local_count < kScopeInfoMaxInlinedLocalNamesSize ? local_count : 0;"),
  "ContextLocalNamesHashtableOffset inlined_count 公式锚缺失");
must(inl.includes("return ContextLocalNamesHashtableOffset() + (has_hashtable ? kTaggedSize : 0);"),
  "ContextLocalInfosOffset hashtable 步进锚缺失");
must(inl.includes("return ContextLocalInfosOffset() + context_local_count() * kTaggedSize;"),
  "SavedClassVariableInfoOffset 锚缺失");
must(inl.includes("HasSavedClassVariableBit::decode(Flags())"), "SavedClassVariable 位段锚缺失");
must(inl.includes("FunctionVariableBits::decode(Flags()) != VariableAllocationInfo::NONE"),
  "FunctionVariable 判定锚缺失");
must(inl.includes("sizeof(FunctionVariableInfo)"), "InferredFunctionNameOffset 结构尺寸步进锚缺失");
must(inl.includes("HasInferredFunctionNameBit::decode(Flags())"), "HasInferredFunctionName 位段锚缺失");
must(inl.includes("HasOuterScopeInfoBit::decode(Flags())"), "HasOuterScopeInfo 位段锚缺失");
must(inl.includes("return ModuleInfoOffset() + (is_module ? kTaggedSize : 0);"),
  "ModuleVariablesOffset 锚缺失");
must(inl.includes("return ModuleVariablesOffset() + mod_var_count * sizeof(ModuleVariableInfo);"),
  "ModuleVariablesHashtableOffset 公式锚缺失");
must(inl.includes("HasModuleVariablesHashtable() ? kTaggedSize : 0"), "DependentCodeOffset 锚缺失");
must(inl.includes("SloppyEvalCanExtendVarsBit::decode(Flags())"), "UnusedParameterBits 位段锚缺失");
must(inl.includes("return UnusedParameterBitsOffset() + (is_function ? kTaggedSize : 0);"),
  "AllocatedSize is_function 步进锚缺失");
// DataSlotIndex
must(inl.includes("constexpr int DataSlotIndex(int byte_offset) {"), "DataSlotIndex 定义锚缺失");
must(inl.includes("return (byte_offset - OFFSET_OF_DATA_START(ScopeInfo)) / kTaggedSize;"),
  "DataSlotIndex 公式锚缺失");
// Smi 打包
must(inl.includes("Smi::From31BitPattern(value)"), "context_local_infos 31 位打包锚缺失");

// ── 3. 方法清单 ────────────────────────────────────────────────────────
const inlDefs = [...new Set([...inl.matchAll(/^(?:[A-Za-z_][\w:<>,*&]*[ \t]+)?ScopeInfo::(\w+)\(/gm)].map((m) => m[1]))];
must(inlDefs.length >= 40, `-inl 方法应 ≥40, 实得 ${inlDefs.length}`);
for (const need of ["flags", "parameter_count", "ContextLocalNamesOffset", "ModuleVariablesOffset",
  "AllocatedSize", "module_variable_count", "context_local_names", "context_local_infos"])
  must(inlDefs.includes(need), `-inl 缺方法 ${need}`);
const ccDefs = [...new Set([...cc.matchAll(/^(?:[A-Za-z_][\w:<>,*&]*[ \t]+)?ScopeInfo::(\w+)\(/gm)].map((m) => m[1]))];
must(ccDefs.length >= 60, `cc 方法应 ≥60, 实得 ${ccDefs.length}`);
for (const need of ["Create", "Equals", "ContextLength", "ContextHeaderLength", "HasContextExtensionSlot",
  "CreateForWithScope", "CreateForEmptyFunction", "CreateGlobalThisBinding", "Empty", "scope_type"])
  must(ccDefs.includes(need), `cc 缺方法 ${need}`);
// cc 关键语义锚
must(cc.includes("int ScopeInfo::ContextLength() const {"), "ContextLength 定义锚缺失");
must(cc.includes("int ScopeInfo::ContextHeaderLength() const {"), "ContextHeaderLength 定义锚缺失");

// ── 4. 生成: 偏移链参数化 const fn ────────────────────────────────────
const body = `${banner("src/objects/scope-info.cc.js", "src/objects/scope-info.{cc,-inl.h}", "差集 (h/tq 归 scope-info.h.js 席): 变长条件字段偏移链 (参数化展开)")}
//! 差集: ScopeInfo 变长尾区布局规则。
//! 偏移链 12 环 1:1 参数化 — 调用方传入 flags 解码结果与计数, 全链 const 可求值。
//! 配置: 64 位非压缩 (kTaggedSize=8, FunctionVariableInfo=16, ModuleVariableInfo=24)。

/// 内联名阈值 (原版 kScopeInfoMaxInlinedLocalNamesSize, globals.h)。
pub const K_SCOPE_INFO_MAX_INLINED_LOCAL_NAMES_SIZE: usize = ${K_MAX_INLINED};

/// FunctionVariableInfo 尺寸 (name + context_or_stack_slot_index, 2 tagged)。
pub const SZ_FUNCTION_VARIABLE_INFO: usize = ${SZ_FUNCTION_VARIABLE_INFO};
/// ModuleVariableInfo 尺寸 (name + index + properties, 3 tagged)。
pub const SZ_MODULE_VARIABLE_INFO: usize = ${SZ_MODULE_VARIABLE_INFO};

/// 偏移链入参: flags 位段解码结果 + 各计数 (一次解包, 全链共享)。
#[derive(Debug, Clone, Copy)]
pub struct ScopeInfoLayoutInputs {
  /// ScopeTypeBits == MODULE_SCOPE
  pub is_module: bool,
  /// context_local_count
  pub context_local_count: usize,
  /// HasSavedClassVariableBit
  pub has_saved_class_variable: bool,
  /// FunctionVariableBits != NONE
  pub has_function_variable: bool,
  /// HasInferredFunctionNameBit
  pub has_inferred_name: bool,
  /// HasOuterScopeInfoBit
  pub has_outer: bool,
  /// module_variable_count (仅 module 有效)
  pub module_variable_count: usize,
  /// HasModuleVariablesHashtable()
  pub has_module_variables_hashtable: bool,
  /// SloppyEvalCanExtendVarsBit
  pub sloppy_eval_can_extend_vars: bool,
  /// ScopeTypeBits == FUNCTION_SCOPE
  pub is_function: bool,
}

impl ScopeInfoLayoutInputs {
  /// 全 false/0 基准形态 (非 module、无任何可选区)。
  pub const EMPTY: Self = Self {
    is_module: false, context_local_count: 0, has_saved_class_variable: false,
    has_function_variable: false, has_inferred_name: false, has_outer: false,
    module_variable_count: 0, has_module_variables_hashtable: false,
    sloppy_eval_can_extend_vars: false, is_function: false,
  };

  /// v8: -inl ModuleVariableCountOffset — 恒为 data_start (module 无该区时长度 0)。
  #[inline]
  #[must_use]
  pub const fn module_variable_count_offset(&self, data_start: usize) -> usize {
    data_start
  }

  /// v8: -inl ContextLocalNamesOffset — module 时 + kTaggedSize。
  #[inline]
  #[must_use]
  pub const fn context_local_names_offset(&self, data_start: usize, tagged_size: usize) -> usize {
    self.module_variable_count_offset(data_start) + (self.is_module as usize) * tagged_size
  }

  /// v8: -inl ContextLocalNamesHashtableOffset —
  /// local_count < 75 内联 local_count 格, 否则 0 格 (后续整表区)。
  #[inline]
  #[must_use]
  pub const fn context_local_names_hashtable_offset(&self, data_start: usize, tagged_size: usize) -> usize {
    let local_count = self.context_local_count;
    let inlined_count = if local_count < K_SCOPE_INFO_MAX_INLINED_LOCAL_NAMES_SIZE { local_count } else { 0 };
    self.context_local_names_offset(data_start, tagged_size) + inlined_count * tagged_size
  }

  /// v8: -inl ContextLocalInfosOffset — 名区超阈值时 + 1 格 hashtable 指针。
  #[inline]
  #[must_use]
  pub const fn context_local_infos_offset(&self, data_start: usize, tagged_size: usize) -> usize {
    let has_hashtable = self.context_local_count >= K_SCOPE_INFO_MAX_INLINED_LOCAL_NAMES_SIZE;
    self.context_local_names_hashtable_offset(data_start, tagged_size) + (has_hashtable as usize) * tagged_size
  }

  /// v8: -inl SavedClassVariableInfoOffset — + context_local_count 格。
  #[inline]
  #[must_use]
  pub const fn saved_class_variable_info_offset(&self, data_start: usize, tagged_size: usize) -> usize {
    self.context_local_infos_offset(data_start, tagged_size) + self.context_local_count * tagged_size
  }

  /// v8: -inl FunctionVariableInfoOffset — 有 saved class variable 时 + 1 格。
  #[inline]
  #[must_use]
  pub const fn function_variable_info_offset(&self, data_start: usize, tagged_size: usize) -> usize {
    self.saved_class_variable_info_offset(data_start, tagged_size)
      + (self.has_saved_class_variable as usize) * tagged_size
  }

  /// v8: -inl InferredFunctionNameOffset — 有函数变量区时 + sizeof(FunctionVariableInfo)。
  #[inline]
  #[must_use]
  pub const fn inferred_function_name_offset(&self, data_start: usize, tagged_size: usize) -> usize {
    self.function_variable_info_offset(data_start, tagged_size)
      + (self.has_function_variable as usize) * SZ_FUNCTION_VARIABLE_INFO
  }

  /// v8: -inl OuterScopeInfoOffset — 有 inferred name 时 + 1 格。
  #[inline]
  #[must_use]
  pub const fn outer_scope_info_offset(&self, data_start: usize, tagged_size: usize) -> usize {
    self.inferred_function_name_offset(data_start, tagged_size) + (self.has_inferred_name as usize) * tagged_size
  }

  /// v8: -inl ModuleInfoOffset — 有 outer 时 + 1 格。
  #[inline]
  #[must_use]
  pub const fn module_info_offset(&self, data_start: usize, tagged_size: usize) -> usize {
    self.outer_scope_info_offset(data_start, tagged_size) + (self.has_outer as usize) * tagged_size
  }

  /// v8: -inl ModuleVariablesOffset — module 时 + 1 格 (module_info)。
  #[inline]
  #[must_use]
  pub const fn module_variables_offset(&self, data_start: usize, tagged_size: usize) -> usize {
    self.module_info_offset(data_start, tagged_size) + (self.is_module as usize) * tagged_size
  }

  /// v8: -inl ModuleVariablesHashtableOffset — + module_variable_count * 24。
  #[inline]
  #[must_use]
  pub const fn module_variables_hashtable_offset(&self, data_start: usize, tagged_size: usize) -> usize {
    let mod_var_count = if self.is_module { self.module_variable_count } else { 0 };
    self.module_variables_offset(data_start, tagged_size) + mod_var_count * SZ_MODULE_VARIABLE_INFO
  }

  /// v8: -inl DependentCodeOffset — 有 module 变量 hashtable 时 + 1 格。
  #[inline]
  #[must_use]
  pub const fn dependent_code_offset(&self, data_start: usize, tagged_size: usize) -> usize {
    self.module_variables_hashtable_offset(data_start, tagged_size)
      + (self.has_module_variables_hashtable as usize) * tagged_size
  }

  /// v8: -inl UnusedParameterBitsOffset — sloppy eval 可扩展时 + 1 格。
  #[inline]
  #[must_use]
  pub const fn unused_parameter_bits_offset(&self, data_start: usize, tagged_size: usize) -> usize {
    self.dependent_code_offset(data_start, tagged_size)
      + (self.sloppy_eval_can_extend_vars as usize) * tagged_size
  }

  /// v8: -inl AllocatedSize — 函数作用域时 + 1 格 = 总分配大小。
  #[inline]
  #[must_use]
  pub const fn allocated_size(&self, data_start: usize, tagged_size: usize) -> usize {
    self.unused_parameter_bits_offset(data_start, tagged_size) + (self.is_function as usize) * tagged_size
  }
}

/// v8: -inl scope_info_internal::DataSlotIndex —
/// (byte_offset - data_start) / kTaggedSize (须对齐)。
#[inline]
#[must_use]
pub const fn data_slot_index(byte_offset: usize, data_start: usize, tagged_size: usize) -> usize {
  (byte_offset - data_start) / tagged_size
}

// —— 方法清单 ——
/// v8: src/objects/scope-info-inl.h (${inlDefs.length} 方法)
pub const SCOPE_INFO_INL_METHODS: &[&str] = &[
${inlDefs.map((d) => `  "${d}",`).join("\n")}
];

/// v8: src/objects/scope-info.cc (${ccDefs.length} 方法; Create/ContextLength 等语义面留 Developer)
pub const SCOPE_INFO_CC_METHODS: &[&str] = &[
${ccDefs.map((d) => `  "${d}",`).join("\n")}
];

// —— 提取不变式断言 ——
const _: () = {
  const T: usize = 8;
  const D: usize = 16; // OFFSET_OF_DATA_START(ScopeInfo) 登记值 (map+length+padding, 归 h 席复核)
  assert!(K_SCOPE_INFO_MAX_INLINED_LOCAL_NAMES_SIZE == 75);
  assert!(SZ_FUNCTION_VARIABLE_INFO == 16);
  assert!(SZ_MODULE_VARIABLE_INFO == 24);
  // 非模块最小函数形态: names@D, infos@D, saved@D, fvi@D, inferred@D, outer@D, modinfo@D, modvars@D, ht@D, dep@D, unused@D, size=D+8
  let x = ScopeInfoLayoutInputs { is_function: true, ..ScopeInfoLayoutInputs::EMPTY };
  assert!(x.context_local_names_offset(D, T) == D);
  assert!(x.module_variables_offset(D, T) == D);
  assert!(x.allocated_size(D, T) == D + 8);
  // 模块形态: names 前移 1 格
  let m = ScopeInfoLayoutInputs { is_module: true, ..ScopeInfoLayoutInputs::EMPTY };
  assert!(m.context_local_names_offset(D, T) == D + 8);
  assert!(m.module_variables_offset(D, T) == D + 8);
  // 3 个局部名: infos = names + 3 格
  let l = ScopeInfoLayoutInputs { context_local_count: 3, ..ScopeInfoLayoutInputs::EMPTY };
  assert!(l.context_local_names_hashtable_offset(D, T) == D + 24);
  assert!(l.context_local_infos_offset(D, T) == D + 24);
  assert!(l.saved_class_variable_info_offset(D, T) == D + 48);
  // 超 75 阈值: hashtable 占位, saved = infos + 75 + 1
  let big = ScopeInfoLayoutInputs { context_local_count: 80, ..ScopeInfoLayoutInputs::EMPTY };
  assert!(big.context_local_names_hashtable_offset(D, T) == D);
  assert!(big.context_local_infos_offset(D, T) == D + 8);
  assert!(big.saved_class_variable_info_offset(D, T) == D + 8 + 640);
  // 函数变量区: inferred = fvi + 16
  let f = ScopeInfoLayoutInputs { has_function_variable: true, ..ScopeInfoLayoutInputs::EMPTY };
  assert!(f.inferred_function_name_offset(D, T) == D + 16);
  // 模块变量区: ht = modvars + count*24
  let mv = ScopeInfoLayoutInputs { is_module: true, module_variable_count: 2, ..ScopeInfoLayoutInputs::EMPTY };
  assert!(mv.module_variables_hashtable_offset(D, T) == D + 8 + 48);
  assert!(data_slot_index(D + 24, D, T) == 3);
  assert!(SCOPE_INFO_INL_METHODS.len() == ${inlDefs.length});
  assert!(SCOPE_INFO_CC_METHODS.len() == ${ccDefs.length});
};
`;

const p = writeSrc("v8-objects-js-func", "scope_info_cc.rs", body);
console.log(
  `scope-info.cc.js: ${p} — 偏移链 12 环 + DataSlotIndex、阈值 ${K_MAX_INLINED}、` +
    `结构 ${SZ_FUNCTION_VARIABLE_INFO}/${SZ_MODULE_VARIABLE_INFO}、-inl ${inlDefs.length}、cc ${ccDefs.length}、断言 ${checks} 项通过`,
);
