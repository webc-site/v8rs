//! ObjectType 单元测试 (object-type.h)

use aok::{OK, Void};
use v8_objects_core::object_type::ObjectType;

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

#[test]
fn test_object_type_variants_and_strings() -> Void {
  assert_eq!(ObjectType::COUNT, 386);

  assert_eq!(ObjectType::Object.to_str(), "Object");
  assert_eq!(ObjectType::Smi.to_str(), "Smi");
  assert_eq!(ObjectType::TaggedIndex.to_str(), "TaggedIndex");
  assert_eq!(ObjectType::HeapObject.to_str(), "HeapObject");
  assert_eq!(ObjectType::Map.to_str(), "Map");
  assert_eq!(ObjectType::HeapNumber.to_str(), "HeapNumber");
  assert_eq!(ObjectType::BigInt.to_str(), "BigInt");
  assert_eq!(ObjectType::JSObject.to_str(), "JSObject");
  assert_eq!(ObjectType::JSArray.to_str(), "JSArray");

  OK
}
