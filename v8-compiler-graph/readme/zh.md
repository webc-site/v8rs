# v8-compiler-graph

V8 `src/compiler` TurboFan IR 基础层 (第一梯队) 的 Rust 转写。原版 8 文件族约 4.3k 行 C++ (opcodes.h、operator.h/cc、operator-properties.h/cc、node.h/cc、node-properties.h/cc、turbofan-graph.h/cc、use-info.h), 补登 bytecode-graph-builder.h/cc 建图纯面子集; Rust 侧 src 约 5.6k 行 + tests 约 1.7k 行, 全部由 `tool/src/compiler/` 定制转换脚本生成基线后就地修订。

内存模型: Node 走 v8-zone bump 分配 + Use 前缀布局 1:1 (inputs 内联于 Node 后 / 溢出走 OutOfLineInputs, Use 双向链); Node 可变字段包 `UnsafeCell` (共享视图 `&Node` 下合法写, 对应原版 const 方法改内存)。Graph 借用 `&'z Zone<'a>` (1:1 原版 `TFGraph(Zone*)`), 节点生命周期同 zone。

## 转换脚本 (tool/src/compiler/, 复跑 = 重生成基线 → cargo fmt)

| 脚本 | 原版来源 | 产出 | 内置对数 |
|---|---|---|---|
| `opcodes.h.js` | `src/compiler/opcodes.h` + `opcodes.cc` + `operator-properties.cc` (谓词表) | `src/opcodes.rs` | X-macro 树全展开: **变体总数 1036** (C 预处理器 `cpp` 金标准交叉核对 13 项计数全一致: control=22 common=47 constant=13 inner=27 mconst=8 simplified=322 machine=234 simd128=287 js=124 atomic=27 mcmp=16 scmp=20 jscmp=6); 目标构建 wasm 开/SIMD128 开/SIMD256 关/硬件沙箱关/CPED 开; 变体唯一性、区间谓词 ⟺ 表位全量一致、NeedsExactContext JS 124 码四组互斥分区 69/30/2/23、HasFrameStateInput 真组 92+参数化 1 (JSCallRuntime) |
| `operator.js` | `src/compiler/operator.h` + `operator.cc` | `src/operator.rs` | Property 7 位值 + kFoldable/kEliminatable/kKontrol/kPure 组合逐位一致; OPERATOR_PROPERTY_LIST 打印序 |
| `operator-properties.js` | `src/compiler/operator-properties.h` + `.cc` | `src/operator_properties.rs` | 6 项哨兵 (case 分组/GetTotalInputCount 公式逐字); 分区数据由 opcodes.h.js 保证 |
| `node.js` | `src/compiler/node.h` + `node.cc` | `src/node.rs` | 11 项布局哨兵 (Use 前缀步长/GetUsePtr 逆序/capacity 下限 1/余量 +3/OOL 扩容 2n+3/位域宽) + 20 函数转写面清点; Rust const 断言 Node=24B/Use=24B/OOL=16B、kOutlineMarker=15/kMaxInlineCapacity=14 |
| `node-properties.js` | `src/compiler/node-properties.h` + `.cc` | `src/node_properties.rs` | 布局公式 10 条逐字 + 转写面 23 函数清点 |
| `turbofan-graph.js` | `src/compiler/turbofan-graph.h` + `.cc` | `src/graph.rs` | 8 项哨兵 (NextNodeId 自增/工厂委托/SubgraphScope 还原/NodeCount 公式) |
| `use-info.js` | `src/compiler/use-info.h` + `representation-change.cc:24-145` (Truncation 格) + `compiler/globals.h` (CheckForMinusZeroMode) + `codegen/machine-type.h` (最小子集) + `feedback-source.h` (占位) | `src/use_info.rs` + `src/machine_type.rs` | 变体序提取 MachineRepresentation=21/MachineSemantic=11/TruncationKind=7/TypeCheckKind=13; Truncation 格 14 项哨兵 |
| `bytecode-graph-builder.js` | `src/compiler/bytecode-graph-builder.h` + `.cc` (建图纯面子集) | `src/build_graph.rs` | 结构锚 12 必现 (入口 `BuildGraphFromBytecode` 声明 / `broker->target_native_context()` / CreateGraph 头尾 SetStart·SetEnd 路 / 扩容公式逐字) + 对数断言 10 字面计数 (`native_context_node_` 面 6 处; 缓冲字段对各 1; `fill_n`+`buffer[count]=control`+`NewNode(phi_op,…,true)` Phi/EffectPhi 同形各 2) + 行号锚 14 项逐锚核对本 checkout 定位行号 (384/1183/1193/1207/1211/4671/4673/4798/4801/4802/4803/4806/4810/4811/4812/4885/4894, 复跑即校验, 防行号漂移) |

