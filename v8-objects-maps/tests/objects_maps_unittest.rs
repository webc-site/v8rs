// v8-objects-maps/tests/objects_maps_unittest.rs

//! 单元测试：全面验证 DescriptorArray、Transitions、PropertyArray 与 MapUpdater 核心逻辑。

use std::{mem::size_of, ptr, ptr::write};

use aok::{OK, Void};
use v8_objects_arrays::fixed_array::WeakFixedArray;
use v8_objects_core::{
  heap_object::HeapObject,
  instance_type::InstanceType,
  map::{Map, NO_SLACK_TRACKING},
  name::{HashFieldType, Name},
  property_details::{
    PropertyAttributes, PropertyConstness, PropertyDetails, PropertyKind, PropertyLocation,
    Representation,
  },
  smi::Smi,
  tagged::Tagged,
};
use v8_objects_maps::{
  Error,
  descriptor_array::{
    DescriptorArrayBuf, DescriptorArrayExt, DescriptorLookupCache, FastIterableState,
    K_MAX_ELEMENTS_FOR_LINEAR_SEARCH,
  },
  map_updater::MapUpdater,
  name_ext::NameTaggedExt,
  property_array::{K_MAX_HASH, K_MAX_LENGTH, PropertyArrayBuf, PropertyArrayExt},
  transitions::{Encoding, TransitionArrayExt, TransitionsAccessor},
};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

fn create_test_map(instance_type: InstanceType, instance_size_bytes: usize) -> Map {
  Map::new(Tagged::NULL, instance_type, instance_size_bytes)
}

fn create_test_name(hash: u32) -> Name {
  Name::with_hash_field(
    Tagged::NULL,
    Name::create_hash_field_value(hash, HashFieldType::Hash),
  )
}

/// 1. DescriptorArray 测试: 属性追加、堆排序、线性/二分查找与缓存查找
#[test]
fn test_descriptor_array_operations() -> Void {
  let mut meta_map = create_test_map(InstanceType::Map, 40);
  let meta_tagged: Tagged<Map> = Tagged::from_ptr(&raw mut meta_map);

  let array_buf = DescriptorArrayBuf::new(meta_tagged, 8);
  let tagged = array_buf.as_tagged();

  assert_eq!(tagged.number_of_all_descriptors(), 8);
  assert_eq!(tagged.number_of_descriptors(), 0);
  assert_eq!(tagged.fast_iterable(), FastIterableState::Unknown);

  tagged.set_fast_iterable(FastIterableState::JsonFast);
  assert_eq!(tagged.fast_iterable(), FastIterableState::JsonFast);

  // 构造 3 个不同哈希的属性名
  let mut name1 = create_test_name(300);
  let mut name2 = create_test_name(100);
  let mut name3 = create_test_name(200);

  let t_name1: Tagged<Name> = Tagged::from_ptr(&raw mut name1);
  let t_name2: Tagged<Name> = Tagged::from_ptr(&raw mut name2);
  let t_name3: Tagged<Name> = Tagged::from_ptr(&raw mut name3);

  assert_eq!(t_name1.hash(), 300);
  assert_eq!(t_name2.hash(), 100);
  assert_eq!(t_name3.hash(), 200);

  let details1 = PropertyDetails::for_fast(
    PropertyKind::Data,
    PropertyAttributes::NONE,
    PropertyLocation::Field,
    PropertyConstness::Const,
    Representation::smi(),
  );
  let details2 = PropertyDetails::for_fast(
    PropertyKind::Data,
    PropertyAttributes::READ_ONLY,
    PropertyLocation::Field,
    PropertyConstness::Mutable,
    Representation::tagged(),
  );
  let details3 = PropertyDetails::for_fast(
    PropertyKind::Data,
    PropertyAttributes::NONE,
    PropertyLocation::Field,
    PropertyConstness::Const,
    Representation::heap_object(),
  );

  let val1 = Smi::from_int(42).cast();
  let val2 = Smi::from_int(84).cast();
  let val3 = Smi::from_int(126).cast();

  tagged.append(t_name1, val1, details1)?;
  tagged.append(t_name2, val2, details2)?;
  tagged.append(t_name3, val3, details3)?;

  assert_eq!(tagged.number_of_descriptors(), 3);
  assert_eq!(tagged.get_key(0)?, t_name1);
  assert_eq!(tagged.get_key(1)?, t_name2);
  assert_eq!(tagged.get_key(2)?, t_name3);

  // 排序前线性查找
  assert_eq!(tagged.linear_search(t_name1, 3), Some(0));
  assert_eq!(tagged.linear_search(t_name2, 3), Some(1));
  assert_eq!(tagged.linear_search(t_name3, 3), Some(2));

  // 堆排序
  tagged.sort();

  // 排序后物理插入顺序不变
  assert_eq!(tagged.get_key(0)?, t_name1);
  assert_eq!(tagged.get_key(1)?, t_name2);
  assert_eq!(tagged.get_key(2)?, t_name3);

  // 排序后的排序键应按哈希升序排列 (100 -> 200 -> 300)
  assert_eq!(tagged.get_sorted_key(0), t_name2);
  assert_eq!(tagged.get_sorted_key(1), t_name3);
  assert_eq!(tagged.get_sorted_key(2), t_name1);

  // 二分查找
  assert_eq!(tagged.binary_search(t_name1, 3), Some(0));
  assert_eq!(tagged.binary_search(t_name2, 3), Some(1));
  assert_eq!(tagged.binary_search(t_name3, 3), Some(2));

  // 统一 search 入口
  assert_eq!(tagged.search(t_name2, 3), Some(1));

  // 查找缓存测试
  let mut cache = DescriptorLookupCache::new();
  let mut obj_map = create_test_map(InstanceType::JSObject, 32);
  obj_map.set_number_of_own_descriptors(3);
  let t_obj_map: Tagged<Map> = Tagged::from_ptr(&raw mut obj_map);

  let cached_res1 = tagged.search_with_cache(&mut cache, t_name2, t_obj_map);
  assert_eq!(cached_res1, Some(1));

  // 第二次查找命中缓存
  let cached_res2 = tagged.search_with_cache(&mut cache, t_name2, t_obj_map);
  assert_eq!(cached_res2, Some(1));

  OK
}

