// 迁移自 instance-type-inl.h 手写检查器的语义规约 + casting-unittest.cc 中
// 类型判定可承载部分; isolate/static-roots 依赖部分跳过 (登记于脚本头)。
use aok::{OK, Void};
use v8_objects_core::{instance_type::InstanceType as T, instance_type_checker as itc};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

/// 规约 (instance-type-inl.h:359-372 + 位域):
/// 6 个内部化字符串变体命中, 未内部化与共享串不命中。
#[test]
fn internalized_string_checkers() -> Void {
  for t in [
    T::InternalizedTwoByteString,
    T::InternalizedOneByteString,
    T::ExternalInternalizedTwoByteString,
    T::ExternalInternalizedOneByteString,
    T::UncachedExternalInternalizedTwoByteString,
    T::UncachedExternalInternalizedOneByteString,
  ] {
    assert!(t.is_internalized(), "{t:?}");
    assert!(itc::is_internalized_string(t), "{t:?}");
    assert!(itc::is_string(t));
    assert!(itc::is_name(t));
  }
  // 未内部化 / 共享 / 非字符串
  for t in [
    T::SeqTwoByteString,
    T::SeqOneByteString,
    T::ConsOneByteString,
    T::SharedSeqOneByteString,
  ] {
    assert!(!itc::is_internalized_string(t), "{t:?}");
    assert!(itc::is_string(t));
  }
  assert!(!itc::is_internalized_string(T::Symbol));
  assert!(!itc::is_internalized_string(T::HeapNumber));
  OK
}

/// 规约 (instance-type-inl.h:374-513): 表示 (rep) 判定矩阵。
#[test]
fn string_representation_checkers() -> Void {
  assert!(itc::is_seq_string(T::SeqTwoByteString));
  assert!(itc::is_seq_string(T::InternalizedOneByteString));
  assert!(!itc::is_seq_string(T::ConsOneByteString));
  assert!(itc::is_cons_string(T::ConsTwoByteString));
  assert!(itc::is_cons_string(T::ConsOneByteString));
  assert!(itc::is_sliced_string(T::SlicedTwoByteString));
  assert!(itc::is_thin_string(T::ThinOneByteString));
  assert!(itc::is_external_string(T::ExternalTwoByteString));
  assert!(itc::is_external_string(
    T::UncachedExternalInternalizedTwoByteString
  ));
  assert!(!itc::is_external_string(T::SeqTwoByteString));
  assert!(itc::is_uncached_external_string(
    T::UncachedExternalTwoByteString
  ));
  assert!(itc::is_uncached_external_string(
    T::SharedUncachedExternalOneByteString
  ));
  assert!(!itc::is_uncached_external_string(T::ExternalTwoByteString));
  assert!(itc::is_shared_string(T::SharedSeqOneByteString));
  assert!(!itc::is_shared_string(T::SeqOneByteString));
  // 间接 (bit0=1): cons/sliced/thin; 直接: seq/external
  assert!(itc::is_indirect_string(T::ConsTwoByteString));
  assert!(itc::is_indirect_string(T::SlicedOneByteString));
  assert!(itc::is_indirect_string(T::ThinTwoByteString));
  assert!(itc::is_direct_string(T::SeqOneByteString));
  assert!(itc::is_direct_string(T::ExternalTwoByteString));
  assert!(!itc::is_direct_string(T::ConsOneByteString));
  // 非字符串全部不命中表示判定
  for t in [T::Symbol, T::HeapNumber, T::JSObject] {
    assert!(!itc::is_seq_string(t));
    assert!(!itc::is_cons_string(t));
    assert!(!itc::is_thin_string(t));
  }
  OK
}

/// 规约 (instance-type-inl.h:388-414, 515-544): 编码判定。
#[test]
fn string_encoding_checkers() -> Void {
  assert!(itc::is_one_byte_string(T::SeqOneByteString));
  assert!(itc::is_one_byte_string(T::ConsOneByteString));
  assert!(!itc::is_one_byte_string(T::SeqTwoByteString));
  assert!(itc::is_two_byte_string(T::SeqTwoByteString));
  assert!(itc::is_two_byte_string(T::ThinTwoByteString));
  assert!(!itc::is_two_byte_string(T::ThinOneByteString));
  // 组合判定
  assert!(itc::is_seq_one_byte_string(T::SeqOneByteString));
  assert!(!itc::is_seq_one_byte_string(T::SeqTwoByteString));
  assert!(itc::is_seq_two_byte_string(T::SeqTwoByteString));
  assert!(itc::is_external_one_byte_string(T::ExternalOneByteString));
  assert!(itc::is_external_two_byte_string(T::ExternalTwoByteString));
  assert!(!itc::is_external_one_byte_string(T::ExternalTwoByteString));
  OK
}

