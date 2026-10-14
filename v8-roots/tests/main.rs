use aok::{OK, Void};
use log::info;
use v8_objects_core::{
  heap_number::HeapNumber,
  map::Map,
  tagged::{Object, Tagged},
};
use v8_roots::{Error, ReadOnlyRoots, RootIndex, RootsTable};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_root_index_boundaries() -> Void {
  info!("验证 RootIndex 分区边界与计数契约");

  // 1. 总量与分区计数契约断言
  assert_eq!(RootIndex::K_ENTRIES_COUNT, 1157);
  assert_eq!(RootIndex::K_READ_ONLY_ROOTS_COUNT, 1044);
  assert_eq!(RootIndex::K_IMMORTAL_IMMOVABLE_ROOTS_COUNT, 1123);
  assert_eq!(RootIndex::K_MUTABLE_ROOTS_COUNT, 105);
  assert_eq!(RootIndex::K_SMI_ROOTS_COUNT, 8);
  assert_eq!(RootsTable::K_ENTRIES_COUNT, 1157);
  assert_eq!(RootsTable::K_READ_ONLY_ROOTS_COUNT, 1044);
  assert_eq!(RootsTable::K_IMMORTAL_IMMOVABLE_ROOTS_COUNT, 1123);
  assert_eq!(RootsTable::K_MUTABLE_ROOTS_COUNT, 105);
  assert_eq!(RootsTable::K_SMI_ROOTS_COUNT, 8);
  assert_eq!(ReadOnlyRoots::K_ENTRIES_COUNT, 1044);
  assert_eq!(
    RootIndex::K_READ_ONLY_ROOTS_COUNT
      + RootIndex::K_MUTABLE_ROOTS_COUNT
      + RootIndex::K_SMI_ROOTS_COUNT,
    RootIndex::K_ENTRIES_COUNT
  );

  // 2. 连续性与区间首尾断言
  assert_eq!(RootIndex::K_FIRST_ROOT.as_usize(), 0);
  assert_eq!(RootIndex::K_LAST_ROOT.as_usize(), 1156);

  assert_eq!(RootIndex::K_FIRST_READ_ONLY_ROOT.as_usize(), 0);
  assert_eq!(RootIndex::K_LAST_READ_ONLY_ROOT.as_usize(), 1043);

  assert_eq!(RootIndex::K_FIRST_STRONG_ROOT.as_usize(), 1044);
  assert_eq!(RootIndex::K_LAST_STRONG_ROOT.as_usize(), 1148);

  assert_eq!(RootIndex::K_FIRST_IMMORTAL_IMMOVABLE_ROOT.as_usize(), 0);
  assert_eq!(RootIndex::K_LAST_IMMORTAL_IMMOVABLE_ROOT.as_usize(), 1122);

  assert_eq!(RootIndex::K_FIRST_SMI_ROOT.as_usize(), 1149);
  assert_eq!(RootIndex::K_LAST_SMI_ROOT.as_usize(), 1156);

  // 3. 边界处 is_read_only 判定
  assert!(RootIndex::CppGCManagedBaseMap.is_read_only());
  assert!(!RootIndex::ArgumentsIteratorAccessor.is_read_only());

  // 4. 边界处 is_immortal_immovable 判定
  assert!(RootIndex::EmptyWasmDispatchTable.is_immortal_immovable());
  assert!(!RootIndex::SmiStringCache.is_immortal_immovable());

  // 5. 整数转换与有效性校验
  assert_eq!(RootIndex::from_usize(0), Some(RootIndex::UndefinedValue));
  assert_eq!(
    RootIndex::from_usize(1156),
    Some(RootIndex::InterpreterEntryReturnPCOffset)
  );
  assert_eq!(RootIndex::from_usize(1157), None);

  let err = RootIndex::try_from(9999usize);
  assert_eq!(err, Err(Error::InvalidRootIndex(9999)));

  let ok_idx = RootIndex::try_from(0usize)?;
  assert_eq!(ok_idx, RootIndex::UndefinedValue);

  OK
}

