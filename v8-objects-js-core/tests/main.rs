// v8-objects-js-core/tests/main.rs

use aok::{OK, Void};
use v8_objects_arrays::fixed_array_base::FixedArrayBase;
use v8_objects_core::{
  heap_number::HeapNumber,
  instance_type::InstanceType,
  map::Map,
  property_details::{
    PropertyAttributes, PropertyConstness, PropertyDetails, PropertyKind, PropertyLocation,
    Representation,
  },
  smi::Smi,
  tagged::Tagged,
};
use v8_objects_js_core::{
  AliasedArgumentsEntry, ArgumentLocation, FieldEncoding, FieldIndex, JSArgumentsObject, JSArray,
  JSArrayBuf, JSArrayExt, JSObject, JSObjectBuf, JSObjectExt, JSReceiver, JSReceiverExt,
  JSSloppyArgumentsObject, JSStrictArgumentsObject, K_DICTIONARY_ELEMENTS_KIND,
  K_SHARED_ARRAY_ELEMENTS_KIND, K_THE_HOLE_SENTINEL, SloppyArgumentsElements,
  SloppyArgumentsElementsBuf, SloppyArgumentsElementsExt,
};
use v8_objects_maps::{
  DescriptorArrayExt, PropertyArray, descriptor_array::DescriptorArrayBuf,
  property_array::PropertyArrayBuf,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

fn create_test_map(instance_type: InstanceType, instance_size_bytes: usize) -> Map {
  Map::new(Tagged::NULL, instance_type, instance_size_bytes)
}

#[test]
fn test_js_receiver_basic() -> Void {
  let meta_map = create_test_map(InstanceType::JSObject, 16);
  let meta_tagged: Tagged<Map> = Tagged::from_ref(&meta_map);

  let mut receiver = JSReceiver::new(meta_tagged, Tagged::NULL);
  assert_eq!(receiver.map(), meta_tagged);
  assert_eq!(receiver.raw_properties_or_hash(), Tagged::NULL);

  let val = Tagged::new(Smi::from_int(42).ptr());
  receiver.set_raw_properties_or_hash(val);
  assert_eq!(receiver.raw_properties_or_hash(), val);
  assert!(receiver.has_fast_properties());

  // 原型链测试
  let proto_map = create_test_map(InstanceType::JSObject, 16);
  let proto_tagged: Tagged<Map> = Tagged::from_ref(&proto_map);
  let proto_obj = JSReceiver::new(proto_tagged, Tagged::NULL);
  let proto_obj_tagged = Tagged::from_ref(&proto_obj).cast();

  let mut obj_map = create_test_map(InstanceType::JSObject, 16);
  obj_map.prototype = proto_obj_tagged;
  let obj_map_tagged: Tagged<Map> = Tagged::from_ref(&obj_map);
  let mut obj = JSReceiver::new(obj_map_tagged, Tagged::NULL);

  assert_eq!(obj.get_prototype(), proto_obj_tagged);
  assert!(obj.has_in_prototype_chain(proto_obj_tagged));

  let unrelated = Tagged::new(0x1000);
  assert!(!obj.has_in_prototype_chain(unrelated));

  // 原型链循环递归保护测试
  let mut cycle_map1 = create_test_map(InstanceType::JSObject, 16);
  let mut cycle_map2 = create_test_map(InstanceType::JSObject, 16);
  let cycle_obj1 = JSReceiver::new(Tagged::from_ref(&cycle_map1), Tagged::NULL);
  let cycle_obj2 = JSReceiver::new(Tagged::from_ref(&cycle_map2), Tagged::NULL);
  cycle_map1.prototype = Tagged::from_ref(&cycle_obj2).cast();
  cycle_map2.prototype = Tagged::from_ref(&cycle_obj1).cast();
  // 触发原型链回溯
  assert!(!cycle_obj1.has_in_prototype_chain(unrelated));
  assert_eq!(cycle_map1.prototype, Tagged::from_ref(&cycle_obj2).cast());
  assert_eq!(cycle_map2.prototype, Tagged::from_ref(&cycle_obj1).cast());

  // 哈希测试
  assert_eq!(obj.get_identity_hash(), None);
  obj.set_identity_hash(12345)?;
  assert_eq!(obj.get_identity_hash(), Some(12345));

  // 验证 0 与负数非合法哈希 (DCHECK_NE(PropertyArray::kNoHashSentinel, hash))
  assert!(obj.set_identity_hash(0).is_err());
  assert!(obj.set_identity_hash(-1).is_err());

  OK
}

#[test]
fn test_js_object_in_object_properties() -> Void {
  let mut map = create_test_map(
    InstanceType::JSObject,
    JSObject::K_HEADER_SIZE + 3 * size_of::<usize>(),
  );
  map.set_in_object_properties_start_in_words(3);
  map.set_instance_size_in_words(6);
  let map_tagged = Tagged::from_ref(&map);

  let empty_elements = FixedArrayBase::new(map_tagged, 0);
  let empty_elements_tagged = Tagged::from_ref(&empty_elements);

  let mut obj_buf = JSObjectBuf::new(map_tagged, Tagged::NULL, empty_elements_tagged, 3)?;

  let field0 = FieldIndex::for_property_index(map_tagged, 0, Representation::tagged());
  let field1 = FieldIndex::for_property_index(map_tagged, 1, Representation::tagged());
  let field2 = FieldIndex::for_property_index(map_tagged, 2, Representation::tagged());

  assert!(field0.is_inobject());
  assert!(field1.is_inobject());
  assert!(field2.is_inobject());
  assert_eq!(field0.property_index(), 0);
  assert_eq!(field1.property_index(), 1);
  assert_eq!(field2.property_index(), 2);

  let val0 = Tagged::new(Smi::from_int(100).ptr());
  let val1 = Tagged::new(Smi::from_int(200).ptr());
  let val2 = Tagged::new(Smi::from_int(300).ptr());

  obj_buf.fast_property_at_put(field0, val0)?;
  obj_buf.fast_property_at_put(field1, val1)?;
  obj_buf.fast_property_at_put(field2, val2)?;

  assert_eq!(obj_buf.raw_fast_property_at(field0)?, val0);
  assert_eq!(obj_buf.raw_fast_property_at(field1)?, val1);
  assert_eq!(obj_buf.raw_fast_property_at(field2)?, val2);

  // 越界保护检查
  let oob_field = FieldIndex::for_in_object_offset(9999, FieldEncoding::Tagged);
  assert!(obj_buf.raw_fast_property_at(oob_field).is_err());

  // elements kind 谓词族测试
  map.set_elements_kind(0); // PACKED_SMI
  assert!(obj_buf.has_fast_elements());
  assert!(obj_buf.has_fast_packed_elements());
  assert!(!obj_buf.has_holey_elements());
  assert!(!obj_buf.has_dictionary_elements());

  map.set_elements_kind(1); // HOLEY_SMI
  assert!(obj_buf.has_fast_elements());
  assert!(!obj_buf.has_fast_packed_elements());
  assert!(obj_buf.has_holey_elements());

  map.set_elements_kind(K_SHARED_ARRAY_ELEMENTS_KIND); // SHARED_ARRAY_ELEMENTS (12)
  assert!(!obj_buf.has_fast_elements());
  assert!(!obj_buf.has_dictionary_elements());

  map.set_elements_kind(K_DICTIONARY_ELEMENTS_KIND); // DICTIONARY_ELEMENTS (13)
  assert!(!obj_buf.has_fast_elements());
  assert!(obj_buf.has_dictionary_elements());

  OK
}

#[test]
fn test_js_object_out_of_object_properties() -> Void {
  let mut map = create_test_map(
    InstanceType::JSObject,
    JSObject::K_HEADER_SIZE + size_of::<usize>(),
  );
  map.set_instance_size_in_words(4);
  map.set_in_object_properties_start_in_words(3);
  let map_tagged = Tagged::from_ref(&map);

  let prop_array_buf = PropertyArrayBuf::new(map_tagged, 4)?;
  let prop_array_tagged = prop_array_buf.as_tagged();

  let empty_elements = FixedArrayBase::new(map_tagged, 0);
  let empty_elements_tagged = Tagged::from_ref(&empty_elements);

  let mut obj_buf = JSObjectBuf::new(
    map_tagged,
    Tagged::new(prop_array_tagged.ptr()),
    empty_elements_tagged,
    1,
  )?;

  // property_index 0 在对象内，1 在对象外 (PropertyArray index 0)
  let field_in = FieldIndex::for_property_index(map_tagged, 0, Representation::tagged());
  let field_out = FieldIndex::for_property_index(map_tagged, 1, Representation::tagged());

  assert!(field_in.is_inobject());
  assert!(!field_out.is_inobject());
  assert_eq!(field_out.outobject_array_index(), 0);
  assert_eq!(field_out.property_index(), 1);

  let val_in = Tagged::new(Smi::from_int(11).ptr());
  let val_out = Tagged::new(Smi::from_int(22).ptr());

  obj_buf.fast_property_at_put(field_in, val_in)?;
  obj_buf.fast_property_at_put(field_out, val_out)?;

  assert_eq!(obj_buf.raw_fast_property_at(field_in)?, val_in);
  assert_eq!(obj_buf.raw_fast_property_at(field_out)?, val_out);

  // 验证 PropertyDetails 封装
  let details_in = PropertyDetails::for_fast_field(
    PropertyKind::Data,
    PropertyAttributes::NONE,
    PropertyLocation::Field,
    PropertyConstness::Mutable,
    Representation::tagged(),
    (3 * size_of::<usize>()) as u32,
    true,
  );
  let details_out = PropertyDetails::for_fast_field(
    PropertyKind::Data,
    PropertyAttributes::NONE,
    PropertyLocation::Field,
    PropertyConstness::Mutable,
    Representation::tagged(),
    PropertyArray::K_HEADER_SIZE as u32,
    false,
  );

  assert_eq!(obj_buf.get_property_by_details(details_in)?, val_in);
  assert_eq!(obj_buf.get_property_by_details(details_out)?, val_out);

  let new_val = Tagged::new(Smi::from_int(999).ptr());
  obj_buf.set_property_by_details(details_out, new_val)?;
  assert_eq!(obj_buf.get_property_by_details(details_out)?, new_val);

  OK
}

#[test]
fn test_js_array_in_object_property_offset_isolation() -> Void {
  // 重点核验：JSArray 头部包含 length 字段 (4 words)，对象内属性必须从 4 words 开始，
  // 决不能与普通 JSObject (3 words) 混淆，更不能覆盖 length 槽位。
  let mut map = create_test_map(
    InstanceType::JSArray,
    JSArray::K_HEADER_SIZE + 2 * size_of::<usize>(),
  );
  map.set_in_object_properties_start_in_words(4);
  map.set_instance_size_in_words(6);
  let map_tagged = Tagged::from_ref(&map);

  let empty_elements = FixedArrayBase::new(map_tagged, 0);
  let empty_elements_tagged = Tagged::from_ref(&empty_elements);

  let mut array_buf = JSArrayBuf::new(
    map_tagged,
    Tagged::NULL,
    empty_elements_tagged,
    Tagged::new(Smi::from_int(777).ptr()),
    2,
  )?;

  let field0 = FieldIndex::for_property_index(map_tagged, 0, Representation::tagged());
  let field1 = FieldIndex::for_property_index(map_tagged, 1, Representation::tagged());

  assert!(field0.is_inobject());
  assert!(field1.is_inobject());
  // 必须位于 4 words 偏移处 (即 32 字节)，而非 24 字节 (length 槽位)
  assert_eq!(field0.offset(), 4 * size_of::<usize>());
  assert_eq!(field1.offset(), 5 * size_of::<usize>());
  assert_eq!(field0.property_index(), 0);
  assert_eq!(field1.property_index(), 1);

  // 初始长度应为 777
  assert_eq!(array_buf.length_u32()?, 777);

  // 向对象内属性写入数据
  let prop_val = Tagged::new(Smi::from_int(888).ptr());
  array_buf.fast_property_at_put(field0, prop_val)?;

  // 核心断言：写入 in-object property 0 绝不能篡改 length
  assert_eq!(array_buf.length_u32()?, 777);
  assert_eq!(array_buf.raw_fast_property_at(field0)?, prop_val);

  // 修改 length 也绝不能影响 in-object property
  array_buf.set_length_u32(999)?;
  assert_eq!(array_buf.length_u32()?, 999);
  assert_eq!(array_buf.raw_fast_property_at(field0)?, prop_val);

  OK
}

#[test]
fn test_js_array_length_and_elements() -> Void {
  let mut map = create_test_map(InstanceType::JSArray, JSArray::K_HEADER_SIZE);
  map.set_elements_kind(0); // PACKED_SMI
  let map_tagged = Tagged::from_ref(&map);

  let empty_elements = FixedArrayBase::new(map_tagged, 10);
  let empty_elements_tagged = Tagged::from_ref(&empty_elements);

  let mut array_buf = JSArrayBuf::new(
    map_tagged,
    Tagged::NULL,
    empty_elements_tagged,
    Tagged::new(Smi::from_int(0).ptr()),
    0,
  )?;

  assert_eq!(array_buf.length_u32()?, 0);
  assert!(array_buf.is_fast_elements());
  assert_eq!(array_buf.elements(), empty_elements_tagged);

  array_buf.set_length_u32(128)?;
  assert_eq!(array_buf.length_u32()?, 128);

  // 快元素数组规范化阈值判断
  assert!(!array_buf.set_length_would_normalize(100));
  assert!(array_buf.set_length_would_normalize(33 * 1024 * 1024));

  // 慢元素切换：当已经是字典模式元素时，扩容绝不会“规范化为字典”
  map.set_elements_kind(K_DICTIONARY_ELEMENTS_KIND); // DICTIONARY_ELEMENTS (13)
  assert!(!array_buf.is_fast_elements());
  assert!(!array_buf.set_length_would_normalize(33 * 1024 * 1024));

  // HeapNumber 形式长度
  let heap_num = HeapNumber::new(map_tagged, 65536.0);
  let heap_num_tagged = Tagged::from_ref(&heap_num);
  array_buf.set_length(Tagged::new(heap_num_tagged.ptr()));
  assert_eq!(array_buf.length_u32()?, 65536);

  // 容量限制测试：JSArray 最大支持 251 个对象内属性
  assert_eq!(JSArray::K_MAX_IN_OBJECT_PROPERTIES, 251);
  let oob_buf = JSArrayBuf::new(
    map_tagged,
    Tagged::NULL,
    empty_elements_tagged,
    Tagged::new(Smi::from_int(0).ptr()),
    252,
  );
  assert!(oob_buf.is_err());

  OK
}

#[test]
fn test_field_index_descriptor_and_ic_encoding() -> Void {
  let mut map = create_test_map(InstanceType::JSObject, JSObject::K_HEADER_SIZE);
  map.set_in_object_properties_start_in_words(3);
  let map_tagged = Tagged::from_ref(&map);

  // 1. GetLoadByFieldIndex 编码测试
  // 对象内 tagged 属性 (offset_in_words = 3): index = 0, double = 0 -> 0 << 1 = 0
  let field_in_tagged =
    FieldIndex::for_in_object_offset(3 * size_of::<usize>(), FieldEncoding::Tagged);
  assert_eq!(field_in_tagged.get_load_by_field_index(), 0);

  // 对象内 double 属性: index = 0, double = 1 -> (0 << 1) | 1 = 1
  let field_in_double =
    FieldIndex::for_in_object_offset(3 * size_of::<usize>(), FieldEncoding::Double);
  assert_eq!(field_in_double.get_load_by_field_index(), 1);

  // 对象外 tagged 属性 (offset_in_words = 2, PropertyArray 头部 2 words):
  // raw_index = 0, encoded = -0 - 1 = -1, (-1 << 1) = -2
  let field_out_tagged = FieldIndex::for_out_object_index(0, FieldEncoding::Tagged);
  assert_eq!(field_out_tagged.get_load_by_field_index(), -2);

  // 2. FieldIndex::for_descriptor 测试
  let details = PropertyDetails::for_fast_field(
    PropertyKind::Data,
    PropertyAttributes::NONE,
    PropertyLocation::Field,
    PropertyConstness::Mutable,
    Representation::tagged(),
    (3 * size_of::<usize>()) as u32,
    true,
  );

  let desc_buf = DescriptorArrayBuf::new(map_tagged, 4);
  let desc_tagged = desc_buf.as_tagged();
  desc_tagged.append(Tagged::new(0x1111), Tagged::NULL, details)?;
  map.instance_descriptors = Tagged::new(desc_tagged.ptr());
  let map_tagged = Tagged::from_ref(&map);

  let field_desc = FieldIndex::for_descriptor(map_tagged, 0)?;
  assert!(field_desc.is_inobject());
  assert_eq!(field_desc.offset(), 3 * size_of::<usize>());

  OK
}

#[test]
fn test_js_arguments_object() -> Void {
  let map = create_test_map(
    InstanceType::JSArgumentsObject,
    JSSloppyArgumentsObject::K_SIZE,
  );
  let map_tagged = Tagged::from_ref(&map);
  let empty_elements = FixedArrayBase::new(map_tagged, 0);
  let empty_elements_tagged = Tagged::from_ref(&empty_elements);

  // 0. 常量断言验证
  assert_eq!(JSArgumentsObject::K_HEADER_SIZE, JSObject::K_HEADER_SIZE);
  assert_eq!(
    SloppyArgumentsElements::K_HEADER_SIZE,
    size_of::<SloppyArgumentsElements>()
  );

  // 1. 松散参数对象槽位读写
  let sloppy_buf = JSObjectBuf::new(
    map_tagged,
    Tagged::NULL,
    empty_elements_tagged,
    JSSloppyArgumentsObject::K_IN_OBJECT_PROPERTY_COUNT,
  )?;
  let sloppy_tagged = sloppy_buf.as_tagged();

  let len_val = Tagged::new(Smi::from_int(3).ptr());
  let callee_val = Tagged::new(0x2000);

  JSSloppyArgumentsObject::set_length(sloppy_tagged, len_val);
  JSSloppyArgumentsObject::set_callee(sloppy_tagged, callee_val);

  assert_eq!(JSSloppyArgumentsObject::length(sloppy_tagged), len_val);
  assert_eq!(JSSloppyArgumentsObject::callee(sloppy_tagged), callee_val);

  // 2. 严格参数对象槽位读写
  let strict_buf = JSObjectBuf::new(
    map_tagged,
    Tagged::NULL,
    empty_elements_tagged,
    JSStrictArgumentsObject::K_IN_OBJECT_PROPERTY_COUNT,
  )?;
  let strict_tagged = strict_buf.as_tagged();
  JSStrictArgumentsObject::set_length(strict_tagged, len_val);
  assert_eq!(JSStrictArgumentsObject::length(strict_tagged), len_val);

  // 3. 别名条目 AliasedArgumentsEntry
  let entry = AliasedArgumentsEntry::new(map_tagged, 7);
  assert_eq!(entry.aliased_context_slot(), 7);

  // 4. SloppyArgumentsElements
  let dummy_context = Tagged::new(0x3000);
  let mut elements_buf =
    SloppyArgumentsElementsBuf::new(map_tagged, dummy_context, empty_elements_tagged, 2)?;

  // 形参 0 映射到 context slot 5
  let mapped_smi = Tagged::new(Smi::from_int(5).ptr());
  elements_buf.set_mapped_entry(0, mapped_smi)?;
  assert_eq!(elements_buf.mapped_entry(0)?, mapped_smi);

  // 形参 1 为解绑 (TheHole 哨兵)
  elements_buf.set_mapped_entry(1, K_THE_HOLE_SENTINEL)?;

  assert_eq!(
    elements_buf.lookup_argument(0)?,
    ArgumentLocation::ContextSlot(5)
  );
  assert_eq!(
    elements_buf.lookup_argument(1)?,
    ArgumentLocation::UnmappedArray(1)
  );
  assert_eq!(
    elements_buf.lookup_argument(2)?,
    ArgumentLocation::UnmappedArray(2)
  );

  assert!(elements_buf.mapped_entry(99).is_err());

  OK
}
