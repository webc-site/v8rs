# v8-torque 阶段一：签名先行骨架

> 原版 `v8/src/torque` (~29k 行 C++) → Rust 自含编译器骨架。
> 生成链路：`tool/src/torque/*.js`（10 个组脚本，writeSrc 直出）→ `src/*.rs` → 门禁。
> 修订约定：**就地改 src**；复跑脚本会覆盖修订（仅限重生成基线），复跑后需再跑
> `cargo +nightly fmt -p v8-torque`。脚本本身确定性（复跑两次 diff 为空）。

## 一、脚本组 → 产出模块映射

| 脚本 | 覆盖 v8 原版文件 | 落位 Rust 模块 |
|---|---|---|
| `misc.js` | constants.h | `constants.rs` |
| | source-positions.h/.cc | `source_positions.rs` |
| | utils.h/.cc | `utils.rs` |
| | global-context.h/.cc | `global_context.rs` |
| | torque-compiler.h/.cc | `torque_compiler.rs` |
| | torque.cc | `torque.rs` |
| | kythe-data.h/.cc | `kythe_data.rs` |
| | server-data.h/.cc | `server_data.rs` |
| | layout-loader.h/.cc | `layout_loader.rs` |
| | runtime-support.h, runtime-macro-shims.h | `runtime_support.rs`, `runtime_macro_shims.rs`（占位登记） |
| `ast.js` | ast.h | `ast.rs` |
| `types.js` | types.h/.cc | `types.rs` |
| | type-oracle.h/.cc | `type_oracle.rs` |
| | type-visitor.h/.cc | `type_visitor.rs` |
| `declarable.js` | declarable.h/.cc | `declarable.rs` |
| | declarations.h/.cc | `declarations.rs` |
| | parameter-difference.h | `parameter_difference.rs` |
| | type-inference.h/.cc | `type_inference.rs` |
| `cfg.js` | instructions.h/.cc | `instructions.rs` |
| | cfg.h/.cc | `cfg.rs` |
| `parser.js` | earley-parser.h/.cc | `earley_parser.rs` |
| | torque-parser.h/.cc | `torque_parser.rs` |
| `visitor.js` | ast-visitor.h | `ast_visitor.rs` |
| | declaration-visitor.h/.cc | `declaration_visitor.rs` |
| | implementation-visitor.h/.cc | `implementation_visitor.rs` |
| `generators.js` | torque-code-generator.h/.cc | `torque_code_generator.rs` |
| | csa-generator.h/.cc | `csa_generator.rs` |
| | cc-generator.h/.cc | `cc_generator.rs` |
| | tsa-generator.h/.cc | `tsa_generator.rs` |
| | class-debug-reader-generator.cc | `class_debug_reader_generator.rs` |
| | instance-type-generator.cc | `instance_type_generator.rs` |
| | cpp-builder.h/.cc | `cpp_builder.rs` |
| `ls.js` | ls/globals.h | `ls/globals.rs` |
| | ls/json.h/.cc | `ls/json.rs` |
| | ls/json-parser.h/.cc | `ls/json_parser.rs` |
| | ls/message.h | `ls/message.rs` |
| | ls/message-handler.h/.cc | `ls/message_handler.rs` |
| | ls/message-macros.h | `ls/message_macros.rs`（占位登记） |
| | ls/message-pipe.h | `ls/message_pipe.rs` |
| | ls/torque-language-server.cc | `ls/torque_language_server.rs` |

同名族 .h/.cc 合并一个模块，.cc 部分以 `// ── 原版 xxx.cc ──` 分隔行衔接；
.cc 全量签名清单以 `// L<行号>: <原签名>` 注释保留（**填体正交任务源**，共 982 条：.cc 977 + .h 5）。

## 二、类型层覆盖统计（对数核对）

