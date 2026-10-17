// 1:1 移植自 v8/test/unittests/codegen/aligned-slot-allocator-unittest.cc (W11CG)
// 原版 fixture AlignedSlotAllocatorUnitTest 的两个 Helper 以自由函数承载。

use aok::{OK, Void};
use log::info;
use v8_codegen_core::{AlignedSlotAllocator, SLOT_SIZE};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版 fixture::Allocate(size, expected): NextSlot/Allocate 一致、按 size 对齐、Size 越过槽尾
fn check_allocate(allocator: &mut AlignedSlotAllocator, size: i32, expected: i32) {
  let next = allocator.next_slot(size);
  let result = allocator.allocate(size);
  assert_eq!(next, result, "NextSlot/Allocate are consistent.");
  assert_eq!(expected, result);
  assert_eq!(0, result & (size - 1), "result 按 size 对齐");
  // 原版 std::bit_ceil<uint32_t>(size)
  let slot_end = result + (size as u32).next_power_of_two() as i32;
  assert!(slot_end <= allocator.size, "allocator Size 越过槽尾");
}

/// 原版 fixture::AllocateUnaligned(size, expected, expected1, expected2, expected4)
fn check_allocate_unaligned(
  allocator: &mut AlignedSlotAllocator,
  size: i32,
  expected: i32,
  expected1: i32,
  expected2: i32,
  expected4: i32,
) {
  let size_before = allocator.size;
  let result = allocator.allocate_unaligned(size);
  assert_eq!(
    size_before, result,
    "AllocateUnaligned/Size are consistent."
  );
  assert_eq!(expected, result);
  assert_eq!(result + size, allocator.size);
  assert_eq!(expected1, allocator.next_slot(1));
  assert_eq!(expected2, allocator.next_slot(2));
  assert_eq!(expected4, allocator.next_slot(4));
}

#[test]
fn test_num_slots_for_width() -> Void {
  info!("测试 NumSlotsForWidth: 1..=4*kSlotSize 逐字节换算槽数");
  let k_slot_bytes = SLOT_SIZE;
  for slot_size in 1..=(4 * k_slot_bytes) {
    assert_eq!(
      AlignedSlotAllocator::num_slots_for_width(slot_size),
      (slot_size + k_slot_bytes - 1) / k_slot_bytes
    );
  }
  OK
}

#[test]
fn test_allocate1() -> Void {
  info!("测试 Allocate(1) 贪心片段复用");
  let mut allocator = AlignedSlotAllocator::default();
  check_allocate(&mut allocator, 1, 0);
  assert_eq!(2, allocator.next_slot(2));
  assert_eq!(4, allocator.next_slot(4));

  check_allocate(&mut allocator, 1, 1);
  assert_eq!(2, allocator.next_slot(2));
  assert_eq!(4, allocator.next_slot(4));

  check_allocate(&mut allocator, 1, 2);
  assert_eq!(4, allocator.next_slot(2));
  assert_eq!(4, allocator.next_slot(4));

  check_allocate(&mut allocator, 1, 3);
  assert_eq!(4, allocator.next_slot(2));
  assert_eq!(4, allocator.next_slot(4));

  // Make sure we use 1-fragments.
  check_allocate(&mut allocator, 1, 4);
  check_allocate(&mut allocator, 2, 6);
  check_allocate(&mut allocator, 1, 5);

  // Make sure we use 2-fragments.
  check_allocate(&mut allocator, 2, 8);
  check_allocate(&mut allocator, 1, 10);
  check_allocate(&mut allocator, 1, 11);
  OK
}

#[test]
fn test_allocate2() -> Void {
  info!("测试 Allocate(2) 二槽片段复用");
  let mut allocator = AlignedSlotAllocator::default();
  check_allocate(&mut allocator, 2, 0);
  assert_eq!(2, allocator.next_slot(1));
  assert_eq!(4, allocator.next_slot(4));

  check_allocate(&mut allocator, 2, 2);
  assert_eq!(4, allocator.next_slot(1));
  assert_eq!(4, allocator.next_slot(4));

  // Make sure we use 2-fragments.
  check_allocate(&mut allocator, 1, 4);
  check_allocate(&mut allocator, 2, 6);
  check_allocate(&mut allocator, 2, 8);
  OK
}