/// 2. Transitions 测试: 单一弱过渡演化到 TransitionArray 多分支及其动态扩容
#[test]
fn test_transitions_evolution_and_search() -> Void {
  let mut root_map = create_test_map(InstanceType::JSObject, 32);
  let t_root: Tagged<Map> = Tagged::from_ptr(&raw mut root_map);

  let accessor0 = TransitionsAccessor::new(t_root);
  assert_eq!(accessor0.encoding(), Encoding::Uninitialized);

  // 准备分支目标 Map 1
  let mut target_map1 = create_test_map(InstanceType::JSObject, 32);
  target_map1.set_number_of_own_descriptors(1);
  let desc_buf1 = DescriptorArrayBuf::new(Tagged::NULL, 4);
  let t_desc1 = desc_buf1.as_tagged();
  let mut name1 = create_test_name(10);
  let t_name1 = Tagged::from_ptr(&raw mut name1);
  let det1 = PropertyDetails::for_fast(
    PropertyKind::Data,
    PropertyAttributes::NONE,
    PropertyLocation::Field,
    PropertyConstness::Mutable,
    Representation::tagged(),
  );
  t_desc1.append(t_name1, Smi::from_int(1).cast(), det1)?;
  target_map1.instance_descriptors = t_desc1.cast();
  let t_target1: Tagged<Map> = Tagged::from_ptr(&raw mut target_map1);

  // 第一次插入过渡 -> 变为单一弱引用过渡
  let mut allocator = Vec::<u64>::with_capacity(128);
  TransitionsAccessor::insert(t_root, t_name1, t_target1, &mut allocator)?;

  let accessor1 = TransitionsAccessor::new(t_root);
  assert_eq!(accessor1.encoding(), Encoding::WeakRef);
  assert_eq!(accessor1.get_target_from_weak_ref(), Some(t_target1));
  assert_eq!(
    accessor1.search_transition(t_name1, PropertyKind::Data, PropertyAttributes::NONE),
    Some(t_target1)
  );

  // 准备分支目标 Map 2
  let mut target_map2 = create_test_map(InstanceType::JSObject, 32);
  target_map2.set_number_of_own_descriptors(1);
  let desc_buf2 = DescriptorArrayBuf::new(Tagged::NULL, 4);
  let t_desc2 = desc_buf2.as_tagged();
  let mut name2 = create_test_name(20);
  let t_name2 = Tagged::from_ptr(&raw mut name2);
  let det2 = PropertyDetails::for_fast(
    PropertyKind::Data,
    PropertyAttributes::NONE,
    PropertyLocation::Field,
    PropertyConstness::Mutable,
    Representation::tagged(),
  );
  t_desc2.append(t_name2, Smi::from_int(2).cast(), det2)?;
  target_map2.instance_descriptors = t_desc2.cast();
  let t_target2: Tagged<Map> = Tagged::from_ptr(&raw mut target_map2);

  // 第二次插入过渡 -> 裂变为多分支 TransitionArray (容量 4)
  TransitionsAccessor::insert(t_root, t_name2, t_target2, &mut allocator)?;

  let accessor2 = TransitionsAccessor::new(t_root);
  assert_eq!(accessor2.encoding(), Encoding::FullTransitionArray);
  let array2 = accessor2.transitions().ok_or(Error::NotFound)?;
  assert_eq!(array2.number_of_transitions(), 2);
  assert_eq!(array2.capacity(), 4);

  assert_eq!(
    accessor2.search_transition(t_name1, PropertyKind::Data, PropertyAttributes::NONE),
    Some(t_target1)
  );
  assert_eq!(
    accessor2.search_transition(t_name2, PropertyKind::Data, PropertyAttributes::NONE),
    Some(t_target2)
  );

  // 遍历过渡树
  let mut collected_targets = Vec::new();
  accessor2.for_each_transition(|m| {
    collected_targets.push(m);
  });
  assert_eq!(collected_targets.len(), 2);

  let mut tree_maps = Vec::new();
  TransitionsAccessor::traverse_transition_tree(t_root, |m| {
    tree_maps.push(m);
  });
  // 包含根节点与 2 个子节点
  assert_eq!(tree_maps.len(), 3);

  OK
}

/// 3. PropertyArray 测试: 槽位读写、CAS、哈希编码与切片批量复制
#[test]
fn test_property_array_operations() -> Void {
  let mut meta_map = create_test_map(InstanceType::Map, 40);
  let t_meta: Tagged<Map> = Tagged::from_ptr(&raw mut meta_map);

  let buf = PropertyArrayBuf::new(t_meta, 8)?;
  let tagged = buf.as_tagged();

  assert_eq!(tagged.length(), 8);
  assert_eq!(tagged.hash(), 0);

  // 设置哈希
  tagged.set_hash(12345)?;
  assert_eq!(tagged.hash(), 12345);
  assert_eq!(tagged.length(), 8);

  // 读写槽位
  let smi1 = Smi::from_int(101).cast();
  let smi2 = Smi::from_int(202).cast();
  tagged.set(0, smi1)?;
  tagged.set(1, smi2)?;

  assert_eq!(tagged.get(0)?, smi1);
  assert_eq!(tagged.get(1)?, smi2);

  // 元素交换 (swap_elements)
  tagged.swap_elements(0, 1)?;
  assert_eq!(tagged.get(0)?, smi2);
  assert_eq!(tagged.get(1)?, smi1);

  // CAS 操作
  let updated_smi = Smi::from_int(999).cast();
  let cas_res = tagged.compare_and_swap(0, smi2, updated_smi)?;
  assert_eq!(cas_res, smi2);
  assert_eq!(tagged.get(0)?, updated_smi);

  // 切片批量复制
  let buf2 = PropertyArrayBuf::new(t_meta, 8)?;
  let tagged2 = buf2.as_tagged();
  tagged2.copy_elements(0, tagged, 0, 2)?;
  assert_eq!(tagged2.get(0)?, updated_smi);
  assert_eq!(tagged2.get(1)?, smi1);

  OK
}

/// 4. MapUpdater 测试: 字段就地泛化、ElementsKind 重配置与 Inobject Slack Tracking
#[test]
fn test_map_updater_operations() -> Void {
  let mut map = create_test_map(InstanceType::JSObject, 32);
  map.set_number_of_own_descriptors(1);
  map.set_in_object_unused_property_fields(2);
  map.start_inobject_slack_tracking();
  assert!(map.is_inobject_slack_tracking_in_progress());

  let desc_buf = DescriptorArrayBuf::new(Tagged::NULL, 4);
  let t_desc = desc_buf.as_tagged();
  let mut name = create_test_name(50);
  let t_name = Tagged::from_ptr(&raw mut name);

  // 初始为 Smi + Const
  let details = PropertyDetails::for_fast(
    PropertyKind::Data,
    PropertyAttributes::NONE,
    PropertyLocation::Field,
    PropertyConstness::Const,
    Representation::smi(),
  );
  t_desc.append(t_name, Smi::from_int(10).cast(), details)?;
  map.instance_descriptors = t_desc.cast();

  let t_map: Tagged<Map> = Tagged::from_ptr(&raw mut map);

  // 就地泛化 Smi -> Tagged, Const -> Mutable
  let mut updater = MapUpdater::new(t_map);
  let updated_map = updater.reconfigure_to_data_field(
    0,
    PropertyAttributes::NONE,
    PropertyConstness::Mutable,
    Representation::tagged(),
  )?;

  assert_eq!(updated_map, t_map);
  assert!(!unsafe { (*t_map.as_ptr()).is_deprecated() });

  let cur_details = t_desc.get_details(0)?;
  assert_eq!(cur_details.constness(), PropertyConstness::Mutable);
  assert_eq!(cur_details.representation(), Representation::tagged());

  // ElementsKind 重配置
  updater.reconfigure_elements_kind(3)?;
  assert_eq!(unsafe { (*t_map.as_ptr()).elements_kind() }, 3);

  // 结束 Inobject Slack Tracking
  MapUpdater::complete_inobject_slack_tracking(t_map)?;
  assert!(!unsafe { (*t_map.as_ptr()).is_inobject_slack_tracking_in_progress() });
  assert_eq!(
    unsafe { (*t_map.as_ptr()).construction_counter() },
    NO_SLACK_TRACKING
  );

  OK
}

