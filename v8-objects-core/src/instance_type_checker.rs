// 由 tool/src/objects/instance-type-checker.h.js 从 v8/src/objects/instance-type-checker.h + instance-type-inl.h 生成; 修订就地改, 复跑覆盖
// 桶机械展开: SINGLE 275 (含追加 2) / RANGE 59 (含追加 1); 自定义检查器 1:1; static-roots 快路径与 RootIndex 依赖推迟

//! 实例类型检查器 (InstanceTypeChecker): 对象类型判定体系。
//!
//! SINGLE 桶 = 精确比较, RANGE 桶 = FIRST_/LAST_ 区间比较,
//! 均由 metagen 求解树机械展开; 自定义检查器为字符串位域与组合判定。
//! 对 Tagged<Map> 的判定经 objects::Cast 体系消费本模块 (等价非 static-roots 路径)。

use crate::instance_type::{
  InstanceType, K_CONS_STRING_TAG, K_EXTERNAL_ONE_BYTE_STRING_TAG, K_EXTERNAL_STRING_TAG,
  K_EXTERNAL_TWO_BYTE_STRING_TAG, K_IS_INDIRECT_STRING_MASK, K_IS_INDIRECT_STRING_TAG,
  K_IS_NOT_STRING_MASK, K_ONE_BYTE_STRING_TAG, K_SEQ_ONE_BYTE_STRING_TAG, K_SEQ_STRING_TAG,
  K_SEQ_TWO_BYTE_STRING_TAG, K_SHARED_STRING_MASK, K_SHARED_STRING_TAG, K_SLICED_STRING_TAG,
  K_STRING_ENCODING_MASK, K_STRING_REPRESENTATION_AND_ENCODING_MASK, K_STRING_REPRESENTATION_MASK,
  K_STRING_TAG, K_THIN_STRING_TAG, K_TWO_BYTE_STRING_TAG, K_UNCACHED_EXTERNAL_STRING_MASK,
  K_UNCACHED_EXTERNAL_STRING_TAG, LAST_CUSTOM_ELEMENTS_RECEIVER, LAST_SPECIAL_RECEIVER_TYPE,
  PROPERTY_DICTIONARY_TYPE,
};

// ── 单类型检查器 (INSTANCE_TYPE_CHECKERS_SINGLE, INSTANCE_TYPE_CHECKER1 展开) ──
/// 原版 InstanceTypeChecker::IsSymbol (== SYMBOL_TYPE)。
#[inline]
#[must_use]
pub const fn is_symbol(ty: InstanceType) -> bool {
  ty.raw() == 128
}

/// 原版 InstanceTypeChecker::IsBigIntBase (== BIG_INT_BASE_TYPE)。
#[inline]
#[must_use]
pub const fn is_big_int_base(ty: InstanceType) -> bool {
  ty.raw() == 129
}

/// 原版 InstanceTypeChecker::IsHeapNumber (== HEAP_NUMBER_TYPE)。
#[inline]
#[must_use]
pub const fn is_heap_number(ty: InstanceType) -> bool {
  ty.raw() == 130
}

/// 原版 InstanceTypeChecker::IsOddball (== ODDBALL_TYPE)。
#[inline]
#[must_use]
pub const fn is_oddball(ty: InstanceType) -> bool {
  ty.raw() == 131
}

/// 原版 InstanceTypeChecker::IsPromiseFulfillReactionJobTask (== PROMISE_FULFILL_REACTION_JOB_TASK_TYPE)。
#[inline]
#[must_use]
pub const fn is_promise_fulfill_reaction_job_task(ty: InstanceType) -> bool {
  ty.raw() == 132
}

/// 原版 InstanceTypeChecker::IsPromiseRejectReactionJobTask (== PROMISE_REJECT_REACTION_JOB_TASK_TYPE)。
#[inline]
#[must_use]
pub const fn is_promise_reject_reaction_job_task(ty: InstanceType) -> bool {
  ty.raw() == 133
}

/// 原版 InstanceTypeChecker::IsAsyncResumeTask (== ASYNC_RESUME_TASK_TYPE)。
#[inline]
#[must_use]
pub const fn is_async_resume_task(ty: InstanceType) -> bool {
  ty.raw() == 134
}

/// 原版 InstanceTypeChecker::IsCallableTask (== CALLABLE_TASK_TYPE)。
#[inline]
#[must_use]
pub const fn is_callable_task(ty: InstanceType) -> bool {
  ty.raw() == 135
}

/// 原版 InstanceTypeChecker::IsCallbackTask (== CALLBACK_TASK_TYPE)。
#[inline]
#[must_use]
pub const fn is_callback_task(ty: InstanceType) -> bool {
  ty.raw() == 136
}

/// 原版 InstanceTypeChecker::IsPromiseResolveThenableJobTask (== PROMISE_RESOLVE_THENABLE_JOB_TASK_TYPE)。
#[inline]
#[must_use]
pub const fn is_promise_resolve_thenable_job_task(ty: InstanceType) -> bool {
  ty.raw() == 137
}

/// 原版 InstanceTypeChecker::IsLoadHandler (== LOAD_HANDLER_TYPE)。
#[inline]
#[must_use]
pub const fn is_load_handler(ty: InstanceType) -> bool {
  ty.raw() == 138
}

/// 原版 InstanceTypeChecker::IsStoreHandler (== STORE_HANDLER_TYPE)。
#[inline]
#[must_use]
pub const fn is_store_handler(ty: InstanceType) -> bool {
  ty.raw() == 139
}

/// 原版 InstanceTypeChecker::IsAccessCheckInfo (== ACCESS_CHECK_INFO_TYPE)。
#[inline]
#[must_use]
pub const fn is_access_check_info(ty: InstanceType) -> bool {
  ty.raw() == 140
}

/// 原版 InstanceTypeChecker::IsAccessorPair (== ACCESSOR_PAIR_TYPE)。
#[inline]
#[must_use]
pub const fn is_accessor_pair(ty: InstanceType) -> bool {
  ty.raw() == 141
}

/// 原版 InstanceTypeChecker::IsAliasedArgumentsEntry (== ALIASED_ARGUMENTS_ENTRY_TYPE)。
#[inline]
#[must_use]
pub const fn is_aliased_arguments_entry(ty: InstanceType) -> bool {
  ty.raw() == 142
}

/// 原版 InstanceTypeChecker::IsAllocationMemento (== ALLOCATION_MEMENTO_TYPE)。
#[inline]
#[must_use]
pub const fn is_allocation_memento(ty: InstanceType) -> bool {
  ty.raw() == 143
}

/// 原版 InstanceTypeChecker::IsArrayBoilerplateDescription (== ARRAY_BOILERPLATE_DESCRIPTION_TYPE)。
#[inline]
#[must_use]
pub const fn is_array_boilerplate_description(ty: InstanceType) -> bool {
  ty.raw() == 144
}

/// 原版 InstanceTypeChecker::IsAsyncGeneratorRequest (== ASYNC_GENERATOR_REQUEST_TYPE)。
#[inline]
#[must_use]
pub const fn is_async_generator_request(ty: InstanceType) -> bool {
  ty.raw() == 145
}

/// 原版 InstanceTypeChecker::IsBreakPoint (== BREAK_POINT_TYPE)。
#[inline]
#[must_use]
pub const fn is_break_point(ty: InstanceType) -> bool {
  ty.raw() == 146
}

/// 原版 InstanceTypeChecker::IsBreakPointInfo (== BREAK_POINT_INFO_TYPE)。
#[inline]
#[must_use]
pub const fn is_break_point_info(ty: InstanceType) -> bool {
  ty.raw() == 147
}

/// 原版 InstanceTypeChecker::IsBytecodeWrapper (== BYTECODE_WRAPPER_TYPE)。
#[inline]
#[must_use]
pub const fn is_bytecode_wrapper(ty: InstanceType) -> bool {
  ty.raw() == 148
}

/// 原版 InstanceTypeChecker::IsCallSiteInfo (== CALL_SITE_INFO_TYPE)。
#[inline]
#[must_use]
pub const fn is_call_site_info(ty: InstanceType) -> bool {
  ty.raw() == 149
}

/// 原版 InstanceTypeChecker::IsClassBoilerplate (== CLASS_BOILERPLATE_TYPE)。
#[inline]
#[must_use]
pub const fn is_class_boilerplate(ty: InstanceType) -> bool {
  ty.raw() == 150
}

/// 原版 InstanceTypeChecker::IsClassPositions (== CLASS_POSITIONS_TYPE)。
#[inline]
#[must_use]
pub const fn is_class_positions(ty: InstanceType) -> bool {
  ty.raw() == 151
}

/// 原版 InstanceTypeChecker::IsCodeWrapper (== CODE_WRAPPER_TYPE)。
#[inline]
#[must_use]
pub const fn is_code_wrapper(ty: InstanceType) -> bool {
  ty.raw() == 152
}

/// 原版 InstanceTypeChecker::IsDebugScriptScopeInfo (== DEBUG_SCRIPT_SCOPE_INFO_TYPE)。
#[inline]
#[must_use]
pub const fn is_debug_script_scope_info(ty: InstanceType) -> bool {
  ty.raw() == 153
}

/// 原版 InstanceTypeChecker::IsEnumCache (== ENUM_CACHE_TYPE)。
#[inline]
#[must_use]
pub const fn is_enum_cache(ty: InstanceType) -> bool {
  ty.raw() == 154
}

/// 原版 InstanceTypeChecker::IsErrorStackData (== ERROR_STACK_DATA_TYPE)。
#[inline]
#[must_use]
pub const fn is_error_stack_data(ty: InstanceType) -> bool {
  ty.raw() == 155
}

/// 原版 InstanceTypeChecker::IsFeedbackCell (== FEEDBACK_CELL_TYPE)。
#[inline]
#[must_use]
pub const fn is_feedback_cell(ty: InstanceType) -> bool {
  ty.raw() == 156
}

/// 原版 InstanceTypeChecker::IsForInEnumeratorHolder (== FOR_IN_ENUMERATOR_HOLDER_TYPE)。
#[inline]
#[must_use]
pub const fn is_for_in_enumerator_holder(ty: InstanceType) -> bool {
  ty.raw() == 157
}

/// 原版 InstanceTypeChecker::IsFunctionTemplateRareData (== FUNCTION_TEMPLATE_RARE_DATA_TYPE)。
#[inline]
#[must_use]
pub const fn is_function_template_rare_data(ty: InstanceType) -> bool {
  ty.raw() == 158
}

/// 原版 InstanceTypeChecker::IsModuleRequest (== MODULE_REQUEST_TYPE)。
#[inline]
#[must_use]
pub const fn is_module_request(ty: InstanceType) -> bool {
  ty.raw() == 159
}

/// 原版 InstanceTypeChecker::IsPromiseCapability (== PROMISE_CAPABILITY_TYPE)。
#[inline]
#[must_use]
pub const fn is_promise_capability(ty: InstanceType) -> bool {
  ty.raw() == 160
}

/// 原版 InstanceTypeChecker::IsPromiseReaction (== PROMISE_REACTION_TYPE)。
#[inline]
#[must_use]
pub const fn is_promise_reaction(ty: InstanceType) -> bool {
  ty.raw() == 161
}

/// 原版 InstanceTypeChecker::IsPropertyDescriptorObject (== PROPERTY_DESCRIPTOR_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_property_descriptor_object(ty: InstanceType) -> bool {
  ty.raw() == 162
}

/// 原版 InstanceTypeChecker::IsPrototypeInfo (== PROTOTYPE_INFO_TYPE)。
#[inline]
#[must_use]
pub const fn is_prototype_info(ty: InstanceType) -> bool {
  ty.raw() == 163
}

/// 原版 InstanceTypeChecker::IsPrototypeSharedClosureInfo (== PROTOTYPE_SHARED_CLOSURE_INFO_TYPE)。
#[inline]
#[must_use]
pub const fn is_prototype_shared_closure_info(ty: InstanceType) -> bool {
  ty.raw() == 164
}

/// 原版 InstanceTypeChecker::IsRegExpBoilerplateDescription (== REG_EXP_BOILERPLATE_DESCRIPTION_TYPE)。
#[inline]
#[must_use]
pub const fn is_reg_exp_boilerplate_description(ty: InstanceType) -> bool {
  ty.raw() == 165
}

/// 原版 InstanceTypeChecker::IsRegExpDataWrapper (== REG_EXP_DATA_WRAPPER_TYPE)。
#[inline]
#[must_use]
pub const fn is_reg_exp_data_wrapper(ty: InstanceType) -> bool {
  ty.raw() == 166
}

