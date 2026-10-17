// v8-objects-maps/tests/concurrent_transition_array_unittest.rs

//! 对数基线: v8/test/unittests/objects/concurrent-transition-array-unittest.cc
//! (复审环 round1 [轮7 W7O2Rb] 阶段 C 固化)。
//!
//! 原用例为多线程并发 Search/Insert 场景 (需 Isolate + 后台线程), 本 crate 的过渡面尚无
//! Isolate/句柄线程语境, 故此处固化 **不依赖线程的原版可观察断言**: 过渡槽位序号算式、
//! 线性/二分查找的插入点语义、比较器符号规则、原型/侧步过渡哨兵判定。
//! 与并发前置直接相关、当前实现尚未满足原版语义的断言, 以 `#[ignore]` 附缺陷编号固化,
//! 待 src 侧修正后取消 ignore 即成验收用例。

use std::{mem::forget, ptr::write, sync::Mutex};

use aok::{OK, Void};
use v8_objects_arrays::fixed_array::WeakFixedArray;
use v8_objects_core::{
  instance_type::InstanceType,
  map::Map,
  name::{HashFieldType, Name},
  property_details::{PropertyAttributes, PropertyKind},
  smi::Smi,
  tagged::{MaybeObject, Tagged},
};
use v8_objects_maps::transitions::{
  Encoding, K_ENTRY_KEY_INDEX, K_ENTRY_SIZE, K_ENTRY_TARGET_INDEX, K_FIRST_INDEX,
  K_MAX_ELEMENTS_FOR_LINEAR_SEARCH, K_MAX_NUMBER_OF_TRANSITIONS, K_PROTO_TRANSITION_HEADER_SIZE,
  K_PROTO_TRANSITION_NUMBER_OF_ENTRIES_OFFSET, K_PROTOTYPE_TRANSITIONS_INDEX,
  K_SIDE_STEP_TRANSITIONS_INDEX, K_TRANSITION_LENGTH_INDEX, TransitionArray, TransitionArrayBuf,
  TransitionArrayExt, TransitionsAccessor,
};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

/// 模拟地址契约进程级台账: 以指针位 (非 usize) 持有 leak 出的堆对象。
/// Miri 泄漏检查将全局根可达的内存判为非泄漏, 无需 ignore-leaks 豁免,
/// 别名/UB/泄漏检查全部保持开启 (W11MPR round1; 实测 usize 表不解除泄漏报告, 指针表才有效)。
struct Lease(*mut u64);
// SAFETY: 表仅经 Mutex 保护跨线程收编指针位, 台账内指针从不跨线程解引用。
unsafe impl Send for Lease {}
impl Drop for Lease {
  fn drop(&mut self) {
    // 永不执行: 全局表条目驻留至进程结束; 仅声明指针所有权语义。
    forget(unsafe { Box::from_raw(self.0) });
  }
}

fn lease_ptr(ptr: *mut u64) {
  static LEASES: Mutex<Vec<Lease>> = Mutex::new(Vec::new());
  LEASES.lock().unwrap().push(Lease(ptr));
}

fn lease_map(ptr: *mut Map) {
  lease_ptr(ptr as *mut u64);
}

fn lease_name(ptr: *mut Name) {
  lease_ptr(ptr as *mut u64);
}

fn lease_words(ptr: *mut [u64]) {
  lease_ptr(ptr as *mut u64);
}

fn t_map() -> Tagged<Map> {
  let map = Map::new(Tagged::NULL, InstanceType::JSObject, 32);
  let r = Box::leak(Box::new(map));
  let ptr = r as *mut Map;
  lease_map(ptr);
  Tagged::from_ptr(ptr)
}

fn t_name(hash: u32) -> Tagged<Name> {
  let name = Name::with_hash_field(
    Tagged::NULL,
    Name::create_hash_field_value(hash, HashFieldType::Hash),
  );
  let r = Box::leak(Box::new(name));
  let ptr = r as *mut Name;
  lease_name(ptr);
  Tagged::from_ptr(ptr)
}

