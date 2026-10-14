//! 实例类型 → 唯一只读 map 根索引查表 (instance-type-inl.h: UniqueMapOfInstanceType)

use aok::{OK, Void};
use v8_objects_core::{
  instance_type::InstanceType, instance_type_inl::unique_map_of_instance_type,
};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

#[test]
fn test_unique_map_root_index_lookup() -> Void {
  // STRUCT_LIST / 显式 case 命中: 返回 k##Name##Map 的名字载体
  assert_eq!(
    unique_map_of_instance_type(InstanceType::PromiseCapability),
    Some("PromiseCapabilityMap")
  );
  assert_eq!(
    unique_map_of_instance_type(InstanceType::Script),
    Some("ScriptMap")
  );
  // 表外类型 (Map 非唯一只读 map 根) → None
  assert_eq!(unique_map_of_instance_type(InstanceType::Map), None);
  assert_eq!(unique_map_of_instance_type(InstanceType::Oddball), None);

  OK
}
