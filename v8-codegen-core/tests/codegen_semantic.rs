use std::panic::{AssertUnwindSafe, catch_unwind};

use aok::{OK, Void};
use v8_codegen_core::{
  AliasingKind, AllocationSiteOverrideMode, AssemblerBase, AssemblerOptions, CatchPrediction,
  CodeCommentsIterator, CodeCommentsWriter, CodeDesc, CodeFactory, CodeReference,
  CodeReferenceKind, ElementsKind, EncodingMode, Error, ExternalReference, FixedSizeSignature,
  HandlerTable, JumpTableInfoIterator, JumpTableInfoWriter, K_NO_HANDLER_FOUND,
  MachineRepresentation, RegisterConfiguration, Signature, SignatureBuilder,
};

const INT32_BYTES: usize = 4;

fn encode_handler(offset: i32, was_used: bool, prediction: CatchPrediction) -> i32 {
  let p = prediction as i32;
  (offset << 4) | ((was_used as i32) << 3) | p
}

fn build_range_table(entries: &[[i32; 4]]) -> Vec<i32> {
  let mut v = Vec::new();
  for e in entries {
    v.extend_from_slice(e);
  }
  v
}

#[test]
fn handler_table_range_round_trip() -> Void {
  let entries = [
    [
      10i32,
      20i32,
      encode_handler(40, false, CatchPrediction::Uncaught),
      -1,
    ],
    [
      21,
      30,
      encode_handler(80, true, CatchPrediction::Promise),
      99,
    ],
    [
      31,
      40,
      encode_handler(120, false, CatchPrediction::Caught),
      7,
    ],
    [
      41,
      50,
      encode_handler(0, false, CatchPrediction::Uncaught),
      0,
    ],
    [0, 0, 0, 0],
  ];

  let mut buf = build_range_table(&entries);
  let size = (buf.len() * INT32_BYTES) as u32;
  let mut ht = HandlerTable::from_raw(buf.as_mut_ptr() as usize, size, EncodingMode::RangeBased);
  assert_eq!(ht.num_entries(), 5);

  assert_eq!(ht.get_range_start(0), 10);
  assert_eq!(ht.get_range_end(0), 20);
  assert_eq!(ht.get_handler_offset(0), 40);
  assert_eq!(ht.get_range_data(0), -1);
  assert_eq!(ht.get_range_prediction(0), CatchPrediction::Uncaught);
  assert!(!ht.handler_was_used(0));

  assert_eq!(ht.get_range_start(1), 21);
  assert_eq!(ht.get_range_data(1), 99);
  assert_eq!(ht.get_range_prediction(1), CatchPrediction::Promise);
  assert!(ht.handler_was_used(1));

  // 修改表项并回读
  ht.set_range(2, 55, 66, 42, 42, CatchPrediction::AsyncAwait);
  assert_eq!(ht.get_range_start(2), 55);
  assert_eq!(ht.get_range_end(2), 66);
  assert_eq!(ht.get_handler_offset(2), 42);
  assert_eq!(ht.get_range_data(2), 42);
  assert_eq!(ht.get_range_prediction(2), CatchPrediction::AsyncAwait);

  ht.set_handler_was_used(3, true);
  assert!(ht.handler_was_used(3));

  // 越界访问抛出 panic
  let res = catch_unwind(AssertUnwindSafe(|| {
    let _ = ht.get_range_start(99);
  }));
  assert!(res.is_err());

  OK
}

#[test]
fn handler_table_range_lookup() -> Void {
  let entries = [
    [
      10,
      20,
      encode_handler(0, false, CatchPrediction::Uncaught),
      0,
    ],
    [
      21,
      30,
      encode_handler(0, false, CatchPrediction::Uncaught),
      0,
    ],
    [
      31,
      40,
      encode_handler(0, false, CatchPrediction::Uncaught),
      0,
    ],
  ];
  let buf = build_range_table(&entries);
  let size = (buf.len() * INT32_BYTES) as u32;
  let ht = HandlerTable::from_raw(buf.as_ptr() as usize, size, EncodingMode::RangeBased);

  assert_eq!(ht.lookup_handler_index_for_range(10), 0);
  assert_eq!(ht.lookup_handler_index_for_range(21), 1);
  assert_eq!(ht.lookup_handler_index_for_range(39), 2);
  assert_eq!(ht.lookup_handler_index_for_range(5), K_NO_HANDLER_FOUND);
  assert_eq!(ht.lookup_handler_index_for_range(100), K_NO_HANDLER_FOUND);

  OK
}