/// 原版 InstanceTypeChecker::IsScript (== SCRIPT_TYPE)。
#[inline]
#[must_use]
pub const fn is_script(ty: InstanceType) -> bool {
  ty.raw() == 167
}

/// 原版 InstanceTypeChecker::IsScriptOrModule (== SCRIPT_OR_MODULE_TYPE)。
#[inline]
#[must_use]
pub const fn is_script_or_module(ty: InstanceType) -> bool {
  ty.raw() == 168
}

/// 原版 InstanceTypeChecker::IsSourceTextModuleInfoEntry (== SOURCE_TEXT_MODULE_INFO_ENTRY_TYPE)。
#[inline]
#[must_use]
pub const fn is_source_text_module_info_entry(ty: InstanceType) -> bool {
  ty.raw() == 169
}

/// 原版 InstanceTypeChecker::IsStackFrameInfo (== STACK_FRAME_INFO_TYPE)。
#[inline]
#[must_use]
pub const fn is_stack_frame_info(ty: InstanceType) -> bool {
  ty.raw() == 170
}

/// 原版 InstanceTypeChecker::IsStackTraceInfo (== STACK_TRACE_INFO_TYPE)。
#[inline]
#[must_use]
pub const fn is_stack_trace_info(ty: InstanceType) -> bool {
  ty.raw() == 171
}

/// 原版 InstanceTypeChecker::IsTemplateObjectDescription (== TEMPLATE_OBJECT_DESCRIPTION_TYPE)。
#[inline]
#[must_use]
pub const fn is_template_object_description(ty: InstanceType) -> bool {
  ty.raw() == 172
}

/// 原版 InstanceTypeChecker::IsTuple2 (== TUPLE2_TYPE)。
#[inline]
#[must_use]
pub const fn is_tuple2(ty: InstanceType) -> bool {
  ty.raw() == 173
}

/// 原版 InstanceTypeChecker::IsWasmExceptionTag (== WASM_EXCEPTION_TAG_TYPE)。
#[inline]
#[must_use]
pub const fn is_wasm_exception_tag(ty: InstanceType) -> bool {
  ty.raw() == 174
}

/// 原版 InstanceTypeChecker::IsUncompiledDataWithPreparseDataAndJob (== UNCOMPILED_DATA_WITH_PREPARSE_DATA_AND_JOB_TYPE)。
#[inline]
#[must_use]
pub const fn is_uncompiled_data_with_preparse_data_and_job(ty: InstanceType) -> bool {
  ty.raw() == 176
}

/// 原版 InstanceTypeChecker::IsUncompiledDataWithoutPreparseDataWithJob (== UNCOMPILED_DATA_WITHOUT_PREPARSE_DATA_WITH_JOB_TYPE)。
#[inline]
#[must_use]
pub const fn is_uncompiled_data_without_preparse_data_with_job(ty: InstanceType) -> bool {
  ty.raw() == 178
}

/// 原版 InstanceTypeChecker::IsAtomRegExpData (== ATOM_REG_EXP_DATA_TYPE)。
#[inline]
#[must_use]
pub const fn is_atom_reg_exp_data(ty: InstanceType) -> bool {
  ty.raw() == 180
}

/// 原版 InstanceTypeChecker::IsIrRegExpData (== IR_REG_EXP_DATA_TYPE)。
#[inline]
#[must_use]
pub const fn is_ir_reg_exp_data(ty: InstanceType) -> bool {
  ty.raw() == 181
}

/// 原版 InstanceTypeChecker::IsWasmCapiFunctionData (== WASM_CAPI_FUNCTION_DATA_TYPE)。
#[inline]
#[must_use]
pub const fn is_wasm_capi_function_data(ty: InstanceType) -> bool {
  ty.raw() == 183
}

/// 原版 InstanceTypeChecker::IsWasmExportedFunctionData (== WASM_EXPORTED_FUNCTION_DATA_TYPE)。
#[inline]
#[must_use]
pub const fn is_wasm_exported_function_data(ty: InstanceType) -> bool {
  ty.raw() == 184
}

/// 原版 InstanceTypeChecker::IsBytecodeArray (== BYTECODE_ARRAY_TYPE)。
#[inline]
#[must_use]
pub const fn is_bytecode_array(ty: InstanceType) -> bool {
  ty.raw() == 185
}

/// 原版 InstanceTypeChecker::IsCode (== CODE_TYPE)。
#[inline]
#[must_use]
pub const fn is_code(ty: InstanceType) -> bool {
  ty.raw() == 186
}

/// 原版 InstanceTypeChecker::IsDebugInfo (== DEBUG_INFO_TYPE)。
#[inline]
#[must_use]
pub const fn is_debug_info(ty: InstanceType) -> bool {
  ty.raw() == 187
}

/// 原版 InstanceTypeChecker::IsInterpreterData (== INTERPRETER_DATA_TYPE)。
#[inline]
#[must_use]
pub const fn is_interpreter_data(ty: InstanceType) -> bool {
  ty.raw() == 188
}

/// 原版 InstanceTypeChecker::IsWasmDispatchTable (== WASM_DISPATCH_TABLE_TYPE)。
#[inline]
#[must_use]
pub const fn is_wasm_dispatch_table(ty: InstanceType) -> bool {
  ty.raw() == 189
}

/// 原版 InstanceTypeChecker::IsWasmInternalFunction (== WASM_INTERNAL_FUNCTION_TYPE)。
#[inline]
#[must_use]
pub const fn is_wasm_internal_function(ty: InstanceType) -> bool {
  ty.raw() == 190
}

/// 原版 InstanceTypeChecker::IsWasmSuspenderObject (== WASM_SUSPENDER_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_wasm_suspender_object(ty: InstanceType) -> bool {
  ty.raw() == 191
}

/// 原版 InstanceTypeChecker::IsWasmTrustedInstanceData (== WASM_TRUSTED_INSTANCE_DATA_TYPE)。
#[inline]
#[must_use]
pub const fn is_wasm_trusted_instance_data(ty: InstanceType) -> bool {
  ty.raw() == 192
}

/// 原版 InstanceTypeChecker::IsProtectedFixedArray (== PROTECTED_FIXED_ARRAY_TYPE)。
#[inline]
#[must_use]
pub const fn is_protected_fixed_array(ty: InstanceType) -> bool {
  ty.raw() == 193
}

/// 原版 InstanceTypeChecker::IsProtectedWeakFixedArray (== PROTECTED_WEAK_FIXED_ARRAY_TYPE)。
#[inline]
#[must_use]
pub const fn is_protected_weak_fixed_array(ty: InstanceType) -> bool {
  ty.raw() == 194
}

/// 原版 InstanceTypeChecker::IsTrustedByteArray (== TRUSTED_BYTE_ARRAY_TYPE)。
#[inline]
#[must_use]
pub const fn is_trusted_byte_array(ty: InstanceType) -> bool {
  ty.raw() == 195
}

/// 原版 InstanceTypeChecker::IsTrustedFixedArray (== TRUSTED_FIXED_ARRAY_TYPE)。
#[inline]
#[must_use]
pub const fn is_trusted_fixed_array(ty: InstanceType) -> bool {
  ty.raw() == 196
}

/// 原版 InstanceTypeChecker::IsTrustedWeakFixedArray (== TRUSTED_WEAK_FIXED_ARRAY_TYPE)。
#[inline]
#[must_use]
pub const fn is_trusted_weak_fixed_array(ty: InstanceType) -> bool {
  ty.raw() == 197
}

/// 原版 InstanceTypeChecker::IsInstructionStream (== INSTRUCTION_STREAM_TYPE)。
#[inline]
#[must_use]
pub const fn is_instruction_stream(ty: InstanceType) -> bool {
  ty.raw() == 198
}

/// 原版 InstanceTypeChecker::IsSharedFunctionInfoWrapper (== SHARED_FUNCTION_INFO_WRAPPER_TYPE)。
#[inline]
#[must_use]
pub const fn is_shared_function_info_wrapper(ty: InstanceType) -> bool {
  ty.raw() == 199
}

/// 原版 InstanceTypeChecker::IsTrustedForeign (== TRUSTED_FOREIGN_TYPE)。
#[inline]
#[must_use]
pub const fn is_trusted_foreign(ty: InstanceType) -> bool {
  ty.raw() == 200
}

/// 原版 InstanceTypeChecker::IsWasmDispatchTableForImports (== WASM_DISPATCH_TABLE_FOR_IMPORTS_TYPE)。
#[inline]
#[must_use]
pub const fn is_wasm_dispatch_table_for_imports(ty: InstanceType) -> bool {
  ty.raw() == 201
}

/// 原版 InstanceTypeChecker::IsWasmImportData (== WASM_IMPORT_DATA_TYPE)。
#[inline]
#[must_use]
pub const fn is_wasm_import_data(ty: InstanceType) -> bool {
  ty.raw() == 202
}

/// 原版 InstanceTypeChecker::IsAccessorInfo (== ACCESSOR_INFO_TYPE)。
#[inline]
#[must_use]
pub const fn is_accessor_info(ty: InstanceType) -> bool {
  ty.raw() == 203
}

/// 原版 InstanceTypeChecker::IsForeign (== FOREIGN_TYPE)。
#[inline]
#[must_use]
pub const fn is_foreign(ty: InstanceType) -> bool {
  ty.raw() == 204
}

/// 原版 InstanceTypeChecker::IsBaseNameDictionary (== BASE_NAME_DICTIONARY_TYPE)。
#[inline]
#[must_use]
pub const fn is_base_name_dictionary(ty: InstanceType) -> bool {
  ty.raw() == 206
}

/// 原版 InstanceTypeChecker::IsDictionary (== DICTIONARY_TYPE)。
#[inline]
#[must_use]
pub const fn is_dictionary(ty: InstanceType) -> bool {
  ty.raw() == 207
}

/// 原版 InstanceTypeChecker::IsEphemeronHashTable (== EPHEMERON_HASH_TABLE_TYPE)。
#[inline]
#[must_use]
pub const fn is_ephemeron_hash_table(ty: InstanceType) -> bool {
  ty.raw() == 208
}

/// 原版 InstanceTypeChecker::IsGlobalDictionary (== GLOBAL_DICTIONARY_TYPE)。
#[inline]
#[must_use]
pub const fn is_global_dictionary(ty: InstanceType) -> bool {
  ty.raw() == 209
}

/// 原版 InstanceTypeChecker::IsHashTable (== HASH_TABLE_TYPE)。
#[inline]
#[must_use]
pub const fn is_hash_table(ty: InstanceType) -> bool {
  ty.raw() == 210
}

/// 原版 InstanceTypeChecker::IsNameDictionary (== NAME_DICTIONARY_TYPE)。
#[inline]
#[must_use]
pub const fn is_name_dictionary(ty: InstanceType) -> bool {
  ty.raw() == 211
}

/// 原版 InstanceTypeChecker::IsNameToIndexHashTable (== NAME_TO_INDEX_HASH_TABLE_TYPE)。
#[inline]
#[must_use]
pub const fn is_name_to_index_hash_table(ty: InstanceType) -> bool {
  ty.raw() == 212
}

/// 原版 InstanceTypeChecker::IsNumberDictionary (== NUMBER_DICTIONARY_TYPE)。
#[inline]
#[must_use]
pub const fn is_number_dictionary(ty: InstanceType) -> bool {
  ty.raw() == 213
}

/// 原版 InstanceTypeChecker::IsObjectHashTableBase (== OBJECT_HASH_TABLE_BASE_TYPE)。
#[inline]
#[must_use]
pub const fn is_object_hash_table_base(ty: InstanceType) -> bool {
  ty.raw() == 214
}

/// 原版 InstanceTypeChecker::IsOrderedHashMap (== ORDERED_HASH_MAP_TYPE)。
#[inline]
#[must_use]
pub const fn is_ordered_hash_map(ty: InstanceType) -> bool {
  ty.raw() == 215
}

/// 原版 InstanceTypeChecker::IsOrderedHashSet (== ORDERED_HASH_SET_TYPE)。
#[inline]
#[must_use]
pub const fn is_ordered_hash_set(ty: InstanceType) -> bool {
  ty.raw() == 216
}

/// 原版 InstanceTypeChecker::IsOrderedHashTable (== ORDERED_HASH_TABLE_TYPE)。
#[inline]
#[must_use]
pub const fn is_ordered_hash_table(ty: InstanceType) -> bool {
  ty.raw() == 217
}

