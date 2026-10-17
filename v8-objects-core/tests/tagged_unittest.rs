use core::f64::consts::E;

use aok::{OK, Void};
use v8_objects_core::{
  heap_number::HeapNumber,
  heap_object::HeapObject,
  instance_type::InstanceType,
  map::Map,
  name::{HashFieldType, Name},
  oddball::{Oddball, OddballKind},
  symbol::{PrivateSymbolKind, Symbol},
  tagged::{Object, Tagged},
  tagged_constants::{HEAP_OBJECT_TAG, WEAK_HEAP_OBJECT_TAG},
};
use v8_zone::{AccountingAllocator, Zone};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

#[test]
fn test_tagged_pointer_properties() -> Void {
  // 空指针验证
  let null_ptr: Tagged<HeapObject> = Tagged::NULL;
  assert!(null_ptr.is_null());
  assert_eq!(null_ptr.ptr(), 0);
  assert!(null_ptr.is_smi());
  assert!(!null_ptr.is_heap_object());

  // 堆对象强引用指针测试
  let aligned_address: usize = 0x1000;
  let tagged = unsafe { Tagged::<HeapObject>::from_raw_address(aligned_address) };

  assert_eq!(tagged.ptr(), aligned_address | HEAP_OBJECT_TAG);
  assert!(tagged.is_heap_object());
  assert!(!tagged.is_smi());
  assert!(!tagged.is_weak());
  assert!(!tagged.is_null());

  // SAFETY 显式证明还原堆起始对齐地址
  assert_eq!(tagged.address(), aligned_address);

  // 安全解包
  assert_eq!(tagged.to_heap_object(), Some(tagged));
  assert_eq!(tagged.to_smi(), None);

  // 弱引用指针测试
  let weak_raw = aligned_address | WEAK_HEAP_OBJECT_TAG;
  let weak_tagged = Tagged::<HeapObject>::new(weak_raw);
  assert!(weak_tagged.is_weak());
  assert!(!weak_tagged.is_heap_object());
  assert!(!weak_tagged.is_smi());
  assert_eq!(weak_tagged.address(), aligned_address); // 掩码清除 WEAK_HEAP_OBJECT_TAG(3) 还原对齐堆基址

  OK
}

#[test]
fn test_heap_object_and_map_structure() -> Void {
  // 验证 Map 首部偏移为 0
  assert_eq!(HeapObject::MAP_OFFSET, 0);

  let meta_map = Tagged::<Map>::new(0x2001);
  let mut map = Map::new(meta_map, InstanceType::JSObject, 32);

  assert_eq!(map.header.map, meta_map);
  assert_eq!(map.instance_type(), InstanceType::JSObject);
  assert_eq!(map.instance_size(), 32);

  // Bits1 位域标志设置与读取测试
  assert!(!map.is_callable());
  map.set_is_callable(true);
  assert!(map.is_callable());

  assert!(!map.is_constructor());
  map.set_is_constructor(true);
  assert!(map.is_constructor());

  assert!(!map.is_undetectable());
  map.set_is_undetectable(true);
  assert!(map.is_undetectable());

  // Bits2 位域标志
  assert!(!map.new_target_is_base());
  map.set_new_target_is_base(true);
  assert!(map.new_target_is_base());

  // Bits3 位域标志与数值
  map.set_enum_length(128);
  assert_eq!(map.enum_length(), 128);

  map.set_number_of_own_descriptors(16);
  assert_eq!(map.number_of_own_descriptors(), 16);

  assert!(!map.is_prototype_map());
  map.set_is_prototype_map(true);
  assert!(map.is_prototype_map());

  assert!(!map.is_dictionary_map());
  map.set_is_dictionary_map(true);
  assert!(map.is_dictionary_map());

  assert!(!map.is_extensible());
  map.set_is_extensible(true);
  assert!(map.is_extensible());

  OK
}

