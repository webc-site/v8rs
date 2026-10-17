// v8-compiler-broker/tests/main.rs
//! v8-compiler-broker 集成测试套件
//! 涵盖 HeapRef 快照读取、MapRef 属性元数据查询、Broker 模式状态机、
//! PropertyCell/AllocationSite 引用、Oddball 精确布尔判定、
//! 以及 CompilationDependencies 全量假设校验与事务生命周期。

use aok::{OK, Void};
use log::info;
use v8_compiler_broker::{
  AccessMode, AllocationSiteData, BrokerMode, CompilationDependencies, CompilationDependency,
  DescriptorSnapshot, ElementsKind, Error, FieldIndex, JSFunctionData, JSHeapBroker, JSObjectData,
  MapData, OddballType, ProcessedFeedbackSnapshot, PropertyCellData, SharedFunctionInfoData,
};
use v8_compiler_graph::FeedbackSource;
use v8_objects_core::{
  instance_type::InstanceType,
  property_details::{
    PropertyAttributes, PropertyConstness, PropertyDetails, PropertyKind, PropertyLocation,
    Representation,
  },
};
use v8_objects_js_core::FieldEncoding;
use v8_objects_js_func::FunctionKind;
use v8_zone::{AccountingAllocator, Zone};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_broker_mode_transitions() -> Void {
  info!("> 测试 JSHeapBroker 模式状态迁移与序列化权限门禁");
  let allocator = AccountingAllocator::default();
  let zone = Zone::new(&allocator, "test-broker-mode");
  let mut broker = JSHeapBroker::new(&zone);

  assert_eq!(broker.mode(), BrokerMode::Disabled);
  assert!(!broker.serializing_allowed());

  // 1. Disabled 模式下严禁序列化写入与反馈注册
  let disabled_smi_err = broker.make_smi(42);
  assert_eq!(
    disabled_smi_err.err(),
    Some(Error::SerializationNotAllowed(BrokerMode::Disabled))
  );

  let fb_source = FeedbackSource {
    vector: 0x5000 as *const (),
    slot: 1,
  };
  let disabled_fb_err = broker.set_feedback(fb_source, ProcessedFeedbackSnapshot::Insufficient);
  assert_eq!(
    disabled_fb_err.err(),
    Some(Error::SerializationNotAllowed(BrokerMode::Disabled))
  );

  // 2. Disabled -> Serializing
  broker.initialize_and_start_serializing(Some(0x1000))?;
  assert_eq!(broker.mode(), BrokerMode::Serializing);
  assert!(broker.serializing_allowed());
  assert_eq!(broker.target_native_context(), Some(0x1000));

  // 允许在 Serializing 模式下正常注册数据
  let smi_ok = broker.make_smi(42)?;
  assert_eq!(smi_ok.as_smi(), Some(42));

  // 重复进入 Serializing 应报错
  assert!(broker.initialize_and_start_serializing(None).is_err());

  // 3. Serializing -> Serialized
  broker.stop_serializing()?;
  assert_eq!(broker.mode(), BrokerMode::Serialized);
  assert!(!broker.serializing_allowed());

  // Serialized 态禁止再进行序列化操作
  let serialized_smi_err = broker.make_smi(42);
  assert_eq!(
    serialized_smi_err.err(),
    Some(Error::SerializationNotAllowed(BrokerMode::Serialized))
  );

  // 4. Serialized -> Retired
  broker.retire()?;
  assert_eq!(broker.mode(), BrokerMode::Retired);
  assert!(!broker.serializing_allowed());

  // Retired 态严禁写入
  let retired_smi_err = broker.make_smi(42);
  assert_eq!(
    retired_smi_err.err(),
    Some(Error::SerializationNotAllowed(BrokerMode::Retired))
  );

  // Retired 态无法重新转入任何前置态
  assert!(broker.stop_serializing().is_err());
  assert!(broker.retire().is_err());

  OK
}

