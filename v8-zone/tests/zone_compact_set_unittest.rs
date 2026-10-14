//! 原版 v8/test/unittests/zone/zone-compact-set-unittest.cc 转写
//! 用例名保留原名 (TEST_F(ZoneCompactSetTest, Empty) → zone_compact_set_test_empty)

use aok::{OK, Void};
use v8_zone::{AccountingAllocator, Zone, ZoneCompactSet, ZoneCompactSetTraits};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版 HandleLike + operator== (zone-compact-set-unittest.cc:14-18)
#[derive(Clone, Copy, Debug)]
struct HandleLike {
  ptr: *mut i32,
}

impl PartialEq for HandleLike {
  fn eq(&self, other: &Self) -> bool {
    self.ptr == other.ptr
  }
}

impl Eq for HandleLike {}

/// 原版 ZoneCompactSetTraits<HandleLike> 特化 (data_type = int)
impl ZoneCompactSetTraits for HandleLike {
  fn handle_to_pointer(self) -> *mut () {
    self.ptr.cast()
  }

  fn pointer_to_handle(ptr: *mut ()) -> Self {
    HandleLike { ptr: ptr.cast() }
  }
}

type Set = ZoneCompactSet<HandleLike>;

/// 原版 TEST_F(ZoneCompactSetTest, NewHandleLike)
fn new_handle_like(zone: &Zone<'_>, value: i32) -> aok::Result<HandleLike> {
  Ok(HandleLike {
    ptr: zone.make(value)? as *mut i32,
  })
}

/// 原版 TEST_F(ZoneCompactSetTest, Empty)
#[test]
fn zone_compact_set_test_empty() -> Void {
  let set: Set = ZoneCompactSet::new();
  assert_eq!(set.size(), 0);
  assert!(set.is_empty());
  OK
}

/// 原版 TEST_F(ZoneCompactSetTest, SingleValue)
#[test]
fn zone_compact_set_test_single_value() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let mut set: Set = ZoneCompactSet::new();

  let handle = new_handle_like(&zone, 5)?;
  set.insert(handle, &zone)?;

  assert_eq!(set.size(), 1);
  assert!(!set.is_empty());
  assert_eq!(set.at(0), Some(handle));
  assert!(set.contains(handle));
  OK
}

/// 原版 TEST_F(ZoneCompactSetTest, MultipleValue)
#[test]
fn zone_compact_set_test_multiple_value() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let mut set: Set = ZoneCompactSet::new();

  let handle1 = new_handle_like(&zone, 5)?;
  let handle2 = new_handle_like(&zone, 8)?;
  let handle3 = new_handle_like(&zone, 2)?;
  let handle4 = new_handle_like(&zone, 1)?;

  set.insert(handle1, &zone)?;
  set.insert(handle2, &zone)?;
  set.insert(handle3, &zone)?;
  set.insert(handle4, &zone)?;

  assert_eq!(set.size(), 4);
  assert!(!set.is_empty());

  assert!(set.contains(handle1));
  assert!(set.contains(handle2));
  assert!(set.contains(handle3));
  assert!(set.contains(handle4));
  OK
}

/// 原版 TEST_F(ZoneCompactSetTest, DuplicateValue)
#[test]
fn zone_compact_set_test_duplicate_value() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let mut set: Set = ZoneCompactSet::new();

  let handle1 = new_handle_like(&zone, 5)?;
  let handle2 = new_handle_like(&zone, 8)?;

  set.insert(handle1, &zone)?;
  set.insert(handle1, &zone)?;
  set.insert(handle2, &zone)?;
  set.insert(handle2, &zone)?;

  assert_eq!(set.size(), 2);
  assert!(!set.is_empty());

  assert!(set.contains(handle1));
  assert!(set.contains(handle2));
  OK
}

/// 原版 TEST_F(ZoneCompactSetTest, RemoveSingleValue)
#[test]
fn zone_compact_set_test_remove_single_value() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let mut set: Set = ZoneCompactSet::new();

  let handle1 = new_handle_like(&zone, 5)?;

  set.insert(handle1, &zone)?;

  assert_eq!(set.size(), 1);

  set.remove(handle1, &zone)?;

  assert_eq!(set.size(), 0);
  assert!(set.is_empty());

  assert!(!set.contains(handle1));
  OK
}

