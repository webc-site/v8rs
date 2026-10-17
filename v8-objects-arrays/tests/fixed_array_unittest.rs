//! FixedArray 族规约测试。
//!
//! 规约来源 (实测搜索 v8/test/unittests/objects):
//! - weakarraylist-unittest.cc (CastMaybeObjectToUnion / Compact / OutOfPlaceCompact)
//! - array-list-unittest.cc (ArrayList set/get/length 子集)
//! - fixed-array-inl.h / fixed-array-base.h 语义锚定 (布局/容量/增长/裁剪不变式)
//!
//! 跳过登记 (推迟, 归 L3 heap/factory):
//! - OutOfPlaceCompact (factory CompactWeakArrayList)
//! - ArrayList::Add/EnsureSpace/ToFixedArray (isolate+factory)
//! - New/Allocate 全族 (右裁实际裁剪走 Heap::RightTrimArray)
//!
//! 替代登记: ReadOnlyRoots 值 (undefined/the_hole/cow map) 以参数注入
//! (Smi 或专用值替代, 语义只依赖 "非清除/等值比较")。

pub mod common;

use std::mem;

use aok::{OK, Void};
use common::Arena;
use v8_objects_arrays::{
  Error,
  fixed_array::{
    ArrayList, FixedArray, FixedArrayExt, K_ARRAY_LIST_HEADER_SIZE, K_WEAK_ARRAY_LIST_HEADER_SIZE,
    TaggedArray, TaggedArrayExt, WeakArrayList, WeakArrayListCc, WeakArrayListExt, WeakFixedArray,
    is_fixed_array_exact,
  },
  fixed_array_base::{
    FixedArrayBase, FixedArrayBaseExt, K_HEADER_SIZE, K_LENGTH_OFFSET, K_MAX_FIXED_ARRAY_CAPACITY,
    K_MAX_REGULAR_LENGTH,
  },
};
use v8_objects_core::{
  heap_object::HeapObject,
  instance_type::InstanceType,
  objects::{WriteBarrierMode, is},
  smi::Smi,
  tagged::{MaybeObject, Object, Tagged, make_weak},
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 竞技场内分配 FixedArray (len 槽位)。
fn fixed_array(arena: &mut Arena, len: u32) -> Tagged<FixedArray> {
  let map = arena.map(InstanceType::FixedArray);
  let tail = Tagged::<FixedArray>::size_for(len) - mem::size_of::<FixedArray>();
  // SAFETY: 竞技场对象布局契约成立 (头 + len 槽)
  arena.alloc_with(FixedArray::new(map, len), tail)
}

/// CastMaybeObjectToUnion (weakarraylist-unittest.cc:14-24):
/// 弱化 / Smi / 清除哨兵三态谓词。
#[test]
fn weak_array_list_cast_maybe_object_to_union() -> Void {
  let mut arena = Arena::new();
  let obj = fixed_array(&mut arena, 0);

  // SAFETY: 竞技场对象有效
  let weak = make_weak(obj).cast::<MaybeObject>();
  let smi = Smi::from_int(1).cast::<MaybeObject>();
  let cleared = Tagged::<MaybeObject>::cleared();

  assert!(weak.is_weak_live());
  assert!(smi.is_smi());
  assert!(cleared.is_cleared());
  OK
}

/// Compact (weakarraylist-unittest.cc:26-44):
/// [weak, smi, cleared, cleared, 未置槽(非清除)] → 紧缩后 length=3。
///
/// 未置槽在原版为 undefined (强值非清除, 计入存活); 此处以 Smi 42 替代
/// (替代登记见文件头)。
#[test]
fn weak_array_list_compact() -> Void {
  let mut arena = Arena::new();
  let map = arena.map(InstanceType::WeakArrayList);
  // SAFETY: 竞技场对象布局契约成立 (头 + 10 槽)
  let list = arena.alloc_with(
    WeakArrayList::new(map, 10, 0),
    Tagged::<WeakArrayList>::size_for(10) - mem::size_of::<WeakArrayList>(),
  );

  let obj = fixed_array(&mut arena, 0);
  let weak_ref = make_weak(obj).cast::<MaybeObject>();
  let smi = Smi::from_int(0).cast::<MaybeObject>();
  let cleared = Tagged::<MaybeObject>::cleared();
  let undefined_standin = Smi::from_int(42).cast::<MaybeObject>();

  // SAFETY: 对象有效, 越界由 debug 断言拦截
  unsafe {
    assert_eq!(list.length(), 0);
    assert_eq!(list.capacity(), 10);

    list.set_relaxed(0, weak_ref, WriteBarrierMode::UpdateWriteBarrier);
    list.set_relaxed(1, smi, WriteBarrierMode::UpdateWriteBarrier);
    list.set_relaxed(2, cleared, WriteBarrierMode::UpdateWriteBarrier);
    list.set_relaxed(3, cleared, WriteBarrierMode::UpdateWriteBarrier);
    list.set_relaxed(4, undefined_standin, WriteBarrierMode::UpdateWriteBarrier);
    list.set_length(5);

    list.compact();
    assert_eq!(list.length(), 3);
    assert_eq!(list.capacity(), 10);
    assert_eq!(list.get(0), weak_ref);
    assert_eq!(list.get(1), smi);
    assert_eq!(list.get(2), undefined_standin);
  }
  OK
}

/// ArrayList set/get/length 子集 (array-list-unittest.cc:32-38):
/// Smi 槽读写往返 + set_length 收缩保序。
/// Add/EnsureSpace/ToFixedArray 推迟 (文件头登记)。
#[test]
fn array_list_get_set_length() -> Void {
  let mut arena = Arena::new();
  let map = arena.map(InstanceType::ArrayList);
  // SAFETY: 竞技场对象布局契约成立 (头 + 8 槽)
  let array = arena.alloc_with(
    ArrayList::new(map, 8, 3),
    Tagged::<ArrayList>::size_for(8) - mem::size_of::<ArrayList>(),
  );

  // SAFETY: 对象有效
  unsafe {
    array.set_smi(0, Smi::from_int(100));
    array.set_smi(1, Smi::from_int(200));
    array.set_smi(2, Smi::from_int(300));
    assert_eq!(array.ulength(), 3);

    array.set_smi(2, Smi::from_int(400));
    assert_eq!(array.get(2).to_smi().map(Smi::value), Some(400));

    array.set_length(2);
    assert_eq!(array.ulength(), 2);
    assert_eq!(array.get(0).to_smi().map(Smi::value), Some(100));
    assert_eq!(array.get(1).to_smi().map(Smi::value), Some(200));
  }
  OK
}

/// snapshot 布局锚点 (fixed-array-base.h:33-42, fixed-array.h:646-648/725-727):
/// 压缩默认配置 (K_TAGGED_SIZE=4) 下的头尺寸/容量上限/常规页上限。
#[test]
fn layout_snapshot_anchors() -> Void {
  assert_eq!(K_LENGTH_OFFSET, 4);
  assert_eq!(K_HEADER_SIZE, 8);
  assert_eq!(K_MAX_FIXED_ARRAY_CAPACITY, 128 * 1024 * 1024);
  // kMaxRegularHeapObjectSize = 半页 = 128K; (128K - 8) / 4
  assert_eq!(K_MAX_REGULAR_LENGTH, (128 * 1024 - K_HEADER_SIZE) / 4);
  assert_eq!(K_WEAK_ARRAY_LIST_HEADER_SIZE, 12);
  assert_eq!(K_ARRAY_LIST_HEADER_SIZE, 12);
  OK
}

/// SizeFor / OffsetOfElementAt / MaxRegularCapacity (fixed-array.h:210-225)。
#[test]
fn size_for_and_offsets() -> Void {
  // 运行时机器: 结构体逻辑布局 (Tagged 槽 usize 宽)
  let objects_off = mem::size_of::<FixedArray>();
  assert_eq!(FixedArray::OBJECTS_OFFSET, objects_off);
  assert_eq!(
    Tagged::<FixedArray>::size_for(3),
    objects_off + 3 * mem::size_of::<usize>()
  );
  assert_eq!(
    Tagged::<FixedArray>::offset_of_element_at(1),
    Tagged::<FixedArray>::size_for(1)
  );
  assert_eq!(FixedArray::ELEMENT_SIZE, mem::size_of::<usize>());
  // 常规页容量 = (半页 - 数据区偏移) / 元素大小
  assert_eq!(
    Tagged::<FixedArray>::max_regular_capacity(),
    (128 * 1024 - objects_off) / mem::size_of::<usize>()
  );
  assert_eq!(
    Tagged::<FixedArray>::MAX_CAPACITY,
    K_MAX_FIXED_ARRAY_CAPACITY
  );
  OK
}

/// NewCapacityForIndex (fixed-array-inl.h:351-361) 与
/// CapacityForLength (fixed-array.h:600)。
#[test]
fn capacity_growth() -> Void {
  // do-while: 至少增长一轮 (index=old=0 → 单轮 0+0+16=16)
  assert_eq!(Tagged::<FixedArray>::new_capacity_for_index(0, 0), 16);
  // 16 不覆盖 16 → 第二轮 16+8+16=40
  assert_eq!(Tagged::<FixedArray>::new_capacity_for_index(16, 0), 40);
  assert_eq!(Tagged::<FixedArray>::new_capacity_for_index(17, 16), 40);
  assert_eq!(Tagged::<FixedArray>::new_capacity_for_index(40, 16), 76);

  // length + max(length/2, 2)
  assert_eq!(
    <Tagged<WeakArrayList> as WeakArrayListExt>::capacity_for_length(0),
    2
  );
  assert_eq!(
    <Tagged<WeakArrayList> as WeakArrayListExt>::capacity_for_length(1),
    3
  );
  assert_eq!(
    <Tagged<WeakArrayList> as WeakArrayListExt>::capacity_for_length(2),
    4
  );
  assert_eq!(
    <Tagged<WeakArrayList> as WeakArrayListExt>::capacity_for_length(10),
    15
  );
  assert_eq!(
    <Tagged<WeakArrayList> as WeakArrayListExt>::size_for_capacity(4),
    Tagged::<WeakArrayList>::size_for(4)
  );
  OK
}

/// get/set 四序 + swap + compare_and_swap (fixed-array-inl.h:49-160)。
#[test]
fn element_access_orderings() -> Void {
  let mut arena = Arena::new();
  let fa = fixed_array(&mut arena, 2);
  let v7 = Smi::from_int(7).cast::<Object>();
  let v9 = Smi::from_int(9).cast::<Object>();
  let v11 = Smi::from_int(11).cast::<Object>();

  // SAFETY: 对象有效
  unsafe {
    fa.set_smi(0, Smi::from_int(7));
    fa.set(1, v9, WriteBarrierMode::UpdateWriteBarrier);
    assert_eq!(fa.get(0), v7);
    assert_eq!(fa.get_relaxed(0), v7);
    assert_eq!(fa.get_acquire(0), v7);
    assert_eq!(fa.get_seqcst(1), v9);

    fa.set_smi_release(1, Smi::from_int(9));
    fa.set_smi_seqcst(1, Smi::from_int(9));
    fa.set_release(0, v7, WriteBarrierMode::SkipWriteBarrier);
    fa.set_seqcst(0, v7, WriteBarrierMode::SkipWriteBarrier);
    fa.set_smi_relaxed(0, Smi::from_int(7));

    // swap → 旧值
    let old = fa.swap(0, v11, WriteBarrierMode::UpdateWriteBarrier);
    assert_eq!(old, v7);
    assert_eq!(fa.get(0), v11);

    // cas: 期望命中 → 换回; 期望失配 → 返回当前值不动
    let r = fa.compare_and_swap(0, v11, v9, WriteBarrierMode::UpdateWriteBarrier);
    assert_eq!(r, v11);
    let r = fa.compare_and_swap(0, v11, v11, WriteBarrierMode::UpdateWriteBarrier);
    assert_eq!(r, v9);
    assert_eq!(fa.get(0), v9);
  }
  OK
}

/// RightTrim 不变式 (fixed-array-inl.h:198-206):
/// 0 < new <= capacity; 实际裁剪推迟 heap (Err Unsupported 登记)。
#[test]
fn right_trim_invariants() -> Void {
  let mut arena = Arena::new();
  let fa = fixed_array(&mut arena, 3);

  assert!(fa.right_trim(0).is_err());
  assert!(fa.right_trim(4).is_err());
  assert!(fa.right_trim(3).is_ok());
  assert_eq!(
    fa.right_trim(2).unwrap_err(),
    Error::Unsupported("RightTrim: Heap::RightTrimArray 归 L3 heap")
  );
  OK
}

/// IsCowArray (roots 注入版; fixed-array-base-inl.h:73-76)。
#[test]
fn is_cow_array_with_injected_map() -> Void {
  let mut arena = Arena::new();
  let fa = fixed_array(&mut arena, 1);
  let other_map = arena.map(InstanceType::WeakFixedArray);

  // SAFETY: 对象有效 (读 map 槽)
  unsafe {
    let fa_map = fa.as_heap_object().map();
    assert!(fa.is_cow_array(fa_map));
    assert!(!fa.is_cow_array(other_map));
  }
  OK
}

/// Cast 谓词 (casting.h DEF_CAST_TRAITS 对应; 区间 vs 精确)。
#[test]
fn cast_predicates() -> Void {
  let mut arena = Arena::new();
  let fa = fixed_array(&mut arena, 0);
  let obj = fa.cast::<Object>();
  assert!(is::<FixedArray>(obj));
  assert!(is_fixed_array_exact(obj));

  // 区间内非精确类型 (NumberDictionary=213 ∈ FIRST..LAST_FIXED_ARRAY_TYPE):
  // is_fixed_array 真, is_fixed_array_exact 假
  let map = arena.map(InstanceType::NumberDictionary);
  // SAFETY: 竞技场对象布局契约成立
  let dict = arena.alloc_with(FixedArray::new(map, 0), 0);
  let dict_obj = dict.cast::<Object>();
  assert!(is::<FixedArray>(dict_obj));
  assert!(!is_fixed_array_exact(dict_obj));

  // 区间外 (WeakFixedArray=222): 非 FixedArray, 属 WeakFixedArray
  let weak_map = arena.map(InstanceType::WeakFixedArray);
  // SAFETY: 同上
  let wfa = arena.alloc_with(FixedArray::new(weak_map, 0), 0);
  assert!(!is::<FixedArray>(wfa.cast::<Object>()));
  assert!(is::<WeakFixedArray>(wfa.cast::<Object>()));
  OK
}

/// WeakArrayList 计数/查找/移除/迭代 (fixed-array.cc:290-352 + h:667-679)。
#[test]
fn weak_array_list_live_counts_iter_remove() -> Void {
  let mut arena = Arena::new();
  let map = arena.map(InstanceType::WeakArrayList);
  // SAFETY: 竞技场对象布局契约成立
  let list = arena.alloc_with(
    WeakArrayList::new(map, 4, 3),
    Tagged::<WeakArrayList>::size_for(4) - mem::size_of::<WeakArrayList>(),
  );

  let obj = fixed_array(&mut arena, 0);
  let weak_ref = make_weak(obj).cast::<MaybeObject>();
  let smi = Smi::from_int(0).cast::<MaybeObject>();
  let cleared = Tagged::<MaybeObject>::cleared();

  // SAFETY: 对象有效
  unsafe {
    list.set_relaxed(0, weak_ref, WriteBarrierMode::UpdateWriteBarrier);
    list.set_relaxed(1, smi, WriteBarrierMode::UpdateWriteBarrier);
    list.set_relaxed(2, cleared, WriteBarrierMode::UpdateWriteBarrier);

    assert!(!list.is_full());
    assert_eq!(list.count_live_weak_references(), 1);
    assert_eq!(list.count_live_elements(), 2);
    assert!(list.contains(smi));
    assert!(list.contains(weak_ref));
    let other_obj = fixed_array(&mut arena, 0);
    assert!(!list.contains(make_weak(other_obj).cast::<MaybeObject>()));

    // 迭代: 仅产出活弱引用指向的堆对象
    let collected: Vec<_> =
      <Tagged<WeakArrayList> as WeakArrayListExt>::iter_weak_heap_objects(list).collect();
    assert_eq!(collected, [obj.cast::<HeapObject>()]);

    // RemoveOne(smi): 尾元素 (cleared) 补位 → 槽 1 变 cleared, 槽 2 置清除哨兵
    assert!(list.remove_one(smi));
    assert_eq!(list.length(), 2);
    assert!(!list.contains(smi));
    assert!(!list.remove_one(smi));
  }

  // 空表早退 (原版 last_index=-1 循环不执行; u32 下溢防护)
  let empty_map = arena.map(InstanceType::WeakArrayList);
  let empty = arena.alloc_with(WeakArrayList::new(empty_map, 0, 0), 0);
  // SAFETY: 对象有效
  assert!(!unsafe { empty.remove_one(cleared) });
  OK
}

/// MemsetTagged/fill + hole 族 (fixed-array-inl.h:379-396)。
#[test]
fn fill_and_holes() -> Void {
  let mut arena = Arena::new();
  let fa = fixed_array(&mut arena, 4);
  let v5 = Smi::from_int(5).cast::<Object>();
  let hole = Smi::from_int(-1).cast::<Object>();

  // SAFETY: 对象有效
  unsafe {
    fa.fill(1, 3, v5, WriteBarrierMode::SkipWriteBarrier);
    assert_eq!(fa.get(1), v5);
    assert_eq!(fa.get(2), v5);
    assert_ne!(fa.get(0), v5);

    assert!(!fa.is_the_hole(hole, 0));
    fa.set_the_hole(hole, 0);
    assert!(fa.is_the_hole(hole, 0));

    fa.fill_with_holes(hole, 1, 3);
    assert!(fa.is_the_hole(hole, 1));
    assert!(fa.is_the_hole(hole, 2));
  }
  OK
}

/// FixedArrayBase length 语义 (fixed-array-base-inl.h:19-37, 67-71):
/// 四序读写 + 新空间分配上限 (shift 注入版)。
#[test]
fn fixed_array_base_length_semantics() -> Void {
  let mut arena = Arena::new();
  let map = arena.map(InstanceType::FixedArray);
  // SAFETY: 竞技场对象布局契约成立 (头 + 尾区)
  let base = arena.alloc_with(FixedArrayBase::new(map, 7), 8);

  // SAFETY: 对象有效
  unsafe {
    assert_eq!(base.length(), 7);
    assert_eq!(base.ulength(), 7);
    assert_eq!(base.length_relaxed(), 7);
    assert_eq!(base.length_acquire(), 7);
    base.set_length(9);
    assert_eq!(base.length(), 9);
    base.set_length_release(11);
    assert_eq!(base.length_acquire(), 11);
  }

  // ((128K - 8) >> 2) = tagged 槽 shift
  assert_eq!(
    FixedArrayBase::max_length_for_new_space_allocation(2),
    (128 * 1024 - K_HEADER_SIZE) >> 2
  );
  OK
}

/// TaggedArrayExt::iter 元素迭代器验证
#[test]
fn tagged_array_iter() -> Void {
  let mut arena = Arena::new();
  let fa = fixed_array(&mut arena, 3);
  let v1 = Smi::from_int(10).cast::<Object>();
  let v2 = Smi::from_int(20).cast::<Object>();
  let v3 = Smi::from_int(30).cast::<Object>();

  // SAFETY: 测试环境使用独占分配的有效 arena 内存，索引处于有效容量内
  unsafe {
    fa.set(0, v1, WriteBarrierMode::SkipWriteBarrier);
    fa.set(1, v2, WriteBarrierMode::SkipWriteBarrier);
    fa.set(2, v3, WriteBarrierMode::SkipWriteBarrier);

    let collected: Vec<_> = fa.iter().collect();
    assert_eq!(collected, [v1, v2, v3]);
    assert_eq!(fa.iter().len(), 3);
  }
  OK
}

/// MoveElements/CopyElements 槽位级落地 (fixed-array-inl.h:162-196, 398-408):
/// 同数组重叠自搬 (memmove 语义) + 跨数组拷贝 + len=0 早退。
/// Heap::MoveRange/CopyRange 写屏障簿记走 stub (脚本头登记)。
#[test]
fn move_and_copy_elements() -> Void {
  let mut arena = Arena::new();
  let fa = fixed_array(&mut arena, 4);
  let v1 = Smi::from_int(1).cast::<Object>();
  let v2 = Smi::from_int(2).cast::<Object>();
  let v3 = Smi::from_int(3).cast::<Object>();
  let skip = WriteBarrierMode::SkipWriteBarrier;

  // SAFETY: 对象有效, 区间在 length(=4) 内
  unsafe {
    fa.set(0, v1, skip);
    fa.set(1, v2, skip);
    fa.set(2, v3, skip);

    // 重叠自搬 [0,3) → [1,4): memmove 保源序
    TaggedArrayExt::move_elements(&fa, 1, &fa, 0, 3, skip);
    assert_eq!(fa.get(1), v1);
    assert_eq!(fa.get(2), v2);
    assert_eq!(fa.get(3), v3);
    assert_eq!(fa.get(0), v1);

    // len=0 早退不动
    TaggedArrayExt::copy_elements(&fa, 2, &fa, 0, 0, skip);
    assert_eq!(fa.get(2), v2);
  }

  // 跨数组拷贝: src[0,2) → fa[0,2)
  let src = fixed_array(&mut arena, 2);
  // SAFETY: 对象有效
  unsafe {
    src.set(0, v3, skip);
    src.set(1, v1, skip);
    TaggedArrayExt::copy_elements(&fa, 0, &src, 0, 2, skip);
    assert_eq!(fa.get(0), v3);
    assert_eq!(fa.get(1), v1);
  }
  OK
}
