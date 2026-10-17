//! 12.6 延期台账恢复 (轮10 W10OC): Map 语义面 (get_visitor_id/visitor_id_to_str/
//! visitor_id_from_u8/extended_map_size_for_kind/位域/记账族) 已 1:1 落地, 解冻入编译。

// 迁移规约:
//   map.h/map-inl.h 位域与字段语义自证 (原版无独立 cctest/test-maps 文件,
//     实测该路径不存在; map.h:1259 static_assert 与 map-inl.h 各访问器的
//     DCHECK 不变量固化为断言)
//   map.cc:69-559 GetVisitorId 锚点子集 (string 表示法分派 + 关键类型映射;
//     transition 深层用例推迟, 归 v8-objects-maps)
//   map.cc:2742-2757 ToString(VisitorId)
//   map-inl.h:400-632 实例尺寸/属性记账/slack 族
//   map.h:443-479 slack tracking 常量
use aok::{OK, Void};
use v8_common::ExtendedMapKind;
use v8_objects_core::{
  Error,
  instance_type::InstanceType as Ty,
  map::{
    ExtendedMap, GENEROUS_ALLOCATION_COUNT, INVALID_ENUM_CACHE_SENTINEL, JS_OBJECT_FIELDS_ADDED,
    K_INSTANCE_TYPE_OFFSET, K_MAP_OFFSET, K_PROTOTYPE_OFFSET, K_SIZE, Map, NO_SLACK_TRACKING,
    ObjectFields, SLACK_TRACKING_COUNTER_END, SLACK_TRACKING_COUNTER_START, VisitorId,
    extended_map_size_for_kind, get_visitor_id, object_fields_from, visitor_id_from_u8,
    visitor_id_to_str,
  },
  tagged::Tagged,
};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

fn map_of(ty: Ty, instance_size_bytes: usize) -> Map {
  Map::new(Tagged::NULL, ty, instance_size_bytes)
}

/// 规约 (map-inl.h:400-419): instance_size 以 word 编码存储, 字节语义换算;
/// 0 = kVariableSizeSentinel。
#[test]
fn instance_size_word_encoding() -> Void {
  let m = map_of(Ty::HeapNumber, 16);
  assert_eq!(m.instance_size_in_words(), 4);
  assert_eq!(m.instance_size(), 16);

  let mut v = map_of(Ty::SeqOneByteString, 0);
  assert_eq!(v.instance_size(), 0); // 变长哨兵
  v.set_instance_size(44);
  assert_eq!(v.instance_size(), 44);
  assert_eq!(v.instance_size_in_words(), 11);
  v.set_instance_size_in_words(7);
  assert_eq!(v.instance_size(), 28);
  OK
}

/// 规约 (map.h 布局注释 + v8-internal.h:1013):
/// 压缩指针默认配置 (kTaggedSize=4) 的偏移锚点。
#[test]
fn layout_offset_anchors() -> Void {
  assert_eq!(K_MAP_OFFSET, 0);
  assert_eq!(K_INSTANCE_TYPE_OFFSET, 8); // 1*kApiTaggedSize + kApiInt32Size
  assert_eq!(K_PROTOTYPE_OFFSET, 16);
  assert_eq!(K_SIZE, 40);
  OK
}

/// 规约 (map.h:338-356 Bits1): 7 个布尔位互不串扰。
#[test]
fn bits1_flags() -> Void {
  let mut m = map_of(Ty::JSObject, 40);
  m.set_is_callable(true);
  m.set_is_undetectable(true);
  m.set_has_named_interceptor(true);
  m.set_is_access_check_needed(true);
  m.set_has_indexed_interceptor(true);
  m.set_is_constructor(true);
  m.set_is_extended_map(true);
  assert!(m.is_callable() && m.is_undetectable());
  assert!(m.has_named_interceptor() && m.is_access_check_needed());
  assert!(m.has_indexed_interceptor() && m.is_constructor() && m.is_extended_map());
  assert_eq!(m.bit_field, 0b0111_1111);

  m.set_is_callable(false);
  m.set_is_access_check_needed(false);
  assert!(!m.is_callable() && !m.is_access_check_needed());
  assert!(m.is_undetectable());
  assert_eq!(m.bit_field, 0b0111_0110); // 保留 bit1/2/4/5/6
  OK
}

