//! Primitive & Trusted 数组族规约测试。
//!
//! 规约来源 (v8/src/objects):
//! - fixed-primitive-array.h / -inl.h (FixedDoubleArray, ByteArray, TrustedByteArray)
//! - trusted-object.h / -inl.h (TrustedObject, ExposedTrustedObject, TrustedPointerMember)
//! - fixed-array.h / -inl.h (ProtectedFixedArray, TrustedFixedArray)

pub mod common;

use std::{
  f64::consts::{E, PI},
  mem,
};

use aok::{OK, Void};
use common::Arena;
use v8_common::K_HOLE_NAN_INT64;
use v8_objects_arrays::{
  Error,
  fixed_array::{ProtectedFixedArray, TaggedArrayExt, TrustedObjectOrSmi},
  primitive_array::{
    ByteArray, ByteArrayExt, CANONICAL_QUIET_NAN_BITS, FixedDoubleArray, FixedDoubleArrayExt,
    TrustedByteArray, TrustedByteArrayExt,
  },
  trusted::{
    ExposedTrustedObject, TrustedObject, TrustedObjectExt, TrustedPointerMember,
    clear_trusted_pointer_field, is_trusted_pointer_field_empty,
    is_trusted_pointer_field_unpublished,
  },
};
use v8_objects_core::{
  heap_object::HeapObject,
  instance_type::InstanceType,
  objects::{WriteBarrierMode, is},
  smi::Smi,
  tagged::Tagged,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 竞技场分配 FixedDoubleArray
fn alloc_fixed_double_array(arena: &mut Arena, len: u32) -> Tagged<FixedDoubleArray> {
  let map = arena.map(InstanceType::FixedDoubleArray);
  let tail = FixedDoubleArray::size_for(len) - mem::size_of::<FixedDoubleArray>();
  arena.alloc_with(FixedDoubleArray::new(map, len), tail)
}

/// 竞技场分配 ByteArray
fn alloc_byte_array(arena: &mut Arena, len: u32) -> Tagged<ByteArray> {
  let map = arena.map(InstanceType::ByteArray);
  let tail = ByteArray::size_for(len) - mem::size_of::<ByteArray>();
  arena.alloc_with(ByteArray::new(map, len), tail)
}

/// 竞技场分配 TrustedByteArray
fn alloc_trusted_byte_array(arena: &mut Arena, len: u32) -> Tagged<TrustedByteArray> {
  let map = arena.map(InstanceType::TrustedByteArray);
  let tail = TrustedByteArray::size_for(len) - mem::size_of::<TrustedByteArray>();
  arena.alloc_with(TrustedByteArray::new(map, len), tail)
}

/// 竞技场分配 ProtectedFixedArray
fn alloc_protected_fixed_array(arena: &mut Arena, len: u32) -> Tagged<ProtectedFixedArray> {
  let map = arena.map(InstanceType::ProtectedFixedArray);
  let tail = Tagged::<ProtectedFixedArray>::size_for(len) - mem::size_of::<ProtectedFixedArray>();
  arena.alloc_with(ProtectedFixedArray::new(map, len), tail)
}

/// FixedDoubleArray 标量与二进制比特读写
#[test]
fn fixed_double_array_scalar_and_bits() -> Void {
  let mut arena = Arena::new();
  let arr = alloc_fixed_double_array(&mut arena, 4);

  assert_eq!(arr.length(), 4);
  assert!(arr.is_in_bounds(0));
  assert!(arr.is_in_bounds(3));
  assert!(!arr.is_in_bounds(4));

  // SAFETY: 测试环境使用独占分配的有效 arena 内存，索引处于有效容量内
  unsafe {
    arr.set(0, PI);
    assert_eq!(arr.get_scalar(0), PI);
    assert_eq!(arr.get_representation(0), PI.to_bits());

    arr.set_raw(1, 0x1234_5678_9ABC_DEF0);
    assert_eq!(arr.get_representation(1), 0x1234_5678_9ABC_DEF0);
  }

  // 边界检查安全方法
  assert!(arr.set_checked(2, E).is_ok());
  assert_eq!(arr.get_scalar_checked(2)?, E);

  assert_eq!(
    arr.get_scalar_checked(4).unwrap_err(),
    Error::OutOfBounds {
      index: 4,
      length: 4
    }
  );
  assert_eq!(
    arr.set_checked(10, 1.0).unwrap_err(),
    Error::OutOfBounds {
      index: 10,
      length: 4
    }
  );

  OK
}

/// FixedDoubleArray Quiet NaN 规整与 Hole 标记语义
#[test]
fn fixed_double_array_holes_and_nan() -> Void {
  let mut arena = Arena::new();
  let arr = alloc_fixed_double_array(&mut arena, 4);

  // SAFETY: 测试环境使用独占分配的有效 arena 内存，索引处于有效容量内
  unsafe {
    // 任意非标 NaN (如 signaling NaN) 规整为 canonical quiet NaN
    let signaling_nan = f64::from_bits(0x7FF0_0000_0000_0001);
    arr.set(0, signaling_nan);
    assert_eq!(arr.get_representation(0), CANONICAL_QUIET_NAN_BITS);

    // 标准 NAN
    arr.set(1, f64::NAN);
    assert_eq!(arr.get_representation(1), CANONICAL_QUIET_NAN_BITS);

    // Hole 标记测试
    assert!(!arr.is_the_hole(2));
    arr.set_the_hole(2);
    assert!(arr.is_the_hole(2));
    assert_eq!(arr.get_representation(2), K_HOLE_NAN_INT64);

    // 区间填充 Hole
    arr.fill_with_holes(0, 4);
    for i in 0..4 {
      assert!(arr.is_the_hole(i));
    }
  }

  // 安全读取拦截 Hole
  assert_eq!(
    arr.get_scalar_checked(0).unwrap_err(),
    Error::Invariant("访问了 Hole 标记元素")
  );

  OK
}

/// FixedDoubleArray 内存搬迁与尺寸布局
#[test]
fn fixed_double_array_move_elements_and_layout() -> Void {
  let mut arena = Arena::new();
  let arr = alloc_fixed_double_array(&mut arena, 5);

  // SAFETY: 测试环境使用独占分配的有效 arena 内存，索引处于有效容量内
  unsafe {
    arr.set(0, 10.0);
    arr.set(1, 20.0);
    arr.set(2, 30.0);
    arr.set(3, 40.0);
    arr.set(4, 50.0);

    // 内存重叠搬迁: [10, 20, 30] 搬迁到 [1, 2, 3] -> [10, 10, 20, 30, 50]
    arr.move_elements(1, 0, 3);
    assert_eq!(arr.get_scalar(0), 10.0);
    assert_eq!(arr.get_scalar(1), 10.0);
    assert_eq!(arr.get_scalar(2), 20.0);
    assert_eq!(arr.get_scalar(3), 30.0);
    assert_eq!(arr.get_scalar(4), 50.0);

    // len=0 空操作
    arr.move_elements(0, 0, 0);
  }

  // 尺寸计算
  assert_eq!(FixedDoubleArray::size_for(0), 16);
  assert_eq!(FixedDoubleArray::size_for(1), 24);
  assert_eq!(FixedDoubleArray::size_for(2), 32);
  assert!(FixedDoubleArray::max_regular_length() > 0);

  // Cast 检查
  let obj = arr.cast();
  assert!(is::<FixedDoubleArray>(obj));

  OK
}

/// ByteArray 裸字节与非对齐原生类型读写
#[test]
fn byte_array_raw_bytes_and_unaligned_ints() -> Void {
  let mut arena = Arena::new();
  let arr = alloc_byte_array(&mut arena, 16);

  assert_eq!(arr.length(), 16);

  // SAFETY: 测试环境使用独占分配的有效 arena 内存，偏移处于有效容量内
  unsafe {
    // 单字节读写
    arr.set_byte(0, 0xAA);
    arr.set_byte(15, 0x55);
    assert_eq!(arr.get_byte(0), 0xAA);
    assert_eq!(arr.get_byte(15), 0x55);

    // 非对齐 u16
    arr.set_u16(1, 0x1234);
    assert_eq!(arr.get_u16(1), 0x1234);
    // 小端字节验证
    assert_eq!(arr.get_byte(1), 0x34);
    assert_eq!(arr.get_byte(2), 0x12);

    // 非对齐 u32 (兼 get_int/set_int 别名)
    arr.set_u32(3, 0xA1B2C3D4);
    assert_eq!(arr.get_u32(3), 0xA1B2C3D4);
    assert_eq!(arr.get_int(3), 0xA1B2C3D4);
    assert_eq!(arr.get_byte(3), 0xD4);
    assert_eq!(arr.get_byte(4), 0xC3);
    assert_eq!(arr.get_byte(5), 0xB2);
    assert_eq!(arr.get_byte(6), 0xA1);

    arr.set_int(3, 0x01020304);
    assert_eq!(arr.get_u32(3), 0x01020304);

    // 非对齐 u64
    arr.set_u64(7, 0x1122334455667788);
    assert_eq!(arr.get_u64(7), 0x1122334455667788);
  }

  // 边界检查安全方法
  assert!(arr.set_byte_checked(0, 0xFE).is_ok());
  assert_eq!(arr.get_byte_checked(0)?, 0xFE);

  assert_eq!(
    arr.get_byte_checked(16).unwrap_err(),
    Error::OutOfBounds {
      index: 16,
      length: 16
    }
  );
  assert_eq!(
    arr.set_byte_checked(20, 0).unwrap_err(),
    Error::OutOfBounds {
      index: 20,
      length: 16
    }
  );

  OK
}

/// ByteArray 复制与尺寸布局
#[test]
fn byte_array_copy_and_layout() -> Void {
  let mut arena = Arena::new();
  let arr = alloc_byte_array(&mut arena, 8);

  // SAFETY: 测试环境使用独占分配的有效 arena 内存，偏移处于有效容量内
  unsafe {
    for i in 0..8 {
      arr.set_byte(i, i as u8);
    }

    // copy_bytes 搬迁 [0, 1, 2] 到 [3, 4, 5]
    arr.copy_bytes(3, 0, 3);
    assert_eq!(arr.get_byte(3), 0);
    assert_eq!(arr.get_byte(4), 1);
    assert_eq!(arr.get_byte(5), 2);

    // len=0 空操作
    arr.copy_bytes(0, 0, 0);
  }

  // 尺寸计算
  assert_eq!(ByteArray::size_for(0), 16);
  assert_eq!(ByteArray::size_for(1), 24); // 16 + 1 = 17 -> 8 对齐到 24
  assert_eq!(ByteArray::size_for(8), 24); // 16 + 8 = 24
  assert_eq!(ByteArray::size_for(9), 32); // 16 + 9 = 25 -> 8 对齐到 32
  assert_eq!(ByteArray::length_for(24), 8);
  assert_eq!(ByteArray::length_for(8), 0);
  assert!(ByteArray::max_regular_length() > 0);

  // Cast 检查
  let obj = arr.cast();
  assert!(is::<ByteArray>(obj));

  OK
}

/// TrustedByteArray 语义验证
#[test]
fn trusted_byte_array_semantics() -> Void {
  let mut arena = Arena::new();
  let arr = alloc_trusted_byte_array(&mut arena, 12);

  assert_eq!(arr.length(), 12);

  // SAFETY: 测试环境使用独占分配的有效 arena 内存，偏移处于有效容量内
  unsafe {
    arr.set_byte(0, 0x7F);
    arr.set_byte(11, 0x80);
    assert_eq!(arr.get_byte(0), 0x7F);
    assert_eq!(arr.get_byte(11), 0x80);

    arr.set_u16(1, 0x1234);
    assert_eq!(arr.get_u16(1), 0x1234);

    arr.set_u32(2, 0xCAFEBABE);
    assert_eq!(arr.get_u32(2), 0xCAFEBABE);

    arr.set_u64(4, 0x0123_4567_89AB_CDEF);
    assert_eq!(arr.get_u64(4), 0x0123_4567_89AB_CDEF);

    // 复制测试
    arr.copy_bytes(8, 0, 4);
    assert_eq!(arr.get_byte(8), 0x7F);
  }

  assert!(arr.set_byte_checked(0, 0x11).is_ok());
  assert_eq!(arr.get_byte_checked(0)?, 0x11);
  assert_eq!(
    arr.get_byte_checked(12).unwrap_err(),
    Error::OutOfBounds {
      index: 12,
      length: 12
    }
  );

  // 尺寸计算
  assert_eq!(TrustedByteArray::size_for(0), 16);
  assert_eq!(TrustedByteArray::size_for(4), 24);
  assert_eq!(TrustedByteArray::length_for(24), 8);
  assert_eq!(TrustedByteArray::length_for(15), 0);
  assert!(TrustedByteArray::max_regular_length() > 0);

  // Cast 检查
  let obj = arr.cast();
  assert!(is::<TrustedByteArray>(obj));

  OK
}

/// ProtectedFixedArray 语义验证
#[test]
fn protected_fixed_array_semantics() -> Void {
  let mut arena = Arena::new();
  let arr = alloc_protected_fixed_array(&mut arena, 4);

  // SAFETY: 测试环境使用独占分配的有效 arena 内存，索引处于有效容量内
  unsafe {
    assert_eq!(arr.length(), 4);
    assert_eq!(arr.capacity(), 4);

    let val1 = Smi::from_int(123).cast::<TrustedObjectOrSmi>();
    let val2 = Smi::from_int(456).cast::<TrustedObjectOrSmi>();

    arr.set(0, val1, WriteBarrierMode::UpdateWriteBarrier);
    arr.set_relaxed(1, val2, WriteBarrierMode::UpdateWriteBarrier);

    assert_eq!(arr.get(0), val1);
    assert_eq!(arr.get_relaxed(1), val2);
  }

  // 边界检查
  let val = Smi::from_int(789).cast::<TrustedObjectOrSmi>();
  assert!(arr.set_checked(2, val).is_ok());
  assert_eq!(arr.get_checked(2)?, val);

  assert_eq!(
    arr.get_checked(4).unwrap_err(),
    Error::OutOfBounds {
      index: 4,
      length: 4
    }
  );

  // Cast 检查
  let obj = arr.cast();
  assert!(is::<ProtectedFixedArray>(obj));

  OK
}

/// TrustedObject, ExposedTrustedObject, TrustedPointerMember 及受保护指针槽
#[test]
fn trusted_object_and_pointer_member() -> Void {
  let mut arena = Arena::new();

  // TrustedObject / ExposedTrustedObject 构造与类型断言
  let t_map = arena.map(InstanceType::ProtectedFixedArray);
  let trusted_header = TrustedObject::new(t_map);
  assert_eq!(trusted_header.header.map(), t_map);

  let exp_map = arena.map(InstanceType::BytecodeArray);
  let exp_header = ExposedTrustedObject::new(exp_map);
  assert_eq!(exp_header.trusted.header.map(), exp_map);

  // 分配带受保护指针槽的受信对象 (头 8 字节 + 2 个 tagged 指针槽 16 字节)
  let host = arena.alloc_with(TrustedObject::new(t_map), mem::size_of::<usize>() * 2);
  let target = arena.alloc_with(TrustedObject::new(t_map), 0);

  let field_offset = mem::size_of::<TrustedObject>();
  // SAFETY: 测试环境使用独占分配的有效 arena 内存，偏移处于有效容量内
  unsafe {
    // 初始槽为空 (0 = Smi::zero)
    assert!(host.is_protected_pointer_field_empty(field_offset));
    assert!(host.is_protected_pointer_field_empty_acquire(field_offset));

    // 写入目标受信对象
    host.write_protected_pointer_field(field_offset, target);
    assert!(!host.is_protected_pointer_field_empty(field_offset));
    let read_back: Tagged<TrustedObject> = host.read_protected_pointer_field(field_offset);
    assert_eq!(read_back, target);

    // Release 写入与 Acquire 读取
    host.write_protected_pointer_field_release(field_offset, target);
    let read_back_acq: Tagged<TrustedObject> =
      host.read_protected_pointer_field_acquire(field_offset);
    assert_eq!(read_back_acq, target);

    // 清空受保护槽位
    host.clear_protected_pointer_field(field_offset);
    assert!(host.is_protected_pointer_field_empty(field_offset));

    // free functions
    let host_heap = host.as_heap_object();
    assert!(is_trusted_pointer_field_empty(host_heap, field_offset));
    assert!(!is_trusted_pointer_field_unpublished());
    clear_trusted_pointer_field(host_heap, field_offset);
  }

  // TrustedPointerMember 成员操作
  let member: TrustedPointerMember<TrustedObject> = TrustedPointerMember::new();
  assert!(member.is_empty());
  assert!(!member.is_unpublished());

  let host_heap = HeapObject::new(t_map);
  member.store(&host_heap, target, WriteBarrierMode::SkipWriteBarrier);
  assert!(!member.is_empty());
  assert_eq!(member.load(), target);
  assert_eq!(member.acquire_load(), target);

  member.clear(&host_heap);
  assert!(member.is_empty());

  OK
}

/// Primitive 数组切片视图与长度内存序语义
#[test]
fn primitive_array_slice_and_orderings() -> Void {
  let mut arena = Arena::new();

  // FixedDoubleArray 切片视图与内存序
  let mut darr = alloc_fixed_double_array(&mut arena, 3);
  // SAFETY: 测试环境使用独占分配的有效 arena 内存，索引处于有效容量内
  unsafe {
    darr.set(0, 1.5);
    darr.set(1, 2.5);
    darr.set(2, 3.5);

    let slice = darr.as_f64_slice();
    assert_eq!(slice, [1.5, 2.5, 3.5]);

    let slice_mut = darr.as_f64_slice_mut();
    slice_mut[1] = 4.5;
    assert_eq!(darr.get_scalar(1), 4.5);

    assert_eq!(darr.length_relaxed(), 3);
    assert_eq!(darr.length_acquire(), 3);
    assert_eq!(darr.allocated_size(), FixedDoubleArray::size_for(3));

    darr.set_length(2);
    assert_eq!(darr.length(), 2);
    darr.set_length_release(3);
    assert_eq!(darr.length_acquire(), 3);
  }

  // ByteArray 零拷贝切片与内存序
  let mut barr = alloc_byte_array(&mut arena, 4);
  // SAFETY: 测试环境使用独占分配的有效 arena 内存，偏移处于有效容量内
  unsafe {
    barr.set_byte(0, 0x10);
    barr.set_byte(1, 0x20);
    barr.set_byte(2, 0x30);
    barr.set_byte(3, 0x40);

    assert_eq!(barr.raw_bytes(), &[0x10, 0x20, 0x30, 0x40]);
    barr.raw_bytes_mut()[2] = 0x99;
    assert_eq!(barr.get_byte(2), 0x99);

    assert_eq!(barr.length_relaxed(), 4);
    assert_eq!(barr.length_acquire(), 4);
    assert_eq!(barr.allocated_size(), ByteArray::size_for(4));

    barr.set_length(2);
    assert_eq!(barr.length(), 2);
    barr.set_length_release(4);
    assert_eq!(barr.length_acquire(), 4);
  }

  // TrustedByteArray 零拷贝切片与内存序
  let mut tbarr = alloc_trusted_byte_array(&mut arena, 4);
  // SAFETY: 测试环境使用独占分配的有效 arena 内存，偏移处于有效容量内
  unsafe {
    tbarr.set_byte(0, 0xAA);
    tbarr.set_byte(1, 0xBB);
    assert_eq!(tbarr.raw_bytes()[..2], [0xAA, 0xBB]);
    tbarr.raw_bytes_mut()[0] = 0xCC;
    assert_eq!(tbarr.get_byte(0), 0xCC);

    assert_eq!(tbarr.length_relaxed(), 4);
    assert_eq!(tbarr.length_acquire(), 4);
    assert_eq!(tbarr.allocated_size(), TrustedByteArray::size_for(4));

    tbarr.set_length(1);
    assert_eq!(tbarr.length(), 1);
    tbarr.set_length_release(4);
    assert_eq!(tbarr.length_acquire(), 4);
  }

  OK
}
