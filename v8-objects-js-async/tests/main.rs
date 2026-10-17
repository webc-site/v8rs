// v8-objects-js-async/tests/main.rs
// JSPromise 与 JSGeneratorObject 核心异步对象族综合单元测试

use core::ffi::c_void;

use aok::{OK, Void};
use log::info;
use v8_objects_arrays::{fixed_array::FixedArray, fixed_array_base::FixedArrayBase};
use v8_objects_core::{
  heap_object::HeapObject, instance_type::InstanceType, map::Map, smi::Smi, tagged::Tagged,
};
use v8_objects_js_async::{
  AsyncGeneratorRequest, AsyncGeneratorRequestBuf, AsyncGeneratorRequestExt, Error,
  GeneratorFields, JSAsyncFunctionExt, JSAsyncFunctionObject, JSAsyncFunctionObjectBuf,
  JSAsyncGeneratorExt, JSAsyncGeneratorObject, JSAsyncGeneratorObjectBuf, JSGeneratorExt,
  JSGeneratorObject, JSGeneratorObjectBuf, JSPromiseBuf, JSPromiseExt, K_GENERATOR_CLOSED,
  K_GENERATOR_EXECUTING, K_MAX_ASYNC_FUNCTION_IN_OBJECT_PROPERTIES,
  K_MAX_ASYNC_GENERATOR_IN_OBJECT_PROPERTIES, K_MAX_ASYNC_TASK_ID,
  K_MAX_GENERATOR_IN_OBJECT_PROPERTIES, K_MAX_PROMISE_IN_OBJECT_PROPERTIES, PromiseState,
  ResumeMode, TaggedJSGeneratorExt,
  backing_store_cc::{
    AllocationStatus, BackingStore, BackingStoreInit, K_FULL_GUARD_SIZE32, K_MAX_MEMORY64_SIZE,
    K_NEXT_BACKING_STORE_ID_INIT, WasmMemoryFlag,
  },
  get_next_async_task_id,
  intl_objects::Notation,
  managed::{CppGCManagedBase, CppGCManagedWrapper, ExternalMemoryAccounter, ManagedPtrDestructor},
  option_utils::{
    CoercedOptionValue, default_number_option, get_number_option, get_number_option_as_double,
    get_string_option_enum, get_string_or_boolean_option,
  },
  value_serializer_skel::{ValueDeserializerBuf, ValueSerializerBuf},
};
use v8_objects_js_func::{ContextBuf, JSFunction, SharedFunctionInfo};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

fn create_test_map(instance_type: InstanceType, instance_size_bytes: usize) -> Map {
  Map::new(Tagged::NULL, instance_type, instance_size_bytes)
}

#[test]
fn test_promise_flags_and_state() -> Void {
  info!("Testing JSPromise flags, bitfields, and PromiseState");

  let map = create_test_map(InstanceType::JSPromise, 40);
  let map_tagged = Tagged::from_ref(&map);
  let dummy_elements = FixedArrayBase {
    header: HeapObject { map: map_tagged },
    length: 0,
  };
  let elements_tagged = Tagged::from_ref(&dummy_elements);

  let mut promise = JSPromiseBuf::new(
    map_tagged,
    Tagged::NULL,
    elements_tagged,
    Tagged::NULL,
    0, // initial flags: pending, no handler, not silent, no async task id
    0,
  )?;

  // 1. 初始状态
  assert_eq!(promise.status(), PromiseState::Pending);
  assert!(!promise.has_handler());
  assert!(!promise.is_silent());
  assert!(!promise.is_native_resolver_invoked());
  assert!(!promise.has_async_task_id());
  assert_eq!(promise.async_task_id(), 0);

  // 2. 位域标志设置与独立性检验
  promise.set_has_handler(true);
  assert!(promise.has_handler());
  assert_eq!(promise.status(), PromiseState::Pending);

  promise.set_is_silent(true);
  assert!(promise.is_silent());
  assert!(promise.has_handler());

  promise.set_is_native_resolver_invoked(true);
  assert!(promise.is_native_resolver_invoked());
  assert!(promise.is_silent());
  assert!(promise.has_handler());

  // 3. 异步任务 ID 读写与掩码截断
  promise.set_async_task_id(65535);
  assert!(promise.has_async_task_id());
  assert_eq!(promise.async_task_id(), 65535);
  assert!(promise.is_native_resolver_invoked());
  assert!(promise.is_silent());
  assert!(promise.has_handler());
  assert_eq!(promise.status(), PromiseState::Pending);

  // 4. 最大边界异步任务 ID 与全部位域标志共存
  promise.set_has_handler(true);
  promise.set_is_silent(true);
  promise.set_is_native_resolver_invoked(true);
  promise.set_async_task_id(K_MAX_ASYNC_TASK_ID);
  assert!(promise.has_async_task_id());
  assert_eq!(promise.async_task_id(), K_MAX_ASYNC_TASK_ID);
  assert!(promise.has_handler());
  assert!(promise.is_silent());
  assert!(promise.is_native_resolver_invoked());
  assert_eq!(promise.status(), PromiseState::Pending);

  // 清除标志
  promise.set_has_handler(false);
  assert!(!promise.has_handler());
  assert!(promise.is_silent());
  assert!(promise.is_native_resolver_invoked());
  assert_eq!(promise.async_task_id(), K_MAX_ASYNC_TASK_ID);

  // 5. get_next_async_task_id 回绕与跳零测试
  assert_eq!(get_next_async_task_id(0), 1);
  assert_eq!(get_next_async_task_id(100), 101);
  assert_eq!(get_next_async_task_id(K_MAX_ASYNC_TASK_ID), 1);

  // 5. PromiseState 枚举与转换测试
  assert_eq!(PromiseState::try_from(0)?, PromiseState::Pending);
  assert_eq!(PromiseState::try_from(1)?, PromiseState::Fulfilled);
  assert_eq!(PromiseState::try_from(2)?, PromiseState::Rejected);
  assert!(matches!(
    PromiseState::try_from(3),
    Err(Error::InvalidPromiseState(3))
  ));
  assert_eq!(PromiseState::Pending.as_str(), "pending");
  assert_eq!(PromiseState::Fulfilled.as_str(), "fulfilled");
  assert_eq!(PromiseState::Rejected.as_str(), "rejected");

  OK
}

