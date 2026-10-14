//! object-predicates 单元测试

use aok::{OK, Void};
use v8_objects_core::{
  heap_object::HeapObject,
  instance_type::InstanceType,
  map::Map,
  object_predicates::{
    EXPLICIT_PREDICATES, HEAP_OBJECT_TYPE_PREDICATES, OBJECT_TYPE_PREDICATES, PREDICATE_TOTAL,
    STRUCT_PREDICATES,
  },
  object_predicates_inl::{is_number, is_numeric, is_tagged_index, is_zero},
  smi::Smi,
  tagged::{Object, Tagged, TaggedIndex},
};
use v8_zone::{AccountingAllocator, Zone};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

#[test]
fn test_object_predicates_metadata() -> Void {
  assert_eq!(OBJECT_TYPE_PREDICATES.len(), 3);
  assert_eq!(HEAP_OBJECT_TYPE_PREDICATES.len(), 338);
  assert_eq!(EXPLICIT_PREDICATES.len(), 5);
  assert_eq!(STRUCT_PREDICATES.len(), 40);
  assert_eq!(PREDICATE_TOTAL, 386);

  assert!(OBJECT_TYPE_PREDICATES.contains(&"Number"));
  assert!(OBJECT_TYPE_PREDICATES.contains(&"Numeric"));
  assert!(OBJECT_TYPE_PREDICATES.contains(&"Primitive"));

  OK
}

#[test]
fn test_is_tagged_index_and_is_zero() -> Void {
  // Smi(0) 是 zero 也是合法的 TaggedIndex
  let zero = Smi::zero().cast::<Object>();
  assert!(is_zero(zero));
  assert!(is_tagged_index(zero));

  // 非零 64 位 Smi (shift 32) 其解出的 TaggedIndex 值域超界 (42 << 31)
  let smi42 = Smi::from_int(42).cast::<Object>();
  assert!(!is_zero(smi42));
  assert!(!is_tagged_index(smi42));

  // 正确编码的 TaggedIndex (shift 1, 31 位)
  let valid_index = TaggedIndex::from_intptr(42).cast::<Object>();
  assert!(is_tagged_index(valid_index));

  // 堆对象不是 zero 也不是 TaggedIndex
  let heap_obj = Tagged::<Object>::new(0x2001);
  assert!(!is_zero(heap_obj));
  assert!(!is_tagged_index(heap_obj));

  OK
}

#[test]
fn test_is_number_and_is_numeric() -> Void {
  let allocator = AccountingAllocator::default();
  let zone = Zone::new(&allocator, "test-predicates");

  // 1. Smi 恒为 Number 和 Numeric
  let smi = Smi::from_int(99).cast::<Object>();
  unsafe {
    assert!(is_number(smi));
    assert!(is_numeric(smi));
  }

  // 2. HeapNumber 在堆上
  let meta_map = Tagged::<Map>::new(0x1001);
  let number_map =
    Tagged::<Map>::allocate_in_zone(&zone, Map::new(meta_map, InstanceType::HeapNumber, 16))?;
  let heap_number =
    Tagged::<HeapObject>::allocate_in_zone(&zone, HeapObject::new(number_map))?.cast::<Object>();
  unsafe {
    assert!(is_number(heap_number));
    assert!(is_numeric(heap_number));
  }

  // 3. BigIntBase 在堆上 (是 Numeric 但非 Number)
  let bigint_map =
    Tagged::<Map>::allocate_in_zone(&zone, Map::new(meta_map, InstanceType::BigIntBase, 16))?;
  let bigint_obj =
    Tagged::<HeapObject>::allocate_in_zone(&zone, HeapObject::new(bigint_map))?.cast::<Object>();
  unsafe {
    assert!(!is_number(bigint_obj));
    assert!(is_numeric(bigint_obj));
  }

  // 4. 普通 JSObject (既非 Number 也非 Numeric)
  let js_map =
    Tagged::<Map>::allocate_in_zone(&zone, Map::new(meta_map, InstanceType::JSObject, 32))?;
  let js_obj =
    Tagged::<HeapObject>::allocate_in_zone(&zone, HeapObject::new(js_map))?.cast::<Object>();
  unsafe {
    assert!(!is_number(js_obj));
    assert!(!is_numeric(js_obj));
  }

  OK
}

