mod common;

use common::Arena;
use v8_objects_collections::ordered_hash_table::{
  K_CLEARED_TABLE_SENTINEL, K_INITIAL_CAPACITY, OrderedHashMapTable, OrderedHashSet,
  OrderedHashSetExt, OrderedHashSetTable, OrderedNameDictionary, OrderedNameDictionaryExt,
  OrderedNameDictionaryTable,
};
use v8_objects_core::{
  instance_type::InstanceType,
  property_details::{PropertyAttributes, PropertyConstness, PropertyDetails, PropertyKind},
  smi::Smi,
  tagged::{Object, Tagged},
};

fn smi(val: i32) -> Tagged<Object> {
  Smi::from_int(val).cast()
}

fn sample_details(index: i32) -> PropertyDetails {
  PropertyDetails::for_dictionary(
    PropertyKind::Data,
    PropertyAttributes::NONE,
    PropertyConstness::Const,
    index as u32,
  )
}

#[test]
fn test_ordered_hash_set_basic() {
  let mut arena = Arena::new();
  let map = arena.map(InstanceType::OrderedHashSet);
  let mut set = OrderedHashSetTable::new(map).expect("创建 OrderedHashSetTable 失败");

  assert_eq!(set.len(), 0);
  assert!(set.is_empty());
  assert_eq!(set.capacity(), K_INITIAL_CAPACITY);

  // 插入元素
  assert!(set.insert(smi(10)).expect("插入失败"));
  assert!(set.insert(smi(20)).expect("插入失败"));
  assert_eq!(set.len(), 2);
  assert!(!set.is_empty());

  // 重复插入应返回 false
  assert!(!set.insert(smi(10)).expect("重复插入失败"));
  assert_eq!(set.len(), 2);

  // 查询
  assert!(set.contains(smi(10)));
  assert!(set.contains(smi(20)));
  assert!(!set.contains(smi(30)));

  // 迭代顺序应严格保持插入顺序 [10, 20]
  let items: Vec<i32> = set.iter().map(Smi::to_int).collect();
  assert_eq!(items, vec![10, 20]);

  // 删除元素
  assert!(set.remove(smi(10)));
  assert_eq!(set.len(), 1);
  assert!(!set.contains(smi(10)));
  assert!(set.contains(smi(20)));

  // 删除不存在的元素返回 false
  assert!(!set.remove(smi(999)));

  // 迭代时应跳过墓碑/空穴，保持剩余元素顺序
  let items_after: Vec<i32> = set.iter().map(Smi::to_int).collect();
  assert_eq!(items_after, vec![20]);
}

#[test]
fn test_ordered_hash_set_growth_and_shrink() {
  let mut arena = Arena::new();
  let map = arena.map(InstanceType::OrderedHashSet);
  let mut set = OrderedHashSetTable::new(map).expect("创建失败");

  // 插入超过初始容量 (4) 触发自动扩容
  for i in 0..16 {
    assert!(set.insert(smi(i)).expect("插入失败"));
  }

  assert_eq!(set.len(), 16);
  assert!(set.capacity() >= 16);

  // 验证所有元素可被查到且保持顺序
  for i in 0..16 {
    assert!(set.contains(smi(i)));
  }

  let items: Vec<i32> = set.iter().map(Smi::to_int).collect();
  let expected: Vec<i32> = (0..16).collect();
  assert_eq!(items, expected);

  // 连续删除多数元素触发缩容
  for i in 4..16 {
    assert!(set.remove(smi(i)));
  }

  assert_eq!(set.len(), 4);
  for i in 0..4 {
    assert!(set.contains(smi(i)));
  }
}