#[test]
fn test_promise_lifecycle_and_reactions() -> Void {
  info!("Testing JSPromise lifecycle, reactions and resolution");

  let map = create_test_map(InstanceType::JSPromise, 40);
  let map_tagged = Tagged::from_ref(&map);
  let dummy_elements = FixedArrayBase {
    header: HeapObject { map: map_tagged },
    length: 0,
  };
  let elements_tagged = Tagged::from_ref(&dummy_elements);

  // 1. Pending 状态下挂载 reaction
  let dummy_reaction = Tagged::new(Smi::from_int(777).ptr());
  let mut promise = JSPromiseBuf::new(
    map_tagged,
    Tagged::NULL,
    elements_tagged,
    dummy_reaction,
    0,
    2,
  )?;

  assert_eq!(promise.status(), PromiseState::Pending);
  assert_eq!(promise.reactions()?, dummy_reaction);
  // Pending 状态下读取 result 必须报错
  assert!(matches!(promise.result(), Err(Error::PromiseStillPending)));

  // 重置回 Pending 属于非法操作
  assert!(matches!(
    promise.set_status(PromiseState::Pending),
    Err(Error::Invariant(_))
  ));

  // 2. Fulfill 结算流程
  let resolve_val = Tagged::new(Smi::from_int(9999).ptr());
  promise.fulfill(resolve_val)?;

  assert_eq!(promise.status(), PromiseState::Fulfilled);
  assert_eq!(promise.result()?, resolve_val);
  // 结算后读取 reactions 必须报错
  assert!(matches!(promise.reactions(), Err(Error::PromiseNotPending)));

  // 已结算 Promise 再次 Fulfill 或 Reject 必须报错
  assert!(matches!(
    promise.fulfill(resolve_val),
    Err(Error::PromiseNotPending)
  ));
  assert!(matches!(
    promise.reject(resolve_val),
    Err(Error::PromiseNotPending)
  ));

  // 3. Reject 结算流程
  let mut rejected_promise = JSPromiseBuf::new(
    map_tagged,
    Tagged::NULL,
    elements_tagged,
    Tagged::NULL,
    0,
    0,
  )?;
  let error_reason = Tagged::new(Smi::from_int(500).ptr());
  rejected_promise.reject(error_reason)?;

  assert_eq!(rejected_promise.status(), PromiseState::Rejected);
  assert_eq!(rejected_promise.result()?, error_reason);
  assert!(matches!(
    rejected_promise.reactions(),
    Err(Error::PromiseNotPending)
  ));

  OK
}

