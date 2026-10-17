//! HeapObject 字段槽地址 (heap-object-field-inl.h RawField / heap-object-set-map-inl.h map_slot)
//! 纯地址运算面: field_address = 对象基址 + offset, 不涉及堆内存读写。

use aok::{OK, Void};
use v8_common::AllocationType;
use v8_objects_core::{heap_object::HeapObject, map::Map, tagged::Tagged};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

#[test]
fn test_field_address_and_map_slot() -> Void {
  let ho = HeapObject::new(Tagged::<Map>::new(0x1001));
  let base = &ho as *const HeapObject as usize;

  // map_slot = MapField::address = 基址 + 0
  assert_eq!(ho.map_slot(), base);
  // RawField(offset) = field_address(offset) = 基址 + offset
  assert_eq!(ho.raw_field(0), base);
  assert_eq!(ho.raw_field(4), base + 4);
  // 各槽构造族同址语义 (tag_range 为元信息, 不影响地址)
  assert_eq!(ho.raw_maybe_weak_field(8), base + 8);
  assert_eq!(ho.raw_instruction_stream_field(8), base + 8);
  assert_eq!(ho.raw_external_pointer_field(8, 0), base + 8);
  assert_eq!(ho.raw_cpp_heap_pointer_field(8), base + 8);
  assert_eq!(ho.raw_indirect_pointer_field(8, 0), base + 8);

  OK
}

#[test]
fn test_sandboxed_pointer_and_bounded_size() -> Void {
  // 64 字节对齐的堆对象模拟缓冲区
  #[repr(align(8))]
  struct TestBuffer([u8; 64]);
  let mut buf = TestBuffer([0u8; 64]);
  let ho = unsafe { &mut *(buf.0.as_mut_ptr() as *mut HeapObject) };

  // 1. SandboxedPointer 测试
  let cage_base = 0x0000_1000_0000_0000usize;
  let target_ptr = cage_base + 0x1234_5600usize;
  ho.write_sandboxed_pointer_field(8, cage_base, target_ptr);
  let read_back = ho.read_sandboxed_pointer_field(8, cage_base);
  assert_eq!(read_back, target_ptr);

  // 2. BoundedSize 测试 (<= 32GB)
  let test_size = 1024 * 1024 * 64; // 64MB
  ho.write_bounded_size_field(16, test_size);
  let read_size = ho.read_bounded_size_field(16);
  assert_eq!(read_size, test_size);

  OK
}

#[test]
fn test_atomic_field_accessors() -> Void {
  #[repr(align(8))]
  struct TestBuffer([u8; 64]);
  let mut buf = TestBuffer([0u8; 64]);
  let ho = unsafe { &mut *(buf.0.as_mut_ptr() as *mut HeapObject) };

  // 1. Relaxed u32
  ho.relaxed_write_field(8, 0x1234_5678u32);
  assert_eq!(ho.relaxed_read_field::<u32>(8), 0x1234_5678u32);

  // 2. Acquire/Release u64
  ho.release_write_field(16, 0xfeed_beef_cafe_babe_u64);
  assert_eq!(ho.acquire_read_field::<u64>(16), 0xfeed_beef_cafe_babe_u64);

  // 3. CAS
  let cas_res = ho.seq_cst_compare_and_swap_field(8, 0x1234_5678u32, 0x9999_aaaa_u32);
  assert_eq!(cas_res, Ok(0x1234_5678u32));
  assert_eq!(ho.relaxed_read_field::<u32>(8), 0x9999_aaaa_u32);

  // CAS 失败
  let cas_fail = ho.seq_cst_compare_and_swap_field(8, 0x1111_2222u32, 0x0u32);
  assert_eq!(cas_fail, Err(0x9999_aaaa_u32));

  OK
}

#[test]
fn test_set_map_family_and_cas() -> Void {
  use v8_objects_core::heap_object_set_map_inl::{
    AllocationWitness, EmitWriteBarrier, SKIP_WRITE_BARRIER, StoreOrder, UPDATE_WRITE_BARRIER,
  };

  #[repr(align(8))]
  struct TestBuffer([u8; 64]);
  let mut buf = TestBuffer([0u8; 64]);
  let ho = unsafe { &mut *(buf.0.as_mut_ptr() as *mut HeapObject) };

  let map1 = Tagged::<Map>::new(0x2001);
  let map2 = Tagged::<Map>::new(0x3001);

  // 1. set_map_word Relaxed
  ho.set_map_word(map1, StoreOrder::Relaxed);
  assert_eq!(ho.map, map1);

  // 2. set_map_release
  ho.set_map_release(map2, StoreOrder::Release);
  assert_eq!(ho.map, map2);

  // 3. set_map_after_allocation
  ho.set_map_after_allocation(map1, EmitWriteBarrier::No);
  assert_eq!(ho.map, map1);

  // 4. AllocationWitness 屏障推导
  assert_eq!(
    AllocationWitness::write_barrier_mode_for_allocation(0, AllocationType::ReadOnly as u8),
    SKIP_WRITE_BARRIER
  );
  assert_eq!(
    AllocationWitness::write_barrier_mode_for_allocation(0, AllocationType::Young as u8),
    SKIP_WRITE_BARRIER
  );
  assert_eq!(
    AllocationWitness::write_barrier_mode_for_allocation(0, AllocationType::Old as u8),
    UPDATE_WRITE_BARRIER
  );

  OK
}
