//! 原版 v8/test/unittests/torque/torque-unittest.cc 的断言规约登记与已迁用例
//!
//! harness: `TestCompileTorque` (kTestTorquePrelude + source, CompileTorque,
//! collect_language_server_data + force_assert_statements, torque-unittest.cc:218-232);
//! kTestTorquePrelude 1:1 见 tests/torque_prelude.txt。
//!
//! 已迁条目 (parse 层可判定; declare/生成阶段填体波次逐批解锁其余):
//! - `Torque.StackDeleteRange` → tests/utils.rs::utils_stack_range_and_delete
//! - `Torque.TypeNamingConventionLintError` (L319-324)
//! - `Torque.StructNamingConventionLintError` (L326-331)
//! - `Torque.DoubleUnderScorePrefixIllegalForIdentifiers` (L451-458)
//! - `Torque.ImportNonExistentFile` (L537-540)
//! - `Torque.Enums` 失败半段: 空枚举体 → "unexpected token \"}\"" (L690-696;
//!   成功半段需 declare/finalize, 待后续波次)
//! - `Torque.CatchFirstHandler` (L903-915; catch 位次校验在 parse 动作)
//! - `Torque.Prelude`/`Enums` 成功段/其余 `ExpectSuccessfulCompilation` 类:
//!   需 PredeclarationVisitor/DeclarationVisitor/TypeOracle/ImplementationVisitor
//!   (compile-compiler.cc:84-134), 登记待 declare 填体波次
//!
//! 公共断言原语 (torque-unittest.cc:234-305):
//! - ExpectSuccessfulCompilation: messages == []
//! - ExpectFailingCompilation: messages 非空, 条数 >= pattern 数, 逐条 HasSubstr 匹配,
//!   pattern 附带位置时断言 message.position 有值且 start == 指定 LineAndColumn
//! - SubstrTester(msg, line, col): HasSubstr(msg) + 位置 (line + CountPreludeLines()
//!   - 1, col - 1), 即相对 prelude 尾部的 0 基行列
//!
//! == 余下登记 (declare/生成波次解锁) ==
//! - ClassDefinition (L333-373)、TypeDeclarationOrder (L375-393)、
//!   ConditionalFields (L399-427, "aligned")、ConstexprLetBindingDoesNotCrash
//!   (L429-433)、FailedImplicitCastFromConstexprDoesNotCrash (L435-449)
//! - UnusedLetBindingLintError (L461-468)、UnderscorePrefixSilencesUnusedWarning
//!   (L470-476)、UsingUnderscorePrefixedIdentifierError (L482-490)、
//!   UnusedArgumentLintError (L493-498)、UsingUnderscorePrefixedArgumentSilencesWarning
//!   (L500-504)、UnusedLabelLintError (L506-511)、
//!   UsingUnderScorePrefixLabelSilencesWarning (L513-517)、
//!   NoUnusedWarningForImplicitArguments (L519-523)、
//!   NoUnusedWarningForVariablesOnlyUsedInDchecks (L525-531)、
//!   LetShouldBeConstLintError (L543-550)、LetShouldBeConstIsSkippedForStructs
//!   (L552-560) — lint 源自 implementation-visitor.cc ReportAllUnusedMacros 等
//! - GenericAbstractType (L566-597)、SpecializationRequesters (L599-678, 带
//!   SubstrTester 精确行列)、Enums 成功段 (L681-689)、EnumInTypeswitch (L698-756)、
//!   EnumTypeAnnotations (L758-773)、ConstClassFields (L775-851)、References
//!   (L853-901)、BitFieldLogicalAnd (L917-934)、FieldAccessOnNonClassType
//!   (L936-945)、UnusedImplicit (L947-994)、ImplicitTemplateParameterInference
//!   (L996-1036)、BuiltinReturnsNever (L1038-1071)
//! - TorqueLayoutLoader 组 (~40 TEST, TestCompileTorqueWithLayout): 需
//!   layout-loader 的 import/verify 主体 (layout-loader.cc), 全数登记待布局波次
//!   (VeriferAcceptsMatchingRecord L1190 起, 断言清单见 git 历史/原版)