/// 规约 (map.h:373-385 Bits2): new_target/immutable 布尔 + elements_kind 6 bit 原始存取。
#[test]
fn bits2_elements_kind() -> Void {
  let mut m = map_of(Ty::JSArray, 40);
  m.set_new_target_is_base(true);
  m.set_is_immutable_prototype(true);
  assert_eq!(m.bit_field2, 0b0000_0011);
  m.set_elements_kind(0b101_101); // 45
  assert_eq!(m.elements_kind(), 45);
  assert_eq!(m.bit_field2, 0b1011_0111); // 布尔 2 位 + kind<<2
  m.set_elements_kind(0);
  assert_eq!(m.elements_kind(), 0);
  assert!(m.new_target_is_base() && m.is_immutable_prototype());
  OK
}

/// 规约 (map.h:391-421 Bits3): 枚举长度/描述符数/独立布尔位/构造计数器布局。
#[test]
fn bits3_layout() -> Void {
  let mut m = map_of(Ty::JSObject, 40);
  m.set_enum_length(1023);
  assert_eq!(m.enum_length(), INVALID_ENUM_CACHE_SENTINEL);
  m.set_number_of_own_descriptors(1020);
  assert_eq!(m.number_of_own_descriptors(), 1020);
  // 两个字段不串扰: enum 0..9 | nod 10..19
  assert_eq!(m.bit_field3 & 0x3FF, 1023);
  assert_eq!((m.bit_field3 >> 10) & 0x3FF, 1020);

  m.set_is_prototype_map(true);
  m.set_owns_descriptors(false);
  m.set_is_in_retained_map_list(true);
  m.set_may_have_interesting_properties(true);
  assert!(m.is_prototype_map() && m.is_in_retained_map_list());
  assert!(m.may_have_interesting_properties());
  assert!(!m.owns_descriptors());
  assert!(m.is_abandoned_prototype_map()); // proto && !owns

  m.set_construction_counter(7);
  assert_eq!(m.construction_counter(), 7);
  assert_eq!(m.bit_field3 >> 29, 7);
  OK
}

/// 规约 (map-inl.h:866-875): set_is_dictionary_map 与 is_unstable 联动。
#[test]
fn dictionary_map_couples_unstable() -> Void {
  let mut m = map_of(Ty::JSObject, 40);
  assert!(!m.is_dictionary_map() && m.is_stable());
  m.set_is_dictionary_map(true);
  assert!(m.is_dictionary_map());
  assert!(!m.is_stable()); // 联动置 unstable
  m.set_is_dictionary_map(false);
  assert!(m.is_stable());
  m.mark_unstable();
  assert!(!m.is_stable());
  OK
}

/// 规约 (map-inl.h:421-483): inobject 属性/构造函数序号双义槽。
#[test]
fn inobject_properties_slot() -> Void {
  let mut m = map_of(Ty::JSObject, 8 * 4);
  m.set_in_object_properties_start_in_words(3);
  assert_eq!(m.get_in_object_properties_start_in_words(), 3);
  assert_eq!(m.get_in_object_properties(), 5);
  assert!(m.is_field_in_object(4));
  assert!(!m.is_field_in_object(5));
  assert_eq!(m.get_in_object_property_offset(0), 12);
  assert_eq!(m.get_in_object_property_offset(4), 28);

  // 原始类型 map: 同槽位存构造函数序号
  let mut p = map_of(Ty::HeapNumber, 16);
  p.set_constructor_function_index(9);
  assert_eq!(p.get_constructor_function_index(), 9);
  assert_eq!(Map::NO_CONSTRUCTOR_FUNCTION_INDEX, 0);
  OK
}

