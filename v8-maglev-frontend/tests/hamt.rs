//! v8/test/unittests/maglev/hamt-unittest.cc 1:1 迁移 (Wave 9 W9MF)
//! 原版 fixture: HAMTTest : TestWithZone (zone 直给) + HAMTMismatchedShapesTest (派生)。
//! Rust 侧: AccountingAllocator + Zone 就地构造; IntHAMT = Hamt<i32, i32>。

use std::collections::BTreeMap;

use aok::{OK, Void};
use v8_maglev_frontend::{Hamt, HamtIterator};
use v8_zone::{AccountingAllocator, Zone};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// v8: test/unittests/maglev/hamt-unittest.cc:18 using IntHAMT = HAMT<int, int>
type IntHamt = Hamt<i32, i32>;

/// v8: hamt-unittest.cc:21-28 HAMTTest::ToMap (迭代器全量收图)
fn to_map(hamt: &IntHamt) -> BTreeMap<i32, i32> {
  let mut result = BTreeMap::new();
  for (key, value) in hamt.iter() {
    result.insert(key, value);
  }
  result
}

/// v8: hamt-unittest.cc:38-40 SameRoot (原版友元直读 root_)
fn same_root(a: &IntHamt, b: &IntHamt) -> bool {
  a.same_root(b)
}

/// v8: hamt-unittest.cc:43-48 TEST_F(HAMTTest, EmptyState)
#[test]
fn empty_state() -> Void {
  let allocator = AccountingAllocator::new();
  let _zone = Zone::new(&allocator, "HAMTTest");
  let hamt = IntHamt::new();

  assert!(hamt.begin().same(&hamt.end()));
  assert!(hamt.find(100).is_none());
  OK
}

/// v8: hamt-unittest.cc:50-72 TEST_F(HAMTTest, BasicInsertAndFind)
#[test]
fn basic_insert_and_find() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "HAMTTest");
  let h0 = IntHamt::new();

  // Insert first element
  let h1 = h0.insert(&zone, 1, 100)?;

  // Verify h1 has it
  let val = h1.find(1);
  assert_eq!(val, Some(&100));

  // Verify h0 is still empty (Persistence/Immutability)
  assert!(h0.find(1).is_none());

  // Insert second element
  let h2 = h1.insert(&zone, 2, 200)?;

  assert_eq!(h2.find(1), Some(&100));
  assert_eq!(h2.find(2), Some(&200));

  // Verify h1 didn't change
  assert!(h1.find(2).is_none());
  OK
}

/// v8: hamt-unittest.cc:74-83 TEST_F(HAMTTest, OverwriteValues)
#[test]
fn overwrite_values() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "HAMTTest");
  let h0 = IntHamt::new();
  let h0 = h0.insert(&zone, 42, 100)?;

  // Overwrite key 42 with 999
  let h1 = h0.insert(&zone, 42, 999)?;

  assert_eq!(h0.find(42), Some(&100)); // Old tree unchanged
  assert_eq!(h1.find(42), Some(&999)); // New tree updated
  OK
}

/// v8: hamt-unittest.cc:85-107 TEST_F(HAMTTest, LargeInsertionToTriggerBranching)
#[test]
fn large_insertion_to_trigger_branching() -> Void {
  // Insert enough items to force creation of Branch nodes and multiple levels.
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "HAMTTest");
  let mut hamt = IntHamt::new();
  let mut expected = BTreeMap::new();

  for i in 0..1000 {
    hamt = hamt.insert(&zone, i, i * 2)?;
    expected.insert(i, i * 2);
  }

  // Verify all items are present
  for (key, value) in &expected {
    assert_eq!(
      hamt.find(*key).copied(),
      Some(*value),
      "Failed to find key {key}"
    );
  }

  // Verify Iterator covers everything
  let actual = to_map(&hamt);
  assert_eq!(expected, actual);
  OK
}

/// v8: hamt-unittest.cc:109-135 TEST_F(HAMTTest, ForceHashCollision_Chaining)
#[test]
fn force_hash_collision_chaining() -> Void {
  // Scenario: Two different keys have the EXACT same hash → same Leaf chain via 'next'.
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "HAMTTest");
  let mut hamt = IntHamt::new();
  let k_fixed_hash: usize = 0xDEADBEEF;

  // Insert Key 1
  hamt = hamt.insert_with_hash(&zone, 1, 10, k_fixed_hash)?;

  // Insert Key 2 with SAME hash
  hamt = hamt.insert_with_hash(&zone, 2, 20, k_fixed_hash)?;

  // Verify both exist
  assert_eq!(hamt.find_with_hash(1, k_fixed_hash), Some(&10));
  assert_eq!(hamt.find_with_hash(2, k_fixed_hash), Some(&20));

  // Verify non-existent key with same hash returns null
  assert!(hamt.find_with_hash(3, k_fixed_hash).is_none());
  OK
}