#[test]
fn test_heap_refs_smi_and_oddball() -> Void {
  info!("> 测试 Smi 与 Oddball 基础快照读取及精确布尔判定 (无地址奇偶性)");
  let allocator = AccountingAllocator::default();
  let zone = Zone::new(&allocator, "test-heap-refs-basic");
  let mut broker = JSHeapBroker::new(&zone);
  broker.initialize_and_start_serializing(None)?;

  // Smi
  let smi_ref = broker.make_smi(12345)?;
  assert!(smi_ref.is_smi());
  assert_eq!(smi_ref.as_smi(), Some(12345));
  assert!(!smi_ref.is_heap_object());
  assert_eq!(smi_ref.to_boolean(), Some(true));

  let smi_zero = broker.make_smi(0)?;
  assert_eq!(smi_zero.as_smi(), Some(0));
  assert_eq!(smi_zero.to_boolean(), Some(false));

  // Oddball: Null & Undefined
  let null_ref = broker.make_oddball(0x2000, OddballType::Null, 0x10)?;
  assert!(null_ref.is_heap_object());
  assert!(null_ref.is_null());
  assert!(!null_ref.is_undefined());
  assert_eq!(null_ref.to_boolean(), Some(false));

  let undef_ref = broker.make_oddball(0x2008, OddballType::Undefined, 0x10)?;
  assert!(undef_ref.is_undefined());
  assert!(!undef_ref.is_null());
  assert_eq!(undef_ref.to_boolean(), Some(false));

  // Oddball: 显式 True 与 False 判定
  // 关键验证: 偶数地址存储 True，奇数地址存储 False，彻底阻绝基于地址奇偶性的错误判定
  let even_addr_true = broker.make_oddball_boolean(0x2002, true, 0x10)?;
  assert_eq!(even_addr_true.address() % 2, 0);
  assert_eq!(even_addr_true.to_boolean(), Some(true));
  let oddball_true = even_addr_true.as_oddball().ok_or(Error::TypeMismatch {
    expected: "Oddball",
    actual: "NonOddball",
  })?;
  assert_eq!(oddball_true.oddball_type(), OddballType::True);
  assert!(oddball_true.to_boolean_value());

  let odd_addr_false = broker.make_oddball_boolean(0x2003, false, 0x10)?;
  assert_eq!(odd_addr_false.address() % 2, 1);
  assert_eq!(odd_addr_false.to_boolean(), Some(false));
  let oddball_false = odd_addr_false.as_oddball().ok_or(Error::TypeMismatch {
    expected: "Oddball",
    actual: "NonOddball",
  })?;
  assert_eq!(oddball_false.oddball_type(), OddballType::False);
  assert!(!oddball_false.to_boolean_value());

  // 通过 make_oddball 直接传入 OddballType::True / OddballType::False
  let true_ref = broker.make_oddball(0x2004, OddballType::True, 0x10)?;
  assert_eq!(true_ref.to_boolean(), Some(true));
  let false_ref = broker.make_oddball(0x2005, OddballType::False, 0x10)?;
  assert_eq!(false_ref.to_boolean(), Some(false));

  OK
}

