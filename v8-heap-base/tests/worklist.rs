//! worklist-unittest.cc 1:1 迁移 (条目类型 SomeObject* → usize 指代)。
use std::thread::{spawn, yield_now};

use aok::{OK, Void};
use v8_heap_base::{Worklist, WorklistBase, WorklistLocal, WorklistSegment as Segment};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

const K_MIN_SEGMENT_SIZE: usize = 64;
type TestWorklist = Worklist<usize, K_MIN_SEGMENT_SIZE>;

// TEST(WorkListTest, SegmentCreate) cc:24-29
#[test]
fn segment_create() -> Void {
  let segment = Segment::<usize>::create(K_MIN_SEGMENT_SIZE);
  assert!(segment.is_empty());
  assert_eq!(segment.size(), 0);
  assert!(!segment.is_full());
  OK
}

// TEST(WorkListTest, SegmentPush) cc:31-36
#[test]
fn segment_push() -> Void {
  let mut segment = Segment::<usize>::create(K_MIN_SEGMENT_SIZE);
  assert_eq!(segment.size(), 0);
  segment.push(0);
  assert_eq!(segment.size(), 1);
  OK
}

// TEST(WorkListTest, SegmentPushPop) cc:38-47
#[test]
fn segment_push_pop() -> Void {
  let mut segment = Segment::<usize>::create(K_MIN_SEGMENT_SIZE);
  segment.push(0);
  assert_eq!(segment.size(), 1);
  let object = segment.pop();
  assert_eq!(segment.size(), 0);
  assert_eq!(object, Some(0));
  OK
}

// TEST(WorkListTest, SegmentIsEmpty) cc:49-54
#[test]
fn segment_is_empty() -> Void {
  let mut segment = Segment::<usize>::create(K_MIN_SEGMENT_SIZE);
  assert!(segment.is_empty());
  segment.push(0);
  assert!(!segment.is_empty());
  OK
}

// TEST(WorkListTest, SegmentIsFull) cc:56-63
#[test]
fn segment_is_full() -> Void {
  let mut segment = Segment::<usize>::create(K_MIN_SEGMENT_SIZE);
  assert!(!segment.is_full());
  for _ in 0..segment.capacity() {
    segment.push(0);
  }
  assert!(segment.is_full());
  OK
}

// TEST(WorkListTest, SegmentClear) cc:65-74
#[test]
fn segment_clear() -> Void {
  let mut segment = Segment::<usize>::create(K_MIN_SEGMENT_SIZE);
  segment.push(0);
  assert!(!segment.is_empty());
  segment.clear();
  assert!(segment.is_empty());
  for _ in 0..segment.capacity() {
    segment.push(0);
  }
  OK
}

// TEST(WorkListTest, SegmentUpdateFalse) cc:76-83
#[test]
fn segment_update_false() -> Void {
  let mut segment = Segment::<usize>::create(K_MIN_SEGMENT_SIZE);
  segment.push(1);
  segment.update(|_, _| false);
  assert!(segment.is_empty());
  OK
}

// TEST(WorkListTest, SegmentUpdate) cc:85-99
#[test]
fn segment_update() -> Void {
  let mut segment = Segment::<usize>::create(K_MIN_SEGMENT_SIZE);
  segment.push(1);
  segment.update(|_, out| {
    *out = 2;
    true
  });
  assert_eq!(segment.pop(), Some(2));
  OK
}

// TEST(WorkListTest, CreateEmpty) cc:101-106
#[test]
fn create_empty() -> Void {
  let worklist = TestWorklist::new();
  let worklist_local = WorklistLocal::new(&worklist);
  assert!(worklist_local.is_local_empty());
  assert!(worklist.is_empty());
  OK
}

// TEST(WorkListTest, LocalPushPop) cc:108-117
#[test]
fn local_push_pop() -> Void {
  let worklist = TestWorklist::new();
  let mut worklist_local = WorklistLocal::new(&worklist);
  worklist_local.push(42);
  assert!(!worklist_local.is_local_empty());
  assert_eq!(worklist_local.pop(), Some(42));
  OK
}

// TEST(WorkListTest, LocalPushStaysPrivate) cc:119-134
#[test]
fn local_push_stays_private() -> Void {
  let worklist = TestWorklist::new();
  let mut local1 = WorklistLocal::new(&worklist);
  let mut local2 = WorklistLocal::new(&worklist);
  assert!(worklist.is_empty());
  assert_eq!(worklist.size(), 0);
  local1.push(42);
  assert_eq!(worklist.size(), 0);
  assert_eq!(local2.pop(), None);
  assert_eq!(local1.pop(), Some(42));
  assert_eq!(worklist.size(), 0);
  OK
}