#[test]
fn test_basic_predicates_and_sentinel() -> Void {
  use v8_objects_core::{
    object_predicates_inl::{
      is_heap_object, is_no_shared_name_sentinel, is_object, is_smi, is_weak,
    },
    tagged::{MaybeObject, make_weak},
  };

  let smi = Smi::from_int(10).cast::<Object>();
  let heap_obj = Tagged::<Object>::new(0x2001);
  assert!(is_smi(smi));
  assert!(!is_smi(heap_obj));
  assert!(!is_heap_object(smi));
  assert!(is_heap_object(heap_obj));

  let zero = Smi::zero().cast::<Object>();
  assert!(is_no_shared_name_sentinel(zero));
  assert!(!is_no_shared_name_sentinel(smi));

  let weak_ref: Tagged<MaybeObject> =
    make_weak(unsafe { Tagged::<HeapObject>::from_raw_address(0x1000) }).cast();
  assert!(is_weak(weak_ref));
  assert!(!is_object(weak_ref));

  let strong_ref: Tagged<MaybeObject> =
    unsafe { Tagged::<HeapObject>::from_raw_address(0x1000) }.cast();
  assert!(!is_weak(strong_ref));
  assert!(is_object(strong_ref));

  OK
}

#[test]
fn test_nan_minus_zero_and_symbol_predicates() -> Void {
  use v8_objects_core::{
    heap_number::HeapNumber,
    object_predicates_inl::{
      is_minus_zero, is_nan, is_private_symbol, is_public_symbol, is_unique_name,
    },
    symbol::{PrivateSymbolKind, Symbol},
  };

  let allocator = AccountingAllocator::default();
  let zone = Zone::new(&allocator, "test-nan-symbols");
  let meta_map = Tagged::<Map>::new(0x1001);

  // 1. HeapNumber: NaN, -0.0, 1.5
  let number_map =
    Tagged::<Map>::allocate_in_zone(&zone, Map::new(meta_map, InstanceType::HeapNumber, 16))?;
  let nan_obj =
    Tagged::<HeapNumber>::allocate_in_zone(&zone, HeapNumber::new(number_map, f64::NAN))?
      .cast::<Object>();
  let minus_zero_obj =
    Tagged::<HeapNumber>::allocate_in_zone(&zone, HeapNumber::new(number_map, -0.0))?
      .cast::<Object>();
  let normal_num = Tagged::<HeapNumber>::allocate_in_zone(&zone, HeapNumber::new(number_map, 1.5))?
    .cast::<Object>();

  unsafe {
    assert!(is_nan(nan_obj));
    assert!(!is_nan(minus_zero_obj));
    assert!(!is_nan(normal_num));

    assert!(is_minus_zero(minus_zero_obj));
    assert!(!is_minus_zero(nan_obj));
    assert!(!is_minus_zero(normal_num));

    // Smi 不是 NaN 也不是 -0.0
    let smi = Smi::zero().cast::<Object>();
    assert!(!is_nan(smi));
    assert!(!is_minus_zero(smi));
  }

  // 2. Symbol: Public, Private, UniqueName
  let symbol_map =
    Tagged::<Map>::allocate_in_zone(&zone, Map::new(meta_map, InstanceType::Symbol, 32))?;

  let mut public_sym = Symbol::new(symbol_map, Tagged::<Object>::NULL);
  public_sym.set_private_symbol_kind(PrivateSymbolKind::Public);
  let pub_obj = Tagged::<Symbol>::allocate_in_zone(&zone, public_sym)?.cast::<Object>();

  let mut priv_sym = Symbol::new(symbol_map, Tagged::<Object>::NULL);
  priv_sym.set_private_symbol_kind(PrivateSymbolKind::Internal);
  let priv_obj = Tagged::<Symbol>::allocate_in_zone(&zone, priv_sym)?.cast::<Object>();

  unsafe {
    assert!(is_public_symbol(pub_obj));
    assert!(!is_private_symbol(pub_obj));

    assert!(!is_public_symbol(priv_obj));
    assert!(is_private_symbol(priv_obj));

    assert!(is_unique_name(pub_obj));
    assert!(is_unique_name(priv_obj));
  }

  OK
}