use aok::{OK, Void};
use log::info;
use v8_torque::{TorqueCompilerOptions, TorqueCompilerResult, compile_torque};

const PRELUDE: &str = include_str!("torque_prelude.txt");

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版 TestCompileTorque (torque-unittest.cc:218-232)
fn test_compile_torque(source: &str) -> TorqueCompilerResult {
  let full = format!("{PRELUDE}{source}");
  let options = TorqueCompilerOptions {
    collect_language_server_data: true,
    force_assert_statements: true,
    ..TorqueCompilerOptions::default()
  };
  match compile_torque(&full, options) {
    Ok(r) => r,
    Err(e) => panic!("TestCompileTorque 基础设施错误: {e}"),
  }
}

/// 原版 ExpectFailingCompilation (L249-268, 无位置断言形态):
/// messages 非空且逐条按序 HasSubstr
fn expect_failing(source: &str, patterns: &[&str]) {
  let result = test_compile_torque(source);
  assert!(
    !result.messages.is_empty(),
    "expected failing compilation, got zero messages"
  );
  let limit = patterns.len().min(result.messages.len());
  for (i, pattern) in patterns.iter().take(limit).enumerate() {
    assert!(
      result.messages[i].message.contains(pattern),
      "messages[{i}] = {:?} 不含 {pattern:?}",
      result.messages[i].message
    );
  }
}

/// 原版 TEST(Torque, TypeNamingConventionLintError) (L319-324)
#[test]
fn torque_type_naming_convention_lint_error() -> Void {
  info!("> torque_type_naming_convention_lint_error");
  expect_failing("\n    type foo generates 'TNode<Foo>';\n  ", &["\"foo\""]);
  OK
}

/// 原版 TEST(Torque, StructNamingConventionLintError) (L326-331)
#[test]
fn torque_struct_naming_convention_lint_error() -> Void {
  info!("> torque_struct_naming_convention_lint_error");
  expect_failing("\n    struct foo {}\n  ", &["\"foo\""]);
  OK
}

/// 原版 TEST(Torque, DoubleUnderScorePrefixIllegalForIdentifiers) (L451-458):
/// `__x` 首字符为下划线 → identifier pattern 不匹配 → "Lexer Error"
#[test]
fn torque_double_under_score_prefix_illegal_for_identifiers() -> Void {
  info!("> torque_double_under_score_prefix_illegal_for_identifiers");
  expect_failing(
    "\n    @export macro Foo(): void {\n      let __x;\n    }\n  ",
    &["Lexer Error"],
  );
  OK
}

/// 原版 TEST(Torque, ImportNonExistentFile) (L537-540)
#[test]
fn torque_import_non_existent_file() -> Void {
  info!("> torque_import_non_existent_file");
  expect_failing("import \"foo/bar.tq\"", &["File 'foo/bar.tq' not found."]);
  OK
}

/// 原版 TEST(Torque, Enums) 失败半段 (L690-696): 空枚举体 → 非空列表无条目
/// → "unexpected token \"}\""; 成功半段 (L681-689) 待 declare 波次
#[test]
fn torque_enums_empty_enum_body_fails() -> Void {
  info!("> torque_enums_empty_enum_body_fails");
  expect_failing(
    "\n    extern enum MyEmptyEnum {\n    }\n  ",
    &["unexpected token \"}\""],
  );
  OK
}

/// 原版 TEST(Torque, CatchFirstHandler) (L903-915): catch 位于 label 后
/// → parse 动作 Error("catch handler always has to be first...")
#[test]
fn torque_catch_first_handler() -> Void {
  info!("> torque_catch_first_handler");
  expect_failing(
    "\n    @export\n    macro Test(): void {\n      try {\n      } label Foo {\n      } catch (_e, _m) {}\n    }\n  ",
    &["catch handler always has to be first, before any label handler"],
  );
  OK
}
