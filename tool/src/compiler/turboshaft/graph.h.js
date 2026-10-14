#!/usr/bin/env bun
// tool/src/compiler/turboshaft/graph.h.js
// 机械提取 v8/src/compiler/turboshaft/graph.{h,cc}
// → v8-turboshaft/src/graph.rs
//
// 提取面: Block::Kind / Block::CustomDataKind / Graph::Origin 三枚举 + kind 名表
// (graph.cc operator<<) + 常量表 (kInvalidPredecessorIndex/kMinCapacity/初始容量/
// SLOTS_PER_ID 换算) + OperationBuffer/Block/Graph 结构体字段与方法签名骨架。
// 语义实现禁编造, 方法仅登记签名 (unimplemented 骨架)。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/graph.h");
const cc = readV8("src/compiler/turboshaft/graph.cc");

// ---------------------------------------------------------------------------
// 1. 枚举提取
// ---------------------------------------------------------------------------
function enumVariants(re, label, expectCount) {
  const m = h.match(re);
  if (!m) throw new Error(`未找到 ${label}`);
  const body = m[1].replace(/\/\/[^\n]*/g, "");
  const names = [...body.matchAll(/k(\w+)\s*(?:=[^,}]+)?/g)].map((x) => x[1]);
  if (expectCount && names.length !== expectCount) {
    throw new Error(`${label} ${names.length} != ${expectCount}`);
  }
  return names.map((n) => ({ raw: `k${n}`, name: n }));
}

const blockKinds = enumVariants(/enum class Kind : uint8_t\s*\{([^}]+)\}/, "Block::Kind", 3);
const customKinds = enumVariants(/enum class CustomDataKind\s*\{([^}]+)\}/, "Block::CustomDataKind", 3);
const origins = enumVariants(/enum class Origin\s*\{([^}]+)\}/, "Graph::Origin", 4);

// kind 名表 (graph.cc operator<<): kLoopHeader→LOOP, kMerge→MERGE, kBranchTarget→BLOCK
const kindStrMatch = cc.match(/operator<<[\s\S]*?Block::Kind& kind\)\s*\{([\s\S]*?)\n\}/);
if (!kindStrMatch) throw new Error("未找到 Block::Kind operator<<");
const kindStrMap = new Map();
for (const m of kindStrMatch[1].matchAll(/case Block::Kind::k(\w+):\s*return os << "(\w+)";/g)) {
  kindStrMap.set(m[1], m[2]);
}
if (kindStrMap.size !== 3) throw new Error(`Kind 名表 ${kindStrMap.size} != 3`);

// ---------------------------------------------------------------------------
// 2. 常量提取 + 哨兵核对
// ---------------------------------------------------------------------------
function numConst(re, label, expect) {
  const m = h.match(re);
  if (!m) throw new Error(`未找到 ${label}`);
  if (expect !== undefined && Number(m[1]) !== expect) throw new Error(`${label}=${m[1]} 预期 ${expect}`);
  return Number(m[1]);
}
const invalidPredIndex = numConst(/static constexpr int kInvalidPredecessorIndex = (-?\d+);/, "kInvalidPredecessorIndex", -1);
const minBlockCapacity = numConst(/constexpr size_t kMinCapacity = (\d+);/, "kMinCapacity", 32);
const initialCapacity = numConst(/initial_capacity = (\d+)\)/, "Graph 初始容量", 2048);

