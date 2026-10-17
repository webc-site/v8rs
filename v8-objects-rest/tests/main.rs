// v8-objects-rest/tests/main.rs

use std::ptr::write;

use aok::{OK, Void};
use log::info;
use v8_objects_arrays::fixed_array::{ProtectedFixedArray, TaggedArrayExt, TrustedWeakFixedArray};
use v8_objects_core::{
  heap_object::HeapObject,
  instance_type::InstanceType,
  map::Map,
  name::Name,
  objects::WriteBarrierMode,
  property_details::{PropertyAttributes, PropertyCellType, PropertyDetails, PropertyKind},
  smi::Smi,
  tagged::{MaybeObject, Object, Tagged},
};
use v8_objects_rest::{
  AbstractCode, AbstractCodeExt, BytecodeArrayBuf, BytecodeArrayExt, ClosureCountTransition,
  CodeExt, DeoptimizationData, DeoptimizationDataExt, DeoptimizationLiteralArray,
  DeoptimizationLiteralArrayExt, DeoptimizationLiteralKind, DictionaryTemplateInfo, Error,
  FeedbackCell, FeedbackSlot, FeedbackSlotKind, FeedbackVector, FeedbackVectorBuf,
  FeedbackVectorExt, FunctionTemplateInfo, FunctionTemplateRareData, ObjectTemplateInfo,
  PrivateSymbolKind, PropertyCell, Symbol, TemplateInfo, TemplateInfoWithProperties,
  WellKnownSymbolId,
  code::{CodeBuf, K_NO_BUILTIN_ID},
  code_kind::{
    CodeKind, CodeKindFlag, JS_FUNCTION_CODE_KINDS_MASK, OPTIMIZED_JS_FUNCTION_CODE_KINDS_MASK,
  },
  instruction_stream::{
    from_entry_address, from_target_address, size_for as istream_size_for,
    trailing_padding_size_for,
  },
  turbofan_types_inl::{
    TurbofanBitsetType, TurbofanBitsetTypeExt, TurbofanHeapConstantType,
    TurbofanHeapConstantTypeExt, TurbofanOtherNumberConstantType,
    TurbofanOtherNumberConstantTypeExt, TurbofanRangeType, TurbofanRangeTypeExt, TurbofanUnionType,
    TurbofanUnionTypeExt,
  },
  turboshaft_types::{
    TurboshaftFloat64RangeType, TurboshaftFloat64RangeTypeExt, TurboshaftFloat64SetType,
    TurboshaftFloat64SetTypeExt, TurboshaftWord32RangeType, TurboshaftWord32RangeTypeExt,
    TurboshaftWord32SetType, TurboshaftWord32SetTypeExt, TurboshaftWord64RangeType,
    TurboshaftWord64RangeTypeExt, TurboshaftWord64SetType, TurboshaftWord64SetTypeExt,
  },
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// 模拟 Dummy Map 句柄
const FAKE_MAP_ADDR: usize = 0x1000;
const FAKE_NAME_ADDR: usize = 0x2000;
const FAKE_VAL_ADDR: usize = 0x3000;
const FAKE_DEP_ADDR: usize = 0x4000;

fn fake_map() -> Tagged<Map> {
  Tagged::new(FAKE_MAP_ADDR | 1)
}

fn fake_name() -> Tagged<Name> {
  Tagged::new(FAKE_NAME_ADDR | 1)
}

fn fake_obj(addr: usize) -> Tagged<Object> {
  Tagged::new(addr | 1)
}

fn fake_typed_map(instance_type: InstanceType) -> (Box<Map>, Tagged<Map>) {
  let map = Box::new(Map {
    header: HeapObject::new(Tagged::NULL),
    instance_size_in_words: 0,
    inobject_properties_start_or_constructor_function_index: 0,
    used_or_unused_instance_size_in_words: 0,
    visitor_id: 0,
    instance_type,
    bit_field: 0,
    bit_field2: 0,
    bit_field3: 0,
    prototype: Tagged::NULL,
    constructor_or_back_pointer_or_native_context: Tagged::NULL,
    instance_descriptors: Tagged::NULL,
    dependent_code: Tagged::NULL,
    prototype_validity_cell: Tagged::NULL,
    transitions_or_prototype_info: Tagged::NULL,
  });
  let tagged = unsafe { Tagged::from_raw_address(&*map as *const Map as usize) };
  (map, tagged)
}

fn alloc_protected_fixed_array(
  map: Tagged<Map>,
  len: u32,
) -> (Vec<u64>, Tagged<ProtectedFixedArray>) {
  let total_bytes = <Tagged<ProtectedFixedArray> as TaggedArrayExt>::size_for(len);
  let words = total_bytes.div_ceil(8);
  let mut storage = vec![0u64; words];
  let base = storage.as_mut_ptr() as usize;
  let header = ProtectedFixedArray::new(map, len);
  unsafe {
    write(base as *mut ProtectedFixedArray, header);
    (storage, Tagged::from_raw_address(base))
  }
}

fn alloc_trusted_weak_fixed_array(
  map: Tagged<Map>,
  len: u32,
) -> (Vec<u64>, Tagged<TrustedWeakFixedArray>) {
  let total_bytes = <Tagged<TrustedWeakFixedArray> as TaggedArrayExt>::size_for(len);
  let words = total_bytes.div_ceil(8);
  let mut storage = vec![0u64; words];
  let base = storage.as_mut_ptr() as usize;
  let header = TrustedWeakFixedArray::new(map, len);
  unsafe {
    write(base as *mut TrustedWeakFixedArray, header);
    (storage, Tagged::from_raw_address(base))
  }
}

#[test]
fn test_symbol_lifecycle_and_flags() -> Void {
  info!("测试 Symbol 基础构造与标志位更新");
  let map = fake_map();
  let desc = fake_obj(0x5000);

  // 1. 普通符号构造与默认状态
  let mut sym = Symbol::new(map, desc);
  assert_eq!(sym.description(), desc);
  assert_eq!(sym.flags(), 0);
  assert_eq!(sym.private_symbol_kind(), PrivateSymbolKind::Public);
  assert!(!sym.is_any_private());
  assert!(!sym.is_well_known_symbol());
  assert!(!sym.is_in_public_symbol_table());
  assert!(!sym.is_interesting_symbol());

  // 2. 标志位独立置位与清除
  sym.set_is_well_known_symbol(true);
  assert!(sym.is_well_known_symbol());
  sym.set_is_in_public_symbol_table(true);
  assert!(sym.is_in_public_symbol_table());
  sym.set_is_interesting_symbol(true);
  assert!(sym.is_interesting_symbol());

  sym.set_is_well_known_symbol(false);
  assert!(!sym.is_well_known_symbol());
  assert!(sym.is_in_public_symbol_table());
  assert!(sym.is_interesting_symbol());

  // 3. 私有类别变迁
  sym.set_private_symbol_kind(PrivateSymbolKind::Internal);
  assert!(sym.is_any_private());
  assert!(sym.is_private_internal());
  assert!(!sym.is_any_private_name());

  sym.set_private_symbol_kind(PrivateSymbolKind::FieldName);
  assert!(sym.is_any_private_name());
  assert!(!sym.is_private_brand());

  sym.set_private_symbol_kind(PrivateSymbolKind::Brand);
  assert!(sym.is_any_private_name());
  assert!(sym.is_private_brand());

  // 4. 特殊构造器验证
  let well_known_sym = Symbol::well_known(map, desc);
  assert!(well_known_sym.is_well_known_symbol());
  assert_eq!(
    well_known_sym.private_symbol_kind(),
    PrivateSymbolKind::Public
  );

  let public_sym = Symbol::public(map, desc);
  assert!(public_sym.is_in_public_symbol_table());

  let brand_sym = Symbol::private_brand(map, desc);
  assert!(brand_sym.is_private_brand());

  // 5. 符号标识符名称表
  assert_eq!(WellKnownSymbolId::Iterator.name(), "Symbol.iterator");
  assert_eq!(
    WellKnownSymbolId::AsyncIterator.name(),
    "Symbol.asyncIterator"
  );
  assert_eq!(WellKnownSymbolId::ToStringTag.name(), "Symbol.toStringTag");

  OK
}

#[test]
fn test_property_cell_transitions() -> Void {
  info!("测试 PropertyCell 状态迁移与兼容性判定");
  let map = fake_map();
  let name = fake_name();
  let details = PropertyDetails::for_global_dictionary(
    PropertyKind::Data,
    PropertyAttributes::NONE,
    PropertyCellType::Undefined,
    1,
  );
  let mut cell = PropertyCell::new(map, name, details, Tagged::NULL, fake_obj(FAKE_DEP_ADDR));

  assert_eq!(cell.name(), name);
  assert_eq!(cell.cell_type(), PropertyCellType::Undefined);
  assert_eq!(cell.value(), Tagged::NULL);

  // 兼容性判定与非法迁移
  assert!(PropertyCell::check_data_is_compatible(
    details,
    Tagged::NULL
  ));
  assert!(!PropertyCell::check_data_is_compatible(
    details,
    fake_obj(FAKE_VAL_ADDR)
  ));
  assert!(!cell.can_transition_to(details, Tagged::NULL)); // Undefined -> Undefined 非法
  assert_eq!(
    PropertyCell::initial_type(Tagged::NULL),
    PropertyCellType::Undefined
  );
  assert_eq!(
    PropertyCell::initial_type(fake_obj(FAKE_VAL_ADDR)),
    PropertyCellType::Constant
  );

  // Undefined 状态下无论写入何值一律晋升至 Constant
  assert_eq!(
    cell.updated_type(Tagged::NULL, details),
    PropertyCellType::Constant
  );
  assert_eq!(
    cell.updated_type(fake_obj(FAKE_VAL_ADDR), details),
    PropertyCellType::Constant
  );

  // 迁移至 Constant (Smi)
  let smi1 = Tagged::<Object>::new(Smi::from_int(10).ptr());
  let smi2 = Tagged::<Object>::new(Smi::from_int(20).ptr());
  let smi3 = Tagged::<Object>::new(Smi::from_int(30).ptr());
  let const_details = details.set_cell_type(PropertyCellType::Constant);
  assert!(cell.can_transition_to(const_details, smi1));
  cell.transition(const_details, smi1);

  assert_eq!(cell.cell_type(), PropertyCellType::Constant);
  assert_eq!(cell.value(), smi1);

  // 同值更新保持 Constant，同类型不同值 (Smi -> Smi) 变为 ConstantType
  assert_eq!(
    cell.updated_type(smi1, const_details),
    PropertyCellType::Constant
  );
  assert_eq!(
    cell.updated_type(smi2, const_details),
    PropertyCellType::ConstantType
  );

  // 迁移至 ConstantType 并保持
  let const_type_details = details.set_cell_type(PropertyCellType::ConstantType);
  assert!(cell.can_transition_to(const_type_details, smi2));
  cell.transition(const_type_details, smi2);
  assert_eq!(cell.cell_type(), PropertyCellType::ConstantType);

  // ConstantType 下再次更新 Smi 保持 ConstantType
  assert_eq!(
    cell.updated_type(smi3, const_type_details),
    PropertyCellType::ConstantType
  );

  // ConstantType 下更新为 HeapObject 退化为 Mutable
  let heap_val = fake_obj(0x9999);
  assert_eq!(
    cell.updated_type(heap_val, const_type_details),
    PropertyCellType::Mutable
  );

  // 迁移至 Mutable
  let mutable_details = details.set_cell_type(PropertyCellType::Mutable);
  assert!(cell.can_transition_to(mutable_details, heap_val));
  cell.transition(mutable_details, heap_val);
  assert_eq!(cell.cell_type(), PropertyCellType::Mutable);
  assert_eq!(cell.value(), heap_val);

  // Mutable 状态不能再回退到 Constant
  assert!(!cell.can_transition_to(const_details, smi1));

  OK
}

#[test]
fn test_feedback_cell_atomic_budget() -> Void {
  info!("测试 FeedbackCell 基础存取与原子预算计数");
  let map = fake_map();
  let init_val = fake_obj(FAKE_VAL_ADDR);
  let mut fc = FeedbackCell::new(map, init_val, 1000);

  assert_eq!(fc.value(), init_val);
  assert_eq!(fc.interrupt_budget(), 1000);
  assert_eq!(fc.dispatch_handle(), FeedbackCell::NULL_DISPATCH_HANDLE);

  // 调度句柄更新
  fc.set_dispatch_handle(0x1234_5678);
  assert_eq!(fc.dispatch_handle(), 0x1234_5678);

  // 原子预算递减与递增
  let rem = fc.decrement_interrupt_budget(200);
  assert_eq!(rem, 800);
  assert_eq!(fc.interrupt_budget(), 800);

  let inc = fc.increment_interrupt_budget(50);
  assert_eq!(inc, 850);
  assert_eq!(fc.interrupt_budget(), 850);

  // 清空预算
  fc.clear_interrupt_budget();
  assert_eq!(fc.interrupt_budget(), 0);

  // 重置反馈向量
  let fallback = fake_obj(0x7777);
  fc.set_interrupt_budget(500);
  fc.reset_feedback_vector(fallback);
  assert_eq!(fc.interrupt_budget(), 0);
  assert_eq!(fc.value(), fallback);

  // 状态迁移枚举检查
  let t = ClosureCountTransition::NoneToOne;
  assert_eq!(t, ClosureCountTransition::NoneToOne);

  OK
}

#[test]
fn test_feedback_vector_buf_and_slots() -> Void {
  info!("测试 FeedbackVectorBuf 内存分配、槽位读写与 OSR/分层状态");
  let map = fake_map();
  let fc_val = fake_obj(0x1111);
  let fc = FeedbackCell::new(map, fc_val, 0);
  let fc_tagged = Tagged::from_ref(&fc);

  // 分配 4 个槽位的 FeedbackVector
  assert_eq!(
    FeedbackVector::size_for(4),
    FeedbackVector::K_HEADER_SIZE + 4 * FeedbackVector::K_SLOT_SIZE
  );
  // 容量超限防御
  let overflow_err = FeedbackVectorBuf::new(map, FeedbackVector::K_MAX_SLOTS + 1, fc_tagged);
  assert!(matches!(overflow_err, Err(Error::CapacityExceeded { .. })));

  let fv_buf = FeedbackVectorBuf::new(map, 4, fc_tagged)?;
  assert_eq!(fv_buf.length(), 4);
  assert_eq!(fv_buf.parent_feedback_cell(), fc_tagged);
  assert_eq!(fv_buf.invocation_count(), 0);
  assert_eq!(fv_buf.osr_urgency(), 0);
  assert!(!fv_buf.maybe_has_turbofan_osr_code());
  assert!(!fv_buf.maybe_has_maglev_osr_code());
  assert!(!fv_buf.maybe_has_optimized_osr_code());
  assert!(!fv_buf.tiering_in_progress());

  // 调用计数
  fv_buf.set_invocation_count(42);
  assert_eq!(fv_buf.invocation_count(), 42);
  fv_buf.clear_invocation_count();
  assert_eq!(fv_buf.invocation_count(), 0);

  fv_buf.set_invocation_count_before_stable(5);
  assert_eq!(fv_buf.invocation_count_before_stable(), 5);

  // OSR 状态与辅助函数
  fv_buf.set_osr_urgency(3);
  assert_eq!(fv_buf.osr_urgency(), 3);
  fv_buf.reset_osr_urgency();
  assert_eq!(fv_buf.osr_urgency(), 0);
  fv_buf.request_osr_at_next_opportunity();
  assert_eq!(fv_buf.osr_urgency(), FeedbackVector::K_MAX_OSR_URGENCY);

  fv_buf.set_maybe_has_turbofan_osr_code(true);
  assert!(fv_buf.maybe_has_turbofan_osr_code());
  assert!(fv_buf.maybe_has_optimized_osr_code());
  assert_eq!(fv_buf.osr_urgency(), FeedbackVector::K_MAX_OSR_URGENCY); // 其它位保持
  fv_buf.set_maybe_has_maglev_osr_code(true);
  assert!(fv_buf.maybe_has_maglev_osr_code());
  assert!(fv_buf.maybe_has_optimized_osr_code());

  // 分层编译标志
  fv_buf.set_tiering_in_progress(true);
  assert!(fv_buf.tiering_in_progress());
  fv_buf.set_osr_tiering_in_progress(true);
  assert!(fv_buf.osr_tiering_in_progress());
  fv_buf.set_interrupt_budget_reset_by_ic_change(true);
  assert!(fv_buf.interrupt_budget_reset_by_ic_change());

  // 槽位读写 (普通与同步序)
  let val0 = fake_obj(0xaa00);
  let val1 = fake_obj(0xbb00);
  let val2 = fake_obj(0xcc00);
  let val3 = fake_obj(0xdd00);

  fv_buf.set(0, val0)?;
  fv_buf.set(1, val1)?;
  fv_buf.synchronized_set(2, val2)?;
  fv_buf.synchronized_set(3, val3)?;

  assert_eq!(fv_buf.get(0)?, val0);
  assert_eq!(fv_buf.get(1)?, val1);
  assert_eq!(fv_buf.synchronized_get(2)?, val2);
  assert_eq!(fv_buf.synchronized_get(3)?, val3);

  // 越界读写检查
  assert!(fv_buf.get(4).is_err());
  assert!(fv_buf.set(4, val0).is_err());
  assert!(fv_buf.synchronized_get(10).is_err());
  assert!(fv_buf.synchronized_set(10, val0).is_err());

  // FeedbackSlotKind 与 FeedbackSlot 封装
  let kind = FeedbackSlotKind::from_u8(4)?;
  assert_eq!(kind, FeedbackSlotKind::Call);
  let slot = FeedbackSlot(2);
  assert!(slot.is_valid());
  assert_eq!(slot.to_int(), 2);
  assert!(!FeedbackSlot::invalid().is_valid());

  OK
}

#[test]
fn test_templates_hierarchy_and_flags() -> Void {
  info!("测试 TemplateInfo、FunctionTemplateInfo 与 ObjectTemplateInfo 层次结构与位域");
  let map = fake_map();

  // 1. TemplateInfo
  let mut ti = TemplateInfo::new(map);
  assert!(!ti.is_cacheable());
  assert!(!ti.should_promote_to_read_only());
  assert_eq!(ti.serial_number(), 0);

  ti.set_is_cacheable(true);
  assert!(ti.is_cacheable());
  ti.set_should_promote_to_read_only(true);
  assert!(ti.should_promote_to_read_only());
  ti.set_serial_number(12345);
  assert_eq!(ti.serial_number(), 12345);
  assert!(ti.is_cacheable());
  assert!(ti.should_promote_to_read_only());

  // 2. TemplateInfoWithProperties
  let mut tiwp = TemplateInfoWithProperties::new(map);
  assert_eq!(tiwp.number_of_properties(), 0);
  tiwp.set_number_of_properties(8);
  assert_eq!(tiwp.number_of_properties(), 8);

  // 3. FunctionTemplateInfo
  let mut fti = FunctionTemplateInfo::new(map);
  assert!(!fti.is_object_template_call_handler());
  assert!(!fti.has_side_effects());
  assert!(!fti.undetectable());
  assert!(!fti.needs_access_check());
  assert!(!fti.read_only_prototype());
  assert!(!fti.remove_prototype());
  assert!(!fti.accept_any_receiver());
  assert!(!fti.published());

  fti.set_is_object_template_call_handler(true);
  assert!(fti.is_object_template_call_handler());
  fti.set_has_side_effects(true);
  assert!(fti.has_side_effects());
  fti.set_undetectable(true);
  assert!(fti.undetectable());
  fti.set_needs_access_check(true);
  assert!(fti.needs_access_check());
  fti.set_read_only_prototype(true);
  assert!(fti.read_only_prototype());
  fti.set_remove_prototype(true);
  assert!(fti.remove_prototype());
  fti.set_accept_any_receiver(true);
  assert!(fti.accept_any_receiver());
  fti.set_published(true);
  assert!(fti.published());

  // 接收者类型范围
  fti.set_allowed_receiver_instance_type_range_start(0x0100);
  fti.set_allowed_receiver_instance_type_range_end(0x0200);
  assert_eq!(fti.allowed_receiver_instance_type_range_start(), 0x0100);
  assert_eq!(fti.allowed_receiver_instance_type_range_end(), 0x0200);
  assert!(fti.published()); // 保证其它标志位不受破坏

  // 形参长度与回调
  fti.length = 2;
  fti.instance_type = 0x0150;
  fti.callback = 0xdead_beef;
  assert_eq!(fti.length, 2);
  assert_eq!(fti.instance_type, 0x0150);
  assert_eq!(fti.callback, 0xdead_beef);

  // 4. ObjectTemplateInfo
  let ctor = fake_obj(0x8888);
  let mut oti = ObjectTemplateInfo::new(map, ctor);
  assert_eq!(oti.constructor, ctor);
  assert!(!oti.is_immutable_prototype());
  assert!(!oti.is_code_kind());
  assert_eq!(oti.embedder_field_count(), 0);

  oti.set_immutable_prototype(true);
  assert!(oti.is_immutable_prototype());
  oti.set_code_kind(true);
  assert!(oti.is_code_kind());
  oti.set_embedder_field_count(3);
  assert_eq!(oti.embedder_field_count(), 3);
  assert!(oti.is_immutable_prototype());
  assert!(oti.is_code_kind());

  // 5. DictionaryTemplateInfo
  let prop_names = fake_obj(0x7777);
  let mut dti = DictionaryTemplateInfo::new(map, prop_names);
  assert_eq!(dti.property_names(), prop_names);
  let new_prop_names = fake_obj(0x6666);
  dti.set_property_names(new_prop_names);
  assert_eq!(dti.property_names(), new_prop_names);

  // 6. FunctionTemplateRareData
  let ftrd = FunctionTemplateRareData::new(map);
  assert_eq!(ftrd.prototype_template, Tagged::NULL);
  assert_eq!(ftrd.prototype_provider_template, Tagged::NULL);
  assert_eq!(ftrd.parent_template, Tagged::NULL);

  OK
}

#[test]
fn test_code_kind_ordering_and_flags() -> Void {
  info!("测试 CodeKind 语义、位域掩码与分层断言");
  // 1. 分层升级次序断言
  assert!((CodeKind::InterpretedFunction as u8) < (CodeKind::Baseline as u8));
  assert!((CodeKind::Baseline as u8) < (CodeKind::Maglev as u8));
  assert!((CodeKind::Maglev as u8) < (CodeKind::TurbofanJs as u8));

  // 2. 谓词判定
  assert!(CodeKind::InterpretedFunction.is_interpreted_js_function());
  assert!(!CodeKind::Baseline.is_interpreted_js_function());

  assert!(CodeKind::Baseline.is_baselined_js_function());
  assert!(!CodeKind::Maglev.is_baselined_js_function());

  assert!(CodeKind::InterpretedFunction.is_unoptimized_js_function());
  assert!(CodeKind::Baseline.is_unoptimized_js_function());
  assert!(!CodeKind::Maglev.is_unoptimized_js_function());

  assert!(CodeKind::Maglev.is_optimized_js_function());
  assert!(CodeKind::TurbofanJs.is_optimized_js_function());
  assert!(!CodeKind::Baseline.is_optimized_js_function());

  assert!(CodeKind::Baseline.is_js_function());
  assert!(CodeKind::TurbofanJs.is_js_function());
  assert!(!CodeKind::Builtin.is_js_function());
  assert!(CodeKind::Builtin.is_builtin_or_js_function());

  assert!(CodeKind::Maglev.can_osr());
  assert!(CodeKind::TurbofanJs.can_osr());
  assert!(!CodeKind::Baseline.can_osr());

  assert!(CodeKind::InterpretedFunction.can_tier_up());
  assert!(CodeKind::Baseline.can_tier_up());
  assert!(CodeKind::Maglev.can_tier_up());
  assert!(!CodeKind::TurbofanJs.can_tier_up());

  assert!(CodeKind::Maglev.uses_deoptimization_data());
  assert!(CodeKind::TurbofanJs.uses_deoptimization_data());
  assert!(!CodeKind::Baseline.uses_deoptimization_data());

  assert!(CodeKind::Baseline.uses_bytecode_offset_table());
  assert!(!CodeKind::TurbofanJs.uses_bytecode_offset_table());

  assert!(CodeKind::Baseline.may_lack_source_position_table());
  assert!(CodeKind::Builtin.may_lack_source_position_table());
  assert!(!CodeKind::TurbofanJs.may_lack_source_position_table());

  assert_eq!(CodeKind::for_top_tier(), CodeKind::TurbofanJs);

  // 3. 标记字符串与变体
  assert_eq!(CodeKind::InterpretedFunction.to_marker(false, false), "~");
  assert_eq!(CodeKind::Baseline.to_marker(false, false), "^");
  assert_eq!(CodeKind::Maglev.to_marker(false, false), "+");
  assert_eq!(CodeKind::Maglev.to_marker(true, false), "+'");
  assert_eq!(CodeKind::Maglev.to_marker(false, true), "o+");
  assert_eq!(CodeKind::TurbofanJs.to_marker(false, false), "*");
  assert_eq!(CodeKind::TurbofanJs.to_marker(true, false), "*'");
  assert_eq!(CodeKind::TurbofanJs.to_marker(false, true), "o*");

  // 4. 位集形式转换与掩码
  let flag = CodeKind::Baseline.to_code_kind_flag();
  assert_eq!(flag, CodeKindFlag::BASELINE);
  assert_eq!(
    CodeKindFlag::from(CodeKind::TurbofanJs),
    CodeKindFlag::TURBOFAN_JS
  );
  assert!(JS_FUNCTION_CODE_KINDS_MASK.contains_kind(CodeKind::InterpretedFunction));
  assert!(JS_FUNCTION_CODE_KINDS_MASK.contains_kind(CodeKind::Baseline));
  assert!(JS_FUNCTION_CODE_KINDS_MASK.contains_kind(CodeKind::Maglev));
  assert!(JS_FUNCTION_CODE_KINDS_MASK.contains_kind(CodeKind::TurbofanJs));
  assert!(!JS_FUNCTION_CODE_KINDS_MASK.contains_kind(CodeKind::Builtin));

  assert!(OPTIMIZED_JS_FUNCTION_CODE_KINDS_MASK.contains_kind(CodeKind::Maglev));
  assert!(OPTIMIZED_JS_FUNCTION_CODE_KINDS_MASK.contains_kind(CodeKind::TurbofanJs));
  assert!(!OPTIMIZED_JS_FUNCTION_CODE_KINDS_MASK.contains_kind(CodeKind::Baseline));

  OK
}

#[test]
fn test_code_layout_and_accessors() -> Void {
  info!("测试 Code 结构体布局、标志位域与参数存取");
  let map = fake_map();
  let code_buf = CodeBuf::new(map);

  // 1. 标志初始化与读写
  code_buf.initialize_flags(CodeKind::TurbofanJs, true, true);
  assert_eq!(code_buf.kind()?, CodeKind::TurbofanJs);
  assert!(code_buf.is_turbofanned());
  assert!(code_buf.is_context_specialized());
  assert!(!code_buf.marked_for_deoptimization());

  code_buf.set_builtin_id(1); // 满足 DCHECK(is_builtin)
  assert!(code_buf.is_builtin());
  code_buf.set_is_disabled_builtin(true);
  assert!(code_buf.is_disabled_builtin());
  code_buf.set_is_disabled_builtin(false);
  assert!(!code_buf.is_disabled_builtin());

  code_buf.set_marked_for_deoptimization(true);
  assert!(code_buf.marked_for_deoptimization());

  // DCHECK_IMPLIES(flag, marked_for_deoptimization)
  code_buf.set_embedded_objects_cleared(true);
  assert!(code_buf.embedded_objects_cleared());
  code_buf.set_embedded_objects_cleared(false);
  assert!(!code_buf.embedded_objects_cleared());

  code_buf.set_marked_for_deoptimization(false);
  assert!(!code_buf.marked_for_deoptimization());

  code_buf.set_can_have_weak_objects(true);
  assert!(code_buf.can_have_weak_objects());

  // 2. 内建号存取
  code_buf.set_builtin_id(K_NO_BUILTIN_ID);
  assert!(!code_buf.is_builtin());
  code_buf.set_builtin_id(123);
  assert!(code_buf.is_builtin());
  assert_eq!(code_buf.builtin_id(), 123);

  // 3. 参数计数与 wasm-js 双半区
  code_buf.set_parameter_count(6);
  assert_eq!(code_buf.parameter_count(), 6);
  assert_eq!(code_buf.parameter_count_without_receiver(), 5);

  code_buf.set_wasm_js_tagged_parameter_count(15);
  code_buf.set_wasm_js_first_tagged_parameter(25);
  assert_eq!(code_buf.wasm_js_tagged_parameter_count(), 15);
  assert_eq!(code_buf.wasm_js_first_tagged_parameter(), 25);

  // 4. OSR 与元数据偏移
  code_buf.set_osr_offset(77);
  assert_eq!(code_buf.osr_offset(), 77);

  code_buf.set_handler_table_offset(16);
  assert_eq!(code_buf.handler_table_offset(), 16);
  code_buf.set_code_comments_offset(32);
  assert_eq!(code_buf.code_comments_offset(), 32);
  code_buf.set_unwinding_info_offset(48);
  assert_eq!(code_buf.unwinding_info_offset(), 48);
  assert_eq!(code_buf.jump_table_info_offset(), 48); // 重定向
  assert_eq!(code_buf.code_comments_size(), 16); // 48 - 32

  // 5. 去优化数据槽检查与清空
  assert!(!code_buf.has_deoptimization_data_or_interpreter_data());
  code_buf.clear_deoptimization_data_and_interpreter_data();
  assert!(!code_buf.has_deoptimization_data_or_interpreter_data());

  // 6. 清理填充
  code_buf.clear_padding();

  OK
}

#[test]
fn test_bytecode_array_semantics_and_padding() -> Void {
  info!("测试 BytecodeArray 字段存取、字节拷贝与 clear_padding 计算");
  let map = fake_map();
  let length = 10;
  let ba_buf = BytecodeArrayBuf::new(map, length);

  assert_eq!(ba_buf.length(), 10);
  assert_eq!(ba_buf.bytecode_array_size(), 80); // (64 + 10 + 7) & !7 = 80

  // 1. 字节序列读写
  for i in 0..length {
    ba_buf.set(i, (i as u8) * 11);
  }
  for i in 0..length {
    assert_eq!(ba_buf.get(i), (i as u8) * 11);
  }

  // 2. 寄存器与形参
  unsafe {
    (*ba_buf.as_mut_ptr()).frame_size = 48;
    (*ba_buf.as_mut_ptr()).parameter_size = 4;
    (*ba_buf.as_mut_ptr()).max_arguments = 2;
  }
  assert_eq!(ba_buf.register_count(), 6); // 48 / 8
  assert_eq!(ba_buf.parameter_count(), 4);
  assert_eq!(ba_buf.parameter_count_without_receiver(), 3);
  assert_eq!(ba_buf.max_frame_size(), 48 + (2 << 3)); // 48 + 16 = 64

  // 3. 字节码跨数组拷贝
  let dest_buf = BytecodeArrayBuf::new(map, length);
  ba_buf.copy_bytecodes_to(dest_buf.tagged());
  for i in 0..length {
    assert_eq!(dest_buf.get(i), (i as u8) * 11);
  }

  // 4. 清除填充 (严格验证无越界写)
  ba_buf.clear_padding();

  OK
}

#[test]
fn test_instruction_stream_helpers() -> Void {
  info!("测试 InstructionStream 地址还原与尺寸计算");
  let entry = 0x2000_1000;
  let target = from_target_address(entry);
  assert_eq!(target, entry - 32 + 1);

  let ptr_slot = entry;
  let from_entry = from_entry_address(&raw const ptr_slot as usize);
  assert_eq!(from_entry, target);

  let body_size = 100;
  let pad = trailing_padding_size_for(body_size, 64);
  let total = istream_size_for(body_size, 64);
  assert_eq!(total, 32 + body_size + pad);
  assert_eq!(total % 64, 0);

  OK
}

#[test]
fn test_turboshaft_and_turbofan_types() -> Void {
  info!("测试 Turboshaft 与 Turbofan 核心类型访问面与非对齐读写");
  let map = fake_map();

  // 1. TurboshaftWord32RangeType
  let range32 = TurboshaftWord32RangeType {
    header: HeapObject::new(map),
    from: 10,
    to: 200,
  };
  let t_range32 = Tagged::from_ref(&range32);
  assert_eq!(t_range32.from(), 10);
  assert_eq!(t_range32.to(), 200);
  t_range32.set_from(50);
  t_range32.set_to(500);
  assert_eq!(t_range32.from(), 50);
  assert_eq!(t_range32.to(), 500);

  // 2. TurboshaftWord32SetType
  let mut s32_buf = vec![0u64; 8];
  let s32_base = s32_buf.as_mut_ptr() as usize;
  unsafe {
    (s32_base as *mut TurboshaftWord32SetType).write(TurboshaftWord32SetType {
      header: HeapObject::new(map),
      set_size: 3,
    });
  }
  let t_s32: Tagged<TurboshaftWord32SetType> = unsafe { Tagged::from_raw_address(s32_base) };
  assert_eq!(t_s32.set_size(), 3);
  t_s32.set_element(0, 101);
  t_s32.set_element(1, 102);
  t_s32.set_element(2, 103);
  assert_eq!(t_s32.element(0), 101);
  assert_eq!(t_s32.element(1), 102);
  assert_eq!(t_s32.element(2), 103);

  // 3. TurboshaftWord64RangeType
  let range64 = TurboshaftWord64RangeType {
    header: HeapObject::new(map),
    from_high: 0x1,
    from_low: 0x2,
    to_high: 0x3,
    to_low: 0x4,
  };
  let t_range64 = Tagged::from_ref(&range64);
  assert_eq!(t_range64.get_from_high(), 0x1);
  assert_eq!(t_range64.get_from_low(), 0x2);
  assert_eq!(t_range64.to_high(), 0x3);
  assert_eq!(t_range64.to_low(), 0x4);
  t_range64.set_from_high(0xA);
  t_range64.set_from_low(0xB);
  t_range64.set_to_high(0xC);
  t_range64.set_to_low(0xD);
  assert_eq!(t_range64.get_from_high(), 0xA);
  assert_eq!(t_range64.get_from_low(), 0xB);
  assert_eq!(t_range64.to_high(), 0xC);
  assert_eq!(t_range64.to_low(), 0xD);

  // 4. TurboshaftWord64SetType (高低段分离存储)
  let mut s64_buf = vec![0u64; 16];
  let s64_base = s64_buf.as_mut_ptr() as usize;
  unsafe {
    (s64_base as *mut TurboshaftWord64SetType).write(TurboshaftWord64SetType {
      header: HeapObject::new(map),
      set_size: 2,
    });
  }
  let t_s64: Tagged<TurboshaftWord64SetType> = unsafe { Tagged::from_raw_address(s64_base) };
  t_s64.set_element_high(0, 0x1111);
  t_s64.set_element_high(1, 0x2222);
  t_s64.set_element_low(0, 0x3333);
  t_s64.set_element_low(1, 0x4444);
  assert_eq!(t_s64.element_high(0), 0x1111);
  assert_eq!(t_s64.element_high(1), 0x2222);
  assert_eq!(t_s64.element_low(0), 0x3333);
  assert_eq!(t_s64.element_low(1), 0x4444);

  // 5. TurboshaftFloat64RangeType (非对齐 double)
  let mut f_range_buf = [0u8; 32];
  let f_range_base = f_range_buf.as_mut_ptr() as usize;
  unsafe {
    (f_range_base as *mut TurboshaftFloat64RangeType).write(TurboshaftFloat64RangeType {
      header: HeapObject::new(map),
      special_values: 1,
      padding: 0,
      min: [0; 8],
      max: [0; 8],
    });
  }
  let t_f_range: Tagged<TurboshaftFloat64RangeType> =
    unsafe { Tagged::from_raw_address(f_range_base) };
  assert_eq!(t_f_range.special_values(), 1);
  t_f_range.set_min(-42.5);
  t_f_range.set_max(100.25);
  assert_eq!(TurboshaftFloat64RangeTypeExt::min(&t_f_range), -42.5);
  assert_eq!(TurboshaftFloat64RangeTypeExt::max(&t_f_range), 100.25);

  // 6. TurboshaftFloat64SetType (非对齐 double 柔性尾)
  let mut f_set_buf = vec![0u64; 8];
  let f_set_base = f_set_buf.as_mut_ptr() as usize;
  unsafe {
    (f_set_base as *mut TurboshaftFloat64SetType).write(TurboshaftFloat64SetType {
      header: HeapObject::new(map),
      special_values: 2,
      set_size: 2,
    });
  }
  let t_f_set: Tagged<TurboshaftFloat64SetType> = unsafe { Tagged::from_raw_address(f_set_base) };
  assert_eq!(t_f_set.special_values(), 2);
  assert_eq!(t_f_set.set_size(), 2);
  t_f_set.set_element(0, 12.345);
  t_f_set.set_element(1, 67.891);
  assert_eq!(t_f_set.element(0), 12.345);
  assert_eq!(t_f_set.element(1), 67.891);

  // 7. TurbofanBitsetType
  let bitset = TurbofanBitsetType {
    header: HeapObject::new(map),
    bitset_low: 0x1234_5678,
    bitset_high: 0x9,
  };
  let t_bitset = Tagged::from_ref(&bitset);
  assert_eq!(t_bitset.bitset_low(), 0x1234_5678);
  assert_eq!(t_bitset.bitset_high(), 0x9);
  t_bitset.set_bitset_low(0xABCD);
  assert_eq!(t_bitset.bitset_low(), 0xABCD);

  // 8. TurbofanUnionType
  let union_type = TurbofanUnionType {
    header: HeapObject::new(map),
    type1: fake_obj(0x1000),
    type2: fake_obj(0x2000),
  };
  let t_union = Tagged::from_ref(&union_type);
  assert_eq!(t_union.type1(), fake_obj(0x1000).cast());
  assert_eq!(t_union.type2(), fake_obj(0x2000).cast());
  t_union.set_type1(fake_obj(0x3000).cast(), WriteBarrierMode::SkipWriteBarrier);
  assert_eq!(t_union.type1(), fake_obj(0x3000).cast());

  // 9. TurbofanRangeType (非对齐 double)
  let tf_range = TurbofanRangeType {
    header: HeapObject::new(map),
    min: [0; 8],
    max: [0; 8],
  };
  let t_tf_range = Tagged::from_ref(&tf_range);
  t_tf_range.set_min(-10.5);
  t_tf_range.set_max(20.5);
  assert_eq!(TurbofanRangeTypeExt::min(&t_tf_range), -10.5);
  assert_eq!(TurbofanRangeTypeExt::max(&t_tf_range), 20.5);

  // 10. TurbofanHeapConstantType & TurbofanOtherNumberConstantType
  let heap_const = TurbofanHeapConstantType {
    header: HeapObject::new(map),
    constant: fake_obj(0x4000),
  };
  let t_heap_const = Tagged::from_ref(&heap_const);
  assert_eq!(t_heap_const.constant(), fake_obj(0x4000).cast());
  t_heap_const.set_constant(fake_obj(0x5000).cast(), WriteBarrierMode::SkipWriteBarrier);
  assert_eq!(t_heap_const.constant(), fake_obj(0x5000).cast());

  let num_const = TurbofanOtherNumberConstantType {
    header: HeapObject::new(map),
    constant: [0; 8],
  };
  let t_num_const = Tagged::from_ref(&num_const);
  t_num_const.set_constant(99.99);
  assert_eq!(t_num_const.constant(), 99.99);

  OK
}

#[test]
fn test_abstract_code_dual_dispatch() -> Void {
  info!("测试 AbstractCode 双分发 (Code 支与 BytecodeArray 支)");
  let (_code_map_guard, code_map) = fake_typed_map(InstanceType::Code);
  let (_ba_map_guard, ba_map) = fake_typed_map(InstanceType::BytecodeArray);

  // 1. Code 支
  let code_buf = CodeBuf::new(code_map);
  code_buf.initialize_flags(CodeKind::TurbofanJs, true, true);
  code_buf.set_builtin_id(10);
  let code_ac: Tagged<AbstractCode> = code_buf.tagged().cast();

  assert!(code_ac.is_code_variant());
  assert_eq!(code_ac.get_code(), code_buf.tagged());
  assert_eq!(code_ac.to_union_type(), code_ac);
  assert_eq!(code_ac.kind()?, CodeKind::TurbofanJs);
  assert_eq!(code_ac.builtin_id(), 10);
  assert!(code_ac.is_context_specialized());
  assert_eq!(code_ac.instruction_size()?, code_buf.instruction_size());
  assert_eq!(code_ac.instruction_start(), code_buf.instruction_start());
  assert_eq!(code_ac.instruction_end(), code_buf.instruction_end());

  // 2. BytecodeArray 支
  let ba_buf = BytecodeArrayBuf::new(ba_map, 32);
  let ba_ac: Tagged<AbstractCode> = ba_buf.tagged().cast();

  assert!(!ba_ac.is_code_variant());
  assert_eq!(ba_ac.get_bytecode_array(), ba_buf.tagged());
  assert_eq!(ba_ac.to_union_type(), ba_ac);
  assert_eq!(ba_ac.kind()?, CodeKind::INTERPRETED_FUNCTION);
  assert_eq!(ba_ac.builtin_id(), K_NO_BUILTIN_ID);
  assert!(!ba_ac.has_instruction_stream());
  assert!(!ba_ac.is_context_specialized());
  assert_eq!(ba_ac.osr_offset(), -1);
  assert_eq!(ba_ac.instruction_size()?, 32);
  assert_eq!(
    ba_ac.instruction_start(),
    ba_buf.get_first_bytecode_address()
  );
  assert_eq!(
    ba_ac.instruction_end(),
    ba_buf.get_first_bytecode_address() + 32
  );

  // 3. contains (BytecodeArray 支)
  let base = ba_ac.address();
  assert!(ba_ac.contains(base)?);
  assert!(ba_ac.contains(base + 16)?);
  assert!(!ba_ac.contains(base + 1000)?);

  OK
}

#[test]
fn test_deoptimization_data_and_literal_array() -> Void {
  info!("测试 DeoptimizationData、DeoptimizationLiteralArray 与 DeoptimizationLiteralKind");

  // 1. DeoptimizationLiteralKind 枚举值与别名
  assert_eq!(DeoptimizationLiteralKind::Object as u8, 0);
  assert_eq!(DeoptimizationLiteralKind::Number as u8, 1);
  assert_eq!(DeoptimizationLiteralKind::SignedBigInt64 as u8, 2);
  assert_eq!(DeoptimizationLiteralKind::UnsignedBigInt64 as u8, 3);
  assert_eq!(DeoptimizationLiteralKind::Invalid as u8, 4);
  assert_eq!(DeoptimizationLiteralKind::WasmI31Ref as u8, 5);
  assert_eq!(DeoptimizationLiteralKind::WasmInt32 as u8, 6);
  assert_eq!(DeoptimizationLiteralKind::WasmFloat32 as u8, 7);
  assert_eq!(DeoptimizationLiteralKind::WasmFloat64 as u8, 8);
  assert_eq!(
    DeoptimizationLiteralKind::WASM_INT64,
    DeoptimizationLiteralKind::SignedBigInt64
  );
  assert_eq!(DeoptimizationLiteralKind::COUNT, 9);
  assert_eq!(
    DeoptimizationLiteralKind::from_repr(0),
    Some(DeoptimizationLiteralKind::Object)
  );
  assert_eq!(
    DeoptimizationLiteralKind::from_repr(8),
    Some(DeoptimizationLiteralKind::WasmFloat64)
  );
  assert_eq!(DeoptimizationLiteralKind::from_repr(9), None);

  // 2. DeoptimizationData 空数据安全分支 (round1 修复验证: len=0 返回 Ok(0))
  let map = fake_map();
  let (_empty_storage, empty_array) = alloc_protected_fixed_array(map, 0);
  let empty_data: Tagged<DeoptimizationData> = empty_array.cast();
  assert_eq!(empty_data.ulength()?, 0);
  assert_eq!(empty_data.deopt_count()?, 0);

  // 3. DeoptimizationData 边界防御 (0 < len < 12 报错 OutOfBounds)
  let (_partial_storage, partial_array) = alloc_protected_fixed_array(map, 6);
  let partial_data: Tagged<DeoptimizationData> = partial_array.cast();
  assert!(partial_data.deopt_count().is_err());

  // 4. DeoptimizationData 有效条目读写
  let (_full_storage, full_array) = alloc_protected_fixed_array(map, 18); // 12 + 2 * 3 = 18
  let deopt_data: Tagged<DeoptimizationData> = full_array.cast();
  assert_eq!(deopt_data.ulength()?, 18);
  assert_eq!(deopt_data.deopt_count()?, 2);

  // 设置并读取 bytecode offset
  deopt_data.set_bytecode_offset(0, 100)?;
  assert_eq!(
    deopt_data.bytecode_offset_or_builtin_continuation_id(0)?,
    100
  );
  deopt_data.set_bytecode_offset(1, 200)?;
  assert_eq!(
    deopt_data.bytecode_offset_or_builtin_continuation_id(1)?,
    200
  );

  // 越界条目报错
  assert!(
    deopt_data
      .bytecode_offset_or_builtin_continuation_id(2)
      .is_err()
  );
  assert!(deopt_data.set_bytecode_offset(2, 300).is_err());

  // 5. DeoptimizationLiteralArray 读写与防御
  let (_lit_storage, lit_array) = alloc_trusted_weak_fixed_array(map, 4);
  let lit_data: Tagged<DeoptimizationLiteralArray> = lit_array.cast();
  assert_eq!(lit_data.ulength()?, 4);

  // 写入已清除弱引用哨兵，get 应安全返回 Invariant 错误而非 panic
  unsafe {
    lit_array.set(
      0,
      Tagged::<MaybeObject>::cleared(),
      WriteBarrierMode::SkipWriteBarrier,
    );
  }
  assert!(lit_data.get(0).is_err());

  // 写入 Smi
  let test_smi = Smi::from_int(12345).cast::<Object>();
  lit_data.set(1, test_smi)?;
  assert_eq!(lit_data.get(1)?, test_smi);

  // 写入 BytecodeArray (自动换装为其 wrapper)
  let (_ba_map_guard, ba_map) = fake_typed_map(InstanceType::BytecodeArray);
  let ba_buf = BytecodeArrayBuf::new(ba_map, 16);
  lit_data.set(2, ba_buf.tagged().cast())?;
  // wrapper 初始为 NULL
  assert_eq!(lit_data.get(2)?, Tagged::<Object>::NULL);

  // 越界读写防御
  assert!(lit_data.get(4).is_err());
  assert!(lit_data.set(4, test_smi).is_err());

  OK
}
