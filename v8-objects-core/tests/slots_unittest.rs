//! 全宽槽族单测 (slots.{h,-inl.h} + slots-atomic-inl.h)

use aok::{OK, Void};
use v8_objects_core::{
  map::Map,
  slots::{
    AtomicSlot, FullHeapObjectSlot, FullMaybeObjectSlot, FullObjectSlot, K_SLOT_DATA_SIZE,
    SlotBase, copy_tagged, external_pointer_slot_has_handle, memset_pointer, memset_tagged,
    relaxed_memset_tagged,
  },
  tagged::{HeapObjectReference, MaybeObject, Object, Tagged},
};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

#[test]
fn test_full_slot_base_arithmetic() -> Void {
  let base = SlotBase::new(0x2000);
  assert_eq!(base.location(), 0x2000);
  assert_eq!(base.address, 0x2000);

  // step: ptr ± n * K_SLOT_DATA_SIZE (8 bytes)
  assert_eq!(base.step(1).location(), 0x2000 + K_SLOT_DATA_SIZE);
  assert_eq!(base.step(2).location(), 0x2000 + 2 * K_SLOT_DATA_SIZE);
  assert_eq!(base.step(-1).location(), 0x2000 - K_SLOT_DATA_SIZE);

  // distance: (ptr差) / K_SLOT_DATA_SIZE
  let other = SlotBase::new(0x2000);
  assert_eq!(base.step(4).distance(other), 4);
  assert_eq!(other.distance(base.step(4)), -4);

  OK
}

#[test]
fn test_full_object_slot_operations() -> Void {
  let mut storage = 0usize;
  let slot = FullObjectSlot::new(&mut storage as *mut usize as usize);
  assert_eq!(slot.location(), &storage as *const usize as usize);
  assert_eq!(slot.address(), &storage as *const usize as usize);
  const { assert!(!FullObjectSlot::K_CAN_BE_WEAK) };

  // step and distance
  let next = slot.step(1);
  assert_eq!(next.location(), slot.location() + K_SLOT_DATA_SIZE);
  assert_eq!(next.distance(slot), 1);
  assert_eq!(slot.distance(next), -1);

  // store & load
  let val = Tagged::<Object>::new(0x1234_5671);
  unsafe {
    slot.store(val);
    assert_eq!(slot.load(), val);
    assert_eq!(slot.relaxed_load(), val);
    assert_eq!(slot.acquire_load(), val);
    assert_eq!(slot.relaxed_load_raw(), 0x1234_5671);
  }

  // relaxed_store & release_store
  let val2 = Tagged::<Object>::new(0x8765_4321);
  unsafe {
    slot.relaxed_store(val2);
    assert_eq!(slot.load(), val2);
    slot.release_store(val);
    assert_eq!(slot.load(), val);
  }

  // CAS
  let val3 = Tagged::<Object>::new(0xaaaa_bbbb);
  unsafe {
    // 成功 CAS
    let old = slot.relaxed_compare_and_swap(val, val3);
    assert_eq!(old, val);
    assert_eq!(slot.load(), val3);

    // 失败 CAS
    let failed = slot.release_compare_and_swap(val, val2);
    assert_eq!(failed, val3);
    assert_eq!(slot.load(), val3);
  }

  // from_value
  let obj_tagged = Tagged::<Object>::new(0x9999);
  let slot_from_val = FullObjectSlot::from_value(&obj_tagged);
  assert_eq!(
    slot_from_val.location(),
    &obj_tagged as *const Tagged<Object> as usize
  );

  // raw_to_tagged
  let raw_tagged = FullObjectSlot::raw_to_tagged(0, 0x1111);
  assert_eq!(raw_tagged.ptr(), 0x1111);

  // map 相关
  let map_ptr = Tagged::<Map>::new(0x4001);
  unsafe {
    slot.store_map(map_ptr);
    assert_eq!(slot.load_map(), map_ptr);
    assert!(slot.contains_map_value(0x4001));
    assert!(slot.relaxed_contains_map_value(0x4001));
    assert!(!slot.contains_map_value(0x5001));
  }

  OK
}