fn alloc_weak_fixed_array(capacity: u32) -> Tagged<WeakFixedArray> {
  use v8_objects_arrays::fixed_array::TaggedArrayExt;
  let bytes = <Tagged<WeakFixedArray>>::size_for(capacity);
  let words = bytes.div_ceil(8);
  let storage: &'static mut [u64] = Box::leak(vec![0u64; words].into_boxed_slice());
  let base = storage.as_mut_ptr() as usize;
  lease_words(storage as *mut [u64]);
  let header = WeakFixedArray::new(Tagged::NULL, capacity);
  unsafe {
    write(base as *mut WeakFixedArray, header);
  }
  unsafe { Tagged::from_raw_address(base) }
}

/// 原版 transitions.h:149/320 + :353-361 布局哨兵数字 (金标准字面量)。
#[test]
fn test_transition_array_layout_constants_against_header() -> Void {
  // kMaxNumberOfTransitions = 1024 + 512 = 1536 (transitions.h:149)
  assert_eq!(K_MAX_NUMBER_OF_TRANSITIONS, 1024 + 512);
  // kMaxElementsForLinearSearch = 32 (transitions.h:320)
  assert_eq!(K_MAX_ELEMENTS_FOR_LINEAR_SEARCH, 32);
  // 头三槽: 原型过渡 / 侧步过渡 / 过渡长度; 首条目下标 = 3 (transitions.h:353-356)
  assert_eq!(K_PROTOTYPE_TRANSITIONS_INDEX, 0);
  assert_eq!(K_SIDE_STEP_TRANSITIONS_INDEX, 1);
  assert_eq!(K_TRANSITION_LENGTH_INDEX, 2);
  assert_eq!(K_FIRST_INDEX, 3);
  // 条目内 key/target 序与条目宽 (transitions.h:359-361)
  assert_eq!(K_ENTRY_KEY_INDEX, 0);
  assert_eq!(K_ENTRY_TARGET_INDEX, 1);
  assert_eq!(K_ENTRY_SIZE, 2);
  OK
}

/// 原版 ToKeyIndex/ToTargetIndex (transitions.h:364-372) 与 Capacity (transitions-inl.h:455-460)。
#[test]
fn test_full_field_transitions_index_math() -> Void {
  // n 项过渡的 key/target 槽号 = kFirstIndex + n*kEntrySize + {0,1}
  for n in 0..8usize {
    assert_eq!(
      TransitionArray::to_key_index(n),
      K_FIRST_INDEX + n * K_ENTRY_SIZE
    );
    assert_eq!(
      TransitionArray::to_target_index(n),
      K_FIRST_INDEX + n * K_ENTRY_SIZE + K_ENTRY_TARGET_INDEX
    );
    assert_eq!(
      TransitionArray::to_target_index(n),
      TransitionArray::to_key_index(n) + 1
    );
  }
  // length 槽 (u32) = kFirstIndex + capacity*kEntrySize → Capacity 反解 = capacity
  let buf = TransitionArrayBuf::new(t_map(), 5);
  let array = buf.as_tagged();
  assert_eq!(array.capacity(), 5);
  assert_eq!(array.number_of_transitions(), 0);
  // 原版 Capacity: len <= kFirstIndex ⇒ 0 (transitions-inl.h:457-458)
  let tiny = TransitionArrayBuf::new(t_map(), 0);
  assert_eq!(tiny.as_tagged().capacity(), 0);
  OK
}