| 体系 | 原版 | Rust 建模 | 备注 |
|---|---|---|---|
| AST 节点 kind | AST_NODE_KIND_LIST 60 | `AstNodeKind` 60 / `AstNode` 60 变体 | 谓词 is_expression/is_type_expression/is_statement/is_declaration/is_callable 1:1 |
| 指令 | TORQUE_INSTRUCTION_LIST 25（后端无关 3 + 后端相关 22） | `InstructionKind` 25 / `Instruction` 25 | 逐指令 struct 25 个，字段机械提取 |
| Declarable | Declarable::Kind 12 | `DeclarableKind` 12 / `DeclarableData` 12 | Scope 存储抽 `ScopeData` |
| Type | TypeBase::Kind 7 | `TypeKind` 7 / `TypeData` 7 | Abstract/BuiltinPointer/Union/BitFieldStruct/Struct/Class/Top 全量 |
| ParseResult | ParseResultHolderBase::TypeId 57 | `ParseResultTypeId` 57 / `ParseResultValue` 57 | 载荷类型逐一映射 |
| 文法 token | torque-parser.cc `Token("…")` 68 | `GRAMMAR_TOKENS` 68 | 生成态保留 |
| 访问器 | AstVisitor<Derived> 60 节点 | `AstVisitor` trait 60 visit_* + visit_node/run 默认分派 | 唯一继承者 TsaGenerator 有 trait impl 骨架 |
| TypeOracle | 静态方法 ~170 | `impl TypeOracle` 74 方法（40+ get_x_type 机械生成） | 其余见 .cc 清单 |
| 常量 | constants.h 95 项 | `constants.rs` 95 | 字符串常量 + 位标志全量 |

## 三、骨架架构决策（继承树 → Rust 映射）

1. **C++ 虚继承树 → enum 判别 + 载荷 struct**：`AstNode`/`Instruction`/`TypeData`/
   `DeclarableData`/`ParseResultValue` 五棵树全部 enum 化；公共基类字段抽独立 struct
   （`NodeHeader`、`AggregateCommon`、`CallableCommon`、`MacroCommon`、`ValueCommon`）。
   指针互联一律 arena 索引：`NodeId`/`TypeId`/`DeclarableId`/`BlockId`/`SourceId`/
   `SymbolId`/`RuleId`/`ItemId`/`BindingId`/`InsnRef`。
2. **ContextualClass 上下文单例 → 驱动持有结构体**：GlobalContext / TypeOracle /
   SourceFileMap / KytheData / LanguageServerData / Logger / TorqueMessages /
   CurrentScope / CurrentSourcePosition 等，Rust 版一律显式传参（各偏差在模块头注释登记）。
3. **CRTP → trait**：`AstVisitor<Derived>` → trait + 默认 `visit_node` 分派 + 默认 `run`；
   `TorqueCodeGenerator` 虚函数 → 后端 struct 组合基类字段。
4. **模板去泛型**：`GenericDeclarable<S,D>` → 两个具体 Data；`Binding<T>` →
   `BindingValue` 两变体；`Stack<T>` 保留泛型。
5. **异常流 → Result**：`ReportError/Error(...).Throw()/TorqueAbortCompilation` →
   `Error::{Message, Abort}`（error.rs 集中）；未填语义函数统一 `Err(Error::Unimplemented)`
   或显式占位 + `// TODO(填体)`；禁 panic。
6. **C++ 生成 C++ → Rust 生成 Rust**：cpp::File/ostream 输出面保留（服务 cc-debug 与
   过渡期），后续 CSA 生成器产出目标改为 Rust 文本；`runtime-support.h` /
   `runtime-macro-shims.h` 无对应物，保留占位模块登记职责。
7. **容器**：std::map/set → BTreeMap/BTreeSet（确定性序，差分友好）；unordered_map →
   BTreeMap（骨架期，后续按 10.2 换 gxhash）。

## 四、填体正交任务清单（按依赖闭包切块）

每块独立可派发子代理；"依赖"为需先冻结的块。预计规模 ≈ 原版 .cc 体量。

