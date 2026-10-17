//! layered_hash_map 收编核验 — 分层插入/查取/弃层/扩容链序不变式
//! v8: src/compiler/turboshaft/layered-hash-map.h 语义面 (原版无对应 unittest, 机械面自证)

use aok::{OK, Void};
use log::info;
use v8_turboshaft::LayeredHashMap;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 基本面: 单层插入 / 查找 / 缺失键
#[test]
fn layered_hash_map_insert_get_contains() -> Void {
  info!("> LayeredHashMap insert/get/contains");
  let mut map = LayeredHashMap::<u64, u32>::with_capacity(0);
  map.start_layer();
  map.insert_new_key(5, 50);
  map.insert_new_key(21, 210);
  assert_eq!(map.get(&5), Some(50));
  assert_eq!(map.get(&21), Some(210));
  assert!(map.contains(&5));
  assert!(!map.contains(&37));
  assert_eq!(map.get(&37), None);
  OK
}

/// 分层语义: DropLastLayer 只弃最后一层, 前层保留
#[test]
fn layered_hash_map_drop_last_layer_keeps_older_layers() -> Void {
  info!("> LayeredHashMap DropLastLayer 前层保留");
  let mut map = LayeredHashMap::<u64, u32>::with_capacity(16);
  map.start_layer();
  map.insert_new_key(1, 100);
  map.insert_new_key(17, 101); // 与 1 同桶 (初表 mask=15) — 探测链后位
  map.insert_new_key(33, 102);
  map.start_layer();
  map.insert_new_key(2, 200);
  map.insert_new_key(18, 201);
  map.drop_last_layer();
  assert_eq!(map.get(&2), None);
  assert_eq!(map.get(&18), None);
  // 前层链式条目全部仍可命中 (空槽终止探测不得截断前层链)
  assert_eq!(map.get(&1), Some(100));
  assert_eq!(map.get(&17), Some(101));
  assert_eq!(map.get(&33), Some(102));
  OK
}

/// 扩容不变式 (原版注释核心): 重灌按深度升序, DropLastLayer 后无空洞
#[test]
fn layered_hash_map_resize_preserves_depth_order() -> Void {
  info!("> LayeredHashMap ResizeIfNeeded 深度升序重灌");
  let mut map = LayeredHashMap::<u64, u32>::with_capacity(16);
  // 层 1: 同桶链 (bucket 1) 三条 + 桶 2 起三条
  map.start_layer();
  for (i, k) in [1u64, 17, 33, 2, 18, 34].iter().enumerate() {
    map.insert_new_key(*k, i as u32);
  }
  // 层 2: 继续装填至占用比 0.75 触发 2× 扩容 (count>=12)
  map.start_layer();
  for (i, k) in [3u64, 19, 35, 4, 20, 36, 5, 21, 37, 6].iter().enumerate() {
    map.insert_new_key(*k, 100 + i as u32);
  }
  // 扩容后所有键仍可命中
  for k in [1u64, 17, 33, 2, 18, 34] {
    assert!(map.contains(&k), "扩容后层 1 键 {k} 应保留");
  }
  for k in [3u64, 19, 35, 4, 20, 36, 5, 21, 37, 6] {
    assert!(map.contains(&k), "扩容后层 2 键 {k} 应保留");
  }
  map.drop_last_layer();
  for k in [3u64, 19, 35, 4, 20, 36, 5, 21, 37, 6] {
    assert!(!map.contains(&k), "DropLastLayer 后层 2 键 {k} 应消失");
  }
  for (i, k) in [1u64, 17, 33, 2, 18, 34].iter().enumerate() {
    assert_eq!(map.get(k), Some(i as u32), "DropLastLayer 不得截断层 1 链");
  }
  OK
}

/// 不支持更新语义: 先弃层再重插 (原版类头注释)
#[test]
fn layered_hash_map_reinsert_after_drop() -> Void {
  info!("> LayeredHashMap 弃层重插");
  let mut map = LayeredHashMap::<u64, u32>::with_capacity(16);
  map.start_layer();
  map.insert_new_key(7, 70);
  map.drop_last_layer();
  map.start_layer();
  map.insert_new_key(7, 77);
  assert_eq!(map.get(&7), Some(77));
  OK
}

/// hash==0 哨兵修正: ComputeHash(0) → 1, 键 0 可正常存取
#[test]
fn layered_hash_map_zero_key_hash_sentinel_fix() -> Void {
  info!("> LayeredHashMap ComputeHash 0→1");
  let mut map = LayeredHashMap::<u64, u32>::with_capacity(16);
  map.start_layer();
  map.insert_new_key(0, 42);
  assert_eq!(map.get(&0), Some(42));
  map.start_layer();
  map.insert_new_key(1, 43); // hash(1)=1 与 hash(0) 修正值同 hash 不同键 → 探测链
  assert_eq!(map.get(&0), Some(42));
  assert_eq!(map.get(&1), Some(43));
  map.drop_last_layer();
  assert_eq!(map.get(&0), Some(42));
  assert_eq!(map.get(&1), None);
  OK
}

/// 跨层同哈希碰撞链: 三层同桶键经扩容重排后, 逐层 DropLastLayer 不得截断更深层链
/// v8: layered-hash-map.h ResizeIfNeeded 深度升序重灌不变式 (原版注释 a1/a2/a3 场景)
#[test]
fn layered_hash_map_cross_layer_collision_chain_survives_resize() -> Void {
  info!("> LayeredHashMap 跨层碰撞链保序");
  let mut map = LayeredHashMap::<u64, u32>::with_capacity(16);
  // mask=15: 键 4,20,36,52 同落 bucket 4 (hash 直传, id&mask)
  let l0 = [4u64, 20, 36, 52];
  let l1 = [4u64 + 64, 20 + 64, 36 + 64, 52 + 64];
  let l2 = [4u64 + 128, 20 + 128, 36 + 128, 52 + 128];
  // 层 0/1: 同桶深度递增链 (a1<a2 深度序)
  map.start_layer();
  for (i, k) in l0.iter().enumerate() {
    map.insert_new_key(*k, i as u32);
  }
  map.start_layer();
  for (i, k) in l1.iter().enumerate() {
    map.insert_new_key(*k, 10 + i as u32);
  }
  // 填充至触发 2× 扩容 (cap16 → 需 entry_count ≥ 12)
  map.start_layer();
  for (i, k) in l2.iter().enumerate() {
    map.insert_new_key(*k, 20 + i as u32);
  }
  for extra in [8u64, 24, 40, 56, 72, 88] {
    map.insert_new_key(extra, 999);
  }
  // 扩容后全部命中
  for k in l0.iter().chain(l1.iter()).chain(l2.iter()) {
    assert!(map.contains(k), "扩容后键 {k} 应命中");
  }
  // 逐层弃: 先弃含 l2 + extra 的层 3, 再 l1, 再 l0; 每步深层链不被 null 隔断
  map.drop_last_layer();
  for k in l2.iter() {
    assert!(!map.contains(k), "弃层 3 后 {k} 应消失");
  }
  for k in l0.iter().chain(l1.iter()) {
    assert!(map.contains(k), "弃最外层不得影响 {k} (同桶深链续接)");
  }
  map.drop_last_layer(); // 层 2 (l1)
  for k in l1.iter() {
    assert!(!map.contains(k));
  }
  for k in l0.iter() {
    assert!(map.contains(k), "弃中间层不得截断层 1 链 {k}");
  }
  map.drop_last_layer(); // 层 1 (l0)
  for k in l0.iter() {
    assert!(!map.contains(k));
  }
  OK
}