#[test]
fn test_name_and_symbol_representation() -> Void {
  let map_ptr = Tagged::<Map>::new(0x3001);

  // Name 初始哈希未计算状态
  let mut name = Name::new(map_ptr);
  assert!(!name.has_hash_code());
  assert_eq!(name.hash_field_type(), HashFieldType::Empty);
  assert_eq!(name.try_get_hash(), None);

  // 设置属性哈希并检验提取
  name.set_hash(0x1A2B3C);
  assert!(name.has_hash_code());
  assert_eq!(name.hash_field_type(), HashFieldType::Hash);
  assert_eq!(name.hash(), 0x1A2B3C);
  assert_eq!(name.try_get_hash(), Some(0x1A2B3C));

  // Symbol 构造与属性测试
  let desc = Tagged::<Object>::new(0x4001);
  let mut sym = Symbol::new(map_ptr, desc);
  assert_eq!(sym.description(), desc);
  assert_eq!(sym.private_symbol_kind(), PrivateSymbolKind::Public);
  assert!(!sym.is_any_private());

  // 设置为内部私有符号
  sym.set_private_symbol_kind(PrivateSymbolKind::Internal);
  assert_eq!(sym.private_symbol_kind(), PrivateSymbolKind::Internal);
  assert!(sym.is_any_private());
  assert!(sym.is_private_internal());
  assert!(!sym.is_any_private_name());

  // 设置为私有字段
  sym.set_private_symbol_kind(PrivateSymbolKind::FieldName);
  assert!(sym.is_any_private_name());
  assert!(!sym.is_private_brand());

  // 众所周知符号与公共符号表测试
  assert!(!sym.is_well_known_symbol());
  sym.set_is_well_known_symbol(true);
  assert!(sym.is_well_known_symbol());

  assert!(!sym.is_in_public_symbol_table());
  sym.set_is_in_public_symbol_table(true);
  assert!(sym.is_in_public_symbol_table());

  assert!(!sym.is_interesting_symbol());
  sym.set_is_interesting_symbol(true);
  assert!(sym.is_interesting_symbol());

  OK
}

#[test]
fn test_heap_number_and_oddball_representation() -> Void {
  let map_ptr = Tagged::<Map>::new(0x5001);

  // HeapNumber 基础存取
  let mut num = HeapNumber::new(map_ptr, 1.25);
  assert_eq!(num.value(), 1.25);
  assert_eq!(num.value_as_bits(), 1.25f64.to_bits());
  assert!(!num.is_the_hole());

  num.set_value(-42.5);
  assert_eq!(num.value(), -42.5);

  // Hole NaN 判定
  let hole_num = HeapNumber::from_bits(map_ptr, HeapNumber::HOLE_NAN_INT64);
  assert!(hole_num.is_the_hole());
  assert!(hole_num.is_nan());

  // Oddball 基础行为与单例枚举测试
  let empty_obj = Tagged::<Object>::NULL;

  let odd_true = Oddball::new(
    map_ptr,
    OddballKind::True,
    1.0,
    empty_obj,
    empty_obj,
    empty_obj,
  );
  assert!(odd_true.is_boolean());
  assert!(odd_true.is_true());
  assert!(!odd_true.is_false());
  assert_eq!(odd_true.to_bool(), Some(true));
  assert_eq!(odd_true.to_number_raw(), 1.0);

  let odd_false = Oddball::new(
    map_ptr,
    OddballKind::False,
    0.0,
    empty_obj,
    empty_obj,
    empty_obj,
  );
  assert!(odd_false.is_boolean());
  assert!(odd_false.is_false());
  assert_eq!(odd_false.to_bool(), Some(false));

  let odd_null = Oddball::new(
    map_ptr,
    OddballKind::Null,
    0.0,
    empty_obj,
    empty_obj,
    empty_obj,
  );
  assert!(odd_null.is_null());
  assert!(!odd_null.is_boolean());
  assert_eq!(odd_null.to_bool(), None);

  let odd_undefined = Oddball::new(
    map_ptr,
    OddballKind::Undefined,
    f64::NAN,
    empty_obj,
    empty_obj,
    empty_obj,
  );
  assert!(odd_undefined.is_undefined());

  // 注: the_hole/uninitialized 本版本为独立 Hole 类 (hole.h), 非 Oddball kind
  // (oddball.h:58-62 值域仅 0/1/3/4), 无对应构造。

  OK
}

#[test]
fn test_zone_allocation_integration() -> Void {
  let allocator = AccountingAllocator::default();
  let zone = Zone::new(&allocator, "test-zone");

  let map_ptr = Tagged::<Map>::new(0x6001);
  let heap_num = HeapNumber::new(map_ptr, E);

  let tagged_num = Tagged::allocate_in_zone(&zone, heap_num)?;
  assert!(tagged_num.is_heap_object());

  let read_back = unsafe { tagged_num.as_ref() };
  assert_eq!(read_back.value(), E);
  assert_eq!(read_back.header.map, map_ptr);

  OK
}
