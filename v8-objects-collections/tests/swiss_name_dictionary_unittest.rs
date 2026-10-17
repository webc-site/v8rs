use v8_objects_core::property_details::PropertyAttributes;
mod common;

use common::Arena;
use v8_objects_collections::swiss_name_dictionary::{
  Group16, K_DELETED, K_EMPTY, K_SENTINEL, K_SWISS_INITIAL_CAPACITY, ProbeSequence,
  SwissNameDictionaryTable, h1_from_hash, h2_from_hash,
};
use v8_objects_core::{
  instance_type::InstanceType,
  property_details::{PropertyConstness, PropertyDetails, PropertyKind},
  smi::Smi,
  tagged::{Object, Tagged},
};

fn smi(val: i32) -> Tagged<Object> {
  Tagged::new(Smi::from_int(val).ptr())
}

fn sample_details(_index: i32) -> PropertyDetails {
  PropertyDetails::for_dictionary(
    PropertyKind::Data,
    PropertyAttributes::NONE,
    PropertyConstness::Const,
    0,
  )
}

#[test]
fn test_swiss_group16_matching() {
  let mut ctrl = [K_EMPTY; 16];
  ctrl[0] = 0x42;
  ctrl[3] = K_DELETED;
  ctrl[7] = 0x42;
  ctrl[15] = K_SENTINEL;

  let group = unsafe { Group16::load(ctrl.as_ptr()) };

  // Match 0x42
  let mask_42 = group.match_byte(0x42);
  let mut matches = Vec::new();
  for idx in mask_42 {
    matches.push(idx);
  }
  assert_eq!(matches, vec![0, 7]);

  // Match Empty
  let mask_empty = group.match_empty();
  let empty_count = mask_empty.count();
  assert_eq!(empty_count, 12); // 16 - 4 occupied/special

  // Match Empty or Deleted
  let mask_empty_or_deleted = group.match_empty_or_deleted();
  let empty_or_deleted_count = mask_empty_or_deleted.count();
  assert_eq!(empty_or_deleted_count, 13); // includes slot 3 (K_DELETED)
}

#[test]
fn test_swiss_h1_h2_and_probe_sequence() {
  let hash: u32 = 0x1234_5678;
  let h1 = h1_from_hash(hash);
  let h2 = h2_from_hash(hash);

  // H1 是 hash >> 7
  assert_eq!(h1, hash >> 7);
  // H2 是 hash & 0x7f
  assert_eq!(h2, (hash & 0x7f) as u8);

  // ProbeSequence 步长
  let mut probe = ProbeSequence::new(h1, 32);
  assert_eq!(probe.offset(), h1 & 31);

  probe.next();
  assert_eq!(probe.offset(), (h1 + 16) & 31);
}

#[test]
fn test_swiss_name_dictionary_basic() {
  let mut arena = Arena::new();
  let map = arena.map(InstanceType::SwissNameDictionary);
  let byte_array_map = arena.map(InstanceType::ByteArray);

  let mut dict =
    SwissNameDictionaryTable::new(map, byte_array_map).expect("创建 SwissNameDictionary 失败");

  assert_eq!(dict.len(), 0);
  assert!(dict.is_empty());
  assert_eq!(dict.capacity(), K_SWISS_INITIAL_CAPACITY);

  // 插入条目
  let k1 = smi(101);
  let v1 = smi(1001);
  let d1 = sample_details(1);

  let k2 = smi(202);
  let v2 = smi(2002);
  let d2 = sample_details(2);

  assert_eq!(dict.insert(k1, v1, d1).expect("插入 k1 失败"), None);
  assert_eq!(dict.insert(k2, v2, d2).expect("插入 k2 失败"), None);
  assert_eq!(dict.len(), 2);
  assert!(!dict.is_empty());

  // 重复插入应更新值与属性，并返回旧值
  let v1_updated = smi(9999);
  assert_eq!(
    dict.insert(k1, v1_updated, d1).expect("更新 k1 失败"),
    Some(v1)
  );
  assert_eq!(dict.len(), 2);

  // 查询
  assert!(dict.contains_key(k1));
  assert!(dict.contains_key(k2));
  assert!(!dict.contains_key(smi(303)));

  assert_eq!(dict.get(k1).map(Smi::to_int), Some(9999));
  assert_eq!(dict.get(k2).map(Smi::to_int), Some(2002));

  // 顺序迭代 (基于 EnumTable 维持插入顺序)
  let items: Vec<(i32, i32)> = dict
    .iter()
    .map(|(k, v, _d)| (Smi::to_int(k), Smi::to_int(v)))
    .collect();
  assert_eq!(items, vec![(101, 9999), (202, 2002)]);

  // 删除
  assert!(dict.remove(k1));
  assert_eq!(dict.len(), 1);
  assert!(!dict.contains_key(k1));
  assert!(dict.contains_key(k2));

  // 重新插入被删除的键
  assert_eq!(dict.insert(k1, v1, d1).expect("重新插入 k1 失败"), None);
  assert_eq!(dict.len(), 2);
  assert!(dict.contains_key(k1));
}