/// 原版 InstanceTypeChecker::IsOrderedNameDictionary (== ORDERED_NAME_DICTIONARY_TYPE)。
#[inline]
#[must_use]
pub const fn is_ordered_name_dictionary(ty: InstanceType) -> bool {
  ty.raw() == 218
}

/// 原版 InstanceTypeChecker::IsRegisteredSymbolTable (== REGISTERED_SYMBOL_TABLE_TYPE)。
#[inline]
#[must_use]
pub const fn is_registered_symbol_table(ty: InstanceType) -> bool {
  ty.raw() == 219
}

/// 原版 InstanceTypeChecker::IsSimpleNameDictionary (== SIMPLE_NAME_DICTIONARY_TYPE)。
#[inline]
#[must_use]
pub const fn is_simple_name_dictionary(ty: InstanceType) -> bool {
  ty.raw() == 220
}

/// 原版 InstanceTypeChecker::IsSimpleNumberDictionary (== SIMPLE_NUMBER_DICTIONARY_TYPE)。
#[inline]
#[must_use]
pub const fn is_simple_number_dictionary(ty: InstanceType) -> bool {
  ty.raw() == 221
}

/// 原版 InstanceTypeChecker::IsTransitionArray (== TRANSITION_ARRAY_TYPE)。
#[inline]
#[must_use]
pub const fn is_transition_array(ty: InstanceType) -> bool {
  ty.raw() == 223
}

/// 原版 InstanceTypeChecker::IsByteArray (== BYTE_ARRAY_TYPE)。
#[inline]
#[must_use]
pub const fn is_byte_array(ty: InstanceType) -> bool {
  ty.raw() == 224
}

/// 原版 InstanceTypeChecker::IsClosureFeedbackCellArray (== CLOSURE_FEEDBACK_CELL_ARRAY_TYPE)。
#[inline]
#[must_use]
pub const fn is_closure_feedback_cell_array(ty: InstanceType) -> bool {
  ty.raw() == 225
}

/// 原版 InstanceTypeChecker::IsFixedDoubleArray (== FIXED_DOUBLE_ARRAY_TYPE)。
#[inline]
#[must_use]
pub const fn is_fixed_double_array(ty: InstanceType) -> bool {
  ty.raw() == 226
}

/// 原版 InstanceTypeChecker::IsObjectBoilerplateDescription (== OBJECT_BOILERPLATE_DESCRIPTION_TYPE)。
#[inline]
#[must_use]
pub const fn is_object_boilerplate_description(ty: InstanceType) -> bool {
  ty.raw() == 227
}

/// 原版 InstanceTypeChecker::IsRegExpMatchInfo (== REG_EXP_MATCH_INFO_TYPE)。
#[inline]
#[must_use]
pub const fn is_reg_exp_match_info(ty: InstanceType) -> bool {
  ty.raw() == 228
}

/// 原版 InstanceTypeChecker::IsSloppyArgumentsElements (== SLOPPY_ARGUMENTS_ELEMENTS_TYPE)。
#[inline]
#[must_use]
pub const fn is_sloppy_arguments_elements(ty: InstanceType) -> bool {
  ty.raw() == 229
}

/// 原版 InstanceTypeChecker::IsWeakHomomorphicFixedArray (== WEAK_HOMOMORPHIC_FIXED_ARRAY_TYPE)。
#[inline]
#[must_use]
pub const fn is_weak_homomorphic_fixed_array(ty: InstanceType) -> bool {
  ty.raw() == 230
}

/// 原版 InstanceTypeChecker::IsAwaitContext (== AWAIT_CONTEXT_TYPE)。
#[inline]
#[must_use]
pub const fn is_await_context(ty: InstanceType) -> bool {
  ty.raw() == 231
}

/// 原版 InstanceTypeChecker::IsBlockContext (== BLOCK_CONTEXT_TYPE)。
#[inline]
#[must_use]
pub const fn is_block_context(ty: InstanceType) -> bool {
  ty.raw() == 232
}

/// 原版 InstanceTypeChecker::IsCatchContext (== CATCH_CONTEXT_TYPE)。
#[inline]
#[must_use]
pub const fn is_catch_context(ty: InstanceType) -> bool {
  ty.raw() == 233
}

/// 原版 InstanceTypeChecker::IsDebugEvaluateContext (== DEBUG_EVALUATE_CONTEXT_TYPE)。
#[inline]
#[must_use]
pub const fn is_debug_evaluate_context(ty: InstanceType) -> bool {
  ty.raw() == 234
}

/// 原版 InstanceTypeChecker::IsEvalContext (== EVAL_CONTEXT_TYPE)。
#[inline]
#[must_use]
pub const fn is_eval_context(ty: InstanceType) -> bool {
  ty.raw() == 235
}

/// 原版 InstanceTypeChecker::IsFunctionContext (== FUNCTION_CONTEXT_TYPE)。
#[inline]
#[must_use]
pub const fn is_function_context(ty: InstanceType) -> bool {
  ty.raw() == 236
}

/// 原版 InstanceTypeChecker::IsModuleContext (== MODULE_CONTEXT_TYPE)。
#[inline]
#[must_use]
pub const fn is_module_context(ty: InstanceType) -> bool {
  ty.raw() == 237
}

/// 原版 InstanceTypeChecker::IsNativeContext (== NATIVE_CONTEXT_TYPE)。
#[inline]
#[must_use]
pub const fn is_native_context(ty: InstanceType) -> bool {
  ty.raw() == 238
}

/// 原版 InstanceTypeChecker::IsScriptContext (== SCRIPT_CONTEXT_TYPE)。
#[inline]
#[must_use]
pub const fn is_script_context(ty: InstanceType) -> bool {
  ty.raw() == 239
}

/// 原版 InstanceTypeChecker::IsWithContext (== WITH_CONTEXT_TYPE)。
#[inline]
#[must_use]
pub const fn is_with_context(ty: InstanceType) -> bool {
  ty.raw() == 240
}

/// 原版 InstanceTypeChecker::IsTurboshaftFloat64RangeType (== TURBOSHAFT_FLOAT64_RANGE_TYPE_TYPE)。
#[inline]
#[must_use]
pub const fn is_turboshaft_float64_range_type(ty: InstanceType) -> bool {
  ty.raw() == 241
}

/// 原版 InstanceTypeChecker::IsTurboshaftFloat64SetType (== TURBOSHAFT_FLOAT64_SET_TYPE_TYPE)。
#[inline]
#[must_use]
pub const fn is_turboshaft_float64_set_type(ty: InstanceType) -> bool {
  ty.raw() == 242
}

/// 原版 InstanceTypeChecker::IsTurboshaftWord32RangeType (== TURBOSHAFT_WORD32_RANGE_TYPE_TYPE)。
#[inline]
#[must_use]
pub const fn is_turboshaft_word32_range_type(ty: InstanceType) -> bool {
  ty.raw() == 243
}

/// 原版 InstanceTypeChecker::IsTurboshaftWord32SetType (== TURBOSHAFT_WORD32_SET_TYPE_TYPE)。
#[inline]
#[must_use]
pub const fn is_turboshaft_word32_set_type(ty: InstanceType) -> bool {
  ty.raw() == 244
}

/// 原版 InstanceTypeChecker::IsTurboshaftWord64RangeType (== TURBOSHAFT_WORD64_RANGE_TYPE_TYPE)。
#[inline]
#[must_use]
pub const fn is_turboshaft_word64_range_type(ty: InstanceType) -> bool {
  ty.raw() == 245
}

/// 原版 InstanceTypeChecker::IsTurboshaftWord64SetType (== TURBOSHAFT_WORD64_SET_TYPE_TYPE)。
#[inline]
#[must_use]
pub const fn is_turboshaft_word64_set_type(ty: InstanceType) -> bool {
  ty.raw() == 246
}

/// 原版 InstanceTypeChecker::IsTurbofanBitsetType (== TURBOFAN_BITSET_TYPE_TYPE)。
#[inline]
#[must_use]
pub const fn is_turbofan_bitset_type(ty: InstanceType) -> bool {
  ty.raw() == 247
}

/// 原版 InstanceTypeChecker::IsTurbofanHeapConstantType (== TURBOFAN_HEAP_CONSTANT_TYPE_TYPE)。
#[inline]
#[must_use]
pub const fn is_turbofan_heap_constant_type(ty: InstanceType) -> bool {
  ty.raw() == 248
}

/// 原版 InstanceTypeChecker::IsTurbofanOtherNumberConstantType (== TURBOFAN_OTHER_NUMBER_CONSTANT_TYPE_TYPE)。
#[inline]
#[must_use]
pub const fn is_turbofan_other_number_constant_type(ty: InstanceType) -> bool {
  ty.raw() == 249
}

/// 原版 InstanceTypeChecker::IsTurbofanRangeType (== TURBOFAN_RANGE_TYPE_TYPE)。
#[inline]
#[must_use]
pub const fn is_turbofan_range_type(ty: InstanceType) -> bool {
  ty.raw() == 250
}

/// 原版 InstanceTypeChecker::IsTurbofanUnionType (== TURBOFAN_UNION_TYPE_TYPE)。
#[inline]
#[must_use]
pub const fn is_turbofan_union_type(ty: InstanceType) -> bool {
  ty.raw() == 251
}

/// 原版 InstanceTypeChecker::IsSmallOrderedHashMap (== SMALL_ORDERED_HASH_MAP_TYPE)。
#[inline]
#[must_use]
pub const fn is_small_ordered_hash_map(ty: InstanceType) -> bool {
  ty.raw() == 252
}

/// 原版 InstanceTypeChecker::IsSmallOrderedHashSet (== SMALL_ORDERED_HASH_SET_TYPE)。
#[inline]
#[must_use]
pub const fn is_small_ordered_hash_set(ty: InstanceType) -> bool {
  ty.raw() == 253
}

/// 原版 InstanceTypeChecker::IsSmallOrderedHashTableImpl (== SMALL_ORDERED_HASH_TABLE_IMPL_TYPE)。
#[inline]
#[must_use]
pub const fn is_small_ordered_hash_table_impl(ty: InstanceType) -> bool {
  ty.raw() == 254
}

/// 原版 InstanceTypeChecker::IsSmallOrderedNameDictionary (== SMALL_ORDERED_NAME_DICTIONARY_TYPE)。
#[inline]
#[must_use]
pub const fn is_small_ordered_name_dictionary(ty: InstanceType) -> bool {
  ty.raw() == 255
}

/// 原版 InstanceTypeChecker::IsFunctionTemplateInfo (== FUNCTION_TEMPLATE_INFO_TYPE)。
#[inline]
#[must_use]
pub const fn is_function_template_info(ty: InstanceType) -> bool {
  ty.raw() == 256
}

/// 原版 InstanceTypeChecker::IsObjectTemplateInfo (== OBJECT_TEMPLATE_INFO_TYPE)。
#[inline]
#[must_use]
pub const fn is_object_template_info(ty: InstanceType) -> bool {
  ty.raw() == 257
}

/// 原版 InstanceTypeChecker::IsDictionaryTemplateInfo (== DICTIONARY_TEMPLATE_INFO_TYPE)。
#[inline]
#[must_use]
pub const fn is_dictionary_template_info(ty: InstanceType) -> bool {
  ty.raw() == 258
}

/// 原版 InstanceTypeChecker::IsWasmCustomMap (== WASM_CUSTOM_MAP_TYPE)。
#[inline]
#[must_use]
pub const fn is_wasm_custom_map(ty: InstanceType) -> bool {
  ty.raw() == 260
}

/// 原版 InstanceTypeChecker::IsSourceTextModule (== SOURCE_TEXT_MODULE_TYPE)。
#[inline]
#[must_use]
pub const fn is_source_text_module(ty: InstanceType) -> bool {
  ty.raw() == 261
}

/// 原版 InstanceTypeChecker::IsSyntheticModule (== SYNTHETIC_MODULE_TYPE)。
#[inline]
#[must_use]
pub const fn is_synthetic_module(ty: InstanceType) -> bool {
  ty.raw() == 262
}

/// 原版 InstanceTypeChecker::IsAllocationSite (== ALLOCATION_SITE_TYPE)。
#[inline]
#[must_use]
pub const fn is_allocation_site(ty: InstanceType) -> bool {
  ty.raw() == 263
}

/// 原版 InstanceTypeChecker::IsArrayList (== ARRAY_LIST_TYPE)。
#[inline]
#[must_use]
pub const fn is_array_list(ty: InstanceType) -> bool {
  ty.raw() == 264
}

