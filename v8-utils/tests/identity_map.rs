// 迁移自 v8/test/unittests/utils/identity-map-unittest.cc
// 断言规约: TEST_F(IdentityMapTest, X) → fn x (snake_case); CHECK* → assert*。
// 环境替换: TestWithIsolateAndZone → 纯地址键;
//   smi(v)  = (v as u64) << 32        (64 位 Smi 编码, Internals::IntegralToSmi)
//   num(v)  = 假堆地址 (0x1000 + i*8) | 1 (heap object tag, Internals::HasHeapObjectTag)
// 跳过: ExplicitGC / GCShortCutting / IdentityMapOnClientIsolate (依赖真 GC、共享堆)。
use std::collections::HashSet;

use aok::{OK, Void};
use v8_utils::identity_map::IdentityMap;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// Smi 键编码 (上游 DirectHandle<Smi> + smi())
fn smi(value: i64) -> u64 {
  (value as u64) << 32
}

/// 假堆对象地址键 (上游 num() = factory->NewNumber)
fn num(i: i64) -> u64 {
  (0x1000 + (i as u64) * 8) | 1 // heap object tag
}

/// 测试辅助 (上游 IdentityMapTester)
struct IdentityMapTester {
  map: IdentityMap<usize>,
}

impl Default for IdentityMapTester {
  fn default() -> Self {
    Self::new()
  }
}

impl IdentityMapTester {
  fn new() -> Self {
    IdentityMapTester {
      map: IdentityMap::new(),
    }
  }

  // TestInsertFind: 插入后反复查找
  fn test_insert_find(&mut self, key1: u64, val1: usize, key2: u64, val2: usize) {
    assert!(self.map.find(key1).is_none());
    assert!(self.map.find(key2).is_none());

    // Set {key1} the first time.
    let find_result = self.map.find_or_insert(key1);
    *find_result.entry = val1;

    for _ in 0..3 {
      {
        let r = self.map.find_or_insert(key1);
        assert!(r.already_exists);
        assert_eq!(*r.entry, val1);
        assert!(self.map.find(key2).is_none());
      }
      {
        let nentry = self.map.find(key1);
        assert_eq!(nentry, Some(&val1));
        assert!(self.map.find(key2).is_none());
      }
    }

    // Set {key2} the first time.
    let find_result2 = self.map.find_or_insert(key2);
    assert!(!find_result2.already_exists);
    *find_result2.entry = val2;

    for _ in 0..3 {
      {
        let r = self.map.find_or_insert(key2);
        assert!(r.already_exists);
        assert_eq!(*r.entry, val2);
      }
      {
        let nentry = self.map.find(key2);
        assert_eq!(nentry, Some(&val2));
      }
      assert_eq!(self.map.find(key1), Some(&val1));
    }
  }

  // TestFindDelete: 插入后逐一删除
  fn test_find_delete(&mut self, key1: u64, val1: usize, key2: u64, val2: usize) {
    assert!(self.map.find(key1).is_none());
    assert!(self.map.find(key2).is_none());

    *self.map.find_or_insert(key1).entry = val1;
    *self.map.find_or_insert(key2).entry = val2;

    for _ in 0..3 {
      assert_eq!(self.map.find(key2), Some(&val2));
      assert_eq!(self.map.find(key1), Some(&val1));
    }

    // Delete {key1}
    let mut deleted_entry_1 = 0usize;
    assert!(self.map.delete(key1, Some(&mut deleted_entry_1)));
    assert_eq!(deleted_entry_1, val1);

    for _ in 0..3 {
      assert!(self.map.find(key1).is_none());
      assert_eq!(self.map.find(key2), Some(&val2));
    }

    // Delete {key2}
    let mut deleted_entry_2 = 0usize;
    assert!(self.map.delete(key2, Some(&mut deleted_entry_2)));
    assert_eq!(deleted_entry_2, val2);

    for _ in 0..3 {
      assert!(self.map.find(key1).is_none());
      assert!(self.map.find(key2).is_none());
    }
  }

  // CheckFind
  fn check_find(&mut self, key: u64, value: usize) {
    let entry = self.map.find(key);
    assert_eq!(entry, Some(&value));
  }

  // CheckFindOrInsert
  fn check_find_or_insert(&mut self, key: u64, value: usize) {
    let r = self.map.find_or_insert(key);
    assert!(r.already_exists);
    assert_eq!(*r.entry, value);
  }

  // CheckDelete
  fn check_delete(&mut self, key: u64, value: usize) {
    let mut entry = 0usize;
    assert!(self.map.delete(key, Some(&mut entry)));
    assert_eq!(entry, value);
  }