## 模块映射

| 原版 (v8/src/compiler) | Rust 模块 | 说明 |
|---|---|---|
| `opcodes.h` + `opcodes.cc` | `src/opcodes.rs` | IrOpcode 枚举 1036 变体 (repr(u16), 判别值=原版枚举序); `OPCODE_PROPS` 位表 21 位 (CONTROL/COMMON/…/EXACT_CONTEXT_*); 区间谓词 is_common/is_control/is_js/is_machine + 位表谓词; mnemonic 走 strum IntoStaticStr; `from_repr_unchecked` 供热路径 (`Node::opcode`) |
| `operator.h` + `operator.cc` | `src/operator.rs` | Operator (opcode/Properties 位/mnemonic/端口计数) + `OperatorCounts` 收敛构造参数 (就地修订) + Operator1<T> 单参数模板; ZeroIfEliminatable/NoThrow/Pure const fn; 原版 CHECK/CheckRange → `Result` |
| `operator-properties.h` + `.cc` | `src/operator_properties.rs` | HasContextInput/HasFrameStateInput/GetTotalInputCount/IsBasicBlockBegin/NeedsExactContext; 谓词 opcode 面在 IrOpcode 位表, 参数化分支 (JSCallRuntime/JSCreateArguments) `Err(NeedsOpParam)` 推迟 |
| `node.h` + `node.cc` | `src/node.rs` | Node/Use/OutOfLineInputs/Edge + InputEdges/Inputs/UseEdges/Uses 迭代器; zone bump + Use 前缀布局; AppendInput/ReplaceInput/RemoveInput/InsertInput(s)/TrimInputCount/EnsureInputCount/ReplaceUses/OwnedBy(2)/Kill/Clone/Verify(debug) |
| `node-properties.h` + `.cc` | `src/node_properties.rs` | first/past index 族 + 输入访问器 + 边类别 + 替换器族 + ReplaceUses/CollectControlProjections/FindFrameStateBefore/FindSuccessfulControlProjection/GetOuterContext/IsSame/Equals/HashCode |
| `turbofan-graph.h` + `.cc` | `src/graph.rs` | Graph (借用 zone, Cell 发号器/装饰器 Vec<Rc<dyn GraphDecorator>> 依数据指针身份增删) + SubgraphScope RAII; mark_max 保留 + reserve_mark_domain 标记域轮转分配 (NodeMarker 第二梯队) |
| `use-info.h` (+ `representation-change.cc` Truncation 格) | `src/use_info.rs` | UseInfo/Truncation/IdentifyZeros/TypeCheckKind/CheckForMinusZeroMode; 格上 Generalize/LessGeneral 全量迁移 (不相容组合原版 FATAL → Err) |
| `codegen/machine-type.h` (最小子集) | `src/machine_type.rs` | MachineRepresentation(21)/MachineSemantic(11)/MachineType + 常用构造 + PointerRepresentation (64 位 Word64); 完整迁移属 codegen 层, 后续迁出 |
| `bytecode-graph-builder.h` + `.cc` (建图纯面) | `src/build_graph.rs` | 落地 `InputBuffer::ensure` (EnsureInputBufferSize cc:4671, 扩容公式 `需求+64+旧容量`) / `create_start`·`create_end` (CreateGraph Start/End 骨架 cc:1191-1193/1207-1211, DCHECK 非空→Err) / `new_phi` (NewPhi cc:4798 与 NewEffectPhi cc:4806 同形合一, count 份输入+尾置 control, incomplete 建节点); `common()->Start/End/Phi/EffectPhi` 算子构造属 v8-compiler-ops, 按 crate 惯例作参注入 |
| — (手写) | `src/error.rs` / `src/lib.rs` | 原版 CHECK/FATAL/UNREACHABLE → `Error` 集中映射; 对外导出收敛 |