#[test]
fn handler_table_return_round_trip() -> Void {
  let mut buf = [0i32; 8];
  let size = (buf.len() * INT32_BYTES) as u32;
  let mut ht = HandlerTable::from_raw(
    buf.as_mut_ptr() as usize,
    size,
    EncodingMode::ReturnAddressBased,
  );
  assert_eq!(ht.num_entries(), 4);

  ht.set_return_entry(0, 10, 100);
  ht.set_return_entry(3, 40, 400);

  assert_eq!(ht.get_return_offset(0), 10);
  assert_eq!(ht.get_return_offset(3), 40);
  assert_eq!(ht.get_return_handler(0), 100);
  assert_eq!(ht.get_return_handler(3), 400);
  assert_eq!(ht.get_return_handler(2), 0);
  assert_eq!(ht.get_return_prediction(0), CatchPrediction::Uncaught);

  OK
}

#[test]
fn handler_table_return_lookup() -> Void {
  let mut buf = [0i32; 8];
  let size = (buf.len() * INT32_BYTES) as u32;
  let mut ht = HandlerTable::from_raw(
    buf.as_mut_ptr() as usize,
    size,
    EncodingMode::ReturnAddressBased,
  );

  ht.set_return_entry(0, 10, 100);
  ht.set_return_entry(1, 20, 200);
  ht.set_return_entry(2, 30, 300);

  assert_eq!(ht.lookup_return(10), 100);
  assert_eq!(ht.lookup_return(20), 200);
  assert_eq!(ht.lookup_return(30), 300);
  assert_eq!(ht.lookup_return(5), K_NO_HANDLER_FOUND);
  assert_eq!(ht.lookup_return(99), K_NO_HANDLER_FOUND);

  OK
}

#[test]
fn code_comments_writer_and_iterator() -> Void {
  let mut writer = CodeCommentsWriter::default();
  writer.add(100, "hello world".to_string());
  let buf = writer.emit_to_vec();

  let it = unsafe { CodeCommentsIterator::new(buf.as_ptr(), buf.len() as u32) };
  assert!(it.has_current());
  assert_eq!(it.get_comment(), "hello world");
  assert_eq!(it.get_comment_size(), 12);
  assert_eq!(it.get_pc_offset(), 100);

  OK
}

#[test]
fn jump_table_info_iterator() -> Void {
  let mut writer = JumpTableInfoWriter::default();
  writer.add(10, 0x1000);
  writer.add(20, 0x2000);
  let buf = writer.emit_to_vec();

  let mut assm = AssemblerBase::new(AssemblerOptions::default());
  writer.emit(&mut assm)?;
  assert_eq!(assm.buffer_slice(), buf.as_slice());

  let mut it = JumpTableInfoIterator::new(buf.as_ptr() as usize, buf.len() as u32);
  assert!(it.has_current());
  assert_eq!(it.get_pc_offset(), 10);
  assert_eq!(it.get_target(), 0x1000);

  it.next();
  assert!(it.has_current());
  assert_eq!(it.get_pc_offset(), 20);
  assert_eq!(it.get_target(), 0x2000);

  it.next();
  assert!(!it.has_current());

  OK
}

#[test]
fn signature_builder_round_trip() -> Void {
  let mut sig = Signature::from_parts(0, 0, Vec::new());
  let mut builder = SignatureBuilder::new(&mut sig, 1, 2);
  builder.add_return(MachineRepresentation::Float64);
  builder.add_param(MachineRepresentation::Word32);
  builder.add_param(MachineRepresentation::SandboxedPointer);
  let s = builder.get();

  assert_eq!(s.return_count(), 1);
  assert_eq!(s.parameter_count(), 2);
  let reps = s.all();
  assert_eq!(reps[0], MachineRepresentation::Float64);
  assert_eq!(reps[1], MachineRepresentation::Word32);
  assert_eq!(reps[2], MachineRepresentation::SandboxedPointer);
  assert_eq!(*s.get_return(0), MachineRepresentation::Float64);
  assert_eq!(*s.get_param(0), MachineRepresentation::Word32);
  assert_eq!(*s.get_param(1), MachineRepresentation::SandboxedPointer);

  OK
}

