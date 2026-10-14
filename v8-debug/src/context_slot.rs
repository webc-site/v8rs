// 由 tool/src/objects/contexts.tq.js 从 v8/src/objects/contexts.{tq,h} 生成; 修订就地改, 复跑覆盖
// Context::Field X-macro 全量展开 (294 native 字段, V8_TEMPORAL_SUPPORT 关闭分支; 开启时追加 10 条 TEMPORAL_* 登记于文档)
//! Context 槽位体系: `Context::Field` 枚举 + NativeContext 字段名表 + 查找标志
//! + Context/ScriptContextTable/ContextCell 布局公式, 压缩指针 K_TAGGED_SIZE=4 默认基准, 双态。
//!
//! 差集登记: ContextCellState/ContextMode/ContextCell 字段表/Context 子类清单归
//! tool/src/debug/contexts.js → v8-debug/src/contexts.rs; 本文件独占 X-macro 展开,
//! 对方 static_assert 锚所需 FIRST_WEAK_SLOT 与 size_for(NATIVE_CONTEXT_SLOTS) 数值由此供给。
//!
//! TEMPORAL 追加字段默认关闭共 10 条: JS_TEMPORAL_DURATION_FUNCTION_INDEX, JS_TEMPORAL_INSTANT_FUNCTION_INDEX, JS_TEMPORAL_PLAIN_DATE_FUNCTION_INDEX, JS_TEMPORAL_PLAIN_DATE_TIME_FUNCTION_INDEX, JS_TEMPORAL_PLAIN_MONTH_DAY_FUNCTION_INDEX, JS_TEMPORAL_PLAIN_TIME_FUNCTION_INDEX, JS_TEMPORAL_PLAIN_YEAR_MONTH_FUNCTION_INDEX, JS_TEMPORAL_TIME_ZONE_FUNCTION_INDEX, JS_TEMPORAL_ZONED_DATE_TIME_FUNCTION_INDEX, TEMPORAL_OBJECT_INDEX。

use v8_common::{K_HEAP_OBJECT_TAG, K_TAGGED_SIZE, TAGGED_SIZE_8_BYTES};