依赖: v8-zone (bump 分配) / v8-base (BitField32, hash_combine) / strum (枚举) / thiserror / log。dev: aok / ctor / log_init。

## 测试映射 (82/82 通过)

| 原测试文件 | Rust 测试文件 | 用例 | 说明 |
|---|---|---|---|
| `test/unittests/compiler/node-unittest.cc` | `tests/node_unittest.rs` | 23 | New/NewWithInputs/InputIterator×3/UseIterator×3/OwnedBy/ReplaceUsesNone/AppendInput/TrimThenAppend/BigNodes(512 外联) 全量 1:1; 补 RemoveInput/InsertInput(s)/ReplaceInput/EnsureInputCount/Kill-IsDead/BranchUseCount/Clone/zone 复用/Display/Operator1/Mark 语义 |
| `test/unittests/compiler/opcodes-unittest.cc` | `tests/opcodes_unittest.rs` | 13 | IsCommon/IsControl/IsJs/IsConstant/IsComparison/Mnemonic 全量遍历 (表位 ⟺ 区间交叉); 补总数/判别值/构建配置抽样/小谓词/FrameState 92 计数/ExactContext 分区 69-30-2-23/顶层段计数/位正交 |
| `test/unittests/compiler/node-properties-unittest.cc` | `tests/node_properties_unittest.rs` | 15 | ReplaceUses 四路 + CollectControlProjections Branch/Call/Switch; 补布局 index 族/访问器/边类别/替换器族/控制投影/IsSame/Equals-Hash/OuterContext/FindFrameStateBefore/IsValueIdentity/谓词族 |
| — (原测试需要 operator 参数/对象层) | `tests/operator_unittest.rs` | 12 | Properties 位与组合/ZeroIf*/构造计数与校验/props_str; OperatorProperties: context/frame_state/total(6-5-2)/basic_block_begin(11)/needs_exact_context 四组+Err 面 |
| — (原版无官方单测) | `tests/graph_unittest.rs` | 9 | NewNode 唯一 id/NodeCount/NewNodeUnchecked/装饰器触发-移除(身份)/CloneNode/SubgraphScope RAII/simd 面/Mark 域/zone 访问 |
| — (原版无官方单测) | `tests/use_info_unittest.rs` | 7 | Truncation 格偏序/Generalize 规则与 Err/±0 识别/查询面与 description/UseInfo 构造面/minus_zero_check/相等与 hash/FeedbackSource 占位/MachineType 子集 |
| — (原测试需 broker/字节码, 未就位) | `tests/build_graph_unittest.rs` | 3 | 建图不变量: InputBuffer 扩容公式逐字 (10→74→218, 够用零重分配)/CreateGraph Start-End 骨架 (发号连续/空 exit_controls Err 不消费 id/输入边序/use 链各 1)/new_phi (count 份输入+尾置 control/use 链 3+1/缓冲复用不重分配) |
| — (骨架) | `tests/main.rs` | 1 | 冒烟 |

## 语义偏差登记

- Operator 构造收敛为 `(opcode, properties, mnemonic, OperatorCounts)` (就地修订; 原版 9 位置参数); opcode 域校验 [0, LAST_U16] 为 Rust 加强 (原版仅 DCHECK)。
- `Operator1<T>` 以组合代继承; 原版 `OpParameter<T>(op)` unchecked 下取不迁移 — 参数经强类型 `&Operator1<T>` 直取; ClassTag (DEBUG 构建) 不迁移。
- Graph 的 zone 为借用 (`Graph::new(&zone)`); 装饰器 `Vec<Rc<dyn GraphDecorator>>` (原版 ZoneVector<GraphDecorator*>, Box 无法保身份删除); simd_stores/decorators 用 std Vec (zone 容器自引用不可行)。
- 原版 CHECK/DCHECK/FATAL/UNREACHABLE → `Result<_, Error>` (错误集中 `src/error.rs`); debug_assert 保留纯断言面。
- Node::New 的 null 输入 FATAL 分支由 `&Node` 类型天然消除 (Clone/Insert 族内部路径仍可产生 null 槽, `input_at` 返回 Option)。
- Node 的 `type_` 字段与 Print(std::ostream) 深度展示推迟 (turbofan-types 层)。
- `InputBuffer`: 原版 `Node**` 裸槽 + `EnsureInputBufferSize` 返回基址; Rust 侧槽类型化为 `&'z Node` (非空由类型保证), ensure 不返回裸指针, 经 fill/set_at/view 读写 (view 契约 = 原版"先写后递给 NewNode"同一调用方纪律, 越界 debug_assert)。zone 择源偏差: 原版 `local_zone()`, Rust `new_phi` 走 `graph.zone()` — 输入缓冲为建节点瞬时缓冲, 图形无差别, 同为无释放 bump 纪律。
- `new_phi`: NewPhi/NewEffectPhi 两原版函数仅算子构造点之差, 合一落地, 算子作参 (原版 `common()->Phi(kTagged,n)`/`EffectPhi(n)`); `create_start`/`create_end` 同理 (`common()->Start(arity)`/`End(n)` 与 arity 公式属 v8-compiler-ops)。
- CreateGraph 尾部 DCHECK_NE(0u, exit_controls_.size()) → `Err(CountOutOfRange)` (crate CHECK 映射惯例)。