/// v8: hamt-unittest.cc:137-158 TEST_F(HAMTTest, ForceHashCollision_BranchSplit)
#[test]
fn force_hash_collision_branch_split() -> Void {
  // Scenario: collide on first 5 bits (Level 0), differ on next 5 (Level 1) → new Branch.
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "HAMTTest");
  let mut hamt = IntHamt::new();

  // Hash A: ...00000 00000 (0)
  let hash_a: usize = 0;
  // Hash B: ...00001 00000 (32) — Level 0 同 A, Level 1 分叉
  let hash_b: usize = 1 << 5;

  hamt = hamt.insert_with_hash(&zone, 1, 10, hash_a)?;
  hamt = hamt.insert_with_hash(&zone, 2, 20, hash_b)?;

  assert_eq!(hamt.find_with_hash(1, hash_a), Some(&10));
  assert_eq!(hamt.find_with_hash(2, hash_b), Some(&20));
  OK
}

/// v8: hamt-unittest.cc:160-182 TEST_F(HAMTTest, ForceHashCollision_DeepRecursion)
#[test]
fn force_hash_collision_deep_recursion() -> Void {
  // Force a collision that persists for several levels (递归 InsertRec 验证)。
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "HAMTTest");
  let h0 = IntHamt::new();

  // Both hashes are 0 for the first 10 bits; diverge at Level 2 (bits 10-14).
  let hash_a: usize = 0;
  let hash_b: usize = 1 << 10; // 1024

  let h1 = h0.insert_with_hash(&zone, 1, 100, hash_a)?;
  let h2 = h1.insert_with_hash(&zone, 2, 200, hash_b)?;

  assert!(h0.find_with_hash(1, hash_a).is_none());
  assert!(h0.find_with_hash(2, hash_b).is_none());

  assert_eq!(h1.find_with_hash(1, hash_a), Some(&100));
  assert!(h1.find_with_hash(2, hash_b).is_none());

  assert_eq!(h2.find_with_hash(1, hash_a), Some(&100));
  assert_eq!(h2.find_with_hash(2, hash_b), Some(&200));
  OK
}

/// v8: hamt-unittest.cc:184-209 TEST_F(HAMTTest, UpdateInCollisionChain)
#[test]
fn update_in_collision_chain() -> Void {
  // 碰撞链中段更新 → InsertInCollisionListRec 链上路径复制。
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "HAMTTest");
  let mut h0 = IntHamt::new();
  let k_hash: usize = 0x555;

  // Create chain: [Key1] -> [Key2] -> [Key3]
  h0 = h0.insert_with_hash(&zone, 1, 10, k_hash)?;
  h0 = h0.insert_with_hash(&zone, 2, 20, k_hash)?;
  h0 = h0.insert_with_hash(&zone, 3, 30, k_hash)?;

  // Update Key 2 (middle of chain)
  let h1 = h0.insert_with_hash(&zone, 2, 222, k_hash)?;

  // Verify h1 (New Tree)
  assert_eq!(h1.find_with_hash(1, k_hash), Some(&10)); // Should still be found
  assert_eq!(h1.find_with_hash(2, k_hash), Some(&222)); // Updated
  assert_eq!(h1.find_with_hash(3, k_hash), Some(&30)); // Should still be found

  // Verify h0 (Old Tree) - Persistence Check
  assert_eq!(h0.find_with_hash(2, k_hash), Some(&20)); // Should be original value
  OK
}

/// v8: hamt-unittest.cc:211-238 TEST_F(HAMTTest, MergeIntoIsLeftJoin)
#[test]
fn merge_into_is_left_join() -> Void {
  // Left Join (Keep all from A, update from B).
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "HAMTTest");
  let mut h_a = IntHamt::new();
  h_a = h_a.insert(&zone, 1, 10)?; // In A, not B
  h_a = h_a.insert(&zone, 2, 20)?; // In A and B (Conflict)

  let mut h_b = IntHamt::new();
  h_b = h_b.insert(&zone, 2, 200)?; // In A and B
  h_b = h_b.insert(&zone, 3, 300)?; // In B, not A

  // Perform merge: Result = A merge B (summer = f(this_value, other_value))
  let result = h_a.merge_into(&zone, &h_b, |a: i32, b: i32| a + b)?;

  // 1 was in A, not B. Should remain 10.
  assert_eq!(result.find(1), Some(&10));

  // 2 was in both. Should be 20 + 200 = 220.
  assert_eq!(result.find(2), Some(&220));

  // 3 was in B only. Since implementation iterates mapA (LHS), 不应入结果。
  assert!(result.find(3).is_none());
  OK
}

