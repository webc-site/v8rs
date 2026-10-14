//! basic-slot-set-unittest.cc 1:1 迁移 + API 语义补充。
use std::thread::spawn;

use aok::{OK, Void};
use v8_heap_base::{
  AccessMode, BasicSlotSet, EmptyBucketMode, K_BITS_PER_BUCKET, SlotCallbackResult,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

const G: usize = size_of::<usize>(); // kTestGranularity
type TestSlotSet = BasicSlotSet<G>;
const K_TEST_PAGE_SIZE: usize = 1 << 17;
const K_BUCKETS_TEST_PAGE: usize = TestSlotSet::buckets_for_size(K_TEST_PAGE_SIZE);

// 槽偏移在测试中恒合法, lookup/insert 的 Result 可安全解包
fn ins(set: &TestSlotSet, i: usize) {
  set.insert(AccessMode::Atomic, i).unwrap();
}
fn look(set: &TestSlotSet, i: usize) -> bool {
  set.lookup(i).unwrap()
}

// TEST(BasicSlotSet, InsertAndLookup1) cc:21-33
#[test]
fn insert_and_lookup1() -> Void {
  let set = TestSlotSet::allocate(K_BUCKETS_TEST_PAGE);
  let mut i = 0;
  while i < K_TEST_PAGE_SIZE {
    assert!(!look(&set, i));
    i += G;
  }
  i = 0;
  while i < K_TEST_PAGE_SIZE {
    ins(&set, i);
    i += G;
  }
  i = 0;
  while i < K_TEST_PAGE_SIZE {
    assert!(look(&set, i));
    i += G;
  }
  OK
}

// TEST(BasicSlotSet, InsertAndLookup2) cc:35-50
#[test]
fn insert_and_lookup2() -> Void {
  let set = TestSlotSet::allocate(K_BUCKETS_TEST_PAGE);
  let mut i = 0;
  while i < K_TEST_PAGE_SIZE {
    if i % 7 == 0 {
      ins(&set, i);
    }
    i += G;
  }
  i = 0;
  while i < K_TEST_PAGE_SIZE {
    assert_eq!(look(&set, i), i % 7 == 0);
    i += G;
  }
  OK
}

// TEST(BasicSlotSet, Iterate) cc:52-81
#[test]
fn iterate() -> Void {
  let set = TestSlotSet::allocate(K_BUCKETS_TEST_PAGE);
  let mut i = 0;
  while i < K_TEST_PAGE_SIZE {
    if i % 7 == 0 {
      ins(&set, i);
    }
    i += G;
  }
  set.iterate(
    0,
    0,
    K_BUCKETS_TEST_PAGE,
    |slot| {
      if slot % 3 == 0 {
        SlotCallbackResult::KeepSlot
      } else {
        SlotCallbackResult::RemoveSlot
      }
    },
    EmptyBucketMode::KeepEmptyBuckets,
  );
  i = 0;
  while i < K_TEST_PAGE_SIZE {
    assert_eq!(look(&set, i), i % 21 == 0);
    i += G;
  }
  OK
}

// TEST(BasicSlotSet, IterateFromHalfway) cc:83-114
#[test]
fn iterate_from_halfway() -> Void {
  let set = TestSlotSet::allocate(K_BUCKETS_TEST_PAGE);
  let mut i = 0;
  while i < K_TEST_PAGE_SIZE {
    if i % 7 == 0 {
      ins(&set, i);
    }
    i += G;
  }
  set.iterate(
    0,
    K_BUCKETS_TEST_PAGE / 2,
    K_BUCKETS_TEST_PAGE,
    |slot| {
      if slot % 3 == 0 {
        SlotCallbackResult::KeepSlot
      } else {
        SlotCallbackResult::RemoveSlot
      }
    },
    EmptyBucketMode::KeepEmptyBuckets,
  );
  i = 0;
  while i < K_TEST_PAGE_SIZE {
    let expect = if i < K_TEST_PAGE_SIZE / 2 {
      i % 7 == 0
    } else {
      i % 21 == 0
    };
    assert_eq!(look(&set, i), expect);
    i += G;
  }
  OK
}

// TEST(BasicSlotSet, Remove) cc:116-140
#[test]
fn remove() -> Void {
  let set = TestSlotSet::allocate(K_BUCKETS_TEST_PAGE);
  let mut i = 0;
  while i < K_TEST_PAGE_SIZE {
    if i % 7 == 0 {
      ins(&set, i);
    }
    i += G;
  }
  i = 0;
  while i < K_TEST_PAGE_SIZE {
    if i % 3 != 0 {
      set.remove(i).unwrap();
    }
    i += G;
  }
  i = 0;
  while i < K_TEST_PAGE_SIZE {
    assert_eq!(look(&set, i), i % 21 == 0);
    i += G;
  }
  OK
}

// CheckRemoveRangeOn (basic-slot-set-unittest.cc:143-164)
fn check_remove_range_on(start: usize, end: usize) {
  let set = TestSlotSet::allocate(K_BUCKETS_TEST_PAGE);
  let first = if start == 0 { 0 } else { start - G };
  let last = if end == K_TEST_PAGE_SIZE {
    end - G
  } else {
    end
  };
  for mode in [
    EmptyBucketMode::FreeEmptyBuckets,
    EmptyBucketMode::KeepEmptyBuckets,
  ] {
    let mut i = first;
    while i <= last {
      ins(&set, i);
      i += G;
    }
    set
      .remove_range(start, end, K_BUCKETS_TEST_PAGE, mode)
      .unwrap();
    if first != start {
      assert!(look(&set, first));
    }
    if last == end {
      assert!(look(&set, last));
    }
    let mut i = start;
    while i < end {
      assert!(!look(&set, i));
      i += G;
    }
  }
}

// TEST(BasicSlotSet, RemoveRange) cc:167-195
#[test]
fn remove_range() -> Void {
  check_remove_range_on(0, K_TEST_PAGE_SIZE);
  check_remove_range_on(G, 1023 * G);
  for start in 0..=32usize {
    check_remove_range_on(start * G, (start + 1) * G);
    check_remove_range_on(start * G, (start + 2) * G);
    const K_ENDS: [usize; 7] = [32, 64, 100, 128, 1024, 1500, 2048];
    for &e in K_ENDS.iter() {
      for k in -3i64..=3 {
        let end = (e as i64 + k) as usize;
        if start < end {
          check_remove_range_on(start * G, end * G);
        }
      }
    }
  }
  let set = TestSlotSet::allocate(K_BUCKETS_TEST_PAGE);
  for mode in [
    EmptyBucketMode::FreeEmptyBuckets,
    EmptyBucketMode::KeepEmptyBuckets,
  ] {
    ins(&set, K_TEST_PAGE_SIZE / 2);
    set
      .remove_range(0, K_TEST_PAGE_SIZE, K_BUCKETS_TEST_PAGE, mode)
      .unwrap();
    let mut i = 0;
    while i < K_TEST_PAGE_SIZE {
      assert!(!look(&set, i));
      i += G;
    }
  }
  OK
}

// API 语义补充: 越界错误 + 索引换算 + 空桶释放
#[test]
fn api_semantics() -> Void {
  // 越界偏移 → SlotBucketOutOfRange (原 CHECK → Err)
  let set = TestSlotSet::allocate(K_BUCKETS_TEST_PAGE);
  assert!(set.insert(AccessMode::Atomic, K_TEST_PAGE_SIZE).is_err());
  assert!(set.lookup(K_TEST_PAGE_SIZE).is_err());
  // remove_range 端点越界 → Err (原 CHECK_LE)
  assert!(
    set
      .remove_range(
        0,
        K_TEST_PAGE_SIZE + G,
        K_BUCKETS_TEST_PAGE,
        EmptyBucketMode::KeepEmptyBuckets
      )
      .is_err()
  );
  // 索引换算: bucket ↔ offset 互逆, buckets_for_size 上取整
  assert_eq!(TestSlotSet::bucket_for_slot(0), 0);
  assert_eq!(TestSlotSet::bucket_for_slot(K_BITS_PER_BUCKET * G), 1);
  assert_eq!(TestSlotSet::offset_for_bucket(1), K_BITS_PER_BUCKET * G);
  assert_eq!(TestSlotSet::buckets_for_size(1), 1);
  assert_eq!(TestSlotSet::buckets_for_size(K_BITS_PER_BUCKET * G), 1);
  assert_eq!(TestSlotSet::buckets_for_size(K_BITS_PER_BUCKET * G + 1), 2);
  assert_eq!(set.num_buckets(), K_BUCKETS_TEST_PAGE);
  // free_bucket_if_empty: 空桶释放返回 true, 再插不残留
  assert!(set.free_bucket_if_empty(0));
  ins(&set, 0);
  assert!(!set.free_bucket_if_empty(0));
  // contains == lookup (原版二者同体)
  assert!(set.contains(0).unwrap());
  OK
}

// API 语义补充: iterate 返回保留槽计数 + FreeEmptyBuckets 释放后仍可重新插入
#[test]
fn iterate_count_and_free_buckets() -> Void {
  let set = TestSlotSet::allocate(K_BUCKETS_TEST_PAGE);
  for i in (0..K_TEST_PAGE_SIZE).step_by(G) {
    ins(&set, i);
  }
  let kept = set.iterate(
    0,
    0,
    K_BUCKETS_TEST_PAGE,
    |_| SlotCallbackResult::KeepSlot,
    EmptyBucketMode::FreeEmptyBuckets,
  );
  assert_eq!(kept, K_TEST_PAGE_SIZE / G);
  // 释放路径后重插仍可查
  let removed = set.iterate(
    0,
    0,
    K_BUCKETS_TEST_PAGE,
    |_| SlotCallbackResult::RemoveSlot,
    EmptyBucketMode::FreeEmptyBuckets,
  );
  assert_eq!(removed, 0);
  ins(&set, 0);
  assert!(look(&set, 0));
  OK
}

// 并发回归: 多线程并发 insert 与 lookup (验证 Send + Sync 稳固)
#[test]
fn concurrent_insert_and_lookup() -> Void {
  use std::sync::Arc;

  let set = Arc::new(TestSlotSet::allocate(K_BUCKETS_TEST_PAGE));
  let mut handles = Vec::new();
  const K_THREADS: usize = 4;
  const K_SLOTS_PER_THREAD: usize = 200;

  for t in 0..K_THREADS {
    let s = Arc::clone(&set);
    handles.push(spawn(move || {
      for i in 0..K_SLOTS_PER_THREAD {
        let slot = (t * K_SLOTS_PER_THREAD + i) * G;
        s.insert(AccessMode::Atomic, slot).unwrap();
        assert!(s.lookup(slot).unwrap());
      }
    }));
  }

  for h in handles {
    h.join().unwrap();
  }

  for t in 0..K_THREADS {
    for i in 0..K_SLOTS_PER_THREAD {
      let slot = (t * K_SLOTS_PER_THREAD + i) * G;
      assert!(look(&set, slot));
    }
  }
  OK
}
