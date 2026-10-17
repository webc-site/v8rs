[English](#en) | [中文](#zh)

---

<a name="en"></a>

# v8-ast (English)

Rust port of V8 `src/ast`, the L2 frontend layer on the M4 critical path (v8-base → v8-zone → **v8-ast** → v8-parsing). Upstream: 19 files / ~15.3k lines of C++.

See [中文](zh.md) for the complete file→module mapping table, X-macro expansion counts and phase-3 breakdown. Key points:

- C++ virtual dispatch through multiple inheritance → Rust tagged enums + `enum_dispatch`. `AstNode` (60 variants, ordered per `AST_NODE_LIST` + `FAILURE_NODE_LIST`) is the dynamic dispatch enum; `Statement` (22) / `Expression` (35) / `Declaration` (2) bridge enums mirror `AsStatement()/AsExpression()` downcasts.
- Conversion scripts under `tool/src/ast/` generate the `*_generated.rs` baselines (idempotent, hash-verified); `bun tool/src/ast/ast.h.js` etc. regenerate them.
- `AstStringConstant` table: 177 entries (128 one-char ASCII + 49 named) expanded from the upstream X-macro; node-type discriminants 0..=59 match upstream expansion order 1:1.
- Upstream pointer identities (`Scope*` / `Variable*`) map to arena handles (`ScopeId` / `VariableId` owned by `ScopeTree`); nodes reference scopes/variables via `Option<ScopeId>` / `Option<VariableId>` without `Rc<RefCell>` or unsafe.
- Gates: `cargo check` clean, clippy `--all-targets -D warnings` clean, `cargo fmt --check` clean, nextest 23/23.


---

<a name="zh"></a>

# v8-ast

V8 `src/ast` 的 Rust 转写, L2 前端层, M4 最小闭环关键路径 (v8-base → v8-zone → **v8-ast** → v8-parsing)。原版 19 文件 / 约 15.3k 行 C++; 当前落地节点全集、类别桥接、Visitor、值工厂、变量层与模块描述符、节点字段补全、工厂全 60 节点构造器 + Zone 分配路径、scopes.cc 解析期语义第一阶段, src 约 5.7k 行 (含生成基线) + tests 约 1.7k 行。

架构总纲: C++ 多继承虚函数分派 (AstNode ← Statement/Expression ← 具体节点) → Rust tagged enum + enum_dispatch。`AstNode` 统一枚举 (60 变体, 顺序 = `AST_NODE_LIST` + `FAILURE_NODE_LIST` 展开) 承担动态派发; `Statement`(22)/`Expression`(35)/`Declaration`(2) 桥接枚举承担原版 `AsStatement()/AsExpression()` 向下转型 (`from_node`/`into_node`, 类型不符返回 `None`)。

架构决策:

- 值语义主干保持 (60 结构体无生命周期参数, 集合字段 `Vec`, 子节点 `Box`), 与第一阶段签名兼容。
- 上游指针身份 (Scope*/Variable*) → arena 句柄 (`ScopeId`/`VariableId`, `ScopeTree` 承载全部解析期作用域与变量), 节点字段以 `Option<ScopeId>`/`Option<VariableId>` 挂接, 无 `Rc<RefCell>`、无 unsafe。
- Zone 分配路径以工厂后端形态接入: `AstNodeFactory::with_zone` 后 `alloc_node` (原版 NewAstNode 节点驻留) + `new_zone_list`/`zone_vec_from_slice`/`zone_vec` (集合物化); 避开 v8-zone 的 Snapshot/reset 族。AstNode 值集合的整体 zone 物化需要节点类型带 `'z/'a` 生命周期 (全链路手术), 留待 v8-parsing 接入时决策, 当前以节点驻留 + 叶子集合/辅助结构物化覆盖解析器真实用法。

- [文件 → 模块映射](#文件-模块映射)
- [X-macro 展开对数 (脚本日志 + `const _: ()` 断言 + 测试三方互证)](#x-macro-展开对数-脚本日志-const-_-断言-测试三方互证)
- [语义要点 / 偏差登记](#语义要点-偏差登记)
- [测试映射](#测试映射)
- [第三阶段拆块建议](#第三阶段拆块建议)

## 文件 → 模块映射

脚本生成基线 (`tool/src/ast/*.js`, 复跑幂等, 已哈希验证; `bun tool/src/ast/ast.h.js` 等复跑再生) + Developer 修订层; `*_generated.rs` 由脚本直出, 复跑覆盖。

| 原版 (v8/src/ast) | Rust 模块 | 转换脚本 | 说明 |
|---|---|---|---|
| `ast.h` + `src/parsing/token.h` | `src/ast_generated.rs` | `tool/src/ast/ast.h.js` | AstNodeType 枚举 (60 项, 判别值 = 上游展开序) / 操作符族 (Binary 15 / Compare 10 / Unary 7 / Count 2 / CompoundAssign 15) / Statement/Expression/Declaration 桥接枚举 / 位域基类 AstNodeBase (NodeTypeField 6 位) |
| `ast.h` / `ast.cc` | `src/ast.rs` | — (修订层) | 60 节点结构体 (第二阶段字段补全: FunctionLiteral 20 字段 / ClassLiteral 15 字段 / VariableProxy 消解位 + var 绑定 / Assignment op / Block/TryCatch/ForEach 作用域挂接) + `AstNode` 统一枚举 + `AstNodeFactory` 全 60 节点构造器 + zone 分配路径 + FunctionSyntaxKind(5)/CatchPrediction(5)/AstAssignOp(16)/PreparseData 辅助类型 |
| `ast.h` AstVisitor | `src/visitor.rs` | — (修订层) | `AstVisitor` trait: visit/enter_node/walk_node/leave_node 剪枝骨架 + 60 个 `visit_*` 方法全集 (适配 ClassLiteral 公开/私有成员、AutoAccessor proxy、SuperPropertyReference proxy) |
| `ast-value-factory.h` / `.cc` | `src/value.rs` + `src/value_generated.rs` | `tool/src/ast/ast-value-factory.h.js` | AstRawString (单/双字节 + Latin-1 规范化 + 数组索引/特殊索引判定) / AstConsString / AstBigInt / AstValue / AstValueFactory (字典去重); 常量表 `AstStringConstant` 177 项 = 单字符 128 (heap-symbols.h 生成器宏, 判别值 = 字节值) + 命名 49 (`AST_STRING_CONSTANTS` X-macro) |
| `modules.h` / `modules.cc` | `src/modules.rs` + `src/modules_generated.rs` | `tool/src/ast/modules.h.js` | SourceTextModuleDescriptor: 导入/导出/星号重导出 + `MakeIndirectExportsExplicit` + `AssignCellIndices` 槽位分配; ModuleImportPhase / CellIndexKind / ScannerLocation |
| `variables.h` / `variables.cc` | `src/variables.rs` + `src/variables_generated.rs` | `tool/src/ast/variables.h.js` | Variable (模式/种类/位置/初始化/maybe-assigned/静态 + scope 句柄 + `AllocateTo`/`ForceContextAllocation`/`SetMaybeAssigned` 分配辅助 + 空洞检测分析与 64 位位图); 常量: HoleCheckState / ForceHoleInitializationFlag 位标志 / kUncacheableHoleCheckBitmapIndex / kHoleCheckBitmapBits; 变量模式等共享枚举在 v8-common |
| `scopes.h` / `scopes.cc` | `src/scopes.rs` | — (修订层) | 解析期语义第一阶段 (arena 句柄): 作用域树构建 / Declare 族 (本地/重声明检测/var 提升/catch/形参/函数名/生成器对象/new.target/临时) / Lookup 族 (沿链 + with 降级 + sloppy-eval 降级 + 动态全局回退) / unresolved 登记 + 递归消解 / Allocate 族 (receiver/形参/栈槽/context 槽/递归分配) / context 链长与类别判定; ScopeInfo 反序列化与 preparse 数据路径推迟 |
| `prettyprinter.h` / `.cc` | 未转写 | — | 1.7k 行, 依赖节点字段全集, 第三阶段随打印断言需求转写 |
| `ast-source-ranges.h` / `source-range-ast-visitor.*` | 未转写 | — | 源码区间注记, 依赖 AstTraversalVisitor 泛型遍历, 第三阶段 |
| `ast-traversal-visitor.h` | `visitor.rs` 部分覆盖 | — | 泛型自动递归 visitor; 其递归语义已由 walk_node/默认 visit_* 承担, 泛型模板形态按需回补 |
| `ast-function-literal-id-reindexer.h` / `.cc` | 未转写 | — | 193 行, 依赖 AstTraversalVisitor + ProducedPreparseData, 随解析器接入转写 |

依赖: v8-zone / v8-strings / v8-numbers / v8-flags / v8-common (`cargo add --path`); `enum_dispatch` (消 dyn 派发) / `strum` (枚举 Display/FromRepr) / `gxhash` (工厂字典) / `thiserror` (error.rs: Redeclaration / Zone 透明转发 / NoZoneBackend)。dev-deps: aok / ctor / log / log_init。

## X-macro 展开对数 (脚本日志 + `const _: ()` 断言 + 测试三方互证)

| 上游 X-macro | 展开数 | 本地锚点 |
|---|---|---|
| `DECLARATION_NODE_LIST` | 2 | `Declaration::COUNT` / `AstNodeType::DECLARATION_COUNT` |
| `STATEMENT_NODE_LIST` (含 ITERATION 5 + BREAKABLE 2) | 22 | `Statement::COUNT` / `STATEMENT_COUNT` / `is_statement` 位集 |
| `EXPRESSION_NODE_LIST` (含 LITERAL 3) | 35 | `Expression::COUNT` / `EXPRESSION_COUNT` / `is_expression` 位集 |
| `AST_NODE_LIST` | 59 | `AstNodeType::AST_NODE_COUNT`, 判别值 0..=58 与上游展开序逐项一致 (测试 `EXPECTED_ORDER`) |
| `FAILURE_NODE_LIST` | 1 | `FailureExpression` = 59, 不属任何类别 |
| `AST_STRING_CONSTANTS` + 单字符生成器 | 177 (128+49) | `AstStringConstant::COUNT` / `ONE_CHAR_COUNT` (`kMaxOneCharStringValue`) |
| Visitor `GENERATE_VISIT_CASE` | 60 | `walk_node` 分发臂 60/60 全命中 (测试实例化每变体并访问) |
| `FunctionSyntaxKind` (function-syntax-kind.h) | 5 | `FunctionSyntaxKind::COUNT` + FromRepr 对数 |
| `HandlerTable::CatchPrediction` | 5 | `CatchPrediction::from_repr(4)` 对数 |
| 赋值 Token 族 (Token::Value 赋值子集) | 16 | `AstAssignOp::COUNT` = 1 纯赋值 + 15 复合, `from_compound`/`binary_op` 双射对数 |

## 语义要点 / 偏差登记

- 节点 id: 上游 AstNode 无 id, 仅 `FunctionLiteral::function_literal_id_` (解析器分配, `kFunctionLiteralIdTopLevel = 0`)。本地以 `AstNodeFactory::next_function_literal_id` 计数器承接 (自 1 起, 顶层 0 保留), `FUNCTION_LITERAL_ID_UNASSIGNED = -1` 为本 crate 哨兵。
- 位置信息: 独立字段 `position: i32` + `bit_field: u32` (低 6 位 NodeTypeField), 与上游 AstNode 布局语义一致; `NO_SOURCE_POSITION = -1` / `FUNCTION_LITERAL_RETURN_POSITION = -2`。
- `AstRawString` 持字节副本 (非 zone 指针): 原版以 `const AstRawString*` 全序指针身份去重; Rust 以值语义 + 哈希字典去重, `PartialEq/Eq/Hash` 按内容 (含 `is_one_byte`), 比较语义 (`Ord`) 按码点序对照 `AstRawString::Compare`。
- `AstValueFactory::one_character_string` 越界 (>=128) 返回 `None`, 原版为 DCHECK 前置 (禁 panic 折衷)。
- FunctionLiteral 参数名: 上游由 `DeclarationScope::params()` (Variable*) 承载, 节点不存; 本地保留 `params: Vec<AstRawString>` 值语义快照 + `scope: Option<ScopeId>` 双轨, 解析器接入后以 scope 为主。
- `PreparseData` 为字节 + 子树的最小真实结构 (原版 ProducedPreparseData 持有者在 v8-parsing), 消费端随 v8-parsing 扩展。
- 作用域/变量 arena: `ScopeTree` 内部 `debug_assert` 前置 + 直接索引 (句柄仅由本树分配, 100% 有效; 跨树混用属调用方逻辑错误), `ScopeId::from_raw/to_raw` 显式口供测试与跨层构造。
- 重声明检测: 词法冲突返回 `Error::Redeclaration`; 豁免条件 = 已有与新增均为 `SloppyBlockFunctionVariable` (上游 web 兼容, bug 4693), 与上游 `DeclareVariableName` 的 `*ok` 语义逐条对照。
- `ContextChainLength` 为 `[this, scope)` 半开区间 (终点不计入), 与上游循环边界一致; `ContextHeaderLength` 按 `HasContextExtensionSlot` 取 2/3 (原版 Context::MIN_CONTEXT[_EXTENDED]_SLOTS)。
- receiver 声明条件 = `(function && !arrow) || module` (上游 `has_this_declaration`), script/eval 作用域不声明 this。
- Zone 分配路径: `AstNodeFactory` 三形态 (new/with_flags/with_zone); zone 形态的分配 API 全部 `Result` (v8-zone Error 透明转发), 无 zone 后端调用返回 `Error::NoZoneBackend`; 未触碰 v8-zone 的 `ZoneScope::reset`/`ZoneSnapshot::restore` 族。
- `AstNodeBase::node_type` 对越界位域回退 `FailureExpression` (原版 BitField::decode 为 UB 截断, 收紧为安全回退)。

## 测试映射

| 原测试 | Rust 测试 (`tests/main.rs`) | 说明 |
|---|---|---|
| `unittests/parser/ast-value-unittest.cc` AstRawStringAsArrayIndex | `test_ast_raw_string_as_array_index` | 边界 0 / 1234567 / 4294967294 / 溢出 / 前导 0 / 非数字 / 双字节 |
| `unittests/parser/ast-value-unittest.cc` BigIntToBooleanIsTrue | `test_bigint_to_boolean_is_true` | 0/0b0/0o0/0x0/前导零全 false, 非零全 true |
| `unittests/parser/ast-value-unittest.cc` GetStringNormalizesOneByteContent | `test_get_string_normalizes_one_byte_content` | 双字节 Latin-1 规范化单字节 |
| `cctest/test-ast.cc` | 未迁移 | 依赖解析产物 (ParseProgram), 随 v8-parsing 接入 |
| — (自建) | `test_node_type_dispatch_roundtrip_all_60` | 60 变体判别值/派发/名称三方对数 + 类别计数; 样本全走工厂构造器 (全覆盖佐证) |
| — (自建) | `test_visitor_walk_dispatch_full_coverage` | 60 变体实例化 + walk 分发臂全命中 |
| — (自建) | `test_node_field_completion` | 第二阶段字段补全对数: FunctionLiteral 全参构造 / FunctionSyntaxKind 5 值 / CatchPrediction / AstAssignOp 16 值双射 / ClassLiteral 完整成员 / VariableProxy bind_to / Assignment op |
| — (自建) | `test_zone_allocation_path` | 节点驻留 (alloc_node 可变返回) / ZoneList 增长 / ZoneVector 移动 push / 切片物化 / 辅助结构驻留 / NoZoneBackend 错误 / zone 记账非零 |
| — (自建) | `test_scopes_declaration_and_lookup` | 树构建 / let-var 重声明矩阵 (含 sloppy 豁免) / var 提升 / 沿链查找 / 动态全局回退 / catch 绑定 / 形参 / DeclarationScope 专属声明族 |
| — (自建) | `test_scopes_dynamic_lookup_fallbacks` | with 降级 LOOKUP / sloppy-eval 降级 DynamicGlobal / eval 标记向上传播 |
| — (自建) | `test_scopes_variable_allocation` | receiver Parameter(-1) / 形参索引 / 栈槽 / 未用不分配 / script 词法 context 槽 header=2 / catch THROWN_OBJECT_INDEX / eval 强制 context / context 链长 / 最近 context 外层 |
| — (自建) | `test_subclass_node_enums_conversion` / `test_ast_visitor*` / `test_class_literal_*` | 桥接转换/剪枝/先序后序/静态元素与新成员遍历 |
| — (自建) | `test_ast_string_constants_and_factory` / `test_variable_hole_check_bitmap_and_state` / `test_function_literal_id_allocation` | 常量表/空洞检测位图/id 分配 |

门禁: `cargo check` 0 错 / clippy `--all-targets -D warnings` 0 警告 / `cargo fmt --check` 干净 / nextest 23/23 全绿 (CARGO_TARGET_DIR 隔离); src 内 `todo!`/`panic!`/`unwrap` 零残留。

## 第三阶段拆块建议

1. prettyprinter 转写: `AstPrinter` 全量 (1.7k 行), 字段已补全可支撑打印断言对数。
2. scopes 补齐: `ScopeInfo` 反序列化 (`LookupInScopeInfo`/`DeserializeScopeChain`)、`AnalyzePartially`/`SavePreparseData` (preparse 数据双向)、`ModuleScope::AllocateModuleVariables` (对接 modules 槽位)。
3. source-ranges + id-reindexer + traversal-visitor 泛型层。
4. AstNode 集合字段的 zone 物化决策: 节点类型带 `'z/'a` 的全链路手术, 随 v8-parsing 的指针图需求评估 (当前值语义 + 节点驻留已覆盖构造期)。
5. `cctest/test-ast.cc` 与 `unittests/parser` 剩余用例迁移。

