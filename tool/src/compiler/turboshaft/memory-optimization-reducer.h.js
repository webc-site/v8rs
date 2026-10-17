#!/usr/bin/env bun
// tool/src/compiler/turboshaft/memory-optimization-reducer.h.js
// 机械提取 v8/src/compiler/turboshaft/memory-optimization-reducer.{h,cc}
// → v8-turboshaft/src/memory_optimization_reducer.rs.skel
//
// 提取面: AllocationFolding 枚举; MemoryAnalyzer 数据面 (BlockState/侧表);
// 分析器方法面 (Process* 系列); reducer REDUCE 面; 折叠/写屏障规则锚点。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/memory-optimization-reducer.h");
const cc = readV8("src/compiler/turboshaft/memory-optimization-reducer.cc");

// 1. AllocationFolding
const afM = h.match(/enum class AllocationFolding\s*\{([^}]+)\}/);
if (!afM) throw new Error("未找到 AllocationFolding");
const foldingKinds = [...afM[1].matchAll(/\bk(Do\w+|Dont\w+)\b/g)].map((m) => m[1]);
if (foldingKinds.length !== 2) throw new Error(`AllocationFolding ${foldingKinds.length} != 2`);

// 2. BlockState 字段
const bsM = h.match(/struct BlockState\s*\{([\s\S]*?)\n  \};/);
if (!bsM) throw new Error("未找到 MemoryAnalyzer::BlockState");
const bsFields = [...bsM[1].matchAll(/^\s*(?:const\s+)?([\w:<>,* ]+?)\s+(\w+_\w*)\s*(=|;)/gm)]
  .map((m) => `${m[1]} ${m[2]}`);
if (bsFields.length !== 2) throw new Error(`BlockState 字段 ${bsFields.length} != 2`);

// 3. 分析器方法面 (.cc 定义 + .h 声明)
const analyzerMethods = [...cc.matchAll(/^(?:void|bool) MemoryAnalyzer::(\w+)\(/gm)].map((m) => m[1]);
for (const must of ["Run", "Process", "ProcessAllocation", "ProcessStore", "MergeCurrentStateIntoSuccessor"]) {
  if (!analyzerMethods.includes(must)) throw new Error(`MemoryAnalyzer 缺 ${must}`);
}
const analyzerPredicates = [...h.matchAll(/^\s{2}(bool) (Is\w+|Skip\w+|ReservedSize)\(/gm)].map((m) => m[2]);

// 4. REDUCE 面
const reduces = [...h.matchAll(/\bREDUCE\((\w+)\)/g)].map((m) => m[1]);

// 5. 哨兵
for (const [re, name] of [
  [/kMaxRegularHeapObjectSize/, "fold 上限常量"],
  [/UnwrapAllocate/, "UnwrapAllocate"],
  [/MemoryAnalyzer::AllocationFolding::kDoAllocationFolding/, "折叠开关"],
  [/info->allocation_folding\(\)/, "折叠 flag 源 (OptimizedCompilationInfo)"],
]) {
  if (!(re.test(h) || re.test(cc))) throw new Error(`哨兵缺失: ${name}`);
}

let rust = banner(
  "src/compiler/turboshaft/memory-optimization-reducer.h.js",
  "src/compiler/turboshaft/memory-optimization-reducer.{h,cc}",
  "内存优化: 折叠分配与写屏障消除",
);

rust += `
/// 原版 MemoryAnalyzer::AllocationFolding — 分配折叠开关
/// v8: src/compiler/turboshaft/memory-optimization-reducer.h MemoryAnalyzer::AllocationFolding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocationFolding {
  ${foldingKinds[0]},
  ${foldingKinds[1]},
}

/// 原版 MemoryAnalyzer::BlockState — 每块折叠状态
/// v8: src/compiler/turboshaft/memory-optimization-reducer.h MemoryAnalyzer::BlockState
pub struct BlockState {
  /// last_allocation: 该块内最近一次 young 分配
  pub last_allocation: Option<&'static crate::AllocateOp>,
  /// reserved_size: 折叠预留字节数
  pub reserved_size: Option<u32>,
}

/// 原版 MemoryAnalyzer — 输入图分析面
/// v8: src/compiler/turboshaft/memory-optimization-reducer.{h,cc} MemoryAnalyzer
pub struct MemoryAnalyzer {
  /// 分配折叠状态表: FixedBlockSidetable<Option<BlockState>>
  pub block_states: Vec<Option<BlockState>>,
  /// folded_into: AllocateOp -> 折叠进的 AllocateOp
  pub folded_into: HashMap<AllocKey, AllocKey>,
  /// skipped_write_barriers: 可省写屏障的 store 集
  pub skipped_write_barriers: HashSet<OpIndex>,
  /// reserved_size: 每次分配的折叠预留
  pub reserved_size: HashMap<AllocKey, u32>,
  /// allocation_folding: 由 compilation info 的 allocation_folding() 决定
  pub allocation_folding: AllocationFolding,
  pub is_wasm: bool,
}

impl MemoryAnalyzer {
  /// 分析主循环 (cc): Process / ProcessBlockTerminator 分派
  pub fn run(&mut self) { /* v8: cc MemoryAnalyzer::Run */ }
  /// v8: cc MemoryAnalyzer::Process — 单 op 分派
  pub fn process(&mut self, op: &Operation) { /* ... */ }
  /// v8: cc MemoryAnalyzer::ProcessBlockTerminator — MergeCurrentStateIntoSuccessor
  pub fn process_block_terminator(&mut self, op: &Operation) { /* ... */ }
  /// v8: cc MemoryAnalyzer::ProcessAllocation — 建折叠链
  pub fn process_allocation(&mut self, alloc: &AllocateOp) { /* ... */ }
  /// v8: cc MemoryAnalyzer::ProcessStore — 判定写屏障可省
  pub fn process_store(&mut self, store: &StoreOp) { /* ... */ }
  /// v8: cc MemoryAnalyzer::MergeCurrentStateIntoSuccessor — 状态合并
  pub fn merge_current_state_into_successor(&mut self, succ: &Block) { /* ... */ }
}

/// 原版 reducer 面 (输出图改写):
${reduces.map((r) => `/// - REDUCE(${r})`).join("\n")}
/// - Allocate: young 且常量 size <= kMaxRegularHeapObjectSize 走折叠/快速路径
/// - LoadExternalPointer: sandbox 指针表解码
/// - MajorGCForCompilerTesting: 编译期触发 GC (调 runtime)
/// v8: src/compiler/turboshaft/memory-optimization-reducer.h MemoryOptimizationReducer
`;

const p = writeSrc("v8-turboshaft", "memory_optimization_reducer.rs.skel", rust);
console.log(`written ${p}`);
console.log(
  `AllocationFolding=${foldingKinds.length} BlockState字段=${bsFields.length} 分析方法=${analyzerMethods.length} 谓词=${analyzerPredicates.length} REDUCE=${reduces.length}`,
);