  // SimulateGCByIncrementingSmisBy
  fn simulate_gc_by_incrementing_smis_by(&mut self, shift: i64) {
    self.map.simulate_gc_by_incrementing_smis_by(shift);
  }

  fn resize(&mut self) {
    self.map.resize_for_testing(self.map.capacity() * 4);
  }

  fn rehash(&mut self) {
    self.map.rehash_for_testing();
  }
}

// IterateCollisionTest(stride)
fn iterate_collision_test(stride: i64) {
  let mut load = 15i64;
  while load <= 120 {
    let mut t = IdentityMapTester::new();

    // Add entries to the map.
    let mut next = 1i64;
    for _ in 0..load {
      t.map.insert(smi(next), next as usize);
      t.check_find(smi(next), next as usize);
      next += stride;
    }
    // Iterate through the map and check we see all elements only once.
    let mut seen = HashSet::new();
    {
      let scope = t.map.iter_scope();
      scope.for_each(|_k, v| {
        assert!(seen.insert(*v), "迭代重复值 {v}");
      });
    }
    // Check get and find on map (上游: seen 集合完备性).
    let mut next = 1i64;
    for _ in 0..load {
      assert!(seen.contains(&(next as usize)), "迭代漏值 {next}");
      t.check_find(smi(next), next as usize);
      t.check_find_or_insert(smi(next), next as usize);
      next += stride;
    }
    load *= 2;
  }
}

// CollisionTest(stride, rehash, resize)
fn collision_test(stride: i64, rehash: bool, resize: bool) {
  let mut load = 15i64;
  while load <= 120 {
    let mut t = IdentityMapTester::new();

    {
      let mut next = 1i64;
      for _ in 0..load {
        t.map.insert(smi(next), next as usize);
        t.check_find(smi(next), next as usize);
        next += stride;
      }
    }
    if resize {
      t.resize(); // Explicit resize (internal method).
    }
    if rehash {
      t.rehash(); // Explicit rehash (internal method).
    }
    {
      let mut next = 1i64;
      for _ in 0..load {
        t.check_find(smi(next), next as usize);
        t.check_find_or_insert(smi(next), next as usize);
        next += stride;
      }
    }
    load *= 2;
  }
}

// Find_smi_not_found
#[test]
fn find_smi_not_found() -> Void {
  let mut t = IdentityMapTester::new();
  for i in 0..100 {
    assert!(t.map.find(smi(i)).is_none());
  }
  OK
}

// Find_num_not_found
#[test]
fn find_num_not_found() -> Void {
  let mut t = IdentityMapTester::new();
  for i in 0..100 {
    assert!(t.map.find(num(i + 1)).is_none());
  }
  OK
}

// Delete_smi_not_found
#[test]
fn delete_smi_not_found() -> Void {
  let mut t = IdentityMapTester::new();
  for i in 0..100 {
    let mut deleted_value = 0xdeadbeefusize;
    assert!(!t.map.delete(smi(i), Some(&mut deleted_value)));
    assert_eq!(0xdeadbeef, deleted_value); // 未删除时输出不变
  }
  OK
}

// Delete_num_not_found
#[test]
fn delete_num_not_found() -> Void {
  let mut t = IdentityMapTester::new();
  for i in 0..100 {
    let mut deleted_value = 0xdeadbeefusize;
    assert!(!t.map.delete(num(i + 1), Some(&mut deleted_value)));
    assert_eq!(0xdeadbeef, deleted_value);
  }
  OK
}

// GetFind_smi_0
#[test]
fn get_find_smi_0() -> Void {
  let mut t = IdentityMapTester::new();
  t.test_insert_find(smi(0), 0x1000, smi(1), 0x2000);
  OK
}

// GetFind_smi_13
#[test]
fn get_find_smi_13() -> Void {
  let mut t = IdentityMapTester::new();
  t.test_insert_find(smi(13), 0x1000, smi(17), 0x2000);
  OK
}

// GetFind_num_13
#[test]
fn get_find_num_13() -> Void {
  let mut t = IdentityMapTester::new();
  t.test_insert_find(num(13), 0x1000, num(17), 0x2000);
  OK
}

// Delete_smi_13
#[test]
fn delete_smi_13() -> Void {
  let mut t = IdentityMapTester::new();
  t.test_find_delete(smi(13), 0x1000, smi(17), 0x2000);
  assert!(t.map.empty());
  OK
}

// Delete_num_13
#[test]
fn delete_num_13() -> Void {
  let mut t = IdentityMapTester::new();
  t.test_find_delete(num(13), 0x1000, num(17), 0x2000);
  assert!(t.map.empty());
  OK
}

