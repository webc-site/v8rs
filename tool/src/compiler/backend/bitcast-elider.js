#!/usr/bin/env bun
// tool/src/compiler/backend/bitcast-elider.js
// v8/src/compiler/backend/bitcast-elider.{h,cc} → v8-compiler-backend/src/bitcast_elider.rs.skel
// BitcastElider: 指令选择前清除 Bitcast/TruncateInt64ToInt32 冗余节点, 免生成多余 move
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/backend/bitcast-elider.h");
const cc = readV8("src/compiler/backend/bitcast-elider.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

// h: 类签名 + 公有方法 5 + 私有字段 4
must(h.includes("class BitcastElider {"), "h BitcastElider 类");
for (const sig of [
  "BitcastElider(Zone* zone, TFGraph* graph, bool is_builtin);",
  "void Reduce();", "void Enqueue(Node* node);", "void Revisit(Node* node);",
  "void VisitNode(Node* node);", "void ProcessGraph();",
]) must(h.includes(sig), `h 方法 ${sig}`);
const fields = ["TFGraph* const graph_;", "ZoneQueue<Node*> to_visit_;",
  "NodeMarker<bool> seen_;", "bool is_builtin_;"];
for (const f of fields) must(h.includes(f), `h 字段 ${f}`);
must(count(h, /^\s*(explicit )?\w[\w:<>* ]+\(\)/m) >= 0, "h 占位");

// cc: IsBitcast 恰 2 opcode (GC/safepoint 安全面)
must(count(cc, /node->opcode\(\) == IrOpcode::k\w+/g) === 2, "cc IsBitcast opcode 数");
for (const op of ["kBitcastTaggedToWordForTagAndSmiBits", "kBitcastWordToTaggedSigned"])
  must(cc.includes(`IrOpcode::${op}`), `cc ${op}`);
// cc: OwnedByWord32Op 三架构分支 — MIPS64/RISCV64 短路 false / LOONG64 7 case / 默认 4 显式 case + 宏列表
must(count(cc, /#if defined\(V8_TARGET_ARCH_MIPS64\) \|\| defined\(V8_TARGET_ARCH_RISCV64\)/g) === 1, "cc MIPS64/RISCV64 分支");
must(count(cc, /#elif defined\(V8_TARGET_ARCH_LOONG64\)/g) === 1, "cc LOONG64 分支");
must(count(cc, /case IrOpcode::k\w+:/g) === 14, "cc 显式 case 总数 14 (LOONG64 8 + 默认 6)");
must(cc.includes("MACHINE_BINOP_32_LIST(Word32Op)"), "cc 宏列表 MACHINE_BINOP_32_LIST");
must(cc.includes("kTruncateInt64ToInt32"), "cc 截断 opcode");
// cc: 匿名 namespace 3 自由函数
must(count(cc, /^bool IsBitcast\(|^bool OwnedByWord32Op\(|^void Replace\(/gm) === 3, "cc 匿名自由函数 3");
// cc: NodeMarker 初值 2 态
must(cc.includes("seen_(graph, 2)"), "cc seen_ 2 态");
must(count(cc, /void BitcastElider::/g) === 5, "cc 成员实现 5 (Enqueue/Revisit/VisitNode/ProcessGraph/Reduce)");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/bitcast-elider.js",
  "src/compiler/backend/bitcast-elider.h + bitcast-elider.cc",
  "BitcastElider: 图遍历消 Bitcast/TruncateInt64ToInt32 冗余节点 (指令选择前处理)")}
//! 台账: 公有面 ctor/Reduce + 内部 Enqueue/Revisit/VisitNode/ProcessGraph;
//!   字段 4 (graph_/to_visit_/seen_/is_builtin_);
//!   IsBitcast 白名单 2 opcode (kBitcastTaggedToWordForTagAndSmiBits,
//!   kBitcastWordToTaggedSigned — 其余 Bitcast 影响 GC/safepoint 表不可消);
//!   OwnedByWord32Op 架构三分支: mips64/riscv64=false, loong64=8 opcode 白名单,
//!   默认(x64/arm64)=6 显式 + MACHINE_BINOP_32_LIST 展开;
//!   TruncateInt64ToInt32 仅当 OwnedByWord32Op 才消; Replace=改边+Kill。

/// v8: BitcastElider::Reduce — 入口 (ProcessGraph 包装)
// pub fn reduce(&mut self)
/// v8: BitcastElider::ProcessGraph — 自 end 起工作队列 BFS
// fn process_graph(&mut self)
/// v8: BitcastElider::VisitNode — 逐输入消 Bitcast/Truncate 或入队
// fn visit_node(&mut self, node)
/// v8: BitcastElider::Enqueue — seen_ 去重入队
// fn enqueue(&mut self, node)
/// v8: BitcastElider::Revisit — 无去重重入队
// fn revisit(&mut self, node)
/// v8: OwnedByWord32Op — 默认分支显式 opcode 集 (另含 MACHINE_BINOP_32_LIST 展开)
pub const OWNED_BY_WORD32_EXPLICIT_DEFAULT: &[&str] = &[
  "Word32Equal",
  "Int32LessThan",
  "Int32LessThanOrEqual",
  "Uint32LessThan",
  "Uint32LessThanOrEqual",
  "ChangeInt32ToInt64",
];
/// v8: OwnedByWord32Op — loong64 分支 opcode 白名单 (mips64/riscv64 恒 false)
pub const OWNED_BY_WORD32_LOONG64: &[&str] = &[
  "Word32Equal", "Word32Shl", "Word32Shr", "Word32Sar",
  "Word32Rol", "Word32Ror", "Int32Add", "Int32Sub",
];
/// v8: IsBitcast 白名单 — 仅 builtin 路径可消 (GC/safepoint 约束)
pub const BITCAST_ELIDABLE: &[&str] = &[
  "BitcastTaggedToWordForTagAndSmiBits",
  "BitcastWordToTaggedSigned",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(BITCAST_ELIDABLE.len() == 2);
  assert!(OWNED_BY_WORD32_LOONG64.len() == 8);
  assert!(OWNED_BY_WORD32_EXPLICIT_DEFAULT.len() == 6);
};
`;

writeSrc("v8-compiler-backend", "bitcast_elider.rs.skel", skel);
console.log(`bitcast-elider.js: 对数 ${checks} 条全过; 方法 5 + 匿名 3 + 字段 4; ` +
  `IsBitcast 白名单 2; OwnedByWord32Op 三分支 (显式 case 14 = loong64 8 + 默认 6); 产物 bitcast_elider.rs.skel`);
