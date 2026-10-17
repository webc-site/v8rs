use std::{f64::consts::PI, mem};

use aok::{OK, Void};
use common::Arena;
use v8_objects_arrays::{
  elements_impl::{
    CopyObjectElementsArgs, K_COPY_TO_END_AND_INITIALIZE_TO_HOLE, copy_double_to_double_elements,
    copy_from_float16_slice_to_f32, copy_from_float16_slice_to_f64, copy_from_float16_to_f32,
    copy_from_float16_to_f64, copy_object_to_double_elements, copy_object_to_object_elements,
    copy_packed_smi_to_double_elements, copy_smi_to_double_elements, get_write_barrier_mode,
    is_float16_raw_bits_zero, is_identity_mapping, is_identity_mapping_slice, sort_indices,
    sort_indices_slice,
  },
  elements_kind::ElementsKind,
  fixed_array::{FixedArray, TaggedArrayExt},
  primitive_array::{FixedDoubleArray, FixedDoubleArrayExt},
};
use v8_objects_core::{
  heap_number::HeapNumber,
  heap_object::HeapObject,
  instance_type::InstanceType,
  objects::WriteBarrierMode,
  smi::Smi,
  tagged::{Object, Tagged},
};

mod common;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

fn alloc_fixed_array(arena: &mut Arena, len: u32) -> Tagged<FixedArray> {
  let map = arena.map(InstanceType::FixedArray);
  let tail = Tagged::<FixedArray>::size_for(len) - mem::size_of::<FixedArray>();
  arena.alloc_with(FixedArray::new(map, len), tail)
}

fn alloc_fixed_double_array(arena: &mut Arena, len: u32) -> Tagged<FixedDoubleArray> {
  let map = arena.map(InstanceType::FixedDoubleArray);
  let tail = FixedDoubleArray::size_for(len) - mem::size_of::<FixedDoubleArray>();
  arena.alloc_with(FixedDoubleArray::new(map, len), tail)
}

fn alloc_heap_number(arena: &mut Arena, val: f64) -> Tagged<HeapNumber> {
  let map = arena.map(InstanceType::HeapNumber);
  arena.alloc_with(HeapNumber::new(map, val), 0)
}

fn alloc_oddball(arena: &mut Arena) -> Tagged<Object> {
  let map = arena.map(InstanceType::Oddball);
  let obj: Tagged<HeapObject> = arena.alloc_with(HeapObject::new(map), 0);
  obj.cast()
}

#[test]
fn test_float16_raw_bits_zero() -> Void {
  // +0.0 (0x0000) 与 -0.0 (0x8000) 符号位掩码后均为 0
  assert!(is_float16_raw_bits_zero(0x0000));
  assert!(is_float16_raw_bits_zero(0x8000));

  // 非零值判定为 false
  assert!(!is_float16_raw_bits_zero(0x0001)); // subnormal
  assert!(!is_float16_raw_bits_zero(0x3c00)); // 1.0
  assert!(!is_float16_raw_bits_zero(0xbc00)); // -1.0
  assert!(!is_float16_raw_bits_zero(0x7c00)); // +inf
  assert!(!is_float16_raw_bits_zero(0xfc00)); // -inf
  assert!(!is_float16_raw_bits_zero(0x7e00)); // nan
  OK
}

#[test]
fn test_identity_mapping() -> Void {
  let mapping = [0usize, 1, 2, 3, 4];
  assert!(is_identity_mapping_slice(&mapping, 0));
  assert!(is_identity_mapping_slice(&mapping, 2));
  assert!(is_identity_mapping_slice(&mapping, 5));

  let non_identity = [0usize, 1, 99, 3, 4];
  assert!(!is_identity_mapping_slice(&non_identity, 0));
  assert!(is_identity_mapping_slice(&non_identity, 3));

  let kinds = [
    ElementsKind::PackedSmiElements,
    ElementsKind::HoleySmiElements,
    ElementsKind::PackedElements,
  ];
  assert!(is_identity_mapping(&kinds, 0));
  OK
}