#[test]
fn test_allocate4() -> Void {
  info!("测试 Allocate(4) 四槽组分配");
  let mut allocator = AlignedSlotAllocator::default();
  check_allocate(&mut allocator, 4, 0);
  assert_eq!(4, allocator.next_slot(1));
  assert_eq!(4, allocator.next_slot(2));

  check_allocate(&mut allocator, 1, 4);
  check_allocate(&mut allocator, 4, 8);

  check_allocate(&mut allocator, 2, 6);
  check_allocate(&mut allocator, 4, 12);
  OK
}

#[test]
fn test_allocate_unaligned() -> Void {
  info!("测试 AllocateUnaligned 尾部保留与片段复位");
  let mut allocator = AlignedSlotAllocator::default();
  check_allocate_unaligned(&mut allocator, 1, 0, 1, 2, 4);
  check_allocate_unaligned(&mut allocator, 1, 1, 2, 2, 4);

  check_allocate(&mut allocator, 1, 2);

  check_allocate_unaligned(&mut allocator, 2, 3, 5, 6, 8);

  // Advance to leave 1- and 2- fragments below Size.
  check_allocate(&mut allocator, 4, 8);

  // AllocateUnaligned should allocate at the end, and clear fragments.
  check_allocate_unaligned(&mut allocator, 0, 12, 12, 12, 12);
  OK
}

#[test]
fn test_large_allocate_unaligned() -> Void {
  info!("测试 LargeAllocateUnaligned 跨组尾部保留");
  let mut allocator = AlignedSlotAllocator::default();
  check_allocate_unaligned(&mut allocator, 11, 0, 11, 12, 12);
  check_allocate_unaligned(&mut allocator, 11, 11, 22, 22, 24);
  check_allocate_unaligned(&mut allocator, 13, 22, 35, 36, 36);
  OK
}

#[test]
fn test_size() -> Void {
  info!("测试 Size: 分配后槽区高水位推进与片段消耗");
  let mut allocator = AlignedSlotAllocator::default();
  allocator.allocate(1);
  assert_eq!(1, allocator.size);
  // Allocate 2, leaving a fragment at 1. Size should be at 4.
  allocator.allocate(2);
  assert_eq!(4, allocator.size);
  // Allocate should consume fragment.
  assert_eq!(1, allocator.allocate(1));
  // Size should still be 4.
  assert_eq!(4, allocator.size);
  OK
}

#[test]
fn test_align() -> Void {
  info!("测试 Align 对齐补片与片段复位");
  let mut allocator = AlignedSlotAllocator::default();
  assert_eq!(0, allocator.align(1));
  assert_eq!(0, allocator.size);

  // Allocate 1 to become misaligned.
  check_allocate(&mut allocator, 1, 0);

  // 4-align.
  assert_eq!(3, allocator.align(4));
  assert_eq!(4, allocator.next_slot(1));
  assert_eq!(4, allocator.next_slot(2));
  assert_eq!(4, allocator.next_slot(4));
  assert_eq!(4, allocator.size);

  // Allocate 2 to become misaligned.
  check_allocate(&mut allocator, 2, 4);

  // 4-align.
  assert_eq!(2, allocator.align(4));
  assert_eq!(8, allocator.next_slot(1));
  assert_eq!(8, allocator.next_slot(2));
  assert_eq!(8, allocator.next_slot(4));
  assert_eq!(8, allocator.size);

  // No change when we're already aligned.
  assert_eq!(0, allocator.align(2));
  assert_eq!(8, allocator.next_slot(1));
  assert_eq!(8, allocator.next_slot(2));
  assert_eq!(8, allocator.next_slot(4));
  assert_eq!(8, allocator.size);
  OK
}