#[test]
fn test_roots_table_initialization_and_access() -> Void {
  info!("验证 RootsTable 内存管理与槽位访问");

  let mut table = RootsTable::new();

  // 初始全 NULL，require 返回 UninitializedRoot 错误
  assert!(!table.is_initialized(RootIndex::UndefinedValue));
  assert_eq!(table.get(RootIndex::UndefinedValue), Tagged::NULL);
  assert_eq!(table[RootIndex::UndefinedValue], Tagged::NULL);
  assert_eq!(
    table.require(RootIndex::UndefinedValue),
    Err(Error::UninitializedRoot("undefined_value"))
  );

  // 模拟填入堆对象指针 (最低位为 1 的 HeapObject Tag)
  let dummy_ptr = Tagged::<Object>::new(0x10001);
  table.set(RootIndex::UndefinedValue, dummy_ptr)?;
  assert!(table.is_initialized(RootIndex::UndefinedValue));
  assert_eq!(table.get(RootIndex::UndefinedValue), dummy_ptr);
  assert_eq!(table[RootIndex::UndefinedValue], dummy_ptr);
  assert_eq!(table.require(RootIndex::UndefinedValue)?, dummy_ptr);

  // 测试 get_at 越界防护
  assert_eq!(table.get_at(0), Some(dummy_ptr));
  assert_eq!(table.get_at(RootIndex::K_ENTRIES_COUNT), None);

  // 测试 slot 及其位置判断 is_root_handle_location
  let slot_undefined = table.slot(RootIndex::UndefinedValue);
  let slot_smi = table.slot(RootIndex::LastScriptId);
  assert_eq!(
    table.is_root_handle_location(slot_undefined),
    Some(RootIndex::UndefinedValue)
  );
  assert_eq!(
    table.is_root_handle_location(slot_smi),
    Some(RootIndex::LastScriptId)
  );

  let external_ptr = 0x50000usize as *const Tagged<Object>;
  assert_eq!(table.is_root_handle_location(external_ptr), None);

  // 测试各根分区切片长度与互斥覆盖
  assert_eq!(
    table.read_only_slice().len(),
    RootsTable::K_READ_ONLY_ROOTS_COUNT
  );
  assert_eq!(
    table.mutable_slice().len(),
    RootsTable::K_MUTABLE_ROOTS_COUNT
  );
  assert_eq!(table.smi_slice().len(), RootsTable::K_SMI_ROOTS_COUNT);
  assert_eq!(
    table.strong_or_read_only_slice().len(),
    RootsTable::K_READ_ONLY_ROOTS_COUNT + RootsTable::K_MUTABLE_ROOTS_COUNT
  );
  assert_eq!(table.as_slice().len(), RootsTable::K_ENTRIES_COUNT);

  OK
}

#[test]
fn test_roots_table_write_protection() -> Void {
  info!("验证 RootsTable 只读根写保护机制");

  let mut table = RootsTable::new();
  let dummy_val1 = Tagged::<Object>::new(0x20001);
  let dummy_val2 = Tagged::<Object>::new(0x30001);

  // 未开启写保护时，可正常写入只读根
  table.set(RootIndex::TrueValue, dummy_val1)?;
  assert_eq!(table.get(RootIndex::TrueValue), dummy_val1);

  // 开启只读保护
  table.write_protect_read_only_roots();
  assert!(table.is_read_only_write_protected());

  // 再次写入只读根触发拒绝，返回零拷贝静态字符串错误
  let res = table.set(RootIndex::TrueValue, dummy_val2);
  assert_eq!(res, Err(Error::ReadOnlyRootMutation("true_value")));
  assert_eq!(table.get(RootIndex::TrueValue), dummy_val1);

  // 可变根不受只读保护影响
  table.set(RootIndex::LastScriptId, dummy_val2)?;
  assert_eq!(table.get(RootIndex::LastScriptId), dummy_val2);

  // 内部辅助 unchecked_set 仍可穿透写入
  table.unchecked_set(RootIndex::TrueValue, dummy_val2);
  assert_eq!(table.get(RootIndex::TrueValue), dummy_val2);

  // 解除写保护
  table.unprotect_read_only_roots();
  assert!(!table.is_read_only_write_protected());
  table.set(RootIndex::TrueValue, dummy_val1)?;
  assert_eq!(table.get(RootIndex::TrueValue), dummy_val1);

  OK
}

