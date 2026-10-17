// v8-objects-maps/tests/test_map_updater_and_deps.rs
//! 针对 MapUpdater 状态机、DependentCode 去优化、FieldIndex::ForSmiLoadHandler
//! 及 PrototypeIterator 的集成测试。

use aok::{OK, Void};
use v8_objects_core::{
  instance_type::InstanceType,
  map::Map,
  property_details::{Kind, Representation},
  tagged::Tagged,
};
use v8_objects_maps::{
  dependent_code_inl::{
    DependencyGroup, DependentCodeRegistry, deoptimize_dependency_groups, get_dependent_code,
    install_dependency, mark_code_for_deoptimization,
  },
  field_index::Encoding,
  field_index_inl::{
    field_encoding, for_in_object_offset, for_property_index, for_smi_load_handler,
  },
  map_updater_cc_generated::{
    State, equal_immutable_values, generalize_field_type, map_updater_find_root_map,
    map_updater_find_target_map, map_updater_generalize_field, map_updater_map_updater,
    map_updater_normalize, map_updater_reconfigure_elements_kind,
  },
  prototype::{K_MAX_PROXIES_VISITED, PrototypeIterator, WhereToEnd},
};

fn create_test_boxed_map(instance_type: InstanceType, instance_size: usize) -> Box<Map> {
  let mut map = Box::new(Map::new(Tagged::NULL, instance_type, instance_size));
  map.set_in_object_properties_start_in_words(4);
  map.header.map = Tagged::from_ptr(&raw const *map);
  map
}

#[test]
fn test_field_index_for_smi_load_handler() -> Void {
  let mut map = create_test_boxed_map(InstanceType::JSObject, 32);
  let tagged_map = Tagged::from_ptr(&raw mut *map);

  // 1. in-object, tagged field, word offset 4
  // bits 0..3 = 5 (kField), bits 6..16 = 4 (offset in words), bit 17 = 1 (inobject), bit 18 = 0 (tagged)
  let handler_inobject = 5 | (4 << 6) | (1 << 17);
  let fi = for_smi_load_handler(tagged_map, handler_inobject);
  assert!(fi.is_inobject());
  assert_eq!(fi.encoding(), Encoding::Tagged);
  assert_eq!(fi.offset(), 4 * 4);

  // 2. in-object, double field, word offset 6
  // bit 18 = 1 (double)
  let handler_double = 5 | (6 << 6) | (1 << 17) | (1 << 18);
  let fi_double = for_smi_load_handler(tagged_map, handler_double);
  assert!(fi_double.is_inobject());
  assert_eq!(fi_double.encoding(), Encoding::Double);
  assert_eq!(fi_double.offset(), 6 * 4);

  // 3. out-of-object field, word offset 2
  // bit 17 = 0 (out of object)
  let handler_outobject = 5 | (2 << 6);
  let fi_out = for_smi_load_handler(tagged_map, handler_outobject);
  assert!(!fi_out.is_inobject());
  assert_eq!(fi_out.encoding(), Encoding::Tagged);
  assert_eq!(fi_out.offset(), 2 * 4);

  // 4. for_in_object_offset 边界与对齐
  let fi_in_tagged = for_in_object_offset(16, Encoding::Tagged);
  assert!(fi_in_tagged.is_inobject());
  assert_eq!(fi_in_tagged.offset(), 16);

  let fi_in_double = for_in_object_offset(24, Encoding::Double);
  assert!(fi_in_double.is_inobject());
  assert_eq!(fi_in_double.offset(), 24);

  // 5. field_encoding 映射校验
  assert_eq!(
    field_encoding(Representation::from_kind(Kind::Double)),
    Encoding::Double
  );
  assert_eq!(
    field_encoding(Representation::from_kind(Kind::Smi)),
    Encoding::Tagged
  );
  assert_eq!(
    field_encoding(Representation::from_kind(Kind::Tagged)),
    Encoding::Tagged
  );

  // 6. for_property_index 边界测试
  let fi_prop_in = for_property_index(tagged_map, 0, Representation::tagged());
  assert!(fi_prop_in.is_inobject());

  let fi_prop_out = for_property_index(tagged_map, 100, Representation::tagged());
  assert!(!fi_prop_out.is_inobject());

  OK
}

