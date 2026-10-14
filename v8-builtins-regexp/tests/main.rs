use std::str::FromStr;

use aok::{OK, Void};
use log::info;
use strum::IntoEnumIterator;
use v8_builtins_regexp::{
  Error, REGEXP_TQ_DECLS, RegexpBuiltin, reg_exp_capture1_getter, reg_exp_capture2_getter,
  reg_exp_capture3_getter, reg_exp_capture4_getter, reg_exp_capture5_getter,
  reg_exp_capture6_getter, reg_exp_capture7_getter, reg_exp_capture8_getter,
  reg_exp_capture9_getter, reg_exp_constructor, reg_exp_escape, reg_exp_exec_atom,
  reg_exp_experimental_trampoline, reg_exp_input_getter, reg_exp_input_setter,
  reg_exp_interpreter_trampoline, reg_exp_last_match_getter, reg_exp_last_paren_getter,
  reg_exp_left_context_getter, reg_exp_match_fast, reg_exp_prototype_compile,
  reg_exp_prototype_exec, reg_exp_prototype_exec_slow, reg_exp_prototype_match,
  reg_exp_prototype_match_all, reg_exp_prototype_replace, reg_exp_prototype_search,
  reg_exp_prototype_source_getter, reg_exp_prototype_split, reg_exp_prototype_test,
  reg_exp_prototype_test_fast, reg_exp_prototype_to_string, reg_exp_replace,
  reg_exp_right_context_getter, reg_exp_search_fast, reg_exp_split,
  reg_exp_string_iterator_prototype_next,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_builtin_count_and_strum() -> Void {
  let all: Vec<_> = RegexpBuiltin::iter().collect();
  assert_eq!(all.len(), 37);

  for builtin in all {
    let name = builtin.name();
    assert_eq!(builtin.to_string(), name);
    let parsed = RegexpBuiltin::from_str(name)?;
    assert_eq!(parsed, builtin);
    assert_eq!(
      builtin.descriptor_name(),
      format!("Builtin_{name}_InterfaceDescriptor")
    );
  }
  OK
}

#[test]
fn test_factory_functions() -> Void {
  assert_eq!(
    reg_exp_capture1_getter(),
    RegexpBuiltin::RegExpCapture1Getter
  );
  assert_eq!(
    reg_exp_capture2_getter(),
    RegexpBuiltin::RegExpCapture2Getter
  );
  assert_eq!(
    reg_exp_capture3_getter(),
    RegexpBuiltin::RegExpCapture3Getter
  );
  assert_eq!(
    reg_exp_capture4_getter(),
    RegexpBuiltin::RegExpCapture4Getter
  );
  assert_eq!(
    reg_exp_capture5_getter(),
    RegexpBuiltin::RegExpCapture5Getter
  );
  assert_eq!(
    reg_exp_capture6_getter(),
    RegexpBuiltin::RegExpCapture6Getter
  );
  assert_eq!(
    reg_exp_capture7_getter(),
    RegexpBuiltin::RegExpCapture7Getter
  );
  assert_eq!(
    reg_exp_capture8_getter(),
    RegexpBuiltin::RegExpCapture8Getter
  );
  assert_eq!(
    reg_exp_capture9_getter(),
    RegexpBuiltin::RegExpCapture9Getter
  );
  assert_eq!(reg_exp_constructor(), RegexpBuiltin::RegExpConstructor);
  assert_eq!(reg_exp_escape(), RegexpBuiltin::RegExpEscape);
  assert_eq!(reg_exp_exec_atom(), RegexpBuiltin::RegExpExecAtom);
  assert_eq!(
    reg_exp_experimental_trampoline(),
    RegexpBuiltin::RegExpExperimentalTrampoline
  );
  assert_eq!(reg_exp_input_getter(), RegexpBuiltin::RegExpInputGetter);
  assert_eq!(reg_exp_input_setter(), RegexpBuiltin::RegExpInputSetter);
  assert_eq!(
    reg_exp_interpreter_trampoline(),
    RegexpBuiltin::RegExpInterpreterTrampoline
  );
  assert_eq!(
    reg_exp_last_match_getter(),
    RegexpBuiltin::RegExpLastMatchGetter
  );
  assert_eq!(
    reg_exp_last_paren_getter(),
    RegexpBuiltin::RegExpLastParenGetter
  );
  assert_eq!(
    reg_exp_left_context_getter(),
    RegexpBuiltin::RegExpLeftContextGetter
  );
  assert_eq!(reg_exp_match_fast(), RegexpBuiltin::RegExpMatchFast);
  assert_eq!(
    reg_exp_prototype_compile(),
    RegexpBuiltin::RegExpPrototypeCompile
  );
  assert_eq!(reg_exp_prototype_exec(), RegexpBuiltin::RegExpPrototypeExec);
  assert_eq!(
    reg_exp_prototype_exec_slow(),
    RegexpBuiltin::RegExpPrototypeExecSlow
  );
  assert_eq!(
    reg_exp_prototype_match(),
    RegexpBuiltin::RegExpPrototypeMatch
  );
  assert_eq!(
    reg_exp_prototype_match_all(),
    RegexpBuiltin::RegExpPrototypeMatchAll
  );
  assert_eq!(
    reg_exp_prototype_replace(),
    RegexpBuiltin::RegExpPrototypeReplace
  );
  assert_eq!(
    reg_exp_prototype_search(),
    RegexpBuiltin::RegExpPrototypeSearch
  );
  assert_eq!(
    reg_exp_prototype_source_getter(),
    RegexpBuiltin::RegExpPrototypeSourceGetter
  );
  assert_eq!(
    reg_exp_prototype_split(),
    RegexpBuiltin::RegExpPrototypeSplit
  );
  assert_eq!(reg_exp_prototype_test(), RegexpBuiltin::RegExpPrototypeTest);
  assert_eq!(
    reg_exp_prototype_test_fast(),
    RegexpBuiltin::RegExpPrototypeTestFast
  );
  assert_eq!(
    reg_exp_prototype_to_string(),
    RegexpBuiltin::RegExpPrototypeToString
  );
  assert_eq!(reg_exp_replace(), RegexpBuiltin::RegExpReplace);
  assert_eq!(
    reg_exp_right_context_getter(),
    RegexpBuiltin::RegExpRightContextGetter
  );
  assert_eq!(reg_exp_search_fast(), RegexpBuiltin::RegExpSearchFast);
  assert_eq!(reg_exp_split(), RegexpBuiltin::RegExpSplit);
  assert_eq!(
    reg_exp_string_iterator_prototype_next(),
    RegexpBuiltin::RegExpStringIteratorPrototypeNext
  );
  OK
}

#[test]
fn test_category_predicates() -> Void {
  let static_count = RegexpBuiltin::iter()
    .filter(|b| b.is_static_getter_setter())
    .count();
  assert_eq!(static_count, 15);

  let proto_count = RegexpBuiltin::iter()
    .filter(|b| b.is_prototype_method())
    .count();
  assert_eq!(proto_count, 10);

  let fast_path_count = RegexpBuiltin::iter().filter(|b| b.is_fast_path()).count();
  assert_eq!(fast_path_count, 6);

  let trampoline_count = RegexpBuiltin::iter().filter(|b| b.is_trampoline()).count();
  assert_eq!(trampoline_count, 2);

  let ctor_escape_count = RegexpBuiltin::iter()
    .filter(|b| b.is_constructor_or_escape())
    .count();
  assert_eq!(ctor_escape_count, 2);

  // 15 + 10 + 6 + 2 + 2 = 35, plus RegExpPrototypeExecSlow and RegExpStringIteratorPrototypeNext
  assert!(!RegexpBuiltin::RegExpPrototypeExecSlow.is_fast_path());
  assert!(!RegexpBuiltin::RegExpPrototypeExecSlow.is_prototype_method());
  assert!(!RegexpBuiltin::RegExpStringIteratorPrototypeNext.is_prototype_method());

  OK
}

#[test]
fn test_global_id_and_tq_decls() -> Void {
  // 基础表内置 (22 项在 BUILTIN_TABLE 中)
  assert!(RegexpBuiltin::RegExpConstructor.global_id().is_some());
  assert!(RegexpBuiltin::RegExpCapture1Getter.global_id().is_some());
  assert!(
    RegexpBuiltin::RegExpInterpreterTrampoline
      .global_id()
      .is_some()
  );

  // Torque 独有内置 (不在基础表 BUILTIN_TABLE 中)
  assert!(RegexpBuiltin::RegExpPrototypeExec.global_id().is_none());
  assert!(RegexpBuiltin::RegExpPrototypeMatch.global_id().is_none());

  assert_eq!(REGEXP_TQ_DECLS.len(), 24);
  assert_eq!(REGEXP_TQ_DECLS[0].0, "extern macro");
  assert_eq!(REGEXP_TQ_DECLS[0].1, "RegExpBuiltinsAssembler");
  OK
}

#[test]
fn test_error_display() -> Void {
  let err1 = Error::InvalidFlags("xyz".into());
  assert_eq!(err1.to_string(), "Invalid regular expression flags: xyz");

  let err2 = Error::SyntaxError("unexpected token".into());
  assert_eq!(
    err2.to_string(),
    "Regular expression syntax error: unexpected token"
  );

  let err3 = Error::StackOverflow;
  assert_eq!(err3.to_string(), "RegExp stack overflow");

  let err4 = Error::ExecutionFailed("timeout".into());
  assert_eq!(err4.to_string(), "RegExp execution failed: timeout");

  info!("All RegExp tests passed cleanly");
  OK
}
