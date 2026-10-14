// v8-objects-maps/tests/test_transitions.rs

//! 对数基线: v8/test/cctest/test-transitions.cc (原用例名 1:1 保留, todo.md 7.3/12.8)。
//!
//! 原版用例依赖 Isolate/Factory/CcTest, 此处按本 crate 堆模拟 harness 移植 (同
//! objects_maps_unittest.rs 的 create_test_map/create_test_name 模式), 断言与原版逐条对齐:
//! - TransitionArray_SimpleFieldTransitions / _DifferentFieldNames /
//!   _SameFieldNamesDifferentAttributesSimple / _SameFieldNamesDifferentAttributes /
//!   _LinearSearchHandlesUnsortedArray 为本波可迁件。
//! - 未迁件 (原因就地登记): TransitionArray_FullFieldTransitions 依赖 TransitionKindFlag
//!   (PROPERTY_TRANSITION 强制 Full 编码), 本 crate insert 面无 kind flag 参数 (冻结台账 MF-E4 遗留);
//!   UNINITIALIZED_InsertToBinarySearchSizeAfterRehashing 与
//!   UNINITIALIZED_RehashNoDuplicatesWithinLinearSearchSize 依赖快照初始化 Isolate 与堆 GC 语义,
//!   解锁 crate v8-runtime / v8-heap-core 后补迁。
//!
//! 另固化 readme/zh.md 台账波 10 W10MAPS 填体断言 (TR-ISO 7 件): :17/:333/:517/:554/:572/:588/:893
//! 经 usize↔Tagged 地址契约调用 `transitions_cc_generated` 面。

use std::{mem::forget, ptr::write, sync::Mutex};

use aok::{OK, Void};
use v8_objects_arrays::fixed_array::{TaggedArrayExt, WeakFixedArray};
use v8_objects_core::{
  instance_type::InstanceType,
  map::Map,
  name::{HashFieldType, Name},
  property_details::{
    PropertyAttributes, PropertyConstness, PropertyDetails, PropertyKind, PropertyLocation,
    Representation,
  },
  smi::Smi,
  tagged::{MaybeObject, Tagged},
};
use v8_objects_maps::{
  descriptor_array::{DescriptorArrayBuf, DescriptorArrayExt},
  name_ext::NameTaggedExt,
  transitions::{
    Encoding, K_MAX_ELEMENTS_FOR_LINEAR_SEARCH, K_PROTO_TRANSITION_NUMBER_OF_ENTRIES_OFFSET,
    TransitionArrayExt, TransitionsAccessor,
  },
  transitions_cc_generated,
};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

fn create_test_map(instance_type: InstanceType, instance_size_bytes: usize) -> Map {
  Map::new(Tagged::NULL, instance_type, instance_size_bytes)
}

