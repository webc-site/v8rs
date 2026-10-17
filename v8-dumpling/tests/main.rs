use std::str::FromStr;

use aok::{OK, Void};
use log::info;
use v8_dumpling::{
  CC_HELPER_FUNCTIONS, DEFAULT_DUMPLING_DEPTH, DumpFrameTarget, DumpFrameType, DumplingFlags,
  DumplingLastFrame, DumplingManager, Error, INITIAL_ARG_CAPACITY, INITIAL_REG_CAPACITY,
  INVALID_ACC, INVALID_ARG_COUNT, INVALID_BYTECODE_OFFSET, INVALID_FUNCTION_ID, INVALID_REG_COUNT,
  ObjectOrNonMaterializedObject, SimpleDumplingJSFrame, TaggedObject, TranslatedValue,
  UNINITIALIZED_SLOT_VALUE, differential_fuzzing_print, differential_fuzzing_print_object,
  print_sanitized_char, print_sanitized_str,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_dump_frame_type() -> Void {
  info!("> test_dump_frame_type");
  let types = [
    (
      DumpFrameType::InterpreterFrame,
      "---I\n",
      "InterpreterFrame",
    ),
    (DumpFrameType::SparkplugFrame, "---S\n", "SparkplugFrame"),
    (DumpFrameType::MaglevFrame, "---M\n", "MaglevFrame"),
    (DumpFrameType::TurbofanFrame, "---T\n", "TurbofanFrame"),
  ];

  for (ft, header, name) in types {
    assert_eq!(ft.header_tag(), header);
    assert_eq!(ft.to_string(), name);
    assert_eq!(
      DumpFrameType::from_str(name).map_err(|e| Error::InvalidFormat(e.to_string()))?,
      ft
    );
  }

  assert!(DumpFrameType::InterpreterFrame.is_interpreter_frame());
  assert!(!DumpFrameType::InterpreterFrame.is_sparkplug_frame());
  assert!(DumpFrameType::SparkplugFrame.is_sparkplug_frame());
  assert!(DumpFrameType::MaglevFrame.is_maglev_frame());
  assert!(DumpFrameType::TurbofanFrame.is_turbofan_frame());
  OK
}

#[test]
fn test_object_dumping_primitives() -> Void {
  info!("> test_object_dumping_primitives");

  // 1. 辅助函数全集台账
  assert_eq!(CC_HELPER_FUNCTIONS.len(), 10);
  assert!(CC_HELPER_FUNCTIONS.contains(&"PrintSanitizedChar"));
  assert!(CC_HELPER_FUNCTIONS.contains(&"HeapObjectFuzzingPrint"));

  // 2. 字符转义测试
  let mut s = String::new();
  print_sanitized_char(b'\n' as u16, &mut s);
  assert_eq!(s, "\\n");

  let mut s = String::new();
  print_sanitized_str("hello\tworld", &mut s);
  assert_eq!(s, "hello\\x09world");

  let mut s = String::new();
  print_sanitized_str("a🦀b", &mut s);
  assert_eq!(s, "a\\ud83e\\udd80b");

  // 3. TranslatedValue
  let tv = TranslatedValue::new(42);
  let obj_tv = ObjectOrNonMaterializedObject::TranslatedValue(tv);
  assert!(obj_tv.is_translated_value());
  assert!(!obj_tv.is_tagged());
  assert_eq!(
    differential_fuzzing_print(&obj_tv, DEFAULT_DUMPLING_DEPTH),
    "<non-materialized>"
  );

  // 4. TaggedObject 各种类型
  let smi_obj = ObjectOrNonMaterializedObject::smi(123);
  assert_eq!(differential_fuzzing_print(&smi_obj, 3), "123");

  let flt_obj = ObjectOrNonMaterializedObject::heap_number(12.5);
  assert_eq!(differential_fuzzing_print(&flt_obj, 3), "12.5");

  let str_obj = ObjectOrNonMaterializedObject::string("abc");
  assert_eq!(differential_fuzzing_print(&str_obj, 3), "abc");

  let fn_obj = TaggedObject::JSFunction {
    start_position: 1024,
    name: "testFn".to_string(),
  };
  assert_eq!(
    differential_fuzzing_print_object(&fn_obj, 3),
    "<JSFunction pos:1024>"
  );

  let arr_obj = TaggedObject::JSArray;
  assert_eq!(differential_fuzzing_print_object(&arr_obj, 3), "<JSArray>");

  let obj_named = TaggedObject::JSObject {
    constructor_name: Some("MyClass".to_string()),
  };
  assert_eq!(
    differential_fuzzing_print_object(&obj_named, 3),
    "<MyClass>"
  );

  let obj_anon = TaggedObject::JSObject {
    constructor_name: None,
  };
  assert_eq!(
    differential_fuzzing_print_object(&obj_anon, 3),
    "<JSObject>"
  );

  let meta_map = TaggedObject::Map {
    instance_type_or_elements_kind: "MapType".to_string(),
    is_meta: true,
  };
  assert_eq!(differential_fuzzing_print_object(&meta_map, 3), "<MetaMap>");

  let norm_map = TaggedObject::Map {
    instance_type_or_elements_kind: "PackedSmi".to_string(),
    is_meta: false,
  };
  assert_eq!(
    differential_fuzzing_print_object(&norm_map, 3),
    "<Map(PackedSmi)>"
  );

  let ctx = TaggedObject::Context {
    name: "NativeContext",
    length: 16,
  };
  assert_eq!(
    differential_fuzzing_print_object(&ctx, 3),
    "<NativeContext[16]>"
  );

  let farr = TaggedObject::FixedArray { length: 8 };
  assert_eq!(
    differential_fuzzing_print_object(&farr, 3),
    "<FixedArray[8]>"
  );

  let hole = TaggedObject::Hole("TheHole");
  assert_eq!(differential_fuzzing_print_object(&hole, 3), "<TheHole>");

  assert_eq!(
    differential_fuzzing_print_object(&TaggedObject::Undefined, 3),
    "<undefined>"
  );
  assert_eq!(
    differential_fuzzing_print_object(&TaggedObject::Null, 3),
    "<null>"
  );
  assert_eq!(
    differential_fuzzing_print_object(&TaggedObject::True, 3),
    "<true>"
  );
  assert_eq!(
    differential_fuzzing_print_object(&TaggedObject::False, 3),
    "<false>"
  );
  assert_eq!(
    differential_fuzzing_print_object(&TaggedObject::OddOddball("Special".to_string()), 3),
    "<Odd Oddball: Special>"
  );

  assert_eq!(
    differential_fuzzing_print_object(&TaggedObject::GlobalProxy, 3),
    "<global object>"
  );
  assert_eq!(
    differential_fuzzing_print_object(&TaggedObject::AccessorInfo, 3),
    "<AccessorInfo>"
  );
  assert_eq!(
    differential_fuzzing_print_object(&TaggedObject::AccessorPair, 3),
    "<AccessorPair>"
  );
  assert_eq!(
    differential_fuzzing_print_object(&TaggedObject::JSProxy, 3),
    "<JSProxy>"
  );
  assert_eq!(
    differential_fuzzing_print_object(&TaggedObject::BigInt("9999999999".to_string()), 3),
    "<BigIntBase 9999999999>"
  );
  assert_eq!(
    differential_fuzzing_print_object(&TaggedObject::ClassPositions, 3),
    "<ClassPositions>"
  );
  assert_eq!(
    differential_fuzzing_print_object(&TaggedObject::Symbol("sym".to_string()), 3),
    "<Symbol(sym)>"
  );
  assert_eq!(
    differential_fuzzing_print_object(&TaggedObject::ClassBoilerplate, 3),
    "<ClassBoilerplate>"
  );
  assert_eq!(
    differential_fuzzing_print_object(&TaggedObject::Script, 3),
    "<Script>"
  );
  assert_eq!(
    differential_fuzzing_print_object(&TaggedObject::FeedbackVector { length: 4 }, 3),
    "<FeedbackVector[4]>"
  );
  OK
}

#[test]
fn test_dumpling_manager_diff_logic() -> Void {
  info!("> test_dumpling_manager_diff_logic");
  let flags = DumplingFlags::default();
  let mut mgr = DumplingManager::new(flags);

  // 1. 初始状态
  let default_frame = DumplingLastFrame::default();
  assert_eq!(default_frame.bytecode_offset, INVALID_BYTECODE_OFFSET);
  assert_eq!(mgr.last_frame().bytecode_offset, INVALID_BYTECODE_OFFSET);
  assert_eq!(mgr.last_frame().acc, INVALID_ACC);
  assert_eq!(mgr.last_frame().arg_count, INVALID_ARG_COUNT);
  assert_eq!(mgr.last_frame().reg_count, INVALID_REG_COUNT);
  assert_eq!(mgr.last_frame().function_id, INVALID_FUNCTION_ID);
  assert_eq!(mgr.last_frame().args.len(), INITIAL_ARG_CAPACITY);
  assert_eq!(mgr.last_frame().regs.len(), INITIAL_REG_CAPACITY);
  assert_eq!(mgr.last_frame().args[0], UNINITIALIZED_SLOT_VALUE);

  // 2. 差分转储: 首次写入产生 Some, 相同写入产生 None
  assert_eq!(mgr.dump_bytecode_offset(10), Some("10".to_string()));
  assert_eq!(mgr.dump_bytecode_offset(10), None);
  assert_eq!(mgr.dump_bytecode_offset(14), Some("14".to_string()));

  assert_eq!(mgr.dump_function_id(42), Some("42".to_string()));
  assert_eq!(mgr.dump_function_id(42), None);

  assert_eq!(mgr.dump_acc("val_a".to_string()), Some("val_a".to_string()));
  assert_eq!(mgr.dump_acc("val_a".to_string()), None);
  assert_eq!(mgr.dump_acc("val_b".to_string()), Some("val_b".to_string()));

  assert_eq!(mgr.dump_arg_count(2), Some("2".to_string()));
  assert_eq!(mgr.dump_arg_count(2), None);

  assert_eq!(mgr.dump_reg_count(4), Some("4".to_string()));
  assert_eq!(mgr.dump_reg_count(4), None);

  assert_eq!(
    mgr.dump_arg(0, "param0".to_string()),
    Some("param0".to_string())
  );
  assert_eq!(mgr.dump_arg(0, "param0".to_string()), None);

  assert_eq!(
    mgr.dump_reg(0, "reg0".to_string()),
    Some("reg0".to_string())
  );
  assert_eq!(mgr.dump_reg(0, "reg0".to_string()), None);

  // 3. 超出初始容量的扩展槽位
  assert_eq!(
    mgr.dump_reg(200, "reg200".to_string()),
    Some("reg200".to_string())
  );
  assert!(mgr.last_frame().regs.len() > 200);

  // 4. 重置上一帧缓存
  mgr.reset_last_frame();
  assert_eq!(mgr.last_frame().bytecode_offset, INVALID_BYTECODE_OFFSET);
  assert_eq!(mgr.last_frame().acc, INVALID_ACC);
  OK
}

#[test]
fn test_dumpling_positions_serialization() -> Void {
  info!("> test_dumpling_positions_serialization");
  let flags = DumplingFlags::default();
  let mut mgr = DumplingManager::new(flags);

  mgr.record_dump_position(101, 10);
  mgr.record_dump_position(101, 20);
  mgr.record_dump_position(101, 10); // 去重
  mgr.record_dump_position(202, 30);

  let serialized = mgr.write_dump_positions_to_string();
  let expected = "101 10 20\n202 30\n";
  assert_eq!(serialized, expected);

  let mut mgr2 = DumplingManager::new(DumplingFlags::default());
  mgr2.load_dump_positions_from_str(&serialized)?;
  assert_eq!(mgr2.dump_positions().len(), 2);
  assert!(
    mgr2
      .dump_positions()
      .get(&101)
      .map(|s| s.contains(&10) && s.contains(&20))
      .unwrap_or(false)
  );
  assert!(
    mgr2
      .dump_positions()
      .get(&202)
      .map(|s| s.contains(&30))
      .unwrap_or(false)
  );
  OK
}

#[test]
fn test_dumpling_manager_do_print_lifecycle() -> Void {
  info!("> test_dumpling_manager_do_print_lifecycle");
  let flags = DumplingFlags {
    generate_dump_positions: true,
    ..Default::default()
  };
  let mut mgr = DumplingManager::new(flags);

  let mut frame = SimpleDumplingJSFrame::new(
    vec![
      ObjectOrNonMaterializedObject::smi(10),
      ObjectOrNonMaterializedObject::string("r1"),
      ObjectOrNonMaterializedObject::translated(99), // accumulator (reg_idx = 2)
    ],
    vec![ObjectOrNonMaterializedObject::smi(1)],
    ObjectOrNonMaterializedObject::string("mainFunc"),
  );

  // 第 1 帧: 首次转储，全部输出
  mgr.print_dumped_frame(
    &mut frame,
    DumpFrameTarget::new(
      100, // function_id
      0,   // bytecode_offset
      DumpFrameType::InterpreterFrame,
      1, // param_count
      2, // register_count
    ),
  )?;

  let out1 = mgr.get_output().to_string();
  assert!(out1.starts_with("---I\n"));
  assert!(out1.contains("b:0\n"));
  assert!(out1.contains("f:100\n"));
  assert!(out1.contains("x:<non-materialized>\n"));
  assert!(out1.contains("n:1\n"));
  assert!(out1.contains("m:2\n"));
  assert!(out1.contains("a0:1\n"));
  assert!(out1.contains("r0:10\n"));
  assert!(out1.contains("r1:r1\n"));

  // 第 2 帧: 仅 bytecode_offset 变动为 4，其余不变，差分输出
  mgr.print_dumped_frame(
    &mut frame,
    DumpFrameTarget::new(
      100, // function_id 相同
      4,   // bytecode_offset 改变
      DumpFrameType::InterpreterFrame,
      1,
      2,
    ),
  )?;

  let out2 = mgr.get_output();
  let frame2_part = &out2[out1.len()..];
  assert!(frame2_part.starts_with("---I\n"));
  assert!(frame2_part.contains("b:4\n"));
  // 未变动项不重复输出
  assert!(!frame2_part.contains("f:100\n"));
  assert!(!frame2_part.contains("x:<non-materialized>\n"));
  assert!(!frame2_part.contains("n:1\n"));
  assert!(!frame2_part.contains("m:2\n"));
  assert!(!frame2_part.contains("a0:1\n"));
  assert!(!frame2_part.contains("r0:10\n"));

  // 验证位置记录已生效
  assert!(
    mgr
      .dump_positions()
      .get(&100)
      .map(|s| s.contains(&0) && s.contains(&4))
      .unwrap_or(false)
  );

  // 周期准备: 清空状态
  mgr.prepare_for_next_reprl_cycle();
  assert_eq!(mgr.get_output(), "");
  assert!(mgr.dump_positions().is_empty());
  assert_eq!(mgr.last_frame().bytecode_offset, INVALID_BYTECODE_OFFSET);
  OK
}

#[test]
fn test_isolate_disabled_and_filtering() -> Void {
  info!("> test_isolate_disabled_and_filtering");
  let flags = DumplingFlags {
    load_dump_positions: true,
    ..Default::default()
  };
  let mut mgr = DumplingManager::new(flags);

  // 1. 白名单过滤: 仅允许 (200, 16)
  mgr.record_dump_position(200, 16);

  let mut frame = SimpleDumplingJSFrame::new(
    vec![ObjectOrNonMaterializedObject::smi(0)],
    vec![],
    ObjectOrNonMaterializedObject::string("func"),
  );

  // 尝试在非白名单位置 (200, 10) 转储，应被跳过（输出仅有帧头）
  mgr.print_dumped_frame(
    &mut frame,
    DumpFrameTarget::new(200, 10, DumpFrameType::SparkplugFrame, 0, 0),
  )?;
  let out = mgr.get_output();
  assert_eq!(out, "---S\n");

  // 2. 隔离区全局禁用
  mgr.set_isolate_dump_disabled();
  assert!(mgr.is_isolate_dump_disabled());
  assert!(!mgr.is_dumping_enabled());

  let err = mgr.print_dumped_frame(
    &mut frame,
    DumpFrameTarget::new(200, 16, DumpFrameType::SparkplugFrame, 0, 0),
  );
  assert!(matches!(err, Err(Error::DumpingDisabled)));
  OK
}