/// Context 通用槽位 (原版 Context::Field 前三项 + 尾部合计项, 全量镜像)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum ContextSlot {
  /// SCOPE_INFO_INDEX
  ScopeInfoIndex = 0,
  /// PREVIOUS_INDEX
  PreviousIndex = 1,
  /// EXTENSION_INDEX (仅 ScopeInfo::HasContextExtensionSlot 为真时存在)
  ExtensionIndex = 2,
  /// GLOBAL_PROXY_INDEX (JSGlobalProxy) — accessor global_proxy_object
  GlobalProxyIndex = 3,
  /// EMBEDDER_DATA_INDEX (HeapObject) — accessor embedder_data
  EmbedderDataIndex = 4,
  /// CONTINUATION_PRESERVED_EMBEDDER_DATA_INDEX (HeapObject) — accessor continuation_preserved_embedder_data
  ContinuationPreservedEmbedderDataIndex = 5,
  /// GENERATOR_NEXT_INTERNAL (JSFunction) — accessor generator_next_internal
  GeneratorNextInternal = 6,
  /// ASYNC_MODULE_EVALUATE_INTERNAL (JSFunction) — accessor async_module_evaluate_internal
  AsyncModuleEvaluateInternal = 7,
  /// REFLECT_APPLY_INDEX (JSFunction) — accessor reflect_apply
  ReflectApplyIndex = 8,
  /// REFLECT_CONSTRUCT_INDEX (JSFunction) — accessor reflect_construct
  ReflectConstructIndex = 9,
  /// PERFORM_PROMISE_THEN_INDEX (JSFunction) — accessor perform_promise_then
  PerformPromiseThenIndex = 10,
  /// PROMISE_THEN_INDEX (JSFunction) — accessor promise_then
  PromiseThenIndex = 11,
  /// PROMISE_RESOLVE_INDEX (JSFunction) — accessor promise_resolve
  PromiseResolveIndex = 12,
  /// FUNCTION_PROTOTYPE_APPLY_INDEX (JSFunction) — accessor function_prototype_apply
  FunctionPrototypeApplyIndex = 13,
  /// UINT8_ARRAY_FUN_INDEX (JSFunction) — accessor uint8_array_fun
  Uint8ArrayFunIndex = 14,
  /// INT8_ARRAY_FUN_INDEX (JSFunction) — accessor int8_array_fun
  Int8ArrayFunIndex = 15,
  /// UINT16_ARRAY_FUN_INDEX (JSFunction) — accessor uint16_array_fun
  Uint16ArrayFunIndex = 16,
  /// INT16_ARRAY_FUN_INDEX (JSFunction) — accessor int16_array_fun
  Int16ArrayFunIndex = 17,
  /// UINT32_ARRAY_FUN_INDEX (JSFunction) — accessor uint32_array_fun
  Uint32ArrayFunIndex = 18,
  /// INT32_ARRAY_FUN_INDEX (JSFunction) — accessor int32_array_fun
  Int32ArrayFunIndex = 19,
  /// BIGUINT64_ARRAY_FUN_INDEX (JSFunction) — accessor biguint64_array_fun
  Biguint64ArrayFunIndex = 20,
  /// BIGINT64_ARRAY_FUN_INDEX (JSFunction) — accessor bigint64_array_fun
  Bigint64ArrayFunIndex = 21,
  /// UINT8_CLAMPED_ARRAY_FUN_INDEX (JSFunction) — accessor uint8_clamped_array_fun
  Uint8ClampedArrayFunIndex = 22,
  /// FLOAT32_ARRAY_FUN_INDEX (JSFunction) — accessor float32_array_fun
  Float32ArrayFunIndex = 23,
  /// FLOAT64_ARRAY_FUN_INDEX (JSFunction) — accessor float64_array_fun
  Float64ArrayFunIndex = 24,
  /// FLOAT16_ARRAY_FUN_INDEX (JSFunction) — accessor float16_array_fun
  Float16ArrayFunIndex = 25,
  /// RAB_GSAB_UINT8_ARRAY_MAP_INDEX (Map) — accessor rab_gsab_uint8_array_map
  RabGsabUint8ArrayMapIndex = 26,
  /// RAB_GSAB_INT8_ARRAY_MAP_INDEX (Map) — accessor rab_gsab_int8_array_map
  RabGsabInt8ArrayMapIndex = 27,
  /// RAB_GSAB_UINT16_ARRAY_MAP_INDEX (Map) — accessor rab_gsab_uint16_array_map
  RabGsabUint16ArrayMapIndex = 28,
  /// RAB_GSAB_INT16_ARRAY_MAP_INDEX (Map) — accessor rab_gsab_int16_array_map
  RabGsabInt16ArrayMapIndex = 29,
  /// RAB_GSAB_UINT32_ARRAY_MAP_INDEX (Map) — accessor rab_gsab_uint32_array_map
  RabGsabUint32ArrayMapIndex = 30,
  /// RAB_GSAB_INT32_ARRAY_MAP_INDEX (Map) — accessor rab_gsab_int32_array_map
  RabGsabInt32ArrayMapIndex = 31,
  /// RAB_GSAB_BIGUINT64_ARRAY_MAP_INDEX (Map) — accessor rab_gsab_biguint64_array_map
  RabGsabBiguint64ArrayMapIndex = 32,
  /// RAB_GSAB_BIGINT64_ARRAY_MAP_INDEX (Map) — accessor rab_gsab_bigint64_array_map
  RabGsabBigint64ArrayMapIndex = 33,
  /// RAB_GSAB_UINT8_CLAMPED_ARRAY_MAP_INDEX (Map) — accessor rab_gsab_uint8_clamped_array_map
  RabGsabUint8ClampedArrayMapIndex = 34,
  /// RAB_GSAB_FLOAT32_ARRAY_MAP_INDEX (Map) — accessor rab_gsab_float32_array_map
  RabGsabFloat32ArrayMapIndex = 35,
  /// RAB_GSAB_FLOAT64_ARRAY_MAP_INDEX (Map) — accessor rab_gsab_float64_array_map
  RabGsabFloat64ArrayMapIndex = 36,
  /// RAB_GSAB_FLOAT16_ARRAY_MAP_INDEX (Map) — accessor rab_gsab_float16_array_map
  RabGsabFloat16ArrayMapIndex = 37,
  /// ABSTRACT_MODULE_SOURCE_FUNCTION_INDEX (JSFunction) — accessor abstract_module_source_function
  AbstractModuleSourceFunctionIndex = 38,
  /// ABSTRACT_MODULE_SOURCE_PROTOTYPE_INDEX (JSObject) — accessor abstract_module_source_prototype
  AbstractModuleSourcePrototypeIndex = 39,
  /// ACCESSOR_PROPERTY_DESCRIPTOR_MAP_INDEX (Map) — accessor accessor_property_descriptor_map
  AccessorPropertyDescriptorMapIndex = 40,
  /// ALLOW_CODE_GEN_FROM_STRINGS_INDEX (Object) — accessor allow_code_gen_from_strings
  AllowCodeGenFromStringsIndex = 41,
  /// ARRAY_BUFFER_FUN_INDEX (JSFunction) — accessor array_buffer_fun
  ArrayBufferFunIndex = 42,
  /// ARRAY_BUFFER_MAP_INDEX (Map) — accessor array_buffer_map
  ArrayBufferMapIndex = 43,
  /// ARRAY_BUFFER_NOINIT_FUN_INDEX (JSFunction) — accessor array_buffer_noinit_fun
  ArrayBufferNoinitFunIndex = 44,
  /// ARRAY_FUNCTION_INDEX (JSFunction) — accessor array_function
  ArrayFunctionIndex = 45,
  /// ARRAY_JOIN_STACK_INDEX (HeapObject) — accessor array_join_stack
  ArrayJoinStackIndex = 46,
  /// ARRAY_FROM_ASYNC_INDEX (JSFunction) — accessor from_async
  ArrayFromAsyncIndex = 47,
  /// ASYNC_FROM_SYNC_ITERATOR_MAP_INDEX (Map) — accessor async_from_sync_iterator_map
  AsyncFromSyncIteratorMapIndex = 48,
  /// ASYNC_FUNCTION_FUNCTION_INDEX (JSFunction) — accessor async_function_constructor
  AsyncFunctionFunctionIndex = 49,
  /// ASYNC_FUNCTION_OBJECT_MAP_INDEX (Map) — accessor async_function_object_map
  AsyncFunctionObjectMapIndex = 50,
  /// ASYNC_GENERATOR_FUNCTION_FUNCTION_INDEX (JSFunction) — accessor async_generator_function_function
  AsyncGeneratorFunctionFunctionIndex = 51,
  /// BIGINT_FUNCTION_INDEX (JSFunction) — accessor bigint_function
  BigintFunctionIndex = 52,
  /// BOOLEAN_FUNCTION_INDEX (JSFunction) — accessor boolean_function
  BooleanFunctionIndex = 53,
  /// BOUND_FUNCTION_WITH_CONSTRUCTOR_MAP_INDEX (Map) — accessor bound_function_with_constructor_map
  BoundFunctionWithConstructorMapIndex = 54,
  /// BOUND_FUNCTION_WITHOUT_CONSTRUCTOR_MAP_INDEX (Map) — accessor bound_function_without_constructor_map
  BoundFunctionWithoutConstructorMapIndex = 55,
  /// CALL_AS_CONSTRUCTOR_DELEGATE_INDEX (JSFunction) — accessor call_as_constructor_delegate
  CallAsConstructorDelegateIndex = 56,
  /// CALL_AS_FUNCTION_DELEGATE_INDEX (JSFunction) — accessor call_as_function_delegate
  CallAsFunctionDelegateIndex = 57,
  /// CALLSITE_FUNCTION_INDEX (JSFunction) — accessor callsite_function
  CallsiteFunctionIndex = 58,
  /// CONTEXT_EXTENSION_FUNCTION_INDEX (JSFunction) — accessor context_extension_function
  ContextExtensionFunctionIndex = 59,
  /// DATA_PROPERTY_DESCRIPTOR_MAP_INDEX (Map) — accessor data_property_descriptor_map
  DataPropertyDescriptorMapIndex = 60,
  /// DATA_VIEW_FUN_INDEX (JSFunction) — accessor data_view_fun
  DataViewFunIndex = 61,
  /// DATE_FUNCTION_INDEX (JSFunction) — accessor date_function
  DateFunctionIndex = 62,
  /// DEBUG_CONTEXT_ID_INDEX ((UnionOf<Smi, Undefined>)) — accessor debug_context_id
  DebugContextIdIndex = 63,
  /// EMPTY_FUNCTION_INDEX (JSFunction) — accessor empty_function
  EmptyFunctionIndex = 64,
  /// ERROR_MESSAGE_FOR_CODE_GEN_FROM_STRINGS_INDEX (Object) — accessor error_message_for_code_gen_from_strings
  ErrorMessageForCodeGenFromStringsIndex = 65,
  /// ERROR_MESSAGE_FOR_WASM_CODE_GEN_INDEX (Object) — accessor error_message_for_wasm_code_gen
  ErrorMessageForWasmCodeGenIndex = 66,
  /// ERRORS_THROWN_INDEX (Smi) — accessor errors_thrown
  ErrorsThrownIndex = 67,
  /// EXTRAS_BINDING_OBJECT_INDEX (JSObject) — accessor extras_binding_object
  ExtrasBindingObjectIndex = 68,
  /// FAST_ALIASED_ARGUMENTS_MAP_INDEX (Map) — accessor fast_aliased_arguments_map
  FastAliasedArgumentsMapIndex = 69,
  /// FAST_TEMPLATE_INSTANTIATIONS_CACHE_INDEX (FixedArray) — accessor fast_template_instantiations_cache
  FastTemplateInstantiationsCacheIndex = 70,
  /// FUNCTION_FUNCTION_INDEX (JSFunction) — accessor function_function
  FunctionFunctionIndex = 71,
  /// FUNCTION_PROTOTYPE_INDEX (JSObject) — accessor function_prototype
  FunctionPrototypeIndex = 72,
  /// GENERATOR_FUNCTION_FUNCTION_INDEX (JSFunction) — accessor generator_function_function
  GeneratorFunctionFunctionIndex = 73,
  /// GENERATOR_OBJECT_PROTOTYPE_MAP_INDEX (Map) — accessor generator_object_prototype_map
  GeneratorObjectPrototypeMapIndex = 74,
  /// ASYNC_GENERATOR_OBJECT_PROTOTYPE_MAP_INDEX (Map) — accessor async_generator_object_prototype_map
  AsyncGeneratorObjectPrototypeMapIndex = 75,
  /// INITIAL_ARRAY_ITERATOR_MAP_INDEX (Map) — accessor initial_array_iterator_map
  InitialArrayIteratorMapIndex = 76,
  /// INITIAL_ARRAY_ITERATOR_PROTOTYPE_INDEX (JSObject) — accessor initial_array_iterator_prototype
  InitialArrayIteratorPrototypeIndex = 77,
  /// INITIAL_ARRAY_PROTOTYPE_INDEX (JSObject) — accessor initial_array_prototype
  InitialArrayPrototypeIndex = 78,
  /// INITIAL_ERROR_PROTOTYPE_INDEX (JSObject) — accessor initial_error_prototype
  InitialErrorPrototypeIndex = 79,
  /// INITIAL_GENERATOR_PROTOTYPE_INDEX (JSObject) — accessor initial_generator_prototype
  InitialGeneratorPrototypeIndex = 80,
  /// INITIAL_ASYNC_ITERATOR_PROTOTYPE_INDEX (JSObject) — accessor initial_async_iterator_prototype
  InitialAsyncIteratorPrototypeIndex = 81,
  /// INITIAL_ASYNC_GENERATOR_PROTOTYPE_INDEX (JSObject) — accessor initial_async_generator_prototype
  InitialAsyncGeneratorPrototypeIndex = 82,
  /// INITIAL_ITERATOR_FUNCTION_INDEX (JSFunction) — accessor initial_iterator_function
  InitialIteratorFunctionIndex = 83,
  /// INITIAL_ITERATOR_HELPER_PROTOTYPE_INDEX (JSObject) — accessor initial_iterator_helper_prototype
  InitialIteratorHelperPrototypeIndex = 84,
  /// INITIAL_ITERATOR_PROTOTYPE_INDEX (JSObject) — accessor initial_iterator_prototype
  InitialIteratorPrototypeIndex = 85,
  /// INITIAL_DISPOSABLE_STACK_PROTOTYPE_INDEX (JSObject) — accessor initial_disposable_stack_prototype
  InitialDisposableStackPrototypeIndex = 86,
  /// INITIAL_MAP_ITERATOR_PROTOTYPE_INDEX (JSObject) — accessor initial_map_iterator_prototype
  InitialMapIteratorPrototypeIndex = 87,
  /// INITIAL_MAP_PROTOTYPE_MAP_INDEX (Map) — accessor initial_map_prototype_map
  InitialMapPrototypeMapIndex = 88,
  /// INITIAL_OBJECT_PROTOTYPE_INDEX (JSObject) — accessor initial_object_prototype
  InitialObjectPrototypeIndex = 89,
  /// INITIAL_SET_ITERATOR_PROTOTYPE_INDEX (JSObject) — accessor initial_set_iterator_prototype
  InitialSetIteratorPrototypeIndex = 90,
  /// INITIAL_SET_PROTOTYPE_INDEX (JSObject) — accessor initial_set_prototype
  InitialSetPrototypeIndex = 91,
  /// INITIAL_SET_PROTOTYPE_MAP_INDEX (Map) — accessor initial_set_prototype_map
  InitialSetPrototypeMapIndex = 92,
  /// INITIAL_STRING_ITERATOR_MAP_INDEX (Map) — accessor initial_string_iterator_map
  InitialStringIteratorMapIndex = 93,
  /// INITIAL_STRING_ITERATOR_PROTOTYPE_INDEX (JSObject) — accessor initial_string_iterator_prototype
  InitialStringIteratorPrototypeIndex = 94,
  /// INITIAL_STRING_PROTOTYPE_INDEX (JSObject) — accessor initial_string_prototype
  InitialStringPrototypeIndex = 95,
  /// INITIAL_WEAKMAP_PROTOTYPE_MAP_INDEX (Map) — accessor initial_weakmap_prototype_map
  InitialWeakmapPrototypeMapIndex = 96,
  /// INITIAL_WEAKSET_PROTOTYPE_MAP_INDEX (Map) — accessor initial_weakset_prototype_map
  InitialWeaksetPrototypeMapIndex = 97,
  /// INTL_COLLATOR_FUNCTION_INDEX (JSFunction) — accessor intl_collator_function
  IntlCollatorFunctionIndex = 98,
  /// INTL_DATE_TIME_FORMAT_FUNCTION_INDEX (JSFunction) — accessor intl_date_time_format_function
  IntlDateTimeFormatFunctionIndex = 99,
  /// INTL_DISPLAY_NAMES_FUNCTION_INDEX (JSFunction) — accessor intl_display_names_function
  IntlDisplayNamesFunctionIndex = 100,
  /// INTL_DURATION_FORMAT_FUNCTION_INDEX (JSFunction) — accessor intl_duration_format_function
  IntlDurationFormatFunctionIndex = 101,
  /// INTL_NUMBER_FORMAT_FUNCTION_INDEX (JSFunction) — accessor intl_number_format_function
  IntlNumberFormatFunctionIndex = 102,
  /// INTL_LOCALE_FUNCTION_INDEX (JSFunction) — accessor intl_locale_function
  IntlLocaleFunctionIndex = 103,
  /// INTL_LIST_FORMAT_FUNCTION_INDEX (JSFunction) — accessor intl_list_format_function
  IntlListFormatFunctionIndex = 104,
  /// INTL_PLURAL_RULES_FUNCTION_INDEX (JSFunction) — accessor intl_plural_rules_function
  IntlPluralRulesFunctionIndex = 105,
  /// INTL_RELATIVE_TIME_FORMAT_FUNCTION_INDEX (JSFunction) — accessor intl_relative_time_format_function
  IntlRelativeTimeFormatFunctionIndex = 106,
  /// INTL_SEGMENTER_FUNCTION_INDEX (JSFunction) — accessor intl_segmenter_function
  IntlSegmenterFunctionIndex = 107,
  /// INTL_SEGMENTS_MAP_INDEX (Map) — accessor intl_segments_map
  IntlSegmentsMapIndex = 108,
  /// INTL_SEGMENT_DATA_OBJECT_MAP_INDEX (Map) — accessor intl_segment_data_object_map
  IntlSegmentDataObjectMapIndex = 109,
  /// INTL_SEGMENT_DATA_OBJECT_WORDLIKE_MAP_INDEX (Map) — accessor intl_segment_data_object_wordlike_map
  IntlSegmentDataObjectWordlikeMapIndex = 110,
  /// INTL_SEGMENT_ITERATOR_MAP_INDEX (Map) — accessor intl_segment_iterator_map
  IntlSegmentIteratorMapIndex = 111,
  /// ITERATOR_FILTER_HELPER_MAP_INDEX (Map) — accessor iterator_filter_helper_map
  IteratorFilterHelperMapIndex = 112,
  /// ITERATOR_MAP_HELPER_MAP_INDEX (Map) — accessor iterator_map_helper_map
  IteratorMapHelperMapIndex = 113,
  /// ITERATOR_TAKE_HELPER_MAP_INDEX (Map) — accessor iterator_take_helper_map
  IteratorTakeHelperMapIndex = 114,
  /// ITERATOR_DROP_HELPER_MAP_INDEX (Map) — accessor iterator_drop_helper_map
  IteratorDropHelperMapIndex = 115,
  /// ITERATOR_FLAT_MAP_HELPER_MAP_INDEX (Map) — accessor iterator_flatMap_helper_map
  IteratorFlatMapHelperMapIndex = 116,
  /// ITERATOR_CONCAT_HELPER_MAP_INDEX (Map) — accessor iterator_concat_helper_map
  IteratorConcatHelperMapIndex = 117,
  /// ITERATOR_ZIP_HELPER_MAP_INDEX (Map) — accessor iterator_zip_helper_map
  IteratorZipHelperMapIndex = 118,
  /// ITERATOR_ZIP_KEYED_HELPER_MAP_INDEX (Map) — accessor iterator_zip_keyed_helper_map
  IteratorZipKeyedHelperMapIndex = 119,
  /// ITERATOR_FUNCTION_INDEX (JSFunction) — accessor iterator_function
  IteratorFunctionIndex = 120,
  /// VALID_ITERATOR_WRAPPER_MAP_INDEX (Map) — accessor valid_iterator_wrapper_map
  ValidIteratorWrapperMapIndex = 121,
  /// ITERATOR_RESULT_MAP_INDEX (Map) — accessor iterator_result_map
  IteratorResultMapIndex = 122,
  /// JS_ARRAY_PACKED_SMI_ELEMENTS_MAP_INDEX (Map) — accessor js_array_packed_smi_elements_map
  JsArrayPackedSmiElementsMapIndex = 123,
  /// JS_ARRAY_HOLEY_SMI_ELEMENTS_MAP_INDEX (Map) — accessor js_array_holey_smi_elements_map
  JsArrayHoleySmiElementsMapIndex = 124,
  /// JS_ARRAY_PACKED_ELEMENTS_MAP_INDEX (Map) — accessor js_array_packed_elements_map
  JsArrayPackedElementsMapIndex = 125,
  /// JS_ARRAY_HOLEY_ELEMENTS_MAP_INDEX (Map) — accessor js_array_holey_elements_map
  JsArrayHoleyElementsMapIndex = 126,
  /// JS_ARRAY_PACKED_DOUBLE_ELEMENTS_MAP_INDEX (Map) — accessor js_array_packed_double_elements_map
  JsArrayPackedDoubleElementsMapIndex = 127,
  /// JS_ARRAY_HOLEY_DOUBLE_ELEMENTS_MAP_INDEX (Map) — accessor js_array_holey_double_elements_map
  JsArrayHoleyDoubleElementsMapIndex = 128,
  /// JS_ARRAY_TEMPLATE_LITERAL_OBJECT_MAP (Map) — accessor js_array_template_literal_object_map
  JsArrayTemplateLiteralObjectMap = 129,
  /// JS_DISPOSABLE_STACK_FUNCTION_INDEX (JSFunction) — accessor js_disposable_stack_function
  JsDisposableStackFunctionIndex = 130,
  /// JS_ASYNC_DISPOSABLE_STACK_FUNCTION_INDEX (JSFunction) — accessor js_async_disposable_stack_function
  JsAsyncDisposableStackFunctionIndex = 131,
  /// JS_DISPOSABLE_STACK_MAP_INDEX (Map) — accessor js_disposable_stack_map
  JsDisposableStackMapIndex = 132,
  /// JS_MAP_FUN_INDEX (JSFunction) — accessor js_map_fun
  JsMapFunIndex = 133,
  /// JS_MAP_MAP_INDEX (Map) — accessor js_map_map
  JsMapMapIndex = 134,
  /// JS_MODULE_NAMESPACE_MAP (Map) — accessor js_module_namespace_map
  JsModuleNamespaceMap = 135,
  /// JS_DEFERRED_MODULE_NAMESPACE_MAP (Map) — accessor js_deferred_module_namespace_map
  JsDeferredModuleNamespaceMap = 136,
  /// JS_RAW_JSON_MAP (Map) — accessor js_raw_json_map
  JsRawJsonMap = 137,
  /// JS_SET_FUN_INDEX (JSFunction) — accessor js_set_fun
  JsSetFunIndex = 138,
  /// JS_SET_MAP_INDEX (Map) — accessor js_set_map
  JsSetMapIndex = 139,
  /// JS_WEAK_MAP_FUN_INDEX (JSFunction) — accessor js_weak_map_fun
  JsWeakMapFunIndex = 140,
  /// JS_WEAK_SET_FUN_INDEX (JSFunction) — accessor js_weak_set_fun
  JsWeakSetFunIndex = 141,
  /// JS_WEAK_REF_FUNCTION_INDEX (JSFunction) — accessor js_weak_ref_fun
  JsWeakRefFunctionIndex = 142,
  /// JS_FINALIZATION_REGISTRY_FUNCTION_INDEX (JSFunction) — accessor js_finalization_registry_fun
  JsFinalizationRegistryFunctionIndex = 143,
  /// JSON_OBJECT (JSObject) — accessor json_object
  JsonObject = 144,
  /// PROMISE_WITHRESOLVERS_RESULT_MAP_INDEX (Map) — accessor promise_withresolvers_result_map
  PromiseWithresolversResultMapIndex = 145,
  /// META_MAP_INDEX (Map) — accessor meta_map
  MetaMapIndex = 146,
  /// FUNCTION_CONTEXT_MAP_INDEX (Map) — accessor function_context_map
  FunctionContextMapIndex = 147,
  /// MODULE_CONTEXT_MAP_INDEX (Map) — accessor module_context_map
  ModuleContextMapIndex = 148,
  /// EVAL_CONTEXT_MAP_INDEX (Map) — accessor eval_context_map
  EvalContextMapIndex = 149,
  /// SCRIPT_CONTEXT_MAP_INDEX (Map) — accessor script_context_map
  ScriptContextMapIndex = 150,
  /// AWAIT_CONTEXT_MAP_INDEX (Map) — accessor await_context_map
  AwaitContextMapIndex = 151,
  /// BLOCK_CONTEXT_MAP_INDEX (Map) — accessor block_context_map
  BlockContextMapIndex = 152,
  /// CATCH_CONTEXT_MAP_INDEX (Map) — accessor catch_context_map
  CatchContextMapIndex = 153,
  /// WITH_CONTEXT_MAP_INDEX (Map) — accessor with_context_map
  WithContextMapIndex = 154,
  /// DEBUG_EVALUATE_CONTEXT_MAP_INDEX (Map) — accessor debug_evaluate_context_map
  DebugEvaluateContextMapIndex = 155,
  /// JS_RAB_GSAB_DATA_VIEW_MAP_INDEX (Map) — accessor js_rab_gsab_data_view_map
  JsRabGsabDataViewMapIndex = 156,
  /// MAP_CACHE_INDEX (Object) — accessor map_cache
  MapCacheIndex = 157,
  /// MAP_KEY_ITERATOR_MAP_INDEX (Map) — accessor map_key_iterator_map
  MapKeyIteratorMapIndex = 158,
  /// MAP_KEY_VALUE_ITERATOR_MAP_INDEX (Map) — accessor map_key_value_iterator_map
  MapKeyValueIteratorMapIndex = 159,
  /// MAP_VALUE_ITERATOR_MAP_INDEX (Map) — accessor map_value_iterator_map
  MapValueIteratorMapIndex = 160,
  /// MATH_RANDOM_INDEX_INDEX (Smi) — accessor math_random_index
  MathRandomIndexIndex = 161,
  /// MATH_RANDOM_STATE_INDEX (ByteArray) — accessor math_random_state
  MathRandomStateIndex = 162,
  /// MATH_RANDOM_CACHE_INDEX ((UnionOf<Undefined, FixedDoubleArray>)) — accessor math_random_cache
  MathRandomCacheIndex = 163,
  /// NORMALIZED_MAP_CACHE_INDEX (Object) — accessor normalized_map_cache
  NormalizedMapCacheIndex = 164,
  /// NUMBER_FUNCTION_INDEX (JSFunction) — accessor number_function
  NumberFunctionIndex = 165,
  /// OBJECT_FUNCTION_INDEX (JSFunction) — accessor object_function
  ObjectFunctionIndex = 166,
  /// OBJECT_FUNCTION_PROTOTYPE_INDEX (JSObject) — accessor object_function_prototype
  ObjectFunctionPrototypeIndex = 167,
  /// OBJECT_FUNCTION_PROTOTYPE_MAP_INDEX (Map) — accessor object_function_prototype_map
  ObjectFunctionPrototypeMapIndex = 168,
  /// PROMISE_HOOK_INIT_FUNCTION_INDEX (Object) — accessor promise_hook_init_function
  PromiseHookInitFunctionIndex = 169,
  /// PROMISE_HOOK_BEFORE_FUNCTION_INDEX (Object) — accessor promise_hook_before_function
  PromiseHookBeforeFunctionIndex = 170,
  /// PROMISE_HOOK_AFTER_FUNCTION_INDEX (Object) — accessor promise_hook_after_function
  PromiseHookAfterFunctionIndex = 171,
  /// PROMISE_HOOK_RESOLVE_FUNCTION_INDEX (Object) — accessor promise_hook_resolve_function
  PromiseHookResolveFunctionIndex = 172,
  /// PROXY_CALLABLE_MAP_INDEX (Map) — accessor proxy_callable_map
  ProxyCallableMapIndex = 173,
  /// PROXY_CONSTRUCTOR_MAP_INDEX (Map) — accessor proxy_constructor_map
  ProxyConstructorMapIndex = 174,
  /// PROXY_FUNCTION_INDEX (JSFunction) — accessor proxy_function
  ProxyFunctionIndex = 175,
  /// PROXY_MAP_INDEX (Map) — accessor proxy_map
  ProxyMapIndex = 176,
  /// PROXY_REVOCABLE_RESULT_MAP_INDEX (Map) — accessor proxy_revocable_result_map
  ProxyRevocableResultMapIndex = 177,
  /// PROMISE_PROTOTYPE_INDEX (JSObject) — accessor promise_prototype
  PromisePrototypeIndex = 178,
  /// RECORDER_CONTEXT_ID (Object) — accessor recorder_context_id
  RecorderContextId = 179,
  /// REGEXP_EXEC_FUNCTION_INDEX (JSFunction) — accessor regexp_exec_function
  RegexpExecFunctionIndex = 180,
  /// REGEXP_FUNCTION_INDEX (JSFunction) — accessor regexp_function
  RegexpFunctionIndex = 181,
  /// REGEXP_LAST_MATCH_INFO_INDEX (RegExpMatchInfo) — accessor regexp_last_match_info
  RegexpLastMatchInfoIndex = 182,
  /// REGEXP_MATCH_ALL_FUNCTION_INDEX (JSFunction) — accessor regexp_match_all_function
  RegexpMatchAllFunctionIndex = 183,
  /// REGEXP_MATCH_FUNCTION_INDEX (JSFunction) — accessor regexp_match_function
  RegexpMatchFunctionIndex = 184,
  /// REGEXP_PROTOTYPE_INDEX (JSObject) — accessor regexp_prototype
  RegexpPrototypeIndex = 185,
  /// REGEXP_PROTOTYPE_MAP_INDEX (Map) — accessor regexp_prototype_map
  RegexpPrototypeMapIndex = 186,
  /// REGEXP_REPLACE_FUNCTION_INDEX (JSFunction) — accessor regexp_replace_function
  RegexpReplaceFunctionIndex = 187,
  /// REGEXP_RESULT_MAP_INDEX (Map) — accessor regexp_result_map
  RegexpResultMapIndex = 188,
  /// REGEXP_RESULT_WITH_INDICES_MAP_INDEX (Map) — accessor regexp_result_with_indices_map
  RegexpResultWithIndicesMapIndex = 189,
  /// REGEXP_RESULT_INDICES_MAP_INDEX (Map) — accessor regexp_result_indices_map
  RegexpResultIndicesMapIndex = 190,
  /// REGEXP_SEARCH_FUNCTION_INDEX (JSFunction) — accessor regexp_search_function
  RegexpSearchFunctionIndex = 191,
  /// REGEXP_SPLIT_FUNCTION_INDEX (JSFunction) — accessor regexp_split_function
  RegexpSplitFunctionIndex = 192,
  /// INITIAL_REGEXP_STRING_ITERATOR_PROTOTYPE_MAP_INDEX (Map) — accessor initial_regexp_string_iterator_prototype_map
  InitialRegexpStringIteratorPrototypeMapIndex = 193,
  /// SCRIPT_CONTEXT_TABLE_INDEX (ScriptContextTable) — accessor script_context_table
  ScriptContextTableIndex = 194,
  /// SCRIPT_EXECUTION_CALLBACK_INDEX (Object) — accessor script_execution_callback
  ScriptExecutionCallbackIndex = 195,
  /// TEMPORAL_GET_EPOCH_NANOSECONDS_CALLBACK_INDEX (Object) — accessor temporal_get_epoch_nanoseconds_callback
  TemporalGetEpochNanosecondsCallbackIndex = 196,
  /// SECURITY_TOKEN_INDEX (Object) — accessor security_token
  SecurityTokenIndex = 197,
  /// SERIALIZED_OBJECTS (HeapObject) — accessor serialized_objects
  SerializedObjects = 198,
  /// SET_VALUE_ITERATOR_MAP_INDEX (Map) — accessor set_value_iterator_map
  SetValueIteratorMapIndex = 199,
  /// SET_KEY_VALUE_ITERATOR_MAP_INDEX (Map) — accessor set_key_value_iterator_map
  SetKeyValueIteratorMapIndex = 200,
  /// SHARED_ARRAY_BUFFER_FUN_INDEX (JSFunction) — accessor shared_array_buffer_fun
  SharedArrayBufferFunIndex = 201,
  /// SLOPPY_ARGUMENTS_MAP_INDEX (Map) — accessor sloppy_arguments_map
  SloppyArgumentsMapIndex = 202,
  /// SLOW_ALIASED_ARGUMENTS_MAP_INDEX (Map) — accessor slow_aliased_arguments_map
  SlowAliasedArgumentsMapIndex = 203,
  /// STRICT_ARGUMENTS_MAP_INDEX (Map) — accessor strict_arguments_map
  StrictArgumentsMapIndex = 204,
  /// SLOW_OBJECT_WITH_NULL_PROTOTYPE_MAP (Map) — accessor slow_object_with_null_prototype_map
  SlowObjectWithNullPrototypeMap = 205,
  /// SLOW_OBJECT_WITH_OBJECT_PROTOTYPE_MAP (Map) — accessor slow_object_with_object_prototype_map
  SlowObjectWithObjectPrototypeMap = 206,
  /// SLOW_TEMPLATE_INSTANTIATIONS_CACHE_INDEX (EphemeronHashTable) — accessor slow_template_instantiations_cache
  SlowTemplateInstantiationsCacheIndex = 207,
  /// ATOMICS_WAITASYNC_PROMISES (OrderedHashSet) — accessor atomics_waitasync_promises
  AtomicsWaitasyncPromises = 208,
  /// SET_UINT8_ARRAY_RESULT_MAP (Map) — accessor set_unit8_array_result_map
  SetUint8ArrayResultMap = 209,
  /// WASM_DEBUG_MAPS (FixedArray) — accessor wasm_debug_maps
  WasmDebugMaps = 210,
  /// INITIAL_ARRAY_PROTOTYPE_VALIDITY_CELL_INDEX (Cell) — accessor initial_array_prototype_validity_cell
  InitialArrayPrototypeValidityCellIndex = 211,
  /// SLOPPY_FUNCTION_MAP_INDEX (Map) — accessor sloppy_function_map
  SloppyFunctionMapIndex = 212,
  /// SLOPPY_FUNCTION_WITH_NAME_MAP_INDEX (Map) — accessor sloppy_function_with_name_map
  SloppyFunctionWithNameMapIndex = 213,
  /// SLOPPY_FUNCTION_WITHOUT_PROTOTYPE_MAP_INDEX (Map) — accessor sloppy_function_without_prototype_map
  SloppyFunctionWithoutPrototypeMapIndex = 214,
  /// SLOPPY_FUNCTION_WITH_READONLY_PROTOTYPE_MAP_INDEX (Map) — accessor sloppy_function_with_readonly_prototype_map
  SloppyFunctionWithReadonlyPrototypeMapIndex = 215,
  /// STRICT_FUNCTION_MAP_INDEX (Map) — accessor strict_function_map
  StrictFunctionMapIndex = 216,
  /// STRICT_FUNCTION_WITH_NAME_MAP_INDEX (Map) — accessor strict_function_with_name_map
  StrictFunctionWithNameMapIndex = 217,
  /// STRICT_FUNCTION_WITH_READONLY_PROTOTYPE_MAP_INDEX (Map) — accessor strict_function_with_readonly_prototype_map
  StrictFunctionWithReadonlyPrototypeMapIndex = 218,
  /// STRICT_FUNCTION_WITHOUT_PROTOTYPE_MAP_INDEX (Map) — accessor strict_function_without_prototype_map
  StrictFunctionWithoutPrototypeMapIndex = 219,
  /// METHOD_WITH_NAME_MAP_INDEX (Map) — accessor method_with_name_map
  MethodWithNameMapIndex = 220,
  /// ASYNC_FUNCTION_MAP_INDEX (Map) — accessor async_function_map
  AsyncFunctionMapIndex = 221,
  /// ASYNC_FUNCTION_WITH_NAME_MAP_INDEX (Map) — accessor async_function_with_name_map
  AsyncFunctionWithNameMapIndex = 222,
  /// GENERATOR_FUNCTION_MAP_INDEX (Map) — accessor generator_function_map
  GeneratorFunctionMapIndex = 223,
  /// GENERATOR_FUNCTION_WITH_NAME_MAP_INDEX (Map) — accessor generator_function_with_name_map
  GeneratorFunctionWithNameMapIndex = 224,
  /// ASYNC_GENERATOR_FUNCTION_MAP_INDEX (Map) — accessor async_generator_function_map
  AsyncGeneratorFunctionMapIndex = 225,
  /// ASYNC_GENERATOR_FUNCTION_WITH_NAME_MAP_INDEX (Map) — accessor async_generator_function_with_name_map
  AsyncGeneratorFunctionWithNameMapIndex = 226,
  /// CLASS_FUNCTION_MAP_INDEX (Map) — accessor class_function_map
  ClassFunctionMapIndex = 227,
  /// STRING_FUNCTION_INDEX (JSFunction) — accessor string_function
  StringFunctionIndex = 228,
  /// STRING_FUNCTION_PROTOTYPE_MAP_INDEX (Map) — accessor string_function_prototype_map
  StringFunctionPrototypeMapIndex = 229,
  /// SYMBOL_FUNCTION_INDEX (JSFunction) — accessor symbol_function
  SymbolFunctionIndex = 230,
  /// IS_WASM_JS_INSTALLED_INDEX (Smi) — accessor is_wasm_js_installed
  IsWasmJsInstalledIndex = 231,
  /// IS_WASM_JSPI_INSTALLED_INDEX (Smi) — accessor is_wasm_jspi_installed
  IsWasmJspiInstalledIndex = 232,
  /// WASM_WEBASSEMBLY_OBJECT_INDEX (JSObject) — accessor wasm_webassembly_object
  WasmWebassemblyObjectIndex = 233,
  /// WASM_EXPORTED_FUNCTION_MAP_INDEX (Map) — accessor wasm_exported_function_map
  WasmExportedFunctionMapIndex = 234,
  /// WASM_TAG_CONSTRUCTOR_INDEX (JSFunction) — accessor wasm_tag_constructor
  WasmTagConstructorIndex = 235,
  /// WASM_EXCEPTION_CONSTRUCTOR_INDEX (JSFunction) — accessor wasm_exception_constructor
  WasmExceptionConstructorIndex = 236,
  /// WASM_GLOBAL_CONSTRUCTOR_INDEX (JSFunction) — accessor wasm_global_constructor
  WasmGlobalConstructorIndex = 237,
  /// WASM_INSTANCE_CONSTRUCTOR_INDEX (JSFunction) — accessor wasm_instance_constructor
  WasmInstanceConstructorIndex = 238,
  /// WASM_JS_TAG_INDEX (JSObject) — accessor wasm_js_tag
  WasmJsTagIndex = 239,
  /// WASM_MEMORY_CONSTRUCTOR_INDEX (JSFunction) — accessor wasm_memory_constructor
  WasmMemoryConstructorIndex = 240,
  /// WASM_MODULE_CONSTRUCTOR_INDEX (JSFunction) — accessor wasm_module_constructor
  WasmModuleConstructorIndex = 241,
  /// WASM_TABLE_CONSTRUCTOR_INDEX (JSFunction) — accessor wasm_table_constructor
  WasmTableConstructorIndex = 242,
  /// WASM_SUSPENDING_CONSTRUCTOR_INDEX (JSFunction) — accessor wasm_suspending_constructor
  WasmSuspendingConstructorIndex = 243,
  /// WASM_SUSPENDER_CONSTRUCTOR_INDEX (JSFunction) — accessor wasm_suspender_constructor
  WasmSuspenderConstructorIndex = 244,
  /// WASM_SUSPENDING_MAP (Map) — accessor wasm_suspending_map
  WasmSuspendingMap = 245,
  /// WASM_SUSPENDING_PROTOTYPE (JSObject) — accessor wasm_suspending_prototype
  WasmSuspendingPrototype = 246,
  /// WASM_MEMORY_MAP_DESCRIPTOR_TEMPLATE_INDEX (ObjectTemplateInfo) — accessor wasm_memory_map_descriptor_template
  WasmMemoryMapDescriptorTemplateIndex = 247,
  /// WASM_MEMORY_MAP_DESCRIPTOR_CONSTRUCTOR_INDEX (JSFunction) — accessor wasm_memory_map_descriptor_constructor
  WasmMemoryMapDescriptorConstructorIndex = 248,
  /// TEMPLATE_WEAKMAP_INDEX (HeapObject) — accessor template_weakmap
  TemplateWeakmapIndex = 249,
  /// TYPED_ARRAY_FUN_INDEX (JSFunction) — accessor typed_array_function
  TypedArrayFunIndex = 250,
  /// TYPED_ARRAY_PROTOTYPE_INDEX (JSObject) — accessor typed_array_prototype
  TypedArrayPrototypeIndex = 251,
  /// ARRAY_ENTRIES_ITERATOR_INDEX (JSFunction) — accessor array_entries_iterator
  ArrayEntriesIteratorIndex = 252,
  /// ARRAY_FOR_EACH_ITERATOR_INDEX (JSFunction) — accessor array_for_each_iterator
  ArrayForEachIteratorIndex = 253,
  /// ARRAY_KEYS_ITERATOR_INDEX (JSFunction) — accessor array_keys_iterator
  ArrayKeysIteratorIndex = 254,
  /// ARRAY_VALUES_ITERATOR_INDEX (JSFunction) — accessor array_values_iterator
  ArrayValuesIteratorIndex = 255,
  /// ERROR_FUNCTION_INDEX (JSFunction) — accessor error_function
  ErrorFunctionIndex = 256,
  /// ERROR_TO_STRING (JSFunction) — accessor error_to_string
  ErrorToString = 257,
  /// EVAL_ERROR_FUNCTION_INDEX (JSFunction) — accessor eval_error_function
  EvalErrorFunctionIndex = 258,
  /// AGGREGATE_ERROR_FUNCTION_INDEX (JSFunction) — accessor aggregate_error_function
  AggregateErrorFunctionIndex = 259,
  /// GLOBAL_EVAL_FUN_INDEX (JSFunction) — accessor global_eval_fun
  GlobalEvalFunIndex = 260,
  /// GLOBAL_PARSE_FLOAT_FUN_INDEX (JSFunction) — accessor global_parse_float_fun
  GlobalParseFloatFunIndex = 261,
  /// GLOBAL_PARSE_INT_FUN_INDEX (JSFunction) — accessor global_parse_int_fun
  GlobalParseIntFunIndex = 262,
  /// GLOBAL_PROXY_FUNCTION_INDEX (JSFunction) — accessor global_proxy_function
  GlobalProxyFunctionIndex = 263,
  /// MAP_DELETE_INDEX (JSFunction) — accessor map_delete
  MapDeleteIndex = 264,
  /// MAP_GET_INDEX (JSFunction) — accessor map_get
  MapGetIndex = 265,
  /// MAP_HAS_INDEX (JSFunction) — accessor map_has
  MapHasIndex = 266,
  /// MAP_SET_INDEX (JSFunction) — accessor map_set
  MapSetIndex = 267,
  /// FUNCTION_HAS_INSTANCE_INDEX (JSFunction) — accessor function_has_instance
  FunctionHasInstanceIndex = 268,
  /// FUNCTION_TO_STRING_INDEX (JSFunction) — accessor function_to_string
  FunctionToStringIndex = 269,
  /// OBJECT_TO_STRING (JSFunction) — accessor object_to_string
  ObjectToString = 270,
  /// OBJECT_VALUE_OF_FUNCTION_INDEX (JSFunction) — accessor object_value_of_function
  ObjectValueOfFunctionIndex = 271,
  /// PROMISE_ALL_INDEX (JSFunction) — accessor promise_all
  PromiseAllIndex = 272,
  /// PROMISE_ALL_SETTLED_INDEX (JSFunction) — accessor promise_all_settled
  PromiseAllSettledIndex = 273,
  /// PROMISE_ANY_INDEX (JSFunction) — accessor promise_any
  PromiseAnyIndex = 274,
  /// PROMISE_FUNCTION_INDEX (JSFunction) — accessor promise_function
  PromiseFunctionIndex = 275,
  /// RANGE_ERROR_FUNCTION_INDEX (JSFunction) — accessor range_error_function
  RangeErrorFunctionIndex = 276,
  /// REFERENCE_ERROR_FUNCTION_INDEX (JSFunction) — accessor reference_error_function
  ReferenceErrorFunctionIndex = 277,
  /// SET_ADD_INDEX (JSFunction) — accessor set_add
  SetAddIndex = 278,
  /// SET_DELETE_INDEX (JSFunction) — accessor set_delete
  SetDeleteIndex = 279,
  /// SET_HAS_INDEX (JSFunction) — accessor set_has
  SetHasIndex = 280,
  /// SHADOW_REALM_IMPORT_VALUE_REJECTED_INDEX (JSFunction) — accessor shadow_realm_import_value_rejected
  ShadowRealmImportValueRejectedIndex = 281,
  /// SUPPRESSED_ERROR_FUNCTION_INDEX (JSFunction) — accessor suppressed_error_function
  SuppressedErrorFunctionIndex = 282,
  /// SYNTAX_ERROR_FUNCTION_INDEX (JSFunction) — accessor syntax_error_function
  SyntaxErrorFunctionIndex = 283,
  /// TYPE_ERROR_FUNCTION_INDEX (JSFunction) — accessor type_error_function
  TypeErrorFunctionIndex = 284,
  /// URI_ERROR_FUNCTION_INDEX (JSFunction) — accessor uri_error_function
  UriErrorFunctionIndex = 285,
  /// WASM_COMPILE_ERROR_FUNCTION_INDEX (JSFunction) — accessor wasm_compile_error_function
  WasmCompileErrorFunctionIndex = 286,
  /// WASM_LINK_ERROR_FUNCTION_INDEX (JSFunction) — accessor wasm_link_error_function
  WasmLinkErrorFunctionIndex = 287,
  /// WASM_SUSPEND_ERROR_FUNCTION_INDEX (JSFunction) — accessor wasm_suspend_error_function
  WasmSuspendErrorFunctionIndex = 288,
  /// WASM_RUNTIME_ERROR_FUNCTION_INDEX (JSFunction) — accessor wasm_runtime_error_function
  WasmRuntimeErrorFunctionIndex = 289,
  /// WEAKMAP_SET_INDEX (JSFunction) — accessor weakmap_set
  WeakmapSetIndex = 290,
  /// WEAKMAP_GET_INDEX (JSFunction) — accessor weakmap_get
  WeakmapGetIndex = 291,
  /// WEAKMAP_DELETE_INDEX (JSFunction) — accessor weakmap_delete
  WeakmapDeleteIndex = 292,
  /// WEAKSET_ADD_INDEX (JSFunction) — accessor weakset_add
  WeaksetAddIndex = 293,
  /// WRAPPED_FUNCTION_MAP_INDEX (Map) — accessor wrapped_function_map
  WrappedFunctionMapIndex = 294,
  /// RETAINED_MAPS (Object) — accessor retained_maps
  RetainedMaps = 295,
  /// SHARED_SPACE_JS_OBJECT_HAS_INSTANCE_INDEX (JSFunction) — accessor shared_space_js_object_has_instance
  SharedSpaceJsObjectHasInstanceIndex = 296,
  /// NEXT_CONTEXT_LINK (弱引用区起点)
  NextContextLink = 297,
  /// NATIVE_CONTEXT_SLOTS (总槽数, 不含 microtask_queue 尾部)
  NativeContextSlots = 298,
}

