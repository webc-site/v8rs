use aok::{OK, Void};
use log::info;
use v8_execution_isolate::{
  EncodedCSignature, Error, ExternalCallbackScope, InterruptLevel, InterruptType, Isolate,
  IsolateFieldId, MicrotaskQueue, MicrotasksPolicy, Protector, Protectors, StackGuard, StateTag,
  ThreadId, interrupt_flags,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_protectors_lifecycle() -> Void {
  let mut protectors = Protectors::new();
  assert!(protectors.is_all_intact());
  assert_eq!(protectors.raw_bits(), Protectors::ALL_VALID_MASK);

  // 逐一核验 22 项初始有效
  for p in Protector::ALL {
    assert!(protectors.is_intact(p));
  }

  // 逐个失效并验证独立性
  protectors.invalidate(Protector::ArrayBufferDetaching);
  assert!(!protectors.is_intact(Protector::ArrayBufferDetaching));
  assert!(!protectors.is_all_intact());
  assert!(protectors.is_intact(Protector::ArrayBufferMutable));
  assert!(protectors.is_intact(Protector::TypedArraySpeciesLookupChain));

  // 专用便利方法
  assert!(protectors.is_array_iterator_lookup_chain_intact());
  protectors.invalidate_array_iterator_lookup_chain();
  assert!(!protectors.is_array_iterator_lookup_chain_intact());

  info!("Protectors 22 项状态机测试通过");
  OK
}

#[test]
fn test_stack_guard_interrupts() -> Void {
  let guard = StackGuard::new();

  // 初始无中断
  assert!(!guard.has_any_interrupt());
  assert_eq!(guard.pending_interrupts(), 0);

  // 设置物理栈底为 0x7FFF_0000
  guard.set_stack_limit(0x7FFF_0000);
  assert_eq!(guard.real_jslimit(), 0x7FFF_0000);
  assert_eq!(guard.jslimit(), 0x7FFF_0000);

  // 请求 GC 中断
  guard.request_gc();
  assert!(guard.check_gc());
  assert!(guard.has_any_interrupt());
  assert_eq!(guard.jslimit(), StackGuard::INTERRUPT_LIMIT);

  // 等级检查：NoGC 等级下不应触达 GC (因为 GC 是 NoHeapWrites)
  assert!(!guard.has_interrupt_for_level(InterruptLevel::NoGC));
  assert!(guard.has_interrupt_for_level(InterruptLevel::NoHeapWrites));
  assert!(guard.has_interrupt_for_level(InterruptLevel::AnyEffect));

  // 严格验证 3 级中断过滤掩码
  assert_eq!(
    interrupt_flags::interrupt_level_mask(InterruptLevel::NoGC),
    0x041
  );
  assert_eq!(
    interrupt_flags::interrupt_level_mask(InterruptLevel::NoHeapWrites),
    0xD73
  );
  assert_eq!(
    interrupt_flags::interrupt_level_mask(InterruptLevel::AnyEffect),
    0xFFF
  );

  // 请求 TerminateExecution (NoGC)
  guard.request_terminate_execution();
  assert!(guard.check_terminate_execution());
  assert!(guard.has_interrupt_for_level(InterruptLevel::NoGC));

  // 清除 GC
  guard.clear_gc();
  assert!(!guard.check_gc());
  assert!(guard.check_terminate_execution());

  // 清除 Terminate
  guard.clear_terminate_execution();
  assert!(!guard.has_any_interrupt());
  assert_eq!(guard.jslimit(), 0x7FFF_0000);

  // 栈溢出检查
  assert!(!guard.is_stack_overflow(0x7FFF_1000));
  assert!(guard.is_stack_overflow(0x7FFE_F000));

  info!("StackGuard 中断与栈限测试通过");
  OK
}

#[test]
fn test_vm_state_and_thread_id() -> Void {
  // StateTag 谓词
  assert!(!StateTag::JS.is_external());
  assert!(!StateTag::JS.is_idle());
  assert!(StateTag::EXTERNAL.is_external());
  assert!(StateTag::IDLE.is_idle());
  assert!(StateTag::IDLE_EXTERNAL.is_external());
  assert!(StateTag::IDLE_EXTERNAL.is_idle());

  // ThreadId
  let tid = ThreadId::current();
  assert!(tid.is_valid());
  assert_eq!(ThreadId::from_integer(tid.to_integer()), tid);
  assert!(!ThreadId::invalid().is_valid());

  // 中断位与类型匹配验证
  assert_eq!(InterruptType::GC.mask(), interrupt_flags::GC_REQUEST);
  assert_eq!(
    InterruptType::TerminateExecution.mask(),
    interrupt_flags::TERMINATE_EXECUTION
  );

  // ExternalCallbackScope
  let mut state = StateTag::JS;
  {
    let _scope = ExternalCallbackScope::new(&mut state, 0x1234);
    assert_eq!(_scope.current_tag(), StateTag::EXTERNAL);
    assert_eq!(_scope.callback_address(), 0x1234);
  }
  assert_eq!(state, StateTag::JS);

  // EncodedCSignature
  let mut sig = EncodedCSignature::new(4);
  assert!(sig.is_valid());
  assert_eq!(sig.parameter_count(), 4);
  assert_eq!(sig.fp_parameter_count(), 0);

  sig.set_float(0);
  sig.set_float(2);
  sig.set_return_float();

  assert!(sig.is_float(0));
  assert!(!sig.is_float(1));
  assert!(sig.is_float(2));
  assert!(!sig.is_float(3));
  assert!(sig.is_return_float());
  assert_eq!(sig.fp_parameter_count(), 2);

  info!("VMState、ThreadId 与 EncodedCSignature 测试通过");
  OK
}

#[test]
fn test_microtask_queue() -> Void {
  use core::sync::atomic::{AtomicUsize, Ordering};
  use std::sync::Arc;

  let mut queue = MicrotaskQueue::new();
  assert_eq!(queue.microtasks_policy(), MicrotasksPolicy::Auto);
  assert!(queue.is_empty());

  let counter = Arc::new(AtomicUsize::new(0));

  let c1 = counter.clone();
  queue.enqueue(move || {
    c1.fetch_add(1, Ordering::SeqCst);
  });

  let c2 = counter.clone();
  queue.enqueue(move || {
    c2.fetch_add(10, Ordering::SeqCst);
  });

  assert_eq!(queue.size(), 2);
  let executed = queue.perform_checkpoint();
  assert_eq!(executed, 2);
  assert_eq!(queue.finished_microtask_count(), 2);
  assert!(queue.is_empty());
  assert_eq!(counter.load(Ordering::SeqCst), 11);

  // 作用域深度与状态变更
  assert_eq!(queue.microtasks_depth(), 0);
  queue.increment_scope_depth();
  assert_eq!(queue.microtasks_depth(), 1);
  queue.decrement_scope_depth();
  assert_eq!(queue.microtasks_depth(), 0);

  // 细粒度 pop_task 验证
  queue.enqueue(|| {});
  assert_eq!(queue.size(), 1);
  let task = queue.pop_task();
  assert!(task.is_some());
  assert!(queue.is_empty());

  info!("MicrotaskQueue 测试通过");
  OK
}

#[test]
fn test_isolate_lifecycle_and_execution_limits() -> Void {
  use core::sync::atomic::{AtomicUsize, Ordering};
  use std::sync::Arc;

  let isolate = Arc::new(Isolate::new());
  assert_eq!(isolate.current_vm_state(), StateTag::IDLE);

  // 状态流转
  let prev = isolate.set_vm_state(StateTag::JS);
  assert_eq!(prev, StateTag::IDLE);
  assert_eq!(isolate.current_vm_state(), StateTag::JS);

  // IsolateExternalScope RAII 测试
  {
    let scope = isolate.enter_external_scope(0x5678);
    assert_eq!(scope.current_state(), StateTag::EXTERNAL);
    assert_eq!(scope.callback_address(), 0x5678);
    assert_eq!(isolate.current_vm_state(), StateTag::EXTERNAL);
  }
  assert_eq!(isolate.current_vm_state(), StateTag::JS);

  // 保护器联动
  assert!(isolate.is_protector_intact(Protector::ArraySpeciesLookupChain));
  isolate.invalidate_protector(Protector::ArraySpeciesLookupChain);
  assert!(!isolate.is_protector_intact(Protector::ArraySpeciesLookupChain));

  // 异常传播
  assert!(!isolate.has_pending_exception());
  isolate.throw_exception(0xDEAD_BEEF);
  assert!(isolate.has_pending_exception());

  // 门禁在有异常时应拦截
  let err = isolate.check_execution_limits(0x7FFF_FFFF);
  assert_eq!(err, Err(Error::PendingException(0xDEAD_BEEF)));

  // 清除异常
  let ex = isolate.clear_pending_exception();
  assert_eq!(ex, Some(0xDEAD_BEEF));
  assert!(!isolate.has_pending_exception());

  // 终止执行
  assert!(!isolate.is_execution_terminating());
  isolate.terminate_execution();
  assert!(isolate.is_execution_terminating());

  let err = isolate.check_execution_limits(0x7FFF_FFFF);
  assert_eq!(err, Err(Error::Terminated));

  isolate.cancel_terminate_execution();
  assert!(!isolate.is_execution_terminating());

  // 微任务联动与动态链式排空 (验证无死锁)
  let chain_sum = Arc::new(AtomicUsize::new(0));
  let iso_ref = isolate.clone();
  let c_sum = chain_sum.clone();
  isolate.enqueue_microtask(move || {
    c_sum.fetch_add(1, Ordering::SeqCst);
    // 在微任务执行中动态注入下一级微任务
    let iso2 = iso_ref.clone();
    let c_sum2 = c_sum.clone();
    iso_ref.enqueue_microtask(move || {
      c_sum2.fetch_add(10, Ordering::SeqCst);
      // 在微任务执行中再次重入 perform_microtask_checkpoint 应安全返回 0 (防重入爆炸)
      let inner_exec = iso2.perform_microtask_checkpoint();
      assert_eq!(inner_exec, 0);
    });
  });

  let exec_count = isolate.perform_microtask_checkpoint();
  assert_eq!(exec_count, 2);
  assert_eq!(chain_sum.load(Ordering::SeqCst), 11);

  // IsolateFieldId 校验 (全 84 项 0..83 与 V8 严格对齐)
  assert_eq!(IsolateFieldId::COUNT, 84);
  assert_eq!(IsolateFieldId::ALL.len(), 84);
  assert_eq!(IsolateFieldId::CageBase.as_u8(), 0);
  assert_eq!(IsolateFieldId::CageBase.name(), "CageBase");
  assert_eq!(IsolateFieldId::TablesAlignmentPadding.as_u8(), 10);
  assert_eq!(IsolateFieldId::ActiveStack.name(), "ActiveStack");
  assert_eq!(IsolateFieldId::ActiveSuspender.name(), "ActiveSuspender");
  assert_eq!(
    IsolateFieldId::DefaultLocaleMayRequireSpecialCaseMapping.name(),
    "DefaultLocaleMayRequireSpecialCaseMapping"
  );
  assert_eq!(
    IsolateFieldId::RawArgumentsPadding.name(),
    "RawArgumentsPadding"
  );
  assert_eq!(IsolateFieldId::NoHeapWriteInterruptRequest.as_u8(), 83);
  assert_eq!(
    IsolateFieldId::NoHeapWriteInterruptRequest.name(),
    "NoHeapWriteInterruptRequest"
  );

  // 遍历全部 84 项确保连续性与唯一性
  for (idx, field) in IsolateFieldId::ALL.iter().enumerate() {
    assert_eq!(field.as_u8() as usize, idx);
    assert!(!field.name().is_empty());
  }

  info!("Isolate 核心生命周期与综合门禁测试通过");
  OK
}
