use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use parking_lot::{Mutex, RwLock};

use crate::{
  error::{Error, Result},
  microtask::MicrotaskQueue,
  protectors::{Protector, Protectors},
  stack_guard::StackGuard,
  vm_state::{StateTag, ThreadId},
};

/// V8 核心隔离实例 (对应原版 v8::internal::Isolate)
/// 管理独立执行上下文、栈防护、保护器、微任务调度与运行状态
#[derive(Debug)]
pub struct Isolate {
  /// 绑定的线程标识符
  thread_id: ThreadId,
  /// 栈限制与中断管理
  stack_guard: StackGuard,
  /// 运行时优化保护器状态
  protectors: RwLock<Protectors>,
  /// 微任务队列
  microtask_queue: Mutex<MicrotaskQueue>,
  /// 虚拟机当前状态标签
  current_state: Mutex<StateTag>,
  /// 是否存在未捕获的挂起异常
  has_pending_exception: AtomicBool,
  /// 挂起异常的对象指针/地址
  pending_exception: AtomicUsize,
}

impl Default for Isolate {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

impl Isolate {
  /// 创建新的 Isolate 实例，默认绑定当前线程
  pub fn new() -> Self {
    Self {
      thread_id: ThreadId::current(),
      stack_guard: StackGuard::new(),
      protectors: RwLock::new(Protectors::new()),
      microtask_queue: Mutex::new(MicrotaskQueue::new()),
      current_state: Mutex::new(StateTag::IDLE),
      has_pending_exception: AtomicBool::new(false),
      pending_exception: AtomicUsize::new(0),
    }
  }

  /// 获取绑定的 ThreadId
  #[inline]
  pub const fn thread_id(&self) -> ThreadId {
    self.thread_id
  }

  /// 获取 StackGuard 引用
  #[inline]
  pub const fn stack_guard(&self) -> &StackGuard {
    &self.stack_guard
  }

  /// 检查特定保护器是否依然完整有效
  #[inline]
  pub fn is_protector_intact(&self, protector: Protector) -> bool {
    self.protectors.read().is_intact(protector)
  }

  /// 将特定保护器置为失效 (触发去优化依赖)
  pub fn invalidate_protector(&self, protector: Protector) {
    self.protectors.write().invalidate(protector);
  }

  /// 将特定微任务入队
  pub fn enqueue_microtask<F>(&self, task: F)
  where
    F: FnOnce() + Send + 'static,
  {
    self.microtask_queue.lock().enqueue(task);
  }

  /// 执行微任务检查点排空队列 (支持执行过程中动态添加微任务，且不发生死锁)
  pub fn perform_microtask_checkpoint(&self) -> usize {
    {
      let mut mq = self.microtask_queue.lock();
      if mq.is_running() || mq.is_empty() {
        return 0;
      }
      mq.set_running(true);
    }

    struct RunningGuard<'a>(&'a Mutex<MicrotaskQueue>);
    impl Drop for RunningGuard<'_> {
      #[inline]
      fn drop(&mut self) {
        self.0.lock().set_running(false);
      }
    }
    let _guard = RunningGuard(&self.microtask_queue);

    let mut executed = 0;
    while executed < MicrotaskQueue::MAX_NESTING_DEPTH {
      let task = self.microtask_queue.lock().pop_task();
      match task {
        Some(task) => {
          task();
          executed += 1;
          self.microtask_queue.lock().record_finished();
        }
        None => break,
      }
    }

    executed
  }

  /// 当前虚拟机状态
  pub fn current_vm_state(&self) -> StateTag {
    *self.current_state.lock()
  }

  /// 切换虚拟机运行状态并返回前一个状态
  pub fn set_vm_state(&self, new_state: StateTag) -> StateTag {
    let mut state = self.current_state.lock();
    let old = *state;
    *state = new_state;
    old
  }

  /// 抛出/登记挂起异常
  pub fn throw_exception(&self, exception_address: usize) {
    self
      .pending_exception
      .store(exception_address, Ordering::Release);
    self.has_pending_exception.store(true, Ordering::Release);
  }

  /// 是否有挂起异常
  #[inline]
  pub fn has_pending_exception(&self) -> bool {
    self.has_pending_exception.load(Ordering::Acquire)
  }

  /// 清除并返回挂起的异常对象地址
  pub fn clear_pending_exception(&self) -> Option<usize> {
    if self.has_pending_exception.swap(false, Ordering::SeqCst) {
      Some(self.pending_exception.swap(0, Ordering::SeqCst))
    } else {
      None
    }
  }

  /// 请求外部强制终止执行 (对应原版 TerminateExecution)
  pub fn terminate_execution(&self) {
    self.stack_guard.request_terminate_execution();
  }

  /// 是否收到了强制终止请求
  #[inline]
  pub fn is_execution_terminating(&self) -> bool {
    self.stack_guard.check_terminate_execution()
  }

  /// 取消/清除终止执行状态
  pub fn cancel_terminate_execution(&self) {
    self.stack_guard.clear_terminate_execution();
  }

  /// 栈限与中断综合门禁检查
  /// 在每个函数调用入口或循环回边处调用
  pub fn check_execution_limits(&self, current_sp: usize) -> Result<()> {
    // 1. 检查强制终止
    if self.is_execution_terminating() {
      return Err(Error::Terminated);
    }

    // 2. 检查物理栈溢出
    if self.stack_guard.is_stack_overflow(current_sp) {
      return Err(Error::StackOverflow);
    }

    // 3. 检查是否有未处理挂起异常
    if self.has_pending_exception() {
      let addr = self.pending_exception.load(Ordering::Acquire);
      return Err(Error::PendingException(addr));
    }

    Ok(())
  }

  /// 进入外部回调执行作用域，将状态切换为 StateTag::EXTERNAL，离开作用域自动恢复
  pub fn enter_external_scope(&self, callback_address: usize) -> IsolateExternalScope<'_> {
    let previous_state = self.set_vm_state(StateTag::EXTERNAL);
    IsolateExternalScope {
      isolate: self,
      previous_state,
      callback_address,
    }
  }
}

/// Isolate 级别的外部回调作用域守卫 (对应原版 ExternalCallbackScope)
#[derive(Debug)]
pub struct IsolateExternalScope<'a> {
  isolate: &'a Isolate,
  previous_state: StateTag,
  callback_address: usize,
}

impl<'a> IsolateExternalScope<'a> {
  /// 当前状态
  #[inline]
  pub fn current_state(&self) -> StateTag {
    self.isolate.current_vm_state()
  }

  /// 关联的回调函数地址
  #[inline]
  pub const fn callback_address(&self) -> usize {
    self.callback_address
  }
}

impl Drop for IsolateExternalScope<'_> {
  #[inline]
  fn drop(&mut self) {
    self.isolate.set_vm_state(self.previous_state);
  }
}