// TEST(WorkListTest, LocalClear) cc:136-158
#[test]
fn local_clear() -> Void {
  let worklist = TestWorklist::new();
  let mut local = WorklistLocal::new(&worklist);
  // push 段:
  assert!(local.is_local_empty());
  local.push(1);
  assert!(!local.is_local_empty());
  local.clear();
  assert!(local.is_local_empty());
  // pop 段:
  local.push(1);
  local.push(1);
  assert!(!local.is_local_empty());
  local.publish();
  assert!(local.is_local_empty());
  assert!(local.pop().is_some());
  assert!(!local.is_local_empty());
  local.clear();
  assert!(local.is_local_empty());
  OK
}

// TEST(WorkListTest, GlobalUpdateNull) cc:160-173
#[test]
fn global_update_null() -> Void {
  let worklist = TestWorklist::new();
  let mut local = WorklistLocal::new(&worklist);
  for _ in 0..K_MIN_SEGMENT_SIZE {
    local.push(1);
  }
  local.push(1);
  local.publish();
  worklist.update(|_, _| false);
  assert!(worklist.is_empty());
  assert_eq!(worklist.size(), 0);
  OK
}

// TEST(WorkListTest, GlobalUpdate) cc:175-204
#[test]
fn global_update() -> Void {
  let worklist = TestWorklist::new();
  let mut local = WorklistLocal::new(&worklist);
  for _ in 0..K_MIN_SEGMENT_SIZE {
    local.push(1); // objectA
  }
  for _ in 0..K_MIN_SEGMENT_SIZE {
    local.push(2); // objectB
  }
  local.push(1);
  local.publish();
  worklist.update(|object, out| {
    if *object != 1 {
      *out = 3; // objectC
      true
    } else {
      false
    }
  });
  for _ in 0..K_MIN_SEGMENT_SIZE {
    assert_eq!(local.pop(), Some(3));
  }
  OK
}

// TEST(WorkListTest, FlushToGlobalPushSegment) cc:206-217
#[test]
fn flush_to_global_push_segment() -> Void {
  let worklist = TestWorklist::new();
  let mut local0 = WorklistLocal::new(&worklist);
  let mut local1 = WorklistLocal::new(&worklist);
  local0.push(1);
  local0.publish();
  assert_eq!(worklist.size(), 1);
  assert_eq!(local1.pop(), Some(1));
  OK
}

// TEST(WorkListTest, FlushToGlobalPopSegment) cc:219-232
#[test]
fn flush_to_global_pop_segment() -> Void {
  let worklist = TestWorklist::new();
  let mut local0 = WorklistLocal::new(&worklist);
  let mut local1 = WorklistLocal::new(&worklist);
  local0.push(1);
  local0.push(1);
  assert_eq!(local0.pop(), Some(1)); // pop 段持 1 条
  local0.publish();
  assert_eq!(worklist.size(), 1);
  assert_eq!(local1.pop(), Some(1));
  OK
}

// TEST(WorkListTest, Clear) cc:234-245
#[test]
fn clear() -> Void {
  let worklist = TestWorklist::new();
  let mut local = WorklistLocal::new(&worklist);
  local.push(1);
  local.publish();
  assert_eq!(worklist.size(), 1);
  worklist.clear();
  assert!(worklist.is_empty());
  assert_eq!(worklist.size(), 0);
  OK
}

// TEST(WorkListTest, SingleSegmentSteal) cc:247-266
#[test]
fn single_segment_steal() -> Void {
  let worklist = TestWorklist::new();
  let mut local1 = WorklistLocal::new(&worklist);
  let mut local2 = WorklistLocal::new(&worklist);
  for _ in 0..K_MIN_SEGMENT_SIZE {
    local1.push(7);
  }
  local1.publish();
  assert_eq!(worklist.size(), 1);
  // 窃取:
  for _ in 0..K_MIN_SEGMENT_SIZE {
    assert_eq!(local2.pop(), Some(7));
    assert_eq!(local1.pop(), None);
  }
  assert!(worklist.is_empty());
  assert_eq!(worklist.size(), 0);
  OK
}

// TEST(WorkListTest, MultipleSegmentsStolen) cc:268-305
#[test]
fn multiple_segments_stolen() -> Void {
  let worklist = TestWorklist::new();
  let mut local1 = WorklistLocal::new(&worklist);
  let mut local2 = WorklistLocal::new(&worklist);
  let mut local3 = WorklistLocal::new(&worklist);
  for _ in 0..K_MIN_SEGMENT_SIZE {
    local1.push(1); // dummy1
  }
  local1.publish();
  for _ in 0..K_MIN_SEGMENT_SIZE {
    local1.push(2); // dummy2
  }
  local1.publish();
  assert_eq!(worklist.size(), 2);
  // 窃取:
  let expect_bag2 = local2.pop();
  assert!(expect_bag2.is_some());
  let expect_bag2 = expect_bag2.unwrap();
  let expect_bag3 = local3.pop();
  assert!(expect_bag3.is_some());
  let expect_bag3 = expect_bag3.unwrap();
  assert_eq!(worklist.size(), 0);
  assert_ne!(expect_bag2, expect_bag3);
  assert!(expect_bag2 == 1 || expect_bag2 == 2);
  assert!(expect_bag3 == 1 || expect_bag3 == 2);
  for _ in 1..K_MIN_SEGMENT_SIZE {
    assert_eq!(local2.pop(), Some(expect_bag2));
    assert_eq!(local1.pop(), None);
  }
  for _ in 1..K_MIN_SEGMENT_SIZE {
    assert_eq!(local3.pop(), Some(expect_bag3));
    assert_eq!(local1.pop(), None);
  }
  assert!(worklist.is_empty());
  OK
}