#[test]
fn test_full_maybe_object_slot_operations() -> Void {
  let mut storage = 0usize;
  let slot = FullMaybeObjectSlot::new(&mut storage as *mut usize as usize);
  const { assert!(FullMaybeObjectSlot::K_CAN_BE_WEAK) };

  // step and distance
  let next = slot.step(2);
  assert_eq!(next.location(), slot.location() + 2 * K_SLOT_DATA_SIZE);
  assert_eq!(next.distance(slot), 2);
  assert_eq!(slot.distance(next), -2);

  let weak_val = Tagged::<MaybeObject>::new(0x2003); // 弱引用标志 0b11
  unsafe {
    slot.store(weak_val);
    assert_eq!(slot.load(), weak_val);
    assert_eq!(slot.relaxed_load(), weak_val);
    assert_eq!(slot.relaxed_load_raw(), 0x2003);

    let strong_val = Tagged::<MaybeObject>::new(0x2001);
    slot.relaxed_store(strong_val);
    assert_eq!(slot.load(), strong_val);

    let cas_res = slot.release_compare_and_swap(strong_val, weak_val);
    assert_eq!(cas_res, strong_val);
    assert_eq!(slot.load(), weak_val);
    let maybe_val = Tagged::<MaybeObject>::new(0x2001);
    let slot_from_val = FullMaybeObjectSlot::from_value(&maybe_val);
    assert_eq!(
      slot_from_val.location(),
      &maybe_val as *const Tagged<MaybeObject> as usize
    );
  }

  OK
}

#[test]
fn test_full_heap_object_slot_operations() -> Void {
  use v8_objects_core::{heap_object::HeapObject, slots::full_heap_object_slot_store_heap_object};

  let mut storage = 0usize;
  let slot = FullHeapObjectSlot::new(&mut storage as *mut usize as usize);
  const { assert!(FullHeapObjectSlot::K_CAN_BE_WEAK) };

  // step and distance
  let next = slot.step(1);
  assert_eq!(next.location(), slot.location() + K_SLOT_DATA_SIZE);
  assert_eq!(next.distance(slot), 1);
  assert_eq!(slot.distance(next), -1);

  let heap_ref = Tagged::<HeapObjectReference>::new(0x3001);
  unsafe {
    slot.store(heap_ref);
    assert_eq!(slot.load(), heap_ref);

    let heap_obj = slot.to_heap_object();
    assert_eq!(heap_obj.ptr(), 0x3001);
    assert!(heap_obj.is_heap_object());

    // store_heap_object
    let heap_target = Tagged::<HeapObject>::new(0x4001);
    slot.store_heap_object(heap_target);
    assert_eq!(slot.to_heap_object(), heap_target);

    // full_heap_object_slot_store_heap_object 自由函数
    let heap_target2 = Tagged::<HeapObject>::new(0x5001);
    full_heap_object_slot_store_heap_object(&slot, heap_target2);
    assert_eq!(slot.to_heap_object(), heap_target2);
  }

  OK
}

#[test]
fn test_slot_free_functions_and_atomic() -> Void {
  assert!(external_pointer_slot_has_handle());

  // AtomicSlot
  let mut atomic_storage = 42u32;
  let atomic_slot = AtomicSlot::new(&mut atomic_storage as *mut u32 as usize);
  unsafe {
    assert_eq!(atomic_slot.reference_load(), 42);
  }
  assert_eq!(atomic_slot.index(3), atomic_slot.address + 3 * 4);

  // copy_tagged
  let src = [10u8, 20, 30, 40, 50, 60, 70, 80];
  let mut dst = [0u8; 8];
  unsafe {
    copy_tagged(dst.as_mut_ptr() as usize, src.as_ptr() as usize, 2);
  }
  assert_eq!(dst, src);

  // memset_pointer
  let mut pointer_storage = [0usize; 4];
  let start_slot = FullObjectSlot::new(pointer_storage.as_mut_ptr() as usize);
  let fill_val = Tagged::<Object>::new(0xcafe_babe);
  unsafe {
    memset_pointer(start_slot, fill_val, 4);
  }
  assert_eq!(pointer_storage, [0xcafe_babe; 4]);

  // memset_tagged
  let mut tagged_storage = [0u32; 4];
  let maybe_obj = Tagged::<MaybeObject>::new(0x1001);
  unsafe {
    memset_tagged(tagged_storage.as_mut_ptr() as usize, maybe_obj, 4);
  }
  assert_eq!(tagged_storage, [0x1001; 4]);

  // relaxed_memset_tagged
  let mut relaxed_storage = [0u32; 4];
  unsafe {
    relaxed_memset_tagged(relaxed_storage.as_mut_ptr() as usize, maybe_obj, 4);
  }
  assert_eq!(relaxed_storage, [0x1001; 4]);

  OK
}
