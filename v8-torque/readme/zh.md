# v8-torque 测试覆盖映射

规约来源: `todo.md` 7.3 (规约先行 / 断言强度守恒 / 覆盖映射入 README / 跳过必写理由)。
每个 Rust 测试文件头部即为该文件的断言规约清单 (原版文件:行 → 断言), 本表只做原测文件级映射。

## 1. 原版 unittests 映射 (`v8/test/unittests/torque/`)

| 原版文件 | 用例数 | Rust 落点 | 状态 |
| --- | --- | --- | --- |
| `torque-utils-unittest.cc` | 2 | `v8-torque/tests/utils.rs` (`torque_utils_file_uri_decode_illegal`、`torque_utils_file_uri_decode`) | 1:1 全移植 (测试名保留原版 `TEST(Suite, Name)`; 另 +7 派生覆盖 utils.h|cc 语义面) |
| `earley-parser-unittest.cc` | 1 | `v8-torque/tests/earley_parser.rs` (`earley_parser_simple_arithmetic`) | 1:1 全移植 (+1 本仓回归 `earley_parser_ambiguous_grammar_reported`, CheckAmbiguity 语义, earley-parser.cc:100-117) |
| `ls-json-unittest.cc` | 6 | `v8-torque/tests/ls_json.rs` | 6 例 1:1 全移植 (+1 序列化往返 `language_server_json_serialize_roundtrip`) |
| `ls-message-unittest.cc` | 8 | — | 跳过 (见 §2) |
| `ls-server-data-unittest.cc` | 10 | `v8-torque/tests/ls_server_data.rs` | 1/10 可迁 (`GotoTypeDefinitionNoDataForFile`), 9 例跳过 |
| `torque-unittest.cc` (`Torque.*` 35 + `TorqueLayoutLoader.*` 52) | 87 | `v8-torque/tests/torque_unittest.rs` | 6 例已迁全过 (parse 层), 81 例跳过 |

## 2. 跳过理由 (逐条, 均为上游实现未填体, 非用例本身不可移植)

- `ls-json-unittest.cc` 6 例: **已迁全过** (`src/ls/json_parser.rs` parse_json +
  `src/ls/json.rs` serialize_to_string 填体; 实现为手写单遍递归下降, 原版为
  Earley 文法驱动 (ls/json-parser.cc JsonGrammar) — 可观测行为等价, 登记偏差)。
- `ls-server-data-unittest.cc` 9 例 (GotoTypeDefinition / GotoTypeDefinitionExtends /
  GotoLabelDefinitionInSignature / GotoLabelDefinitionInTryBlock / GotoDefinitionClassSuperType /
  GotoLabelDefinitionInSignatureGotoStmt / GotoLabelDefinitionInTryBlockGoto /
  GotoLabelDefinitionGotoInOtherwise / SymbolsArePopulated):
  依赖整条编译管线的 declare/生成段 (`PredeclarationVisitor::Predeclare` →
  `DeclarationVisitor::Visit` → `TypeOracle::FinalizeAggregateTypes` →
  `ImplementationVisitor` → `LanguageServerData` 采集, torque-compiler.cc:84-139)。
  `src/declarations.rs` (待填体)、`src/type_oracle.rs` (Finalize 族待填体)、
  `src/implementation_visitor.rs` 均未填体, 无法产出 `Symbols`/跳转结果。
  parse 段 (`parse_torque`) 已通, declare 填体波次解锁。
- `ls-message-unittest.cc` 8 例: 依赖 v8-torque LSP 消息面 (LanguageServerMessage/
  handle_message/writer 协议族) — 未实现 (`src/ls/message*.rs` 为骨架)。
  LSP 面就位后删除测试文件头部 `#![cfg(any())]` 恢复。