impl ContextSlot {
  /// native 字段数 (不含 3 个通用槽与尾部 2 项)
  pub const NATIVE_FIELD_COUNT: usize = 294;
  /// 枚举变体总数
  pub const COUNT: usize = 299;

  /// 数值反查 (原版隐式整型转换)
  #[inline]
  #[must_use]
  pub const fn from_repr(v: u16) -> Option<Self> {
    match v {
      0 => Some(Self::ScopeInfoIndex),
      1 => Some(Self::PreviousIndex),
      2 => Some(Self::ExtensionIndex),
      3 => Some(Self::GlobalProxyIndex),
      4 => Some(Self::EmbedderDataIndex),
      5 => Some(Self::ContinuationPreservedEmbedderDataIndex),
      6 => Some(Self::GeneratorNextInternal),
      7 => Some(Self::AsyncModuleEvaluateInternal),
      8 => Some(Self::ReflectApplyIndex),
      9 => Some(Self::ReflectConstructIndex),
      10 => Some(Self::PerformPromiseThenIndex),
      11 => Some(Self::PromiseThenIndex),
      12 => Some(Self::PromiseResolveIndex),
      13 => Some(Self::FunctionPrototypeApplyIndex),
      14 => Some(Self::Uint8ArrayFunIndex),
      15 => Some(Self::Int8ArrayFunIndex),
      16 => Some(Self::Uint16ArrayFunIndex),
      17 => Some(Self::Int16ArrayFunIndex),
      18 => Some(Self::Uint32ArrayFunIndex),
      19 => Some(Self::Int32ArrayFunIndex),
      20 => Some(Self::Biguint64ArrayFunIndex),
      21 => Some(Self::Bigint64ArrayFunIndex),
      22 => Some(Self::Uint8ClampedArrayFunIndex),
      23 => Some(Self::Float32ArrayFunIndex),
      24 => Some(Self::Float64ArrayFunIndex),
      25 => Some(Self::Float16ArrayFunIndex),
      26 => Some(Self::RabGsabUint8ArrayMapIndex),
      27 => Some(Self::RabGsabInt8ArrayMapIndex),
      28 => Some(Self::RabGsabUint16ArrayMapIndex),
      29 => Some(Self::RabGsabInt16ArrayMapIndex),
      30 => Some(Self::RabGsabUint32ArrayMapIndex),
      31 => Some(Self::RabGsabInt32ArrayMapIndex),
      32 => Some(Self::RabGsabBiguint64ArrayMapIndex),
      33 => Some(Self::RabGsabBigint64ArrayMapIndex),
      34 => Some(Self::RabGsabUint8ClampedArrayMapIndex),
      35 => Some(Self::RabGsabFloat32ArrayMapIndex),
      36 => Some(Self::RabGsabFloat64ArrayMapIndex),
      37 => Some(Self::RabGsabFloat16ArrayMapIndex),
      38 => Some(Self::AbstractModuleSourceFunctionIndex),
      39 => Some(Self::AbstractModuleSourcePrototypeIndex),
      40 => Some(Self::AccessorPropertyDescriptorMapIndex),
      41 => Some(Self::AllowCodeGenFromStringsIndex),
      42 => Some(Self::ArrayBufferFunIndex),
      43 => Some(Self::ArrayBufferMapIndex),
      44 => Some(Self::ArrayBufferNoinitFunIndex),
      45 => Some(Self::ArrayFunctionIndex),
      46 => Some(Self::ArrayJoinStackIndex),
      47 => Some(Self::ArrayFromAsyncIndex),
      48 => Some(Self::AsyncFromSyncIteratorMapIndex),
      49 => Some(Self::AsyncFunctionFunctionIndex),
      50 => Some(Self::AsyncFunctionObjectMapIndex),
      51 => Some(Self::AsyncGeneratorFunctionFunctionIndex),
      52 => Some(Self::BigintFunctionIndex),
      53 => Some(Self::BooleanFunctionIndex),
      54 => Some(Self::BoundFunctionWithConstructorMapIndex),
      55 => Some(Self::BoundFunctionWithoutConstructorMapIndex),
      56 => Some(Self::CallAsConstructorDelegateIndex),
      57 => Some(Self::CallAsFunctionDelegateIndex),
      58 => Some(Self::CallsiteFunctionIndex),
      59 => Some(Self::ContextExtensionFunctionIndex),
      60 => Some(Self::DataPropertyDescriptorMapIndex),
      61 => Some(Self::DataViewFunIndex),
      62 => Some(Self::DateFunctionIndex),
      63 => Some(Self::DebugContextIdIndex),
      64 => Some(Self::EmptyFunctionIndex),
      65 => Some(Self::ErrorMessageForCodeGenFromStringsIndex),
      66 => Some(Self::ErrorMessageForWasmCodeGenIndex),
      67 => Some(Self::ErrorsThrownIndex),
      68 => Some(Self::ExtrasBindingObjectIndex),
      69 => Some(Self::FastAliasedArgumentsMapIndex),
      70 => Some(Self::FastTemplateInstantiationsCacheIndex),
      71 => Some(Self::FunctionFunctionIndex),
      72 => Some(Self::FunctionPrototypeIndex),
      73 => Some(Self::GeneratorFunctionFunctionIndex),
      74 => Some(Self::GeneratorObjectPrototypeMapIndex),
      75 => Some(Self::AsyncGeneratorObjectPrototypeMapIndex),
      76 => Some(Self::InitialArrayIteratorMapIndex),
      77 => Some(Self::InitialArrayIteratorPrototypeIndex),
      78 => Some(Self::InitialArrayPrototypeIndex),
      79 => Some(Self::InitialErrorPrototypeIndex),
      80 => Some(Self::InitialGeneratorPrototypeIndex),
      81 => Some(Self::InitialAsyncIteratorPrototypeIndex),
      82 => Some(Self::InitialAsyncGeneratorPrototypeIndex),
      83 => Some(Self::InitialIteratorFunctionIndex),
      84 => Some(Self::InitialIteratorHelperPrototypeIndex),
      85 => Some(Self::InitialIteratorPrototypeIndex),
      86 => Some(Self::InitialDisposableStackPrototypeIndex),
      87 => Some(Self::InitialMapIteratorPrototypeIndex),
      88 => Some(Self::InitialMapPrototypeMapIndex),
      89 => Some(Self::InitialObjectPrototypeIndex),
      90 => Some(Self::InitialSetIteratorPrototypeIndex),
      91 => Some(Self::InitialSetPrototypeIndex),
      92 => Some(Self::InitialSetPrototypeMapIndex),
      93 => Some(Self::InitialStringIteratorMapIndex),
      94 => Some(Self::InitialStringIteratorPrototypeIndex),
      95 => Some(Self::InitialStringPrototypeIndex),
      96 => Some(Self::InitialWeakmapPrototypeMapIndex),
      97 => Some(Self::InitialWeaksetPrototypeMapIndex),
      98 => Some(Self::IntlCollatorFunctionIndex),
      99 => Some(Self::IntlDateTimeFormatFunctionIndex),
      100 => Some(Self::IntlDisplayNamesFunctionIndex),
      101 => Some(Self::IntlDurationFormatFunctionIndex),
      102 => Some(Self::IntlNumberFormatFunctionIndex),
      103 => Some(Self::IntlLocaleFunctionIndex),
      104 => Some(Self::IntlListFormatFunctionIndex),
      105 => Some(Self::IntlPluralRulesFunctionIndex),
      106 => Some(Self::IntlRelativeTimeFormatFunctionIndex),
      107 => Some(Self::IntlSegmenterFunctionIndex),
      108 => Some(Self::IntlSegmentsMapIndex),
      109 => Some(Self::IntlSegmentDataObjectMapIndex),
      110 => Some(Self::IntlSegmentDataObjectWordlikeMapIndex),
      111 => Some(Self::IntlSegmentIteratorMapIndex),
      112 => Some(Self::IteratorFilterHelperMapIndex),
      113 => Some(Self::IteratorMapHelperMapIndex),
      114 => Some(Self::IteratorTakeHelperMapIndex),
      115 => Some(Self::IteratorDropHelperMapIndex),
      116 => Some(Self::IteratorFlatMapHelperMapIndex),
      117 => Some(Self::IteratorConcatHelperMapIndex),
      118 => Some(Self::IteratorZipHelperMapIndex),
      119 => Some(Self::IteratorZipKeyedHelperMapIndex),
      120 => Some(Self::IteratorFunctionIndex),
      121 => Some(Self::ValidIteratorWrapperMapIndex),
      122 => Some(Self::IteratorResultMapIndex),
      123 => Some(Self::JsArrayPackedSmiElementsMapIndex),
      124 => Some(Self::JsArrayHoleySmiElementsMapIndex),
      125 => Some(Self::JsArrayPackedElementsMapIndex),
      126 => Some(Self::JsArrayHoleyElementsMapIndex),
      127 => Some(Self::JsArrayPackedDoubleElementsMapIndex),
      128 => Some(Self::JsArrayHoleyDoubleElementsMapIndex),
      129 => Some(Self::JsArrayTemplateLiteralObjectMap),
      130 => Some(Self::JsDisposableStackFunctionIndex),
      131 => Some(Self::JsAsyncDisposableStackFunctionIndex),
      132 => Some(Self::JsDisposableStackMapIndex),
      133 => Some(Self::JsMapFunIndex),
      134 => Some(Self::JsMapMapIndex),
      135 => Some(Self::JsModuleNamespaceMap),
      136 => Some(Self::JsDeferredModuleNamespaceMap),
      137 => Some(Self::JsRawJsonMap),
      138 => Some(Self::JsSetFunIndex),
      139 => Some(Self::JsSetMapIndex),
      140 => Some(Self::JsWeakMapFunIndex),
      141 => Some(Self::JsWeakSetFunIndex),
      142 => Some(Self::JsWeakRefFunctionIndex),
      143 => Some(Self::JsFinalizationRegistryFunctionIndex),
      144 => Some(Self::JsonObject),
      145 => Some(Self::PromiseWithresolversResultMapIndex),
      146 => Some(Self::MetaMapIndex),
      147 => Some(Self::FunctionContextMapIndex),
      148 => Some(Self::ModuleContextMapIndex),
      149 => Some(Self::EvalContextMapIndex),
      150 => Some(Self::ScriptContextMapIndex),
      151 => Some(Self::AwaitContextMapIndex),
      152 => Some(Self::BlockContextMapIndex),
      153 => Some(Self::CatchContextMapIndex),
      154 => Some(Self::WithContextMapIndex),
      155 => Some(Self::DebugEvaluateContextMapIndex),
      156 => Some(Self::JsRabGsabDataViewMapIndex),
      157 => Some(Self::MapCacheIndex),
      158 => Some(Self::MapKeyIteratorMapIndex),
      159 => Some(Self::MapKeyValueIteratorMapIndex),
      160 => Some(Self::MapValueIteratorMapIndex),
      161 => Some(Self::MathRandomIndexIndex),
      162 => Some(Self::MathRandomStateIndex),
      163 => Some(Self::MathRandomCacheIndex),
      164 => Some(Self::NormalizedMapCacheIndex),
      165 => Some(Self::NumberFunctionIndex),
      166 => Some(Self::ObjectFunctionIndex),
      167 => Some(Self::ObjectFunctionPrototypeIndex),
      168 => Some(Self::ObjectFunctionPrototypeMapIndex),
      169 => Some(Self::PromiseHookInitFunctionIndex),
      170 => Some(Self::PromiseHookBeforeFunctionIndex),
      171 => Some(Self::PromiseHookAfterFunctionIndex),
      172 => Some(Self::PromiseHookResolveFunctionIndex),
      173 => Some(Self::ProxyCallableMapIndex),
      174 => Some(Self::ProxyConstructorMapIndex),
      175 => Some(Self::ProxyFunctionIndex),
      176 => Some(Self::ProxyMapIndex),
      177 => Some(Self::ProxyRevocableResultMapIndex),
      178 => Some(Self::PromisePrototypeIndex),
      179 => Some(Self::RecorderContextId),
      180 => Some(Self::RegexpExecFunctionIndex),
      181 => Some(Self::RegexpFunctionIndex),
      182 => Some(Self::RegexpLastMatchInfoIndex),
      183 => Some(Self::RegexpMatchAllFunctionIndex),
      184 => Some(Self::RegexpMatchFunctionIndex),
      185 => Some(Self::RegexpPrototypeIndex),
      186 => Some(Self::RegexpPrototypeMapIndex),
      187 => Some(Self::RegexpReplaceFunctionIndex),
      188 => Some(Self::RegexpResultMapIndex),
      189 => Some(Self::RegexpResultWithIndicesMapIndex),
      190 => Some(Self::RegexpResultIndicesMapIndex),
      191 => Some(Self::RegexpSearchFunctionIndex),
      192 => Some(Self::RegexpSplitFunctionIndex),
      193 => Some(Self::InitialRegexpStringIteratorPrototypeMapIndex),
      194 => Some(Self::ScriptContextTableIndex),
      195 => Some(Self::ScriptExecutionCallbackIndex),
      196 => Some(Self::TemporalGetEpochNanosecondsCallbackIndex),
      197 => Some(Self::SecurityTokenIndex),
      198 => Some(Self::SerializedObjects),
      199 => Some(Self::SetValueIteratorMapIndex),
      200 => Some(Self::SetKeyValueIteratorMapIndex),
      201 => Some(Self::SharedArrayBufferFunIndex),
      202 => Some(Self::SloppyArgumentsMapIndex),
      203 => Some(Self::SlowAliasedArgumentsMapIndex),
      204 => Some(Self::StrictArgumentsMapIndex),
      205 => Some(Self::SlowObjectWithNullPrototypeMap),
      206 => Some(Self::SlowObjectWithObjectPrototypeMap),
      207 => Some(Self::SlowTemplateInstantiationsCacheIndex),
      208 => Some(Self::AtomicsWaitasyncPromises),
      209 => Some(Self::SetUint8ArrayResultMap),
      210 => Some(Self::WasmDebugMaps),
      211 => Some(Self::InitialArrayPrototypeValidityCellIndex),
      212 => Some(Self::SloppyFunctionMapIndex),
      213 => Some(Self::SloppyFunctionWithNameMapIndex),
      214 => Some(Self::SloppyFunctionWithoutPrototypeMapIndex),
      215 => Some(Self::SloppyFunctionWithReadonlyPrototypeMapIndex),
      216 => Some(Self::StrictFunctionMapIndex),
      217 => Some(Self::StrictFunctionWithNameMapIndex),
      218 => Some(Self::StrictFunctionWithReadonlyPrototypeMapIndex),
      219 => Some(Self::StrictFunctionWithoutPrototypeMapIndex),
      220 => Some(Self::MethodWithNameMapIndex),
      221 => Some(Self::AsyncFunctionMapIndex),
      222 => Some(Self::AsyncFunctionWithNameMapIndex),
      223 => Some(Self::GeneratorFunctionMapIndex),
      224 => Some(Self::GeneratorFunctionWithNameMapIndex),
      225 => Some(Self::AsyncGeneratorFunctionMapIndex),
      226 => Some(Self::AsyncGeneratorFunctionWithNameMapIndex),
      227 => Some(Self::ClassFunctionMapIndex),
      228 => Some(Self::StringFunctionIndex),
      229 => Some(Self::StringFunctionPrototypeMapIndex),
      230 => Some(Self::SymbolFunctionIndex),
      231 => Some(Self::IsWasmJsInstalledIndex),
      232 => Some(Self::IsWasmJspiInstalledIndex),
      233 => Some(Self::WasmWebassemblyObjectIndex),
      234 => Some(Self::WasmExportedFunctionMapIndex),
      235 => Some(Self::WasmTagConstructorIndex),
      236 => Some(Self::WasmExceptionConstructorIndex),
      237 => Some(Self::WasmGlobalConstructorIndex),
      238 => Some(Self::WasmInstanceConstructorIndex),
      239 => Some(Self::WasmJsTagIndex),
      240 => Some(Self::WasmMemoryConstructorIndex),
      241 => Some(Self::WasmModuleConstructorIndex),
      242 => Some(Self::WasmTableConstructorIndex),
      243 => Some(Self::WasmSuspendingConstructorIndex),
      244 => Some(Self::WasmSuspenderConstructorIndex),
      245 => Some(Self::WasmSuspendingMap),
      246 => Some(Self::WasmSuspendingPrototype),
      247 => Some(Self::WasmMemoryMapDescriptorTemplateIndex),
      248 => Some(Self::WasmMemoryMapDescriptorConstructorIndex),
      249 => Some(Self::TemplateWeakmapIndex),
      250 => Some(Self::TypedArrayFunIndex),
      251 => Some(Self::TypedArrayPrototypeIndex),
      252 => Some(Self::ArrayEntriesIteratorIndex),
      253 => Some(Self::ArrayForEachIteratorIndex),
      254 => Some(Self::ArrayKeysIteratorIndex),
      255 => Some(Self::ArrayValuesIteratorIndex),
      256 => Some(Self::ErrorFunctionIndex),
      257 => Some(Self::ErrorToString),
      258 => Some(Self::EvalErrorFunctionIndex),
      259 => Some(Self::AggregateErrorFunctionIndex),
      260 => Some(Self::GlobalEvalFunIndex),
      261 => Some(Self::GlobalParseFloatFunIndex),
      262 => Some(Self::GlobalParseIntFunIndex),
      263 => Some(Self::GlobalProxyFunctionIndex),
      264 => Some(Self::MapDeleteIndex),
      265 => Some(Self::MapGetIndex),
      266 => Some(Self::MapHasIndex),
      267 => Some(Self::MapSetIndex),
      268 => Some(Self::FunctionHasInstanceIndex),
      269 => Some(Self::FunctionToStringIndex),
      270 => Some(Self::ObjectToString),
      271 => Some(Self::ObjectValueOfFunctionIndex),
      272 => Some(Self::PromiseAllIndex),
      273 => Some(Self::PromiseAllSettledIndex),
      274 => Some(Self::PromiseAnyIndex),
      275 => Some(Self::PromiseFunctionIndex),
      276 => Some(Self::RangeErrorFunctionIndex),
      277 => Some(Self::ReferenceErrorFunctionIndex),
      278 => Some(Self::SetAddIndex),
      279 => Some(Self::SetDeleteIndex),
      280 => Some(Self::SetHasIndex),
      281 => Some(Self::ShadowRealmImportValueRejectedIndex),
      282 => Some(Self::SuppressedErrorFunctionIndex),
      283 => Some(Self::SyntaxErrorFunctionIndex),
      284 => Some(Self::TypeErrorFunctionIndex),
      285 => Some(Self::UriErrorFunctionIndex),
      286 => Some(Self::WasmCompileErrorFunctionIndex),
      287 => Some(Self::WasmLinkErrorFunctionIndex),
      288 => Some(Self::WasmSuspendErrorFunctionIndex),
      289 => Some(Self::WasmRuntimeErrorFunctionIndex),
      290 => Some(Self::WeakmapSetIndex),
      291 => Some(Self::WeakmapGetIndex),
      292 => Some(Self::WeakmapDeleteIndex),
      293 => Some(Self::WeaksetAddIndex),
      294 => Some(Self::WrappedFunctionMapIndex),
      295 => Some(Self::RetainedMaps),
      296 => Some(Self::SharedSpaceJsObjectHasInstanceIndex),
      297 => Some(Self::NextContextLink),
      298 => Some(Self::NativeContextSlots),
      _ => None,
    }
  }
}