#[test]
fn test_dependent_code_lifecycle_and_deoptimization() -> Void {
  let mut real_map = create_test_boxed_map(InstanceType::JSObject, 32);
  let real_map_addr = Tagged::from_ptr(&raw mut *real_map).ptr();
  let code_1 = 0xAAAA_0001;
  let code_2 = 0xAAAA_0002;

  // 1. 注册依赖
  install_dependency(
    real_map_addr,
    code_1,
    DependencyGroup::FieldConstGroup.mask(),
  );
  install_dependency(
    real_map_addr,
    code_2,
    DependencyGroup::InitialMapChangedGroup.mask(),
  );

  let entries = get_dependent_code(real_map_addr);
  assert_eq!(entries.len(), 2);
  assert_eq!(entries[0].code, code_1);
  assert_eq!(entries[1].code, code_2);

  // 2. 相同 code 再次注册应合并组掩码
  install_dependency(
    real_map_addr,
    code_1,
    DependencyGroup::FieldTypeGroup.mask(),
  );
  let updated_entries = get_dependent_code(real_map_addr);
  assert_eq!(updated_entries.len(), 2);
  assert_eq!(
    updated_entries[0].groups,
    DependencyGroup::FieldConstGroup.mask() | DependencyGroup::FieldTypeGroup.mask()
  );

  // 3. 不匹配的组不触发标记
  let marked_none = mark_code_for_deoptimization(
    "isolate",
    real_map_addr,
    DependencyGroup::PrototypeCheckGroup.mask(),
  );
  assert!(!marked_none);

  // 4. 匹配 FieldConstGroup 触发标记
  let marked = mark_code_for_deoptimization(
    "isolate",
    real_map_addr,
    DependencyGroup::FieldConstGroup.mask(),
  );
  assert!(marked);

  // 5. 执行 deoptimize，仅失效匹配组
  deoptimize_dependency_groups(
    "isolate",
    real_map_addr,
    DependencyGroup::InitialMapChangedGroup.mask(),
  );

  // 6. 独立 DependentCodeRegistry 单元测试：前向 retain_mut 压缩与空槽清理
  let mut registry = DependentCodeRegistry::default();
  let test_obj = 0x9999_0001;
  registry.install_dependency(test_obj, 101, DependencyGroup::TransitionGroup.mask());
  registry.install_dependency(test_obj, 102, DependencyGroup::PrototypeCheckGroup.mask());
  registry.install_dependency(test_obj, 103, DependencyGroup::FieldConstGroup.mask());
  assert_eq!(registry.get_entries(test_obj).len(), 3);

  // 过滤并压缩：移除 code == 102
  registry.iterate_and_compact(test_obj, |entry| entry.code == 102);
  let compacted = registry.get_entries(test_obj);
  assert_eq!(compacted.len(), 2);
  assert_eq!(compacted[0].code, 101);
  assert_eq!(compacted[1].code, 103);

  // 全去优化并验证条目自动清理
  assert!(registry.deoptimize_dependency_groups(test_obj, 0xFFFF_FFFF));
  assert_eq!(registry.deopt_count(), 1);
  assert!(registry.get_entries(test_obj).is_empty());

  OK
}

#[test]
fn test_map_updater_state_machine_flow() -> Void {
  let mut map = create_test_boxed_map(InstanceType::JSObject, 32);
  let tagged_map = Tagged::from_ptr(&raw mut *map);
  let map_ptr = tagged_map.ptr();

  // 1. 初始化 MapUpdater
  let initialized_map = map_updater_map_updater(0, map_ptr);
  assert_eq!(initialized_map, map_ptr);

  // 2. 状态机步骤 1: FindRootMap
  let st1 = map_updater_find_root_map();
  assert_eq!(st1, State::AtRootMap as u8);

  // 3. 状态机步骤 2: FindTargetMap
  let st2 = map_updater_find_target_map();
  assert_eq!(st2, State::End as u8);

  // 4. 重配置 elements_kind
  let reconfigured_kind = map_updater_reconfigure_elements_kind(3);
  assert_eq!(reconfigured_kind, map_ptr);

  // 5. 字段泛化操作
  map_updater_generalize_field(map_ptr, 0, 0, 1, 0);

  // 6. 字典模式规格化
  let norm_state = map_updater_normalize("test");
  assert_eq!(norm_state, State::End as u8);

  // 7. 辅助函数与编译期计算
  assert!(equal_immutable_values(42, 42));
  assert!(!equal_immutable_values(42, 43));
  assert_eq!(generalize_field_type(0, 10, 0, 10, 0), 10);
  assert_eq!(generalize_field_type(0, 10, 0, 20, 0), 0);

  OK
}

#[test]
fn test_prototype_iterator_lifecycle() -> Void {
  let mut map1 = create_test_boxed_map(InstanceType::JSObject, 32);
  let map1_addr = Tagged::from_ptr(&raw mut *map1).ptr();
  let mut map2 = create_test_boxed_map(InstanceType::JSObject, 32);
  let map2_addr = Tagged::from_ptr(&raw mut *map2).ptr();
  let mut map3 = create_test_boxed_map(InstanceType::JSObject, 32);
  let map3_addr = Tagged::from_ptr(&raw mut *map3).ptr();

  let mut iter = PrototypeIterator::new_handle("isolate", map1_addr, 0, WhereToEnd::EndAtNull);
  assert!(!iter.is_at_end());
  assert!(iter.has_access());

  iter.advance();
  assert!(iter.is_at_end());

  let mut iter_tagged =
    PrototypeIterator::new_tagged("isolate", map2_addr, 1, WhereToEnd::EndAtNonHidden);
  assert!(iter_tagged.is_at_end());

  let success = iter_tagged.advance_following_proxies();
  assert!(success);

  // 验证超出最大代理层数时的安全熔断 (K_MAX_PROXIES_VISITED)
  let mut proxy_iter =
    PrototypeIterator::new_handle("isolate", map3_addr, 0, WhereToEnd::EndAtNull);
  proxy_iter.seen_proxies = K_MAX_PROXIES_VISITED;
  assert!(!proxy_iter.advance_following_proxies_ignoring_access_checks());

  OK
}