/// v8: hamt-unittest.cc:240-256 TEST_F(HAMTTest, MergeIntoWithoutChangesDoesNotAllocate)
#[test]
fn merge_into_without_changes_does_not_allocate() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "HAMTTest");
  let mut h_a = IntHamt::new();
  let mut h_b = IntHamt::new();
  for i in 0..200 {
    h_a = h_a.insert(&zone, i, i)?;
    // {h_b} holds smaller values, so a max-merge into {h_a} changes nothing.
    h_b = h_b.insert(&zone, i, i - 1)?;
  }

  let allocated_before = zone.allocation_size();
  let result = h_a.merge_into(&zone, &h_b, i32::max)?;

  assert_eq!(zone.allocation_size(), allocated_before);
  assert!(same_root(&result, &h_a));
  assert_eq!(to_map(&result), to_map(&h_a));
  OK
}

/// v8: hamt-unittest.cc:258-273 TEST_F(HAMTTest, MergeIntoWithoutChangesDoesNotAllocateWithCollisions)
#[test]
fn merge_into_without_changes_does_not_allocate_with_collisions() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "HAMTTest");
  let mut h_a = IntHamt::new();
  let mut h_b = IntHamt::new();
  // Same hash for every key, so everything ends up in one collision chain.
  for i in 0..10 {
    h_a = h_a.insert_with_hash(&zone, i, i, 0x1234)?;
    h_b = h_b.insert_with_hash(&zone, i, i - 1, 0x1234)?;
  }

  let allocated_before = zone.allocation_size();
  let result = h_a.merge_into(&zone, &h_b, i32::max)?;

  assert_eq!(zone.allocation_size(), allocated_before);
  assert!(same_root(&result, &h_a));
  OK
}

/// v8: hamt-unittest.cc:275-302 TEST_F(HAMTTest, MergeIntoWithOneChangeIsAPathCopy)
#[test]
fn merge_into_with_one_change_is_a_path_copy() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "HAMTTest");
  let mut h_a = IntHamt::new();
  let mut h_b = IntHamt::new();
  for i in 0..200 {
    h_a = h_a.insert(&zone, i, i)?;
    h_b = h_b.insert(&zone, i, i - 1)?;
  }
  // A single key now wins the merge.
  h_b = h_b.insert(&zone, 42, 4242)?;

  let allocated_before = zone.allocation_size();
  let result = h_a.merge_into(&zone, &h_b, i32::max)?;
  let allocated = zone.allocation_size() - allocated_before;

  assert!(!same_root(&result, &h_a));
  assert_eq!(result.find(42), Some(&4242));
  for i in 0..200 {
    if i == 42 {
      continue;
    }
    assert_eq!(result.find(i), Some(&i));
  }
  // Only the path to the changed key is rebuilt, not the whole map.
  assert!(allocated < 1024, "path copy 分配超限: {allocated}");
  OK
}

// ── v8: hamt-unittest.cc:304-332 class HAMTMismatchedShapesTest : public HAMTTest ──
// Hashes picked so key 1 is a lone leaf in {small} but shares a level-1 branch
// with key 3 in {big}:
//   0x01: level 0 bit 1, level 1 bit 0
//   0x21: level 0 bit 1, level 1 bit 1
//   0x02: level 0 bit 2
//   small: root{1: Leaf(1), 2: Leaf(2)}
//   big:   root{1: Branch{0: Leaf(1), 1: Leaf(3)}, 2: Leaf(2)}

const K_HASH1: usize = 0x01;
const K_HASH2: usize = 0x02;
const K_HASH3: usize = 0x21;

/// v8: hamt-unittest.cc:318-322 MakeSmall
fn make_small(zone: &Zone<'_>, v1: i32, v2: i32) -> aok::Result<IntHamt> {
  let h = IntHamt::new();
  let h = h.insert_with_hash(zone, 1, v1, K_HASH1)?;
  Ok(h.insert_with_hash(zone, 2, v2, K_HASH2)?)
}