/// native 字段名表 (原版顺序, 索引 i ↔ ContextSlot 值 3+i)。
pub const NATIVE_CONTEXT_NAMES: [&str; 294] = [
  "GLOBAL_PROXY_INDEX",
  "EMBEDDER_DATA_INDEX",
  "CONTINUATION_PRESERVED_EMBEDDER_DATA_INDEX",
  "GENERATOR_NEXT_INTERNAL",
  "ASYNC_MODULE_EVALUATE_INTERNAL",
  "REFLECT_APPLY_INDEX",
  "REFLECT_CONSTRUCT_INDEX",
  "PERFORM_PROMISE_THEN_INDEX",
  "PROMISE_THEN_INDEX",
  "PROMISE_RESOLVE_INDEX",
  "FUNCTION_PROTOTYPE_APPLY_INDEX",
  "UINT8_ARRAY_FUN_INDEX",
  "INT8_ARRAY_FUN_INDEX",
  "UINT16_ARRAY_FUN_INDEX",
  "INT16_ARRAY_FUN_INDEX",
  "UINT32_ARRAY_FUN_INDEX",
  "INT32_ARRAY_FUN_INDEX",
  "BIGUINT64_ARRAY_FUN_INDEX",
  "BIGINT64_ARRAY_FUN_INDEX",
  "UINT8_CLAMPED_ARRAY_FUN_INDEX",
  "FLOAT32_ARRAY_FUN_INDEX",
  "FLOAT64_ARRAY_FUN_INDEX",
  "FLOAT16_ARRAY_FUN_INDEX",
  "RAB_GSAB_UINT8_ARRAY_MAP_INDEX",
  "RAB_GSAB_INT8_ARRAY_MAP_INDEX",
  "RAB_GSAB_UINT16_ARRAY_MAP_INDEX",
  "RAB_GSAB_INT16_ARRAY_MAP_INDEX",
  "RAB_GSAB_UINT32_ARRAY_MAP_INDEX",
  "RAB_GSAB_INT32_ARRAY_MAP_INDEX",
  "RAB_GSAB_BIGUINT64_ARRAY_MAP_INDEX",
  "RAB_GSAB_BIGINT64_ARRAY_MAP_INDEX",
  "RAB_GSAB_UINT8_CLAMPED_ARRAY_MAP_INDEX",
  "RAB_GSAB_FLOAT32_ARRAY_MAP_INDEX",
  "RAB_GSAB_FLOAT64_ARRAY_MAP_INDEX",
  "RAB_GSAB_FLOAT16_ARRAY_MAP_INDEX",
  "ABSTRACT_MODULE_SOURCE_FUNCTION_INDEX",
  "ABSTRACT_MODULE_SOURCE_PROTOTYPE_INDEX",
  "ACCESSOR_PROPERTY_DESCRIPTOR_MAP_INDEX",
  "ALLOW_CODE_GEN_FROM_STRINGS_INDEX",
  "ARRAY_BUFFER_FUN_INDEX",
  "ARRAY_BUFFER_MAP_INDEX",
  "ARRAY_BUFFER_NOINIT_FUN_INDEX",
  "ARRAY_FUNCTION_INDEX",
  "ARRAY_JOIN_STACK_INDEX",
  "ARRAY_FROM_ASYNC_INDEX",
  "ASYNC_FROM_SYNC_ITERATOR_MAP_INDEX",
  "ASYNC_FUNCTION_FUNCTION_INDEX",
  "ASYNC_FUNCTION_OBJECT_MAP_INDEX",
  "ASYNC_GENERATOR_FUNCTION_FUNCTION_INDEX",
  "BIGINT_FUNCTION_INDEX",
  "BOOLEAN_FUNCTION_INDEX",
  "BOUND_FUNCTION_WITH_CONSTRUCTOR_MAP_INDEX",
  "BOUND_FUNCTION_WITHOUT_CONSTRUCTOR_MAP_INDEX",
  "CALL_AS_CONSTRUCTOR_DELEGATE_INDEX",
  "CALL_AS_FUNCTION_DELEGATE_INDEX",
  "CALLSITE_FUNCTION_INDEX",
  "CONTEXT_EXTENSION_FUNCTION_INDEX",
  "DATA_PROPERTY_DESCRIPTOR_MAP_INDEX",
  "DATA_VIEW_FUN_INDEX",
  "DATE_FUNCTION_INDEX",
  "DEBUG_CONTEXT_ID_INDEX",
  "EMPTY_FUNCTION_INDEX",
  "ERROR_MESSAGE_FOR_CODE_GEN_FROM_STRINGS_INDEX",
  "ERROR_MESSAGE_FOR_WASM_CODE_GEN_INDEX",
  "ERRORS_THROWN_INDEX",
  "EXTRAS_BINDING_OBJECT_INDEX",
  "FAST_ALIASED_ARGUMENTS_MAP_INDEX",
  "FAST_TEMPLATE_INSTANTIATIONS_CACHE_INDEX",
  "FUNCTION_FUNCTION_INDEX",
  "FUNCTION_PROTOTYPE_INDEX",
  "GENERATOR_FUNCTION_FUNCTION_INDEX",
  "GENERATOR_OBJECT_PROTOTYPE_MAP_INDEX",
  "ASYNC_GENERATOR_OBJECT_PROTOTYPE_MAP_INDEX",
  "INITIAL_ARRAY_ITERATOR_MAP_INDEX",
  "INITIAL_ARRAY_ITERATOR_PROTOTYPE_INDEX",
  "INITIAL_ARRAY_PROTOTYPE_INDEX",
  "INITIAL_ERROR_PROTOTYPE_INDEX",
  "INITIAL_GENERATOR_PROTOTYPE_INDEX",
  "INITIAL_ASYNC_ITERATOR_PROTOTYPE_INDEX",
  "INITIAL_ASYNC_GENERATOR_PROTOTYPE_INDEX",
  "INITIAL_ITERATOR_FUNCTION_INDEX",
  "INITIAL_ITERATOR_HELPER_PROTOTYPE_INDEX",
  "INITIAL_ITERATOR_PROTOTYPE_INDEX",
  "INITIAL_DISPOSABLE_STACK_PROTOTYPE_INDEX",
  "INITIAL_MAP_ITERATOR_PROTOTYPE_INDEX",
  "INITIAL_MAP_PROTOTYPE_MAP_INDEX",
  "INITIAL_OBJECT_PROTOTYPE_INDEX",
  "INITIAL_SET_ITERATOR_PROTOTYPE_INDEX",
  "INITIAL_SET_PROTOTYPE_INDEX",
  "INITIAL_SET_PROTOTYPE_MAP_INDEX",
  "INITIAL_STRING_ITERATOR_MAP_INDEX",
  "INITIAL_STRING_ITERATOR_PROTOTYPE_INDEX",
  "INITIAL_STRING_PROTOTYPE_INDEX",
  "INITIAL_WEAKMAP_PROTOTYPE_MAP_INDEX",
  "INITIAL_WEAKSET_PROTOTYPE_MAP_INDEX",
  "INTL_COLLATOR_FUNCTION_INDEX",
  "INTL_DATE_TIME_FORMAT_FUNCTION_INDEX",
  "INTL_DISPLAY_NAMES_FUNCTION_INDEX",
  "INTL_DURATION_FORMAT_FUNCTION_INDEX",
  "INTL_NUMBER_FORMAT_FUNCTION_INDEX",
  "INTL_LOCALE_FUNCTION_INDEX",
  "INTL_LIST_FORMAT_FUNCTION_INDEX",
  "INTL_PLURAL_RULES_FUNCTION_INDEX",
  "INTL_RELATIVE_TIME_FORMAT_FUNCTION_INDEX",
  "INTL_SEGMENTER_FUNCTION_INDEX",
  "INTL_SEGMENTS_MAP_INDEX",
  "INTL_SEGMENT_DATA_OBJECT_MAP_INDEX",
  "INTL_SEGMENT_DATA_OBJECT_WORDLIKE_MAP_INDEX",
  "INTL_SEGMENT_ITERATOR_MAP_INDEX",
  "ITERATOR_FILTER_HELPER_MAP_INDEX",
  "ITERATOR_MAP_HELPER_MAP_INDEX",
  "ITERATOR_TAKE_HELPER_MAP_INDEX",
  "ITERATOR_DROP_HELPER_MAP_INDEX",
  "ITERATOR_FLAT_MAP_HELPER_MAP_INDEX",
  "ITERATOR_CONCAT_HELPER_MAP_INDEX",
  "ITERATOR_ZIP_HELPER_MAP_INDEX",
  "ITERATOR_ZIP_KEYED_HELPER_MAP_INDEX",
  "ITERATOR_FUNCTION_INDEX",
  "VALID_ITERATOR_WRAPPER_MAP_INDEX",
  "ITERATOR_RESULT_MAP_INDEX",
  "JS_ARRAY_PACKED_SMI_ELEMENTS_MAP_INDEX",
  "JS_ARRAY_HOLEY_SMI_ELEMENTS_MAP_INDEX",
  "JS_ARRAY_PACKED_ELEMENTS_MAP_INDEX",
  "JS_ARRAY_HOLEY_ELEMENTS_MAP_INDEX",
  "JS_ARRAY_PACKED_DOUBLE_ELEMENTS_MAP_INDEX",
  "JS_ARRAY_HOLEY_DOUBLE_ELEMENTS_MAP_INDEX",
  "JS_ARRAY_TEMPLATE_LITERAL_OBJECT_MAP",
  "JS_DISPOSABLE_STACK_FUNCTION_INDEX",
  "JS_ASYNC_DISPOSABLE_STACK_FUNCTION_INDEX",
  "JS_DISPOSABLE_STACK_MAP_INDEX",
  "JS_MAP_FUN_INDEX",
  "JS_MAP_MAP_INDEX",
  "JS_MODULE_NAMESPACE_MAP",
  "JS_DEFERRED_MODULE_NAMESPACE_MAP",
  "JS_RAW_JSON_MAP",
  "JS_SET_FUN_INDEX",
  "JS_SET_MAP_INDEX",
  "JS_WEAK_MAP_FUN_INDEX",
  "JS_WEAK_SET_FUN_INDEX",
  "JS_WEAK_REF_FUNCTION_INDEX",
  "JS_FINALIZATION_REGISTRY_FUNCTION_INDEX",
  "JSON_OBJECT",
  "PROMISE_WITHRESOLVERS_RESULT_MAP_INDEX",
  "META_MAP_INDEX",
  "FUNCTION_CONTEXT_MAP_INDEX",
  "MODULE_CONTEXT_MAP_INDEX",
  "EVAL_CONTEXT_MAP_INDEX",
  "SCRIPT_CONTEXT_MAP_INDEX",
  "AWAIT_CONTEXT_MAP_INDEX",
  "BLOCK_CONTEXT_MAP_INDEX",
  "CATCH_CONTEXT_MAP_INDEX",
  "WITH_CONTEXT_MAP_INDEX",
  "DEBUG_EVALUATE_CONTEXT_MAP_INDEX",
  "JS_RAB_GSAB_DATA_VIEW_MAP_INDEX",
  "MAP_CACHE_INDEX",
  "MAP_KEY_ITERATOR_MAP_INDEX",
  "MAP_KEY_VALUE_ITERATOR_MAP_INDEX",
  "MAP_VALUE_ITERATOR_MAP_INDEX",
  "MATH_RANDOM_INDEX_INDEX",
  "MATH_RANDOM_STATE_INDEX",
  "MATH_RANDOM_CACHE_INDEX",
  "NORMALIZED_MAP_CACHE_INDEX",
  "NUMBER_FUNCTION_INDEX",
  "OBJECT_FUNCTION_INDEX",
  "OBJECT_FUNCTION_PROTOTYPE_INDEX",
  "OBJECT_FUNCTION_PROTOTYPE_MAP_INDEX",
  "PROMISE_HOOK_INIT_FUNCTION_INDEX",
  "PROMISE_HOOK_BEFORE_FUNCTION_INDEX",
  "PROMISE_HOOK_AFTER_FUNCTION_INDEX",
  "PROMISE_HOOK_RESOLVE_FUNCTION_INDEX",
  "PROXY_CALLABLE_MAP_INDEX",
  "PROXY_CONSTRUCTOR_MAP_INDEX",
  "PROXY_FUNCTION_INDEX",
  "PROXY_MAP_INDEX",
  "PROXY_REVOCABLE_RESULT_MAP_INDEX",
  "PROMISE_PROTOTYPE_INDEX",
  "RECORDER_CONTEXT_ID",
  "REGEXP_EXEC_FUNCTION_INDEX",
  "REGEXP_FUNCTION_INDEX",
  "REGEXP_LAST_MATCH_INFO_INDEX",
  "REGEXP_MATCH_ALL_FUNCTION_INDEX",
  "REGEXP_MATCH_FUNCTION_INDEX",
  "REGEXP_PROTOTYPE_INDEX",
  "REGEXP_PROTOTYPE_MAP_INDEX",
  "REGEXP_REPLACE_FUNCTION_INDEX",
  "REGEXP_RESULT_MAP_INDEX",
  "REGEXP_RESULT_WITH_INDICES_MAP_INDEX",
  "REGEXP_RESULT_INDICES_MAP_INDEX",
  "REGEXP_SEARCH_FUNCTION_INDEX",
  "REGEXP_SPLIT_FUNCTION_INDEX",
  "INITIAL_REGEXP_STRING_ITERATOR_PROTOTYPE_MAP_INDEX",
  "SCRIPT_CONTEXT_TABLE_INDEX",
  "SCRIPT_EXECUTION_CALLBACK_INDEX",
  "TEMPORAL_GET_EPOCH_NANOSECONDS_CALLBACK_INDEX",
  "SECURITY_TOKEN_INDEX",
  "SERIALIZED_OBJECTS",
  "SET_VALUE_ITERATOR_MAP_INDEX",
  "SET_KEY_VALUE_ITERATOR_MAP_INDEX",
  "SHARED_ARRAY_BUFFER_FUN_INDEX",
  "SLOPPY_ARGUMENTS_MAP_INDEX",
  "SLOW_ALIASED_ARGUMENTS_MAP_INDEX",
  "STRICT_ARGUMENTS_MAP_INDEX",
  "SLOW_OBJECT_WITH_NULL_PROTOTYPE_MAP",
  "SLOW_OBJECT_WITH_OBJECT_PROTOTYPE_MAP",
  "SLOW_TEMPLATE_INSTANTIATIONS_CACHE_INDEX",
  "ATOMICS_WAITASYNC_PROMISES",
  "SET_UINT8_ARRAY_RESULT_MAP",
  "WASM_DEBUG_MAPS",
  "INITIAL_ARRAY_PROTOTYPE_VALIDITY_CELL_INDEX",
  "SLOPPY_FUNCTION_MAP_INDEX",
  "SLOPPY_FUNCTION_WITH_NAME_MAP_INDEX",
  "SLOPPY_FUNCTION_WITHOUT_PROTOTYPE_MAP_INDEX",
  "SLOPPY_FUNCTION_WITH_READONLY_PROTOTYPE_MAP_INDEX",
  "STRICT_FUNCTION_MAP_INDEX",
  "STRICT_FUNCTION_WITH_NAME_MAP_INDEX",
  "STRICT_FUNCTION_WITH_READONLY_PROTOTYPE_MAP_INDEX",
  "STRICT_FUNCTION_WITHOUT_PROTOTYPE_MAP_INDEX",
  "METHOD_WITH_NAME_MAP_INDEX",
  "ASYNC_FUNCTION_MAP_INDEX",
  "ASYNC_FUNCTION_WITH_NAME_MAP_INDEX",
  "GENERATOR_FUNCTION_MAP_INDEX",
  "GENERATOR_FUNCTION_WITH_NAME_MAP_INDEX",
  "ASYNC_GENERATOR_FUNCTION_MAP_INDEX",
  "ASYNC_GENERATOR_FUNCTION_WITH_NAME_MAP_INDEX",
  "CLASS_FUNCTION_MAP_INDEX",
  "STRING_FUNCTION_INDEX",
  "STRING_FUNCTION_PROTOTYPE_MAP_INDEX",
  "SYMBOL_FUNCTION_INDEX",
  "IS_WASM_JS_INSTALLED_INDEX",
  "IS_WASM_JSPI_INSTALLED_INDEX",
  "WASM_WEBASSEMBLY_OBJECT_INDEX",
  "WASM_EXPORTED_FUNCTION_MAP_INDEX",
  "WASM_TAG_CONSTRUCTOR_INDEX",
  "WASM_EXCEPTION_CONSTRUCTOR_INDEX",
  "WASM_GLOBAL_CONSTRUCTOR_INDEX",
  "WASM_INSTANCE_CONSTRUCTOR_INDEX",
  "WASM_JS_TAG_INDEX",
  "WASM_MEMORY_CONSTRUCTOR_INDEX",
  "WASM_MODULE_CONSTRUCTOR_INDEX",
  "WASM_TABLE_CONSTRUCTOR_INDEX",
  "WASM_SUSPENDING_CONSTRUCTOR_INDEX",
  "WASM_SUSPENDER_CONSTRUCTOR_INDEX",
  "WASM_SUSPENDING_MAP",
  "WASM_SUSPENDING_PROTOTYPE",
  "WASM_MEMORY_MAP_DESCRIPTOR_TEMPLATE_INDEX",
  "WASM_MEMORY_MAP_DESCRIPTOR_CONSTRUCTOR_INDEX",
  "TEMPLATE_WEAKMAP_INDEX",
  "TYPED_ARRAY_FUN_INDEX",
  "TYPED_ARRAY_PROTOTYPE_INDEX",
  "ARRAY_ENTRIES_ITERATOR_INDEX",
  "ARRAY_FOR_EACH_ITERATOR_INDEX",
  "ARRAY_KEYS_ITERATOR_INDEX",
  "ARRAY_VALUES_ITERATOR_INDEX",
  "ERROR_FUNCTION_INDEX",
  "ERROR_TO_STRING",
  "EVAL_ERROR_FUNCTION_INDEX",
  "AGGREGATE_ERROR_FUNCTION_INDEX",
  "GLOBAL_EVAL_FUN_INDEX",
  "GLOBAL_PARSE_FLOAT_FUN_INDEX",
  "GLOBAL_PARSE_INT_FUN_INDEX",
  "GLOBAL_PROXY_FUNCTION_INDEX",
  "MAP_DELETE_INDEX",
  "MAP_GET_INDEX",
  "MAP_HAS_INDEX",
  "MAP_SET_INDEX",
  "FUNCTION_HAS_INSTANCE_INDEX",
  "FUNCTION_TO_STRING_INDEX",
  "OBJECT_TO_STRING",
  "OBJECT_VALUE_OF_FUNCTION_INDEX",
  "PROMISE_ALL_INDEX",
  "PROMISE_ALL_SETTLED_INDEX",
  "PROMISE_ANY_INDEX",
  "PROMISE_FUNCTION_INDEX",
  "RANGE_ERROR_FUNCTION_INDEX",
  "REFERENCE_ERROR_FUNCTION_INDEX",
  "SET_ADD_INDEX",
  "SET_DELETE_INDEX",
  "SET_HAS_INDEX",
  "SHADOW_REALM_IMPORT_VALUE_REJECTED_INDEX",
  "SUPPRESSED_ERROR_FUNCTION_INDEX",
  "SYNTAX_ERROR_FUNCTION_INDEX",
  "TYPE_ERROR_FUNCTION_INDEX",
  "URI_ERROR_FUNCTION_INDEX",
  "WASM_COMPILE_ERROR_FUNCTION_INDEX",
  "WASM_LINK_ERROR_FUNCTION_INDEX",
  "WASM_SUSPEND_ERROR_FUNCTION_INDEX",
  "WASM_RUNTIME_ERROR_FUNCTION_INDEX",
  "WEAKMAP_SET_INDEX",
  "WEAKMAP_GET_INDEX",
  "WEAKMAP_DELETE_INDEX",
  "WEAKSET_ADD_INDEX",
  "WRAPPED_FUNCTION_MAP_INDEX",
  "RETAINED_MAPS",
  "SHARED_SPACE_JS_OBJECT_HAS_INSTANCE_INDEX",
];