#[test]
fn test_swiss_name_dictionary_growth() {
  let mut arena = Arena::new();
  let map = arena.map(InstanceType::SwissNameDictionary);
  let byte_array_map = arena.map(InstanceType::ByteArray);

  let mut dict = SwissNameDictionaryTable::new(map, byte_array_map).expect("创建失败");

  // 初始容量为 8，当元素达到 8 * 7 / 8 = 7 时触发瑞士表扩容
  for i in 0..12 {
    let k = smi(i);
    let v = smi(i * 10);
    dict.insert(k, v, sample_details(i)).expect("插入失败");
  }

  assert_eq!(dict.len(), 12);
  assert!(dict.capacity() > K_SWISS_INITIAL_CAPACITY);

  // 验证所有元素可检索
  for i in 0..12 {
    assert_eq!(dict.get(smi(i)).map(Smi::to_int), Some(i * 10));
  }
}

#[test]
fn test_swiss_name_dictionary_rehash_in_place() {
  use v8_objects_collections::swiss_name_dictionary::SwissNameDictionaryExt;

  let mut arena = Arena::new();
  let map = arena.map(InstanceType::SwissNameDictionary);
  let byte_array_map = arena.map(InstanceType::ByteArray);

  let mut dict = SwissNameDictionaryTable::new(map, byte_array_map).expect("创建失败");

  for i in 1..=5 {
    dict
      .insert(smi(i), smi(i * 100), sample_details(i))
      .expect("插入失败");
  }

  assert_eq!(dict.len(), 5);
  assert_eq!(dict.tagged().number_of_deleted_elements(), 0);

  // 删除元素 2 和 4
  assert!(dict.remove(smi(2)));
  assert!(dict.remove(smi(4)));
  assert_eq!(dict.len(), 3);
  assert_eq!(dict.tagged().number_of_deleted_elements(), 2);

  // 原地重哈希，清除已删除槽
  dict.rehash_in_place();

  assert_eq!(dict.len(), 3);
  assert_eq!(dict.tagged().number_of_deleted_elements(), 0);

  // 验证保留元素与顺序 (1, 3, 5)
  assert_eq!(dict.get(smi(1)).map(Smi::to_int), Some(100));
  assert_eq!(dict.get(smi(3)).map(Smi::to_int), Some(300));
  assert_eq!(dict.get(smi(5)).map(Smi::to_int), Some(500));
  assert_eq!(dict.get(smi(2)), None);
  assert_eq!(dict.get(smi(4)), None);

  let order: Vec<i32> = dict.iter().map(|(k, ..)| Smi::to_int(k)).collect();
  assert_eq!(order, vec![1, 3, 5]);
}

#[test]
fn test_swiss_name_dictionary_cc_generated_functions() {
  use v8_objects_collections::{
    swiss_name_dictionary::SwissNameDictionaryExt, swiss_name_dictionary_cc_generated as cc,
  };

  let mut arena = Arena::new();
  let map = arena.map(InstanceType::SwissNameDictionary);
  let byte_array_map = arena.map(InstanceType::ByteArray);

  let mut dict = SwissNameDictionaryTable::new(map, byte_array_map).expect("创建失败");

  let k1 = smi(10);
  let v1 = smi(100);
  let d_enum = PropertyDetails::for_dictionary(
    PropertyKind::Data,
    PropertyAttributes::NONE,
    PropertyConstness::Const,
    0,
  );
  let k2 = smi(20);
  let v2 = smi(200);
  let d_dont_enum = PropertyDetails::for_dictionary(
    PropertyKind::Data,
    PropertyAttributes::DONT_ENUM,
    PropertyConstness::Const,
    0,
  );

  dict.insert(k1, v1, d_enum).expect("插入 k1");
  dict.insert(k2, v2, d_dont_enum).expect("插入 k2");

  let addr = dict.tagged().ptr() as u64;

  // 1. 可枚举属性计数
  assert_eq!(cc::number_of_enumerable_properties(addr), 1);

  // 2. 慢速反查: v1 (100) → k1 (10)
  let found_key_addr = cc::slow_reverse_lookup(0, addr, v1.ptr() as u64);
  assert_eq!(found_key_addr, k1.ptr() as u64);

  // 慢速反查不存在的值返回 0
  assert_eq!(cc::slow_reverse_lookup(0, addr, 9999), 0);

  // 3. 自等价测试
  assert!(cc::equals_for_testing(addr, addr));

  // 4. delete_entry
  let entry = dict.tagged().find_entry(k1);
  assert!(entry.is_found());
  cc::delete_entry(0, addr, entry.as_uint32());
  assert_eq!(dict.tagged().number_of_elements(), 1);
  assert_eq!(dict.tagged().number_of_deleted_elements(), 1);

  // 5. rehash_in_place
  cc::rehash_in_place(0, addr);
  assert_eq!(dict.tagged().number_of_elements(), 1);
  assert_eq!(dict.tagged().number_of_deleted_elements(), 0);
  assert_eq!(dict.get(k2).map(Smi::to_int), Some(200));
}
