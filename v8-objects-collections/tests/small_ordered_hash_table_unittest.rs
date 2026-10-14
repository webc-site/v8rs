mod common;

use common::Arena;
use v8_objects_collections::small_ordered_hash_table::{
  K_SMALL_MAX_CAPACITY, K_SMALL_MIN_CAPACITY, SmallOrderedHashMapTable, SmallOrderedHashSet,
  SmallOrderedHashSetExt, SmallOrderedHashSetTable,
};
use v8_objects_core::{
  instance_type::InstanceType,
  smi::Smi,
  tagged::{Object, Tagged},
};

fn smi(val: i32) -> Tagged<Object> {
  Smi::from_int(val).cast()
}

#[test]
fn test_small_ordered_hash_set_basic() {
  let mut arena = Arena::new();
  let map = arena.map(InstanceType::SmallOrderedHashSet);
  let mut set = SmallOrderedHashSetTable::new(map).expect("创建失败");

  assert_eq!(set.len(), 0);
  assert!(set.is_empty());
  assert_eq!(set.capacity(), K_SMALL_MIN_CAPACITY);

  // 插入数据
  assert!(set.insert(smi(1)).expect("插入失败"));
  assert!(set.insert(smi(2)).expect("插入失败"));
  assert_eq!(set.len(), 2);
  assert!(!set.is_empty());

  // 重复插入
  assert!(!set.insert(smi(1)).expect("重复插入失败"));
  assert_eq!(set.len(), 2);

  // 包含性检查
  assert!(set.contains(smi(1)));
  assert!(set.contains(smi(2)));
  assert!(!set.contains(smi(3)));

  // 顺序迭代
  let items: Vec<i32> = set.iter().map(Smi::to_int).collect();
  assert_eq!(items, vec![1, 2]);

  // 删除
  assert!(set.remove(smi(1)));
  assert_eq!(set.len(), 1);
  assert!(!set.contains(smi(1)));
  assert!(set.contains(smi(2)));

  let items_after: Vec<i32> = set.iter().map(Smi::to_int).collect();
  assert_eq!(items_after, vec![2]);
}

#[test]
fn test_small_ordered_hash_set_growth() {
  let mut arena = Arena::new();
  let map = arena.map(InstanceType::SmallOrderedHashSet);
  let mut set = SmallOrderedHashSetTable::new(map).expect("创建失败");

  // 插入 32 个元素，验证单字节桶与链表的正确倍增扩容
  for i in 0..32 {
    assert!(set.insert(smi(i)).expect("插入失败"));
  }

  assert_eq!(set.len(), 32);
  assert!(set.capacity() >= 32);

  for i in 0..32 {
    assert!(set.contains(smi(i)));
  }

  let items: Vec<i32> = set.iter().map(Smi::to_int).collect();
  let expected: Vec<i32> = (0..32).collect();
  assert_eq!(items, expected);
}

#[test]
fn test_small_ordered_hash_map_basic() {
  let mut arena = Arena::new();
  let map = arena.map(InstanceType::SmallOrderedHashMap);
  let mut map_table = SmallOrderedHashMapTable::new(map).expect("创建失败");

  assert_eq!(map_table.len(), 0);
  assert!(map_table.is_empty());

  // 插入
  assert_eq!(map_table.insert(smi(10), smi(100)).expect("插入失败"), None);
  assert_eq!(map_table.insert(smi(20), smi(200)).expect("插入失败"), None);
  assert_eq!(map_table.len(), 2);

  // 获取
  assert_eq!(map_table.get(smi(10)).map(Smi::to_int), Some(100));
  assert_eq!(map_table.get(smi(20)).map(Smi::to_int), Some(200));
  assert_eq!(map_table.get(smi(30)), None);

  // 覆盖写
  let old = map_table.insert(smi(10), smi(105)).expect("更新失败");
  assert_eq!(old.map(Smi::to_int), Some(100));
  assert_eq!(map_table.get(smi(10)).map(Smi::to_int), Some(105));
  assert_eq!(map_table.len(), 2);

  // 迭代
  let pairs: Vec<(i32, i32)> = map_table
    .iter()
    .map(|(k, v)| (Smi::to_int(k), Smi::to_int(v)))
    .collect();
  assert_eq!(pairs, vec![(10, 105), (20, 200)]);

  // 删除
  assert!(map_table.remove(smi(10)));
  assert_eq!(map_table.len(), 1);
  assert_eq!(map_table.get(smi(10)), None);
}

#[test]
fn test_small_ordered_table_capacity_bounds() {
  let mut arena = Arena::new();
  let map = arena.map(InstanceType::SmallOrderedHashSet);
  let mut storage = Vec::new();

  // 低于最小容量应自动 clamp 至 4
  let t1 = SmallOrderedHashSet::allocate(&mut storage, map, 1).expect("分配失败");
  assert_eq!(t1.capacity(), K_SMALL_MIN_CAPACITY);

  // 超过最大容量应严格 clamp 至 254 (K_SMALL_MAX_CAPACITY)
  let t2 = SmallOrderedHashSet::allocate(&mut storage, map, 1000).expect("分配失败");
  assert_eq!(t2.capacity(), K_SMALL_MAX_CAPACITY);
  assert!(t2.capacity() <= K_SMALL_MAX_CAPACITY);

  // 验证 clear 接口
  let mut set_table = SmallOrderedHashSetTable::new(map).expect("创建失败");
  set_table.insert(smi(42)).expect("插入失败");
  assert_eq!(set_table.len(), 1);
  set_table.clear().expect("清空失败");
  assert_eq!(set_table.len(), 0);
  assert!(set_table.is_empty());
}
