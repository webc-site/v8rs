use std::str::FromStr;

use aok::{OK, Void};
use log::info;
use strum::IntoEnumIterator;
use v8_builtins_misc::{
  DATA_TIME_JSON_TQ_DECLS, DateTimeJsonBuiltin, Error, NUMBER_TQ_DECLS, NumberBuiltin,
  OBJECT_FUNC_TQ_DECLS, ObjectFuncBuiltin, SYSTEM_TQ_DECLS, SystemBuiltin, WASM_TQ_DECLS,
  WasmBuiltin, abort, big_int_constructor, clone_object_ic, date_now, function_constructor,
  json_parse, math_sum_precise, number_prototype_to_exponential, wasm_constructor_wrapper,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_number_builtins() -> Void {
  let all: Vec<_> = NumberBuiltin::iter().collect();
  assert_eq!(all.len(), 30);

  for builtin in &all {
    let name = builtin.name();
    assert_eq!(builtin.to_string(), name);
    let parsed = NumberBuiltin::from_str(name)?;
    assert_eq!(&parsed, builtin);
    assert_eq!(
      builtin.descriptor_name(),
      format!("Builtin_{name}_InterfaceDescriptor")
    );
    let _ = builtin.global_id();
  }

  // 工厂函数抽样验证
  assert_eq!(big_int_constructor(), NumberBuiltin::BigIntConstructor);
  assert_eq!(math_sum_precise(), NumberBuiltin::MathSumPrecise);
  assert_eq!(
    number_prototype_to_exponential(),
    NumberBuiltin::NumberPrototypeToExponential
  );

  // 谓词验证
  assert!(NumberBuiltin::BigIntConstructor.is_bigint());
  assert!(NumberBuiltin::NumberPrototypeToExponential.is_number_prototype());
  assert!(NumberBuiltin::MathSumPrecise.is_math());
  assert!(NumberBuiltin::AddStringBaseline.is_baseline_or_feedback());
  assert!(NumberBuiltin::AddLhsIsStringConstantInternalizeTrampoline.is_trampoline());

  // 谓词精确统计
  assert_eq!(all.iter().filter(|b| b.is_bigint()).count(), 10);
  assert_eq!(all.iter().filter(|b| b.is_number_prototype()).count(), 4);
  assert_eq!(all.iter().filter(|b| b.is_math()).count(), 1);
  assert_eq!(
    all.iter().filter(|b| b.is_baseline_or_feedback()).count(),
    10
  );
  assert_eq!(all.iter().filter(|b| b.is_trampoline()).count(), 2);

  // 元数据
  assert_eq!(NUMBER_TQ_DECLS.len(), 159);
  OK
}

#[test]
fn test_object_func_builtins() -> Void {
  let all: Vec<_> = ObjectFuncBuiltin::iter().collect();
  assert_eq!(all.len(), 103);

  for builtin in &all {
    let name = builtin.name();
    assert_eq!(builtin.to_string(), name);
    let parsed = ObjectFuncBuiltin::from_str(name)?;
    assert_eq!(&parsed, builtin);
    assert_eq!(
      builtin.descriptor_name(),
      format!("Builtin_{name}_InterfaceDescriptor")
    );
    let _ = builtin.global_id();
  }

  // 工厂函数与谓词验证
  assert_eq!(
    function_constructor(),
    ObjectFuncBuiltin::FunctionConstructor
  );
  assert!(ObjectFuncBuiltin::CallSitePrototypeGetFileName.is_callsite());
  assert!(ObjectFuncBuiltin::CallSitePrototypeIsConstructor.is_callsite());
  assert!(!ObjectFuncBuiltin::CallSitePrototypeIsConstructor.is_constructor());
  assert!(ObjectFuncBuiltin::FunctionConstructor.is_constructor());
  assert!(ObjectFuncBuiltin::FunctionPrototypeBind.is_function_prototype());
  assert!(ObjectFuncBuiltin::ErrorConstructor.is_error());
  assert!(ObjectFuncBuiltin::ConstructBaseline.is_baseline_or_feedback());

  // 谓词精确统计
  assert_eq!(all.iter().filter(|b| b.is_callsite()).count(), 22);
  assert_eq!(all.iter().filter(|b| b.is_constructor()).count(), 18);
  assert_eq!(all.iter().filter(|b| b.is_function_prototype()).count(), 6);
  assert_eq!(all.iter().filter(|b| b.is_error()).count(), 4);
  assert_eq!(
    all.iter().filter(|b| b.is_baseline_or_feedback()).count(),
    11
  );

  // 元数据
  assert_eq!(OBJECT_FUNC_TQ_DECLS.len(), 30);
  OK
}

#[test]
fn test_data_time_json_builtins() -> Void {
  let all: Vec<_> = DateTimeJsonBuiltin::iter().collect();
  assert_eq!(all.len(), 165);

  for builtin in &all {
    let name = builtin.name();
    assert_eq!(builtin.to_string(), name);
    let parsed = DateTimeJsonBuiltin::from_str(name)?;
    assert_eq!(&parsed, builtin);
    assert_eq!(
      builtin.descriptor_name(),
      format!("Builtin_{name}_InterfaceDescriptor")
    );
    let _ = builtin.global_id();
  }

  // 工厂函数与谓词验证
  assert_eq!(date_now(), DateTimeJsonBuiltin::DateNow);
  assert_eq!(json_parse(), DateTimeJsonBuiltin::JsonParse);

  assert!(DateTimeJsonBuiltin::DateNow.is_date());
  assert!(!DateTimeJsonBuiltin::DateTimeFormatConstructor.is_date());
  assert!(DateTimeJsonBuiltin::DateTimeFormatConstructor.is_intl());
  assert!(DateTimeJsonBuiltin::V8BreakIteratorConstructor.is_intl());
  assert!(DateTimeJsonBuiltin::StringPrototypeLocaleCompareIntl.is_intl());
  assert!(DateTimeJsonBuiltin::SegmentIteratorPrototypeNext.is_intl());
  assert!(DateTimeJsonBuiltin::DatePrototypeToTemporalInstant.is_temporal());
  assert!(DateTimeJsonBuiltin::JsonParse.is_json());
  assert!(DateTimeJsonBuiltin::ConsoleTime.is_console());

  // 谓词精确统计与互斥性验证
  assert_eq!(all.iter().filter(|b| b.is_date()).count(), 51);
  assert_eq!(all.iter().filter(|b| b.is_intl()).count(), 98);
  assert_eq!(all.iter().filter(|b| b.is_temporal()).count(), 8);
  assert_eq!(all.iter().filter(|b| b.is_json()).count(), 5);
  assert_eq!(all.iter().filter(|b| b.is_console()).count(), 5);

  // Date 与 Intl 严密互斥
  for builtin in &all {
    assert!(
      !(builtin.is_date() && builtin.is_intl()),
      "Builtin {:?} cannot be both Date and Intl",
      builtin
    );
  }

  // 元数据
  assert_eq!(DATA_TIME_JSON_TQ_DECLS.len(), 1);
  OK
}

#[test]
fn test_system_builtins() -> Void {
  let all: Vec<_> = SystemBuiltin::iter().collect();
  assert_eq!(all.len(), 274);

  for builtin in &all {
    let name = builtin.name();
    assert_eq!(builtin.to_string(), name);
    let parsed = SystemBuiltin::from_str(name)?;
    assert_eq!(&parsed, builtin);
    assert_eq!(
      builtin.descriptor_name(),
      format!("Builtin_{name}_InterfaceDescriptor")
    );
    let _ = builtin.global_id();
  }

  // 工厂函数与谓词验证
  assert_eq!(abort(), SystemBuiltin::Abort);
  assert_eq!(clone_object_ic(), SystemBuiltin::CloneObjectIC);

  assert!(SystemBuiltin::KeyedLoadIC.is_ic());
  assert!(SystemBuiltin::CloneObjectIC.is_ic());
  assert!(SystemBuiltin::DefineNamedOwnIC.is_ic());
  assert!(SystemBuiltin::LookupGlobalIC.is_ic());
  assert!(!SystemBuiltin::InterpreterEntryTrampoline.is_ic());
  assert!(!SystemBuiltin::IndirectPointerBarrierIgnoreFP.is_ic());
  assert!(!SystemBuiltin::GlobalDecodeURIComponent.is_ic());
  assert!(!SystemBuiltin::GlobalEncodeURIComponent.is_ic());
  assert!(SystemBuiltin::InterpreterEntryTrampoline.is_trampoline());
  assert!(SystemBuiltin::CallFunctionForwardVarargs.is_call());
  assert!(SystemBuiltin::InterpreterEntryTrampoline.is_interpreter());
  assert!(SystemBuiltin::ShadowRealmConstructor.is_shadow_realm());
  assert!(SystemBuiltin::AtomicsConditionWait.is_atomics());

  // 谓词精确统计
  assert_eq!(all.iter().filter(|b| b.is_ic()).count(), 85);
  assert_eq!(all.iter().filter(|b| b.is_trampoline()).count(), 22);
  assert_eq!(all.iter().filter(|b| b.is_call()).count(), 37);
  assert_eq!(all.iter().filter(|b| b.is_interpreter()).count(), 8);
  assert_eq!(all.iter().filter(|b| b.is_shadow_realm()).count(), 6);
  assert_eq!(all.iter().filter(|b| b.is_atomics()).count(), 9);

  // 元数据
  assert_eq!(SYSTEM_TQ_DECLS.len(), 27);
  OK
}

#[test]
fn test_wasm_builtins() -> Void {
  let all: Vec<_> = WasmBuiltin::iter().collect();
  assert_eq!(all.len(), 9);

  for builtin in &all {
    let name = builtin.name();
    assert_eq!(builtin.to_string(), name);
    let parsed = WasmBuiltin::from_str(name)?;
    assert_eq!(&parsed, builtin);
    assert_eq!(
      builtin.descriptor_name(),
      format!("Builtin_{name}_InterfaceDescriptor")
    );
    let _ = builtin.global_id();
  }

  // 工厂函数与谓词验证
  assert_eq!(
    wasm_constructor_wrapper(),
    WasmBuiltin::WasmConstructorWrapper
  );
  assert!(WasmBuiltin::WasmFloat32ToNumber.is_conversion());
  assert!(WasmBuiltin::WasmConstructorWrapper.is_wrapper());
  assert!(WasmBuiltin::JSToWasmLazyDeoptContinuation.is_deopt());

  // 谓词精确统计
  assert_eq!(all.iter().filter(|b| b.is_conversion()).count(), 3);
  assert_eq!(all.iter().filter(|b| b.is_wrapper()).count(), 4);
  assert_eq!(all.iter().filter(|b| b.is_deopt()).count(), 1);

  // 元数据
  assert_eq!(WASM_TQ_DECLS.len(), 191);
  OK
}

#[test]
fn test_error_display() -> Void {
  let err1 = Error::NumberError("NaN overflow".into());
  assert_eq!(
    err1.to_string(),
    "Number/BigInt operation error: NaN overflow"
  );

  let err2 = Error::DateTimeError("invalid timezone".into());
  assert_eq!(
    err2.to_string(),
    "Date/Time or Intl format error: invalid timezone"
  );

  let err3 = Error::SystemError("dispatch trap".into());
  assert_eq!(
    err3.to_string(),
    "System builtin dispatch error: dispatch trap"
  );

  let err4 = Error::WasmError("unreachable".into());
  assert_eq!(err4.to_string(), "WebAssembly runtime error: unreachable");

  info!("All miscellaneous builtin tests passed cleanly!");
  OK
}