#[test]
fn test_generator_lifecycle_and_continuation() -> Void {
  info!("Testing JSGeneratorObject continuation, states, and ResumeMode");

  let map = create_test_map(InstanceType::JSGeneratorObject, 88);
  let map_tagged = Tagged::from_ref(&map);
  let dummy_elements = FixedArrayBase {
    header: HeapObject { map: map_tagged },
    length: 0,
  };
  let elements_tagged = Tagged::from_ref(&dummy_elements);

  let dummy_registers = FixedArray::new(map_tagged, 4);
  let registers_tagged = Tagged::from_ref(&dummy_registers);

  let ctx_buf = ContextBuf::new(map_tagged, 3)?;
  let ctx_tagged = ctx_buf.as_tagged();

  let sfi = SharedFunctionInfo::new(
    map_tagged,
    Tagged::NULL,
    Tagged::NULL,
    Tagged::NULL,
    Tagged::NULL,
  );
  let sfi_tagged = Tagged::from_ref(&sfi);

  let js_func = JSFunction::new(
    map_tagged,
    Tagged::NULL,
    elements_tagged,
    0,
    sfi_tagged,
    ctx_tagged,
    Tagged::NULL,
  );
  let js_func_tagged = Tagged::from_ref(&js_func);

  let generator = JSGeneratorObject::from_parts(
    map_tagged,
    Tagged::NULL,
    elements_tagged,
    GeneratorFields {
      function: js_func_tagged,
      context: ctx_tagged,
      parameters_and_registers: registers_tagged,
      ..Default::default()
    },
  );

  let mut gen_buf = JSGeneratorObjectBuf::new(generator, 4)?;

  // 1. 初始状态：挂起
  assert!(gen_buf.is_suspended());
  assert!(!gen_buf.is_closed());
  assert!(!gen_buf.is_executing());
  assert_eq!(gen_buf.continuation(), 0);
  assert_eq!(gen_buf.resume_mode()?, ResumeMode::Next);

  // 2. 状态转换为执行中
  gen_buf.mark_executing();
  assert!(gen_buf.is_executing());
  assert!(!gen_buf.is_suspended());
  assert!(!gen_buf.is_closed());
  assert_eq!(gen_buf.continuation(), K_GENERATOR_EXECUTING);

  // 3. 产生值并挂起在字节码位置 42
  let yield_val = Tagged::new(Smi::from_int(1234).ptr());
  gen_buf.set_yielded_value(yield_val);
  gen_buf.suspend_at(42)?;
  assert!(gen_buf.is_suspended());
  assert_eq!(gen_buf.continuation(), 42);
  assert_eq!(gen_buf.yielded_value(), yield_val);

  // 挂起在负数偏移是非法的
  assert!(matches!(gen_buf.suspend_at(-5), Err(Error::Invariant(_))));

  // 4. ResumeMode 切换
  gen_buf.set_resume_mode(ResumeMode::Throw);
  assert_eq!(gen_buf.resume_mode()?, ResumeMode::Throw);
  assert_eq!(gen_buf.resume_mode()?.as_str(), "throw");

  gen_buf.set_resume_mode(ResumeMode::Return);
  assert_eq!(gen_buf.resume_mode()?, ResumeMode::Return);
  assert_eq!(gen_buf.resume_mode()?.as_str(), "return");

  gen_buf.set_resume_mode(ResumeMode::Rethrow);
  assert_eq!(gen_buf.resume_mode()?, ResumeMode::Rethrow);

  // 非法 ResumeMode 校验
  assert!(matches!(
    ResumeMode::try_from(100),
    Err(Error::InvalidResumeMode(100))
  ));

  // 5. 关闭生成器
  gen_buf.mark_closed();
  assert!(gen_buf.is_closed());
  assert!(!gen_buf.is_suspended());
  assert!(!gen_buf.is_executing());
  assert_eq!(gen_buf.continuation(), K_GENERATOR_CLOSED);

  OK
}

