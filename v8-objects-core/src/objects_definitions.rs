// 由 tool/src/objects/objects-definitions.h.js 从 v8/src/objects/objects-definitions.h 生成; 修订就地改, 复跑覆盖
// X-macro 全量展开 (clang -E 对数, 配置 INTL+WASM, TEMPORAL/METAGEN 关) — INSTANCE_TYPE_LIST 归 instance_type.rs

/// 字符串类型表 (原版 STRING_TYPE_LIST, 列: 类型|大小表达式|根名|类名; 顺序关系只读堆布局)
pub const STRING_TYPE_LIST: [&str; 24] = [
  "SHARED_SEQ_TWO_BYTE_STRING_TYPE|kVariableSizeSentinel|shared_seq_two_byte_string|SharedSeqTwoByteString",
  "SHARED_SEQ_ONE_BYTE_STRING_TYPE|kVariableSizeSentinel|shared_seq_one_byte_string|SharedSeqOneByteString",
  "SEQ_TWO_BYTE_STRING_TYPE|kVariableSizeSentinel|seq_two_byte_string|SeqTwoByteString",
  "SEQ_ONE_BYTE_STRING_TYPE|kVariableSizeSentinel|seq_one_byte_string|SeqOneByteString",
  "INTERNALIZED_TWO_BYTE_STRING_TYPE|kVariableSizeSentinel|internalized_two_byte_string|InternalizedTwoByteString",
  "INTERNALIZED_ONE_BYTE_STRING_TYPE|kVariableSizeSentinel|internalized_one_byte_string|InternalizedOneByteString",
  "EXTERNAL_INTERNALIZED_TWO_BYTE_STRING_TYPE|",
  "EXTERNAL_INTERNALIZED_ONE_BYTE_STRING_TYPE|",
  "UNCACHED_EXTERNAL_INTERNALIZED_TWO_BYTE_STRING_TYPE|",
  "UNCACHED_EXTERNAL_INTERNALIZED_ONE_BYTE_STRING_TYPE|",
  "UNCACHED_EXTERNAL_TWO_BYTE_STRING_TYPE|",
  "UNCACHED_EXTERNAL_ONE_BYTE_STRING_TYPE|",
  "SHARED_UNCACHED_EXTERNAL_TWO_BYTE_STRING_TYPE|",
  "SHARED_UNCACHED_EXTERNAL_ONE_BYTE_STRING_TYPE|",
  "SHARED_EXTERNAL_TWO_BYTE_STRING_TYPE|",
  "SHARED_EXTERNAL_ONE_BYTE_STRING_TYPE|",
  "EXTERNAL_TWO_BYTE_STRING_TYPE|",
  "EXTERNAL_ONE_BYTE_STRING_TYPE|",
  "CONS_TWO_BYTE_STRING_TYPE|",
  "CONS_ONE_BYTE_STRING_TYPE|",
  "SLICED_TWO_BYTE_STRING_TYPE|",
  "SLICED_ONE_BYTE_STRING_TYPE|",
  "THIN_TWO_BYTE_STRING_TYPE|",
  "THIN_ONE_BYTE_STRING_TYPE|",
];

