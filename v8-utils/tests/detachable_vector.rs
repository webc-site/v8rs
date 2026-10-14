// 迁移自 v8/test/unittests/utils/detachable-vector-unittest.cc
// 断言规约: TEST(DetachableVector, X) → fn x; size 断言以 usize 直比。
// DetachLeaksBackingStore 上游依赖 ASAN 检泄漏; Rust 版断言所有权移交语义 (等价强度)。
use aok::{OK, Void};
use v8_utils::detachable_vector::{DetachableVector, MINIMUM_CAPACITY};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// ConstructIsEmpty
#[test]
fn construct_is_empty() -> Void {
  let v: DetachableVector<i32> = DetachableVector::new();
  assert_eq!(0, v.size());
  assert!(v.is_empty());
  OK
}

// PushAddsElement
#[test]
fn push_adds_element() -> Void {
  let mut v: DetachableVector<i32> = DetachableVector::new();
  v.push_back(1);
  assert_eq!(&1, v.front());
  assert_eq!(&1, v.back());
  assert_eq!(&1, v.at(0));
  assert_eq!(1, v.size());
  assert!(!v.is_empty());
  OK
}

// AfterFreeIsEmpty
#[test]
fn after_free_is_empty() -> Void {
  let mut v: DetachableVector<i32> = DetachableVector::new();
  v.push_back(1);
  v.free();
  assert_eq!(0, v.size());
  assert!(v.is_empty());
  OK
}

// DetachLeaksBackingStore: 上游 memcpy 位拷贝转移 + detach; Rust 版 detach_into 移交
#[test]
fn detach_leaks_backing_store() -> Void {
  let mut v: DetachableVector<i32> = DetachableVector::new();
  let mut v2: DetachableVector<i32> = DetachableVector::new();

  assert!(v2.is_empty());

  // Force allocation of the backing store.
  v.push_back(1);
  // 上游 memcpy(&v2, &v, ...) 位拷贝接管; Rust 版等价物为 detach_into 显式移交。
  let store = v.detach_into();
  assert_eq!(1, store.len());

  // We have transferred the backing store to the second vector.
  // store 析构时释放 (上游靠 v2 析构)。
  assert_eq!(0, v.size());
  assert!(v.is_empty());
  let _ = &mut v2;
  OK
}

// PushAndPopWithReallocation
#[test]
fn push_and_pop_with_reallocation() -> Void {
  let mut v: DetachableVector<usize> = DetachableVector::new();

  assert_eq!(0, v.capacity());
  assert_eq!(0, v.size());
  v.push_back(0);
  assert_eq!(MINIMUM_CAPACITY, v.capacity());
  assert_eq!(1, v.size());

  // Push values until the reallocation happens.
  for i in 1..=MINIMUM_CAPACITY {
    v.push_back(i);
  }
  assert_eq!(2 * MINIMUM_CAPACITY, v.capacity());
  assert_eq!(MINIMUM_CAPACITY + 1, v.size());

  assert_eq!(&MINIMUM_CAPACITY, v.back());
  v.pop_back();

  v.push_back(100);
  assert_eq!(&100, v.back());
  v.pop_back();
  assert_eq!(&(MINIMUM_CAPACITY - 1), v.back());
  OK
}

// ShrinkToFit
#[test]
fn shrink_to_fit() -> Void {
  let mut v: DetachableVector<usize> = DetachableVector::new();

  // shrink_to_fit doesn't affect the empty capacity DetachableVector.
  assert_eq!(0, v.capacity());
  v.shrink_to_fit();
  assert_eq!(0, v.capacity());

  // Do not shrink the buffer if it's smaller than kMinimumCapacity.
  v.push_back(0);
  assert_eq!(MINIMUM_CAPACITY, v.capacity());
  v.shrink_to_fit();
  assert_eq!(MINIMUM_CAPACITY, v.capacity());

  // Fill items to |v| until the buffer grows twice.
  for i in 0..2 * MINIMUM_CAPACITY {
    v.push_back(i);
  }
  assert_eq!(2 * MINIMUM_CAPACITY + 1, v.size());
  assert_eq!(4 * MINIMUM_CAPACITY, v.capacity());

  // Do not shrink the buffer if the number of unused slots is not large enough.
  v.shrink_to_fit();
  assert_eq!(2 * MINIMUM_CAPACITY + 1, v.size());
  assert_eq!(4 * MINIMUM_CAPACITY, v.capacity());

  v.pop_back();
  v.pop_back();
  v.shrink_to_fit();
  assert_eq!(2 * MINIMUM_CAPACITY - 1, v.size());
  assert_eq!(2 * MINIMUM_CAPACITY - 1, v.capacity());
  OK
}
