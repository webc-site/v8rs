// 迁移规约:
//   name.h:168-262 哈希字段位段族 (HashFieldType/转发索引位/数组索引常量)
//   name-inl.h:113-291 静态判定与构造 (IsHashFieldComputed/IsForwardingIndex/
//     CreateHashFieldValue/CreateInternalized|ExternalForwardingIndex/
//     ContainsCachedArrayIndex/HasHashCode/TryGetHash)
//   name.cc 侧 (string.cc:1940 set_raw_hash_field_if_empty) 与转发表慢路径:
//     归 v8-objects-strings (登记于 name.h.js 脚本头), 此处不迁
//   hashcode-unittest.cc / symbols-unittest.cc: isolate/handle 依赖 → 推迟
use aok::{OK, Void};
use v8_objects_core::{
  name::{
    ARRAY_INDEX_LENGTH_SHIFT, ARRAY_INDEX_VALUE_SIZE, EMPTY_HASH_FIELD,
    FORWARDING_INDEX_VALUE_MASK, HASH_MASK, HashFieldType as Ty, MAX_ARRAY_INDEX,
    MAX_CACHED_ARRAY_INDEX_LENGTH, Name,
  },
  tagged::Tagged,
};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

/// 规约 (name.h:179-187 + name-inl.h:114-116):
/// bit0=0 → 已计算 (Hash 0b10 / IntegerIndex 0b00); bit0=1 → 空或转发。
#[test]
fn hash_field_type_predicates() -> Void {
  assert!(Name::is_hash_field_computed(0b10));
  assert!(Name::is_hash_field_computed(0b00));
  assert!(!Name::is_hash_field_computed(0b11));
  assert!(!Name::is_hash_field_computed(0b01));

  assert!(Name::is_hash(0b10));
  assert!(Name::is_integer_index(0b00));
  assert!(Name::is_forwarding_index(0b01));
  assert_eq!(EMPTY_HASH_FIELD, 0b11);
  OK
}

/// 规约 (name-inl.h:150-154 CreateHashFieldValue + name-inl.h:234-241 hash):
/// 30 位哈希载荷往返; 溢出位被截断 (HashBits::kMax 掩码)。
#[test]
fn create_hash_field_value_roundtrip() -> Void {
  let f = Name::create_hash_field_value(0x1234_5678, Ty::Hash);
  assert!(Name::is_hash(f));
  let n = Name::with_hash_field(Tagged::NULL, f);
  assert_eq!(n.hash(), 0x1234_5678 & (HASH_MASK >> 2));

  // 载荷超 30 位截断
  let big = Name::create_hash_field_value(u32::MAX, Ty::Hash);
  assert_eq!(
    Name::with_hash_field(Tagged::NULL, big).hash(),
    (1u32 << 30) - 1
  );

  // IntegerIndex 型哈希值同样可作属性表哈希
  let idx = Name::create_hash_field_value(77, Ty::IntegerIndex);
  assert!(Name::is_integer_index(idx));
  assert!(Name::is_hash_field_computed(idx));
  OK
}

/// 规约 (name-inl.h:156-169):
/// 内部化/外部转发索引: bit2/bit3 标志 + 28 位行号往返。
#[test]
fn forwarding_index_roundtrip() -> Void {
  let internalized = Name::create_internalized_forwarding_index(9527);
  assert!(Name::is_forwarding_index(internalized));
  assert!(Name::is_internalized_forwarding_index(internalized));
  assert!(!Name::is_external_forwarding_index(internalized));
  assert_eq!((internalized & FORWARDING_INDEX_VALUE_MASK) >> 4, 9527);

  let external = Name::create_external_forwarding_index(1);
  assert!(Name::is_external_forwarding_index(external));
  assert!(!Name::is_internalized_forwarding_index(external));

  // 28 位载荷上界
  let max = Name::create_internalized_forwarding_index((1 << 28) - 1);
  assert!(Name::is_internalized_forwarding_index(max));
  OK
}

/// 规约 (name-inl.h:171-183 HasHashCode 族 — 语义修正点):
/// Hash/IntegerIndex → true; Empty → false; ForwardingIndex → true (可换算)。
#[test]
fn has_hash_code_semantics() -> Void {
  let hashed = Name::with_hash_field(
    Default::default(),
    Name::create_hash_field_value(5, Ty::Hash),
  );
  assert!(hashed.has_hash_code());
  assert_eq!(hashed.try_get_hash(), Some(5));

  let empty = Name::new(Tagged::NULL);
  assert_eq!(empty.raw_hash_field(), EMPTY_HASH_FIELD);
  assert!(!empty.has_hash_code());
  assert_eq!(empty.try_get_hash(), None);
  assert_eq!(empty.hash_field_type(), Ty::Empty);

  let fwd = Name::with_hash_field(Tagged::NULL, Name::create_external_forwarding_index(9));
  assert!(fwd.has_hash_code());
  assert!(fwd.has_forwarding_index());
  assert!(fwd.has_external_forwarding_index());
  // 转发表换算推迟 (归 v8-objects-strings): try_get_hash 不出值
  assert_eq!(fwd.try_get_hash(), None);
  OK
}

/// 规约 (name.h:198-262 数组索引常量 + name-inl.h:289-291):
/// 缓存数组索引 = 低 26 位放索引值 + 高 6 位放长度, 类型位为 IntegerIndex。
#[test]
fn cached_array_index_mask() -> Void {
  assert_eq!(ARRAY_INDEX_VALUE_SIZE, 24); // 原版 kArrayIndexValueBits
  assert_eq!(ARRAY_INDEX_LENGTH_SHIFT, 26);
  assert_eq!(MAX_CACHED_ARRAY_INDEX_LENGTH, 7);
  assert_eq!(MAX_ARRAY_INDEX, u32::MAX - 1);

  // 形如 (len << 26) | (index << 2) | 0b00 → 含缓存数组索引
  let cached = (3u32 << 26) | (42 << 2);
  assert!(Name::contains_cached_array_index(cached));
  // Hash 型 / 空型 / 转发型 → 不含
  for raw in [
    Name::create_hash_field_value(7, Ty::Hash),
    EMPTY_HASH_FIELD,
    Name::create_internalized_forwarding_index(1),
  ] {
    assert!(!Name::contains_cached_array_index(raw), "raw={raw:#x}");
  }
  // 长度超 kMaxCachedArrayIndexLength (7) → 不含
  assert!(!Name::contains_cached_array_index((8 << 26) | (1 << 2)));
  OK
}

/// 规约 (name-inl.h:234-254 hash/TryGetHash + 便捷 set_hash)。
#[test]
fn hash_accessors() -> Void {
  let mut n = Name::new(Default::default());
  n.set_hash(0x3FFF_FFFF);
  assert_eq!(n.hash(), 0x3FFF_FFFF);
  assert_eq!(n.try_get_hash(), Some(0x3FFF_FFFF));
  assert!(n.has_hash_code());
  n.set_raw_hash_field(EMPTY_HASH_FIELD);
  assert_eq!(n.hash_field_type(), Ty::Empty);
  OK
}