#[test]
fn test_async_generator_queue_and_awaiting() -> Void {
  info!("Testing JSAsyncGeneratorObject queue linking and awaiting status");

  let gen_map = create_test_map(InstanceType::JSAsyncGeneratorObject, 104);
  let gen_map_tagged = Tagged::from_ref(&gen_map);
  let dummy_elements = FixedArrayBase {
    header: HeapObject {
      map: gen_map_tagged,
    },
    length: 0,
  };
  let elements_tagged = Tagged::from_ref(&dummy_elements);
  let dummy_registers = FixedArray::new(gen_map_tagged, 0);
  let registers_tagged = Tagged::from_ref(&dummy_registers);

  let struct_map = create_test_map(InstanceType::AsyncGeneratorRequest, 40);
  let struct_map_tagged = Tagged::from_ref(&struct_map);

  let promise_buf = JSPromiseBuf::new(
    struct_map_tagged,
    Tagged::NULL,
    elements_tagged,
    Tagged::NULL,
    0,
    0,
  )?;

  // 1. 构造异步请求队列链表: req1 -> req2
  let req2 = AsyncGeneratorRequestBuf::new(AsyncGeneratorRequest::new(
    struct_map_tagged,
    Tagged::NULL,
    ResumeMode::Return as i32,
    Tagged::new(Smi::from_int(200).ptr()),
    promise_buf.as_tagged(),
  ));
  let req2_tagged = Tagged::new(req2.ptr());

  let req1 = AsyncGeneratorRequestBuf::new(AsyncGeneratorRequest::new(
    struct_map_tagged,
    req2_tagged,
    ResumeMode::Next as i32,
    Tagged::new(Smi::from_int(100).ptr()),
    promise_buf.as_tagged(),
  ));
  let req1_tagged = Tagged::new(req1.ptr());

  // 验证 req1 / req2 字段
  assert_eq!(req1.resume_mode()?, ResumeMode::Next);
  assert_eq!(req1.value(), Tagged::new(Smi::from_int(100).ptr()));
  assert_eq!(req1.next(), req2_tagged);
  assert_eq!(req2.resume_mode()?, ResumeMode::Return);

  // 2. 构造 JSAsyncGeneratorObject
  let base_gen = JSGeneratorObject::from_parts(
    gen_map_tagged,
    Tagged::NULL,
    elements_tagged,
    GeneratorFields {
      parameters_and_registers: registers_tagged,
      ..Default::default()
    },
  );

  let async_gen = JSAsyncGeneratorObject::new(base_gen, req1_tagged, false);
  let mut async_gen_buf = JSAsyncGeneratorObjectBuf::new(async_gen, 2)?;

  // 验证 is_awaiting 与 queue
  assert!(!async_gen_buf.is_awaiting());
  async_gen_buf.set_is_awaiting(true);
  assert!(async_gen_buf.is_awaiting());

  assert_eq!(async_gen_buf.queue(), req1_tagged);
  // 推进队列至 req2
  async_gen_buf.set_queue(req1.next());
  assert_eq!(async_gen_buf.queue(), req2_tagged);

  // 验证继承的 JSGeneratorExt 操作与 as_generator 向上转换
  assert!(async_gen_buf.is_suspended());
  assert!(async_gen_buf.as_generator().is_suspended());
  async_gen_buf.mark_executing();
  assert!(async_gen_buf.is_executing());
  assert!(async_gen_buf.as_generator().is_executing());

  OK
}

#[test]
fn test_async_function_object() -> Void {
  info!("Testing JSAsyncFunctionObject closures and promise bindings");

  let map = create_test_map(InstanceType::JSAsyncFunctionObject, 112);
  let map_tagged = Tagged::from_ref(&map);
  let dummy_elements = FixedArrayBase {
    header: HeapObject { map: map_tagged },
    length: 0,
  };
  let elements_tagged = Tagged::from_ref(&dummy_elements);
  let dummy_registers = FixedArray::new(map_tagged, 0);
  let registers_tagged = Tagged::from_ref(&dummy_registers);

  let promise_buf = JSPromiseBuf::new(
    map_tagged,
    Tagged::NULL,
    elements_tagged,
    Tagged::NULL,
    0,
    0,
  )?;

  let base_gen = JSGeneratorObject::from_parts(
    map_tagged,
    Tagged::NULL,
    elements_tagged,
    GeneratorFields {
      parameters_and_registers: registers_tagged,
      ..Default::default()
    },
  );

  let resolve_closure = Tagged::new(Smi::from_int(111).ptr());
  let reject_closure = Tagged::new(Smi::from_int(222).ptr());

  let async_func = JSAsyncFunctionObject::new(
    base_gen,
    promise_buf.as_tagged(),
    resolve_closure,
    reject_closure,
  );
  let mut async_func_buf = JSAsyncFunctionObjectBuf::new(async_func, 2)?;

  assert_eq!(async_func_buf.promise(), promise_buf.as_tagged());
  assert_eq!(async_func_buf.await_resolve_closure(), resolve_closure);
  assert_eq!(async_func_buf.await_reject_closure(), reject_closure);
  assert_eq!(async_func_buf.in_object_count(), 2);

  // 闭包变更
  let new_resolve = Tagged::new(Smi::from_int(333).ptr());
  async_func_buf.set_await_resolve_closure(new_resolve);
  assert_eq!(async_func_buf.await_resolve_closure(), new_resolve);

  // 挂起状态与 as_generator 向上转换
  assert!(async_func_buf.is_suspended());
  assert!(async_func_buf.as_generator().is_suspended());
  async_func_buf.mark_closed();
  assert!(async_func_buf.is_closed());
  assert!(async_func_buf.as_generator().is_closed());

  OK
}