/// 线性段查找语义 (原版 SearchName/LinearSearchName, transitions-inl.h:262-331):
/// 命中返下标, 未命中插入点恒为 number_of_transitions (非有序插入位)。
#[test]
fn test_full_field_transitions_only_search_linear() -> Void {
  let buf = TransitionArrayBuf::new(t_map(), 8);
  let array = buf.as_tagged();
  // 三个 key, 哈希刻意乱序 (线性段不要求有序, 原版 :275 阈值内走 LinearSearchName)
  let n30 = t_name(30);
  let n10 = t_name(10);
  let n20 = t_name(20);
  array.set_key(0, n30);
  array.set_key(1, n10);
  array.set_key(2, n20);
  array.set_number_of_transitions(3);

  assert_eq!(array.number_of_transitions(), 3);
  let mut ins = None;
  assert_eq!(array.search_name(n10, false, &mut ins), Some(1));
  assert_eq!(array.search_name(n20, false, &mut ins), Some(2));
  // 未命中: 原版线性搜索把插入点写为 len (transitions-inl.h:327-329)
  let miss = t_name(7);
  assert_eq!(array.search_name(miss, false, &mut ins), None);
  assert_eq!(ins, Some(3));
  // count == 0 时插入点为 0 (transitions-inl.h:266-270)
  let empty = TransitionArrayBuf::new(t_map(), 4);
  let mut ins0 = None;
  assert_eq!(empty.as_tagged().search_name(miss, false, &mut ins0), None);
  assert_eq!(ins0, Some(0));
  OK
}

/// 二分段查找语义 (原版 BinarySearchName, transitions-inl.h:282-319):
/// lower_bound 定首个不劣哈希, 命中返下标; 未命中插入点 = i + (entry_hash > hash ? 0 : 1)。
#[test]
fn test_full_field_transitions_search_binary() -> Void {
  let count = K_MAX_ELEMENTS_FOR_LINEAR_SEARCH + 8; // 35 > 32 ⇒ 走二分
  let buf = TransitionArrayBuf::new(t_map(), count);
  let array = buf.as_tagged();
  // 按哈希升序铺 key (二分前置: 数组已排序, 原版 SLOW_DCHECK IsSortedNoDuplicates :264)
  let mut names = Vec::new();
  for i in 0..count {
    let n = t_name((i as u32) * 2); // 0,2,4,...,68: 互异哈希, 规避碰撞分支
    array.set_key(i, n);
    names.push(n);
  }
  array.set_number_of_transitions(count);

  let mut ins = None;
  assert_eq!(array.search_name(names[17], false, &mut ins), Some(17));
  assert_eq!(array.search_name(names[0], false, &mut ins), Some(0));
  assert_eq!(
    array.search_name(names[count - 1], false, &mut ins),
    Some(count - 1)
  );
  // 缺 21 (奇数, 不在表内): 首个不劣哈希项为 22 即 names[11], entry_hash(22) > 21 ⇒ 插入点 11
  let absent = t_name(21);
  assert_eq!(array.search_name(absent, false, &mut ins), None);
  assert_eq!(ins, Some(11));
  // 超过末项 (末 key 哈希 = 2*(count-1)): 插入点 = end
  let beyond = t_name(2 * count as u32 + 1);
  assert_eq!(array.search_name(beyond, false, &mut ins), None);
  assert_eq!(ins, Some(count));
  OK
}

/// 比较器符号规则 (原版 CompareDetails, transitions-inl.h:434-448):
/// kind 优先, 其次 attributes; 相等返 0, 严格小于返 -1, 否则 1。
#[test]
fn test_full_field_transitions_compare_details_order() -> Void {
  let data = PropertyKind::Data;
  let accessor = PropertyKind::Accessor;
  let none = PropertyAttributes::NONE;
  let dontl = PropertyAttributes::DONT_ENUM;
  assert_eq!(TransitionArray::compare_details(data, none, data, none), 0);
  // kind1 < kind2 ⇒ -1 (原版按底层枚举值比较)
  let cmp = TransitionArray::compare_details(data, none, accessor, none);
  assert_eq!(
    cmp,
    if (data as u8) < (accessor as u8) {
      -1
    } else {
      1
    }
  );
  // kind 相同, attributes 分歧
  let cmp_a = TransitionArray::compare_details(data, none, data, dontl);
  assert_eq!(cmp_a, if none.0 < dontl.0 { -1 } else { 1 });
  // 反对称性 (仅 details 面)
  assert_eq!(
    cmp_a,
    -TransitionArray::compare_details(data, dontl, data, none)
  );
  OK
}