#[test]
fn code_reference_code_desc_dispatch() -> Void {
  let mut cd = CodeDesc::new();
  cd.buffer = 0x1000usize as *mut u8;
  cd.buffer_size = 100;
  cd.instr_size = 50;
  cd.reloc_offset = 60;
  cd.reloc_size = 40;
  cd.code_comments_offset = 50;
  cd.code_comments_size = 10;
  cd.jump_table_info_offset = 55;
  cd.jump_table_info_size = 5;

  let r = CodeReference::from_code_desc_ref(&cd);
  assert_eq!(r.kind(), CodeReferenceKind::CodeDesc);
  assert_eq!(r.instruction_start(), 0x1000);
  assert_eq!(r.instruction_end(), 0x1000 + 50);
  assert_eq!(r.instruction_size(), 50);
  assert_eq!(r.relocation_start(), 0x1000 + 60);
  assert_eq!(r.relocation_end(), 0x1000 + 100);
  assert_eq!(r.relocation_size(), 40);
  assert_eq!(r.code_comments(), 0x1000 + 50);
  assert_eq!(r.code_comments_size(), 10);
  assert_eq!(r.jump_table_info(), 0x1000 + 55);
  assert_eq!(r.jump_table_info_size(), 5);

  OK
}

#[test]
fn register_config_get_aliases_combine() -> Void {
  let rc = RegisterConfiguration::new_for_test_with_aliasing(AliasingKind::Combine);
  let mut alias_base = -1;

  let count = rc.get_aliases(
    MachineRepresentation::Float64 as u8,
    2,
    MachineRepresentation::Float32 as u8,
    &mut alias_base,
  );
  assert_eq!(count, 2);
  assert_eq!(alias_base, 4);
  assert!(rc.are_aliases(
    MachineRepresentation::Float64 as u8,
    2,
    MachineRepresentation::Float32 as u8,
    4,
  ));
  assert!(rc.are_aliases(
    MachineRepresentation::Float64 as u8,
    2,
    MachineRepresentation::Float32 as u8,
    5,
  ));
  assert!(!rc.are_aliases(
    MachineRepresentation::Float64 as u8,
    2,
    MachineRepresentation::Float32 as u8,
    6,
  ));

  OK
}

#[test]
fn code_factory_dispatch() -> Void {
  let c1 = CodeFactory::runtime_c_entry(0, 1, false);
  assert_eq!(c1.descriptor(), 0);

  let c2 = CodeFactory::call(0, 0);
  assert_ne!(c2.code(), 0);

  let c3 =
    CodeFactory::array_no_argument_constructor(0, 0, AllocationSiteOverrideMode::DontOverride);
  assert_ne!(c3.code(), 0);

  let c4 = CodeFactory::array_no_argument_constructor(
    0,
    0,
    AllocationSiteOverrideMode::DisableAllocationSites,
  );
  assert_ne!(c4.code(), 0);
  assert_ne!(c3.code(), c4.code());

  let c5 =
    CodeFactory::array_single_argument_constructor(0, 0, AllocationSiteOverrideMode::DontOverride);
  assert_ne!(c5.code(), 0);

  // 测试 call_function 派发分支
  let cf_null = CodeFactory::call_function(0, 0);
  let cf_not_null = CodeFactory::call_function(0, 1);
  let cf_any = CodeFactory::call_function(0, 2);
  assert_ne!(cf_null.code(), cf_not_null.code());
  assert_ne!(cf_not_null.code(), cf_any.code());

  // 测试 load_global_ic / optimized 分支
  let ic_inside = CodeFactory::load_global_ic(0, 0);
  let ic_not_inside = CodeFactory::load_global_ic(0, 1);
  assert_ne!(ic_inside.code(), ic_not_inside.code());

  let ic_opt_inside = CodeFactory::load_global_ic_in_optimized_code(0, 0);
  let ic_opt_not_inside = CodeFactory::load_global_ic_in_optimized_code(0, 1);
  assert_ne!(ic_opt_inside.code(), ic_opt_not_inside.code());

  // 测试 string_add 标志分支
  let sa_none = CodeFactory::string_add(0, 0);
  let sa_left = CodeFactory::string_add(0, 1);
  let sa_right = CodeFactory::string_add(0, 2);
  assert_ne!(sa_none.code(), sa_left.code());
  assert_ne!(sa_left.code(), sa_right.code());

  // 测试 fast_new_function_context (Eval=2 vs Function=5)
  let fnc_eval = CodeFactory::fast_new_function_context(0, 2);
  let fnc_fn = CodeFactory::fast_new_function_context(0, 5);
  assert_ne!(fnc_eval.code(), fnc_fn.code());

  // 测试 c_entry 分支 (wasm, stack, register, builtin exit)
  let ce_wasm = CodeFactory::c_entry(0, 1, 0, false, true);
  let ce_stack_no_exit = CodeFactory::c_entry(0, 1, 0, false, false);
  let ce_stack_exit = CodeFactory::c_entry(0, 1, 0, true, false);
  let ce_reg_no_exit = CodeFactory::c_entry(0, 1, 1, false, false);
  assert_ne!(ce_wasm.code(), ce_stack_no_exit.code());
  assert_ne!(ce_stack_no_exit.code(), ce_stack_exit.code());
  assert_ne!(ce_stack_no_exit.code(), ce_reg_no_exit.code());

  OK
}