/// 编译期字符串相等 (str 的 const PartialEq 尚未稳定, 手写逐字节比较)
#[inline]
#[must_use]
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

/// native 槽位名反查 (值域 3..297)。
#[inline]
#[must_use]
pub fn native_field_name(index: u16) -> Option<&'static str> {
  NATIVE_CONTEXT_NAMES
    .get(usize::from(index).wrapping_sub(3))
    .copied()
}

// —— 显式别名与区间常量 (原版 enum 内赋值项) ——
/// 最少上下文槽数 (= EXTENSION_INDEX)
pub const MIN_CONTEXT_SLOTS: u16 = 2;
/// 含扩展槽最少槽数
pub const MIN_CONTEXT_EXTENDED_SLOTS: u16 = 3;
/// catch 上下文抛出值槽
pub const THROWN_OBJECT_INDEX: u16 = 2;
/// debug evaluate 上下文包装槽
pub const WRAPPED_CONTEXT_INDEX: u16 = 3;
/// 弱引用区首槽 (= NEXT_CONTEXT_LINK)
pub const FIRST_WEAK_SLOT: u16 = 297;
/// JSArray 六种 ElementsKind 地图区首槽
pub const FIRST_JS_ARRAY_MAP_SLOT: u16 = 123;
/// Builtin::kFastNewClosure 函数地图区首槽
pub const FIRST_FUNCTION_MAP_INDEX: u16 = 212;
/// 函数地图区末槽
pub const LAST_FUNCTION_MAP_INDEX: u16 = 227;
/// TypedArray 构造器区首槽 (顺序敏感)
pub const FIRST_FIXED_TYPED_ARRAY_FUN_INDEX: u16 = 14;
/// RAB/GSAB TypedArray 地图区首槽
pub const FIRST_RAB_GSAB_TYPED_ARRAY_MAP_INDEX: u16 = 26;