/// Struct 族表 (原版 STRUCT_LIST, 列: 实例类型名|类名|字段名)
pub const STRUCT_LIST: [&str; 40] = [
  "PROMISE_FULFILL_REACTION_JOB_TASK_TYPE|PromiseFulfillReactionJobTask|promise_fulfill_reaction_job_task",
  "PROMISE_REJECT_REACTION_JOB_TASK_TYPE|PromiseRejectReactionJobTask|promise_reject_reaction_job_task",
  "ASYNC_RESUME_TASK_TYPE|AsyncResumeTask|async_resume_task",
  "CALLABLE_TASK_TYPE|CallableTask|callable_task",
  "CALLBACK_TASK_TYPE|CallbackTask|callback_task",
  "PROMISE_RESOLVE_THENABLE_JOB_TASK_TYPE|PromiseResolveThenableJobTask|promise_resolve_thenable_job_task",
  "ACCESS_CHECK_INFO_TYPE|AccessCheckInfo|access_check_info",
  "ACCESSOR_PAIR_TYPE|AccessorPair|accessor_pair",
  "ALIASED_ARGUMENTS_ENTRY_TYPE|AliasedArgumentsEntry|aliased_arguments_entry",
  "ALLOCATION_MEMENTO_TYPE|AllocationMemento|allocation_memento",
  "ARRAY_BOILERPLATE_DESCRIPTION_TYPE|ArrayBoilerplateDescription|array_boilerplate_description",
  "ASYNC_GENERATOR_REQUEST_TYPE|AsyncGeneratorRequest|async_generator_request",
  "BREAK_POINT_TYPE|BreakPoint|break_point",
  "BREAK_POINT_INFO_TYPE|BreakPointInfo|break_point_info",
  "BYTECODE_WRAPPER_TYPE|BytecodeWrapper|bytecode_wrapper",
  "CALL_SITE_INFO_TYPE|CallSiteInfo|call_site_info",
  "CLASS_BOILERPLATE_TYPE|ClassBoilerplate|class_boilerplate",
  "CLASS_POSITIONS_TYPE|ClassPositions|class_positions",
  "CODE_WRAPPER_TYPE|CodeWrapper|code_wrapper",
  "DEBUG_SCRIPT_SCOPE_INFO_TYPE|DebugScriptScopeInfo|debug_script_scope_info",
  "ENUM_CACHE_TYPE|EnumCache|enum_cache",
  "ERROR_STACK_DATA_TYPE|ErrorStackData|error_stack_data",
  "FOR_IN_ENUMERATOR_HOLDER_TYPE|ForInEnumeratorHolder|for_in_enumerator_holder",
  "FUNCTION_TEMPLATE_RARE_DATA_TYPE|FunctionTemplateRareData|function_template_rare_data",
  "MODULE_REQUEST_TYPE|ModuleRequest|module_request",
  "PROMISE_CAPABILITY_TYPE|PromiseCapability|promise_capability",
  "PROMISE_REACTION_TYPE|PromiseReaction|promise_reaction",
  "PROPERTY_DESCRIPTOR_OBJECT_TYPE|PropertyDescriptorObject|property_descriptor_object",
  "PROTOTYPE_INFO_TYPE|PrototypeInfo|prototype_info",
  "PROTOTYPE_SHARED_CLOSURE_INFO_TYPE|PrototypeSharedClosureInfo|prototype_shared_closure_info",
  "REG_EXP_BOILERPLATE_DESCRIPTION_TYPE|RegExpBoilerplateDescription|regexp_boilerplate_description",
  "REG_EXP_DATA_WRAPPER_TYPE|RegExpDataWrapper|regexp_data_wrapper",
  "SCRIPT_TYPE|Script|script",
  "SCRIPT_OR_MODULE_TYPE|ScriptOrModule|script_or_module",
  "SOURCE_TEXT_MODULE_INFO_ENTRY_TYPE|SourceTextModuleInfoEntry|module_info_entry",
  "STACK_FRAME_INFO_TYPE|StackFrameInfo|stack_frame_info",
  "STACK_TRACE_INFO_TYPE|StackTraceInfo|stack_trace_info",
  "TEMPLATE_OBJECT_DESCRIPTION_TYPE|TemplateObjectDescription|template_object_description",
  "TUPLE2_TYPE|Tuple2|tuple2",
  "WASM_EXCEPTION_TAG_TYPE|WasmExceptionTag|wasm_exception_tag",
];