#[test]
fn test_ordered_hash_map_basic() {
  let mut arena = Arena::new();
  let map = arena.map(InstanceType::OrderedHashMap);
  let mut table = OrderedHashMapTable::new(map).expect("创建 OrderedHashMapTable 失败");

  assert_eq!(table.len(), 0);
  assert!(table.is_empty());

  // 插入键值对
  assert_eq!(table.insert(smi(1), smi(100)).expect("插入失败"), None);
  assert_eq!(table.insert(smi(2), smi(200)).expect("插入失败"), None);
  assert_eq!(table.len(), 2);

  // 查值
  assert_eq!(table.get(smi(1)).map(Smi::to_int), Some(100));
  assert_eq!(table.get(smi(2)).map(Smi::to_int), Some(200));
  assert_eq!(table.get(smi(3)), None);

  // 更新键的值
  let old = table.insert(smi(1), smi(101)).expect("更新失败");
  assert_eq!(old.map(Smi::to_int), Some(100));
  assert_eq!(table.get(smi(1)).map(Smi::to_int), Some(101));
  assert_eq!(table.len(), 2);

  // 迭代顺序
  let pairs: Vec<(i32, i32)> = table
    .iter()
    .map(|(k, v)| (Smi::to_int(k), Smi::to_int(v)))
    .collect();
  assert_eq!(pairs, vec![(1, 101), (2, 200)]);

  // 删除
  assert!(table.remove(smi(1)));
  assert_eq!(table.len(), 1);
  assert_eq!(table.get(smi(1)), None);
  assert_eq!(table.get(smi(2)).map(Smi::to_int), Some(200));
}

#[test]
fn test_ordered_hash_table_cleared_sentinel() {
  let mut arena = Arena::new();
  let map = arena.map(InstanceType::OrderedHashSet);
  let mut storage = Vec::new();
  let set = OrderedHashSet::allocate(&mut storage, map, K_INITIAL_CAPACITY).expect("分配失败");

  assert_eq!(set.number_of_elements(), 0);
  assert_eq!(set.number_of_deleted_elements(), 0);

  // 调用 clear() 验证旧表自动打上 K_CLEARED_TABLE_SENTINEL 哨兵并指向新表
  let new_set = set.clear(&mut storage, map).expect("clear 失败");
  assert!(set.is_obsolete());
  assert_eq!(
    set.number_of_deleted_elements(),
    K_CLEARED_TABLE_SENTINEL as usize
  );
  assert_eq!(set.used_capacity(), 0);
  assert_eq!(set.next_table(), Some(new_set));

  // 验证安全包装类的 clear
  let mut table = OrderedHashSetTable::new(map).expect("创建失败");
  table.insert(smi(1)).expect("插入失败");
  table.insert(smi(2)).expect("插入失败");
  assert_eq!(table.len(), 2);
  table.clear().expect("清空失败");
  assert_eq!(table.len(), 0);
  assert!(table.is_empty());

  // 验证 rehash 过程中墓碑索引记录 (SetRemovedIndexAt / RemovedIndexAt)
  let s = OrderedHashSet::allocate(&mut storage, map, K_INITIAL_CAPACITY).expect("分配失败");
  let s = s.add(&mut storage, map, smi(10)).expect("add 失败");
  let s = s.add(&mut storage, map, smi(20)).expect("add 失败");
  assert!(s.delete(smi(10))); // 删除 entry 0
  assert_eq!(s.number_of_deleted_elements(), 1);

  let new_s = s
    .rehash(&mut storage, map, K_INITIAL_CAPACITY)
    .expect("rehash 失败");
  assert!(s.is_obsolete());
  assert_eq!(s.next_table(), Some(new_s));
  assert_eq!(s.removed_index_at(0), 0); // 旧表中下标 0 的洞被记录
}