/// 变量查找标志 (原版 enum ContextLookupFlags, 位标志)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ContextLookupFlags(pub u8);

impl ContextLookupFlags {
  /// DONT_FOLLOW_CHAINS: 不追链
  pub const DONT_FOLLOW_CHAINS: Self = Self(0);
  /// FOLLOW_CONTEXT_CHAIN: 追上下文链
  pub const FOLLOW_CONTEXT_CHAIN: Self = Self(1);
  /// FOLLOW_PROTOTYPE_CHAIN: 追原型链
  pub const FOLLOW_PROTOTYPE_CHAIN: Self = Self(2);
  /// FOLLOW_CHAINS: 双链皆追
  pub const FOLLOW_CHAINS: Self = Self(3);

  /// 含判定
  #[inline]
  #[must_use]
  pub const fn contains(self, other: Self) -> bool {
    self.0 & other.0 == other.0
  }

  /// 合并
  #[inline]
  #[must_use]
  pub const fn union(self, other: Self) -> Self {
    Self(self.0 | other.0)
  }
}

// —— Context 布局 (原版 kElementsOffset..kExtendedHeaderSize 公式链) ——
/// elements 首槽偏移 (= map + length 头, 原版 OFFSET_OF_DATA_START(Context))
pub const K_ELEMENTS_OFFSET: usize = 2 * K_TAGGED_SIZE;
/// kHeaderSize (同 kElementsOffset)
pub const K_HEADER_SIZE: usize = K_ELEMENTS_OFFSET;
/// scope_info 槽偏移
pub const K_SCOPE_INFO_OFFSET: usize = K_ELEMENTS_OFFSET;
/// previous 槽偏移
pub const K_PREVIOUS_OFFSET: usize = K_SCOPE_INFO_OFFSET + K_TAGGED_SIZE;
/// 过渡头尺寸 (MIN_CONTEXT_SLOTS 尚未移除前的头大小)
pub const K_TODO_HEADER_SIZE: usize = K_PREVIOUS_OFFSET + K_TAGGED_SIZE;
/// extension 槽偏移 (存在时为头后首槽)
pub const K_EXTENSION_OFFSET: usize = K_TODO_HEADER_SIZE;
/// extension 槽大小
pub const K_EXTENSION_SIZE: usize =
  (MIN_CONTEXT_EXTENDED_SLOTS as usize - MIN_CONTEXT_SLOTS as usize) * K_TAGGED_SIZE;
