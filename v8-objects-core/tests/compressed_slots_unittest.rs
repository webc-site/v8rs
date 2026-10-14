//! 压缩槽基类 + MaybeObject 清除值纯逻辑面 (compressed-slots-inl.h / maybe-object-inl.h)
//! 钉死测试态: 主 cage base = 0 (shared-cage), 解压为裸值零扩展。

use aok::{OK, Void};
use v8_common::PtrComprCageBase;
use v8_objects_core::{
  compressed_slots::{CompressedObjectSlot, SlotBase},
  maybe_object::{cleared_trusted_value, cleared_value},
  tagged::{Object, Tagged},
};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

#[test]
fn test_slot_base_arithmetic() -> Void {
  // 原版 operator++/--: ptr ± kSlotDataSize (=4)
  let base = SlotBase::new(0x100);
  assert_eq!(base.location(), 0x100);
  assert_eq!(base.step(1).location(), 0x104);
  assert_eq!(base.step(-1).location(), 0xfc);

  // 原版 operator-(a, b): 元素距离 = (ptr 差) / kSlotDataSize
  let other = SlotBase::new(0x100);
  assert_eq!(base.step(3).distance(other), 3);
  assert_eq!(other.distance(base.step(3)), -3);

  OK
}

#[test]
fn test_compressed_object_slot_construction() -> Void {
  // new/location: 槽地址原样承载 (构造不解引用, 安全)
  let slot = CompressedObjectSlot::new(0x2000);
  assert_eq!(slot.location(), 0x2000);
  assert_eq!(slot.address(), 0x2000);

  // from_value: 取 Tagged 字所在裸地址 (原版 CompressedObjectSlot(Tagged<Object>*))
  let value = Tagged::<Object>::new(0x1001);
  let slot = CompressedObjectSlot::from_value(&value);
  assert_eq!(slot.location(), &value as *const _ as usize);

  // RawToTagged(cage_base, raw): base|raw; 测试态 base=0 → 裸值零扩展
  let tagged = CompressedObjectSlot::raw_to_tagged(PtrComprCageBase::new(0), 0x33);
  assert_eq!(tagged.ptr(), 0x33);

  OK
}

#[test]
fn test_cleared_weak_values() -> Void {
  // ClearedValue / ClearedTrustedValue: decompress(kClearedWeakHeapObjectLower32)
  // 测试态双 cage base=0 → 值即低 32 位哨兵 3。
  assert_eq!(cleared_value().ptr(), 3);
  assert_eq!(cleared_trusted_value().ptr(), 3);

  OK
}