#[test]
fn test_capacity_boundaries() -> Void {
  info!("Testing in-object property capacity boundaries");

  let map = create_test_map(InstanceType::JSPromise, 40);
  let map_tagged = Tagged::from_ref(&map);
  let dummy_elements = FixedArrayBase {
    header: HeapObject { map: map_tagged },
    length: 0,
  };
  let elements_tagged = Tagged::from_ref(&dummy_elements);

  // 1. Promise 超额容量
  let res_promise = JSPromiseBuf::new(
    map_tagged,
    Tagged::NULL,
    elements_tagged,
    Tagged::NULL,
    0,
    K_MAX_PROMISE_IN_OBJECT_PROPERTIES + 1,
  );
  assert!(matches!(res_promise, Err(Error::InvalidCapacity(_))));

  // 2. Generator 超额容量
  let dummy_registers = FixedArray::new(map_tagged, 0);
  let registers_tagged = Tagged::from_ref(&dummy_registers);
  let base_gen = JSGeneratorObject::from_parts(
    map_tagged,
    Tagged::NULL,
    elements_tagged,
    GeneratorFields {
      parameters_and_registers: registers_tagged,
      ..Default::default()
    },
  );
  let res_gen = JSGeneratorObjectBuf::new(base_gen, K_MAX_GENERATOR_IN_OBJECT_PROPERTIES + 1);
  assert!(matches!(res_gen, Err(Error::InvalidCapacity(_))));

  // 3. AsyncGenerator 超额容量
  let async_gen = JSAsyncGeneratorObject::new(base_gen, Tagged::NULL, false);
  let res_async_gen =
    JSAsyncGeneratorObjectBuf::new(async_gen, K_MAX_ASYNC_GENERATOR_IN_OBJECT_PROPERTIES + 1);
  assert!(matches!(res_async_gen, Err(Error::InvalidCapacity(_))));

  // 4. AsyncFunction 超额容量
  let promise_buf = JSPromiseBuf::new(
    map_tagged,
    Tagged::NULL,
    elements_tagged,
    Tagged::NULL,
    0,
    0,
  )?;
  let async_func = JSAsyncFunctionObject::new(
    base_gen,
    promise_buf.as_tagged(),
    Tagged::NULL,
    Tagged::NULL,
  );
  let res_async_func =
    JSAsyncFunctionObjectBuf::new(async_func, K_MAX_ASYNC_FUNCTION_IN_OBJECT_PROPERTIES + 1);
  assert!(matches!(res_async_func, Err(Error::InvalidCapacity(_))));

  OK
}