// GetFind_smi_17m
#[test]
fn get_find_smi_17m() -> Void {
  const INTERVAL: i64 = 17;
  const SHIFT: i64 = 1099;
  let mut t = IdentityMapTester::new();

  let mut i = 1;
  while i < 100 {
    t.map.insert(smi(i), (i + SHIFT) as usize);
    i += INTERVAL;
  }

  let mut i = 1;
  while i < 100 {
    t.check_find(smi(i), (i + SHIFT) as usize);
    i += INTERVAL;
  }

  let mut i = 1;
  while i < 100 {
    t.check_find_or_insert(smi(i), (i + SHIFT) as usize);
    i += INTERVAL;
  }

  for i in 1..100 {
    let entry = t.map.find(smi(i));
    if i % INTERVAL != 1 {
      assert!(entry.is_none());
    } else {
      assert_eq!(entry, Some(&((i + SHIFT) as usize)));
    }
  }
  OK
}

// Delete_smi_17m
#[test]
fn delete_smi_17m() -> Void {
  const INTERVAL: i64 = 17;
  const SHIFT: i64 = 1099;
  let mut t = IdentityMapTester::new();

  let mut i = 1;
  while i < 100 {
    t.map.insert(smi(i), (i + SHIFT) as usize);
    i += INTERVAL;
  }

  let mut i = 1;
  while i < 100 {
    t.check_delete(smi(i), (i + SHIFT) as usize);
    let mut j = 1;
    while j < 100 {
      let entry = t.map.find(smi(j));
      if j <= i {
        assert!(entry.is_none());
      } else {
        assert_eq!(entry, Some(&((j + SHIFT) as usize)));
      }
      j += INTERVAL;
    }
    i += INTERVAL;
  }
  OK
}

// GetFind_num_1000
#[test]
fn get_find_num_1000() -> Void {
  const PRIME: i64 = 137;
  let mut t = IdentityMapTester::new();
  let (val1, val2) = (0x1111usize, 0x2222usize);
  for i in 0..1000 {
    t.test_insert_find(smi(i * PRIME), val1, smi(i * PRIME + 1), val2);
  }
  OK
}

// Delete_num_1000
#[test]
fn delete_num_1000() -> Void {
  const PRIME: i64 = 137;
  let mut t = IdentityMapTester::new();

  for i in 0..1000 {
    t.map.insert(smi(i * PRIME), (i * PRIME) as usize);
  }

  // Delete every second value in reverse.
  let mut i = 999i64;
  while i >= 0 {
    let mut entry = 0usize;
    assert!(t.map.delete(smi(i * PRIME), Some(&mut entry)));
    assert_eq!(entry, (i * PRIME) as usize);
    i -= 2;
  }

  for i in 0..1000 {
    let entry = t.map.find(smi(i * PRIME));
    if i % 2 == 1 {
      assert!(entry.is_none());
    } else {
      assert_eq!(entry, Some(&((i * PRIME) as usize)));
    }
  }

  // Delete the rest.
  let mut i = 0i64;
  while i < 1000 {
    let mut entry = 0usize;
    assert!(t.map.delete(smi(i * PRIME), Some(&mut entry)));
    assert_eq!(entry, (i * PRIME) as usize);
    i += 2;
  }

  for i in 0..1000 {
    assert!(t.map.find(smi(i * PRIME)).is_none());
  }
  OK
}

// GetFind_smi_gc
#[test]
fn get_find_smi_gc() -> Void {
  const KEY: i64 = 33;
  const SHIFT: i64 = 1211;
  let mut t = IdentityMapTester::new();

  t.map.insert(smi(KEY), 0x3000);
  t.simulate_gc_by_incrementing_smis_by(SHIFT);
  t.check_find(smi(KEY + SHIFT), 0x3000);
  t.check_find_or_insert(smi(KEY + SHIFT), 0x3000);
  OK
}

// Delete_smi_gc
#[test]
fn delete_smi_gc() -> Void {
  const KEY: i64 = 33;
  const SHIFT: i64 = 1211;
  let mut t = IdentityMapTester::new();

  t.map.insert(smi(KEY), 0x3000);
  t.simulate_gc_by_incrementing_smis_by(SHIFT);
  t.check_delete(smi(KEY + SHIFT), 0x3000);
  OK
}

