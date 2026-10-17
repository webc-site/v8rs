// 由 tool/src/objects/scope-info.cc.js 从 v8/src/objects/scope-info.{cc,-inl.h} 生成; 修订就地改, 复跑覆盖
// 差集 (h/tq 归 scope-info.h.js 席): 变长条件字段偏移链 (参数化展开)

//! 差集: ScopeInfo 变长尾区布局规则。
//! 偏移链 12 环 1:1 参数化 — 调用方传入 flags 解码结果与计数, 全链 const 可求值。
//! 配置: 64 位非压缩 (kTaggedSize=8, FunctionVariableInfo=16, ModuleVariableInfo=24)。

/// 内联名阈值 (原版 kScopeInfoMaxInlinedLocalNamesSize, globals.h)。
pub const K_SCOPE_INFO_MAX_INLINED_LOCAL_NAMES_SIZE: usize = 75;

/// FunctionVariableInfo 尺寸 (name + context_or_stack_slot_index, 2 tagged)。
pub const SZ_FUNCTION_VARIABLE_INFO: usize = 16;
/// ModuleVariableInfo 尺寸 (name + index + properties, 3 tagged)。
pub const SZ_MODULE_VARIABLE_INFO: usize = 24;

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
/// v8: src/objects/scope-info-inl.h (63 方法)
pub const SCOPE_INFO_INL_METHODS: &[&str] = &[
  "HasSimpleParameters",
  "HasContextCells",
  "is_hoisted_in_context",
  "Flags",
  "ParameterCount",
  "ContextLocalCount",
  "flags",
  "set_flags",
  "parameter_count",
  "set_parameter_count",
  "context_local_count",
  "set_context_local_count",
  "position_info_start",
  "set_position_info_start",
  "position_info_end",
  "set_position_info_end",
  "ModuleVariableCountOffset",
  "ContextLocalNamesOffset",
  "ContextLocalNamesHashtableOffset",
  "ContextLocalInfosOffset",
  "SavedClassVariableInfoOffset",
  "FunctionVariableInfoOffset",
  "InferredFunctionNameOffset",
  "OuterScopeInfoOffset",
  "ModuleInfoOffset",
  "ModuleVariablesOffset",
  "ModuleVariablesHashtableOffset",
  "DependentCodeOffset",
  "UnusedParameterBitsOffset",
  "AllocatedSize",
  "module_variable_count",
  "set_module_variable_count",
  "context_local_names",
  "set_context_local_names",
  "context_local_names_hashtable",
  "set_context_local_names_hashtable",
  "context_local_infos",
  "set_context_local_infos",
  "set_saved_class_variable_info",
  "set_function_variable_info_name",
  "function_variable_info_context_or_stack_slot_index",
  "set_function_variable_info_context_or_stack_slot_index",
  "set_inferred_function_name",
  "outer_scope_info",
  "set_outer_scope_info",
  "module_info",
  "set_module_info",
  "module_variables_name",
  "set_module_variables_name",
  "module_variables_index",
  "set_module_variables_index",
  "module_variables_properties",
  "set_module_variables_properties",
  "HasModuleVariablesHashtable",
  "module_variables_hashtable",
  "set_module_variables_hashtable",
  "dependent_code",
  "set_dependent_code",
  "unused_parameter_bits",
  "set_unused_parameter_bits",
  "InitializeTaggedMembers",
  "HasInlinedLocalNames",
  "IterateLocalNames",
];

/// v8: src/objects/scope-info.cc (83 方法; Create/ContextLength 等语义面留 Developer)
pub const SCOPE_INFO_CC_METHODS: &[&str] = &[
  "Equals",
  "Create",
  "CreateForWithScope",
  "CreateGlobalThisBinding",
  "CreateForEmptyFunction",
  "CreateForNativeContext",
  "CreateForShadowRealmNativeContext",
  "CreateForBootstrapping",
  "get",
  "set",
  "CopyElements",
  "RawFieldOfElementAt",
  "length",
  "Empty",
  "IsEmpty",
  "scope_type",
  "is_script_scope",
  "SloppyEvalCanExtendVars",
  "language_mode",
  "is_declaration_scope",
  "ContextLength",
  "UniqueIdInScript",
  "HasContextExtensionSlot",
  "SomeContextHasExtension",
  "mark_some_context_has_extension",
  "ContextHeaderLength",
  "HasReceiver",
  "HasAllocatedReceiver",
  "ClassScopeHasPrivateBrand",
  "HasSavedClassVariable",
  "IsSloppyNormalJSFunction",
  "CanOnlyAccessFixedFormalParameters",
  "HasFunctionName",
  "HasContextAllocatedFunctionName",
  "HasInferredFunctionName",
  "HasSharedFunctionName",
  "SetFunctionName",
  "SetInferredFunctionName",
  "HasOuterScopeInfo",
  "IsDebugEvaluateScope",
  "SetIsDebugEvaluateScope",
  "PrivateNameLookupSkipsOuterClass",
  "IsReplModeScope",
  "IsWrappedFunctionScope",
  "HasContext",
  "InferredFunctionName",
  "FunctionDebugName",
  "StartPosition",
  "EndPosition",
  "SetPositionInfo",
  "OuterScopeInfo",
  "ModuleDescriptorInfo",
  "ContextInlinedLocalName",
  "ContextLocalMode",
  "ContextLocalIsStaticFlag",
  "ContextLocalInitFlag",
  "ContextLocalIsParameter",
  "ContextLocalParameterNumber",
  "ContextLocalInitializerPosition",
  "ContextLocalMaybeAssignedFlag",
  "VariableIsSynthetic",
  "ModuleVariableCount",
  "ModuleIndex",
  "InlinedLocalNamesLookup",
  "ContextSlotIndex",
  "ReceiverContextSlotIndex",
  "ParametersStartIndex",
  "FunctionContextSlotIndex",
  "function_kind",
  "ContextLocalNamesIndex",
  "ContextLocalInfosIndex",
  "SavedClassVariableInfoIndex",
  "FunctionVariableInfoIndex",
  "InferredFunctionNameIndex",
  "OuterScopeInfoIndex",
  "ModuleInfoIndex",
  "ModuleVariableCountIndex",
  "ModuleVariablesIndex",
  "ModuleVariablesHashtableIndex",
  "ModuleVariable",
  "DependentCodeIndex",
  "UnusedParameterBitsIndex",
  "Hash",
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
  assert!(SCOPE_INFO_INL_METHODS.len() == 63);
  assert!(SCOPE_INFO_CC_METHODS.len() == 83);
};