#[test]
fn test_ordered_name_dictionary_basic() {
  let mut arena = Arena::new();
  let map = arena.map(InstanceType::OrderedNameDictionary);
  let mut dict =
    OrderedNameDictionaryTable::new(map).expect("创建 OrderedNameDictionaryTable 失败");

  assert_eq!(dict.len(), 0);
  assert!(dict.is_empty());
  assert_eq!(dict.capacity(), K_INITIAL_CAPACITY);
  assert_eq!(dict.hash(), 0);
  dict.set_hash(42);
  assert_eq!(dict.hash(), 42);

  let k1 = smi(1);
  let v1 = smi(100);
  let d1 = sample_details(1);

  let k2 = smi(2);
  let v2 = smi(200);
  let d2 = sample_details(2);

  // 插入
  assert_eq!(dict.insert(k1, v1, d1).expect("插入 k1 失败"), None);
  assert_eq!(dict.insert(k2, v2, d2).expect("插入 k2 失败"), None);
  assert_eq!(dict.len(), 2);
  assert!(!dict.is_empty());

  // 查询
  assert!(dict.contains_key(k1));
  assert!(dict.contains_key(k2));
  assert!(!dict.contains_key(smi(3)));

  let (val1, det1) = dict.get(k1).expect("获取 k1 失败");
  assert_eq!(Smi::to_int(val1), 100);
  assert_eq!(det1.attributes(), PropertyAttributes::NONE);

  // 更新键值与详情
  let v1_new = smi(101);
  let d1_new = PropertyDetails::for_dictionary(
    PropertyKind::Data,
    PropertyAttributes::READ_ONLY,
    PropertyConstness::Const,
    1,
  );
  let old = dict.insert(k1, v1_new, d1_new).expect("更新 k1 失败");
  assert_eq!(old.map(Smi::to_int), Some(100));
  assert_eq!(dict.len(), 2);

  let (val1_updated, det1_updated) = dict.get(k1).expect("获取更新后的 k1 失败");
  assert_eq!(Smi::to_int(val1_updated), 101);
  assert!(
    det1_updated
      .attributes()
      .contains(PropertyAttributes::READ_ONLY)
  );

  // 迭代顺序验证
  let items: Vec<(i32, i32)> = dict
    .iter()
    .map(|(k, v, _)| (Smi::to_int(k), Smi::to_int(v)))
    .collect();
  assert_eq!(items, vec![(1, 101), (2, 200)]);

  // 删除
  assert!(dict.remove(k1));
  assert_eq!(dict.len(), 1);
  assert!(!dict.contains_key(k1));
  assert!(dict.contains_key(k2));

  // 清空
  dict.clear().expect("清空失败");
  assert_eq!(dict.len(), 0);
  assert!(dict.is_empty());
}

#[test]
fn test_ordered_name_dictionary_raw_ext() {
  let mut arena = Arena::new();
  let map = arena.map(InstanceType::OrderedNameDictionary);
  let mut storage = Vec::new();

  let dict =
    OrderedNameDictionary::allocate(&mut storage, map, K_INITIAL_CAPACITY).expect("分配失败");
  assert_eq!(dict.number_of_elements(), 0);
  assert_eq!(dict.number_of_deleted_elements(), 0);
  assert_eq!(dict.capacity(), K_INITIAL_CAPACITY);
  assert_eq!(dict.hash(), 0);

  dict.set_hash(12345);
  assert_eq!(dict.hash(), 12345);

  let k1 = smi(11);
  let v1 = smi(111);
  let d1 = sample_details(11);

  let dict = dict
    .add(&mut storage, map, k1, v1, d1)
    .expect("添加 k1 失败");
  assert_eq!(dict.number_of_elements(), 1);
  assert!(dict.has_key(k1));

  let entry = dict.find_entry(k1);
  assert!(entry.is_found());
  assert_eq!(dict.key_at(entry), k1);
  assert_eq!(dict.value_at(entry), v1);
  assert_eq!(dict.details_at(entry), d1);

  // 删除 entry
  assert!(dict.delete_entry(entry));
  assert_eq!(dict.number_of_elements(), 0);
  assert_eq!(dict.number_of_deleted_elements(), 1);

  // 重哈希保序与墓碑记录
  let new_dict = dict
    .rehash(&mut storage, map, K_INITIAL_CAPACITY)
    .expect("rehash 失败");
  assert_eq!(new_dict.number_of_elements(), 0);
  assert_eq!(new_dict.number_of_deleted_elements(), 0);
  assert_eq!(new_dict.hash(), 12345); // 哈希值在重哈希后继承
  assert!(dict.is_obsolete()); // 旧表标记为 obsolete
  assert_eq!(dict.next_table(), Some(new_dict));
  assert_eq!(dict.removed_index_at(0), entry.as_usize()); // 洞索引正确记录

  // 清空标志与哨兵
  let cleared = new_dict.clear(&mut storage, map).expect("clear 失败");
  assert!(new_dict.is_obsolete());
  assert_eq!(new_dict.next_table(), Some(cleared));
  assert_eq!(
    new_dict.number_of_deleted_elements(),
    K_CLEARED_TABLE_SENTINEL as isize as usize
  );
  assert_eq!(cleared.number_of_elements(), 0);
}