#[test]
fn test_read_only_roots_getters_and_singletons() -> Void {
  info!("验证 ReadOnlyRoots 强类型单例获取与字符索引");

  let mut table = RootsTable::new();

  let p_hole = Tagged::<Object>::new(0x101);
  let p_undefined = Tagged::<Object>::new(0x201);
  let p_null = Tagged::<Object>::new(0x301);
  let p_true = Tagged::<Object>::new(0x401);
  let p_false = Tagged::<Object>::new(0x501);
  let p_empty_str = Tagged::<Object>::new(0x601);
  let p_empty_arr = Tagged::<Object>::new(0x701);
  let p_empty_bytes = Tagged::<Object>::new(0x801);
  let p_meta_map = Tagged::<Object>::new(0x901);
  let p_fixed_map = Tagged::<Object>::new(0xa01);

  table.unchecked_set(RootIndex::TheHoleValue, p_hole);
  table.unchecked_set(RootIndex::UndefinedValue, p_undefined);
  table.unchecked_set(RootIndex::NullValue, p_null);
  table.unchecked_set(RootIndex::TrueValue, p_true);
  table.unchecked_set(RootIndex::FalseValue, p_false);
  table.unchecked_set(RootIndex::EmptyString, p_empty_str);
  table.unchecked_set(RootIndex::EmptyFixedArray, p_empty_arr);
  table.unchecked_set(RootIndex::EmptyByteArray, p_empty_bytes);
  table.unchecked_set(RootIndex::MetaMap, p_meta_map);
  table.unchecked_set(RootIndex::FixedArrayMap, p_fixed_map);

  let p_char_a = Tagged::<Object>::new(0xb01);
  table.unchecked_set(RootIndex::UppercaseAString, p_char_a);

  let ro = table.read_only_roots();

  assert_eq!(ro.len(), 1044);
  assert!(!ro.is_empty());
  assert_eq!(ro.as_slice().len(), 1044);
  assert_eq!(ro.require(RootIndex::TheHoleValue)?, p_hole);
  assert_eq!(
    ro.require(RootIndex::InfinityValue),
    Err(Error::UninitializedRoot("infinity_value"))
  );

  assert_eq!(ro.the_hole_value().ptr(), p_hole.ptr());
  assert_eq!(ro.undefined_value().ptr(), p_undefined.ptr());
  assert_eq!(ro.null_value().ptr(), p_null.ptr());
  assert_eq!(ro.true_value().ptr(), p_true.ptr());
  assert_eq!(ro.false_value().ptr(), p_false.ptr());
  assert_eq!(ro.boolean_value(true).ptr(), p_true.ptr());
  assert_eq!(ro.boolean_value(false).ptr(), p_false.ptr());
  assert_eq!(ro.empty_string().ptr(), p_empty_str.ptr());
  assert_eq!(ro.empty_fixed_array().ptr(), p_empty_arr.ptr());
  assert_eq!(ro.empty_byte_array().ptr(), p_empty_bytes.ptr());
  assert_eq!(ro.meta_map().ptr(), p_meta_map.ptr());
  assert_eq!(ro.fixed_array_map().ptr(), p_fixed_map.ptr());

  // 单字符字符串索引映射 (测试 RootIndex 与 RootsTable 双侧)
  assert_eq!(
    RootIndex::single_character_string_index(0),
    RootIndex::AsciiNulString
  );
  assert_eq!(
    RootsTable::single_character_string_index(0),
    RootIndex::AsciiNulString
  );
  assert_eq!(
    RootIndex::single_character_string_index(b'A'),
    RootIndex::UppercaseAString
  );
  assert_eq!(
    RootsTable::single_character_string_index(b'A'),
    RootIndex::UppercaseAString
  );
  assert_eq!(
    RootIndex::single_character_string_index(b'a'),
    RootIndex::AString
  );
  assert_eq!(
    RootIndex::single_character_string_index(255),
    RootIndex::Latin1FfString
  );

  assert_eq!(ro.single_character_string(b'A').ptr(), p_char_a.ptr());

  OK
}