#[test]
fn test_write_barrier_mode() -> Void {
  // Smi 与 Double 类型无需写屏障
  assert_eq!(
    get_write_barrier_mode(ElementsKind::PackedSmiElements),
    WriteBarrierMode::SkipWriteBarrier
  );
  assert_eq!(
    get_write_barrier_mode(ElementsKind::HoleySmiElements),
    WriteBarrierMode::SkipWriteBarrier
  );
  assert_eq!(
    get_write_barrier_mode(ElementsKind::PackedDoubleElements),
    WriteBarrierMode::SkipWriteBarrier
  );
  assert_eq!(
    get_write_barrier_mode(ElementsKind::HoleyDoubleElements),
    WriteBarrierMode::SkipWriteBarrier
  );

  // Object 类型需更新写屏障
  assert_eq!(
    get_write_barrier_mode(ElementsKind::PackedElements),
    WriteBarrierMode::UpdateWriteBarrier
  );
  assert_eq!(
    get_write_barrier_mode(ElementsKind::HoleyElements),
    WriteBarrierMode::UpdateWriteBarrier
  );
  assert_eq!(
    get_write_barrier_mode(ElementsKind::DictionaryElements),
    WriteBarrierMode::UpdateWriteBarrier
  );
  OK
}

#[test]
fn test_sort_indices_slice_and_array() -> Void {
  let mut arena = Arena::new();

  // 1. 切片排序测试：Smis 升序排列，Undefined 沉底
  let smi1 = Smi::from_int(1).cast::<Object>();
  let smi2 = Smi::from_int(2).cast::<Object>();
  let smi5 = Smi::from_int(5).cast::<Object>();
  // 使用有效合法对齐的堆对象 (Oddball) 作为非数值/Undefined 哨兵
  let undef = alloc_oddball(&mut arena);

  let mut slice = [smi5, undef, smi2, undef, smi1];
  sort_indices_slice(&mut slice);
  assert_eq!(slice[0], smi1);
  assert_eq!(slice[1], smi2);
  assert_eq!(slice[2], smi5);
  assert_eq!(slice[3], undef);
  assert_eq!(slice[4], undef);

  // 2. FixedArray 实例原位排序测试
  let fa = alloc_fixed_array(&mut arena, 5);
  unsafe {
    fa.set(
      0,
      Smi::from_int(40).cast(),
      WriteBarrierMode::SkipWriteBarrier,
    );
    fa.set(
      1,
      Smi::from_int(10).cast(),
      WriteBarrierMode::SkipWriteBarrier,
    );
    fa.set(
      2,
      Smi::from_int(50).cast(),
      WriteBarrierMode::SkipWriteBarrier,
    );
    fa.set(
      3,
      Smi::from_int(20).cast(),
      WriteBarrierMode::SkipWriteBarrier,
    );
    fa.set(
      4,
      Smi::from_int(30).cast(),
      WriteBarrierMode::SkipWriteBarrier,
    );

    sort_indices(fa, 5);

    assert_eq!(Smi::value(fa.get(0).cast()), 10);
    assert_eq!(Smi::value(fa.get(1).cast()), 20);
    assert_eq!(Smi::value(fa.get(2).cast()), 30);
    assert_eq!(Smi::value(fa.get(3).cast()), 40);
    assert_eq!(Smi::value(fa.get(4).cast()), 50);
  }

  // 3. 混合 Smi 与 HeapNumber 与 Undefined 排序测试
  let hn_1_5 = alloc_heap_number(&mut arena, 1.5).cast::<Object>();
  let hn_3_5 = alloc_heap_number(&mut arena, 3.5).cast::<Object>();
  let mut mixed_slice = [smi5, undef, hn_1_5, smi1, undef, hn_3_5];
  sort_indices_slice(&mut mixed_slice);
  assert_eq!(mixed_slice[0], smi1);
  assert_eq!(mixed_slice[1], hn_1_5);
  assert_eq!(mixed_slice[2], hn_3_5);
  assert_eq!(mixed_slice[3], smi5);
  assert_eq!(mixed_slice[4], undef);
  assert_eq!(mixed_slice[5], undef);
  OK
}

