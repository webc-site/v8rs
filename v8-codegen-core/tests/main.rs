use aok::{OK, Void};
use log::info;
use v8_codegen_core::{
  AssemblerBase, AssemblerOptions, BufferGrowthStrategy, CallInterfaceDescriptor,
  CallInterfaceDescriptorData, CallInterfaceDescriptorFlags, Distance, Error, ExternalReference,
  ExternalReferenceType, K_MAXIMAL_BUFFER_SIZE, K_MINIMAL_BUFFER_SIZE, Label, RecordingMode,
  RegistersInit, RelocInfo, RelocInfoWriter, RelocIterator, RelocMode, SafepointEntry,
  SafepointTable, SafepointTableBuilder, SourcePositionTableBuilder, SourcePositionTableIterator,
  StackArgumentOrder, all_real_modes_mask, apply_mask, code_target_mask, compute_new_buffer_size,
  embedded_object_mode_mask, js_dispatch_handle_mode_mask, mode_mask, post_codegen_relocation_mask,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_reloc_info_modes_and_predicates() -> Void {
  info!("测试 RelocInfo 模式与谓词");

  let info_code = RelocInfo::new(0x1000, RelocMode::CodeTarget);
  assert!(info_code.is_real_reloc_mode());
  assert!(info_code.is_code_target());
  assert!(info_code.is_code_target_mode());
  assert!(!info_code.is_embedded_object());

  let info_obj = RelocInfo::new(0x1010, RelocMode::FullEmbeddedObject);
  assert!(info_obj.is_real_reloc_mode());
  assert!(info_obj.is_embedded_object());
  assert!(info_obj.is_full_embedded_object());
  assert!(!info_obj.is_compressed_embedded_object());

  let info_comp = RelocInfo::new(0x1020, RelocMode::CompressedEmbeddedObject);
  assert!(info_comp.is_embedded_object());
  assert!(info_comp.is_compressed_embedded_object());

  let info_wasm = RelocInfo::new(0x1030, RelocMode::WasmCall);
  assert!(info_wasm.is_wasm_call());
  assert!(info_wasm.is_shareable_reloc_mode());

  let info_ext = RelocInfo::new(0x1040, RelocMode::ExternalReference);
  assert!(info_ext.is_external_reference());

  let info_deopt = RelocInfo::new(0x1050, RelocMode::DeoptScriptOffset);
  assert!(info_deopt.is_deopt());

  // 掩码一致性校验
  assert_eq!(mode_mask(RelocMode::CodeTarget), 1 << 1);
  assert_eq!(
    code_target_mask(),
    (1 << (RelocMode::CodeTarget as u32)) | (1 << (RelocMode::RelativeCodeTarget as u32))
  );
  assert_eq!(
    embedded_object_mode_mask(),
    (1 << (RelocMode::FullEmbeddedObject as u32))
      | (1 << (RelocMode::CompressedEmbeddedObject as u32))
  );
  assert_eq!(
    js_dispatch_handle_mode_mask(),
    1 << (RelocMode::JsDispatchHandle as u32)
  );
  assert_ne!(all_real_modes_mask(), 0);
  assert_ne!(apply_mask(), 0);
  assert_ne!(post_codegen_relocation_mask(), 0);

  OK
}

#[test]
fn test_reloc_writer_and_iterator() -> Void {
  info!("测试 RelocInfoWriter 序列化与 RelocIterator 过滤迭代");

  let mut writer = RelocInfoWriter::new();
  writer.write(RelocInfo::new(0x10, RelocMode::CodeTarget));
  writer.write(RelocInfo::new(0x20, RelocMode::FullEmbeddedObject));
  writer.write(RelocInfo::new(0x30, RelocMode::ExternalReference));
  writer.write(RelocInfo::new(0x100, RelocMode::WasmCall)); // 跨越长 pc 偏移

  let encoded = writer.encode();
  assert!(!encoded.is_empty());

  let records = writer.records();
  assert_eq!(records.len(), 4);

  // 全模式迭代
  let all_iter = RelocIterator::all(records);
  assert_eq!(all_iter.count(), 4);

  // 仅过滤 CodeTarget 模式
  let target_mask = mode_mask(RelocMode::CodeTarget);
  let code_iter = RelocIterator::new(records, target_mask);
  let matched: Vec<_> = code_iter.collect();
  assert_eq!(matched.len(), 1);
  assert_eq!(matched[0].pc, 0x10);

  // 过滤 Real modes
  let real_iter = RelocIterator::new(records, all_real_modes_mask());
  let real_matched: Vec<_> = real_iter.collect();
  assert_eq!(real_matched.len(), 4);

  OK
}

#[test]
fn test_label_binding_and_linking() -> Void {
  info!("测试 Label 标号绑定与链式转移状态");

  let mut label = Label::new();
  assert!(label.is_unused());
  assert!(!label.is_bound());
  assert!(!label.is_linked());
  assert!(!label.is_near_linked());
  assert_eq!(label.pos(), Err(Error::LabelUnused));

  // 远跳转链接
  label.link_to(42, Distance::Far);
  assert!(label.is_linked());
  assert!(!label.is_bound());
  assert_eq!(label.pos(), Ok(42));

  // 近跳转链接
  label.link_to(100, Distance::Near);
  assert!(label.is_near_linked());
  assert_eq!(label.near_link_pos(), Ok(100));

  // 清除链接并绑定
  label.unuse();
  label.unuse_near();
  assert!(label.is_unused());

  label.bind_to(256)?;
  assert!(label.is_bound());
  assert!(!label.is_unused());
  assert!(!label.is_linked());
  assert_eq!(label.pos(), Ok(256));

  // 重复绑定应触发错误
  let double_bind_res = label.bind_to(512);
  assert_eq!(double_bind_res, Err(Error::LabelAlreadyBound(256)));

  OK
}

#[test]
fn test_assembler_base_buffer_and_emit() -> Void {
  info!("测试 AssemblerBase 缓冲区扩容策略与字节输出");

  // 扩容计算
  let doubled = compute_new_buffer_size(K_MINIMAL_BUFFER_SIZE, BufferGrowthStrategy::Double)?;
  assert_eq!(doubled, K_MINIMAL_BUFFER_SIZE * 2);

  let capped = compute_new_buffer_size(10 * 1024 * 1024, BufferGrowthStrategy::DoubleCapped1MB)?;
  assert_eq!(capped, 10 * 1024 * 1024 + 1024 * 1024);

  let overflow_res = compute_new_buffer_size(K_MAXIMAL_BUFFER_SIZE, BufferGrowthStrategy::Double);
  assert!(overflow_res.is_err());

  // 基础汇编器操作
  let options = AssemblerOptions::default();
  let mut assembler = AssemblerBase::new(options);
  assert_eq!(assembler.pc_offset(), 0);

  // CPU 特性启用
  assert!(!assembler.is_cpu_feature_enabled(5));
  assembler.enable_cpu_feature(5);
  assert!(assembler.is_cpu_feature_enabled(5));

  // 写入基础指令数据
  assembler.emit_u8(0x90)?; // NOP
  assembler.emit_u16(0x1234)?;
  assembler.emit_u32(0xdeadbeef)?;
  assembler.emit_u64(0x0123456789abcdef)?;
  assembler.emit_bytes(&[0xaa, 0xbb, 0xcc])?;

  assert_eq!(assembler.pc_offset(), 1 + 2 + 4 + 8 + 3);
  let bytes = assembler.buffer_slice();
  assert_eq!(bytes[0], 0x90);
  assert_eq!(&bytes[1..3], &0x1234u16.to_le_bytes());
  assert_eq!(&bytes[3..7], &0xdeadbeefu32.to_le_bytes());

  let taken = assembler.take_buffer();
  assert_eq!(taken.len(), 18);

  OK
}

#[test]
fn test_safepoint_table_builder_and_parse() -> Void {
  info!("测试 SafepointTable 序列化与反序列化及栈位图查询");

  let mut builder = SafepointTableBuilder::new(16, true, true);

  // 第一个 safepoint: pc = 10, 带 deopt 和 slot 0, 3 置位
  let mut entry1 = SafepointEntry::new(10);
  entry1.deopt_index = 0;
  entry1.trampoline_pc = 100;
  entry1.tagged_register_indexes = 0b101;
  entry1.set_tagged_slot(0);
  entry1.set_tagged_slot(3);
  builder.add_entry(entry1);

  // 第二个 safepoint: pc = 50, 无 deopt, slot 0, 3, 7 置位
  let mut entry2 = SafepointEntry::new(50);
  entry2.tagged_register_indexes = 0b010;
  entry2.set_tagged_slot(0);
  entry2.set_tagged_slot(3);
  entry2.set_tagged_slot(7);
  builder.add_entry(entry2);

  let encoded = builder.emit();
  assert!(encoded.len() >= 8);

  let table = SafepointTable::parse(&encoded)?;
  assert_eq!(table.stack_slots, 16);
  assert!(table.has_tagged_registers);
  assert!(table.has_deopt_data);
  assert_eq!(table.entries.len(), 2);

  // 验证 entry1
  let parsed1 = &table.entries[0];
  assert_eq!(parsed1.pc, 10);
  assert_eq!(parsed1.deopt_index, 0);
  assert_eq!(parsed1.trampoline_pc, 100);
  assert_eq!(parsed1.tagged_register_indexes, 0b101);
  assert!(parsed1.is_tagged_slot(0));
  assert!(parsed1.is_tagged_slot(3));
  assert!(!parsed1.is_tagged_slot(1));
  assert!(!parsed1.is_tagged_slot(7));

  // 验证 entry2 (增量恢复后的栈位图)
  let parsed2 = &table.entries[1];
  assert_eq!(parsed2.pc, 50);
  assert_eq!(parsed2.deopt_index, -1);
  assert_eq!(parsed2.tagged_register_indexes, 0b010);
  assert!(parsed2.is_tagged_slot(0));
  assert!(parsed2.is_tagged_slot(3));
  assert!(parsed2.is_tagged_slot(7));
  assert!(!parsed2.is_tagged_slot(2));

  // 查找检索测试
  let found = table.find_entry(30);
  assert!(found.is_some());
  assert_eq!(found.unwrap().pc, 10);

  let found_exact = table.find_entry(50);
  assert!(found_exact.is_some());
  assert_eq!(found_exact.unwrap().pc, 50);

  let return_pc = table.find_return_pc(50);
  assert_eq!(return_pc, Some(50));

  OK
}

#[test]
fn test_call_interface_descriptor() -> Void {
  info!("测试 CallInterfaceDescriptor 参数寄存器约定与标志位");

  let mut data = CallInterfaceDescriptorData::new();
  let flags = CallInterfaceDescriptorFlags::NO_CONTEXT
    .union(CallInterfaceDescriptorFlags::CALLEE_SAVE_REGISTERS);

  data.initialize_registers(RegistersInit {
    flags,
    tag: 12345,
    return_count: 1,
    param_count: 4,
    stack_order: StackArgumentOrder::HighToLow,
    register_params: &[0, 1, 2],
    return_registers: &[0],
  })?;

  let desc = CallInterfaceDescriptor::new(data);
  assert!(!desc.has_context_parameter());
  assert!(desc.callee_save_registers());
  assert!(!desc.allow_var_args());
  assert_eq!(desc.return_count(), 1);
  assert_eq!(desc.param_count(), 4);
  assert_eq!(desc.register_param_count(), 3);
  assert_eq!(desc.stack_param_count(), 1);
  assert_eq!(desc.stack_order(), StackArgumentOrder::HighToLow);
  assert_eq!(desc.register_param(0), Some(0));
  assert_eq!(desc.register_param(1), Some(1));
  assert_eq!(desc.register_param(2), Some(2));
  assert_eq!(desc.register_param(3), None);
  assert_eq!(desc.register_return(0), Some(0));

  OK
}

#[test]
fn test_external_reference() -> Void {
  info!("测试 ExternalReference 外部符号与类型");

  let null_ref = ExternalReference::null();
  assert!(null_ref.is_null());
  assert_eq!(null_ref.raw(), 0);

  let addr_ref = ExternalReference::from_raw_address(0x7fff_0000);
  assert!(!addr_ref.is_null());
  assert_eq!(addr_ref.address(), 0x7fff_0000);
  assert_eq!(addr_ref.type_(), ExternalReferenceType::BuiltinCall);

  let sym_ref = ExternalReference::from_symbol("abort_with_reason", 0x8888_0000)
    .with_type(ExternalReferenceType::FastCCall);
  assert_eq!(sym_ref.name(), Some("abort_with_reason"));
  assert_eq!(sym_ref.type_(), ExternalReferenceType::FastCCall);
  assert_eq!(sym_ref.raw(), 0x8888_0000);

  OK
}

#[test]
fn test_source_position_table_encoding_decoding() -> Void {
  info!("测试 SourcePositionTable 增量压缩编码与解码遍历");

  let mut builder = SourcePositionTableBuilder::new(RecordingMode::RecordSourcePositions);

  // 记录连续多个源码位置
  builder.add_position(0, 10, true, true);
  builder.add_position(15, 25, false, true);
  builder.add_position(30, 20, true, false); // 源码位置向前跳 (负 delta)
  builder.add_position(100, 200, false, false);

  let bytes = builder.emit();
  assert!(!bytes.is_empty());

  let it = SourcePositionTableIterator::new(&bytes);
  let mut decoded = Vec::new();
  for entry in it {
    decoded.push(entry);
  }

  assert_eq!(decoded.len(), 4);

  assert_eq!(decoded[0].code_offset, 0);
  assert_eq!(decoded[0].source_position, 10);
  assert!(decoded[0].is_statement);
  assert!(decoded[0].is_breakable);

  assert_eq!(decoded[1].code_offset, 15);
  assert_eq!(decoded[1].source_position, 25);
  assert!(!decoded[1].is_statement);
  assert!(decoded[1].is_breakable);

  assert_eq!(decoded[2].code_offset, 30);
  assert_eq!(decoded[2].source_position, 20);
  assert!(decoded[2].is_statement);
  assert!(!decoded[2].is_breakable);

  assert_eq!(decoded[3].code_offset, 100);
  assert_eq!(decoded[3].source_position, 200);
  assert!(!decoded[3].is_statement);
  assert!(!decoded[3].is_breakable);

  OK
}

#[test]
fn test_allocation_site_tracking() -> Void {
  use v8_codegen_core::{
    ElementsKind, allocation_site_should_track, allocation_site_should_track_with_flag,
  };

  // PackedSmi / HoleySmi 元素类型应开启跟踪
  assert!(allocation_site_should_track(ElementsKind::PackedSmi as i32));
  assert!(allocation_site_should_track(ElementsKind::HoleySmi as i32));

  // Packed / PackedDouble 等非 Smi 元素类型不应开启跟踪
  assert!(!allocation_site_should_track(ElementsKind::Packed as i32));
  assert!(!allocation_site_should_track(
    ElementsKind::PackedDouble as i32
  ));

  // 当显式关闭 flag 时，任何元素类型都不跟踪
  assert!(!allocation_site_should_track_with_flag(
    ElementsKind::PackedSmi as i32,
    false
  ));
  assert!(allocation_site_should_track_with_flag(
    ElementsKind::PackedSmi as i32,
    true
  ));

  OK
}