#[test]
fn test_property_cell_and_allocation_site_refs() -> Void {
  info!("> 测试 PropertyCellRef 与 AllocationSiteRef 快照构建与导航");
  let allocator = AccountingAllocator::default();
  let zone = Zone::new(&allocator, "test-cell-site");
  let mut broker = JSHeapBroker::new(&zone);
  broker.initialize_and_start_serializing(None)?;

  let map_data = MapData {
    instance_size: 32,
    instance_type: InstanceType::JSObject,
    in_object_properties: 0,
    number_of_own_descriptors: 0,
    elements_kind: ElementsKind::PackedSmi,
    is_stable: true,
    is_constructor: false,
    is_callable: false,
    is_deprecated: false,
    is_undetectable: false,
    prototype: None,
    constructor_or_back_pointer: None,
    descriptors: vec![],
    construction_counter: 0,
  };
  let meta_map = broker.make_map(0x1000, map_data)?;

  // 1. PropertyCellRef
  let target_val = broker.make_smi(999)?;
  let prop_details = PropertyDetails::for_fast_field(
    PropertyKind::Data,
    PropertyAttributes::READ_ONLY,
    PropertyLocation::Field,
    PropertyConstness::Const,
    Representation::tagged(),
    0,
    true,
  );
  let cell_data = PropertyCellData {
    details: prop_details,
    value_address: target_val.address(),
  };
  let cell_ref = broker.make_property_cell(0x3000, cell_data, meta_map.address())?;

  assert_eq!(cell_ref.address(), 0x3000);
  assert!(cell_ref.property_details().is_read_only());
  assert_eq!(cell_ref.value_address(), target_val.address());

  // 导航读取值
  let cell_val = cell_ref.value(&broker)?;
  assert_eq!(cell_val.as_smi(), Some(999));

  // 对象引用转换检查
  let cell_obj = cell_ref.object();
  assert!(cell_obj.is_property_cell());
  assert_eq!(
    cell_obj.as_property_cell().map(|c| c.address()),
    Some(0x3000)
  );

  // 2. AllocationSiteRef
  let boilerplate_obj = broker.make_js_object(
    0x4000,
    JSObjectData {
      in_object_properties: vec![],
      elements_kind: ElementsKind::PackedDouble,
    },
    meta_map.address(),
  )?;

  let site_data = AllocationSiteData {
    elements_kind: ElementsKind::PackedDouble,
    boilerplate_address: Some(boilerplate_obj.address()),
    is_pretenured: true,
  };
  let site_ref = broker.make_allocation_site(0x5000, site_data, meta_map.address())?;

  assert_eq!(site_ref.address(), 0x5000);
  assert_eq!(site_ref.elements_kind(), ElementsKind::PackedDouble);
  assert!(site_ref.is_pretenured());
  assert_eq!(
    site_ref.boilerplate_address(),
    Some(boilerplate_obj.address())
  );

  // 导航读取样板对象
  let bp = site_ref.boilerplate(&broker)?;
  assert!(bp.is_some());
  assert_eq!(bp.map(|b| b.address()), Some(0x4000));

  let site_obj = site_ref.object();
  assert!(site_obj.is_allocation_site());
  assert_eq!(
    site_obj.as_allocation_site().map(|s| s.address()),
    Some(0x5000)
  );

  OK
}

#[test]
fn test_map_ref_metadata_and_descriptors() -> Void {
  info!("> 测试 MapRef 属性元数据查询与描述符索引");
  let allocator = AccountingAllocator::default();
  let zone = Zone::new(&allocator, "test-map-ref");
  let mut broker = JSHeapBroker::new(&zone);
  broker.initialize_and_start_serializing(None)?;

  let field_idx = FieldIndex::new(true, 16, FieldEncoding::Tagged, 4, 16);
  let prop_details = PropertyDetails::for_fast_field(
    PropertyKind::Data,
    PropertyAttributes::NONE,
    PropertyLocation::Field,
    PropertyConstness::Const,
    Representation::tagged(),
    16,
    true,
  );

  let desc = DescriptorSnapshot {
    key: "x",
    details: prop_details,
    field_index: Some(field_idx),
    field_owner_map: Some(0x8000),
  };

  let proto_obj = broker.make_oddball(0x3000, OddballType::Null, 0x8000)?;
  let map_data = MapData {
    instance_size: 48,
    instance_type: InstanceType::JSObject,
    in_object_properties: 4,
    number_of_own_descriptors: 1,
    elements_kind: ElementsKind::PackedSmi,
    is_stable: true,
    is_constructor: true,
    is_callable: false,
    is_deprecated: false,
    is_undetectable: false,
    prototype: Some(proto_obj.address()),
    constructor_or_back_pointer: None,
    descriptors: vec![desc],
    construction_counter: 7,
  };

  let map_ref = broker.make_map(0x8000, map_data)?;
  assert_eq!(map_ref.instance_size(), 48);
  assert_eq!(map_ref.instance_type(), InstanceType::JSObject);
  assert_eq!(map_ref.in_object_properties(), 4);
  assert_eq!(map_ref.number_of_own_descriptors(), 1);
  assert_eq!(map_ref.elements_kind(), ElementsKind::PackedSmi);
  assert!(map_ref.is_stable());
  assert!(map_ref.is_constructor());
  assert!(!map_ref.is_callable());
  assert!(!map_ref.is_deprecated());
  assert_eq!(map_ref.construction_counter(), 7);

  // 描述符查询
  assert_eq!(map_ref.descriptors().len(), 1);
  assert_eq!(map_ref.get_property_name(0), Some("x"));
  let read_details = map_ref.get_property_details(0);
  assert!(read_details.is_some());
  assert_eq!(
    read_details.map(|d| d.constness()),
    Some(PropertyConstness::Const)
  );

  let read_field_idx = map_ref.get_field_index(0);
  assert_eq!(read_field_idx, Some(field_idx));

  // 查找字段所属 Map
  let owner = map_ref.find_field_owner(0, &broker)?;
  assert!(owner.is_some());
  assert_eq!(owner.map(|m| m.address()), Some(0x8000));

  // 原型查找
  let proto = map_ref.prototype(&broker)?;
  assert!(proto.is_some());
  assert_eq!(proto.map(|p| p.address()), Some(0x3000));

  OK
}