/// 5. DescriptorArray 堆排序各种元素个数（0, 1, 2, 4, 5）与哈希碰撞边界测试
#[test]
fn test_descriptor_array_heap_sort_edge_cases() -> Void {
  let mut meta_map = create_test_map(InstanceType::Map, 40);
  let meta_tagged: Tagged<Map> = Tagged::from_ptr(&raw mut meta_map);

  // 0 elements
  let buf0 = DescriptorArrayBuf::new(meta_tagged, 4);
  let tagged0 = buf0.as_tagged();
  tagged0.sort();
  assert_eq!(tagged0.number_of_descriptors(), 0);
  let mut test_name = create_test_name(123);
  assert_eq!(
    tagged0.binary_search(Tagged::from_ptr(&raw mut test_name), 0),
    None
  );

  // 1 element
  let buf1 = DescriptorArrayBuf::new(meta_tagged, 4);
  let tagged1 = buf1.as_tagged();
  let mut name1 = create_test_name(50);
  let t_name1 = Tagged::from_ptr(&raw mut name1);
  let det = PropertyDetails::for_fast(
    PropertyKind::Data,
    PropertyAttributes::NONE,
    PropertyLocation::Field,
    PropertyConstness::Const,
    Representation::smi(),
  );
  tagged1.append(t_name1, Smi::from_int(1).cast(), det)?;
  tagged1.sort();
  assert_eq!(tagged1.number_of_descriptors(), 1);
  assert_eq!(tagged1.get_sorted_key(0), t_name1);
  assert_eq!(tagged1.binary_search(t_name1, 1), Some(0));

  // 2 elements: reverse order [200, 100]
  let buf2 = DescriptorArrayBuf::new(meta_tagged, 4);
  let tagged2 = buf2.as_tagged();
  let mut name_200 = create_test_name(200);
  let mut name_100 = create_test_name(100);
  let t_200 = Tagged::from_ptr(&raw mut name_200);
  let t_100 = Tagged::from_ptr(&raw mut name_100);
  tagged2.append(t_200, Smi::from_int(200).cast(), det)?;
  tagged2.append(t_100, Smi::from_int(100).cast(), det)?;
  tagged2.sort();
  assert_eq!(tagged2.get_sorted_key(0), t_100);
  assert_eq!(tagged2.get_sorted_key(1), t_200);
  assert_eq!(tagged2.binary_search(t_100, 2), Some(1));
  assert_eq!(tagged2.binary_search(t_200, 2), Some(0));

  // 4 elements (even): [400, 100, 300, 200]
  let buf4 = DescriptorArrayBuf::new(meta_tagged, 4);
  let tagged4 = buf4.as_tagged();
  let mut name_400 = create_test_name(400);
  let mut name_300 = create_test_name(300);
  let t_400 = Tagged::from_ptr(&raw mut name_400);
  let t_300 = Tagged::from_ptr(&raw mut name_300);
  tagged4.append(t_400, Smi::from_int(4).cast(), det)?;
  tagged4.append(t_100, Smi::from_int(1).cast(), det)?;
  tagged4.append(t_300, Smi::from_int(3).cast(), det)?;
  tagged4.append(t_200, Smi::from_int(2).cast(), det)?;
  tagged4.sort();
  assert_eq!(tagged4.get_sorted_key(0), t_100);
  assert_eq!(tagged4.get_sorted_key(1), t_200);
  assert_eq!(tagged4.get_sorted_key(2), t_300);
  assert_eq!(tagged4.get_sorted_key(3), t_400);
  assert_eq!(tagged4.binary_search(t_100, 4), Some(1));
  assert_eq!(tagged4.binary_search(t_200, 4), Some(3));
  assert_eq!(tagged4.binary_search(t_300, 4), Some(2));
  assert_eq!(tagged4.binary_search(t_400, 4), Some(0));

  // 5 elements (odd): [500, 200, 400, 100, 300]
  let buf5 = DescriptorArrayBuf::new(meta_tagged, 5);
  let tagged5 = buf5.as_tagged();
  let mut name_500 = create_test_name(500);
  let t_500 = Tagged::from_ptr(&raw mut name_500);
  tagged5.append(t_500, Smi::from_int(5).cast(), det)?;
  tagged5.append(t_200, Smi::from_int(2).cast(), det)?;
  tagged5.append(t_400, Smi::from_int(4).cast(), det)?;
  tagged5.append(t_100, Smi::from_int(1).cast(), det)?;
  tagged5.append(t_300, Smi::from_int(3).cast(), det)?;
  tagged5.sort();
  assert_eq!(tagged5.get_sorted_key(0), t_100);
  assert_eq!(tagged5.get_sorted_key(1), t_200);
  assert_eq!(tagged5.get_sorted_key(2), t_300);
  assert_eq!(tagged5.get_sorted_key(3), t_400);
  assert_eq!(tagged5.get_sorted_key(4), t_500);

  // 哈希碰撞边界测试: 两个不同 Name 对象拥有相同哈希 250
  let buf_coll = DescriptorArrayBuf::new(meta_tagged, 4);
  let tagged_coll = buf_coll.as_tagged();
  let mut name_c1 = create_test_name(250);
  let mut name_c2 = create_test_name(250);
  let t_c1 = Tagged::from_ptr(&raw mut name_c1);
  let t_c2 = Tagged::from_ptr(&raw mut name_c2);
  tagged_coll.append(t_300, Smi::from_int(300).cast(), det)?;
  tagged_coll.append(t_c1, Smi::from_int(251).cast(), det)?;
  tagged_coll.append(t_100, Smi::from_int(100).cast(), det)?;
  tagged_coll.append(t_c2, Smi::from_int(252).cast(), det)?;
  tagged_coll.sort();
  assert_eq!(tagged_coll.get_sorted_key(0), t_100);
  assert_eq!(tagged_coll.get_sorted_key(3), t_300);
  assert_eq!(tagged_coll.binary_search(t_c1, 4), Some(1));
  assert_eq!(tagged_coll.binary_search(t_c2, 4), Some(3));
  assert_eq!(tagged_coll.binary_search(t_100, 4), Some(2));
  assert_eq!(tagged_coll.binary_search(t_300, 4), Some(0));

  // 查找不存在的 Name (相同哈希 250 但不同指针)
  let mut name_c_missing = create_test_name(250);
  let t_c_missing = Tagged::from_ptr(&raw mut name_c_missing);
  assert_eq!(tagged_coll.binary_search(t_c_missing, 4), None);

  // 查找哈希越界值
  let mut name_min_missing = create_test_name(50);
  assert_eq!(
    tagged_coll.binary_search(Tagged::from_ptr(&raw mut name_min_missing), 4),
    None
  );
  let mut name_max_missing = create_test_name(999);
  assert_eq!(
    tagged_coll.binary_search(Tagged::from_ptr(&raw mut name_max_missing), 4),
    None
  );

  OK
}

/// 6. DescriptorArray 跨越线性查找阈值（32 -> 35）的自动排序与二分查找测试
#[test]
fn test_descriptor_array_large_scale_and_threshold_crossing() -> Void {
  let mut meta_map = create_test_map(InstanceType::Map, 40);
  let meta_tagged: Tagged<Map> = Tagged::from_ptr(&raw mut meta_map);

  let buf = DescriptorArrayBuf::new(meta_tagged, 40);
  let tagged = buf.as_tagged();

  let det = PropertyDetails::for_fast(
    PropertyKind::Data,
    PropertyAttributes::NONE,
    PropertyLocation::Field,
    PropertyConstness::Const,
    Representation::smi(),
  );

  // 追加 35 个不同哈希的属性（哈希乱序打乱）
  let mut names = Vec::new();
  for i in 0..35 {
    let hash = ((i * 17 + 13) % 1000) as u32 + 10;
    let name = create_test_name(hash);
    names.push(name);
  }

  for (i, name) in names.iter_mut().enumerate() {
    let t_name = Tagged::from_ptr(name as *mut Name);
    tagged.append(t_name, Smi::from_int(i as i32).cast(), det)?;
  }

  assert_eq!(tagged.number_of_descriptors(), 35);
  assert!(tagged.number_of_descriptors() as usize > K_MAX_ELEMENTS_FOR_LINEAR_SEARCH);

  // 验证 search 入口在 >32 时自动走二分查找且全部命中
  for (i, name) in names.iter_mut().enumerate() {
    let t_name = Tagged::from_ptr(name as *mut Name);
    let found = tagged.search(t_name, 35);
    assert_eq!(found, Some(i));
    let binary_found = tagged.binary_search(t_name, 35);
    assert_eq!(binary_found, Some(i));
  }

  let mut absent_name = create_test_name(9999);
  assert_eq!(
    tagged.search(Tagged::from_ptr(&raw mut absent_name), 35),
    None
  );

  OK
}

