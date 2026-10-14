//! Struct 族规约测试。
//!
//! 规约来源 (实测搜索 v8/test/unittests/objects/ 无 struct-unittest.cc):
//! - object-unittest.cc:85 TEST(Object, StructListOrder) — STRUCT_LIST 判序,
//!   本族按子类判别值 (FIRST/LAST_STRUCT_TYPE 与四子类单型值) 迁移锚点部分;
//! - 其余 struct.h/-inl.h 语义 (布局/槽读写/分派谓词) 以原版 file:line 锚定兜底
//!   (票单登记); access-check-unittest.cc 的 Tuple2/AccessorPair 用法属 API 层
//!   (Isolate/Handle), 归 L3 不迁。

pub mod common;

use std::mem;

use aok::{OK, Void};
use common::Arena;
use v8_objects_arrays::{
  fixed_array::FixedArray,
  struct_::{
    AccessorPair, AccessorPairExt, ClassPositions, ClassPositionsExt, ForInEnumeratorHolder,
    ForInEnumeratorHolderExt, K_CACHE_LENGTH_OFFSET, K_ELEMENTS_LENGTH_OFFSET,
    K_ENUM_CACHE_MAP_OFFSET, K_NAMED_KEYS_OFFSET, Struct, Tuple2, Tuple2Ext,
  },
};
use v8_objects_core::{
  heap_object::HeapObject,
  instance_type::{FIRST_STRUCT_TYPE, InstanceType, LAST_STRUCT_TYPE},
  objects::{AccessorComponent, WriteBarrierMode, is, try_cast},
  smi::Smi,
  tagged::{Object, Tagged},
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 结构体逻辑布局锚点 (struct.h:32 static_assert + struct.h.js 槽偏移推导):
/// Struct 头 == HeapObject 头; 子类字段自 8 起按 Tagged 槽 8/16/24/32。
#[test]
fn struct_layout_anchors() -> Void {
  // Struct 标记基类不加字段 (struct.h:32)
  assert_eq!(mem::size_of::<Struct>(), mem::size_of::<HeapObject>());
  // Tuple2/AccessorPair/ClassPositions: 头 8 + 2 槽 = 24, 槽偏移 8/16
  assert_eq!(mem::size_of::<Tuple2>(), 24);
  assert_eq!(mem::offset_of!(Tuple2, value1), 8);
  assert_eq!(mem::offset_of!(Tuple2, value2), 16);
  assert_eq!(mem::size_of::<AccessorPair>(), 24);
  assert_eq!(mem::offset_of!(AccessorPair, getter), 8);
  assert_eq!(mem::offset_of!(AccessorPair, setter), 16);
  assert_eq!(mem::size_of::<ClassPositions>(), 24);
  assert_eq!(mem::offset_of!(ClassPositions, start), 8);
  assert_eq!(mem::offset_of!(ClassPositions, end), 16);
  // ForInEnumeratorHolder: 头 8 + 4 槽 = 40, ObjectTraits 偏移 8/16/24/32 (struct.h:179-189)
  assert_eq!(mem::size_of::<ForInEnumeratorHolder>(), 40);
  assert_eq!(K_ENUM_CACHE_MAP_OFFSET, 8);
  assert_eq!(K_NAMED_KEYS_OFFSET, 16);
  assert_eq!(K_ELEMENTS_LENGTH_OFFSET, 24);
  assert_eq!(K_CACHE_LENGTH_OFFSET, 32);
  OK
}

/// StructListOrder 判别值锚点 (object-unittest.cc:85-98 的本族部分):
/// 四子类单型值落于 FIRST_STRUCT_TYPE..=LAST_STRUCT_TYPE 且严格升序。
#[test]
fn struct_subclass_discriminator_order() -> Void {
  let disc = [
    InstanceType::AccessorPair.raw(),
    InstanceType::ClassPositions.raw(),
    InstanceType::ForInEnumeratorHolder.raw(),
    InstanceType::Tuple2.raw(),
  ];
  assert_eq!(disc, [141, 151, 157, 173]);
  for w in disc.windows(2) {
    assert!(w[0] < w[1], "STRUCT_LIST 判序: {w:?}");
  }
  let first = InstanceType::from_raw(FIRST_STRUCT_TYPE).is_ok();
  let last = InstanceType::from_raw(LAST_STRUCT_TYPE).is_ok();
  assert!(first && last);
  for d in disc {
    assert!(d >= InstanceType::AccessorPair.raw() && d <= InstanceType::Tuple2.raw());
  }
  OK
}

/// Tuple2 槽读写 (struct-inl.h:37-66): 构造初值 + load/relaxed/acquire 读序
/// 与 set/relaxed/release 写序往返。
#[test]
fn tuple2_slot_accessors() -> Void {
  let mut arena = Arena::new();
  let map = arena.map(InstanceType::Tuple2);
  let v1 = Smi::from_int(11).cast::<Object>();
  let v2 = Smi::from_int(22).cast::<Object>();
  let t = arena.alloc_with(Tuple2::new(map, v1, v2), 0);

  // SAFETY: 竞技场对象有效且无并发别名
  unsafe {
    assert_eq!(t.value1(), v1);
    assert_eq!(t.value1_relaxed(), v1);
    assert_eq!(t.value1_acquire(), v1);
    assert_eq!(t.value2(), v2);
    assert_eq!(t.value2_relaxed(), v2);

    let v3 = Smi::from_int(33).cast::<Object>();
    t.set_value1(v3, WriteBarrierMode::UpdateWriteBarrier);
    assert_eq!(t.value1(), v3);
    let v4 = Smi::from_int(44).cast::<Object>();
    t.set_value1_relaxed(v4, WriteBarrierMode::SkipWriteBarrier);
    assert_eq!(t.value1_acquire(), v4);
    t.set_value1_release(v1, WriteBarrierMode::UpdateWriteBarrier);
    assert_eq!(t.value1(), v1);

    t.set_value2(v3, WriteBarrierMode::UpdateWriteBarrier);
    assert_eq!(t.value2(), v3);
    t.set_value2_relaxed(v2, WriteBarrierMode::SkipWriteBarrier);
    assert_eq!(t.value2_relaxed(), v2);
  }
  OK
}

/// AccessorPair 语义 (struct-inl.h:68-128): null_value 初值 (Tagged::NULL 哨兵
/// 承接) → get/set 按组件分派 → SetComponents 跳 null → Equals 双等判定。
#[test]
fn accessor_pair_components() -> Void {
  let mut arena = Arena::new();
  let map = arena.map(InstanceType::AccessorPair);
  let pair = arena.alloc_with(AccessorPair::new(map), 0);

  let getter = Smi::from_int(1).cast::<Object>();
  let setter = Smi::from_int(2).cast::<Object>();
  let null = Tagged::NULL;

  // SAFETY: 竞技场对象有效
  unsafe {
    // 初值 = null_value 哨兵
    assert!(pair.getter().is_null());
    assert!(pair.setter().is_null());

    pair.set(AccessorComponent::AccessorGetter, getter);
    pair.set(AccessorComponent::AccessorSetter, setter);
    assert_eq!(pair.get(AccessorComponent::AccessorGetter), getter);
    assert_eq!(pair.get(AccessorComponent::AccessorSetter), setter);

    pair.set_release(AccessorComponent::AccessorGetter, setter);
    assert_eq!(pair.getter_acquire(), setter);
    pair.set_getter_release(getter, WriteBarrierMode::UpdateWriteBarrier);

    assert!(pair.equals(getter, setter));
    assert!(!pair.equals(getter, getter));

    // SetComponents: null 实参跳过不动 (struct-inl.h:120-123)
    pair.set_components(null, Smi::from_int(9).cast::<Object>());
    assert_eq!(pair.getter(), getter);
    assert_eq!(pair.setter(), Smi::from_int(9).cast::<Object>());

    pair.set_setter(setter, WriteBarrierMode::SkipWriteBarrier);
    assert_eq!(pair.setter_acquire(), setter);
  }
  OK
}

/// ClassPositions Smi 编解码 (struct-inl.h:130-138): start_/end_ const Smi 字段
/// 往返, 含负值 (原版 int 域)。
#[test]
fn class_positions_smi_roundtrip() -> Void {
  let mut arena = Arena::new();
  let map = arena.map(InstanceType::ClassPositions);
  let cp = arena.alloc_with(ClassPositions::new(map, -7, 12345), 0);

  // SAFETY: 竞技场对象有效, 槽值构造期即 Smi
  unsafe {
    assert_eq!(cp.start(), -7);
    assert_eq!(cp.end(), 12345);
  }
  OK
}

/// ForInEnumeratorHolder 四槽只读访问 (struct-inl.h:140-165)。
#[test]
fn for_in_enumerator_holder_slots() -> Void {
  let mut arena = Arena::new();
  let holder_map = arena.map(InstanceType::ForInEnumeratorHolder);
  let enum_map = arena.map(InstanceType::Tuple2);
  // SAFETY: 竞技场头 + 2 槽布局契约 (fixed_array_unittest 同法)
  let keys = arena.alloc_with(FixedArray::new(enum_map, 2), 2 * mem::size_of::<usize>());
  let holder = arena.alloc_with(
    ForInEnumeratorHolder::new(
      holder_map,
      enum_map,
      keys,
      Smi::from_int(5),
      Smi::from_int(2),
    ),
    0,
  );

  // SAFETY: 竞技场对象有效
  unsafe {
    assert_eq!(holder.enum_cache_map(), enum_map);
    assert_eq!(holder.named_keys(), keys);
    assert_eq!(holder.elements_length().value(), 5);
    assert_eq!(holder.cache_length().value(), 2);
  }
  OK
}

/// 族判别谓词 (casting.h DEF_CAST_TRAITS + instance_type_checker):
/// IsStruct 区间 vs 四子类精确; 区间外负例; TryCast 通路。
#[test]
fn struct_cast_predicates() -> Void {
  let mut arena = Arena::new();
  let v = Smi::from_int(1).cast::<Object>();
  // map 分配先落局部: alloc_with(&mut arena) 的实参求值期再变更 arena 属双借 (E0499)
  let t2_map = arena.map(InstanceType::Tuple2);
  // SAFETY: 竞技场对象有效
  let t2 = arena.alloc_with(Tuple2::new(t2_map, v, v), 0);
  let ap_map = arena.map(InstanceType::AccessorPair);
  let ap = arena.alloc_with(AccessorPair::new(ap_map), 0);
  let cp_map = arena.map(InstanceType::ClassPositions);
  let cp = arena.alloc_with(ClassPositions::new(cp_map, 0, 1), 0);
  let fa_map = arena.map(InstanceType::FixedArray);
  // SAFETY: 竞技场头布局契约 (0 槽)
  let fa = arena.alloc_with(FixedArray::new(fa_map, 0), 0);

  let t2_obj = t2.cast::<Object>();
  assert!(is::<Struct>(t2_obj));
  assert!(is::<Tuple2>(t2_obj));
  assert!(!is::<AccessorPair>(t2_obj));
  assert!(is::<AccessorPair>(ap.cast::<Object>()));
  assert!(is::<Struct>(ap.cast::<Object>()));
  assert!(is::<ClassPositions>(cp.cast::<Object>()));

  // 区间外负例: FixedArray (结构族区间 132..=174 之外)
  let fa_obj = fa.cast::<Object>();
  assert!(!is::<Struct>(fa_obj));
  assert!(!is::<Tuple2>(fa_obj));

  // TryCast 通路 (casting.h TryCast)
  assert_eq!(try_cast::<Tuple2>(t2_obj), Some(t2));
  assert_eq!(try_cast::<Tuple2>(fa_obj), None);
  // Smi 无实例类型 → 判别恒假
  assert!(!is::<Struct>(v));
  OK
}
