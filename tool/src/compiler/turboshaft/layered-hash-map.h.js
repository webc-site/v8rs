#!/usr/bin/env bun
// tool/src/compiler/turboshaft/layered-hash-map.h.js
// 机械提取 v8/src/compiler/turboshaft/layered-hash-map.h
// → v8-turboshaft/src/layered_hash_map.rs.skel
//
// 提取面: LayeredHashMap 常量 (kNeedResizePercentage/kGrowthFactor) +
// 方法面 (InsertNewKey/Get/Contains/FindEntryForKey/Resize) + LayeredHashSet。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/layered-hash-map.h");

// 1. 常量 (值核对)
function numConst(re, label, expect) {
  const m = h.match(re);
  if (!m) throw new Error(`未找到 ${label}`);
  if (Number(m[1]) !== expect) throw new Error(`${label}=${m[1]} 预期 ${expect}`);
  return Number(m[1]);
}
const resizePct = numConst(/static constexpr double kNeedResizePercentage = ([\d.]+);/, "kNeedResizePercentage", 0.75);
const growthFactor = numConst(/static constexpr int kGrowthFactor = (\d+);/, "kGrowthFactor", 2);

// 2. LayeredHashMap 方法面
const classM = h.match(/template <class Key, class Value>\s*class LayeredHashMap\s*\{([\s\S]*?)\n\};/);
if (!classM) throw new Error("未找到 LayeredHashMap");
const mapMethods = [...classM[1].matchAll(/^\s*(?:void|std::optional<Value>|bool|Entry\*|size_t) (\w+)\(/gm)].map((m) => m[1]);
for (const m of ["StartLayer", "DropLastLayer", "InsertNewKey", "Get", "Contains", "FindEntryForKey", "ResizeIfNeeded"]) {
  if (!mapMethods.includes(m)) throw new Error(`LayeredHashMap 缺方法: ${m}`);
}

// 3. Entry/Resize 与 LayeredHashSet
for (const [re, label] of [
  [/struct Entry\s*\{/, "Entry"],
  [/Entry\* FindEntryForKey\(Key key, size_t hash = 0\);/, "FindEntryForKey 声明"],
  [/void InsertNewKey\(Key key, Value value\);/, "InsertNewKey 声明"],
  [/std::optional<Value> Get\(Key key\);/, "Get 声明"],
  [/bool Contains\(Key key\)/, "Contains"],
  [/void ResizeIfNeeded\(\);/, "ResizeIfNeeded (私有)"],
  [/void StartLayer\(\);/, "StartLayer — 开新层"],
  [/void DropLastLayer\(\);/, "DropLastLayer — 弃层"],
  [/\/\/ In addition to the regular Insert\/Get\/Contains functions of hash maps/, "设计注释锚"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${label}`);
}

// 4. 生成 Rust
let rust = banner(
  "src/compiler/turboshaft/layered-hash-map.h.js",
  "src/compiler/turboshaft/layered-hash-map.h",
  "分层哈希表: 支配树分层的 map/set 骨架",
);
rust += `
/// 原版 LayeredHashMap::kNeedResizePercentage — 触发扩容的占用比
pub const NEED_RESIZE_PERCENTAGE: f64 = ${resizePct};
/// 原版 LayeredHashMap::kGrowthFactor — 扩容倍数
pub const GROWTH_FACTOR: usize = ${growthFactor};

/// v8: src/compiler/turboshaft/layered-hash-map.h LayeredHashMap
///
/// 方法面 (原版 ${mapMethods.length} 项): ${mapMethods.join(" / ")}
/// Entry: key/hash/value + doubly_threaded 链 (next 同桶指针)。
/// 分层: 支配树新层时快照, 回溯时丢弃本层插入 —
/// 与 SnapshotTable 互补, 用于按块的缓存失效。
pub struct LayeredHashMap<K, V> {
  pub entries: Vec<(K, V)>,
}

impl<K, V> LayeredHashMap<K, V> {
  /// 原版 void InsertNewKey(Key key, Value value)
  /// v8: src/compiler/turboshaft/layered-hash-map.h LayeredHashMap::InsertNewKey
  pub fn insert_new_key(&mut self, key: K, value: V) {
    unimplemented!("Developer 修订: FindEntryForKey 定位 + 装填触发 Resize")
  }
  /// 原版 std::optional<Value> Get(Key key)
  pub fn get(&self, key: &K) -> Option<&V> {
    unimplemented!("签名骨架, Developer 修订")
  }
  /// 原版 bool Contains(Key key)
  pub fn contains(&self, key: &K) -> bool {
    unimplemented!("签名骨架, Developer 修订")
  }
  /// 原版 void StartLayer() — 支配树新层 (层链 depth_neighboring_entry)
  pub fn start_layer(&mut self) {
    unimplemented!("签名骨架, Developer 修订")
  }
  /// 原版 void DropLastLayer() — 回溯弃层 (本层插入失效)
  pub fn drop_last_layer(&mut self) {
    unimplemented!("签名骨架, Developer 修订")
  }
  /// 原版 void ResizeIfNeeded() — 占用 > ${resizePct} 时 ${growthFactor}× 扩容
  pub fn resize_if_needed(&mut self) {
    unimplemented!("签名骨架, Developer 修订")
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(NEED_RESIZE_PERCENTAGE == 0.75);
  assert!(GROWTH_FACTOR == 2);
};
`;

const p = writeSrc("v8-turboshaft", "layered_hash_map.rs.skel", rust);
console.log(`written ${p}`);
console.log(`常量=2 (${resizePct}/${growthFactor}) 方法面=${mapMethods.length} (${mapMethods.join("/")})`);
