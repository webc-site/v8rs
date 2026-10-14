#!/usr/bin/env bun
// tool/src/compiler/turboshaft/late-load-elimination-reducer.h.js
// 机械提取 v8/src/compiler/turboshaft/late-load-elimination-reducer.{h,cc}
// → v8-turboshaft/src/late_load_elimination_reducer.rs.skel
//
// 提取面: LoadEliminationReplacement::Kind (5 变体) + RawBaseAssumption (2) +
// MemoryAddress/MapMaskAndOr/KeyData/OffsetListTraits/BaseListTraits/BaseData/
// MemoryContentTable 结构面 + LateLoadEliminationAnalyzer 字段/REDUCE(AssumeMap) +
// cc 侧 analyzer 方法定义表。语义实现禁编造。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/late-load-elimination-reducer.h");
const cc = readV8("src/compiler/turboshaft/late-load-elimination-reducer.cc");

// 1. 枚举
const kindM = h.match(/class LoadEliminationReplacement\s*\{[\s\S]*?enum class Kind\s*\{([\s\S]*?)\};/);
if (!kindM) throw new Error("未找到 LoadEliminationReplacement::Kind");
const kindVariants = [...kindM[1].replace(/\/\/[^\n]*/g, "").matchAll(/\bk(\w+)\b/g)].map((m) => m[1]);
const expectKind = ["None", "LoadElimination", "TaggedLoadToInt32Load", "TaggedBitcastElimination", "Int32TruncationElimination"];
if (JSON.stringify(kindVariants) !== JSON.stringify(expectKind)) {
  throw new Error(`Kind 变体 ${JSON.stringify(kindVariants)} != 预期 5 项`);
}
const rawM = h.match(/enum class RawBaseAssumption\s*\{([\s\S]*?)\};/);
if (!rawM) throw new Error("未找到 RawBaseAssumption");
const rawVariants = [...rawM[1].replace(/\/\/[^\n]*/g, "").matchAll(/\bk(\w+)\b/g)].map((m) => m[1]);
if (JSON.stringify(rawVariants) !== JSON.stringify(["NoInnerPointer", "MaybeInnerPointer"])) {
  throw new Error(`RawBaseAssumption ${JSON.stringify(rawVariants)} != 2 项`);
}

// 2. REDUCE 面
const reduces = [...h.matchAll(/\bREDUCE\((\w+)\)\s*\(/g)].map((m) => m[1]);
if (JSON.stringify(reduces) !== JSON.stringify(["AssumeMap"])) {
  throw new Error(`REDUCE ${JSON.stringify(reduces)} != [AssumeMap]`);
}

// 3. 结构面哨兵 (h)
for (const [re, label] of [
  [/struct MapMaskAndOr/, "MapMaskAndOr"],
  [/struct MemoryAddress/, "MemoryAddress"],
  [/struct KeyData/, "KeyData"],
  [/struct OffsetListTraits/, "OffsetListTraits"],
  [/struct BaseListTraits/, "BaseListTraits"],
  [/struct BaseData/, "BaseData"],
  [/class MemoryContentTable/, "MemoryContentTable (快照表)"],
  [/v8::base::DoublyThreadedList<Key, OffsetListTraits>/, "DoublyThreadedList (offset)"],
  [/v8::base::DoublyThreadedList<Key, BaseListTraits>/, "DoublyThreadedList (base)"],
  [/class V8_EXPORT_PRIVATE LateLoadEliminationAnalyzer/, "LateLoadEliminationAnalyzer"],
  [/class V8_EXPORT_PRIVATE LateLoadEliminationReducer : public Next/, "LateLoadEliminationReducer"],
  [/TURBOSHAFT_REDUCER_BOILERPLATE\(LateLoadElimination\)/, "BOILERPLATE(LateLoadElimination)"],
  [/using MapKey = MapTable::Key;/, "MapKey 别名"],
  [/using MemorySnapshot = MemoryContentTable::Snapshot;/, "MemorySnapshot 别名"],
]) {
  if (!re.test(h)) throw new Error(`h 哨兵缺失: ${label}`);
}

// 4. cc 侧 analyzer 方法定义 (保序对数)
const ccMethods = [...cc.matchAll(/(?:std::ostream& operator<<|[A-Za-z_][\w:<>&*]*\s+)LateLoadEliminationAnalyzer::(\w+)\(/g)].map((m) => m[1]);
const expectCc = ["Run", "ProcessBlock", "ProcessLoad", "ProcessTrustedLoad", "ProcessStore", "WipeAllMaps"];
for (const name of expectCc) {
  if (!ccMethods.includes(name)) throw new Error(`cc 缺 LateLoadEliminationAnalyzer::${name}`);
}
if (!/std::ostream& operator<<\(std::ostream& os, const MemoryAddress& mem\)/.test(cc)) {
  throw new Error("cc MemoryAddress operator<< 缺失");
}

// 5. 生成 Rust
let rust = banner(
  "src/compiler/turboshaft/late-load-elimination-reducer.h.js",
  "src/compiler/turboshaft/late-load-elimination-reducer.{h,cc}",
  "晚期载入消除: 内存内容快照表与替换判定骨架",
);
rust += `
/// v8: src/compiler/turboshaft/late-load-elimination-reducer.h LoadEliminationReplacement::Kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplacementKind {
${expectKind.map((v, i) => `  /// k${v} (${i})`).join("\n")}
  ${expectKind.map((v) => v.replace(/^./, (c) => c.toLowerCase())).join(", ")},
}

/// v8: src/compiler/turboshaft/late-load-elimination-reducer.h RawBaseAssumption
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RawBaseAssumption {
  /// kNoInnerPointer — CSA 路径 (无内部指针)
  noInnerPointer,
  /// kMaybeInnerPointer — JS 路径 (可能有内部指针)
  maybeInnerPointer,
}

/// v8: src/compiler/turboshaft/late-load-elimination-reducer.h MemoryAddress
/// base + (可选) index 二元组 (operator<< 见 cc)
pub struct MemoryAddress {
  pub base: crate::index::OpIndex,
  pub index: Option<crate::index::OpIndex>,
}

/// v8: src/compiler/turboshaft/late-load-elimination-reducer.h BaseData
/// with_offsets / with_indices: DoublyThreadedList<Key> 双链
pub struct BaseData {
  pub with_offsets: (),
  pub with_indices: (),
}

/// v8: src/compiler/turboshaft/late-load-elimination-reducer.h LateLoadEliminationAnalyzer
///
/// MemoryContentTable (base,offset→value 快照表) + MapTable 精确 map 跟踪;
/// cc 方法面 (定义核对 ${ccMethods.filter((m) => expectCc.includes(m)).length}/${expectCc.length}):
${expectCc.map((m) => `/// - ${m}`).join("\n")}
pub struct LateLoadEliminationAnalyzer {
  pub raw_base_assumption: RawBaseAssumption,
}

impl LateLoadEliminationAnalyzer {
  /// 原版 void Run() — v8: src/compiler/turboshaft/late-load-elimination-reducer.cc
  pub fn run(&mut self) {
    unimplemented!("签名骨架, Developer 修订")
  }
  /// 原版 void ProcessBlock(const Block& block, ...) — 块序处理 + 快照回滚
  pub fn process_block(&mut self) {
    unimplemented!("签名骨架, Developer 修订")
  }
  /// 原版 void ProcessLoad(OpIndex op_idx, const LoadOp& load, ...)
  pub fn process_load(&mut self) {
    unimplemented!("签名骨架, Developer 修订")
  }
  /// 原版 void ProcessTrustedLoad(...) — trusted 空间载入
  pub fn process_trusted_load(&mut self) {
    unimplemented!("签名骨架, Developer 修订")
  }
  /// 原版 void ProcessStore(OpIndex op_idx, const StoreOp& store, ...)
  pub fn process_store(&mut self) {
    unimplemented!("签名骨架, Developer 修订")
  }
  /// 原版 void WipeAllMaps() — 任意写后清 map 状态
  pub fn wipe_all_maps(&mut self) {
    unimplemented!("签名骨架, Developer 修订")
  }
}

/// v8: src/compiler/turboshaft/late-load-elimination-reducer.h LateLoadEliminationReducer
pub struct LateLoadEliminationReducer;

impl LateLoadEliminationReducer {
  /// 原版 V<None> REDUCE(AssumeMap)(V<HeapObject>, ZoneRefSet<Map>)
  /// v8: src/compiler/turboshaft/late-load-elimination-reducer.h LateLoadEliminationReducer::REDUCE(AssumeMap)
  pub fn reduce_assume_map(&mut self) {
    unimplemented!("签名骨架, Developer 修订")
  }
}
`;

const p = writeSrc("v8-turboshaft", "late_load_elimination_reducer.rs.skel", rust);
console.log(`written ${p}`);
console.log(`枚举=2 (${expectKind.length}+${rawVariants.length} 变体) REDUCE=1 结构哨兵=14 cc方法=${expectCc.length}`);