/// 规约: SINGLE/RANGE 桶抽查 (锚定代表性类型)。
#[test]
fn bucket_checkers() -> Void {
  // 精确比较
  assert!(itc::is_symbol(T::Symbol));
  assert!(!itc::is_symbol(T::HeapNumber));
  assert!(itc::is_heap_number(T::HeapNumber));
  assert!(itc::is_big_int(T::BigIntBase));
  assert!(itc::is_oddball(T::Oddball));
  assert!(itc::is_fixed_array_exact(T::FixedArray));
  assert!(itc::is_map(T::Map));
  assert!(itc::is_free_space(T::FreeSpace));
  assert!(itc::is_filler(T::Filler));
  // 区间
  assert!(itc::is_primitive_heap_object(T::HeapNumber));
  assert!(itc::is_primitive_heap_object(T::Symbol));
  assert!(!itc::is_primitive_heap_object(T::JSObject));
  assert!(itc::is_name(T::Symbol));
  assert!(itc::is_string(T::SharedUncachedExternalOneByteString));
  assert!(itc::is_struct(T::Tuple2));
  assert!(itc::is_trusted_object(T::BytecodeArray));
  assert!(!itc::is_trusted_object(T::Tuple2));
  assert!(itc::is_context(T::NativeContext));
  assert!(itc::is_js_receiver(T::JSProxy));
  assert!(itc::is_js_receiver(T::JSArray));
  assert!(!itc::is_js_receiver(T::Symbol));
  assert!(itc::is_js_object(T::JSArray));
  assert!(!itc::is_js_object(T::JSProxy));
  assert!(itc::is_js_function(T::JSFunctionWithPrototype));
  assert!(itc::is_js_function(T::JSClassConstructor));
  assert!(itc::is_callable_js_function(T::JSFunctionWithoutPrototype));
  assert!(!itc::is_callable_js_function(T::JSClassConstructor));
  // 数组/哈希字典族 (v2 采集器回归)
  assert!(itc::is_fixed_array(T::FixedArray));
  assert!(itc::is_weak_array_list(T::WeakArrayList));
  assert!(itc::is_byte_array(T::ByteArray));
  assert!(itc::is_hash_table(T::HashTable));
  assert!(itc::is_name_dictionary(T::NameDictionary));
  assert!(itc::is_number_dictionary(T::NumberDictionary));
  assert!(itc::is_swiss_name_dictionary(T::SwissNameDictionary));
  assert!(itc::is_transition_array(T::TransitionArray));
  assert!(itc::is_ordered_hash_set(T::OrderedHashSet));
  // MULTIPLE 桶型: JSObject 区间涵盖其同型子类
  assert!(itc::is_js_object(T::JSMap));
  assert!(itc::is_js_object(T::JSSet));
  OK
}

/// 规约 (instance-type-inl.h:597-658): 组合/边界判定。
#[test]
fn composite_checkers() -> Void {
  // 引用可比: 排除 String/BigInt/HeapNumber
  assert!(itc::is_reference_comparable(T::Symbol));
  assert!(itc::is_reference_comparable(T::JSObject));
  assert!(!itc::is_reference_comparable(T::SeqOneByteString));
  assert!(!itc::is_reference_comparable(T::BigIntBase));
  assert!(!itc::is_reference_comparable(T::HeapNumber));
  // GcSafeCode == Code; AbstractCode = BytecodeArray|Code
  assert_eq!(itc::is_gc_safe_code(T::Code), itc::is_code(T::Code));
  assert!(itc::is_abstract_code(T::BytecodeArray));
  assert!(itc::is_abstract_code(T::Code));
  assert!(!itc::is_abstract_code(T::Symbol));
  // FreeSpaceOrFiller
  assert!(itc::is_free_space_or_filler(T::FreeSpace));
  assert!(itc::is_free_space_or_filler(T::Filler));
  assert!(!itc::is_free_space_or_filler(T::FixedArray));
  // MaybeReadOnlyJSObject = JSExternalObject | JSMessageObject
  assert!(itc::is_maybe_read_only_js_object(T::JSExternalObject));
  assert!(itc::is_maybe_read_only_js_object(T::JSMessageObject));
  assert!(!itc::is_maybe_read_only_js_object(T::JSObject));
  // PropertyDictionary (swiss 默认关 → NameDictionary)
  assert!(itc::is_property_dictionary(T::NameDictionary));
  assert!(!itc::is_property_dictionary(T::NumberDictionary));
  // NativeContextSpecific: context true; 非 JSReceiver false;
  // JSReceiver 默认 true, 包装/共享/wasm 例外
  assert!(itc::is_native_context_specific(T::NativeContext));
  assert!(itc::is_native_context_specific(T::FunctionContext));
  assert!(!itc::is_native_context_specific(T::Symbol));
  assert!(itc::is_native_context_specific(T::JSArray));
  assert!(!itc::is_native_context_specific(T::JSExternalObject));
  assert!(!itc::is_native_context_specific(T::JSSharedStruct));
  assert!(!itc::is_native_context_specific(T::WasmStruct));
  // JSApiWrapperObject = JSAPIObjectWithEmbedderSlots | JSSpecialObject
  assert!(itc::is_js_api_wrapper_object(T::JSSpecialApiObject));
  assert!(!itc::is_js_api_wrapper_object(T::JSArray));
  // CppHeapPointerWrapper 聚合
  assert!(itc::is_cpp_heap_pointer_wrapper_object(
    T::CppHeapExternalObject
  ));
  assert!(itc::is_cpp_heap_pointer_wrapper_object(T::CppGcmanagedBase));
  assert!(itc::is_cpp_heap_pointer_wrapper_object(
    T::EmbedderDataArray
  ));
  // special/custom receiver 边界 (objects-inl.h:916/929)
  assert!(itc::is_special_receiver_instance_type(T::JSProxy));
  assert!(!itc::is_special_receiver_instance_type(
    T::JSPrimitiveWrapper
  ));
  assert!(itc::is_custom_elements_receiver_instance_type(
    T::JSPrimitiveWrapper
  ));
  OK
}
