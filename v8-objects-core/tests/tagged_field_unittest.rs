//! 单元测试: TaggedMember / TaggedField / UnalignedMember (原版 tagged-field.h + tagged-field-inl.h)

use std::{
  mem::{align_of, size_of},
  ptr::copy_nonoverlapping,
};

use aok::{OK, Void};
use v8_objects_core::{
  heap_object::HeapObject,
  map::Map,
  map_word::MapWord,
  objects::WriteBarrierMode,
  smi::Smi,
  tagged::{Object, Tagged},
  tagged_field::{TaggedField, TaggedMember, UnalignedDoubleMember, UnalignedValueMember},
};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

/// 宿主缓冲: 偏移 0/8/16 三个 tagged 槽位
#[repr(C)]
struct Host {
  slot0: usize,
  slot1: usize,
  slot2: usize,
}

/// 构造指向栈上宿主缓冲的 Tagged<HeapObject> (写路径指针派生自 &mut)
fn host(buf: &mut Host) -> Tagged<HeapObject> {
  Tagged::from_ptr(buf as *mut Host as *const HeapObject)
}

#[test]
fn test_tagged_member_layout() -> Void {
  // 原版 static_assert: alignof/sizeof(TaggedMember) == alignof/sizeof(Tagged_t)
  assert_eq!(align_of::<TaggedMember<Object>>(), align_of::<usize>());
  assert_eq!(size_of::<TaggedMember<Object>>(), size_of::<usize>());

  // 原版 static_assert: alignof(UnalignedDoubleMember) == alignof(Tagged_t), sizeof == 8
  assert_eq!(align_of::<UnalignedDoubleMember>(), align_of::<usize>());
  assert_eq!(size_of::<UnalignedDoubleMember>(), size_of::<f64>());

  OK
}

#[test]
fn test_tagged_member_load_store_orderings() -> Void {
  let v0: Tagged<Object> = Smi::from_int(0).cast();
  let v1: Tagged<Object> = Smi::from_int(1).cast();
  let v2: Tagged<Object> = Smi::from_int(2).cast();
  let v3: Tagged<Object> = Smi::from_int(3).cast();

  let m: TaggedMember<Object> = TaggedMember::new(v0);
  assert_eq!(m.load(), v0);
  assert_eq!(m.relaxed_load(), v0);
  assert_eq!(m.acquire_load(), v0);
  assert_eq!(m.seqcst_load(), v0);

  m.relaxed_store(v1, WriteBarrierMode::UpdateWriteBarrier);
  assert_eq!(m.load(), v1);

  m.release_store(v2, WriteBarrierMode::UpdateWriteBarrier);
  assert_eq!(m.load(), v2);

  m.seqcst_store(v3, WriteBarrierMode::SkipWriteBarrier);
  assert_eq!(m.seqcst_load(), v3);

  // 无屏障写族
  m.store_no_write_barrier(v0);
  assert_eq!(m.load(), v0);
  m.relaxed_store_no_write_barrier(v1);
  assert_eq!(m.load(), v1);
  m.release_store_no_write_barrier(v2);
  assert_eq!(m.load(), v2);
  m.seqcst_store_no_write_barrier(v3);
  assert_eq!(m.load(), v3);

  // Clone 语义: 读取当前值重建槽位
  let cloned = m.clone();
  assert_eq!(cloned.load(), v3);

  // Default: 零槽位
  assert_eq!(TaggedMember::<Object>::default().load().ptr(), 0);

  OK
}

#[test]
fn test_tagged_member_swap_and_cas() -> Void {
  let v0: Tagged<Object> = Smi::from_int(0).cast();
  let v1: Tagged<Object> = Smi::from_int(1).cast();
  let v2: Tagged<Object> = Smi::from_int(2).cast();

  // SeqCst 交换返回旧值
  let m: TaggedMember<Object> = TaggedMember::new(v0);
  let old = m.seqcst_swap(v1, WriteBarrierMode::UpdateWriteBarrier);
  assert_eq!(old, v0);
  assert_eq!(m.load(), v1);

  // CAS 命中: 返回旧值 (== expected)
  let old = m.seqcst_compare_and_swap(v1, v2, WriteBarrierMode::UpdateWriteBarrier);
  assert_eq!(old, v1);
  assert_eq!(m.load(), v2);

  // CAS 未命中: 返回槽位当前值
  let old = m.seqcst_compare_and_swap(v0, v1, WriteBarrierMode::UpdateWriteBarrier);
  assert_eq!(old, v2);
  assert_eq!(m.load(), v2);

  OK
}

#[test]
fn test_tagged_field_address_and_location() -> Void {
  let mut buf = Host {
    slot0: 0,
    slot1: 0,
    slot2: 0,
  };
  let h = host(&mut buf);

  // address = host.address() + OFFSET + offset
  assert_eq!(TaggedField::<Object>::address(h, 0), h.address());
  assert_eq!(TaggedField::<Object, 8>::address(h, 0), h.address() + 8);
  assert_eq!(TaggedField::<Object, 8>::address(h, 4), h.address() + 12);

  // 槽位定位到对应字段
  // SAFETY: h 指向有效栈上结构, 偏移落在对象范围内且 8 字节对齐
  let p1 = unsafe { TaggedField::<Object, 8>::location(h, 0) };
  assert_eq!(p1 as usize, &buf.slot1 as *const usize as usize);

  OK
}