// GetFind_smi_gc2
#[test]
fn get_find_smi_gc2() -> Void {
  let (key1, key2) = (1i64, 33i64);
  const SHIFT: i64 = 1211;
  let mut t = IdentityMapTester::new();

  t.map.insert(smi(key1), key1 as usize);
  t.map.insert(smi(key2), key2 as usize);
  t.simulate_gc_by_incrementing_smis_by(SHIFT);
  t.check_find(smi(key1 + SHIFT), key1 as usize);
  t.check_find_or_insert(smi(key1 + SHIFT), key1 as usize);
  t.check_find(smi(key2 + SHIFT), key2 as usize);
  t.check_find_or_insert(smi(key2 + SHIFT), key2 as usize);
  OK
}

// Delete_smi_gc2
#[test]
fn delete_smi_gc2() -> Void {
  let (key1, key2) = (1i64, 33i64);
  const SHIFT: i64 = 1211;
  let mut t = IdentityMapTester::new();

  t.map.insert(smi(key1), key1 as usize);
  t.map.insert(smi(key2), key2 as usize);
  t.simulate_gc_by_incrementing_smis_by(SHIFT);
  t.check_delete(smi(key1 + SHIFT), key1 as usize);
  t.check_delete(smi(key2 + SHIFT), key2 as usize);
  OK
}

// GetFind_smi_gc_n
#[test]
fn get_find_smi_gc_n() -> Void {
  const SHIFT: i64 = 12011;
  let mut t = IdentityMapTester::new();
  let keys: [i64; 12] = [
    1,
    2,
    7,
    8,
    15,
    23,
    1 + 32,
    2 + 32,
    7 + 32,
    8 + 32,
    15 + 32,
    23 + 32,
  ];
  // Initialize the map first.
  let mut i = 0;
  while i < keys.len() {
    t.test_insert_find(
      smi(keys[i]),
      keys[i] as usize,
      smi(keys[i + 1]),
      keys[i + 1] as usize,
    );
    i += 2;
  }
  // Check the above initialization.
  for &k in &keys {
    t.check_find(smi(k), k as usize);
  }
  // Simulate a GC by "moving" the smis in the internal keys array.
  t.simulate_gc_by_incrementing_smis_by(SHIFT);
  // Check that searching for the incremented smis finds the same values.
  for &k in &keys {
    t.check_find(smi(k + SHIFT), k as usize);
  }
  // Check that searching for the incremented smis gets the same values.
  for &k in &keys {
    t.check_find_or_insert(smi(k + SHIFT), k as usize);
  }
  OK
}

// Delete_smi_gc_n
#[test]
fn delete_smi_gc_n() -> Void {
  const SHIFT: i64 = 12011;
  let mut t = IdentityMapTester::new();
  let keys: [i64; 12] = [
    1,
    2,
    7,
    8,
    15,
    23,
    1 + 32,
    2 + 32,
    7 + 32,
    8 + 32,
    15 + 32,
    23 + 32,
  ];
  for &k in &keys {
    t.map.insert(smi(k), k as usize);
  }
  t.simulate_gc_by_incrementing_smis_by(SHIFT);
  for &k in &keys {
    t.check_delete(smi(k + SHIFT), k as usize);
  }
  OK
}