| 块 | 内容（原版文件） | 预计行数 | 依赖 | 并发关系 |
|---|---|---|---|---|
| T1 | utils 语义：字符串工具族 / ResidueClass / FileUriDecode / 命名谓词（utils.cc） | ~400 | 无 | 可全并发 |
| T2 | earley 内核：Lexer::RunLexer / RunEarleyAlgorithm / ParseTokens / Item AST 重建 / Grammar 组合器补齐（earley-parser.cc） | ~350 | T1 | 与 T3 并发 |
| T3 | Torque 文法：TorqueGrammar::new 全文法构造 + ~150 产生式动作（torque-parser.cc，可按 类型/表达式/语句/声明/注解 切 5 个子块并发） | ~3000 | T2 | 子块间并发 |
| T4 | Declarable/Declarations 语义：Scope 查找链 / 各 Create/Declare / 约束计算（declarable.cc, declarations.cc） | ~520 | T3 | 与 T5 并发 |
| T5 | 类型系统语义：is_* 谓词 / CommonSupertype / SubtractType / Lower* / SizeOf + TypeOracle 查询族 + TypeVisitor 全部（types.cc, type-oracle.cc, type-visitor.cc） | ~2150 | T4 | 与 T6 并发 |
| T6 | CFG 语义：Instruction::type_instruction ×25 / RecomputeDefinitionLocations / CfgAssembler 栈操作族 / OptimizeCfg / ComputeInputDefinitions（instructions.cc, cfg.cc） | ~1050 | T5 | 与 T4 并发（T5 冻结签名后） |
| T7 | ImplementationVisitor 前半：求值 / GetLocationReference / GenerateCall 族 / 隐式转换（implementation-visitor.cc 前半） | ~1500 | T6 | 与 T8 串行 |
| T8 | ImplementationVisitor 后半：宏内联 / typeswitch 脱糖 / 控制流语句 / 类初始化（implementation-visitor.cc 后半） | ~1500 | T7 | — |
| T9 | 声明访问器：PredeclarationVisitor / DeclarationVisitor / 泛型特化（declaration-visitor.cc） | ~490 | T4、T5、T15(弱) | 与 T7 并发 |
| T10 | 生成器后端：TorqueCodeGenerator 公共 + CSA / CC 逐指令发射（torque-code-generator.cc, csa-generator.cc, cc-generator.cc） | ~1680 | T8 | csa/cc 两子块并发 |
| T11 | TSA 生成器 + InstanceType + ClassDebugReader（tsa-generator.cc, instance-type-generator.cc, class-debug-reader-generator.cc） | ~2900 | T10 | 三子块并发 |
| T12 | cpp-builder print 系列 + 文件落地（cpp-builder.cc） | ~170 | T10 | 随 T10 |
| T13 | 编译驱动：CompileTorque 全流程 / 参数解析 / GlobalContext 构造 / TargetArchitecture 尺寸（torque-compiler.cc, torque.cc, global-context.cc） | ~390 | T3-T9 | 最后汇聚 |
| T14 | LS 组：JSON 解析/序列化 / message_handler 分派 / pipe 帧 / 主循环（ls/*） | ~700 | T13 | 与 T10-T12 并发 |
| T15 | Kythe + ServerData + LayoutLoader（kythe-data.cc, server-data.cc, layout-loader.cc） | ~2080 | T5, T13 | 两子块并发 |

填体红线：不改签名；每块完成后跑 crate 级门禁（fmt/clippy/test）；
对应原版 .cc 签名清单在各模块尾部注释（`// L<行>:` 前缀），逐条销号。

T9 对 T15 为弱依赖（签名冻结即可，不构成 DAG 硬边）：declaration-visitor.cc 仅调用
`KytheData::AddFunctionDefinition` / `LanguageServerData::AddDefinition` 等记录器
（declaration-visitor.cc:206-467，include 见 ：10-11），这些签名已在骨架冻结；
若作硬依赖，则 T9→T15→T13→T9 成环。T15 保留对 T13 的硬依赖有据：
layout-loader.cc:593-597 取 `TargetArchitecture` 尺寸（语义在 T13 的 global-context.cc）。

## 五、预期依赖登记（填体阶段引入，骨架期零新增）

- `gxhash::{HashMap, HashSet}` — 替换骨架期 BTreeMap/BTreeSet 中非语义有序场景
  （todo.md 十.2 横切；语义有序处保留 BTree）。
- `log` — 升 dev 依赖为运行时日志（ls/globals.rs Logger、编译过程 trace）。
- `strum` — 枚举 EnumString/IntoEnumIterator（constants 位标志、AST kind ↔ 名字转换）。
- `enum_dispatch` — 评估 `Instruction`/`AstNode` 分派热点（先基准再引）。
- `phf` — `GRAMMAR_TOKENS`/常量静态表（若编译期查找成为热点）。
- `thiserror` 已就位（error.rs）。

## 六、复现与门禁

```sh
# 基线重生成（会覆盖 src 修订; 复跑两次 diff 为空; assemble 必须最后跑）
for f in ast cfg declarable generators ls misc parser types visitor; do bun tool/src/torque/$f.js; done
bun tool/src/torque/assemble.js   # lib.rs / ls.rs 装配 (扫 pub 项生成导出清单, lib.rs 实际导出 494 项)
cargo +nightly fmt -p v8-torque   # 复跑脚本后必须重 fmt (脚本产物未格式化)
cargo +nightly clippy -p v8-torque --all-features --all-targets -- -D warnings -W clippy::absolute_paths
cargo test -p v8-torque
```