/// 规约 (map-inl.h:507-632): used/unused 双义记账 + 新增字段推进。
#[test]
fn unused_property_fields_accounting() -> Void {
  // JSObject 形状: 8 words, 属性起始 3 → 5 个内置槽
  let mut m = map_of(Ty::JSObject, 8 * 4);
  m.set_in_object_properties_start_in_words(3);

  // 全内置: 记 "已用偏移" (>= kFieldsAdded=3)
  m.set_in_object_unused_property_fields(5);
  assert_eq!(m.unused_property_fields(), 5);
  assert_eq!(m.unused_in_object_properties(), 5);
  assert!(!m.has_out_of_object_properties());
  assert_eq!(m.used_instance_size(), 3 * 4); // start=3 words

  // 逐字段消耗 5 个内置槽
  for expect in (1..=5).rev() {
    m.account_added_property_field();
    assert_eq!(m.unused_property_fields(), expect - 1);
  }
  assert!(!m.has_out_of_object_properties()); // value=8 仍为内置编码 (全部用满)
  // 内置满后再加 → 转外置: 属性数组 slack = kFieldsAdded-1 = 2
  m.account_added_property_field();
  assert!(m.has_out_of_object_properties());
  assert_eq!(m.unused_property_fields(), 2);
  m.account_added_property_field();
  assert_eq!(m.unused_property_fields(), 1);
  m.account_added_property_field();
  assert_eq!(m.unused_property_fields(), 0);
  // 外置耗尽 → slack 回绕到 kFieldsAdded-1
  m.account_added_property_field();
  assert_eq!(m.unused_property_fields(), 2);
  assert_eq!(m.used_instance_size(), 8 * 4); // 外置期 = 全实例大小

  // 显式外置 slack 设置
  m.set_out_of_object_unused_property_fields(2);
  assert_eq!(m.unused_property_fields(), 2);
  assert_eq!(m.unused_in_object_properties(), 0);
  OK
}

/// 规约 (map-inl.h:585-600): 记账复制与实例大小差校正。
#[test]
fn copy_unused_property_fields_family() -> Void {
  let mut src = map_of(Ty::JSObject, 8 * 4);
  src.set_in_object_properties_start_in_words(3);
  src.set_in_object_unused_property_fields(2);

  let mut dst = map_of(Ty::JSObject, 8 * 4);
  dst.set_in_object_properties_start_in_words(3);
  dst.copy_unused_property_fields(&src);
  assert_eq!(dst.unused_property_fields(), 2);

  // 校正: 目标更大 (10 words), 未用槽数保持 (value += size 差)
  let mut bigger = map_of(Ty::JSObject, 10 * 4);
  bigger.set_in_object_properties_start_in_words(3);
  bigger.copy_unused_property_fields_adjusted_for_instance_size(&src);
  assert_eq!(bigger.unused_property_fields(), 2); // 10 - (6+2)
  OK
}

/// 规约 (map.h:443-479 + map-inl.h:1247 + map.cc:2662): slack tracking 常量与启停。
#[test]
fn slack_tracking() -> Void {
  assert_eq!(SLACK_TRACKING_COUNTER_START, 7);
  assert_eq!(SLACK_TRACKING_COUNTER_END, 1);
  assert_eq!(NO_SLACK_TRACKING, 0);
  assert_eq!(GENEROUS_ALLOCATION_COUNT, 7);
  assert_eq!(JS_OBJECT_FIELDS_ADDED, 3);

  let mut m = map_of(Ty::JSObject, 8 * 4);
  m.set_in_object_properties_start_in_words(3);
  m.set_in_object_unused_property_fields(0);
  m.start_inobject_slack_tracking(); // 无未用槽 → 不启动
  assert!(!m.is_inobject_slack_tracking_in_progress());

  m.set_in_object_unused_property_fields(4);
  m.start_inobject_slack_tracking();
  assert!(m.is_inobject_slack_tracking_in_progress());
  assert_eq!(m.construction_counter(), 7);
  assert_eq!(m.instance_size_from_slack(2), 8 * 4 - 2 * 4);

  assert_eq!(Map::slack_for_array_size(2, 10), 1);
  assert_eq!(Map::slack_for_array_size(8, 10), 2);
  assert_eq!(Map::slack_for_array_size(40, 50), 10); // min(10, 40/4)
  OK
}