// TEST(WorkListTest, MergeGlobalPool) cc:307-331
#[test]
fn merge_global_pool() -> Void {
  let worklist1 = TestWorklist::new();
  let mut local1 = WorklistLocal::new(&worklist1);
  for _ in 0..K_MIN_SEGMENT_SIZE {
    local1.push(7);
  }
  local1.publish();
  // 全局池合并进新 Worklist:
  let worklist2 = TestWorklist::new();
  let mut local2 = WorklistLocal::new(&worklist2);
  assert_eq!(worklist2.size(), 0);
  worklist2.merge(&worklist1);
  assert_eq!(worklist2.size(), 1);
  assert!(!worklist2.is_empty());
  for _ in 0..K_MIN_SEGMENT_SIZE {
    assert_eq!(local2.pop(), Some(7));
    assert_eq!(local1.pop(), None);
  }
  assert!(worklist1.is_empty());
  assert!(worklist2.is_empty());
  OK
}

// API 语义补充: WorklistBase 开关 + Local::Merge (全局池合并) + 迭代只读
#[test]
fn predictable_order_switch_and_local_merge_iterate() -> Void {
  WorklistBase::enforce_predictable_order();
  assert!(WorklistBase::predictable_order());

  let wl_a = TestWorklist::new();
  let wl_b = TestWorklist::new();
  let mut local_b = WorklistLocal::new(&wl_b);
  local_b.push(6);
  local_b.publish();
  assert_eq!(wl_b.size(), 1);

  // 原 Local::Merge: 合并两个全局池 (a ← b)
  let mut local_a = WorklistLocal::new(&wl_a);
  local_a.merge(&mut local_b);
  assert_eq!(wl_a.size(), 1);
  assert!(wl_b.is_empty());

  // 原 Iterate: 只读遍历已发布段
  let mut seen = Vec::new();
  wl_a.iterate(|e| seen.push(*e));
  assert_eq!(seen, vec![6]);

  assert_eq!(local_a.pop(), Some(6));
  assert!(wl_a.is_empty());
  OK
}

// 并发回归: clear 与 publish 交错时 size 计数不得与实际段数脱钩
// (原版 worklist.h:111/141 的 fetch_add/store(0) 均在 MutexGuard 内;
//  修复前 Rust 侧在锁外, store(0) 会覆盖并发 push 晚到的 fetch_add)。
#[test]
fn concurrent_clear_publish_size_invariant() -> Void {
  use std::sync::{
    Arc, Barrier,
    atomic::{AtomicBool, Ordering},
  };

  const K_THREADS: usize = 4;
  const K_PUBLISH_PER_THREAD: usize = 2000;

  let worklist = Arc::new(TestWorklist::new());
  let barrier = Arc::new(Barrier::new(K_THREADS + 1));
  let clearing = Arc::new(AtomicBool::new(true));
  let mut handles = Vec::new();

  for t in 0..K_THREADS {
    let wl = Arc::clone(&worklist);
    let b = Arc::clone(&barrier);
    handles.push(spawn(move || {
      b.wait();
      let mut local = WorklistLocal::new(&wl);
      for i in 0..K_PUBLISH_PER_THREAD {
        // 每段恰好 1 条: 终态条目数 == 段数, 与 size 直接可比。
        local.push(t * K_PUBLISH_PER_THREAD + i);
        local.publish();
      }
    }));
  }

  // 清理线程持续 clear, 直到生产者全部完成 (保证竞态窗口充分交错)。
  let wl_clear = Arc::clone(&worklist);
  let b_clear = Arc::clone(&barrier);
  let flag = Arc::clone(&clearing);
  let clearer = spawn(move || {
    b_clear.wait();
    while flag.load(Ordering::Relaxed) {
      wl_clear.clear();
      yield_now();
    }
  });

  // 全部生产线程停机 → 停止清理 → 校验不变式: size == 实际段数 == 条目数。
  for h in handles {
    h.join().unwrap();
  }
  clearing.store(false, Ordering::Relaxed);
  clearer.join().unwrap();

  let mut entries = 0;
  worklist.iterate(|_| entries += 1);
  assert_eq!(
    worklist.size(),
    entries,
    "size 与实际段数脱钩: clear/publish 竞态丢失计数"
  );

  // 吸干全部段, 终态归零 (同时验证段链完整可弹)。
  let mut local = WorklistLocal::new(&worklist);
  let mut popped = 0;
  while local.pop().is_some() {
    popped += 1;
  }
  local.publish();
  assert_eq!(popped, entries);
  assert!(worklist.is_empty());
  OK
}
