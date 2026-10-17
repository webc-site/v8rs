#!/usr/bin/env bun
// tool/src/compiler/turboshaft/snapshot-table-opindex.h.js
// 机械提取 v8/src/compiler/turboshaft/snapshot-table-opindex.h
// → v8-turboshaft/src/snapshot_table_opindex.rs.skel
//
// 提取面: SparseOpIndexSnapshotTable 方法面 (OpIndex<->Key 稀疏映射)。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/snapshot-table-opindex.h");

// 1. 哨兵
for (const [re, name] of [
  [/class SparseOpIndexSnapshotTable : public SnapshotTable/, "稀疏包装类"],
  [/ZoneAbslFlatHashMap<OpIndex, Key> indices_to_keys_;/, "indices_to_keys_"],
  [/Value Get\(OpIndex idx\) const/, "Get(OpIndex)"],
  [/Value GetPredecessorValue\(OpIndex idx, int predecessor_index\)/, "GetPredecessorValue"],
  [/bool Set\(OpIndex idx, Value new_value\)/, "Set(OpIndex)"],
  [/void NewKey\(OpIndex idx, KeyData data, Value initial_value/, "NewKey"],
  [/bool HasKeyFor\(OpIndex idx\) const/, "HasKeyFor"],
  [/std::optional<Key> TryGetKeyFor\(OpIndex idx\) const/, "TryGetKeyFor"],
  [/Key GetOrCreateKey\(OpIndex idx\)/, "GetOrCreateKey 私有"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 2. 方法计数
const methods = ["Get", "GetPredecessorValue", "Set", "NewKey", "NewKey", "HasKeyFor", "TryGetKeyFor", "begin", "end", "GetOrCreateKey"];

let rust = banner(
  "src/compiler/turboshaft/snapshot-table-opindex.h.js",
  "src/compiler/turboshaft/snapshot-table-opindex.h",
  "稀疏快照表: OpIndex 到 Key 的映射包装",
);

rust += `
/// 原版 SparseOpIndexSnapshotTable — SnapshotTable 的 OpIndex 稀疏包装,
/// indices_to_keys_ 用 ZoneAbslFlatHashMap<OpIndex, Key>;
/// 适合"少数 OpIndex 有 key"的场景; 未建 key 时 Get 返回 Value{}。
/// v8: src/compiler/turboshaft/snapshot-table-opindex.h
pub struct SparseOpIndexSnapshotTable<V, D = ()> {
  pub base: SnapshotTable<V, D>,
  pub indices_to_keys: HashMap<OpIndex, Key>,
}

impl<V: Default, D> SparseOpIndexSnapshotTable<V, D> {
  /// v8: Get(OpIndex) — 无 key 返回 Value{}
  pub fn get(&self, idx: OpIndex) -> V { todo!() }
  /// v8: GetPredecessorValue(OpIndex, predecessor_index)
  pub fn get_predecessor_value(&self, idx: OpIndex, pred: usize) -> V { todo!() }
  /// v8: Set(OpIndex, new_value) — 经 GetOrCreateKey
  pub fn set(&mut self, idx: OpIndex, v: V) -> bool { todo!() }
  /// v8: NewKey(OpIndex, KeyData, initial_value) — 重复建 key DCHECK
  pub fn new_key(&mut self, idx: OpIndex, data: D, v: V) { todo!() }
  /// v8: HasKeyFor / TryGetKeyFor
  pub fn has_key_for(&self, idx: OpIndex) -> bool { todo!() }
  pub fn try_get_key_for(&self, idx: OpIndex) -> Option<Key> { todo!() }
  /// v8: begin/end — 暴露 indices_to_keys_ 迭代
  pub fn iter(&self) -> impl Iterator<Item = (OpIndex, Key)> + '_ { todo!() }
  /// v8 (私有): GetOrCreateKey — 缺则 NewKey 并登记
  fn get_or_create_key(&mut self, idx: OpIndex) -> Key { todo!() }
}
`;

const p = writeSrc("v8-turboshaft", "snapshot_table_opindex.rs.skel", rust);
console.log(`written ${p}`);
console.log(`方法面=${methods.length}`);