#[test]
fn test_copy_object_to_object() -> Void {
  let mut arena = Arena::new();
  let from = alloc_fixed_array(&mut arena, 4);
  let to = alloc_fixed_array(&mut arena, 6);
  let hole = Smi::from_int(-1).cast::<Object>();

  unsafe {
    for i in 0..4 {
      from.set(
        i,
        Smi::from_int(i as i32 * 10).cast(),
        WriteBarrierMode::SkipWriteBarrier,
      );
    }

    // 拷贝并填充剩余项为 hole
    let args = CopyObjectElementsArgs {
      from_kind: ElementsKind::PackedSmiElements,
      from_start: 1,
      to_kind: ElementsKind::HoleySmiElements,
      to_start: 1,
      raw_copy_size: K_COPY_TO_END_AND_INITIALIZE_TO_HOLE,
      hole_value: hole,
    };
    copy_object_to_object_elements(from, to, args);

    // 槽 1..4 应为 10, 20, 30
    assert_eq!(Smi::value(to.get(1).cast()), 10);
    assert_eq!(Smi::value(to.get(2).cast()), 20);
    assert_eq!(Smi::value(to.get(3).cast()), 30);
    // 剩余槽 4..6 应为 hole
    assert_eq!(to.get(4), hole);
    assert_eq!(to.get(5), hole);
  }
  OK
}

#[test]
fn test_copy_double_to_double() -> Void {
  let mut arena = Arena::new();
  let from = alloc_fixed_double_array(&mut arena, 4);
  let to = alloc_fixed_double_array(&mut arena, 6);

  unsafe {
    for i in 0..4 {
      from.set(i, (i as f64) * 1.5);
    }

    copy_double_to_double_elements(from, 1, to, 0, K_COPY_TO_END_AND_INITIALIZE_TO_HOLE);

    assert_eq!(to.get_scalar(0), 1.5);
    assert_eq!(to.get_scalar(1), 3.0);
    assert_eq!(to.get_scalar(2), 4.5);
    // 槽 3..6 为 hole
    for i in 3..6 {
      assert!(to.is_the_hole(i));
    }

    // 同数组内存重叠自搬迁 (向后搬迁)
    copy_double_to_double_elements(to, 0, to, 1, 2);
    assert_eq!(to.get_scalar(1), 1.5);
    assert_eq!(to.get_scalar(2), 3.0);

    // 同数组内存重叠自搬迁 (向前搬迁)
    copy_double_to_double_elements(to, 1, to, 0, 2);
    assert_eq!(to.get_scalar(0), 1.5);
    assert_eq!(to.get_scalar(1), 3.0);

    // copy_size == 0 安全早退
    copy_double_to_double_elements(to, 0, to, 0, 0);
  }
  OK
}

#[test]
fn test_copy_smi_and_packed_smi_to_double() -> Void {
  let mut arena = Arena::new();
  let from = alloc_fixed_array(&mut arena, 4);
  let to = alloc_fixed_double_array(&mut arena, 5);
  let hole = Smi::from_int(-999).cast::<Object>();

  unsafe {
    from.set(
      0,
      Smi::from_int(10).cast(),
      WriteBarrierMode::SkipWriteBarrier,
    );
    from.set(1, hole, WriteBarrierMode::SkipWriteBarrier);
    from.set(
      2,
      Smi::from_int(30).cast(),
      WriteBarrierMode::SkipWriteBarrier,
    );
    from.set(
      3,
      Smi::from_int(40).cast(),
      WriteBarrierMode::SkipWriteBarrier,
    );

    // 1. CopySmiToDoubleElements
    copy_smi_to_double_elements(from, 0, to, 0, 3, hole);
    assert_eq!(to.get_scalar(0), 10.0);
    assert!(to.is_the_hole(1));
    assert_eq!(to.get_scalar(2), 30.0);

    // 2. CopyPackedSmiToDoubleElements
    let to_packed = alloc_fixed_double_array(&mut arena, 4);
    copy_packed_smi_to_double_elements(from, 2, to_packed, 0, 4, 2);
    assert_eq!(to_packed.get_scalar(0), 30.0);
    assert_eq!(to_packed.get_scalar(1), 40.0);
  }
  OK
}