#[test]
fn test_backing_store_reservation_and_value_semantics() -> Void {
  info!("Testing BackingStore reservation formula, ctor flags and wrap/empty paths");

  // 1. GetWasmReservationSize 公式覆盖 wasm32 全 guard / wasm64+arm64 附加页 / 非 guard 直返
  const PAGE: usize = 0x4000; // 16K (macOS arm64)
  assert_eq!(
    BackingStore::get_wasm_reservation_size(
      true,
      4 * 1024 * 1024 * 1024,
      WasmMemoryFlag::WasmMemory32,
      PAGE
    ),
    K_FULL_GUARD_SIZE32
  );
  assert_eq!(
    BackingStore::get_wasm_reservation_size(
      true,
      K_MAX_MEMORY64_SIZE,
      WasmMemoryFlag::WasmMemory64,
      PAGE
    ),
    K_MAX_MEMORY64_SIZE + PAGE,
  );
  assert_eq!(
    BackingStore::get_wasm_reservation_size(false, 4096, WasmMemoryFlag::NotWasm, PAGE),
    4096,
  );

  // 2. EmptyBackingStore: 空态、非 shared / 非 wasm / 非 resizable
  let empty = BackingStore::empty_backing_store(false);
  assert!(empty.is_empty());
  assert_eq!(empty.byte_capacity(), 0);
  assert_eq!(empty.byte_length(), 0);
  assert!(!empty.is_shared());
  assert!(!empty.is_wasm_memory());
  assert!(!empty.is_resizable_by_js());
  assert_eq!(empty.wasm_memory_flag(), WasmMemoryFlag::NotWasm);

  // 3. Shared 空态: is_shared 位翻转
  let shared_empty = BackingStore::empty_backing_store(true);
  assert!(shared_empty.is_shared());

  // 4. id 单调递增 (K_NEXT_BACKING_STORE_ID_INIT 起)
  assert!(empty.id() >= K_NEXT_BACKING_STORE_ID_INIT);
  assert!(shared_empty.id() > empty.id());

  // 5. WrapAllocation: CustomDeleter 位落, EmptyDeleter 位根据 fn Option 语义识别
  extern "C" fn dummy_deleter(_d: *mut c_void, _l: usize, _dd: *mut c_void) {}
  let wrapped = BackingStore::wrap_allocation(None, 64, Some(dummy_deleter), None, false);
  assert_eq!(wrapped.byte_length(), 64);
  assert_eq!(wrapped.max_byte_length(), 64);
  assert_eq!(wrapped.byte_capacity(), 64);
  assert!(!wrapped.is_shared());
  assert!(!wrapped.is_resizable_by_js());
  // 非 EmptyDeleter → 计入外部内存压力
  assert_eq!(wrapped.per_isolate_accounting_length(), 64);

  // 6. WrapAllocation with EmptyDeleter → EmptyDeleter 位置, 外部内存不计
  let wrapped_empty_del = BackingStore::wrap_allocation(None, 32, None, None, false);
  assert_eq!(wrapped_empty_del.per_isolate_accounting_length(), 0);
  // Flag::EmptyDeleter 由 wrap 路径写入 (位标读回校核)
  assert_eq!(wrapped_empty_del.byte_length(), 32);

  // 7. MakeWasmMemoryResizableByJS 语义 (仅非 shared wasm 内存):
  //    直接经 ctor 构造 unshared wasm32 backing store, 再翻位标
  //    byte_capacity != 0 满足 CHECK_IMPLIES(is_wasm_memory, byte_capacity != 0)
  let mut wasm_bs = BackingStore::new(BackingStoreInit {
    buffer_start: None,
    byte_length: 4096,
    max_byte_length: 4096,
    byte_capacity: 4096,
    shared: false,
    resizable: false,
    immutable: false,
    wasm_memory: WasmMemoryFlag::WasmMemory32,
    has_guard_regions: false,
    custom_deleter: false,
    empty_deleter: false,
  });
  assert!(wasm_bs.is_wasm_memory());
  assert!(!wasm_bs.is_wasm_memory64());
  assert!(!wasm_bs.is_resizable_by_js());
  wasm_bs.make_wasm_memory_resizable_by_js(true);
  assert!(wasm_bs.is_resizable_by_js());
  wasm_bs.make_wasm_memory_resizable_by_js(false);
  assert!(!wasm_bs.is_resizable_by_js());
  // wasm 内存的 per_isolate_accounting_length 恒为 0 (原 .h)
  assert_eq!(wasm_bs.per_isolate_accounting_length(), 0);

  // 8. AllocationStatus 枚举不变式
  assert_eq!(AllocationStatus::COUNT, 4);
  assert_eq!(AllocationStatus::Success as u8, 0);
  assert_eq!(AllocationStatus::AddressSpaceLimitReachedFailure as u8, 2);
  assert_eq!(AllocationStatus::OtherFailure as u8, 3);

  // 9. Guard Regions 与 Immutable 位标检验
  let guarded_bs = BackingStore::new(BackingStoreInit {
    buffer_start: None,
    byte_length: 4096,
    max_byte_length: 4096,
    byte_capacity: 4096,
    shared: false,
    resizable: false,
    immutable: false,
    wasm_memory: WasmMemoryFlag::WasmMemory32,
    has_guard_regions: true,
    custom_deleter: false,
    empty_deleter: false,
  });
  assert!(guarded_bs.has_guard_regions());
  assert!(guarded_bs.is_wasm_memory());

  let immutable_bs = BackingStore::new(BackingStoreInit {
    buffer_start: None,
    byte_length: 128,
    max_byte_length: 128,
    byte_capacity: 128,
    shared: false,
    resizable: false,
    immutable: true,
    wasm_memory: WasmMemoryFlag::NotWasm,
    has_guard_regions: false,
    custom_deleter: false,
    empty_deleter: false,
  });
  assert!(immutable_bs.is_immutable());

  OK
}

// 计账替身: 累计 Increase/Decrease 差额, 校验 managed 收支核语义
#[derive(Debug, Default)]
struct CountingAccounter {
  inc: usize,
  dec: usize,
}

impl ExternalMemoryAccounter for CountingAccounter {
  fn increase(&mut self, delta: usize) {
    self.inc += delta;
  }
  fn decrease(&mut self, delta: usize) {
    self.dec += delta;
  }
}

#[test]
fn test_managed_external_memory_accounting() -> Void {
  info!("Testing CppGCManagedWrapper/ManagedPtrDestructor UpdateEstimatedSize 收支核");

  // 1. ctor: estimated_size > 0 且 isolate 存活即 Increase (managed.h CppGCManagedWrapper)
  let w = CppGCManagedWrapper::new(7, 100, 0x1000, None, true, CountingAccounter::default());
  assert_eq!(w.type_id(), 7);
  assert_eq!(w.estimated_size(), 100);
  assert_eq!(w.shared_ptr_ptr(), 0x1000);
  assert_eq!(w.external_memory_accounter.inc, 100);
  assert_eq!(w.external_memory_accounter.dec, 0);

  // 2. isolate 不存活或 estimated_size 为 0 → 不计
  let w_dead = CppGCManagedWrapper::new(1, 50, 0, None, false, CountingAccounter::default());
  assert_eq!(w_dead.external_memory_accounter.inc, 0);
  let w_zero = CppGCManagedWrapper::new(1, 0, 0, None, true, CountingAccounter::default());
  assert_eq!(w_zero.external_memory_accounter.inc, 0);

  // 3. UpdateEstimatedSize: 等值早返; 增大记差额 Increase; 减小记差额 Decrease
  let mut w = w;
  w.update_estimated_size(100); // 等值 → 无变化
  assert_eq!(w.external_memory_accounter.inc, 100);
  assert_eq!(w.external_memory_accounter.dec, 0);
  w.update_estimated_size(140); // +40 → Increase(40)
  assert_eq!(w.external_memory_accounter.inc, 140);
  assert_eq!(w.estimated_size(), 140);
  w.update_estimated_size(90); // -50 → Decrease(50)
  assert_eq!(w.external_memory_accounter.dec, 50);
  assert_eq!(w.estimated_size(), 90);

  // 4. ManagedPtrDestructor 同构收支核 (managed.cc:34-47)
  let mut node = ManagedPtrDestructor {
    estimated_size: 20,
    prev: 0,
    next: 0,
    shared_ptr_ptr: 0,
    destructor: None,
    shared: false,
    global_handle_location: 0,
    external_memory_accounter: CountingAccounter::default(),
  };
  node.update_estimated_size(35);
  assert_eq!(node.external_memory_accounter.inc, 15);
  assert_eq!(node.estimated_size, 35);
  node.update_estimated_size(10);
  assert_eq!(node.external_memory_accounter.dec, 25);
  assert_eq!(node.estimated_size, 10);

  // 5. CppGCManagedBase::header_size 布局公式 (heap_object + cpp_heap_pointer)
  assert_eq!(CppGCManagedBase::header_size(8, 4), 12);

  OK
}