- `torque-unittest.cc` 余 81 例:
  - `Torque.Prelude` (L309) 与全部 `ExpectSuccessfulCompilation` 类: 需
    `CompileCurrentAst` 全程跑通 (predeclare + declare + finalize +
    ImplementationVisitor 生成); parse 段已通, `src/declaration_visitor.rs`/
    `src/implementation_visitor.rs` 入口仍返回 `Error::Unimplemented`。
  - lint/语义诊断类 (UnusedLetBindingLintError、LetShouldBeConstLintError、
    GenericAbstractType、SpecializationRequesters、ConstClassFields、References、
    BitFieldLogicalAnd、FieldAccessOnNonClassType、UnusedImplicit、
    ImplicitTemplateParameterInference、BuiltinReturnsNever 等): 断言点在
    declaration-visitor.cc / implementation-visitor.cc / type-inference.cc, 待 declare 波次。
  - `TorqueLayoutLoader.*` 52 例: 需 `src/layout_loader.rs` 的 import/verify 主体
    (layout-loader.cc), 待布局波次。
  - 其中命名谓词本体已在 `tests/utils.rs::utils_naming_convention_predicates` 直接覆盖
    (断言取 `utils.cc:223-254`)。

## 3. 已迁用例明细 (`torque-unittest.cc` parse 层, 全过)

| Rust 测试 (`tests/torque_unittest.rs`) | 原版 | 断言 |
| --- | --- | --- |
| `torque_type_naming_convention_lint_error` | L319-324 | `type foo` → 消息含 `"foo"` (lint 在 parse 动作 MakeAbstractTypeDeclaration) |
| `torque_struct_naming_convention_lint_error` | L326-331 | `struct foo {}` → 含 `"foo"` |
| `torque_double_under_score_prefix_illegal_for_identifiers` | L451-458 | `let __x` → "Lexer Error" (identifier pattern 拒双下划线首字符) |
| `torque_import_non_existent_file` | L537-540 | `import "foo/bar.tq"` → "File 'foo/bar.tq' not found." |
| `torque_enums_empty_enum_body_fails` | L690-696 (Enums 失败半段) | 空枚举体 → "unexpected token \"}\"" (成功半段待 declare) |
| `torque_catch_first_handler` | L903-915 | catch 在 label 后 → "catch handler always has to be first..." |

harness 断言原语 (torque-unittest.cc:234-268): `ExpectFailingCompilation` = messages 非空 +
逐条按序 HasSubstr; `kTestTorquePrelude` 1:1 见 `tests/torque_prelude.txt`
(collect_language_server_data + force_assert_statements, torque-unittest.cc:218-232)。
prelude 已随 parse 链全量跑通零诊断。带位置断言的 `SubstrTester` 形态
(SpecializationRequesters 等) 待 declare 波次。

## 4. 上游无专属 unittest、按源码语义补的用例

原版 `v8/src/torque` 的 `types.h|cc`、`utils.h|cc` 无对应 unittest 文件 (已在 §2 注明),
故按源码行号先立规约再写断言, 强度守恒:

| Rust 文件 | 用例数 | 规约来源 |
| --- | --- | --- |
| `v8-torque/tests/types.rs` | 12 | `v8/src/torque/types.h`/`types.cc` (IsSubtypeOf L104-116、ToString L38-57、SimpleName L59-73、ComputeName L550-571、ToExplicitString/SimpleNameImpl 各 kind 覆写、GetGenerated*TypeName L183-230/520/626-633、constexpr 链接 types.h:275-313 + type-oracle.h:30-33、IsTransient L437-443/689、Combine L666-674、Signature L891-910) |
| `v8-torque/tests/utils.rs` | 9 (2 原测 + 7 派生) | `v8/src/torque/utils.cc:24-77/167-323/385-394` + `utils.h:171-292/423-492/494-560` |

## 5. 测试侧登记的偏差

- 类型 arena: 原版 `const Type*` 指针同一性 → `TypeId` (arena 下标), 故 `is_subtype_of` 的下标相等
  即原版指针相等; 并集成员 `std::set<Type*, TypeLess>` → `BTreeSet<TypeId>`, 渲染序确定, 不依赖随机序。