/// 7. DescriptorLookupCache 命中、缺失、替换与 clear 测试
#[test]
fn test_descriptor_lookup_cache_comprehensive() -> Void {
  let mut cache = DescriptorLookupCache::new();
  let mut map1 = create_test_map(InstanceType::JSObject, 32);
  let t_map1: Tagged<Map> = Tagged::from_ptr(&raw mut map1);
  let mut name1 = create_test_name(100);
  let t_name1 = Tagged::from_ptr(&raw mut name1);

  // 初始 lookup 为 absent
  assert_eq!(
    cache.lookup(t_map1, t_name1),
    DescriptorLookupCache::K_ABSENT
  );

  // 更新为存在 (index 5)
  cache.update(t_map1, t_name1, 5);
  assert_eq!(cache.lookup(t_map1, t_name1), 5);

  // 更新为缺失 (-1)
  let mut name2 = create_test_name(200);
  let t_name2 = Tagged::from_ptr(&raw mut name2);
  cache.update(t_map1, t_name2, -1);
  assert_eq!(cache.lookup(t_map1, t_name2), -1);

  // clear 清空
  cache.clear();
  assert_eq!(
    cache.lookup(t_map1, t_name1),
    DescriptorLookupCache::K_ABSENT
  );
  assert_eq!(
    cache.lookup(t_map1, t_name2),
    DescriptorLookupCache::K_ABSENT
  );

  OK
}

/// 8. TransitionsAccessor 单弱引用同名更新、多分支裂变与扩容后原型/侧步过渡保持
#[test]
fn test_transitions_weak_ref_update_and_preservation() -> Void {
  let mut root_map = create_test_map(InstanceType::JSObject, 32);
  let t_root: Tagged<Map> = Tagged::from_ptr(&raw mut root_map);

  let mut target_map1 = create_test_map(InstanceType::JSObject, 32);
  target_map1.set_number_of_own_descriptors(1);
  let desc_buf1 = DescriptorArrayBuf::new(Tagged::NULL, 4);
  let t_desc1 = desc_buf1.as_tagged();
  let mut name1 = create_test_name(10);
  let t_name1 = Tagged::from_ptr(&raw mut name1);
  let det = PropertyDetails::for_fast(
    PropertyKind::Data,
    PropertyAttributes::NONE,
    PropertyLocation::Field,
    PropertyConstness::Mutable,
    Representation::tagged(),
  );
  t_desc1.append(t_name1, Smi::from_int(1).cast(), det)?;
  target_map1.instance_descriptors = t_desc1.cast();
  let t_target1: Tagged<Map> = Tagged::from_ptr(&raw mut target_map1);

  let mut allocator = Vec::<u64>::with_capacity(1024);
  TransitionsAccessor::insert(t_root, t_name1, t_target1, &mut allocator)?;

  let accessor1 = TransitionsAccessor::new(t_root);
  assert_eq!(accessor1.encoding(), Encoding::WeakRef);
  assert_eq!(accessor1.get_target_from_weak_ref(), Some(t_target1));

  // 同名过渡再次插入到 WeakRef -> 原地更新 target，保持 WeakRef
  let mut target_map1_v2 = create_test_map(InstanceType::JSObject, 32);
  target_map1_v2.set_number_of_own_descriptors(1);
  target_map1_v2.instance_descriptors = t_desc1.cast();
  let t_target1_v2: Tagged<Map> = Tagged::from_ptr(&raw mut target_map1_v2);

  TransitionsAccessor::insert(t_root, t_name1, t_target1_v2, &mut allocator)?;
  let accessor1_updated = TransitionsAccessor::new(t_root);
  assert_eq!(accessor1_updated.encoding(), Encoding::WeakRef);
  assert_eq!(
    accessor1_updated.get_target_from_weak_ref(),
    Some(t_target1_v2)
  );

  // 插入第二条不同名称过渡 -> 演化为 TransitionArray (容量 4)
  let mut target_map2 = create_test_map(InstanceType::JSObject, 32);
  target_map2.set_number_of_own_descriptors(1);
  let desc_buf2 = DescriptorArrayBuf::new(Tagged::NULL, 4);
  let t_desc2 = desc_buf2.as_tagged();
  let mut name2 = create_test_name(20);
  let t_name2 = Tagged::from_ptr(&raw mut name2);
  t_desc2.append(t_name2, Smi::from_int(2).cast(), det)?;
  target_map2.instance_descriptors = t_desc2.cast();
  let t_target2: Tagged<Map> = Tagged::from_ptr(&raw mut target_map2);

  TransitionsAccessor::insert(t_root, t_name2, t_target2, &mut allocator)?;
  let accessor2 = TransitionsAccessor::new(t_root);
  assert_eq!(accessor2.encoding(), Encoding::FullTransitionArray);
  let array2 = accessor2.transitions().ok_or(Error::NotFound)?;
  assert_eq!(array2.number_of_transitions(), 2);
  assert_eq!(array2.capacity(), 4);

  // 为 TransitionArray 附加原型过渡与侧步过渡
  let mut proto_storage = vec![0u64; 8];
  let p_ptr = proto_storage.as_mut_ptr() as usize;
  unsafe {
    ptr::write(
      p_ptr as *mut WeakFixedArray,
      WeakFixedArray::new(Tagged::NULL, 4),
    );
  }
  let proto_tagged: Tagged<WeakFixedArray> = unsafe { Tagged::from_raw_address(p_ptr) };
  array2.set_prototype_transitions(proto_tagged);

  let mut side_storage = vec![0u64; 8];
  let s_ptr = side_storage.as_mut_ptr() as usize;
  unsafe {
    ptr::write(
      s_ptr as *mut WeakFixedArray,
      WeakFixedArray::new(Tagged::NULL, 4),
    );
  }
  let side_tagged: Tagged<WeakFixedArray> = unsafe { Tagged::from_raw_address(s_ptr) };
  array2.set_side_step_transitions(side_tagged);

  assert!(accessor2.has_prototype_transitions());
  assert!(accessor2.has_side_step_transitions());
  assert_eq!(accessor2.get_prototype_transitions(), Some(proto_tagged));
  assert_eq!(accessor2.get_side_step_transitions(), Some(side_tagged));

  // 连续插入第 3, 4, 5 个过渡，触发容量从 4 翻倍扩容至 8
  let mut names: Vec<Name> = (3..=5).map(|i| create_test_name(i * 10)).collect();
  let desc_bufs: Vec<DescriptorArrayBuf> = (3..=5)
    .map(|_| DescriptorArrayBuf::new(Tagged::NULL, 4))
    .collect();
  let mut target_maps: Vec<Map> = (3..=5)
    .map(|_| create_test_map(InstanceType::JSObject, 32))
    .collect();

  for i in 0..3 {
    let t_d = desc_bufs[i].as_tagged();
    let t_nm = Tagged::from_ptr(&raw mut names[i]);
    t_d.append(t_nm, Smi::from_int((i + 3) as i32).cast(), det)?;
    target_maps[i].set_number_of_own_descriptors(1);
    target_maps[i].instance_descriptors = t_d.cast();
    let t_tm = Tagged::from_ptr(&raw mut target_maps[i]);
    TransitionsAccessor::insert(t_root, t_nm, t_tm, &mut allocator)?;
  }

  let accessor_grown = TransitionsAccessor::new(t_root);
  assert_eq!(accessor_grown.encoding(), Encoding::FullTransitionArray);
  let array_grown = accessor_grown.transitions().ok_or(Error::NotFound)?;
  assert_eq!(array_grown.number_of_transitions(), 5);
  assert!(array_grown.capacity() >= 8);

  // 扩容后原型过渡与侧步过渡必须完整保留
  assert!(accessor_grown.has_prototype_transitions());
  assert!(accessor_grown.has_side_step_transitions());
  assert_eq!(
    accessor_grown.get_prototype_transitions(),
    Some(proto_tagged)
  );
  assert_eq!(
    accessor_grown.get_side_step_transitions(),
    Some(side_tagged)
  );

  OK
}