/// 原版 InstanceTypeChecker::IsCell (== CELL_TYPE)。
#[inline]
#[must_use]
pub const fn is_cell(ty: InstanceType) -> bool {
  ty.raw() == 265
}

/// 原版 InstanceTypeChecker::IsContextCell (== CONTEXT_CELL_TYPE)。
#[inline]
#[must_use]
pub const fn is_context_cell(ty: InstanceType) -> bool {
  ty.raw() == 266
}

/// 原版 InstanceTypeChecker::IsCoverageInfo (== COVERAGE_INFO_TYPE)。
#[inline]
#[must_use]
pub const fn is_coverage_info(ty: InstanceType) -> bool {
  ty.raw() == 267
}

/// 原版 InstanceTypeChecker::IsCppGCManagedBase (== CPP_GCMANAGED_BASE_TYPE)。
#[inline]
#[must_use]
pub const fn is_cpp_gc_managed_base(ty: InstanceType) -> bool {
  ty.raw() == 268
}

/// 原版 InstanceTypeChecker::IsCppHeapExternalObject (== CPP_HEAP_EXTERNAL_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_cpp_heap_external_object(ty: InstanceType) -> bool {
  ty.raw() == 269
}

/// 原版 InstanceTypeChecker::IsDescriptorArray (== DESCRIPTOR_ARRAY_TYPE)。
#[inline]
#[must_use]
pub const fn is_descriptor_array(ty: InstanceType) -> bool {
  ty.raw() == 270
}

/// 原版 InstanceTypeChecker::IsDoubleStringCache (== DOUBLE_STRING_CACHE_TYPE)。
#[inline]
#[must_use]
pub const fn is_double_string_cache(ty: InstanceType) -> bool {
  ty.raw() == 271
}

/// 原版 InstanceTypeChecker::IsEmbedderDataArray (== EMBEDDER_DATA_ARRAY_TYPE)。
#[inline]
#[must_use]
pub const fn is_embedder_data_array(ty: InstanceType) -> bool {
  ty.raw() == 272
}

/// 原版 InstanceTypeChecker::IsFeedbackMetadata (== FEEDBACK_METADATA_TYPE)。
#[inline]
#[must_use]
pub const fn is_feedback_metadata(ty: InstanceType) -> bool {
  ty.raw() == 273
}

/// 原版 InstanceTypeChecker::IsFeedbackVector (== FEEDBACK_VECTOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_feedback_vector(ty: InstanceType) -> bool {
  ty.raw() == 274
}

/// 原版 InstanceTypeChecker::IsFiller (== FILLER_TYPE)。
#[inline]
#[must_use]
pub const fn is_filler(ty: InstanceType) -> bool {
  ty.raw() == 275
}

/// 原版 InstanceTypeChecker::IsFreeSpace (== FREE_SPACE_TYPE)。
#[inline]
#[must_use]
pub const fn is_free_space(ty: InstanceType) -> bool {
  ty.raw() == 276
}

/// 原版 InstanceTypeChecker::IsHashSeedWrapper (== HASH_SEED_WRAPPER_TYPE)。
#[inline]
#[must_use]
pub const fn is_hash_seed_wrapper(ty: InstanceType) -> bool {
  ty.raw() == 277
}

/// 原版 InstanceTypeChecker::IsHole (== HOLE_TYPE)。
#[inline]
#[must_use]
pub const fn is_hole(ty: InstanceType) -> bool {
  ty.raw() == 278
}

/// 原版 InstanceTypeChecker::IsInterceptorInfo (== INTERCEPTOR_INFO_TYPE)。
#[inline]
#[must_use]
pub const fn is_interceptor_info(ty: InstanceType) -> bool {
  ty.raw() == 279
}

/// 原版 InstanceTypeChecker::IsMegaDomHandler (== MEGA_DOM_HANDLER_TYPE)。
#[inline]
#[must_use]
pub const fn is_mega_dom_handler(ty: InstanceType) -> bool {
  ty.raw() == 280
}

/// 原版 InstanceTypeChecker::IsOnHeapBasicBlockProfilerData (== ON_HEAP_BASIC_BLOCK_PROFILER_DATA_TYPE)。
#[inline]
#[must_use]
pub const fn is_on_heap_basic_block_profiler_data(ty: InstanceType) -> bool {
  ty.raw() == 281
}

/// 原版 InstanceTypeChecker::IsPreparseData (== PREPARSE_DATA_TYPE)。
#[inline]
#[must_use]
pub const fn is_preparse_data(ty: InstanceType) -> bool {
  ty.raw() == 282
}

/// 原版 InstanceTypeChecker::IsPropertyArray (== PROPERTY_ARRAY_TYPE)。
#[inline]
#[must_use]
pub const fn is_property_array(ty: InstanceType) -> bool {
  ty.raw() == 283
}

/// 原版 InstanceTypeChecker::IsPropertyCell (== PROPERTY_CELL_TYPE)。
#[inline]
#[must_use]
pub const fn is_property_cell(ty: InstanceType) -> bool {
  ty.raw() == 284
}

/// 原版 InstanceTypeChecker::IsScopeInfo (== SCOPE_INFO_TYPE)。
#[inline]
#[must_use]
pub const fn is_scope_info(ty: InstanceType) -> bool {
  ty.raw() == 285
}

/// 原版 InstanceTypeChecker::IsScriptContextTable (== SCRIPT_CONTEXT_TABLE_TYPE)。
#[inline]
#[must_use]
pub const fn is_script_context_table(ty: InstanceType) -> bool {
  ty.raw() == 286
}

/// 原版 InstanceTypeChecker::IsSharedFunctionInfo (== SHARED_FUNCTION_INFO_TYPE)。
#[inline]
#[must_use]
pub const fn is_shared_function_info(ty: InstanceType) -> bool {
  ty.raw() == 287
}

/// 原版 InstanceTypeChecker::IsSortState (== SORT_STATE_TYPE)。
#[inline]
#[must_use]
pub const fn is_sort_state(ty: InstanceType) -> bool {
  ty.raw() == 288
}

/// 原版 InstanceTypeChecker::IsSwissNameDictionary (== SWISS_NAME_DICTIONARY_TYPE)。
#[inline]
#[must_use]
pub const fn is_swiss_name_dictionary(ty: InstanceType) -> bool {
  ty.raw() == 289
}

/// 原版 InstanceTypeChecker::IsUninitializedHeapNumber (== UNINITIALIZED_HEAP_NUMBER_TYPE)。
#[inline]
#[must_use]
pub const fn is_uninitialized_heap_number(ty: InstanceType) -> bool {
  ty.raw() == 290
}

/// 原版 InstanceTypeChecker::IsWasmContinuationObject (== WASM_CONTINUATION_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_wasm_continuation_object(ty: InstanceType) -> bool {
  ty.raw() == 291
}

/// 原版 InstanceTypeChecker::IsWasmFastApiCallData (== WASM_FAST_API_CALL_DATA_TYPE)。
#[inline]
#[must_use]
pub const fn is_wasm_fast_api_call_data(ty: InstanceType) -> bool {
  ty.raw() == 292
}

/// 原版 InstanceTypeChecker::IsWasmFuncRef (== WASM_FUNC_REF_TYPE)。
#[inline]
#[must_use]
pub const fn is_wasm_func_ref(ty: InstanceType) -> bool {
  ty.raw() == 293
}

/// 原版 InstanceTypeChecker::IsWasmResumeData (== WASM_RESUME_DATA_TYPE)。
#[inline]
#[must_use]
pub const fn is_wasm_resume_data(ty: InstanceType) -> bool {
  ty.raw() == 295
}

/// 原版 InstanceTypeChecker::IsWasmStackObject (== WASM_STACK_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_wasm_stack_object(ty: InstanceType) -> bool {
  ty.raw() == 296
}

/// 原版 InstanceTypeChecker::IsWasmStringViewIter (== WASM_STRING_VIEW_ITER_TYPE)。
#[inline]
#[must_use]
pub const fn is_wasm_string_view_iter(ty: InstanceType) -> bool {
  ty.raw() == 297
}

/// 原版 InstanceTypeChecker::IsWasmTypeInfo (== WASM_TYPE_INFO_TYPE)。
#[inline]
#[must_use]
pub const fn is_wasm_type_info(ty: InstanceType) -> bool {
  ty.raw() == 298
}

/// 原版 InstanceTypeChecker::IsWeakArrayList (== WEAK_ARRAY_LIST_TYPE)。
#[inline]
#[must_use]
pub const fn is_weak_array_list(ty: InstanceType) -> bool {
  ty.raw() == 299
}

/// 原版 InstanceTypeChecker::IsWeakCell (== WEAK_CELL_TYPE)。
#[inline]
#[must_use]
pub const fn is_weak_cell(ty: InstanceType) -> bool {
  ty.raw() == 300
}

/// 原版 InstanceTypeChecker::IsWasmArray (== WASM_ARRAY_TYPE)。
#[inline]
#[must_use]
pub const fn is_wasm_array(ty: InstanceType) -> bool {
  ty.raw() == 301
}

/// 原版 InstanceTypeChecker::IsWasmStruct (== WASM_STRUCT_TYPE)。
#[inline]
#[must_use]
pub const fn is_wasm_struct(ty: InstanceType) -> bool {
  ty.raw() == 302
}

/// 原版 InstanceTypeChecker::IsJSProxy (== JS_PROXY_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_proxy(ty: InstanceType) -> bool {
  ty.raw() == 303
}

/// 原版 InstanceTypeChecker::IsJSDeferredModuleNamespace (== JS_DEFERRED_MODULE_NAMESPACE_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_deferred_module_namespace(ty: InstanceType) -> bool {
  ty.raw() == 305
}

/// 原版 InstanceTypeChecker::IsJSGlobalObject (== JS_GLOBAL_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_global_object(ty: InstanceType) -> bool {
  ty.raw() == 306
}

/// 原版 InstanceTypeChecker::IsJSGlobalProxy (== JS_GLOBAL_PROXY_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_global_proxy(ty: InstanceType) -> bool {
  ty.raw() == 307
}

/// 原版 InstanceTypeChecker::IsJSSpecialApiObject (== JS_SPECIAL_API_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_special_api_object(ty: InstanceType) -> bool {
  ty.raw() == 1040
}

/// 原版 InstanceTypeChecker::IsJSPrimitiveWrapper (== JS_PRIMITIVE_WRAPPER_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_primitive_wrapper(ty: InstanceType) -> bool {
  ty.raw() == 1041
}

/// 原版 InstanceTypeChecker::IsJSLastDummyApiObject (== JS_LAST_DUMMY_API_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_last_dummy_api_object(ty: InstanceType) -> bool {
  ty.raw() == 2058
}

/// 原版 InstanceTypeChecker::IsJSDataView (== JS_DATA_VIEW_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_data_view(ty: InstanceType) -> bool {
  ty.raw() == 2059
}

/// 原版 InstanceTypeChecker::IsJSRabGsabDataView (== JS_RAB_GSAB_DATA_VIEW_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_rab_gsab_data_view(ty: InstanceType) -> bool {
  ty.raw() == 2060
}

/// 原版 InstanceTypeChecker::IsJSDetachedTypedArray (== JS_DETACHED_TYPED_ARRAY_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_detached_typed_array(ty: InstanceType) -> bool {
  ty.raw() == 2062
}

/// 原版 InstanceTypeChecker::IsJSArrayBuffer (== JS_ARRAY_BUFFER_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_array_buffer(ty: InstanceType) -> bool {
  ty.raw() == 2063
}

/// 原版 InstanceTypeChecker::IsJSFunctionWithoutPrototype (== JS_FUNCTION_WITHOUT_PROTOTYPE_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_function_without_prototype(ty: InstanceType) -> bool {
  ty.raw() == 2064
}

/// 原版 InstanceTypeChecker::IsBigint64TypedArrayConstructor (== BIGINT64_TYPED_ARRAY_CONSTRUCTOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_bigint64_typed_array_constructor(ty: InstanceType) -> bool {
  ty.raw() == 2066
}

/// 原版 InstanceTypeChecker::IsBiguint64TypedArrayConstructor (== BIGUINT64_TYPED_ARRAY_CONSTRUCTOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_biguint64_typed_array_constructor(ty: InstanceType) -> bool {
  ty.raw() == 2067
}

