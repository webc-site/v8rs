//! active-system-pages-unittest.cc 1:1 迁移。
use aok::{OK, Void};
use v8_heap_base::ActiveSystemPages;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// TEST(ActiveSystemPagesTest, Add) cc:12-22
#[test]
fn add() -> Void {
  let mut pages = ActiveSystemPages::new();
  const K_PAGE_SIZE_BITS: usize = 0;
  assert_eq!(pages.add(0, 1, K_PAGE_SIZE_BITS), 1);
  assert_eq!(pages.add(1, 2, K_PAGE_SIZE_BITS), 1);
  assert_eq!(pages.add(63, 64, K_PAGE_SIZE_BITS), 1);
  assert_eq!(pages.size(K_PAGE_SIZE_BITS), 3);

  // 二次加同页:
  assert_eq!(pages.add(0, 2, K_PAGE_SIZE_BITS), 0);
  OK
}

// TEST(ActiveSystemPagesTest, AddUnalignedRange) cc:24-36
#[test]
fn add_unaligned_range() -> Void {
  let mut pages = ActiveSystemPages::new();
  const K_PAGE_SIZE_BITS: usize = 12;
  const K_PAGE_SIZE: usize = 1 << K_PAGE_SIZE_BITS;
  const K_WORD_SIZE: usize = 8;
  assert_eq!(pages.add(0, K_PAGE_SIZE + K_WORD_SIZE, K_PAGE_SIZE_BITS), 2);
  assert_eq!(
    pages.add(
      3 * K_PAGE_SIZE - K_WORD_SIZE,
      3 * K_PAGE_SIZE,
      K_PAGE_SIZE_BITS
    ),
    1
  );
  assert_eq!(
    pages.add(
      K_PAGE_SIZE + K_WORD_SIZE,
      3 * K_PAGE_SIZE - K_WORD_SIZE,
      K_PAGE_SIZE_BITS
    ),
    0
  );
  assert_eq!(pages.size(K_PAGE_SIZE_BITS), 3 * K_PAGE_SIZE);
  OK
}

// TEST(ActiveSystemPagesTest, AddFullBitset) cc:38-43
#[test]
fn add_full_bitset() -> Void {
  let mut pages = ActiveSystemPages::new();
  const K_PAGE_SIZE_BITS: usize = 0;
  assert_eq!(pages.add(0, 64, K_PAGE_SIZE_BITS), 64);
  assert_eq!(pages.add(0, 64, K_PAGE_SIZE_BITS), 0);
  assert_eq!(pages.size(K_PAGE_SIZE_BITS), 64);
  OK
}

// TEST(ActiveSystemPagesTest, Reduce) cc:46-55
#[test]
fn reduce() -> Void {
  let mut original = ActiveSystemPages::new();
  const K_PAGE_SIZE_BITS: usize = 0;
  assert_eq!(original.add(0, 3, K_PAGE_SIZE_BITS), 3);

  let mut updated = ActiveSystemPages::new();
  assert_eq!(updated.add(1, 3, K_PAGE_SIZE_BITS), 2);

  assert_eq!(original.reduce(updated), 1);
  OK
}

// TEST(ActiveSystemPagesTest, ReduceFullBitset) cc:57-66
#[test]
fn reduce_full_bitset() -> Void {
  let mut original = ActiveSystemPages::new();
  const K_PAGE_SIZE_BITS: usize = 0;
  assert_eq!(original.add(0, 64, K_PAGE_SIZE_BITS), 64);

  let mut updated = ActiveSystemPages::new();
  assert_eq!(updated.add(63, 64, K_PAGE_SIZE_BITS), 1);

  assert_eq!(original.reduce(updated), 63);
  OK
}

// TEST(ActiveSystemPagesTest, Clear) cc:68-78
#[test]
fn clear() -> Void {
  let mut pages = ActiveSystemPages::new();
  const K_PAGE_SIZE_BITS: usize = 0;
  assert_eq!(pages.add(0, 64, K_PAGE_SIZE_BITS), 64);
  assert_eq!(pages.clear(), 64);
  assert_eq!(pages.size(K_PAGE_SIZE_BITS), 0);

  assert_eq!(pages.add(0, 2, K_PAGE_SIZE_BITS), 2);
  assert_eq!(pages.clear(), 2);
  assert_eq!(pages.size(K_PAGE_SIZE_BITS), 0);
  OK
}

// API 语义补充: Init 清空并置入 header 覆盖页
#[test]
fn init() -> Void {
  let mut pages = ActiveSystemPages::new();
  const K_PAGE_SIZE_BITS: usize = 12;
  const K_PAGE_SIZE: usize = 1 << K_PAGE_SIZE_BITS;
  // 先置脏位
  pages.add(0, 64 * K_PAGE_SIZE, K_PAGE_SIZE_BITS);
  assert_eq!(pages.size(K_PAGE_SIZE_BITS), 64 * K_PAGE_SIZE);

  // 初始化 header 为 2 个系统页
  let added = pages.init(2 * K_PAGE_SIZE, K_PAGE_SIZE_BITS, 16 * K_PAGE_SIZE);
  assert_eq!(added, 2);
  assert_eq!(pages.size(K_PAGE_SIZE_BITS), 2 * K_PAGE_SIZE);
  OK
}