/// 9. PropertyArray 容量、哈希界限与越界防卫测试
#[test]
fn test_property_array_boundaries_and_errors() -> Void {
  let mut meta_map = create_test_map(InstanceType::Map, 40);
  let t_meta: Tagged<Map> = Tagged::from_ptr(&raw mut meta_map);

  // 长度边界
  let buf0 = PropertyArrayBuf::new(t_meta, 0)?;
  assert_eq!(buf0.as_tagged().length(), 0);

  let buf_max = PropertyArrayBuf::new(t_meta, K_MAX_LENGTH)?;
  assert_eq!(buf_max.as_tagged().length(), K_MAX_LENGTH);

  // 超出最大容量 1023
  assert!(PropertyArrayBuf::new(t_meta, K_MAX_LENGTH + 1).is_err());

  // 哈希边界
  let buf = PropertyArrayBuf::new(t_meta, 4)?;
  let tagged = buf.as_tagged();
  tagged.set_hash(0)?;
  assert_eq!(tagged.hash(), 0);

  tagged.set_hash(K_MAX_HASH as i32)?;
  assert_eq!(tagged.hash(), K_MAX_HASH as i32);

  assert!(matches!(
    tagged.set_hash((K_MAX_HASH + 1) as i32),
    Err(Error::HashOverflow(_))
  ));
  assert!(matches!(tagged.set_hash(-1), Err(Error::HashOverflow(_))));

  // 越界保护
  let smi1 = Smi::from_int(10).cast();
  assert!(matches!(tagged.get(4), Err(Error::OutOfBounds { .. })));
  assert!(matches!(
    tagged.set(4, smi1),
    Err(Error::OutOfBounds { .. })
  ));
  assert!(matches!(
    tagged.swap(4, smi1),
    Err(Error::OutOfBounds { .. })
  ));
  assert!(matches!(
    tagged.compare_and_swap(4, smi1, smi1),
    Err(Error::OutOfBounds { .. })
  ));

  // copy_elements 边界
  let buf_dst = PropertyArrayBuf::new(t_meta, 4)?;
  let tagged_dst = buf_dst.as_tagged();
  assert!(matches!(
    tagged_dst.copy_elements(3, tagged, 0, 2),
    Err(Error::OutOfBounds { .. })
  ));
  assert!(matches!(
    tagged_dst.copy_elements(0, tagged, 3, 2),
    Err(Error::OutOfBounds { .. })
  ));

  OK
}

/// 10. MapUpdater 深度原型/过渡链 RootMap 搜索、字段拥有者与多分支树泛化
#[test]
fn test_map_updater_deep_tree_and_field_owner() -> Void {
  let mut meta_map = create_test_map(InstanceType::Map, 40);
  let meta_tagged: Tagged<Map> = Tagged::from_ptr(&raw mut meta_map);

  // 构建三层 Map: Map0 -> Map1 -> Map2
  let mut map0 = create_test_map(InstanceType::JSObject, 32);
  map0.header.map = meta_tagged;
  map0.set_number_of_own_descriptors(1);
  let desc_buf0 = DescriptorArrayBuf::new(Tagged::NULL, 4);
  let t_desc0 = desc_buf0.as_tagged();
  let mut name0 = create_test_name(10);
  let t_name0 = Tagged::from_ptr(&raw mut name0);
  let det0 = PropertyDetails::for_fast(
    PropertyKind::Data,
    PropertyAttributes::NONE,
    PropertyLocation::Field,
    PropertyConstness::Const,
    Representation::smi(),
  );
  t_desc0.append(t_name0, Smi::from_int(1).cast(), det0)?;
  map0.instance_descriptors = t_desc0.cast();
  let t_map0: Tagged<Map> = Tagged::from_ptr(&raw mut map0);

  let mut map1 = create_test_map(InstanceType::JSObject, 32);
  map1.header.map = meta_tagged;
  map1.set_number_of_own_descriptors(1);
  map1.instance_descriptors = t_desc0.cast();
  map1.constructor_or_back_pointer_or_native_context = t_map0.cast();
  let t_map1: Tagged<Map> = Tagged::from_ptr(&raw mut map1);

  let mut map2 = create_test_map(InstanceType::JSObject, 32);
  map2.header.map = meta_tagged;
  map2.set_number_of_own_descriptors(2);
  let desc_buf2 = DescriptorArrayBuf::new(Tagged::NULL, 4);
  let t_desc2 = desc_buf2.as_tagged();
  let mut name1 = create_test_name(20);
  let t_name1 = Tagged::from_ptr(&raw mut name1);
  let det1 = PropertyDetails::for_fast(
    PropertyKind::Data,
    PropertyAttributes::NONE,
    PropertyLocation::Field,
    PropertyConstness::Const,
    Representation::smi(),
  );
  t_desc2.append(t_name0, Smi::from_int(1).cast(), det0)?;
  t_desc2.append(t_name1, Smi::from_int(2).cast(), det1)?;
  map2.instance_descriptors = t_desc2.cast();
  map2.constructor_or_back_pointer_or_native_context = t_map1.cast();
  let t_map2: Tagged<Map> = Tagged::from_ptr(&raw mut map2);

  // find_root_map: 从 map2 沿 back pointer 回溯到 map0
  assert_eq!(MapUpdater::find_root_map(t_map2), t_map0);
  assert_eq!(MapUpdater::find_root_map(t_map1), t_map0);

  // find_field_owner: 描述符 0 在 map0 引入；描述符 1 在 map2 引入
  assert_eq!(MapUpdater::find_field_owner(t_map2, 0), t_map0);
  assert_eq!(MapUpdater::find_field_owner(t_map2, 1), t_map2);

  // 跨过渡树泛化: 将 map0 关联过渡树到 map1
  let mut allocator = Vec::<u64>::new();
  TransitionsAccessor::insert(t_map0, t_name1, t_map1, &mut allocator)?;

  // 泛化字段 0: Const + Smi -> Mutable + Tagged
  MapUpdater::generalize_field(
    t_map1,
    0,
    PropertyConstness::Mutable,
    Representation::tagged(),
  )?;

  let details_root = t_desc0.get_details(0)?;
  assert_eq!(details_root.constness(), PropertyConstness::Mutable);
  assert_eq!(details_root.representation(), Representation::tagged());

  OK
}

