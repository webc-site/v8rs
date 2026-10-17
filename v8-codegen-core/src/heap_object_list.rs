// 由 tool/src/codegen/heap-object-list.h.js 从 v8/src/codegen/heap-object-list.h 生成; 修订就地改, 复跑覆盖
// 堆不可移动对象清单 (X-macro 机械展开)

// 三元组语义: (CamelName, underscore_name, class_name)。
// BUILTINS_WITH_SFI_OBJECT_LIST 由 BUILTINS_WITH_SFI_LIST_GENERATOR(torque 生成)驱动,
// 本地无法展开 — 登记跳过, MUTABLE 表不含该挂载点展开。

/// 可变不可移动对象清单 (protector 族)
pub const HEAP_MUTABLE_IMMOVABLE_OBJECT_LIST: &[(&str, &str, &str)] = &[
  (
    "ArrayIteratorProtector",
    "array_iterator_protector",
    "ArrayIteratorProtector",
  ),
  (
    "ArraySpeciesProtector",
    "array_species_protector",
    "ArraySpeciesProtector",
  ),
  (
    "IsConcatSpreadableProtector",
    "is_concat_spreadable_protector",
    "IsConcatSpreadableProtector",
  ),
  (
    "MapIteratorProtector",
    "map_iterator_protector",
    "MapIteratorProtector",
  ),
  (
    "NoElementsProtector",
    "no_elements_protector",
    "NoElementsProtector",
  ),
  ("MegaDOMProtector", "mega_dom_protector", "MegaDOMProtector"),
  (
    "NumberStringNotRegexpLikeProtector",
    "number_string_not_regexp_like_protector",
    "NumberStringNotRegexpLikeProtector",
  ),
  (
    "PromiseResolveProtector",
    "promise_resolve_protector",
    "PromiseResolveProtector",
  ),
  (
    "PromiseSpeciesProtector",
    "promise_species_protector",
    "PromiseSpeciesProtector",
  ),
  (
    "PromiseThenProtector",
    "promise_then_protector",
    "PromiseThenProtector",
  ),
  (
    "RegExpSpeciesProtector",
    "regexp_species_protector",
    "RegExpSpeciesProtector",
  ),
  (
    "SetIteratorProtector",
    "set_iterator_protector",
    "SetIteratorProtector",
  ),
  (
    "StringIteratorProtector",
    "string_iterator_protector",
    "StringIteratorProtector",
  ),
  (
    "StringWrapperToPrimitiveProtector",
    "string_wrapper_to_primitive_protector",
    "StringWrapperToPrimitiveProtector",
  ),
  (
    "TypedArraySpeciesProtector",
    "typed_array_species_protector",
    "TypedArraySpeciesProtector",
  ),
];