#[test]
fn test_protector_and_trusted_map_ranges() -> Void {
  info!("验证 Protector 与受信任 Map 连续性范围");

  // 1. 受信任 Map 范围检测
  assert!(RootIndex::AtomRegExpDataMap.is_in_trusted_object_map_list());
  assert!(RootsTable::is_in_trusted_object_map_list(
    RootIndex::AtomRegExpDataMap
  ));
  assert!(RootIndex::UncompiledDataWithPreparseDataAndJobMap.is_in_trusted_object_map_list());
  assert!(RootsTable::is_in_trusted_object_map_list(
    RootIndex::UncompiledDataWithPreparseDataAndJobMap
  ));
  assert!(!RootIndex::UndefinedValue.is_in_trusted_object_map_list());
  assert!(!RootsTable::is_in_trusted_object_map_list(
    RootIndex::UndefinedValue
  ));

  assert!(RootIndex::BytecodeArrayMap.is_in_serializable_trusted_object_map_list());
  assert!(RootsTable::is_in_serializable_trusted_object_map_list(
    RootIndex::BytecodeArrayMap
  ));
  assert!(
    RootIndex::UncompiledDataWithPreparseDataAndJobMap.is_in_serializable_trusted_object_map_list()
  );
  assert!(!RootIndex::AtomRegExpDataMap.is_in_serializable_trusted_object_map_list());

  // 2. Protector 名称范围检测
  let mut table = RootsTable::new();
  let first_prot = 0x10000;
  let last_prot = 0x11000;
  table.unchecked_set(
    RootIndex::K_FIRST_NAME_FOR_PROTECTOR,
    Tagged::<Object>::new(first_prot),
  );
  table.unchecked_set(
    RootIndex::K_LAST_NAME_FOR_PROTECTOR,
    Tagged::<Object>::new(last_prot),
  );

  let ro = table.read_only_roots();
  assert!(ro.is_name_for_protector(Tagged::<Object>::new(first_prot)));
  assert!(ro.is_name_for_protector(Tagged::<Object>::new(0x10800)));
  assert!(ro.is_name_for_protector(Tagged::<Object>::new(last_prot)));
  assert!(!ro.is_name_for_protector(Tagged::<Object>::new(0xfff0)));
  assert!(!ro.is_name_for_protector(Tagged::<Object>::new(0x12000)));

  OK
}

#[test]
fn test_root_names_parity() -> Void {
  info!("验证 ROOT_NAMES 与 V8 原版 1:1 对齐");

  // 1. 首尾及核心根名称字面量对齐
  assert_eq!(RootIndex::UndefinedValue.name(), "undefined_value");
  assert_eq!(RootIndex::TheHoleValue.name(), "the_hole_value");
  assert_eq!(RootIndex::NullValue.name(), "null_value");
  assert_eq!(RootIndex::TrueValue.name(), "true_value");
  assert_eq!(RootIndex::FalseValue.name(), "false_value");
  assert_eq!(RootIndex::EmptyString.name(), "empty_string");
  assert_eq!(RootIndex::ByteArrayMap.name(), "byte_array_map");
  assert_eq!(RootIndex::FixedArrayMap.name(), "fixed_array_map");
  assert_eq!(RootIndex::LastScriptId.name(), "last_script_id");
  assert_eq!(
    RootIndex::InterpreterEntryReturnPCOffset.name(),
    "interpreter_entry_return_pc_offset"
  );

  // 2. 全量 1157 项非空且首尾匹配
  assert_eq!(v8_roots::ROOT_NAMES.len(), RootIndex::K_ENTRIES_COUNT);
  for (idx, name) in v8_roots::ROOT_NAMES.iter().enumerate() {
    assert!(!name.is_empty(), "根名称不能为空: 索引 {idx}");
    assert_eq!(
      RootIndex::from_usize(idx).map(|r| r.name()),
      Some(*name),
      "RootIndex::name 与 ROOT_NAMES 不一致: 索引 {idx}"
    );
  }

  OK
}