#[test]
fn test_ordered_name_dictionary_growth_and_shrink() {
  let mut arena = Arena::new();
  let map = arena.map(InstanceType::OrderedNameDictionary);
  let mut dict = OrderedNameDictionaryTable::new(map).expect("创建失败");

  // 插入 16 项触发多轮自动扩容
  for i in 0..16 {
    dict
      .insert(smi(i), smi(i * 10), sample_details(i))
      .expect("插入失败");
  }

  assert_eq!(dict.len(), 16);
  assert!(dict.capacity() >= 16);

  // 验证所有元素可检索且值一致
  for i in 0..16 {
    let (v, _) = dict.get(smi(i)).expect("查键失败");
    assert_eq!(Smi::to_int(v), i * 10);
  }

  // 验证迭代保序
  let keys: Vec<i32> = dict.iter().map(|(k, ..)| Smi::to_int(k)).collect();
  let expected: Vec<i32> = (0..16).collect();
  assert_eq!(keys, expected);

  // 删除部分项触发缩容
  for i in 4..16 {
    assert!(dict.remove(smi(i)));
  }

  assert_eq!(dict.len(), 4);
  for i in 0..4 {
    assert!(dict.contains_key(smi(i)));
  }
}

#[test]
fn test_ordered_hash_table_cc_generated_rules() {
  use v8_objects_collections::ordered_hash_table_cc_generated as cc;

  // 1. normalized_capacity 归一化容量为 2 的幂且下限为 4
  assert_eq!(cc::normalized_capacity(0), 4);
  assert_eq!(cc::normalized_capacity(1), 4);
  assert_eq!(cc::normalized_capacity(3), 4);
  assert_eq!(cc::normalized_capacity(4), 4);
  assert_eq!(cc::normalized_capacity(5), 8);
  assert_eq!(cc::normalized_capacity(8), 8);
  assert_eq!(cc::normalized_capacity(9), 16);
  assert_eq!(cc::normalized_capacity(16), 16);

  // 2. number_of_buckets 桶数为容量的一半
  assert_eq!(cc::number_of_buckets(4), 2);
  assert_eq!(cc::number_of_buckets(8), 4);
  assert_eq!(cc::number_of_buckets(16), 8);
  assert_eq!(cc::number_of_buckets(32), 16);

  // 3. backing_length 计算
  // prefix_size = 0, num_buckets = 4 (capacity 8), entry_size = 2
  // hash_table_start_index = 3, total = 3 + 4 + 8*2 = 23
  assert_eq!(cc::backing_length(3, 8, 2), 3 + 4 + 16);
  assert_eq!(cc::backing_length(3, 8, 3), 3 + 4 + 24);

  // 4. ensure_capacity_target
  // 未满: None
  assert_eq!(cc::ensure_capacity_target(2, 1, 4), None);
  // capacity 0: 扩到 4
  assert_eq!(cc::ensure_capacity_target(0, 0, 0), Some(4));
  // 满且删除元素达到一半: 原容量重哈希
  assert_eq!(cc::ensure_capacity_target(2, 2, 4), Some(4));
  // 满且删除元素不足一半: 容量翻倍
  assert_eq!(cc::ensure_capacity_target(3, 1, 4), Some(8));

  // 5. shrink_target
  // nof >= cap / 4: 不缩容
  assert_eq!(cc::shrink_target(4, 16), None);
  assert_eq!(cc::shrink_target(5, 16), None);
  // nof < cap / 4: 减半
  assert_eq!(cc::shrink_target(3, 16), Some(8));
  assert_eq!(cc::shrink_target(1, 16), Some(8));
}
