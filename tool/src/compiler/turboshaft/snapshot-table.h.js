#!/usr/bin/env bun
// tool/src/compiler/turboshaft/snapshot-table.h.js
// 机械提取 v8/src/compiler/turboshaft/snapshot-table.h
// → v8-turboshaft/src/snapshot_table.rs.skel
//
// 提取面: SnapshotTable 常量 (kNoMergeOffset/kNoMergedPredecessor/
// kInvalidOffset); Entry/Key/Snapshot 结构; 方法面; ChangeTracking 包装。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/snapshot-table.h");

// 1. 常量
const noMerge = [...h.matchAll(/kNoMergeOffset =\s*\n?\s*std::numeric_limits<uint32_t>::max\(\);/g)].length;
const noPred = [...h.matchAll(/kNoMergedPredecessor =\s*\n?\s*std::numeric_limits<uint32_t>::max\(\);/g)].length;
const invalidOff = [...h.matchAll(/kInvalidOffset = std::numeric_limits<size_t>::max\(\);/g)].length;
if (noMerge !== 2 || noPred !== 2 || invalidOff !== 1) {
  throw new Error(`常量计数 noMerge=${noMerge} noPred=${noPred} invalidOff=${invalidOff}`);
}

// 2. 哨兵
for (const [re, name] of [
  [/struct NoKeyData \{\};/, "NoKeyData"],
  [/struct NoChangeCallback \{/, "NoChangeCallback"],
  [/class SnapshotTableKey/, "Key 类"],
  [/class SnapshotTable \{/, "表类"],
  [/class MaybeSnapshot;/, "MaybeSnapshot 前置"],
  [/void RevertCurrentSnapshot\(ChangeCallback& change_callback\)/, "Revert"],
  [/void ReplaySnapshot\(SnapshotData\* snapshot/, "Replay"],
  [/void MergePredecessors\(base::Vector<const Snapshot> predecessors,/, "MergePredecessors"],
  [/class ChangeTrackingSnapshotTable : public SnapshotTable/, "ChangeTracking"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 3. 公开方法面
const api = [
  "StartNewSnapshot", "Seal", "Set", "Get", "GetPredecessorValue",
  "NewKey", "IsSealed", "RevertCurrentSnapshot", "ReplaySnapshot",
  "MergePredecessors", "CurrentSnapshot",
];

let rust = banner(
  "src/compiler/turboshaft/snapshot-table.h.js",
  "src/compiler/turboshaft/snapshot-table.h",
  "快照表: 可回滚键值表 (SSA 重命名基座)",
);

rust += `
/// 原版快照表常量 (小端无关, 数值即语义):
/// - kNoMergeOffset = kNoMergedPredecessor = u32::MAX (Entry 与表两处声明)
/// - kInvalidOffset = usize::MAX (SnapshotData::log_begin 未设)
/// v8: src/compiler/turboshaft/snapshot-table.h
pub const NO_MERGE_OFFSET: u32 = u32::MAX;
pub const NO_MERGED_PREDECESSOR: u32 = u32::MAX;
pub const INVALID_OFFSET: usize = usize::MAX;

/// 原版 SnapshotTableEntry — value + merge_offset + last_merged_predecessor
/// (KeyData 经 EBO 放基类)
/// v8: snapshot-table.h SnapshotTableEntry
pub struct SnapshotTableEntry<V, D> {
  pub value: V,
  pub merge_offset: u32,
  pub last_merged_predecessor: u32,
  pub data: D,
}

/// 原版 SnapshotTable — Key 是表内指针 (指针大小, 廉价拷贝);
/// LogEntry 记 (table_entry, old_value, new_value) 三元组。
/// 方法面:
${api.map((m) => `/// - ${m}`).join("\n")}
/// - StartNewSnapshot(predecessors, merge_fun): 单前驱=精确回滚,
///   多前驱走 merge_values_
/// - Seal / RevertCurrentSnapshot (逆序回放 log) / ReplaySnapshot (正放)
/// - MergePredecessors: RecordMergeValue 记每前驱末值后合并
/// v8: snapshot-table.h SnapshotTable
pub struct SnapshotTable<V, D = ()> {
  pub log: Vec<LogEntry<V>>,
}

/// 原版 ChangeTrackingSnapshotTable — 回调 OnNewKey/OnValueChange
/// v8: snapshot-table.h ChangeTrackingSnapshotTable
pub trait ChangeCallback<V, D> {
  /// v8: NoChangeCallback 为默认空实现
  fn on_new_key(&mut self, key: Key, value: V);
  fn on_value_change(&mut self, key: Key, old: V, new: V);
}
`;

const p = writeSrc("v8-turboshaft", "snapshot_table.rs.skel", rust);
console.log(`written ${p}`);
console.log(`常量noMerge=${noMerge} noPred=${noPred} invalid=${invalidOff} 方法面=${api.length}`);