#[test]
fn handler_table_lazy_deopt_and_formatting() -> Void {
  use v8_codegen_core::K_LAZY_DEOPT;

  let mut buf = [0i32; 8];
  let size = (buf.len() * INT32_BYTES) as u32;
  let mut ht = HandlerTable::from_raw(buf.as_mut_ptr() as usize, size, EncodingMode::RangeBased);

  // K_LAZY_DEOPT 边界测试 (28 位全 1: 0x0fff_ffff)
  ht.set_range_handler(0, K_LAZY_DEOPT, CatchPrediction::AsyncAwait);
  assert_eq!(ht.get_range_handler(0), K_LAZY_DEOPT);
  assert_eq!(ht.get_range_prediction(0), CatchPrediction::AsyncAwait);

  // 格式化输出测试
  let range_str = ht.format_range();
  assert!(range_str.contains("ASYNC_AWAIT"));

  // Return 表格式化测试
  let mut ret_buf = [0i32; 4];
  let ret_size = (ret_buf.len() * INT32_BYTES) as u32;
  let mut ret_ht = HandlerTable::from_raw(
    ret_buf.as_mut_ptr() as usize,
    ret_size,
    EncodingMode::ReturnAddressBased,
  );
  ret_ht.set_return_entry(0, 0x1234, 0x5678);
  let ret_str = ret_ht.format_return();
  assert!(ret_str.contains("1234"));

  OK
}

#[test]
fn handler_table_emit_with_assembler() -> Void {
  use v8_codegen_core::{AssemblerBase, AssemblerOptions};

  let mut masm = AssemblerBase::new(AssemblerOptions::default());
  let start = HandlerTable::emit_return_table_start(&mut masm)?;
  assert_eq!(start, 0);

  HandlerTable::emit_return_entry(&mut masm, 0x10, 0x20)?;
  assert_eq!(masm.pc_offset(), 8);

  // 解析生成的字节
  let bytes = masm.buffer_slice();
  assert_eq!(bytes.len(), 8);
  let offset = i32::from_le_bytes(bytes[0..4].try_into().unwrap());
  let bitfield = i32::from_le_bytes(bytes[4..8].try_into().unwrap());
  assert_eq!(offset, 0x10);
  assert_eq!(bitfield, 0x20 << 4);

  OK
}

#[test]
fn external_reference_semantics() -> Void {
  let r1 = ExternalReference::from_raw_address(0x1234);
  assert_eq!(r1.raw(), 0x1234);
  assert!(!r1.is_null());

  let r2 = ExternalReference::null();
  assert!(r2.is_null());

  let r3 = ExternalReference::from_symbol("test_symbol", 0x5678);
  assert_eq!(r3.name(), Some("test_symbol"));
  assert_eq!(r3.raw(), 0x5678);

  OK
}