#[test]
fn test_heap_number_and_sfi_constants() -> Void {
  info!("验证 HeapNumber 与 Builtin SFI 常量与分区方法");

  // 1. HeapNumber 分区常数与断言 (对齐 v8/test/unittests/objects/roots-unittest.cc)
  assert_eq!(RootIndex::K_FIRST_HEAP_NUMBER_ROOT, RootIndex::NanValue);
  assert_eq!(
    RootIndex::K_LAST_HEAP_NUMBER_ROOT,
    RootIndex::SmiMaxValuePlusOne
  );
  assert!(RootIndex::NanValue.is_heap_number());
  assert!(RootIndex::SmiMaxValuePlusOne.is_heap_number());
  assert!(!RootIndex::UndefinedValue.is_heap_number());
  assert!(!RootIndex::EmptyFixedArray.is_heap_number());

  // 2. Builtins With SFI 分区常数
  assert_eq!(RootIndex::K_BUILTINS_WITH_SFI_ROOTS_COUNT, 28);
  assert_eq!(
    RootIndex::K_FIRST_BUILTIN_WITH_SFI_ROOT,
    RootIndex::ProxyRevokeSharedFun
  );
  assert_eq!(
    RootIndex::K_LAST_BUILTIN_WITH_SFI_ROOT,
    RootIndex::AsyncIteratorPrototypeAsyncDisposeResolveClosureSharedFun
  );

  // 3. JSReceiver Map 首项
  assert_eq!(
    RootIndex::K_FIRST_JS_RECEIVER_MAP_ROOT,
    RootIndex::JSSharedArrayMap
  );

  // 4. is_strong 与 is_smi 状态判定
  assert!(RootIndex::ArgumentsIteratorAccessor.is_strong());
  assert!(RootIndex::ErrorStackSetterSharedFun.is_strong());
  assert!(!RootIndex::UndefinedValue.is_strong());
  assert!(!RootIndex::LastScriptId.is_strong());

  assert!(RootIndex::LastScriptId.is_smi());
  assert!(RootIndex::InterpreterEntryReturnPCOffset.is_smi());
  assert!(!RootIndex::UndefinedValue.is_smi());
  assert!(!RootIndex::ArgumentsIteratorAccessor.is_smi());

  OK
}

#[test]
fn test_one_pointer_filler_and_find_heap_number() -> Void {
  info!("验证 one_pointer_filler_map_word 与 find_heap_number 行为");

  let mut table = RootsTable::new();
  let fake_filler_map_ptr = 0x40001usize;
  table.unchecked_set(
    RootIndex::OnePointerFillerMap,
    Tagged::<Object>::new(fake_filler_map_ptr),
  );

  let ro = table.read_only_roots();
  let mw = ro.one_pointer_filler_map_word();
  assert_eq!(mw.to_map().ptr(), fake_filler_map_ptr);

  // 测试 find_heap_number
  let meta_map = Tagged::<Map>::new(0x10001);
  let mut heap_num_storage = HeapNumber::new(meta_map, 42.5);
  let heap_num_ptr = Tagged::<Object>::new((&mut heap_num_storage as *mut _ as usize) | 1);

  table.unchecked_set(RootIndex::NanValue, heap_num_ptr);
  let found = unsafe { table.find_heap_number(42.5) };
  assert!(found.is_some());
  assert_eq!(found.map(|t| t.ptr()), Some(heap_num_ptr.ptr()));

  let ro2 = table.read_only_roots();
  let found_ro = unsafe { ro2.find_heap_number(42.5) };
  assert_eq!(found_ro.map(|t| t.ptr()), Some(heap_num_ptr.ptr()));

  let not_found = unsafe { table.find_heap_number(999.0) };
  assert!(not_found.is_none());

  OK
}