/// 不可变不可移动对象清单 (maps/strings/哨兵族)
pub const HEAP_IMMUTABLE_IMMOVABLE_OBJECT_LIST: &[(&str, &str, &str)] = &[
  (
    "AllocationSiteWithoutWeakNextMap",
    "allocation_site_without_weaknext_map",
    "AllocationSiteWithoutWeakNextMap",
  ),
  (
    "AllocationSiteWithWeakNextMap",
    "allocation_site_map",
    "AllocationSiteMap",
  ),
  (
    "arguments_to_string",
    "arguments_to_string",
    "ArgumentsToString",
  ),
  ("ArrayListMap", "array_list_map", "ArrayListMap"),
  ("Array_string", "Array_string", "ArrayString"),
  ("array_to_string", "array_to_string", "ArrayToString"),
  ("BooleanMap", "boolean_map", "BooleanMap"),
  ("boolean_to_string", "boolean_to_string", "BooleanToString"),
  (
    "class_fields_symbol",
    "class_fields_symbol",
    "ClassFieldsSymbol",
  ),
  (
    "ConsOneByteStringMap",
    "cons_one_byte_string_map",
    "ConsOneByteStringMap",
  ),
  (
    "ConsTwoByteStringMap",
    "cons_two_byte_string_map",
    "ConsTwoByteStringMap",
  ),
  (
    "constructor_string",
    "constructor_string",
    "ConstructorString",
  ),
  ("date_to_string", "date_to_string", "DateToString"),
  ("default_string", "default_string", "DefaultString"),
  ("EmptyArrayList", "empty_array_list", "EmptyArrayList"),
  ("EmptyByteArray", "empty_byte_array", "EmptyByteArray"),
  ("EmptyFixedArray", "empty_fixed_array", "EmptyFixedArray"),
  (
    "EmptyOrderedHashSet",
    "empty_ordered_hash_set",
    "EmptyOrderedHashSet",
  ),
  ("EmptyScopeInfo", "empty_scope_info", "EmptyScopeInfo"),
  (
    "EmptyPropertyDictionary",
    "empty_property_dictionary",
    "EmptyPropertyDictionary",
  ),
  (
    "EmptyOrderedPropertyDictionary",
    "empty_ordered_property_dictionary",
    "EmptyOrderedPropertyDictionary",
  ),
  (
    "EmptySwissPropertyDictionary",
    "empty_swiss_property_dictionary",
    "EmptySwissPropertyDictionary",
  ),
  (
    "EmptySlowElementDictionary",
    "empty_slow_element_dictionary",
    "EmptySlowElementDictionary",
  ),
  (
    "EmptyWeakArrayList",
    "empty_weak_array_list",
    "EmptyWeakArrayList",
  ),
  ("empty_string", "empty_string", "EmptyString"),
  ("error_to_string", "error_to_string", "ErrorToString"),
  ("error_string", "error_string", "ErrorString"),
  ("errors_string", "errors_string", "ErrorsString"),
  ("FalseValue", "false_value", "False"),
  ("FixedArrayMap", "fixed_array_map", "FixedArrayMap"),
  (
    "FixedCOWArrayMap",
    "fixed_cow_array_map",
    "FixedCOWArrayMap",
  ),
  ("Function_string", "function_string", "FunctionString"),
  (
    "function_to_string",
    "function_to_string",
    "FunctionToString",
  ),
  ("get_string", "get_string", "GetString"),
  (
    "has_instance_symbol",
    "has_instance_symbol",
    "HasInstanceSymbol",
  ),
  ("has_string", "has_string", "HasString"),
  ("Infinity_string", "Infinity_string", "InfinityString"),
  (
    "is_concat_spreadable_symbol",
    "is_concat_spreadable_symbol",
    "IsConcatSpreadableSymbol",
  ),
  ("Iterator_string", "Iterator_string", "IteratorString"),
  ("iterator_symbol", "iterator_symbol", "IteratorSymbol"),
  ("keys_string", "keys_string", "KeysString"),
  (
    "async_iterator_symbol",
    "async_iterator_symbol",
    "AsyncIteratorSymbol",
  ),
  ("length_string", "length_string", "LengthString"),
  ("LoadHandler5Map", "load_handler5_map", "LoadHandler5Map"),
  (
    "ManyClosuresCellMap",
    "many_closures_cell_map",
    "ManyClosuresCellMap",
  ),
  ("match_symbol", "match_symbol", "MatchSymbol"),
  (
    "megamorphic_symbol",
    "megamorphic_symbol",
    "MegamorphicSymbol",
  ),
  ("mega_dom_symbol", "mega_dom_symbol", "MegaDOMSymbol"),
  ("message_string", "message_string", "MessageString"),
  (
    "minus_Infinity_string",
    "minus_Infinity_string",
    "MinusInfinityString",
  ),
  ("MinusZeroValue", "minus_zero_value", "MinusZero"),
  ("name_string", "name_string", "NameString"),
  ("NanValue", "nan_value", "Nan"),
  ("NaN_string", "NaN_string", "NaNString"),
  ("next_string", "next_string", "NextString"),
  (
    "NoClosuresCellMap",
    "no_closures_cell_map",
    "NoClosuresCellMap",
  ),
  ("null_to_string", "null_to_string", "NullToString"),
  ("NullValue", "null_value", "Null"),
  ("number_string", "number_string", "NumberString"),
  ("number_to_string", "number_to_string", "NumberToString"),
  ("Object_string", "Object_string", "ObjectString"),
  ("object_string", "object_string", "objectString"),
  ("object_to_string", "object_to_string", "ObjectToString"),
  (
    "SeqOneByteStringMap",
    "seq_one_byte_string_map",
    "SeqOneByteStringMap",
  ),
  (
    "OneClosureCellMap",
    "one_closure_cell_map",
    "OneClosureCellMap",
  ),
  (
    "OnePointerFillerMap",
    "one_pointer_filler_map",
    "OnePointerFillerMap",
  ),
  (
    "PromiseCapabilityMap",
    "promise_capability_map",
    "PromiseCapabilityMap",
  ),
  (
    "promise_forwarding_handler_symbol",
    "promise_forwarding_handler_symbol",
    "PromiseForwardingHandlerSymbol",
  ),
  (
    "PromiseFulfillReactionJobTaskMap",
    "promise_fulfill_reaction_job_task_map",
    "PromiseFulfillReactionJobTaskMap",
  ),
  (
    "promise_handled_by_symbol",
    "promise_handled_by_symbol",
    "PromiseHandledBySymbol",
  ),
  (
    "PromiseReactionMap",
    "promise_reaction_map",
    "PromiseReactionMap",
  ),
  (
    "PromiseRejectReactionJobTaskMap",
    "promise_reject_reaction_job_task_map",
    "PromiseRejectReactionJobTaskMap",
  ),
  (
    "PromiseResolveThenableJobTaskMap",
    "promise_resolve_thenable_job_task_map",
    "PromiseResolveThenableJobTaskMap",
  ),
  ("prototype_string", "prototype_string", "PrototypeString"),
  ("replace_symbol", "replace_symbol", "ReplaceSymbol"),
  ("regexp_to_string", "regexp_to_string", "RegexpToString"),
  ("resolve_string", "resolve_string", "ResolveString"),
  ("return_string", "return_string", "ReturnString"),
  ("search_symbol", "search_symbol", "SearchSymbol"),
  ("size_string", "size_string", "SizeString"),
  ("species_symbol", "species_symbol", "SpeciesSymbol"),
  ("StaleRegister", "stale_register", "StaleRegister"),
  ("StoreHandler0Map", "store_handler0_map", "StoreHandler0Map"),
  ("string_string", "string_string", "StringString"),
  ("string_to_string", "string_to_string", "StringToString"),
  ("suppressed_string", "suppressed_string", "SuppressedString"),
  (
    "SeqTwoByteStringMap",
    "seq_two_byte_string_map",
    "SeqTwoByteStringMap",
  ),
  ("TheHoleValue", "the_hole_value", "TheHole"),
  (
    "PropertyCellHoleValue",
    "property_cell_hole_value",
    "PropertyCellHole",
  ),
  (
    "HashTableHoleValue",
    "hash_table_hole_value",
    "HashTableHole",
  ),
  ("PromiseHoleValue", "promise_hole_value", "PromiseHole"),
  ("TdzHoleValue", "tdz_hole_value", "TdzHole"),
  ("then_string", "then_string", "ThenString"),
  ("toJSON_string", "toJSON_string", "ToJSONString"),
  ("toString_string", "toString_string", "ToStringString"),
  (
    "to_primitive_symbol",
    "to_primitive_symbol",
    "ToPrimitiveSymbol",
  ),
  (
    "to_string_tag_symbol",
    "to_string_tag_symbol",
    "ToStringTagSymbol",
  ),
  ("TrueValue", "true_value", "True"),
  (
    "undefined_to_string",
    "undefined_to_string",
    "UndefinedToString",
  ),
  ("UndefinedValue", "undefined_value", "Undefined"),
  ("UninitializedValue", "uninitialized_value", "Uninitialized"),
  (
    "uninitialized_symbol",
    "uninitialized_symbol",
    "UninitializedSymbol",
  ),
  (
    "UndefinedContextCell",
    "undefined_context_cell",
    "UndefinedContextCell",
  ),
  ("valueOf_string", "valueOf_string", "ValueOfString"),
  (
    "wasm_cross_instance_call_symbol",
    "wasm_cross_instance_call_symbol",
    "WasmCrossInstanceCallSymbol",
  ),
  ("zero_string", "zero_string", "ZeroString"),
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(HEAP_MUTABLE_IMMOVABLE_OBJECT_LIST.len() == 15);
  assert!(HEAP_IMMUTABLE_IMMOVABLE_OBJECT_LIST.len() == 106);
  assert!(HEAP_MUTABLE_IMMOVABLE_OBJECT_LIST[0].0 == "ArrayIteratorProtector");
  assert!(HEAP_IMMUTABLE_IMMOVABLE_OBJECT_LIST[0].0 == "AllocationSiteWithoutWeakNextMap");
  // 三元组三列非空
  for MUT in HEAP_MUTABLE_IMMOVABLE_OBJECT_LIST {
    assert!(!MUT.0.is_empty() && !MUT.1.is_empty());
  }
  for IM in HEAP_IMMUTABLE_IMMOVABLE_OBJECT_LIST {
    assert!(!IM.0.is_empty() && !IM.1.is_empty());
  }
};