/// v8: hamt-unittest.cc:323-328 MakeBig
fn make_big(zone: &Zone<'_>, v1: i32, v2: i32, v3: i32) -> aok::Result<IntHamt> {
  let h = IntHamt::new();
  let h = h.insert_with_hash(zone, 1, v1, K_HASH1)?;
  let h = h.insert_with_hash(zone, 3, v3, K_HASH3)?;
  Ok(h.insert_with_hash(zone, 2, v2, K_HASH2)?)
}

/// v8: hamt-unittest.cc:334-349 TEST_F(HAMTMismatchedShapesTest, WithoutChangesDoesNotAllocate)
#[test]
fn mismatched_shapes_without_changes_does_not_allocate() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "HAMTMismatchedShapesTest");
  let small = make_small(&zone, 10, 20)?;
  let big = make_big(&zone, 5, 5, 5)?;

  // Leaf(1) x Branch{1, 3}: MergeIntoLeaf.
  let allocated_before = zone.allocation_size();
  let result = small.merge_into(&zone, &big, i32::max)?;
  assert_eq!(zone.allocation_size(), allocated_before);
  assert!(same_root(&result, &small));

  // Branch{1, 3} x Leaf(1): MergeIntoBranch.
  let allocated_before = zone.allocation_size();
  let result = big.merge_into(&zone, &small, i32::min)?;
  assert_eq!(zone.allocation_size(), allocated_before);
  assert!(same_root(&result, &big));
  OK
}

/// v8: hamt-unittest.cc:351-366 TEST_F(HAMTMismatchedShapesTest, WithChanges)
#[test]
fn mismatched_shapes_with_changes() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "HAMTMismatchedShapesTest");
  let small = make_small(&zone, 10, 20)?;
  let big = make_big(&zone, 50, 5, 5)?;

  // Leaf(1) x Branch{1, 3}: only key 1 changes; key 3 is not in {small}.
  let result = small.merge_into(&zone, &big, i32::max)?;
  assert!(!same_root(&result, &small));
  assert_eq!(to_map(&result), to_map_from(&[(1, 50), (2, 20)]));
  assert_eq!(to_map(&small), to_map_from(&[(1, 10), (2, 20)]));

  // Branch{1, 3} x Leaf(1): only key 1 changes; key 3 keeps its value.
  let result = big.merge_into(&zone, &small, i32::min)?;
  assert!(!same_root(&result, &big));
  assert_eq!(to_map(&result), to_map_from(&[(1, 10), (2, 5), (3, 5)]));
  assert_eq!(to_map(&big), to_map_from(&[(1, 50), (2, 5), (3, 5)]));
  OK
}

/// std::map 字面量桥 (原版 :358 比较惯用)
fn to_map_from(items: &[(i32, i32)]) -> BTreeMap<i32, i32> {
  items.iter().copied().collect()
}

/// v8: hamt-unittest.cc:368-383 TEST_F(HAMTTest, IteratorTraversal)
#[test]
fn iterator_traversal() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "HAMTTest");
  let mut hamt = IntHamt::new();
  hamt = hamt.insert(&zone, 10, 1)?;
  hamt = hamt.insert(&zone, 20, 2)?;
  hamt = hamt.insert(&zone, 30, 3)?;

  // 原版手工迭代: begin() != end(), ++it, (*it).first
  let end: HamtIterator<'_, i32, i32> = hamt.end();
  let mut it = hamt.begin();
  let mut count = 0;
  let mut sum_keys = 0;
  while !it.same(&end) {
    count += 1;
    if let Some((key, _value)) = it.item() {
      sum_keys += key;
    }
    it.advance();
  }

  assert_eq!(count, 3);
  assert_eq!(sum_keys, 10 + 20 + 30);
  OK
}

/// v8: hamt-unittest.cc:385-400 TEST_F(HAMTTest, IteratorDeepStructure)
#[test]
fn iterator_deep_structure() -> Void {
  // Force a deep structure and ensure iterator doesn't crash or miss items
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "HAMTTest");
  let k_num_items: i32 = 2000;
  let mut hamt = IntHamt::new();
  for i in 0..k_num_items {
    hamt = hamt.insert(&zone, i, i)?;
  }

  let mut items_seen = 0;
  for (key, value) in hamt.iter() {
    items_seen += 1;
    // Verify key integrity
    assert_eq!(key, value);
  }
  assert_eq!(items_seen, k_num_items as usize);
  OK
}
