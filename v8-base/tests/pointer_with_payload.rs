//! v8-base/src/pointer_with_payload.rs 集成测试 — 原版无独立 unittest
//! (pointer-with-payload.h 在 v8 由下游使用方覆盖), 按原版 header 语义写规约:
//! 掩码隔离 / Update/Set* 幂等 / GetPointerWithKnownPayload 免掩码路径 /
//! 对齐位断言。region_allocator LargestFit tie-break 回归一并归档本文件族
//! (独立于既有 suites, 防跨文件竞争)。

use aok::{OK, Void};
use v8_base::{
  pointer_with_payload::{PointerWithPayload, available_bits},
  region_allocator::{AllocationStrategy, RegionAllocator, RegionState},
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 掩码隔离规约: 指针与 payload 互不串扰
#[test]
fn pointer_payload_masks() -> Void {
  let mut backing = 0xAA_AAAA_u64;
  let ptr = &mut backing as *mut u64;
  let mut pp = PointerWithPayload::<3>::from_pointer_and_payload(ptr, 5);
  assert_eq!(pp.get_payload(), 5);
  assert_eq!(pp.get_pointer::<u64>(), ptr);
  pp.set_payload(7);
  assert_eq!(pp.get_payload(), 7);
  assert_eq!(pp.get_pointer::<u64>(), ptr);
  let other = 0xBB_BBBB_u64;
  pp.set_pointer(&other as *const u64 as *mut u64);
  assert_eq!(pp.get_payload(), 7);
  assert_eq!(pp.get_pointer::<u64>(), &other as *const u64 as *mut u64);
  assert_eq!(pp.raw(), (&other as *const u64 as usize) | 7);
  OK
}

/// 构造器形态与已知 payload 免掩码路径
#[test]
fn constructors_and_known_payload() -> Void {
  // from_payload: 空指针 + payload
  let pp = PointerWithPayload::<2>::from_payload(3);
  assert!(pp.get_pointer::<u8>().is_null());
  assert_eq!(pp.get_payload(), 3);
  // from_pointer: 零 payload (u64 对齐 8, 承载 2 payload 位)
  let mut backing = 1_u64;
  let backing_ptr = &mut backing as *mut u64;
  let pp = PointerWithPayload::<2>::from_pointer(backing_ptr);
  assert_eq!(pp.get_payload(), 0);
  assert_eq!(pp.get_pointer::<u64>(), backing_ptr);
  // GetPointerWithKnownPayload: 已知 payload 走减法
  let mut pp = PointerWithPayload::<3>::from_pointer_and_payload(backing_ptr, 6);
  assert_eq!(pp.get_pointer_with_known_payload::<u64>(6), backing_ptr);
  // update 覆盖
  let mut another = 2_u64;
  let another_ptr = &mut another as *mut u64;
  pp.update(another_ptr, 1);
  assert_eq!(pp.get_pointer::<u64>(), another_ptr);
  assert_eq!(pp.get_payload(), 1);
  // new/default 全零
  assert_eq!(PointerWithPayload::<3>::new().raw(), 0);
  assert_eq!(PointerWithPayload::<3>::default().raw(), 0);
  OK
}

/// available_bits 对齐推导 (原版 PointerWithPayloadTraits)
#[test]
fn traits_available_bits() -> Void {
  assert_eq!(available_bits::<u64>(), 3); // align 8
  assert_eq!(available_bits::<u32>(), 2); // align 4
  assert_eq!(available_bits::<u8>(), 1); // align 1
  assert_eq!(available_bits::<u16>(), 1); // align 2
  OK
}

/// LargestFit 同尺寸 tie-break 回归: 须取 begin 最小 (原版
/// region-allocator.cc:78-88 严格更大语义, 防本轮修复回归)
#[test]
fn region_largest_fit_tie_break_lowest_begin() -> Void {
  // 全区 [0x1000, 0xB000), 页 0x1000
  let mut ra = RegionAllocator::new(0x1000, 0xA000, 0x1000).unwrap();
  // 切三刀, 造出三块等尺寸 0x2000 空闲 + 若干更小块:
  //   分配 [0x3000,0x4000) → 空闲 [0x1000,0x3000) 0x2000 / [0x4000,0xB000) 0x7000
  assert!(ra.allocate_region_at(0x3000, 0x1000, RegionState::Allocated));
  //   分配 [0x6000,0x9000) → 空闲 0x2000/[0x4000,0x6000) 与 0x2000/[0x9000,0xB000)
  assert!(ra.allocate_region_at(0x6000, 0x3000, RegionState::Allocated));
  // 最大空闲 = 0x2000, 三块候选 begin ∈ {0x1000, 0x4000, 0x9000}
  // LargestFit 须取 begin 最小的 0x1000 (修复前 next_back() 会错取 0x9000)
  let got = ra.allocate_region_with_strategy(0x2000, AllocationStrategy::LargestFit);
  assert_eq!(got, Some(0x1000));
  OK
}
