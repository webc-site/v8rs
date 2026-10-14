// 1:1 转写 v8/test/unittests/heap/marking-progress-tracker-unittest.cc
// DeathTest (DEBUG-only EXPECT_DEATH) 无对应安全断言面, 见 readme 推迟台账。
use aok::{OK, Void};
use v8_heap_gc::MarkingProgressTracker;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// 原版 static constexpr size_t kObjectSize = 1 << 18;
const K_OBJECT_SIZE: usize = 1 << 18;

// TEST(MarkingProgressTracker, DefaultDisabled)
#[test]
fn default_disabled() -> Void {
  let tracker = MarkingProgressTracker::new();
  assert!(!tracker.is_enabled());
  OK
}

// TEST(MarkingProgressTracker, EnabledAfterExplicitEnable)
#[test]
fn enabled_after_explicit_enable() -> Void {
  let mut tracker = MarkingProgressTracker::new();
  tracker.enable(K_OBJECT_SIZE);
  assert!(tracker.is_enabled());
  OK
}

// TEST(MarkingProgressTracker, ZerothChunkFirst)
#[test]
fn zeroth_chunk_first() -> Void {
  let mut tracker = MarkingProgressTracker::new();
  tracker.enable(K_OBJECT_SIZE);
  assert!(tracker.is_enabled());
  assert_eq!(tracker.get_next_chunk_to_mark(), 0);
  OK
}

// TEST(MarkingProgressTracker, NumberOfChunks)
#[test]
fn number_of_chunks() -> Void {
  let mut tracker = MarkingProgressTracker::new();
  tracker.enable(K_OBJECT_SIZE);
  let expected = K_OBJECT_SIZE.div_ceil(MarkingProgressTracker::K_CHUNK_SIZE);
  assert_eq!(tracker.total_number_of_chunks(), expected);
  OK
}

// TEST(MarkingProgressTracker, GetNextChunkToMarkIncrements)
#[test]
fn get_next_chunk_to_mark_increments() -> Void {
  let mut tracker = MarkingProgressTracker::new();
  tracker.enable(K_OBJECT_SIZE);
  let num_chunks = tracker.total_number_of_chunks();
  assert!(tracker.is_enabled());
  for i in 0..num_chunks {
    assert_eq!(tracker.get_next_chunk_to_mark(), i);
  }
  OK
}

// TEST(MarkingProgressTracker, ResetIfEnabledOnDisabled)
#[test]
fn reset_if_enabled_on_disabled() -> Void {
  let mut tracker = MarkingProgressTracker::new();
  tracker.reset_if_enabled();
  assert!(!tracker.is_enabled());
  OK
}

// TEST(MarkingProgressTracker, ResetIfEnabledOnEnabled)
#[test]
fn reset_if_enabled_on_enabled() -> Void {
  let mut tracker = MarkingProgressTracker::new();
  tracker.enable(K_OBJECT_SIZE);
  tracker.get_next_chunk_to_mark();
  tracker.reset_if_enabled();
  assert!(tracker.is_enabled());
  assert_eq!(tracker.get_next_chunk_to_mark(), 0);
  OK
}