/// 扩展头尺寸
pub const K_EXTENDED_HEADER_SIZE: usize = K_TODO_HEADER_SIZE + K_EXTENSION_SIZE;

/// 对象总大小 (原版 Context::SizeFor)
#[inline]
#[must_use]
pub const fn size_for(length: usize) -> usize {
  K_ELEMENTS_OFFSET + length * K_TAGGED_SIZE
}

/// 槽 i 相对对象起址偏移 (原版 Context::OffsetOfElementAt)
#[inline]
#[must_use]
pub const fn offset_of_element_at(index: usize) -> usize {
  size_for(index)
}

/// 槽 i 相对 tagged 指针偏移 (原版 Context::SlotOffset)
#[inline]
#[must_use]
pub const fn slot_offset(index: usize) -> usize {
  offset_of_element_at(index) - K_HEAP_OBJECT_TAG
}

// —— ScriptContextTable 布局 (原版显式公式) ——
/// capacity 偏移 (= sizeof(HeapObject))
pub const K_SCT_CAPACITY_OFFSET: usize = K_TAGGED_SIZE;
/// length 偏移 (capacity 为 uint32)
pub const K_SCT_LENGTH_OFFSET: usize = K_SCT_CAPACITY_OFFSET + 4;
/// 头大小 (names_to_context_index 之后, objects 变长区起点)
pub const K_SCT_HEADER_SIZE: usize = K_SCT_LENGTH_OFFSET
  + 4
  + if TAGGED_SIZE_8_BYTES {
    K_TAGGED_SIZE
  } else {
    4
  };