/// 11. TransitionsAccessor 插入 35 个多分支过渡，验证跨越 32 门槛自动触发排序及二分查找全命中
#[test]
fn test_transitions_threshold_crossing_35_branches() -> Void {
  let mut root_map = create_test_map(InstanceType::JSObject, 32);
  let t_root: Tagged<Map> = Tagged::from_ptr(&raw mut root_map);

  let mut allocator = Vec::<u64>::with_capacity(4096);
  let mut names = Vec::new();
  let mut target_maps = Vec::new();
  let mut desc_bufs = Vec::new();

  let det = PropertyDetails::for_fast(
    PropertyKind::Data,
    PropertyAttributes::NONE,
    PropertyLocation::Field,
    PropertyConstness::Mutable,
    Representation::tagged(),
  );

  for i in 0..35 {
    let hash = ((i * 19 + 7) % 1000) as u32 + 10;
    let name = create_test_name(hash);
    names.push(name);

    let desc_buf = DescriptorArrayBuf::new(Tagged::NULL, 4);
    let mut target = create_test_map(InstanceType::JSObject, 32);
    target.set_number_of_own_descriptors(1);
    desc_bufs.push(desc_buf);
    target_maps.push(target);
  }

  // 模拟地址契约: 循环外一次性提升元素区裸指针, 循环内不得再经 IndexMut 重借,
  // 否则 Stacked Borrows 会弹出上一轮暴露标签, 令 insert 内旧 Tagged 解引用成 UB
  let names_ptr = names.as_mut_ptr();
  let targets_ptr = target_maps.as_mut_ptr();

  for (i, desc_buf) in desc_bufs.iter_mut().enumerate().take(35) {
    // SAFETY: i < 35 == 各 Vec 已建长度，裸指针偏移均落在元素区内
    let t_nm = unsafe { Tagged::from_ptr(names_ptr.add(i)) };
    desc_buf
      .as_tagged()
      .append(t_nm, Smi::from_int(i as i32).cast(), det)?;
    unsafe {
      (*targets_ptr.add(i)).instance_descriptors = desc_buf.as_tagged().cast();
    }
    let t_target = unsafe { Tagged::from_ptr(targets_ptr.add(i)) };
    TransitionsAccessor::insert(t_root, t_nm, t_target, &mut allocator)?;
  }

  let accessor = TransitionsAccessor::new(t_root);
  assert_eq!(accessor.encoding(), Encoding::FullTransitionArray);
  let array = accessor.transitions().ok_or(Error::NotFound)?;
  assert_eq!(array.number_of_transitions(), 35);
  assert!(array.capacity() >= 35);

  // 跨越 32 门限后已自动触发 sort(true)，所有 35 个分支必须通过二分查找准确检索到
  for i in 0..35 {
    // SAFETY: 同上，偏移落在元素区内
    let t_nm = unsafe { Tagged::from_ptr(names_ptr.add(i)) };
    let t_target = unsafe { Tagged::from_ptr(targets_ptr.add(i)) };
    let found = accessor.search_transition(t_nm, PropertyKind::Data, PropertyAttributes::NONE);
    assert_eq!(found, Some(t_target));
  }

  let mut absent_name = create_test_name(9999);
  assert_eq!(
    accessor.search_transition(
      Tagged::from_ptr(&raw mut absent_name),
      PropertyKind::Data,
      PropertyAttributes::NONE
    ),
    None
  );

  OK
}

/// 12. TransitionsAccessor 同名不同属性共存、同名同属性就地更新与 OutOfMemory 防御
#[test]
fn test_transitions_same_name_different_attributes_and_oom() -> Void {
  let mut root_map = create_test_map(InstanceType::JSObject, 32);
  let t_root: Tagged<Map> = Tagged::from_ptr(&raw mut root_map);

  let mut shared_name = create_test_name(888);
  let t_shared_name = Tagged::from_ptr(&raw mut shared_name);

  // 1. 第一个过渡: (Data, NONE)
  let mut target1 = create_test_map(InstanceType::JSObject, 32);
  target1.set_number_of_own_descriptors(1);
  let desc1 = DescriptorArrayBuf::new(Tagged::NULL, 4);
  let det1 = PropertyDetails::for_fast(
    PropertyKind::Data,
    PropertyAttributes::NONE,
    PropertyLocation::Field,
    PropertyConstness::Mutable,
    Representation::tagged(),
  );
  desc1
    .as_tagged()
    .append(t_shared_name, Smi::from_int(1).cast(), det1)?;
  target1.instance_descriptors = desc1.as_tagged().cast();
  let t_target1 = Tagged::from_ptr(&raw mut target1);

  let mut allocator = Vec::<u64>::with_capacity(512);
  TransitionsAccessor::insert(t_root, t_shared_name, t_target1, &mut allocator)?;

  let acc1 = TransitionsAccessor::new(t_root);
  assert_eq!(acc1.encoding(), Encoding::WeakRef);

  // 2. 第二个过渡: 同名但不同属性 (Data, READ_ONLY) -> 无法原地更新，必须裂变为 TransitionArray
  let mut target2 = create_test_map(InstanceType::JSObject, 32);
  target2.set_number_of_own_descriptors(1);
  let desc2 = DescriptorArrayBuf::new(Tagged::NULL, 4);
  let det2 = PropertyDetails::for_fast(
    PropertyKind::Data,
    PropertyAttributes::READ_ONLY,
    PropertyLocation::Field,
    PropertyConstness::Mutable,
    Representation::tagged(),
  );
  desc2
    .as_tagged()
    .append(t_shared_name, Smi::from_int(2).cast(), det2)?;
  target2.instance_descriptors = desc2.as_tagged().cast();
  let t_target2 = Tagged::from_ptr(&raw mut target2);

  TransitionsAccessor::insert(t_root, t_shared_name, t_target2, &mut allocator)?;

  let acc2 = TransitionsAccessor::new(t_root);
  assert_eq!(acc2.encoding(), Encoding::FullTransitionArray);
  let array2 = acc2.transitions().ok_or(Error::NotFound)?;
  assert_eq!(array2.number_of_transitions(), 2);

  // 两个同名但不同属性的过渡必须同时能检索到
  assert_eq!(
    acc2.search_transition(t_shared_name, PropertyKind::Data, PropertyAttributes::NONE),
    Some(t_target1)
  );
  assert_eq!(
    acc2.search_transition(
      t_shared_name,
      PropertyKind::Data,
      PropertyAttributes::READ_ONLY
    ),
    Some(t_target2)
  );

  // 3. 第三个过渡: 同名同属性 (Data, NONE) 的新目标 target1_v2 -> 原地更新 target1
  let mut target1_v2 = create_test_map(InstanceType::JSObject, 32);
  target1_v2.set_number_of_own_descriptors(1);
  target1_v2.instance_descriptors = desc1.as_tagged().cast();
  let t_target1_v2 = Tagged::from_ptr(&raw mut target1_v2);

  TransitionsAccessor::insert(t_root, t_shared_name, t_target1_v2, &mut allocator)?;
  assert_eq!(array2.number_of_transitions(), 2);
  assert_eq!(
    acc2.search_transition(t_shared_name, PropertyKind::Data, PropertyAttributes::NONE),
    Some(t_target1_v2)
  );

  // 4. OutOfMemory 防御测试: 限制 allocator 容量为当前已分配大小
  let mut exhausted_allocator = Vec::<u64>::with_capacity(0); // 0 capacity 立即 OOM
  let mut new_name = create_test_name(999);
  let mut target_oom = create_test_map(InstanceType::JSObject, 32);
  target_oom.set_number_of_own_descriptors(1);
  let desc_oom = DescriptorArrayBuf::new(Tagged::NULL, 4);
  desc_oom.as_tagged().append(
    Tagged::from_ptr(&raw mut new_name),
    Smi::from_int(3).cast(),
    det1,
  )?;
  target_oom.instance_descriptors = desc_oom.as_tagged().cast();
  let t_target_oom = Tagged::from_ptr(&raw mut target_oom);

  // 当前容量为 4，已用 2。插入 2 个填满容量
  let mut n3 = create_test_name(333);
  let mut n4 = create_test_name(444);
  TransitionsAccessor::insert(
    t_root,
    Tagged::from_ptr(&raw mut n3),
    t_target1,
    &mut allocator,
  )?;
  TransitionsAccessor::insert(
    t_root,
    Tagged::from_ptr(&raw mut n4),
    t_target1,
    &mut allocator,
  )?;
  assert_eq!(array2.number_of_transitions(), 4);
  assert_eq!(array2.capacity(), 4);

  // 第 5 个过渡需要扩容，但传入 exhausted_allocator -> 触发 OutOfMemory
  let oom_res = TransitionsAccessor::insert(
    t_root,
    Tagged::from_ptr(&raw mut new_name),
    t_target_oom,
    &mut exhausted_allocator,
  );
  assert!(matches!(oom_res, Err(Error::OutOfMemory)));

  // 原 TransitionArray 与 Map 状态未受破坏
  let acc_after_oom = TransitionsAccessor::new(t_root);
  assert_eq!(acc_after_oom.encoding(), Encoding::FullTransitionArray);
  let array_after = acc_after_oom.transitions().ok_or(Error::NotFound)?;
  assert_eq!(array_after.number_of_transitions(), 4);

  OK
}

