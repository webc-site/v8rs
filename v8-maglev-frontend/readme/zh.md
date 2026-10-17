# v8-maglev-frontend — 台账

> Wave 9 席位 W9MF/W9MFb 接力 (target/wave9/W9MF.txt 25 原版文件)。
> 破环原则: 编译宿主 (MaglevCompilationInfo/Isolate/broker/feedback vector/IR 控制流面) 以 seam 承接;
> 深水逐原函数名登记「原函数名 / 前置条件 / 解锁 crate」三要素 (todo.md 12.6), 上游就位即重开。

## 一、就位面 (已挂载 + 门禁绿)

| 模块 | 原版文件 (本席) | 状态 |
| --- | --- | --- |
| `hamt.rs` | src/maglev/hamt.h | 全量挂载 + `unittests/maglev/hamt-unittest.cc` 16 例 1:1 |
| `graph.rs` | src/maglev/maglev-graph.{h,cc} | 挂载; arena/常量 intern/块位/sweep/计数器/stack-slots 数据面就位; 7 处深水见 §三 |
| `labeller.rs` | src/maglev/maglev-graph-labeller.{h,cc} | 挂载; 节点/块标签注册数据面就位; 打印族深水见 §四 |
| `frame_state.rs` | src/maglev/maglev-interpreter-frame-state.{h,cc} | 挂载; MergeDead 计数核 + 枚举/接缝就位; merge 状态机深水见 §四 |

依赖 (cargo add 落 Cargo.toml, 只读引用他席): `v8-maglev-ir` (Opcode/BLOCK_FLAG_IS_DEAD/is_constant_node) · `v8-interpreter` (Register) · `v8-zone` (Zone) · `gxhash` (HashMap/HashSet)。

## 二、转换脚本面核查 (tool/src/maglev/, 十一)

席位表内文件族对应脚本齐备、首行 `#!/usr/bin/env bun`、纯 ESM、只 import node 内置 + `../../lib.js`、`writeSrc` 直写本 crate src/、产物带 `/// v8:` 锚:

| 脚本 | 覆盖原版 (本席) | 产物 |
| --- | --- | --- |
| `hamt.h.js` | hamt.h | v8-maglev-frontend/src/hamt.rs |
| `maglev-graph.cc.js` | maglev-graph.{h,cc} | v8-maglev-frontend/src/graph.rs |
| `frontend.js` | graph-builder/reducer/inlining/loop-peeler/compilation-{info,unit}/interpreter-frame-state/known-node-aspects/kna-processor/map-inference | v8-maglev-frontend/src/frontend.rs.skel (草案, 未挂载 → §四) |
| `passes.js` | graph-optimizer/truncation/phi-representation-selector/cse/graph-processor/range*/post-hoc/pre-regalloc/pipeline-statistics/tracer/turbolev-escape-analysis/hamt | v8-maglev-frontend/src/passes.rs.skel (草案, 未挂载 → §四) |

注: 按十一「复跑仅限重生成基线, 修订就地改」, W9MFb 未复跑 `maglev-graph.cc.js`/`hamt.h.js` (graph.rs 已就地修订 usize/u32 对齐、`todo!` 深水印、must_use 收敛; 复跑会覆盖)。`labeller.rs`/`frame_state.rs` 为 W9MFb 依原版 Developer 手撰数据面 (§11「脚本产物非成品, 仍走 Developer 修订」), 无独立生成脚本 (非枚举/常量表族, 属语义体)。

## 三、graph.rs 残余 `todo!` 深水 (7 处, 逐原函数名 1:1)