// —— ContextCell 布局数值偏移 (字段表归 contexts.rs, 此处独占偏移公式; 成员序 tagged_value/dependent_code/state/[padding]/double_value) ——
/// state 偏移 (map + 2 个 tagged 槽之后)
pub const K_CC_STATE_OFFSET: usize = 3 * K_TAGGED_SIZE;
/// double_value 偏移 (仅 TAGGED_SIZE_8_BYTES 有 optional_padding)
pub const K_CC_DOUBLE_VALUE_OFFSET: usize = if TAGGED_SIZE_8_BYTES {
  4 * K_TAGGED_SIZE
} else {
  3 * K_TAGGED_SIZE + 4
};
/// ContextCell 总大小
pub const K_CC_SIZE: usize = K_CC_DOUBLE_VALUE_OFFSET + 8;

// —— contexts.tq 布局类登记 (ContextCell 字段表与 Context 子类清单归 contexts.rs, 此处登记变长尾数组字段序) ——
/// extern class Context 字段序 (elements 为变长尾数组)
pub const CONTEXT_TQ_FIELDS: &[&str] = &["length", "elements"];
/// extern class ScriptContextTable 字段序 (objects 为变长尾数组)
pub const SCT_TQ_FIELDS: &[&str] = &["capacity", "length", "names_to_context_index", "objects"];

// —— 提取不变式断言 ——
const _: () = {
  assert!(ContextSlot::NATIVE_FIELD_COUNT == 294);
  assert!(ContextSlot::COUNT == 299);
  assert!(ContextSlot::ExtensionIndex as u16 == 2);
  assert!(ContextSlot::NextContextLink as u16 == 297);
  assert!(ContextSlot::NativeContextSlots as u16 == 298);
  assert!(FIRST_WEAK_SLOT == ContextSlot::NextContextLink as u16);
  assert!(FIRST_JS_ARRAY_MAP_SLOT == ContextSlot::JsArrayPackedSmiElementsMapIndex as u16);
  assert!(FIRST_FUNCTION_MAP_INDEX == 212);
  assert!(LAST_FUNCTION_MAP_INDEX > FIRST_FUNCTION_MAP_INDEX);
  assert!(NATIVE_CONTEXT_NAMES.len() == 294);
  assert!(str_eq(NATIVE_CONTEXT_NAMES[0], "GLOBAL_PROXY_INDEX"));
  assert!(str_eq(
    NATIVE_CONTEXT_NAMES[293],
    "SHARED_SPACE_JS_OBJECT_HAS_INSTANCE_INDEX"
  ));
  assert!(ContextLookupFlags::FOLLOW_CHAINS.contains(ContextLookupFlags::FOLLOW_CONTEXT_CHAIN));
  assert!(MIN_CONTEXT_SLOTS == 2 && MIN_CONTEXT_EXTENDED_SLOTS == 3);
  assert!(size_for(298) == K_ELEMENTS_OFFSET + 298 * K_TAGGED_SIZE);
  if TAGGED_SIZE_8_BYTES {
    assert!(K_EXTENDED_HEADER_SIZE == 40);
    assert!(K_SCT_HEADER_SIZE == 24);
    assert!(K_CC_SIZE == 40);
  } else {
    assert!(K_EXTENDED_HEADER_SIZE == 20);
    assert!(K_SCT_HEADER_SIZE == 16);
    assert!(K_CC_SIZE == 24);
  }
};