#[test]
fn test_tagged_field_load_store() -> Void {
  let mut buf = Host {
    slot0: 0,
    slot1: 0,
    slot2: 0,
  };
  let h = host(&mut buf);
  let v1: Tagged<Object> = Smi::from_int(11).cast();
  let v2: Tagged<Object> = Smi::from_int(22).cast();

  // SAFETY: h 指向有效栈上结构, 偏移 8/16 均在范围内, 独占访问
  unsafe {
    TaggedField::<Object, 8>::store(h, v1);
    assert_eq!(buf.slot1, v1.ptr());

    TaggedField::<Object, 16>::store_at(h, 0, v2);
    assert_eq!(buf.slot2, v2.ptr());

    assert_eq!(TaggedField::<Object, 8>::load(h, 0), v1);
    assert_eq!(TaggedField::<Object, 16>::load(h, 0), v2);

    // 原子序族
    TaggedField::<Object, 8>::relaxed_store(h, v2);
    assert_eq!(TaggedField::<Object, 8>::relaxed_load(h, 0), v2);
    assert_eq!(TaggedField::<Object, 8>::acquire_load(h, 0), v2);
    assert_eq!(TaggedField::<Object, 8>::seqcst_load(h, 0), v2);

    TaggedField::<Object, 8>::release_store(h, v1);
    assert_eq!(TaggedField::<Object, 8>::relaxed_load(h, 0), v1);

    TaggedField::<Object, 8>::seqcst_store(h, v2);
    assert_eq!(TaggedField::<Object, 8>::seqcst_load(h, 0), v2);

    TaggedField::<Object, 8>::relaxed_store_at(h, 8, v1);
    assert_eq!(TaggedField::<Object, 8>::load(h, 8), v1);

    TaggedField::<Object, 8>::release_store_at(h, 0, v2);
    assert_eq!(TaggedField::<Object, 8>::load(h, 0), v2);

    TaggedField::<Object, 8>::seqcst_store_at(h, 0, v1);
    assert_eq!(TaggedField::<Object, 8>::load(h, 0), v1);

    // 交换/CAS
    let old = TaggedField::<Object, 8>::seqcst_swap(h, 0, v2);
    assert_eq!(old, v1);
    assert_eq!(TaggedField::<Object, 8>::load(h, 0), v2);

    let cur = TaggedField::<Object, 8>::release_compare_and_swap(h, v2.ptr(), v1);
    assert_eq!(cur, v2.ptr());
    assert_eq!(TaggedField::<Object, 8>::load(h, 0), v1);

    // 未命中返回当前值
    let cur = TaggedField::<Object, 8>::relaxed_compare_and_swap(h, v2.ptr(), v1);
    assert_eq!(cur, v1.ptr());
    assert_eq!(TaggedField::<Object, 8>::load(h, 0), v1);

    let old = TaggedField::<Object, 8>::seqcst_compare_and_swap(h, 0, v1, v2);
    assert_eq!(old, v1);
    assert_eq!(TaggedField::<Object, 8>::load(h, 0), v2);
  }

  OK
}

#[test]
fn test_tagged_field_map_word_ops() -> Void {
  let mut buf = Host {
    slot0: 0,
    slot1: 0,
    slot2: 0,
  };
  let h = host(&mut buf);
  let map = Tagged::<Map>::new(0x2001);

  // SAFETY: h 指向有效栈上结构, 槽位 0 在范围内
  unsafe {
    TaggedField::<MapWord>::relaxed_store_map_word(h, MapWord::from_map(map));
    assert_eq!(
      MapWord::from_map(map),
      TaggedField::<MapWord>::relaxed_load_map_word(h)
    );

    let fwd = MapWord::from_forwarding_address(h, h);
    TaggedField::<MapWord>::release_store_map_word(h, fwd);
    assert_eq!(fwd, TaggedField::<MapWord>::relaxed_load_map_word(h));
    assert!(TaggedField::<MapWord>::relaxed_load_map_word(h).is_forwarding_address());
  }

  OK
}

#[test]
fn test_unaligned_members() -> Void {
  // UnalignedValueMember: 任意 T 的非对齐读写
  let mut m32: UnalignedValueMember<u32> = UnalignedValueMember::new(0xDEADBEEF);
  assert_eq!(m32.value(), 0xDEADBEEF);
  m32.set_value(42);
  assert_eq!(m32.value(), 42);

  let m64: UnalignedValueMember<u64> = UnalignedValueMember::new(u64::MAX);
  assert_eq!(m64.value(), u64::MAX);

  // 位级观察: 把成员放进奇数偏移缓冲, 验证确实逐字节落位
  let mut raw = [0u8; 24];
  let m = UnalignedDoubleMember::new(1.5);
  // SAFETY: raw 长度 24, 偏移 1 起 8 字节在范围内
  unsafe {
    copy_nonoverlapping(
      &m as *const UnalignedDoubleMember as *const u8,
      raw.as_mut_ptr().add(1),
      8,
    );
  }
  // SAFETY: 偏移 1 + 8 字节在 raw 范围内
  let read_back = unsafe { raw.as_ptr().add(1).cast::<u64>().read_unaligned() };
  assert_eq!(read_back, 1.5f64.to_bits());

  // UnalignedDoubleMember: f64 与位模式互转, NaN 位保留
  let mut d = UnalignedDoubleMember::new(2.5);
  assert_eq!(d.value(), 2.5);
  d.set_value(-7.25);
  assert_eq!(d.value(), -7.25);
  d.set_value_as_bits(f64::NAN.to_bits() | 1);
  assert!(d.value().is_nan());
  assert_eq!(d.value_as_bits(), f64::NAN.to_bits() | 1);
  assert_eq!(UnalignedDoubleMember::default().value(), 0.0);

  OK
}