/// 规约 (map.h:1051-1055 + map-inl.h:391-398): visitor id 类别边界与存取往返。
#[test]
fn visitor_id_encoding() -> Void {
  // data-only 列表在前: HeapNumber 为 data-only, Map 含指针
  assert_eq!(
    object_fields_from(VisitorId::VisitHeapNumber),
    ObjectFields::DataOnly
  );
  assert_eq!(
    object_fields_from(VisitorId::VisitMap),
    ObjectFields::MaybePointers
  );
  assert!((VisitorId::VisitHeapNumber as u8) < (VisitorId::DataOnlyVisitorIdCount as u8));
  assert!((VisitorId::VisitMap as u8) > (VisitorId::DataOnlyVisitorIdCount as u8));

  let mut m = map_of(Ty::HeapNumber, 16);
  m.set_visitor_id(VisitorId::VisitHeapNumber);
  assert_eq!(m.visitor_id(), Some(VisitorId::VisitHeapNumber));
  assert!(visitor_id_from_u8(255).is_none());

  // ToString 锚点 (map.cc:2742)
  assert_eq!(
    visitor_id_to_str(VisitorId::VisitHeapNumber),
    Some("HeapNumber")
  );
  assert_eq!(visitor_id_to_str(VisitorId::VisitMap), Some("Map"));
  assert_eq!(
    visitor_id_to_str(VisitorId::DataOnlyVisitorIdCount),
    Some("DataOnlyVisitorIdCount")
  );
  OK
}

/// 规约 (map.h:62-127 POINTER_VISITOR_ID_LIST 宏序): wasm 组在 WeakCell 之前、
/// SIMPLE_HEAP_OBJECT_LIST1 (ArrayList...) 在 WeakCell 之后 — 判别值顺序锚点
/// (visitor_id 为堆存储 u8, snapshot 兼容前提)。
#[test]
fn visitor_id_discriminant_order() -> Void {
  use VisitorId::*;
  let order = [
    VisitThinString,
    VisitTransitionArray,
    VisitWasmArray,
    VisitWasmTypeInfo,
    VisitWeakCell,
    VisitArrayList,
    VisitByteArray,
    VisitWeakHomomorphicFixedArray,
    VisitBytecodeArray,
    VisitCode,
    VisitWasmSuspenderObject,
    VisitorIdCount,
  ];
  for w in order.windows(2) {
    assert!((w[0] as u8) < (w[1] as u8), "{:?} < {:?}", w[0], w[1]);
  }
  // 相邻步距: wasm 组紧接 WeakCell, WeakCell 紧接 ArrayList
  assert_eq!(VisitWasmTypeInfo as u8 + 1, VisitWeakCell as u8);
  assert_eq!(VisitWeakCell as u8 + 1, VisitArrayList as u8);
  // data-only 段不含 wasm 之外的指针项: Hole 是最后一个 data-only 成员
  assert!((VisitHole as u8) < (DataOnlyVisitorIdCount as u8));
  assert_eq!(DataOnlyVisitorIdCount as u8, VisitWasmNull as u8 + 1);
  OK
}

/// 规约 (map-inl.h:906-918 CanTransition 三谓词合取):
/// 只读空间包装对象 (JSExternalObject/JSMessageObject) 与共享空间 JSObject
/// (JSSharedArray/JSSharedStruct) 不可 transition。
#[test]
fn can_transition_exclusions() -> Void {
  assert!(map_of(Ty::JSObject, 40).can_transition());
  assert!(map_of(Ty::JSArray, 40).can_transition());
  // IsMaybeReadOnlyJSObject: 跨 API 传数据包装对象
  assert!(!map_of(Ty::JSExternalObject, 40).can_transition());
  assert!(!map_of(Ty::JSMessageObject, 40).can_transition());
  // IsAlwaysSharedSpaceJSObject: 共享空间固定形状
  assert!(!map_of(Ty::JSSharedArray, 40).can_transition());
  assert!(!map_of(Ty::JSSharedStruct, 40).can_transition());
  // 非 JSObject 族
  assert!(!map_of(Ty::HeapNumber, 16).can_transition());
  assert!(!map_of(Ty::Map, 40).can_transition());
  OK
}

