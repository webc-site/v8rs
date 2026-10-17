#!/usr/bin/env bun
// tool/src/compiler/turboshaft/store-store-elimination-reducer-inl.h.js
// 机械提取 v8/src/compiler/turboshaft/store-store-elimination-reducer-inl.h
// → v8-turboshaft/src/store_store_elimination_reducer.rs.skel
//
// 提取面: StoreObservability 枚举 (显式值); MaybeRedundantStoresKeyData;
// MaybeRedundantStoresTable; RedundantStoreAnalysis 流程锚。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/store-store-elimination-reducer-inl.h");

// 1. StoreObservability 枚举 (带显式值)
const soM = h.match(/enum class StoreObservability \{([^}]+)\}/);
if (!soM) throw new Error("未找到 StoreObservability");
const observabilities = [...soM[1].matchAll(/k(\w+) = (\d+)/g)].map((m) => ({ name: m[1], value: +m[2] }));
if (observabilities.length !== 3) throw new Error(`StoreObservability ${observabilities.length} != 3`);

// 2. KeyData 字段
const kdM = h.match(/struct MaybeRedundantStoresKeyData \{([\s\S]*?)\n\};/);
if (!kdM) throw new Error("未找到 MaybeRedundantStoresKeyData");
const kdFields = [...kdM[1].matchAll(/^\s*(\w[\w:<>* ]*?)\s+(\w+);\s*$/gm)].map((m) => `${m[1].trim()} ${m[2]}`);
if (kdFields.length !== 3) throw new Error(`KeyData 字段 ${kdFields.length} != 3`);

// 3. 哨兵
for (const [re, name] of [
  [/class MaybeRedundantStoresTable/, "侧表类"],
  [/class RedundantStoreAnalysis \{/, "分析类"],
  [/void Run\(ZoneSet<OpIndex>& eliminable_stores,/, "Run 双输出"],
  [/ZoneMap<OpIndex, uint64_t>& mergeable_store_pairs/, "mergeable 对"],
  [/static constexpr int kBackedgeAdjust = 2;/, "回边调整"],
  [/class StoreStoreEliminationReducer : public Next/, "reducer 类"],
  [/store.kind.tagged_base/, "on-heap 判定"],
  [/!store.index\(\).valid\(\)/, "定偏移判定"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

let rust = banner(
  "src/compiler/turboshaft/store-store-elimination-reducer-inl.h.js",
  "src/compiler/turboshaft/store-store-elimination-reducer-inl.h",
  "store-store 消除: 冗余存储分析",
);

rust += `
/// 原版 StoreObservability — 可观察性三档 (显式值即偏序)
/// v8: store-store-elimination-reducer-inl.h StoreObservability
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum StoreObservability {
  /// kUnobservable = 0: 可消除
  Unobservable = 0,
  /// kGCObservable = 1: GC 可见 (保守半档)
  GcObservable = 1,
  /// kObservable = 2: 完全可见, 不可消除
  Observable = 2,
}

/// 原版 MaybeRedundantStoresKeyData — 冗余键 (base+offset+size)
/// v8: store-store-elimination-reducer-inl.h MaybeRedundantStoresKeyData
pub struct MaybeRedundantStoresKeyData {
${kdFields.map((f) => `  /// 原版 ${f}\n  pub ${f.split(" ").slice(-1)[0]}: ${f.split(" ")[0]},`).join("\n")}
}

/// 原版 RedundantStoreAnalysis — 快照表上做前向分析:
/// - Run(eliminable_stores, mergeable_store_pairs) 双输出
/// - 循环回边按 kBackedgeAdjust = 2 放宽, needs_revisit 时二扫
/// - on-heap (kind.tagged_base) + 定偏移 (!index().valid()) 才候选
/// - 负索引特殊键: (OpIndex, int32_t) 复合 key_mapping_
/// v8: store-store-elimination-reducer-inl.h RedundantStoreAnalysis
pub struct RedundantStoreAnalysis<'g> {
  pub graph: &'g Graph,
  pub eliminable_stores: Vec<OpIndex>,
  /// mergeable_store_pairs: 可合并的相邻同值 store 对 (u64 打包)
  pub mergeable_store_pairs: Vec<(OpIndex, u64)>,
}

/// 原版 StoreStoreEliminationReducer — 消费分析结果的复制期 reducer
/// v8: store-store-elimination-reducer-inl.h StoreStoreEliminationReducer
pub struct StoreStoreEliminationReducer;
`;

const p = writeSrc("v8-turboshaft", "store_store_elimination_reducer.rs.skel", rust);
console.log(`written ${p}`);
console.log(`Observability=${observabilities.length} KeyData字段=${kdFields.length}`);