- `TypeOracle::GetUnionType` 的 interning 未填体 (`src/type_oracle.rs`),
  测试用同构直填 (`kind/parent/id/data` 与 `push_type` 一致, `id` 取下标) 构造并集;
  填体后可切回公开入口, 断言不变。
- `tests/utils.rs::torque_utils_file_uri_decode` 用 `cfg!(windows)` 复刻原版 `#ifdef V8_OS_WIN`
  双分支 (`utils.cc:87-91` 前缀 `file:///` vs `file://`), 单平台只执行对应分支, 与原版一致。
- parse 上下文: 原版 `CurrentAst`/`CurrentSourcePosition`/`TorqueMessages`/`SourceFileMap`/
  `TargetArchitecture`/`BuildFlags` 体系 (base::ContextualClass) → `src/context.rs`
  thread_local 直译 (编译器单线程); `Rule::RunAction` 逐动作设 `CurrentSourcePosition`
  (earley-parser.cc:56), 错误流 `Error(...).Throw()` → `Err(Error::Abort)` 由
  `compile_torque` 收敛 (torque-compiler.cc:162 catch)。
- BuildFlags 值对齐上游默认 x64 release (sandbox/ptr-compr/intl/webassembly 开,
  tsan/drumbrake 关); `@if`/`@ifnot` 分支随 TargetArchitecture 尺寸 (kTaggedSize=4)。
- 编译器选项/结果/入口: `TorqueCompilerOptions`/`TorqueCompilerResult`/
  `TorqueCompilationUnit`/`compile_torque`/`compile_torque_files`/`compile_torque_for_kythe`
  (torque-compiler.h/cc 1:1); `compile_torque` 当前为 parse 段管线: 上下文束 + parse +
  诊断收集, parse 干净且 declare 未填体时提前收敛返回空消息 (见 §6, declare 填体后移除)。
- ls JSON: 原版 json-parser.cc 以 Earley 文法 (JsonGrammar) 解析, Rust 版为等价手写
  递归下降 (`src/ls/json_parser.rs`), 错误消息面 ("Lexer Error: unknown token" /
  "Parser Error: unexpected token") 与原版一致。
- 运行方式: 门禁为
  `CARGO_TARGET_DIR=target/<专属> cargo clippy -p v8-torque --all-targets -- -D warnings`、
  `CARGO_TARGET_DIR=target/<专属> cargo nextest run -p v8-torque` (37 例全过)。

## 6. 深水区登记 (后续波次)

| 板块 | 位置 | 说明 |
| --- | --- | --- |
| declare 段 | `src/declaration_visitor.rs`、`src/declarations.rs` (35 处待填体) | PredeclarationVisitor/DeclarationVisitor 全程 (torque-compiler.cc:84-109); 解锁 `ExpectSuccessfulCompilation` 类与 ls-server-data 9 例 |
| 类型收尾 | `src/type_oracle.rs` | GetUnionType interning、FinalizeAggregateTypes、GetGenericTypeInstance、reference/slice/weak/lazy 泛型族 (type-oracle.cc) |
| 生成段 | `src/implementation_visitor.rs`、`src/csa_generator.rs`、`src/cc_generator.rs`、`src/tsa_generator.rs`、`src/torque_code_generator.rs` | ImplementationVisitor → CFG → CSA/TSA 代码生成 (torque-compiler.cc:110-134) |
| 布局链 | `src/layout_loader.rs` | import/verify_cpp_layouts 主体 (layout-loader.cc), 解锁 TorqueLayoutLoader 52 例 |
| LSP 消息面 | `src/ls/message*.rs`、`src/ls/torque_language_server.rs` | handle_message/compilation_finished/writer 协议族 (ls/message-handler.cc), 解锁 ls-message 8 例 |
| kythe | `src/kythe_data.rs`、`compile_torque_for_kythe` | kythe 数据采集 (kythe-data.cc) |
| `compile_torque` 收敛分支 | `src/torque_compiler.rs` | parse 干净时提前返回的兜底, declare 填体后移除, Unimplemented 一律冒泡 |