/// Struct 族 Map 表 (原版 STRUCT_MAPS_LIST, 列: map 根名|Map 类名)
pub const STRUCT_MAPS_LIST: [&str; 40] = [
  "PromiseFulfillReactionJobTaskMap|promise_fulfill_reaction_job_task_map",
  "PromiseRejectReactionJobTaskMap|promise_reject_reaction_job_task_map",
  "AsyncResumeTaskMap|async_resume_task_map",
  "CallableTaskMap|callable_task_map",
  "CallbackTaskMap|callback_task_map",
  "PromiseResolveThenableJobTaskMap|promise_resolve_thenable_job_task_map",
  "AccessCheckInfoMap|access_check_info_map",
  "AccessorPairMap|accessor_pair_map",
  "AliasedArgumentsEntryMap|aliased_arguments_entry_map",
  "AllocationMementoMap|allocation_memento_map",
  "ArrayBoilerplateDescriptionMap|array_boilerplate_description_map",
  "AsyncGeneratorRequestMap|async_generator_request_map",
  "BreakPointMap|break_point_map",
  "BreakPointInfoMap|break_point_info_map",
  "BytecodeWrapperMap|bytecode_wrapper_map",
  "CallSiteInfoMap|call_site_info_map",
  "ClassBoilerplateMap|class_boilerplate_map",
  "ClassPositionsMap|class_positions_map",
  "CodeWrapperMap|code_wrapper_map",
  "DebugScriptScopeInfoMap|debug_script_scope_info_map",
  "EnumCacheMap|enum_cache_map",
  "ErrorStackDataMap|error_stack_data_map",
  "ForInEnumeratorHolderMap|for_in_enumerator_holder_map",
  "FunctionTemplateRareDataMap|function_template_rare_data_map",
  "ModuleRequestMap|module_request_map",
  "PromiseCapabilityMap|promise_capability_map",
  "PromiseReactionMap|promise_reaction_map",
  "PropertyDescriptorObjectMap|property_descriptor_object_map",
  "PrototypeInfoMap|prototype_info_map",
  "PrototypeSharedClosureInfoMap|prototype_shared_closure_info_map",
  "RegExpBoilerplateDescriptionMap|regexp_boilerplate_description_map",
  "RegExpDataWrapperMap|regexp_data_wrapper_map",
  "ScriptMap|script_map",
  "ScriptOrModuleMap|script_or_module_map",
  "SourceTextModuleInfoEntryMap|module_info_entry_map",
  "StackFrameInfoMap|stack_frame_info_map",
  "StackTraceInfoMap|stack_trace_info_map",
  "TemplateObjectDescriptionMap|template_object_description_map",
  "Tuple2Map|tuple2_map",
  "WasmExceptionTagMap|wasm_exception_tag_map",
];

/// AllocationSite 形态表 (原版 ALLOCATION_SITE_LIST, 列: 实例类型|类名|形态|根名)
pub const ALLOCATION_SITE_LIST: [&str; 2] = [
  "ALLOCATION_SITE_TYPE|AllocationSite|WithWeakNext|allocation_site",
  "ALLOCATION_SITE_TYPE|AllocationSite|WithoutWeakNext|allocation_site_without_weaknext",
];

/// DataHandler 形态表 (原版 DATA_HANDLER_LIST, 列: 实例类型|类名|变体序号|根名)
pub const DATA_HANDLER_LIST: [&str; 9] = [
  "LOAD_HANDLER_TYPE|LoadHandler|1|load_handler1",
  "LOAD_HANDLER_TYPE|LoadHandler|2|load_handler2",
  "LOAD_HANDLER_TYPE|LoadHandler|3|load_handler3",
  "LOAD_HANDLER_TYPE|LoadHandler|4|load_handler4",
  "LOAD_HANDLER_TYPE|LoadHandler|5|load_handler5",
  "STORE_HANDLER_TYPE|StoreHandler|0|store_handler0",
  "STORE_HANDLER_TYPE|StoreHandler|1|store_handler1",
  "STORE_HANDLER_TYPE|StoreHandler|2|store_handler2",
  "STORE_HANDLER_TYPE|StoreHandler|3|store_handler3",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(STRING_TYPE_LIST.len() == 24);
  assert!(STRING_TYPE_LIST[0].starts_with("SHARED_SEQ_TWO_BYTE_STRING_TYPE|"));
  assert!(STRUCT_LIST.len() == 40);
  assert!(STRUCT_MAPS_LIST.len() == 40);
  assert!(STRUCT_LIST[0].contains("PromiseFulfillReactionJobTask"));
  assert!(STRUCT_LIST[39].contains("WasmExceptionTag"));
  assert!(ALLOCATION_SITE_LIST.len() == 2);
  assert!(DATA_HANDLER_LIST.len() == 9);
};