/// 原版 InstanceTypeChecker::IsFloat16TypedArrayConstructor (== FLOAT16_TYPED_ARRAY_CONSTRUCTOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_float16_typed_array_constructor(ty: InstanceType) -> bool {
  ty.raw() == 2068
}

/// 原版 InstanceTypeChecker::IsFloat32TypedArrayConstructor (== FLOAT32_TYPED_ARRAY_CONSTRUCTOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_float32_typed_array_constructor(ty: InstanceType) -> bool {
  ty.raw() == 2069
}

/// 原版 InstanceTypeChecker::IsFloat64TypedArrayConstructor (== FLOAT64_TYPED_ARRAY_CONSTRUCTOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_float64_typed_array_constructor(ty: InstanceType) -> bool {
  ty.raw() == 2070
}

/// 原版 InstanceTypeChecker::IsInt16TypedArrayConstructor (== INT16_TYPED_ARRAY_CONSTRUCTOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_int16_typed_array_constructor(ty: InstanceType) -> bool {
  ty.raw() == 2071
}

/// 原版 InstanceTypeChecker::IsInt32TypedArrayConstructor (== INT32_TYPED_ARRAY_CONSTRUCTOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_int32_typed_array_constructor(ty: InstanceType) -> bool {
  ty.raw() == 2072
}

/// 原版 InstanceTypeChecker::IsInt8TypedArrayConstructor (== INT8_TYPED_ARRAY_CONSTRUCTOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_int8_typed_array_constructor(ty: InstanceType) -> bool {
  ty.raw() == 2073
}

/// 原版 InstanceTypeChecker::IsUint16TypedArrayConstructor (== UINT16_TYPED_ARRAY_CONSTRUCTOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_uint16_typed_array_constructor(ty: InstanceType) -> bool {
  ty.raw() == 2074
}

/// 原版 InstanceTypeChecker::IsUint32TypedArrayConstructor (== UINT32_TYPED_ARRAY_CONSTRUCTOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_uint32_typed_array_constructor(ty: InstanceType) -> bool {
  ty.raw() == 2075
}

/// 原版 InstanceTypeChecker::IsUint8ClampedTypedArrayConstructor (== UINT8_CLAMPED_TYPED_ARRAY_CONSTRUCTOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_uint8_clamped_typed_array_constructor(ty: InstanceType) -> bool {
  ty.raw() == 2076
}

/// 原版 InstanceTypeChecker::IsUint8TypedArrayConstructor (== UINT8_TYPED_ARRAY_CONSTRUCTOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_uint8_typed_array_constructor(ty: InstanceType) -> bool {
  ty.raw() == 2077
}

/// 原版 InstanceTypeChecker::IsJSArrayConstructor (== JS_ARRAY_CONSTRUCTOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_array_constructor(ty: InstanceType) -> bool {
  ty.raw() == 2078
}

/// 原版 InstanceTypeChecker::IsJSPromiseConstructor (== JS_PROMISE_CONSTRUCTOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_promise_constructor(ty: InstanceType) -> bool {
  ty.raw() == 2079
}

/// 原版 InstanceTypeChecker::IsJSRegExpConstructor (== JS_REG_EXP_CONSTRUCTOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_reg_exp_constructor(ty: InstanceType) -> bool {
  ty.raw() == 2080
}

/// 原版 InstanceTypeChecker::IsJSClassConstructor (== JS_CLASS_CONSTRUCTOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_class_constructor(ty: InstanceType) -> bool {
  ty.raw() == 2081
}

/// 原版 InstanceTypeChecker::IsJSBoundFunction (== JS_BOUND_FUNCTION_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_bound_function(ty: InstanceType) -> bool {
  ty.raw() == 2082
}

/// 原版 InstanceTypeChecker::IsJSWrappedFunction (== JS_WRAPPED_FUNCTION_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_wrapped_function(ty: InstanceType) -> bool {
  ty.raw() == 2083
}

/// 原版 InstanceTypeChecker::IsJSArrayIteratorPrototype (== JS_ARRAY_ITERATOR_PROTOTYPE_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_array_iterator_prototype(ty: InstanceType) -> bool {
  ty.raw() == 2084
}

/// 原版 InstanceTypeChecker::IsJSIteratorPrototype (== JS_ITERATOR_PROTOTYPE_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_iterator_prototype(ty: InstanceType) -> bool {
  ty.raw() == 2085
}

/// 原版 InstanceTypeChecker::IsJSMapIteratorPrototype (== JS_MAP_ITERATOR_PROTOTYPE_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_map_iterator_prototype(ty: InstanceType) -> bool {
  ty.raw() == 2086
}

/// 原版 InstanceTypeChecker::IsJSObjectPrototype (== JS_OBJECT_PROTOTYPE_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_object_prototype(ty: InstanceType) -> bool {
  ty.raw() == 2087
}

/// 原版 InstanceTypeChecker::IsJSPromisePrototype (== JS_PROMISE_PROTOTYPE_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_promise_prototype(ty: InstanceType) -> bool {
  ty.raw() == 2088
}

/// 原版 InstanceTypeChecker::IsJSRegExpPrototype (== JS_REG_EXP_PROTOTYPE_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_reg_exp_prototype(ty: InstanceType) -> bool {
  ty.raw() == 2089
}

/// 原版 InstanceTypeChecker::IsJSSetIteratorPrototype (== JS_SET_ITERATOR_PROTOTYPE_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_set_iterator_prototype(ty: InstanceType) -> bool {
  ty.raw() == 2090
}

/// 原版 InstanceTypeChecker::IsJSSetPrototype (== JS_SET_PROTOTYPE_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_set_prototype(ty: InstanceType) -> bool {
  ty.raw() == 2091
}

/// 原版 InstanceTypeChecker::IsJSStringIteratorPrototype (== JS_STRING_ITERATOR_PROTOTYPE_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_string_iterator_prototype(ty: InstanceType) -> bool {
  ty.raw() == 2092
}

/// 原版 InstanceTypeChecker::IsJSTypedArrayPrototype (== JS_TYPED_ARRAY_PROTOTYPE_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_typed_array_prototype(ty: InstanceType) -> bool {
  ty.raw() == 2093
}

/// 原版 InstanceTypeChecker::IsJSIteratorConcatHelper (== JS_ITERATOR_CONCAT_HELPER_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_iterator_concat_helper(ty: InstanceType) -> bool {
  ty.raw() == 2094
}

/// 原版 InstanceTypeChecker::IsJSIteratorDropHelper (== JS_ITERATOR_DROP_HELPER_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_iterator_drop_helper(ty: InstanceType) -> bool {
  ty.raw() == 2095
}

/// 原版 InstanceTypeChecker::IsJSIteratorFilterHelper (== JS_ITERATOR_FILTER_HELPER_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_iterator_filter_helper(ty: InstanceType) -> bool {
  ty.raw() == 2096
}

/// 原版 InstanceTypeChecker::IsJSIteratorFlatMapHelper (== JS_ITERATOR_FLAT_MAP_HELPER_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_iterator_flat_map_helper(ty: InstanceType) -> bool {
  ty.raw() == 2097
}

/// 原版 InstanceTypeChecker::IsJSIteratorMapHelper (== JS_ITERATOR_MAP_HELPER_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_iterator_map_helper(ty: InstanceType) -> bool {
  ty.raw() == 2098
}

/// 原版 InstanceTypeChecker::IsJSIteratorTakeHelper (== JS_ITERATOR_TAKE_HELPER_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_iterator_take_helper(ty: InstanceType) -> bool {
  ty.raw() == 2099
}

/// 原版 InstanceTypeChecker::IsJSIteratorZipKeyedHelper (== JS_ITERATOR_ZIP_KEYED_HELPER_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_iterator_zip_keyed_helper(ty: InstanceType) -> bool {
  ty.raw() == 2101
}

/// 原版 InstanceTypeChecker::IsJSMapKeyIterator (== JS_MAP_KEY_ITERATOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_map_key_iterator(ty: InstanceType) -> bool {
  ty.raw() == 2102
}

/// 原版 InstanceTypeChecker::IsJSMapKeyValueIterator (== JS_MAP_KEY_VALUE_ITERATOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_map_key_value_iterator(ty: InstanceType) -> bool {
  ty.raw() == 2103
}

/// 原版 InstanceTypeChecker::IsJSMapValueIterator (== JS_MAP_VALUE_ITERATOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_map_value_iterator(ty: InstanceType) -> bool {
  ty.raw() == 2104
}

/// 原版 InstanceTypeChecker::IsJSSetKeyValueIterator (== JS_SET_KEY_VALUE_ITERATOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_set_key_value_iterator(ty: InstanceType) -> bool {
  ty.raw() == 2105
}

/// 原版 InstanceTypeChecker::IsJSSetValueIterator (== JS_SET_VALUE_ITERATOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_set_value_iterator(ty: InstanceType) -> bool {
  ty.raw() == 2106
}

/// 原版 InstanceTypeChecker::IsOrderedHashTableIterator (== ORDERED_HASH_TABLE_ITERATOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_ordered_hash_table_iterator(ty: InstanceType) -> bool {
  ty.raw() == 2107
}

/// 原版 InstanceTypeChecker::IsJSAtomicsCondition (== JS_ATOMICS_CONDITION_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_atomics_condition(ty: InstanceType) -> bool {
  ty.raw() == 2108
}

/// 原版 InstanceTypeChecker::IsJSAtomicsMutex (== JS_ATOMICS_MUTEX_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_atomics_mutex(ty: InstanceType) -> bool {
  ty.raw() == 2109
}

/// 原版 InstanceTypeChecker::IsJSSharedArray (== JS_SHARED_ARRAY_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_shared_array(ty: InstanceType) -> bool {
  ty.raw() == 2110
}

/// 原版 InstanceTypeChecker::IsJSSharedStruct (== JS_SHARED_STRUCT_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_shared_struct(ty: InstanceType) -> bool {
  ty.raw() == 2111
}

/// 原版 InstanceTypeChecker::IsJSAsyncDisposableStack (== JS_ASYNC_DISPOSABLE_STACK_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_async_disposable_stack(ty: InstanceType) -> bool {
  ty.raw() == 2113
}

/// 原版 InstanceTypeChecker::IsJSSyncDisposableStack (== JS_SYNC_DISPOSABLE_STACK_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_sync_disposable_stack(ty: InstanceType) -> bool {
  ty.raw() == 2114
}

/// 原版 InstanceTypeChecker::IsJSAsyncFunctionObject (== JS_ASYNC_FUNCTION_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_async_function_object(ty: InstanceType) -> bool {
  ty.raw() == 2116
}

/// 原版 InstanceTypeChecker::IsJSAsyncGeneratorObject (== JS_ASYNC_GENERATOR_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_async_generator_object(ty: InstanceType) -> bool {
  ty.raw() == 2117
}

/// 原版 InstanceTypeChecker::IsJSMap (== JS_MAP_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_map(ty: InstanceType) -> bool {
  ty.raw() == 2118
}

/// 原版 InstanceTypeChecker::IsJSSet (== JS_SET_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_set(ty: InstanceType) -> bool {
  ty.raw() == 2119
}

/// 原版 InstanceTypeChecker::IsJSPromise (== JS_PROMISE_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_promise(ty: InstanceType) -> bool {
  ty.raw() == 2121
}

/// 原版 InstanceTypeChecker::IsJSWeakMap (== JS_WEAK_MAP_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_weak_map(ty: InstanceType) -> bool {
  ty.raw() == 2122
}

/// 原版 InstanceTypeChecker::IsJSWeakSet (== JS_WEAK_SET_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_weak_set(ty: InstanceType) -> bool {
  ty.raw() == 2123
}

/// 原版 InstanceTypeChecker::IsJSArgumentsObject (== JS_ARGUMENTS_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_arguments_object(ty: InstanceType) -> bool {
  ty.raw() == 2124
}

/// 原版 InstanceTypeChecker::IsJSArray (== JS_ARRAY_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_array(ty: InstanceType) -> bool {
  ty.raw() == 2125
}

/// 原版 InstanceTypeChecker::IsJSArrayIterator (== JS_ARRAY_ITERATOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_array_iterator(ty: InstanceType) -> bool {
  ty.raw() == 2126
}

/// 原版 InstanceTypeChecker::IsJSAsyncFromSyncIterator (== JS_ASYNC_FROM_SYNC_ITERATOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_async_from_sync_iterator(ty: InstanceType) -> bool {
  ty.raw() == 2127
}

/// 原版 InstanceTypeChecker::IsJSCollator (== JS_COLLATOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_collator(ty: InstanceType) -> bool {
  ty.raw() == 2128
}