#[test]
fn handler_table_safety_and_mode_checks() -> Void {
  use v8_codegen_core::{AssemblerBase, AssemblerOptions, K_LAZY_DEOPT};

  // 1. 测试在 ReturnAddressBased 模式下误调用 set_range_* 会安全 panic
  let mut ret_buf = [0i32; 8];
  let ret_size = (ret_buf.len() * INT32_BYTES) as u32;
  let mut ret_ht = HandlerTable::from_raw(
    ret_buf.as_mut_ptr() as usize,
    ret_size,
    EncodingMode::ReturnAddressBased,
  );
  let res_mode = catch_unwind(AssertUnwindSafe(|| {
    ret_ht.set_range_start(0, 100);
  }));
  assert!(res_mode.is_err());

  // 2. 测试 set_return_handler 对越界 handler offset 的断言校验
  let res_overflow = catch_unwind(AssertUnwindSafe(|| {
    ret_ht.set_return_handler(0, K_LAZY_DEOPT + 1);
  }));
  assert!(res_overflow.is_err());

  let res_neg = catch_unwind(AssertUnwindSafe(|| {
    ret_ht.set_return_handler(0, -1);
  }));
  assert!(res_neg.is_err());

  // 3. 测试 emit_return_entry 对越界 handler offset 返回 Err(Error::InvalidHandlerTable)
  let mut masm = AssemblerBase::new(AssemblerOptions::default());
  let emit_err = HandlerTable::emit_return_entry(&mut masm, 0, K_LAZY_DEOPT + 1);
  assert!(matches!(emit_err, Err(Error::InvalidHandlerTable(_))));

  // 4. 测试 ElementsKind 与默认函数重载
  let c_packed = CodeFactory::array_no_argument_constructor(
    0,
    ElementsKind::Packed as i32,
    AllocationSiteOverrideMode::DisableAllocationSites,
  );
  let c_holey = CodeFactory::array_no_argument_constructor(
    0,
    ElementsKind::Holey as i32,
    AllocationSiteOverrideMode::DisableAllocationSites,
  );
  assert_ne!(c_packed.code(), c_holey.code());

  let c_single_packed = CodeFactory::array_single_argument_constructor(
    0,
    ElementsKind::PackedDouble as i32,
    AllocationSiteOverrideMode::DisableAllocationSites,
  );
  assert_ne!(c_single_packed.code(), 0);

  // 5. 测试默认参数调用辅助函数
  let call_def = CodeFactory::call_default(0);
  let call_any = CodeFactory::call(0, 2);
  assert_eq!(call_def.code(), call_any.code());

  let call_fn_def = CodeFactory::call_function_default(0);
  let call_fn_any = CodeFactory::call_function(0, 2);
  assert_eq!(call_fn_def.code(), call_fn_any.code());

  let sa_def = CodeFactory::string_add_default(0);
  let sa_none = CodeFactory::string_add(0, 0);
  assert_eq!(sa_def.code(), sa_none.code());

  let centry_def = CodeFactory::c_entry_default(0);
  let centry_manual = CodeFactory::c_entry(0, 1, 0, false, false);
  assert_eq!(centry_def.code(), centry_manual.code());

  let rcentry_def = CodeFactory::runtime_c_entry_default(0);
  let rcentry_manual = CodeFactory::runtime_c_entry(0, 1, false);
  assert_eq!(rcentry_def.code(), rcentry_manual.code());

  OK
}

#[test]
fn fixed_size_signature_semantics() -> Void {
  let sig: FixedSizeSignature<MachineRepresentation, 1, 2> = FixedSizeSignature::new();
  assert_eq!(sig.return_count(), 1);
  assert_eq!(sig.parameter_count(), 2);
  assert_eq!(sig.all().len(), 3);
  assert_eq!(*sig.get_return(0), MachineRepresentation::None);
  assert_eq!(*sig.get_param(0), MachineRepresentation::None);
  assert_eq!(*sig.get_param(1), MachineRepresentation::None);

  OK
}