#[test]
fn test_copy_object_to_double() -> Void {
  let mut arena = Arena::new();
  let from = alloc_fixed_array(&mut arena, 3);
  let to = alloc_fixed_double_array(&mut arena, 4);
  let pi = PI;
  let hn = alloc_heap_number(&mut arena, pi);
  let hole = Smi::from_int(-1).cast::<Object>();

  unsafe {
    from.set(
      0,
      Smi::from_int(7).cast(),
      WriteBarrierMode::SkipWriteBarrier,
    );
    from.set(1, hn.cast(), WriteBarrierMode::SkipWriteBarrier);
    from.set(2, hole, WriteBarrierMode::SkipWriteBarrier);

    copy_object_to_double_elements(from, 0, to, 0, K_COPY_TO_END_AND_INITIALIZE_TO_HOLE, hole);

    assert_eq!(to.get_scalar(0), 7.0);
    assert!((to.get_scalar(1) - pi).abs() < 1e-10);
    assert!(to.is_the_hole(2));
    assert!(to.is_the_hole(3)); // 尾部初始化为 hole
  }
  OK
}

#[test]
fn test_copy_from_float16_backing_store() -> Void {
  let raw_fp16 = [0x0000u16, 0x3c00, 0xc000, 0x7bff];
  let mut f32_buf = [0.0f32; 4];
  let mut f64_buf = [0.0f64; 4];

  copy_from_float16_slice_to_f32(&raw_fp16, &mut f32_buf);
  assert_eq!(f32_buf[0], 0.0);
  assert_eq!(f32_buf[1], 1.0);
  assert_eq!(f32_buf[2], -2.0);
  assert_eq!(f32_buf[3], 65504.0);

  copy_from_float16_slice_to_f64(&raw_fp16, &mut f64_buf);
  assert_eq!(f64_buf[0], 0.0);
  assert_eq!(f64_buf[1], 1.0);
  assert_eq!(f64_buf[2], -2.0);
  assert_eq!(f64_buf[3], 65504.0);

  // 指针级调用与 0 长度安全验证
  unsafe {
    copy_from_float16_to_f32(raw_fp16.as_ptr(), f32_buf.as_mut_ptr(), 0);
    copy_from_float16_to_f32(raw_fp16.as_ptr(), f32_buf.as_mut_ptr(), 4);
    assert_eq!(f32_buf[1], 1.0);
    copy_from_float16_to_f64(raw_fp16.as_ptr(), f64_buf.as_mut_ptr(), 4);
    assert_eq!(f64_buf[1], 1.0);
  }
  OK
}

#[test]
fn test_copy_to_end_destination_clamping() -> Void {
  let mut arena = Arena::new();
  let from = alloc_fixed_array(&mut arena, 5);
  let to = alloc_fixed_double_array(&mut arena, 3);
  let hole = Smi::from_int(-1).cast::<Object>();

  unsafe {
    for i in 0..5 {
      from.set(
        i,
        Smi::from_int((i + 1) as i32 * 10).cast(),
        WriteBarrierMode::SkipWriteBarrier,
      );
    }

    // 1. CopySmiToDoubleElements 目标容量更小时应自动受限于 to_len (安全无越界)
    copy_smi_to_double_elements(from, 1, to, 0, K_COPY_TO_END_AND_INITIALIZE_TO_HOLE, hole);
    assert_eq!(to.get_scalar(0), 20.0);
    assert_eq!(to.get_scalar(1), 30.0);
    assert_eq!(to.get_scalar(2), 40.0);

    // 2. CopyPackedSmiToDoubleElements 同样安全约束
    let to_packed = alloc_fixed_double_array(&mut arena, 2);
    copy_packed_smi_to_double_elements(
      from,
      1,
      to_packed,
      0,
      5,
      K_COPY_TO_END_AND_INITIALIZE_TO_HOLE,
    );
    assert_eq!(to_packed.get_scalar(0), 20.0);
    assert_eq!(to_packed.get_scalar(1), 30.0);

    // 3. CopyObjectToDoubleElements 同样安全约束
    let to_obj = alloc_fixed_double_array(&mut arena, 2);
    copy_object_to_double_elements(
      from,
      2,
      to_obj,
      0,
      K_COPY_TO_END_AND_INITIALIZE_TO_HOLE,
      hole,
    );
    assert_eq!(to_obj.get_scalar(0), 30.0);
    assert_eq!(to_obj.get_scalar(1), 40.0);
  }
  OK
}