/// 原版 TEST_F(ZoneCompactSetTest, RemoveFromMultipleValue)
#[test]
fn zone_compact_set_test_remove_from_multiple_value() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let mut set: Set = ZoneCompactSet::new();

  let handle1 = new_handle_like(&zone, 5)?;
  let handle2 = new_handle_like(&zone, 8)?;

  set.insert(handle1, &zone)?;
  set.insert(handle2, &zone)?;

  assert_eq!(set.size(), 2);

  set.remove(handle1, &zone)?;

  assert_eq!(set.size(), 1);
  assert!(!set.is_empty());

  assert!(!set.contains(handle1));
  assert!(set.contains(handle2));
  OK
}

/// 原版 TEST_F(ZoneCompactSetTest, RemoveFromEvenMoreMultipleValue)
#[test]
fn zone_compact_set_test_remove_from_even_more_multiple_value() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let mut set: Set = ZoneCompactSet::new();

  let handle1 = new_handle_like(&zone, 5)?;
  let handle2 = new_handle_like(&zone, 8)?;
  let handle3 = new_handle_like(&zone, 1)?;
  let handle4 = new_handle_like(&zone, 2)?;

  set.insert(handle1, &zone)?;
  set.insert(handle2, &zone)?;
  set.insert(handle3, &zone)?;
  set.insert(handle4, &zone)?;

  assert_eq!(set.size(), 4);

  set.remove(handle2, &zone)?;

  assert_eq!(set.size(), 3);
  assert!(!set.is_empty());

  assert!(set.contains(handle1));
  assert!(!set.contains(handle2));
  assert!(set.contains(handle3));
  assert!(set.contains(handle4));
  OK
}

/// 原版 TEST_F(ZoneCompactSetTest, RemoveNonExistent)
#[test]
fn zone_compact_set_test_remove_non_existent() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let mut set: Set = ZoneCompactSet::new();

  let handle1 = new_handle_like(&zone, 5)?;
  let handle2 = new_handle_like(&zone, 8)?;
  let handle3 = new_handle_like(&zone, 1)?;

  set.insert(handle1, &zone)?;
  set.insert(handle2, &zone)?;

  set.remove(handle3, &zone)?;

  assert_eq!(set.size(), 2);
  assert!(!set.is_empty());

  assert!(set.contains(handle1));
  assert!(set.contains(handle2));
  assert!(!set.contains(handle3));
  OK
}

/// 原版 TEST_F(ZoneCompactSetTest, ContainsEmptySubset)
#[test]
fn zone_compact_set_test_contains_empty_subset() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let mut set: Set = ZoneCompactSet::new();
  let subset: Set = ZoneCompactSet::new();

  let handle1 = new_handle_like(&zone, 5)?;
  let handle2 = new_handle_like(&zone, 8)?;

  set.insert(handle1, &zone)?;
  set.insert(handle2, &zone)?;

  assert!(set.contains_set(&subset));
  assert!(!subset.contains_set(&set));
  OK
}

/// 原版 TEST_F(ZoneCompactSetTest, ContainsSingleElementSubset)
#[test]
fn zone_compact_set_test_contains_single_element_subset() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let mut set: Set = ZoneCompactSet::new();
  let mut subset: Set = ZoneCompactSet::new();

  let handle1 = new_handle_like(&zone, 5)?;
  let handle2 = new_handle_like(&zone, 8)?;

  set.insert(handle1, &zone)?;
  set.insert(handle2, &zone)?;

  subset.insert(handle1, &zone)?;

  assert!(set.contains_set(&subset));
  assert!(!subset.contains_set(&set));
  OK
}

/// 原版 TEST_F(ZoneCompactSetTest, ContainsMultiElementSubset)
#[test]
fn zone_compact_set_test_contains_multi_element_subset() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let mut set: Set = ZoneCompactSet::new();
  let mut subset: Set = ZoneCompactSet::new();

  let handle1 = new_handle_like(&zone, 5)?;
  let handle2 = new_handle_like(&zone, 8)?;
  let handle3 = new_handle_like(&zone, 2)?;
  let handle4 = new_handle_like(&zone, 1)?;

  set.insert(handle1, &zone)?;
  set.insert(handle2, &zone)?;
  set.insert(handle3, &zone)?;
  set.insert(handle4, &zone)?;

  subset.insert(handle2, &zone)?;
  subset.insert(handle3, &zone)?;

  assert!(set.contains_set(&subset));
  assert!(!subset.contains_set(&set));
  OK
}