| 原函数名 (file:line) | 前置条件 (缺什么) | 解锁 crate |
| --- | --- | --- |
| `Graph::RemoveUnreachableBlocks` (maglev-graph.cc:75-130) | ForEachNodeAndControl / GetLiveCatchBlock / ForEachSuccessor / RemovePredecessorAt 控制流宿主 + 节点输入边操作 | v8-maglev-ir (Control/BasicBlock 宿主) |
| `Graph::UnwrapDeoptFrames` (maglev-graph.cc:19-28) | EagerDeoptInfo::Unwrap / LazyDeoptInfo::Unwrap (DeoptFrame 展开) | v8-maglev-ir (DeoptFrame 宿主) |
| `Graph::is_osr` (maglev-graph.h:268-270) | compilation_info_->toplevel_compilation_unit()->is_osr() | v8-maglev-frontend builder 席 (MaglevCompilationInfo/Unit) + v8-interpreter |
| `Graph::MayNeedContextPhis` (maglev-graph.cc:155-167) | #ifdef DEBUG; is_osr + shared_function_info().kind() IsResumableFunction + inlined_functions_ | v8-execution-isolate + v8-objects-js-func (SharedFunctionInfo::kind) |
| `Graph::is_tracing_enabled` (maglev-graph.h:372-374) | compilation_info_->is_tracing_enabled() (tracer 旗标) | v8-maglev-frontend builder 席 (MaglevCompilationInfo) + v8-tracing |
| `Graph::compilation_info` (maglev-graph.h:383) | compilation_info_ 宿主返回 (zone()/broker() 全族委托源) | v8-maglev-frontend builder 席 (MaglevCompilationInfo) |
| `Graph::GetHeapNumberConstant` (maglev-graph.cc:30-45) | LocalIsolate NewHeapNumberFromBits + CanonicalPersistentHandle (bits 未 intern 建 HeapNumber) | v8-execution-isolate + v8-objects-strings + v8-compiler-broker |

## 四、建图前端 + 优化遍族深水 (frontend.rs.skel / passes.rs.skel, 不强挂)

两草案覆盖 21+ 原版文件 (graph-builder/reducer/inlining/loop-peeler/compilation-*/frame-state/kna/known-node-aspects/map-inference + optimizer/truncation/phi/CSE/range/escape/tracer/statistics)。全族依赖 Isolate/反馈向量/broker/IR 节点宿主, 属破环深水, 依 W9MFb 指令「依赖 Isolate/反馈向量的深水登记台账不强挂」。按 reducer 概念族 (WithKNA/WithEagerDeopt/…) 需 Rust trait bound 单态化派发, 逐文件另派。核心代表符号 (组|符号|前置|解锁 crate):

| 组 | 原函数名 (file) | 前置条件 | 解锁 crate |
| --- | --- | --- | --- |
| 建图 | `MaglevGraphBuilder::BuildGraph` (graph-builder.cc) | 字节码解码循环 + 逐 handler + MaglevCompilationInfo/Unit + feedback vector + KNA | v8-interpreter + v8-maglev-ir + v8-execution-isolate + v8-compiler-broker |
| 建图 | 逐 bytecode handler (graph-builder.cc `#include bytecode-*`) | 同上, 每 opcode 一个 handler | v8-maglev-ir + v8-interpreter + v8-objects-* |
| 帧态 | `MergePointInterpreterFrameState::New/NewForLoop/NewForCatchBlock/NewForPeel` (interpreter-frame-state.cc) | MaglevCompilationUnit + BytecodeLivenessState + LoopInfo + zone 分配 | v8-maglev-ir + v8-interpreter + v8-execution-isolate |
| 帧态 | `MergePointInterpreterFrameState::Merge/InitializeLoop/MergeThrow` (interpreter-frame-state.cc) | KnownNodeAspects + Graph Phi 建节点 + ContextScopeInfo merge 状态机 | v8-maglev-ir + v8-objects-maps (KnownNodeAspects/Map) |
| KNA | `KnownNodeAspects::Recompute/Constant/StaticTypeForMap` (known-node-aspects.cc) | JSHeapBroker + handle deref + map 推断 | v8-compiler-broker + v8-objects-maps + v8-heap-base |
| KNA | `RecomputeKnownNodeAspectsProcessor::Process` (kna-processor.cc) | Graph 节点遍历 + KnownNodeAspects 回滚 | v8-maglev-ir (graph-processor 框架) |
| 内联 | `MaglevInliner::Process` (maglev-inlining.cc) | CallLinkInfo feedback + OptimizedCompilationInfo + 嵌套 builder | v8-compiler-broker + v8-execution-isolate + v8-ic |
| 剥循环 | `MaglevLoopPeeler::Process` (maglev-loop-peeler.cc) | LoopInfo + 字节码复制 + 递归 builder | v8-interpreter + v8-maglev-ir |
| 优化 | `MaglevGraphOptimizer::ReduceGraph` (graph-optimizer.cc) | 遍序全链 + saved_kna_ 回滚 + LoopLabel | v8-maglev-ir + 上述全族 |
| 截断 | `PropagateTruncationProcessor`/`TruncationProcessor` (maglev-truncation.cc) | UseInfo/Rep 表示 + graph-processor 框架 | v8-maglev-ir + v8-compiler-ops |
| Phi 去标 | `MaglevPhiRepresentationSelector` (phi-representation-selector.cc) | UseRepresentation hint + Untagging/Retagging kind | v8-maglev-ir + repr 面 |
| CSE | `CSEProcessor` (maglev-cse.h) | 结构哈希 + Effect 位 (graph-processor 框架) | v8-maglev-ir |
| 区间 | `RangeProcessor`/`RangeVerification` (range-analysis/verification.cc) | NodeRanges + LessEqualConstraint + tracer | v8-maglev-ir + v8-compiler-ops |
| 逃逸 | `EscapeAnalysis::Process` (turbolev-escape-analysis.cc) | HAMT 不可变表 (已就位) + VirtualObject/InlinedAllocation 宿主 | v8-maglev-ir |
| 打印 | `MaglevGraphPrinter`/`PrintNode`/`PrintNodeLabel` (graph-printer.cc; labeller 打印族) | IR 节点结构反射 + std::ostream + regalloc 数据 | v8-maglev-ir + v8-maglev-backend (regalloc) |
| 校验 | `MaglevGraphVerifier` (graph-verifier.h) | 全节点类型/输入边校验 + Isolate debug | v8-maglev-ir |
| deopt 帧 | `MaglevDeoptFrameVisitor` (maglev-deopt-frame-visitor.h) | bytecode→frame 反遍历 + frame 参数/常量重建 | v8-maglev-ir + v8-interpreter + v8-objects-* |

