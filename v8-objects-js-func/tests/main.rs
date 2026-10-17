// v8-objects-js-func/tests/main.rs
// JSFunction、SharedFunctionInfo 与 Context 综合单元测试

use aok::{OK, Void};
use log::info;
use v8_objects_arrays::fixed_array_base::FixedArrayBase;
use v8_objects_core::{
  heap_object::HeapObject, instance_type::InstanceType, map::Map, smi::Smi, tagged::Tagged,
};
use v8_objects_js_core::{
  field_index::{FieldEncoding, FieldIndex},
  js_object::JSObjectExt,
};
use v8_objects_js_func::{
  ContextBuf, ContextExt, Error, FunctionKind, FunctionSyntaxKind, JSFunction, JSFunctionBuf,
  JSFunctionExt, K_EMBEDDER_DATA_INDEX, K_GLOBAL_OBJECT_INDEX, K_GLOBAL_PROXY_INDEX,
  SharedFunctionInfo, SharedFunctionInfoExt, TaggedJSFunctionExt,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

fn create_test_map(instance_type: InstanceType, instance_size_bytes: usize) -> Map {
  Map::new(Tagged::NULL, instance_type, instance_size_bytes)
}

#[test]
fn test_shared_function_info_flags() -> Void {
  info!("Testing SharedFunctionInfo flags and predicates");

  let map = create_test_map(InstanceType::JSObject, 64);
  let map_tagged = Tagged::from_ref(&map);
  let dummy_script = HeapObject { map: map_tagged };
  let dummy_script_tagged = Tagged::from_ref(&dummy_script);

  let sfi = SharedFunctionInfo::new(
    map_tagged,
    Tagged::NULL,
    Tagged::NULL,
    Tagged::NULL,
    dummy_script_tagged,
  );

  // 初始状态
  assert_eq!(sfi.kind(), FunctionKind::NormalFunction);
  assert!(sfi.is_constructable());
  assert!(!sfi.is_async());
  assert!(!sfi.is_generator());
  assert!(!sfi.is_native());
  assert!(!sfi.is_strict());
  assert!(sfi.is_user_javascript());

  // 状态变更与位域独立性验证
  sfi.set_kind(FunctionKind::AsyncFunction);
  assert_eq!(sfi.kind(), FunctionKind::AsyncFunction);
  assert!(sfi.is_async());
  assert!(!sfi.is_constructable());
  assert!(!sfi.is_generator());

  sfi.set_kind(FunctionKind::GeneratorFunction);
  assert_eq!(sfi.kind(), FunctionKind::GeneratorFunction);
  assert!(sfi.is_generator());
  assert!(!sfi.is_async());

  sfi.set_kind(FunctionKind::BaseConstructor);
  assert!(sfi.is_class_constructor());
  assert!(sfi.is_constructable());

  sfi.set_strict(true);
  assert!(sfi.is_strict());

  sfi.set_native(true);
  assert!(sfi.is_native());
  assert!(!sfi.is_user_javascript());

  sfi.set_native(false);
  assert!(sfi.is_user_javascript());

  sfi.set_allows_lazy_compilation(true);
  assert!(sfi.allows_lazy_compilation());

  sfi.set_toplevel(true);
  assert!(sfi.is_toplevel());

  // FunctionSyntaxKind 与 is_wrapped 验证
  assert_eq!(sfi.syntax_kind(), FunctionSyntaxKind::AnonymousExpression);
  assert!(!sfi.is_wrapped());

  sfi.set_syntax_kind(FunctionSyntaxKind::Wrapped);
  assert_eq!(sfi.syntax_kind(), FunctionSyntaxKind::Wrapped);
  assert!(sfi.is_wrapped());

  sfi.set_syntax_kind(FunctionSyntaxKind::Declaration);
  assert_eq!(sfi.syntax_kind(), FunctionSyntaxKind::Declaration);
  assert!(!sfi.is_wrapped());

  // FunctionKind::is_generator 全覆盖区间测试 (16..=21 为 true，其余均为 false)
  for k in 0..=27u8 {
    let kind = FunctionKind::from_u8(k);
    let expected = (16..=21).contains(&k);
    assert_eq!(
      kind.is_generator(),
      expected,
      "Kind {k} is_generator mismatch"
    );
  }

  // 代龄 CAS 操作验证
  assert_eq!(sfi.age(), 0);
  let old_age = sfi.compare_exchange_age(0, 10);
  assert_eq!(old_age, 0);
  assert_eq!(sfi.age(), 10);

  let mismatch = sfi.compare_exchange_age(5, 20);
  assert_eq!(mismatch, 10);
  assert_eq!(sfi.age(), 10);

  OK
}

#[test]
fn test_context_slots_and_hierarchy() -> Void {
  info!("Testing Context slots, allocation and hierarchy traversal");

  let map = create_test_map(InstanceType::JSObject, 16);
  let map_tagged = Tagged::from_ref(&map);

  // 槽位索引规范核对
  assert_eq!(K_GLOBAL_OBJECT_INDEX, 2);
  assert_eq!(K_GLOBAL_PROXY_INDEX, 3);
  assert_eq!(K_EMBEDDER_DATA_INDEX, 4);

  // 1. 创建根 NativeContext (容量 5 槽)
  let mut native_ctx = ContextBuf::new(map_tagged, 5)?;
  assert_eq!(native_ctx.length(), 5);
  assert!(native_ctx.is_native_context()?);

  let scope_info_root = Tagged::new(Smi::from_int(100).ptr());
  native_ctx.set_scope_info(scope_info_root)?;
  assert_eq!(native_ctx.scope_info()?, scope_info_root);

  // 2. 创建中层 ModuleContext (previous 指向 native_ctx)
  let mut module_ctx = ContextBuf::new(map_tagged, 4)?;
  module_ctx.set_previous(Some(native_ctx.as_tagged()))?;
  assert!(!module_ctx.is_native_context()?);
  assert_eq!(
    module_ctx.previous()?.map(|c| c.ptr()),
    Some(native_ctx.as_tagged().ptr())
  );

  // 3. 创建叶子 FunctionContext (previous 指向 module_ctx)
  let mut func_ctx = ContextBuf::new(map_tagged, 3)?;
  func_ctx.set_previous(Some(module_ctx.as_tagged()))?;
  assert_eq!(
    func_ctx.previous()?.map(|c| c.ptr()),
    Some(module_ctx.as_tagged().ptr())
  );

  // 4. 链式溯源至 NativeContext
  let resolved_root = func_ctx.native_context()?;
  assert_eq!(resolved_root.ptr(), native_ctx.as_tagged().ptr());

  // 5. 槽位越界保护
  let err = func_ctx.get(10);
  assert!(matches!(err, Err(Error::IndexOutOfBounds { .. })));

  // 6. 扩展槽位读写
  let ext_val = Tagged::new(Smi::from_int(888).ptr());
  func_ctx.set_extension(ext_val)?;
  assert_eq!(func_ctx.extension()?, ext_val);

  OK
}

#[test]
fn test_js_function_lifecycle() -> Void {
  info!("Testing JSFunction lifecycle and Ext accessors");

  let map = create_test_map(InstanceType::JSObject, 56);
  let map_tagged = Tagged::from_ref(&map);

  let sfi = SharedFunctionInfo::new(
    map_tagged,
    Tagged::NULL,
    Tagged::NULL,
    Tagged::NULL,
    Tagged::NULL,
  );
  let sfi_tagged = Tagged::from_ref(&sfi);

  let ctx = ContextBuf::new(map_tagged, 3)?;
  let ctx_tagged = ctx.as_tagged();

  let dummy_elements = FixedArrayBase {
    header: HeapObject { map: map_tagged },
    length: 0,
  };
  let elements_tagged = Tagged::from_ref(&dummy_elements);

  let mut func = JSFunction::new(
    map_tagged,
    Tagged::NULL,
    elements_tagged,
    0, // dispatch_handle = 0 (未编译)
    sfi_tagged,
    ctx_tagged,
    Tagged::NULL,
  );

  assert!(!func.is_compiled());
  assert_eq!(func.dispatch_handle(), 0);
  assert_eq!(func.shared().ptr(), sfi_tagged.ptr());
  assert_eq!(func.context().ptr(), ctx_tagged.ptr());
  assert_eq!(func.feedback_cell(), Tagged::NULL);

  // 派发句柄更新与编译状态同步
  func.set_dispatch_handle(42);
  assert!(func.is_compiled());
  assert_eq!(func.dispatch_handle(), 42);

  // 反馈单元读写
  let fb_cell = Tagged::new(Smi::from_int(777).ptr());
  func.set_feedback_cell(fb_cell);
  assert_eq!(func.feedback_cell(), fb_cell);

  // 无原型槽位的 JSFunction 调用 prototype_or_initial_map 必须安全返回错误
  assert!(!func.has_prototype_slot());
  assert!(matches!(
    func.prototype_or_initial_map(),
    Err(Error::NoPrototypeSlot)
  ));

  OK
}

#[test]
fn test_js_function_buf_with_prototype_and_in_object_props() -> Void {
  info!("Testing JSFunctionBuf with prototype and in-object properties");

  // 实例尺寸：JSFunctionWithPrototype 头 (64 字节) + 2 个对象内属性 (16 字节) = 80 字节 (10 words)
  let mut map = create_test_map(InstanceType::JSObject, 80);
  map.instance_size_in_words = 10;
  let map_tagged = Tagged::from_ref(&map);

  let sfi = SharedFunctionInfo::new(
    map_tagged,
    Tagged::NULL,
    Tagged::NULL,
    Tagged::NULL,
    Tagged::NULL,
  );
  let sfi_tagged = Tagged::from_ref(&sfi);

  let ctx = ContextBuf::new(map_tagged, 3)?;
  let ctx_tagged = ctx.as_tagged();

  let dummy_elements = FixedArrayBase {
    header: HeapObject { map: map_tagged },
    length: 0,
  };
  let elements_tagged = Tagged::from_ref(&dummy_elements);

  let base_func = JSFunction::new(
    map_tagged,
    Tagged::NULL,
    elements_tagged,
    100,
    sfi_tagged,
    ctx_tagged,
    Tagged::NULL,
  );

  // 创建带 prototype 槽位及 2 个 in-object 属性的独立内存容器
  let mut func_buf = JSFunctionBuf::new(base_func, true, 2)?;
  assert_eq!(func_buf.in_object_count(), 2);
  assert!(func_buf.has_prototype_slot());
  assert!(func_buf.is_compiled());
  assert_eq!(func_buf.dispatch_handle(), 100);

  // 1. Prototype / initial_map 槽位读写
  let proto_obj = Tagged::new(Smi::from_int(9999).ptr());
  func_buf.set_prototype_or_initial_map(proto_obj)?;
  assert_eq!(func_buf.prototype_or_initial_map()?, proto_obj);

  // 2. 对象内属性读写测试
  // 第一个对象内属性槽位位于 JSObject 头部之后 (offset = 64 字节)
  let field_idx_0 = FieldIndex::for_in_object_offset(64, FieldEncoding::Tagged);
  let val_0 = Tagged::new(Smi::from_int(123).ptr());
  func_buf.fast_property_at_put(field_idx_0, val_0)?;
  assert_eq!(func_buf.raw_fast_property_at(field_idx_0)?, val_0);

  // 第二个对象内属性槽位 (offset = 72 字节)
  let field_idx_1 = FieldIndex::for_in_object_offset(72, FieldEncoding::Tagged);
  let val_1 = Tagged::new(Smi::from_int(456).ptr());
  func_buf.fast_property_at_put(field_idx_1, val_1)?;
  assert_eq!(func_buf.raw_fast_property_at(field_idx_1)?, val_1);

  // 3. 辅助转换方法校验
  let tagged_func = func_buf.as_tagged();
  assert_eq!(tagged_func.as_object().ptr(), tagged_func.ptr());
  assert_eq!(tagged_func.as_receiver().ptr(), tagged_func.ptr());

  OK
}

#[test]
fn test_tagged_js_function_dynamic_prototype_slot() -> Void {
  info!("Testing Tagged<JSFunction>::has_prototype_slot dynamic check and prototype r/w");

  // 1. 无原型槽位的 Map
  let map_without_proto = create_test_map(InstanceType::JSFunctionWithoutPrototype, 56);
  let map_without_proto_tagged = Tagged::from_ref(&map_without_proto);

  let func_without = JSFunction::new(
    map_without_proto_tagged,
    Tagged::NULL,
    Tagged::NULL,
    1,
    Tagged::NULL,
    Tagged::NULL,
    Tagged::NULL,
  );
  let tagged_without = Tagged::from_ref(&func_without);
  assert!(!tagged_without.has_prototype_slot());
  assert!(matches!(
    tagged_without.prototype_or_initial_map(),
    Err(Error::NoPrototypeSlot)
  ));

  // 2. 带原型槽位的 Map
  let map_with_proto = create_test_map(InstanceType::JSFunctionWithPrototype, 64);
  let map_with_proto_tagged = Tagged::from_ref(&map_with_proto);

  let func_with = JSFunction::new(
    map_with_proto_tagged,
    Tagged::NULL,
    Tagged::NULL,
    2,
    Tagged::NULL,
    Tagged::NULL,
    Tagged::NULL,
  );
  // 通过 JSFunctionBuf 分配包含 prototype 槽位的内存
  let func_buf = JSFunctionBuf::new(func_with, true, 0)?;
  let tagged_with = func_buf.as_tagged();
  assert!(tagged_with.has_prototype_slot());

  // 验证通过 Tagged 指针读取与写入 prototype_or_initial_map
  let dummy_proto = Tagged::new(Smi::from_int(777).ptr());
  let mut tagged_mut = tagged_with;
  tagged_mut.set_prototype_or_initial_map(dummy_proto)?;
  assert_eq!(tagged_mut.prototype_or_initial_map()?, dummy_proto);

  OK
}