/// 原型/侧步过渡槽位存在性判定 (原版 transitions-inl.h:45-59, :100-118)。
#[test]
fn test_full_field_transitions_prototype_and_sidestep_slots() -> Void {
  let buf = TransitionArrayBuf::new(t_map(), 4);
  let array = buf.as_tagged();
  assert!(!array.has_prototype_transitions());
  assert!(!array.has_side_step_transitions());
  assert_eq!(array.get_prototype_transitions(), None);
  assert_eq!(array.get_side_step_transitions(), None);

  // 挂两枚 WeakFixedArray (原版哨兵: 非 Smi::zero() 即视为存在)
  let proto: Tagged<WeakFixedArray> = alloc_weak_fixed_array(4);
  array.set_prototype_transitions(proto);
  assert!(array.has_prototype_transitions());
  assert_eq!(array.get_prototype_transitions(), Some(proto));

  let side: Tagged<WeakFixedArray> = alloc_weak_fixed_array(4);
  array.set_side_step_transitions(side);
  assert!(array.has_side_step_transitions());
  assert_eq!(array.get_side_step_transitions(), Some(side));
  OK
}

/// Encoding 枚举序与原版一致 (transitions.h:226-233: kPrototypeInfo, kUninitialized,
/// kMigrationTarget, kWeakRef, kFullTransitionArray, kPrototypeSharedClosureInfo)。
#[test]
fn test_full_field_transitions_encoding_discriminants() -> Void {
  assert_eq!(Encoding::PrototypeInfo as u8, 0);
  assert_eq!(Encoding::Uninitialized as u8, 1);
  assert_eq!(Encoding::MigrationTarget as u8, 2);
  assert_eq!(Encoding::WeakRef as u8, 3);
  assert_eq!(Encoding::FullTransitionArray as u8, 4);
  assert_eq!(Encoding::PrototypeSharedClosureInfo as u8, 5);
  OK
}

// ── 以下四条为 round1 findings 的验收断言, 修好对应 must-fix 后去掉 #[ignore] ──

/// [MF-E2] 原版 CompareNames (transitions-inl.h:422-431) 在 key 不同且哈希相等时
/// 恒返 -1 ("collision ⇒ key1 is always 'less'"); 现实现改用指针序定序。
#[test]
fn test_collision_key_order_matches_compare_names() -> Void {
  let a = t_name(99);
  let b = t_name(99);
  let ab = TransitionArray::compare_keys(
    a,
    PropertyKind::Data,
    PropertyAttributes::NONE,
    b,
    PropertyKind::Data,
    PropertyAttributes::NONE,
  );
  assert_eq!(ab, -1, "原版: 同哈希不同 key ⇒ key1 less");
  OK
}

/// [MF-E9] 原版 SearchName 在 concurrent_search 下 **强制线性** (transitions-inl.h:275-276),
/// 现实现无该形参, 超过 32 项即二分 (未排序数组上二分被原版 :286-287 明令禁止)。
#[test]
fn test_concurrent_search_forces_linear() -> Void {
  // 签名: search_name(name, concurrent: bool, out) — 并发位为真时 >32 项亦走线性
  let count = K_MAX_ELEMENTS_FOR_LINEAR_SEARCH + 4;
  let buf = TransitionArrayBuf::new(t_map(), count);
  let array = buf.as_tagged();
  // 数组刻意逆序铺设: 元素递减，二分必失效，线性必命中
  let mut names = Vec::new();
  for i in 0..count {
    let n = t_name(((count - 1 - i) as u32) * 2);
    array.set_key(i, n);
    names.push(n);
  }
  array.set_number_of_transitions(count);
  let mut ins = None;
  // 并发查找强制走线性搜索，在乱序/逆序数组上仍能正确命中！
  assert_eq!(
    array.search_name(names[count - 1], true, &mut ins),
    Some(count - 1)
  );
  OK
}