/// 原版 TEST_F(ZoneCompactSetTest, DoesNotContainsNonSubset)
/// (原版用例名拼写保留)
#[test]
fn zone_compact_set_test_does_not_contains_non_subset() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let mut set: Set = ZoneCompactSet::new();
  let mut other_set: Set = ZoneCompactSet::new();

  let handle1 = new_handle_like(&zone, 5)?;
  let handle2 = new_handle_like(&zone, 8)?;
  let handle3 = new_handle_like(&zone, 2)?;
  let handle4 = new_handle_like(&zone, 1)?;

  set.insert(handle1, &zone)?;
  set.insert(handle2, &zone)?;
  set.insert(handle3, &zone)?;

  other_set.insert(handle2, &zone)?;
  other_set.insert(handle4, &zone)?;

  assert!(!set.contains_set(&other_set));
  assert!(!other_set.contains_set(&set));
  OK
}

/// 补充: 拷贝/迭代/并集/建表构造 (zone-compact-set.h Clone/Union/range 构造)
#[test]
fn zone_compact_set_test_clone_union_and_iter() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let mut set: Set = ZoneCompactSet::new();

  let handle1 = new_handle_like(&zone, 5)?;
  let handle2 = new_handle_like(&zone, 8)?;
  set.insert(handle1, &zone)?;
  set.insert(handle2, &zone)?;

  // Clone(zone): 元素与有序性保持
  let cloned = set.clone_into(&zone)?;
  assert_eq!(cloned, set);

  // 迭代交出全部元素 (有序列表态)
  let mut seen = 0;
  for handle in set.iter() {
    assert!(set.contains(handle));
    seen += 1;
  }
  assert_eq!(seen, 2);

  // Union: 并入新值
  let handle3 = new_handle_like(&zone, 2)?;
  set.union(
    &ZoneCompactSet::from_iter([handle3].into_iter().collect::<Vec<_>>(), &zone)?,
    &zone,
  )?;
  assert_eq!(set.size(), 3);
  assert!(set.contains(handle3));

  // range 构造: 多值直接建有序表
  let built: Set = ZoneCompactSet::from_iter(vec![handle2, handle1], &zone)?;
  assert_eq!(built.size(), 2);
  assert!(built.contains(handle1) && built.contains(handle2));
  OK
}

/// round5 补: 原版 friend hash_value(ZoneCompactSet) = data_.raw()
/// (zone-compact-set.h:264-266, 下游 maglev-cse.h gvn_hash_value 经
/// base::hash_value 分发消费); Rust 侧 std::hash::Hash 对打包字散列,
/// 同值集合散列一致, 不同集合散列不同
#[test]
fn zone_compact_set_test_hash_value_matches_packed_word() -> Void {
  use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
  };

  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");

  let handle = new_handle_like(&zone, 7)?;
  let singleton: Set = ZoneCompactSet::from_handle(handle);

  let mut h1 = DefaultHasher::new();
  singleton.hash(&mut h1);
  let mut h2 = DefaultHasher::new();
  singleton.hash(&mut h2);
  // 同值同散列
  assert_eq!(h1.finish(), h2.finish());

  // 同一元素两种编码 (单值态 vs 建表态) 散列不同
  let packed: Set = ZoneCompactSet::from_iter(vec![handle, handle], &zone)?;
  let mut h3 = DefaultHasher::new();
  packed.hash(&mut h3);
  let mut h4 = DefaultHasher::new();
  singleton.hash(&mut h4);
  assert_ne!(h3.finish(), h4.finish());

  // 空集散列稳定
  let empty: Set = ZoneCompactSet::new();
  let mut h5 = DefaultHasher::new();
  empty.hash(&mut h5);
  let mut h6 = DefaultHasher::new();
  ZoneCompactSet::<HandleLike>::new().hash(&mut h6);
  assert_eq!(h5.finish(), h6.finish());
  OK
}
