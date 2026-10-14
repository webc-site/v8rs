use std::str::FromStr;

use aok::{OK, Void};
use log::info;
use strum::IntoEnumIterator;
use v8_runtime::{
  DebugPrintValueType, Error, MiscRuntimeFunction, NumbersRuntimeFunction, ObjectPair,
  ObjectRuntimeFunction, OptimizationStatus, RUNTIME_FUNCTION_COUNT, RUNTIME_FUNCTIONS,
  RuntimeCallProperty, RuntimeEnv, RuntimeFunctionId, add, array_is_array, async_function_await,
  atomics_add, big_int_compare_to_number, compile_lazy, date_current_time, debug_break_on_bytecode,
  find_runtime_function, is_js_proxy,
  js_finalization_registry_register_weak_cell_with_unregister_token, map_grow, new_array,
  new_object, number_to_string_slow, promise_hook_init, reg_exp_exec, string_add, typed_array_set,
  wasm_compile_lazy,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_runtime_table_and_descriptors() -> Void {
  info!("Testing runtime table count and descriptors");
  assert_eq!(RUNTIME_FUNCTION_COUNT, 714);
  assert_eq!(RUNTIME_FUNCTIONS.len(), 714);

  let desc = find_runtime_function("ArrayIncludes_Slow").expect("should find ArrayIncludes_Slow");
  assert_eq!(desc.name, "ArrayIncludes_Slow");
  assert_eq!(desc.id, RuntimeFunctionId::ArrayIncludesSlow);
  assert_eq!(desc.nargs, 3);
  assert_eq!(desc.result_size, 1);

  let non_exist = find_runtime_function("NonExistentRuntimeFunction");
  assert!(non_exist.is_none());

  // Test RuntimeFunctionId strum deserialization with aliases
  let id_from_c = RuntimeFunctionId::from_str("kArrayIncludes_Slow").expect("from_str k name");
  let id_from_raw = RuntimeFunctionId::from_str("ArrayIncludes_Slow").expect("from_str raw name");
  let id_from_pascal =
    RuntimeFunctionId::from_str("ArrayIncludesSlow").expect("from_str pascal name");
  assert_eq!(id_from_c, RuntimeFunctionId::ArrayIncludesSlow);
  assert_eq!(id_from_raw, RuntimeFunctionId::ArrayIncludesSlow);
  assert_eq!(id_from_pascal, RuntimeFunctionId::ArrayIncludesSlow);

  assert_eq!(id_from_c.name(), "ArrayIncludes_Slow");
  assert_eq!(id_from_c.c_enum_name(), "kArrayIncludes_Slow");
  assert!(id_from_c.descriptor().is_some());

  // Check iterator count and verify all 714 valid functions have valid descriptors
  let count = RuntimeFunctionId::iter().count();
  assert_eq!(count, 715); // 714 functions + NumFunctions guard

  for id in RuntimeFunctionId::iter() {
    if id == RuntimeFunctionId::NumFunctions {
      assert_eq!(id.name(), "NumFunctions");
      assert_eq!(id.c_enum_name(), "kNumFunctions");
      assert!(id.descriptor().is_none());
      continue;
    }
    let desc = id
      .descriptor()
      .expect("descriptor must exist for function id");
    assert_eq!(desc.id, id);
    assert_eq!(desc.name, id.name());
    assert!(id.c_enum_name().starts_with('k'));
  }

  OK
}

#[test]
fn test_runtime_auxiliary_types() -> Void {
  info!("Testing auxiliary types");
  let opt = OptimizationStatus::Optimized;
  assert_eq!(opt.to_string(), "Optimized");

  let dp = DebugPrintValueType::HeapObject;
  assert_eq!(dp as u16, 2);

  let call_prop = RuntimeCallProperty::CannotTriggerGC;
  assert_eq!(call_prop.to_string(), "kCannotTriggerGC");

  let pair = ObjectPair { x: 42, y: 100 };
  assert_eq!(pair.x, 42);
  assert_eq!(pair.y, 100);

  struct DummyEnv;
  impl RuntimeEnv for DummyEnv {
    fn isolate_address(&self) -> u64 {
      0x1234_5678
    }
    fn has_pending_exception(&self) -> bool {
      false
    }
  }
  let env = DummyEnv;
  assert_eq!(env.isolate_address(), 0x1234_5678);
  assert!(!env.has_pending_exception());

  OK
}

#[test]
fn test_numbers_runtime_functions() -> Void {
  info!("Testing NumbersRuntimeFunction");
  let count = NumbersRuntimeFunction::iter().count();
  assert_eq!(count, 64);

  let fn_add = add();
  assert_eq!(fn_add, NumbersRuntimeFunction::Add);
  assert_eq!(fn_add.name(), "Add");
  assert!(fn_add.is_operators());
  assert!(!fn_add.is_strings());

  let fn_num = number_to_string_slow();
  assert!(fn_num.is_numbers());
  assert!(!fn_num.is_operators());

  let fn_str_add = string_add();
  assert!(fn_str_add.is_strings());
  assert!(!fn_str_add.is_numbers());

  let fn_atomic = atomics_add();
  assert!(fn_atomic.is_atomics());

  let fn_bigint = big_int_compare_to_number();
  assert!(fn_bigint.is_bigint());

  let fn_date = date_current_time();
  assert!(fn_date.is_date());
  assert!(fn_date.descriptor().is_some());
  assert_eq!(
    fn_date.function_id(),
    Some(RuntimeFunctionId::DateCurrentTime)
  );

  assert!(fn_add.descriptor().is_some());
  assert_eq!(fn_add.function_id(), Some(RuntimeFunctionId::Add));

  // Strum deserialization with prefix
  let from_c = NumbersRuntimeFunction::from_str("Runtime_Add").expect("from_str with Runtime_");
  assert_eq!(from_c, NumbersRuntimeFunction::Add);

  // Exhaustive verification of all 64 functions
  for f in NumbersRuntimeFunction::iter() {
    let desc = f.descriptor().expect("descriptor must exist");
    assert_eq!(f.function_id(), Some(desc.id));
    let cat_count = (f.is_numbers() as u8)
      + (f.is_strings() as u8)
      + (f.is_bigint() as u8)
      + (f.is_atomics() as u8)
      + (f.is_date() as u8)
      + (f.is_operators() as u8);
    assert_eq!(
      cat_count, 1,
      "NumbersRuntimeFunction {:?} must match exactly 1 category",
      f
    );
  }

  OK
}

#[test]
fn test_object_runtime_functions() -> Void {
  info!("Testing ObjectRuntimeFunction");
  let count = ObjectRuntimeFunction::iter().count();
  assert_eq!(count, 123);

  let fn_obj = new_object();
  assert_eq!(fn_obj, ObjectRuntimeFunction::NewObject);
  assert!(fn_obj.is_object());

  let fn_arr = new_array();
  assert!(fn_arr.is_array());
  let fn_is_arr = array_is_array();
  assert!(fn_is_arr.is_array());

  let fn_map = map_grow();
  assert!(fn_map.is_collections());

  let fn_ta = typed_array_set();
  assert!(fn_ta.is_typedarray());

  let fn_proxy = is_js_proxy();
  assert!(fn_proxy.is_proxy());

  assert!(fn_obj.descriptor().is_some());
  assert_eq!(fn_obj.function_id(), Some(RuntimeFunctionId::NewObject));
  assert!(fn_arr.descriptor().is_some());
  assert_eq!(fn_arr.function_id(), Some(RuntimeFunctionId::NewArray));

  // Exhaustive verification of all 123 functions
  for f in ObjectRuntimeFunction::iter() {
    let desc = f.descriptor().expect("descriptor must exist");
    assert_eq!(f.function_id(), Some(desc.id));
    let cat_count = (f.is_object() as u8)
      + (f.is_array() as u8)
      + (f.is_collections() as u8)
      + (f.is_typedarray() as u8)
      + (f.is_classes() as u8)
      + (f.is_proxy() as u8);
    assert_eq!(
      cat_count, 1,
      "ObjectRuntimeFunction {:?} must match exactly 1 category",
      f
    );
  }

  OK
}

#[test]
fn test_misc_runtime_functions() -> Void {
  info!("Testing MiscRuntimeFunction");
  let count = MiscRuntimeFunction::iter().count();
  assert_eq!(count, 294);

  let fn_compile = compile_lazy();
  assert!(fn_compile.is_compiler());

  let fn_debug = debug_break_on_bytecode();
  assert!(fn_debug.is_debug());

  let fn_async = async_function_await();
  assert!(fn_async.is_generator());

  let fn_prom = promise_hook_init();
  assert!(fn_prom.is_promise());

  let fn_re = reg_exp_exec();
  assert!(fn_re.is_regexp());

  let fn_wasm = wasm_compile_lazy();
  assert!(fn_wasm.is_wasm());

  let fn_weak = js_finalization_registry_register_weak_cell_with_unregister_token();
  assert!(fn_weak.is_weak_refs());
  assert_eq!(
    fn_weak.function_id(),
    Some(RuntimeFunctionId::JSFinalizationRegistryRegisterWeakCellWithUnregisterToken)
  );

  assert!(fn_compile.descriptor().is_some());
  assert_eq!(
    fn_compile.function_id(),
    Some(RuntimeFunctionId::CompileLazy)
  );

  assert!(fn_async.descriptor().is_some());
  assert_eq!(
    fn_async.function_id(),
    Some(RuntimeFunctionId::AsyncFunctionAwait)
  );

  // Exhaustive verification of all 294 functions
  for f in MiscRuntimeFunction::iter() {
    let desc = f.descriptor().expect("descriptor must exist");
    assert_eq!(f.function_id(), Some(desc.id));
    let cat_count = (f.is_scopes() as u8)
      + (f.is_module() as u8)
      + (f.is_debug() as u8)
      + (f.is_compiler() as u8)
      + (f.is_internal() as u8)
      + (f.is_wasm() as u8)
      + (f.is_function() as u8)
      + (f.is_generator() as u8)
      + (f.is_promise() as u8)
      + (f.is_regexp() as u8)
      + (f.is_symbol() as u8)
      + (f.is_literals() as u8)
      + (f.is_forin() as u8)
      + (f.is_futex() as u8)
      + (f.is_intl() as u8)
      + (f.is_shadow_realm() as u8)
      + (f.is_trace() as u8)
      + (f.is_weak_refs() as u8);
    assert_eq!(
      cat_count, 1,
      "MiscRuntimeFunction {:?} must match exactly 1 category",
      f
    );
  }

  OK
}

#[test]
fn test_error_variants() -> Void {
  info!("Testing Error variants");
  let e1 = Error::FunctionNotFound("foobar".to_string());
  assert_eq!(e1.to_string(), "Runtime function not found: foobar");

  let e2 = Error::InvalidArgumentCount("foo".to_string(), 2, 3);
  assert_eq!(
    e2.to_string(),
    "Invalid argument count for foo: expected 2, got 3"
  );

  let e3 = Error::ExecutionTrapped("trap".to_string());
  assert_eq!(e3.to_string(), "Runtime execution trapped: trap");

  OK
}