// 哨兵: 关键结构存在性
const sentinels = [
  [/class OperationBuffer/, "OperationBuffer"],
  [/class Block : public RandomAccessStackDominatorNode<Block>/, "Block 基类链"],
  [/class Graph\s*\{/, "Graph"],
  [/OperationStorageSlot\* Allocate\(size_t slot_count\)/, "OperationBuffer::Allocate"],
  [/uint32_t ComputeDominator\(\);/, "Block::ComputeDominator"],
  [/BlockIndex BlockIndexOf\(OpIndex op\) const/, "Graph::BlockIndexOf"],
  [/void KillOperation\(OpIndex i\)/, "Graph::KillOperation"],
];
for (const [re, name] of sentinels) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// Graph 字段面 (private 段)
const graphFields = [
  ["origin", "Origin", "origin_ — 图来源"],
  ["operations", "OperationBuffer", "operations_ — 操作缓冲"],
  ["bound_blocks", "Vec<Block>", "bound_blocks_ — 已绑定块表"],
  ["all_blocks", "Vec<Block>", "all_blocks_ — 全块存储 (含未绑定)"],
  ["op_to_block", "SideTable<OpIndex, BlockIndex>", "op_to_block_"],
  ["block_permutation", "Vec<BlockIndex>", "block_permutation_"],
  ["graph_zone", "Zone", "graph_zone_"],
  ["source_positions", "Sidetable", "source_positions_"],
  ["operation_origins", "Sidetable", "operation_origins_"],
  ["operation_types", "Sidetable", "operation_types_"],
  ["stack_checks_to_remove", "Vec<OpIndex>", "stack_checks_to_remove_"],
];
for (const [, , comment] of graphFields) {
  const en = comment.split(" — ")[0]; // 前半段为原版字段名
  if (!h.includes(en)) throw new Error(`Graph 字段缺失: ${en}`);
}

// Graph 方法签名面 (登记清单; 禁编造语义)
const graphSignatures = [
  ["get", "Get(OpIndex) -> &Operation", "按 OpIndex 取操作 (opcode 越界 DCHECK)"],
  ["kill_operation", "KillOperation(OpIndex)", "替换为 DeadOp"],
  ["start_block", "StartBlock() -> &Block", "块 0"],
  ["get_block", "Get(BlockIndex) -> &Block", "按 BlockIndex 取已绑定块"],
  ["index", "Index(&Operation) -> OpIndex", "操作 → 索引"],
  ["block_of", "BlockOf(OpIndex) -> BlockIndex", "索引 → 所属块 (begin 上界二分)"],
  ["set_block_of", "SetBlockOf(BlockIndex, OpIndex)", "登记 op→block"],
  ["block_index_of", "BlockIndexOf(OpIndex) -> BlockIndex", "查 op_to_block 表"],
  ["next_index", "NextIndex(OpIndex) -> OpIndex", "缓冲内后继"],
  ["previous_index", "PreviousIndex(OpIndex) -> OpIndex", "缓冲内前驱"],
  ["next_operation_index", "next_operation_index() -> OpIndex", "EndIndex 别名"],
  ["next_block_index", "next_block_index() -> BlockIndex", "bound_blocks_.size()"],
  ["block_count", "block_count() -> u32", "已绑定块数"],
  ["op_id_count", "op_id_count() -> u32", "(size + kSlotsPerId-1) / kSlotsPerId"],
  ["op_id_capacity", "op_id_capacity() -> u32", "capacity / kSlotsPerId"],
  ["reset", "Reset()", "回收复用 (跨 phase 内存循环)"],
  ["allocate_new_blocks", "AllocateNewBlocks()", "块存储扩容 (kMinCapacity=32, ×2)"],
  ["compute_dominator_block", "Block::ComputeDominator() -> u32", "前驱支配集已算前提下求本块支配深度"],
  ["print_dominator_tree", "Block::PrintDominatorTree()", "支配树 ASCII 打印 (graph.cc)"],
];
for (const [, sig] of graphSignatures) {
  const first = sig.split("(")[0];
  const camel = first
    .split("_")
    .map((w, i) => (i === 0 ? w : w ? w[0].toUpperCase() + w.slice(1) : w))
    .join("");
  if (!h.includes(first) && !cc.includes(first) && !h.includes(camel) && !cc.includes(camel)) {
    throw new Error(`签名缺失: ${first}`);
  }
}

// ---------------------------------------------------------------------------
// 3. 生成 Rust
// ---------------------------------------------------------------------------
let rust = banner(
  "src/compiler/turboshaft/graph.h.js",
  "src/compiler/turboshaft/graph.{h,cc}",
  "Turboshaft 图结构: OperationBuffer/Block/Graph 枚举、常量与签名骨架",
);

rust += `
use crate::index::{BlockIndex, OpIndex, OperationStorageSlot, SLOTS_PER_ID};

/// 原版 Block::Kind — 块种类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BlockKind {
`;
blockKinds.forEach((v, i) => {
  rust += `  /// ${v.raw}\n  ${v.name} = ${i},\n`;
});
rust += `}

impl BlockKind {
  /// 原版 operator<< 名 (graph.cc)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
`;
blockKinds.forEach((v) => {
  rust += `      BlockKind::${v.name} => "${kindStrMap.get(v.name)}",\n`;
});
rust += `    }
  }
}

/// 原版 Block::CustomDataKind — 块级临时数据种类 (算法间约定, 跨图不保留)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CustomDataKind {
`;
customKinds.forEach((v, i) => {
  rust += `  /// ${v.raw}${i === 0 ? " — 未设置" : ""}\n  ${v.name} = ${i},\n`;
});
rust += `}

/// 原版 Graph::Origin — 图来源
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Origin {
`;
origins.forEach((v, i) => {
  rust += `  /// ${v.raw}\n  ${v.name} = ${i},\n`;
});
rust += `}

/// 原版 kInvalidPredecessorIndex — 非直接前驱时返回的哨兵
pub const INVALID_PREDECESSOR_INDEX: i32 = ${invalidPredIndex};

/// 原版 Block 扩容下限 (AllocateNewBlocks 内 kMinCapacity)
pub const MIN_BLOCK_CAPACITY: usize = ${minBlockCapacity};

/// 原版 Graph 默认初始容量 (操作槽数; 跨 phase 内存复用的关键)
pub const DEFAULT_INITIAL_CAPACITY: usize = ${initialCapacity};

/// 原版 Block — 基本块 (RandomAccessStackDominatorNode 支配树节点)
///
/// 字段面登记 (原版 Block 私有域):
/// - kind: BlockKind
/// - begin/end: OpIndex (块内操作区间, Invalid 未绑定)
/// - index: BlockIndex (Invalid 未绑定)
/// - last_predecessor / neighboring_predecessor: 前驱链表 (逆序挂链)
/// - single_loop_predecessor: 环回前驱专用槽
/// - predecessor_count: u32
/// - origin: 优化前块来源 (DEBUG 追溯)
/// - custom_data: u32 (算法临时数据)
pub struct Block {
  /// kind_
  pub kind: BlockKind,
  /// begin_
  pub begin: OpIndex,
  /// end_
  pub end: OpIndex,
  /// index_
  pub index: BlockIndex,
  /// predecessor_count_
  pub predecessor_count: u32,
  /// custom_data_
  pub custom_data: u32,
}

/// 原版 OperationBuffer — 操作顺序存储 (双向迭代: 尺寸记于首末两个 id)
///
/// 字段面: begin/end/end_cap 槽指针; operation_sizes: u16 表
/// ((capacity+1)/kSlotsPerId 项)。
/// 方法签名面:
/// - Allocate(slot_count) -> 槽指针 (余量不足先 Grow)
/// - RemoveLast() — 回滚末操作 (读末 id 尺寸)
/// - Index(ptr) -> OpIndex — 指针 → 偏移索引
/// - Get(OpIndex) -> 槽指针 / Next / Previous / BeginIndex / EndIndex
/// - SlotCount(OpIndex) -> u16 / Reset / capacity / size / Grow
pub struct OperationBuffer {
  /// operation_sizes_ — 首/末 id 双写尺寸
  pub operation_sizes: Vec<u16>,
}

/// 原版 Graph — Turboshaft 图属主 (跨 phase 复用内存)
///
/// 字段与方法面见下方清单; 语义实现由 Developer 修订补齐。
pub struct Graph {
`;
for (const [field, ty, comment] of graphFields) {
  rust += `  /// ${comment}\n  pub ${field}: /* ${ty} */ (),\n`;
}
rust += `}

impl Graph {
  /// 原版 Graph(graph_zone, origin, initial_capacity = ${initialCapacity})
  pub fn new(origin: Origin) -> Self {
    unimplemented!("Developer 修订: 组装各 sidetable")
  }
`;

for (const [rustName, cppSig, comment] of graphSignatures) {
  rust += `
  /// 原版 ${cppSig} — ${comment}
  pub fn ${rustName}(&self) {
    unimplemented!("签名骨架, Developer 修订")
  }
`;
}

rust += `}

// —— 提取不变式断言 ——
const _: () = {
  assert!(INVALID_PREDECESSOR_INDEX == -1);
  assert!(MIN_BLOCK_CAPACITY == 32);
  assert!(DEFAULT_INITIAL_CAPACITY == 2048);
  assert!(BlockKind::Merge == BlockKind::Merge);
  assert!(Origin::PureTurboshaft as u8 == 3);
  assert!(CustomDataKind::Unset as u8 == 0);
  assert!(SLOTS_PER_ID == 2);
};
`;

const p = writeSrc("v8-turboshaft", "graph.rs", rust);
console.log(`written ${p}`);
console.log(`Block::Kind=${blockKinds.length} CustomDataKind=${customKinds.length} Origin=${origins.length} kindName=${kindStrMap.size} 常量=3 字段=${graphFields.length} 签名=${graphSignatures.length}`);