/// 原版 InstanceTypeChecker::IsJSContextExtensionObject (== JS_CONTEXT_EXTENSION_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_context_extension_object(ty: InstanceType) -> bool {
  ty.raw() == 2129
}

/// 原版 InstanceTypeChecker::IsJSDate (== JS_DATE_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_date(ty: InstanceType) -> bool {
  ty.raw() == 2130
}

/// 原版 InstanceTypeChecker::IsJSDateTimeFormat (== JS_DATE_TIME_FORMAT_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_date_time_format(ty: InstanceType) -> bool {
  ty.raw() == 2131
}

/// 原版 InstanceTypeChecker::IsJSDisplayNames (== JS_DISPLAY_NAMES_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_display_names(ty: InstanceType) -> bool {
  ty.raw() == 2132
}

/// 原版 InstanceTypeChecker::IsJSDurationFormat (== JS_DURATION_FORMAT_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_duration_format(ty: InstanceType) -> bool {
  ty.raw() == 2133
}

/// 原版 InstanceTypeChecker::IsJSError (== JS_ERROR_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_error(ty: InstanceType) -> bool {
  ty.raw() == 2134
}

/// 原版 InstanceTypeChecker::IsJSExternalObject (== JS_EXTERNAL_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_external_object(ty: InstanceType) -> bool {
  ty.raw() == 2135
}

/// 原版 InstanceTypeChecker::IsJSFinalizationRegistry (== JS_FINALIZATION_REGISTRY_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_finalization_registry(ty: InstanceType) -> bool {
  ty.raw() == 2136
}

/// 原版 InstanceTypeChecker::IsJSListFormat (== JS_LIST_FORMAT_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_list_format(ty: InstanceType) -> bool {
  ty.raw() == 2137
}

/// 原版 InstanceTypeChecker::IsJSLocale (== JS_LOCALE_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_locale(ty: InstanceType) -> bool {
  ty.raw() == 2138
}

/// 原版 InstanceTypeChecker::IsJSMessageObject (== JS_MESSAGE_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_message_object(ty: InstanceType) -> bool {
  ty.raw() == 2139
}

/// 原版 InstanceTypeChecker::IsJSNumberFormat (== JS_NUMBER_FORMAT_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_number_format(ty: InstanceType) -> bool {
  ty.raw() == 2140
}

/// 原版 InstanceTypeChecker::IsJSPluralRules (== JS_PLURAL_RULES_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_plural_rules(ty: InstanceType) -> bool {
  ty.raw() == 2141
}

/// 原版 InstanceTypeChecker::IsJSRawJson (== JS_RAW_JSON_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_raw_json(ty: InstanceType) -> bool {
  ty.raw() == 2142
}

/// 原版 InstanceTypeChecker::IsJSRegExp (== JS_REG_EXP_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_reg_exp(ty: InstanceType) -> bool {
  ty.raw() == 2143
}

/// 原版 InstanceTypeChecker::IsJSRegExpStringIterator (== JS_REG_EXP_STRING_ITERATOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_reg_exp_string_iterator(ty: InstanceType) -> bool {
  ty.raw() == 2144
}

/// 原版 InstanceTypeChecker::IsJSRelativeTimeFormat (== JS_RELATIVE_TIME_FORMAT_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_relative_time_format(ty: InstanceType) -> bool {
  ty.raw() == 2145
}

/// 原版 InstanceTypeChecker::IsJSSegmentIterator (== JS_SEGMENT_ITERATOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_segment_iterator(ty: InstanceType) -> bool {
  ty.raw() == 2146
}

/// 原版 InstanceTypeChecker::IsJSSegmenter (== JS_SEGMENTER_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_segmenter(ty: InstanceType) -> bool {
  ty.raw() == 2147
}

/// 原版 InstanceTypeChecker::IsJSSegments (== JS_SEGMENTS_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_segments(ty: InstanceType) -> bool {
  ty.raw() == 2148
}

/// 原版 InstanceTypeChecker::IsJSShadowRealm (== JS_SHADOW_REALM_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_shadow_realm(ty: InstanceType) -> bool {
  ty.raw() == 2149
}

/// 原版 InstanceTypeChecker::IsJSStringIterator (== JS_STRING_ITERATOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_string_iterator(ty: InstanceType) -> bool {
  ty.raw() == 2150
}

/// 原版 InstanceTypeChecker::IsJSV8BreakIterator (== JS_V8_BREAK_ITERATOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_jsv8_break_iterator(ty: InstanceType) -> bool {
  ty.raw() == 2151
}

/// 原版 InstanceTypeChecker::IsJSValidIteratorWrapper (== JS_VALID_ITERATOR_WRAPPER_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_valid_iterator_wrapper(ty: InstanceType) -> bool {
  ty.raw() == 2152
}

/// 原版 InstanceTypeChecker::IsJSWeakRef (== JS_WEAK_REF_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_weak_ref(ty: InstanceType) -> bool {
  ty.raw() == 2153
}

/// 原版 InstanceTypeChecker::IsWasmCustomMapWrapper (== WASM_CUSTOM_MAP_WRAPPER_TYPE)。
#[inline]
#[must_use]
pub const fn is_wasm_custom_map_wrapper(ty: InstanceType) -> bool {
  ty.raw() == 2154
}

/// 原版 InstanceTypeChecker::IsWasmExceptionPackage (== WASM_EXCEPTION_PACKAGE_TYPE)。
#[inline]
#[must_use]
pub const fn is_wasm_exception_package(ty: InstanceType) -> bool {
  ty.raw() == 2155
}

/// 原版 InstanceTypeChecker::IsWasmGlobalObject (== WASM_GLOBAL_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_wasm_global_object(ty: InstanceType) -> bool {
  ty.raw() == 2156
}

/// 原版 InstanceTypeChecker::IsWasmInstanceObject (== WASM_INSTANCE_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_wasm_instance_object(ty: InstanceType) -> bool {
  ty.raw() == 2157
}

/// 原版 InstanceTypeChecker::IsWasmMemoryObject (== WASM_MEMORY_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_wasm_memory_object(ty: InstanceType) -> bool {
  ty.raw() == 2158
}

/// 原版 InstanceTypeChecker::IsWasmModuleObject (== WASM_MODULE_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_wasm_module_object(ty: InstanceType) -> bool {
  ty.raw() == 2159
}

/// 原版 InstanceTypeChecker::IsWasmSuspendingObject (== WASM_SUSPENDING_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_wasm_suspending_object(ty: InstanceType) -> bool {
  ty.raw() == 2160
}

/// 原版 InstanceTypeChecker::IsWasmTableObject (== WASM_TABLE_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_wasm_table_object(ty: InstanceType) -> bool {
  ty.raw() == 2161
}

/// 原版 InstanceTypeChecker::IsWasmTagObject (== WASM_TAG_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_wasm_tag_object(ty: InstanceType) -> bool {
  ty.raw() == 2162
}

/// 原版 InstanceTypeChecker::IsWasmValueObject (== WASM_VALUE_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_wasm_value_object(ty: InstanceType) -> bool {
  ty.raw() == 2163
}

/// 原版 InstanceTypeChecker::IsBigInt (== BIGINT_TYPE)。
#[inline]
#[must_use]
pub const fn is_big_int(ty: InstanceType) -> bool {
  ty.raw() == 129
}

/// 原版 InstanceTypeChecker::IsFixedArrayExact (== FIXED_ARRAY_TYPE)。
#[inline]
#[must_use]
pub const fn is_fixed_array_exact(ty: InstanceType) -> bool {
  ty.raw() == 205
}

// ── 区间检查器 (INSTANCE_TYPE_CHECKERS_RANGE, INSTANCE_TYPE_CHECKER_RANGE1 展开) ──
/// 原版 InstanceTypeChecker::IsString (FIRST_STRING_TYPE..=LAST_STRING_TYPE)。
#[inline]
#[must_use]
pub const fn is_string(ty: InstanceType) -> bool {
  ty.raw() <= 127
}

/// 原版 InstanceTypeChecker::IsName (FIRST_NAME_TYPE..=LAST_NAME_TYPE)。
#[inline]
#[must_use]
pub const fn is_name(ty: InstanceType) -> bool {
  ty.raw() <= 128
}

/// 原版 InstanceTypeChecker::IsPrimitiveHeapObject (FIRST_PRIMITIVE_HEAP_OBJECT_TYPE..=LAST_PRIMITIVE_HEAP_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_primitive_heap_object(ty: InstanceType) -> bool {
  ty.raw() <= 131
}

/// 原版 InstanceTypeChecker::IsPromiseReactionJobTask (FIRST_PROMISE_REACTION_JOB_TASK_TYPE..=LAST_PROMISE_REACTION_JOB_TASK_TYPE)。
#[inline]
#[must_use]
pub const fn is_promise_reaction_job_task(ty: InstanceType) -> bool {
  ty.raw() >= 132 && ty.raw() <= 133
}

/// 原版 InstanceTypeChecker::IsMicrotask (FIRST_MICROTASK_TYPE..=LAST_MICROTASK_TYPE)。
#[inline]
#[must_use]
pub const fn is_microtask(ty: InstanceType) -> bool {
  ty.raw() >= 132 && ty.raw() <= 137
}

/// 原版 InstanceTypeChecker::IsDataHandler (FIRST_DATA_HANDLER_TYPE..=LAST_DATA_HANDLER_TYPE)。
#[inline]
#[must_use]
pub const fn is_data_handler(ty: InstanceType) -> bool {
  ty.raw() >= 138 && ty.raw() <= 139
}

/// 原版 InstanceTypeChecker::IsStruct (FIRST_STRUCT_TYPE..=LAST_STRUCT_TYPE)。
#[inline]
#[must_use]
pub const fn is_struct(ty: InstanceType) -> bool {
  ty.raw() >= 132 && ty.raw() <= 174
}

/// 原版 InstanceTypeChecker::IsUncompiledDataWithPreparseData (FIRST_UNCOMPILED_DATA_WITH_PREPARSE_DATA_TYPE..=LAST_UNCOMPILED_DATA_WITH_PREPARSE_DATA_TYPE)。
#[inline]
#[must_use]
pub const fn is_uncompiled_data_with_preparse_data(ty: InstanceType) -> bool {
  ty.raw() >= 175 && ty.raw() <= 176
}

/// 原版 InstanceTypeChecker::IsUncompiledDataWithoutPreparseData (FIRST_UNCOMPILED_DATA_WITHOUT_PREPARSE_DATA_TYPE..=LAST_UNCOMPILED_DATA_WITHOUT_PREPARSE_DATA_TYPE)。
#[inline]
#[must_use]
pub const fn is_uncompiled_data_without_preparse_data(ty: InstanceType) -> bool {
  ty.raw() >= 177 && ty.raw() <= 178
}

/// 原版 InstanceTypeChecker::IsUncompiledData (FIRST_UNCOMPILED_DATA_TYPE..=LAST_UNCOMPILED_DATA_TYPE)。
#[inline]
#[must_use]
pub const fn is_uncompiled_data(ty: InstanceType) -> bool {
  ty.raw() >= 175 && ty.raw() <= 178
}

/// 原版 InstanceTypeChecker::IsRegExpData (FIRST_REG_EXP_DATA_TYPE..=LAST_REG_EXP_DATA_TYPE)。
#[inline]
#[must_use]
pub const fn is_reg_exp_data(ty: InstanceType) -> bool {
  ty.raw() >= 179 && ty.raw() <= 181
}

/// 原版 InstanceTypeChecker::IsWasmFunctionData (FIRST_WASM_FUNCTION_DATA_TYPE..=LAST_WASM_FUNCTION_DATA_TYPE)。
#[inline]
#[must_use]
pub const fn is_wasm_function_data(ty: InstanceType) -> bool {
  ty.raw() >= 182 && ty.raw() <= 184
}

/// 原版 InstanceTypeChecker::IsExposedTrustedObject (FIRST_EXPOSED_TRUSTED_OBJECT_TYPE..=LAST_EXPOSED_TRUSTED_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_exposed_trusted_object(ty: InstanceType) -> bool {
  ty.raw() >= 175 && ty.raw() <= 192
}

/// 原版 InstanceTypeChecker::IsTrustedFixedArrayBase (FIRST_TRUSTED_FIXED_ARRAY_BASE_TYPE..=LAST_TRUSTED_FIXED_ARRAY_BASE_TYPE)。
#[inline]
#[must_use]
pub const fn is_trusted_fixed_array_base(ty: InstanceType) -> bool {
  ty.raw() >= 193 && ty.raw() <= 197
}