// GetFind_smi_num_gc_n: Smi 移动而 heap number 不动
#[test]
fn get_find_smi_num_gc_n() -> Void {
  const SHIFT: i64 = 12019;
  let mut t = IdentityMapTester::new();
  let smi_keys: [i64; 5] = [1, 2, 7, 15, 23];
  let num_keys: [i64; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; // num(i.i) 假地址
  // Initialize the map first.
  for &k in &smi_keys {
    t.map.insert(smi(k), k as usize);
  }
  for &k in &num_keys {
    t.map.insert(num(k), k as usize);
  }
  // Check the above initialization.
  for &k in &smi_keys {
    t.check_find(smi(k), k as usize);
  }
  for &k in &num_keys {
    t.check_find(num(k), k as usize);
  }

  // Simulate a GC by moving SMIs.
  t.simulate_gc_by_incrementing_smis_by(SHIFT);

  // Check that searching for the incremented smis finds the same values.
  for &k in &smi_keys {
    t.check_find(smi(k + SHIFT), k as usize);
    t.check_find_or_insert(smi(k + SHIFT), k as usize);
  }

  // Check that searching for the numbers finds the same values.
  for &k in &num_keys {
    t.check_find(num(k), k as usize);
    t.check_find_or_insert(num(k), k as usize);
  }
  OK
}

// Delete_smi_num_gc_n
#[test]
fn delete_smi_num_gc_n() -> Void {
  const SHIFT: i64 = 12019;
  let mut t = IdentityMapTester::new();
  let smi_keys: [i64; 5] = [1, 2, 7, 15, 23];
  let num_keys: [i64; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  for &k in &smi_keys {
    t.map.insert(smi(k), k as usize);
  }
  for &k in &num_keys {
    t.map.insert(num(k), k as usize);
  }

  t.simulate_gc_by_incrementing_smis_by(SHIFT);

  for &k in &smi_keys {
    t.check_delete(smi(k + SHIFT), k as usize);
  }
  for &k in &num_keys {
    t.check_delete(num(k), k as usize);
  }
  OK
}

// Delete_smi_resizes
#[test]
fn delete_smi_resizes() -> Void {
  const KEY_COUNT: i64 = 1024;
  const VALUE_OFFSET: i64 = 27;
  let mut t = IdentityMapTester::new();

  // Insert one element to initialize map.
  t.map.insert(smi(0), VALUE_OFFSET as usize);

  let initial_capacity = t.map.capacity();
  assert!(initial_capacity < KEY_COUNT as usize);

  // Insert another kKeyCount - 1 keys.
  for i in 1..KEY_COUNT {
    t.map.insert(smi(i), (i + VALUE_OFFSET) as usize);
  }

  // Check capacity increased.
  assert!(t.map.capacity() > initial_capacity);
  assert!(t.map.capacity() >= KEY_COUNT as usize);

  // Delete all the keys.
  for i in 0..KEY_COUNT {
    t.check_delete(smi(i), (i + VALUE_OFFSET) as usize);
  }

  // Should resize back to initial capacity.
  assert_eq!(t.map.capacity(), initial_capacity);
  OK
}

// Iterator_smi_num
#[test]
fn iterator_smi_num() -> Void {
  let mut t = IdentityMapTester::new();
  let smi_keys: [i64; 5] = [1, 2, 7, 15, 23];
  let num_keys: [i64; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  // Initialize the map.
  for (i, &k) in smi_keys.iter().enumerate() {
    t.map.insert(smi(k), i);
  }
  for (i, &k) in num_keys.iter().enumerate() {
    t.map.insert(num(k), i + 5);
  }

  // Check iterator sees all values once.
  {
    let scope = t.map.iter_scope();
    let mut seen = HashSet::new();
    scope.for_each(|_k, v| {
      assert!(seen.insert(*v), "迭代重复值 {v}");
    });
    // 上游以外部 set 校验 0..15 全见
    for i in 0..15usize {
      assert!(seen.contains(&i), "迭代漏值 {i}");
    }
  }
  OK
}

// Iterator_smi_num_gc
#[test]
fn iterator_smi_num_gc() -> Void {
  const SHIFT: i64 = 16039;
  let mut t = IdentityMapTester::new();
  let smi_keys: [i64; 5] = [1, 2, 7, 15, 23];
  let num_keys: [i64; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  for (i, &k) in smi_keys.iter().enumerate() {
    t.map.insert(smi(k), i);
  }
  for (i, &k) in num_keys.iter().enumerate() {
    t.map.insert(num(k), i + 5);
  }

  // Simulate GC by moving the SMIs.
  t.simulate_gc_by_incrementing_smis_by(SHIFT);

  // Check iterator sees all values.
  {
    let scope = t.map.iter_scope();
    let mut seen = HashSet::new();
    scope.for_each(|_k, v| {
      assert!(seen.insert(*v), "迭代重复值 {v}");
    });
    for i in 0..15usize {
      assert!(seen.contains(&i), "迭代漏值 {i}");
    }
  }
  OK
}

// IterateCollisions_1..7
#[test]
fn iterate_collisions_1() -> Void {
  iterate_collision_test(1);
  OK
}
#[test]
fn iterate_collisions_2() -> Void {
  iterate_collision_test(2);
  OK
}
#[test]
fn iterate_collisions_3() -> Void {
  iterate_collision_test(3);
  OK
}
#[test]
fn iterate_collisions_5() -> Void {
  iterate_collision_test(5);
  OK
}
#[test]
fn iterate_collisions_7() -> Void {
  iterate_collision_test(7);
  OK
}

// Collisions_1..7
#[test]
fn collisions_1() -> Void {
  collision_test(1, false, false);
  OK
}
#[test]
fn collisions_2() -> Void {
  collision_test(2, false, false);
  OK
}
#[test]
fn collisions_3() -> Void {
  collision_test(3, false, false);
  OK
}
#[test]
fn collisions_5() -> Void {
  collision_test(5, false, false);
  OK
}
#[test]
fn collisions_7() -> Void {
  collision_test(7, false, false);
  OK
}
#[test]
fn resize() -> Void {
  collision_test(9, false, true);
  OK
}
#[test]
fn rehash() -> Void {
  collision_test(11, true, false);
  OK
}