#[test]
fn test_option_utils_pure_cores() -> Void {
  info!(
    "Testing ECMA402 option-utils 纯语义核 (DefaultNumberOption / GetOption 枚举 / StringOrBoolean)"
  );

  // DefaultNumberOption: undefined→fallback; floor 收敛; NaN/越界→RangeError
  assert_eq!(default_number_option(None, 0, 10, 5, "p")?, 5);
  assert_eq!(default_number_option(Some(3.7), 0, 10, 5, "p")?, 3);
  assert_eq!(default_number_option(Some(-2.0), -5, 10, 5, "p")?, -2);
  assert!(matches!(
    default_number_option(Some(f64::NAN), 0, 10, 5, "p"),
    Err(Error::PropertyValueOutOfRange("p"))
  ));
  assert!(matches!(
    default_number_option(Some(11.0), 0, 10, 5, "p"),
    Err(Error::PropertyValueOutOfRange("p"))
  ));

  // GetNumberOption 转发校验核 (Get 面由上层承接)
  assert_eq!(get_number_option(Some(7.2), 0, 100, 1, "n")?, 7);

  // GetNumberOptionAsDouble: undefined→default; NaN→RangeError; 否则原值
  assert_eq!(get_number_option_as_double(None, 3.5, "d")?, 3.5);
  assert_eq!(get_number_option_as_double(Some(2.25), 3.5, "d")?, 2.25);
  assert!(matches!(
    get_number_option_as_double(Some(f64::NAN), 3.5, "d"),
    Err(Error::PropertyValueOutOfRange("d"))
  ));

  // GetStringOption<T> 枚举匹配 (str_values ↔ enum_values 等长)
  let strs = ["compact", "scientific", "standard"];
  let enums = [Notation::COMPACT, Notation::SCIENTIFIC, Notation::STANDARD];
  assert_eq!(
    get_string_option_enum(Some("scientific"), &strs, &enums, None, "notation")?,
    Notation::SCIENTIFIC
  );
  assert_eq!(
    get_string_option_enum(None, &strs, &enums, Some(Notation::STANDARD), "notation")?,
    Notation::STANDARD
  );
  assert!(matches!(
    get_string_option_enum(Some("nope"), &strs, &enums, None, "notation"),
    Err(Error::ValueOutOfRange("notation"))
  ));

  // GetStringOrBooleanOption<T>: undefined→fallback / true→true / false→false / "true"/"false"串→fallback / 枚举匹配
  let bstrs = ["constrain", "expand"];
  let benums = [10i32, 20i32];
  let (tval, fval, fb) = (1i32, 0i32, -1i32);
  assert_eq!(
    get_string_or_boolean_option(
      CoercedOptionValue::Undefined,
      &bstrs,
      &benums,
      tval,
      fval,
      fb,
      "tz"
    )?,
    fb
  );
  assert_eq!(
    get_string_or_boolean_option(
      CoercedOptionValue::True,
      &bstrs,
      &benums,
      tval,
      fval,
      fb,
      "tz"
    )?,
    tval
  );
  assert_eq!(
    get_string_or_boolean_option(
      CoercedOptionValue::Falsy,
      &bstrs,
      &benums,
      tval,
      fval,
      fb,
      "tz"
    )?,
    fval
  );
  // "true"/"false" 字符串 → fallback (原版 step 7)
  assert_eq!(
    get_string_or_boolean_option(
      CoercedOptionValue::Str("true"),
      &bstrs,
      &benums,
      tval,
      fval,
      fb,
      "tz"
    )?,
    fb
  );
  assert_eq!(
    get_string_or_boolean_option(
      CoercedOptionValue::Str("expand"),
      &bstrs,
      &benums,
      tval,
      fval,
      fb,
      "tz"
    )?,
    benums[1]
  );
  assert!(matches!(
    get_string_or_boolean_option(
      CoercedOptionValue::Str("bad"),
      &bstrs,
      &benums,
      tval,
      fval,
      fb,
      "tz"
    ),
    Err(Error::ValueOutOfRange("tz"))
  ));

  OK
}