#[test]
fn test_compressed_object_slot_operations() -> Void {
  use v8_objects_core::{map::Map, tagged::Tagged};

  let mut storage = 0u32;
  let slot = CompressedObjectSlot::new(&mut storage as *mut u32 as usize);
  assert_eq!(slot.location(), &storage as *const u32 as usize);
  assert_eq!(slot.address(), &storage as *const u32 as usize);
  const { assert!(!CompressedObjectSlot::K_CAN_BE_WEAK) };

  // step and distance
  let next = slot.step(1);
  assert_eq!(next.location(), slot.location() + 4);
  assert_eq!(next.distance(slot), 1);
  assert_eq!(slot.distance(next), -1);

  // store & load
  let val = Tagged::<Object>::new(0x1001);
  unsafe {
    slot.store(val);
    assert_eq!(slot.load(), val);
    assert_eq!(slot.relaxed_load(), val);
    assert_eq!(slot.acquire_load(), val);
    assert_eq!(slot.relaxed_load_raw(), 0x1001);
  }

  // relaxed_store & release_store
  let val2 = Tagged::<Object>::new(0x2001);
  unsafe {
    slot.relaxed_store(val2);
    assert_eq!(slot.load(), val2);
    slot.release_store(val);
    assert_eq!(slot.load(), val);
  }

  // CAS: 成功与失败（验证失败时返回槽中实际值）
  let val3 = Tagged::<Object>::new(0x3001);
  unsafe {
    let old = slot.release_compare_and_swap(val, val3);
    assert_eq!(old, val);
    assert_eq!(slot.load(), val3);

    // 失败 CAS: 预期 val (0x1001) 但槽中为 val3 (0x3001)，应返回实际值 val3
    let failed = slot.release_compare_and_swap(val, val2);
    assert_eq!(failed, val3);
    assert_eq!(slot.load(), val3);
  }

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
fn test_compressed_maybe_object_slot_operations() -> Void {
  use v8_objects_core::{
    compressed_slots::CompressedMaybeObjectSlot,
    tagged::{MaybeObject, Tagged},
  };

  let mut storage = 0u32;
  let slot = CompressedMaybeObjectSlot::new(&mut storage as *mut u32 as usize);
  const { assert!(CompressedMaybeObjectSlot::K_CAN_BE_WEAK) };

  // step and distance
  let next = slot.step(2);
  assert_eq!(next.location(), slot.location() + 8);
  assert_eq!(next.distance(slot), 2);
  assert_eq!(slot.distance(next), -2);

  // from_value / from_object_value
  let maybe_val = Tagged::<MaybeObject>::new(0x1003);
  let slot_from_val = CompressedMaybeObjectSlot::from_value(&maybe_val);
  assert_eq!(slot_from_val.location(), &maybe_val as *const _ as usize);

  let obj_val = Tagged::<Object>::new(0x2001);
  let slot_from_obj = CompressedMaybeObjectSlot::from_object_value(&obj_val);
  assert_eq!(slot_from_obj.location(), &obj_val as *const _ as usize);

  // store & load
  unsafe {
    slot.store(maybe_val);
    assert_eq!(slot.load(), maybe_val);
    assert_eq!(slot.relaxed_load(), maybe_val);
    assert_eq!(slot.relaxed_load_raw(), 0x1003);

    let strong_val = Tagged::<MaybeObject>::new(0x2001);
    slot.relaxed_store(strong_val);
    assert_eq!(slot.load(), strong_val);

    // CAS u32 raw: 成功与失败
    let old = slot.release_compare_and_swap(0x2001, 0x3001);
    assert_eq!(old, 0x2001);
    assert_eq!(slot.relaxed_load_raw(), 0x3001);

    // 失败 CAS: 预期 0x9999 但槽中为 0x3001，应返回实际值 0x3001
    let failed = slot.release_compare_and_swap(0x9999, 0x5555);
    assert_eq!(failed, 0x3001);
    assert_eq!(slot.relaxed_load_raw(), 0x3001);

    // CAS tagged
    let cur = Tagged::<MaybeObject>::new(0x3001);
    let target = Tagged::<MaybeObject>::new(0x4001);
    let res = slot.release_compare_and_swap_tagged(cur, target);
    assert_eq!(res, cur);
    assert_eq!(slot.load(), target);
  }

  OK
}

#[test]
fn test_compressed_heap_object_slot_operations() -> Void {
  use v8_objects_core::{
    compressed_slots::CompressedHeapObjectSlot,
    heap_object::HeapObject,
    tagged::{HeapObjectReference, Tagged},
  };

  let mut storage = 0u32;
  let slot = CompressedHeapObjectSlot::new(&mut storage as *mut u32 as usize);
  assert_eq!(slot.location(), &storage as *const u32 as usize);
  assert_eq!(slot.address(), &storage as *const u32 as usize);
  const { assert!(CompressedHeapObjectSlot::K_CAN_BE_WEAK) };

  // step and distance
  let next = slot.step(1);
  assert_eq!(next.location(), slot.location() + 4);
  assert_eq!(next.distance(slot), 1);

  // from_value / from_heap_object
  let heap_ref = Tagged::<HeapObjectReference>::new(0x3001);
  let slot_from_ref = CompressedHeapObjectSlot::from_value(&heap_ref);
  assert_eq!(slot_from_ref.location(), &heap_ref as *const _ as usize);

  let heap_obj = Tagged::<HeapObject>::new(0x4001);
  let slot_from_obj = CompressedHeapObjectSlot::from_heap_object(&heap_obj);
  assert_eq!(slot_from_obj.location(), &heap_obj as *const _ as usize);

  // store & load
  unsafe {
    slot.store(heap_ref);
    assert_eq!(slot.load(), heap_ref);
    assert_eq!(slot.to_heap_object().ptr(), 0x3001);

    slot.store_heap_object(heap_obj);
    assert_eq!(slot.to_heap_object(), heap_obj);
  }

  OK
}
