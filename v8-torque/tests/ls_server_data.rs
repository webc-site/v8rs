//! 原版 v8/test/unittests/torque/ls-server-data-unittest.cc 迁移 (可迁 1/10)
//!
//! 断言规约清单 (先规约后断言, 强度守恒):
//! - language_server_goto_type_definition_no_data_for_file: 空定义表上查询必返回
//!   None 且不崩溃 (回归测试, ls-server-data-unittest.cc:82-90)
//!
//! 其余 9 用例经 TestCompiler::Compile → compile_torque 产出 LanguageServerData;
//! compile_torque 未填体 (torque_compiler.rs:66), 规约先行登记, 解锁后逐条生成
//! `#[test]` (与 torque_unittest.rs 同波次):
//! - GotoTypeDefinition (L33-60): 编译 5 行源 (type void/never, T1/T2 generates
//!   'TNode<Object>', macro SomeMacro(a: T1, b: T2): T1 { return a; });
//!   FindDefinition(id, (4,19)) == (id, (2,5), (2,7)); FindDefinition(id, (4,26))
//!   == (id, (3,5), (3,7))
//! - GotoTypeDefinitionExtends (L62-80): type T2 extends T1;
//!   FindDefinition(id, (3,16)) == (id, (2,5), (2,7))
//! - GotoLabelDefinitionInSignature (L96-118, 原版 Fuchsia 跳过, 非 Fuchsia 跑):
//!   Foo(): never labels Fail + Bar(): void labels Bailout, Foo() otherwise
//!   Bailout; FindDefinition(id, (6,18)) == (id, (5,25), (5,32))
//! - GotoLabelDefinitionInTryBlock (L121-144): try { Foo() otherwise Bailout; }
//!   label Bailout {}; FindDefinition(id, (6,25)) == (id, (7,8), (7,15))
//! - GotoDefinitionClassSuperType (L150-169, 原版 Fuchsia 跳过): extern class
//!   HeapObject extends Tagged; FindDefinition(id, (3,33)) == (id, (2,5), (2,11))
//! - GotoLabelDefinitionInSignatureGotoStmt (L171-190): goto Fail;
//!   FindDefinition(id, (3,7)) == (id, (2,26), (2,30))
//! - GotoLabelDefinitionInTryBlockGoto (L192-212): try { goto Bailout; } label
//!   Bailout {}; FindDefinition(id, (3,13)) == (id, (4,8), (4,15))
//! - GotoLabelDefinitionGotoInOtherwise (L214-237): Foo() otherwise goto
//!   Bailout; FindDefinition(id, (6,30)) == (id, (7,8), (7,15))
//! - SymbolsArePopulated (L239-258): 编译带缩进源; SymbolsForSourceId(id) 非空

use aok::{OK, Void};
use log::info;
use v8_torque::{LanguageServerData, LineAndColumn, SourceFileMap};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版 TEST(LanguageServer, GotoTypeDefinitionNoDataForFile) 1:1
/// (回归测试: 本步骤不得崩溃)
#[test]
fn language_server_goto_type_definition_no_data_for_file() -> Void {
  info!("> language_server_goto_type_definition_no_data_for_file");

  let server_data = LanguageServerData::default();
  let mut file_map = SourceFileMap::new(String::new());
  let test_id = file_map.add_source("test.tq".to_owned());

  assert!(
    server_data
      .find_definition(test_id, LineAndColumn::with_unknown_offset(0, 0))
      .is_none()
  );
  OK
}