// 序列化侧写入 → 交还 Vec<u8> (release 后 out_of_memory_ 应为 false)
fn ser_bytes(write: impl FnOnce(&mut ValueSerializerBuf)) -> Vec<u8> {
  let mut ser = ValueSerializerBuf::new();
  write(&mut ser);
  assert!(!ser.is_out_of_memory());
  ser.release()
}

#[test]
fn test_value_serializer_buffer_primitives_roundtrip() -> Void {
  info!("Testing structured-clone 缓冲级 varint/zigzag/double/raw 线上原语 1:1 往返");

  // 1. Header: tag 0xFF + varint(kLatestVersion=16)
  let bytes = ser_bytes(|s| s.write_header());
  let mut des = ValueDeserializerBuf::new(&bytes);
  des.read_header()?;
  assert_eq!(des.version(), 16);

  // 2. varint u32 边界 (单字节/多字节/u32::MAX)
  for v in [0u32, 1, 127, 128, 300, 1 << 28, u32::MAX] {
    let bytes = ser_bytes(|s| s.write_varint_u32(v));
    let mut d = ValueDeserializerBuf::new(&bytes);
    assert_eq!(d.read_varint_u32(), Some(v));
  }

  // 3. varint u64 大值
  for v in [0u64, 1 << 32, u64::MAX] {
    let bytes = ser_bytes(|s| s.write_varint_u64(v));
    let mut d = ValueDeserializerBuf::new(&bytes);
    assert_eq!(d.read_varint_u64(), Some(v));
  }

  // 4. zigzag i32 (符号位翻转往返)
  for v in [0i32, 1, -1, 2, -2, i32::MAX, i32::MIN, 1234, -567890] {
    let bytes = ser_bytes(|s| s.write_zigzag(v));
    let mut d = ValueDeserializerBuf::new(&bytes);
    assert_eq!(d.read_zigzag(), Some(v));
  }

  // 5. double (主机序 8 字节直落)
  for v in [0.0f64, 1.5, -2.25, f64::MAX, f64::MIN_POSITIVE] {
    let bytes = ser_bytes(|s| s.write_double(v));
    let mut d = ValueDeserializerBuf::new(&bytes);
    assert_eq!(d.read_double(), Some(v));
  }

  // 6. one-byte string: varint(len) + raw
  {
    let text: &[u8] = b"v8 structured clone";
    let bytes = ser_bytes(|s| s.write_one_byte_string(text));
    let mut d = ValueDeserializerBuf::new(&bytes);
    assert_eq!(d.read_varint_u32(), Some(text.len() as u32));
    assert_eq!(d.read_raw_bytes(text.len()), Some(text));
  }

  // 7. tag peek/read 与 kPadding 跳过
  {
    let mut ser = ValueSerializerBuf::new();
    ser.write_byte(0x00); // kPadding
    ser.write_tag(0x42);
    let bytes = ser.release();
    let mut d = ValueDeserializerBuf::new(&bytes);
    assert_eq!(d.peek_tag(), Some(0x42)); // 前瞻跳填充, 不移游标
    assert_eq!(d.read_tag(), Some(0x42)); // 消费亦跳填充
    assert_eq!(d.peek_tag(), None);
  }

  // 8. ReadRawTwoBytes 尺寸/越界拒绝 (非依赖对齐的确定分支)
  {
    let mut d = ValueDeserializerBuf::new(&[0u8; 8]);
    assert_eq!(d.read_raw_two_bytes(3), None); // 奇数 size
    assert_eq!(d.read_raw_two_bytes(100), None); // 越界
  }

  // 9. 越界读原语返回 None (不 panic)
  {
    let mut d = ValueDeserializerBuf::new(&[]);
    assert_eq!(d.read_byte(), None);
    assert_eq!(d.read_varint_u32(), None);
    assert_eq!(d.read_double(), None);
    assert_eq!(d.read_size_t(), None);
  }

  // 10. ReadSizeT 上限校验
  for v in [0usize, 4096, usize::MAX / 2] {
    let bytes = ser_bytes(|s| s.write_uint64(v as u64));
    let mut d = ValueDeserializerBuf::new(&bytes);
    assert_eq!(d.read_size_t(), Some(v));
  }

  OK
}