fn create_test_name(hash: u32) -> Name {
  Name::with_hash_field(
    Tagged::NULL,
    Name::create_hash_field_value(hash, HashFieldType::Hash),
  )
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

fn lease_desc(ptr: *mut DescriptorArrayBuf) {
  lease_ptr(ptr as *mut u64);
}

fn lease_words(ptr: *mut [u64]) {
  lease_ptr(ptr as *mut u64);
}

fn lease_map_ref(r: &'static mut Map) -> Tagged<Map> {
  let ptr = r as *mut Map;
  lease_map(ptr);
  Tagged::from_ptr(ptr)
}

fn lease_name_ref(r: &'static mut Name) -> Tagged<Name> {
  let ptr = r as *mut Name;
  lease_name(ptr);
  Tagged::from_ptr(ptr)
}

/// 地址在测试期内恒定的 Name (供 usize 地址契约与跨插入比较), 经台账登记。
fn leak_name(hash: u32) -> Tagged<Name> {
  let name = create_test_name(hash);
  lease_name_ref(Box::leak(Box::new(name)))
}

fn data_details(attributes: PropertyAttributes) -> PropertyDetails {
  PropertyDetails::for_fast(
    PropertyKind::Data,
    attributes,
    PropertyLocation::Field,
    PropertyConstness::Mutable,
    Representation::tagged(),
  )
}

/// 构造带单个末描述符 (name/details) 的目标 Map 并 leak, 等价原版
/// Map::CopyWithField(OMIT_TRANSITION) 的过渡目标形态。
fn leak_target_map(name: Tagged<Name>, details: PropertyDetails, value: i32) -> Tagged<Map> {
  let mut map = create_test_map(InstanceType::JSObject, 32);
  map.set_number_of_own_descriptors(1);
  let buf = Box::leak(Box::new(DescriptorArrayBuf::new(Tagged::NULL, 4)));
  let t_desc = buf.as_tagged();
  lease_desc(buf as *mut DescriptorArrayBuf);
  // 原版 CopyWithField 追加 name/FieldType::Any 描述符, 此处以 Smi 占位值等价
  let appended = t_desc.append(name, Smi::from_int(value).cast(), details);
  assert!(appended.is_ok());
  map.instance_descriptors = t_desc.cast();
  lease_map_ref(Box::leak(Box::new(map)))
}

/// 单次 insert: 复用测试级共享分配器 (数组内存随分配器存活, 与 objects_maps_unittest 同模式)
fn insert(allocator: &mut Vec<u64>, root: Tagged<Map>, name: Tagged<Name>, target: Tagged<Map>) {
  let res = TransitionsAccessor::insert(root, name, target, allocator);
  assert!(res.is_ok());
}

/// 原版 TEST(TransitionArray_SimpleFieldTransitions): 两个不同名单过渡,
/// 首插弱引用编码, 二插升 Full 数组, SearchTransition/键值配对逐条对齐。
#[test]
fn test_transition_array_simple_field_transitions() -> Void {
  let root = lease_map_ref(Box::leak(Box::new(create_test_map(
    InstanceType::JSObject,
    32,
  ))));
  let mut allocator = Vec::<u64>::with_capacity(4096);
  let t_name1 = leak_name(10);
  let t_name2 = leak_name(20);
  let t_map1 = leak_target_map(t_name1, data_details(PropertyAttributes::NONE), 1);
  let t_map2 = leak_target_map(t_name2, data_details(PropertyAttributes::NONE), 2);

  insert(&mut allocator, root, t_name1, t_map1);
  {
    let accessor = TransitionsAccessor::new(root);
    // 原版 CHECK(transitions.IsWeakRefEncoding())
    assert_eq!(accessor.encoding(), Encoding::WeakRef);
    // 原版 CHECK_EQ(*map1, transitions.SearchTransition(...)); 单弱过渡下 GetTarget(0)
    // 与 NumberOfTransitions()==1 由 WeakRef 编码语义承载 (crate 面无弱编码槽位枚举)
    assert_eq!(
      accessor.search_transition(t_name1, PropertyKind::Data, PropertyAttributes::NONE),
      Some(t_map1)
    );
    assert_eq!(accessor.get_target_from_weak_ref(), Some(t_map1));
  }

  insert(&mut allocator, root, t_name2, t_map2);
  {
    let accessor = TransitionsAccessor::new(root);
    // 原版 CHECK(transitions.IsFullTransitionArrayEncoding())
    assert_eq!(accessor.encoding(), Encoding::FullTransitionArray);
    let array = accessor.transitions().expect("Full 编码必有过渡数组");
    assert_eq!(array.number_of_transitions(), 2);
    assert_eq!(
      accessor.search_transition(t_name1, PropertyKind::Data, PropertyAttributes::NONE),
      Some(t_map1)
    );
    assert_eq!(
      accessor.search_transition(t_name2, PropertyKind::Data, PropertyAttributes::NONE),
      Some(t_map2)
    );
    // 原版 for 循环 GetKey/GetTarget 配对检查
    for i in 0..2usize {
      let key = array.get_key(i);
      let target = array.get_target(i);
      assert!(
        (key == t_name1 && target == Some(t_map1)) || (key == t_name2 && target == Some(t_map2))
      );
    }
  }
  OK
}

/// 原版 TEST(TransitionArray_DifferentFieldNames): PROPS_COUNT=10 个不同名过渡全命中。
#[test]
fn test_transition_array_different_field_names() -> Void {
  const PROPS_COUNT: usize = 10;
  let root = lease_map_ref(Box::leak(Box::new(create_test_map(
    InstanceType::JSObject,
    32,
  ))));

  let mut allocator = Vec::<u64>::with_capacity(4096);
  let mut names = Vec::with_capacity(PROPS_COUNT);
  let mut maps = Vec::with_capacity(PROPS_COUNT);
  for i in 0..PROPS_COUNT {
    // prop%d 内部化名以互异哈希占位 (原版依赖真实字符串哈希, 此处保持互异序)
    let name = leak_name(1000 + (i as u32) * 37);
    let map = leak_target_map(name, data_details(PropertyAttributes::NONE), i as i32);
    insert(&mut allocator, root, name, map);
    names.push(name);
    maps.push(map);
  }

  let accessor = TransitionsAccessor::new(root);
  for i in 0..PROPS_COUNT {
    // 原版 CHECK_EQ(*maps[i], transitions.SearchTransition(*names[i], kData, NONE))
    assert_eq!(
      accessor.search_transition(names[i], PropertyKind::Data, PropertyAttributes::NONE),
      Some(maps[i])
    );
  }
  let array = accessor.transitions().expect("10 过渡必为 Full 编码");
  assert_eq!(array.number_of_transitions(), PROPS_COUNT);
  // 原版 GetKey/GetTarget 交叉配对循环
  for i in 0..PROPS_COUNT {
    let key = array.get_key(i);
    let target = array.get_target(i);
    for j in 0..PROPS_COUNT {
      if key == names[j] {
        assert_eq!(target, Some(maps[j]));
        break;
      }
    }
  }
  OK
}

/// 原版 TEST(TransitionArray_SameFieldNamesDifferentAttributesSimple):
/// 同名 8 种属性组合 (ATTRS_COUNT==8), 全命中且键均为该 name。
#[test]
fn test_transition_array_same_field_names_different_attributes_simple() -> Void {
  // 原版 static_assert(ATTRS_COUNT == 8): (READ_ONLY|DONT_ENUM|DONT_DELETE)+1
  const ATTRS_COUNT: u8 = (1 | 2 | 4) + 1;
  let root = lease_map_ref(Box::leak(Box::new(create_test_map(
    InstanceType::JSObject,
    32,
  ))));
  let mut allocator = Vec::<u64>::with_capacity(4096);
  let t_name = leak_name(500);

  let mut attr_maps = Vec::with_capacity(ATTRS_COUNT as usize);
  for i in 0..ATTRS_COUNT {
    let attributes = PropertyAttributes(i);
    let map = leak_target_map(t_name, data_details(attributes), i as i32);
    insert(&mut allocator, root, t_name, map);
    attr_maps.push(map);
  }

  let accessor = TransitionsAccessor::new(root);
  for i in 0..ATTRS_COUNT {
    let attributes = PropertyAttributes(i);
    // 原版 CHECK_EQ(*attr_maps[i], transitions.SearchTransition(*name, kData, attributes))
    assert_eq!(
      accessor.search_transition(t_name, PropertyKind::Data, attributes),
      Some(attr_maps[i as usize])
    );
  }
  let array = accessor.transitions().expect("8 过渡必为 Full 编码");
  // 原版: 同键多过渡, GetKey(i) 全等于 name (不关心序)
  for i in 0..array.number_of_transitions() {
    assert_eq!(array.get_key(i), t_name);
  }
  OK
}

/// 原版 TEST(TransitionArray_SameFieldNamesDifferentAttributes):
/// 10 不同名 (NONE) + 同名 foo 8 属性组合, 共 18 过渡; 遍历条目按 key 分类校验。
#[test]
fn test_transition_array_same_field_names_different_attributes() -> Void {
  const PROPS_COUNT: usize = 10;
  const ATTRS_COUNT: u8 = (1 | 2 | 4) + 1;
  let root = lease_map_ref(Box::leak(Box::new(create_test_map(
    InstanceType::JSObject,
    32,
  ))));

  let mut allocator = Vec::<u64>::with_capacity(4096);
  let mut names = Vec::with_capacity(PROPS_COUNT);
  let mut maps = Vec::with_capacity(PROPS_COUNT);
  for i in 0..PROPS_COUNT {
    let name = leak_name(1000 + (i as u32) * 37);
    let map = leak_target_map(name, data_details(PropertyAttributes::NONE), i as i32);
    insert(&mut allocator, root, name, map);
    names.push(name);
    maps.push(map);
  }

  let t_foo = leak_name(500);
  let mut attr_maps = Vec::with_capacity(ATTRS_COUNT as usize);
  for i in 0..ATTRS_COUNT {
    let attributes = PropertyAttributes(i);
    let map = leak_target_map(t_foo, data_details(attributes), 100 + i as i32);
    insert(&mut allocator, root, t_foo, map);
    attr_maps.push(map);
  }

  let accessor = TransitionsAccessor::new(root);
  for i in 0..ATTRS_COUNT {
    let attr = PropertyAttributes(i);
    // 原版 CHECK_EQ(*attr_maps[i], transitions.SearchTransition(*name, kData, attr))
    assert_eq!(
      accessor.search_transition(t_foo, PropertyKind::Data, attr),
      Some(attr_maps[i as usize])
    );
  }
  let array = accessor.transitions().expect("18 过渡必为 Full 编码");
  // 原版 CHECK_EQ(PROPS_COUNT + ATTRS_COUNT, transitions.NumberOfTransitions())
  assert_eq!(
    array.number_of_transitions(),
    PROPS_COUNT + ATTRS_COUNT as usize
  );
  for i in 0..array.number_of_transitions() {
    let key = array.get_key(i);
    let target = array.get_target(i);
    if key == t_foo {
      // 原版: target->GetLastDescriptorDetails().attributes() 反查属性桶
      let (_kind, attributes) = TransitionsAccessor::get_target_details(key, target.unwrap());
      assert_eq!(target, Some(attr_maps[attributes.0 as usize]));
    } else {
      for j in 0..PROPS_COUNT {
        if key == names[j] {
          assert_eq!(target, Some(maps[j]));
          break;
        }
      }
    }
  }
  OK
}

/// 原版 TEST(TransitionArray_LinearSearchHandlesUnsortedArray):
/// kCount=16 (线性搜索阈内), sort(force) 后逆序交换, 仍全部可查; 重插同名必须原地覆盖。
#[test]
fn test_transition_array_linear_search_handles_unsorted_array() -> Void {
  let count = K_MAX_ELEMENTS_FOR_LINEAR_SEARCH / 2;
  let root = lease_map_ref(Box::leak(Box::new(create_test_map(
    InstanceType::JSObject,
    32,
  ))));

  let mut allocator = Vec::<u64>::with_capacity(4096);
  let mut names: Vec<Tagged<Name>> = Vec::with_capacity(count);
  for i in 0..count {
    let name = leak_name(7 + (i as u32) * 101);
    let next = leak_target_map(name, data_details(PropertyAttributes::NONE), i as i32);
    insert(&mut allocator, root, name, next);
    names.push(name);
  }

  {
    // 原版: Sort(force=true) 后按对逆序交换, 制造非升序数组
    let accessor = TransitionsAccessor::new(root);
    let array = accessor.transitions().expect("Full 编码");
    assert_eq!(array.number_of_transitions(), count);
    array.sort(true);
    let mut i = 0usize;
    let mut j = count - 1;
    while i < j {
      let ki = array.get_key(i);
      let kj = array.get_key(j);
      let ti = array.get_raw_target(i);
      let tj = array.get_raw_target(j);
      array.set_key(i, kj);
      array.set_raw_target(i, tj);
      array.set_key(j, ki);
      array.set_raw_target(j, ti);
      i += 1;
      j -= 1;
    }
    // 原版 CHECK_GE(GetKey(i)->hash(), GetKey(i+1)->hash()): 已翻转为非升序
    for k in 0..count - 1 {
      assert!(array.get_key(k).hash() >= array.get_key(k + 1).hash());
    }
  }

  {
    // 原版: 每个 name 仍必须命中 (线性搜索不得早退)
    let accessor = TransitionsAccessor::new(root);
    let array = accessor.transitions().expect("Full 编码");
    for name in &names {
      let mut insertion = None;
      let idx = array
        .search_name(*name, false, &mut insertion)
        .expect("乱序数组内线性搜索必须命中");
      assert_eq!(array.get_key(idx), *name);
    }
  }

  // 原版: 重插任一现有 key 必须原地覆盖目标而非重复追加
  for (i, name) in names.iter().enumerate() {
    let fresh = leak_target_map(
      *name,
      data_details(PropertyAttributes::NONE),
      500 + i as i32,
    );
    insert(&mut allocator, root, *name, fresh);
    let accessor = TransitionsAccessor::new(root);
    let array = accessor.transitions().expect("Full 编码");
    assert_eq!(array.number_of_transitions(), count);
    assert_eq!(
      accessor.search_transition(*name, PropertyKind::Data, PropertyAttributes::NONE),
      Some(fresh)
    );
  }
  OK
}

/// 台账转断言 (TR-ISO, transitions.cc:893 operator<<): SideStepTransition::Kind → 串。
#[test]
fn test_side_step_transition_kind_to_str() -> Void {
  // 原版 transitions.cc:889-897: kCloneObject/kObjectAssign/kObjectAssignValidityCell
  assert_eq!(
    transitions_cc_generated::side_step_transition_kind_to_str(0, 0),
    "Clone-object-IC-map"
  );
  assert_eq!(
    transitions_cc_generated::side_step_transition_kind_to_str(0, 1),
    "Object.assign-map"
  );
  assert_eq!(
    transitions_cc_generated::side_step_transition_kind_to_str(0, 2),
    "Object.assign-validity-cell"
  );
  // 越界值: 原版 ostream 默认分支不可达, Rust 侧以空串兜底
  assert_eq!(
    transitions_cc_generated::side_step_transition_kind_to_str(0, 3),
    ""
  );
  OK
}

/// 台账转断言 (TR-ISO, transitions.cc:17 GetSimpleTransition): 弱编码⇒目标, 其余⇒空。
#[test]
fn test_generated_get_simple_transition() -> Void {
  let root = lease_map_ref(Box::leak(Box::new(create_test_map(
    InstanceType::JSObject,
    32,
  ))));
  let mut allocator = Vec::<u64>::with_capacity(4096);
  // 原版 :21-24 default (kUninitialized) ⇒ 空 (usize 0)
  assert_eq!(
    transitions_cc_generated::transitions_accessor_get_simple_transition(0, root.address()),
    0
  );

  let t_name = leak_name(11);
  let t_map = leak_target_map(t_name, data_details(PropertyAttributes::NONE), 1);
  insert(&mut allocator, root, t_name, t_map);
  // 原版 kWeakRef ⇒ 弱目标
  assert_eq!(
    transitions_cc_generated::transitions_accessor_get_simple_transition(0, root.address()),
    t_map.ptr()
  );

  // 升 Full 后回到空分支
  let t_name2 = leak_name(22);
  let t_map2 = leak_target_map(t_name2, data_details(PropertyAttributes::NONE), 2);
  insert(&mut allocator, root, t_name2, t_map2);
  assert_eq!(
    transitions_cc_generated::transitions_accessor_get_simple_transition(0, root.address()),
    0
  );
  OK
}

/// 台账转断言 (TR-ISO, transitions.cc:333 CanHaveMoreTransitions): 字典 map⇒false;
/// Full 编码按 1536 上限; 其余编码⇒true。
#[test]
fn test_generated_can_have_more_transitions() -> Void {
  let root = lease_map_ref(Box::leak(Box::new(create_test_map(
    InstanceType::JSObject,
    32,
  ))));
  let mut allocator = Vec::<u64>::with_capacity(4096);
  // 原版 :335 非字典 + kUninitialized ⇒ true
  assert!(
    transitions_cc_generated::transitions_accessor_can_have_more_transitions(0, root.address())
  );

  let t_name = leak_name(31);
  let t_map = leak_target_map(t_name, data_details(PropertyAttributes::NONE), 1);
  insert(&mut allocator, root, t_name, t_map);
  // kWeakRef ⇒ true
  assert!(
    transitions_cc_generated::transitions_accessor_can_have_more_transitions(0, root.address())
  );

  let t_name2 = leak_name(32);
  let t_map2 = leak_target_map(t_name2, data_details(PropertyAttributes::NONE), 2);
  insert(&mut allocator, root, t_name2, t_map2);
  // kFullTransitionArray 且 2 < kMaxNumberOfTransitions(1536) ⇒ true
  assert!(
    transitions_cc_generated::transitions_accessor_can_have_more_transitions(0, root.address())
  );

  // 原版 :335 map->is_dictionary_map() ⇒ false (波 10 填体断言)
  let dict = Box::leak(Box::new(create_test_map(InstanceType::JSObject, 32)));
  dict.set_is_dictionary_map(true);
  let addr = Tagged::from_ptr(dict as *mut Map).address();
  lease_map(dict as *mut Map);
  assert!(!transitions_cc_generated::transitions_accessor_can_have_more_transitions(0, addr));
  OK
}

/// 台账转断言 (TR-ISO, transitions.cc:517 SetNumberOfPrototypeTransitions):
/// 原型过渡数组槽 0 写入 Smi::FromUInt(value)。
#[test]
fn test_generated_set_number_of_prototype_transitions() -> Void {
  let bytes = <Tagged<WeakFixedArray>>::size_for(4);
  let words = bytes.div_ceil(8);
  // 模拟地址契约: 先 leak 再经 *mut 暴露可写 provenance, 供后续槽位原子读写。
  let storage: &'static mut [u64] = Box::leak(vec![0u64; words].into_boxed_slice());
  let base = storage.as_mut_ptr() as usize;
  lease_words(storage as *mut [u64]);
  let header = WeakFixedArray::new(Tagged::NULL, 4);
  // SAFETY: base 指向已分配并按 u64 对齐的有效 storage 首地址 (与 concurrent 用例同构)
  unsafe {
    write(base as *mut WeakFixedArray, header);
  }
  let array: Tagged<WeakFixedArray> = unsafe { Tagged::from_raw_address(base) };

  // 原版 :520-521 set(kProtoTransitionNumberOfEntriesOffset, Smi::FromUInt(value))
  transitions_cc_generated::transition_array_set_number_of_prototype_transitions(
    array.address(),
    7,
  );
  assert_eq!(
    unsafe { array.get(K_PROTO_TRANSITION_NUMBER_OF_ENTRIES_OFFSET as u32) }.ptr(),
    Smi::from_uint(7).cast::<MaybeObject>().ptr()
  );

  transitions_cc_generated::transition_array_set_number_of_prototype_transitions(
    array.address(),
    0,
  );
  assert_eq!(
    unsafe { array.get(K_PROTO_TRANSITION_NUMBER_OF_ENTRIES_OFFSET as u32) }.ptr(),
    Smi::from_uint(0).cast::<MaybeObject>().ptr()
  );
  OK
}

/// 台账转断言 (TR-ISO, transitions.cc:554 SetMigrationTarget): 仅空过渡 + deprecated
/// map 缓存迁移目标 (Release store), 非空编码直接返回。
#[test]
fn test_generated_set_migration_target() -> Void {
  let mut allocator = Vec::<u64>::with_capacity(4096);
  // 原版 :559-561: encoding != kUninitialized ⇒ no-op; 否则 set_raw_transitions(target)
  let map = Box::leak(Box::new(create_test_map(InstanceType::JSObject, 32)));
  map.set_is_deprecated(true);
  let t_map = Tagged::from_ptr(map as *mut Map);
  let target = lease_map_ref(Box::leak(Box::new(create_test_map(
    InstanceType::JSObject,
    32,
  ))));

  transitions_cc_generated::transitions_accessor_set_migration_target(
    0,
    t_map.address(),
    target.address(),
  );
  // 原版直写 raw_transitions 槽: 读回即目标地址位
  assert_eq!(map.transitions_or_prototype_info.ptr(), target.ptr());
  lease_map(map as *mut Map);

  // 已有过渡编码 (kWeakRef ⇒ 非 Uninitialized) ⇒ 保持原槽位不变
  let root = Box::leak(Box::new(create_test_map(InstanceType::JSObject, 32)));
  root.set_is_deprecated(true);
  let t_root = Tagged::from_ptr(root as *mut Map);
  let t_name = leak_name(41);
  let t_child = leak_target_map(t_name, data_details(PropertyAttributes::NONE), 1);
  insert(&mut allocator, t_root, t_name, t_child);
  let before = root.transitions_or_prototype_info.ptr();
  transitions_cc_generated::transitions_accessor_set_migration_target(
    0,
    t_root.address(),
    target.address(),
  );
  assert_eq!(root.transitions_or_prototype_info.ptr(), before);
  lease_map(root as *mut Map);
  OK
}

/// 台账转断言 (TR-ISO, transitions.cc:572/:588 ReplaceTransitions): 数组位直接发布入
/// transitions 槽, DirectHandle 版转发同效。
#[test]
fn test_generated_replace_transitions() -> Void {
  // 准备一个真实 Full 数组 (经由两次 insert 得到)
  let src = lease_map_ref(Box::leak(Box::new(create_test_map(
    InstanceType::JSObject,
    32,
  ))));
  let mut allocator = Vec::<u64>::with_capacity(4096);
  let t_name1 = leak_name(61);
  let t_name2 = leak_name(62);
  insert(
    &mut allocator,
    src,
    t_name1,
    leak_target_map(t_name1, data_details(PropertyAttributes::NONE), 1),
  );
  insert(
    &mut allocator,
    src,
    t_name2,
    leak_target_map(t_name2, data_details(PropertyAttributes::NONE), 2),
  );
  let array = TransitionsAccessor::new(src)
    .transitions()
    .expect("Full 编码必有过渡数组");

  // :572 Tagged 版: 原版 :583 set_raw_transitions(new_transitions, kReleaseStore)
  let dst1 = Box::leak(Box::new(create_test_map(InstanceType::JSObject, 32)));
  let t_dst1 = Tagged::from_ptr(dst1 as *mut Map);
  transitions_cc_generated::transitions_accessor_replace_transitions(
    0,
    t_dst1.address(),
    array.ptr(),
  );
  assert_eq!(dst1.transitions_or_prototype_info.ptr(), array.ptr());
  let accessor1 = TransitionsAccessor::new(t_dst1);
  assert_eq!(accessor1.encoding(), Encoding::FullTransitionArray);
  assert_eq!(accessor1.transitions(), Some(array));
  lease_map(dst1 as *mut Map);

  // :588 DirectHandle 版: 解包转发 :591 ⇒ 与 Tagged 版等效
  let dst2 = Box::leak(Box::new(create_test_map(InstanceType::JSObject, 32)));
  let t_dst2 = Tagged::from_ptr(dst2 as *mut Map);
  transitions_cc_generated::transitions_accessor_replace_transitions_2(
    0,
    t_dst2.address(),
    array.ptr(),
  );
  assert_eq!(dst2.transitions_or_prototype_info.ptr(), array.ptr());
  assert_eq!(TransitionsAccessor::new(t_dst2).transitions(), Some(array));
  lease_map(dst2 as *mut Map);
  OK
}