#[test]
fn test_js_function_and_shared_function_info() -> Void {
  info!("> 测试 JSFunctionRef 与 SharedFunctionInfoRef 引用导航");
  let allocator = AccountingAllocator::default();
  let zone = Zone::new(&allocator, "test-func-ref");
  let mut broker = JSHeapBroker::new(&zone);
  broker.initialize_and_start_serializing(None)?;

  // 1. 创建 Map
  let map_data = MapData {
    instance_size: 32,
    instance_type: InstanceType::JSFunctionWithPrototype,
    in_object_properties: 0,
    number_of_own_descriptors: 0,
    elements_kind: ElementsKind::PackedSmi,
    is_stable: true,
    is_constructor: true,
    is_callable: true,
    is_deprecated: false,
    is_undetectable: false,
    prototype: None,
    constructor_or_back_pointer: None,
    descriptors: vec![],
    construction_counter: 0,
  };
  let func_map = broker.make_map(0x4000, map_data)?;

  // 2. 创建 SharedFunctionInfo
  let sfi_data = SharedFunctionInfoData {
    builtin_id: Some(42),
    function_kind: FunctionKind::NormalFunction,
    is_compiled: true,
    is_toplevel: false,
    has_bytecode_array: true,
    parameter_count: 2,
  };
  let sfi_ref = broker.make_shared_function_info(0x5000, sfi_data, func_map.address())?;
  assert_eq!(sfi_ref.builtin_id(), Some(42));
  assert_eq!(sfi_ref.function_kind(), FunctionKind::NormalFunction);
  assert!(sfi_ref.is_compiled());
  assert!(!sfi_ref.is_toplevel());
  assert!(sfi_ref.has_bytecode_array());
  assert_eq!(sfi_ref.parameter_count(), 2);

  // 3. 创建 JSFunction
  let proto_obj = broker.make_oddball(0x6000, OddballType::Null, func_map.address())?;
  let func_data = JSFunctionData {
    shared_address: sfi_ref.address(),
    initial_map_address: Some(func_map.address()),
    prototype_address: Some(proto_obj.address()),
    context_address: None,
    has_prototype: true,
  };
  let func_ref = broker.make_js_function(0x7000, func_data, func_map.address())?;

  assert!(func_ref.has_initial_map());
  assert_eq!(func_ref.initial_map(&broker)?.address(), func_map.address());
  assert!(func_ref.has_instance_prototype());
  assert_eq!(
    func_ref.instance_prototype(&broker)?.address(),
    proto_obj.address()
  );
  assert_eq!(func_ref.shared(&broker)?.address(), sfi_ref.address());

  OK
}