## 五、tests 延期台账 (unittests/maglev, 12 例)

原用例名 · 前置 (fixture 依赖) · 解锁 crate。builder/processor fixture 均 `TestWithNativeContextAndZone` + `isolate()->AsLocalIsolate()`, 需 Isolate/native context/字节码运行面。

| 原用例名 (file) | 前置条件 | 解锁 crate |
| --- | --- | --- |
| `MaglevGraphBuilderTest.LoopEffectEpochComparison` (graph-builder-unittest.cc:23) | TestWithNativeContextAndZone + LocalIsolate + MaglevGraphBuilder + LoopEffects | v8-execution-isolate + v8-maglev-ir + 建图族 (§四) |
| `MaglevGraphBuilderTest.TrailingArgumentsRemoval` (:87) | 同上 + 字节码 + feedback vector | v8-execution-isolate + 建图族 |
| `MaglevGraphBuilderTest.UnusedArgumentsRemoval` (:120) | 同上 | v8-execution-isolate + 建图族 |
| `MaglevGraphBuilderTest.UnusedArgumentsRemovalHighIndex` (:153) | 同上 | v8-execution-isolate + 建图族 |
| `MaglevGraphBuilderTest.UnusedArgumentsRemovalWithTruncation` (:189) | 同上 + 截断遍 | v8-execution-isolate + 建图族 + §四截断 |
| `MaglevGraphBuilderTest.UnusedArgumentsRemovalSignExtension` (:227) | 同上 | v8-execution-isolate + 建图族 |
| `MaglevGraphProcessorTest.GraphBackwardMultiProcessorOrder` (graph-processor-unittest.cc:128) | TestWithNativeContextAndZone + 真图构造 + Processor 框架 | v8-maglev-ir (graph-processor 框架) + 建图族 |
| `MaglevGraphProcessorTest.LoopHeaderAvailableExpressions` (:162) | 同上 + 循环头 avail 集 | v8-maglev-ir + 建图族 |
| `MaglevGraphProcessorTest.SubgraphBufferStashingNodesFromBefore` (:191) | 同上 + subgraph buffer 暂存 | v8-maglev-ir + 建图族 |
| `MaglevGraphProcessorTest.SubgraphBufferStashingNestedParentAndChild` (:219) | 同上 | v8-maglev-ir + 建图族 |
| `MaglevGraphProcessorTest.SubgraphEmptyTopLevelWithNestedNonEmpty` (:239) | 同上 | v8-maglev-ir + 建图族 |
| `MaglevGraphProcessorTest.SubgraphAbruptExit` (:264) | 同上 | v8-maglev-ir + 建图族 |

> 非本席: `maglev-assembler-unittest.cc` (assembler 族 → v8-maglev-backend/codegen 席), `node-type-unittest.cc` (NodeType 族 → v8-maglev-ir 席, 已冻结)。