/// [MF-E7] 原版 number_of_transitions 有 `ulength() < kFirstIndex ⇒ 0` 前置守卫
/// (transitions-inl.h:405), 现实现直接读槽 2。
#[test]
fn test_number_of_transitions_length_guard() -> Void {
  // 构造 length 槽 < 3 的畸形数组后 number_of_transitions() 返 0 且不越界读
  let buf = TransitionArrayBuf::new(t_map(), 4);
  let array = buf.as_tagged();
  // 把底层 length 压到 kFirstIndex 以下, 复刻原版兜底路径
  unsafe {
    (*array.as_mut_ptr()).base.length = (K_FIRST_INDEX - 1) as u32;
  }
  assert_eq!(array.number_of_transitions(), 0);
  OK
}

/// [MF-E9b] 原版哨兵判据是 `!= Smi::zero()` (transitions-inl.h:47/58),
/// 现实现写为 `!is_null() && !is_smi()` ⇒ 非零 Smi 哨兵被误判为"无原型过渡"。
#[test]
fn test_nonzero_smi_sentinel_counts_as_present() -> Void {
  let buf = TransitionArrayBuf::new(t_map(), 4);
  let array = buf.as_tagged();
  let smi_one: Tagged<MaybeObject> = Smi::from_int(1).cast();
  array.set_raw_slot(K_PROTOTYPE_TRANSITIONS_INDEX, smi_one);
  assert!(
    array.has_prototype_transitions(),
    "原版: 非 Smi::zero() 即为有"
  );
  OK
}

/// [N-E2] 原版 NumberOfPrototypeTransitions (transitions-inl.h:121-129)
/// 与 TraverseTransitionTree 包含原型过渡分支 (transitions.cc:661-676)。
#[test]
fn test_prototype_transitions_count_and_traversal() -> Void {
  use v8_objects_arrays::fixed_array::TaggedArrayExt;
  use v8_objects_core::{objects::WriteBarrierMode, tagged::make_weak};

  // 构造 proto transitions 数组: 长度 3 (头 1 槽 + 2 目标 Map)
  let proto_tagged: Tagged<WeakFixedArray> = alloc_weak_fixed_array(3);
  // 槽 0 写入条目数 (Smi 2)
  unsafe {
    proto_tagged.set(
      K_PROTO_TRANSITION_NUMBER_OF_ENTRIES_OFFSET as u32,
      Smi::from_int(2).cast(),
      WriteBarrierMode::SkipWriteBarrier,
    );
  }
  let target_map1 = t_map();
  let target_map2 = t_map();
  unsafe {
    proto_tagged.set(
      K_PROTO_TRANSITION_HEADER_SIZE as u32,
      make_weak(target_map1).cast(),
      WriteBarrierMode::SkipWriteBarrier,
    );
    proto_tagged.set(
      (K_PROTO_TRANSITION_HEADER_SIZE + 1) as u32,
      make_weak(target_map2).cast(),
      WriteBarrierMode::SkipWriteBarrier,
    );
  }

  // 校验 number_of_prototype_transitions
  assert_eq!(
    TransitionArray::number_of_prototype_transitions(proto_tagged),
    2
  );

  // 挂载到 root_map 的 TransitionArray 上
  let root_map = t_map();
  let buf = TransitionArrayBuf::new(root_map, 4);
  let array = buf.as_tagged();
  array.set_prototype_transitions(proto_tagged);

  // 另外挂一个普通属性过渡 target_map3
  let target_map3 = t_map();
  let prop_name = t_name(100);
  TransitionArrayExt::set(&array, 0, prop_name, make_weak(target_map3).cast());
  array.set_number_of_transitions(1);

  unsafe {
    (*root_map.as_mut_ptr()).transitions_or_prototype_info = array.cast();
  }

  // 遍历过渡树，验证遍历收集到了原型过渡目标与属性过渡目标
  let mut visited = Vec::new();
  TransitionsAccessor::traverse_transition_tree(root_map, |m| {
    visited.push(m);
  });

  // 包含 root_map, target_map1, target_map2, target_map3 共 4 个 Map
  assert_eq!(visited.len(), 4);
  assert_eq!(visited[0], root_map);
  assert!(visited.contains(&target_map1));
  assert!(visited.contains(&target_map2));
  assert!(visited.contains(&target_map3));

  OK
}