## 推迟登记 (第二梯队 / v8-compiler-ops)

- `bytecode-graph-builder.cc` 建图主体 (台账名 NativeContext::BuildGraph 锚定单元) 硬耦合面 → **v8-compiler-frontend** (todo.md 看板归属, 需 JSHeapBroker/JSGraph/BytecodeAnalysis/interpreter 字节码面): `BytecodeGraphBuilder` 构造与 `CreateGraph` 全链 (cc:1059-1210)、`Environment`/依赖追踪 (cc:552-1116)、`MakeNode` context/frame-state/effect/control 布线与隐式异常延续 (cc:4707-4796)、`VisitBytecodes`/`Visit*` 字节码 handler 族 (cc:1473 起)、`ExitThenEnterExceptionHandlers` (cc:4679, HandlerTable)、`CreateNativeContextNode`/`BuildLoadNativeContextField`/`CreateFeedbackVectorNode` (cc:1147-1166, jsgraph 常量节点需 broker)、`GetParameter`/`cached_parameters_` (cc:1123-1145, 需 ParameterInfo)、`MergeControl`/`MergeEffect`/`MergeValue` (cc:4815-4870, 需 common()->Loop/Merge/Phi 参数算子重建)、OSR/peeled loops/`AdvanceIteratorsTo` (cc:1268-1471)、node_origins/source_positions 表 (装饰器面)。
- `common-operator.h` `StartNode::OutputArityForFormalParameterCount` (cc:775) 与 `common()->Start/End/Phi/EffectPhi/Loop/Merge` 构造 → v8-compiler-ops (本 crate 的 `create_start`/`create_end`/`new_phi` 已按算子作参预留调用位)。
- `common-operator.h` / `machine-operator.h` / `simplified-operator.h` / `js-operator.h` → v8-compiler-ops crate (opcodes.rs 位表为其地基)。
- `verifier.h` (NewNode 的 VerifyNode 校验), `turbofan-graph-visualizer` (Graph::Print/AsRPO)。
- `node-marker.h` (NodeMarker/NodeMarkerBase, mark_max 单调域接管), `node-cache.h` / `node-matchers.h` / `aux-data.h` / `schedule.h`。
- NodeProperties: FindProjection/CollectValueProjections (需 Operator1<size_t> 参数)、MergeControlToEnd/RemoveControlFromEnd (需 CommonOperatorBuilder)、GetProjectionType (需 CallDescriptor/MachineType 全量)、InferMapsUnsafe/GetJSCreateMap/CanBePrimitive/CanBeNullOrUndefined (需 JSHeapBroker/MapInference)、Type 族 (需 turbofan-types)。
- OperatorProperties: JSCallRuntime 分支 (Runtime::NeedsExactContext / Linkage::NeedsFrameStateInput, 需运行时函数表)。
- `representation-change.cc` 其余部分 (需 simplified/machine operator 层); FeedbackSource 对象层升级 (feedback-vector)。
- `codegen/machine-type.h` 完整迁移 (ElementSize*/MachineReprToString 等, 属 codegen 层)。

门禁: clippy `--all-targets -D warnings` 0 条; nextest 82/82。

开发日志与台账锚定裁定: 见 ../../log/v8-compiler-graph.md