/// 原版 InstanceTypeChecker::IsTrustedObject (FIRST_TRUSTED_OBJECT_TYPE..=LAST_TRUSTED_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_trusted_object(ty: InstanceType) -> bool {
  ty.raw() >= 175 && ty.raw() <= 202
}

/// 原版 InstanceTypeChecker::IsFixedArray (FIRST_FIXED_ARRAY_TYPE..=LAST_FIXED_ARRAY_TYPE)。
#[inline]
#[must_use]
pub const fn is_fixed_array(ty: InstanceType) -> bool {
  ty.raw() >= 205 && ty.raw() <= 221
}

/// 原版 InstanceTypeChecker::IsWeakFixedArray (FIRST_WEAK_FIXED_ARRAY_TYPE..=LAST_WEAK_FIXED_ARRAY_TYPE)。
#[inline]
#[must_use]
pub const fn is_weak_fixed_array(ty: InstanceType) -> bool {
  ty.raw() >= 222 && ty.raw() <= 223
}

/// 原版 InstanceTypeChecker::IsFixedArrayBase (FIRST_FIXED_ARRAY_BASE_TYPE..=LAST_FIXED_ARRAY_BASE_TYPE)。
#[inline]
#[must_use]
pub const fn is_fixed_array_base(ty: InstanceType) -> bool {
  ty.raw() >= 205 && ty.raw() <= 230
}

/// 原版 InstanceTypeChecker::IsContext (FIRST_CONTEXT_TYPE..=LAST_CONTEXT_TYPE)。
#[inline]
#[must_use]
pub const fn is_context(ty: InstanceType) -> bool {
  ty.raw() >= 231 && ty.raw() <= 240
}

/// 原版 InstanceTypeChecker::IsTurboshaftFloat64Type (FIRST_TURBOSHAFT_FLOAT64_TYPE_TYPE..=LAST_TURBOSHAFT_FLOAT64_TYPE_TYPE)。
#[inline]
#[must_use]
pub const fn is_turboshaft_float64_type(ty: InstanceType) -> bool {
  ty.raw() >= 241 && ty.raw() <= 242
}

/// 原版 InstanceTypeChecker::IsTurboshaftWord32Type (FIRST_TURBOSHAFT_WORD32_TYPE_TYPE..=LAST_TURBOSHAFT_WORD32_TYPE_TYPE)。
#[inline]
#[must_use]
pub const fn is_turboshaft_word32_type(ty: InstanceType) -> bool {
  ty.raw() >= 243 && ty.raw() <= 244
}

/// 原版 InstanceTypeChecker::IsTurboshaftWord64Type (FIRST_TURBOSHAFT_WORD64_TYPE_TYPE..=LAST_TURBOSHAFT_WORD64_TYPE_TYPE)。
#[inline]
#[must_use]
pub const fn is_turboshaft_word64_type(ty: InstanceType) -> bool {
  ty.raw() >= 245 && ty.raw() <= 246
}

/// 原版 InstanceTypeChecker::IsTurboshaftType (FIRST_TURBOSHAFT_TYPE_TYPE..=LAST_TURBOSHAFT_TYPE_TYPE)。
#[inline]
#[must_use]
pub const fn is_turboshaft_type(ty: InstanceType) -> bool {
  ty.raw() >= 241 && ty.raw() <= 246
}

/// 原版 InstanceTypeChecker::IsTurbofanType (FIRST_TURBOFAN_TYPE_TYPE..=LAST_TURBOFAN_TYPE_TYPE)。
#[inline]
#[must_use]
pub const fn is_turbofan_type(ty: InstanceType) -> bool {
  ty.raw() >= 247 && ty.raw() <= 251
}

/// 原版 InstanceTypeChecker::IsSmallOrderedHashTable (FIRST_SMALL_ORDERED_HASH_TABLE_TYPE..=LAST_SMALL_ORDERED_HASH_TABLE_TYPE)。
#[inline]
#[must_use]
pub const fn is_small_ordered_hash_table(ty: InstanceType) -> bool {
  ty.raw() >= 252 && ty.raw() <= 255
}

/// 原版 InstanceTypeChecker::IsTemplateInfoWithProperties (FIRST_TEMPLATE_INFO_WITH_PROPERTIES_TYPE..=LAST_TEMPLATE_INFO_WITH_PROPERTIES_TYPE)。
#[inline]
#[must_use]
pub const fn is_template_info_with_properties(ty: InstanceType) -> bool {
  ty.raw() >= 256 && ty.raw() <= 257
}

/// 原版 InstanceTypeChecker::IsTemplateInfo (FIRST_TEMPLATE_INFO_TYPE..=LAST_TEMPLATE_INFO_TYPE)。
#[inline]
#[must_use]
pub const fn is_template_info(ty: InstanceType) -> bool {
  ty.raw() >= 256 && ty.raw() <= 258
}

/// 原版 InstanceTypeChecker::IsMap (FIRST_MAP_TYPE..=LAST_MAP_TYPE)。
#[inline]
#[must_use]
pub const fn is_map(ty: InstanceType) -> bool {
  ty.raw() >= 259 && ty.raw() <= 260
}

/// 原版 InstanceTypeChecker::IsModule (FIRST_MODULE_TYPE..=LAST_MODULE_TYPE)。
#[inline]
#[must_use]
pub const fn is_module(ty: InstanceType) -> bool {
  ty.raw() >= 261 && ty.raw() <= 262
}

/// 原版 InstanceTypeChecker::IsWasmObject (FIRST_WASM_OBJECT_TYPE..=LAST_WASM_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_wasm_object(ty: InstanceType) -> bool {
  ty.raw() >= 301 && ty.raw() <= 302
}

/// 原版 InstanceTypeChecker::IsJSModuleNamespace (FIRST_JS_MODULE_NAMESPACE_TYPE..=LAST_JS_MODULE_NAMESPACE_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_module_namespace(ty: InstanceType) -> bool {
  ty.raw() >= 304 && ty.raw() <= 305
}

/// 原版 InstanceTypeChecker::IsJSSpecialObject (FIRST_JS_SPECIAL_OBJECT_TYPE..=LAST_JS_SPECIAL_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_special_object(ty: InstanceType) -> bool {
  ty.raw() >= 304 && ty.raw() <= 1040
}

/// 原版 InstanceTypeChecker::IsJSCustomElementsObject (FIRST_JS_CUSTOM_ELEMENTS_OBJECT_TYPE..=LAST_JS_CUSTOM_ELEMENTS_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_custom_elements_object(ty: InstanceType) -> bool {
  ty.raw() >= 304 && ty.raw() <= 1041
}

/// 原版 InstanceTypeChecker::IsJSApiObject (FIRST_JS_API_OBJECT_TYPE..=LAST_JS_API_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_api_object(ty: InstanceType) -> bool {
  ty.raw() >= 1058 && ty.raw() <= 2058
}

/// 原版 InstanceTypeChecker::IsJSDataViewOrRabGsabDataView (FIRST_JS_DATA_VIEW_OR_RAB_GSAB_DATA_VIEW_TYPE..=LAST_JS_DATA_VIEW_OR_RAB_GSAB_DATA_VIEW_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_data_view_or_rab_gsab_data_view(ty: InstanceType) -> bool {
  ty.raw() >= 2059 && ty.raw() <= 2060
}

/// 原版 InstanceTypeChecker::IsJSTypedArray (FIRST_JS_TYPED_ARRAY_TYPE..=LAST_JS_TYPED_ARRAY_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_typed_array(ty: InstanceType) -> bool {
  ty.raw() >= 2061 && ty.raw() <= 2062
}

/// 原版 InstanceTypeChecker::IsJSArrayBufferView (FIRST_JS_ARRAY_BUFFER_VIEW_TYPE..=LAST_JS_ARRAY_BUFFER_VIEW_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_array_buffer_view(ty: InstanceType) -> bool {
  ty.raw() >= 2059 && ty.raw() <= 2062
}

/// 原版 InstanceTypeChecker::IsJSAPIObjectWithEmbedderSlots (FIRST_JS_APIOBJECT_WITH_EMBEDDER_SLOTS_TYPE..=LAST_JS_APIOBJECT_WITH_EMBEDDER_SLOTS_TYPE)。
#[inline]
#[must_use]
pub const fn is_jsapi_object_with_embedder_slots(ty: InstanceType) -> bool {
  ty.raw() >= 1058 && ty.raw() <= 2063
}

/// 原版 InstanceTypeChecker::IsTypedArrayConstructor (FIRST_TYPED_ARRAY_CONSTRUCTOR_TYPE..=LAST_TYPED_ARRAY_CONSTRUCTOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_typed_array_constructor(ty: InstanceType) -> bool {
  ty.raw() >= 2066 && ty.raw() <= 2077
}

/// 原版 InstanceTypeChecker::IsJSFunctionWithPrototype (FIRST_JS_FUNCTION_WITH_PROTOTYPE_TYPE..=LAST_JS_FUNCTION_WITH_PROTOTYPE_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_function_with_prototype(ty: InstanceType) -> bool {
  ty.raw() >= 2065 && ty.raw() <= 2081
}

/// 原版 InstanceTypeChecker::IsJSFunction (FIRST_JS_FUNCTION_TYPE..=LAST_JS_FUNCTION_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_function(ty: InstanceType) -> bool {
  ty.raw() >= 2064 && ty.raw() <= 2081
}

/// 原版 InstanceTypeChecker::IsJSFunctionOrBoundFunctionOrWrappedFunction (FIRST_JS_FUNCTION_OR_BOUND_FUNCTION_OR_WRAPPED_FUNCTION_TYPE..=LAST_JS_FUNCTION_OR_BOUND_FUNCTION_OR_WRAPPED_FUNCTION_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_function_or_bound_function_or_wrapped_function(ty: InstanceType) -> bool {
  ty.raw() >= 2064 && ty.raw() <= 2083
}

/// 原版 InstanceTypeChecker::IsJSInternalPrototypeBase (FIRST_JS_INTERNAL_PROTOTYPE_BASE_TYPE..=LAST_JS_INTERNAL_PROTOTYPE_BASE_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_internal_prototype_base(ty: InstanceType) -> bool {
  ty.raw() >= 2084 && ty.raw() <= 2093
}

/// 原版 InstanceTypeChecker::IsJSIteratorHelperSimple (FIRST_JS_ITERATOR_HELPER_SIMPLE_TYPE..=LAST_JS_ITERATOR_HELPER_SIMPLE_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_iterator_helper_simple(ty: InstanceType) -> bool {
  ty.raw() >= 2094 && ty.raw() <= 2099
}

/// 原版 InstanceTypeChecker::IsJSIteratorZipHelper (FIRST_JS_ITERATOR_ZIP_HELPER_TYPE..=LAST_JS_ITERATOR_ZIP_HELPER_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_iterator_zip_helper(ty: InstanceType) -> bool {
  ty.raw() >= 2100 && ty.raw() <= 2101
}

/// 原版 InstanceTypeChecker::IsJSIteratorHelper (FIRST_JS_ITERATOR_HELPER_TYPE..=LAST_JS_ITERATOR_HELPER_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_iterator_helper(ty: InstanceType) -> bool {
  ty.raw() >= 2094 && ty.raw() <= 2101
}

/// 原版 InstanceTypeChecker::IsJSMapIterator (FIRST_JS_MAP_ITERATOR_TYPE..=LAST_JS_MAP_ITERATOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_map_iterator(ty: InstanceType) -> bool {
  ty.raw() >= 2102 && ty.raw() <= 2104
}

/// 原版 InstanceTypeChecker::IsJSSetIterator (FIRST_JS_SET_ITERATOR_TYPE..=LAST_JS_SET_ITERATOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_set_iterator(ty: InstanceType) -> bool {
  ty.raw() >= 2105 && ty.raw() <= 2106
}

/// 原版 InstanceTypeChecker::IsJSCollectionIterator (FIRST_JS_COLLECTION_ITERATOR_TYPE..=LAST_JS_COLLECTION_ITERATOR_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_collection_iterator(ty: InstanceType) -> bool {
  ty.raw() >= 2102 && ty.raw() <= 2107
}

/// 原版 InstanceTypeChecker::IsJSSynchronizationPrimitive (FIRST_JS_SYNCHRONIZATION_PRIMITIVE_TYPE..=LAST_JS_SYNCHRONIZATION_PRIMITIVE_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_synchronization_primitive(ty: InstanceType) -> bool {
  ty.raw() >= 2108 && ty.raw() <= 2109
}