/// 规约 (map.cc:69-559 GetVisitorId): 关键类型映射锚点。
#[test]
fn get_visitor_id_anchors() -> Void {
  let cases = [
    (Ty::HeapNumber, VisitorId::VisitHeapNumber),
    (Ty::Map, VisitorId::VisitMap),
    (Ty::Oddball, VisitorId::VisitOddball),
    (Ty::Symbol, VisitorId::VisitSymbol),
    (Ty::BigIntBase, VisitorId::VisitBigInt),
    (Ty::JSArray, VisitorId::VisitJSObjectFast),
    (Ty::JSProxy, VisitorId::VisitJSProxy),
    (Ty::JSDate, VisitorId::VisitJSDate),
    (Ty::JSApiObject, VisitorId::VisitJSApiObject),
    (Ty::ByteArray, VisitorId::VisitByteArray),
    (Ty::SwissNameDictionary, VisitorId::VisitSwissNameDictionary),
  ];
  for (ty, want) in cases {
    assert_eq!(get_visitor_id(&map_of(ty, 16)).unwrap(), want, "{ty:?}");
  }
  OK
}

/// 规约 (map.cc:74-100): string 表示法分派 (seq/cons/sliced/external/thin × 编码)。
#[test]
fn get_visitor_id_string_dispatch() -> Void {
  let cases = [
    (Ty::SeqOneByteString, VisitorId::VisitSeqOneByteString),
    (Ty::SeqTwoByteString, VisitorId::VisitSeqTwoByteString),
    (Ty::ConsOneByteString, VisitorId::VisitShortcutCandidate), // 非内部化 cons
    (Ty::ConsTwoByteString, VisitorId::VisitShortcutCandidate),
    (Ty::SlicedOneByteString, VisitorId::VisitSlicedString),
    (Ty::ExternalOneByteString, VisitorId::VisitExternalString),
    (Ty::ThinOneByteString, VisitorId::VisitThinString),
  ];
  for (ty, want) in cases {
    assert_eq!(get_visitor_id(&map_of(ty, 0)).unwrap(), want, "{ty:?}");
  }
  OK
}

/// 规约 (map.cc:297-301, 556-558): JS_PROMISE 与抽象占位类型 → Err
/// (embedder 字段判定归 v8-objects-js-core; 占位类型原版 FATAL)。
#[test]
fn get_visitor_id_defers() -> Void {
  assert!(matches!(
    get_visitor_id(&map_of(Ty::JSPromise, 40)),
    Err(Error::Unsupported(_))
  ));
  assert!(matches!(
    get_visitor_id(&map_of(Ty::Dictionary, 16)),
    Err(Error::Unsupported(_))
  ));
  OK
}

/// 规约 (map.h:498-535 + map-inl.h:281-283): 可 transition 的快 elements 类型白名单。
#[test]
fn can_have_fast_transitionable_elements_kind() -> Void {
  for ty in [Ty::JSArray, Ty::JSPrimitiveWrapper, Ty::JSArgumentsObject] {
    assert!(
      Map::can_have_fast_transitionable_elements_kind_of(ty),
      "{ty:?}"
    );
  }
  assert!(!Map::can_have_fast_transitionable_elements_kind_of(
    Ty::JSObject
  ));
  assert!(map_of(Ty::JSObject, 40).can_transition());
  assert!(!map_of(Ty::HeapNumber, 16).can_transition());
  OK
}

/// 规约 (map.h:1264-1297 ExtendedMap): BitsEx 类别/尺寸编码。
#[test]
fn extended_map_bits_ex() -> Void {
  let mut em = ExtendedMap {
    map: map_of(Ty::JSObject, 40),
    bit_field_ex: 0,
    padding: [0; 3],
  };
  em.set_map_kind_and_size(ExtendedMapKind::JSInterceptorMap, 48);
  assert_eq!(em.map_kind(), ExtendedMapKind::JSInterceptorMap);
  assert_eq!(em.map_size_in_words(), 12);
  assert_eq!(em.map_size(), 48);
  assert_eq!(em.bit_field_ex(), 12 << 3); // map_kind(0) | size(12)<<3
  assert_eq!(ExtendedMap::K_MINIMUM_SIZE, 44);
  // JSInterceptorMap 具体尺寸归 v8-objects-maps 确认 (登记)
  assert!(extended_map_size_for_kind(ExtendedMapKind::JSInterceptorMap).is_err());
  OK
}