#[test]
fn test_compilation_dependencies_full_validation() -> Void {
  info!(
    "> 测试 CompilationDependencies 全量假设校验 (包含 GlobalProperty, Protector, ElementsKind, NoSlackTrackingChange)"
  );
  let allocator = AccountingAllocator::default();
  let zone = Zone::new(&allocator, "test-comp-deps-full");
  let mut broker = JSHeapBroker::new(&zone);
  broker.initialize_and_start_serializing(None)?;

  let map_data = MapData {
    instance_size: 32,
    instance_type: InstanceType::JSObject,
    in_object_properties: 2,
    number_of_own_descriptors: 0,
    elements_kind: ElementsKind::PackedSmi,
    is_stable: true,
    is_constructor: true,
    is_callable: false,
    is_deprecated: false,
    is_undetectable: false,
    prototype: None,
    constructor_or_back_pointer: None,
    descriptors: vec![],
    construction_counter: 7,
  };
  let map_ref = broker.make_map(0x9000, map_data)?;

  let sfi_data = SharedFunctionInfoData {
    builtin_id: None,
    function_kind: FunctionKind::NormalFunction,
    is_compiled: true,
    is_toplevel: false,
    has_bytecode_array: true,
    parameter_count: 1,
  };
  let sfi_ref = broker.make_shared_function_info(0x9100, sfi_data, map_ref.address())?;
  let proto_obj = broker.make_oddball(0x9200, OddballType::Null, map_ref.address())?;

  let func_data = JSFunctionData {
    shared_address: sfi_ref.address(),
    initial_map_address: Some(map_ref.address()),
    prototype_address: Some(proto_obj.address()),
    context_address: None,
    has_prototype: true,
  };
  let func_ref = broker.make_js_function(0x9300, func_data, map_ref.address())?;

  // 创建 PropertyCell
  let cell_data = PropertyCellData {
    details: PropertyDetails::for_fast_field(
      PropertyKind::Data,
      PropertyAttributes::READ_ONLY,
      PropertyLocation::Field,
      PropertyConstness::Const,
      Representation::tagged(),
      0,
      true,
    ),
    value_address: func_ref.address(),
  };
  let cell_ref = broker.make_property_cell(0x9400, cell_data, map_ref.address())?;

  // 创建 AllocationSite
  let site_data = AllocationSiteData {
    elements_kind: ElementsKind::PackedSmi,
    boilerplate_address: None,
    is_pretenured: false,
  };
  let site_ref = broker.make_allocation_site(0x9500, site_data, map_ref.address())?;

  // 依赖记录器
  let mut deps = CompilationDependencies::new();
  deps.depend_on_initial_map(func_ref, map_ref)?;
  deps.depend_on_stable_map(map_ref)?;
  deps.depend_on_prototype_property(
    func_ref,
    proto_obj.as_heap_object().unwrap_or(func_ref.heap_object()),
  )?;
  deps.depend_on_global_property(cell_ref.address(), true)?;
  deps.depend_on_protector(42, true)?;
  deps.depend_on_no_slack_tracking_change(map_ref)?;
  deps.depend_on_elements_kind(site_ref.address(), ElementsKind::PackedSmi)?;

  assert_eq!(deps.count(), 7);

  // 切换到只读模式
  broker.stop_serializing()?;

  // 1. 验证全部通过
  assert!(deps.validate(&broker)?);

  // 2. 校验保护器失效场景
  broker.invalidate_protector(42);
  let prot_err = deps.validate(&broker);
  assert!(prot_err.is_err());
  broker.set_protector(42, true); // 恢复有效

  // 3. 校验 SlackTracking 计数器失配场景
  let mut bad_deps = CompilationDependencies::new();
  bad_deps.record_dependency(CompilationDependency::NoSlackTrackingChange {
    map_address: map_ref.address(),
    expected_construction_counter: 0, // 期望为 0，实际为 7
  })?;
  let slack_err = bad_deps.validate(&broker);
  assert!(slack_err.is_err());

  // 4. 校验 ElementsKind 失配场景
  let mut bad_elem_deps = CompilationDependencies::new();
  bad_elem_deps.depend_on_elements_kind(site_ref.address(), ElementsKind::HoleyDouble)?;
  let elem_err = bad_elem_deps.validate(&broker);
  assert!(elem_err.is_err());

  // 5. 校验 GlobalProperty 只读标志失配场景
  let mut bad_cell_deps = CompilationDependencies::new();
  bad_cell_deps.depend_on_global_property(cell_ref.address(), false)?; // 期望 false，实际 true
  let cell_err = bad_cell_deps.validate(&broker);
  assert!(cell_err.is_err());

  OK
}