// ─────────────────────────────────────────────────────────────────────────
// W6O2 语义面回填测试: FieldIndex / PropertyDescriptor::ToAttributes /
// MegaDomHandler / PropertyDescriptorObject inline 存取器
// ─────────────────────────────────────────────────────────────────────────

use v8_objects_maps::{
  field_index::{self as fi, Encoding as FiEncoding, FieldIndex},
  field_index_inl as fii,
  megadom_handler_inl::{MegaDomHandler, MegaDomHandlerExt},
  property_descriptor::PropertyDescriptor,
  property_descriptor_object_inl::PropertyDescriptorObject,
};

/// 2.1 FieldIndex 位段编解码 round-trip (对齐 field-index.h 段序)。
#[test]
fn test_field_index_bitfield_roundtrip() -> Void {
  // inobject, offset=12 字节 (词3), kDouble, inobject_count=5, first_field_offset=8
  let fi = FieldIndex::new(true, 12, FiEncoding::Double, 5, 8);
  assert!(fi.is_inobject());
  assert!(fi.is_double());
  assert_eq!(fi.encoding(), FiEncoding::Double);
  assert_eq!(fi.offset(), 12);
  assert_eq!(fi.inobject_property_count(), 5);
  assert_eq!(fi.first_field_offset_in_storage(), 8);
  // offset_in_words = 12/kTaggedSize(4) = 3
  assert_eq!(fi.offset_in_words(), 3);
  // property_index = (12-8)/4 = 1 (inobject)
  assert_eq!(fi.property_index(), 1);
  // stub key 仅保留 IsInObject|Encoding|Offset 三段
  let stub = fi.get_field_access_stub_key();
  assert_eq!(
    stub & fi::IS_IN_OBJECT_BITS_MASK,
    1u64 << fi::IS_IN_OBJECT_BITS_SHIFT
  );
  assert_eq!(
    stub & fi::ENCODING_BITS_MASK,
    (FiEncoding::Double as u64) << fi::ENCODING_BITS_SHIFT
  );

  // out-of-object: is_inobject=false, inobject_count=2, first_field_offset=8(词2*4)
  let fo = FieldIndex::new(false, 16, FiEncoding::Tagged, 2, 8);
  assert!(!fo.is_inobject());
  assert!(!fo.is_double());
  assert_eq!(fo.offset(), 16);
  assert_eq!(fo.offset_in_words(), 4);
  // outobject_array_index = offset_in_words - PROPERTY_ARRAY_DATA_START_IN_WORDS(2) = 2
  assert_eq!(fo.outobject_array_index(), 2);
  // property_index = (16-8)/4 + 2 = 4
  assert_eq!(fo.property_index(), 4);

  // 签名 1:1: zeroed 全段为 0
  let z = FieldIndex::zeroed();
  assert!(!z.is_inobject());
  assert_eq!(z.offset(), 0);
  assert_eq!(z.encoding(), FiEncoding::Tagged);

  OK
}

/// 2.2 ForInObjectOffset 纯构造 (field-index-inl.h:21 → FieldIndex(true, offset, enc, 0, 0))。
#[test]
fn test_field_index_for_in_object_offset() -> Void {
  let tagged = fii::for_in_object_offset(16, FiEncoding::Tagged);
  assert!(tagged.is_inobject());
  assert_eq!(tagged.offset(), 16);
  assert_eq!(tagged.encoding(), FiEncoding::Tagged);
  assert_eq!(tagged.inobject_property_count(), 0);
  assert_eq!(tagged.first_field_offset_in_storage(), 0);

  let dbl = fii::for_in_object_offset(24, FiEncoding::Double);
  assert!(dbl.is_double());
  assert_eq!(dbl.offset(), 24);
  // get_load_by_field_index: inobject, offset_in_words=6, header=3 → (6-3)<<1 = 6, double 位=1 → 7
  assert_eq!(dbl.get_load_by_field_index_self(), 7);

  OK
}

/// 2.3 GetLoadByFieldIndex 编码规则 (field-index-inl.h:76)。
#[test]
fn test_field_index_get_load_by_field_index() -> Void {
  // inobject 首槽 (offset_in_words == JS_OBJECT_HEADER_SIZE_IN_WORDS) → 0
  assert_eq!(
    fii::get_load_by_field_index(fi::JS_OBJECT_HEADER_SIZE_IN_WORDS, true, false),
    0
  );
  // inobject 次槽 + double box → (1<<1)|1 = 3
  assert_eq!(
    fii::get_load_by_field_index(fi::JS_OBJECT_HEADER_SIZE_IN_WORDS + 1, true, true),
    3
  );
  // out-object 首槽 → -raw-1, raw=0 → -1; <<1 → -2
  assert_eq!(
    fii::get_load_by_field_index(fi::PROPERTY_ARRAY_DATA_START_IN_WORDS, false, false),
    -2
  );
  // out-object 次槽 raw=1 → -2; <<1 → -4
  assert_eq!(
    fii::get_load_by_field_index(fi::PROPERTY_ARRAY_DATA_START_IN_WORDS + 1, false, false),
    -4
  );
  OK
}