/// 原版 InstanceTypeChecker::IsAlwaysSharedSpaceJSObject (FIRST_ALWAYS_SHARED_SPACE_JS_OBJECT_TYPE..=LAST_ALWAYS_SHARED_SPACE_JS_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_always_shared_space_js_object(ty: InstanceType) -> bool {
  ty.raw() >= 2108 && ty.raw() <= 2111
}

/// 原版 InstanceTypeChecker::IsJSDisposableStackBase (FIRST_JS_DISPOSABLE_STACK_BASE_TYPE..=LAST_JS_DISPOSABLE_STACK_BASE_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_disposable_stack_base(ty: InstanceType) -> bool {
  ty.raw() >= 2112 && ty.raw() <= 2114
}

/// 原版 InstanceTypeChecker::IsJSGeneratorObject (FIRST_JS_GENERATOR_OBJECT_TYPE..=LAST_JS_GENERATOR_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_generator_object(ty: InstanceType) -> bool {
  ty.raw() >= 2115 && ty.raw() <= 2117
}

/// 原版 InstanceTypeChecker::IsJSCollection (FIRST_JS_COLLECTION_TYPE..=LAST_JS_COLLECTION_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_collection(ty: InstanceType) -> bool {
  ty.raw() >= 2118 && ty.raw() <= 2119
}

/// 原版 InstanceTypeChecker::IsJSObjectWithEmbedderSlots (FIRST_JS_OBJECT_WITH_EMBEDDER_SLOTS_TYPE..=LAST_JS_OBJECT_WITH_EMBEDDER_SLOTS_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_object_with_embedder_slots(ty: InstanceType) -> bool {
  ty.raw() >= 2120 && ty.raw() <= 2121
}

/// 原版 InstanceTypeChecker::IsJSWeakCollection (FIRST_JS_WEAK_COLLECTION_TYPE..=LAST_JS_WEAK_COLLECTION_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_weak_collection(ty: InstanceType) -> bool {
  ty.raw() >= 2122 && ty.raw() <= 2123
}

/// 原版 InstanceTypeChecker::IsJSObject (FIRST_JS_OBJECT_TYPE..=LAST_JS_OBJECT_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_object(ty: InstanceType) -> bool {
  ty.raw() >= 304 && ty.raw() <= 2163
}

/// 原版 InstanceTypeChecker::IsJSReceiver (FIRST_JS_RECEIVER_TYPE..=LAST_JS_RECEIVER_TYPE)。
#[inline]
#[must_use]
pub const fn is_js_receiver(ty: InstanceType) -> bool {
  ty.raw() >= 301 && ty.raw() <= 2163
}

/// 原版 InstanceTypeChecker::IsCallableJSFunction (FIRST_CALLABLE_JS_FUNCTION_TYPE..=LAST_CALLABLE_JS_FUNCTION_TYPE)。
#[inline]
#[must_use]
pub const fn is_callable_js_function(ty: InstanceType) -> bool {
  ty.raw() >= 2064 && ty.raw() <= 2080
}

// ── 自定义检查器 (INSTANCE_TYPE_CHECKERS_CUSTOM + instance-type-inl.h 手写段) ──
// IsSharedString 的 v8_flags.shared_string_table 分支推迟 (需 v8-flags 集成)。
#[inline]
#[must_use]
pub const fn is_internalized_string(ty: InstanceType) -> bool {
  ty.is_internalized()
}

/// 原版 IsSeqString: 表示位 == kSeqStringTag。
#[inline]
#[must_use]
pub const fn is_seq_string(ty: InstanceType) -> bool {
  let v = ty.raw() as u32;
  (v & (K_IS_NOT_STRING_MASK | K_STRING_REPRESENTATION_MASK)) == K_SEQ_STRING_TAG
}

/// 原版 IsSeqOneByteString。
#[inline]
#[must_use]
pub const fn is_seq_one_byte_string(ty: InstanceType) -> bool {
  let v = ty.raw() as u32;
  (v & (K_IS_NOT_STRING_MASK | K_STRING_REPRESENTATION_AND_ENCODING_MASK))
    == (K_STRING_TAG | K_SEQ_ONE_BYTE_STRING_TAG)
}

/// 原版 IsSeqTwoByteString。
#[inline]
#[must_use]
pub const fn is_seq_two_byte_string(ty: InstanceType) -> bool {
  let v = ty.raw() as u32;
  (v & (K_IS_NOT_STRING_MASK | K_STRING_REPRESENTATION_AND_ENCODING_MASK))
    == (K_STRING_TAG | K_SEQ_TWO_BYTE_STRING_TAG)
}

/// 原版 IsExternalString。
#[inline]
#[must_use]
pub const fn is_external_string(ty: InstanceType) -> bool {
  let v = ty.raw() as u32;
  (v & (K_IS_NOT_STRING_MASK | K_STRING_REPRESENTATION_MASK)) == K_EXTERNAL_STRING_TAG
}

/// 原版 IsExternalOneByteString。
#[inline]
#[must_use]
pub const fn is_external_one_byte_string(ty: InstanceType) -> bool {
  let v = ty.raw() as u32;
  (v & (K_IS_NOT_STRING_MASK | K_STRING_REPRESENTATION_AND_ENCODING_MASK))
    == (K_STRING_TAG | K_EXTERNAL_ONE_BYTE_STRING_TAG)
}

/// 原版 IsExternalTwoByteString。
#[inline]
#[must_use]
pub const fn is_external_two_byte_string(ty: InstanceType) -> bool {
  let v = ty.raw() as u32;
  (v & (K_IS_NOT_STRING_MASK | K_STRING_REPRESENTATION_AND_ENCODING_MASK))
    == (K_STRING_TAG | K_EXTERNAL_TWO_BYTE_STRING_TAG)
}

/// 原版 IsUncachedExternalString。
#[inline]
#[must_use]
pub const fn is_uncached_external_string(ty: InstanceType) -> bool {
  let v = ty.raw() as u32;
  (v & (K_IS_NOT_STRING_MASK | K_UNCACHED_EXTERNAL_STRING_MASK | K_STRING_REPRESENTATION_MASK))
    == (K_EXTERNAL_STRING_TAG | K_UNCACHED_EXTERNAL_STRING_TAG)
}

/// 原版 IsConsString。
#[inline]
#[must_use]
pub const fn is_cons_string(ty: InstanceType) -> bool {
  let v = ty.raw() as u32;
  (v & (K_IS_NOT_STRING_MASK | K_STRING_REPRESENTATION_MASK)) == K_CONS_STRING_TAG
}

/// 原版 IsSlicedString。
#[inline]
#[must_use]
pub const fn is_sliced_string(ty: InstanceType) -> bool {
  let v = ty.raw() as u32;
  (v & (K_IS_NOT_STRING_MASK | K_STRING_REPRESENTATION_MASK)) == K_SLICED_STRING_TAG
}

/// 原版 IsThinString。
#[inline]
#[must_use]
pub const fn is_thin_string(ty: InstanceType) -> bool {
  let v = ty.raw() as u32;
  (v & (K_IS_NOT_STRING_MASK | K_STRING_REPRESENTATION_MASK)) == K_THIN_STRING_TAG
}

/// 原版 IsOneByteString (编码位)。
#[inline]
#[must_use]
pub const fn is_one_byte_string(ty: InstanceType) -> bool {
  (ty.raw() as u32 & K_STRING_ENCODING_MASK) == K_ONE_BYTE_STRING_TAG
}

/// 原版 IsTwoByteString (编码位)。
#[inline]
#[must_use]
pub const fn is_two_byte_string(ty: InstanceType) -> bool {
  (ty.raw() as u32 & K_STRING_ENCODING_MASK) == K_TWO_BYTE_STRING_TAG
}

/// 原版 IsSharedString (shared_string_table 分支推迟)。
#[inline]
#[must_use]
pub const fn is_shared_string(ty: InstanceType) -> bool {
  (ty.raw() as u32 & K_SHARED_STRING_MASK) == K_SHARED_STRING_TAG
}

/// 原版 IsIndirectString (cons/sliced/thin)。
#[inline]
#[must_use]
pub const fn is_indirect_string(ty: InstanceType) -> bool {
  let v = ty.raw() as u32;
  (v & (K_IS_NOT_STRING_MASK | K_IS_INDIRECT_STRING_MASK)) == K_IS_INDIRECT_STRING_TAG
}

/// 原版 IsDirectString。
#[inline]
#[must_use]
pub const fn is_direct_string(ty: InstanceType) -> bool {
  !is_indirect_string(ty)
}

/// 原版 IsReferenceComparable: 引用可比 (String/BigInt/HeapNumber 除外)。
#[inline]
#[must_use]
pub const fn is_reference_comparable(ty: InstanceType) -> bool {
  !is_string(ty) && !is_big_int(ty) && !matches!(ty, InstanceType::HeapNumber)
}

/// 原版 IsGcSafeCode。
#[inline]
#[must_use]
pub const fn is_gc_safe_code(ty: InstanceType) -> bool {
  is_code(ty)
}

/// 原版 IsAbstractCode: BytecodeArray 或 Code。
#[inline]
#[must_use]
pub const fn is_abstract_code(ty: InstanceType) -> bool {
  is_bytecode_array(ty) || is_code(ty)
}

/// 原版 IsFreeSpaceOrFiller。
#[inline]
#[must_use]
pub const fn is_free_space_or_filler(ty: InstanceType) -> bool {
  matches!(ty, InstanceType::FreeSpace | InstanceType::Filler)
}

/// 原版 IsMaybeReadOnlyJSObject: 仅用于跨 API 传数据的包装对象。
#[inline]
#[must_use]
pub const fn is_maybe_read_only_js_object(ty: InstanceType) -> bool {
  is_js_external_object(ty) || is_js_message_object(ty)
}

/// 原版 IsPropertyDictionary。
#[inline]
#[must_use]
pub const fn is_property_dictionary(ty: InstanceType) -> bool {
  ty.raw() == PROPERTY_DICTIONARY_TYPE.raw()
}

/// 原版 IsNativeContextSpecific: 必须绑定某 native context 的堆对象类型。
#[inline]
#[must_use]
pub const fn is_native_context_specific(ty: InstanceType) -> bool {
  // 所有 context map 都绑定 native context。
  if is_context(ty) {
    return true;
  }
  // 非 JSReceiver 不绑定任何 native context。
  if !is_js_receiver(ty) {
    return false;
  }
  if is_maybe_read_only_js_object(ty) {
    // 跨 API 包装对象不绑定。
    return false;
  }
  if is_always_shared_space_js_object(ty) {
    // 共享空间 JSObject 不绑定。
    return false;
  }
  // Wasm struct/array 不绑定。
  !is_wasm_object(ty)
}

/// 原版 IsJSApiWrapperObject。
#[inline]
#[must_use]
pub const fn is_js_api_wrapper_object(ty: InstanceType) -> bool {
  is_jsapi_object_with_embedder_slots(ty) || is_js_special_object(ty)
}

/// 原版 IsCppHeapPointerWrapperObject。
#[inline]
#[must_use]
pub const fn is_cpp_heap_pointer_wrapper_object(ty: InstanceType) -> bool {
  is_js_api_wrapper_object(ty)
    || is_cpp_heap_external_object(ty)
    || is_cpp_gc_managed_base(ty)
    || is_embedder_data_array(ty)
}

/// 原版 IsSpecialReceiverInstanceType (objects-inl.h:916)。
#[inline]
#[must_use]
pub const fn is_special_receiver_instance_type(ty: InstanceType) -> bool {
  ty.raw() <= LAST_SPECIAL_RECEIVER_TYPE.raw()
}

/// 原版 IsCustomElementsReceiverInstanceType (objects-inl.h:929)。
#[inline]
#[must_use]
pub const fn is_custom_elements_receiver_instance_type(ty: InstanceType) -> bool {
  ty.raw() <= LAST_CUSTOM_ELEMENTS_RECEIVER.raw()
}

// ── 对数登记 ──
/// SINGLE 桶检查器数 (含 BigInt/FixedArrayExact 追加)。
pub const SINGLE_CHECKER_COUNT: usize = 275;
/// RANGE 桶检查器数 (含 CallableJSFunction 追加)。
pub const RANGE_CHECKER_COUNT: usize = 59;
/// MULTIPLE 桶条目数 (不产生 Is 检查器, 仅类型登记)。
pub const MULTIPLE_BUCKET_COUNT: usize = 16;