#[test]
fn test_compilation_dependencies_commit_lifecycle() -> Void {
  info!("> 测试 CompilationDependencies 事务生命周期 (固化、阻断重复提交与追加写入)");
  let allocator = AccountingAllocator::default();
  let zone = Zone::new(&allocator, "test-commit-lifecycle");
  let mut broker = JSHeapBroker::new(&zone);
  broker.initialize_and_start_serializing(None)?;

  let map_data = MapData {
    instance_size: 32,
    instance_type: InstanceType::JSObject,
    in_object_properties: 0,
    number_of_own_descriptors: 0,
    elements_kind: ElementsKind::PackedSmi,
    is_stable: true,
    is_constructor: false,
    is_callable: false,
    is_deprecated: false,
    is_undetectable: false,
    prototype: None,
    constructor_or_back_pointer: None,
    descriptors: vec![],
    construction_counter: 0,
  };
  let map_ref = broker.make_map(0x1000, map_data)?;

  let mut deps = CompilationDependencies::new();
  deps.depend_on_stable_map(map_ref)?;
  deps.depend_on_no_slack_tracking_change(map_ref)?;
  assert_eq!(deps.count(), 2);
  assert!(!deps.is_committed());
  assert!(deps.committed_dependencies().is_none());

  broker.stop_serializing()?;

  // 首次提交：验证并通过，进入 Committed 状态并固化依赖列表
  let committed = deps.commit(&broker)?;
  assert!(committed);
  assert!(deps.is_committed());
  assert_eq!(deps.count(), 2);
  assert!(deps.committed_dependencies().is_some());

  // 提交后校验依然有效
  assert!(deps.validate(&broker)?);

  // 重复提交阻断
  let double_commit_err = deps.commit(&broker);
  assert_eq!(
    double_commit_err.err(),
    Some(Error::DependenciesAlreadyCommitted)
  );

  // 提交后追加写入阻断
  let write_after_commit_err = deps.depend_on_stable_map(map_ref);
  assert_eq!(
    write_after_commit_err.err(),
    Some(Error::DependenciesAlreadyCommitted)
  );

  OK
}

#[test]
fn test_broker_feedback_and_misc() -> Void {
  info!("> 测试 Broker 反馈快照与辅助接口");
  let allocator = AccountingAllocator::default();
  let zone = Zone::new(&allocator, "test-feedback");
  let mut broker = JSHeapBroker::new(&zone);
  broker.initialize_and_start_serializing(None)?;

  let fb_source = FeedbackSource {
    vector: 0x5000 as *const (),
    slot: 2,
  };

  assert!(!broker.has_feedback(&fb_source));
  broker.set_feedback(fb_source, ProcessedFeedbackSnapshot::BinaryHint(1))?;
  assert!(broker.has_feedback(&fb_source));
  assert_eq!(
    broker.get_feedback(&fb_source),
    Some(&ProcessedFeedbackSnapshot::BinaryHint(1))
  );

  let js_obj_data = JSObjectData {
    in_object_properties: vec![Some(0x1234)],
    elements_kind: ElementsKind::PackedSmi,
  };
  let ho_ref = broker.make_js_object(0x6000, js_obj_data, 0x10)?;
  assert_eq!(ho_ref.address(), 0x6000);

  let mode_load = AccessMode::Load;
  assert!(!mode_load.is_any_store());
  assert!(!mode_load.is_defining_store());

  let mode_store = AccessMode::StoreInLiteral;
  assert!(mode_store.is_any_store());
  assert!(mode_store.is_defining_store());

  let kind_smi = ElementsKind::PackedSmi;
  assert!(kind_smi.is_packed());
  assert!(!kind_smi.is_holey());
  assert!(!kind_smi.is_dictionary());

  OK
}