/// 2.4 PropertyDescriptor::ToAttributes (property-descriptor.h:112)。
#[test]
fn test_property_descriptor_to_attributes() -> Void {
  use v8_objects_core::property_details::PropertyAttributes;

  // 全默认 (has_* 皆 false) → NONE
  let d = PropertyDescriptor::empty();
  assert_eq!(d.to_attributes(), PropertyAttributes::NONE);

  // has_enumerable 且 enumerable=false → DONT_ENUM
  let mut d = PropertyDescriptor::empty();
  d.has_enumerable = true;
  d.enumerable = false;
  assert_eq!(d.to_attributes(), PropertyAttributes::DONT_ENUM);

  // 显式 enumerable=true → 不置 DONT_ENUM
  let mut d = PropertyDescriptor::empty();
  d.has_enumerable = true;
  d.enumerable = true;
  assert_eq!(d.to_attributes(), PropertyAttributes::NONE);

  // has_configurable && !configurable → DONT_DELETE
  let mut d = PropertyDescriptor::empty();
  d.has_configurable = true;
  d.configurable = false;
  assert_eq!(d.to_attributes(), PropertyAttributes::DONT_DELETE);

  // has_writable && !writable → READ_ONLY
  let mut d = PropertyDescriptor::empty();
  d.has_writable = true;
  d.writable = false;
  assert_eq!(d.to_attributes(), PropertyAttributes::READ_ONLY);

  // 组合: 全非枚举/非配置/只读 → DONT_ENUM|DONT_DELETE|READ_ONLY (=7)
  let mut d = PropertyDescriptor::empty();
  d.has_enumerable = true;
  d.has_configurable = true;
  d.has_writable = true;
  assert_eq!(d.to_attributes(), PropertyAttributes(7));

  OK
}

/// 2.5 MegaDomHandler accessor/context 槽存取 (megadom-handler-inl.h)。
#[test]
fn test_megadom_handler_slots() -> Void {
  use v8_objects_core::{objects::WriteBarrierMode, tagged::MaybeObject};

  // 构造一个可写的宿主缓冲区 (3 个 usize 宽字段: map/accessor/context)。
  let mut buf = vec![0usize; size_of::<MegaDomHandler>() / size_of::<usize>()];
  let base = buf.as_mut_ptr() as usize;
  let host: Tagged<MegaDomHandler> =
    // SAFETY: buf 为对齐的可写缓冲区, 覆盖 MegaDomHandler 全字段。
    unsafe { Tagged::from_raw_address(base) };

  let marker = Tagged::<MaybeObject>::new(0x1234);
  host.set_accessor(marker, WriteBarrierMode::UpdateWriteBarrier);
  assert_eq!(host.accessor(), marker);
  assert_eq!(host.accessor_acquire(), marker);

  let released = Tagged::<MaybeObject>::new(0x5678);
  host.set_accessor_release(released, WriteBarrierMode::UpdateWriteBarrier);
  assert_eq!(host.accessor(), released);

  let ctx = Tagged::<MaybeObject>::new(0x9abc);
  host.set_context(ctx, WriteBarrierMode::UpdateWriteBarrier);
  assert_eq!(host.context(), ctx);

  OK
}

/// 2.6 PropertyDescriptorObject flags/value/get/set 存取 (property-descriptor-object-inl.h)。
#[test]
fn test_property_descriptor_object_accessors() -> Void {
  use v8_objects_core::{objects::WriteBarrierMode, tagged::Object};
  use v8_objects_maps::property_descriptor_object_inl::PropertyDescriptorObjectExt;

  let mut pdo = PropertyDescriptorObject {
    header: HeapObject::new(Tagged::NULL),
    flags: Smi::from_int(0),
    value: Tagged::<Object>::NULL,
    get: Tagged::<Object>::NULL,
    set: Tagged::<Object>::NULL,
  };

  // flags Smi 装箱 round-trip
  pdo.set_flags(0b1_1010_0101); // 各位混合 (= 421)
  assert_eq!(pdo.flags(), 0b1_1010_0101);
  assert_eq!(pdo.flags(), 421);
  assert!(pdo.is_enumerable());
  assert!(!pdo.has_enumerable());
  assert!(pdo.is_configurable());
  assert!(!pdo.has_configurable());
  assert!(!pdo.is_writable());
  assert!(pdo.has_writable());
  assert!(!pdo.has_value());
  assert!(pdo.has_get());
  assert!(pdo.has_set());
  assert!(pdo.is_data_descriptor()); // has_writable is true
  assert!(pdo.is_accessor_descriptor()); // has_get & has_set are true
  assert!(!pdo.is_generic_descriptor());

  // value/get/set 透传
  let hole_like = Tagged::<Object>::new(0x22);
  pdo.set_value(hole_like, WriteBarrierMode::UpdateWriteBarrier);
  assert_eq!(pdo.value(), hole_like);
  let g = Tagged::<Object>::new(0x33);
  pdo.set_get(g, WriteBarrierMode::UpdateWriteBarrier);
  assert_eq!(pdo.get(), g);
  let s = Tagged::<Object>::new(0x44);
  pdo.set_set(s, WriteBarrierMode::UpdateWriteBarrier);
  assert_eq!(pdo.set(), s);

  // 布局: Struct 继承 HeapObject 1 槽 + 4 数据槽 = 5 槽
  assert_eq!(PropertyDescriptorObject::slot_count(), 5);

  // 测试 Tagged<PropertyDescriptorObject> 堆上读写扩展
  let mut storage = vec![0usize; PropertyDescriptorObject::slot_count()];
  let base_ptr = storage.as_mut_ptr() as *mut PropertyDescriptorObject;
  unsafe {
    write(base_ptr, pdo);
  }
  let tagged =
    unsafe { Tagged::<PropertyDescriptorObject>::from_raw_address(storage.as_ptr() as usize) };
  assert_eq!(tagged.flags(), 421);
  assert_eq!(tagged.value(), hole_like);
  assert_eq!(tagged.get(), g);
  assert_eq!(tagged.get_set(), s);
  assert_eq!(tagged.set(), s);
  assert!(tagged.is_enumerable());
  assert!(!tagged.has_enumerable());
  assert!(tagged.is_configurable());
  assert!(!tagged.has_configurable());
  assert!(!tagged.is_writable());
  assert!(tagged.has_writable());
  assert!(!tagged.has_value());
  assert!(tagged.has_get());
  assert!(tagged.has_set());
  assert!(tagged.is_data_descriptor());
  assert!(tagged.is_accessor_descriptor());
  assert!(!tagged.is_generic_descriptor());

  tagged.set_flags(123);
  assert_eq!(tagged.flags(), 123);

  OK
}

/// 台账转断言 (readme/zh.md MU-ISO, map-updater.cc:27 EqualImmutableValues 波 10 填体):
/// 原版为纯 Tagged 同址比较 (obj1 == obj2), usize 地址契约下等价于位相等。
#[test]
fn test_map_updater_generated_equal_immutable_values() -> Void {
  use v8_objects_maps::map_updater_cc_generated::equal_immutable_values;

  let mut map_a = create_test_map(InstanceType::JSObject, 32);
  let t_a: Tagged<Map> = Tagged::from_ptr(&raw mut map_a);
  let mut map_b = create_test_map(InstanceType::JSObject, 32);
  let t_b: Tagged<Map> = Tagged::from_ptr(&raw mut map_b);

  // 同址 ⇒ true (原版 :29 return obj1 == obj2)
  assert!(equal_immutable_values(t_a.ptr(), t_a.ptr()));
  // 异址 ⇒ false
  assert!(!equal_immutable_values(t_a.ptr(), t_b.ptr()));
  // 纯位比较: hole/undefined 在堆模拟下即不同编码位, 不等价语义由位相等承载
  assert!(equal_immutable_values(0, 0));
  assert!(!equal_immutable_values(0, 1));
  OK
}
