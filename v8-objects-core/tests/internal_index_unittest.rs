//! InternalIndex 单元测试 (internal-index.h)

use aok::{OK, Void};
use v8_objects_core::internal_index::InternalIndex;

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

#[test]
fn test_internal_index_basic() -> Void {
  let idx = InternalIndex::new(5);
  assert!(idx.is_found());
  assert!(!idx.is_not_found());
  assert_eq!(idx.raw_value(), 5);
  assert_eq!(idx.as_uint32(), 5);
  assert_eq!(idx.as_int(), 5);

  let not_found = InternalIndex::not_found();
  assert!(not_found.is_not_found());
  assert!(!not_found.is_found());
  assert_eq!(not_found.raw_value(), InternalIndex::NOT_FOUND);

  // adjust_up & adjust_down
  assert_eq!(idx.adjust_up(3).raw_value(), 8);
  assert_eq!(idx.adjust_down(2).raw_value(), 3);

  // range
  let r = InternalIndex::range(0, 10);
  assert_eq!(r.start, 0);
  assert_eq!(r.end, 10);
  assert_eq!(r.count(), 10);

  OK
}
